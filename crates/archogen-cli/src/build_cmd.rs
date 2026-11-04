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
//! 5. Write the package in `DIR.building` and rename it into place, so `DIR` is at every moment a whole package or
//!    absent ([`Place`]).
//!
//! ⛔ **Never a programmatic operation.** §10.4 rules that generation writes a crate tree and stays a human or CI action,
//! so nothing else reaches this file: the command table excludes it, and only the command line holds a launcher.

use std::io::Write;
use std::path::{Path, PathBuf};

use archogen_api::{Elaborated, Limits, Plan, Resolve, Response};
use archogen_digest::Digest;
use archogen_emit::{emit, Emitted, Inputs, ASSIGNMENT_FILE};
use archogen_plan::{lower_checked, Acceptance, Generator, Refusal};

use crate::built::Built;
use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{self, Checked, Expected};
use crate::launch::Launch;
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
        Ok(leftover) => {
            if let Some(error) = leftover {
                let _ = writeln!(
                    err,
                    "archogen: the package is in place, and the earlier one stays at {}: {error}; remove it",
                    place.replaced.display()
                );
            }
        }
        Err(error) => {
            place.abandon();
            let _ = writeln!(
                err,
                "archogen: {}: cannot put the generated package in place at {}: {error}",
                Status::ToolFailure.slug(),
                place.dir.display()
            );
            let _ = writeln!(
                err,
                "  hint: give `--out` a writable directory on the same filesystem as the project"
            );
            return Status::ToolFailure;
        }
    }
    report(parsed, path, &place.dir, plan, &emitted, out);
    Status::Ok
}

/// Where a package goes: `dir`, by way of a stage beside it, so that it lands whole.
///
/// ⛔ **A build deletes nothing it did not write.** It removes only a stage this very run made, and an earlier build that
/// [`Built`] finds exactly as a build wrote it: every file listed and unchanged, nothing unlisted. Anything else is
/// refused and left alone, a stage some other run left among it: a run that stopped, or the report of a disagreement,
/// which is not this run's to delete.
struct Place {
    dir: PathBuf,
    stage: PathBuf,
    /// Where an earlier build waits between the two renames.
    replaced: PathBuf,
    /// Whether something stands at `dir`.
    occupied: bool,
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
        let beside = |suffix: &str| {
            let mut name = dir.as_os_str().to_owned();
            name.push(suffix);
            PathBuf::from(name)
        };
        let (stage, replaced) = (beside(".building"), beside(".replaced"));
        for other in [&stage, &replaced] {
            if std::fs::symlink_metadata(other).is_ok() {
                return Err(usage(
                    format!(
                        "{} exists, left by a build that stopped or holding the report of a disagreement, and a build \
                         deletes nothing it did not write",
                        other.display()
                    ),
                    "look at it, and move or remove it; or name another directory",
                ));
            }
        }
        let occupied = match std::fs::symlink_metadata(&dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
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
            Ok(_) => true,
        };
        if occupied {
            let empty = std::fs::read_dir(&dir)
                .map_err(|error| {
                    usage(
                        format!("cannot use `--out {out_dir}`: {error}"),
                        "name another directory",
                    )
                })?
                .next()
                .is_none();
            if !empty {
                Self::earlier_build(&dir).map_err(|why| {
                    usage(
                        format!(
                            "`--out {out_dir}` is not an earlier build as a build wrote it, and a build deletes nothing \
                             it did not write: {why}"
                        ),
                        "name a new directory, or move what is yours out of it",
                    )
                })?;
            }
        }
        let mut made = Vec::new();
        let mut parent = dir.parent();
        while let Some(up) = parent.filter(|up| !up.as_os_str().is_empty() && !up.exists()) {
            made.push(up.to_path_buf());
            parent = up.parent();
        }
        Ok(Self {
            dir,
            stage,
            replaced,
            occupied,
            made,
        })
    }

    /// Why `dir` is not exactly a build's, if it is not.
    fn earlier_build(dir: &Path) -> Result<(), String> {
        let built = Built::read(dir)?;
        match built.edited(dir).or_else(|| built.foreign(dir)) {
            Some(why) => Err(why),
            None => Ok(()),
        }
    }

    /// The stage made, by this run alone: a stage that appears meanwhile is another run's.
    fn open(&self) -> Result<(), Failed> {
        let opened = self
            .stage
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::create_dir(&self.stage));
        opened.map_err(|error| {
            Failed::Tool(format!(
                "cannot make the directory {}: {error}",
                self.stage.display()
            ))
        })
    }

    /// The stage gone, with whatever this run made to reach it that stayed empty.
    fn abandon(&self) {
        let _ = std::fs::remove_dir_all(&self.stage);
        for made in &self.made {
            let _ = std::fs::remove_dir(made);
        }
    }

    /// The package in place, and any trouble removing what it replaced, which leaves the new package standing. Two
    /// renames, so `dir` is at every moment a whole package or absent, and an earlier build is restored if the second
    /// fails.
    fn land(&self) -> std::io::Result<Option<std::io::Error>> {
        if self.occupied {
            std::fs::rename(&self.dir, &self.replaced)?;
        }
        if let Err(error) = std::fs::rename(&self.stage, &self.dir) {
            if self.occupied {
                let _ = std::fs::rename(&self.replaced, &self.dir);
            }
            return Err(error);
        }
        if self.occupied {
            if let Err(error) = std::fs::remove_dir_all(&self.replaced) {
                return Ok(Some(error));
            }
        }
        Ok(None)
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
        let language: Vec<String> = archogen_api::kind_modules()
            .into_iter()
            .map(|(name, _)| name)
            .collect();
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
            write_file(&self.stage.join(&file.path), &file.contents).map_err(Failed::Tool)?;
        }
        Ok(emitted)
    }

    /// The assignment written in the stage, and the installed checker run over it.
    fn checked(&self, launcher: &dyn Launch, err: &mut dyn Write) -> Result<Checked, Failed> {
        let assignment = self.stage.join(ASSIGNMENT_FILE);
        write_file(&assignment, &self.plan.text).map_err(Failed::Tool)?;
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
                "  hint: give `--out` a writable directory on the same filesystem as the project"
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

/// `contents` written at `file`, its directory made.
fn write_file(file: &Path, contents: &str) -> Result<(), String> {
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("cannot make the directory {}: {error}", parent.display()))?;
    }
    std::fs::write(file, contents)
        .map_err(|error| format!("cannot write {}: {error}", file.display()))
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
    let _ = writeln!(
        out,
        "  next: cargo run --manifest-path {}/Cargo.toml",
        dir.display()
    );
}
