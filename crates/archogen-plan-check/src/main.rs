//! `archogen-plan-check` (leaf `M3.5.6.1`): the configuration checker's executable — the contract between the
//! `archogen` front and its sibling, as `M3.5.1`'s design states it. It reads a description and the assignment record a
//! search wrote for it, runs every check in order ([`check::plan_file`]), writes `archogen-result/1` where it is told —
//! on standard output when it is told nowhere — and exits with the status's code, so the front accepts a result only
//! when the record and the exit code agree.
//!
//! ```text
//! archogen-plan-check [--profile ID] [--result PATH] [--] DESCRIPTION PLAN
//! archogen-plan-check --mcp
//! ```
//!
//! `archogen plan-check` hands its arguments here unparsed, as git hands a command to its sibling, so this is also the
//! form `ROADMAP.md` §10.2 writes, `archogen plan-check DESCRIPTION PLAN` (leaf `M3.5.1.4.1`); the front, when it runs
//! the checker over a plan of its own, names the record's path and puts the paths after `--`.
//!
//! ⭐ **The record names what ran and what it judged**: the producer is this executable's name and the sha256 of the
//! file that ran, read through `current_exe`; the subject is the digest of the program the description became and the
//! sha256 of the plan's bytes. A usage error — an argument it cannot take, a file it cannot read — writes no record:
//! nothing was judged.
//!
//! ⛔ **The record lands whole or not at all**: it is written beside its path and renamed into place, so a reader never
//! meets half of one.
//!
//! `--mcp` serves the same judgement as an MCP tool ([`archogen_plan_check::mcp`], leaf `M3.5.1.6.4`), one JSON-RPC
//! message per line on standard input and output until the input ends; its records name the same producer, the file
//! that runs, read once when it starts.

use std::io::{BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use archogen_digest::Digest;
use archogen_outcome::record::{Finding, Producer, Record, Severity};
use archogen_outcome::Status;
use archogen_plan_check::judgement::{self, Inputs, Unjudged};
use archogen_plan_check::mcp::{self, Checker};
use archogen_plan_check::NAME;
use eadl_front::module::{DirectoryModules, ModuleSource};

/// How it is called.
const USAGE: &str =
    "usage: archogen-plan-check [--profile ID] [--result PATH] [--] DESCRIPTION PLAN
       archogen-plan-check --mcp";

/// What `--help` prints.
const HELP: &str = "archogen-plan-check — check a resolved plan against its description, apart from the search that \
wrote it

usage: archogen-plan-check [--profile ID] [--result PATH] [--] DESCRIPTION PLAN
       archogen-plan-check --mcp

  DESCRIPTION      the eADL description, a module or a plain description
  PLAN             the assignment record the search wrote for it, archogen-assignment/2
  --result PATH    where the result record, archogen-result/1, is written; without it, on standard output. It replaces
                   only an empty file or a record this engine reads, and keeps its mode
  --               every argument after it is a path, even one that begins with `-`
  --profile ID     the profile the plan was resolved under (default: rt-static-up-v1)
  --mcp            serve the same check as the MCP tool `plan-check`, which takes the texts rather than paths: one
                   JSON-RPC message per line on standard input and output, until the input ends; alone

The exit code is the result's status, as `archogen` writes it: 0 ok; 2 usage, no record written; 3 stale-input, the
plan written for another description — resolve again; 10 to 13 the description's own status; 70 tool-failure, a
corrupted plan or a checker that could not finish.";

fn main() -> ExitCode {
    // `std::env::args` panics on an argument that is no text, and exit 101 is a code no status has.
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(std::ffi::OsString::into_string)
        .collect();
    let Ok(args) = args else {
        let status = refuse(
            &mut std::io::stderr().lock(),
            Status::Usage,
            "an argument is not UTF-8",
        );
        return ExitCode::from(u8::try_from(status.code()).unwrap_or(2));
    };
    let status = run(
        &args,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout().lock(),
        &mut std::io::stderr().lock(),
    );
    ExitCode::from(u8::try_from(status.code()).unwrap_or(u8::MAX))
}

/// What the arguments ask.
enum Asked {
    Help,
    Version,
    Serve,
    Check {
        profile: Option<String>,
        result: Option<PathBuf>,
        description: PathBuf,
        plan: PathBuf,
    },
}

fn parse(args: &[String]) -> Result<Asked, String> {
    let (mut profile, mut result, mut serve) = (None, None, false);
    let mut rest = args.iter();
    let mut paths: Vec<&String> = Vec::new();
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
            "--mcp" => serve = true,
            "--profile" => value("--profile", &mut profile)?,
            "--result" => value("--result", &mut result)?,
            "--" => {
                paths.extend(rest.by_ref());
                break;
            }
            other if other.starts_with('-') && other.len() > 1 => {
                return Err(format!(
                "`{other}` is no option of this checker; a path that begins with `-` follows `--`"
            ))
            }
            _ => paths.push(arg),
        }
    }
    if serve {
        return if profile.is_none() && result.is_none() && paths.is_empty() {
            Ok(Asked::Serve)
        } else {
            Err("`--mcp` takes no other argument: the tool's calls carry the texts".to_owned())
        };
    }
    match paths.as_slice() {
        [description, plan] => Ok(Asked::Check {
            profile,
            result: result.map(PathBuf::from),
            description: PathBuf::from(description),
            plan: PathBuf::from(plan),
        }),
        _ => Err(format!(
            "two paths, the description and the plan, are expected, and {} were given",
            paths.len()
        )),
    }
}

