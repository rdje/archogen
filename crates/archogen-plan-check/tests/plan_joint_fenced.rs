//! Check (5), the joint rules, as the fenced context of `M3.5.5.8` reads them from `docs/semantics/model.md` §8: per
//! rule, a plan accepted and its corruptions refused. Each plan goes through checks (1) to (4) first, which must
//! accept it — a plan they refuse is no case of check (5) — then through [`joint_fenced`]; the few corruptions check
//! (3) refuses first are run apart, to hold check (5)'s own reading of them. An engine adapter's instance is judged by
//! the rules as model §8 rule 15 and `docs/specs/adapters/decision_engine-adapters.md` §4 say, and one time base by rule
//! 14, each from the record and the model alone; a task released by its own source by rule 16, written from its text
//! alone by the fenced context of `M4.1.2.3`.

use std::collections::BTreeMap;

use archogen_outcome::record::{Finding, Severity};
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Instance, Provider, Requester};
use archogen_plan_check::check::{
    bindings, description, joint, record, Verdict, PLAN_BINDING, PLAN_DESCRIPTION, PLAN_JOINT,
    PLAN_RECORD, PLAN_UNSATISFIED,
};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::{Profile, RT_STATIC_UP_V1};

/// The record's name in every finding's span.
const RECORD: &str = "plan.eadl";

/// The description's file, the root module `app`.
const SOURCE: &str = "app.eadl";

/// A description, its declarations the root module `app`'s, so their names are unqualified.
struct Description {
    sources: SourceMap,
    program: Program,
}

fn describe(declarations: &str) -> Description {
    let modules = MemoryModules::new().with(
        "app",
        &format!("(defmodule app\n  (version 1 0)\n{declarations})"),
    );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    Description { sources, program }
}

fn named(name: &str) -> Requester {
    Requester::Named(name.to_owned())
}

fn task(system: &str, task: &str) -> Requester {
    Requester::Task {
        system: system.to_owned(),
        task: task.to_owned(),
    }
}

/// An engine adapter's instance as a requester, from its canonical text.
fn instance(text: &str) -> Requester {
    Requester::parse(text).unwrap_or_else(|refusal| panic!("{text}: {refusal}"))
}

/// A provider from its canonical text: a name, or an instance.
fn provider(text: &str) -> Provider {
    Provider::parse(text).unwrap_or_else(|refusal| panic!("{text}: {refusal}"))
}

/// One binding: requester, fact, provider — a name, or an instance's canonical text.
type Bind = (Requester, &'static str, &'static str);

impl Description {
    /// The record a search would write for the system `s`: `binds` its bindings, `placement` its placement.
    fn plan(&self, placement: Option<(&str, &str)>, binds: &[Bind]) -> Assignment {
        Assignment {
            profile: RT_STATIC_UP_V1.id.to_owned(),
            system: "s".to_owned(),
            program: program_digest(&self.program),
            placement: placement.map(|(p, r)| (p.to_owned(), r.to_owned())),
            bindings: binds
                .iter()
                .map(|(r, f, p)| ((r.clone(), (*f).to_owned()), provider(p)))
                .collect::<BTreeMap<_, _>>(),
        }
    }

    /// Checks (1) to (4) under `profile`, each of which must accept, then check (5)'s verdict.
    fn joint_under(&self, profile: &Profile, assignment: &Assignment) -> Result<(), Verdict> {
        let text = assignment.to_canonical();
        if let Err(verdict) = record(&self.program, profile, RECORD, &text) {
            panic!("check (1) refuses the plan:\n{text}{verdict:#?}");
        }
        if let Err(verdict) = description(&self.sources, &self.program, profile) {
            panic!("check (2) refuses the description: {verdict:#?}");
        }
        if let Err(verdict) = bindings(&self.sources, &self.program, RECORD, &text, assignment) {
            panic!("checks (3) and (4) refuse the plan:\n{text}{verdict:#?}");
        }
        joint(
            &self.sources,
            &self.program,
            profile,
            RECORD,
            &text,
            assignment,
        )
    }

    fn joint(&self, assignment: &Assignment) -> Result<(), Verdict> {
        self.joint_under(&RT_STATIC_UP_V1, assignment)
    }

    /// `binds` as a plan with no placement, judged.
    fn bound(&self, binds: &[Bind]) -> Result<(), Verdict> {
        self.joint(&self.plan(None, binds))
    }
}

/// The findings of a plan check (5) refuses: `tool-failure`, each `plan-joint`, an error with a place and a repair, and
/// each naming `(rule N)` — every one of them when `every`, else at least one.
fn refusal(verdict: Result<(), Verdict>, rule: u32, every: bool) -> Vec<Finding> {
    let verdict = verdict.expect_err("check (5) refuses the plan");
    assert_eq!(verdict.status, Status::ToolFailure, "{verdict:#?}");
    assert!(!verdict.findings.is_empty());
    let naming = |f: &Finding| f.message.contains(&format!("(rule {rule})"));
    for finding in &verdict.findings {
        assert_eq!(finding.code, PLAN_JOINT, "{finding:#?}");
        assert_eq!(finding.severity, Severity::Error);
        assert!(!finding.repair.is_empty(), "{finding:#?}");
        assert!(finding.primary.line >= 1 && !finding.primary.message.is_empty());
        assert!(!every || naming(finding), "{finding:#?}");
    }
    assert!(
        verdict.findings.iter().any(naming),
        "{:#?}",
        verdict.findings
    );
    verdict.findings
}

/// The messages of a plan check (5) refuses, every finding under `(rule N)`.
fn refused(verdict: Result<(), Verdict>, rule: u32) -> String {
    messages(&refusal(verdict, rule, true))
}

fn messages(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(|f| f.message.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The messages of a description check (5) refuses whatever the assignment: `status`, every finding `plan-description`
/// naming `(rule N)` and pointing into the description.
fn refused_description(verdict: Result<(), Verdict>, status: Status, rule: u32) -> String {
    let verdict = verdict.expect_err("check (5) refuses the description");
    assert_eq!(verdict.status, status, "{verdict:#?}");
    for finding in &verdict.findings {
        assert_eq!(finding.code, PLAN_DESCRIPTION, "{finding:#?}");
        assert_eq!(finding.severity, Severity::Error);
        assert!(
            finding.message.contains(&format!("(rule {rule})")),
            "{finding:#?}"
        );
        assert!(!finding.repair.is_empty(), "{finding:#?}");
        assert_eq!(finding.primary.source, SOURCE, "{finding:#?}");
        assert!(finding.primary.line > 2, "{finding:#?}");
    }
    messages(&verdict.findings)
}

/// The record line of the binding of `requester`'s `fact`, 1-based.
fn line_of(plan: &Assignment, requester: &Requester, fact: &str) -> u32 {
    let clause = format!("(bind {} {fact} ", requester.canonical());
    let index = plan
        .to_canonical()
        .lines()
        .position(|line| line.starts_with(&clause))
        .expect("the binding is in the record");
    u32::try_from(index + 1).expect("a short record")
}

// ------------------------------------------------------------------------------------------------------------------
// Rules 1 and 7: requesters, and the system's hardware.

/// `reference.md`'s `plan-joint` row, a task needing the console itself beside it.
const CONSOLE: &str = "
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (task log (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs observable-output))
    (requires (uses console.write))
    (platform (uses soc)))";

#[test]
fn rule_7_each_requester_the_system_serves_bound_to_its_hardware_holds() {
    let d = describe(CONSOLE);
    // A service and a task, two requesters of one shared fact at one provider: a shared fact takes no channel (rule 2).
    assert_eq!(
        d.bound(&[
            (named("console.write"), "observable-output", "console.uart"),
            (task("s", "log"), "observable-output", "console.uart"),
        ]),
        Ok(())
    );
}

#[test]
fn rule_7_a_requester_bound_outside_the_system_s_hardware_is_refused_at_its_binding() {
    let d = describe(CONSOLE);
    for (stray, other) in [
        (named("console.write"), task("s", "log")),
        (task("s", "log"), named("console.write")),
    ] {
        let plan = d.plan(
            None,
            &[
                (stray.clone(), "observable-output", "spare.uart"),
                (other, "observable-output", "console.uart"),
            ],
        );
        let findings = refusal(d.joint(&plan), 7, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        let finding = &findings[0];
        assert!(
            finding
                .message
                .contains(&format!("`{}`", stray.canonical()))
                && finding.message.contains("`spare.uart`")
                && finding.message.contains("hardware of `s`"),
            "{finding:#?}"
        );
        assert_eq!(finding.primary.source, RECORD);
        assert_eq!(
            finding.primary.line,
            line_of(&plan, &stray, "observable-output")
        );
        // The provider, where the requester requires the fact, and the system, in the description.
        assert_eq!(finding.secondary.len(), 3, "{finding:#?}");
        assert!(finding.secondary.iter().all(|l| l.source == SOURCE));
    }
}

#[test]
fn rule_7_hardware_is_composed_at_any_depth_and_never_through_refines() {
    // `soc` composes `bus.bridge` inside `requires`, which composes `console.uart` in a `uses` clause of its own;
    // `soc` refines `soc.base`, whose `spare.uart` is no hardware of `s`.
    let d = describe(
        "
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock bus.bridge (offers (bus-width 32 bit) (reachable-at-privilege machine)) (uses console.uart))
      (defplatform soc.base (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses spare.uart)))
      (defplatform soc (refines soc.base) (offers (core-count 1 tick) (reachable-at-privilege machine))
        (requires (uses bus.bridge)))
      (defservice console.write (requires (needs observable-output)))
      (defsystem s (requires (uses console.write)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[(named("console.write"), "observable-output", "console.uart")]),
        Ok(())
    );
    let message = refused(
        d.bound(&[(named("console.write"), "observable-output", "spare.uart")]),
        7,
    );
    assert!(message.contains("`spare.uart`"), "{message}");
}

#[test]
fn rule_7_a_block_a_service_names_is_reached_but_is_no_hardware() {
    // `those` in rule 7 are the blocks and platforms found: a service's `needs` of a block makes it a requester the
    // system serves, never hardware.
    let d = describe(
        "
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
      (defservice console.write (requires (needs observable-output spare.uart)))
      (defsystem s (requires (uses console.write)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[(named("console.write"), "observable-output", "console.uart")]),
        Ok(())
    );
    refused(
        d.bound(&[(named("console.write"), "observable-output", "spare.uart")]),
        7,
    );
}

#[test]
fn rules_1_and_7_a_binding_check_three_refuses_first_is_refused_by_check_five_s_own_reading_too() {
    let d = describe(CONSOLE);
    let held = (task("s", "log"), "observable-output", "console.uart");
    let cases: [(Bind, u32, &str); 6] = [
        (
            (
                named("console.nowhere"),
                "observable-output",
                "console.uart",
            ),
            1,
            "names no declaration",
        ),
        (
            (task("s", "ghost"), "observable-output", "console.uart"),
            1,
            "no task `ghost`",
        ),
        (
            (task("t", "beat"), "observable-output", "console.uart"),
            7,
            "an assignment is one system's",
        ),
        (
            (named("spare.uart"), "observable-output", "console.uart"),
            7,
            "no requester `s` serves",
        ),
        (
            (named("console.write"), "console.uart", "console.uart"),
            1,
            "no fact of the vocabulary",
        ),
        (
            (named("console.write"), "observable-output", "console.write"),
            7,
            "no block or platform",
        ),
    ];
    for (corrupt, rule, words) in cases {
        let plan = d.plan(None, &[held.clone(), corrupt]);
        let text = plan.to_canonical();
        assert!(record(&d.program, &RT_STATIC_UP_V1, RECORD, &text).is_ok());
        assert_eq!(
            description(&d.sources, &d.program, &RT_STATIC_UP_V1),
            Ok(())
        );
        assert!(
            bindings(&d.sources, &d.program, RECORD, &text, &plan).is_err(),
            "check (3) refuses it first:\n{text}"
        );
        let findings = refusal(
            joint(
                &d.sources,
                &d.program,
                &RT_STATIC_UP_V1,
                RECORD,
                &text,
                &plan,
            ),
            rule,
            false,
        );
        assert!(messages(&findings).contains(words), "{findings:#?}");
    }
}

// ------------------------------------------------------------------------------------------------------------------
// Rules 2 to 4: consumed facts, channels, and a node several users reach.

/// A counter offering both consumed functions with `channels` as `pool` writes it, a lazy one whose `relative-delay`
/// is `false`, the services that use them — `time.wake` needing the deadline bare — and the system `system`.
fn timers(pool: &str, system: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.delay (offers (relative-delay true) (absolute-deadline true) (supported-horizon 3600 s) {pool}
        (reachable-at-privilege machine)))
      (defblock timer.lazy (offers (relative-delay false) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.delay timer.lazy)))
      (defservice time.sleep (requires (needs relative-delay)))
      (defservice time.alarm (requires (relative-delay true)))
      (defservice time.deadline (requires (absolute-deadline (supported-horizon (at-least 10 s)))))
      (defservice time.wake (requires (needs absolute-deadline)))
      (defservice time.both (requires (needs relative-delay) (absolute-deadline true)))
      (defservice time.never (requires (relative-delay false)))
      (defservice time.none (requires (relative-delay (exactly false))))
      {system}"
    ))
}

