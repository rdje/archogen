//! `archogen resolve` (leaf `M3.4.4`; `ROADMAP.md` §10.2): one system of a description searched for a plan by the
//! engine API's `resolve`, the plan written, and — unless the user writes `--unchecked`, a downgrade never made
//! quietly — the configuration checker run over it, the command's status its verdict (findings §16 (a), `M3.5.1`'s
//! design).
//!
//! # The order of operations is the contract
//!
//! 1. Refuse `--locked` before reading anything: there is no lock data to honor it with until `M4.7`, and a plan that
//!    accepted the flag silently would be an unlocked one wearing a locked one's label.
//! 2. Ask the **engine API**'s `resolve` ([`archogen_api::resolve_with`]), the search alone, with `unchecked` set — it
//!    checks the description as `archogen check` does, and answers the check's verdict for one it does not accept,
//!    the search's refusal, or the plan. The command judges nothing itself, so it can do nothing the API cannot.
//! 3. The plan written whole at `--out`, beside it and renamed into place.
//! 4. Unless `--unchecked`: [`consume::check_plan`] runs `archogen-plan-check` over it, its record in `<PLAN>.check/`,
//!    and the status is the checker's verdict — `ok`, or `tool-failure` for a plan the checker refuses. Running a
//!    program is the command line's alone (§10.4), which is why this step is here and not in the API.
//!
//! ⚠️ **Experimental.** It searches the description's own blocks and platforms: a requirement only a realization meets
//! waits on the catalog's providers (`M3.7`), and the MCP server's `resolve` is `M3.4.6`'s.

use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use archogen_api::{Limits, Plan, Resolve};
use archogen_digest::Digest;

use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{self, Expected};
use crate::launch::Launch;
use crate::spec::Maturity;
use crate::status::Status;

/// Run `archogen resolve`.
pub fn run(
    parsed: &Parsed,
    launcher: &dyn Launch,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Status {
    let path = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");
    let plan_path = PathBuf::from(
        parsed
            .value("out")
            .expect("the parser guarantees `--out` is required"),
    );
    let refuse_locked = |err: &mut dyn Write| -> Option<Status> {
        if !parsed.flag("locked") {
            return None;
        }
        let _ = writeln!(
            err,
            "archogen: {}: `--locked` is not implemented for `archogen resolve`",
            Status::Unimplemented.slug()
        );
        let _ = writeln!(
            err,
            "  hint: there is no lock data yet to check a resolution against, and a plan that accepted the flag \
             silently would be an unlocked one wearing a locked one's label. Lock data is ROADMAP.md §10.3 and \
             task-tree leaf M4.7. Re-run without `--locked`"
        );
        Some(Status::Unimplemented)
    };
    let resolve = Resolve {
        system: parsed.value("system"),
        unchecked: true,
    };
    // No budget, as `check` asks: the command line trusts the files it was given.
    let resolution = match ask_with(
        path,
        parsed.value("profile"),
        refuse_locked,
        err,
        |request| archogen_api::resolve_with(request, &resolve, Limits::NONE),
    ) {
        Ok(resolution) => resolution,
        Err(status) => return status,
    };
    let response = &resolution.response;
    let Some(plan) = &resolution.plan else {
        if response.judged.is_none() {
            return report_not_judged(response, err);
        }
        let _ = write!(err, "{}", response.render_diagnostics());
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path} — nothing was resolved",
            response.status.slug(),
            response.diagnostics.len()
        );
        return response.status;
    };
    let profile = response
        .judged
        .as_ref()
        .map(|judged| judged.profile)
        .expect("a plan is written for a judged description");
    if let Err(unwritten) = write_whole(&plan_path, &plan.text) {
        let _ = writeln!(
            err,
            "archogen: {}: {}",
            unwritten.status.slug(),
            unwritten.message
        );
        let _ = writeln!(err, "  hint: {}", unwritten.repair);
        return unwritten.status;
    }
    let _ = writeln!(
        out,
        "{path}: resolved system `{}` against profile `{profile}` into {} ({} binding(s))",
        plan.system,
        plan_path.display(),
        plan.bindings
    );
    print_owed(plan, out);
    // What it searches over today, as the command table states it, so `--help` and this line cannot differ.
    if let Maturity::Experimental {
        scope,
        completed_by,
    } = parsed.spec().maturity
    {
        let _ = writeln!(out, "  experimental: {scope}; completed by {completed_by}");
    }
    if parsed.flag("unchecked") {
        let _ = writeln!(
            err,
            "archogen: unchecked: `--unchecked` — the plan was not checked by `{}`; nothing independent judged it",
            consume::PLAN_CHECKER
        );
        return Status::Ok;
    }
    let Some(program) = Digest::parse(&format!("sha256:{}", plan.program)) else {
        let _ = writeln!(
            err,
            "archogen: {}: the engine named the program by `{}`, which is no digest",
            Status::ToolFailure.slug(),
            plan.program
        );
        return Status::ToolFailure;
    };
    let mut dir = plan_path.clone().into_os_string();
    dir.push(".check");
    let dir = PathBuf::from(dir);
    let expected = Expected {
        description: Path::new(path),
        plan: &plan_path,
        program,
        plan_sha256: Digest::of(plan.text.as_bytes()),
        profile,
    };
    match consume::check_plan(launcher, &expected, &dir, err).map(|checked| checked.record) {
        Ok(record) if record.status() == Status::Ok => {
            let _ = writeln!(
                out,
                "  checked by `{}`: accepted ({})",
                consume::PLAN_CHECKER,
                dir.join(consume::RECORD).display()
            );
            Status::Ok
        }
        Ok(record) => {
            let _ = writeln!(
                err,
                "archogen: {}: `{}` refuses the plan the search wrote ({} finding(s), {}): the search and the \
                 checker disagree, and the plan is not one to build on",
                record.status().slug(),
                consume::PLAN_CHECKER,
                record.findings().len(),
                dir.join(consume::RECORD).display()
            );
            let _ = writeln!(
                err,
                "  hint: report it, with the checker's findings above: one of the two programs misreads this \
                 description"
            );
            record.status()
        }
        Err(refusal) => {
            let _ = writeln!(
                err,
                "archogen: {}: {}",
                refusal.status.slug(),
                refusal.message
            );
            let _ = writeln!(err, "  hint: {}", refusal.repair);
            refusal.status
        }
    }
}

