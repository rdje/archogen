//! The assignment record, `archogen-assignment/2` (leaf `M3.5.4.1`; `/2` by `M3.2.5`): its canonical text frozen in a
//! golden that is never rewritten, every record read back as itself, and every text that is not one refused — another
//! format, `/1`'s golden among them, kept unchanged beside `/2`'s; another adapter catalog; a kind the checker's table
//! does not hold, or an instance of another shape; a provider or a requester of another kind; a binding out of order or
//! twice. And the digest of the elaborated program moves with an imported module, its `with` and its version, never
//! with whitespace.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_plan_check::adapters::VERSION;
use archogen_plan_check::assignment::{
    program_digest, program_text, Assignment, Instance, Provider, Refusal, Requester, FORMAT,
    FORMAT_3,
};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};

/// A system importing a sub-hardware and a sub-OS module, the book's opening example's shape
/// (`docs/semantics/modules/`), one of them with a defaulted parameter.
fn library(timer_width: u32, timer_version: &str, root_with: &str) -> MemoryModules {
    MemoryModules::new()
        .with(
            "hw.timer",
            &format!(
                "(defmodule hw.timer\n  (version {timer_version})\n  (param tick-rate (default (tick-rate 10 MHz)))\n  \
                 (export timer.counter)\n  (defblock timer.counter\n    (offers (counter-width {timer_width} bit))))"
            ),
        )
        .with(
            "hw.soc",
            &format!(
                "(defmodule hw.soc\n  (version 1 2)\n  (import hw.timer (as timer) (version (at-least 1 0)){root_with})\n  \
                 (export soc.bus)\n  (defplatform soc.bus\n    (offers (region device.timer (base 0x1000_0000))\n      (region soc.ram (base 0x8000_0000) (size 128 MiB) (executable true)))))"
            ),
        )
        .with(
            "os.time",
            "(defmodule os.time\n  (version 2 0)\n  (export time.monotonic)\n  (defservice time.monotonic\n    \
             (requires (unambiguous-horizon (at-least 60 s)))))",
        )
        .with(
            "app.system",
            "(defmodule app.system\n  (version 1 0)\n  (import hw.soc  (as platform) (version (at-least 1 1)))\n  \
             (import os.time (as clock)    (version (at-least 2 0)))\n  (export app.rt)\n  (defsystem app.rt\n    \
             (requires (uses clock.time.monotonic))))",
        )
}

fn program(modules: &MemoryModules) -> Program {
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, modules, "app.system");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    program
}

/// The device every reference binding names.
const TIMER: &str = "platform.timer.timer.counter";

fn named(name: &str) -> Requester {
    Requester::Named(name.to_owned())
}

fn sensor() -> Requester {
    Requester::Task {
        system: "app.rt".to_owned(),
        task: "sensor".to_owned(),
    }
}

/// The record both goldens hold the program and placement of: a placement, a task's binding and two of named
/// requesters, written out of order.
fn record(bindings: BTreeMap<(Requester, String), Provider>) -> Assignment {
    Assignment {
        profile: "rt-static-up-v1".to_owned(),
        system: "app.rt".to_owned(),
        program: program_digest(&program(&library(32, "1 0", ""))),
        target: None,
        placement: Some(("platform.soc.bus".to_owned(), "soc.ram".to_owned())),
        bindings,
    }
}

/// `/1`'s reference record, its bindings to a block alone.
fn reference() -> Assignment {
    let mut bindings = BTreeMap::new();
    bindings.insert((sensor(), "time.monotonic".to_owned()), TIMER.into());
    bindings.insert(
        (
            named("clock.time.monotonic"),
            "unambiguous-horizon".to_owned(),
        ),
        TIMER.into(),
    );
    bindings.insert(
        (named("clock.time.monotonic"), "counter-width".to_owned()),
        TIMER.into(),
    );
    record(bindings)
}

/// An instance of `kind` over the reference's timer, serving `serves`.
fn instance(kind: &str, serves: Option<Requester>) -> Instance {
    Instance {
        kind: kind.to_owned(),
        over: TIMER.to_owned(),
        serves: serves.map(Box::new),
    }
}

