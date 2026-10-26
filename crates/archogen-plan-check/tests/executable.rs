//! The configuration checker's executable (leaf `M3.5.6.1`), run as the front runs it: a description and a plan in, a
//! result record out where it is told, and an exit code equal to the record's status — or, for a usage error, no record
//! at all.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_digest::Digest;
use archogen_outcome::record::{Judged, Record};
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, program_text, Assignment, Requester};
use archogen_plan_check::check::{plan_file, PLAN_STALE, PLAN_UNSATISFIED};
use eadl_front::module::DirectoryModules;
use eadl_front::SourceMap;
use eadl_model::profile::RT_STATIC_UP_V1;

const EXE: &str = env!("CARGO_BIN_EXE_archogen-plan-check");

/// A plain description: a console service on a platform composing a serial port.
const PLAIN: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";

/// A fresh scratch directory under the target directory, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "plan-check-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// The program `description` becomes, as the executable reads it.
fn program_of(description: &Path) -> eadl_front::Program {
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            description.display().to_string(),
            std::fs::read_to_string(description).unwrap(),
        )
        .unwrap();
    let modules = DirectoryModules::new(description.parent().unwrap());
    plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.eadl",
        "",
    )
    .0
}

/// The plan binding the console to `provider`, for the program `description` becomes.
fn plan(description: &Path, requester: &str, provider: &str) -> String {
    let mut bindings = std::collections::BTreeMap::new();
    bindings.insert(
        (
            Requester::Named(requester.to_owned()),
            "observable-output".to_owned(),
        ),
        provider.to_owned(),
    );
    Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "s".to_owned(),
        program: program_digest(&program_of(description)),
        placement: None,
        bindings,
    }
    .to_canonical()
}

/// Run the executable with `args`.
fn run(args: &[&str]) -> Output {
    Command::new(EXE)
        .args(args)
        .env_clear()
        .output()
        .expect("the executable runs")
}

/// Check `plan_text` against the description at `description`: the exit status, the record and stderr.
fn check(description: &Path, plan_text: &str) -> (i32, Record, String) {
    let dir = description.parent().unwrap();
    let plan_path = dir.join("plan.eadl");
    std::fs::write(&plan_path, plan_text).unwrap();
    let result = dir.join("result.json");
    let output = run(&[
        "--result",
        result.to_str().unwrap(),
        "--",
        description.to_str().unwrap(),
        plan_path.to_str().unwrap(),
    ]);
    let record = Record::read(
        &std::fs::read(&result).expect("a record is written"),
        1 << 20,
    )
    .expect("the record reads");
    let code = output.status.code().expect("an exit code");
    // The front's second channel: the exit code is the record's status.
    assert_eq!(code, record.status().code(), "{record:?}");
    (
        code,
        record,
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn a_plan_of_a_plain_description_is_accepted_and_the_record_names_what_ran_and_what_it_judged() {
    let dir = scratch();
    let description = dir.join("system.eadl");
    std::fs::write(&description, PLAIN).unwrap();
    let plan_text = plan(&description, "console.write", "console.uart");
    let (code, record, stderr) = check(&description, &plan_text);
    assert_eq!(code, 0, "{stderr}");
    assert!(record.accepts() && record.findings().is_empty());
    assert_eq!(record.producer().name, "archogen-plan-check");
    assert_eq!(
        record.producer().sha256,
        Digest::of(&std::fs::read(EXE).unwrap())
    );
    let program = program_of(&description);
    assert_eq!(
        record.subject().program,
        Digest::of(program_text(&program).as_bytes())
    );
    assert_eq!(
        record.subject().judged,
        Judged::Plan(Digest::of(plan_text.as_bytes()))
    );
    assert_eq!(record.profile(), RT_STATIC_UP_V1.id);
    // A plain description is one root instance, no module, version 0.0.
    assert!(
        program_text(&program).starts_with("instance . - 0.0\n(defblock console.uart"),
        "{}",
        program_text(&program)
    );
}

#[test]
fn a_module_is_elaborated_from_its_directory() {
    let dir = scratch();
    std::fs::write(
        dir.join("hw.board.eadl"),
        "(defmodule hw.board (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart))))",
    )
    .unwrap();
    let description = dir.join("app.eadl");
    std::fs::write(
        &description,
        "(defmodule app (version 1 0) (import hw.board (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (requires (uses console.write))
    (platform (uses hw.soc))))",
    )
    .unwrap();
    let plan_text = plan(&description, "console.write", "hw.console.uart");
    let (code, record, stderr) = check(&description, &plan_text);
    assert_eq!((code, record.status()), (0, Status::Ok), "{stderr}");
    assert!(program_text(&program_of(&description)).contains("instance hw hw.board 1.0\n"));
}

