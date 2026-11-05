//! `archogen build` (leaf `M4.2.3.1`; `docs/decisions/decision_hosted-build.md` §5): from a description to a package that
//! compiles and runs, through the search, the configuration checker, the lowering and the emitter, each the real one.
//! `tests/functional_fixtures.rs` compares what a generated system observes; this file holds the command.
//!
//! Everything is written under `CARGO_TARGET_TMPDIR`: inside `target/`, on the repository's own volume, never `/tmp`.

mod common;

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_cli::consume::{PLAN_CHECKER, RECORD};
use archogen_cli::launch::{Ended, Launch, Launched, NoLaunch, Unlaunched};
use archogen_cli::Status;
use archogen_digest::Digest;
use archogen_json::Value;
use archogen_outcome::record::{Finding, Judged, Label, Producer, Record, Severity, Subject};
use common::{checker_digest, front_digest, invoke, Checker};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn example(relative: &str) -> String {
    repo_root().join(relative).display().to_string()
}

/// `HEARTBEAT`'s path from the repository's root.
const HEARTBEAT: &str = "examples/s0-heartbeat/system.eadl";

/// A directory of its own for one test, removed first, with nothing of an earlier run beside it.
fn scratch(case: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("build")
        .join(case);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("the scratch directory");
    dir
}

/// `archogen build DESCRIPTION --out DIR`, with the checker.
fn build(description: &str, dir: &Path) -> (Status, String, String) {
    invoke(
        &["build", description, "--out", &dir.display().to_string()],
        &Checker,
    )
}

/// Every file under `dir` but what a build of the package writes itself, with its bytes.
fn snapshot(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, into: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).expect("a directory") {
            let path = entry.expect("an entry").path();
            let relative = path
                .strip_prefix(root)
                .expect("inside")
                .display()
                .to_string();
            if relative == "target" || relative == "Cargo.lock" {
                continue;
            }
            if path.is_dir() {
                walk(root, &path, into);
            } else {
                into.insert(relative, fs::read(&path).expect("a file"));
            }
        }
    }
    let mut files = BTreeMap::new();
    walk(dir, dir, &mut files);
    files
}

/// The package at `dir` built offline and run: its standard output.
fn run_package(dir: &Path) -> String {
    let output = Command::new(env!("CARGO"))
        .args(["run", "--quiet", "--offline", "--manifest-path"])
        .arg(dir.join("Cargo.toml"))
        // Its own target directory: the generated package must not share, or contend for, the workspace's.
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .expect("cargo runs");
    assert!(
        output.status.success(),
        "the generated system did not build and run:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("utf-8")
}

#[test]
fn a_description_becomes_a_package_that_says_what_it_did_not_do() {
    let dir = scratch("heartbeat").join("out");
    let (status, out, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let files: Vec<String> = snapshot(&dir).into_keys().collect();
    assert_eq!(
        files,
        [
            "Cargo.toml",
            "assignment.eadl",
            "description/system.eadl",
            "plan-check.json",
            "plan.txt",
            "provenance.json",
            "rt-core/Cargo.toml",
            "rt-core/src/fault.rs",
            "rt-core/src/lib.rs",
            "rt-core/src/scheduler.rs",
            "sim-playground/Cargo.toml",
            "sim-playground/src/lib.rs",
            "src/main.rs",
        ]
    );
    assert!(out.contains("generated `system-heartbeat` into"), "{out}");
    assert!(out.contains("13 file(s): "), "{out}");
    assert!(
        out.contains("checked by `archogen-plan-check`: accepted"),
        "{out}"
    );
    assert!(
        out.contains("2 task(s), a run of 3 tick(s) of 1/100 s"),
        "{out}"
    );
    assert!(out.contains("experimental: the hosted playground"), "{out}");
    assert!(out.contains("no job holds application code"), "{out}");
    assert!(out.contains("the plan is not complete"), "{out}");
    assert!(out.contains("next: cargo run --manifest-path"), "{out}");
    assert!(!scratch_has_stage(&dir));
}

/// Whether `DIR.building` is left beside `dir`.
fn scratch_has_stage(dir: &Path) -> bool {
    let mut stage = dir.as_os_str().to_owned();
    stage.push(".building");
    Path::new(&stage).exists()
}

#[test]
#[cfg_attr(
    miri,
    ignore = "runs cargo as a child process, which Miri does not support"
)]
fn a_generated_system_builds_offline_and_runs_without_an_edit() {
    let dir = scratch("runs").join("out");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let before = snapshot(&dir);
    let trace = run_package(&dir);
    assert!(
        trace.starts_with("; archogen-playground-trace/1\n; system heartbeat\n; plan sha256:"),
        "{trace}"
    );
    assert!(trace.contains("0 s release beat\n"), "{trace}");
    assert!(trace.contains("; no job runs application code"), "{trace}");
    // "Without editing generated output" as a fact about the run.
    assert_eq!(
        before,
        snapshot(&dir),
        "a generated file changed between generation and execution"
    );
}

#[test]
fn one_description_is_one_package_wherever_it_lies_and_wherever_it_goes() {
    let text = fs::read_to_string(example(HEARTBEAT)).expect("the description");
    let root = scratch("reproducible");
    let (first_home, second_home) = (root.join("here"), root.join("there/deeper"));
    fs::create_dir_all(&first_home).expect("a directory");
    fs::create_dir_all(&second_home).expect("a directory");
    let (first, second) = (
        first_home.join("system.eadl"),
        second_home.join("system.eadl"),
    );
    fs::write(&first, &text).expect("written");
    fs::write(&second, &text).expect("written");
    let (first_out, second_out) = (root.join("one"), root.join("elsewhere/two"));
    for (description, out) in [(&first, &first_out), (&second, &second_out)] {
        let (status, _, err) = build(&description.display().to_string(), out);
        assert_eq!(status, Status::Ok, "{err}");
    }
    assert_eq!(snapshot(&first_out), snapshot(&second_out));
    let provenance = fs::read_to_string(first_out.join("provenance.json")).expect("provenance");
    let scratch_root = root.display().to_string();
    assert!(!provenance.contains(&scratch_root), "{provenance}");

    // A module tree too, each of its files where it lies, and imported by more than one: recorded once.
    let tree = repo_root().join("crates/archogen-plan/tests/corpus/modules");
    let (one, two) = (root.join("tree/one"), root.join("tree/deeper/two"));
    for home in [&one, &two] {
        fs::create_dir_all(home).expect("a directory");
        for name in ["app.main.eadl", "hw.dev.eadl", "os.svc.eadl"] {
            fs::copy(tree.join(name), home.join(name)).expect("copied");
        }
    }
    let (one_out, two_out) = (root.join("tree-one"), root.join("elsewhere/tree-two"));
    for (home, out) in [(&one, &one_out), (&two, &two_out)] {
        let (status, _, err) = build(&home.join("app.main.eadl").display().to_string(), out);
        assert_eq!(status, Status::Ok, "{err}");
    }
    assert_eq!(snapshot(&one_out), snapshot(&two_out));
}

#[test]
fn a_description_that_does_not_check_does_not_build() {
    // There is one frontend, so `build` can never accept what `check` refuses.
    let dir = scratch("refused").join("out");
    let description = example("examples/bounded-queue/system.eadl");
    let (status, out, err) = build(&description, &dir);
    let (checked, _, check_err) = invoke(&["check", &description], &NoLaunch);
    assert_eq!(status, checked, "{err}\n{check_err}");
    assert_eq!(status, Status::UnsupportedProfile, "{err}");
    assert!(err.contains("nothing was generated"), "{err}");
    assert!(out.is_empty(), "{out}");
    assert!(!dir.exists() && !scratch_has_stage(&dir));
}

#[test]
fn a_bad_unit_is_a_malformed_description_not_a_tool_failure() {
    // A mistyped unit once exited 70 and told its author to file a bug about the toolchain (leaf `M1.28.1`).
    let root = scratch("bad-unit");
    let text = fs::read_to_string(example(HEARTBEAT)).expect("the description");
    let corrupted = text.replacen("(period 10 ms)", "(period 10 parsec)", 1);
    assert_ne!(corrupted, text, "nothing was corrupted");
    let source = root.join("system.eadl");
    fs::write(&source, corrupted).expect("written");
    let (status, out, err) = build(&source.display().to_string(), &root.join("out"));
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert_eq!(status.code(), 10);
    assert!(err.contains("quantity-unknown-unit"), "{err}");
    assert!(out.is_empty(), "{out}");
}

