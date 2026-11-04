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
    assert!(out.contains("12 file(s): "), "{out}");
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
    assert!(out.contains(", as the plan infers"), "{out}");
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

/// A checker that refuses every plan it is shown, as `archogen-plan-check` refuses a corrupted one.
struct Refusing;

impl Launch for Refusing {
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
            Status::ToolFailure,
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
            }],
        )
        .expect("a record");
        fs::write(result, record.write()).expect("the record written");
        Ok(Launched {
            ended: Ended::Code(Status::ToolFailure.code()),
            sha256: checker_digest(),
            front: front_digest(),
        })
    }
}

#[test]
fn a_checker_refusing_the_search_s_plan_leaves_what_a_report_needs_and_no_package() {
    let dir = scratch("disagreed").join("out");
    let (status, out, err) = invoke(
        &[
            "build",
            &example(HEARTBEAT),
            "--out",
            &dir.display().to_string(),
        ],
        &Refusing,
    );
    assert_eq!(status, Status::ToolFailure, "{err}");
    assert!(err.contains("the search and the checker disagree"), "{err}");
    assert!(out.is_empty(), "{out}");
    assert!(!dir.exists(), "a refused plan was placed");
    let mut stage = dir.as_os_str().to_owned();
    stage.push(".building");
    let stage = PathBuf::from(stage);
    assert!(stage.join("assignment.eadl").is_file());
    assert!(stage.join(RECORD).is_file());
    assert!(
        !stage.join("Cargo.toml").exists(),
        "a refused plan was emitted"
    );
}

#[test]
fn an_earlier_build_is_replaced_and_stays_when_the_next_is_refused() {
    let root = scratch("earlier");
    let dir = root.join("out");
    let (status, _, err) = build(&example(HEARTBEAT), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    // A file of the earlier build's that this one does not write goes with it.
    fs::write(dir.join("stale.txt"), "from an earlier build").expect("written");
    let (status, _, err) = build(&example("examples/s0-heartbeat/system-changed.eadl"), &dir);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(
        !dir.join("stale.txt").exists(),
        "the earlier build was not replaced"
    );
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

#[test]
fn a_directory_that_is_no_build_is_refused_and_left_alone() {
    let root = scratch("foreign");
    let dir = root.join("out");
    fs::create_dir_all(&dir).expect("a directory");
    fs::write(dir.join("notes.txt"), "mine").expect("written");
    let (status, out, err) = build(&example(HEARTBEAT), &dir);
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

    // A file where the directory would go is no directory, and an empty directory takes the build.
    let file = root.join("file");
    fs::write(&file, "mine").expect("written");
    let (status, _, err) = build(&example(HEARTBEAT), &file);
    assert_eq!(status, Status::Usage, "{err}");
    assert_eq!(fs::read_to_string(&file).expect("kept"), "mine");
    let empty = root.join("empty");
    fs::create_dir_all(&empty).expect("a directory");
    let (status, _, err) = build(&example(HEARTBEAT), &empty);
    assert_eq!(status, Status::Ok, "{err}");
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

#[test]
fn every_provenance_entry_resolves_from_both_ends() {
    // ⭐ The failure to fear is not an absent entry but one that points somewhere wrong, which sends a reader somewhere
    // confidently. So each task's is followed to the generated line and to the description's form.
    let root = scratch("provenance");
    let dir = root.join("out");
    let description = example(HEARTBEAT);
    let (status, _, err) = build(&description, &dir);
    assert_eq!(status, Status::Ok, "{err}");
    let text = fs::read_to_string(dir.join("provenance.json")).expect("provenance");
    let value = archogen_json::read(text.as_bytes(), text.len()).expect("JSON");
    let main = fs::read_to_string(dir.join("src/main.rs")).expect("the tables");
    let source = fs::read_to_string(&description).expect("the description");
    let Some(Value::Array(entries)) = value.get("entries") else {
        panic!("no entries in {text}");
    };
    let mut tasks = 0;
    for entry in entries {
        let table = entry.get("table").and_then(Value::as_str).expect("a table");
        let at = main
            .lines()
            .nth(line_at(entry, ["generated", "line"]) - 1)
            .expect("the generated line");
        if !table.starts_with("TASKS[") {
            continue;
        }
        tasks += 1;
        // The generated line names the task, and the plan's line and the description's form must name the same one.
        let name = at
            .split("name: \"")
            .nth(1)
            .and_then(|rest| rest.split('"').next())
            .unwrap_or_else(|| panic!("{table}: {at}"));
        let plan = entry
            .get("plan")
            .and_then(Value::as_str)
            .expect("a plan line");
        assert!(
            plan.contains(&format!("(task heartbeat {name})")),
            "{table}: {plan}"
        );
        let file = entry
            .get("source")
            .and_then(|origin| origin.get("file"))
            .and_then(Value::as_str);
        assert_eq!(file, Some("system.eadl"), "{table}");
        let form = source
            .lines()
            .nth(line_at(entry, ["source", "line"]) - 1)
            .expect("the source line");
        assert!(form.contains(&format!("(task {name}")), "{table}: {form}");
    }
    assert_eq!(tasks, 2);
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
