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

use crate::assignment::{program_digest, Assignment, FORMAT, FORMAT_3};
use crate::target::{on_target, Target};

/// Check (1)'s code: the record does not read — another format, `archogen-assignment/1` among them, another adapter
/// catalog, a kind the checker's table does not hold, an instance of another shape — or names another profile or a
/// system the description does not declare.
pub const PLAN_RECORD: &str = "plan-record";

/// The stale plan's code: the record's digest is not the elaborated program's.
pub const PLAN_STALE: &str = "plan-stale";

/// The code of a refusal of the description by the checker itself rather than one of the language's passes: check (2)'s,
/// and what checks (3) to (5) find that §8 makes the description's whatever the assignment.
pub const PLAN_DESCRIPTION: &str = "plan-description";

/// Check (3)'s code: a requirement that binds left unbound, a binding of no requirement or of a requester the system
/// does not serve, or a provider that is none of the description's blocks or platforms — an engine adapter's instance
/// among them where it programs none, or is not the requester's own, or binds beneath what its kind does not require,
/// elsewhere than the provider it programs.
pub const PLAN_BINDING: &str = "plan-binding";

/// Check (4)'s code: a binding the model's judgement does not satisfy — at an instance, at what it offers over the
/// device it programs, and only for a requester the mediation gate admits.
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
    let refused = |status, line_of: &str, message: String, repair: &str| {
        refused(name, text, status, line_of, message, repair)
    };
    let assignment = read_record(name, text)?;
    // A plan of a system on a target was resolved for another run than one given no target, and nothing failed
    // (`docs/decisions/decision_runtime-plan-format-2.md` §1): a record of other inputs, before any judgement. A plan on
    // its target is judged with the target named, by `plan_file_on_target` and `plan_on_target`.
    if let Some((platform, _)) = &assignment.target {
        return Err(refused(
            Status::StaleInput,
            "target",
            format!(
                "the plan is `{FORMAT_3}`, resolved on the target `{platform}`, and no target is given: name that target, \
                 or resolve again on none"
            ),
            "name the target the plan was resolved on, `--target FILE` to the executable or the MCP tool's `target` \
             argument, to judge it there; or resolve again without a target",
        ));
    }
    of_these_inputs(program, profile, name, text, assignment)
}

/// Check (1) of a plan of a system on `target` (`docs/decisions/decision_runtime-plan-format-2.md` §1, "The target in
/// the identity" and "The checker", as amended `2026-11-15` by leaf `M4.1.7.5.6`; leaf `M4.1.7.5.4.3`): the record reads
/// and is `archogen-assignment/3`; it was resolved for these inputs — its `(program …)` the description's elaborated
/// program's digest, as with no target, and the `HEX` of its `(target NAME "HEX")` [`Target::digest`], which decides
/// which run it is of — else a record of other inputs, refused before any judgement; and it names the profile the check
/// runs under, a system the description declares and, as its `NAME`, the one platform of the file whose digest it writes
/// ([`misnamed`]). `name` names the record in each finding's span.
///
/// # Errors
///
/// The [`Verdict`] refusing it: `stale-input` and [`PLAN_STALE`] for a `/2` record, of no target, for a record of
/// another target's digest — whatever platform it names — and for a stale program, "resolve again"; `tool-failure` and
/// [`PLAN_RECORD`] for a record that does not read or names what the description, the check and the target do not,
/// another platform beside the given file's digest among them.
pub fn record_on_target(
    program: &Program,
    target: &Target,
    profile: &Profile,
    name: &str,
    text: &str,
) -> Result<Assignment, Verdict> {
    let assignment = read_record(name, text)?;
    of_the_target(name, text, &assignment, &target.digest(), &target.platform)?;
    let assignment = of_these_inputs(program, profile, name, text, assignment)?;
    misnamed(name, text, &assignment, target)?;
    Ok(assignment)
}