#[test]
fn a_stale_a_corrupted_and_a_refused_plan_each_exit_with_their_status() {
    let dir = scratch();
    let description = dir.join("system.eadl");
    std::fs::write(&description, PLAIN).unwrap();
    let good = plan(&description, "console.write", "console.uart");
    // Stale: another program's digest — refused before any judgement.
    let stale = good.replace(&program_digest(&program_of(&description)), &"0".repeat(64));
    let (code, record, _) = check(&description, &stale);
    assert_eq!(
        (code, record.status(), record.findings()[0].code.as_str()),
        (3, Status::StaleInput, PLAN_STALE)
    );
    // Corrupted: the console bound to the platform, which does not offer output.
    let moved = plan(&description, "console.write", "soc");
    let (code, record, stderr) = check(&description, &moved);
    assert_eq!(
        (code, record.status(), record.findings()[0].code.as_str()),
        (70, Status::ToolFailure, PLAN_UNSATISFIED)
    );
    assert!(
        stderr.contains("error[plan-unsatisfied]") && stderr.contains("= repair:"),
        "{stderr}"
    );
    // The description refused: its own status, once the plan is the changed description's — before, the plan is
    // stale, and staleness is judged first.
    std::fs::write(
        &description,
        format!("{PLAIN}(defservice broken (requries (needs x)))\n"),
    )
    .unwrap();
    let (code, record, _) = check(&description, &good);
    assert_eq!((code, record.status()), (3, Status::StaleInput));
    let rewritten = plan(&description, "console.write", "console.uart");
    let (code, record, _) = check(&description, &rewritten);
    assert_eq!((code, record.status()), (10, Status::InvalidDescription));
}

#[test]
fn an_earlier_record_at_the_path_is_replaced() {
    let dir = scratch();
    let description = dir.join("system.eadl");
    std::fs::write(&description, PLAIN).unwrap();
    std::fs::write(dir.join("result.json"), "an earlier record").unwrap();
    let (code, record, _) = check(
        &description,
        &plan(&description, "console.write", "console.uart"),
    );
    assert_eq!((code, record.status()), (0, Status::Ok));
    assert!(!dir.join("result.json.part").exists());
}

#[test]
fn a_usage_error_writes_no_record() {
    let dir = scratch();
    let description = dir.join("system.eadl");
    std::fs::write(&description, PLAIN).unwrap();
    std::fs::write(dir.join("plan.eadl"), "(format \"archogen-assignment/1\")").unwrap();
    let result = dir.join("result.json");
    let (d, p, r) = (
        description.to_str().unwrap(),
        dir.join("plan.eadl"),
        result.to_str().unwrap(),
    );
    let p = p.to_str().unwrap();
    let missing = dir.join("absent.eadl");
    for (case, args) in [
        ("no result path", vec!["--", d, p]),
        ("no paths", vec!["--result", r]),
        ("one path", vec!["--result", r, "--", d]),
        (
            "an unknown option",
            vec!["--result", r, "--fast", "--", d, p],
        ),
        (
            "a result path twice",
            vec!["--result", r, "--result", r, "--", d, p],
        ),
        (
            "another profile",
            vec!["--profile", "rt-other-v9", "--result", r, "--", d, p],
        ),
        (
            "an absent description",
            vec!["--result", r, "--", missing.to_str().unwrap(), p],
        ),
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2), "{case}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("archogen-plan-check: usage:"),
            "{case}"
        );
        assert!(!result.exists(), "{case}: a record was written");
    }
}

#[test]
fn help_and_version_answer_and_exit_zero() {
    let help = run(&["--help"]);
    assert_eq!(help.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&help.stdout).contains("--result PATH"));
    let version = run(&["--version"]);
    assert_eq!(version.status.code(), Some(0));
    assert_eq!(
        String::from_utf8_lossy(&version.stdout),
        "archogen-plan-check: reads archogen-assignment/1, writes archogen-result/1\n"
    );
}
