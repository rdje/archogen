//! `archogen build` (leaf `M4.2.3.1`; `docs/decisions/decision_hosted-build.md` §5): from a description to a package that
//! compiles and runs on the hosted playground, through the checked runtime plan, with no hand edit and nothing fetched.
//!
//! # The order of operations is the contract
//!
//! 1. Resolve the profile, and refuse `--locked` before reading anything: there is no lock data to honor it with until
//!    `M4.7`, and a build that took the flag silently would be an unlocked one wearing a locked one's label.
//! 2. Ask the **engine API**'s `resolve`, the search alone. There is exactly one frontend, so `build` can never accept
//!    what `check` refuses, and it reports the check's verdict or the search's rather than a judgement of its own.
//! 3. Run the installed `archogen-plan-check` over the plan the search wrote ([`consume::check_plan`]). Running a program
//!    is the command line's alone (`ROADMAP.md` §10.4).
//! 4. Lower on its acceptance ([`archogen_plan::lower_checked`]) and emit ([`archogen_emit::emit`]), which lowers again and
//!    compares bytes.
//! 5. Write the package in `DIR.building` and rename it into place ([`Place`]).
//!
//! ⛔ **Never a programmatic operation.** §10.4 rules that generation writes a crate tree and stays a human or CI action,
//! so nothing else reaches this file: the command table excludes it, and only the command line holds a launcher.

use std::fs::{Metadata, Permissions};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use archogen_api::{Elaborated, Limits, Plan, Resolve, Response};
use archogen_digest::Digest;
use archogen_emit::{copies, emit, Emitted, Inputs, ASSIGNMENT_FILE, PROVENANCE_FILE};
use archogen_plan::{lower_checked, Acceptance, Generator, Refusal};

use crate::built::{Built, Unbuilt, CARRIED};
use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{self, Checked, Expected};
use crate::launch::Launch;
use crate::place;
use crate::resolve_cmd::print_owed;
use crate::spec::Maturity;
use crate::status::Status;

/// Run `archogen build`.
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
    let out_dir = parsed
        .value("out")
        .expect("the parser guarantees `--out` is required");
    let refuse_locked = |err: &mut dyn Write| -> Option<Status> {
        if !parsed.flag("locked") {
            return None;
        }
        let _ = writeln!(
            err,
            "archogen: {}: `--locked` is not implemented for `archogen build`",
            Status::Unimplemented.slug()
        );
        let _ = writeln!(
            err,
            "  hint: there is no lock data yet to check a build against, and a build that accepted the flag silently \
             would be an unlocked one wearing a locked one's label. Lock data is ROADMAP.md §10.3 and task-tree leaf \
             M4.7. Re-run without `--locked`"
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
    let (Some(plan), Some(elaborated)) = (&resolution.plan, &resolution.elaborated) else {
        if response.judged.is_none() {
            let status = report_not_judged(response, err);
            if !response.diagnostics.is_empty() {
                let _ = write!(err, "{}", response.render_diagnostics());
            }
            return status;
        }
        let _ = write!(err, "{}", response.render_diagnostics());
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path} — nothing was generated",
            response.status.slug(),
            response.diagnostics.len()
        );
        return response.status;
    };
    let mut place = match Place::for_out(out_dir) {
        Ok(place) => place,
        Err(failed) => return refused(&failed, path, err),
    };
    if let Err(failed) = place.open(out_dir) {
        return refused(&failed, path, err);
    }
    let built = Build {
        path,
        response,
        plan,
        elaborated,
        stage: &place.stage,
        made_stage: place.made_stage,
        written: &place.written,
    };
    let emitted = match built.generate(launcher, err) {
        Ok(emitted) => emitted,
        Err(failed) => {
            if !failed.keeps_the_stage() {
                place.abandon();
            }
            let status = refused(&failed, path, err);
            if !failed.keeps_the_stage() {
                place::say_it_stays(&place.stage, err);
            }
            return status;
        }
    };
    match place.land() {
        Ok(landed) => {
            if let Some(remains) = landed.kept {
                let _ = writeln!(
                    err,
                    "archogen: the package is in place, and {} keeps what could not go or what no build wrote: {remains}; \
                     look at it, and remove it",
                    place.replaced.display()
                );
            }
            if let Some(why) = landed.unmoded {
                let _ = writeln!(
                    err,
                    "archogen: the package is in place, and could not take the mode or the group a directory there would have: {why}"
                );
            }
        }
        Err(unplaced) => {
            place.abandon();
            let (why, repair) = unplaced.said(&place);
            let _ = writeln!(
                err,
                "archogen: {}: cannot put the generated package in place at {}: {why}",
                Status::ToolFailure.slug(),
                place.dir.display()
            );
            let _ = writeln!(err, "  hint: {repair}");
            if !matches!(unplaced, Unplaced::StageGone) {
                place::say_it_stays(&place.stage, err);
            }
            return Status::ToolFailure;
        }
    }
    report(parsed, path, &place.dir, plan, &emitted, out);
    Status::Ok
}

