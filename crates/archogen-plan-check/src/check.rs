//! The checks the configuration checker runs on a plan's assignment record (leaf `M3.5.4`; `M3.5`'s design, *The
//! checks* and *Its verdicts*), each finding a code per check, a severity, a message, the record's or the
//! description's span and a repair — the result record's own shapes (`archogen_outcome::record`), so the executable
//! (`M3.5.6`) writes what the checks found and nothing else.
//!
//! ⭐ **A stale plan is the user's input mismatched, not the toolchain's**: a record whose digest is not the
//! elaborated program's is refused before any judgement, as the process status `stale-input` with the repair
//! "resolve again" (`ROADMAP.md` §10.3). A record that reads otherwise wrong is a corrupted plan, `tool-failure` —
//! "never reported as a valid system" (§5.5).

use archogen_outcome::record::{Finding, Label, Severity};
use archogen_outcome::Status;
use eadl_front::{Diagnostic, Program, SourceMap, Verdict as Concluded};
use eadl_model::check::{check_program, shipped_registry};
use eadl_model::profile::Profile;

use crate::assignment::{program_digest, Assignment};

/// Check (1)'s code: the record does not read, or names another profile or a system the description does not declare.
pub const PLAN_RECORD: &str = "plan-record";

/// The stale plan's code: the record's digest is not the elaborated program's.
pub const PLAN_STALE: &str = "plan-stale";

/// Check (2)'s code, where the checker itself refuses the description rather than one of the language's passes.
pub const PLAN_DESCRIPTION: &str = "plan-description";

/// The kind modules the checker compiles in: the language definition its re-check of the description reads, shared
/// with the generator as files and never as a package (`TI-H19`, as `M3.5.2` amends it); each `include_str!` site
/// admitted in `trust/roots.eadl` by a `defadmit` of its own (`RP-H3`).
const KIND_MODULES: &[(&str, &str)] = &[
    (
        "docs/semantics/kinds/core.eadl",
        include_str!("../../../docs/semantics/kinds/core.eadl"),
    ),
    (
        "docs/semantics/kinds/os-rt.eadl",
        include_str!("../../../docs/semantics/kinds/os-rt.eadl"),
    ),
];

/// What a check found against a plan: the status it refuses with, and every finding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verdict {
    /// The process status: `stale-input` for a stale plan, `tool-failure` for a corrupted one.
    pub status: Status,
    /// Every finding, each an error.
    pub findings: Vec<Finding>,
}

/// Check (1) (`M3.5.4.2`): the record reads, its digest is the elaborated program's — else a stale plan, refused
/// before any judgement — and it names the profile the check runs under and a system the description declares.
/// `name` names the record in each finding's span.
///
/// # Errors
///
/// The [`Verdict`] refusing it: `stale-input` and [`PLAN_STALE`] for a stale plan, `tool-failure` and
/// [`PLAN_RECORD`] for a record that does not read or names what the description and the check do not.
pub fn record(
    program: &Program,
    profile: &Profile,
    name: &str,
    text: &str,
) -> Result<Assignment, Verdict> {
    let refused = |status, line_of: &str, message: String, repair: &str| Verdict {
        status,
        findings: vec![Finding {
            code: if status == Status::StaleInput {
                PLAN_STALE
            } else {
                PLAN_RECORD
            }
            .to_owned(),
            severity: Severity::Error,
            message,
            primary: label(name, text, line_of),
            secondary: Vec::new(),
            repair: repair.to_owned(),
        }],
    };
    let assignment = Assignment::read(text).map_err(|refusal| {
        refused(
            Status::ToolFailure,
            "",
            refusal.to_string(),
            "write the plan again with `archogen resolve`: this record is not one it writes",
        )
    })?;
    let digest = program_digest(program);
    if assignment.program != digest {
        return Err(refused(
            Status::StaleInput,
            "program",
            format!(
                "the plan was written for another program: its digest is {}, the description's elaborated program's \
                 is {digest}",
                assignment.program
            ),
            "resolve again: the description, a module it imports, an import's `with` or a version changed after the \
             plan was written",
        ));
    }
    if assignment.profile != profile.id {
        return Err(refused(
            Status::ToolFailure,
            "profile",
            format!(
                "the plan names the profile `{}`, and it is checked under `{}`",
                assignment.profile, profile.id
            ),
            "check the plan under the profile it was resolved under, or resolve again under this one",
        ));
    }
    let declared = program
        .qualified_declarations()
        .into_iter()
        .any(|(qualified, form)| {
            form.head() == Some("defsystem") && qualified == assignment.system
        });
    if !declared {
        return Err(refused(
            Status::ToolFailure,
            "system",
            format!(
                "the plan names the system `{}`, which the description does not declare",
                assignment.system
            ),
            "resolve again: the plan is for a system of another description",
        ));
    }
    Ok(assignment)
}

