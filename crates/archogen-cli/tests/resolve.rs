//! `archogen resolve` (leaf `M3.4.4`): one system searched, the plan written whole, and the configuration checker run
//! over it through the launcher the caller hands over — here a fake that writes the record the case asks for, where
//! the front told it to, as `archogen-plan-check` would — so the front's half is held without a process: the plan,
//! the command's status the checker's verdict, the downgrade `--unchecked` says it is, and every refusal before a plan
//! exists writing none.

use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_cli::consume::PLAN_CHECKER;
use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::{run_with_input, Status};
use archogen_digest::Digest;
use archogen_outcome::record::{Finding, Judged, Label, Producer, Record, Severity, Subject};

/// The checker's sha256 as the launcher verified it.
fn verified() -> Digest {
    Digest::of(b"the checker that ran")
}

/// A fresh scratch directory under the target directory, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "resolve-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("two levels below the root")
        .to_path_buf()
}

/// How the fake checker answers: the status its record and its exit code give.
#[derive(Clone, Copy)]
enum Answer {
    /// A record of this status, and the exit code it carries.
    Record(Status),
    /// The launcher could not start it.
    Unlaunched,
}

/// A launcher standing in for `archogen-plan-check`: it reads the plan the front names, writes a record about it where
/// the front told it to, and ends with the record's code.
struct Checker {
    answer: Answer,
    asked: RefCell<Vec<Vec<String>>>,
}

impl Checker {
    fn answering(answer: Answer) -> Self {
        Self {
            answer,
            asked: RefCell::new(Vec::new()),
        }
    }
}

impl Launch for Checker {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        _out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        assert_eq!(program, PLAN_CHECKER);
        self.asked.borrow_mut().push(args.to_vec());
        let status = match self.answer {
            Answer::Unlaunched => {
                return Err(Unlaunched {
                    message: "no install beside the front".to_owned(),
                    repair: "install the toolchain".to_owned(),
                })
            }
            Answer::Record(status) => status,
        };
        // `--profile P --result R -- DESCRIPTION PLAN`, as the front hands them over.
        let (profile, result, plan) = (&args[1], &args[3], &args[6]);
        let text = std::fs::read_to_string(plan).expect("the plan the front wrote");
        let program = text
            .lines()
            .find_map(|line| line.strip_prefix("(program \"")?.strip_suffix("\")"))
            .expect("the plan names its program");
        let findings = if status == Status::Ok {
            Vec::new()
        } else {
            vec![Finding {
                code: "plan-unsatisfied".to_owned(),
                severity: Severity::Error,
                message: "a binding the relation does not satisfy".to_owned(),
                primary: Label {
                    source: "plan.json".to_owned(),
                    line: 5,
                    column: 1,
                    message: "here".to_owned(),
                },
                secondary: Vec::new(),
                repair: "resolve again".to_owned(),
            }]
        };
        let record = Record::new(
            Producer {
                name: PLAN_CHECKER.to_owned(),
                sha256: verified(),
            },
            Subject {
                program: Digest::parse(&format!("sha256:{program}")).expect("a digest"),
                judged: Judged::Plan(Digest::of(text.as_bytes())),
            },
            profile.clone(),
            status,
            findings,
        )
        .expect("a record");
        std::fs::write(result, record.write()).expect("the record written");
        let _ = writeln!(err, "{PLAN_CHECKER}: {}", status.slug());
        Ok(Launched {
            ended: Ended::Code(status.code()),
            sha256: verified(),
        })
    }
}

/// Run `archogen resolve` with `args` through `launcher`: the status, standard output and standard error.
fn resolve(args: &[&str], launcher: &dyn Launch) -> (Status, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run_with_input(
        std::iter::once("resolve").chain(args.iter().copied()),
        &mut std::io::empty(),
        launcher,
        &mut out,
        &mut err,
    );
    (
        status,
        String::from_utf8(out).expect("UTF-8"),
        String::from_utf8(err).expect("UTF-8"),
    )
}

fn heartbeat() -> String {
    root()
        .join("examples/s0-heartbeat/system.eadl")
        .display()
        .to_string()
}

#[test]
fn a_resolved_plan_is_checked_and_the_checker_s_verdict_is_the_status() {
    let dir = scratch();
    let plan = dir.join("out/plan.json");
    let checker = Checker::answering(Answer::Record(Status::Ok));
    let (status, out, err) = resolve(&[&heartbeat(), "--out", plan.to_str().unwrap()], &checker);
    assert_eq!(status, Status::Ok, "{err}");
    let text = std::fs::read_to_string(&plan).expect("the plan is written, its directory made");
    assert!(text.starts_with(
        "(format \"archogen-assignment/1\")\n(profile rt-static-up-v1)\n(system heartbeat)\n"
    ));
    assert!(
        text.ends_with("(bind console.write observable-output console.uart)\n"),
        "{text}"
    );
    assert!(out.contains("resolved system `heartbeat`"), "{out}");
    assert!(
        out.contains("experimental:") && out.contains("completed by M3.4.6"),
        "{out}"
    );
    assert!(
        out.contains("checked by `archogen-plan-check`: accepted"),
        "{out}"
    );
    assert!(dir.join("out/plan.json.check/plan-check.json").is_file());
    let asked = checker.asked.borrow();
    assert_eq!(asked.len(), 1);
    assert_eq!(asked[0][0..2], ["--profile", "rt-static-up-v1"]);
    assert!(
        asked[0][3].ends_with("plan.json.check/plan-check.json"),
        "{:?}",
        asked[0]
    );
}

