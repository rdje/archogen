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
    if let Err(unwritten) = landing.open() {
        return refused(unwritten, err);
    }
    let landing = landing;
    if let Err(unwritten) = landing.write_plan(&plan.text) {
        let status = refused(unwritten, err);
        place::say_it_stays(&landing.stage, err);
        return status;
    }
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
                place::say_it_stays(&landing.stage, err);
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
        place::say_it_stays(&landing.stage, err);
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
    match consume::check_plan(
        launcher,
        &expected,
        &landing.stage,
        &landing.written.checker_ran,
        err,
    )
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
                 description. The plan and the record stay in {}, for that report, and whatever stood at its name is as it was; \
                 the next run refuses while they stand, so remove them once it is made",
                landing.stage.display()
            );
            record.status()
        }
        Err(refusal) => {
            // What the checker left is what looking into this needs; with none, nothing was judged and nothing stays. One that stood there before it ran is no report of this run's.
            let kept = landing.written.checker_ran.get() && record_in_stage.exists();
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
                    "  the plan and the record stay in {}, and whatever stood at its name is as it was; the next run refuses while they stand: look at them, and remove them",
                    landing.stage.display()
                );
            } else {
                place::say_it_stays(&landing.stage, err);
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

/// What begins a plan, so that what is not one is refused without being read whole.
const A_PLAN_BEGINS: &[u8] = b"(format \"archogen-assignment/";

/// The most the engine's reader of a plan can hold, as of any source: past it, no reader of this engine could tell.
const A_PLAN_AT_MOST: u64 = u32::MAX as u64;

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
    /// What this run wrote in the stage, which on a refusal is all that goes.
    written: place::Written,
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
        // Without a checker the stage holds the plan alone, so no name the checker writes needs room.
        let deepest = if checked {
            consume::RECORD_PART
        } else {
            PLAN_IN_STAGE
        };
        place::room(&stage, deepest, out).map_err(|refused| unmake(refused.into()))?;
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
            written: place::Written::default(),
        })
    }

    /// Where the plan is while it is judged.
    fn plan_in_stage(&self) -> PathBuf {
        self.stage.join(PLAN_IN_STAGE)
    }

    /// The stage made, by this run alone, the identity that tells it from what may come to stand at its name taken at once.
    fn open(&mut self) -> Result<(), Unwritten> {
        place::make_private(&self.stage).map_err(|error| {
            // Never the stage, if this call did not make it. One it made and could not close is empty, and goes only if it still is.
            let mut stays = String::new();
            if error.kind() != ErrorKind::AlreadyExists {
                stays = place::remove_empty_stage(&self.stage);
            }
            place::unmake(&self.made);
            usage(
                format!(
                    "cannot make the directory {}: {error}{stays}",
                    self.stage.display()
                ),
                place::repair(&error),
            )
        })?;
        self.made_stage = place::identity(&self.stage);
        Ok(())
    }

    /// `text` written in the stage. What of it could be written goes with the rest on a refusal.
    fn write_plan(&self, text: &str) -> Result<(), Unwritten> {
        self.written
            .write(&self.stage, PLAN_IN_STAGE, text.as_bytes())
            .map_err(|not_written| {
                self.abandon();
                let (message, repair) = match not_written {
                    place::NotWritten::Came(at) => (
                        format!("{} stands where the plan goes, and this run did not write it", at.display()),
                        "something else is using this run's directory: look at it, and move it",
                    ),
                    place::NotWritten::Failed(_, _, error) => (
                        format!("cannot write the plan: {error}"),
                        "check that its disk has room, that no limit holds what this user may write, and that the umask lets it write what it makes",
                    ),
                };
                Unwritten {
                    status: Status::ToolFailure,
                    message,
                    repair: repair.to_owned(),
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
            // A name too long to stand holds no record.
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::NotFound | ErrorKind::InvalidFilename
                ) =>
            {
                return Ok(None)
            }
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
        let read = place::open_regular(&plan).and_then(|mut opened| {
            let mut bytes = Vec::new();
            opened.read_to_end(&mut bytes).map(|_| bytes)
        });
        match read {
            Ok(bytes) if bytes == text.as_bytes() => {}
            Ok(_) => {
                return Err(came(format!(
                    "{} changed while the plan was judged",
                    plan.display()
                )))
            }
            // Its own plan, in its own stage: a umask that closes it to its owner is the likely cause, and nothing came.
            // Gone, closed or no file. Where the checker has just read it, that can only be something that came, and it stays with the stage; only with none can it be the umask.
            Err(error) if checked || error.kind() != std::io::ErrorKind::PermissionDenied => {
                return Err(came(format!(
                    "{} is gone, closed or no file, while the plan was judged: {error}",
                    plan.display()
                )))
            }
            Err(error) => {
                self.abandon();
                return Err(usage(
                    format!("cannot read back the plan it wrote: {error}"),
                    "check the umask: this user must be able to read what it writes",
                ));
            }
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
            let mut stranded = String::new();
            if placed {
                if let Err(error) = std::fs::remove_file(&record) {
                    stranded = format!(
                        "; and the checker's record could not be taken back ({error}), and is at {}: it speaks of no plan that stands, so look at it, and remove it",
                        record.display()
                    );
                }
            } else if moved_in {
                if let Err(error) = std::fs::remove_file(&new) {
                    stranded = format!(
                        "; and the checker's record could not be taken back ({error}), and is at {}, where a checked run refuses while it stands: look at it, and remove it",
                        new.display()
                    );
                }
            }
            if moved_out {
                if let Err(error) = std::fs::rename(&old, &record) {
                    stranded.push_str(&format!(
                        "; and the earlier record could not be put back ({error}), and is at {}",
                        old.display()
                    ));
                }
            }
            if check_was_free {
                let _ = std::fs::remove_dir(&self.check);
            }
            self.abandon();
            stranded
        };
        let failed = |what: String, repair: &str| Unwritten {
            status: Status::ToolFailure,
            message: what,
            repair: repair.to_owned(),
        };
        let mut said = Vec::new();
        if checked {
            // Its mode given here, in the stage, so that it is never in `check` more open than the one it replaces.
            if let Some(from) = &earlier {
                if let Err(why) = place::take_over(&self.stage.join(consume::RECORD), from) {
                    said.push(format!(
                        "the record at {} could not take the mode or the group of the one it replaced: {why}",
                        record.display()
                    ));
                }
            }
            let moved = if check_was_free {
                std::fs::create_dir(&self.check)
            } else {
                Ok(())
            }
            .and_then(|()| std::fs::rename(self.stage.join(consume::RECORD), &new));
            if let Err(error) = moved {
                let stranded = undo(moved_in, moved_out, placed);
                return Err(failed(
                    format!(
                        "cannot put the checker's record in {}: {error}{stranded}",
                        self.check.display()
                    ),
                    place::repair(&error),
                ));
            }
            moved_in = true;
        }
        if earlier.is_some() {
            if let Err(error) = std::fs::rename(&record, &old) {
                let stranded = undo(moved_in, moved_out, placed);
                return Err(failed(
                    format!(
                        "cannot move the earlier record {}: {error}{stranded}",
                        record.display()
                    ),
                    "check its mode and its directory's, and any flag on it",
                ));
            }
            moved_out = true;
        }
        if checked {
            if let Err(error) = std::fs::rename(&new, &record) {
                let stranded = undo(moved_in, moved_out, placed);
                return Err(failed(
                    format!(
                        "cannot give the checker's record its name at {}: {error}{stranded}",
                        record.display()
                    ),
                    "check what stands at that name, and any flag or access list on it or on its directory",
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
            let stranded = undo(moved_in, moved_out, placed);
            return Err(failed(
                format!("cannot write the plan {out}: {error}{stranded}"),
                "check that what stands at that name can be replaced, and any flag on it",
            ));
        }
        if moved_out {
            if let Err(error) = std::fs::remove_file(&old) {
                said.push(format!(
                    "the plan is in place, and the earlier record could not be removed ({error}): it is at {}, and a checked run refuses while it stands: look at it, and remove it",
                    old.display()
                ));
            }
        }
        if std::fs::remove_dir(&self.stage).is_err() {
            said.push(format!(
                "the plan is in place, and {} could not be removed, or holds what came in it, and the next run refuses while it stands: look at it, and remove it",
                self.stage.display()
            ));
        }
        Ok(said)
    }

    /// What this run wrote gone, with the directories it made where they stayed empty. What came stays with the stage, which is said.
    fn abandon(&self) {
        // Only what this run made: what has come to stand at that name since is not this run's to remove.
        if place::identity(&self.stage) == self.made_stage {
            if self.written.checker_ran.get() {
                consume::remove_record(&self.stage);
            }
            self.written.remove_from(&self.stage);
        }
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
    // Only what begins as a plan is read whole, so that a large file of something else is refused for the cost of its first line.
    let mut bytes = Vec::new();
    let read = std::fs::File::open(file).and_then(|mut opened| {
        (&mut opened)
            .take(A_PLAN_BEGINS.len() as u64)
            .read_to_end(&mut bytes)?;
        if bytes == A_PLAN_BEGINS && meta.len() <= A_PLAN_AT_MOST {
            opened.read_to_end(&mut bytes)?;
        }
        Ok(bytes.len())
    });
    if bytes == A_PLAN_BEGINS && meta.len() > A_PLAN_AT_MOST {
        return Err(usage(
            format!(
                "`--out {out}` is larger than this engine reads a plan, 2^32 bytes less one, so nothing can tell it is one"
            ),
            "name another file, or move what stands there",
        ));
    }
    match read {
        Err(error) => Err(usage(
            format!("cannot read `--out {out}`, to tell it is a plan: {error}"),
            "open it, or name another file",
        )),
        Ok(0) => Ok(Some(meta)),
        Ok(_) if std::str::from_utf8(&bytes).is_ok_and(archogen_api::is_a_plan) => Ok(Some(meta)),
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
        Ok(meta) if meta.is_file() => match consume::reads_as_record(&record) {
            Ok(true) => Ok(()),
            Ok(false) if meta.len() == 0 => Ok(()),
            Ok(false) => Err(no_record(&record)),
            Err(error) => Err(usage(
                format!("cannot read {}: {error}", record.display()),
                "open it, or name another file",
            )),
        },
        Ok(_) => Err(no_record(&record)),
    }
}
