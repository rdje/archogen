//! `archogen check` — elaborate and type-check a description against a profile.
//!
//! The first command of the `ROADMAP.md` §10.2 surface to become real. It runs the frontend
//! pipeline (`eadl_model::check`) and maps its §5.5 verdict to this process's exit code through
//! [`Status::from_verdict`], so the number a script branches on and the word a human reads come
//! from the same place.
//!
//! A description is either one file or a module tree, and [`frontend`] is where the two meet: a file
//! [`is_module_file`] recognises is elaborated from its module path (`docs/semantics/reference.md` §6 rule
//! 7), its names are resolved (rules 9 and 10), and its declarations go through the passes a single file
//! gets. Both commands call it, so they cannot disagree about which file is which or what it means.

use std::io::Write;
use std::path::Path;

use eadl_front::{
    elaborate_source, read, DirectoryModules, ModuleSource, Position, SourceId, SourceMap, Verdict,
};
use eadl_model::check::{check, check_program, shipped_registry, Outcome};
use eadl_model::kind::Registry;
use eadl_model::profile::{self, Profile};

use crate::cli::Parsed;
use crate::status::Status;

/// The closure boundary as report lines: what is inside and what pulled each fact in, then what is outside.
pub fn closure_report(closure: &eadl_model::check::Closure) -> Vec<String> {
    let inside: Vec<String> = closure
        .inside
        .iter()
        .map(|(fact, by)| match by {
            None => format!("{fact} (requested)"),
            Some(owner) => format!("{fact} (needed by {owner})"),
        })
        .collect();
    vec![
        if inside.is_empty() {
            "closure: nothing — this description requests nothing".to_string()
        } else {
            format!("closure: {}", inside.join(", "))
        },
        if closure.outside.is_empty() {
            "outside the closure: nothing".to_string()
        } else {
            format!(
                "outside the closure, needed by nothing requested: {}",
                closure.outside.join(", ")
            )
        },
    ]
}

/// Run `archogen check`.
pub fn run(parsed: &Parsed, out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let path = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");

    // Resolve the profile before reading anything else: a description checked against a profile
    // nobody supports has not been checked.
    let requested = parsed.value("profile").unwrap_or("rt-static-up-v1");
    let Some(active) = profile::supported(requested) else {
        let known: Vec<&str> = profile::SUPPORTED.iter().map(|p| p.id).collect();
        let _ = writeln!(
            err,
            "archogen: {}: `{requested}` is not a supported profile",
            Status::UnsupportedProfile.slug()
        );
        let _ = writeln!(err, "  hint: this build supports {}", known.join(", "));
        return Status::UnsupportedProfile;
    };

    // Only now is the description read. A description checked against a profile nobody supports
    // has not been checked, so the profile is resolved first — and an unreadable file reported
    // after an unsupported profile would send the author to fix the wrong thing.
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            let _ = writeln!(
                err,
                "archogen: {}: cannot read {path}: {error}",
                Status::Usage.slug()
            );
            let _ = writeln!(err, "  hint: give the path of an eADL description");
            return Status::Usage;
        }
    };
    let mut sources = SourceMap::new();
    let registry = match shipped_registry(&mut sources, &embedded_modules()) {
        Ok(registry) => registry,
        Err(diagnostics) => {
            // A broken language definition is a fault in the toolchain, not a verdict about the
            // user's description. §5.5: never reported as a valid system.
            let _ = writeln!(
                err,
                "archogen: {}: the shipped kind modules could not be loaded",
                Status::ToolFailure.slug()
            );
            for diagnostic in &diagnostics {
                let _ = write!(err, "{}", diagnostic.render(&sources));
            }
            return Status::ToolFailure;
        }
    };

    let Ok(id) = sources.add(path.clone(), text) else {
        let _ = writeln!(
            err,
            "archogen: {}: {path} is too large to address",
            Status::ToolFailure.slug()
        );
        return Status::ToolFailure;
    };

    let (outcome, instances) = match frontend(&mut sources, id, path, &registry, active, err) {
        Frontend::Failed(status) => return status,
        Frontend::Checked { outcome, instances } => (outcome, instances),
    };
    let status = Status::from_verdict(outcome.verdict);

    if outcome.is_ok() {
        let _ = writeln!(
            out,
            "{path}: accepted against profile `{}` ({} declaration(s))",
            active.id,
            outcome.declarations.len()
        );
        if let Some(instances) = &instances {
            let _ = writeln!(
                out,
                "  elaborated from {} instance(s): {}",
                instances.len(),
                instances.join(", ")
            );
        }
        // §5.3's second half (leaf `M1.30`): the closure the description was judged over, what pulled each
        // fact in, and what lies outside it. Printed after the verdict and never as a diagnostic, so §4 rule 1
        // — no warning, no note — still holds, and none of it can fail the check.
        for line in closure_report(&outcome.closure) {
            let _ = writeln!(out, "  {line}");
        }
        // ⚠️ Deliberately modest wording. Acceptance means the description is well-formed, in
        // profile, and internally consistent — it is not a statement that any system built from
        // it will behave. §7.1's evidence categories start after this point.
        let _ = writeln!(
            out,
            "  this checks the description, not a system: no resolution, generation or analysis has run"
        );
        return status;
    }

    let _ = write!(err, "{}", outcome.render(&sources));
    let _ = writeln!(
        err,
        "archogen: {}: {} diagnostic(s) in {path}",
        status.slug(),
        outcome.diagnostics.len()
    );
    status
}

