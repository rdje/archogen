//! **F01** — valid sub-HW and sub-OS imports compose.
//! **F02** — circular imports or conflicting exports produce a precise composition error.
//!
//! `ROADMAP.md` §13.1, first gate M1.
//!
//! "Precise" is what these arms are about. "There is a cycle" is a puzzle; `a → b → c → a` is a
//! diagnostic. Likewise a conflicting export has to name *both* sites, because the author
//! looking at one of them cannot see the other.

use std::path::Path;

use eadl_front::module::{
    elaborate, elaborate_source, is_module_name, DirectoryModules, MemoryModules, ModuleSource,
};
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

#[test]
fn a_module_may_state_its_language_version_before_its_declaration() {
    // §8 rule 7, and the interaction that made it non-optional: §6 requires a module file to hold
    // exactly one top-level form, so without the filter in `single_module` a module that states its
    // language version would be refused as `module-multiple-forms`. That would leave the one file kind
    // which most needs a locked version — the unit of reuse, imported by name into other descriptions
    // — the one kind unable to carry it.
    //
    // ⛔ Note the two `version`s in this fixture are different things and that is the point:
    // `(eadl-version eadl/1)` is the *language* version (§8), `(version 1 0)` is the *module's* own
    // version (§6 rule 3, which an import matches with `(at-least …)`). One head symbol for both would
    // have made this file ambiguous by nesting depth alone.
    let modules = MemoryModules::new().with(
        "platform.timer",
        "(eadl-version eadl/1)\n(defmodule platform.timer (version 1 0) (export x) (defblock x (offers (p 1 bit))))",
    );
    let (_, rendered, failed) = run(&modules, "platform.timer");
    assert!(
        !failed,
        "a leading language-version identifier was refused:\n{rendered}"
    );
    assert!(
        !rendered.contains("module-multiple-forms"),
        "the identifier was counted as a stray top-level form:\n{rendered}"
    );
}

#[test]
fn a_stray_form_beside_a_version_identifier_is_still_reported_against_the_stray_form() {
    // ⛔ The filter must not weaken §6. Skipping the identifier means the count is over *declarations*,
    // so a genuinely stray third form still has to be refused — and the label has to point at the stray
    // form. `single_module` used to label `document.forms[1]`, which with an identifier present is the
    // `(defmodule …)` itself: the diagnostic would have told the author to delete their declaration
    // and keep the stray form.
    let modules = MemoryModules::new().with(
        "stray",
        "(eadl-version eadl/1)\n(defmodule stray (version 1 0) (export x) (defblock x (offers (p 1 bit))))\n(defblock y (offers (p 1 bit)))",
    );
    let (_, rendered, failed) = run(&modules, "stray");
    assert!(failed, "a stray form beside the declaration was accepted");
    assert!(rendered.contains("module-multiple-forms"), "{rendered}");
    assert!(
        rendered.contains("defblock y"),
        "the label must point at the stray form, not at the declaration:\n{rendered}"
    );
}

// ── Leaf `M1.29.2`: the rules a command needs, stated in docs/semantics/reference.md §6 rules 7 and 8 ──

#[test]
fn a_module_name_is_one_that_can_only_mean_one_file() {
    // §6 rule 8. The name becomes a file name, so it may not leave the module path, and it may not depend
    // on whether a filesystem folds case.
    for name in [
        "platform.timer",
        "hw.timer",
        "a",
        "os.rt-core",
        "app.two-timers",
        "x9.y_z",
    ] {
        assert!(is_module_name(name), "`{name}` should be a module name");
    }
    for name in [
        "",
        "../hw.timer",
        "hw/timer",
        "HW.Timer",
        "hw.Timer",
        "a..b",
        ".a",
        "a.",
        "9lives",
        "a.9b",
        "hw timer",
        "hw\\timer",
    ] {
        assert!(!is_module_name(name), "`{name}` must not be a module name");
    }
}

#[test]
fn an_import_naming_no_module_is_refused_before_any_file_is_looked_for() {
    let modules = MemoryModules::new()
        .with("root", "(defmodule root (version 1 0) (import ../escape))")
        .with("../escape", "(defmodule ../escape (version 1 0))");
    let (_, rendered, failed) = run(&modules, "root");
    assert!(
        failed,
        "a name that could leave the module path was imported"
    );
    assert!(rendered.contains("module-bad-import"), "{rendered}");
    assert!(rendered.contains("is not a module name"), "{rendered}");
    assert!(
        !rendered.contains("module-not-found"),
        "the name was looked up after it was refused:\n{rendered}"
    );
}

/// A scratch directory under `CARGO_TARGET_TMPDIR`, emptied first — on the repository's own volume.
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("creatable");
    dir
}