/// `mode` with its owner able to write: what it closes to others stays closed, while what went aside is carried from and
/// removed from.
#[cfg(unix)]
fn owner_writes(mode: &Permissions) -> Permissions {
    use std::os::unix::fs::PermissionsExt;
    Permissions::from_mode(mode.mode() | 0o700)
}

/// `mode` with its owner able to write, as far as a platform with no owner's bits can say it.
#[cfg(not(unix))]
#[allow(clippy::permissions_set_readonly_false)]
fn owner_writes(mode: &Permissions) -> Permissions {
    let mut writable = mode.clone();
    writable.set_readonly(false);
    writable
}

/// The stage made, by a call that fails if anything stands there, and private to its owner for as long as it is one, so nobody
/// else reads or fills it whatever `DIR` lets others do. Where `fresh` asks, the mode a new directory takes here comes back, for
/// the package to keep: that alone needs a directory made at it, empty, for the instant before it is closed.
#[cfg(unix)]
fn make_stage(path: &Path, fresh: bool) -> std::io::Result<Option<Permissions>> {
    use std::os::unix::fs::PermissionsExt;
    if !fresh {
        return place::make_private(path).map(|()| None);
    }
    std::fs::create_dir(path)?;
    let mode = std::fs::symlink_metadata(path)?.permissions();
    std::fs::set_permissions(path, Permissions::from_mode(0o700))?;
    Ok(Some(mode))
}

/// The stage made, by a call that fails if anything stands there; a platform with no owner's bits has nothing to close.
#[cfg(not(unix))]
fn make_stage(path: &Path, _fresh: bool) -> std::io::Result<Option<Permissions>> {
    std::fs::create_dir(path).map(|()| None)
}

/// Whether `mode` closes a directory to its owner in any of the three ways cargo needs it open.
#[cfg(unix)]
fn closed_to_owner(mode: &Permissions) -> bool {
    use std::os::unix::fs::PermissionsExt;
    mode.mode() & 0o700 != 0o700
}

/// Whether `mode` closes a directory to writes, as far as a platform with no owner's bits can say it.
#[cfg(not(unix))]
fn closed_to_owner(mode: &Permissions) -> bool {
    mode.readonly()
}

/// What a package's landing leaves to say, the package standing either way.
struct Landed {
    /// What stays at `DIR.replaced`, named.
    kept: Option<String>,
    /// Why the package could not take the mode, or the group, a directory there would have.
    unmoded: Option<std::io::Error>,
}