const THREE: &str =
    "(defsystem s (requires (uses time.sleep time.alarm time.deadline)) (platform (uses soc)))";

fn three() -> [Bind; 3] {
    [
        (named("time.sleep"), "relative-delay", "timer.delay"),
        (named("time.alarm"), "relative-delay", "timer.delay"),
        (named("time.deadline"), "absolute-deadline", "timer.delay"),
    ]
}

#[test]
fn rule_3_requesters_taking_channels_number_at_most_the_provider_s_channels() {
    // `(needs f)`, `(f true)` and the group whose head `f` is each take a channel (rule 2).
    assert_eq!(timers("(channels 3)", THREE).bound(&three()), Ok(()));
    assert_eq!(timers("(channels 4)", THREE).bound(&three()), Ok(()));
    let d = timers("(channels 2)", THREE);
    let plan = d.plan(None, &three());
    let findings = refusal(d.joint(&plan), 3, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    for named in [
        "`timer.delay`",
        "3 requesters",
        "`time.sleep` through `relative-delay`",
        "`time.alarm` through `relative-delay`",
        "`time.deadline` through `absolute-deadline`",
        "offers 2 `channels`",
    ] {
        assert!(message.contains(named), "{named}: {message}");
    }
    // At the record's first binding of them; each requester's requirement and the provider in the description.
    assert_eq!(findings[0].primary.source, RECORD);
    assert_eq!(
        findings[0].primary.line,
        line_of(&plan, &named("time.alarm"), "relative-delay")
    );
    assert_eq!(findings[0].secondary.len(), 4, "{findings:#?}");
}

#[test]
fn rule_3_two_requesters_where_channels_is_undescribed_or_offered_bare_are_an_unknown() {
    let two = "(defsystem s (requires (uses time.sleep time.alarm)) (platform (uses soc)))";
    let binds = &three()[..2];
    for (pool, why) in [
        ("", "does not describe `channels`"),
        ("channels", "offers `channels` without a value"),
    ] {
        let message = refused(timers(pool, two).bound(binds), 3);
        assert!(
            message.contains(why) && message.contains("missing-fact"),
            "{message}"
        );
    }
    // One requester needs no count.
    let one = "(defsystem s (requires (uses time.sleep)) (platform (uses soc)))";
    assert_eq!(timers("", one).bound(&three()[..1]), Ok(()));
}

#[test]
fn rule_2_a_requirement_met_by_false_takes_no_channel() {
    // Two requesters of `(relative-delay false)` at a provider that describes no `channels`: neither takes one.
    let d = timers(
        "",
        "(defsystem s (requires (uses time.sleep time.never time.none)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.sleep"), "relative-delay", "timer.delay"),
            (named("time.never"), "relative-delay", "timer.lazy"),
            (named("time.none"), "relative-delay", "timer.lazy"),
        ]),
        Ok(())
    );
}

#[test]
fn rule_3_counts_requesters_so_one_binding_both_consumed_facts_takes_one_channel() {
    let d = timers(
        "(channels 1)",
        "(defsystem s (requires (uses time.both)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.both"), "absolute-deadline", "timer.delay"),
            (named("time.both"), "relative-delay", "timer.delay"),
        ]),
        Ok(())
    );
    // Beside another requester, the two consumed functions share one pool.
    let d = timers(
        "(channels 1)",
        "(defsystem s (requires (uses time.both time.sleep)) (platform (uses soc)))",
    );
    let message = refused(
        d.bound(&[
            (named("time.both"), "absolute-deadline", "timer.delay"),
            (named("time.both"), "relative-delay", "timer.delay"),
            (named("time.sleep"), "relative-delay", "timer.delay"),
        ]),
        3,
    );
    assert!(
        message.contains("2 requesters")
            && message.contains("`time.both` through `absolute-deadline`, `relative-delay`"),
        "{message}"
    );
}

#[test]
fn rule_4_a_service_several_users_reach_is_one_requester() {
    // Two tasks and the system reach `time.sleep`: one requester of one channel.
    let shared = "(defsystem s
        (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.sleep))
        (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses time.sleep))
        (requires (uses time.sleep)) (platform (uses soc)))";
    assert_eq!(timers("(channels 1)", shared).bound(&three()[..1]), Ok(()));
    // A task programming the counter itself is a second requester.
    let direct = "(defsystem s
        (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.sleep))
        (task c (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs relative-delay))
        (requires (uses time.sleep)) (platform (uses soc)))";
    let message = refused(
        timers("(channels 1)", direct).bound(&[
            (named("time.sleep"), "relative-delay", "timer.delay"),
            (task("s", "c"), "relative-delay", "timer.delay"),
        ]),
        3,
    );
    assert!(
        message.contains("`(task s c)` through `relative-delay`")
            && message.contains("offers 1 `channels`"),
        "{message}"
    );
}

/// Two tasks of the system using `time.wake`, its own level using nothing; each also needing the deadline itself where
/// `own` writes `(needs absolute-deadline)`.
fn wakers(own: &str) -> String {
    format!(
        "(defsystem s
        (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.wake) {own})
        (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses time.wake) {own})
        (platform (uses soc)))"
    )
}

#[test]
fn rule_4_a_node_two_tasks_use_counts_once_at_its_provider_and_its_multiplexer_is_no_finding() {
    // `time.wake` takes a channel through `(needs absolute-deadline)`. Its users are the two tasks — the system's own
    // level is none — so it is the multiplexer, an obligation for the realization to discharge, never a finding; and
    // it counts once at the counter however many reach it, rule 3's count.
    let d = timers("(channels 1 tick)", &wakers(""));
    assert_eq!(
        d.bound(&[(named("time.wake"), "absolute-deadline", "timer.delay")]),
        Ok(())
    );
}

#[test]
fn rule_4_a_user_bound_itself_at_the_node_s_provider_is_a_requester_of_its_own() {
    // Each task needing the deadline itself too, bound to the counter: the node counts once and each task once more,
    // no user again through the node — three requesters, rule 3's.
    let own = wakers("(needs absolute-deadline)");
    let binds = [
        (named("time.wake"), "absolute-deadline", "timer.delay"),
        (task("s", "a"), "absolute-deadline", "timer.delay"),
        (task("s", "b"), "absolute-deadline", "timer.delay"),
    ];
    assert_eq!(timers("(channels 3 tick)", &own).bound(&binds), Ok(()));
    for (pool, offered) in [
        ("(channels 1 tick)", "offers 1 `channels`"),
        ("(channels 2 tick)", "offers 2 `channels`"),
    ] {
        let d = timers(pool, &own);
        let findings = refusal(d.joint(&d.plan(None, &binds)), 3, true);
        assert_eq!(findings.len(), 1, "{pool}: {findings:#?}");
        let message = &findings[0].message;
        for named in [
            "`timer.delay` gives 3 requesters a channel",
            "`time.wake` through `absolute-deadline`",
            "`(task s a)` through `absolute-deadline`",
            "`(task s b)` through `absolute-deadline`",
            offered,
        ] {
            assert!(message.contains(named), "{named}: {message}");
        }
    }
}

#[test]
fn rule_4_a_block_bound_for_a_requirement_of_its_own_counts_once_however_many_tasks_reach_it() {
    // `watchdog` is a node by its own `(needs absolute-deadline)`, and both tasks reach it through `uses`, a call: one
    // requester of the counter's one channel.
    let d = describe(
        "
      (defblock timer.delay (offers (absolute-deadline true) (supported-horizon 3600 s) (channels 1 tick)
        (reachable-at-privilege machine)))
      (defblock watchdog (offers (reachable-at-privilege machine)) (requires (needs absolute-deadline)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.delay)))
      (defsystem s
        (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses watchdog))
        (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses watchdog))
        (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[(named("watchdog"), "absolute-deadline", "timer.delay")]),
        Ok(())
    );
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 5: ownership.