#[test]
fn a_description_with_no_system_has_nothing_to_build() {
    let root = scratch("no-system");
    let source = root.join("library.eadl");
    fs::write(
        &source,
        "(defblock console.uart (offers (observable-output true)))\n",
    )
    .expect("written");
    let (status, _, err) = build(&source.display().to_string(), &root.join("out"));
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("declares no system"), "{err}");
    assert!(!root.join("out").exists());
}

#[test]
fn locked_is_refused_rather_than_accepted_and_ignored() {
    // ⛔ There is no lock data to honor it with until `M4.7`, and a build that took the flag silently would be an unlocked
    // one wearing a locked one's label (§10.3).
    let dir = scratch("locked").join("out");
    let description = example(HEARTBEAT);
    let (status, out, err) = invoke(
        &[
            "build",
            &description,
            "--out",
            &dir.display().to_string(),
            "--locked",
        ],
        &Checker,
    );
    assert_eq!(status, Status::Unimplemented);
    assert_eq!(status.code(), 20);
    assert!(err.contains("M4.7"), "{err}");
    assert!(out.is_empty(), "{out}");
    assert!(!dir.exists());
}

#[test]
fn a_description_of_several_systems_is_built_one_system_at_a_time() {
    let root = scratch("several");
    let text = fs::read_to_string(example(HEARTBEAT)).expect("the description");
    let second = text
        .split("(defsystem heartbeat")
        .nth(1)
        .expect("the system's text")
        .to_owned();
    let source = root.join("system.eadl");
    fs::write(&source, format!("{text}\n(defsystem other{second}")).expect("written");
    let source = source.display().to_string();
    let dir = root.join("out");
    let dir_text = dir.display().to_string();
    let (status, _, err) = build(&source, &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("several systems"), "{err}");
    assert!(!dir.exists());
    let (status, out, err) = invoke(
        &["build", &source, "--out", &dir_text, "--system", "other"],
        &Checker,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(out.contains("`system-other`"), "{out}");
}

#[test]
fn a_task_a_source_releases_is_reported() {
    let dir = scratch("source").join("out");
    let description = repo_root().join("crates/archogen-plan/tests/corpus/sporadic.eadl");
    let text = fs::read_to_string(&description).expect("the fixture");
    let root = dir.parent().expect("a parent").to_path_buf();
    let copy = root.join("system.eadl");
    let versioned = if text.contains("(eadl-version") {
        text
    } else {
        format!("(eadl-version eadl/1)\n{text}")
    };
    fs::write(&copy, versioned).expect("written");
    let (status, out, err) = build(&copy.display().to_string(), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(out.contains("is released by the source `"), "{out}");
    assert!(
        !err.contains("experimental: the checker's acceptance"),
        "{err}"
    );
}

#[test]
fn a_task_the_plan_cannot_say_what_releases_writes_nothing() {
    let dir = scratch("unknown").join("out");
    let (status, out, err) = build(
        &example("examples/s0-heartbeat/system-unsupported.eadl"),
        &dir,
    );
    assert_eq!(status, Status::MissingFact, "{err}");
    assert!(
        err.contains("cannot say what releases task `chime`"),
        "{err}"
    );
    assert!(err.contains("(needs interrupt-source)"), "{err}");
    assert!(err.contains("nothing was generated"), "{err}");
    assert!(out.is_empty(), "{out}");
    assert!(
        !dir.exists() && !scratch_has_stage(&dir),
        "a refusal left a directory behind"
    );
}

#[test]
fn a_build_with_no_checker_to_run_generates_nothing() {
    // What every surface but the executable holds, so a library or a server can never build.
    let dir = scratch("no-checker").join("out");
    let (status, out, err) = invoke(
        &[
            "build",
            &example(HEARTBEAT),
            "--out",
            &dir.display().to_string(),
        ],
        &NoLaunch,
    );
    assert_eq!(status, Status::ToolFailure, "{err}");
    assert!(err.contains("this surface runs no program"), "{err}");
    assert!(err.contains("nothing was generated"), "{err}");
    assert!(out.is_empty(), "{out}");
    assert!(!dir.exists() && !scratch_has_stage(&dir));
}

/// A checker that writes a record of `status` about every plan it is shown, and exits with `code`.
struct Answering {
    status: Status,
    code: i32,
}

impl Launch for Answering {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        _out: &mut dyn Write,
        _err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        assert_eq!(program, PLAN_CHECKER);
        let (profile, result, plan) = (&args[1], &args[3], &args[6]);
        let text = fs::read_to_string(plan).expect("the plan the front wrote");
        let digest = text
            .lines()
            .find_map(|line| line.strip_prefix("(program \"")?.strip_suffix("\")"))
            .expect("the plan names its program");
        let record = Record::new(
            Producer {
                name: PLAN_CHECKER.to_owned(),
                sha256: checker_digest(),
            },
            Subject {
                program: Digest::parse(&format!("sha256:{digest}")).expect("a digest"),
                judged: Judged::Plan(Digest::of(text.as_bytes())),
            },
            profile.clone(),
            self.status,
            refusing_findings(self.status),
        )
        .expect("a record");
        fs::write(result, record.write()).expect("the record written");
        Ok(Launched {
            ended: Ended::Code(self.code),
            sha256: checker_digest(),
            front: front_digest(),
        })
    }
}

/// What a record refusing with `status` must hold.
fn refusing_findings(status: Status) -> Vec<Finding> {
    if status == Status::Ok {
        return Vec::new();
    }
    vec![Finding {
        code: "plan-unsatisfied".to_owned(),
        severity: Severity::Error,
        message: "a binding the relation does not satisfy".to_owned(),
        primary: Label {
            source: "plan".to_owned(),
            line: 5,
            column: 1,
            message: "here".to_owned(),
        },
        secondary: Vec::new(),
        repair: "resolve again".to_owned(),
    }]
}

#[test]
fn whatever_way_the_checker_disagrees_leaves_what_a_report_needs_and_no_package() {
    // Each route: it refuses; it finds the description wanting, which the generator did not; its two channels differ.
    for (case, status, code) in [
        ("refuses", Status::ToolFailure, Status::ToolFailure.code()),
        (
            "wanting",
            Status::InvalidDescription,
            Status::InvalidDescription.code(),
        ),
        ("channels", Status::Ok, 1),
    ] {
        let dir = scratch(case).join("out");
        let (built, out, err) = invoke(
            &[
                "build",
                &example(HEARTBEAT),
                "--out",
                &dir.display().to_string(),
            ],
            &Answering { status, code },
        );
        assert_eq!(built, Status::ToolFailure, "{case}: {err}");
        assert!(out.is_empty(), "{case}: {out}");
        assert!(!dir.exists(), "{case}: a refused plan was placed");
        let stage = beside(&dir, ".building");
        assert!(stage.join("assignment.eadl").is_file(), "{case}");
        assert!(stage.join(RECORD).is_file(), "{case}");
        assert!(
            !stage.join("Cargo.toml").exists(),
            "{case}: a refused plan was emitted"
        );
        assert!(err.contains(".building"), "{case}: {err}");
        // It is there to be read, so the next build does not take it for its own.
        let (again, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(again, Status::Usage, "{case}: {err}");
        assert!(stage.join(RECORD).is_file(), "{case}");
    }
}

#[test]
fn an_earlier_build_is_replaced_and_stays_when_the_next_is_refused() {
    let root = scratch("earlier");
    let dir = root.join("out");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    // What building the package leaves goes with it, untouched: a build wrote none of it, so none of it is a build's to delete.
    fs::create_dir_all(dir.join("target/debug")).expect("a directory");
    fs::write(dir.join("target/debug/heartbeat"), "cargo's").expect("written");
    fs::write(dir.join("Cargo.lock"), "cargo's").expect("written");
    let (status, _, err) = build(&example("examples/s0-heartbeat/system-changed.eadl"), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(
        fs::read_to_string(dir.join("target/debug/heartbeat")).expect("carried"),
        "cargo's"
    );
    assert_eq!(
        fs::read_to_string(dir.join("Cargo.lock")).expect("carried"),
        "cargo's"
    );
    assert!(!scratch_has_stage(&dir) && !beside(&dir, ".replaced").exists());
    let replaced = snapshot(&dir);
    let (status, _, err) = build(
        &example("examples/s0-heartbeat/system-unsupported.eadl"),
        &dir,
    );
    assert_eq!(status, Status::MissingFact, "{err}");
    assert_eq!(
        replaced,
        snapshot(&dir),
        "a refused build touched the earlier one"
    );
    assert!(!scratch_has_stage(&dir));
}

/// `dir`'s name with `suffix`, the place beside it.
fn beside(dir: &Path, suffix: &str) -> PathBuf {
    let mut name = dir.as_os_str().to_owned();
    name.push(suffix);
    PathBuf::from(name)
}

#[test]
fn a_build_deletes_nothing_it_did_not_write() {
    let root = scratch("foreign");
    let description = example(HEARTBEAT);
    // A directory holding anything, and a file: refused, and left alone.
    let dir = root.join("notes");
    fs::create_dir_all(&dir).expect("a directory");
    fs::write(dir.join("notes.txt"), "mine").expect("written");
    let (status, out, err) = build(&description, &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(
        err.contains("a build deletes nothing it did not write"),
        "{err}"
    );
    assert!(out.is_empty(), "{out}");
    assert_eq!(
        fs::read_to_string(dir.join("notes.txt")).expect("kept"),
        "mine"
    );
    assert!(!scratch_has_stage(&dir));
    let file = root.join("file");
    fs::write(&file, "mine").expect("written");
    let (status, _, err) = build(&description, &file);
    assert_eq!(status, Status::Usage, "{err}");
    assert_eq!(fs::read_to_string(&file).expect("kept"), "mine");

    // A build's own provenance beside a file of mine is no build: it lists what it wrote.
    let earlier = root.join("earlier");
    let (status, _, err) = build(&description, &earlier);
    assert_eq!(status, Status::Ok, "{err}");
    let copied = root.join("copied");
    fs::create_dir_all(&copied).expect("a directory");
    fs::copy(
        earlier.join("provenance.json"),
        copied.join("provenance.json"),
    )
    .expect("copied");
    fs::write(copied.join("thesis.txt"), "mine").expect("written");
    let (status, _, err) = build(&description, &copied);
    assert_eq!(status, Status::Usage, "{err}");
    assert_eq!(
        fs::read_to_string(copied.join("thesis.txt")).expect("kept"),
        "mine"
    );

    // Mine beside an earlier build, a file of it edited, and my description kept in it: each refused, each left alone.
    fs::write(earlier.join("thesis.txt"), "mine").expect("written");
    let (status, _, err) = build(&description, &earlier);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(
        err.contains("`thesis.txt` is no file a build wrote"),
        "{err}"
    );
    assert_eq!(
        fs::read_to_string(earlier.join("thesis.txt")).expect("kept"),
        "mine"
    );
    fs::remove_file(earlier.join("thesis.txt")).expect("removed");
    let main = earlier.join("src/main.rs");
    let generated = fs::read_to_string(&main).expect("the tables");
    fs::write(&main, format!("{generated}// mine\n")).expect("written");
    let (status, _, err) = build(&description, &earlier);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(
        err.contains("`src/main.rs` has changed since the build wrote it"),
        "{err}"
    );
    assert!(fs::read_to_string(&main)
        .expect("kept")
        .ends_with("// mine\n"));
    fs::write(&main, generated).expect("restored");
    let text = fs::read_to_string(&description).expect("the description");
    let mine = earlier.join("sys.eadl");
    fs::write(&mine, &text).expect("written");
    let (status, _, err) = build(&mine.display().to_string(), &earlier);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(mine.is_file(), "the description was deleted");
    fs::remove_file(&mine).expect("removed");

    // The description a build recorded, built back into that build, is what it was.
    let recorded = earlier
        .join("description/system.eadl")
        .display()
        .to_string();
    let (status, _, err) = build(&recorded, &earlier);
    assert_eq!(status, Status::Ok, "{err}");

    // An empty directory of mine beside a build, at any depth, is mine.
    let (status, _, err) = build(&description, &earlier);
    assert_eq!(status, Status::Ok, "{err}");
    fs::create_dir_all(earlier.join("photos/2026")).expect("a directory");
    let (status, _, err) = build(&description, &earlier);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("`photos` is no file a build wrote"), "{err}");
    assert!(earlier.join("photos/2026").is_dir());
    fs::remove_dir_all(earlier.join("photos")).expect("removed");

    // Whatever stands under the names cargo uses is carried, not judged: mine under `target`, a file named `target`, a
    // directory named `Cargo.lock`, and the description being built.
    let (status, _, err) = build(&description, &earlier);
    assert_eq!(status, Status::Ok, "{err}");
    fs::create_dir_all(earlier.join("target/thesis")).expect("a directory");
    fs::write(earlier.join("target/thesis/ch1.txt"), "mine").expect("written");
    let inside = earlier.join("target/mine.eadl");
    fs::write(&inside, &text).expect("written");
    fs::create_dir_all(earlier.join("Cargo.lock")).expect("a directory");
    fs::write(earlier.join("Cargo.lock/notes"), "mine").expect("written");
    let (status, _, err) = build(&inside.display().to_string(), &earlier);
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(
        fs::read_to_string(earlier.join("target/thesis/ch1.txt")).expect("kept"),
        "mine"
    );
    assert!(inside.is_file(), "the description was deleted");
    assert_eq!(
        fs::read_to_string(earlier.join("Cargo.lock/notes")).expect("kept"),
        "mine"
    );
    let fresh = root.join("named-target");
    let (status, _, err) = build(&description, &fresh);
    assert_eq!(status, Status::Ok, "{err}");
    fs::write(fresh.join("target"), "mine").expect("written");
    let (status, _, err) = build(&description, &fresh);
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(
        fs::read_to_string(fresh.join("target")).expect("kept"),
        "mine"
    );

    // An empty directory takes a build, keeping the mode it was given.
    let empty = root.join("empty");
    fs::create_dir_all(&empty).expect("a directory");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&empty, fs::Permissions::from_mode(0o700)).expect("private");
    }
    let (status, _, err) = build(&description, &empty);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(empty.join("Cargo.toml").is_file());
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(&empty)
            .expect("a directory")
            .permissions()
            .mode();
        assert_eq!(
            mode & 0o777,
            0o700,
            "a private directory became readable by others"
        );
    }
}