/// `/2`'s reference record: a shared instance, the epoch extender, read by a named requester and a task; an exclusive
/// one, the deadline translator, serving each of them — the task's sorting before the name's, as the canonical text
/// sorts `(` before a name; and each instance's bindings beneath, to the timer it programs. Written out of order.
fn reference_2() -> Assignment {
    let extender = instance("epoch-extender", None);
    let for_sensor = instance("deadline-over-delay", Some(sensor()));
    let for_clock = instance("deadline-over-delay", Some(named("clock.time.monotonic")));
    let mut bindings = BTreeMap::new();
    for (instance, facts) in [
        (
            &extender,
            &[
                "wrap-behavior",
                "tick-rate",
                "relative-delay",
                "observation-coherent",
                "counter-modulus",
            ][..],
        ),
        (
            &for_clock,
            &[
                "tick-rate",
                "relative-delay",
                "observation-coherent",
                "counter-modulus",
            ][..],
        ),
        (
            &for_sensor,
            &[
                "tick-rate",
                "relative-delay",
                "observation-coherent",
                "counter-modulus",
            ][..],
        ),
    ] {
        for fact in facts {
            bindings.insert(
                (Requester::Instance(instance.clone()), (*fact).to_owned()),
                TIMER.into(),
            );
        }
    }
    for (requester, fact, provider) in [
        (sensor(), "tick-rate", &extender),
        (sensor(), "absolute-deadline", &for_sensor),
        (
            named("clock.time.monotonic"),
            "unambiguous-horizon",
            &extender,
        ),
        (named("clock.time.monotonic"), "counter-width", &extender),
        (
            named("clock.time.monotonic"),
            "absolute-deadline",
            &for_clock,
        ),
    ] {
        bindings.insert(
            (requester, fact.to_owned()),
            Provider::Instance(provider.clone()),
        );
    }
    record(bindings)
}

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

#[test]
fn the_record_s_canonical_text_is_frozen_in_its_golden() {
    let record = reference_2();
    let actual = record.to_canonical();
    let path = goldens().join(format!("{}.golden", FORMAT.replace('/', "-")));
    match fs::read_to_string(&path) {
        Ok(golden) => {
            assert_eq!(
                actual, golden,
                "the text of `{FORMAT}` has changed. A golden is never rewritten: move `FORMAT` (and the register, \
                 `docs/book/src/versions.md`) to a new identifier, and the search's constant with it"
            );
            // Written byte for byte, and read back as itself.
            assert_eq!(Assignment::read(&golden), Ok(record));
        }
        Err(_) if std::env::var_os("ARCHOGEN_BLESS_FORMATS").is_some() => {
            fs::create_dir_all(goldens()).expect("the goldens directory is creatable");
            fs::write(&path, actual).expect("the golden is writable");
        }
        Err(_) => panic!(
            "no golden for `{FORMAT}` at {}. A new identifier gets one deliberately: \
             ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-plan-check --test record",
            path.display()
        ),
    }
}

#[test]
fn the_canonical_text_sorts_names_then_tasks_then_instances_each_by_its_text() {
    let text = reference_2().to_canonical();
    let requesters: Vec<&str> = text
        .lines()
        .filter_map(|line| line.strip_prefix("(bind "))
        .map(|rest| {
            let end = if rest.starts_with('(') {
                // A form: up to its matching parenthesis.
                let mut depth = 0;
                rest.char_indices()
                    .find(|(_, c)| {
                        depth += match c {
                            '(' => 1,
                            ')' => -1,
                            _ => 0,
                        };
                        depth == 0
                    })
                    .map_or(rest.len(), |(i, _)| i + 1)
            } else {
                rest.find(' ').unwrap_or(rest.len())
            };
            &rest[..end]
        })
        .collect();
    let mut seen: Vec<&str> = requesters.clone();
    seen.dedup();
    assert_eq!(
        seen,
        [
            "clock.time.monotonic",
            "(task app.rt sensor)",
            "(adapter deadline-over-delay platform.timer.timer.counter (task app.rt sensor))",
            "(adapter deadline-over-delay platform.timer.timer.counter clock.time.monotonic)",
            "(adapter epoch-extender platform.timer.timer.counter)",
        ]
    );
    // Each requester and provider reads back from its canonical text.
    for ((requester, _), provider) in &reference_2().bindings {
        assert_eq!(
            Requester::parse(&requester.canonical()).as_ref(),
            Ok(requester)
        );
        assert_eq!(
            Provider::parse(&provider.canonical()).as_ref(),
            Ok(provider)
        );
    }
}