/// This program as a record names it: its name, and the sha256 of the file that runs.
fn me() -> Result<Producer, String> {
    std::env::current_exe()
        .and_then(std::fs::read)
        .map(|bytes| Producer {
            name: NAME.to_owned(),
            sha256: Digest::of(&bytes),
        })
        .map_err(|error| format!("cannot read its own executable to name it: {error}"))
}

/// Refuse the invocation: nothing was judged, so no record is written.
fn refuse(err: &mut dyn Write, status: Status, why: &str) -> Status {
    let _ = writeln!(err, "{NAME}: {}: {why}", status.slug());
    if status == Status::Usage {
        let _ = writeln!(err, "{USAGE}");
    }
    status
}

fn run(
    args: &[String],
    input: &mut dyn BufRead,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Status {
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
        Ok(Asked::Serve) => {
            let producer = match me() {
                Ok(producer) => producer,
                Err(why) => return refuse(err, Status::ToolFailure, &why),
            };
            return match mcp::serve(input, out, &Checker::new(producer)) {
                Ok(()) => Status::Ok,
                Err(error) => refuse(
                    err,
                    Status::ToolFailure,
                    &format!("the server's streams failed: {error}"),
                ),
            };
        }
        Ok(Asked::Check {
            profile,
            result,
            description,
            plan,
        }) => (profile, result, description, plan),
        Err(why) => return refuse(err, Status::Usage, &why),
    };
    let profile = match judgement::profile(profile.as_deref()) {
        Ok(profile) => profile,
        Err(why) => return refuse(err, Status::Usage, &why),
    };
    if let Some(Err((why, repair))) = result.as_deref().map(replaceable) {
        let refused = refuse(err, Status::Usage, &why);
        let _ = writeln!(err, "  hint: {repair}");
        return refused;
    }
    let producer = match me() {
        Ok(producer) => producer,
        Err(why) => return refuse(err, Status::ToolFailure, &why),
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
    let modules = DirectoryModules::new(description.parent().unwrap_or_else(|| Path::new("")));
    let (description_name, plan_name) = (
        description.display().to_string(),
        plan.display().to_string(),
    );
    let judged = judgement::judge(
        producer,
        &Inputs {
            description_name: &description_name,
            description: &text,
            modules: &modules,
            profile,
            plan_name: &plan_name,
            plan: &plan_bytes,
        },
    );
    if let Some((file, error)) = modules.unreadable().first() {
        return refuse(
            err,
            Status::Usage,
            &format!("cannot read {file}, which the description imports: {error}"),
        );
    }
    let record = match judged {
        Ok(record) => record,
        Err(Unjudged::TooLarge(_)) => {
            return refuse(
                err,
                Status::Usage,
                &format!(
                    "the description {} is too large to address",
                    description.display()
                ),
            )
        }
        Err(Unjudged::Record(refusal)) => {
            return refuse(
                err,
                Status::ToolFailure,
                &format!("its own result does not make a record: {refusal}"),
            )
        }
    };
    render(err, record.findings());
    let status = record.status();
    let written = match &result {
        Some(file) => write(file, &record.write())
            .map_err(|error| format!("cannot write the result record {}: {error}", file.display())),
        None => out
            .write_all(record.write().as_bytes())
            .and_then(|()| out.flush())
            .map_err(|error| format!("cannot write the result record on standard output: {error}")),
    };
    if let Err(why) = written {
        let refused = refuse(err, Status::ToolFailure, &why);
        let _ = writeln!(
            err,
            "  hint: check that its directory can be written and its disk has room; or name another file"
        );
        return refused;
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

/// The most of what stands at `--result` it reads, to tell it is a record. No bound is claimed of what the engine writes: a record past it is refused, truly, with what to do.
const RECORD_AT_MOST: u64 = 1 << 30;

/// What begins a record, so that what is not one is refused without being read whole.
const A_RECORD_BEGINS: &[u8] = b"{\"format\":\"archogen-result/";

/// Whether `--result` can take a result: what stands there nothing, an empty file or a record this engine reads, and what goes beside
/// it free. Anything else may be someone's, the description or the plan among them, so it is refused before anything is judged, and a
/// link is not followed. Only a name found absent is absent. What stops it, and the repair.
fn replaceable(file: &Path) -> Result<(), (String, &'static str)> {
    let shown = file.display();
    let name_a_file = "name a file to hold the result";
    let written = file.as_os_str().as_encoded_bytes();
    if file.file_name().is_none()
        || written
            .last()
            .is_some_and(|last| std::path::is_separator(char::from(*last)))
        || written.ends_with(b"/.")
    {
        return Err((
            format!("`--result {shown}` names no file of its own"),
            name_a_file,
        ));
    }
    let mut part = file.as_os_str().to_owned();
    part.push(".part");
    match std::fs::symlink_metadata(&part) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err((
                format!("cannot use `--result {shown}`: {error}"),
                "name a file with room for `.part` beside it, in a directory this user can reach",
            ))
        }
        Ok(_) => {
            return Err((
                format!("{shown}.part stands, where the result is written first: a run that stopped may have left it"),
                "look at it, and move or remove it",
            ))
        }
    }
    if let Some(parent) = file
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        if !std::fs::metadata(parent).is_ok_and(|meta| meta.is_dir()) {
            return Err((
                format!(
                    "cannot use `--result {shown}`: {} is no directory this user can reach",
                    parent.display()
                ),
                "name a file in a directory that stands",
            ));
        }
    }
    let meta = match std::fs::symlink_metadata(file) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err((
                format!("cannot use `--result {shown}`: {error}"),
                name_a_file,
            ))
        }
        Ok(meta) => meta,
    };
    let no_record = || {
        (
            format!("{shown} stands at `--result` and is no record this engine reads, and a result replaces only a record"),
            "name another file, or move what stands there",
        )
    };
    if !meta.is_file() {
        return Err(no_record());
    }
    let mut bytes = Vec::new();
    let read = std::fs::File::open(file).and_then(|mut opened| {
        (&mut opened)
            .take(A_RECORD_BEGINS.len() as u64)
            .read_to_end(&mut bytes)?;
        if bytes == A_RECORD_BEGINS {
            opened.take(RECORD_AT_MOST).read_to_end(&mut bytes)?;
        }
        Ok(())
    });
    if let Err(error) = read {
        return Err((
            format!("cannot read {shown}: {error}"),
            "open it, or name another file",
        ));
    }
    if bytes.is_empty()
        || Record::read(
            &bytes,
            usize::try_from(RECORD_AT_MOST).unwrap_or(usize::MAX),
        )
        .is_ok()
    {
        Ok(())
    } else {
        Err(no_record())
    }
}

/// Write `text` at `path` whole: beside it, in a file this run makes, then renamed into place, keeping the mode of the file it
/// replaces, private until it does. What already stands beside it is not this run's, so it is refused and left alone, and a link
/// there is never written through.
fn write(path: &Path, text: &str) -> std::io::Result<()> {
    let mut part = path.as_os_str().to_owned();
    part.push(".part");
    let part = PathBuf::from(part);
    let replacing = std::fs::symlink_metadata(path)
        .ok()
        .filter(std::fs::Metadata::is_file);
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    if replacing.is_some() {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&part).map_err(|error| {
        std::io::Error::new(error.kind(), format!("{}: {error}", part.display()))
    })?;
    let written = file.write_all(text.as_bytes());
    drop(file);
    let placed = written
        .and_then(|()| match &replacing {
            Some(meta) => std::fs::set_permissions(&part, meta.permissions()),
            None => Ok(()),
        })
        .and_then(|()| std::fs::rename(&part, path));
    if placed.is_err() {
        let _ = std::fs::remove_file(&part);
    }
    placed
}
