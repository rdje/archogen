//! The front's contract for a result it consumes (leaf `M3.5.1.4.3`), held with a fake launcher that writes a record
//! where the front told it to and ends as the case says: the arguments the front hands over, an earlier record
//! removed, and every way the record, the process and the front's own reckoning can disagree refused as
//! `tool-failure`, naming what disagreed.

use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_cli::consume::{check_plan, Expected, PLAN_CHECKER, RECORD};
use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::Status;
use archogen_digest::Digest;
use archogen_outcome::record::{Finding, Judged, Label, Producer, Record, Severity, Subject};

/// The checker's sha256 as the launcher verified it.
fn verified() -> Digest {
    Digest::of(b"the checker that ran")
}

/// A fresh, canonical scratch directory, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "front-contract-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::canonicalize(&dir).unwrap()
}

/// A record with `status`, about `program` and `plan` under `profile`, produced by `producer`.
fn record(
    status: Status,
    program: Digest,
    plan: Digest,
    profile: &str,
    producer: (&str, Digest),
) -> Record {
    let findings = if status == Status::Ok {
        Vec::new()
    } else {
        vec![Finding {
            code: "plan-joint".to_owned(),
            severity: Severity::Error,
            message: "a finding".to_owned(),
            primary: Label {
                source: "plan.eadl".to_owned(),
                line: 1,
                column: 1,
                message: String::new(),
            },
            secondary: Vec::new(),
            repair: "resolve again".to_owned(),
        }]
    };
    Record::new(
        Producer {
            name: producer.0.to_owned(),
            sha256: producer.1,
        },
        Subject {
            program,
            judged: Judged::Plan(plan),
        },
        profile.to_owned(),
        status,
        findings,
    )
    .unwrap()
}

/// What the fake checker does: the record text it writes, if any, and how it ends.
struct Fake {
    writes: Option<String>,
    ends: Result<Ended, Unlaunched>,
    asked: RefCell<Vec<String>>,
}

impl Launch for Fake {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        _out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        assert_eq!(program, PLAN_CHECKER);
        *self.asked.borrow_mut() = args.to_vec();
        let at = args.iter().position(|a| a == "--result").unwrap() + 1;
        if let Some(text) = &self.writes {
            std::fs::write(&args[at], text).unwrap();
        }
        let _ = writeln!(err, "the checker's findings");
        self.ends.clone().map(|ended| Launched {
            ended,
            sha256: verified(),
            front: Digest::of(b"the front that ran it"),
        })
    }
}

/// One case: the inputs in a fresh directory, the expectation the front computed, and the result of the contract.
struct Case {
    dir: PathBuf,
    program: Digest,
    plan: Digest,
}

impl Case {
    fn new() -> Self {
        let dir = scratch();
        std::fs::write(dir.join("system.eadl"), "(defsystem s)\n").unwrap();
        std::fs::write(dir.join("plan.eadl"), "a plan\n").unwrap();
        Self {
            dir,
            program: Digest::of(b"the program"),
            plan: Digest::of(b"a plan\n"),
        }
    }

    fn good(&self, status: Status) -> String {
        record(
            status,
            self.program,
            self.plan,
            "rt-static-up-v1",
            (PLAN_CHECKER, verified()),
        )
        .write()
    }

    fn run(&self, fake: &Fake) -> (Result<Record, archogen_cli::Refusal>, String) {
        let expected = Expected {
            description: &self.dir.join("system.eadl"),
            sources: None,
            plan: &self.dir.join("plan.eadl"),
            program: self.program,
            plan_sha256: self.plan,
            profile: "rt-static-up-v1",
        };
        let mut err = Vec::new();
        let result =
            check_plan(fake, &expected, &self.dir.join("out/check"), &mut err).map(|checked| {
                // What a build reads back of the run: the record's bytes as written, and the front that ran it.
                let written =
                    std::fs::read_to_string(self.dir.join("out/check").join(RECORD)).unwrap();
                assert_eq!(checked.text, written);
                assert_eq!(checked.front, Digest::of(b"the front that ran it"));
                checked.record
            });
        (result, String::from_utf8(err).unwrap())
    }
}

fn fake(writes: Option<String>, ends: Result<Ended, Unlaunched>) -> Fake {
    Fake {
        writes,
        ends,
        asked: RefCell::new(Vec::new()),
    }
}

/// The refusal's message, which must be `tool-failure`.
fn refused(result: Result<Record, archogen_cli::Refusal>) -> String {
    let refusal = result.expect_err("a refusal");
    assert_eq!(refusal.status, Status::ToolFailure, "{refusal:?}");
    refusal.message
}