/// Why a package was not placed.
enum Unplaced {
    /// Something came, while the build ran, to a place it had seen free.
    Appeared(PathBuf),
    /// What stands at the stage's name is no longer the directory this run made.
    StageGone,
    /// A rename failed, and what stood at `DIR` stands there again, if anything did.
    Renaming(std::io::Error),
    /// The second rename failed, and what stood at `DIR` could not be put back from `DIR.replaced`.
    Stranded {
        error: std::io::Error,
        again: std::io::Error,
    },
}

impl Unplaced {
    /// What went wrong, and the repair for it.
    fn said(&self, place: &Place) -> (String, String) {
        match self {
            Self::Appeared(at) => (
                format!("{} appeared while the build ran", at.display()),
                "look at it, and move or remove it; or name another directory".to_owned(),
            ),
            Self::StageGone => (
                format!(
                    "{} is no longer the directory this run made",
                    place.stage.display()
                ),
                "look at it, and at what else writes there: it stays, and the next build refuses while it stands".to_owned(),
            ),
            Self::Renaming(error) if error.kind() == ErrorKind::NotFound => (
                error.to_string(),
                format!(
                    "something moved or removed what the build was moving while it ran: look at {}, and build again",
                    place.dir.display()
                ),
            ),
            Self::Renaming(error) => (
                error.to_string(),
                "a build renames `--out`, so check its parent's permissions, that it is no mount point, and any flag on either"
                    .to_owned(),
            ),
            Self::Stranded { error, again } => (
                format!(
                    "{error}; and what stood there could not be put back ({again}), and is at {}",
                    place.replaced.display()
                ),
                format!(
                    "{} is what stood at {}: look at it, and move it back or remove it",
                    place.replaced.display(),
                    place.dir.display()
                ),
            ),
        }
    }
}

/// Where a package goes: `dir`, by way of a stage beside it. What each route does is `docs/book/src/building.md`'s table, which
/// `tests/build.rs` runs, and why is `docs/decisions/decision_hosted-build.md` §5: this says neither again.
struct Place {
    dir: PathBuf,
    stage: PathBuf,
    /// Where an earlier build waits between the two renames.
    replaced: PathBuf,
    /// What stands at `dir` now, if anything: as it was judged, and the build it is, if it holds one.
    occupied: Option<(Metadata, Option<Built>)>,
    /// The directories this run must make to reach `dir`, innermost first.
    made: Vec<PathBuf>,
    /// Where nothing stands at `dir`: the mode a new directory takes there, which the package keeps.
    fresh: Option<Permissions>,
    /// The directory this run made as the stage, told apart from one that comes to stand at its name.
    made_stage: Option<(u64, u64)>,
    /// What this run wrote in the stage, which on a refusal is all that goes.
    written: place::Written,
}

