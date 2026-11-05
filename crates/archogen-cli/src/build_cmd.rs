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
//! 5. Write the package in `DIR.building` and rename it into place, so `DIR` holds at every moment what stood there,
//!    nothing, or the whole package ([`Place`]).
//!
//! ⛔ **Never a programmatic operation.** §10.4 rules that generation writes a crate tree and stays a human or CI action,
//! so nothing else reaches this file: the command table excludes it, and only the command line holds a launcher.

use std::fs::Permissions;
use std::io::{ErrorKind, Write};
use std::path::{Component, Path, PathBuf};

use archogen_api::{Elaborated, Limits, Plan, Resolve, Response};
use archogen_digest::Digest;
use archogen_emit::{copies, emit, Emitted, Inputs, ASSIGNMENT_FILE, PROVENANCE_FILE};
use archogen_plan::{lower_checked, Acceptance, Generator, Refusal};

use crate::built::{Built, CARRIED};
use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{self, Checked, Expected};
use crate::launch::Launch;
use crate::resolve_cmd::{missing_parents, place_repair, print_owed};
use crate::spec::Maturity;
use crate::status::Status;

/// The repair for a path with no room for what a build writes beneath it.
const DEEPER: &str = "name a directory nearer the root";

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
    let place = match Place::for_out(out_dir) {
        Ok(place) => place,
        Err(failed) => return refused(&failed, path, err),
    };
    if let Err(failed) = place.open() {
        return refused(&failed, path, err);
    }
    let built = Build {
        path,
        response,
        plan,
        elaborated,
        stage: &place.stage,
    };
    let emitted = match built.generate(launcher, err) {
        Ok(emitted) => emitted,
        Err(failed) => {
            if !failed.keeps_the_stage() {
                place.abandon();
            }
            return refused(&failed, path, err);
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
                    "archogen: the package is in place, and could not take the mode of what it replaced: {why}"
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
            return Status::ToolFailure;
        }
    }
    report(parsed, path, &place.dir, plan, &emitted, out);
    Status::Ok
}

/// `mode` with its owner able to write: what it closes to others stays closed, while a build writes in a directory that will
/// end with `mode` itself.
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

/// A directory made by a call that fails if anything stands there, and no more open than `mode` from that call on, the umask
/// narrowing it until the mode is given exactly: so there is no moment at which it holds more open than `mode` would.
#[cfg(unix)]
fn make_dir(path: &Path, mode: Option<&Permissions>) -> std::io::Result<()> {
    use std::os::unix::fs::{DirBuilderExt, PermissionsExt};
    let mut builder = std::fs::DirBuilder::new();
    if let Some(mode) = mode {
        builder.mode(mode.mode() & 0o7777);
    }
    builder.create(path)
}

/// A directory made by a call that fails if anything stands there; a platform with no owner's bits has nothing to narrow.
#[cfg(not(unix))]
fn make_dir(path: &Path, _mode: Option<&Permissions>) -> std::io::Result<()> {
    std::fs::create_dir(path)
}

/// Whether `mode` closes a directory to its owner's writes.
#[cfg(unix)]
fn closed_to_owner(mode: &Permissions) -> bool {
    use std::os::unix::fs::PermissionsExt;
    mode.mode() & 0o200 == 0
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
    /// Why the package could not take the mode of what it replaced.
    unmoded: Option<std::io::Error>,
}