#[test]
fn an_agreeing_record_is_believed_and_the_front_hands_over_absolute_paths() {
    let case = Case::new();
    let checker = fake(Some(case.good(Status::Ok)), Ok(Ended::Code(0)));
    let (result, err) = case.run(&checker);
    assert!(result.expect("believed").accepts());
    assert_eq!(
        err, "the checker's findings\n",
        "its standard error is relayed"
    );
    let out = std::fs::canonicalize(case.dir.join("out/check")).unwrap();
    assert_eq!(
        checker.asked.borrow().as_slice(),
        [
            "--profile".to_owned(),
            "rt-static-up-v1".to_owned(),
            "--result".to_owned(),
            out.join(RECORD).to_string_lossy().into_owned(),
            "--".to_owned(),
            case.dir.join("system.eadl").to_string_lossy().into_owned(),
            case.dir.join("plan.eadl").to_string_lossy().into_owned(),
        ]
    );
    // A corrupted plan is the checker's verdict too: believed, its status `tool-failure`.
    let (result, _) = case.run(&fake(
        Some(case.good(Status::ToolFailure)),
        Ok(Ended::Code(70)),
    ));
    assert_eq!(result.expect("believed").status(), Status::ToolFailure);
}

#[test]
fn an_earlier_record_is_removed_so_a_checker_that_writes_none_is_refused() {
    let case = Case::new();
    let (result, _) = case.run(&fake(Some(case.good(Status::Ok)), Ok(Ended::Code(0))));
    assert!(result.is_ok());
    // The record of the run before stays in the directory unless the front removes it.
    let message = refused(case.run(&fake(None, Ok(Ended::Code(0)))).0);
    assert!(
        message.contains("exited with 0 and wrote no record"),
        "{message}"
    );
}

#[test]
fn every_disagreement_is_a_tool_failure_naming_it() {
    let case = Case::new();
    let other = Digest::of(b"something else");
    for (name, writes, ends, said) in [
        (
            "the code against the status",
            case.good(Status::Ok),
            Ended::Code(70),
            "it exited with 70, and its record's status is `ok`, 0".to_owned(),
        ),
        (
            "another producer",
            record(
                Status::Ok,
                case.program,
                case.plan,
                "rt-static-up-v1",
                (PLAN_CHECKER, other),
            )
            .write(),
            Ended::Code(0),
            format!("as its producer, and `{PLAN_CHECKER}` {} ran", verified()),
        ),
        (
            "another program",
            record(
                Status::Ok,
                other,
                case.plan,
                "rt-static-up-v1",
                (PLAN_CHECKER, verified()),
            )
            .write(),
            Ended::Code(0),
            format!(
                "its record judged the program {other}, and the description became {}",
                case.program
            ),
        ),
        (
            "another plan",
            record(
                Status::Ok,
                case.program,
                other,
                "rt-static-up-v1",
                (PLAN_CHECKER, verified()),
            )
            .write(),
            Ended::Code(0),
            format!(
                "its record judged the plan {other}, and the plan is {}",
                case.plan
            ),
        ),
        (
            "another profile",
            record(
                Status::Ok,
                case.program,
                case.plan,
                "rt-other-v9",
                (PLAN_CHECKER, verified()),
            )
            .write(),
            Ended::Code(0),
            "under the profile `rt-other-v9`, and the plan under `rt-static-up-v1`".to_owned(),
        ),
        (
            "a record that does not read",
            "{\"format\":\"archogen-result/1\"}\n".to_owned(),
            Ended::Code(0),
            "a result record that cannot be read".to_owned(),
        ),
        (
            "a signal",
            case.good(Status::Ok),
            Ended::Abnormally("signal: 9 (SIGKILL)".to_owned()),
            "ended without an exit code: signal: 9 (SIGKILL)".to_owned(),
        ),
    ] {
        let message = refused(case.run(&fake(Some(writes), Ok(ends))).0);
        assert!(message.contains(&said), "{name}: {message}");
    }
}

#[test]
fn the_checker_finding_the_description_wanting_is_the_roles_disagreeing() {
    let case = Case::new();
    for status in [
        Status::InvalidDescription,
        Status::MissingFact,
        Status::UnsupportedProfile,
        Status::InfeasibleConfiguration,
        Status::StaleInput,
    ] {
        let message = refused(
            case.run(&fake(
                Some(case.good(status)),
                Ok(Ended::Code(status.code())),
            ))
            .0,
        );
        assert!(
            message.contains(&format!(
                "the generator accepted the description and wrote this plan, and the configuration checker finds it `{}`",
                status.slug()
            )),
            "{message}"
        );
    }
    // A status the checker never gives a plan.
    let message = refused(
        case.run(&fake(
            Some(case.good(Status::Unimplemented)),
            Ok(Ended::Code(20)),
        ))
        .0,
    );
    assert!(
        message.contains("a status it never gives a plan"),
        "{message}"
    );
}

#[test]
fn a_checker_not_run_is_refused_with_the_launcher_s_reason() {
    let case = Case::new();
    let unlaunched = Unlaunched {
        message: "the install has no `archogen-plan-check`".to_owned(),
        repair: "install the toolchain again".to_owned(),
    };
    let refusal = case
        .run(&fake(None, Err(unlaunched)))
        .0
        .expect_err("refused");
    assert_eq!(refusal.status, Status::ToolFailure);
    assert_eq!(refusal.message, "the install has no `archogen-plan-check`");
    assert_eq!(refusal.repair, "install the toolchain again");
}