impl Place {
    /// `out_dir` as a place, or why it is none.
    fn for_out(out_dir: &str) -> Result<Self, Failed> {
        let usage = |message: String, repair: &str| Failed::Usage(message, repair.to_owned());
        let dir = place::read(
            out_dir,
            "directory",
            "name a directory to hold the generated package",
        )?;
        let (stage, replaced) = (
            place::beside(&dir, ".building"),
            place::beside(&dir, ".replaced"),
        );
        place::free(
            &stage,
            out_dir,
            "and a build deletes nothing it did not write: a build that stopped may have left it, or it may hold the report of \
             a disagreement",
        )?;
        place::free(
            &replaced,
            out_dir,
            "and a build deletes nothing it did not write: a build may have stopped between its renames, or kept there what \
             could not go or what came while it ran",
        )?;
        let occupied = match std::fs::symlink_metadata(&dir) {
            Err(error) if error.kind() == ErrorKind::NotFound => None,
            Err(error) => {
                return Err(usage(
                    format!("cannot use `--out {out_dir}`: {error}"),
                    place::repair(&error),
                ))
            }
            // A rename would move a link away and leave what it points at as it was.
            Ok(meta) if !meta.is_dir() => {
                return Err(usage(
                    format!(
                        "`--out {out_dir}` is no directory, and a build replaces only a directory"
                    ),
                    "name the directory itself, not a file or a link to it",
                ))
            }
            Ok(meta) => {
                // A package here must be read, so a directory this user cannot read and search is refused as that, tried and
                // not worked out from its bits, which say nothing of a group's directory or of root.
                let closed = |error: std::io::Error| {
                    let repair = if error.kind() == ErrorKind::PermissionDenied {
                        "open it to this user, with `chmod u+rx` if its mode is what closes it; or name another directory"
                    } else {
                        "name another directory"
                    };
                    usage(
                        format!("cannot read and search `--out {out_dir}`, and a package there could not be read: {error}"),
                        repair,
                    )
                };
                let empty = std::fs::read_dir(&dir).map_err(closed)?.next().is_none();
                if let Err(error) = std::fs::symlink_metadata(dir.join(PROVENANCE_FILE)) {
                    if error.kind() != ErrorKind::NotFound {
                        return Err(closed(error));
                    }
                }
                let earlier = if empty {
                    None
                } else {
                    Some(Self::earlier_build(&dir).map_err(|why| {
                        let repair = if matches!(why, Unbuilt::OtherFormat(_)) {
                            "name a new directory, or take that build away: this engine reads no other format, and moving a file out of it changes nothing"
                        } else {
                            "name a new directory, move what is yours out of it, or open what it could not read"
                        };
                        usage(
                            format!(
                                "`--out {out_dir}` cannot be replaced: a build deletes nothing it did not write, and this cannot \
                                 be told to be only what one wrote: {}",
                                why.said()
                            ),
                            repair,
                        )
                    })?)
                };
                Some((meta, earlier))
            }
        };
        let made = place::missing_parents(&dir);
        Ok(Self {
            dir,
            stage,
            replaced,
            occupied,
            made,
            fresh: None,
            made_stage: None,
            written: place::Written::default(),
        })
    }

    /// `dir` as the earlier build it is, or why it is not exactly a build's.
    fn earlier_build(dir: &Path) -> Result<Built, Unbuilt> {
        let built = Built::read(dir)?;
        match built.edited(dir).or_else(|| built.foreign(dir)) {
            Some(why) => Err(why.said().into()),
            None => Ok(built),
        }
    }

    /// The stage made, by this run alone: a stage that appears meanwhile is another run's. Whatever stops it is the place's, so
    /// `usage`, nothing yet being written. A name too long for its stage can first be seen here when the parents it lies under
    /// do not exist yet, since a path is read only as far as its first missing name.
    fn open(&mut self, out_dir: &str) -> Result<(), Failed> {
        if let Err(refused) = place::make_parents(&self.stage) {
            self.unmake();
            return Err(refused.into());
        }
        match make_stage(&self.stage, self.occupied.is_none()) {
            Ok(fresh) => self.fresh = fresh,
            Err(error) => {
                // Never the stage, if this call did not make it: then it is not this run's. One it made and could not close is
                // empty, and goes only if it still is.
                let mut stays = String::new();
                if error.kind() != ErrorKind::AlreadyExists {
                    stays = place::remove_empty_stage(&self.stage);
                }
                self.unmake();
                return Err(Failed::Usage(
                    format!(
                        "cannot make the directory {}: {error}{stays}",
                        self.stage.display()
                    ),
                    place::repair(&error).to_owned(),
                ));
            }
        }
        self.made_stage = place::identity(&self.stage);
        // What a build writes beneath the stage needs room in a path too: the checker is handed its record's path whole.
        let deepest = copies()
            .iter()
            .map(|(path, _)| *path)
            .max_by_key(|path| path.len())
            .unwrap_or_default();
        place::room(&self.stage, deepest, out_dir).map_err(|mut refused| {
            self.abandon();
            refused.message.push_str(&place::if_it_stays(&self.stage));
            refused.into()
        })
    }

    /// The directories this run made to reach the stage gone, where they stayed empty.
    fn unmake(&self) {
        place::unmake(&self.made);
    }