#[test]
fn the_first_format_s_golden_is_kept_unchanged_and_refused_as_another_format() {
    let path = goldens().join("archogen-assignment-1.golden");
    let golden = fs::read_to_string(&path).expect("`/1`'s golden is kept");
    // Its text, unchanged: `/1`'s writer's for the reference record.
    let digest = reference().program;
    assert_eq!(
        golden,
        format!(
            "(format \"archogen-assignment/1\")\n(profile rt-static-up-v1)\n(system app.rt)\n(program \"{digest}\")\n\
             (placement platform.soc.bus soc.ram)\n\
             (bind clock.time.monotonic counter-width platform.timer.timer.counter)\n\
             (bind clock.time.monotonic unambiguous-horizon platform.timer.timer.counter)\n\
             (bind (task app.rt sensor) time.monotonic platform.timer.timer.counter)\n"
        ),
        "`/1`'s golden is never rewritten"
    );
    // `/2` is `/1` with two additions (record §5): the format's identifier, and the adapter catalog after the program.
    assert_eq!(
        reference().to_canonical(),
        golden.replace("archogen-assignment/1", FORMAT).replace(
            &format!("(program \"{digest}\")\n"),
            &format!("(program \"{digest}\")\n(adapters {VERSION})\n")
        )
    );
    // And this checker reads `/2` alone.
    assert_eq!(
        Assignment::read(&golden),
        Err(Refusal::Format(Some("archogen-assignment/1".to_owned())))
    );
}

#[test]
fn every_record_reads_back_as_itself() {
    for record in [reference(), reference_2()] {
        assert_eq!(Assignment::read(&record.to_canonical()), Ok(record.clone()));
        // Without a placement, as a system with no placement memory writes it; with whitespace and comments the
        // reader's.
        let mut hosted = record.clone();
        hosted.placement = None;
        assert_eq!(Assignment::read(&hosted.to_canonical()), Ok(hosted));
        let spaced = record
            .to_canonical()
            .replace('\n', "\n\n  ; a comment, inert\n")
            .replace(" (", "   (");
        assert_eq!(Assignment::read(&spaced), Ok(record));
    }
}

