//! `archogen-plan-check` (leaf `M3.5.6.1`): the configuration checker's executable — the contract between the
//! `archogen` front and its sibling, as `M3.5.1`'s design states it. It reads a description and the assignment record a
//! search wrote for it, runs every check in order ([`check::plan_file`]), writes `archogen-result/1` where it is told —
//! on standard output when it is told nowhere — and exits with the status's code, so the front accepts a result only
//! when the record and the exit code agree.
//!
//! ```text
//! archogen-plan-check [--profile ID] [--target FILE] [--result PATH] [--] DESCRIPTION PLAN
//! archogen-plan-check --mcp
//! ```
//!
//! `--target FILE` (leaf `M4.1.7.5.4.4`; `DC-H2`) judges a plan of a system on a target, `archogen-assignment/3`, with
//! the system put on the target the file describes ([`archogen_plan_check::check::plan_file_on_target`]): a file the
//! system cannot read is a usage error, as an unreadable description is, and the record is the same format, its subject
//! the program and the plan's bytes, whose `(target …)` check (1) holds to the file. Without it, nothing changes.
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
use archogen_plan_check::judgement::{self, Inputs, TargetFile, Unjudged};
use archogen_plan_check::mcp::{self, Checker};
use archogen_plan_check::NAME;
use eadl_front::module::{DirectoryModules, ModuleSource};

/// How it is called.
const USAGE: &str =
    "usage: archogen-plan-check [--profile ID] [--target FILE] [--result PATH] [--] DESCRIPTION PLAN
       archogen-plan-check --mcp";

/// What `--help` prints.
const HELP: &str = "archogen-plan-check — check a resolved plan against its description, apart from the search that \
wrote it

usage: archogen-plan-check [--profile ID] [--target FILE] [--result PATH] [--] DESCRIPTION PLAN
       archogen-plan-check --mcp

  DESCRIPTION      the eADL description, a module or a plain description
  PLAN             the assignment record the search wrote for it: archogen-assignment/2, or archogen-assignment/3 of a
                   system on a target, judged with `--target`
  --target FILE    the target's eADL file the plan was resolved on: its system is put on the target and the plan judged
                   there. A plan of another target, or of none, is stale-input, as a plan on a target is without it
  --result PATH    where the result record, archogen-result/1, is written; without it, on standard output. It replaces
                   only an empty file or a record this engine reads, and keeps its mode
  --               every argument after it is a path, even one that begins with `-`
  --profile ID     the profile the plan was resolved under (default: rt-static-up-v1)
  --mcp            serve the same check as the MCP tool `plan-check`, which takes the texts rather than paths: one
                   JSON-RPC message per line on standard input and output, until the input ends; alone

The exit code is the result's status, as `archogen` writes it: 0 ok; 2 usage, no record written; 3 stale-input, the
plan written for another description — resolve again; 10 to 13 the description's own status; 14 analysis-inconclusive, its memory ceiling reached, no record written; 70 tool-failure, a
corrupted plan or a checker that could not finish.";

/// What this process may hold at once, whatever a description's shape asks of it.
#[global_allocator]
static CEILING: archogen_transport::ceiling::Ceiling = archogen_transport::ceiling::Ceiling;

fn main() -> ExitCode {
    if let Err(why) = archogen_transport::ceiling::from_environment(true) {
        let status = refuse_with(
            &mut std::io::stderr().lock(),
            Status::Usage,
            &why,
            "give a whole number of MiB, or leave it unset",
        );
        return ExitCode::from(u8::try_from(status.code()).unwrap_or(2));
    }
    // `std::env::args` panics on an argument that is no text, and exit 101 is a code no status has.
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(std::ffi::OsString::into_string)
        .collect();
    let Ok(args) = args else {
        let status = refuse_with(
            &mut std::io::stderr().lock(),
            Status::Usage,
            "an argument is not UTF-8",
            "give paths and names this tool can print",
        );
        return ExitCode::from(u8::try_from(status.code()).unwrap_or(2));
    };
    if args.iter().any(|arg| arg.contains(['\n', '\r'])) {
        let status = refuse_with(
            &mut std::io::stderr().lock(),
            Status::Usage,
            "an argument holds a line break, which could write a line of a run's own",
            "give paths and names this tool can print",
        );
        return ExitCode::from(u8::try_from(status.code()).unwrap_or(2));
    }
    let status = run(
        &args,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout().lock(),
        &mut eadl_front::Printable(std::io::stderr().lock()),
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
        target: Option<PathBuf>,
        description: PathBuf,
        plan: PathBuf,
    },
}

