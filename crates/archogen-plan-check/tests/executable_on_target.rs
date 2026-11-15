//! The configuration checker's executable given a target (leaf `M4.1.7.5.4.4`; `DC-H2`), run as the front runs it:
//! `--target FILE` judges a plan of a system on a target, `archogen-assignment/3`, with its refusals in the order
//! `docs/decisions/decision_runtime-plan-format-2.md` §1's "The checker" gives — a file that does not read, the target's
//! among them, then a record of other inputs, `stale-input`, before any judgement, then the judgement.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_outcome::record::Record;
use archogen_outcome::Status;
use archogen_plan_check::assignment::program_digest;
use archogen_plan_check::check::{plan_file, PLAN_STALE};
use archogen_plan_check::target::digest;
use eadl_front::module::DirectoryModules;
use eadl_front::SourceMap;
use eadl_model::profile::RT_STATIC_UP_V1;

const EXE: &str = env!("CARGO_BIN_EXE_archogen-plan-check");

/// A file of the repository, read from its root.
fn repository(file: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(file),
    )
    .unwrap_or_else(|e| panic!("{file} reads: {e}"))
}

/// A fresh scratch directory under the target directory, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "plan-check-target-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// The S0 heartbeat's bindings on the shipped target: the console, and the release requester's eight facts at the timer.
const BINDS: &str = "(bind console.write observable-output target.console)
(bind (release heartbeat) alarm-kind target.timer)
(bind (release heartbeat) alarm-wiring target.timer)
(bind (release heartbeat) counter-modulus target.timer)
(bind (release heartbeat) observation-coherent target.timer)
(bind (release heartbeat) relative-delay target.timer)
(bind (release heartbeat) supported-horizon target.timer)
(bind (release heartbeat) tick-rate target.timer)
(bind (release heartbeat) unambiguous-horizon target.timer)
";

/// A scratch directory holding the heartbeat's description, the shipped target and the heartbeat's `/3` plan on it.
struct Files {
    description: PathBuf,
    target: PathBuf,
    plan: PathBuf,
}

impl Files {
    fn new() -> Self {
        let dir = scratch();
        let description = dir.join("system.eadl");
        std::fs::write(
            &description,
            repository("examples/s0-heartbeat/system.eadl"),
        )
        .unwrap();
        let target = dir.join("riscv-virt-up.eadl");
        let shipped = repository("targets/riscv-virt-up.eadl");
        std::fs::write(&target, &shipped).unwrap();
        let plan = dir.join("plan.eadl");
        std::fs::write(
            &plan,
            record(&program_of(&description), &target_digest(&shipped)),
        )
        .unwrap();
        Self {
            description,
            target,
            plan,
        }
    }
}

/// The digest a plan on the target `text` writes: its forms' program text's, the language-version identifier aside.
fn target_digest(text: &str) -> String {
    let mut sources = SourceMap::new();
    let id = sources.add("target.eadl", text).unwrap();
    let (document, diagnostics) = eadl_front::read(&sources, id);
    assert!(!diagnostics.has_errors());
    digest(&document.declarations().cloned().collect::<Vec<_>>())
}

/// The digest of the program the description at `description` becomes, as the executable reads it.
fn program_of(description: &Path) -> String {
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            description.display().to_string(),
            std::fs::read_to_string(description).unwrap(),
        )
        .unwrap();
    let modules = DirectoryModules::new(description.parent().unwrap());
    let program = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.eadl",
        "",
    )
    .0;
    program_digest(&program)
}

/// The heartbeat's `/3` record on the shipped target, its program digest `program` and its target digest `digest`.
fn record(program: &str, digest: &str) -> String {
    format!(
        "(format \"archogen-assignment/3\")\n(profile rt-static-up-v1)\n(system heartbeat)\n(program \"{program}\")\n\
         (target target.riscv-virt-up \"{digest}\")\n(adapters adapters/1)\n(placement target.riscv-virt-up target.ram)\n\
         {BINDS}"
    )
}

/// Run the executable with `args`.
fn run(args: &[&str]) -> Output {
    Command::new(EXE)
        .args(args)
        .env_clear()
        .output()
        .expect("the executable runs")
}

