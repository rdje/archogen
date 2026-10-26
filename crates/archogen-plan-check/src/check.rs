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
use eadl_front::{Program, SourceMap};
use eadl_model::profile::Profile;

use crate::assignment::{program_digest, Assignment};

/// Check (1)'s code: the record does not read, or names another profile or a system the description does not declare.
pub const PLAN_RECORD: &str = "plan-record";

/// The stale plan's code: the record's digest is not the elaborated program's.
pub const PLAN_STALE: &str = "plan-stale";

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