    /// What this run wrote gone, with whatever it made to reach the stage that stayed empty. What came stays with the stage, which is said.
    fn abandon(&self) {
        // Only what this run made: what has come to stand at that name since is not this run's to remove.
        if place::identity(&self.stage) == self.made_stage {
            if self.written.checker_ran.get() {
                consume::remove_record(&self.stage);
            }
            self.written.remove_from(&self.stage);
        }
        self.unmake();
    }

    /// The package in place, by two renames, and what stood there restored if the second fails, or named if it cannot be. From
    /// there on the package stands: what it replaced is dealt with, and it takes what it keeps of that last, on every way out,
    /// once nothing more is carried to it.
    fn land(&self) -> Result<Landed, Unplaced> {
        // Held once more, the checker having run since: a stage swapped for a link would land the link, or what it points at.
        if place::identity(&self.stage) != self.made_stage {
            return Err(Unplaced::StageGone);
        }
        // Held again, the checker having run since: a rename would silently replace an empty directory that came meanwhile.
        let Some((judged, earlier)) = &self.occupied else {
            if std::fs::symlink_metadata(&self.dir).is_ok() {
                return Err(Unplaced::Appeared(self.dir.clone()));
            }
            std::fs::rename(&self.stage, &self.dir).map_err(Unplaced::Renaming)?;
            let unmoded = self
                .fresh
                .as_ref()
                .and_then(|mode| std::fs::set_permissions(&self.dir, mode.clone()).err());
            return Ok(Landed {
                kept: None,
                unmoded,
            });
        };
        if std::fs::symlink_metadata(&self.replaced).is_ok() {
            return Err(Unplaced::Appeared(self.replaced.clone()));
        }
        // What it keeps is held again too, since its owner may have closed it while the build ran.
        let held = std::fs::symlink_metadata(&self.dir)
            .ok()
            .filter(Metadata::is_dir)
            .unwrap_or_else(|| judged.clone());
        std::fs::rename(&self.dir, &self.replaced).map_err(Unplaced::Renaming)?;
        if let Err(error) = std::fs::rename(&self.stage, &self.dir) {
            return Err(match std::fs::rename(&self.replaced, &self.dir) {
                Ok(()) => Unplaced::Renaming(error),
                Err(again) => Unplaced::Stranded { error, again },
            });
        }
        let kept = self.clear_aside(&held.permissions(), earlier.as_ref());
        let unmoded = place::take_over(&self.dir, &held).err();
        Ok(Landed { kept, unmoded })
    }

    /// What went aside carried and removed, and what is left there, named, if anything. It is opened to its owner for that, and
    /// what stays takes back its mode.
    fn clear_aside(&self, mode: &Permissions, earlier: Option<&Built>) -> Option<String> {
        // It may have been swapped for a link since it was judged: then nothing behind it is this build's, nor is its mode.
        if !std::fs::symlink_metadata(&self.replaced).is_ok_and(|meta| meta.is_dir()) {
            return Some(format!(
                "{} is no longer a directory",
                self.replaced.display()
            ));
        }
        let _ = std::fs::set_permissions(&self.replaced, owner_writes(mode));
        for name in CARRIED {
            let (from, to) = (self.replaced.join(name), self.dir.join(name));
            // What came in the stage under that name is the package's, and a rename would replace it: this stays aside.
            if std::fs::symlink_metadata(&from).is_ok() && std::fs::symlink_metadata(&to).is_err() {
                let _ = std::fs::rename(&from, &to);
            }
        }
        if let Some(earlier) = earlier {
            earlier.remove(&self.replaced);
        }
        if std::fs::remove_dir(&self.replaced).is_ok() {
            return None;
        }
        let remains = self.remaining();
        let _ = std::fs::set_permissions(&self.replaced, mode.clone());
        Some(remains)
    }