/// Check `files`' plan, on `target` where one is given: the exit code, the record written on standard output — whose
/// status is the exit code's — and standard error.
fn check(files: &Files, target: Option<&Path>) -> (i32, Record, String) {
    let mut args: Vec<&str> = Vec::new();
    if let Some(target) = target {
        args.extend(["--target", target.to_str().unwrap()]);
    }
    args.extend([
        "--",
        files.description.to_str().unwrap(),
        files.plan.to_str().unwrap(),
    ]);
    let output = run(&args);
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    let record = Record::read(&output.stdout, 1 << 20)
        .unwrap_or_else(|e| panic!("a record on standard output: {e:?}; {stderr}"));
    let code = output.status.code().expect("an exit code");
    assert_eq!(code, record.status().code(), "{record:?}");
    (code, record, stderr)
}

#[test]
fn the_heartbeat_s_plan_on_the_shipped_target_is_accepted_with_the_target_named() {
    let files = Files::new();
    let (code, record, stderr) = check(&files, Some(&files.target));
    assert_eq!((code, record.status()), (0, Status::Ok), "{stderr}");
    assert!(record.findings().is_empty());
}

#[test]
fn the_same_plan_with_no_target_is_a_record_of_other_inputs() {
    let files = Files::new();
    let (code, record, _) = check(&files, None);
    assert_eq!(
        (code, record.status(), record.findings()[0].code.as_str()),
        (3, Status::StaleInput, PLAN_STALE)
    );
}

#[test]
fn the_same_plan_on_another_target_file_is_a_record_of_other_inputs() {
    let files = Files::new();
    let other = files.target.with_file_name("other.eadl");
    std::fs::write(
        &other,
        repository("targets/riscv-virt-up.eadl").replace(
            "(firmware-traps 0)",
            "(firmware-traps 0) (core-count 1 tick)",
        ),
    )
    .unwrap();
    let (code, record, _) = check(&files, Some(&other));
    assert_eq!(
        (code, record.status(), record.findings()[0].code.as_str()),
        (3, Status::StaleInput, PLAN_STALE)
    );
    // A `/2` record given a target is one too.
    let two = std::fs::read_to_string(&files.plan)
        .unwrap()
        .replace("archogen-assignment/3", "archogen-assignment/2")
        .lines()
        .filter(|line| !line.starts_with("(target ") && !line.contains("(release "))
        .map(|line| format!("{line}\n"))
        .collect::<String>();
    std::fs::write(&files.plan, two).unwrap();
    let (code, record, _) = check(&files, Some(&files.target));
    assert_eq!(
        (code, record.status(), record.findings()[0].code.as_str()),
        (3, Status::StaleInput, PLAN_STALE)
    );
}

#[test]
fn a_target_file_that_does_not_read_is_refused_before_a_stale_record() {
    let files = Files::new();
    // A stale record — another program's digest — beside a target file that does not read.
    let shipped = repository("targets/riscv-virt-up.eadl");
    std::fs::write(
        &files.plan,
        record(&"0".repeat(64), &target_digest(&shipped)),
    )
    .unwrap();
    std::fs::write(&files.target, "(defblock target.timer").unwrap();
    let (code, record, _) = check(&files, Some(&files.target));
    assert_eq!(record.status(), Status::InvalidDescription, "{record:?}");
    assert_eq!(code, Status::InvalidDescription.code());
    assert!(
        record
            .findings()
            .iter()
            .any(|f| f.code == "read-unclosed-list"
                && f.primary.source.ends_with("riscv-virt-up.eadl")),
        "{record:?}"
    );
    assert!(
        record.findings().iter().all(|f| f.code != PLAN_STALE),
        "{record:?}"
    );
}

#[test]
fn a_target_the_system_cannot_read_and_a_target_given_twice_are_usage_errors() {
    let files = Files::new();
    let missing = files.target.with_file_name("nowhere.eadl");
    for args in [
        vec![
            "--target",
            missing.to_str().unwrap(),
            "--",
            files.description.to_str().unwrap(),
            files.plan.to_str().unwrap(),
        ],
        vec![
            "--target",
            files.target.to_str().unwrap(),
            "--target",
            files.target.to_str().unwrap(),
            files.description.to_str().unwrap(),
            files.plan.to_str().unwrap(),
        ],
        vec![
            files.description.to_str().unwrap(),
            files.plan.to_str().unwrap(),
            "--target",
        ],
        vec!["--mcp", "--target", files.target.to_str().unwrap()],
    ] {
        let output = run(&args);
        assert_eq!(output.status.code(), Some(2), "{args:?}: {output:?}");
        assert!(output.stdout.is_empty(), "no record: {args:?}");
    }
    let help = run(&["--help"]);
    assert!(String::from_utf8_lossy(&help.stdout).contains("--target FILE"));
}
