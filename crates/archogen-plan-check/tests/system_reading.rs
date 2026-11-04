//! The system reading check (5) makes public, [`archogen_plan_check::joint::system`]: one system's hardware, the
//! declarations its closure reaches and the requesters it serves (`docs/semantics/model.md` §8 rules 1 and 7). Each
//! expected value is derived from those rules and the joint module's reading of them, never from a run.

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::{joint, PLAN_JOINT};
use archogen_plan_check::joint::{system, SystemReading};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// `modules` elaborated from the root module `root`, which must elaborate.
fn elaborated(modules: &MemoryModules, root: &str) -> (SourceMap, Program) {
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, modules, root);
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    (sources, program)
}

/// A description whose declarations are the root module `app`'s, so their qualified names are their own.
fn describe(declarations: &str) -> (SourceMap, Program) {
    let modules = MemoryModules::new().with(
        "app",
        &format!("(defmodule app\n  (version 1 0)\n{declarations})"),
    );
    elaborated(&modules, "app")
}

fn names(names: &[&str]) -> BTreeSet<String> {
    names.iter().map(|n| (*n).to_owned()).collect()
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

#[test]
fn a_platform_and_the_blocks_it_composes_at_any_depth_are_the_system_s_hardware() {
    // Rule 7: the platform clause names `soc`; `soc` names `console.uart` inside `requires`, which names `timer.counter`
    // in a clause of its own — every block or platform those blocks and platforms name in turn through `uses` or
    // `needs`, a clause of their own or inside `requires`, at any depth. `spare.uart` nothing names.
    let (_, program) = describe(
        "
  (defblock timer.counter (offers (counter-width 32 bit)))
  (defblock console.uart (offers (observable-output true)) (uses timer.counter))
  (defblock spare.uart (offers (observable-output true)))
  (defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart)))
  (defsystem s (platform (uses soc)))",
    );
    let hardware = names(&["soc", "console.uart", "timer.counter"]);
    assert_eq!(
        system(&program, "s"),
        Some(SystemReading {
            hardware: hardware.clone(),
            reached: hardware,
            served: [
                named("s"),
                named("soc"),
                named("console.uart"),
                named("timer.counter"),
            ]
            .into_iter()
            .collect(),
        })
    );
}

#[test]
fn a_service_naming_a_block_does_not_make_it_hardware() {
    // Rule 7: a declaration of another kind ends the hardware's path — a block that only a service names is not
    // hardware — so the block the service `console.write` uses is reached and served, but no hardware.
    let (_, program) = describe(
        "
  (defblock console.uart (offers (observable-output true)))
  (defplatform soc (offers (core-count 1 tick)))
  (defservice console.write (requires (needs observable-output)) (uses console.uart))
  (defsystem s (requires (uses console.write)) (platform (uses soc)))",
    );
    let reading = system(&program, "s").expect("`s` is a system");
    assert_eq!(reading.hardware, names(&["soc"]));
    assert_eq!(
        reading.reached,
        names(&["console.write", "console.uart", "soc"])
    );
    assert_eq!(
        reading.served,
        [
            named("s"),
            named("console.write"),
            named("console.uart"),
            named("soc"),
        ]
        .into_iter()
        .collect()
    );
}

#[test]
fn the_system_serves_its_own_level_its_tasks_and_what_its_tasks_reach() {
    // Rules 1 and 7: each task is a requester of its own; what a task names through `uses` and `needs` is the system's
    // closure too, and a block a task needs is hardware. A fact a requirement names (`observable-output`) is no
    // declaration, and reaches nothing.
    let (_, program) = describe(
        "
  (defblock console.uart (offers (observable-output true)))
  (defplatform soc (offers (core-count 1 tick)))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (task log (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs console.uart))
    (platform (uses soc)))",
    );
    let reading = system(&program, "s").expect("`s` is a system");
    assert_eq!(reading.hardware, names(&["soc", "console.uart"]));
    assert_eq!(
        reading.reached,
        names(&["console.write", "console.uart", "soc"])
    );
    assert_eq!(
        reading.served,
        [
            named("s"),
            task("s", "beat"),
            task("s", "log"),
            named("console.write"),
            named("console.uart"),
            named("soc"),
        ]
        .into_iter()
        .collect()
    );
}

