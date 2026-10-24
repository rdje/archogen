//! The assignment record, `archogen-assignment/1` (leaf `M3.5.4.1`): its canonical text frozen in a golden that is
//! never rewritten, every record read back as itself, and every text that is not one refused — another format, a
//! provider kind `/1` does not name, a requester of another shape, a binding out of order or twice. And the digest
//! of the elaborated program moves with an imported module, its `with` and its version, never with whitespace.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_plan_check::assignment::{
    program_digest, program_text, Assignment, Refusal, Requester, FORMAT,
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

/// The reference record: a placement, a task's binding and two of named requesters, written out of order.
fn reference() -> Assignment {
    let mut bindings = BTreeMap::new();
    bindings.insert(
        (
            Requester::Task {
                system: "app.rt".to_owned(),
                task: "sensor".to_owned(),
            },
            "time.monotonic".to_owned(),
        ),
        "platform.timer.timer.counter".to_owned(),
    );
    bindings.insert(
        (
            Requester::Named("clock.time.monotonic".to_owned()),
            "unambiguous-horizon".to_owned(),
        ),
        "platform.timer.timer.counter".to_owned(),
    );
    bindings.insert(
        (
            Requester::Named("clock.time.monotonic".to_owned()),
            "counter-width".to_owned(),
        ),
        "platform.timer.timer.counter".to_owned(),
    );
    Assignment {
        profile: "rt-static-up-v1".to_owned(),
        system: "app.rt".to_owned(),
        program: program_digest(&program(&library(32, "1 0", ""))),
        placement: Some(("platform.soc.bus".to_owned(), "soc.ram".to_owned())),
        bindings,
    }
}

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

#[test]
fn the_record_s_canonical_text_is_frozen_in_its_golden() {
    let actual = reference().to_canonical();
    let path = goldens().join(format!("{}.golden", FORMAT.replace('/', "-")));
    match fs::read_to_string(&path) {
        Ok(golden) => assert_eq!(
            actual, golden,
            "the text of `{FORMAT}` has changed. A golden is never rewritten: move `FORMAT` (and the register, \
             `docs/book/src/versions.md`) to a new identifier, and the search's constant with it"
        ),
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
fn every_record_reads_back_as_itself() {
    let record = reference();
    assert_eq!(Assignment::read(&record.to_canonical()), Ok(record.clone()));
    // Without a placement, as a system with no placement memory writes it; with whitespace and comments the reader's.
    let mut hosted = record.clone();
    hosted.placement = None;
    assert_eq!(Assignment::read(&hosted.to_canonical()), Ok(hosted));
    let spaced = record
        .to_canonical()
        .replace('\n', "\n\n  ; a comment, inert\n")
        .replace(" (", "   (");
    assert_eq!(Assignment::read(&spaced), Ok(record));
}

#[test]
fn every_text_that_is_no_record_of_the_format_is_refused() {
    let text = reference().to_canonical();
    let other = text.replace(FORMAT, "archogen-assignment/2");
    assert_eq!(
        Assignment::read(&other),
        Err(Refusal::Format(Some("archogen-assignment/2".to_owned())))
    );
    // A provider of a kind `/1` does not name — an adapter, a catalog record — is refused, never read as a name.
    let adapter = text.replace(
        "time.monotonic platform.timer.timer.counter)",
        "time.monotonic (adapter delay-to-deadline/1))",
    );
    assert_eq!(
        Assignment::read(&adapter),
        Err(Refusal::ProviderKind(
            "(adapter delay-to-deadline/1)".to_owned()
        ))
    );
    let requester = text.replace("(task app.rt sensor)", "(job app.rt sensor)");
    assert_eq!(
        Assignment::read(&requester),
        Err(Refusal::RequesterKind("(job app.rt sensor)".to_owned()))
    );
    // Out of order, and twice for one requester and fact.
    let lines: Vec<&str> = text.lines().collect();
    let swapped = [&lines[..5], &[lines[6], lines[5]], &lines[7..]]
        .concat()
        .join("\n");
    assert!(
        matches!(Assignment::read(&swapped), Err(Refusal::NotCanonical(_))),
        "{swapped}"
    );
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
    let short = text.replace(&reference().program, "abc");
    assert!(matches!(
        Assignment::read(&short),
        Err(Refusal::Value {
            clause: "program",
            ..
        })
    ));
    assert!(matches!(
        Assignment::read("(format \"archogen-assignment/1\""),
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
