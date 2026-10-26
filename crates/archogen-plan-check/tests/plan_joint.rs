//! Check (5), the joint rules (leaf `M3.5.5`), from `docs/semantics/model.md` §8: each case a plan run through every
//! check in order, so a corruption reaches check (5) only when checks (1)–(4) accept it.

use std::collections::BTreeMap;

use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::{plan, Verdict, PLAN_JOINT};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// A console service on a platform that composes a serial port; `spare.uart`, a second one, is written where each case
/// puts it.
fn description(spare: &str, system_uses: &str) -> String {
    format!(
        "(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
{spare}
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write{system_uses}))
  (platform (uses soc)))
"
    )
}

/// The console service, as most cases write it.
const CONSOLE: &str = "(defservice console.write (requires (needs observable-output)))";

fn elaborated(declarations: &str) -> (SourceMap, Program) {
    let modules = MemoryModules::new().with(
        "app.case",
        &format!("(defmodule app.case\n  (version 1 0)\n{declarations})"),
    );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app.case");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    (sources, program)
}

/// The console bound to `provider`, every check run.
fn console_at(declarations: &str, provider: &str) -> Result<(), Verdict> {
    let (sources, program) = elaborated(declarations);
    let mut bindings = BTreeMap::new();
    bindings.insert(
        (
            Requester::Named("console.write".to_owned()),
            "observable-output".to_owned(),
        ),
        provider.to_owned(),
    );
    let record = Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "s".to_owned(),
        program: program_digest(&program),
        placement: None,
        bindings,
    };
    plan(
        &sources,
        &program,
        &RT_STATIC_UP_V1,
        "plan.eadl",
        &record.to_canonical(),
    )
}

#[test]
fn rule_7_a_binding_to_the_system_s_hardware_holds() {
    let accepted = description(CONSOLE, "");
    assert_eq!(console_at(&accepted, "console.uart"), Ok(()));
    // A block the system's own `uses` names is its hardware, as one its platform composes is.
    let named = description(CONSOLE, " spare.uart");
    assert_eq!(console_at(&named, "spare.uart"), Ok(()));
}

#[test]
fn rule_7_a_binding_outside_the_system_s_hardware_is_a_conflict() {
    for (case, declarations) in [
        ("composed by nothing", description(CONSOLE, "")),
        // A service's `uses` is no edge of the hardware: only a block or platform composes one.
        (
            "named by a service alone",
            description(
                "(defservice console.write (requires (needs observable-output) (uses spare.uart)))",
                "",
            ),
        ),
        // `refines` is no edge: a refinement keeps the abstract platform's obligations, not its blocks.
        (
            "composed by the platform the system's platform refines",
            description(
                &format!("{CONSOLE}\n(defplatform abstract.soc (requires (uses spare.uart)))"),
                "",
            )
            .replace(
                "(defplatform soc (offers",
                "(defplatform soc (refines abstract.soc) (offers",
            ),
        ),
    ] {
        let verdict = console_at(&declarations, "spare.uart").unwrap_err();
        assert_eq!(verdict.status, Status::ToolFailure, "{case}: {verdict:?}");
        assert_eq!(verdict.findings.len(), 1, "{case}: {verdict:?}");
        let finding = &verdict.findings[0];
        assert_eq!(finding.code, PLAN_JOINT, "{case}");
        assert!(
            finding.message.contains("`spare.uart`")
                && finding.message.contains("system `s`")
                && finding.message.contains("rule 7"),
            "{case}: {finding:?}"
        );
        // The binding in the record, and the requirement where the description writes it.
        assert_eq!(
            (finding.primary.source.as_str(), finding.primary.line),
            ("plan.eadl", 5),
            "{case}"
        );
        assert!(
            finding
                .secondary
                .iter()
                .any(|l| l.source == "app.case.eadl" && l.message == "the requirement"),
            "{case}: {finding:?}"
        );
    }
}

/// A system whose tasks each need a timer's channel, or reach it through one shared service; the timer offers
/// `channels` as `channels` writes it.
fn timed(channels: &str, tasks: &[&str], shared: bool) -> String {
    let need = if shared {
        "(uses sleep)"
    } else {
        "(needs relative-delay)"
    };
    let tasks: String = tasks
        .iter()
        .enumerate()
        .map(|(i, t)| {
            format!(
                "  (task {t} (period 10 ms) (deadline 10 ms) (deadline-from release) (priority {}) {need})\n",
                i + 1
            )
        })
        .collect();
    format!(
        "(defblock timer (offers (relative-delay true) {channels} (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer)))
(defservice sleep (requires (needs relative-delay)))
(defsystem s
{tasks}  (platform (uses soc)))
"
    )
}

