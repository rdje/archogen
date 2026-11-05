//! The front's contract for a result it consumes (leaf `M3.5.1.4.3`; `M3.5.1`'s design, *The contract between front
//! and sibling*): `archogen resolve` (`M3.4`) runs the configuration checker over the plan it wrote, and builds on that
//! plan only when the checker's record, the checker's process and what the front computed itself all agree.
//!
//! ⭐ **Two channels, and the front's own reckoning.** The record says what the checker found; the process's exit code
//! says it again, so one fault in the record's writer or reader cannot turn a refusal into an acceptance alone. The
//! record must also name the program the launcher verified as its producer, and as its subject the very program and
//! plan the front computed digests of, under the profile the front asked for — so a record left by another run, or
//! written about other inputs, is never taken for this one's.
//!
//! ⛔ **Anything else is `tool-failure`, naming what disagreed**: a signal, a code no status has, no record, a record
//! that does not read, a code and a status that differ, another producer, subject or profile. And a checker that finds
//! the description itself wanting — a status of the description's own, or a stale plan the front wrote moments ago —
//! is the two roles disagreeing about one input, which no single verdict may hide: `tool-failure` naming both.

use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use archogen_digest::Digest;
use archogen_outcome::record::{Judged, Record};

use crate::cli::Refusal;
use crate::launch::{Ended, Launch};
use crate::status::Status;

/// The configuration checker's executable.
pub const PLAN_CHECKER: &str = "archogen-plan-check";

/// The record's name in the front's own directory.
pub const RECORD: &str = archogen_emit::RESULT_FILE;

/// The longest name the checker writes beneath the directory it is given: its record, written first beside itself and renamed
/// into place (`crates/archogen-plan-check/src/main.rs`).
pub const RECORD_PART: &str = "plan-check.json.part";

/// The most of a record the front reads.
const RECORD_LIMIT: usize = 1 << 20;

/// What the front computed itself, which the checker's record must name.
#[derive(Debug, Clone, Copy)]
pub struct Expected<'a> {
    /// The description, as the front read it.
    pub description: &'a Path,
    /// Its text, as the front read it, which the checker must be able to read again.
    pub text: Option<&'a str>,
    /// The plan the front wrote for it.
    pub plan: &'a Path,
    /// The digest of the elaborated program the description became, as the front computed it.
    pub program: Digest,
    /// The sha256 of the plan's bytes, as the front wrote them.
    pub plan_sha256: Digest,
    /// The profile the plan was resolved under.
    pub profile: &'a str,
}

/// What the front believes of one run of the checker.
#[derive(Debug, Clone)]
pub struct Checked {
    /// The record: its status the checker's verdict on the plan, `ok`, or `tool-failure` for a plan the rules refuse,
    /// which is a corrupted plan (`ROADMAP.md` §5.5).
    pub record: Record,
    /// The record's text, as the checker wrote it, which a build's lowering reads again and names by its digest.
    pub text: String,
    /// The sha256 of the front that ran the checker, as the launcher verified it.
    pub front: Digest,
}

