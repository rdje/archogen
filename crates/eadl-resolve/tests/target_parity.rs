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
