//! Leaf `M1.29.2`: every module case under `docs/semantics/modules/` through `archogen check` and
//! `archogen build`, and every `module-` code of `docs/semantics/reference.md` §4 reached by one of them.
//!
//! The directory is a module path in the sense of §6 rule 7: library modules, and one root file per case.
//! A root is the file whose comment header carries `expect:` — the §5.5 verdict the description earns —
//! and, for a refusal, `code:`, the one diagnostic code it must produce. "One" is pinned exactly: a case
//! that produced its code *and* another would pass a containment check while an author reads two messages
//! for one mistake, which is how `(version one zero)` was found reporting "declares no version".
//!
//! ⚠️ A case that expects `ok` is answered `unimplemented` naming `M1.29.3` until that leaf lands: the tree
//! elaborates, and no command type-checks an elaborated tree yet. The header states what the description
//! *is*; this suite states what the command *does* today, and the difference is one leaf.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use archogen_cli::check_cmd::MODULE_TYPE_CHECK_OWNER;
use archogen_cli::{run, Status};
use eadl_front::{read, SourceMap, Verdict};

/// The one `module-` code no tracked fixture reaches, and why — `docs/semantics/reference.md` §6 states it.
const UNREACHED: &[(&str, &str)] = &[(
    "module-too-large",
    "fires only on a module source of 2^32 bytes, which no tracked fixture carries",
)];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Run an invocation from the repository root's point of view and return `(status, stdout, stderr)`.
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

/// A fixture's path as a command is given it, anchored at the repository root: `cargo test` runs from the
/// crate's directory.
fn fixture(name: &str) -> String {
    repo_root()
        .join("docs/semantics/modules")
        .join(name)
        .display()
        .to_string()
}

/// Every distinct `error[<code>]` in `rendered`.
fn codes(rendered: &str) -> BTreeSet<String> {
    rendered
        .split("error[")
        .skip(1)
        .filter_map(|rest| rest.split(']').next())
        .map(str::to_string)
        .collect()
}

struct Case {
    /// The root file's path, as a command is given it.
    path: String,
    expect: Verdict,
    code: Option<String>,
}

/// Every file in the fixture directory, with its comment headers.
fn fixtures() -> Vec<(String, Vec<(String, String)>)> {
    let dir = repo_root().join("docs/semantics/modules");
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("docs/semantics/modules exists")
        .map(|entry| entry.expect("a readable entry").path())
        .filter(|path| path.extension().is_some_and(|e| e == "eadl"))
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|path| {
            let text = std::fs::read_to_string(&path).expect("a readable fixture");
            let mut sources = SourceMap::new();
            let id = sources
                .add(path.display().to_string(), text)
                .expect("small");
            let (document, _) = read(&sources, id);
            (path.display().to_string(), document.comment_headers())
        })
        .collect()
}

fn cases() -> Vec<Case> {
    let cases: Vec<Case> = fixtures()
        .into_iter()
        .filter_map(|(path, headers)| {
            let header = |key: &str| {
                headers
                    .iter()
                    .find(|(k, _)| k == key)
                    .map(|(_, v)| v.clone())
            };
            let expect = header("expect")?;
            Some(Case {
                expect: Verdict::parse(&expect)
                    .unwrap_or_else(|| panic!("{path}: `{expect}` is not a §5.5 verdict")),
                code: header("code"),
                path,
            })
        })
        .collect();
    assert!(
        !cases.is_empty(),
        "docs/semantics/modules holds no case, so every leg below is green on nothing"
    );
    cases
}

