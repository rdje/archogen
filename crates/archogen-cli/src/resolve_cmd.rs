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
    let mut landing = match Landing::judge(out_name, !unchecked) {
        Ok(landing) => landing,
        Err(unwritten) => return refused(unwritten, err),
    };
    if let Err(unwritten) = landing.open(&plan.text) {
        return refused(unwritten, err);
    }
    let landing = landing;
    // A landing refused may leave the stage, and the next run refuses while it stands, so that is said.
    let land = |checked: bool, err: &mut dyn Write| -> Option<Status> {
        match landing.land(&plan.text, checked) {
            Ok(said) => {
                for line in said {
                    let _ = writeln!(err, "archogen: {line}");
                }
                None
            }
            Err(unwritten) => {
                let status = refused(unwritten, err);
                if landing.stage.exists() {
                    let _ = writeln!(
                        err,
                        "  {} stays, and the next run refuses while it stands: look at it, and remove it",
                        landing.stage.display()
                    );
                }
                Some(status)
            }
        }
    };
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
        if let Some(status) = land(false, err) {
            return status;
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
        sources: Some(&response.sources),
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
            if let Some(status) = land(true, err) {
                return status;
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

/// What begins a plan.
const A_PLAN_BEGINS: &str = "(format \"archogen-assignment/";

/// The most of what stands at `--out` it reads, to tell it is a plan: far more than any plan holds.
const A_PLAN_AT_MOST: u64 = 1 << 24;

/// Whether `bytes` are a plan as this engine writes one: text that begins so, every line one form. So a file of notes is none, and a
/// plan with a comment added is none: it is what someone edited, as a build with a file edited is no build. An edit within a form is
/// not seen, as `crate::built` says a digest cannot see a forger: the checker is what reads a plan.
fn is_a_plan(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok_and(|text| {
        text.starts_with(A_PLAN_BEGINS)
            && text
                .lines()
                .all(|line| line.starts_with('(') && line.ends_with(')'))
    })
}

/// Where a plan goes, the place judged. What each route does is `docs/book/src/cli.md`'s table, which `tests/resolve.rs` runs: this
/// says it nowhere else. Everything a run writes stays in `stage`, private to its owner, until the plan may land, so what stands at
/// `file` and `check` stands until then, and an output replaces only what its own kind wrote.
struct Landing {
    file: PathBuf,
    stage: PathBuf,
    check: PathBuf,
    /// The directories this run made to reach `file`, innermost first.
    made: Vec<PathBuf>,
    /// The directory this run made as the stage, told apart from one that comes to stand at its name.
    made_stage: Option<(u64, u64)>,
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
            place::room(&check, consume::RECORD_EARLIER, out)
                .map_err(|refused| unmake(refused.into()))?;
            for waiting in [consume::RECORD_PART, consume::RECORD_EARLIER] {
                place::free(
                    &check.join(waiting),
                    out,
                    "and a record waits there on its way: a run that stopped may have left it",
                )
                .map_err(|refused| unmake(refused.into()))?;
            }
        }
        Ok(Self {
            file,
            stage,
            check,
            made,
            made_stage: None,
        })
    }

    /// Where the plan is while it is judged.
    fn plan_in_stage(&self) -> PathBuf {
        self.stage.join(PLAN_IN_STAGE)
    }

    /// The stage made, by this run alone, and `text` written in it.
    fn open(&mut self, text: &str) -> Result<(), Unwritten> {
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
        self.made_stage = place::identity(&self.stage);
        std::fs::write(self.plan_in_stage(), text).map_err(|error| {
            self.abandon();
            Unwritten {
                status: Status::ToolFailure,
                message: format!("cannot write the plan: {error}"),
                repair: "check that its disk has room".to_owned(),
            }
        })
    }

    /// An earlier record standing at the record's name, whose mode the new one takes and which goes aside until the plan stands. With
    /// `checked` that is whatever `record_replaceable` has let stand; without, only what reads as a record, since only that would read
    /// as this plan's, and never through a link. Asked and not answerable is a refusal, since it cannot say what it would leave.
    fn earlier_record(&self, checked: bool) -> Result<Option<std::fs::Metadata>, Unwritten> {
        let unanswerable = |what: &Path, error: &std::io::Error| {
            usage(
                format!(
                    "cannot tell whether an earlier record stands at {}: {error}",
                    what.display()
                ),
                "open it, or name another file",
            )
        };
        match std::fs::symlink_metadata(&self.check) {
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(unanswerable(&self.check, &error)),
            Ok(meta) if !meta.is_dir() => return Ok(None),
            Ok(_) => {}
        }
        let record = self.check.join(consume::RECORD);
        let meta = match std::fs::symlink_metadata(&record) {
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
            Err(error) => return Err(unanswerable(&self.check, &error)),
            Ok(meta) if !meta.is_file() => return Ok(None),
            Ok(meta) => meta,
        };
        if checked {
            return Ok(Some(meta));
        }
        match consume::reads_as_record(&record) {
            Ok(true) => Ok(Some(meta)),
            Ok(false) => Ok(None),
            Err(error) => Err(unanswerable(&record, &error)),
        }
    }

    /// The plan in place, and with `checked` its record beside it, each path held again just before it is used, or nothing changed:
    /// each step undoes only what the ones before it moved. The record goes first, since a flag or a mode that stops it is found
    /// then, and the plan last, so a plan does not land to be refused after. What is worth saying, if anything, is the answer.
    fn land(&self, text: &str, checked: bool) -> Result<Vec<String>, Unwritten> {
        let plan = self.plan_in_stage();
        // A stage no longer this run's, or a plan no longer what was judged, is what something else wrote, so it stays to be looked at.
        let came = |message: String| Unwritten {
            status: Status::ToolFailure,
            message,
            repair: "look at it, and at what else writes there".to_owned(),
        };
        if place::identity(&self.stage) != self.made_stage {
            return Err(came(format!(
                "{} is no longer the directory this run made",
                self.stage.display()
            )));
        }
        if !std::fs::read(&plan).is_ok_and(|bytes| bytes == text.as_bytes()) {
            return Err(came(format!(
                "{} changed while the plan was judged",
                plan.display()
            )));
        }
        let out = self.file.display().to_string();
        let replacing = replaceable(&self.file, &out).inspect_err(|_| self.abandon())?;
        if checked {
            record_replaceable(&self.check, &out).inspect_err(|_| self.abandon())?;
        }
        let earlier = self
            .earlier_record(checked)
            .inspect_err(|_| self.abandon())?;
        let record = self.check.join(consume::RECORD);
        let new = self.check.join(consume::RECORD_PART);
        let old = self.check.join(consume::RECORD_EARLIER);
        for (waiting, used) in [(&new, checked), (&old, earlier.is_some())] {
            if used {
                place::free(
                    waiting,
                    &out,
                    "and a record waits there on its way: a run that stopped may have left it",
                )
                .map_err(|refused| {
                    self.abandon();
                    Unwritten::from(refused)
                })?;
            }
        }
        let check_was_free = checked && std::fs::symlink_metadata(&self.check).is_err();
        // What the steps so far have moved, and the way back from each.
        let (mut moved_in, mut moved_out, mut placed) = (false, false, false);
        let undo = |moved_in: bool, moved_out: bool, placed: bool| {
            if placed {
                let _ = std::fs::remove_file(&record);
            } else if moved_in {
                let _ = std::fs::remove_file(&new);
            }
            if moved_out {
                let _ = std::fs::rename(&old, &record);
            }
            if check_was_free {
                let _ = std::fs::remove_dir(&self.check);
            }
            self.abandon();
        };
        let failed = |what: String, repair: &str| Unwritten {
            status: Status::ToolFailure,
            message: what,
            repair: repair.to_owned(),
        };
        let mut said = Vec::new();
        if checked {
            let moved = if check_was_free {
                std::fs::create_dir(&self.check)
            } else {
                Ok(())
            }
            .and_then(|()| std::fs::rename(self.stage.join(consume::RECORD), &new));
            if let Err(error) = moved {
                undo(moved_in, moved_out, placed);
                return Err(failed(
                    format!(
                        "cannot put the checker's record in {}: {error}",
                        self.check.display()
                    ),
                    place::repair(&error),
                ));
            }
            moved_in = true;
        }
        if earlier.is_some() {
            if let Err(error) = std::fs::rename(&record, &old) {
                undo(moved_in, moved_out, placed);
                return Err(failed(
                    format!(
                        "cannot move the earlier record {}: {error}",
                        record.display()
                    ),
                    "check its mode, and any flag on it",
                ));
            }
            moved_out = true;
        }
        if checked {
            if let Some(from) = &earlier {
                if let Err(why) = place::take_over(&new, from) {
                    said.push(format!(
                        "the record at {} could not take the mode or the group of the one it replaced: {why}",
                        record.display()
                    ));
                }
            }
            if let Err(error) = std::fs::rename(&new, &record) {
                undo(moved_in, moved_out, placed);
                return Err(failed(
                    format!(
                        "cannot give the checker's record its name at {}: {error}",
                        record.display()
                    ),
                    "check any flag on it",
                ));
            }
            placed = true;
        }
        if let Some(from) = &replacing {
            if let Err(why) = place::take_over(&plan, from) {
                said.push(format!(
                    "the plan at {out} could not take the mode or the group of what it replaced: {why}"
                ));
            }
        }
        if let Err(error) = std::fs::rename(&plan, &self.file) {
            undo(moved_in, moved_out, placed);
            return Err(failed(
                format!("cannot write the plan {out}: {error}"),
                "check that what stands at that name can be replaced, and any flag on it",
            ));
        }
        if moved_out {
            let _ = std::fs::remove_file(&old);
        }
        if std::fs::remove_dir(&self.stage).is_err() {
            said.push(format!(
                "the plan is in place, and {} stays, holding what came in it, and the next run refuses while it stands: look at it, and remove it",
                self.stage.display()
            ));
        }
        Ok(said)
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
    let mut bytes = Vec::new();
    let read = std::fs::File::open(file)
        .and_then(|opened| opened.take(A_PLAN_AT_MOST + 1).read_to_end(&mut bytes));
    match read {
        Err(error) => Err(usage(
            format!("cannot read `--out {out}`, to tell it is a plan: {error}"),
            "open it, or name another file",
        )),
        Ok(0) => Ok(Some(meta)),
        Ok(_) if bytes.len() as u64 <= A_PLAN_AT_MOST && is_a_plan(&bytes) => Ok(Some(meta)),
        Ok(_) => Err(usage(
            format!(
                "`--out {out}` is no plan as this engine writes one, and a plan replaces only a plan or an empty file"
            ),
            "name another file, or move what stands there",
        )),
    }
}

/// What stands at the record's name that a record may not replace.
fn no_record(record: &Path) -> Unwritten {
    usage(
        format!(
            "{} stands where the checker's record goes, and is no record this engine reads",
            record.display()
        ),
        "look at it, and move or remove it; or name another file, or say `--unchecked`",
    )
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
        Ok(meta) if meta.is_file() && meta.len() == 0 => Ok(()),
        Ok(meta) if meta.is_file() => match consume::reads_as_record(&record) {
            Ok(true) => Ok(()),
            Ok(false) => Err(no_record(&record)),
            Err(error) => Err(usage(
                format!("cannot read {}: {error}", record.display()),
                "open it, or name another file",
            )),
        },
        Ok(_) => Err(no_record(&record)),
    }
}