/// A counter and a console, each with the region `timer` and `uart` write, on a platform with a bus window over both
/// and a spare device; the system uses `uses`.
fn devices(timer: &str, uart: &str, uses: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine) {timer}))
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine) {uart}))
      (defblock spare.dev (offers (debug-port true) (region regs (base 0x1000_0000) (size 64 KiB))))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)
          (region bus (base 0x1000_0000) (size 1 MiB)))
        (requires (uses timer.a console.uart spare.dev)))
      (defservice time.monotonic (requires (counter-width 32 bit)))
      (defservice console.write (requires (needs observable-output)))
      (defservice sched.core (requires (core-count 1 tick)))
      (defsystem s (requires (uses {uses})) (platform (uses soc)))"
    ))
}

fn two_devices() -> [Bind; 2] {
    [
        (named("time.monotonic"), "counter-width", "timer.a"),
        (named("console.write"), "observable-output", "console.uart"),
    ]
}

#[test]
fn rule_5_two_bound_providers_whose_bytes_do_not_overlap_hold() {
    // Adjacent ranges; and `spare.dev`, bound to nothing, owns nothing though its bytes cover both.
    let d = devices(
        "(region regs (base 0x1000_0000) (size 4 KiB))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write",
    );
    assert_eq!(d.bound(&two_devices()), Ok(()));
    // A region without size owns its base byte: one lying wholly below it overlaps nothing.
    let d = devices(
        "(region regs (base 0x1000_2000))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write",
    );
    assert_eq!(d.bound(&two_devices()), Ok(()));
}

#[test]
fn rule_5_two_bound_providers_owning_one_byte_are_a_conflict_naming_both_ranges() {
    for (timer, uart, range) in [
        (
            "(region regs (base 0x1000_0000) (size 4 KiB))",
            "(region regs (base 0x1000_0800) (size 4 KiB))",
            "[0x10000800, 0x10001800)",
        ),
        // A reserved region is owned too; and a region without size owns its base byte, here the other's first —
        // an overlap, not the unknown of the bytes above it.
        (
            "(region fw (base 0x1000_0000) (size 4 KiB) (reserved true))",
            "(region regs (base 0x1000_0000))",
            "[0x10000000, …) with no size",
        ),
    ] {
        let d = devices(timer, uart, "time.monotonic console.write");
        let plan = d.plan(None, &two_devices());
        let findings = refusal(d.joint(&plan), 5, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        let message = &findings[0].message;
        assert!(
            message.contains("`console.uart`")
                && message.contains("`timer.a`")
                && message.contains("[0x10000000, 0x10001000)")
                && message.contains(range)
                && message.contains("own the same bytes"),
            "{message}"
        );
        assert_eq!(findings[0].primary.source, RECORD);
        assert!(findings[0].secondary.iter().any(|l| l.source == SOURCE));
    }
}

#[test]
fn rule_5_a_region_without_size_below_another_bound_provider_s_bytes_is_an_unknown() {
    let d = devices(
        "(region regs (base 0x1000_0000))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write",
    );
    let message = refused(d.bound(&two_devices()), 5);
    assert!(
        message.contains("is unknown")
            && message.contains("`regs` has no size")
            && message.contains("missing-fact"),
        "{message}"
    );
}

#[test]
fn rule_5_a_provider_composing_another_through_uses_owns_its_bytes_with_it() {
    // `soc`, bound for `sched.core`, composes both blocks: its bus window over theirs is no conflict.
    let d = devices(
        "(region regs (base 0x1000_0000) (size 4 KiB))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write sched.core",
    );
    let mut binds = two_devices().to_vec();
    binds.push((named("sched.core"), "core-count", "soc"));
    assert_eq!(d.bound(&binds), Ok(()));
}

/// A counter and a console whose regions overlap, each composed by the platform; the console names what `link` writes,
/// beside the declaration `more`.
fn linked(link: &str, more: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)
        (region regs (base 0x1000_0000) (size 4 KiB))))
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)
        (region regs (base 0x1000_0800) (size 4 KiB))) {link})
      {more}
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a console.uart)))
      (defservice time.monotonic (requires (counter-width 32 bit)))
      (defservice console.write (requires (needs observable-output)))
      (defsystem s (requires (uses time.monotonic console.write)) (platform (uses soc)))"
    ))
}

#[test]
fn rule_5_composition_is_a_block_s_or_a_platform_s_uses_and_never_a_service_s() {
    // Rule 5: one composes the other when it names it through `uses`, at any depth through blocks and platforms, whose
    // `uses` alone is composition — directly, through a block, or through a platform; the two own the bytes together.
    for (link, more) in [
        ("(uses timer.a)", ""),
        (
            "(uses bus.bridge)",
            "(defblock bus.bridge (offers (bus-width 32 bit)) (uses timer.a))",
        ),
        (
            "(uses board)",
            "(defplatform board (offers (core-count 1 tick)) (requires (uses timer.a)))",
        ),
    ] {
        assert_eq!(linked(link, more).bound(&two_devices()), Ok(()), "{link}");
    }
    // A service's `uses` is a binding — here its own requirement bound to the counter — and composes nothing: the
    // console reaching the counter only through one composes nothing, and their overlap is a conflict.
    let d = linked(
        "(uses time.source)",
        "(defservice time.source (requires (counter-width 32 bit)) (uses timer.a))",
    );
    let mut binds = two_devices().to_vec();
    binds.push((named("time.source"), "counter-width", "timer.a"));
    let findings = refusal(d.bound(&binds), 5, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains("`console.uart`")
            && message.contains("`timer.a`")
            && message.contains("[0x10000000, 0x10001000)")
            && message.contains("[0x10000800, 0x10001800)")
            && message.contains("own the same bytes"),
        "{message}"
    );
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 9: privilege at the profile's level.

/// A console its `console` clauses describe, on a platform that states no level, written to by a service whose
/// requirements beside `(needs observable-output)` are `caller`, and by the task `log`.
fn privileged(console: &str, caller: &str) -> Description {
    describe(&format!(
        "
      (defblock console.uart {console})
      (defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart)))
      (defservice console.write (requires (needs observable-output) {caller}))
      (defsystem s
        (task log (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs observable-output))
        (requires (uses console.write)) (platform (uses soc)))"
    ))
}

fn both_callers() -> [Bind; 2] {
    [
        (named("console.write"), "observable-output", "console.uart"),
        (task("s", "log"), "observable-output", "console.uart"),
    ]
}

/// `rt-static-up-v1`, its callers run at `level`.
fn at_level(level: &'static str) -> Profile {
    Profile {
        caller_level: level,
        ..RT_STATIC_UP_V1
    }
}

#[test]
fn rule_9_each_provider_bound_for_a_caller_is_reachable_at_the_profile_s_level() {
    for console in [
        "(offers (observable-output true) (reachable-at-privilege machine))",
        "(offers (observable-output true) (reachable-at-privilege supervisor machine))",
    ] {
        assert_eq!(
            privileged(console, "").bound(&both_callers()),
            Ok(()),
            "{console}"
        );
    }
    // A caller stating the level, or a set holding it, passes.
    for caller in [
        "(reachable-at-privilege machine)",
        "(reachable-at-privilege supervisor machine)",
    ] {
        let d = privileged(
            "(offers (observable-output true) (reachable-at-privilege machine))",
            caller,
        );
        assert_eq!(d.bound(&both_callers()), Ok(()), "{caller}");
    }
}

#[test]
fn rule_9_a_provider_bound_for_a_caller_and_not_reachable_at_the_level_is_refused() {
    for (console, why, conflict) in [
        (
            "(offers (observable-output true) (reachable-at-privilege supervisor))",
            "is reachable from `supervisor` alone",
            true,
        ),
        (
            "(offers (observable-output true)) (absent reachable-at-privilege)",
            "reachable from no level",
            true,
        ),
        (
            "(offers (observable-output true) reachable-at-privilege)",
            "bare, naming no level",
            false,
        ),
        (
            "(offers (observable-output true))",
            "does not describe `reachable-at-privilege`",
            false,
        ),
    ] {
        let findings = refusal(privileged(console, "").bound(&both_callers()), 9, true);
        assert_eq!(findings.len(), 1, "one finding per provider: {findings:#?}");
        let message = &findings[0].message;
        assert!(
            message.contains(why)
                && message.contains("`console.write` for `observable-output`")
                && message.contains("`(task s log)` for `observable-output`")
                && message.contains("`machine`")
                && (conflict != message.contains("missing-fact")),
            "{message}"
        );
    }
}

#[test]
fn rule_9_a_caller_stating_levels_that_do_not_hold_the_profile_s_is_the_description_s_unsupported_profile(
) {
    // Judged first: the provider, reachable from `supervisor` alone, would be the plan's conflict.
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor))",
        "(reachable-at-privilege supervisor)",
    );
    let message = refused_description(d.bound(&both_callers()), Status::UnsupportedProfile, 9);
    assert!(
        message.contains("`console.write` states it calls from `supervisor`")
            && message.contains("`machine`"),
        "{message}"
    );
}

#[test]
fn rule_9_the_level_is_the_profile_s_never_the_record_s() {
    let supervisor = at_level("supervisor");
    // Under a profile running callers at `supervisor`, a console reachable from `machine` alone is refused ...
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege machine))",
        "",
    );
    let message = refused(
        d.joint_under(&supervisor, &d.plan(None, &both_callers())),
        9,
    );
    assert!(
        message.contains("runs every caller at `supervisor`"),
        "{message}"
    );
    // ... one reachable from both holds, and a caller stating `machine` alone states a level no context runs at.
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor machine))",
        "",
    );
    assert_eq!(
        d.joint_under(&supervisor, &d.plan(None, &both_callers())),
        Ok(())
    );
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor machine))",
        "(reachable-at-privilege machine)",
    );
    refused_description(
        d.joint_under(&supervisor, &d.plan(None, &both_callers())),
        Status::UnsupportedProfile,
        9,
    );
}

