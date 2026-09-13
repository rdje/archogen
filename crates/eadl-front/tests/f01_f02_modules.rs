//! **F01** — valid sub-HW and sub-OS imports compose.
//! **F02** — circular imports or conflicting exports produce a precise composition error.
//!
//! `ROADMAP.md` §13.1, first gate M1.
//!
//! "Precise" is what these arms are about. "There is a cycle" is a puzzle; `a → b → c → a` is a
//! diagnostic. Likewise a conflicting export has to name *both* sites, because the author
//! looking at one of them cannot see the other.

use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::SourceMap;

/// Elaborate `root` and return `(program, rendered diagnostics, had errors)`.
fn run(modules: &MemoryModules, root: &str) -> (eadl_front::Program, String, bool) {
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, modules, root);
    let rendered = diagnostics.render(&sources);
    let failed = diagnostics.has_errors();
    (program, rendered, failed)
}

/// A small sub-hardware and sub-OS library, the shape §5.1.1 describes.
fn library() -> MemoryModules {
    MemoryModules::new()
        .with(
            "hw.timer",
            r"(defmodule hw.timer
               (version 1 0)
               (param tick-rate (default (tick-rate 10 MHz)))
               (export timer.counter)
               (defblock timer.counter
                 (offers (counter-width 32 bit))))",
        )
        .with(
            "hw.soc",
            r"(defmodule hw.soc
               (version 1 2)
               (import hw.timer (as timer) (version (at-least 1 0)))
               (export soc.bus)
               (defplatform soc.bus
                 (offers (region device.timer (base 0x1000_0000)))))",
        )
        .with(
            "os.time",
            r"(defmodule os.time
               (version 2 0)
               (export time.monotonic)
               (defservice time.monotonic
                 (requires (unambiguous-horizon (at-least 60 s)))))",
        )
        .with(
            "app.system",
            r"(defmodule app.system
               (version 1 0)
               (import hw.soc (as platform) (version (at-least 1 1)))
               (import os.time (as clock) (version (at-least 2 0)))
               (export app.rt)
               (defsystem app.rt
                 (requires (uses clock.time.monotonic))))",
        )
}

#[test]
fn f01_sub_hardware_and_sub_os_modules_compose() {
    let (program, rendered, failed) = run(&library(), "app.system");
    assert!(!failed, "a valid composition was refused:\n{rendered}");

    // Four instances: hw.timer, hw.soc, os.time, app.system.
    assert_eq!(program.len(), 4, "expected four instances");

    // ⭐ Children before parents: an instance's imports always have smaller ids than it does.
    // That ordering is what later passes rely on for initialization and resolution order.
    for instance in &program.instances {
        for import in &instance.imports {
            assert!(
                *import < instance.id,
                "instance {} imports {import}, which is not elaborated before it",
                instance.id
            );
        }
    }

    assert_eq!(program.root().module, "app.system");
    assert!(program.root().path.is_empty(), "the root has no alias path");
}

#[test]
fn f01_names_are_namespaced_by_the_alias_path() {
    let (program, _, failed) = run(&library(), "app.system");
    assert!(!failed);
    let names: Vec<String> = program
        .qualified_declarations()
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    assert!(
        names.contains(&"platform.timer.timer.counter".to_string()),
        "a nested import must carry the whole alias path: {names:?}"
    );
    assert!(names.contains(&"platform.soc.bus".to_string()), "{names:?}");
    assert!(
        names.contains(&"clock.time.monotonic".to_string()),
        "{names:?}"
    );
    assert!(
        names.contains(&"app.rt".to_string()),
        "the root is unqualified: {names:?}"
    );
}

#[test]
fn f01_a_module_instantiated_twice_gets_two_independent_instances() {
    // ⭐ §5.1.1: "Instantiate a module more than once without sharing mutable elaboration
    // state." A design that cached one elaborated module per name would make the second import
    // a no-op, and a system with two timers would have one.
    let modules = library().with(
        "app.two-timers",
        r"(defmodule app.two-timers
           (version 1 0)
           (import hw.timer (as fast) (with (tick-rate (tick-rate 100 MHz))))
           (import hw.timer (as slow) (with (tick-rate (tick-rate 1 MHz))))
           (export app.rt)
           (defsystem app.rt
             (requires (uses fast.timer.counter) (uses slow.timer.counter))))",
    );
    let (program, rendered, failed) = run(&modules, "app.two-timers");
    assert!(!failed, "two instantiations were refused:\n{rendered}");

    let timers: Vec<&eadl_front::Instance> = program
        .instances
        .iter()
        .filter(|i| i.module == "hw.timer")
        .collect();
    assert_eq!(
        timers.len(),
        2,
        "the second instantiation collapsed into the first"
    );

    let paths: Vec<&str> = timers.iter().map(|i| i.path.as_str()).collect();
    assert!(
        paths.contains(&"fast") && paths.contains(&"slow"),
        "{paths:?}"
    );

    // Their bindings must differ, which is the whole point of instantiating twice.
    assert_ne!(
        timers[0].bindings, timers[1].bindings,
        "both instances received the same binding — the argument was not applied per instance"
    );
}