#[test]
fn an_imported_platform_s_hardware_is_named_by_its_qualified_names() {
    // The module `hw.board`, imported as `hw`: its declarations are qualified `hw.NAME`, and `soc`'s `uses` resolves
    // in its own instance, to `hw.console.uart`.
    let modules = MemoryModules::new()
        .with(
            "hw.board",
            "(defmodule hw.board (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true)))
  (defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart))))",
        )
        .with(
            "app",
            "(defmodule app (version 1 0) (import hw.board (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (platform (uses hw.soc))))",
        );
    let (_, program) = elaborated(&modules, "app");
    let reading = system(&program, "s").expect("`s` is a system");
    assert_eq!(reading.hardware, names(&["hw.soc", "hw.console.uart"]));
    assert_eq!(
        reading.reached,
        names(&["console.write", "hw.soc", "hw.console.uart"])
    );
    assert_eq!(
        reading.served,
        [
            named("s"),
            task("s", "beat"),
            named("console.write"),
            named("hw.soc"),
            named("hw.console.uart"),
        ]
        .into_iter()
        .collect()
    );
    // A local name is no qualified one: the imported platform is `hw.soc`, never `soc`.
    assert!(!reading.hardware.contains("soc"));
}

#[test]
fn a_name_that_is_no_system_reads_as_none() {
    let (_, program) = describe(
        "
  (defblock console.uart (offers (observable-output true)))
  (defplatform soc (offers (core-count 1 tick)))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (platform (uses soc)))",
    );
    assert!(system(&program, "s").is_some());
    // A block, a platform, a service, a task by its name, and a name nothing declares.
    for name in [
        "console.uart",
        "soc",
        "console.write",
        "beat",
        "s/beat",
        "nothing",
    ] {
        assert_eq!(system(&program, name), None, "`{name}` is no system");
    }
}

#[test]
fn check_5_judges_a_binding_against_the_hardware_the_reading_gives() {
    // Rule 7: a requester the system serves bound to a provider of its hardware holds; bound to a block outside it, one
    // nothing names — a service naming it would refuse the system whatever the binding — it is a conflict naming the
    // requester, the provider and the system.
    let (sources, program) = describe(
        "
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (observable-output true) (reachable-at-privilege machine)))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s (requires (uses console.write)) (platform (uses soc)))",
    );
    let reading = system(&program, "s").expect("`s` is a system");
    assert!(reading.served.contains(&named("console.write")));
    let judged = |provider: &str| {
        let assignment = Assignment {
            profile: RT_STATIC_UP_V1.id.to_owned(),
            system: "s".to_owned(),
            program: program_digest(&program),
            placement: None,
            bindings: BTreeMap::from([(
                (named("console.write"), "observable-output".to_owned()),
                provider.into(),
            )]),
        };
        let text = assignment.to_canonical();
        joint(
            &sources,
            &program,
            &RT_STATIC_UP_V1,
            "plan.eadl",
            &text,
            &assignment,
        )
    };
    assert!(reading.hardware.contains("soc"));
    assert_eq!(judged("soc"), Ok(()));
    assert!(!reading.hardware.contains("console.uart"));
    let verdict = judged("console.uart").expect_err("outside the hardware");
    assert_eq!(verdict.status, Status::ToolFailure, "{verdict:#?}");
    assert!(
        verdict.findings.iter().any(|f| f.code == PLAN_JOINT
            && f.message.contains("outside the hardware of `s`")
            && f.message.contains("(rule 7)")),
        "{verdict:#?}"
    );
}