#[test]
fn every_case_gets_exactly_the_answer_its_header_declares() {
    let mut wrong = Vec::new();
    for case in cases() {
        let (status, out, err) = invoke(&["check", &case.path]);
        let found = codes(&err);
        if case.expect.is_ok() {
            // Until M1.29.3: elaborated, and refused as unimplemented rather than accepted or checked.
            if status != Status::Unimplemented
                || !found.is_empty()
                || !err.contains("elaborated into")
                || !err.contains(MODULE_TYPE_CHECK_OWNER)
            {
                wrong.push(format!(
                    "{}: expected a clean elaboration answered `unimplemented` naming \
                     {MODULE_TYPE_CHECK_OWNER}, got {status:?}:\n{err}",
                    case.path
                ));
            }
            continue;
        }
        let code = case
            .code
            .clone()
            .unwrap_or_else(|| panic!("{}: a refusing case must declare its `code:`", case.path));
        let want: BTreeSet<String> = [code].into_iter().collect();
        if status != Status::from_verdict(case.expect) || found != want || !out.is_empty() {
            wrong.push(format!(
                "{}: expected {:?} with exactly {want:?}, got {status:?} with {found:?}:\n{err}",
                case.path, case.expect
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "{} case(s) answered wrongly:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

#[test]
fn build_answers_every_case_exactly_as_check_does() {
    // One frontend: `build` routes a module file through the same function, so the two commands cannot
    // disagree about a module tree the way they once disagreed about a quantity (`M1.28`).
    let out_root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("module-cases");
    let mut disagreements = Vec::new();
    for (index, case) in cases().iter().enumerate() {
        let out_dir = out_root.join(index.to_string());
        let _ = std::fs::remove_dir_all(&out_dir);
        let (checked, _, check_err) = invoke(&["check", &case.path]);
        let (built, _, build_err) =
            invoke(&["build", &case.path, "--out", &out_dir.display().to_string()]);
        if checked != built || codes(&check_err) != codes(&build_err) {
            disagreements.push(format!("{}: check {checked:?}, build {built:?}", case.path));
        }
        if out_dir.exists() {
            disagreements.push(format!(
                "{}: build refused and still created {}",
                case.path,
                out_dir.display()
            ));
        }
    }
    assert!(disagreements.is_empty(), "{}", disagreements.join("\n"));
}

#[test]
fn every_module_code_the_reference_states_is_reached_by_a_case_or_stated_unreached() {
    // Both directions, against §4 as it is written: a code no case reaches is a rule no command has been
    // seen enforcing, and a case whose code §4 does not state is a rule nobody wrote down.
    let reference = std::fs::read_to_string(repo_root().join("docs/semantics/reference.md"))
        .expect("the reference is readable");
    let stated: BTreeSet<String> = reference
        .lines()
        .filter_map(|line| line.strip_prefix("| `module-"))
        .filter_map(|rest| rest.split('`').next())
        .map(|code| format!("module-{code}"))
        .collect();
    let reached: BTreeSet<String> = cases().into_iter().filter_map(|case| case.code).collect();
    let unreached: BTreeSet<String> = UNREACHED
        .iter()
        .map(|(code, _)| (*code).to_string())
        .collect();

    assert!(
        stated.len() > 20,
        "only {} `module-` rows read from §4 — the table moved and this leg is reading nothing",
        stated.len()
    );
    let missing: Vec<&String> = stated
        .iter()
        .filter(|code| !reached.contains(*code) && !unreached.contains(*code))
        .collect();
    assert!(
        missing.is_empty(),
        "§4 states {missing:?} and no case in docs/semantics/modules reaches it"
    );
    let unstated: Vec<&String> = reached.difference(&stated).collect();
    assert!(
        unstated.is_empty(),
        "cases produce {unstated:?}, which §4 does not state"
    );
    let contradicted: Vec<&String> = reached.intersection(&unreached).collect();
    assert!(
        contradicted.is_empty(),
        "{contradicted:?} is listed as unreached and a case reaches it — drop it from UNREACHED"
    );
}

#[test]
fn every_file_is_either_a_case_or_a_module_a_case_imports() {
    // A root is the file with `expect:`; every other file must be a library module some case reads, or it
    // is a fixture nothing exercises.
    let files = fixtures();
    let roots: Vec<&String> = files
        .iter()
        .filter(|(_, headers)| headers.iter().any(|(k, _)| k == "expect"))
        .map(|(path, _)| path)
        .collect();
    let library: Vec<&String> = files
        .iter()
        .filter(|(_, headers)| headers.iter().all(|(k, _)| k != "expect"))
        .map(|(path, _)| path)
        .collect();
    let texts: Vec<String> = roots
        .iter()
        .chain(library.iter())
        .map(|path| std::fs::read_to_string(path).expect("readable"))
        .collect();
    for module in &library {
        let stem = Path::new(module)
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("a stem");
        let imported = texts.iter().any(|text| {
            text.contains(&format!("(import {stem}")) || text.contains(&format!("(import {stem})"))
        });
        assert!(
            imported,
            "{module} is imported by no fixture, so nothing exercises it"
        );
    }
}

#[test]
fn f01_the_composition_elaborates_into_the_instances_the_book_names() {
    // The names `docs/book/src/modules.md` shows are what the command reports, not what a library test does.
    let (status, _, err) = invoke(&["check", &fixture("app.system.eadl")]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    for instance in [
        "platform.timer = hw.timer 1.0",
        "platform = hw.soc 1.2",
        "clock = os.time 2.0",
        "(root) = app.system 1.0",
    ] {
        assert!(err.contains(instance), "missing `{instance}`:\n{err}");
    }
}

#[test]
fn f01_one_module_imported_twice_is_two_instances() {
    let (status, _, err) = invoke(&["check", &fixture("app.two-timers.eadl")]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(err.contains("fast = hw.timer 1.0"), "{err}");
    assert!(err.contains("slow = hw.timer 1.0"), "{err}");
}

#[test]
fn f02_a_cycle_names_the_whole_chain_through_the_command() {
    let (status, _, err) = invoke(&["check", &fixture("bad.circular-import.eadl")]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert!(
        err.contains("bad.circular-import → cycle.b → cycle.c → bad.circular-import"),
        "{err}"
    );
}

#[test]
fn a_module_that_exists_and_cannot_be_read_is_a_usage_failure_not_a_missing_module() {
    // §6 rule 7: "module not found" about a file that is there would be a false statement about the
    // description. The fixture is written here, not tracked: a file that is not UTF-8 cannot be a member of
    // a suite whose first rule is that every member reads.
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("module-unreadable");
    std::fs::create_dir_all(&dir).expect("creatable");
    std::fs::write(
        dir.join("root.eadl"),
        "(defmodule root (version 1 0) (import hw.locked))\n",
    )
    .expect("writable");
    std::fs::write(dir.join("hw.locked.eadl"), [0xff_u8, 0xfe, 0x28]).expect("writable");
    let root = dir.join("root.eadl").display().to_string();
    let (status, out, err) = invoke(&["check", &root]);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("cannot read"), "{err}");
    assert!(err.contains("hw.locked.eadl"), "{err}");
    assert!(
        !err.contains("error["),
        "an unreadable module was reported as a verdict about the description:\n{err}"
    );
    assert!(out.is_empty(), "{out}");
}