#[test]
fn f01_a_parameter_default_applies_when_no_argument_is_given() {
    let (program, _, failed) = run(&library(), "app.system");
    assert!(!failed);
    let timer = program
        .instances
        .iter()
        .find(|i| i.module == "hw.timer")
        .expect("the timer is instantiated");
    assert_eq!(timer.bindings.len(), 1);
    assert_eq!(timer.bindings[0].0, "tick-rate");
}

#[test]
fn f02_a_circular_import_reports_the_whole_chain() {
    // "There is a cycle" is a puzzle; `a → b → c → a` is a diagnostic.
    let modules = MemoryModules::new()
        .with(
            "a",
            "(defmodule a (version 1 0) (import b) (export x) (defblock x (offers (p 1 bit))))",
        )
        .with(
            "b",
            "(defmodule b (version 1 0) (import c) (export y) (defblock y (offers (p 1 bit))))",
        )
        .with(
            "c",
            "(defmodule c (version 1 0) (import a) (export z) (defblock z (offers (p 1 bit))))",
        );
    let (_, rendered, failed) = run(&modules, "a");
    assert!(failed, "a cycle was not refused");
    assert!(rendered.contains("module-circular-import"), "{rendered}");
    assert!(
        rendered.contains("a → b → c → a"),
        "the whole chain must be named, not just the fact of a cycle:\n{rendered}"
    );
    assert!(rendered.contains("break the cycle"), "{rendered}");
}