    /// What `DIR.replaced` still holds, by name, the first few of it.
    fn remaining(&self) -> String {
        let named = |at: &Path| {
            at.strip_prefix(&self.replaced)
                .unwrap_or(at)
                .display()
                .to_string()
        };
        let mut remains: Vec<String> = Vec::new();
        let mut pending = vec![self.replaced.clone()];
        while let Some(next) = pending.pop() {
            let mut empty = true;
            for entry in std::fs::read_dir(&next).into_iter().flatten().flatten() {
                empty = false;
                let found = entry.path();
                if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                    pending.push(found);
                } else {
                    remains.push(named(&found));
                }
            }
            if empty {
                remains.push(format!("{}/", named(&next)));
            }
        }
        remains.sort();
        let shown = remains
            .iter()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        if remains.len() > 5 {
            format!("{shown} and {} more", remains.len() - 5)
        } else {
            shown
        }
    }
}

/// What one build reads.
struct Build<'a> {
    path: &'a str,
    response: &'a Response,
    plan: &'a Plan,
    elaborated: &'a Elaborated,
    stage: &'a Path,
    /// What the stage is, which it must still be when the package is written in it.
    made_stage: Option<(u64, u64)>,
    /// What this run has written there.
    written: &'a place::Written,
}

impl Build<'_> {
    /// The package written in the stage: the assignment, the checker's record, and what its acceptance lowers to.
    fn generate(&self, launcher: &dyn Launch, err: &mut dyn Write) -> Result<Emitted, Failed> {
        let checked = self.checked(launcher, err)?;
        if place::identity(self.stage) != self.made_stage {
            return Err(Failed::Came(self.stage.to_path_buf()));
        }
        let generator = Generator {
            version: self.response.engine.to_owned(),
            sha256: checked.front,
        };
        let kinds = &self.elaborated.kinds;
        let program = &self.elaborated.program;
        let acceptance = Acceptance::CheckedByFront;
        let lowered = lower_checked(
            program,
            &self.plan.text,
            &checked.text,
            kinds,
            generator.clone(),
            acceptance,
        )
        .map_err(Failed::Refused)?;
        let language = archogen_api::kind_modules();
        let inputs = Inputs {
            program,
            sources: &self.response.sources,
            description: self.path,
            language: &language,
            assignment: &self.plan.text,
            result: &checked.text,
            plan: lowered.plan.text(),
            kinds,
            acceptance,
        };
        let emitted = emit(&inputs, generator).map_err(Failed::Refused)?;
        for file in &emitted.files {
            write_file(self.written, self.stage, &file.path, &file.contents)?;
        }
        Ok(emitted)
    }

    /// The assignment written in the stage, and the installed checker run over it.
    fn checked(&self, launcher: &dyn Launch, err: &mut dyn Write) -> Result<Checked, Failed> {
        let assignment = self.stage.join(ASSIGNMENT_FILE);
        write_file(self.written, self.stage, ASSIGNMENT_FILE, &self.plan.text)?;
        // The record's name is the checker's to fill: here nothing earlier can be this run's.
        if std::fs::symlink_metadata(self.stage.join(consume::RECORD)).is_ok() {
            return Err(Failed::Came(self.stage.join(consume::RECORD)));
        }
        let program = Digest::parse(&format!("sha256:{}", self.plan.program)).ok_or_else(|| {
            Failed::Tool(format!(
                "the engine named the program by `{}`, which is no digest",
                self.plan.program
            ))
        })?;
        let profile = self
            .response
            .judged
            .as_ref()
            .map(|judged| judged.profile)
            .expect("a plan is written for a judged description");
        let expected = Expected {
            description: Path::new(self.path),
            sources: Some(&self.response.sources),
            plan: &assignment,
            program,
            plan_sha256: Digest::of(self.plan.text.as_bytes()),
            profile,
        };
        let ran = &self.written.checker_ran;
        let checked =
            consume::check_plan(launcher, &expected, self.stage, ran, err).map_err(|refusal| {
                // A record that stood there before the checker was handed the stage is no report of this run's.
                Failed::Unchecked(
                    refusal,
                    ran.get() && self.stage.join(consume::RECORD).exists(),
                )
            })?;
        if checked.record.status() != Status::Ok {
            return Err(Failed::Disagreed(Box::new(checked)));
        }
        Ok(checked)
    }
}

