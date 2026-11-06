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

use std::cell::Cell;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use archogen_api::SourceMap;

use archogen_digest::Digest;
use archogen_outcome::record::{Judged, Record};

use crate::cli::Refusal;
use crate::launch::{Ended, Launch};
use crate::place;
use crate::status::Status;

/// The configuration checker's executable.
pub const PLAN_CHECKER: &str = "archogen-plan-check";

/// The record's name in the front's own directory.
pub const RECORD: &str = archogen_emit::RESULT_FILE;

/// The longest name the checker writes beneath the directory it is given: its record, written first beside itself and renamed
/// into place (`crates/archogen-plan-check/src/main.rs`).
pub const RECORD_PART: &str = "plan-check.json.part";

/// Where an earlier record waits while a new one takes its name, so it can go back.
pub const RECORD_EARLIER: &str = "plan-check.json.earlier";

/// The most of a record the front reads.
pub const RECORD_LIMIT: usize = 1 << 30;

/// What begins a record, so that what is not one is refused without being read whole.
const A_RECORD_BEGINS: &[u8] = b"{\"format\":\"archogen-result/";

/// What the front computed itself, which the checker's record must name.
#[derive(Debug, Clone, Copy)]
pub struct Expected<'a> {
    /// The description, as the front read it.
    pub description: &'a Path,
    /// What the front read, which the checker must be able to read again: the description and every module.
    pub sources: Option<&'a SourceMap>,
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
/// directory of the front's own under the command's output; its standard error relayed to `err`. `ran` is set once the checker is handed that directory, nothing then standing at its
/// record's name: only then is what stands there the checker's, and a run's to remove.
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
    ran: &Cell<bool>,
    err: &mut dyn Write,
) -> Result<Checked, Refusal> {
    let record_file = place_record(dir)?;
    // The checker reads it again, so what reads only once would read empty to it, or never end.
    if let Err(error) = std::fs::metadata(expected.description) {
        return Err(Refusal {
            status: Status::Usage,
            message: format!(
                "the checker reads the description again, and {} cannot be read now: {error}",
                expected.description.display()
            ),
            repair: put_it_back(&error).to_owned(),
        });
    }
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
    if let Some(sources) = expected.sources {
        reads_again(sources)?;
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
    ran.set(true);
    let launched = launcher
        .launch(PLAN_CHECKER, &args, &mut std::io::sink(), err)
        .map_err(|unlaunched| failure(unlaunched.message, unlaunched.repair))?;
    let code = match launched.ended {
        Ended::Code(code) => code,
        // Nothing disagrees: it did not finish.
        Ended::Abnormally(how) => {
            return Err(failure(
                format!("`{PLAN_CHECKER}` ended without an exit code: {how}"),
                "nothing was judged, so run this again: if nothing else stopped it and it ends the same way, report it, with what it wrote above".to_owned(),
            ))
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

/// That the checker, reading what the front read again, will read the same: each source a regular file with the same bytes.
/// A pipe would read empty or never end, and `/dev/stdin` reads empty the second time. The engine's own kind modules come from no
/// file, so a source that is one of those, name and text, is passed over.
fn reads_again(sources: &SourceMap) -> Result<(), Refusal> {
    let kinds = archogen_api::kind_modules();
    let refused = |message: String, repair: &str| Refusal {
        status: Status::Usage,
        message,
        repair: repair.to_owned(),
    };
    let as_a_file =
        "give it as a file nothing is changing, or say `--unchecked` where the command has it";
    for source in sources.iter() {
        if kinds
            .iter()
            .any(|(name, text)| *name == source.name && *text == source.text)
        {
            continue;
        }
        let file = Path::new(&source.name);
        match std::fs::metadata(file) {
            Ok(meta) if meta.is_file() => {}
            Ok(_) => {
                return Err(refused(
                    format!(
                        "the checker reads {} again, and it is no regular file, which could read differently or never end",
                        source.name
                    ),
                    as_a_file,
                ))
            }
            Err(error) => {
                return Err(refused(
                    format!(
                        "the checker reads {} again, and it cannot be read now: {error}",
                        source.name
                    ),
                    put_it_back(&error),
                ))
            }
        }
        // The checker reads a module beside the description, by a whole path that may have no room.
        absolute(file, &source.name)?;
        match std::fs::read(file) {
            Ok(again) if again == source.text.as_bytes() => {}
            Ok(_) => {
                return Err(refused(
                    format!(
                        "the checker reads {} again, and it now reads differently from what the front read",
                        source.name
                    ),
                    as_a_file,
                ))
            }
            Err(error) => {
                return Err(refused(
                    format!(
                        "the checker reads {} again, and it cannot be read now: {error}",
                        source.name
                    ),
                    put_it_back(&error),
                ))
            }
        }
    }
    Ok(())
}

/// The repair for a file the front read that the checker cannot read again, by what stopped it.
fn put_it_back(error: &std::io::Error) -> &'static str {
    if error.kind() == std::io::ErrorKind::NotFound {
        "it stood when the front read it: put it back, and run this again"
    } else {
        "open it, and the directories above it"
    }
}

/// The checker's record out of `dir`, if it is a regular file and a record, and what it was writing, if that reads as the start of one: the one file in a stage this run did not write itself, and
/// its own program did. So it is asked only of a run whose checker was handed the stage: before that, nothing at that name is this run's, whatever it reads as.
pub fn remove_record(dir: &Path) {
    let record = dir.join(RECORD);
    if std::fs::symlink_metadata(&record).is_ok_and(|meta| meta.is_file())
        && reads_as_record(&record).unwrap_or(false)
    {
        let _ = std::fs::remove_file(&record);
    }
    let part = dir.join(RECORD_PART);
    if place::open_regular(&part)
        .and_then(|opened| {
            let mut begins = Vec::new();
            let length = u64::try_from(A_RECORD_BEGINS.len()).unwrap_or(u64::MAX);
            opened.take(length).read_to_end(&mut begins)?;
            Ok(A_RECORD_BEGINS.starts_with(&begins))
        })
        .unwrap_or(false)
    {
        let _ = std::fs::remove_file(&part);
    }
}

/// Whether `file` reads as a checker's record, read within the bound a record has. What could not be read is an error and no
/// answer: refusing it is the caller's, since it cannot say.
///
/// # Errors
///
/// What stopped it reading the file.
pub fn reads_as_record(file: &Path) -> std::io::Result<bool> {
    let mut opened = std::fs::File::open(file)?;
    let mut bytes = vec![0; A_RECORD_BEGINS.len()];
    match opened.read_exact(&mut bytes) {
        Ok(()) if bytes == A_RECORD_BEGINS => {}
        Ok(()) => return Ok(false),
        Err(error) if error.kind() == std::io::ErrorKind::UnexpectedEof => return Ok(false),
        Err(error) => return Err(error),
    }
    opened
        .take(u64::try_from(RECORD_LIMIT).unwrap_or(u64::MAX))
        .read_to_end(&mut bytes)?;
    Ok(bytes.len() <= RECORD_LIMIT && Record::read(&bytes, RECORD_LIMIT).is_ok())
}

/// The record's place in `dir`, the directory made. Each command's stage is made by this run, so nothing earlier can be this run's: what stands at that name came from elsewhere, and is
/// never this run's to remove (`R13-2`).
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
    match std::fs::symlink_metadata(&record) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(record),
        Err(error) => Err(failure(
            format!(
                "cannot look at where the checker's record goes, {}: {error}",
                record.display()
            ),
            "something closed this run's directory meanwhile: open it, or name another place"
                .to_owned(),
        )),
        Ok(_) => Err(failure(
            format!(
                "{} stands where the checker's record goes, and this run did not write it",
                record.display()
            ),
            "something else is using this run's directory: look at it, and move it".to_owned(),
        )),
    }
}

/// `file` as an absolute path, which the checker reads as the front does whatever its own directory: its directory
/// canonical, and its own name kept. A description that is a link is read through that link, so its imports are found beside
/// it, where the front found them, and not beside what it points at.
fn absolute(file: &Path, what: &str) -> Result<PathBuf, Refusal> {
    let nearer = "run it from a directory nearer the root, or move what it names nearer it";
    let unnamed = |error: std::io::Error| {
        // Where what it is called is its path, the path is not said twice.
        let message = if file.as_os_str() == what {
            format!("cannot name {what} absolutely: {error}")
        } else {
            format!(
                "cannot name {what}, {}, absolutely: {error}",
                file.display()
            )
        };
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
    let read = place::open_regular(file).and_then(|opened| {
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
                "the checker reads what it was given and writes beside it, under {}, which this run removes after it, unless something else came to stand there: check that \
                 the umask lets this user read and write what it makes, and that nothing moved it; if neither, report it, with what the checker \
                 wrote above",
                file.parent().unwrap_or(file).display()
            ),
        )),
        // Closed while it ran: the checker and the front do not disagree, and only what closed it can say why.
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => Err(failure(
            format!(
                "cannot read `{PLAN_CHECKER}`'s record {}, closed while the checker ran: {error}",
                file.display()
            ),
            "open it and the directories above it: nothing was judged".to_owned(),
        )),
        // Something else stands where it goes, or on the way there: nothing disagrees.
        Err(error) => Err(failure(
            format!(
                "cannot read `{PLAN_CHECKER}`'s record {}: {error}",
                file.display()
            ),
            "something else changed this run's directory while the checker ran, so nothing was judged: look at it, and at what else writes there".to_owned(),
        )),
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

#[cfg(test)]
mod tests {
    use super::{check_plan, place_record, remove_record, Expected, RECORD};
    use crate::launch::{Launch, Launched, Unlaunched};
    use archogen_digest::Digest;
    use archogen_outcome::record::{Judged, Producer, Record, Subject};
    use std::cell::Cell;
    use std::io::Write;
    use std::path::Path;

    struct NeverRun;

    impl Launch for NeverRun {
        fn launch(
            &self,
            _program: &str,
            _args: &[String],
            _out: &mut dyn Write,
            _err: &mut dyn Write,
        ) -> Result<Launched, Unlaunched> {
            panic!("a record stands there, so no checker is handed the stage");
        }
    }

    /// `R15-3`: a record that came before the checker was handed the stage reads as one, and is not this run's.
    #[test]
    fn a_record_no_checker_of_this_run_wrote_is_never_this_run_s_to_remove() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/tmp/consume-came-first");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("made");
        let record = Record::new(
            Producer {
                name: super::PLAN_CHECKER.to_owned(),
                sha256: Digest::of(b"another"),
            },
            Subject {
                program: Digest::of(b"another"),
                judged: Judged::Plan(Digest::of(b"another")),
            },
            "rt-static-up-v1".to_owned(),
            crate::status::Status::Ok,
            Vec::new(),
        )
        .expect("a record")
        .write();
        let standing = dir.join(RECORD);
        std::fs::write(&standing, &record).expect("written");
        assert!(super::reads_as_record(&standing).expect("read"));
        let ran = Cell::new(false);
        let refusal = check_plan(
            &NeverRun,
            &Expected {
                description: &dir.join("description"),
                sources: None,
                plan: &dir.join("plan"),
                program: Digest::of(b"another"),
                plan_sha256: Digest::of(b"another"),
                profile: "rt-static-up-v1",
            },
            &dir,
            &ran,
            &mut Vec::new(),
        )
        .expect_err("it stands there");
        assert!(refusal.message.contains("this run did not write it"));
        assert!(!ran.get());
        // As each command's way out asks.
        if ran.get() {
            remove_record(&dir);
        }
        assert_eq!(std::fs::read_to_string(&standing).expect("kept"), record);
        std::fs::remove_dir_all(&dir).expect("removed");
    }

    /// `R13-2`: what stands at the record's name came from elsewhere, and goes on standing.
    #[test]
    fn what_stands_where_the_record_goes_is_said_and_never_removed() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/tmp/consume-place-record");
        let _ = std::fs::remove_dir_all(&dir);
        let record = place_record(&dir).expect("nothing stands there");
        assert!(!record.exists());
        std::fs::write(&record, "mine").expect("written");
        let refusal = place_record(&dir).expect_err("something stands there");
        assert!(
            refusal.message.contains("this run did not write it"),
            "{}",
            refusal.message
        );
        assert_eq!(std::fs::read_to_string(&record).expect("kept"), "mine");
        assert_eq!(
            record.file_name().and_then(|name| name.to_str()),
            Some(RECORD)
        );
        std::fs::remove_dir_all(&dir).expect("removed");
    }
}