#[test]
fn rule_9_with_no_level_given_says_nothing_and_a_level_outside_the_domain_is_the_engine_s_input_error(
) {
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor))",
        "",
    );
    let plan = d.plan(None, &both_callers());
    assert_eq!(d.joint_under(&at_level(""), &plan), Ok(()));
    let verdict = d
        .joint_under(&at_level("hypervisor"), &plan)
        .expect_err("a level the vocabulary does not name");
    assert_eq!(verdict.status, Status::ToolFailure);
    assert_eq!(verdict.findings.len(), 1);
    assert!(
        verdict.findings[0].message.contains("`hypervisor`")
            && verdict.findings[0]
                .message
                .contains("the engine's input error")
            && verdict.findings[0].message.contains("(rule 9)"),
        "{verdict:#?}"
    );
    assert_eq!(
        verdict.findings[0].primary.line, 2,
        "the record's `profile` clause"
    );
}

#[test]
fn rule_9_judges_callers_never_a_block_needing_another_s_fact() {
    // `timer.a`'s own requirement binds `clock.main`, which states no level: a block has none to be reached at.
    let d = describe(
        "
      (defblock clock.main (offers (clock-rate 10 MHz)))
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)) (requires (clock-rate 10 MHz)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a clock.main)))
      (defservice time.monotonic (requires (counter-width 32 bit)))
      (defsystem s (requires (uses time.monotonic)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.monotonic"), "counter-width", "timer.a"),
            (named("timer.a"), "clock-rate", "clock.main"),
        ]),
        Ok(())
    );
    // The same provider bound for a caller is refused.
    let d = describe(
        "
      (defblock clock.main (offers (clock-rate 10 MHz)))
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a clock.main)))
      (defservice time.monotonic (requires (counter-width 32 bit) (clock-rate 10 MHz)))
      (defsystem s (requires (uses time.monotonic)) (platform (uses soc)))",
    );
    let message = refused(
        d.bound(&[
            (named("time.monotonic"), "counter-width", "timer.a"),
            (named("time.monotonic"), "clock-rate", "clock.main"),
        ]),
        9,
    );
    assert!(message.contains("`clock.main`"), "{message}");
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 10: placement.

/// A console on a platform whose regions `regions` writes, beside a memory block no platform composes.
fn memory(regions: &str) -> Description {
    describe(&format!(
        "
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock spare.mem (offers (debug-port true) (region ram (base 0x4000_0000) (size 1 MiB) (executable true))))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine) {regions})
        (requires (uses console.uart)))
      (defservice console.write (requires (needs observable-output)))
      (defsystem s (requires (uses console.write)) (platform (uses soc)))"
    ))
}

const MAP: &str = "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
  (region fw (base 0x8000_0000) (size 1 MiB) (reserved true))
  (region rom (base 0x2000_0000) (size 1 MiB) (executable true) (reserved true))
  (region regs (base 0x1000_0000) (size 4 KiB))
  (region boot (base 0x0) (size 64 KiB) (reserved true))";

fn write() -> [Bind; 1] {
    [(named("console.write"), "observable-output", "console.uart")]
}

#[test]
fn rule_10_a_plan_places_the_image_in_placement_memory_of_the_system_s_hardware() {
    // A reserved range over placement memory is a hole the plan avoids; ranges wholly below it, each with its size,
    // overlap none of it. A region against placement memory with no size is the description's unknown (rule 10).
    let d = memory(MAP);
    assert_eq!(d.joint(&d.plan(Some(("soc", "ram")), &write())), Ok(()));
    // A system with no placement memory is said nothing of.
    let d = memory("(region regs (base 0x1000_0000) (size 4 KiB))");
    assert_eq!(d.joint(&d.plan(None, &write())), Ok(()));
}

#[test]
fn rule_10_a_placement_that_is_not_placement_memory_of_the_hardware_is_refused() {
    let d = memory(MAP);
    for (placement, why) in [
        (("soc", "rom"), "is reserved"),
        (("soc", "regs"), "is not `(executable true)`"),
        (("soc", "nowhere"), "offers no region `nowhere`"),
        (
            ("spare.mem", "ram"),
            "`spare.mem` is outside the hardware of `s`",
        ),
        (("console.write", "ram"), "is no block or platform"),
    ] {
        let plan = d.plan(Some(placement), &write());
        let findings = refusal(d.joint(&plan), 10, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(findings[0].message.contains(why), "{findings:#?}");
        // At the record's `placement` clause, after `/2`'s `adapters` clause.
        assert_eq!(
            (
                findings[0].primary.source.as_str(),
                findings[0].primary.line
            ),
            (RECORD, 6)
        );
    }
    // A plan placing the image nowhere, where the hardware has placement memory.
    let message = refused(d.joint(&d.plan(None, &write())), 10);
    assert!(
        message.contains("`soc`'s region `ram` [0x80000000, 0x88000000)")
            && message.contains("places the image in none"),
        "{message}"
    );
}

/// Check (5)'s verdict on a plan placing the image in `soc`'s `ram`, reached directly through `joint`: check (2) refuses
/// the description first, description-wide, under `first`, and checks (1), (3) and (4) accept the plan.
fn past_check_two(d: &Description, first: Status) -> Result<(), Verdict> {
    let plan = d.plan(Some(("soc", "ram")), &write());
    let text = plan.to_canonical();
    assert!(record(&d.program, &RT_STATIC_UP_V1, RECORD, &text).is_ok());
    let refused = description(&d.sources, &d.program, &RT_STATIC_UP_V1)
        .expect_err("check (2) refuses it first");
    assert_eq!(refused.status, first, "{refused:#?}");
    assert_eq!(
        bindings(&d.sources, &d.program, RECORD, &text, &plan),
        Ok(())
    );
    joint(
        &d.sources,
        &d.program,
        &RT_STATIC_UP_V1,
        RECORD,
        &text,
        &plan,
    )
}

#[test]
fn rule_10_a_claim_without_size_below_placement_memory_is_the_description_s_missing_fact() {
    // Check (2) refuses it first, description-wide; check (5) judges it again over the system's hardware.
    let d = memory(
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region regs (base 0x1000_0000))",
    );
    let message = refused_description(
        past_check_two(&d, Status::MissingFact),
        Status::MissingFact,
        10,
    );
    assert!(
        message.contains("`regs` has no size") && message.contains("`ram`"),
        "{message}"
    );
}

#[test]
fn rule_10_a_reserved_range_against_placement_memory_with_no_size_is_the_description_s_missing_fact(
) {
    // Rule 10 names a reserved range, whose extent is the hole the plan avoids, among the regions against placement
    // memory: where it has no size and the memory's bytes lie above its base, whether they overlap is `missing-fact`.
    for (regions, open, against) in [
        // A reserved range with no size below placement memory: whether it reaches into it is unknown.
        (
            "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
             (region boot (base 0x7fff_0000) (reserved true))",
            "`boot` has no size",
            "`ram`",
        ),
        // So for an executable one: reserved, it is no placement memory, and a reserved range.
        (
            "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
             (region rom (base 0x2000_0000) (executable true) (reserved true))",
            "`rom` has no size",
            "`ram`",
        ),
        // A hole in one placement region, below another: taken to the first's end, it is unknown against the second.
        (
            "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
             (region more (base 0x9000_0000) (size 1 MiB) (executable true))
             (region fw (base 0x8001_0000) (reserved true))",
            "`fw` has no size",
            "`more`",
        ),
    ] {
        let d = memory(regions);
        let message = refused_description(
            past_check_two(&d, Status::MissingFact),
            Status::MissingFact,
            10,
        );
        assert_eq!(message.lines().count(), 1, "{regions}: {message}");
        assert!(
            message.contains(open) && message.contains(against) && message.contains("is unknown"),
            "{regions}: {message}"
        );
    }
    // Decided with no size: a reserved range above placement memory's last byte overlaps none of it, and one whose base
    // lies inside it is a hole, taken to reach its end, that leaves free bytes below. Checks (1) to (4) accept, and check
    // (5) too.
    for regions in [
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region top (base 0x9000_0000) (reserved true))",
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region fw (base 0x8001_0000) (reserved true))",
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region fw (base 0x87ff_ffff) (reserved true))",
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region lo (base 0x8000_0000) (size 2 MiB) (reserved true))
         (region hi (base 0x8030_0000) (reserved true))",
    ] {
        let d = memory(regions);
        assert_eq!(
            d.joint(&d.plan(Some(("soc", "ram")), &write())),
            Ok(()),
            "{regions}"
        );
    }
}

#[test]
fn rule_10_placement_memory_with_no_size_is_the_description_s_missing_fact_naming_it() {
    for (case, regions) in [
        (
            "nothing against it",
            "(region ram (base 0x8000_0000) (executable true))",
        ),
        (
            "a reserved range above it",
            "(region ram (base 0x8000_0000) (executable true))
             (region fw (base 0x9000_0000) (size 1 MiB) (reserved true))",
        ),
        (
            "a device's registers above it",
            "(region ram (base 0x8000_0000) (executable true))
             (region regs (base 0x9000_0000) (size 4 KiB))",
        ),
        (
            "a reserved range with no size at its base",
            "(region ram (base 0x8000_0000) (executable true))
             (region fw (base 0x8000_0000) (reserved true))",
        ),
    ] {
        let d = memory(regions);
        let message = refused_description(
            past_check_two(&d, Status::MissingFact),
            Status::MissingFact,
            10,
        );
        assert_eq!(
            message,
            "`soc`'s placement memory `ram` has no size: nothing can be laid out in it, in the hardware of `s` \
             (rule 10)",
            "{case}"
        );
    }
    // A region with no size below it is unknown against it besides: its bytes, at least its base, lie above that base.
    let d = memory(
        "(region ram (base 0x8000_0000) (executable true))
         (region regs (base 0x1000_0000))",
    );
    let message = refused_description(
        past_check_two(&d, Status::MissingFact),
        Status::MissingFact,
        10,
    );
    assert!(
        message.contains("`regs` has no size") && message.contains("`ram` has no size:"),
        "{message}"
    );
}