/// Why no package was placed.
enum Failed {
    /// The lowering or the emitter refused.
    Refused(Refusal),
    /// The checker's run did not hold together, or could not happen; whether it left a record to report.
    Unchecked(crate::cli::Refusal, bool),
    /// The checker refused the plan the search wrote, so two programs disagree about one input.
    Disagreed(Box<Checked>),
    /// A file could not be written.
    Tool(String),
    /// Something else filled a name the package writes, or the way to one, while the build ran.
    Came(PathBuf),
    /// The invocation cannot be honored: what, and its repair.
    Usage(String, String),
}

impl From<place::Refused> for Failed {
    fn from(refused: place::Refused) -> Self {
        Self::Usage(refused.message, refused.repair)
    }
}

impl Failed {
    /// Whether the stage holds what must not go: what a report of a disagreement needs, the assignment and the record the checker
    /// left; or what something else put there, which is not this run's to delete.
    const fn keeps_the_stage(&self) -> bool {
        matches!(
            self,
            Self::Disagreed(_) | Self::Unchecked(_, true) | Self::Came(_)
        )
    }
}

/// Say why nothing was generated. The status.
fn refused(failed: &Failed, path: &str, err: &mut dyn Write) -> Status {
    match failed {
        Failed::Refused(refusal) => {
            let status = Status::from_slug(refusal.code).unwrap_or(Status::ToolFailure);
            let _ = writeln!(
                err,
                "archogen: {}: {} — nothing was generated from {path}",
                status.slug(),
                refusal.message
            );
            status
        }
        Failed::Unchecked(refusal, kept) => {
            let _ = writeln!(
                err,
                "archogen: {}: {} — nothing was generated from {path}",
                refusal.status.slug(),
                refusal.message
            );
            let _ = writeln!(err, "  hint: {}", refusal.repair);
            if *kept {
                let _ = writeln!(
                    err,
                    "  the assignment and the record stay in the build's `.building` directory, and the next run refuses while they stand: look at them, and remove them"
                );
            }
            refusal.status
        }
        Failed::Disagreed(checked) => {
            let status = checked.record.status();
            let _ = writeln!(
                err,
                "archogen: {}: `{}` refuses the plan the search wrote ({} finding(s)): the search and the checker \
                 disagree, and nothing was generated from {path}",
                status.slug(),
                consume::PLAN_CHECKER,
                checked.record.findings().len(),
            );
            let _ = writeln!(
                err,
                "  hint: report it, with the plan and the record the build left in its `.building` directory: one of \
                 the two programs misreads this description. The next run refuses while they stand, so remove them once it is made"
            );
            status
        }
        Failed::Tool(message) => {
            let _ = writeln!(
                err,
                "archogen: {}: {message} — nothing was generated from {path}",
                Status::ToolFailure.slug()
            );
            let _ = writeln!(
                err,
                "  hint: check that its disk has room, that no limit holds what this user may write, that the umask lets it write what it makes, and that nothing else moved or closed the build's `.building` directory"
            );
            Status::ToolFailure
        }
        Failed::Came(at) => {
            let _ = writeln!(
                err,
                "archogen: {}: {} came in the stage while the build ran, and a build writes over nothing it did not write — nothing \
                 was generated from {path}",
                Status::ToolFailure.slug(),
                at.display()
            );
            let _ = writeln!(
                err,
                "  hint: it stays in the build's `.building` directory, and the next run refuses while it stands: look at it and at what else writes there, then remove it"
            );
            Status::ToolFailure
        }
        Failed::Usage(message, repair) => {
            let _ = writeln!(err, "archogen: {}: {message}", Status::Usage.slug());
            let _ = writeln!(err, "  hint: {repair}");
            Status::Usage
        }
    }
}