/// Why a package was not placed.
enum Unplaced {
    /// Something came, while the build ran, to a place it had seen free.
    Appeared(PathBuf),
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

/// Where a package goes: `dir`, by way of a stage beside it, so that it lands whole: `dir` holds at every moment what stood
/// there, nothing, or the whole package.
///
/// ⛔ **A build deletes nothing it did not write, but for what it names here.** With `remove_dir_all` it removes only a stage
/// this very run made, with whatever was put in it meanwhile; and it takes an empty directory named by `--out`. Of an earlier
/// build it removes each file [`Built`] lists, file by file and only if still as written, and the directories that held them
/// where that leaves them empty. So whatever appears meanwhile in what it replaces, or is edited, stays, and the command says
/// so. What [`crate::built::CARRIED`] names is moved to the new package. Anything else is refused and left alone, a stage some
/// other run left among it: a run that stopped, or the report of a disagreement, which is not this run's to delete. What is put
/// in the stage while it runs lands with the package, the stage being the package, and a carried name stays aside rather than
/// replace it.
///
/// ⚠️ **What this does not hold against.** `std` has no rename that refuses to replace, and no directory-relative open, so each
/// path is held again just before it is used and not at the moment itself. Another program, working in the same directory in that
/// instant, could have an empty directory it made replaced, or have what it puts at a listed path removed, whatever its bytes,
/// through a link if it swaps a directory for one.
struct Place {
    dir: PathBuf,
    stage: PathBuf,
    /// Where an earlier build waits between the two renames.
    replaced: PathBuf,
    /// What stands at `dir` now, if anything: its mode, and the build it is, if it holds one.
    occupied: Option<(std::fs::Permissions, Option<Built>)>,
    /// The directories this run must make to reach `dir`, innermost first.
    made: Vec<PathBuf>,
}

impl Place {
    /// `out_dir` as a place, or why it is none.
    fn for_out(out_dir: &str) -> Result<Self, Failed> {
        let usage = |message: String, repair: &str| Failed::Usage(message, repair.to_owned());
        // Without a trailing separator, so the stage lies beside `dir` and not in it.
        let dir = Path::new(out_dir).components().as_path().to_path_buf();
        if dir.file_name().is_none() {
            return Err(usage(
                format!("`--out {out_dir}` names no directory of its own"),
                "name a directory to hold the generated package",
            ));
        }
        // `missing/../here` reads as absent until `missing` is made, and is `here` after.
        let mut after_a_name = false;
        for part in dir.components() {
            match part {
                Component::Normal(_) => after_a_name = true,
                Component::ParentDir if after_a_name => {
                    return Err(usage(
                        format!(
                        "`--out {out_dir}` goes back through a name, and a name may not exist yet"
                    ),
                        "name the directory without `..` after a name",
                    ))
                }
                _ => {}
            }
        }
        let beside = |suffix: &str| {
            let mut name = dir.as_os_str().to_owned();
            name.push(suffix);
            PathBuf::from(name)
        };
        let (stage, replaced) = (beside(".building"), beside(".replaced"));
        let beside_it = [
            (
                &stage,
                "a build that stopped may have left it, or it may hold the report of a disagreement",
            ),
            (
                &replaced,
                "a build may have stopped between its renames, or kept there what could not go or what came while it ran",
            ),
        ];
        for (other, maybe) in beside_it {
            match std::fs::symlink_metadata(other) {
                Err(error) if error.kind() == ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(usage(
                        format!(
                            "cannot use {}, which `--out {out_dir}` needs beside it: {error}",
                            other.display()
                        ),
                        place_repair(&error),
                    ))
                }
                Ok(_) => {
                    return Err(usage(
                        format!(
                            "{} exists, and a build deletes nothing it did not write: {maybe}",
                            other.display()
                        ),
                        "look at it, and move or remove it; or name another directory",
                    ))
                }
            }
        }
        let occupied = match std::fs::symlink_metadata(&dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                return Err(usage(
                    format!("cannot use `--out {out_dir}`: {error}"),
                    "name a directory to hold the generated package",
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
                // A package here must be read, so a directory this user cannot read and search is refused as that, and
                // not as a build that is not one. What opens it is its owner's, since it would keep that mode.
                let closed = |error: std::io::Error| {
                    let repair = if error.kind() == ErrorKind::PermissionDenied {
                        "open it with `chmod u+rx`, or name another directory"
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
                        usage(
                            format!(
                                "`--out {out_dir}` is not an earlier build as a build wrote it, and a build deletes nothing \
                                 it did not write: {why}"
                            ),
                            "name a new directory, or move what is yours out of it",
                        )
                    })?)
                };
                Some((meta.permissions(), earlier))
            }
        };
        // What a build writes beneath the stage needs room in a path too, and the checker is handed its record's path whole.
        let deepest = copies()
            .iter()
            .map(|(path, _)| *path)
            .max_by_key(|path| path.len())
            .unwrap_or_default();
        if let Err(error) = std::path::absolute(&stage)
            .and_then(|stage| std::fs::symlink_metadata(stage.join(deepest)))
        {
            if error.kind() == ErrorKind::InvalidFilename {
                return Err(usage(
                    format!(
                        "`--out {out_dir}` leaves no room in a path for what a build writes beneath it, `{deepest}`: {error}"
                    ),
                    DEEPER,
                ));
            }
        }
        let made = missing_parents(&dir);
        Ok(Self {
            dir,
            stage,
            replaced,
            occupied,
            made,
        })
    }

    /// `dir` as the earlier build it is, or why it is not exactly a build's.
    fn earlier_build(dir: &Path) -> Result<Built, String> {
        let built = Built::read(dir)?;
        match built.edited(dir).or_else(|| built.foreign(dir)) {
            Some(why) => Err(why),
            None => Ok(built),
        }
    }

    /// The stage made, by this run alone: a stage that appears meanwhile is another run's. From the first it closes what the
    /// mode of what it will replace closes, since it holds what a private directory would keep private, its owner able to
    /// write: that mode comes last ([`Self::land`]), and a directory closed to its owner's writes still takes a package.
    ///
    /// Whatever stops it is the place's, so `usage`, nothing yet being written. A name too long for its stage can first be seen
    /// here when the parents it lies under do not exist yet, since a path is read only as far as its first missing name.
    fn open(&self) -> Result<(), Failed> {
        let refused = |what: String, error: &std::io::Error| {
            Failed::Usage(format!("{what}: {error}"), place_repair(error).to_owned())
        };
        if let Some(parent) = self
            .stage
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            if let Err(error) = std::fs::create_dir_all(parent) {
                self.unmake();
                return Err(refused(
                    format!("cannot make {}, where `--out` lies", parent.display()),
                    &error,
                ));
            }
        }
        let shown = || format!("cannot make the directory {}", self.stage.display());
        let mode = self.occupied.as_ref().map(|(mode, _)| owner_writes(mode));
        if let Err(error) = make_dir(&self.stage, mode.as_ref()) {
            // Never the stage: this call did not make it, so it is not this run's.
            self.unmake();
            return Err(refused(shown(), &error));
        }
        if let Some(mode) = mode {
            if let Err(error) = std::fs::set_permissions(&self.stage, mode) {
                self.abandon();
                return Err(refused(shown(), &error));
            }
        }
        Ok(())
    }

    /// The directories this run made to reach the stage gone, where they stayed empty.
    fn unmake(&self) {
        for made in &self.made {
            let _ = std::fs::remove_dir(made);
        }
    }

    /// The stage gone, with whatever this run made to reach it that stayed empty.
    fn abandon(&self) {
        let _ = std::fs::remove_dir_all(&self.stage);
        self.unmake();
    }

    /// The package in place, by two renames, so `dir` holds at every moment what stood there, nothing, or the whole package,
    /// and what stood there is restored if the second fails, or named if it cannot be. From there on the package stands: what
    /// it replaced goes as [`Place`] says, and it takes that mode last, on every way out, once nothing more is carried to it.
    fn land(&self) -> Result<Landed, Unplaced> {
        // Held again, the checker having run since: a rename would silently replace an empty directory that came meanwhile.
        let Some((judged, earlier)) = &self.occupied else {
            if std::fs::symlink_metadata(&self.dir).is_ok() {
                return Err(Unplaced::Appeared(self.dir.clone()));
            }
            std::fs::rename(&self.stage, &self.dir).map_err(Unplaced::Renaming)?;
            return Ok(Landed {
                kept: None,
                unmoded: None,
            });
        };
        if std::fs::symlink_metadata(&self.replaced).is_ok() {
            return Err(Unplaced::Appeared(self.replaced.clone()));
        }
        // Its mode held again too, since its owner may have closed it while the build ran, and the stage closed as it is.
        let mode = std::fs::symlink_metadata(&self.dir)
            .ok()
            .filter(std::fs::Metadata::is_dir)
            .map_or_else(|| judged.clone(), |meta| meta.permissions());
        let _ = std::fs::set_permissions(&self.stage, owner_writes(&mode));
        std::fs::rename(&self.dir, &self.replaced).map_err(Unplaced::Renaming)?;
        if let Err(error) = std::fs::rename(&self.stage, &self.dir) {
            return Err(match std::fs::rename(&self.replaced, &self.dir) {
                Ok(()) => Unplaced::Renaming(error),
                Err(again) => Unplaced::Stranded { error, again },
            });
        }
        let kept = self.clear_aside(&mode, earlier.as_ref());
        let unmoded = std::fs::set_permissions(&self.dir, mode).err();
        Ok(Landed { kept, unmoded })
    }

    /// What went aside carried and removed as [`Place`] says, and what is left there, named, if anything. It is opened to its
    /// owner for that, and what stays takes back its mode.
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
}