#[test]
fn every_text_that_is_no_record_of_the_format_is_refused() {
    let text = reference_2().to_canonical();
    // Another format, `/1` among them.
    for other in ["archogen-assignment/1", "archogen-assignment/4"] {
        assert_eq!(
            Assignment::read(&text.replace(FORMAT, other)),
            Err(Refusal::Format(Some(other.to_owned())))
        );
    }
    // `/3` is read, and a `/2` text labelled so lacks the target `/3` writes after its program (leaf `M4.1.7.5.4.1`).
    assert!(matches!(
        Assignment::read(&text.replace(FORMAT, FORMAT_3)),
        Err(Refusal::Clause {
            expected: "target",
            ..
        })
    ));
    // Another adapter catalog, or none.
    assert_eq!(
        Assignment::read(&text.replace("(adapters adapters/1)", "(adapters adapters/2)")),
        Err(Refusal::Catalog("adapters/2".to_owned()))
    );
    assert!(matches!(
        Assignment::read(&text.replace("(adapters adapters/1)\n", "")),
        Err(Refusal::Clause {
            expected: "adapters",
            ..
        })
    ));
    assert!(matches!(
        Assignment::read(
            &text.replace("(adapters adapters/1)", "(adapters adapters/1 adapters/2)")
        ),
        Err(Refusal::Value {
            clause: "adapters",
            ..
        })
    ));
    // A kind the checker's table does not hold, wherever an instance stands.
    assert_eq!(
        Assignment::read(&text.replace("(adapter epoch-extender ", "(adapter epoch-stretcher ")),
        Err(Refusal::AdapterKind {
            kind: "epoch-stretcher".to_owned(),
            found: "(adapter epoch-stretcher platform.timer.timer.counter)".to_owned()
        })
    );
    // An instance whose shape is not its kind's: an exclusive kind without its requester, a shared one with one; over
    // an instance, serving one, or of another length.
    let deadline =
        "(adapter deadline-over-delay platform.timer.timer.counter clock.time.monotonic)";
    let extender = "(adapter epoch-extender platform.timer.timer.counter)";
    for (from, to) in [
        (
            deadline,
            "(adapter deadline-over-delay platform.timer.timer.counter)",
        ),
        (
            extender,
            "(adapter epoch-extender platform.timer.timer.counter clock.time.monotonic)",
        ),
        (
            extender,
            "(adapter epoch-extender (adapter epoch-extender platform.timer.timer.counter))",
        ),
        (
            deadline,
            "(adapter deadline-over-delay platform.timer.timer.counter (adapter epoch-extender platform.timer.timer.counter))",
        ),
        (
            deadline,
            "(adapter deadline-over-delay platform.timer.timer.counter clock.time.monotonic extra)",
        ),
        (extender, "(adapter)"),
    ] {
        let refused = Assignment::read(&text.replace(from, to));
        assert!(
            matches!(refused, Err(Refusal::InstanceShape { ref found, .. }) if found == to),
            "{to}: {refused:?}"
        );
    }
    // A provider of a kind `/2` does not name — a catalog record — is refused, never read as a name.
    let record = text.replacen(extender, "(record abc)", 1);
    assert_eq!(
        Assignment::read(&record),
        Err(Refusal::ProviderKind("(record abc)".to_owned()))
    );
    let requester = text.replace("(task app.rt sensor)", "(job app.rt sensor)");
    assert_eq!(
        Assignment::read(&requester),
        Err(Refusal::RequesterKind("(job app.rt sensor)".to_owned()))
    );
    // Out of order — two names, two instances, and an instance before a task — and twice for one requester and fact.
    let lines: Vec<&str> = text.lines().collect();
    let binds = lines.iter().position(|l| l.starts_with("(bind ")).unwrap();
    let task = lines
        .iter()
        .position(|l| l.starts_with("(bind (task "))
        .unwrap();
    let instances = lines
        .iter()
        .position(|l| l.starts_with("(bind (adapter "))
        .unwrap();
    let extended = lines
        .iter()
        .position(|l| l.starts_with("(bind (adapter epoch-extender "))
        .unwrap();
    for (a, b) in [(binds, binds + 1), (task, instances), (instances, extended)] {
        let mut swapped = lines.clone();
        swapped.swap(a, b);
        let swapped = swapped.join("\n");
        assert!(
            matches!(Assignment::read(&swapped), Err(Refusal::NotCanonical(_))),
            "{swapped}"
        );
    }
    let twice = format!("{text}{}\n", lines[lines.len() - 1]);
    assert!(
        matches!(Assignment::read(&twice), Err(Refusal::NotCanonical(_))),
        "{twice}"
    );
    // A placement that names no region of a provider.
    let unplaced = text.replace(
        "(placement platform.soc.bus soc.ram)",
        "(placement soc.ram)",
    );
    assert!(matches!(
        Assignment::read(&unplaced),
        Err(Refusal::Value {
            clause: "placement",
            ..
        })
    ));
    // A clause missing, a digest that is no sha256, and no eADL at all.
    let no_system = text.replace("(system app.rt)\n", "");
    assert!(matches!(
        Assignment::read(&no_system),
        Err(Refusal::Clause {
            expected: "system",
            ..
        })
    ));
    let short = text.replace(&reference_2().program, "abc");
    assert!(matches!(
        Assignment::read(&short),
        Err(Refusal::Value {
            clause: "program",
            ..
        })
    ));
    assert!(matches!(
        Assignment::read("(format \"archogen-assignment/2\""),
        Err(Refusal::Unreadable(_))
    ));
}