/// What `plan` owes for the engine adapters it binds, if any: the plan is conditional until each is discharged, each
/// line the instance, what it owes, its name and its meaning (`docs/specs/adapters/decision_engine-adapters.md` §9).
/// `archogen explain` prints the same lines for the plan it explains.
pub(crate) fn print_owed(plan: &Plan, out: &mut dyn Write) {
    if plan.obligations.is_empty() {
        return;
    }
    let _ = writeln!(
        out,
        "  conditional: it binds engine adapters, and holds once each obligation below is discharged and each cost \
         charged"
    );
    for owed in &plan.obligations {
        let _ = writeln!(
            out,
            "    {} {} `{}`: {}",
            owed.instance,
            owed.owes.as_str(),
            owed.name,
            owed.doc
        );
    }
}

/// The directories that must be made to reach `path`, innermost first: only these are a run's to remove again.
pub(crate) fn missing_parents(path: &Path) -> Vec<PathBuf> {
    let mut missing = Vec::new();
    let mut parent = path.parent();
    while let Some(up) = parent.filter(|up| !up.as_os_str().is_empty()) {
        if !std::fs::symlink_metadata(up).is_err_and(|error| error.kind() == ErrorKind::NotFound) {
            break;
        }
        missing.push(up.to_path_buf());
        parent = up.parent();
    }
    missing
}

/// The repair for a place an `--out` cannot use, by what stopped it.
pub(crate) fn place_repair(error: &std::io::Error) -> &'static str {
    match error.kind() {
        ErrorKind::InvalidFilename => {
            "leave room in its name for what goes beside it, `.building` and `.replaced` for a package, `.part` for a plan"
        }
        ErrorKind::NotADirectory => {
            "a name on the way to it is no directory: name a path whose parents are directories"
        }
        ErrorKind::PermissionDenied => "name a place this user can reach and write",
        ErrorKind::AlreadyExists => {
            "a name on the way to it is a link to nothing, or something took it meanwhile: look at it, or name another place"
        }
        _ => "name another place",
    }
}

/// Why a plan was not written.
struct Unwritten {
    status: Status,
    message: String,
    repair: String,
}

/// Write `text` at `path` whole: its directory made, the text written at `PATH.part` and renamed into place, over a file or a
/// link standing at `path`, as an output named by the user is. If it cannot land, what it wrote at `PATH.part` goes, with the
/// parents it made. What already stands at `PATH.part` is not this run's: a run that stopped may have left it, so it is refused
/// and left alone. A place it cannot use is `usage`, as `build`'s is.
fn write_whole(path: &Path, text: &str) -> Result<(), Unwritten> {
    let shown = path.display();
    let place = |message: String, repair: &str| Unwritten {
        status: Status::Usage,
        message,
        repair: repair.to_owned(),
    };
    let names_a_directory = path
        .as_os_str()
        .as_encoded_bytes()
        .last()
        .is_some_and(|last| std::path::is_separator(char::from(*last)));
    if path.file_name().is_none() || names_a_directory {
        return Err(place(
            format!("`--out {shown}` names no file of its own"),
            "name the file to hold the plan",
        ));
    }
    if std::fs::symlink_metadata(path).is_ok_and(|meta| meta.is_dir()) {
        return Err(place(
            format!("`--out {shown}` is a directory, and a plan is a file"),
            "name the file to hold the plan",
        ));
    }
    let made = missing_parents(path);
    let unmake = || {
        for dir in &made {
            let _ = std::fs::remove_dir(dir);
        }
    };
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        if let Err(error) = std::fs::create_dir_all(parent) {
            unmake();
            return Err(place(
                format!(
                    "cannot make {}, where `--out` lies: {error}",
                    parent.display()
                ),
                place_repair(&error),
            ));
        }
    }
    let mut part = path.as_os_str().to_owned();
    part.push(".part");
    let part = PathBuf::from(part);
    let opened = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&part);
    let mut file = opened.map_err(|error| {
        unmake();
        if error.kind() == ErrorKind::AlreadyExists {
            place(
                format!(
                    "{} exists, and a plan is written there first: a run that stopped may have left it",
                    part.display()
                ),
                "look at it, and move or remove it; or name another file",
            )
        } else {
            place(
                format!("cannot write {}: {error}", part.display()),
                place_repair(&error),
            )
        }
    })?;
    let written = file.write_all(text.as_bytes());
    drop(file);
    written
        .and_then(|()| std::fs::rename(&part, path))
        .map_err(|error| {
            let _ = std::fs::remove_file(&part);
            unmake();
            Unwritten {
                status: Status::ToolFailure,
                message: format!("cannot write the plan {shown}: {error}"),
                repair: "check that its disk has room, and that what stands at that name can be replaced"
                    .to_owned(),
            }
        })
}