#[test]
fn rule_10_holes_of_no_size_leaving_no_free_byte_are_the_description_s_missing_fact() {
    const RAM: &str = "`soc`'s region `ram` [0x80000000, 0x88000000)";
    for (case, regions, named, labels) in [
        (
            "at the memory's base",
            "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
             (region fw (base 0x8000_0000) (reserved true))",
            "`soc`'s region `fw` [0x80000000, …) with no size",
            1,
        ),
        (
            "a sized hole up to its base",
            "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
             (region lo (base 0x8000_0000) (size 2 MiB) (reserved true))
             (region hi (base 0x8020_0000) (reserved true))",
            "`soc`'s region `hi` [0x80200000, …) with no size",
            1,
        ),
        (
            "two of them, each named, the higher adding no byte",
            "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
             (region lo (base 0x8000_0000) (reserved true))
             (region hi (base 0x8020_0000) (reserved true))",
            "`soc`'s region `lo` [0x80000000, …) with no size, `soc`'s region `hi` [0x80200000, …) with no size",
            2,
        ),
    ] {
        let d = memory(regions);
        let verdict = past_check_two(&d, Status::MissingFact).expect_err(case);
        assert_eq!(verdict.status, Status::MissingFact, "{case}");
        assert_eq!(verdict.findings.len(), 1, "{case}: {verdict:#?}");
        let finding = &verdict.findings[0];
        assert_eq!(finding.code, PLAN_DESCRIPTION, "{case}");
        assert_eq!(
            finding.message,
            format!(
                "{RAM} has no free byte with its holes of no size, {named}, taken to reach its end, and the bytes \
                 surely reserved do not fill it, in the hardware of `s` (rule 10)"
            ),
            "{case}"
        );
        assert_eq!(finding.repair, "give each hole of no size its `(size S)`", "{case}");
        assert_eq!(finding.primary.source, SOURCE, "{case}");
        // Each hole of no size has its place, and so has the memory they leave nothing of.
        assert_eq!(finding.secondary.len(), labels, "{case}: {finding:#?}");
        assert!(
            finding.secondary.iter().all(|l| l.source == SOURCE),
            "{case}: {finding:#?}"
        );
    }
}

#[test]
fn rule_10_memory_surely_reserved_in_every_byte_is_check_two_s_and_no_unknown_of_check_five() {
    // The base of a hole of no size is a byte surely reserved, so these bytes fill the region: no size is asked for.
    let d = memory(
        "(region ram (base 0x8000_0000) (size 1 byte) (executable true))
         (region fw (base 0x8000_0000) (reserved true))",
    );
    assert_eq!(past_check_two(&d, Status::InvalidDescription), Ok(()));
}

/// Placement memory beside a device's region of the size `size`.
fn sized(size: &str) -> Description {
    memory(&format!(
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region regs (base 0x1000_0000) (size {size}))"
    ))
}

#[test]
fn rule_5_a_size_past_the_exact_arithmetic_is_unsupported_profile_with_check_five_called_alone() {
    // Rule 5: "one past the exact arithmetic `unsupported-profile`". A decimal of 39 fractional digits is too precise to
    // hold exactly (§1 rule 5): check (2) refuses it first under that status, and check (5) alone must say the same.
    let d = sized("0.000000000000000000000000000000000000001 byte");
    let message = refused_description(
        past_check_two(&d, Status::UnsupportedProfile),
        Status::UnsupportedProfile,
        5,
    );
    assert!(
        message.contains("a region `soc` offers does not read whole"),
        "{message}"
    );
    // A size that does not read for any other cause stays `invalid-description` in both: `GiB` is no unit.
    let d = sized("9223372036854775807 GiB");
    let message = refused_description(
        past_check_two(&d, Status::InvalidDescription),
        Status::InvalidDescription,
        5,
    );
    assert!(message.contains("`GiB` is not a known unit"), "{message}");
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 13: one function, one provider, and the qualifiers.

/// Two counters offering `a` and `b`, and a service reading time from them, its requirements beside the counter's
/// width and rate `more`; the system uses it, its own requirements `own`.
fn counters(a: &str, b: &str, more: &str, own: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.a (offers {a} (reachable-at-privilege machine)))
      (defblock timer.b (offers {b} (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
      (defservice time.monotonic (requires (counter-width 32 bit) (tick-rate 10 MHz) {more}))
      (defsystem s (requires (uses time.monotonic) {own}) (platform (uses soc)))"
    ))
}

const COUNTER: &str = "(counter-width 32 bit) (tick-rate 10 MHz)";

fn split(width: &'static str, rate: &'static str) -> [Bind; 2] {
    [
        (named("time.monotonic"), "counter-width", width),
        (named("time.monotonic"), "tick-rate", rate),
    ]
}

#[test]
fn rule_13_one_requester_s_facts_of_one_function_bound_to_one_provider_hold() {
    let d = counters(COUNTER, COUNTER, "", "");
    assert_eq!(d.bound(&split("timer.a", "timer.a")), Ok(()));
    assert_eq!(d.bound(&split("timer.b", "timer.b")), Ok(()));
}

#[test]
fn rule_13_a_split_one_provider_meets_whole_is_the_assignment_s_conflict() {
    let d = counters(
        "(counter-width 32 bit) (tick-rate 10 MHz)",
        "(counter-width 64 bit) (tick-rate 10 MHz)",
        "",
        "",
    );
    let plan = d.plan(None, &split("timer.a", "timer.b"));
    let findings = refusal(d.joint(&plan), 13, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains("`time.monotonic`")
            && message.contains("`counter` function to 2 providers")
            && message.contains("`timer.a` for `counter-width`")
            && message.contains("`timer.b` for `tick-rate`")
            && message.contains("meets every one"),
        "{message}"
    );
    assert_eq!(
        findings[0].primary.line,
        line_of(&plan, &named("time.monotonic"), "counter-width")
    );
}

#[test]
fn rule_13_a_split_none_meets_whole_and_one_refutes_nothing_of_is_an_unknown_naming_it() {
    let d = counters("(counter-width 32 bit)", "(tick-rate 10 MHz)", "", "");
    let message = refused(d.bound(&split("timer.a", "timer.b")), 13);
    assert!(
        message.contains("refutes none")
            && message.contains("`timer.a`, where `tick-rate` undescribed")
            && message.contains("`timer.b`, where `counter-width` undescribed")
            && message.contains("missing-fact"),
        "{message}"
    );
}

#[test]
fn rule_13_a_split_each_provider_refutes_part_of_is_a_conflict_naming_each() {
    let d = counters(
        "(counter-width 32 bit) (tick-rate 1 MHz)",
        "(counter-width 16 bit) (tick-rate 10 MHz)",
        "",
        "",
    );
    let message = refused(d.bound(&split("timer.a", "timer.b")), 13);
    assert!(
        message.contains("each refutes one")
            && message.contains("`timer.a` for `counter-width`")
            && message.contains("`timer.b` for `tick-rate`"),
        "{message}"
    );
}

#[test]
fn rule_13_is_per_requester_and_never_spans_two() {
    let d = describe(
        "
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)))
      (defblock timer.b (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
      (defservice time.width (requires (counter-width 32 bit)))
      (defservice time.rate (requires (tick-rate 10 MHz)))
      (defsystem s (requires (uses time.width time.rate)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.width"), "counter-width", "timer.a"),
            (named("time.rate"), "tick-rate", "timer.b"),
        ]),
        Ok(())
    );
}

#[test]
fn rule_13_a_qualifier_is_judged_at_every_provider_its_requester_relies_on() {
    let whole = "(counter-width 32 bit) (tick-rate 10 MHz)";
    // The service's own qualifier, and the system's, which reaches the service through `uses`: no binding of its own.
    for (more, own) in [
        ("(available-in-state idle)", ""),
        ("", "(available-in-state idle)"),
    ] {
        let d = counters(
            &format!("{whole} (available-in-state run idle)"),
            whole,
            more,
            own,
        );
        assert_eq!(d.bound(&split("timer.a", "timer.a")), Ok(()), "{more}{own}");
        for (a, why, conflict) in [
            (
                format!("{whole} (available-in-state run)"),
                "is refused there",
                true,
            ),
            (whole.to_owned(), "is undescribed there", false),
        ] {
            let d = counters(&a, whole, more, own);
            let message = refused(d.bound(&split("timer.a", "timer.a")), 13);
            assert!(
                message.contains(why)
                    && message.contains("`available-in-state`")
                    && message.contains("relies on `timer.a`")
                    && (conflict != message.contains("an unknown")),
                "{message}"
            );
        }
    }
}

#[test]
fn rule_13_never_repeats_rule_9_s_qualifier() {
    // The service states two levels; its provider is reachable from the profile's alone. Rule 9 judges the provider at
    // `machine`, and rule 13 judges `reachable-at-privilege` nowhere.
    let d = counters(
        COUNTER,
        COUNTER,
        "(reachable-at-privilege supervisor machine)",
        "",
    );
    assert_eq!(d.bound(&split("timer.a", "timer.a")), Ok(()));
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 15: an engine adapter's instance, judged by rules 1–3, 5, 7, 9 and 13; and rule 14, one time base.

/// What `deadline-over-delay` requires beneath, as a plan binds it: each fact once, sorted.
const BENEATH_DEADLINE: [&str; 4] = [
    "counter-modulus",
    "observation-coherent",
    "relative-delay",
    "tick-rate",
];

/// What `epoch-extender` requires beneath.
const BENEATH_EXTENDER: [&str; 5] = [
    "counter-modulus",
    "observation-coherent",
    "relative-delay",
    "tick-rate",
    "wrap-behavior",
];

/// Counters an engine adapter may program, and a deadline service: `timer.d`, a relative timer of 32 bits at 1 MHz
/// offering what `d` adds, its channels among them; `timer.short`, a 16-bit modular counter of one channel; `timer.e`, a
/// second counter, offering what `e` adds; `timer.u`, reachable from `user` alone; `timer.abs`, a comparator programmed
/// with absolute times itself, of two channels; `timer.far`, on no platform the system uses. Then the services `more` declares, and the
/// system `system`.
fn adapted(d: &str, e: &str, more: &str, system: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.d (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz)
        (observation-coherent true) {d} (reachable-at-privilege machine)))
      (defblock timer.short (offers (relative-delay true) (counter-modulus 65536) (counter-width 16 bit) (tick-rate 1 MHz)
        (observation-coherent true) (wrap-behavior modular) (channels 1 tick) (reachable-at-privilege machine)))
      (defblock timer.e (offers (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true)
        (reachable-at-privilege machine) {e}))
      (defblock timer.u (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz)
        (observation-coherent true) (channels 1 tick) (reachable-at-privilege user)))
      (defblock timer.abs (offers (absolute-deadline true) (supported-horizon 3600 s) (counter-modulus 4294967296)
        (tick-rate 1 MHz) (channels 2 tick) (reachable-at-privilege machine)))
      (defblock timer.far (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz)
        (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine))
        (requires (uses timer.d timer.short timer.e timer.u timer.abs)))
      (defservice alarm
        (requires (absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 100 us)))))
      {more}
      {system}"
    ))
}