/// Run the configuration checker through `launcher` over the plan `expected` names, its record written in `dir`, a
/// directory of the front's own under the command's output; its standard error relayed to `err`.
///
/// Returns what the front can believe of that run ([`Checked`]).
///
/// # Errors
///
/// A `tool-failure` [`Refusal`] naming what disagreed, or both verdicts where the roles disagree; `usage` where what it would hand
/// the checker cannot be handed: a description that is no regular file, reads differently now, or has no room in a path.
pub fn check_plan(
    launcher: &dyn Launch,
    expected: &Expected<'_>,
    dir: &Path,
    err: &mut dyn Write,
) -> Result<Checked, Refusal> {
    let record_file = place_record(dir)?;
    // The checker reads it again, so what reads only once would read empty to it, or never end.
    if !std::fs::metadata(expected.description).is_ok_and(|meta| meta.is_file()) {
        return Err(Refusal {
            status: Status::Usage,
            message: format!(
                "the checker reads the description again, and {} is no regular file, which could read differently or never end",
                expected.description.display()
            ),
            repair: "give the description as a file, or say `--unchecked` where the command has it".to_owned(),
        });
    }
    let description = absolute(expected.description, "the description")?;
    // A regular file can still read differently: `/dev/stdin` reads empty the second time, and so would the checker.
    if let Some(text) = expected.text {
        if !std::fs::read(&description).is_ok_and(|again| again == text.as_bytes()) {
            return Err(Refusal {
                status: Status::Usage,
                message: format!(
                    "the checker reads the description again, and {} now reads differently from what the front read",
                    expected.description.display()
                ),
                repair: "give the description as a file nothing is changing, or say `--unchecked` where the command has it".to_owned(),
            });
        }
    }
    let plan = absolute(expected.plan, "the plan")?;
    let args: Vec<String> = vec![
        "--profile".to_owned(),
        expected.profile.to_owned(),
        "--result".to_owned(),
        record_file.to_string_lossy().into_owned(),
        "--".to_owned(),
        description.to_string_lossy().into_owned(),
        plan.to_string_lossy().into_owned(),
    ];
    let launched = launcher
        .launch(PLAN_CHECKER, &args, &mut std::io::sink(), err)
        .map_err(|unlaunched| failure(unlaunched.message, unlaunched.repair))?;
    let code = match launched.ended {
        Ended::Code(code) => code,
        Ended::Abnormally(how) => {
            return Err(fault(format!(
                "`{PLAN_CHECKER}` ended without an exit code: {how}"
            )))
        }
    };
    let (record, text) = read_record(&record_file, code)?;
    let mut disagree = Vec::new();
    if record.status().code() != code {
        disagree.push(format!(
            "it exited with {code}, and its record's status is `{}`, {}",
            record.status().slug(),
            record.status().code()
        ));
    }
    let producer = record.producer();
    if producer.name != PLAN_CHECKER || producer.sha256 != launched.sha256 {
        disagree.push(format!(
            "its record names `{}` {} as its producer, and `{PLAN_CHECKER}` {} ran",
            producer.name, producer.sha256, launched.sha256
        ));
    }
    let subject = record.subject();
    if subject.program != expected.program {
        disagree.push(format!(
            "its record judged the program {}, and the description became {}",
            subject.program, expected.program
        ));
    }
    if subject.judged != Judged::Plan(expected.plan_sha256) {
        disagree.push(format!(
            "its record judged {}, and the plan is {}",
            judged(&subject.judged),
            expected.plan_sha256
        ));
    }
    if record.profile() != expected.profile {
        disagree.push(format!(
            "its record is under the profile `{}`, and the plan under `{}`",
            record.profile(),
            expected.profile
        ));
    }
    if !disagree.is_empty() {
        return Err(fault(format!(
            "`{PLAN_CHECKER}`'s result does not hold together: {}",
            disagree.join("; ")
        )));
    }
    match record.status() {
        Status::Ok | Status::ToolFailure => Ok(Checked {
            record,
            text,
            front: launched.front,
        }),
        status @ (Status::StaleInput
        | Status::InvalidDescription
        | Status::MissingFact
        | Status::UnsupportedProfile
        | Status::InfeasibleConfiguration) => Err(failure(
            format!(
                "the roles disagree: the generator accepted the description and wrote this plan, and the configuration \
                 checker finds it `{}`",
                status.slug()
            ),
            "report it, with the checker's findings above: one of the two programs misreads this description"
                .to_owned(),
        )),
        status => Err(fault(format!(
            "`{PLAN_CHECKER}` answered `{}`, a status it never gives a plan",
            status.slug()
        ))),
    }
}

/// Whether `file` reads as a checker's record.
#[must_use]
pub fn is_record(file: &Path) -> bool {
    std::fs::symlink_metadata(file).is_ok_and(|meta| meta.is_file())
        && std::fs::read(file).is_ok_and(|bytes| Record::read(&bytes, RECORD_LIMIT).is_ok())
}

/// The record's place in `dir`, the directory made and any earlier record there removed, so none is taken for this
/// run's.
fn place_record(dir: &Path) -> Result<PathBuf, Refusal> {
    std::fs::create_dir_all(dir).map_err(|error| {
        failure(
            format!(
                "cannot make the directory {} for the checker's record: {error}",
                dir.display()
            ),
            "check that the output directory can be written".to_owned(),
        )
    })?;
    let dir = absolute(dir, "the record's directory")?;
    let record = dir.join(RECORD);
    match std::fs::remove_file(&record) {
        Ok(()) => {}
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(failure(
                format!(
                    "cannot remove the earlier record {}: {error}",
                    record.display()
                ),
                "remove it, or choose another output directory".to_owned(),
            ))
        }
    }
    Ok(record)
}