/// Whether the description at `id` is a module file, in the sense of `docs/semantics/reference.md` §6.
///
/// A file is a module when **any** of its declarations is a `(defmodule …)`, not only the first. §6 makes a
/// module file hold exactly one; a file holding one beside other declarations is still a module file — a
/// malformed one, `module-multiple-forms`, which the elaborator reports — and classifying by the first
/// declaration alone would hand that file to the schema pass to be told `defmodule` is not a kind.
/// Declarations and not forms, because the language-version identifier may precede the module (§8) and is
/// not one.
///
/// A file that does not **read** is not classified at all, so the read pass reports it: a syntax error is a
/// verdict about the bytes, whatever they were meant to be.
#[must_use]
pub fn is_module_file(sources: &SourceMap, id: SourceId) -> bool {
    let (document, diagnostics) = read(sources, id);
    !diagnostics.has_errors()
        && document
            .declarations()
            .any(|form| form.head() == Some("defmodule"))
}

/// The leaf that loads a kind module a user writes (`ROADMAP.md` §5.6 rule 1): extension experiment 1, a new
/// kind built from the existing declaration constructs.
pub const KIND_MODULE_OWNER: &str = "M6.3";

/// The first `(defkind …)` of a file that declares one: the kind it names, and where.
#[derive(Debug)]
pub struct KindModule {
    /// The kind's head symbol, when the declaration names one.
    pub name: Option<String>,
    /// Where the `(defkind …)` begins.
    pub position: Position,
}

/// The first kind the description at `id` declares, if it declares any (leaf `M1.32`).
///
/// A file is a kind module when **any** of its declarations is a `(defkind …)`, by the rule
/// [`is_module_file`] uses and for its reason: classifying by the first declaration alone would hand a
/// file that declares a kind beside other declarations to the schema pass, to be told `defkind` is not a
/// kind. A file that does not read is not classified, so the read pass reports it.
#[must_use]
pub fn kind_module(sources: &SourceMap, id: SourceId) -> Option<KindModule> {
    let (document, diagnostics) = read(sources, id);
    if diagnostics.has_errors() {
        return None;
    }
    let form = document
        .declarations()
        .find(|form| form.head() == Some("defkind"))?;
    let source = sources.get(id)?;
    Some(KindModule {
        name: form
            .items()
            .get(1)
            .and_then(|name| name.as_symbol())
            .map(str::to_string),
        position: source.position(form.span().start),
    })
}

/// Refuse a kind module as what it is: the toolchain loads only the kind modules it ships (leaf `M1.32`).
///
/// ⛔ Not a verdict. Before this, `check` read the file as a description and answered
/// `invalid-description` — `defkind` is not a known kind — which is a statement about a system the tool never
/// read. Nor is it validated alone: a kind that redefines a shipped one is well-formed by itself and clashes
/// only when both are loaded, so a standalone "ok" would be one the tool cannot stand behind.
pub fn refuse_kind_module(err: &mut dyn Write, path: &str, kind: &KindModule) -> Status {
    let declared = kind.name.as_deref().map_or_else(
        || "(defkind …)".to_string(),
        |name| format!("(defkind {name} …)"),
    );
    let _ = writeln!(
        err,
        "archogen: {}: {path}:{}:{} declares a kind, `{declared}`, and no command loads a kind module a user writes yet",
        Status::Unimplemented.slug(),
        kind.position.line,
        kind.position.column
    );
    let _ = writeln!(
        err,
        "  hint: the kinds a description may use are the ones this toolchain ships, embedded in the binary and \
         checked on every run (docs/semantics/kinds/). Loading a kind you write is task-tree leaf \
         {KIND_MODULE_OWNER} (docs/TASK_TREE.md); until then, check a description that uses the shipped kinds"
    );
    Status::Unimplemented
}