#[test]
fn a_directory_source_tells_a_missing_module_from_an_unreadable_one() {
    // §6 rule 7: "not found" about a file that is there would be a false statement about the description,
    // so an unreadable file is recorded for the caller instead.
    let dir = scratch("directory-modules");
    std::fs::write(
        dir.join("hw.present.eadl"),
        "(defmodule hw.present (version 1 0))\n",
    )
    .unwrap();
    std::fs::write(dir.join("hw.garbled.eadl"), [0xff_u8, 0xfe]).unwrap();
    let modules = DirectoryModules::new(&dir);

    let (display, text) = modules
        .load("hw.present")
        .expect("a file that is there is found");
    assert!(display.ends_with("hw.present.eadl"), "{display}");
    assert!(text.contains("defmodule hw.present"));

    assert!(modules.load("hw.absent").is_none());
    assert!(modules.load("hw.garbled").is_none());
    let unreadable = modules.unreadable();
    assert_eq!(
        unreadable.len(),
        1,
        "only the garbled file is unreadable: {unreadable:?}"
    );
    assert!(
        unreadable[0].0.ends_with("hw.garbled.eadl"),
        "{unreadable:?}"
    );
    assert!(
        modules
            .describe_missing("hw.absent")
            .contains("hw.absent.eadl"),
        "the repair must name the file it looked for"
    );
}

#[test]
fn a_directory_source_never_turns_a_non_module_name_into_a_path() {
    // Defence in depth behind `read_import`: a root named through `elaborate` is not an import.
    let dir = scratch("directory-modules-escape");
    let inner = dir.join("inner");
    std::fs::create_dir_all(&inner).unwrap();
    std::fs::write(
        dir.join("outside.eadl"),
        "(defmodule outside (version 1 0))\n",
    )
    .unwrap();
    let modules = DirectoryModules::new(&inner);
    assert!(
        modules.load("../outside").is_none(),
        "a name left the module path"
    );
    assert!(modules.unreadable().is_empty());
}

#[test]
fn the_root_is_read_not_looked_up_so_its_file_name_is_not_compared_with_its_name() {
    // §6 rule 7's last sentence: nothing imported the root, so no importer's name is there to verify.
    let dir = scratch("elaborate-source");
    std::fs::write(
        dir.join("hw.part.eadl"),
        "(defmodule hw.part (version 1 0))\n",
    )
    .unwrap();
    let root_text = "(defmodule app.whatever (version 1 0) (import hw.part))\n".to_string();
    let mut sources = SourceMap::new();
    let root = sources.add("some-other-name.eadl", root_text).unwrap();
    let modules = DirectoryModules::new(&dir);
    let (program, diagnostics) = elaborate_source(&mut sources, &modules, root);
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    assert_eq!(program.len(), 2);
    assert_eq!(program.root().module, "app.whatever");
    assert_eq!(program.instances[0].path, "part");
}

#[test]
fn a_malformed_version_is_one_mistake_and_one_message() {
    // ⛔ Found by making this code reachable from a command: `(version one zero)` was reported twice, the
    // second time as "declares no version" beside the clause that declares one.
    let modules = MemoryModules::new().with("vague", "(defmodule vague (version one zero))");
    let mut sources = SourceMap::new();
    let (_, diagnostics) = elaborate(&mut sources, &modules, "vague");
    let codes: Vec<&str> = diagnostics.items().iter().map(|d| d.code).collect();
    assert_eq!(
        codes,
        ["module-bad-version"],
        "{}",
        diagnostics.render(&sources)
    );
}

#[test]
fn an_empty_module_is_reported_in_the_file_that_is_empty() {
    // ⛔ Found by making this code reachable from a command: the label named source 0, which in a command
    // is the first shipped kind module — so an author was sent to `docs/semantics/kinds/core.eadl:1:1`
    // for a module they had written. Source 0 here is deliberately an unrelated file.
    let modules = MemoryModules::new()
        .with("root", "(defmodule root (version 1 0) (import hollow))")
        .with("hollow", "(eadl-version eadl/1)\n");
    let mut sources = SourceMap::new();
    sources
        .add("unrelated.eadl", "; source 0\n".to_string())
        .unwrap();
    let (_, diagnostics) = elaborate(&mut sources, &modules, "root");
    let rendered = diagnostics.render(&sources);
    assert!(rendered.contains("module-empty"), "{rendered}");
    assert!(rendered.contains("--> hollow.eadl:1:1"), "{rendered}");
    assert!(!rendered.contains("unrelated.eadl"), "{rendered}");
}