/// `file` as an absolute path, which the checker reads as the front does whatever its own directory: its directory
/// canonical, and its own name kept. A description that is a link is read through that link, so its imports are found beside
/// it, where the front found them, and not beside what it points at.
fn absolute(file: &Path, what: &str) -> Result<PathBuf, Refusal> {
    let nearer = "run it from a directory nearer the root, or move what it names nearer it";
    let unnamed = |error: std::io::Error| {
        let message = format!(
            "cannot name {what}, {}, absolutely: {error}",
            file.display()
        );
        match error.kind() {
            std::io::ErrorKind::NotFound => {
                failure(message, "check that it still exists".to_owned())
            }
            std::io::ErrorKind::PermissionDenied => Refusal {
                status: Status::Usage,
                message,
                repair: "this user must be able to reach it, and every directory above it"
                    .to_owned(),
            },
            _ => Refusal {
                status: Status::Usage,
                message,
                repair: nearer.to_owned(),
            },
        }
    };
    std::fs::metadata(file).map_err(unnamed)?;
    let Some(name) = file.file_name() else {
        return std::fs::canonicalize(file).map_err(unnamed);
    };
    let parent = file
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let named = std::fs::canonicalize(parent)
        .map(|parent| parent.join(name))
        .map_err(unnamed)?;
    // It reads from here by a shorter name, and the checker is handed this one.
    match std::fs::symlink_metadata(&named) {
        Err(error) if error.kind() == std::io::ErrorKind::InvalidFilename => Err(Refusal {
            status: Status::Usage,
            message: format!(
                "the checker is handed {what} by its whole path, and that is too long, {} seen from here: {error}",
                file.display()
            ),
            repair: nearer.to_owned(),
        }),
        _ => Ok(named),
    }
}

/// The record at `file`, which the checker that exited with `code` wrote, and its text.
fn read_record(file: &Path, code: i32) -> Result<(Record, String), Refusal> {
    let mut bytes = Vec::new();
    let read = std::fs::File::open(file).and_then(|opened| {
        opened
            .take(u64::try_from(RECORD_LIMIT).unwrap_or(u64::MAX) + 1)
            .read_to_end(&mut bytes)
    });
    match read {
        Ok(_) => {
            let record = Record::read(&bytes, RECORD_LIMIT).map_err(|refusal| {
                fault(format!(
                    "`{PLAN_CHECKER}`'s record {}: {refusal}",
                    file.display()
                ))
            })?;
            // A record that reads is JSON, so this holds; a text that does not is the checker's fault all the same.
            let text = String::from_utf8(bytes).map_err(|error| {
                fault(format!(
                    "`{PLAN_CHECKER}`'s record {}: {error}",
                    file.display()
                ))
            })?;
            Ok((record, text))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(failure(
            format!("`{PLAN_CHECKER}` exited with {code} and wrote no record, so nothing was judged"),
            format!(
                "look at {}, where the record goes: if it can be written and nothing moved it, report it, with what the checker \
                 wrote above",
                file.parent().unwrap_or(file).display()
            ),
        )),
        Err(error) => Err(fault(format!(
            "cannot read `{PLAN_CHECKER}`'s record {}: {error}",
            file.display()
        ))),
    }
}

fn judged(judged: &Judged) -> String {
    match judged {
        Judged::Plan(digest) => format!("the plan {digest}"),
        Judged::Build(digest) => format!("a build, {digest}"),
    }
}

fn failure(message: String, repair: String) -> Refusal {
    Refusal {
        status: Status::ToolFailure,
        message,
        repair,
    }
}

/// A disagreement between the checker's two channels, or between them and the front: no input of the user's is at
/// fault.
fn fault(message: String) -> Refusal {
    failure(
        message,
        "report it, with what the checker wrote above: the checker and the front disagree about one run".to_owned(),
    )
}