#[test]
fn a_plan_the_checker_refuses_is_a_tool_failure_naming_both() {
    let dir = scratch();
    let plan = dir.join("plan.json");
    let checker = Checker::answering(Answer::Record(Status::ToolFailure));
    let (status, _, err) = resolve(&[&heartbeat(), "--out", plan.to_str().unwrap()], &checker);
    assert_eq!(status, Status::ToolFailure, "{err}");
    assert!(
        err.contains("`archogen-plan-check` refuses the plan the search wrote"),
        "{err}"
    );
}

#[test]
fn a_checker_that_cannot_run_is_a_tool_failure_with_its_reason() {
    let dir = scratch();
    let plan = dir.join("plan.json");
    let (status, _, err) = resolve(
        &[&heartbeat(), "--out", plan.to_str().unwrap()],
        &Checker::answering(Answer::Unlaunched),
    );
    assert_eq!(status, Status::ToolFailure, "{err}");
    assert!(err.contains("no install beside the front"), "{err}");
}

#[test]
fn unchecked_writes_the_plan_runs_nothing_and_says_so() {
    let dir = scratch();
    let plan = dir.join("plan.json");
    let checker = Checker::answering(Answer::Unlaunched);
    let (status, out, err) = resolve(
        &[&heartbeat(), "--out", plan.to_str().unwrap(), "--unchecked"],
        &checker,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(plan.is_file());
    assert!(out.contains("resolved system `heartbeat`"), "{out}");
    assert!(
        err.contains("`--unchecked`") && err.contains("not checked"),
        "{err}"
    );
    assert!(checker.asked.borrow().is_empty(), "nothing was run");
}

#[test]
fn every_refusal_before_a_plan_writes_none_and_runs_nothing() {
    let dir = scratch();
    let write = |name: &str, text: &str| {
        let path = dir.join(name);
        std::fs::write(&path, text).unwrap();
        path.display().to_string()
    };
    let platform = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
";
    let system = |name: &str| {
        format!(
            "(defsystem {name}
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
"
        )
    };
    let two = write(
        "two.eadl",
        &format!("{platform}{}{}", system("a"), system("b")),
    );
    let none = write("none.eadl", platform);
    let invalid = write(
        "invalid.eadl",
        "(eadl-version eadl/1)\n(defservice broken (requries (needs x)))\n",
    );
    write(
        "hw.board.eadl",
        "(defmodule hw.board (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart))))",
    );
    let instances = write(
        "app.eadl",
        "(defmodule app (version 1 0) (import hw.board (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (requires (uses console.write))
    (platform (uses hw.soc))))",
    );
    let periodic = root()
        .join("examples/periodic-three/system.eadl")
        .display()
        .to_string();
    let plan = dir.join("plan.json");
    let out = plan.to_str().unwrap();
    for (case, args, status, says) in [
        (
            "--locked",
            vec![heartbeat(), "--locked".to_owned()],
            Status::Unimplemented,
            "`--locked` is not implemented",
        ),
        (
            "an invalid description",
            vec![invalid],
            Status::InvalidDescription,
            "nothing was resolved",
        ),
        ("no system", vec![none], Status::Usage, "declares no system"),
        (
            "two systems",
            vec![two.clone()],
            Status::Usage,
            "`--system` on the command line: `a`, `b`",
        ),
        (
            "a system it does not declare",
            vec![two, "--system".to_owned(), "c".to_owned()],
            Status::Usage,
            "declares no system `c`",
        ),
        (
            "several instances",
            vec![instances],
            Status::UnsupportedProfile,
            "task-tree leaf M3.4.10's",
        ),
        (
            "nothing meets a requirement",
            vec![periodic],
            Status::MissingFact,
            "system `periodic-three` of",
        ),
    ] {
        let checker = Checker::answering(Answer::Record(Status::Ok));
        let mut argv: Vec<&str> = args.iter().map(String::as_str).collect();
        argv.extend(["--out", out]);
        let (got, _, err) = resolve(&argv, &checker);
        assert_eq!(got, status, "{case}: {err}");
        assert!(err.contains(says), "{case}: {err}");
        assert!(!plan.exists(), "{case}: a plan was written");
        assert!(
            checker.asked.borrow().is_empty(),
            "{case}: the checker was run"
        );
    }
    // `--system` naming one of two resolves it.
    let two = dir.join("two.eadl").display().to_string();
    let (got, out_text, err) = resolve(
        &[&two, "--system", "b", "--out", out, "--unchecked"],
        &Checker::answering(Answer::Unlaunched),
    );
    assert_eq!(got, Status::Ok, "{err}");
    assert!(out_text.contains("resolved system `b`"), "{out_text}");
}