/// A `/3` record naming as its `NAME` another platform than `target`'s, the file whose digest it writes
/// (`docs/decisions/decision_runtime-plan-format-2.md` §1, amended `2026-11-15`, leaf `M4.1.7.5.6`): since that file's
/// forms name one platform, a record no run writes — `tool-failure` and [`PLAN_RECORD`], as a record naming another
/// profile or system is, judged once the file has read as a target. The one statement of the refusal, for
/// [`record_on_target`] and [`plan_file_on_target`] alike.
fn misnamed(
    name: &str,
    text: &str,
    assignment: &Assignment,
    target: &Target,
) -> Result<(), Verdict> {
    match &assignment.target {
        Some((platform, _)) if *platform != target.platform => Err(refused(
            name,
            text,
            Status::ToolFailure,
            "target",
            format!(
                "the plan names the target `{platform}` beside the digest of a file whose one platform is `{}`",
                target.platform
            ),
            "write the plan again with `archogen resolve`: this record is not one it writes",
        )),
        _ => Ok(()),
    }
}

/// Check (1)'s target (`docs/decisions/decision_runtime-plan-format-2.md` §1, amended `2026-11-15`, leaf `M4.1.7.5.6`):
/// the record is `archogen-assignment/3` and its digest is `digest`, the given file's — the digest deciding which run it
/// is of, whatever platform it names — else a record of other inputs, `stale-input` and [`PLAN_STALE`]. `shown` names the
/// target given in the message.
fn of_the_target(
    name: &str,
    text: &str,
    assignment: &Assignment,
    digest: &str,
    shown: &str,
) -> Result<(), Verdict> {
    let refused = |line_of: &str, message: String| {
        refused(
            name,
            text,
            Status::StaleInput,
            line_of,
            message,
            "resolve again: the plan was resolved for another run, and nothing failed",
        )
    };
    match &assignment.target {
        None => Err(refused(
            "format",
            format!(
                "the plan is `{FORMAT}`, resolved on no target, and the target `{shown}` is given: resolve again on it"
            ),
        )),
        Some((_, written)) if written != digest => Err(refused(
            "target",
            format!(
                "the plan was resolved on another file of a target than `{shown}`: its digest is {written}, the given \
                 file's is {digest}"
            ),
        )),
        Some(_) => Ok(()),
    }
}

/// A refusal of check (1), one finding at the record's clause `line_of`: [`PLAN_STALE`] for `stale-input`, else
/// [`PLAN_RECORD`].
fn refused(
    name: &str,
    text: &str,
    status: Status,
    line_of: &str,
    message: String,
    repair: &str,
) -> Verdict {
    Verdict {
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
    }
}

/// The record `text`, read: a record that does not read is a corrupted plan, `tool-failure` and [`PLAN_RECORD`].
fn read_record(name: &str, text: &str) -> Result<Assignment, Verdict> {
    Assignment::read(text).map_err(|refusal| {
        refused(
            name,
            text,
            Status::ToolFailure,
            "",
            refusal.to_string(),
            "write the plan again with `archogen resolve`: this record is not one it writes",
        )
    })
}

/// Check (1)'s rest, of a record of the right kind: its digest is the elaborated program's, else a stale plan; and it
/// names the profile the check runs under and a system the description declares.
fn of_these_inputs(
    program: &Program,
    profile: &Profile,
    name: &str,
    text: &str,
    assignment: Assignment,
) -> Result<Assignment, Verdict> {
    let refused = |status, line_of: &str, message: String, repair: &str| {
        refused(name, text, status, line_of, message, repair)
    };
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
        bound(sources, &program, profile, name, text, &assignment)
    })();
    (program, verdict)
}