#[test]
fn f02_a_self_import_is_a_cycle_of_one() {
    let modules = MemoryModules::new().with(
        "solo",
        "(defmodule solo (version 1 0) (import solo) (export x) (defblock x (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "solo");
    assert!(failed);
    assert!(rendered.contains("solo → solo"), "{rendered}");
}

#[test]
fn f02_a_duplicated_export_names_both_sites() {
    let modules = MemoryModules::new().with(
        "dup",
        r"(defmodule dup
           (version 1 0)
           (export thing)
           (export thing)
           (defblock thing (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "dup");
    assert!(failed, "a duplicated export was accepted");
    assert!(rendered.contains("module-conflicting-export"), "{rendered}");
    assert!(
        rendered.contains("first exported here"),
        "the author looking at one site cannot see the other:\n{rendered}"
    );
    assert!(rendered.contains("two answers and no rule"), "{rendered}");
}

#[test]
fn f02_two_imports_under_one_alias_are_refused() {
    let modules = library().with(
        "clash",
        r"(defmodule clash
           (version 1 0)
           (import hw.timer (as t))
           (import os.time (as t))
           (export app.rt)
           (defsystem app.rt (requires (uses t.timer.counter))))",
    );
    let (_, rendered, failed) = run(&modules, "clash");
    assert!(failed, "an alias clash was accepted");
    assert!(rendered.contains("module-conflicting-alias"), "{rendered}");
    assert!(rendered.contains("first bound here"), "{rendered}");
    assert!(
        rendered.contains("every qualified name ambiguous"),
        "the refusal should say what breaks:\n{rendered}"
    );
}

#[test]
fn f02_an_export_of_something_undeclared_is_refused() {
    let modules = MemoryModules::new().with(
        "promise",
        r"(defmodule promise
           (version 1 0)
           (export absent)
           (defblock present (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "promise");
    assert!(failed, "an export naming nothing was accepted");
    assert!(rendered.contains("module-dangling-export"), "{rendered}");
    assert!(
        rendered.contains("this module declares `present`"),
        "{rendered}"
    );
}

#[test]
fn an_incompatible_minor_version_is_refused_with_both_numbers() {
    let modules = library().with(
        "needs-newer",
        r"(defmodule needs-newer
           (version 1 0)
           (import hw.soc (as p) (version (at-least 1 9)))
           (export app.rt)
           (defsystem app.rt (requires (uses p.soc.bus))))",
    );
    let (_, rendered, failed) = run(&modules, "needs-newer");
    assert!(failed);
    assert!(
        rendered.contains("module-incompatible-version"),
        "{rendered}"
    );
    assert!(rendered.contains("at least minor 9"), "{rendered}");
    assert!(rendered.contains("offers 2"), "{rendered}");
}

#[test]
fn a_major_version_difference_is_never_satisfied_by_a_newer_minor() {
    // §15: a locked description keeps its meaning, and a major bump is defined not to preserve
    // it. "Newer" is not "compatible".
    let modules = library().with(
        "wrong-major",
        r"(defmodule wrong-major
           (version 1 0)
           (import os.time (as c) (version (at-least 1 0)))
           (export app.rt)
           (defsystem app.rt (requires (uses c.time.monotonic))))",
    );
    let (_, rendered, failed) = run(&modules, "wrong-major");
    assert!(
        failed,
        "os.time is 2.0 and cannot satisfy a 1.x requirement"
    );
    assert!(rendered.contains("major versions differ"), "{rendered}");
}

#[test]
fn an_unversioned_module_is_refused() {
    let modules = MemoryModules::new().with(
        "bare",
        "(defmodule bare (export x) (defblock x (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "bare");
    assert!(failed);
    assert!(rendered.contains("module-missing-version"), "{rendered}");
}

#[test]
fn a_module_whose_declared_name_differs_is_refused() {
    let modules = MemoryModules::new().with(
        "expected.name",
        "(defmodule other.name (version 1 0) (export x) (defblock x (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "expected.name");
    assert!(failed);
    assert!(rendered.contains("module-name-mismatch"), "{rendered}");
    assert!(rendered.contains("which module it resolved"), "{rendered}");
}

#[test]
fn a_missing_module_is_reported_at_the_import_site() {
    let modules = MemoryModules::new().with(
        "root",
        "(defmodule root (version 1 0) (import absent.module (as a)) (export x) (defblock x (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "root");
    assert!(failed);
    assert!(rendered.contains("module-not-found"), "{rendered}");
    assert!(rendered.contains("imported here"), "{rendered}");
}

#[test]
fn a_required_parameter_with_no_default_must_be_supplied() {
    let modules = MemoryModules::new()
        .with(
            "needy",
            "(defmodule needy (version 1 0) (param size) (export x) (defblock x (offers (p 1 bit))))",
        )
        .with(
            "user",
            "(defmodule user (version 1 0) (import needy (as n)) (export y) (defblock y (offers (p 1 bit))))",
        );
    let (_, rendered, failed) = run(&modules, "user");
    assert!(failed);
    assert!(rendered.contains("module-missing-argument"), "{rendered}");
    assert!(
        rendered.contains("declared here, with no default"),
        "{rendered}"
    );
}

#[test]
fn an_unknown_parameter_lists_the_real_ones() {
    let modules = library().with(
        "typo",
        r"(defmodule typo
           (version 1 0)
           (import hw.timer (as t) (with (tickrate (tick-rate 1 MHz))))
           (export x)
           (defblock x (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "typo");
    assert!(failed);
    assert!(rendered.contains("module-unknown-parameter"), "{rendered}");
    assert!(
        rendered.contains("its parameters are `tick-rate`"),
        "{rendered}"
    );
}

#[test]
fn composition_errors_are_collected_not_thrown_one_at_a_time() {
    // Three problems should cost one edit cycle.
    let modules = MemoryModules::new().with(
        "messy",
        r"(defmodule messy
           (version 1 0)
           (export a)
           (export a)
           (export missing)
           (import nowhere (as n))
           (defblock a (offers (p 1 bit))))",
    );
    let mut sources = SourceMap::new();
    let (_, diagnostics) = elaborate(&mut sources, &modules, "messy");
    assert!(
        diagnostics.len() >= 3,
        "only {} diagnostic(s) for three distinct problems:\n{}",
        diagnostics.len(),
        diagnostics.render(&sources)
    );
}

#[test]
fn a_module_file_holding_two_forms_is_refused() {
    let modules = MemoryModules::new().with(
        "two",
        "(defmodule two (version 1 0) (export x) (defblock x (offers (p 1 bit))))\n(defblock y (offers (p 1 bit)))",
    );
    let (_, rendered, failed) = run(&modules, "two");
    assert!(failed);
    assert!(rendered.contains("module-multiple-forms"), "{rendered}");
}