/// Check (2) (`M3.5.4.3`): the description re-checked with `eadl-model`'s passes — schema, presence, refinement and
/// profile admission, the ones the toolchain's own check runs — over the kind modules the checker compiles in, then
/// the provider-level refusals of rules 3, 5 and 10 ([`crate::providers`]), so a description the toolchain refuses
/// never has a plan accepted. `sources` holds the description's files, as
/// elaborated into `program`.
///
/// # Errors
///
/// The [`Verdict`] refusing it: the description's own status and each of its diagnostics as a finding; `tool-failure`
/// if the checker's kind modules do not load.
pub fn description(
    sources: &SourceMap,
    program: &Program,
    profile: &Profile,
) -> Result<(), Verdict> {
    let mut all = sources.clone();
    let kinds: Vec<(String, String)> = KIND_MODULES
        .iter()
        .map(|(name, text)| ((*name).to_owned(), (*text).to_owned()))
        .collect();
    let registry = shipped_registry(&mut all, &kinds).map_err(|diagnostics| Verdict {
        status: Status::ToolFailure,
        findings: diagnostics.iter().map(|d| finding(&all, d)).collect(),
    })?;
    let outcome = check_program(program, &registry, profile);
    if outcome.is_ok() {
        // The provider-level refusals of rules 3, 5 and 10, which the language's passes do not run (`M3.5.4.6`).
        return crate::providers::refusals(sources, program);
    }
    Err(Verdict {
        status: status_of(outcome.verdict),
        findings: outcome
            .diagnostics
            .iter()
            .map(|d| finding(&all, d))
            .collect(),
    })
}

/// The status for a §5.5 verdict of the language's check: the checker's own mapping, as total as the vocabulary.
#[must_use]
pub const fn status_of(verdict: Concluded) -> Status {
    match verdict {
        Concluded::Ok => Status::Ok,
        Concluded::InvalidDescription => Status::InvalidDescription,
        Concluded::MissingFact => Status::MissingFact,
        Concluded::UnsupportedProfile => Status::UnsupportedProfile,
        Concluded::InfeasibleConfiguration => Status::InfeasibleConfiguration,
        Concluded::AnalysisInconclusive => Status::AnalysisInconclusive,
        Concluded::NotEstablished => Status::NotEstablished,
        Concluded::Counterexample => Status::Counterexample,
        Concluded::ToolFailure => Status::ToolFailure,
    }
}

/// A diagnostic of the language's check as a finding of the result record, each span by its file, line and column.
fn finding(sources: &SourceMap, diagnostic: &Diagnostic) -> Finding {
    let label = |label: &eadl_front::Label| {
        let source = sources.get(label.span.source);
        let position = source.map(|s| s.position(label.span.start));
        Label {
            source: source.map_or_else(String::new, |s| s.name.clone()),
            line: position.map_or(1, |p| p.line),
            column: position.map_or(1, |p| p.column),
            message: label.message.clone(),
        }
    };
    Finding {
        code: diagnostic.code.to_owned(),
        severity: match diagnostic.severity {
            eadl_front::Severity::Error => Severity::Error,
            eadl_front::Severity::Warning => Severity::Warning,
        },
        message: diagnostic.message.clone(),
        primary: label(&diagnostic.primary),
        secondary: diagnostic.secondary.iter().map(label).collect(),
        repair: diagnostic.repair.clone(),
    }
}

/// The record's span for a finding: the clause `head` opens, where it reads, else the record's first line.
fn label(name: &str, text: &str, head: &str) -> Label {
    let mut sources = SourceMap::new();
    let position = sources.add(name, text).ok().and_then(|id| {
        let (document, _) = eadl_front::read(&sources, id);
        let span = document
            .forms
            .iter()
            .find(|form| !head.is_empty() && form.head() == Some(head))?
            .span();
        Some(sources.get(id)?.position(span.start))
    });
    Label {
        source: name.to_owned(),
        line: position.map_or(1, |p| p.line),
        column: position.map_or(1, |p| p.column),
        message: if head.is_empty() {
            "the record".to_owned()
        } else {
            format!("its `{head}` clause")
        },
    }
}
