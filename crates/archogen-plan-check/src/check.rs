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
use eadl_front::module::{elaborate_source, Instance, ModuleSource, Version};
use eadl_front::{Diagnostic, Program, SourceId, SourceMap, Span, Verdict as Concluded};
use eadl_model::check::{check, check_program, shipped_registry};
use eadl_model::kind::Registry;
use eadl_model::profile::Profile;

use crate::assignment::{program_digest, Assignment};

/// Check (1)'s code: the record does not read, or names another profile or a system the description does not declare.
pub const PLAN_RECORD: &str = "plan-record";

/// The stale plan's code: the record's digest is not the elaborated program's.
pub const PLAN_STALE: &str = "plan-stale";

/// Check (2)'s code, where the checker itself refuses the description rather than one of the language's passes.
pub const PLAN_DESCRIPTION: &str = "plan-description";

/// Check (3)'s code: a requirement that binds left unbound, a binding of no requirement or of a requester the system
/// does not serve, or a provider that is none of the description's blocks or platforms.
pub const PLAN_BINDING: &str = "plan-binding";

/// Check (4)'s code: a binding the model's judgement does not satisfy.
pub const PLAN_UNSATISFIED: &str = "plan-unsatisfied";

/// Check (5)'s code: the bindings of one assignment do not hold together by `docs/semantics/model.md` §8's joint rules.
pub const PLAN_JOINT: &str = "plan-joint";

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
    let (all, registry) = registry(sources)?;
    let outcome = check_program(program, &registry, profile);
    judged(sources, program, &all, &outcome)
}

/// The language definition the checker compiles in, loaded beside the description's files: the sources it adds, and
/// the registry of kinds.
fn registry(sources: &SourceMap) -> Result<(SourceMap, Registry), Verdict> {
    let mut all = sources.clone();
    let kinds: Vec<(String, String)> = KIND_MODULES
        .iter()
        .map(|(name, text)| ((*name).to_owned(), (*text).to_owned()))
        .collect();
    let registry = shipped_registry(&mut all, &kinds).map_err(|diagnostics| Verdict {
        status: Status::ToolFailure,
        findings: diagnostics.iter().map(|d| finding(&all, d)).collect(),
    })?;
    Ok((all, registry))
}

/// Check (2)'s verdict from the language's `outcome`: its own status and diagnostics where it refuses, else the
/// provider-level refusals of rules 3, 5 and 10, which the language's passes do not run (`M3.5.4.6`).
fn judged(
    sources: &SourceMap,
    program: &Program,
    all: &SourceMap,
    outcome: &eadl_model::check::Outcome,
) -> Result<(), Verdict> {
    if outcome.is_ok() {
        return crate::providers::refusals(sources, program);
    }
    Err(Verdict {
        status: status_of(outcome.verdict),
        findings: outcome
            .diagnostics
            .iter()
            .map(|d| finding(all, d))
            .collect(),
    })
}

/// The module a plain description's one root instance names: no module name is `-` (`docs/semantics/reference.md` §6
/// rule 8), so no module's instance writes the same line of the digest.
pub const PLAIN: &str = "-";

/// A plain description — a file with no `defmodule` among its forms — as one root instance (`M3.5.6`'s decision): no
/// module, version `0.0`, the declarations the language's check read, in source order; so the record's digest is
/// defined for it, `program_text` writing `instance . - 0.0`.
#[must_use]
pub fn plain(declarations: Vec<eadl_front::Form>) -> Program {
    // Built outside `vec!`: the trust inventory's catalog refuses the word `path` inside a macro's arguments.
    let root = Instance {
        id: 0,
        module: PLAIN.to_owned(),
        version: Version { major: 0, minor: 0 },
        path: String::new(),
        alias: String::new(),
        exports: Vec::new(),
        bindings: Vec::new(),
        imports: Vec::new(),
        declarations,
    };
    Program {
        instances: vec![root],
    }
}