/// Every check over one plan of a system on a target, as the executable and the MCP tool read one given a target (leaf
/// `M4.1.7.5.4.4`; `DC-H2`): the description the file `id` of `sources` holds, read as [`plan_file`] reads it, and the
/// target's file `target`, both in `sources`. The refusals come in the order
/// `docs/decisions/decision_runtime-plan-format-2.md` §1's "The checker" gives: a file that does not read — the
/// description's, which does not read or does not elaborate, then the target's, which does not read as forms — each
/// under its own status; then a record of other inputs, `stale-input` ([`record_on_target`]'s, the target compared by its
/// digest alone, which its forms decide and which decides which run the record is of, so before the file is read as a
/// target); then, in the judgement, the target's own refusals ([`Target::of`]), a record naming another platform beside
/// the file's own digest — a record no run writes, `tool-failure`, as [`record_on_target`] refuses it too (amended
/// `2026-11-15`, leaf `M4.1.7.5.6`) — the system put on the target ([`on_target`]), and checks (2) to (5) as
/// [`plan_on_target`] runs them. `text` is
/// the record `name` names. Returns the program the description became, as far as it did — the result record's subject
/// — with the verdict.
pub fn plan_file_on_target(
    sources: &mut SourceMap,
    id: SourceId,
    target: SourceId,
    modules: &dyn ModuleSource,
    profile: &Profile,
    name: &str,
    text: &str,
) -> (Program, Result<(), Verdict>) {
    let refused_by = |sources: &SourceMap, diagnostics: &eadl_front::Diagnostics| {
        let verdict = diagnostics
            .items()
            .iter()
            .map(|d| Concluded::of_code(d.code))
            .max_by_key(|v| v.precedence())
            .unwrap_or(Concluded::InvalidDescription);
        Verdict {
            status: status_of(verdict),
            findings: diagnostics
                .items()
                .iter()
                .map(|d| finding(sources, d))
                .collect(),
        }
    };
    let (document, read) = eadl_front::read(sources, id);
    let program = if document
        .forms
        .iter()
        .any(|form| form.head() == Some("defmodule"))
    {
        let (program, diagnostics) = elaborate_source(sources, modules, id);
        if diagnostics.has_errors() {
            let refused = refused_by(sources, &diagnostics);
            return (program, Err(refused));
        }
        program
    } else {
        let program = plain(document.declarations().cloned().collect());
        if read.has_errors() {
            return (program, Err(refused_by(sources, &read)));
        }
        program
    };
    let sources: &SourceMap = sources;
    let verdict = (|| {
        let forms = crate::target::forms(sources, target)?;
        let shown = sources
            .get(target)
            .map_or_else(String::new, |source| eadl_front::fit_name(&source.name));
        let assignment = read_record(name, text)?;
        of_the_target(
            name,
            text,
            &assignment,
            &crate::target::digest(&forms),
            &shown,
        )?;
        let assignment = of_these_inputs(&program, profile, name, text, assignment)?;
        let target = Target::of(sources, target, forms)?;
        misnamed(name, text, &assignment, &target)?;
        let on = on_target(sources, &program, &assignment.system, &target)?;
        description(sources, &on.program, profile)?;
        bound(sources, &on.program, profile, name, text, &assignment)
    })();
    (program, verdict)
}

/// Checks (3) and (4) (`M3.5.4.4`; [`crate::bindings`]): each requirement that binds is bound once, each binding is
/// one requirement's, of a requester the system serves, to one of the description's blocks or platforms or an engine
/// adapter's instance programming one, whose requirements beneath are bound to it; and each binding is satisfied, by the
/// model's judgement. `assignment` is what check (1) read from `text`, the record `name`
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
/// rules (`M3.5.5`) — the first that refuses the verdict; but a service or a policy of the plan's system naming hardware
/// outside it is the description's whatever the assignment, and is said before checks (3) and (4) (model §8 rule 7).
/// `text` is the record `name` names; `sources` holds the description's files, as elaborated into `program`.
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
    bound(sources, program, profile, name, text, &assignment)
}