/// What the frontend concluded about the file a command was given.
#[derive(Debug)]
pub enum Frontend {
    /// The invocation failed before any verdict — an imported module that exists and cannot be read. Already
    /// reported; the status is the command's to return.
    Failed(Status),
    /// A §5.5 verdict about the description, with the instances it was elaborated from when it is a module
    /// tree (`<alias path> = <module> <version>`, the root as `(root)`).
    Checked {
        /// The verdict, its diagnostics, and the declarations the later steps read.
        outcome: Outcome,
        /// `Some` for a module tree.
        instances: Option<Vec<String>>,
    },
}

/// Run the frontend over the file at `id`, already in `sources` as `path`: `check` for a description, and for
/// a module file, elaboration from its module path followed by [`check_program`].
///
/// Three answers for a module file, in the order that makes each one true:
///
/// 1. **A module file that exists and cannot be read is a failure of the invocation** (§6 rule 7): `usage`,
///    exactly as for an unreadable description, and none of the elaborator's diagnostics — it would have
///    called the file missing, which is a false statement about the description.
/// 2. **A composition problem is a verdict about the description**: every elaboration diagnostic, with the
///    verdict their codes carry through [`Verdict::of_code`], the one accessor every consumer shares — and
///    nothing past elaboration, because the passes cannot read a tree that did not compose.
/// 3. **A tree that composes is resolved and checked** by [`check_program`], so it gets every pass a single
///    description gets.
pub fn frontend(
    sources: &mut SourceMap,
    id: SourceId,
    path: &str,
    registry: &Registry,
    active: &Profile,
    err: &mut dyn Write,
) -> Frontend {
    if !is_module_file(sources, id) {
        if let Some(kind) = kind_module(sources, id) {
            return Frontend::Failed(refuse_kind_module(err, path, &kind));
        }
        return Frontend::Checked {
            outcome: check(sources, id, registry, active),
            instances: None,
        };
    }

    // §6 rule 7: the module path is the directory holding the description the command was given.
    let module_path = Path::new(path).parent().unwrap_or_else(|| Path::new(""));
    let modules = DirectoryModules::new(module_path);
    let (program, diagnostics) = elaborate_source(sources, &modules, id);

    let unreadable = modules.unreadable();
    if !unreadable.is_empty() {
        for (file, error) in &unreadable {
            let _ = writeln!(
                err,
                "archogen: {}: cannot read {file}, which {path} imports: {error}",
                Status::Usage.slug()
            );
        }
        let _ = writeln!(
            err,
            "  hint: the module exists and could not be read — check its permissions and that it is UTF-8 text"
        );
        return Frontend::Failed(Status::Usage);
    }

    if diagnostics.has_errors() {
        let verdict = diagnostics
            .items()
            .iter()
            .map(|diagnostic| Verdict::of_code(diagnostic.code))
            .max_by_key(|verdict| verdict.precedence())
            .unwrap_or(Verdict::InvalidDescription);
        return Frontend::Checked {
            outcome: Outcome {
                verdict,
                diagnostics: diagnostics.items().to_vec(),
                declarations: Vec::new(),
                closure: eadl_model::check::Closure::default(),
            },
            instances: None,
        };
    }

    let instances = program
        .instances
        .iter()
        .map(|instance| {
            let name = if instance.path.is_empty() {
                "(root)"
            } else {
                instance.path.as_str()
            };
            format!("{name} = {} {}", instance.module, instance.version)
        })
        .collect();
    Frontend::Checked {
        outcome: check_program(&program, registry, active),
        instances: Some(instances),
    }
}

/// The shipped kind modules, owned so a `SourceMap` can take them — the engine API's, which embeds the one
/// language definition every consumer checks against (leaf `API.3.2`).
///
/// Shared with `build`, which runs the same frontend: one language definition, loaded one way.
pub fn embedded_modules() -> Vec<(String, String)> {
    archogen_api::kind_modules()
}