/// Each requester's `relative-delay` bound to the timer, every check run.
fn delays_at_timer(declarations: &str, requesters: &[Requester]) -> Result<(), Verdict> {
    let (sources, program) = elaborated(declarations);
    let bindings = requesters
        .iter()
        .map(|r| ((r.clone(), "relative-delay".to_owned()), "timer".to_owned()))
        .collect();
    let record = Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "s".to_owned(),
        program: program_digest(&program),
        placement: None,
        bindings,
    };
    plan(
        &sources,
        &program,
        &RT_STATIC_UP_V1,
        "plan.eadl",
        &record.to_canonical(),
    )
}

fn task(name: &str) -> Requester {
    Requester::Task {
        system: "s".to_owned(),
        task: name.to_owned(),
    }
}

/// The one finding of a joint refusal.
fn joint_finding(verdict: Verdict) -> archogen_outcome::record::Finding {
    assert_eq!(verdict.status, Status::ToolFailure, "{verdict:?}");
    assert_eq!(verdict.findings.len(), 1, "{verdict:?}");
    let finding = verdict.findings.into_iter().next().unwrap();
    assert_eq!(finding.code, PLAN_JOINT);
    finding
}

#[test]
fn rules_2_to_4_requesters_taking_a_channel_fit_the_provider_s_channels() {
    let ab = [task("a"), task("b")];
    assert_eq!(
        delays_at_timer(&timed("(channels 2)", &["a", "b"], false), &ab),
        Ok(())
    );
    // One requester needs no count, whatever `channels` says.
    assert_eq!(
        delays_at_timer(&timed("channels", &["a"], false), &[task("a")]),
        Ok(())
    );
    // A service several tasks reach is one requester: one channel.
    assert_eq!(
        delays_at_timer(
            &timed("(channels 1)", &["a", "b"], true),
            &[Requester::Named("sleep".to_owned())]
        ),
        Ok(())
    );
}

#[test]
fn rule_3_more_requesters_than_channels_is_a_conflict() {
    let finding = joint_finding(
        delays_at_timer(
            &timed("(channels 2)", &["a", "b", "c"], false),
            &[task("a"), task("b"), task("c")],
        )
        .unwrap_err(),
    );
    assert!(
        finding.message.contains("number 3")
            && finding.message.contains("`channels` 2")
            && finding.message.contains("the task `c` of `s`")
            && finding.message.contains("rule 3"),
        "{finding:?}"
    );
    // Each binding in the record, and what the timer states of its channels.
    assert_eq!(finding.primary.source, "plan.eadl");
    assert!(
        finding
            .secondary
            .iter()
            .any(|l| l.source == "app.case.eadl" && l.message.contains("`timer`")),
        "{finding:?}"
    );
}

#[test]
fn rule_3_two_requesters_where_channels_has_no_value_is_an_unknown() {
    for (channels, why) in [
        ("channels", "without a value"),
        ("", "does not describe `channels`"),
    ] {
        let finding = joint_finding(
            delays_at_timer(
                &timed(channels, &["a", "b"], false),
                &[task("a"), task("b")],
            )
            .unwrap_err(),
        );
        assert!(
            finding.message.contains(why) && finding.message.contains("unknown"),
            "{channels:?}: {finding:?}"
        );
    }
}

/// Two services the system uses, each requiring `relative-delay` as `written`, at a provider offering it as `offered`
/// with one channel at most.
fn two_services(written: &str, offered: &str) -> String {
    format!(
        "(defblock timer (offers (relative-delay {offered}) (channels 1) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer)))
(defservice wait.a (requires (relative-delay {written})))
(defservice wait.b (requires (relative-delay {written})))
(defsystem s
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses wait.a wait.b))
  (platform (uses soc)))
"
    )
}

#[test]
fn rule_2_a_consumed_fact_takes_a_channel_when_met_by_true_and_never_by_false() {
    let both = [
        Requester::Named("wait.a".to_owned()),
        Requester::Named("wait.b".to_owned()),
    ];
    // `(f true)`: each service takes the one channel, and two do not fit.
    let finding = joint_finding(delays_at_timer(&two_services("true", "true"), &both).unwrap_err());
    assert!(finding.message.contains("number 2"), "{finding:?}");
    // `(f false)` takes none, however many read it.
    assert_eq!(
        delays_at_timer(&two_services("false", "false"), &both),
        Ok(())
    );
}