/// The checker, with something appearing in the place while it runs.
struct Meddling {
    file: PathBuf,
}

impl Launch for Meddling {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        fs::write(&self.file, "mine").expect("written");
        Checker.launch(program, args, out, err)
    }
}

#[test]
fn what_appears_while_a_build_runs_stays_and_is_said() {
    let root = scratch("meanwhile");
    let dir = root.join("out");
    let description = example(HEARTBEAT);
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let (status, _, err) = invoke(
        &["build", &description, "--out", &dir.display().to_string()],
        &Meddling {
            file: dir.join("thesis.txt"),
        },
    );
    assert_eq!(status, Status::Ok, "{err}");
    let replaced = beside(&dir, ".replaced");
    assert_eq!(
        fs::read_to_string(replaced.join("thesis.txt")).expect("kept"),
        "mine"
    );
    assert!(err.contains("keeps what could not go"), "{err}");
    assert!(err.contains("thesis.txt"), "{err}");
    assert!(
        !replaced.join("Cargo.toml").exists(),
        "what the earlier build wrote stays behind"
    );
    assert!(dir.join("src/main.rs").is_file());
    // The next build does not take it for its own.
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert_eq!(
        fs::read_to_string(replaced.join("thesis.txt")).expect("kept"),
        "mine"
    );
}

/// The checker, with `change` done to the place while it runs.
struct Changing {
    change: fn(&Path),
    dir: PathBuf,
}

impl Launch for Changing {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        (self.change)(&self.dir);
        Checker.launch(program, args, out, err)
    }
}

