//! The generator's reading of a target and of a system on it held to the configuration checker's (leaf `M4.1.7.5.4.2`;
//! `DC-H2`): `eadl_resolve::target` is the generator's and `archogen_plan_check::target` the checker's, written by a context
//! that never read this crate (`docs/tasks/M3.md`'s fence). Over the shipped target, the examples and one case for each row
//! of `docs/decisions/decision_device-correspondence.md`'s two tables, the two readers accept the same inputs, refuse the
//! rest with the same codes, and read the same hardware for a system on the target. A divergence is a finding against one
//! reader or the record, never a reason to bend this test.

use std::collections::BTreeSet;
use std::path::PathBuf;

use eadl_front::{Form, SourceMap, Span};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn file(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative)).expect("readable")
}

/// What one reader concluded: accepted with the system's hardware there, or refused with its codes.
#[derive(Debug, PartialEq, Eq)]
enum Read {
    /// The system put on the target, its hardware there.
    On(BTreeSet<String>),
    /// The target's file is no target, by these codes.
    NoTarget(BTreeSet<String>),
    /// The system cannot be read there, by these codes.
    NotThere(BTreeSet<String>),
}

fn declarations(sources: &SourceMap, id: eadl_front::SourceId) -> Vec<Form> {
    let (document, diagnostics) = eadl_front::read(sources, id);
    assert!(!diagnostics.has_errors(), "a case reads");
    document
        .forms
        .into_iter()
        .filter(|form| form.head() != Some("eadl-version"))
        .collect()
}

/// The generator's reading.
fn generator(description: &str, target: &str, system: &str) -> Read {
    let mut sources = SourceMap::new();
    let d = sources.add("system.eadl", description).expect("small");
    let t = sources.add("target.eadl", target).expect("small");
    let target_forms = declarations(&sources, t);
    let read = match eadl_resolve::target::Target::read(&target_forms, Span::at(t, 0)) {
        Ok(read) => read,
        Err(errors) => return Read::NoTarget(errors.iter().map(|e| e.code.to_owned()).collect()),
    };
    let forms = declarations(&sources, d);
    let the_system = forms
        .iter()
        .find(|f| {
            f.head() == Some("defsystem")
                && f.items().get(1).and_then(Form::as_symbol) == Some(system)
        })
        .expect("the system");
    match eadl_resolve::target::on_target(the_system, &forms, &read) {
        Ok(there) => {
            Read::On(eadl_resolve::joint::System::read(&there.system, &there.declarations).hardware)
        }
        Err(errors) => Read::NotThere(errors.iter().map(|e| e.code.to_owned()).collect()),
    }
}

/// The checker's reading.
fn checker(description: &str, target: &str, system: &str) -> Read {
    let mut sources = SourceMap::new();
    let d = sources.add("system.eadl", description).expect("small");
    let t = sources.add("target.eadl", target).expect("small");
    let read = match archogen_plan_check::target::Target::read(&sources, t) {
        Ok(read) => read,
        Err(verdict) => {
            return Read::NoTarget(verdict.findings.iter().map(|f| f.code.clone()).collect())
        }
    };
    let program = archogen_plan_check::check::plain(declarations(&sources, d));
    match archogen_plan_check::target::on_target(&sources, &program, system, &read) {
        Ok(there) => Read::On(there.hardware),
        Err(verdict) => Read::NotThere(verdict.findings.iter().map(|f| f.code.clone()).collect()),
    }
}

/// The heartbeat's forms on a playground of its own, each case varying one thing.
const HEARTBEAT: &str = "
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform host.playground (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem heartbeat
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write) (on-overrun fault))
  (requires (uses console.write))
  (platform (uses host.playground)))
";

/// A small machine: a console under one platform.
const BOARD: &str = "
(defblock board.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform board.soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses board.uart)))
";

