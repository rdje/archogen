//! Check (2) (leaf `M3.5.4.3`): the description re-checked with `eadl-model`'s passes over the kind modules the
//! checker compiles in, so a description the toolchain refuses never has a plan accepted — the semantics corpus's own
//! cases (`docs/semantics/cases/`), each read as the root module of a program, judged as the toolchain judges them.

use std::path::Path;

use archogen_outcome::record::Severity;
use archogen_outcome::Status;
use archogen_plan_check::check::{description, status_of};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// A case of the semantics corpus, its declarations made the root module `app.case`.
fn case(name: &str) -> (SourceMap, Program) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/semantics/cases")
        .join(format!("{name}.eadl"));
    let text = std::fs::read_to_string(&path).expect("the case reads");
    let declarations: String = text
        .lines()
        .filter(|line| !line.starts_with(';') && !line.starts_with("(eadl-version"))
        .map(|line| format!("  {line}\n"))
        .collect();
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

#[test]
fn an_accepted_description_passes_check_two() {
    let (sources, program) = case("positive-minimal-system");
    assert_eq!(description(&sources, &program, &RT_STATIC_UP_V1), Ok(()));
}

#[test]
fn a_description_the_toolchain_refuses_has_its_own_status_and_diagnostics() {
    for (name, status) in [
        ("invalid-two-release-models", Status::InvalidDescription),
        ("missing-release-model", Status::MissingFact),
        ("unsupported-duplicate-priority", Status::UnsupportedProfile),
    ] {
        let (sources, program) = case(name);
        let verdict = description(&sources, &program, &RT_STATIC_UP_V1).unwrap_err();
        assert_eq!(verdict.status, status, "{name}: {verdict:?}");
        let error = verdict
            .findings
            .iter()
            .find(|f| f.severity == Severity::Error)
            .unwrap_or_else(|| panic!("{name}: no error finding: {verdict:?}"));
        assert!(
            !error.code.is_empty() && !error.repair.is_empty(),
            "{name}: {error:?}"
        );
        assert_eq!(error.primary.source, "app.case.eadl", "{name}: {error:?}");
        assert!(
            error.primary.line > 1,
            "{name}: the finding points into the description: {error:?}"
        );
    }
}

#[test]
fn the_checker_s_status_mapping_is_total_and_spells_each_verdict_as_the_vocabulary_does() {
    for verdict in eadl_front::Verdict::ALL {
        assert_eq!(status_of(*verdict).slug(), verdict.slug(), "{verdict:?}");
    }
}