// ── Leaf `M1.29.3`: docs/semantics/reference.md §6 rules 9 and 10 — what a name means in an elaborated tree ──

/// A root importing one module that declares more than it exports, and whose own declaration names its
/// sibling; the root declares one name of its own.
fn scope() -> (eadl_front::Program, usize, usize) {
    let modules = MemoryModules::new()
        .with(
            "lib.part",
            r"(defmodule lib.part
               (version 1 0)
               (export shown)
               (defblock shown (offers (p 1 bit)) (uses hidden))
               (defblock hidden (offers (p 1 bit))))",
        )
        .with(
            "root",
            "(defmodule root (version 1 0) (import lib.part (as part)) (defblock mine (offers (p 1 bit))))",
        );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "root");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    let root = program.root().id;
    let part = program.instances[root].imports[0];
    (program, root, part)
}

fn at() -> eadl_front::Span {
    eadl_front::Span::new(eadl_front::SourceId(0), 0, 0)
}

#[test]
fn rule_9_an_instance_knows_its_alias_and_its_exports() {
    let (program, root, part) = scope();
    assert_eq!(program.instances[part].alias, "part");
    assert_eq!(program.instances[part].exports, ["shown"]);
    assert_eq!(
        program.instances[root].alias, "",
        "the root has no importer"
    );
    assert_eq!(program.instances[part].qualify("hidden"), "part.hidden");
}

#[test]
fn rule_10_a_name_resolves_to_its_own_instance_first() {
    let (program, root, part) = scope();
    assert_eq!(
        program.resolve(part, "hidden", at()).unwrap(),
        Some("part.hidden".into()),
        "a module names its own sibling by the local name, exported or not"
    );
    assert_eq!(
        program.resolve(root, "mine", at()).unwrap(),
        Some("mine".into()),
        "the root's own names are unqualified"
    );
}

#[test]
fn rule_10_an_import_shows_what_its_module_exports() {
    let (program, root, _) = scope();
    assert_eq!(
        program.resolve(root, "part.shown", at()).unwrap(),
        Some("part.shown".into())
    );
}

#[test]
fn rule_10_a_declared_name_the_module_does_not_export_is_refused_with_the_export_to_add() {
    let (program, root, _) = scope();
    let refusal = program
        .resolve(root, "part.hidden", at())
        .expect_err("hidden is not exported");
    assert_eq!(refusal.code, "module-not-exported");
    assert!(
        refusal
            .message
            .contains("declared by module `lib.part` but not exported"),
        "{}",
        refusal.message
    );
    assert!(
        refusal.repair.contains("add `(export hidden)`"),
        "{}",
        refusal.repair
    );
}

#[test]
fn rule_10_an_undeclared_name_through_an_alias_is_refused_without_suggesting_an_impossible_export()
{
    // Exporting a name the module does not declare is `module-dangling-export`, so suggesting it would trade
    // one refusal for another.
    let (program, root, _) = scope();
    let refusal = program
        .resolve(root, "part.nowhere", at())
        .expect_err("nothing is called that");
    assert_eq!(refusal.code, "module-not-exported");
    assert!(
        refusal.message.contains("exports no `nowhere`"),
        "{}",
        refusal.message
    );
    assert!(
        !refusal.repair.contains("(export nowhere)"),
        "{}",
        refusal.repair
    );
    assert!(
        refusal.repair.contains("no re-export"),
        "{}",
        refusal.repair
    );
}

#[test]
fn rule_10_a_name_that_is_neither_is_vocabulary_and_left_as_written() {
    let (program, root, part) = scope();
    assert_eq!(program.resolve(root, "counter-width", at()).unwrap(), None);
    assert_eq!(
        program.resolve(part, "absolute-deadline", at()).unwrap(),
        None
    );
    // `partial.x` begins with the characters of an alias and not with the alias as a segment.
    assert_eq!(program.resolve(root, "partial.x", at()).unwrap(), None);
}

#[test]
fn rule_10_an_alias_never_shadows_a_local_declaration() {
    // The instance's own names come first: a local `part.shown` is the root's, not the import's.
    let modules = MemoryModules::new()
        .with("lib.part", "(defmodule lib.part (version 1 0) (export shown) (defblock shown (offers (p 1 bit))))")
        .with(
            "root",
            "(defmodule root (version 1 0) (import lib.part (as part)) (defblock part.hidden (offers (p 1 bit))))",
        );
    let mut sources = SourceMap::new();
    let (program, _) = elaborate(&mut sources, &modules, "root");
    let root = program.root().id;
    assert_eq!(
        program.resolve(root, "part.hidden", at()).unwrap(),
        Some("part.hidden".into()),
        "a local declaration was reported as an unexported name of the import"
    );
}