#[test]
fn a_listed_file_changed_while_a_build_runs_stays_and_is_said() {
    let root = scratch("edited-meanwhile");
    let description = example(HEARTBEAT);
    let again = |dir: &Path, change: fn(&Path)| {
        let (status, _, err) = build(&description, dir);
        assert_eq!(status, Status::Ok, "{err}");
        let (status, _, err) = invoke(
            &["build", &description, "--out", &dir.display().to_string()],
            &Changing {
                change,
                dir: dir.to_path_buf(),
            },
        );
        assert_eq!(status, Status::Ok, "{err}");
        err
    };
    // Edited: it is not what was written.
    let dir = root.join("edited");
    let err = again(&dir, |dir| {
        fs::write(dir.join("plan.txt"), "mine").expect("written")
    });
    assert_eq!(
        fs::read_to_string(beside(&dir, ".replaced").join("plan.txt")).expect("kept"),
        "mine"
    );
    assert!(err.contains("plan.txt"), "{err}");
    // An empty directory of mine: it is named too.
    let dir = root.join("empty");
    let err = again(&dir, |dir| {
        fs::create_dir_all(dir.join("photos")).expect("a directory")
    });
    assert!(beside(&dir, ".replaced").join("photos").is_dir());
    assert!(err.contains("photos/"), "{err}");
    #[cfg(unix)]
    {
        // A pipe in place of a listed file is not read, and a link is not followed or removed.
        let dir = root.join("pipe");
        let err = again(&dir, |dir| {
            let file = dir.join("plan.txt");
            fs::remove_file(&file).expect("removed");
            let made = Command::new("mkfifo")
                .arg(&file)
                .status()
                .expect("mkfifo runs");
            assert!(made.success());
        });
        assert!(err.contains("plan.txt"), "{err}");
        let dir = root.join("link");
        let err = again(&dir, |dir| {
            let file = dir.join("plan.txt");
            fs::remove_file(&file).expect("removed");
            std::os::unix::fs::symlink("elsewhere", &file).expect("a link");
        });
        assert!(err.contains("plan.txt"), "{err}");
        // A directory swapped for a link to a copy holding byte-identical files, and one of mine: nothing behind it goes.
        let dir = root.join("swapped");
        let keep = root.join("keep");
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Ok, "{err}");
        fs::create_dir_all(&keep).expect("a directory");
        for name in ["lib.rs", "scheduler.rs", "fault.rs"] {
            fs::copy(dir.join("rt-core/src").join(name), keep.join(name)).expect("copied");
        }
        let swapping = |dir: &Path| {
            let src = dir.join("rt-core/src");
            fs::remove_dir_all(&src).expect("removed");
            let keep = dir.parent().expect("a parent").join("keep");
            std::os::unix::fs::symlink(keep, &src).expect("a link");
        };
        let (status, _, err) = invoke(
            &["build", &description, "--out", &dir.display().to_string()],
            &Changing {
                change: swapping,
                dir: dir.clone(),
            },
        );
        assert_eq!(status, Status::Ok, "{err}");
        assert_eq!(
            fs::read_dir(&keep).expect("a directory").count(),
            3,
            "{err}"
        );
        // What came at a name beside the place while it ran is refused rather than replaced.
        let dir = root.join("appeared");
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let appearing = |dir: &Path| {
            let mut name = dir.as_os_str().to_owned();
            name.push(".replaced");
            fs::create_dir_all(PathBuf::from(name)).expect("a directory");
        };
        let (status, _, err) = invoke(
            &["build", &description, "--out", &dir.display().to_string()],
            &Changing {
                change: appearing,
                dir: dir.clone(),
            },
        );
        assert_eq!(status, Status::ToolFailure, "{err}");
        assert!(err.contains("appeared while the build ran"), "{err}");
        assert!(err.contains("look at it, and move or remove it"), "{err}");
        assert!(!err.contains("whose parent can be written"), "{err}");
        assert!(beside(&dir, ".replaced").is_dir());
        assert!(dir.join("Cargo.toml").is_file(), "the earlier build stays");
        assert!(!beside(&dir, ".building").exists());
        // At the place itself, where nothing stood: refused, what came left alone, nothing placed.
        let dir = root.join("came");
        let (status, _, err) = invoke(
            &["build", &description, "--out", &dir.display().to_string()],
            &Changing {
                change: |dir| fs::create_dir_all(dir).expect("a directory"),
                dir: dir.clone(),
            },
        );
        assert_eq!(status, Status::ToolFailure, "{err}");
        assert!(err.contains("appeared while the build ran"), "{err}");
        assert!(err.contains("look at it, and move or remove it"), "{err}");
        assert!(!err.contains("whose parent can be written"), "{err}");
        assert_eq!(fs::read_dir(&dir).expect("a directory").count(), 0);
        assert!(!beside(&dir, ".building").exists());
        // What stood there gone while it ran: the repair says that, and not something about the parent.
        let dir = root.join("went");
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let (status, _, err) = invoke(
            &["build", &description, "--out", &dir.display().to_string()],
            &Changing {
                change: |dir| fs::remove_dir_all(dir).expect("removed"),
                dir: dir.clone(),
            },
        );
        assert_eq!(status, Status::ToolFailure, "{err}");
        assert!(
            err.contains("moved or removed what the build was moving"),
            "{err}"
        );
        assert!(!err.contains("whose parent can be written"), "{err}");
        assert!(!beside(&dir, ".building").exists());
    }
}

#[test]
fn what_is_put_in_the_stage_while_a_build_runs_lands_with_it() {
    let dir = scratch("in-the-stage").join("out");
    let description = example(HEARTBEAT);
    let (status, _, err) = invoke(
        &["build", &description, "--out", &dir.display().to_string()],
        &Changing {
            change: |dir| {
                let mut stage = dir.as_os_str().to_owned();
                stage.push(".building");
                fs::write(PathBuf::from(stage).join("mine.txt"), "mine").expect("written");
            },
            dir: dir.clone(),
        },
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(
        fs::read_to_string(dir.join("mine.txt")).expect("landed"),
        "mine"
    );
    // So the package is no longer exactly a build's, and the next build says what is in it.
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("`mine.txt` is no file a build wrote"), "{err}");
}

#[test]
fn what_looking_at_a_package_leaves_is_carried() {
    let dir = scratch("ds-store").join("out");
    let description = example(HEARTBEAT);
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    fs::write(dir.join(".DS_Store"), "the desktop's").expect("written");
    let (status, _, err) = explain(&dir.display().to_string());
    assert_eq!(status, Status::Ok, "{err}");
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(
        fs::read_to_string(dir.join(".DS_Store")).expect("carried"),
        "the desktop's"
    );
    // Deeper down it is a file no build wrote, and named.
    fs::write(dir.join("rt-core/.DS_Store"), "the desktop's").expect("written");
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(
        err.contains("`rt-core/.DS_Store` is no file a build wrote"),
        "{err}"
    );
}

#[test]
#[cfg(unix)]
fn a_private_directory_s_stage_is_private_from_the_first() {
    use std::os::unix::fs::PermissionsExt;
    let dir = scratch("private").join("out");
    fs::create_dir_all(&dir).expect("a directory");
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("private");
    let (status, _, err) = invoke(
        &[
            "build",
            &example(HEARTBEAT),
            "--out",
            &dir.display().to_string(),
        ],
        &Answering {
            status: Status::ToolFailure,
            code: Status::ToolFailure.code(),
        },
    );
    assert_eq!(status, Status::ToolFailure, "{err}");
    let kept = fs::metadata(beside(&dir, ".building"))
        .expect("kept")
        .permissions()
        .mode();
    assert_eq!(kept & 0o777, 0o700);
}

#[test]
#[cfg(unix)]
fn a_description_that_is_a_link_finds_its_imports_where_the_front_does() {
    // The checker once read them beside what the link points at, and said two programs disagreed about one input.
    let root = scratch("linked-description");
    let (here, there) = (root.join("here"), root.join("there"));
    fs::create_dir_all(&here).expect("a directory");
    fs::create_dir_all(&there).expect("a directory");
    let tree = repo_root().join("crates/archogen-plan/tests/corpus/modules");
    fs::copy(tree.join("app.main.eadl"), there.join("app.main.eadl")).expect("copied");
    for name in ["hw.dev.eadl", "os.svc.eadl"] {
        fs::copy(tree.join(name), here.join(name)).expect("copied");
    }
    let link = here.join("app.main.eadl");
    std::os::unix::fs::symlink(there.join("app.main.eadl"), &link).expect("a link");
    let (status, _, err) = build(&link.display().to_string(), &root.join("out"));
    assert_eq!(status, Status::Ok, "{err}");
}