#[test]
fn the_program_digest_moves_with_a_module_its_with_and_its_version_and_with_nothing_else() {
    let base = program_digest(&program(&library(32, "1 0", "")));
    assert_eq!(base.len(), 64);
    assert_eq!(
        base,
        reference().program,
        "the digest is a function of the program"
    );
    for (case, modules) in [
        ("an imported module's declaration", library(16, "1 0", "")),
        ("an imported module's version", library(32, "1 1", "")),
        (
            "an import's `with`",
            library(32, "1 0", " (with (tick-rate (tick-rate 20 MHz)))"),
        ),
    ] {
        let moved = program_digest(&program(&modules));
        assert_ne!(moved, base, "{case}: {}", program_text(&program(&modules)));
    }
    // Whitespace and comments are no part of the elaborated program.
    let respaced = MemoryModules::new()
        .with(
            "hw.timer",
            "; a comment\n(defmodule hw.timer (version 1 0) (param tick-rate (default (tick-rate 10 MHz))) \
             (export timer.counter) (defblock timer.counter (offers (counter-width 32 bit))))",
        )
        .with(
            "hw.soc",
            "(defmodule hw.soc (version 1 2) (import hw.timer (as timer) (version (at-least 1 0))) (export soc.bus) \
             (defplatform soc.bus (offers (region device.timer (base 0x1000_0000))\
             (region soc.ram (base 0x8000_0000) (size 128 MiB) (executable true)))))",
        )
        .with(
            "os.time",
            "(defmodule os.time (version 2 0) (export time.monotonic) (defservice time.monotonic (requires \
             (unambiguous-horizon (at-least 60 s)))))",
        )
        .with(
            "app.system",
            "(defmodule app.system (version 1 0) (import hw.soc (as platform) (version (at-least 1 1))) \
             (import os.time (as clock) (version (at-least 2 0))) (export app.rt) (defsystem app.rt (requires \
             (uses clock.time.monotonic))))",
        );
    assert_eq!(program_digest(&program(&respaced)), base);
}

// ── `archogen-assignment/3`, a plan of a system on a target (leaf `M4.1.7.5.4.1`) ──────────────────────────────────────

/// `/3`'s record as its golden writes it, built here apart from the reader: a target after the program, and the release
/// requester last among requesters, after a name, a task and an instance.
fn reference_3() -> Assignment {
    let mut bindings = BTreeMap::new();
    let extender = Instance {
        kind: "epoch-extender".to_owned(),
        over: "target.timer".to_owned(),
        serves: None,
    };
    let release = || Requester::Release {
        system: "app.rt".to_owned(),
    };
    for (requester, fact, provider) in [
        (
            Requester::Named("console.write".to_owned()),
            "observable-output",
            Provider::Named("target.console".to_owned()),
        ),
        (
            Requester::Task {
                system: "app.rt".to_owned(),
                task: "sensor".to_owned(),
            },
            "counter-width",
            Provider::Instance(extender.clone()),
        ),
        (
            Requester::Instance(extender.clone()),
            "counter-modulus",
            Provider::Named("target.timer".to_owned()),
        ),
        (
            release(),
            "alarm-kind",
            Provider::Named("target.timer".to_owned()),
        ),
        (
            release(),
            "relative-delay",
            Provider::Named("target.timer".to_owned()),
        ),
        (
            release(),
            "supported-horizon",
            Provider::Named("target.timer".to_owned()),
        ),
    ] {
        bindings.insert((requester, fact.to_owned()), provider);
    }
    Assignment {
        profile: "rt-static-up-v1".to_owned(),
        system: "app.rt".to_owned(),
        program: "f116a4715daf8b404d559b210f6f0d09c060158b1c1493fb133bf50197545620".to_owned(),
        target: Some((
            "target.riscv-virt-up".to_owned(),
            "0f1e2d3c4b5a69788796a5b4c3d2e1f00f1e2d3c4b5a69788796a5b4c3d2e1f0".to_owned(),
        )),
        placement: Some(("target.riscv-virt-up".to_owned(), "target.ram".to_owned())),
        bindings,
    }
}