/// Every check over one plan whose description is the file `id` of `sources`, as the executable reads one
/// (`M3.5.6.1`): a module elaborated, its imports found through `modules`; a plain description read by the language's
/// own check and taken as one root instance ([`plain`]). A description that does not read or does not elaborate is
/// refused under its own status before the plan is judged, since no program exists for the plan's digest to name;
/// otherwise check (1) first — a stale plan before any judgement — then checks (2) to (5), as [`plan`] runs them.
/// `text` is the record `name` names. Returns the program the description became, as far as it did — the result
/// record's subject — with the verdict.
pub fn plan_file(
    sources: &mut SourceMap,
    id: SourceId,
    modules: &dyn ModuleSource,
    profile: &Profile,
    name: &str,
    text: &str,
) -> (Program, Result<(), Verdict>) {
    let module = eadl_front::read(sources, id)
        .0
        .forms
        .iter()
        .any(|form| form.head() == Some("defmodule"));
    if module {
        let (program, diagnostics) = elaborate_source(sources, modules, id);
        if diagnostics.has_errors() {
            let verdict = diagnostics
                .items()
                .iter()
                .map(|d| Concluded::of_code(d.code))
                .max_by_key(|v| v.precedence())
                .unwrap_or(Concluded::InvalidDescription);
            let refused = Verdict {
                status: status_of(verdict),
                findings: diagnostics
                    .items()
                    .iter()
                    .map(|d| finding(sources, d))
                    .collect(),
            };
            return (program, Err(refused));
        }
        let verdict = plan(sources, &program, profile, name, text);
        return (program, verdict);
    }
    let (all, registry) = match registry(sources) {
        Ok(loaded) => loaded,
        Err(refused) => return (plain(Vec::new()), Err(refused)),
    };
    let outcome = check(&all, id, &registry, profile);
    let program = plain(outcome.declarations.clone());
    if eadl_front::read(sources, id).1.has_errors() {
        let verdict = judged(sources, &program, &all, &outcome);
        return (program, verdict);
    }
    let verdict = (|| {
        let assignment = record(&program, profile, name, text)?;
        judged(sources, &program, &all, &outcome)?;
        bindings(sources, &program, name, text, &assignment)?;
        joint(sources, &program, profile, name, text, &assignment)
    })();
    (program, verdict)
}

/// Checks (3) and (4) (`M3.5.4.4`; [`crate::bindings`]): each requirement that binds is bound once, each binding is
/// one requirement's, of a requester the system serves, to one of the description's blocks or platforms; and each
/// binding is satisfied, by the model's judgement. `assignment` is what check (1) read from `text`, the record `name`
/// names; `sources` holds the description's files, as elaborated into `program`.
///
/// # Errors
///
/// The [`Verdict`] refusing it: `tool-failure`, a finding per mismatch under [`PLAN_BINDING`] or [`PLAN_UNSATISFIED`];
/// or the description's status under [`PLAN_DESCRIPTION`] where the model refuses what a requester requires or a bound
/// provider offers.
pub fn bindings(
    sources: &SourceMap,
    program: &Program,
    name: &str,
    text: &str,
    assignment: &Assignment,
) -> Result<(), Verdict> {
    crate::bindings::check(sources, program, name, text, assignment)
}

/// Every check over one plan, in order (`M3.5`'s design, *Its verdicts*; `M3.5.4.5`): check (1), a stale plan
/// refused before any judgement; check (2), the description; checks (3) and (4), the bindings; check (5), the joint
/// rules (`M3.5.5`) — the first that refuses the verdict. `text` is the record `name` names; `sources` holds the description's files, as elaborated into
/// `program`.
///
/// # Errors
///
/// The [`Verdict`] of the first check that refuses.
pub fn plan(
    sources: &SourceMap,
    program: &Program,
    profile: &Profile,
    name: &str,
    text: &str,
) -> Result<(), Verdict> {
    let assignment = record(program, profile, name, text)?;
    description(sources, program, profile)?;
    bindings(sources, program, name, text, &assignment)?;
    joint(sources, program, profile, name, text, &assignment)
}

/// Check (5) (`M3.5.5`; [`crate::joint`]): the bindings of one system's assignment hold together by
/// `docs/semantics/model.md` §8's joint rules, judged after checks (3) and (4) accept each binding alone, the system's
/// level the `profile`'s and never the record's. `assignment` is what check (1) read from `text`, the record `name`
/// names; `sources` holds the description's files, as elaborated into `program`.
///
/// # Errors
///
/// The [`Verdict`] refusing it: `tool-failure`, a finding per conflict or unknown under [`PLAN_JOINT`]; or, judged
/// first, `unsupported-profile` under [`PLAN_DESCRIPTION`] where a caller states levels that do not hold the profile's.
pub fn joint(
    sources: &SourceMap,
    program: &Program,
    profile: &Profile,
    name: &str,
    text: &str,
    assignment: &Assignment,
) -> Result<(), Verdict> {
    crate::joint::check(sources, program, profile, name, text, assignment)
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

/// A finding's place in the description: `span`'s file, line and column.
pub(crate) fn at(sources: &SourceMap, span: Span, message: &str) -> Label {
    let source = sources.get(span.source);
    let position = source.map(|s| s.position(span.start));
    Label {
        source: source.map_or_else(String::new, |s| s.name.clone()),
        line: position.map_or(1, |p| p.line),
        column: position.map_or(1, |p| p.column),
        message: message.to_owned(),
    }
}

/// The record's span for a finding: the clause `head` opens, where it reads, else the record's first line.
pub(crate) fn label(name: &str, text: &str, head: &str) -> Label {
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