#[test]
fn an_out_that_goes_back_through_a_name_is_refused() {
    let root = scratch("through");
    let target = format!("{}/missing/../here", root.display());
    let (status, _, err) = invoke(&["build", &example(HEARTBEAT), "--out", &target], &Checker);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("without `..` after a name"), "{err}");
    assert!(!root.join("missing").exists() && !root.join("here").exists());
}

#[test]
#[cfg(unix)]
fn an_argument_that_is_no_text_is_a_usage_error_from_the_executable() {
    use std::os::unix::ffi::OsStrExt;
    let output = Command::new(env!("CARGO_BIN_EXE_archogen"))
        .arg("check")
        .arg(std::ffi::OsStr::from_bytes(b"bad\xff"))
        .output()
        .expect("the executable runs");
    assert_eq!(
        output.status.code(),
        Some(Status::Usage.code()),
        "{output:?}"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("not UTF-8"),
        "{output:?}"
    );
}

#[test]
fn a_stage_some_other_run_left_is_refused_and_left_alone() {
    let root = scratch("stages");
    let description = example(HEARTBEAT);
    // Anything standing at either name beside `DIR`: mine, a build of that name, a description.
    for suffix in [".building", ".replaced"] {
        let dir = root.join(format!("mine{suffix}-case"));
        let other = beside(&dir, suffix);
        fs::create_dir_all(&other).expect("a directory");
        fs::write(other.join("notes.txt"), "mine").expect("written");
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Usage, "{err}");
        assert!(
            err.contains("a build deletes nothing it did not write"),
            "{err}"
        );
        assert_eq!(
            fs::read_to_string(other.join("notes.txt")).expect("kept"),
            "mine"
        );
        assert!(!dir.exists());
    }
    let dir = root.join("x");
    let first = root.join("x.building");
    let (status, _, err) = build(&description, &first);
    assert_eq!(status, Status::Ok, "{err}");
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(
        first.join("Cargo.toml").is_file(),
        "a build named like a stage was deleted"
    );
    let held = first.join("sys.eadl");
    fs::write(
        &held,
        fs::read_to_string(&description).expect("the description"),
    )
    .expect("written");
    let (status, _, err) = build(&held.display().to_string(), &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(held.is_file(), "the description was deleted");
}

/// A directory's mode gives way when the test ends, however it ends, so that the next run's `scratch` can clear it.
#[cfg(unix)]
struct Reopened(Vec<PathBuf>);

#[cfg(unix)]
impl Drop for Reopened {
    fn drop(&mut self) {
        use std::os::unix::fs::PermissionsExt;
        for dir in &self.0 {
            let _ = fs::set_permissions(dir, fs::Permissions::from_mode(0o755));
        }
    }
}

/// `dir`'s mode, its permission bits alone.
#[cfg(unix)]
fn mode_of(dir: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::symlink_metadata(dir)
        .expect("it stands")
        .permissions()
        .mode()
        & 0o777
}

/// `dir` given `mode`.
#[cfg(unix)]
fn set_mode(dir: &Path, mode: u32) {
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(dir, fs::Permissions::from_mode(mode)).expect("its mode");
}

#[test]
#[cfg(unix)]
fn a_directory_closed_to_its_owner_s_writes_takes_a_package_and_keeps_its_mode() {
    let description = example(HEARTBEAT);
    for mode in [0o555, 0o500] {
        let root = scratch(&format!("closed-{mode:o}"));
        let (status, _, err) = build(&description, &root.join("clean"));
        assert_eq!(status, Status::Ok, "{err}");
        let clean = snapshot(&root.join("clean"));
        let dir = root.join("out");
        let _reopened = Reopened(vec![dir.clone()]);
        // Empty, and it holds nothing that would stop it.
        fs::create_dir(&dir).expect("a directory");
        set_mode(&dir, mode);
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Ok, "{err}");
        assert_eq!(mode_of(&dir), mode, "{err}");
        assert_eq!(clean, snapshot(&dir), "the package is not whole");
        assert!(!beside(&dir, ".replaced").exists(), "{err}");
        assert!(!beside(&dir, ".building").exists(), "{err}");
        // An earlier build at that mode, with what cargo left in it: carried, the mode given after that.
        set_mode(&dir, 0o755);
        fs::create_dir(dir.join("target")).expect("a directory");
        fs::write(dir.join("target/kept.txt"), "cargo's").expect("written");
        set_mode(&dir, mode);
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Ok, "{err}");
        assert_eq!(mode_of(&dir), mode, "{err}");
        assert_eq!(clean, snapshot(&dir), "the package is not whole");
        assert_eq!(
            fs::read_to_string(dir.join("target/kept.txt")).expect("carried"),
            "cargo's"
        );
        assert!(!beside(&dir, ".replaced").exists(), "{err}");
    }
}

#[test]
#[cfg(unix)]
fn what_stays_of_an_earlier_build_keeps_the_mode_it_had() {
    let root = scratch("closed-stays");
    let dir = root.join("out");
    let aside = beside(&dir, ".replaced");
    let _reopened = Reopened(vec![
        dir.clone(),
        aside.clone(),
        aside.join("rt-core/src"),
        dir.join("rt-core/src"),
    ]);
    let description = example(HEARTBEAT);
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    set_mode(&dir.join("rt-core/src"), 0o555);
    set_mode(&dir, 0o555);
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(mode_of(&dir), 0o555, "{err}");
    // Running as root, nothing stands in the way, and nothing stays.
    if aside.exists() {
        assert_eq!(mode_of(&aside), 0o555, "{err}");
        assert!(err.contains("rt-core/src/"), "{err}");
        // What did go, went: the top of it was opened for that.
        assert!(!aside.join("Cargo.toml").exists(), "{err}");
    }
}

#[test]
#[cfg(unix)]
fn a_stage_closes_from_the_first_what_the_mode_it_will_take_closes_its_owner_able_to_write() {
    for (mode, stage) in [(0o555, 0o755), (0o550, 0o750), (0o500, 0o700)] {
        let dir = scratch(&format!("stage-{mode:o}")).join("out");
        let _reopened = Reopened(vec![dir.clone()]);
        fs::create_dir(&dir).expect("a directory");
        set_mode(&dir, mode);
        let (status, _, err) = invoke(
            &[
                "build",
                &example(HEARTBEAT),
                "--out",
                &dir.display().to_string(),
            ],
            &Answering {
                status: Status::ToolFailure,
                code: Status::ToolFailure.code(),
            },
        );
        assert_eq!(status, Status::ToolFailure, "{err}");
        assert_eq!(mode_of(&beside(&dir, ".building")), stage, "{err}");
        assert_eq!(
            mode_of(&dir),
            mode,
            "a refused build touched what stood there"
        );
    }
}

#[test]
fn a_name_with_no_room_for_its_stage_is_a_usage_error_and_leaves_nothing() {
    let root = scratch("no-room");
    let description = example(HEARTBEAT);
    let long = "x".repeat(250);
    // Beside a parent that stands, the check reads it at once.
    let (status, _, err) = build(&description, &root.join(&long));
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("room in its name"), "{err}");
    // Under parents that do not stand yet, a path reads as absent up to its first missing name, so only making it can tell.
    let (status, _, err) = build(&description, &root.join("a/b").join(&long));
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("room in its name"), "{err}");
    assert_eq!(
        fs::read_dir(&root).expect("a directory").count(),
        0,
        "the parents it made stayed"
    );
}