impl Build<'_> {
    /// The package written in the stage: the assignment, the checker's record, and what its acceptance lowers to.
    fn generate(&self, launcher: &dyn Launch, err: &mut dyn Write) -> Result<Emitted, Failed> {
        let checked = self.checked(launcher, err)?;
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
            write_file(&self.stage.join(&file.path), &file.contents)?;
        }
        Ok(emitted)
    }

    /// The assignment written in the stage, and the installed checker run over it.
    fn checked(&self, launcher: &dyn Launch, err: &mut dyn Write) -> Result<Checked, Failed> {
        let assignment = self.stage.join(ASSIGNMENT_FILE);
        write_file(&assignment, &self.plan.text)?;
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
            plan: &assignment,
            program,
            plan_sha256: Digest::of(self.plan.text.as_bytes()),
            profile,
        };
        let checked =
            consume::check_plan(launcher, &expected, self.stage, err).map_err(|refusal| {
                Failed::Unchecked(refusal, self.stage.join(consume::RECORD).exists())
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
    /// The invocation cannot be honored: what, and its repair.
    Usage(String, String),
}

impl Failed {
    /// Whether the stage holds what a report of a disagreement needs: the assignment, and the record the checker left.
    const fn keeps_the_stage(&self) -> bool {
        matches!(self, Self::Disagreed(_) | Self::Unchecked(_, true))
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
                    "  the assignment and the record stay in the build's `.building` directory, for that report"
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
                 the two programs misreads this description"
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
                "  hint: check that its disk has room, and that nothing else moved or closed the build's `.building` directory"
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

/// `contents` written at `file`, its directory made. A name a module brought that leaves no room in a path is the place's.
fn write_file(file: &Path, contents: &str) -> Result<(), Failed> {
    let failed = |what: String, error: std::io::Error| {
        if error.kind() == ErrorKind::InvalidFilename {
            Failed::Usage(format!("{what}: {error}"), DEEPER.to_owned())
        } else {
            Failed::Tool(format!("{what}: {error}"))
        }
    };
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent).map_err(|error| {
            failed(
                format!("cannot make the directory {}", parent.display()),
                error,
            )
        })?;
    }
    std::fs::write(file, contents)
        .map_err(|error| failed(format!("cannot write {}", file.display()), error))
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
            "  its mode closes it to its owner's writes, and cargo writes `target` and `Cargo.lock` in it: open it before running it"
        );
    }
    let _ = writeln!(
        out,
        "  next: cargo run --manifest-path {}/Cargo.toml",
        dir.display()
    );
}