/// Every case: what it varies, the description, the target and the system put on it.
fn cases() -> Vec<(&'static str, String, String, &'static str)> {
    let shipped = file("targets/riscv-virt-up.eadl");
    let heartbeat = |from: &str, to: &str| {
        assert_eq!(HEARTBEAT.matches(from).count(), 1, "`{from}`");
        HEARTBEAT.replacen(from, to, 1)
    };
    vec![
        ("the heartbeat on the shipped target", HEARTBEAT.to_owned(), shipped.clone(), "heartbeat"),
        (
            "the S0 example on the shipped target",
            file("examples/s0-heartbeat/system.eadl"),
            shipped.clone(),
            "heartbeat",
        ),
        (
            "periodic-three on the shipped target",
            file("examples/periodic-three/system.eadl"),
            shipped.clone(),
            "periodic-three",
        ),
        ("on a board", HEARTBEAT.to_owned(), BOARD.to_owned(), "heartbeat"),
        (
            "a system with no platform clause",
            heartbeat("  (platform (uses host.playground)))", ")"),
            BOARD.to_owned(),
            "heartbeat",
        ),
        // What a target's file is.
        (
            "a target declaring a service",
            HEARTBEAT.to_owned(),
            format!("{BOARD}(defservice board.log (requires (needs observable-output)))\n"),
            "heartbeat",
        ),
        (
            "a target with two platforms nothing composes",
            HEARTBEAT.to_owned(),
            format!("{BOARD}(defplatform board.other (offers (core-count 1 tick)))\n"),
            "heartbeat",
        ),
        (
            "a target device its platform does not compose",
            HEARTBEAT.to_owned(),
            format!("{BOARD}(defblock board.spare (offers (observable-output true)))\n"),
            "heartbeat",
        ),
        (
            "a target naming one name twice",
            HEARTBEAT.to_owned(),
            format!("{BOARD}(defblock board.uart (offers (observable-output true)))\n"),
            "heartbeat",
        ),
        (
            "a target whose uses holds a list",
            HEARTBEAT.to_owned(),
            BOARD.replacen("(requires (uses board.uart))", "(requires (uses (board.uart)))", 1),
            "heartbeat",
        ),
        (
            "a target refining back into itself",
            HEARTBEAT.to_owned(),
            BOARD.replacen("(defblock board.uart (offers", "(defblock board.uart (refines board.uart) (offers", 1),
            "heartbeat",
        ),
        // What is refused before any search.
        (
            "a name both declare",
            format!("{HEARTBEAT}(defblock board.uart (offers (observable-output true)))\n"),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "the description naming the target's device",
            heartbeat("  (requires (uses console.write))\n", "  (requires (uses console.write board.uart))\n"),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "the target naming what only the description declares",
            HEARTBEAT.to_owned(),
            BOARD.replacen("(requires (uses board.uart))", "(requires (uses board.uart console.uart))", 1),
            "heartbeat",
        ),
        (
            "a task naming a block itself",
            heartbeat("(uses console.write) (on-overrun fault))", "(uses console.write console.uart) (on-overrun fault))"),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "a reached service naming a block itself",
            heartbeat(
                "(defservice console.write (requires (needs observable-output)))",
                "(defservice console.write (requires (uses console.uart)))",
            ),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "the platform stood on declaring absent what the target offers",
            heartbeat(
                "(defplatform host.playground (offers",
                "(defplatform host.playground (absent observable-output) (offers",
            ),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "two platform clauses",
            heartbeat("  (platform (uses host.playground)))", "  (platform (uses host.playground))\n  (platform (uses console.uart)))"),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "the system's hardware refining back into itself",
            heartbeat("(defblock console.uart (offers", "(defblock console.uart (refines console.uart) (offers"),
            BOARD.to_owned(),
            "heartbeat",
        ),
        (
            "another system named, its own clause naming its hardware",
            format!(
                "{}(defsystem logger (requires (uses console.write)) (platform (uses host.playground)))\n",
                heartbeat("  (requires (uses console.write))\n", "  (requires (uses console.write logger))\n")
            ),
            BOARD.to_owned(),
            "heartbeat",
        ),
    ]
}

#[test]
fn the_checker_reads_a_target_and_a_system_on_it_as_the_generator_does() {
    let mut differ = Vec::new();
    for (what, description, target, system) in cases() {
        let generator = generator(&description, &target, system);
        let checker = checker(&description, &target, system);
        if generator != checker {
            differ.push(format!(
                "{what}:\n  generator {generator:?}\n  checker   {checker:?}"
            ));
        }
    }
    assert!(
        differ.is_empty(),
        "{} case(s) read two ways:\n{}",
        differ.len(),
        differ.join("\n")
    );
}

/// Every tracked description, as the verdict ratchet walks them, that reads as a plain description: no module, no kind
/// module, no vocabulary file.
fn plain_descriptions() -> Vec<PathBuf> {
    fn walk(dir: &std::path::Path, top: bool, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if name.starts_with('.')
                    || (top && ["target", "build", "vendor"].contains(&name.as_str()))
                {
                    continue;
                }
                walk(&path, false, out);
            } else if name.ends_with(".eadl") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(&root(), true, &mut out);
    out.sort();
    out.retain(|path| {
        let text = std::fs::read_to_string(path).unwrap_or_default();
        let mut sources = SourceMap::new();
        let Ok(id) = sources.add("d.eadl", text) else {
            return false;
        };
        let (document, diagnostics) = eadl_front::read(&sources, id);
        !diagnostics.has_errors()
            && !document.forms.iter().any(|f| {
                matches!(
                    f.head(),
                    Some("defmodule" | "defkind" | "deffact" | "defadapter")
                )
            })
    });
    out
}