#[test]
fn a_report_that_cannot_be_written_is_said_and_is_the_exit_code() {
    // `--version` fails as it writes, and `check` when the executable flushes: each route says so.
    let description = example(HEARTBEAT);
    let check: [&str; 2] = ["check", &description];
    for args in [&["--version"][..], &check[..]] {
        let (reader, writer) = std::io::pipe().expect("a pipe");
        drop(reader);
        let output = Command::new(env!("CARGO_BIN_EXE_archogen"))
            .args(args)
            .stdout(writer)
            .output()
            .expect("the executable runs");
        assert_eq!(
            output.status.code(),
            Some(Status::ToolFailure.code()),
            "{output:?}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("could not be written whole"),
            "{output:?}"
        );
    }
}

#[test]
#[cfg(unix)]
fn a_link_is_no_place_for_a_build() {
    let root = scratch("link");
    let real = root.join("real");
    fs::create_dir_all(&real).expect("a directory");
    let link = root.join("link");
    std::os::unix::fs::symlink(&real, &link).expect("a link");
    let (status, _, err) = build(&example(HEARTBEAT), &link);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("name the directory itself"), "{err}");
    assert!(fs::symlink_metadata(&link)
        .expect("kept")
        .file_type()
        .is_symlink());
    assert_eq!(fs::read_dir(&real).expect("a directory").count(), 0);
}

#[test]
#[cfg(unix)]
fn what_stands_in_the_way_of_removing_an_earlier_build_leaves_the_new_one_whole() {
    use std::os::unix::fs::PermissionsExt;
    let root = scratch("in-the-way");
    let dir = root.join("out");
    let description = example(HEARTBEAT);
    let (status, _, err) = build(&description, &root.join("clean"));
    assert_eq!(status, Status::Ok, "{err}");
    let clean = snapshot(&root.join("clean"));
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    fs::set_permissions(dir.join("rt-core/src"), fs::Permissions::from_mode(0o555))
        .expect("read-only");
    let (status, _, err) = build(&description, &dir);
    let replaced = beside(&dir, ".replaced");
    let held_on = replaced.exists();
    if held_on {
        fs::set_permissions(
            replaced.join("rt-core/src"),
            fs::Permissions::from_mode(0o755),
        )
        .expect("writable");
    }
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(clean, snapshot(&dir), "the new package is not whole");
    // Running as root, nothing stands in the way, and there is nothing to say.
    assert_eq!(held_on, err.contains("keeps what could not go"), "{err}");
    if held_on {
        // It names what stays, and the next build refuses rather than delete it.
        assert!(err.contains("rt-core/src/"), "{err}");
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Usage, "{err}");
        assert!(err.contains(".replaced exists"), "{err}");
    }
}

#[test]
fn a_refused_build_leaves_no_directory_it_made_to_reach_its_place() {
    let root = scratch("deep");
    let dir = root.join("a/b/c");
    let (status, _, err) = build(
        &example("examples/s0-heartbeat/system-unsupported.eadl"),
        &dir,
    );
    assert_eq!(status, Status::MissingFact, "{err}");
    assert!(!root.join("a").exists());
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(dir.join("Cargo.toml").is_file());
}

#[test]
fn the_out_path_names_the_directory_its_stage_lies_beside() {
    let root = scratch("slash");
    let dir = root.join("out");
    let with_slash = format!("{}/", dir.display());
    let (status, _, err) = invoke(
        &["build", &example(HEARTBEAT), "--out", &with_slash],
        &Checker,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(dir.join("Cargo.toml").is_file());
    assert!(!scratch_has_stage(&dir));
    let (status, _, err) = invoke(&["build", &example(HEARTBEAT), "--out", ".."], &Checker);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("names no directory of its own"), "{err}");
}

/// A line number a provenance entry holds at `path`.
fn line_at(entry: &Value, path: [&str; 2]) -> usize {
    let number = entry
        .get(path[0])
        .and_then(|member| member.get(path[1]))
        .unwrap_or_else(|| panic!("no `{}.{}` in {entry:?}", path[0], path[1]));
    let Value::Number(number) = number else {
        panic!("{number:?} is no number");
    };
    usize::try_from(number.as_i64().expect("a line")).expect("a line")
}

/// The text `entry` holds at `name`.
fn text_at<'v>(entry: &'v Value, name: &str) -> &'v str {
    entry
        .get(name)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("no `{name}` in {entry:?}"))
}

#[test]
fn every_table_entry_resolves_from_both_ends() {
    // ⭐ The failure to fear is not an absent entry but one that points somewhere wrong, which sends a reader somewhere
    // confidently. So each is followed to its generated line and to the plan's, and a task's to the description's form.
    let root = scratch("provenance");
    for (case, relative, tasks, sources, steps) in [
        ("heartbeat", HEARTBEAT, 2, 0, 2),
        (
            "modules",
            "crates/archogen-plan/tests/corpus/modules/app.main.eadl",
            2,
            1,
            4,
        ),
    ] {
        let dir = root.join(case);
        let description = example(relative);
        let (status, _, err) = build(&description, &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let text = fs::read_to_string(dir.join("provenance.json")).expect("provenance");
        let value = archogen_json::read(text.as_bytes(), text.len()).expect("JSON");
        let main = fs::read_to_string(dir.join("src/main.rs")).expect("the tables");
        let plan = fs::read_to_string(dir.join("plan.txt")).expect("the plan");
        let Some(Value::Array(entries)) = value.get("entries") else {
            panic!("no entries in {text}");
        };
        let (mut seen_tasks, mut seen_sources, mut seen_steps) = (0, 0, 0);
        for entry in entries {
            let table = text_at(entry, "table");
            let at = main
                .lines()
                .nth(line_at(entry, ["generated", "line"]) - 1)
                .expect("the generated line");
            let line = text_at(entry, "plan");
            assert!(
                plan.lines().any(|held| held == line),
                "{table}: `{line}` is not the plan's"
            );
            if table.starts_with("TASKS[") {
                seen_tasks += 1;
                // The generated line names the task, and the plan's line and the description's form must name the same.
                let name = at
                    .split("name: \"")
                    .nth(1)
                    .and_then(|rest| rest.split('"').next())
                    .unwrap_or_else(|| panic!("{table}: {at}"));
                assert!(line.contains(&format!(" {name}) ")), "{table}: {line}");
                let origin = entry.get("source").expect("a source");
                let form = dir.join("description").join(text_at(origin, "file"));
                let form = fs::read_to_string(form).expect("the recorded source");
                let form = form
                    .lines()
                    .nth(line_at(entry, ["source", "line"]) - 1)
                    .expect("the source line");
                assert!(form.contains(&format!("(task {name}")), "{table}: {form}");
            } else if table.starts_with("SOURCES[") {
                seen_sources += 1;
                let name = at
                    .split("name: \"")
                    .nth(1)
                    .and_then(|rest| rest.split('"').next())
                    .unwrap_or_else(|| panic!("{table}: {at}"));
                assert!(
                    line.starts_with("(route ") && line.contains(&format!("(source {name})")),
                    "{table}: {line}"
                );
            } else {
                assert!(table.starts_with("INITIALISATION["), "{table}");
                seen_steps += 1;
                assert!(line.starts_with("(step "), "{table}: {line}");
                for node in line.trim_end_matches(')').split(' ').skip(2) {
                    assert!(at.contains(&format!("\"{node}\"")), "{table}: {at}");
                }
            }
        }
        assert_eq!(
            (seen_tasks, seen_sources, seen_steps),
            (tasks, sources, steps),
            "{case}"
        );
        // And every other file of the package is listed, with what it holds.
        let Some(Value::Array(files)) = value.get("files") else {
            panic!("no files in {text}");
        };
        let listed: BTreeMap<String, String> = files
            .iter()
            .map(|file| {
                (
                    text_at(file, "path").to_owned(),
                    text_at(file, "sha256").to_owned(),
                )
            })
            .collect();
        let mut written = snapshot(&dir);
        assert!(written.remove("provenance.json").is_some());
        assert_eq!(listed.len(), written.len(), "{case}");
        for (name, bytes) in written {
            assert_eq!(
                listed.get(&name),
                Some(&Digest::of(&bytes).to_string()),
                "{name}"
            );
        }
    }
}

/// The lines the chapter shows after `$ {command}`, up to the next command or the block's end.
fn shown<'c>(chapter: &'c str, command: &str) -> Vec<&'c str> {
    let prompt = format!("$ {command}");
    chapter
        .lines()
        .skip_while(|line| *line != prompt)
        .skip(1)
        .take_while(|line| *line != "```" && !line.starts_with("$ "))
        .collect()
}

