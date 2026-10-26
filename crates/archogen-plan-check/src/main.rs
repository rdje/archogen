//! `archogen-plan-check` (leaf `M3.5.6.1`): the configuration checker's executable — the contract between the
//! `archogen` front and its sibling, as `M3.5.1`'s design states it. It reads a description and the assignment record a
//! search wrote for it, runs every check in order ([`check::plan_file`]), writes `archogen-result/1` where it is told,
//! and exits with the status's code, so the front accepts a result only when the record and the exit code agree.
//!
//! ```text
//! archogen-plan-check [--profile ID] --result PATH -- DESCRIPTION PLAN
//! ```
//!
//! ⭐ **The record names what ran and what it judged**: the producer is this executable's name and the sha256 of the
//! file that ran, read through `current_exe`; the subject is the digest of the program the description became and the
//! sha256 of the plan's bytes. A usage error — an argument it cannot take, a file it cannot read — writes no record:
//! nothing was judged.
//!
//! ⛔ **The record lands whole or not at all**: it is written beside its path and renamed into place, so a reader never
//! meets half of one.

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use archogen_digest::Digest;
use archogen_outcome::record::{Finding, Judged, Producer, Record, Severity, Subject};
use archogen_outcome::Status;
use archogen_plan_check::assignment::program_text;
use archogen_plan_check::check;
use eadl_front::module::{DirectoryModules, ModuleSource};
use eadl_front::SourceMap;
use eadl_model::profile::{Profile, RT_STATIC_UP_V1};

/// This executable's name, the record's producer.
const NAME: &str = "archogen-plan-check";

/// How it is called.
const USAGE: &str = "usage: archogen-plan-check [--profile ID] --result PATH -- DESCRIPTION PLAN";

/// What `--help` prints.
const HELP: &str = "archogen-plan-check — check a resolved plan against its description, apart from the search that \
wrote it

usage: archogen-plan-check [--profile ID] --result PATH -- DESCRIPTION PLAN

  DESCRIPTION      the eADL description, a module or a plain description
  PLAN             the assignment record the search wrote for it, archogen-assignment/1
  --result PATH    where the result record, archogen-result/1, is written
  --profile ID     the profile the plan was resolved under (default: rt-static-up-v1)

The exit code is the result's status, as `archogen` writes it: 0 ok; 2 usage, no record written; 3 stale-input, the
plan written for another description — resolve again; 10 to 13 the description's own status; 70 tool-failure, a
corrupted plan or a checker that could not finish.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let status = run(
        &args,
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    );
    ExitCode::from(u8::try_from(status.code()).unwrap_or(u8::MAX))
}

/// What the arguments ask.
enum Asked {
    Help,
    Version,
    Check {
        profile: Option<String>,
        result: PathBuf,
        description: PathBuf,
        plan: PathBuf,
    },
}

fn parse(args: &[String]) -> Result<Asked, String> {
    let (mut profile, mut result) = (None, None);
    let mut rest = args.iter();
    let mut paths: Option<Vec<&String>> = None;
    while let Some(arg) = rest.next() {
        let mut value = |option: &str, slot: &mut Option<String>| match rest.next() {
            _ if slot.is_some() => Err(format!("`{option}` is given twice")),
            Some(value) => {
                *slot = Some(value.clone());
                Ok(())
            }
            None => Err(format!("`{option}` takes a value")),
        };
        match arg.as_str() {
            "-h" | "--help" => return Ok(Asked::Help),
            "--version" => return Ok(Asked::Version),
            "--profile" => value("--profile", &mut profile)?,
            "--result" => value("--result", &mut result)?,
            "--" => {
                paths = Some(rest.by_ref().collect());
                break;
            }
            other => {
                return Err(format!(
                "`{other}` is no option of this checker; the description and the plan follow `--`"
            ))
            }
        }
    }
    let result = result.ok_or("`--result PATH` names where the result record is written")?;
    match paths.as_deref() {
        Some([description, plan]) => Ok(Asked::Check {
            profile,
            result: PathBuf::from(result),
            description: PathBuf::from(description),
            plan: PathBuf::from(plan),
        }),
        _ => Err("the description and the plan, two paths, follow `--`".to_owned()),
    }
}

/// Refuse the invocation: nothing was judged, so no record is written.
fn refuse(err: &mut dyn Write, status: Status, why: &str) -> Status {
    let _ = writeln!(err, "{NAME}: {}: {why}", status.slug());
    if status == Status::Usage {
        let _ = writeln!(err, "{USAGE}");
    }
    status
}