/// The target's digest, the identity `/3` writes, is one value in both: the generator's `plan::target_digest` and the checker's
/// `Target::digest` (leaf `M4.1.7.5.4.3`).
#[test]
fn the_checker_and_the_generator_take_one_digest_of_a_target() {
    let shipped = file("targets/riscv-virt-up.eadl");
    for text in [
        shipped.clone(),
        format!("; a comment\n{shipped}"),
        BOARD.to_owned(),
    ] {
        let mut sources = SourceMap::new();
        let t = sources.add("target.eadl", text.as_str()).expect("small");
        let generator = eadl_resolve::plan::target_digest(
            &eadl_resolve::target::Target::read(&declarations(&sources, t), Span::at(t, 0))
                .expect("a target"),
        );
        let checker = archogen_plan_check::target::Target::read(&sources, t)
            .expect("a target")
            .digest();
        assert_eq!(generator, checker);
    }
}

/// Every plan the search writes on the shipped target, over every tracked plain description `check` admits there and each of its
/// systems, is one the checker accepts, `archogen-assignment/3` judged on the target (leaf `M4.1.7.5.4.3`): the release
/// requester's bindings among them. Admitted as `DC-H3` judges — every pass on the description but presence, then presence on
/// the system put on the target — since a plan is written only of a description `check` accepts.
#[test]
fn every_plan_the_search_writes_on_the_target_the_checker_accepts() {
    let v = vocabulary_of();
    let policy = eadl_resolve::search::Policy {
        adapters: eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &v)
            .expect("the catalog"),
        ..eadl_resolve::search::Policy::default()
    };
    let target_text = file("targets/riscv-virt-up.eadl");
    let mut plans = 0;
    let mut releasing = 0;
    let mut refused = Vec::new();
    let registry = eadl_model::check::shipped_registry(&mut SourceMap::new(), &kinds())
        .expect("the kinds load");
    for file in plain_descriptions() {
        let text = std::fs::read_to_string(&file).expect("readable");
        let mut sources = SourceMap::new();
        let d = sources
            .add(file.display().to_string(), text.as_str())
            .expect("small");
        let t = sources
            .add("targets/riscv-virt-up.eadl", target_text.as_str())
            .expect("small");
        let forms = declarations(&sources, d);
        let target = eadl_resolve::target::Target::read(&declarations(&sources, t), Span::at(t, 0))
            .expect("the target");
        let program = eadl_resolve::plan::program_digest(&eadl_resolve::plan::plain(forms.clone()));
        let passes = eadl_model::check::check_but_presence(
            &sources,
            d,
            &registry,
            &eadl_model::profile::RT_STATIC_UP_V1,
        );
        if !passes.is_ok() {
            continue;
        }
        for system in forms.iter().filter(|f| f.head() == Some("defsystem")) {
            let Ok(there) = eadl_resolve::target::on_target(system, &forms, &target) else {
                continue;
            };
            if there
                .report()
                .diagnostics
                .iter()
                .any(|d| d.severity == eadl_front::Severity::Error)
            {
                continue;
            }
            let eadl_resolve::search::Search::Resolved(resolution) =
                eadl_resolve::search::search_on_target(
                    &there,
                    eadl_model::profile::RT_STATIC_UP_V1.caller_level,
                    &v,
                    &policy,
                    eadl_resolve::search::Limits::DEFAULT,
                )
            else {
                continue;
            };
            let plan = eadl_resolve::plan::write_on_target(
                &resolution,
                eadl_model::profile::RT_STATIC_UP_V1.id,
                &program,
                (
                    &target.platform,
                    &eadl_resolve::plan::target_digest(&target),
                ),
            );
            plans += 1;
            releasing += usize::from(plan.contains("(bind (release "));
            let checker_target =
                archogen_plan_check::target::Target::read(&sources, t).expect("the target");
            let checked = archogen_plan_check::check::plan_on_target(
                &sources,
                &archogen_plan_check::check::plain(forms.clone()),
                &checker_target,
                &eadl_model::profile::RT_STATIC_UP_V1,
                "plan.eadl",
                &plan,
            );
            if let Err(verdict) = checked {
                refused.push(format!(
                    "{} `{}`: {:?} {:?}\n{plan}",
                    file.display(),
                    resolution.system,
                    verdict.status,
                    verdict
                        .findings
                        .iter()
                        .map(|f| format!("{}: {}", f.code, f.message))
                        .collect::<Vec<_>>()
                ));
            }
        }
    }
    assert!(
        refused.is_empty(),
        "{} plan(s) the checker refused:\n{}",
        refused.len(),
        refused.join("\n")
    );
    assert!(plans > 0, "no description resolved on the target");
    assert!(
        releasing > 0,
        "no plan on the target bound a release requester"
    );
}

/// The shipped kind modules, by their paths.
fn kinds() -> Vec<(String, String)> {
    [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| ((*r).to_string(), file(r)))
    .collect()
}

fn vocabulary_of() -> eadl_resolve::vocabulary::Vocabulary {
    let registry = eadl_model::check::shipped_registry(&mut SourceMap::new(), &kinds())
        .expect("the kinds load");
    eadl_resolve::vocabulary::Vocabulary::shipped(&mut SourceMap::new(), &registry)
        .expect("the vocabulary reads")
}