fn parse(args: &[String]) -> Result<Asked, String> {
    let (mut profile, mut result, mut target, mut serve) = (None, None, None, false);
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
            "--target" => value("--target", &mut target)?,
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
        return if profile.is_none() && result.is_none() && target.is_none() && paths.is_empty() {
            Ok(Asked::Serve)
        } else {
            Err("`--mcp` takes no other argument: the tool's calls carry the texts".to_owned())
        };
    }
    match paths.as_slice() {
        [description, plan] => Ok(Asked::Check {
            profile,
            result: result.map(PathBuf::from),
            target: target.map(PathBuf::from),
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

/// A refusal with what to do about it, as every refusal states (`docs/book/src/cli.md`).
fn refuse_with(err: &mut dyn Write, status: Status, why: &str, repair: &str) -> Status {
    let _ = writeln!(err, "{NAME}: {}: {why}", status.slug());
    let _ = writeln!(err, "  hint: {repair}");
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
    let (profile, result, target, description, plan) = match parse(args) {
        Ok(Asked::Help) => {
            let _ = writeln!(out, "{HELP}");
            return Status::Ok;
        }
        Ok(Asked::Version) => {
            // The formats it reads and writes, and no value compiled in through `env!`, a channel the trust gate refuses.
            let _ = writeln!(
                out,
                "{NAME}: reads {}, and {} with --target, writes {}",
                archogen_plan_check::assignment::FORMAT,
                archogen_plan_check::assignment::FORMAT_3,
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
            target,
            description,
            plan,
        }) => (profile, result, target, description, plan),
        Err(why) => return refuse(err, Status::Usage, &why),
    };
    let profile = match judgement::profile(profile.as_deref()) {
        Ok(profile) => profile,
        Err(why) => return refuse(err, Status::Usage, &why),
    };
    // What was judged to be replaceable, held to the end: what comes there while this runs is not.
    let standing = result.as_deref().map(identity_of);
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
            return refuse_with(
                err,
                Status::Usage,
                &format!(
                    "cannot read the description {}: {error}",
                    description.display()
                ),
                "name a file that exists, is UTF-8 text and this user can read",
            )
        }
    };
    let plan_bytes = match std::fs::read(&plan) {
        Ok(bytes) => bytes,
        Err(error) => {
            return refuse_with(
                err,
                Status::Usage,
                &format!("cannot read the plan {}: {error}", plan.display()),
                "name a file that exists and this user can read",
            )
        }
    };
    // A target's file, where one is named: one the system cannot read is the invocation's, as an unreadable description is.
    let target_text = match &target {
        None => None,
        Some(file) => match std::fs::read_to_string(file) {
            Ok(text) => Some(text),
            Err(error) => {
                return refuse_with(
                    err,
                    Status::Usage,
                    &format!("cannot read the target {}: {error}", file.display()),
                    "name a target's file that exists, is UTF-8 text and this user can read",
                )
            }
        },
    };
    // A stream that read as nothing was given nothing: `archogen plan-check` starts this with no input, so `/dev/stdin` is empty there, which says nothing of what the plan was written from.
    let target_given = target
        .as_ref()
        .zip(target_text.as_deref())
        .map(|(file, text)| ("target", file, text.is_empty()));
    for (what, given, empty) in [
        ("description", &description, text.is_empty()),
        ("plan", &plan, plan_bytes.is_empty()),
    ]
    .into_iter()
    .chain(target_given)
    {
        if empty && !std::fs::metadata(given).is_ok_and(|meta| meta.is_file()) {
            return refuse_with(
                err,
                Status::Usage,
                &format!(
                    "the {what} {} is no regular file and read as nothing, so there is nothing to judge",
                    given.display()
                ),
                "give it as a file: `archogen plan-check` gives this program no input",
            );
        }
    }
    let modules = DirectoryModules::new(description.parent().unwrap_or_else(|| Path::new("")));
    let (description_name, plan_name) = (
        description.display().to_string(),
        plan.display().to_string(),
    );
    let target_name = target.as_ref().map(|file| file.display().to_string());
    let judged = judgement::judge_on(
        producer,
        &Inputs {
            description_name: &description_name,
            description: &text,
            modules: &modules,
            profile,
            plan_name: &plan_name,
            plan: &plan_bytes,
        },
        target_name
            .as_deref()
            .zip(target_text.as_deref())
            .map(|(name, text)| TargetFile { name, text }),
    );
    if let Some((file, error)) = modules.unreadable().first() {
        return refuse_with(
            err,
            Status::Usage,
            &format!("cannot read {file}, which the description imports: {error}"),
            "name a module that exists, is UTF-8 text and this user can read",
        );
    }
    let record = match judged {
        Ok(record) => record,
        Err(Unjudged::TooLarge(error)) => {
            return refuse(
                err,
                Status::Usage,
                &format!("a file is too large to address: {error}"),
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
        Some(file) => write(file, &record.write(), standing.flatten()).map_err(|error| {
            (
                error.kind(),
                format!("cannot write the result record {}: {error}", file.display()),
            )
        }),
        None => out
            .write_all(record.write().as_bytes())
            .and_then(|()| out.flush())
            .map_err(|error| {
                (
                    error.kind(),
                    format!("cannot write the result record on standard output: {error}"),
                )
            }),
    };
    if let Err((kind, why)) = written {
        let refused = refuse(err, Status::ToolFailure, &why);
        let _ = writeln!(
            err,
            "  hint: {}",
            if kind == std::io::ErrorKind::AlreadyExists {
                "look at what stands there now, move it if it should stay, and run this again; or name another file"
            } else {
                "check that its directory can be written, that no limit or access list holds what this user writes there, and that its disk has room; or name another file"
            }
        );
        return refused;
    }
    let on = target
        .as_ref()
        .map(|file| format!(" on {}", file.display()))
        .unwrap_or_default();
    let _ = writeln!(
        err,
        "{NAME}: {}: {} judged against {}{on}",
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
    if !meta.is_file() || meta.len() > RECORD_AT_MOST {
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

/// What tells what stands at a name from what may come to stand there, or be written into it.
type Standing = (u64, u64, u64, i64, i64);

/// What stands at `path`, if anything does.
#[cfg(unix)]
fn identity_of(path: &Path) -> Option<Standing> {
    use std::os::unix::fs::MetadataExt;
    std::fs::symlink_metadata(path).ok().map(|meta| {
        (
            meta.dev(),
            meta.ino(),
            meta.size(),
            meta.mtime(),
            meta.mtime_nsec(),
        )
    })
}

/// What stands at `path`, if anything does: here, how long it is.
#[cfg(not(unix))]
fn identity_of(path: &Path) -> Option<Standing> {
    std::fs::symlink_metadata(path)
        .ok()
        .map(|meta| (0, 0, meta.len(), 0, 0))
}

/// Write `text` at `path` whole: beside it, in a file this run makes, then renamed into place, keeping the mode of the file it
/// replaces, private until it does. What already stands beside it is not this run's, so it is refused and left alone, and a link
/// there is never written through.
fn write(path: &Path, text: &str, judged: Option<Standing>) -> std::io::Result<()> {
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
        .and_then(|()| {
            // Looked at again just before it is used, as the chapter says: what came meanwhile is no record this judged.
            if identity_of(path) == judged {
                std::fs::rename(&part, path)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "what stands at that name is no longer what stood there when this began, and it is left as it is",
                ))
            }
        });
    if let Err(error) = &placed {
        if let Err(stuck) = std::fs::remove_file(&part) {
            return Err(std::io::Error::new(
                error.kind(),
                format!(
                    "{error}; and {} could not be removed ({stuck}), and the next run refuses while it stands: look at it, and remove it",
                    part.display()
                ),
            ));
        }
    }
    placed
}
