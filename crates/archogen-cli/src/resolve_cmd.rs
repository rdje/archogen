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

use archogen_api::{Limits, Plan, Resolve, SourceMap};
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
    let unchecked = parsed.flag("unchecked");
    let refused = |unwritten: Unwritten, err: &mut dyn Write| {
        let _ = writeln!(
            err,
            "archogen: {}: {}",
            unwritten.status.slug(),
            unwritten.message
        );
        let _ = writeln!(err, "  hint: {}", unwritten.repair);
        unwritten.status
    };
    let landing = match Landing::judge(out_name, &response.sources, !unchecked) {
        Ok(landing) => landing,
        Err(unwritten) => return refused(unwritten, err),
    };
    if let Err(unwritten) = landing.write(&plan.text) {
        return refused(unwritten, err);
    }
    // What it says once the plan stands: so a command that refuses never said it wrote one.
    let said = |out: &mut dyn Write| {
        let _ = writeln!(
            out,
            "{path}: resolved system `{}` against profile `{profile}` into {} ({} binding(s))",
            plan.system,
            landing.file.display(),
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
    };
    if unchecked {
        if let Err(unwritten) = landing.land() {
            return refused(unwritten, err);
        }
        // A record of an earlier plan beside this one would read as this one's.
        let _ = std::fs::remove_file(landing.check.join(consume::RECORD));
        said(out);
        let _ = writeln!(
            err,
            "archogen: unchecked: `--unchecked` — the plan was not checked by `{}`; nothing independent judged it",
            consume::PLAN_CHECKER
        );
        return Status::Ok;
    }
    let Some(program) = Digest::parse(&format!("sha256:{}", plan.program)) else {
        landing.abandon();
        let _ = writeln!(
            err,
            "archogen: {}: the engine named the program by `{}`, which is no digest",
            Status::ToolFailure.slug(),
            plan.program
        );
        return Status::ToolFailure;
    };
    let expected = Expected {
        description: Path::new(path),
        plan: &landing.part,
        program,
        plan_sha256: Digest::of(plan.text.as_bytes()),
        profile,
    };
    let record_file = landing.check.join(consume::RECORD);
    match consume::check_plan(launcher, &expected, &landing.check, err)
        .map(|checked| checked.record)
    {
        Ok(record) if record.status() == Status::Ok => {
            if let Err(unwritten) = landing.land() {
                return refused(unwritten, err);
            }
            said(out);
            let _ = writeln!(
                out,
                "  checked by `{}`: accepted ({})",
                consume::PLAN_CHECKER,
                record_file.display()
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
                record_file.display()
            );
            let _ = writeln!(
                err,
                "  hint: report it, with the checker's findings above: one of the two programs misreads this \
                 description. The plan stays at {}, for that report, and whatever stood at its name is as it was",
                landing.part.display()
            );
            record.status()
        }
        Err(refusal) => {
            // A record that does not hold together is what a report needs; with none, nothing was judged and nothing stays.
            let kept = record_file.exists();
            if !kept {
                landing.abandon();
            }
            let _ = writeln!(
                err,
                "archogen: {}: {}",
                refusal.status.slug(),
                refusal.message
            );
            let _ = writeln!(err, "  hint: {}", refusal.repair);
            if kept {
                let _ = writeln!(
                    err,
                    "  the plan stays at {}, for that report, and whatever stood at its name is as it was",
                    landing.part.display()
                );
            }
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

/// The new file a plan is written in first: private where it will take a file's mode, so the checker can read it whatever that
/// mode is, and that mode given only when it lands.
#[cfg(unix)]
fn create_part(part: &Path, replacing: bool) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    if replacing {
        options.mode(0o600);
    }
    options.open(part)
}

/// The new file a plan is written in first; a platform with no owner's bits has no mode to keep.
#[cfg(not(unix))]
fn create_part(part: &Path, _replacing: bool) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(part)
}

/// Where a plan goes, the place judged. What each route does is `docs/book/src/cli.md`'s table, which `tests/resolve.rs` runs: this
/// says it nowhere else. A plan is judged at `part` and lands at `file` only on acceptance, so what stands at `file` stands until
/// there is a plan that may replace it.
struct Landing {
    file: PathBuf,
    part: PathBuf,
    check: PathBuf,
    /// A regular file standing at `file`, which the plan takes the mode and the group of.
    replacing: Option<std::fs::Metadata>,
    /// The directories this run made to reach `file`, innermost first.
    made: Vec<PathBuf>,
    /// Whether nothing stood at `check`, so that an empty directory there is this run's.
    check_was_free: bool,
}

impl Landing {
    /// `out` as a place, its parents made, or why it is none.
    fn judge(out: &str, sources: &SourceMap, checked: bool) -> Result<Self, Unwritten> {
        let name_a_file = "name the file to hold the plan";
        let refused = |message: String, repair: &str| Unwritten {
            status: Status::Usage,
            message,
            repair: repair.to_owned(),
        };
        let names_a_directory = out.ends_with("/.")
            || out
                .as_bytes()
                .last()
                .is_some_and(|last| std::path::is_separator(char::from(*last)));
        if names_a_directory {
            return Err(refused(
                format!("`--out {out}` names no file of its own"),
                name_a_file,
            ));
        }
        let path = place::read(out, "file", name_a_file)?;
        let standing = std::fs::symlink_metadata(&path).ok();
        if let Some(meta) = &standing {
            if meta.is_dir() {
                return Err(refused(
                    format!("`--out {out}` is a directory, and a plan is a file"),
                    name_a_file,
                ));
            }
            if !(meta.is_file() || meta.is_symlink()) {
                return Err(refused(
                    format!("`--out {out}` is neither a file nor a link, and a plan replaces only those"),
                    name_a_file,
                ));
            }
        }
        if let Some(source) = sources
            .iter()
            .find(|source| place::same_file(Path::new(&source.name), &path))
        {
            return Err(refused(
                format!(
                    "`--out {out}` is `{}`, which this description was read from, and the plan would replace it",
                    source.name
                ),
                name_a_file,
            ));
        }
        let part = place::beside(&path, ".part");
        place::free(
            &part,
            out,
            "and a plan is written there first: a run that stopped may have left it, or a refused one for a report",
        )?;
        let check = place::beside(&path, CHECK_SUFFIX);
        let mut check_was_free = true;
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
                Ok(meta) if meta.is_dir() => check_was_free = false,
                Ok(_) => return Err(refused(
                    format!(
                        "{} stands where the checker's record goes, and is no directory",
                        check.display()
                    ),
                    "look at it, and move or remove it; or name another file, or say `--unchecked`",
                )),
            }
        }
        let made = place::missing_parents(&path);
        let unmake = |unwritten: Unwritten| {
            place::unmake(&made);
            unwritten
        };
        place::make_parents(&path).map_err(|refused| unmake(refused.into()))?;
        if checked {
            place::room(&check, consume::RECORD_PART, out)
                .map_err(|refused| unmake(refused.into()))?;
        }
        Ok(Self {
            replacing: standing.filter(std::fs::Metadata::is_file),
            file: path,
            part,
            check,
            made,
            check_was_free,
        })
    }

    /// `text` written at `part`, new.
    fn write(&self, text: &str) -> Result<(), Unwritten> {
        let mut file = create_part(&self.part, self.replacing.is_some()).map_err(|error| {
            place::unmake(&self.made);
            Unwritten {
                status: Status::Usage,
                message: if error.kind() == ErrorKind::AlreadyExists {
                    format!(
                        "{} came while the plan was being written: a run alongside this one may have made it",
                        self.part.display()
                    )
                } else {
                    format!("cannot write {}: {error}", self.part.display())
                },
                repair: if error.kind() == ErrorKind::AlreadyExists {
                    "look at it, and move or remove it; or name another place".to_owned()
                } else {
                    place::repair(&error).to_owned()
                },
            }
        })?;
        let written = file.write_all(text.as_bytes());
        drop(file);
        written.map_err(|error| {
            self.abandon();
            Unwritten {
                status: Status::ToolFailure,
                message: format!("cannot write {}: {error}", self.part.display()),
                repair: "check that its disk has room".to_owned(),
            }
        })
    }

    /// The plan in place, taking the mode and the group of the file it replaces. It stands or nothing changed.
    fn land(&self) -> Result<(), Unwritten> {
        let shown = self.file.display().to_string();
        let kept = self
            .replacing
            .as_ref()
            .map_or(Ok(()), |from| place::take_over(&self.part, from));
        kept.and_then(|()| std::fs::rename(&self.part, &self.file))
            .map_err(|error| {
                self.abandon();
                Unwritten {
                    status: Status::ToolFailure,
                    message: format!("cannot write the plan {shown}: {error}"),
                    repair:
                        "check that what stands at that name can be replaced, and any flag on it"
                            .to_owned(),
                }
            })
    }

    /// What this run wrote gone, with the directories it made where they stayed empty.
    fn abandon(&self) {
        let _ = std::fs::remove_file(&self.part);
        if self.check_was_free {
            let _ = std::fs::remove_dir(&self.check);
        }
        place::unmake(&self.made);
    }
}