#[test]
#[cfg_attr(
    miri,
    ignore = "runs cargo as a child process, which Miri does not support"
)]
fn the_chapter_shows_what_the_command_says_and_what_the_package_prints() {
    let chapter =
        fs::read_to_string(repo_root().join("docs/book/src/building.md")).expect("the chapter");
    let dir = scratch("chapter").join("out");
    let description = example(HEARTBEAT);
    let spell = |text: &str, description: &str, relative: &str| {
        text.replace(description, relative)
            .replace(&dir.display().to_string(), "build/heartbeat")
    };
    let (status, out, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let said = spell(&out, &description, HEARTBEAT);
    assert_eq!(
        said.lines().collect::<Vec<_>>(),
        shown(
            &chapter,
            "archogen build examples/s0-heartbeat/system.eadl --out build/heartbeat"
        )
    );
    let trace = run_package(&dir);
    assert_eq!(
        trace.lines().collect::<Vec<_>>(),
        shown(
            &chapter,
            "cargo run --quiet --manifest-path build/heartbeat/Cargo.toml"
        )
    );
    let (status, explained, err) = explain(&dir.display().to_string());
    assert_eq!(status, Status::Ok, "{err}");
    assert_eq!(
        spell(&explained, &description, HEARTBEAT)
            .lines()
            .collect::<Vec<_>>(),
        shown(
            &chapter,
            "archogen explain build/heartbeat --requirement observable-output"
        )
    );
    let refused = "examples/s0-heartbeat/system-unsupported.eadl";
    let (status, _, err) = build(&example(refused), &scratch("chapter-refused").join("out"));
    assert_eq!(status.code(), 11, "{err}");
    let said = spell(&err, &example(refused), refused);
    assert_eq!(
        said.lines().collect::<Vec<_>>(),
        shown(
            &chapter,
            "archogen build examples/s0-heartbeat/system-unsupported.eadl --out build/x"
        )
    );
}

/// `archogen explain` of `target`, for `observable-output`.
fn explain(target: &str) -> (Status, String, String) {
    invoke(
        &["explain", target, "--requirement", "observable-output"],
        &NoLaunch,
    )
}

#[test]
fn a_build_is_explained_as_its_description_is_and_its_plan_compared() {
    let dir = scratch("explain").join("out");
    let description = example(HEARTBEAT);
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let (of_description, said, err) = explain(&description);
    assert_eq!(of_description, Status::Ok, "{err}");
    let build_dir = dir.display().to_string();
    let (of_build, out, err) = explain(&build_dir);
    assert_eq!(of_build, Status::Ok, "{err}");
    // The same account, of the same text, under the build's copy of it.
    let recorded = dir.join("description/system.eadl").display().to_string();
    let account = |text: &str, path: &str| -> Vec<String> {
        text.replace(path, "SYSTEM")
            .lines()
            .filter(|line| {
                !line.contains("plan of system")
                    && !line.contains("plan for system")
                    && !line.contains("experimental")
            })
            .map(str::to_owned)
            .collect()
    };
    assert_eq!(account(&said, &description), account(&out, &recorded));
    let held = Digest::of(&fs::read(dir.join("assignment.eadl")).expect("the assignment"));
    assert!(
        out.contains(&format!(
            "is the one this build holds, assignment {held}, which `archogen-plan-check` accepted"
        )),
        "{out}"
    );
    assert!(!out.contains("unchecked"), "{out}");
}

#[test]
#[cfg_attr(
    miri,
    ignore = "runs cargo as a child process, which Miri does not support"
)]
fn a_module_tree_is_recorded_whole_built_run_and_explained_from_the_build() {
    let dir = scratch("modules").join("out");
    let root = example("crates/archogen-plan/tests/corpus/modules/app.main.eadl");
    let (status, out, err) = build(&root, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let recorded: Vec<String> = snapshot(&dir)
        .into_keys()
        .filter(|file| file.starts_with("description/"))
        .collect();
    assert_eq!(
        recorded,
        [
            "description/app.main.eadl",
            "description/hw.dev.eadl",
            "description/os.svc.eadl"
        ]
    );
    assert!(
        out.contains("is released by the source `hw.dev.e`"),
        "{out}"
    );
    assert!(
        run_package(&dir).contains(" release s\n"),
        "the source's task was never released"
    );
    // Its modules read from the build, its plan the build's, its names qualified as the plan writes them.
    let (status, said, err) = invoke(
        &[
            "explain",
            &dir.display().to_string(),
            "--requirement",
            "relative-delay",
        ],
        &NoLaunch,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(said.contains("is the one this build holds"), "{said}");
    assert!(said.contains("bound to `hw.timer.d`"), "{said}");
}

/// A build's file rewritten and its provenance kept in step, as an engine that wrote it would have: what is left to catch
/// it is the search and the record, the digests catching only an edit.
fn rewrite_in_step(dir: &Path, name: &str, contents: &str) {
    let file = dir.join(name);
    let before = Digest::of(&fs::read(&file).expect("the file"));
    fs::write(&file, contents).expect("written");
    let provenance = dir.join("provenance.json");
    let text = fs::read_to_string(&provenance).expect("provenance");
    let moved = text.replace(
        &before.to_string(),
        &Digest::of(contents.as_bytes()).to_string(),
    );
    assert_ne!(moved, text, "`{name}` is not listed");
    rewrite_provenance(dir, &moved);
}

/// A provenance replaced, its own digest kept in step.
fn rewrite_provenance(dir: &Path, text: &str) {
    const MEMBER: &str = "\"self\": \"sha256:";
    let start = text.find(MEMBER).expect("its own digest") + MEMBER.len();
    let blank = "0".repeat(64);
    let blanked = format!("{}{blank}{}", &text[..start], &text[start + 64..]);
    let own = Digest::of(blanked.as_bytes()).hex();
    let written = format!("{}{own}{}", &text[..start], &text[start + 64..]);
    fs::write(dir.join("provenance.json"), written).expect("written");
}

/// What `explain` of `target` says of a build it must not speak for: the error stream.
fn stale(target: &str) -> String {
    let (status, out, err) = explain(target);
    assert_eq!(status, Status::StaleInput, "{out}{err}");
    assert!(
        out.is_empty(),
        "no account of a plan the search did not write: {out}"
    );
    assert!(err.contains("hint: run `archogen build` again"), "{err}");
    err
}

#[test]
fn a_file_of_a_build_edited_since_is_stale_whichever_file_it_is() {
    let names: Vec<String> = {
        let dir = scratch("edited-list").join("out");
        let (status, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let mut written = snapshot(&dir);
        written.remove("provenance.json");
        written.into_keys().collect()
    };
    assert_eq!(names.len(), 12);
    for name in names {
        let dir = scratch("edited").join("out");
        let (status, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let file = dir.join(&name);
        let text = fs::read_to_string(&file).expect("the file");
        fs::write(&file, format!("{text} ")).expect("written");
        let err = stale(&dir.display().to_string());
        assert!(
            err.contains(&format!("`{name}` has changed since the build wrote it")),
            "{err}"
        );
    }
    // A link in place of a file it wrote is no file it wrote, whatever it points at.
    #[cfg(unix)]
    {
        let dir = scratch("edited-link").join("out");
        let (status, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let file = dir.join("description/system.eadl");
        fs::remove_file(&file).expect("removed");
        std::os::unix::fs::symlink(example(HEARTBEAT), &file).expect("a link");
        let err = stale(&dir.display().to_string());
        assert!(err.contains("is no file the build wrote"), "{err}");
    }
}

#[test]
fn a_build_whose_description_gives_another_plan_or_none_is_stale_and_gets_no_account() {
    let description = fs::read_to_string(example(HEARTBEAT)).expect("the description");
    let recorded = "description/system.eadl";
    let fresh = |case: &str| {
        let dir = scratch(case).join("out");
        let (status, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(status, Status::Ok, "{err}");
        dir
    };
    // Another plan: a description that checks, and resolves to another assignment.
    let dir = fresh("stale-other");
    let other = fs::read_to_string(example("examples/s0-heartbeat/system-changed.eadl"))
        .expect("the fixture");
    rewrite_in_step(&dir, recorded, &other);
    let err = stale(&dir.display().to_string());
    assert!(err.contains("holds the assignment"), "{err}");
    // No plan: the check refuses it now, its deadline past its period.
    let dir = fresh("stale-refused");
    let refused = description.replacen("(period 30 ms)", "(period 20 ms)", 1);
    assert_ne!(refused, description, "nothing was changed");
    rewrite_in_step(&dir, recorded, &refused);
    let err = stale(&dir.display().to_string());
    assert!(err.contains("resolves to none today"), "{err}");
    assert!(err.contains("error["), "the reason is said: {err}");
    // No plan: a requirement beside the one asked about has no provider, and nothing is said of the one asked about.
    let dir = fresh("stale-unmet");
    let unmet = description.replacen(
        "(requires (needs observable-output))",
        "(requires (needs observable-output) (needs relative-delay))",
        1,
    );
    assert_ne!(unmet, description, "nothing was changed");
    rewrite_in_step(&dir, recorded, &unmet);
    let err = stale(&dir.display().to_string());
    assert!(err.contains("resolves to none today"), "{err}");
}

#[test]
fn a_record_that_does_not_accept_this_plan_from_this_checker_is_stale() {
    let fresh = |case: &str| {
        let dir = scratch(case).join("out");
        let (status, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(status, Status::Ok, "{err}");
        let accepted = fs::read_to_string(dir.join(RECORD)).expect("the record");
        let record = Record::read(accepted.as_bytes(), accepted.len()).expect("a record");
        (dir, record)
    };
    type Edit = fn(&Record, &mut Producer, &mut Subject) -> (Status, Vec<Finding>);
    let cases: [(&str, Edit); 5] = [
        ("another-checker", |_, producer, _| {
            producer.sha256 = Digest::of(b"another checker");
            (Status::Ok, Vec::new())
        }),
        ("another-plan", |_, _, subject| {
            subject.judged = Judged::Plan(Digest::of(b"another plan"));
            (Status::Ok, Vec::new())
        }),
        ("another-program", |_, _, subject| {
            subject.program = Digest::of(b"another program");
            (Status::Ok, Vec::new())
        }),
        ("another-producer", |_, producer, _| {
            producer.name = "nobody".to_owned();
            (Status::Ok, Vec::new())
        }),
        ("refusing", |_, _, _| {
            (Status::ToolFailure, refusing_findings(Status::ToolFailure))
        }),
    ];
    for (case, change) in cases {
        let (dir, record) = fresh(case);
        let mut producer = record.producer().clone();
        let mut subject = *record.subject();
        let (status, findings) = change(&record, &mut producer, &mut subject);
        let edited = Record::new(
            producer,
            subject,
            record.profile().to_owned(),
            status,
            findings,
        )
        .expect("a record");
        rewrite_in_step(&dir, RECORD, &edited.write());
        let err = stale(&dir.display().to_string());
        assert!(
            err.contains("does not accept that assignment"),
            "{case}: {err}"
        );
    }
}

#[test]
fn a_file_no_build_wrote_or_a_provenance_edited_is_stale_and_gets_no_account() {
    let fresh = |case: &str| {
        let dir = scratch(case).join("out");
        let (status, _, err) = build(&example(HEARTBEAT), &dir);
        assert_eq!(status, Status::Ok, "{err}");
        dir
    };
    // A file added can change what the package does, with no listed file touched.
    let dir = fresh("foreign-build-rs");
    fs::write(dir.join("build.rs"), "fn main() {}\n").expect("written");
    let err = stale(&dir.display().to_string());
    assert!(err.contains("`build.rs` is no file a build wrote"), "{err}");
    // A note in the provenance, and the list untouched: `build` would otherwise delete it, and `explain` not see it.
    let dir = fresh("provenance-note");
    let provenance = dir.join("provenance.json");
    let text = fs::read_to_string(&provenance).expect("provenance");
    let noted = text.replacen("{\n", "{\n  \"note\": \"mine\",\n", 1);
    fs::write(&provenance, &noted).expect("written");
    let err = stale(&dir.display().to_string());
    assert!(err.contains("`provenance.json` has changed"), "{err}");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Usage, "{err}");
    assert_eq!(fs::read_to_string(&provenance).expect("kept"), noted);
    // A list that lacks what every build writes is no build's, its digest in step or not.
    let dir = fresh("no-list");
    let provenance = dir.join("provenance.json");
    let text = fs::read_to_string(&provenance).expect("provenance");
    let start = text.find("\"files\": [").expect("a list");
    rewrite_provenance(&dir, &format!("{}\"files\": []\n}}\n", &text[..start]));
    let (status, _, err) = explain(&dir.display().to_string());
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("which every build writes"), "{err}");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Usage, "{err}");
}

#[test]
#[cfg(unix)]
fn a_provenance_that_is_no_file_is_refused_and_never_read() {
    // A pipe would never end, and a link is no file a build wrote.
    let dir = scratch("provenance-odd").join("out");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let provenance = dir.join("provenance.json");
    let held = dir.with_file_name("held.json");
    fs::rename(&provenance, &held).expect("moved");
    std::os::unix::fs::symlink(&held, &provenance).expect("a link");
    let (status, _, err) = explain(&dir.display().to_string());
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("is no file a build wrote"), "{err}");
    fs::remove_file(&provenance).expect("removed");
    let made = Command::new("mkfifo")
        .arg(&provenance)
        .status()
        .expect("mkfifo runs");
    assert!(made.success());
    let (status, _, err) = explain(&dir.display().to_string());
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("is no file a build wrote"), "{err}");
}

#[test]
fn a_request_that_misses_is_a_usage_error_of_a_healthy_build() {
    let dir = scratch("misses").join("out");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let target = dir.display().to_string();
    for args in [
        vec!["explain", &target, "--requirement", "no-such-fact"],
        vec![
            "explain",
            &target,
            "--requirement",
            "observable-output",
            "--task",
            "nobody",
        ],
        vec![
            "explain",
            &target,
            "--requirement",
            "observable-output",
            "--requester",
            "nobody",
        ],
        vec![
            "explain",
            &target,
            "--requirement",
            "observable-output",
            "--system",
            "other",
        ],
    ] {
        let (status, _, err) = invoke(&args, &NoLaunch);
        assert_eq!(status, Status::Usage, "{args:?}: {err}");
        assert!(!err.contains("stale-input"), "{args:?}: {err}");
    }
    // The system it was built for may be named.
    let (status, _, err) = invoke(
        &[
            "explain",
            &target,
            "--requirement",
            "observable-output",
            "--system",
            "heartbeat",
        ],
        &NoLaunch,
    );
    assert_eq!(status, Status::Ok, "{err}");
}

#[test]
fn a_directory_that_is_no_build_is_a_usage_error_naming_what_it_lacks() {
    let root = scratch("not-a-build");
    let target = root.display().to_string();
    let (status, out, err) = explain(&target);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("`provenance.json`"), "{err}");
    assert!(err.contains("a directory `archogen build` wrote"), "{err}");
    assert!(out.is_empty(), "{out}");

    let dir = root.join("out");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let provenance = dir.join("provenance.json");
    let text = fs::read_to_string(&provenance).expect("provenance");
    let target = dir.display().to_string();
    // A name that would leave the build is never followed, for its description or a file it lists.
    for (old, new) in [
        (
            "\"description\": \"system.eadl\"",
            "\"description\": \"../../secret.eadl\"",
        ),
        ("\"path\": \"plan.txt\"", "\"path\": \"../plan.txt\""),
    ] {
        let outside = text.replace(old, new);
        assert_ne!(outside, text, "{old}");
        fs::write(&provenance, outside).expect("written");
        let (status, _, err) = explain(&target);
        assert_eq!(status, Status::Usage, "{err}");
        assert!(err.contains("lies outside the build"), "{err}");
    }
    // A format this engine does not read is said, never guessed at.
    let older = text.replace(
        archogen_emit::PROVENANCE_FORMAT,
        "archogen-build-provenance/1",
    );
    assert_ne!(older, text);
    fs::write(&provenance, older).expect("written");
    let (status, _, err) = explain(&target);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("this engine reads"), "{err}");
}
