//! `archogen check` — elaborate a module tree, and type-check a description, against a profile.
//!
//! The first command of the `ROADMAP.md` §10.2 surface to become real. It runs the frontend
//! pipeline (`eadl_model::check`) and maps its §5.5 verdict to this process's exit code through
//! [`Status::from_verdict`], so the number a script branches on and the word a human reads come
//! from the same place.
//!
//! ⛔ **A module tree is elaborated and not yet type-checked.** `ROADMAP.md` §10.1 puts elaboration first.
//! A file [`is_module_file`] recognises is elaborated by [`elaborate_module_file`] from the module path of
//! `docs/semantics/reference.md` §6 rule 7, and every composition rule of §6 is enforced — but the later
//! passes cannot read an elaborated program until the name rule of leaf `M1.29.3` exists, so a tree that
//! elaborates cleanly is answered `unimplemented` rather than checked with names nothing resolves. Both
//! commands route through those two functions.

use std::io::Write;
use std::path::Path;

use eadl_front::{elaborate_source, read, DirectoryModules, SourceId, SourceMap, Verdict};
use eadl_model::check::{check, shipped_registry};
use eadl_model::profile;

use crate::cli::Parsed;
use crate::status::Status;

/// The leaf that makes an elaborated module tree something `check` and `build` type-check.
///
/// A constant rather than a literal inside the message, so a test can hold it to the task tree: the
/// refusal names a leaf the tree declares, and it cannot outlive that leaf being closed.
pub const MODULE_TYPE_CHECK_OWNER: &str = "M1.29.3";

/// The kind modules shipped with the toolchain.
///
/// Embedded at compile time rather than read from the working directory: `archogen` must behave
/// identically wherever it is run from, and a language definition that could be shadowed by a
/// file in the current directory is a language definition an accident can change.
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

    if is_module_file(&sources, id) {
        return elaborate_module_file(&mut sources, id, path, err, "");
    }

    let outcome = check(&sources, id, &registry, active);
    let status = Status::from_verdict(outcome.verdict);

    if outcome.is_ok() {
        let _ = writeln!(
            out,
            "{path}: accepted against profile `{}` ({} declaration(s))",
            active.id,
            outcome.declarations.len()
        );
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

/// Elaborate the module file at `id` — already in `sources` as `path` — from its module path, and answer
/// for it. `tail` ends the summary line, so `build` can say that nothing was generated.
///
/// Three answers, in the order that makes each one true:
///
/// 1. **A module file that exists and cannot be read is a failure of the invocation** (§6 rule 7): `usage`,
///    exactly as for an unreadable description, and none of the elaborator's diagnostics — it would have
///    called the file missing, which is a false statement about the description.
/// 2. **A composition problem is a verdict about the description**: every diagnostic, and the verdict their
///    codes carry through [`Verdict::of_code`], the one accessor every consumer shares.
/// 3. **A tree that elaborates cleanly is `unimplemented`**, naming [`MODULE_TYPE_CHECK_OWNER`] and the
///    instances it found — not checked with names nothing resolves, and not accepted either.
pub fn elaborate_module_file(
    sources: &mut SourceMap,
    id: SourceId,
    path: &str,
    err: &mut dyn Write,
    tail: &str,
) -> Status {
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
        return Status::Usage;
    }

    if diagnostics.has_errors() {
        let verdict = diagnostics
            .items()
            .iter()
            .map(|diagnostic| Verdict::of_code(diagnostic.code))
            .max_by_key(|verdict| verdict.precedence())
            .unwrap_or(Verdict::InvalidDescription);
        let status = Status::from_verdict(verdict);
        let _ = write!(err, "{}", diagnostics.render(sources));
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path}{tail}",
            status.slug(),
            diagnostics.len()
        );
        return status;
    }

    let instances: Vec<String> = program
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
    let _ = writeln!(
        err,
        "archogen: {}: {path} elaborated into {} instance(s), and no command type-checks an elaborated \
         module tree yet{tail}",
        Status::Unimplemented.slug(),
        program.len()
    );
    let _ = writeln!(err, "  instances: {}", instances.join(", "));
    let _ = writeln!(
        err,
        "  hint: every composition rule of docs/semantics/reference.md §6 held. The declarations of an \
         elaborated tree need the name rule of task-tree leaf {MODULE_TYPE_CHECK_OWNER} (docs/TASK_TREE.md) \
         before the later passes can read them — how a name written inside an imported module resolves, and \
         what an `export` hides"
    );
    Status::Unimplemented
}

/// The shipped kind modules, owned so a `SourceMap` can take them.
///
/// Shared with `build`, which runs the same frontend: one language definition, loaded one way.
pub fn embedded_modules() -> Vec<(String, String)> {
    KIND_MODULES
        .iter()
        .map(|(name, text)| ((*name).to_string(), (*text).to_string()))
        .collect()
}
