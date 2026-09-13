//! `archogen check` — elaborate and type-check a description against a profile.
//!
//! The first command of the `ROADMAP.md` §10.2 surface to become real. It runs the frontend
//! pipeline (`eadl_model::check`) and maps its §5.5 verdict to this process's exit code through
//! [`Status::from_verdict`], so the number a script branches on and the word a human reads come
//! from the same place.

use std::io::Write;

use eadl_front::SourceMap;
use eadl_model::check::{check, shipped_registry};
use eadl_model::profile;

use crate::cli::Parsed;
use crate::status::Status;

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

fn embedded_modules() -> Vec<(String, String)> {
    KIND_MODULES
        .iter()
        .map(|(name, text)| ((*name).to_string(), (*text).to_string()))
        .collect()
}
