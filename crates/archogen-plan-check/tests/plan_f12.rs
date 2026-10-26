//! F12 (`ROADMAP.md` §14; leaf `M3.5.4.5`): a corrupted plan assignment or a changed requirement is rejected — every
//! check run in order over one plan, as the executable runs them ([`plan`]). Each corruption a test, with the status
//! and code it is refused under; the plan accepted beside them.

use std::collections::BTreeMap;

use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::{
    plan, Verdict, PLAN_BINDING, PLAN_DESCRIPTION, PLAN_RECORD, PLAN_STALE, PLAN_UNSATISFIED,
};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// A system whose task needs a timer's channel and uses a monotonic time service and a console, on a platform with a
/// fast counter and a slow one.
const DESCRIPTION: &str = "\
(defblock timer
  (offers (relative-delay true) (channels 2) (counter-width 32 bit) (tick-rate 1 MHz) (counter-modulus (pow2 32))
          observation-coherent (reachable-at-privilege machine)))
(defblock slow (offers (counter-width 16 bit) (tick-rate 1 MHz) (counter-modulus (pow2 16)) observation-coherent
                       (reachable-at-privilege machine)))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer slow console.uart)))
(defservice time.monotonic
  (requires (counter-width (at-least 32 bit)) (needs tick-rate observation-coherent)))
(defservice console.write (requires (needs observable-output)))
(defservice stray (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1)
        (uses time.monotonic console.write) (needs relative-delay))
  (requires (uses time.monotonic console.write))
  (platform (uses soc)))
";

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

fn named(name: &str) -> Requester {
    Requester::Named(name.to_owned())
}

/// The plan a search would write for [`DESCRIPTION`], or for a description that varies it.
fn written(program: &Program) -> Assignment {
    let mut bindings = BTreeMap::new();
    let beat = Requester::Task {
        system: "s".to_owned(),
        task: "beat".to_owned(),
    };
    for (requester, fact, provider) in [
        (beat, "relative-delay", "timer"),
        (named("time.monotonic"), "counter-width", "timer"),
        (named("time.monotonic"), "tick-rate", "timer"),
        (named("time.monotonic"), "observation-coherent", "timer"),
        (named("console.write"), "observable-output", "console.uart"),
    ] {
        bindings.insert((requester, fact.to_owned()), provider.to_owned());
    }
    Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "s".to_owned(),
        program: program_digest(program),
        placement: None,
        bindings,
    }
}

fn checked(description: &str, record: &str) -> Result<(), Verdict> {
    let (sources, program) = elaborated(description);
    plan(&sources, &program, &RT_STATIC_UP_V1, "plan.eadl", record)
}

/// The status a plan is refused with, and the code of each finding.
fn refused(description: &str, record: &str) -> (Status, Vec<String>) {
    let verdict = checked(description, record).unwrap_err();
    (
        verdict.status,
        verdict.findings.into_iter().map(|f| f.code).collect(),
    )
}

fn plan_of(description: &str) -> Assignment {
    written(&elaborated(description).1)
}

#[test]
fn the_plan_a_search_writes_is_accepted() {
    assert_eq!(
        checked(DESCRIPTION, &plan_of(DESCRIPTION).to_canonical()),
        Ok(())
    );
}

#[test]
fn a_binding_moved_to_a_provider_that_refuses_it() {
    let mut moved = plan_of(DESCRIPTION);
    moved.bindings.insert(
        (named("time.monotonic"), "counter-width".to_owned()),
        "slow".to_owned(),
    );
    assert_eq!(
        refused(DESCRIPTION, &moved.to_canonical()),
        (Status::ToolFailure, vec![PLAN_UNSATISFIED.to_owned()])
    );
}

#[test]
fn a_requirement_left_unbound() {
    let mut unbound = plan_of(DESCRIPTION);
    unbound
        .bindings
        .remove(&(named("console.write"), "observable-output".to_owned()));
    assert_eq!(
        refused(DESCRIPTION, &unbound.to_canonical()),
        (Status::ToolFailure, vec![PLAN_BINDING.to_owned()])
    );
}

#[test]
fn a_binding_twice() {
    let text = plan_of(DESCRIPTION).to_canonical();
    let line = text
        .lines()
        .find(|l| l.contains("observable-output"))
        .unwrap();
    for twice in [
        text.replace(line, &format!("{line}\n{line}")),
        text.replace(
            line,
            &format!("{line}\n{}", line.replace("console.uart", "timer")),
        ),
    ] {
        assert_eq!(
            refused(DESCRIPTION, &twice),
            (Status::ToolFailure, vec![PLAN_RECORD.to_owned()])
        );
    }
}

#[test]
fn a_binding_of_a_requester_the_system_does_not_serve_or_of_no_requirement() {
    for (requester, fact) in [
        (named("stray"), "observable-output"),
        (named("console.write"), "counter-width"),
    ] {
        let mut extra = plan_of(DESCRIPTION);
        extra
            .bindings
            .insert((requester, fact.to_owned()), "console.uart".to_owned());
        assert_eq!(
            refused(DESCRIPTION, &extra.to_canonical()),
            (Status::ToolFailure, vec![PLAN_BINDING.to_owned()])
        );
    }
}

#[test]
fn a_provider_the_description_does_not_declare() {
    let mut moved = plan_of(DESCRIPTION);
    moved.bindings.insert(
        (named("console.write"), "observable-output".to_owned()),
        "console.usb".to_owned(),
    );
    assert_eq!(
        refused(DESCRIPTION, &moved.to_canonical()),
        (Status::ToolFailure, vec![PLAN_BINDING.to_owned()])
    );
}

#[test]
fn a_requirement_changed_after_its_plan_is_stale_and_changed_with_the_digest_recomputed_is_re_judged(
) {
    let raised = DESCRIPTION.replace(
        "(counter-width (at-least 32 bit))",
        "(counter-width (at-least 48 bit))",
    );
    // The plan written before the change: stale, before any judgement.
    assert_eq!(
        refused(&raised, &plan_of(DESCRIPTION).to_canonical()),
        (Status::StaleInput, vec![PLAN_STALE.to_owned()])
    );
    // The same bindings with the digest recomputed: the checker re-reads the requirement rather than trusting the hash.
    assert_eq!(
        refused(&raised, &plan_of(&raised).to_canonical()),
        (Status::ToolFailure, vec![PLAN_UNSATISFIED.to_owned()])
    );
}

#[test]
fn a_description_the_toolchain_refuses_has_no_plan_accepted() {
    let no_channel = DESCRIPTION.replace("(channels 2)", "(channels 0)");
    assert_eq!(
        refused(&no_channel, &plan_of(&no_channel).to_canonical()),
        (
            Status::InvalidDescription,
            vec![PLAN_DESCRIPTION.to_owned()]
        )
    );
}
