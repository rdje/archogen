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