/// Every check over one plan of a system on `target` (`docs/decisions/decision_runtime-plan-format-2.md` §1, "The
/// checker"; `DC-H2`; leaf `M4.1.7.5.4.3`), in the order the checker reads a plan in: check (1) for
/// `archogen-assignment/3` ([`record_on_target`]), a record of other inputs refused before any judgement; then the plan's
/// system put on the target ([`on_target`]), whose refusals are the target's own, in the judgement; then checks (2) to
/// (5), as [`plan`] runs them, over what is read there ([`crate::target::OnTarget::program`]) — the release requester
/// among the requesters checks (3) to (5) judge (`docs/decisions/decision_release-alarm.md` §1). The description's own
/// program is what the record's digest names. `sources` holds the description's files and the target's; `text` is the
/// record `name` names.
///
/// # Errors
///
/// The [`Verdict`] of the first check that refuses.
pub fn plan_on_target(
    sources: &SourceMap,
    program: &Program,
    target: &Target,
    profile: &Profile,
    name: &str,
    text: &str,
) -> Result<(), Verdict> {
    let assignment = record_on_target(program, target, profile, name, text)?;
    let on = on_target(sources, program, &assignment.system, target)?;
    description(sources, &on.program, profile)?;
    bound(sources, &on.program, profile, name, text, &assignment)
}

/// Checks (3) to (5) over a plan checks (1) and (2) accepted. A service or a policy of the plan's system naming hardware
/// outside it is a conflict of that system whatever the assignment (`docs/semantics/model.md` §8 rule 7), so no finding
/// of the bindings stands before it: check (5), which says the description's verdict first and whole, runs alone. It is
/// no refusal of check (2), which judges every system's description and has no plan's system to judge this one for.
fn bound(
    sources: &SourceMap,
    program: &Program,
    profile: &Profile,
    name: &str,
    text: &str,
    assignment: &Assignment,
) -> Result<(), Verdict> {
    if !crate::joint::names_outside_hardware(program, &assignment.system) {
        bindings(sources, program, name, text, assignment)?;
    }
    joint(sources, program, profile, name, text, assignment)
}

/// Check (5) (`M3.5.5`; [`crate::joint`]): the bindings of one system's assignment hold together by
/// `docs/semantics/model.md` §8's joint rules, judged after checks (3) and (4) accept each binding alone — or in their
/// place, where [`plan`] finds rule 7's name of the system — the system's level the `profile`'s and never the record's.
/// `assignment` is what check (1) read from `text`, the record `name` names; `sources` holds the description's files, as
/// elaborated into `program`.
///
/// # Errors
///
/// The [`Verdict`] refusing it: `tool-failure`, a finding per conflict or unknown under [`PLAN_JOINT`]; or, judged
/// first, the description's own status under [`PLAN_DESCRIPTION`] — `unsupported-profile` where a caller states levels
/// that do not hold the profile's, `infeasible-configuration` where a service or a policy the system serves names a
/// block or platform outside its hardware.
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
pub(crate) fn finding(sources: &SourceMap, diagnostic: &Diagnostic) -> Finding {
    let label = |label: &eadl_front::Label| {
        let source = sources.get(label.span.source);
        let position = source.map(|s| s.position(label.span.start));
        Label {
            source: source.map_or_else(String::new, |s| eadl_front::fit_name(&s.name)),
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
        source: source.map_or_else(String::new, |s| eadl_front::fit_name(&s.name)),
        line: position.map_or(1, |p| p.line),
        column: position.map_or(1, |p| p.column),
        message: message.to_owned(),
    }
}

/// The record's span for a finding: the clause `head` opens, where it reads, else the record's first line.
pub(crate) fn label(name: &str, text: &str, head: &str) -> Label {
    let mut sources = SourceMap::new();
    let position = sources.add(name, text).ok().and_then(|id| {
        let (document, _) = eadl_front::read_record(&sources, id);
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