/// `contents` written at `name` in `stage`, by the one way a stage is written. A name a module brought that leaves no room in a path is the place's.
fn write_file(
    written: &place::Written,
    stage: &Path,
    name: &str,
    contents: &str,
) -> Result<(), Failed> {
    written
        .write(stage, name, contents.as_bytes())
        .map_err(|not_written| match not_written {
            place::NotWritten::Came(at) => Failed::Came(at),
            place::NotWritten::Failed(what, at, error) => {
                let message = format!("cannot {what} {}: {error}", at.display());
                if error.kind() == ErrorKind::InvalidFilename {
                    Failed::Usage(message, place::repair(&error).to_owned())
                } else {
                    Failed::Tool(message)
                }
            }
        })
}

/// What was built, what it holds and what it did not do.
fn report(
    parsed: &Parsed,
    path: &str,
    dir: &Path,
    plan: &Plan,
    emitted: &Emitted,
    out: &mut dyn Write,
) {
    let _ = writeln!(
        out,
        "{path}: generated `{}` into {}",
        emitted.package,
        dir.display()
    );
    let files: Vec<&str> = emitted.files.iter().map(|f| f.path.as_str()).collect();
    let _ = writeln!(out, "  {} file(s): {}", files.len(), files.join(", "));
    let _ = writeln!(
        out,
        "  checked by `{}`: accepted ({})",
        consume::PLAN_CHECKER,
        dir.join(consume::RECORD).display()
    );
    let _ = writeln!(
        out,
        "  {} task(s), a run of {} tick(s) of 1/{} s",
        emitted.tasks, emitted.horizon, emitted.ticks_per_second
    );
    for (task, source) in &emitted.source_triggers {
        let _ = writeln!(out, "  task `{task}` is released by the source `{source}`");
    }
    print_owed(plan, out);
    if let Maturity::Experimental {
        scope,
        completed_by,
    } = parsed.spec().maturity
    {
        let _ = writeln!(out, "  experimental: {scope}; completed by {completed_by}");
    }
    let _ = writeln!(
        out,
        "    the plan is not complete: what it owes is unmet, and a run establishes no property. See `archogen help build`."
    );
    if std::fs::symlink_metadata(dir).is_ok_and(|meta| closed_to_owner(&meta.permissions())) {
        let _ = writeln!(
            out,
            "  its mode closes it to its owner, and cargo reads it and may need to make `target` and `Cargo.lock` in it: open it if cargo stops on that"
        );
    }
    let manifest = followable(&dir.join("Cargo.toml").display().to_string());
    let _ = writeln!(out, "  next: cargo run --manifest-path {manifest}");
}

/// `manifest` as a shell and cargo read it: a line nobody can follow as printed is no next step.
fn followable(manifest: &str) -> String {
    let unflagged = if manifest.starts_with('-') {
        format!("./{manifest}")
    } else {
        manifest.to_owned()
    };
    let plain = unflagged
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "_-./,:=@%+".contains(c));
    if plain {
        unflagged
    } else {
        format!("'{}'", unflagged.replace('\'', "'\\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::followable;

    #[test]
    fn a_manifest_is_named_as_a_shell_and_cargo_read_it() {
        assert_eq!(followable("out/Cargo.toml"), "out/Cargo.toml");
        assert_eq!(followable("my dir/Cargo.toml"), "'my dir/Cargo.toml'");
        assert_eq!(followable("-x/Cargo.toml"), "./-x/Cargo.toml");
        assert_eq!(followable("it's/Cargo.toml"), "'it'\\''s/Cargo.toml'");
    }
}
