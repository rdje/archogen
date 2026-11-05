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
//! 3. The plan written in a private stage, `<PLAN>.part/`, the place judged first.
//! 4. Unless `--unchecked`: [`consume::check_plan`] runs `archogen-plan-check` over it, its record in that stage, and the
//!    status is the checker's verdict — `ok`, or `tool-failure` for a plan the checker refuses. Running a program is the
//!    command line's alone (§10.4), which is why this step is here and not in the API.
//! 5. On acceptance, or under `--unchecked`, the plan lands at `--out` and its record in `<PLAN>.check/`, so nothing there moves
//!    until there is a plan that may replace it.
//!
//! ⚠️ **Experimental.** It searches the description's own blocks and platforms: a requirement only a realization meets
//! waits on the catalog's providers (`M3.7`), and the MCP server's `resolve` is `M3.4.6`'s.

use std::io::{ErrorKind, Read, Write};
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
    let landing = match Landing::judge(out_name, !unchecked) {
        Ok(landing) => landing,
        Err(unwritten) => return refused(unwritten, err),
    };
    if let Err(unwritten) = landing.open(&plan.text) {
        return refused(unwritten, err);
    }
    let plan_in_stage = landing.plan_in_stage();
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
        match landing.land(&plan.text, false) {
            Ok(unkept) => landing.warn(unkept, err),
            Err(unwritten) => return refused(unwritten, err),
        }
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
        text: response
            .sources
            .iter()
            .find(|source| source.name == *path)
            .map(|source| source.text.as_str()),
        plan: &plan_in_stage,
        program,
        plan_sha256: Digest::of(plan.text.as_bytes()),
        profile,
    };
    let record_file = landing.check.join(consume::RECORD);
    let record_in_stage = landing.stage.join(consume::RECORD);
    match consume::check_plan(launcher, &expected, &landing.stage, err)
        .map(|checked| checked.record)
    {
        Ok(record) if record.status() == Status::Ok => {
            match landing.land(&plan.text, true) {
                Ok(unkept) => landing.warn(unkept, err),
                Err(unwritten) => return refused(unwritten, err),
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
                record_in_stage.display()
            );
            let _ = writeln!(
                err,
                "  hint: report it, with the checker's findings above: one of the two programs misreads this \
                 description. The plan and the record stay in {}, for that report, and whatever stood at its name is as it was",
                landing.stage.display()
            );
            record.status()
        }
        Err(refusal) => {
            // A record that does not hold together is what a report needs; with none, nothing was judged and nothing stays.
            let kept = record_in_stage.exists();
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
                    "  the plan and the record stay in {}, for that report, and whatever stood at its name is as it was",
                    landing.stage.display()
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

/// What a plan is written as in its stage, whatever it will be named.
const PLAN_IN_STAGE: &str = "plan";

/// What begins a plan: what a plan may replace is known by this, as `crate::built` knows a build by what it lists.
const A_PLAN_BEGINS: &[u8] = b"(format \"archogen-assignment/";

/// Where a plan goes, the place judged. What each route does is `docs/book/src/cli.md`'s table, which `tests/resolve.rs` runs: this
/// says it nowhere else. Everything a run writes stays in `stage`, private to its owner, until the plan may land, so what stands at
/// `file` and `check` stands until then, and an output replaces only what its own kind wrote.
struct Landing {
    file: PathBuf,
    stage: PathBuf,
    check: PathBuf,
    /// The directories this run made to reach `file`, innermost first.
    made: Vec<PathBuf>,
}

impl Landing {
    /// `out` as a place, its parents made, or why it is none.
    fn judge(out: &str, checked: bool) -> Result<Self, Unwritten> {
        let name_a_file = "name the file to hold the plan";
        let names_a_directory = out.ends_with("/.")
            || out
                .as_bytes()
                .last()
                .is_some_and(|last| std::path::is_separator(char::from(*last)));
        if names_a_directory {
            return Err(usage(
                format!("`--out {out}` names no file of its own"),
                name_a_file,
            ));
        }
        let file = place::read(out, "file", name_a_file)?;
        replaceable(&file, out)?;
        let stage = place::beside(&file, ".part");
        place::free(
            &stage,
            out,
            "and a plan is written there first: a run that stopped may have left it, or a refused one for a report",
        )?;
        let check = place::beside(&file, CHECK_SUFFIX);
        if checked {
            record_replaceable(&check, out)?;
        }
        let made = place::missing_parents(&file);
        let unmake = |unwritten: Unwritten| {
            place::unmake(&made);
            unwritten
        };
        place::make_parents(&file).map_err(|refused| unmake(refused.into()))?;
        place::room(&stage, consume::RECORD_PART, out).map_err(|refused| unmake(refused.into()))?;
        if checked {
            place::room(&check, consume::RECORD_PART, out)
                .map_err(|refused| unmake(refused.into()))?;
        }
        Ok(Self {
            file,
            stage,
            check,
            made,
        })
    }

    /// Where the plan is while it is judged.
    fn plan_in_stage(&self) -> PathBuf {
        self.stage.join(PLAN_IN_STAGE)
    }

    /// The stage made, by this run alone, and `text` written in it.
    fn open(&self, text: &str) -> Result<(), Unwritten> {
        place::make_private(&self.stage).map_err(|error| {
            place::unmake(&self.made);
            usage(
                format!(
                    "cannot make the directory {}: {error}",
                    self.stage.display()
                ),
                place::repair(&error),
            )
        })?;
        std::fs::write(self.plan_in_stage(), text).map_err(|error| {
            self.abandon();
            Unwritten {
                status: Status::ToolFailure,
                message: format!("cannot write the plan: {error}"),
                repair: "check that its disk has room".to_owned(),
            }
        })
    }

    /// The plan in place, and with `checked` its record beside it, each path held again just before it is used, or nothing
    /// changed. What goes first is the one step that can tell whether the record's directory takes a record, so a plan does not
    /// land to be refused after. What it could not keep of what it replaced, if anything, is the answer.
    fn land(&self, text: &str, checked: bool) -> Result<Option<std::io::Error>, Unwritten> {
        let refused = |what: String, repair: &str| {
            self.abandon();
            Unwritten {
                status: Status::ToolFailure,
                message: what,
                repair: repair.to_owned(),
            }
        };
        let plan = self.plan_in_stage();
        // What is judged is what lands.
        if !std::fs::read(&plan).is_ok_and(|bytes| bytes == text.as_bytes()) {
            return Err(Unwritten {
                status: Status::ToolFailure,
                message: format!("{} changed while the plan was judged", plan.display()),
                repair: "look at it, and at what else writes there, then remove it".to_owned(),
            });
        }
        let out = self.file.display().to_string();
        let replacing = replaceable(&self.file, &out).inspect_err(|_| self.abandon())?;
        let record = self.check.join(consume::RECORD);
        let aside = self.check.join(consume::RECORD_PART);
        let check_was_free = std::fs::symlink_metadata(&self.check).is_err();
        let undo = || {
            let _ = std::fs::remove_file(&aside);
            if check_was_free {
                let _ = std::fs::remove_dir(&self.check);
            }
        };
        // `aside` holds the record on its way in, or an earlier one on its way out, so either can go back.
        // Asked of a real directory: `is_record` holds the last name to no link, and a link at `check` is on the way to it.
        let earlier = !checked
            && std::fs::symlink_metadata(&self.check).is_ok_and(|meta| meta.is_dir())
            && consume::is_record(&record);
        if checked || earlier {
            place::free(
                &aside,
                &out,
                "and a record is moved there on its way: a run that stopped may have left it",
            )
            .map_err(|refused| {
                self.abandon();
                Unwritten::from(refused)
            })?;
        }
        if checked {
            record_replaceable(&self.check, &out).inspect_err(|_| self.abandon())?;
            let moved = if check_was_free {
                std::fs::create_dir(&self.check)
            } else {
                Ok(())
            }
            .and_then(|()| std::fs::rename(self.stage.join(consume::RECORD), &aside));
            if let Err(error) = moved {
                undo();
                return Err(refused(
                    format!(
                        "cannot put the checker's record in {}: {error}",
                        self.check.display()
                    ),
                    place::repair(&error),
                ));
            }
        } else if earlier {
            if let Err(error) = std::fs::rename(&record, &aside) {
                return Err(refused(
                    format!(
                        "cannot move the earlier record {}, which would read as this plan's: {error}",
                        record.display()
                    ),
                    place::repair(&error),
                ));
            }
        }
        let unkept = replacing
            .as_ref()
            .and_then(|from| place::take_over(&plan, from).err());
        if let Err(error) = std::fs::rename(&plan, &self.file) {
            if earlier {
                let _ = std::fs::rename(&aside, &record);
            } else {
                undo();
            }
            return Err(refused(
                format!("cannot write the plan {out}: {error}"),
                "check that what stands at that name can be replaced, and any flag on it",
            ));
        }
        if checked {
            if let Err(error) = std::fs::rename(&aside, &record) {
                return Err(Unwritten {
                    status: Status::ToolFailure,
                    message: format!(
                        "the plan is in place, and its record could not take its name at {}: {error}",
                        record.display()
                    ),
                    repair: format!("the record is at {}: move it there", aside.display()),
                });
            }
        } else if earlier {
            let _ = std::fs::remove_file(&aside);
        }
        let _ = std::fs::remove_dir(&self.stage);
        Ok(unkept)
    }

    /// Say what the plan could not keep of what it replaced.
    fn warn(&self, unkept: Option<std::io::Error>, err: &mut dyn Write) {
        if let Some(why) = unkept {
            let _ = writeln!(
                err,
                "archogen: the plan is in place at {}, and could not take the mode or the group of what it replaced: {why}",
                self.file.display()
            );
        }
    }

    /// What this run wrote gone, with the directories it made where they stayed empty.
    fn abandon(&self) {
        let _ = std::fs::remove_dir_all(&self.stage);
        place::unmake(&self.made);
    }
}

/// A place refused, nothing yet having moved.
fn usage(message: String, repair: &str) -> Unwritten {
    Unwritten {
        status: Status::Usage,
        message,
        repair: repair.to_owned(),
    }
}

/// Whether what stands at `file` is what a plan may replace: nothing, an empty file, or a plan. It returns the file, whose mode and
/// group the plan takes. Anything else may be someone's, a description or a module among them, however it is named.
fn replaceable(file: &Path, out: &str) -> Result<Option<std::fs::Metadata>, Unwritten> {
    let name_a_file = "name the file to hold the plan";
    let meta = match std::fs::symlink_metadata(file) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(usage(
                format!("cannot use `--out {out}`: {error}"),
                place::repair(&error),
            ))
        }
        Ok(meta) => meta,
    };
    if meta.is_dir() {
        return Err(usage(
            format!("`--out {out}` is a directory, and a plan is a file"),
            name_a_file,
        ));
    }
    if meta.is_symlink() {
        return Err(usage(
            format!("`--out {out}` is a link, and a plan replaces only a plan"),
            "name the file itself, not a link to it",
        ));
    }
    if !meta.is_file() {
        return Err(usage(
            format!("`--out {out}` is no file, and a plan replaces only a plan"),
            name_a_file,
        ));
    }
    let mut begins = vec![0; A_PLAN_BEGINS.len()];
    let read = std::fs::File::open(file).and_then(|mut opened| {
        let mut filled = 0;
        while filled < begins.len() {
            match opened.read(&mut begins[filled..])? {
                0 => break,
                count => filled += count,
            }
        }
        Ok(filled)
    });
    match read {
        Err(error) => Err(usage(
            format!("cannot read `--out {out}`, to tell it is a plan: {error}"),
            "open it, or name another file",
        )),
        Ok(0) => Ok(Some(meta)),
        Ok(filled) if filled == begins.len() && begins == A_PLAN_BEGINS => Ok(Some(meta)),
        Ok(_) => Err(usage(
            format!("`--out {out}` is no plan, and a plan replaces only a plan or an empty file"),
            "name another file, or move what stands there",
        )),
    }
}

