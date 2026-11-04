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
//! 5. Write the package in `DIR.building` and rename it into place, so it lands whole or not at all.
//!
//! ⛔ **Never a programmatic operation.** §10.4 rules that generation writes a crate tree and stays a human or CI action,
//! so nothing else reaches this file: the command table excludes it, and only the command line holds a launcher.

use std::io::Write;
use std::path::{Path, PathBuf};

use archogen_api::{Elaborated, Limits, Plan, Resolve, Response};
use archogen_digest::Digest;
use archogen_emit::{emit, Emitted, Inputs};
use archogen_plan::{lower_checked, Acceptance, Generator, Refusal};

use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{self, Checked, Expected};
use crate::launch::Launch;
use crate::resolve_cmd::print_owed;
use crate::spec::Maturity;
use crate::status::Status;

/// The assignment's name in the package, which the checker judges there.
const ASSIGNMENT: &str = "assignment.eadl";

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
    let dir = match place_for(out_dir) {
        Ok(dir) => dir,
        Err(refusal) => return refused(&refusal, path, err),
    };
    let stage = building(&dir);
    let built = Build {
        path,
        response,
        plan,
        elaborated,
        stage: &stage,
    };
    let emitted = match built.generate(launcher, err) {
        Ok(emitted) => emitted,
        Err(failed) => return failed.report(&stage, path, err),
    };
    if let Err(error) = place(&stage, &dir) {
        let _ = std::fs::remove_dir_all(&stage);
        let _ = writeln!(
            err,
            "archogen: {}: cannot put the generated package in place at {}: {error}",
            Status::ToolFailure.slug(),
            dir.display()
        );
        let _ = writeln!(
            err,
            "  hint: give `--out` a writable directory on the same filesystem as the project"
        );
        return Status::ToolFailure;
    }
    report(parsed, path, &dir, plan, &emitted, out);
    Status::Ok
}

/// `out_dir` as the place a package goes, or why it cannot: a path naming a directory of its own, which is absent, empty
/// or an earlier build's. A build deletes nothing it did not write.
fn place_for(out_dir: &str) -> Result<PathBuf, Failed> {
    // Without a trailing separator, so `DIR.building` lies beside `DIR` and not in it.
    let dir = Path::new(out_dir).components().as_path().to_path_buf();
    if dir.file_name().is_none() {
        return Err(Failed::Usage(
            format!("`--out {out_dir}` names no directory of its own"),
            "name a directory to hold the generated package".to_owned(),
        ));
    }
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(dir),
        Err(error) => {
            return Err(Failed::Usage(
                format!("cannot use `--out {out_dir}`: {error}"),
                "name a directory to hold the generated package".to_owned(),
            ))
        }
    };
    let empty = entries.count() == 0;
    let earlier_build = std::fs::read_to_string(dir.join("provenance.json"))
        .is_ok_and(|text| text.contains("\"format\": \"archogen-build-provenance/"));
    if empty || earlier_build {
        Ok(dir)
    } else {
        Err(Failed::Usage(
            format!(
                "`--out {out_dir}` holds files and no earlier build, and a build deletes nothing it did not write"
            ),
            "name a new directory, or an earlier build's".to_owned(),
        ))
    }
}

/// Where the package is written before it lands at `dir`: beside it, so the rename stays on one filesystem.
fn building(dir: &Path) -> PathBuf {
    let mut stage = dir.as_os_str().to_owned();
    stage.push(".building");
    PathBuf::from(stage)
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

    /// The stage made anew, the assignment written there and the installed checker run over it.
    fn checked(&self, launcher: &dyn Launch, err: &mut dyn Write) -> Result<Checked, Failed> {
        match std::fs::remove_dir_all(self.stage) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(Failed::Tool(format!(
                    "cannot remove the earlier {}: {error}",
                    self.stage.display()
                )))
            }
        }
        let assignment = self.stage.join(ASSIGNMENT);
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
            consume::check_plan(launcher, &expected, self.stage, err).map_err(Failed::Unchecked)?;
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
    /// The checker's run did not hold together, or could not happen.
    Unchecked(crate::cli::Refusal),
    /// The checker refused the plan the search wrote, so two programs disagree about one input.
    Disagreed(Box<Checked>),
    /// A file could not be written.
    Tool(String),
    /// The invocation cannot be honored: what, and its repair.
    Usage(String, String),
}

impl Failed {
    /// Say so; the stage goes, unless it holds what a report of the disagreement needs. The status.
    fn report(self, stage: &Path, path: &str, err: &mut dyn Write) -> Status {
        if !matches!(self, Self::Disagreed(_)) {
            let _ = std::fs::remove_dir_all(stage);
        }
        refused(&self, path, err)
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
        Failed::Unchecked(refusal) => {
            let _ = writeln!(
                err,
                "archogen: {}: {} — nothing was generated from {path}",
                refusal.status.slug(),
                refusal.message
            );
            let _ = writeln!(err, "  hint: {}", refusal.repair);
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

/// The stage renamed to `dir`, whatever [`place_for`] admitted there gone first.
fn place(stage: &Path, dir: &Path) -> std::io::Result<()> {
    match std::fs::remove_dir_all(dir) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    std::fs::rename(stage, dir)
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
    let mut files: Vec<&str> = emitted.files.iter().map(|f| f.path.as_str()).collect();
    files.push(ASSIGNMENT);
    files.push(consume::RECORD);
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
        let _ = writeln!(
            out,
            "  task `{task}` is released by the source `{source}`, as the plan infers"
        );
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