/// The system using the deadline service alone.
const ALARMED: &str = "(defsystem s (requires (uses alarm)) (platform (uses soc)))";

/// The deadline service's own translator over `timer.d`.
const ALARM: &str = "(adapter deadline-over-delay timer.d alarm)";

/// The bindings beneath of the instance `text`: each of `facts` to `over`.
fn beneath(text: &'static str, over: &'static str, facts: &[&'static str]) -> Vec<Bind> {
    facts
        .iter()
        .map(|fact| (instance(text), *fact, over))
        .collect()
}

/// The deadline service bound through its translator over `timer.d`, with the translator's four bindings beneath.
fn alarm() -> Vec<Bind> {
    let mut binds = vec![(named("alarm"), "absolute-deadline", ALARM)];
    binds.extend(beneath(ALARM, "timer.d", &BENEATH_DEADLINE));
    binds
}

#[test]
fn rule_15_a_deadline_service_bound_through_its_own_translator_holds() {
    let d = adapted("(channels 1 tick)", "", "", ALARMED);
    // Its supported horizon `timer.d`'s unambiguous one, `(2^32 − 1) / 1 MHz`; its delivery bound the obliged 100 us.
    assert_eq!(d.bound(&alarm()), Ok(()));
    // Bound to `timer.d` itself, which offers no absolute deadline, check (4) refuses it.
    let direct = d.plan(None, &[(named("alarm"), "absolute-deadline", "timer.d")]);
    let verdict = bindings(
        &d.sources,
        &d.program,
        RECORD,
        &direct.to_canonical(),
        &direct,
    )
    .expect_err("`timer.d` offers no absolute deadline");
    assert_eq!(verdict.status, Status::ToolFailure);
    assert!(
        verdict.findings.iter().all(|f| f.code == PLAN_UNSATISFIED),
        "{verdict:#?}"
    );
}

/// The epoch extender over the 16-bit counter.
const EXTENDED: &str = "(adapter epoch-extender timer.short)";

/// A monotonic time service needing a minute's horizon, beside the counter facts `more` requires.
fn monotonic(more: &str) -> Description {
    adapted(
        "(channels 1 tick)",
        "",
        &format!(
            "(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s)) {more}))"
        ),
        "(defsystem s (requires (uses time.monotonic)) (platform (uses soc)))",
    )
}

#[test]
fn rule_15_a_sixteen_bit_modular_counter_under_an_epoch_extender_meets_a_minute_s_horizon() {
    let d = monotonic("");
    let mut binds = vec![(named("time.monotonic"), "unambiguous-horizon", EXTENDED)];
    binds.extend(beneath(EXTENDED, "timer.short", &BENEATH_EXTENDER));
    // The horizon derived at the instance from its `(pow2 64)` modulus and the counter's rate.
    assert_eq!(d.bound(&binds), Ok(()));
    // Directly, the counter wraps after 65.535 ms: check (4) refuses it.
    let direct = d.plan(
        None,
        &[(
            named("time.monotonic"),
            "unambiguous-horizon",
            "timer.short",
        )],
    );
    let verdict = bindings(
        &d.sources,
        &d.program,
        RECORD,
        &direct.to_canonical(),
        &direct,
    )
    .expect_err("65.535 ms is no minute");
    assert_eq!(verdict.findings.len(), 1, "{verdict:#?}");
    assert_eq!(verdict.findings[0].code, PLAN_UNSATISFIED);
    assert!(
        verdict.findings[0].message.contains("lies outside it"),
        "{verdict:#?}"
    );
}

/// Two deadline services, each with its own translator over `timer.d`, whose channels `channels` writes.
fn two_alarms(channels: &str) -> (Description, Vec<Bind>) {
    const A: &str = "(adapter deadline-over-delay timer.d alarm.a)";
    const B: &str = "(adapter deadline-over-delay timer.d alarm.b)";
    let d = adapted(
        channels,
        "",
        "(defservice alarm.a (requires (absolute-deadline (supported-horizon (at-least 10 s)))))
         (defservice alarm.b (requires (absolute-deadline (supported-horizon (at-least 10 s)))))",
        "(defsystem s (requires (uses alarm.a alarm.b)) (platform (uses soc)))",
    );
    let mut binds = vec![
        (named("alarm.a"), "absolute-deadline", A),
        (named("alarm.b"), "absolute-deadline", B),
    ];
    binds.extend(beneath(A, "timer.d", &BENEATH_DEADLINE));
    binds.extend(beneath(B, "timer.d", &BENEATH_DEADLINE));
    (d, binds)
}

#[test]
fn rules_2_and_3_two_exclusive_instances_over_a_timer_of_one_channel_are_refused() {
    // Each instance takes its own channel beneath, through `relative-delay`.
    let (d, binds) = two_alarms("(channels 2 tick)");
    assert_eq!(d.bound(&binds), Ok(()));
    let (d, binds) = two_alarms("(channels 1 tick)");
    let plan = d.plan(None, &binds);
    let findings = refusal(d.joint(&plan), 3, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    for named in [
        "`timer.d` gives 2 requesters a channel",
        "`(adapter deadline-over-delay timer.d alarm.a)` through `relative-delay`",
        "`(adapter deadline-over-delay timer.d alarm.b)` through `relative-delay`",
        "offers 1 `channels`",
    ] {
        assert!(message.contains(named), "{named}: {message}");
    }
    assert_eq!(findings[0].primary.source, RECORD);
}

#[test]
fn rules_1_3_and_7_an_instance_binding_check_three_refuses_first_is_refused_by_check_five_s_own_reading_too(
) {
    let d = adapted("(channels 2 tick)", "", "", ALARMED);
    const OTHER: &str = "(adapter deadline-over-delay timer.d alarm.other)";
    const IDLE: &str = "(adapter epoch-extender timer.short)";
    let cases: [(Vec<Bind>, u32, &str); 4] = [
        // An exclusive instance bound for a requester it does not name: it offers one channel, its one requester's.
        (
            {
                let mut binds = vec![(named("alarm"), "absolute-deadline", OTHER)];
                binds.extend(beneath(OTHER, "timer.d", &BENEATH_DEADLINE));
                binds
            },
            3,
            "serves `alarm.other` alone",
        ),
        // An instance no requester the system serves is bound to.
        (
            {
                let mut binds = alarm();
                binds.extend(beneath(IDLE, "timer.short", &BENEATH_EXTENDER));
                binds
            },
            7,
            "no requester it serves is bound to it",
        ),
        // A fact its kind does not require beneath.
        (
            {
                let mut binds = alarm();
                binds.push((instance(ALARM), "wrap-behavior", "timer.d"));
                binds
            },
            1,
            "which its kind does not require beneath",
        ),
        // Its binding beneath to a provider other than the one it programs.
        (
            {
                let mut binds = alarm();
                binds.retain(|(r, f, _)| !(*r == instance(ALARM) && *f == "tick-rate"));
                binds.push((instance(ALARM), "tick-rate", "timer.e"));
                binds
            },
            1,
            "to the device it programs, `timer.d`",
        ),
    ];
    for (binds, rule, words) in cases {
        let plan = d.plan(None, &binds);
        let text = plan.to_canonical();
        assert!(record(&d.program, &RT_STATIC_UP_V1, RECORD, &text).is_ok());
        let verdict = bindings(&d.sources, &d.program, RECORD, &text, &plan)
            .expect_err("check (3) refuses it first");
        assert!(
            verdict.findings.iter().all(|f| f.code == PLAN_BINDING),
            "{verdict:#?}"
        );
        let findings = refusal(
            joint(
                &d.sources,
                &d.program,
                &RT_STATIC_UP_V1,
                RECORD,
                &text,
                &plan,
            ),
            rule,
            false,
        );
        assert!(
            messages(&findings).contains(words),
            "{words}: {findings:#?}"
        );
    }
}

#[test]
fn rule_7_an_instance_s_device_is_of_the_system_s_hardware_and_the_instance_is_none() {
    const FAR: &str = "(adapter deadline-over-delay timer.far alarm)";
    let d = adapted("(channels 1 tick)", "", "", ALARMED);
    assert_eq!(d.bound(&alarm()), Ok(()));
    let mut binds = vec![(named("alarm"), "absolute-deadline", FAR)];
    binds.extend(beneath(FAR, "timer.far", &BENEATH_DEADLINE));
    let findings = refusal(d.bound(&binds), 7, true);
    // The binding to the instance, and each of its own bindings beneath.
    assert_eq!(findings.len(), 5, "{findings:#?}");
    let message = messages(&findings);
    assert!(
        message.contains(
            "`alarm` is bound for `absolute-deadline` to `timer.far`, the device \
             `(adapter deadline-over-delay timer.far alarm)` programs, outside the hardware of `s`"
        ) && message.contains(&format!(
            "`{FAR}` is bound for `relative-delay` to `timer.far`, outside the hardware of `s`"
        )),
        "{message}"
    );
}

#[test]
fn rule_9_an_instance_is_a_caller_at_its_kind_s_level_and_states_it_for_its_callers() {
    const LOW: &str = "(adapter deadline-over-delay timer.u alarm)";
    let d = adapted("(channels 1 tick)", "", "", ALARMED);
    let mut binds = vec![(named("alarm"), "absolute-deadline", LOW)];
    binds.extend(beneath(LOW, "timer.u", &BENEATH_DEADLINE));
    // Its device, reachable from `user` alone, is judged at the kind's level — whatever level the profile gives.
    for profile in [RT_STATIC_UP_V1, at_level("")] {
        let findings = refusal(d.joint_under(&profile, &d.plan(None, &binds)), 9, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        let message = &findings[0].message;
        assert!(
            message.contains("`timer.u`, bound for")
                && message.contains(&format!("`{LOW}` for `relative-delay`"))
                && message.contains("is reachable from `user` alone")
                && message.contains("its kind's level, `machine`"),
            "{message}"
        );
    }
    // A caller bound to an instance reads the instance's level: `machine`, never a profile's `supervisor`.
    assert_eq!(d.bound(&alarm()), Ok(()));
    let findings = refusal(
        d.joint_under(&at_level("supervisor"), &d.plan(None, &alarm())),
        9,
        true,
    );
    assert_eq!(findings.len(), 1, "{findings:#?}");
    assert!(
        findings[0].message.contains(&format!(
            "`{ALARM}`, bound for `alarm` for `absolute-deadline`"
        )) && findings[0]
            .message
            .contains("is reachable from `machine` alone")
            && findings[0]
                .message
                .contains("runs every caller at `supervisor`"),
        "{findings:#?}"
    );
}

#[test]
fn rule_5_an_instance_owns_no_region_and_its_device_bound_beneath_owns_its_own() {
    let time = "(defservice time.now (requires (tick-rate 1 MHz)))";
    let system = "(defsystem s (requires (uses alarm time.now)) (platform (uses soc)))";
    let mut binds = alarm();
    binds.push((named("time.now"), "tick-rate", "timer.e"));
    let d = adapted(
        "(channels 1 tick) (region regs (base 0x1000_0000) (size 4 KiB))",
        "(region regs (base 0x2000_0000) (size 4 KiB))",
        time,
        system,
    );
    assert_eq!(d.bound(&binds), Ok(()));
    let d = adapted(
        "(channels 1 tick) (region regs (base 0x1000_0000) (size 4 KiB))",
        "(region regs (base 0x1000_0800) (size 4 KiB))",
        time,
        system,
    );
    let findings = refusal(d.bound(&binds), 5, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains("`timer.d` and `timer.e`, both bound in this assignment")
            && !message.contains("(adapter"),
        "{message}"
    );
}

#[test]
fn rule_13_an_instance_is_one_provider_of_its_function() {
    let d = monotonic("(tick-rate 1 MHz)");
    let mut whole = vec![
        (named("time.monotonic"), "tick-rate", EXTENDED),
        (named("time.monotonic"), "unambiguous-horizon", EXTENDED),
    ];
    whole.extend(beneath(EXTENDED, "timer.short", &BENEATH_EXTENDER));
    // The rate passed through from the counter, the horizon derived at the instance: one provider meets both.
    assert_eq!(d.bound(&whole), Ok(()));
    let mut split = whole.clone();
    split[0] = (named("time.monotonic"), "tick-rate", "timer.short");
    let findings = refusal(d.bound(&split), 13, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains("`counter` function to 2 providers")
            && message.contains(&format!("`{EXTENDED}` for `unambiguous-horizon`"))
            && message.contains("`timer.short` for `tick-rate`")
            && message.contains(&format!("`{EXTENDED}` meets every one")),
        "{message}"
    );
}

/// A deadline service reading time through `time.now`, and one reading its own rate, over a `timer.d` of `channels`.
fn deadline_and_clock(channels: &str) -> Description {
    adapted(
        channels,
        "",
        "(defservice time.now (requires (tick-rate 1 MHz) (counter-modulus 4294967296)))
         (defservice alarm.now (requires (absolute-deadline (supported-horizon (at-least 10 s))) (uses time.now)))
         (defservice alarm.self (requires (absolute-deadline (supported-horizon (at-least 10 s))) (tick-rate 1 MHz)))",
        "(defsystem s (requires (uses alarm.now alarm.self)) (platform (uses soc)))",
    )
}

/// `alarm.now`'s deadline through its translator over `timer.d`.
const NOW: &str = "(adapter deadline-over-delay timer.d alarm.now)";

/// The deadline of `alarm.now`, bound to `deadline`, and the counter facts `time.now` reads, bound to `clock`; with
/// `alarm.self`'s deadline and its own rate at the comparator `timer.abs`.
fn time_base(deadline: &'static str, clock: &'static str) -> Vec<Bind> {
    let mut binds = vec![
        (named("alarm.now"), "absolute-deadline", deadline),
        (named("time.now"), "counter-modulus", clock),
        (named("time.now"), "tick-rate", clock),
        (named("alarm.self"), "absolute-deadline", "timer.abs"),
        (named("alarm.self"), "tick-rate", "timer.abs"),
    ];
    if deadline.starts_with('(') {
        binds.extend(beneath(deadline, "timer.d", &BENEATH_DEADLINE));
    }
    binds
}

#[test]
fn rule_14_a_deadline_and_the_counter_its_requester_reaches_are_bound_at_one_device() {
    // Through a translator, its device the timer the clock reads.
    let d = deadline_and_clock("(channels 1 tick)");
    assert_eq!(d.bound(&time_base(NOW, "timer.d")), Ok(()));
    let plan = d.plan(None, &time_base(NOW, "timer.e"));
    let findings = refusal(d.joint(&plan), 14, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains(&format!(
            "`alarm.now` is bound for `absolute-deadline` at the device `timer.d` through `{NOW}`, and `time.now`"
        )) && message.contains("is bound for `counter-modulus`, `tick-rate` at the device `timer.e`")
            && message.contains("in one counter's time and read in another's"),
        "{message}"
    );
    assert_eq!(
        findings[0].primary.line,
        line_of(&plan, &named("alarm.now"), "absolute-deadline")
    );
    // The other requester's binding, and both devices.
    assert_eq!(findings[0].secondary.len(), 3, "{findings:#?}");
}

#[test]
fn rule_14_judges_direct_bindings_too() {
    // A comparator programmed with absolute times itself, of two channels, and the clock reading it, or another.
    let d = adapted(
        "(channels 1 tick)",
        "",
        "(defservice time.now (requires (tick-rate 1 MHz) (counter-modulus 4294967296)))
         (defservice alarm.now (requires (absolute-deadline (supported-horizon (at-least 10 s))) (uses time.now)))",
        "(defsystem s (requires (uses alarm.now)) (platform (uses soc)))",
    );
    let direct = |clock: &'static str| -> Vec<Bind> {
        vec![
            (named("alarm.now"), "absolute-deadline", "timer.abs"),
            (named("time.now"), "counter-modulus", clock),
            (named("time.now"), "tick-rate", clock),
        ]
    };
    assert_eq!(d.bound(&direct("timer.abs")), Ok(()));
    let message = refused(d.bound(&direct("timer.e")), 14);
    assert!(
        message.contains("at the device `timer.abs`, and `time.now`")
            && message.contains("at the device `timer.e`"),
        "{message}"
    );
}

#[test]
fn rule_14_judges_every_other_requester_and_leaves_the_deadline_s_own_counter_to_rule_13() {
    const OWN: &str = "(adapter deadline-over-delay timer.d alarm.self)";
    let d = deadline_and_clock("(channels 2 tick)");
    // `alarm.self` reads its own rate at a device other than its deadline's: rule 13's unknown — describing what each
    // leaves undescribed makes one whole — never rule 14's conflict.
    let mut binds = time_base(NOW, "timer.d");
    binds.retain(|(r, _, _)| *r != named("alarm.self"));
    binds.push((named("alarm.self"), "absolute-deadline", OWN));
    binds.push((named("alarm.self"), "tick-rate", "timer.e"));
    binds.extend(beneath(OWN, "timer.d", &BENEATH_DEADLINE));
    let message = refused(d.bound(&binds), 13);
    assert!(
        message.contains("`alarm.self` binds the facts of its `counter` function to 2 providers")
            && message.contains("missing-fact")
            && !message.contains("(rule 14)"),
        "{message}"
    );
}

#[test]
fn rule_15_a_plan_naming_a_kind_the_checker_s_table_does_not_hold_is_refused() {
    let d = adapted("(channels 1 tick)", "", "", ALARMED);
    let plan = d.plan(None, &alarm());
    assert!(record(&d.program, &RT_STATIC_UP_V1, RECORD, &plan.to_canonical()).is_ok());
    // As the record reads it: check (1), a corrupted plan.
    let text = plan
        .to_canonical()
        .replace("deadline-over-delay", "deadline-multiplexer");
    let verdict = record(&d.program, &RT_STATIC_UP_V1, RECORD, &text).expect_err("no such kind");
    assert_eq!(verdict.status, Status::ToolFailure);
    assert_eq!(verdict.findings.len(), 1);
    assert_eq!(verdict.findings[0].code, PLAN_RECORD);
    assert!(
        verdict.findings[0]
            .message
            .contains("`deadline-multiplexer`")
            && verdict.findings[0].message.contains("does not hold"),
        "{verdict:#?}"
    );
    // Built past the reader, checks (3) and (5) refuse it by their own readings.
    let rename = |instance: &Instance| Instance {
        kind: "deadline-multiplexer".to_owned(),
        ..instance.clone()
    };
    let unknown = Assignment {
        bindings: plan
            .bindings
            .iter()
            .map(|((requester, fact), provider)| {
                let requester = match requester {
                    Requester::Instance(i) => Requester::Instance(rename(i)),
                    other => other.clone(),
                };
                let provider = match provider {
                    Provider::Instance(i) => Provider::Instance(rename(i)),
                    other => other.clone(),
                };
                ((requester, fact.clone()), provider)
            })
            .collect(),
        ..plan
    };
    let text = unknown.to_canonical();
    let verdict = bindings(&d.sources, &d.program, RECORD, &text, &unknown)
        .expect_err("check (3) refuses a kind it does not hold");
    assert!(
        verdict
            .findings
            .iter()
            .all(|f| f.code == PLAN_BINDING && f.message.contains("does not hold")),
        "{verdict:#?}"
    );
    let findings = refusal(
        joint(
            &d.sources,
            &d.program,
            &RT_STATIC_UP_V1,
            RECORD,
            &text,
            &unknown,
        ),
        1,
        false,
    );
    assert!(
        messages(&findings).contains("does not hold"),
        "{findings:#?}"
    );
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 16: a task released by its own source.

/// A source's clauses: it offers `interrupt-source` and the profile's level, and what `separation` writes inside its
/// `offers` — an offer of `min-arrival-separation`, or nothing.
fn offering(separation: &str) -> String {
    format!("(offers (interrupt-source true) (reachable-at-privilege machine) {separation})")
}

/// A source declaring its separation absent: an unbounded one.
const UNBOUNDED: &str =
    "(offers (interrupt-source true) (reachable-at-privilege machine)) (absent min-arrival-separation)";

/// The sporadic task `rx`, released at least 20 ms apart, bound itself to a source by `(needs interrupt-source)`.
const RX: &str =
    "(task rx (min-separation 20 ms) (deadline 20 ms) (deadline-from release) (priority 1) \
     (needs interrupt-source))";

/// Two sources of interrupts on the system's platform, `uart.rx` and `gpio.irq`, their clauses `uart` and `gpio`;
/// `services` declared beside them, and `tasks` the system's.
fn sourced(uart: &str, gpio: &str, services: &str, tasks: &str) -> Description {
    describe(&format!(
        "
      (defblock uart.rx {uart})
      (defblock gpio.irq {gpio})
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses uart.rx gpio.irq)))
      {services}
      (defsystem s
        {tasks}
        (platform (uses soc)))"
    ))
}

