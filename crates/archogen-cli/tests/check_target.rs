//! `archogen check --target FILE` (leaf `M4.1.7.4.2.1`): a description's systems judged on a machine, the target's file read as the
//! description is and handed to the engine API's `check`, which judges as `DC-H3` and `DC-H4` say (`crates/archogen-api/src/on_target.rs`).
//! The book's transcripts of it are runs, and the verdict of every tracked description on the shipped target is frozen in
//! `tests/verdicts.rs`.

use std::path::{Path, PathBuf};

use archogen_cli::{run, Status};

const TARGET: &str = "targets/riscv-virt-up.eadl";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// `archogen` with `args`, in-process: its status, standard output and standard error.
fn invoke(args: &[&str]) -> (Status, String, String) {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(args.iter().map(|s| (*s).to_string()), &mut out, &mut err);
    (
        status,
        String::from_utf8(out).expect("utf-8 stdout"),
        String::from_utf8(err).expect("utf-8 stderr"),
    )
}

/// The repository-relative `relative`, absolute, as a test names a file from outside the root.
fn at_root(relative: &str) -> String {
    repo_root().join(relative).display().to_string()
}

/// Every block of `docs/book/src/checking.md` opened by `$ archogen check <DESCRIPTION> --target <FILE> ; echo $?`: the description, the target
/// and the lines shown under the command.
fn on_target_transcripts() -> Vec<(String, String, Vec<String>)> {
    let chapter = std::fs::read_to_string(repo_root().join("docs/book/src/checking.md"))
        .expect("the checking chapter is readable");
    let lines: Vec<&str> = chapter.lines().collect();
    let mut found = Vec::new();
    for (at, line) in lines.iter().enumerate() {
        let Some(rest) = line.strip_prefix("$ archogen check ") else {
            continue;
        };
        let Some(command) = rest.strip_suffix(" ; echo $?") else {
            continue;
        };
        let words: Vec<&str> = command.split_whitespace().collect();
        let [description, "--target", target] = words[..] else {
            continue;
        };
        let shown = lines[at + 1..]
            .iter()
            .take_while(|line| **line != "```")
            .map(|line| (*line).to_string())
            .collect();
        found.push((description.to_string(), target.to_string(), shown));
    }
    found
}

#[test]
fn the_book_s_transcripts_on_a_target_are_what_the_command_prints() {
    let transcripts = on_target_transcripts();
    assert!(
        transcripts.len() >= 2,
        "docs/book/src/checking.md shows {} run(s) on a target, and shows one accepted and one refused",
        transcripts.len()
    );
    let root = format!("{}/", repo_root().display());
    let mut statuses = Vec::new();
    for (description, target, shown) in transcripts {
        let (status, out, err) = invoke(&[
            "check",
            &at_root(&description),
            "--target",
            &at_root(&target),
        ]);
        let printed = format!("{err}{out}").replace(&root, "");
        let mut expected: Vec<String> = printed.lines().map(str::to_string).collect();
        expected.push(status.code().to_string());
        assert_eq!(
            shown, expected,
            "the book's transcript of {description} on {target} differs from a run"
        );
        statuses.push(status);
    }
    assert!(statuses.contains(&Status::Ok), "{statuses:?}");
    assert!(
        statuses.iter().any(|status| *status != Status::Ok),
        "{statuses:?}"
    );
}

#[test]
fn the_report_names_the_target_a_description_was_judged_on() {
    let (status, out, _) = invoke(&[
        "check",
        &at_root("examples/s0-heartbeat/system.eadl"),
        "--target",
        &at_root(TARGET),
    ]);
    assert_eq!(status, Status::Ok);
    assert!(
        out.lines()
            .next()
            .is_some_and(|line| line.contains(&format!("on target {}", at_root(TARGET)))),
        "{out}"
    );
    let (status, _, err) = invoke(&[
        "check",
        &at_root("examples/s0-heartbeat/system.eadl"),
        "--target",
        &at_root(&format!("{TARGET}.missing")),
    ]);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("cannot read"), "{err}");
    assert!(err.contains("a target's file"), "{err}");
}

/// A fresh scratch directory under the target directory, one per case.
fn scratch(case: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("check-target-{case}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// `resolve --target` writes the search's plan on the target, `archogen-assignment/3`, its release requester bound to the machine's
/// timer (leaf `M4.1.7.5.3`); the checker reads that format with `M4.1.7.5.4`, so until then it is asked `--unchecked`, and refused
/// without, naming that leaf.
#[test]
fn resolve_on_a_target_writes_the_plan_there() {
    let dir = scratch("resolve");
    let plan = dir.join("plan.eadl");
    let (status, _, err) = invoke(&[
        "resolve",
        &at_root("examples/s0-heartbeat/system.eadl"),
        "--target",
        &at_root(TARGET),
        "--out",
        &plan.display().to_string(),
        "--unchecked",
    ]);
    assert_eq!(status, Status::Ok, "{err}");
    let written = std::fs::read_to_string(&plan).expect("the plan is written");
    assert!(
        written.starts_with("(format \"archogen-assignment/3\")"),
        "{written}"
    );
    assert!(
        written.contains("(bind (release heartbeat) relative-delay target.timer)"),
        "{written}"
    );
    let checked = dir.join("checked.eadl");
    let (status, _, err) = invoke(&[
        "resolve",
        &at_root("examples/s0-heartbeat/system.eadl"),
        "--target",
        &at_root(TARGET),
        "--out",
        &checked.display().to_string(),
    ]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(err.contains("M4.1.7.5.4"), "{err}");
    assert!(
        !checked.exists(),
        "nothing is written for a plan nothing could check"
    );
}

/// `explain --target` accounts for a requirement as the search judged it on the target, the release requester's among them.
#[test]
fn explain_on_a_target_accounts_for_the_release_requester() {
    let (status, out, err) = invoke(&[
        "explain",
        &at_root("examples/s0-heartbeat/system.eadl"),
        "--target",
        &at_root(TARGET),
        "--requirement",
        "relative-delay",
    ]);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(out.contains("(release heartbeat)"), "{out}");
    assert!(out.contains("target.timer"), "{out}");
}

/// A build on a target lowers format `/2`, leaf `M4.1.7.6` and after: until then `build` takes no target.
#[test]
fn build_takes_no_target_yet() {
    let (status, _, err) = invoke(&[
        "build",
        &at_root("examples/s0-heartbeat/system.eadl"),
        "--target",
        &at_root(TARGET),
    ]);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("unknown option `--target`"), "{err}");
}
