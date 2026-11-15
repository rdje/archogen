//! `archogen check` — elaborate and type-check a description against a profile.
//!
//! The first command of the `ROADMAP.md` §10.2 surface to become real, and since leaf `API.3.3` the engine
//! API's first consumer: it reads the file, hands it to [`archogen_api::check`] with the directory its imports
//! resolve from, and prints the [`Response`]. It judges nothing itself, so a capability of `check` is one of
//! the API's and the reverse (`docs/decisions/decision_engine-api.md`), and the exit code is the response's
//! status — the number a script branches on and the word a human reads come from the same place.

use std::io::Write;
use std::path::Path;

use archogen_api::{Closure, Limits, Request, Response, TargetFile};
use eadl_front::DirectoryModules;

use crate::cli::Parsed;
use crate::status::Status;

/// The closure boundary as report lines: what is inside and what pulled each fact in, then what is outside.
pub fn closure_report(closure: &Closure) -> Vec<String> {
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

/// Print a response the API did not judge — each note, the hint, and any diagnostics — and return its status.
/// Shared with `build`, so the two commands word a refusal identically.
pub(crate) fn report_not_judged(response: &Response, err: &mut dyn Write) -> Status {
    for note in &response.notes {
        let _ = writeln!(err, "archogen: {}: {note}", response.status.slug());
    }
    if let Some(hint) = &response.hint {
        let _ = writeln!(err, "  hint: {hint}");
    }
    let _ = write!(err, "{}", response.render_diagnostics());
    response.status
}

/// Read the description at `path`, and the target's file at `target` when one is named, and ask the engine API about
/// them, with the module path §6 rule 7 names: the directory holding the description. `Err` is the status of an
/// invocation that failed before the API was asked — an unsupported profile, reported before a file is read, or a file
/// that cannot be read.
pub(crate) fn ask(
    path: &str,
    requested: Option<&str>,
    target: Option<&str>,
    before_reading: impl FnOnce(&mut dyn Write) -> Option<Status>,
    err: &mut dyn Write,
) -> Result<Response, Status> {
    // No budget: the command line is its own consumer, and trusts the files it was given. The budget protects an
    // instance's host from a consumer it does not control (`docs/decisions/decision_api-instance.md`); the list,
    // module and elaboration limits of the language still apply, because they are the language's.
    ask_with(path, requested, target, before_reading, err, |request| {
        archogen_api::check_with(request, Limits::NONE)
    })
}

/// Read `path` as text, or say why not, `usage`, with `hint` where the reason is no host's limit.
fn read_text(path: &str, hint: &str, err: &mut dyn Write) -> Result<String, Status> {
    std::fs::read_to_string(path).map_err(|error| {
        let _ = writeln!(
            err,
            "archogen: {}: cannot read {path}: {error}",
            Status::Usage.slug()
        );
        let hint = match error.kind() {
            std::io::ErrorKind::PermissionDenied => "open it, and the directories above it",
            std::io::ErrorKind::InvalidFilename => {
                "the path is too long for this host: name it from a directory nearer it"
            }
            _ => hint,
        };
        let _ = writeln!(err, "  hint: {hint}");
        Status::Usage
    })
}

/// [`ask`], the API asked by `operation` — `check`, or `resolve` (leaf `M3.4.4`) — over the same request.
pub(crate) fn ask_with<T>(
    path: &str,
    requested: Option<&str>,
    target: Option<&str>,
    before_reading: impl FnOnce(&mut dyn Write) -> Option<Status>,
    err: &mut dyn Write,
    operation: impl FnOnce(&Request<'_>) -> T,
) -> Result<T, Status> {
    // Resolve the profile before reading anything else: a description checked against a profile nobody
    // supports has not been checked, and an unreadable file reported after an unsupported profile would send
    // the author to fix the wrong thing.
    if let Some(refused) = archogen_api::refuse_profile(requested) {
        return Err(report_not_judged(&refused, err));
    }
    if let Some(status) = before_reading(err) {
        return Err(status);
    }
    let text = read_text(path, "give the path of an eADL description", err)?;
    let machine = match target {
        Some(target) => Some((
            target,
            read_text(
                target,
                "give the path of a target's file, its hardware alone, such as targets/riscv-virt-up.eadl",
                err,
            )?,
        )),
        None => None,
    };
    let modules = DirectoryModules::new(Path::new(path).parent().unwrap_or_else(|| Path::new("")));
    Ok(operation(&Request {
        name: path,
        text: &text,
        profile: requested,
        modules: &modules,
        target: machine
            .as_ref()
            .map(|(name, text)| TargetFile { name, text }),
    }))
}

/// Run `archogen check`.
pub fn run(parsed: &Parsed, out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let path = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");
    let target = parsed.value("target");
    let response = match ask(path, parsed.value("profile"), target, |_| None, err) {
        Ok(response) => response,
        Err(status) => return status,
    };
    // What a judgement on a target was judged on, said where the verdict is.
    let on = target.map_or_else(String::new, |target| format!(" on target {target}"));
    let Some(judged) = &response.judged else {
        return report_not_judged(&response, err);
    };

    if response.is_ok() {
        let _ = writeln!(
            out,
            "{path}: accepted against profile `{}`{on} ({} declaration(s))",
            judged.profile,
            judged.declarations.len()
        );
        if let Some(instances) = &judged.instances {
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
        for line in closure_report(&judged.closure) {
            let _ = writeln!(out, "  {line}");
        }
        // ⚠️ Deliberately modest wording. Acceptance means the description is well-formed, in
        // profile, and internally consistent — it is not a statement that any system built from
        // it will behave. §7.1's evidence categories start after this point.
        let _ = writeln!(
            out,
            "  this checks the description, not a system: no resolution, generation or analysis has run"
        );
        return response.status;
    }

    let _ = write!(err, "{}", response.render_diagnostics());
    let _ = writeln!(
        err,
        "archogen: {}: {} diagnostic(s) in {path}{on}",
        response.status.slug(),
        response.diagnostics.len()
    );
    response.status
}