fn run(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let (profile, result, description, plan) = match parse(args) {
        Ok(Asked::Help) => {
            let _ = writeln!(out, "{HELP}");
            return Status::Ok;
        }
        Ok(Asked::Version) => {
            // The formats it reads and writes, and no value compiled in through `env!`, a channel the trust gate refuses.
            let _ = writeln!(
                out,
                "{NAME}: reads {}, writes {}",
                archogen_plan_check::assignment::FORMAT,
                archogen_outcome::record::FORMAT
            );
            return Status::Ok;
        }
        Ok(Asked::Check {
            profile,
            result,
            description,
            plan,
        }) => (profile, result, description, plan),
        Err(why) => return refuse(err, Status::Usage, &why),
    };
    let profile: &Profile = match profile.as_deref() {
        None => &RT_STATIC_UP_V1,
        Some(id) if id == RT_STATIC_UP_V1.id => &RT_STATIC_UP_V1,
        Some(other) => {
            return refuse(
                err,
                Status::Usage,
                &format!(
                    "the profile `{other}` is not one this checker has; it has `{}`",
                    RT_STATIC_UP_V1.id
                ),
            )
        }
    };
    let me = match std::env::current_exe().and_then(std::fs::read) {
        Ok(bytes) => Digest::of(&bytes),
        Err(error) => {
            return refuse(
                err,
                Status::ToolFailure,
                &format!("cannot read its own executable to name it: {error}"),
            )
        }
    };
    let text = match std::fs::read_to_string(&description) {
        Ok(text) => text,
        Err(error) => {
            return refuse(
                err,
                Status::Usage,
                &format!(
                    "cannot read the description {}: {error}",
                    description.display()
                ),
            )
        }
    };
    let plan_bytes = match std::fs::read(&plan) {
        Ok(bytes) => bytes,
        Err(error) => {
            return refuse(
                err,
                Status::Usage,
                &format!("cannot read the plan {}: {error}", plan.display()),
            )
        }
    };
    // A plan that is no UTF-8 text does not read as a record: check (1) says so.
    let plan_text = String::from_utf8_lossy(&plan_bytes);
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add(description.display().to_string(), text) else {
        return refuse(
            err,
            Status::Usage,
            &format!(
                "the description {} is too large to address",
                description.display()
            ),
        );
    };
    let modules = DirectoryModules::new(description.parent().unwrap_or_else(|| Path::new("")));
    let plan_name = plan.display().to_string();
    let (program, verdict) =
        check::plan_file(&mut sources, id, &modules, profile, &plan_name, &plan_text);
    if let Some((file, error)) = modules.unreadable().first() {
        return refuse(
            err,
            Status::Usage,
            &format!("cannot read {file}, which the description imports: {error}"),
        );
    }
    let (status, findings) = match verdict {
        Ok(()) => (Status::Ok, Vec::new()),
        Err(refused) => (refused.status, refused.findings),
    };
    render(err, &findings);
    let record = Record::new(
        Producer {
            name: NAME.to_owned(),
            sha256: me,
        },
        Subject {
            program: Digest::of(program_text(&program).as_bytes()),
            judged: Judged::Plan(Digest::of(&plan_bytes)),
        },
        profile.id.to_owned(),
        status,
        findings,
    );
    let record = match record {
        Ok(record) => record,
        Err(refusal) => {
            return refuse(
                err,
                Status::ToolFailure,
                &format!("its own result does not make a record: {refusal}"),
            )
        }
    };
    if let Err(error) = write(&result, &record.write()) {
        return refuse(
            err,
            Status::ToolFailure,
            &format!(
                "cannot write the result record {}: {error}",
                result.display()
            ),
        );
    }
    let _ = writeln!(
        err,
        "{NAME}: {}: {} judged against {}",
        status.slug(),
        plan.display(),
        description.display()
    );
    status
}

/// Each finding, as a person reads it.
fn render(err: &mut dyn Write, findings: &[Finding]) {
    for finding in findings {
        let severity = match finding.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        };
        let at = &finding.primary;
        let _ = writeln!(err, "{severity}[{}]: {}", finding.code, finding.message);
        let _ = writeln!(err, "  --> {}:{}:{}", at.source, at.line, at.column);
        for label in &finding.secondary {
            let _ = writeln!(
                err,
                "  ... {}:{}:{}: {}",
                label.source, label.line, label.column, label.message
            );
        }
        let _ = writeln!(err, "  = repair: {}", finding.repair);
    }
}

/// Write `text` at `path` whole: beside it, then renamed into place.
fn write(path: &Path, text: &str) -> std::io::Result<()> {
    let mut part = path.as_os_str().to_owned();
    part.push(".part");
    let part = PathBuf::from(part);
    std::fs::write(&part, text)?;
    std::fs::rename(&part, path)
}
