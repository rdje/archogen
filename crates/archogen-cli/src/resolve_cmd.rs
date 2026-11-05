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
use crate::place;
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
    let out_name = parsed
        .value("out")
        .expect("the parser guarantees `--out` is required");
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
    let plan_path = match write_whole(out_name, &plan.text, !parsed.flag("unchecked")) {
        Ok(plan_path) => plan_path,
        Err(unwritten) => {
            let _ = writeln!(
                err,
                "archogen: {}: {}",
                unwritten.status.slug(),
                unwritten.message
            );
            let _ = writeln!(err, "  hint: {}", unwritten.repair);
            return unwritten.status;
        }
    };
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
    let dir = place::beside(&plan_path, CHECK_SUFFIX);
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

/// What the checker's record lies under, beside the plan.
const CHECK_SUFFIX: &str = ".check";

/// Why a plan was not written.
struct Unwritten {
    status: Status,
    message: String,
    repair: String,
}

impl From<place::Refused> for Unwritten {
    fn from(refused: place::Refused) -> Self {
        Self {
            status: Status::Usage,
            message: refused.message,
            repair: refused.repair,
        }
    }
}

/// The new file a plan is written in first, taking the mode of the file it will replace, if one stands there.
#[cfg(unix)]
fn create_part(
    part: &Path,
    replacing: Option<&std::fs::Metadata>,
) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    if let Some(meta) = replacing {
        options.mode(meta.permissions().mode() & 0o7777);
    }
    let file = options.open(part)?;
    if let Some(meta) = replacing {
        file.set_permissions(meta.permissions())?;
    }
    Ok(file)
}

/// The new file a plan is written in first; a platform with no owner's bits has no mode to keep.
#[cfg(not(unix))]
fn create_part(
    part: &Path,
    _replacing: Option<&std::fs::Metadata>,
) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(part)
}

/// `text` written whole at what `out` names, and where that is. What each route does is `docs/book/src/cli.md`'s table, which
/// `tests/resolve.rs` runs: this says it nowhere else. The place is judged whole before anything moves, the checker's record's
/// place among it where it will run, so a plan never lands to be refused after.
fn write_whole(out: &str, text: &str, checked: bool) -> Result<PathBuf, Unwritten> {
    let name_a_file = "name the file to hold the plan";
    let refused = |message: String, repair: &str| Unwritten {
        status: Status::Usage,
        message,
        repair: repair.to_owned(),
    };
    if out
        .as_bytes()
        .last()
        .is_some_and(|last| std::path::is_separator(char::from(*last)))
    {
        return Err(refused(
            format!("`--out {out}` names no file of its own"),
            name_a_file,
        ));
    }
    let path = place::read(out, "file", name_a_file)?;
    let standing = std::fs::symlink_metadata(&path).ok();
    if standing.as_ref().is_some_and(std::fs::Metadata::is_dir) {
        return Err(refused(
            format!("`--out {out}` is a directory, and a plan is a file"),
            name_a_file,
        ));
    }
    let part = place::beside(&path, ".part");
    place::free(
        &part,
        out,
        "and a plan is written there first: a run that stopped may have left it",
    )?;
    let check = place::beside(&path, CHECK_SUFFIX);
    if checked {
        match std::fs::symlink_metadata(&check) {
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => {
                return Err(refused(
                    format!(
                        "cannot use {}, which `--out {out}` needs beside it: {error}",
                        check.display()
                    ),
                    place::repair(&error),
                ))
            }
            Ok(meta) if meta.is_dir() => {}
            Ok(_) => {
                return Err(refused(
                    format!(
                        "{} stands where the checker's record goes, and is no directory",
                        check.display()
                    ),
                    "look at it, and move or remove it; or name another file, or say `--unchecked`",
                ))
            }
        }
    }
    let made = place::missing_parents(&path);
    let unmake = |unwritten: Unwritten| {
        place::unmake(&made);
        unwritten
    };
    place::make_parents(&path).map_err(|refused| unmake(refused.into()))?;
    if checked {
        place::room(&check, consume::RECORD_PART, out).map_err(|refused| unmake(refused.into()))?;
    }
    let replacing = standing.as_ref().filter(|meta| meta.is_file());
    let mut file = create_part(&part, replacing).map_err(|error| {
        unmake(if error.kind() == ErrorKind::AlreadyExists {
            refused(
                format!(
                    "{} came while the plan was being written: a run alongside this one may have made it",
                    part.display()
                ),
                "look at it, and move or remove it; or name another place",
            )
        } else {
            refused(
                format!("cannot write {}: {error}", part.display()),
                place::repair(&error),
            )
        })
    })?;
    let written = file.write_all(text.as_bytes());
    drop(file);
    let shown = path.display().to_string();
    written
        .and_then(|()| std::fs::rename(&part, &path))
        .map_err(|error| {
            let _ = std::fs::remove_file(&part);
            unmake(Unwritten {
                status: Status::ToolFailure,
                message: format!("cannot write the plan {shown}: {error}"),
                repair: "check that its disk has room, and that what stands at that name can be replaced"
                    .to_owned(),
            })
        })?;
    Ok(path)
}
