//! Check (1) (leaf `M3.5.4.2`): the record reads, its digest is the elaborated program's, and it names the profile
//! the check runs under and a system the description declares. A digest that is not the program's is a stale plan,
//! refused before any judgement as `stale-input`, whatever else is wrong; every other fault a corrupted plan,
//! `tool-failure`. Each refusal one finding at the record's clause.

use std::collections::BTreeMap;

use archogen_outcome::record::Severity;
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::{record, Verdict, PLAN_RECORD, PLAN_STALE};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// A system and the block it uses, the block's counter `width` bits wide.
fn program(width: u32) -> Program {
    let modules = MemoryModules::new().with(
        "app.system",
        &format!(
            "(defmodule app.system\n  (version 1 0)\n  (export app.rt)\n  \
             (defblock app.timer (offers (counter-width {width} bit)))\n  \
             (defsystem app.rt (requires (uses app.timer))))"
        ),
    );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app.system");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    program
}

/// The record a search would write for `program`.
fn written(program: &Program) -> Assignment {
    let mut bindings = BTreeMap::new();
    bindings.insert(
        (
            Requester::Named("app.rt".to_owned()),
            "counter-width".to_owned(),
        ),
        "app.timer".to_owned(),
    );
    Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "app.rt".to_owned(),
        program: program_digest(program),
        placement: None,
        bindings,
    }
}

/// The one finding a refusal carries: its status, code, line and message.
fn refusal(verdict: Verdict) -> (Status, String, u32, String) {
    assert_eq!(verdict.findings.len(), 1, "{verdict:?}");
    let finding = &verdict.findings[0];
    assert_eq!(finding.severity, Severity::Error);
    assert!(!finding.repair.is_empty());
    assert_eq!(finding.primary.source, "plan.eadl");
    (
        verdict.status,
        finding.code.clone(),
        finding.primary.line,
        finding.message.clone(),
    )
}

fn check(program: &Program, text: &str) -> Result<Assignment, Verdict> {
    record(program, &RT_STATIC_UP_V1, "plan.eadl", text)
}

#[test]
fn a_record_of_the_program_under_its_profile_and_system_reads() {
    let program = program(32);
    let record = written(&program);
    assert_eq!(check(&program, &record.to_canonical()), Ok(record));
}

#[test]
fn a_plan_for_another_program_is_stale_before_any_judgement() {
    let stale = written(&program(16));
    let (status, code, line, message) =
        refusal(check(&program(32), &stale.to_canonical()).unwrap_err());
    assert_eq!(
        (status, code.as_str(), line),
        (Status::StaleInput, PLAN_STALE, 4)
    );
    assert!(message.contains(&stale.program), "{message}");
    // Stale before anything else: another profile and an undeclared system beside it are not judged.
    let mut worse = stale;
    worse.profile = "rt-fantasy-v9".to_owned();
    worse.system = "app.nowhere".to_owned();
    let (status, code, ..) = refusal(check(&program(32), &worse.to_canonical()).unwrap_err());
    assert_eq!((status, code.as_str()), (Status::StaleInput, PLAN_STALE));
}

#[test]
fn a_record_naming_another_profile_or_no_system_of_the_description_is_corrupted() {
    let program = program(32);
    let mut profile = written(&program);
    profile.profile = "rt-fantasy-v9".to_owned();
    let (status, code, line, message) =
        refusal(check(&program, &profile.to_canonical()).unwrap_err());
    assert_eq!(
        (status, code.as_str(), line),
        (Status::ToolFailure, PLAN_RECORD, 2)
    );
    assert!(
        message.contains("rt-fantasy-v9") && message.contains(RT_STATIC_UP_V1.id),
        "{message}"
    );
    // A system the description does not declare, and a declaration that is no system.
    for system in ["app.nowhere", "app.timer"] {
        let mut other = written(&program);
        other.system = system.to_owned();
        let (status, code, line, message) =
            refusal(check(&program, &other.to_canonical()).unwrap_err());
        assert_eq!(
            (status, code.as_str(), line),
            (Status::ToolFailure, PLAN_RECORD, 3),
            "{system}"
        );
        assert!(message.contains(system), "{message}");
    }
}

#[test]
fn a_record_that_does_not_read_is_corrupted() {
    let program = program(32);
    for text in [
        "(format \"archogen-assignment/9\")\n",
        "not a record at all (",
        "",
    ] {
        let (status, code, line, _) = refusal(check(&program, text).unwrap_err());
        assert_eq!(
            (status, code.as_str(), line),
            (Status::ToolFailure, PLAN_RECORD, 1),
            "{text:?}"
        );
    }
}