/// `rx` beside the source `uart.rx`, whose clauses are `uart`, and a `gpio.irq` stating 20 ms.
fn rx_beside(uart: &str) -> Description {
    sourced(uart, &offering("(min-arrival-separation 20 ms)"), "", RX)
}

/// `requester`'s own binding of `interrupt-source` to `source`.
fn own(requester: Requester, source: &'static str) -> [Bind; 1] {
    [(requester, "interrupt-source", source)]
}

/// The one finding of a plan rule 16 refuses, binding `rx` to `uart.rx`: at that binding in the record, naming the
/// task and the source, with where the task requires the fact, where it states its separation and where the source
/// is declared.
fn rx_refused(d: &Description) -> Finding {
    let rx = task("s", "rx");
    let plan = d.plan(None, &own(rx.clone(), "uart.rx"));
    let findings = refusal(d.joint(&plan), 16, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let finding = findings[0].clone();
    assert!(
        finding.message.contains("`(task s rx)`") && finding.message.contains("`uart.rx`"),
        "{finding:#?}"
    );
    assert_eq!(finding.primary.source, RECORD);
    assert_eq!(
        finding.primary.line,
        line_of(&plan, &rx, "interrupt-source")
    );
    assert_eq!(finding.secondary.len(), 3, "{finding:#?}");
    assert!(
        finding.secondary.iter().all(|l| l.source == SOURCE),
        "{finding:#?}"
    );
    finding
}

/// A conflict of rule 16's: `unsupported-profile`, never an unknown.
fn conflict(finding: &Finding, why: &str) {
    assert!(
        finding.message.contains(why)
            && finding
                .message
                .contains("a conflict, `unsupported-profile` (rule 16)")
            && !finding.message.contains("missing-fact"),
        "{why}: {finding:#?}"
    );
}

/// An unknown of rule 16's: `missing-fact`, blocking the assignment.
fn unknown(finding: &Finding, why: &str) {
    assert!(
        finding.message.contains(why)
            && finding
                .message
                .contains("`missing-fact`, which blocks the assignment (rule 16)")
            && finding.message.contains("`min-separation` of `20 ms`")
            && !finding.message.contains("unsupported-profile"),
        "{why}: {finding:#?}"
    );
}

#[test]
fn rule_16_a_task_s_own_source_as_far_apart_as_the_task_or_further_holds() {
    for separation in [
        "(min-arrival-separation 20 ms)",
        "(min-arrival-separation 50 ms)",
        "(min-arrival-separation 1 s)",
    ] {
        let d = rx_beside(&offering(separation));
        assert_eq!(
            d.bound(&own(task("s", "rx"), "uart.rx")),
            Ok(()),
            "{separation}"
        );
    }
}

#[test]
fn rule_16_a_task_s_own_source_shorter_than_the_task_is_a_conflict_under_condition_5() {
    let d = rx_beside(&offering("(min-arrival-separation 5 ms)"));
    conflict(
        &rx_refused(&d),
        "states a `min-arrival-separation` of `5 ms`, shorter than the task's `min-separation` of `20 ms`, and the \
         analysis admits a task released on every arrival of a source only where T ≤ T_s, its condition 5",
    );
}

#[test]
fn rule_16_a_task_s_own_source_declaring_its_separation_absent_is_a_conflict_under_condition_3() {
    let d = rx_beside(UNBOUNDED);
    conflict(
        &rx_refused(&d),
        "declares `min-arrival-separation` absent — an unbounded source, which the analysis refuses under its \
         condition 3",
    );
}

#[test]
fn rule_16_a_comparison_past_the_exact_arithmetic_is_a_conflict() {
    // 10^-31 ns is 10^-40 s: read exactly in its own unit, past the arithmetic in the base unit the task's `20 ms`
    // compares in — though plainly shorter, nothing decides it (§7 rule 2).
    let d = rx_beside(&offering(
        "(min-arrival-separation 0.0000000000000000000000000000001 ns)",
    ));
    conflict(
        &rx_refused(&d),
        "states a `min-arrival-separation` of `0.0000000000000000000000000000001 ns` whose comparison with the \
         task's `min-separation` of `20 ms` is past the exact arithmetic (§7 rule 2)",
    );
    // The same 10^-40 s in two spellings whose own comparison overflows: a separation past the arithmetic at the
    // source (§7 rule 3), which no comparison with the task's decides either.
    let d = rx_beside(&offering(
        "(min-arrival-separation 0.0000000000000000000000000000001 ns) \
         (min-arrival-separation 0.0000000000000000000000000000000001 us)",
    ));
    conflict(
        &rx_refused(&d),
        "states a `min-arrival-separation` whose comparison with the task's `min-separation` of `20 ms` is past the \
         exact arithmetic (§7 rule 2)",
    );
}

#[test]
fn rule_16_a_task_s_own_source_stating_no_value_is_an_unknown() {
    for (separation, why) in [
        ("", "states no `min-arrival-separation`"),
        (
            "min-arrival-separation",
            "offers `min-arrival-separation` without a value — bare",
        ),
        (
            "(min-arrival-separation)",
            "offers `min-arrival-separation` without a value — bare",
        ),
    ] {
        let d = rx_beside(&offering(separation));
        unknown(&rx_refused(&d), why);
    }
}

#[test]
fn rule_16_reads_exactly_as_a_value_and_a_bound_as_none() {
    // `(exactly v)` is the value `v`: as far apart as the task holds, shorter is the conflict.
    let d = rx_beside(&offering("(min-arrival-separation (exactly 20 ms))"));
    assert_eq!(d.bound(&own(task("s", "rx"), "uart.rx")), Ok(()));
    let d = rx_beside(&offering("(min-arrival-separation (exactly 5 ms))"));
    conflict(&rx_refused(&d), "of `5 ms`, shorter than");
    // A bound is no value, however far apart it writes: the unknown.
    for bound in [
        "(min-arrival-separation (at-least 20 ms))",
        "(min-arrival-separation (at-least 50 ms))",
    ] {
        let d = rx_beside(&offering(bound));
        unknown(
            &rx_refused(&d),
            "without a value — bare, or only a bound, which the relation reads as no value",
        );
    }
}

#[test]
fn rule_16_compares_a_separation_written_in_another_unit_exactly() {
    for separation in [
        "(min-arrival-separation 20000 us)",
        "(min-arrival-separation 20000000 ns)",
        "(min-arrival-separation 0.02 s)",
        "(min-arrival-separation 20001 us)",
    ] {
        let d = rx_beside(&offering(separation));
        assert_eq!(
            d.bound(&own(task("s", "rx"), "uart.rx")),
            Ok(()),
            "{separation}"
        );
    }
    let d = rx_beside(&offering("(min-arrival-separation 19999 us)"));
    conflict(
        &rx_refused(&d),
        "of `19999 us`, shorter than the task's `min-separation` of `20 ms`",
    );
}

#[test]
fn rule_16_leaves_a_periodic_task_bound_to_a_source_untouched() {
    for uart in [
        offering("(min-arrival-separation 5 ms)"),
        UNBOUNDED.to_owned(),
        offering(""),
        offering("(min-arrival-separation (at-least 50 ms))"),
    ] {
        let d = sourced(
            &uart,
            &offering(""),
            "",
            "(task beat (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 1) \
             (needs interrupt-source))",
        );
        assert_eq!(
            d.bound(&own(task("s", "beat"), "uart.rx")),
            Ok(()),
            "{uart}"
        );
    }
}

#[test]
fn rule_16_leaves_a_task_a_source_reaches_through_a_handler_untouched() {
    // The handler binds the source, never the task: only a task's own binding is the rule's.
    for uart in [
        offering("(min-arrival-separation 5 ms)"),
        UNBOUNDED.to_owned(),
        offering(""),
    ] {
        let d = sourced(
            &uart,
            &offering(""),
            "(defservice rx.handler (requires (needs interrupt-source)))",
            "(task rx (min-separation 20 ms) (deadline 20 ms) (deadline-from release) (priority 1) \
             (uses rx.handler))",
        );
        assert_eq!(
            d.bound(&own(named("rx.handler"), "uart.rx")),
            Ok(()),
            "{uart}"
        );
    }
}

#[test]
fn rule_16_judges_each_task_bound_to_one_source_on_its_own() {
    // `uart.rx`, 20 ms apart, is as far apart as `rx` and nearer than `burst`, 50 ms apart.
    let d = sourced(
        &offering("(min-arrival-separation 20 ms)"),
        &offering(""),
        "",
        &format!(
            "{RX}
        (task burst (min-separation 50 ms) (deadline 50 ms) (deadline-from release) (priority 2) \
             (needs interrupt-source))"
        ),
    );
    let burst = task("s", "burst");
    let plan = d.plan(
        None,
        &[
            (task("s", "rx"), "interrupt-source", "uart.rx"),
            (burst.clone(), "interrupt-source", "uart.rx"),
        ],
    );
    let findings = refusal(d.joint(&plan), 16, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains("`(task s burst)`")
            && !message.contains("`(task s rx)`")
            && message.contains("of `20 ms`, shorter than the task's `min-separation` of `50 ms`"),
        "{message}"
    );
    assert_eq!(
        findings[0].primary.line,
        line_of(&plan, &burst, "interrupt-source")
    );
}

#[test]
fn rule_16_judges_the_binding_so_a_source_far_enough_apart_holds_where_a_faster_one_is_refused() {
    // The description offers both: the plan binding `gpio.irq`, 20 ms apart, is the one the search writes.
    let d = sourced(
        &offering("(min-arrival-separation 5 ms)"),
        &offering("(min-arrival-separation 20 ms)"),
        "",
        RX,
    );
    assert_eq!(d.bound(&own(task("s", "rx"), "gpio.irq")), Ok(()));
    conflict(&rx_refused(&d), "of `5 ms`, shorter than");
}
