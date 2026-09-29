//! `archogen check` — type-check a description against a profile.
//!
//! The first command of the `ROADMAP.md` §10.2 surface to become real. It runs the frontend
//! pipeline (`eadl_model::check`) and maps its §5.5 verdict to this process's exit code through
//! [`Status::from_verdict`], so the number a script branches on and the word a human reads come
//! from the same place.
//!
//! ⛔ **It does not elaborate.** `ROADMAP.md` §10.1 puts module elaboration first, and the elaborator
//! exists (`eadl_front::module`), but no command calls it yet — so a module file is classified by
//! [`module_file`] and refused by [`refuse_module_file`] rather than checked as if it were a description.
//! Both commands route through those two functions, which is what `M1.29.2` replaces with elaboration.

use std::io::Write;

use eadl_front::{read, Form, Position, SourceId, SourceMap};
use eadl_model::check::{check, shipped_registry};
use eadl_model::profile;

use crate::cli::Parsed;
use crate::status::Status;

/// The leaf that makes a module file something `check` and `build` elaborate instead of refuse.
///
/// A constant rather than a literal inside the message, so a test can hold it to the task tree: the
/// refusal names a leaf the tree declares, and it cannot outlive that leaf being closed.
pub const MODULE_ELABORATION_OWNER: &str = "M1.29.2";

/// A description file that is a module, in the sense of `docs/semantics/reference.md` §6.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleFile {
    /// The name its `(defmodule …)` declares, when it declares one.
    pub name: Option<String>,
    /// Where that declaration starts.
    pub position: Position,
}

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

    if let Some(module) = module_file(&sources, id) {
        return refuse_module_file(err, path, &module);
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

/// Classify the description at `id`: `Some` when it is a module file.
///
/// A file is a module when **any** of its declarations is a `(defmodule …)`, not only the first. §6 makes a
/// module file hold exactly one; a file holding one beside other declarations is still a module file — a
/// malformed one, `module-multiple-forms`, which is the elaborator's to report — and classifying by the
/// first declaration alone would hand that file to the schema pass to be told `defmodule` is not a kind.
/// Declarations and not forms, because the language-version identifier may precede the module (§8) and is
/// not one.
///
/// A file that does not **read** is not classified at all, so the read pass reports it: a syntax error is a
/// verdict about the bytes, whatever they were meant to be.
#[must_use]
pub fn module_file(sources: &SourceMap, id: SourceId) -> Option<ModuleFile> {
    let (document, diagnostics) = read(sources, id);
    if diagnostics.has_errors() {
        return None;
    }
    let module = document
        .declarations()
        .find(|form| form.head() == Some("defmodule"))?;
    Some(ModuleFile {
        name: module
            .items()
            .get(1)
            .and_then(Form::as_symbol)
            .map(str::to_string),
        position: sources.get(id)?.position(module.span().start),
    })
}

/// Refuse a module file as `unimplemented`, saying what it is and which leaf makes it checkable.
///
/// ⛔ Leaf `M1.29.1`. Before it, both commands handed a module file to the schema pass, which answered
/// `schema-unknown-kind` for the `defmodule` and then `missing-fact` for the names the module imports —
/// an `invalid-description` **verdict about the system**, exit 10, for a description that is well-formed and
/// that the tool cannot read. The missing capability is the tool's, so the status is the process one
/// `build --locked` returns for the same reason: a command that exists, asked for something it cannot do yet.
pub fn refuse_module_file(err: &mut dyn Write, path: &str, module: &ModuleFile) -> Status {
    let declared = module.name.as_deref().map_or_else(
        || "(defmodule …)".to_string(),
        |name| format!("(defmodule {name} …)"),
    );
    let _ = writeln!(
        err,
        "archogen: {}: {path}:{}:{} is a module, `{declared}`, and no command elaborates a module tree yet",
        Status::Unimplemented.slug(),
        module.position.line,
        module.position.column
    );
    let _ = writeln!(
        err,
        "  hint: the module reader and elaborator exist as a library (docs/semantics/reference.md §6, \
         docs/book/src/modules.md), but no command calls them, so this file's imports would go \
         unresolved. Wiring them in is task-tree leaf {MODULE_ELABORATION_OWNER} (docs/TASK_TREE.md); \
         until then, check a description whose top-level forms are its declarations"
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