/// `/3`'s golden, written apart from both writers: the checker's writes it byte for byte, and its reader reads it back as
/// itself, as `/2`'s are held to their own (the search's writer is held to it in `crates/eadl-resolve/tests/plan.rs`).
#[test]
fn the_target_format_s_canonical_text_is_its_golden() {
    let golden =
        fs::read_to_string(goldens().join(format!("{}.golden", FORMAT_3.replace('/', "-"))))
            .expect("`/3`'s golden");
    assert_eq!(reference_3().to_canonical(), golden);
    assert_eq!(Assignment::read(&golden), Ok(reference_3()));
}

/// What `/3` alone says: a release requester in `/2`, and a `/3` naming no target or no digest, are no record.
#[test]
fn a_release_requester_and_a_target_read_in_the_target_format_alone() {
    let golden = reference_3().to_canonical();
    let line = "(target target.riscv-virt-up \"0f1e2d3c4b5a69788796a5b4c3d2e1f00f1e2d3c4b5a69788796a5b4c3d2e1f0\")\n";
    let as_second = golden.replacen(FORMAT_3, FORMAT, 1).replacen(line, "", 1);
    assert!(
        matches!(Assignment::read(&as_second), Err(Refusal::RequesterKind(found)) if found == "(release app.rt)"),
        "{:?}",
        Assignment::read(&as_second)
    );
    assert!(
        matches!(
            Assignment::read(&golden.replacen(line, "", 1)),
            Err(Refusal::Clause {
                expected: "target",
                ..
            })
        ),
        "a `/3` naming no target"
    );
    assert!(
        matches!(
            Assignment::read(&golden.replacen(
                "\"0f1e2d3c4b5a69788796a5b4c3d2e1f00f1e2d3c4b5a69788796a5b4c3d2e1f0\"",
                "\"0f1e\"",
                1
            )),
            Err(Refusal::Value {
                clause: "target",
                ..
            })
        ),
        "a target that is no digest"
    );
}

/// `/2` §1: a plan resolved on a target, checked with no target, is a record of other inputs, `stale-input`, "resolve again",
/// before any judgement.
#[test]
fn a_plan_on_a_target_checked_with_no_target_is_stale() {
    let text = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/s0-heartbeat/system.eadl"),
    )
    .expect("the example");
    let judge = |plan: &str| {
        let mut sources = SourceMap::new();
        let id = sources.add("system.eadl", text.clone()).expect("small");
        archogen_plan_check::check::plan_file(
            &mut sources,
            id,
            &MemoryModules::new(),
            &eadl_model::profile::RT_STATIC_UP_V1,
            "plan.eadl",
            plan,
        )
    };
    let program = judge("").0;
    let mut record = reference_3();
    record.system = "heartbeat".to_owned();
    record.program = program_digest(&program);
    record.placement = None;
    record.bindings = BTreeMap::new();
    let verdict = judge(&record.to_canonical()).1.expect_err("refused");
    assert_eq!(verdict.status, archogen_outcome::Status::StaleInput);
    assert_eq!(verdict.findings[0].code, "plan-stale");
    assert!(
        verdict.findings[0].message.contains(FORMAT_3)
            && verdict.findings[0].message.contains("no target is given"),
        "{}",
        verdict.findings[0].message
    );
    // Its repair offers the way the plan is judged where it was resolved: the target named, `--target FILE` or the MCP
    // tool's `target`, `record` serving both; and no leaf, the judgement on a target being built.
    let repair = &verdict.findings[0].repair;
    assert!(
        repair.contains("--target FILE") && repair.contains("`target`") && !repair.contains("leaf"),
        "{repair}"
    );
}