/// Whether the checker's record can go at `check`: nothing there, or a directory holding nothing at the record's name, or an empty
/// file, or a record. Anything else may be someone's.
fn record_replaceable(check: &Path, out: &str) -> Result<(), Unwritten> {
    match std::fs::symlink_metadata(check) {
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(usage(
                format!(
                    "cannot use {}, which `--out {out}` needs beside it: {error}",
                    check.display()
                ),
                place::repair(&error),
            ))
        }
        Ok(meta) if meta.is_dir() => {}
        Ok(_) => {
            return Err(usage(
                format!(
                    "{} stands where the checker's record goes, and is no directory",
                    check.display()
                ),
                "look at it, and move or remove it; or name another file, or say `--unchecked`",
            ))
        }
    }
    let record = check.join(consume::RECORD);
    match std::fs::symlink_metadata(&record) {
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(usage(
            format!("cannot use {}: {error}", record.display()),
            place::repair(&error),
        )),
        Ok(meta) if meta.is_file() && (meta.len() == 0 || consume::is_record(&record)) => Ok(()),
        Ok(_) => Err(usage(
            format!(
                "{} stands where the checker's record goes, and is no record",
                record.display()
            ),
            "look at it, and move or remove it; or name another file, or say `--unchecked`",
        )),
    }
}
