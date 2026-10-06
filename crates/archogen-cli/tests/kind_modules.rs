//! What `archogen check` and `archogen build` do with a **kind module** (leaf `M1.32`).
//!
//! `M1.31`'s census found `archogen check docs/semantics/kinds/os-rt.eadl` answering `invalid-description`,
//! exit 10, `defkind` not being a known kind — a verdict about a system the tool never read, given for the
//! toolchain's own language definition. Both commands now refuse a kind module as `unimplemented`, naming the
//! leaf that will load a kind a user writes, and generate nothing.
//!
//! The population of shipped kind modules is derived from `docs/semantics/kinds/`, never listed here, and every
//! file under it must declare a kind (`SR-H3`). Scratch files are written into `CARGO_TARGET_TMPDIR`, on the
//! repository's own volume.
//!
//! The capability vocabulary is answered the same way (leaf `M3.1.2.1`,
//! `docs/decisions/decision_substitutability-relation.md` §1.1): a file that writes a `deffact` is the language's
//! own definition and not a description, exit 20, while a module file is classified first and its `deffact` is the
//! schema's to refuse, exit 10.

use std::path::{Path, PathBuf};

use archogen_api::KIND_MODULE_OWNER;
use archogen_cli::{run, Status};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// A scratch directory for one leg, emptied first. One per leg: tests run in parallel.
fn scratch(leg: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("kind-modules")
        .join(leg);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the target tmpdir is creatable");
    dir
}

/// Write `text` as `name` in `dir` and return its path.
fn write(dir: &Path, name: &str, text: &str) -> String {
    let path = dir.join(name);
    std::fs::write(&path, text).expect("the scratch directory is writable");
    path.display().to_string()
}

/// Run an invocation and return `(status, stdout, stderr)`.
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

/// Every shipped kind module, as a path relative to the repository root.
fn shipped_kind_modules() -> Vec<String> {
    let dir = repo_root().join("docs/semantics/kinds");
    let mut found: Vec<String> = std::fs::read_dir(&dir)
        .expect("docs/semantics/kinds/ is readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "eadl"))
        .map(|path| {
            path.strip_prefix(repo_root())
                .expect("under the root")
                .display()
                .to_string()
        })
        .collect();
    found.sort();
    found
}

#[test]
fn every_shipped_kind_module_is_refused_as_what_it_is_by_check() {
    let modules = shipped_kind_modules();
    assert!(
        !modules.is_empty(),
        "no kind module under docs/semantics/kinds/ — the population this leg reads is gone"
    );
    for module in &modules {
        let path = repo_root().join(module).display().to_string();
        let (status, out, err) = invoke(&["check", &path]);
        assert_eq!(status, Status::Unimplemented, "{module}: {err}");
        assert!(
            out.is_empty(),
            "{module}: a refusal prints no verdict: {out}"
        );
        assert!(
            err.contains("declares a kind, `(defkind "),
            "{module}: {err}"
        );
        assert!(
            err.contains(&format!("task-tree leaf {KIND_MODULE_OWNER}")),
            "{module}: {err}"
        );
        assert!(
            !err.contains("error["),
            "{module}: no diagnostic, no verdict: {err}"
        );
    }
}

#[test]
fn build_refuses_a_kind_module_the_same_way_and_generates_nothing() {
    let dir = scratch("build");
    let out_dir = dir.join("out");
    let path = repo_root()
        .join("docs/semantics/kinds/os-rt.eadl")
        .display()
        .to_string();
    let (status, _, err) = invoke(&["build", &path, "--out", &out_dir.display().to_string()]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(err.contains("`(defkind task …)`"), "{err}");
    assert!(
        !out_dir.exists(),
        "a refused build must not create its output directory"
    );
}

#[test]
fn the_refusal_says_where_the_first_kind_is_declared() {
    // After the language-version identifier (§8), which is not a declaration, and after another declaration:
    // classifying by the first form, or by the first declaration, would miss the kind.
    let dir = scratch("where");
    let path = write(
        &dir,
        "mine.eadl",
        "(eadl-version eadl/1)\n(defblock console.uart (offers (observable-output true)))\n(defkind widget\n  (clauses))\n",
    );
    let (status, _, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(
        err.contains("mine.eadl:3:1 declares a kind, `(defkind widget …)`"),
        "{err}"
    );
}

#[test]
fn a_kind_module_that_does_not_read_is_still_refused_by_the_read_pass() {
    // A syntax error is a verdict about the bytes, whatever they were meant to be.
    let dir = scratch("truncated");
    let path = write(&dir, "truncated.eadl", "(defkind widget\n  (clauses)\n");
    let (status, _, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert!(err.contains("error[read-unclosed-list]"), "{err}");
    assert!(!err.contains("declares a kind"), "{err}");
}

#[test]
fn a_description_with_no_kind_is_not_routed() {
    // The negative arm: routing that caught every file would pass every arm above.
    let path = repo_root()
        .join("examples/periodic-three/system.eadl")
        .display()
        .to_string();
    let (status, out, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(out.contains("accepted against profile"), "{out}");
}

#[test]
fn the_kind_refusal_names_a_leaf_the_tree_declares_and_has_not_closed() {
    // A refusal that names a closed or absent leaf routes the author to nothing. Held to the task tree, so
    // closing the owner without replacing the refusal fails here.
    let tree = std::fs::read_to_string(repo_root().join("docs/tasks/M6.md"))
        .expect("the M6 task tree is readable");
    let header = format!("- ID: `{KIND_MODULE_OWNER}`");
    let mut lines = tree.lines().skip_while(|line| *line != header);
    assert_eq!(
        lines.next(),
        Some(header.as_str()),
        "the refusal names {KIND_MODULE_OWNER}, which docs/tasks/M6.md does not declare"
    );
    let status = lines
        .next()
        .and_then(|line| line.trim_start().strip_prefix("Status: "))
        .expect("a leaf's first field is its status");
    assert!(
        !status.starts_with("`done`"),
        "{KIND_MODULE_OWNER} is closed and the refusal still names it: {status}"
    );
}

/// The lines the book shows under `$ archogen check <relative> ; echo $?`, and the lines a run prints with the exit
/// code it echoes.
fn transcript(relative: &str) -> (Vec<String>, Vec<String>) {
    let command = format!("$ archogen check {relative} ; echo $?");
    let chapter = std::fs::read_to_string(repo_root().join("docs/book/src/checking.md"))
        .expect("the checking chapter is readable");
    let shown: Vec<String> = chapter
        .lines()
        .skip_while(|line| *line != command)
        .skip(1)
        .take_while(|line| *line != "```")
        .map(str::to_string)
        .collect();
    assert!(
        !shown.is_empty(),
        "docs/book/src/checking.md no longer shows `{command}`"
    );
    let root = format!("{}/", repo_root().display());
    let path = repo_root().join(relative).display().to_string();
    let (status, out, err) = invoke(&["check", &path]);
    let printed = format!("{out}{err}").replace(&root, "");
    let mut expected: Vec<String> = printed.lines().map(str::to_string).collect();
    expected.push(status.code().to_string());
    (shown, expected)
}

#[test]
fn the_books_kind_module_transcript_is_what_the_command_prints() {
    // `book_transcripts.rs` checks only blocks holding an `error[` line, and a refusal has none, so these blocks
    // are held here: the lines under each command equal a run, with the exit code it echoes.
    for relative in [
        "docs/semantics/kinds/os-rt.eadl",
        "docs/semantics/vocabulary/vocabulary.eadl",
    ] {
        let (shown, expected) = transcript(relative);
        assert_eq!(
            shown, expected,
            "the book's transcript of {relative} differs from a run"
        );
    }
}

#[test]
fn every_file_that_writes_a_deffact_is_the_language_s_own_definition() {
    // The vocabulary through `check` and `build`, then a description that writes an entry after another
    // declaration: classifying by the first declaration would hand it to the schema (record §1.1, `SR-H3`).
    let vocabulary = repo_root()
        .join("docs/semantics/vocabulary/vocabulary.eadl")
        .display()
        .to_string();
    let (status, out, err) = invoke(&["check", &vocabulary]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(out.is_empty(), "a refusal prints no verdict: {out}");
    assert!(
        err.contains(
            "declares a fact of the capability vocabulary, `(deffact absolute-deadline …)`"
        ),
        "{err}"
    );
    assert!(
        err.contains(&format!("task-tree leaf {KIND_MODULE_OWNER}")),
        "{err}"
    );
    assert!(!err.contains("error["), "no diagnostic, no verdict: {err}");

    let dir = scratch("deffact");
    let out_dir = dir.join("out");
    let (status, _, err) = invoke(&[
        "build",
        &vocabulary,
        "--out",
        &out_dir.display().to_string(),
    ]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(
        !out_dir.exists(),
        "a refused build must not create its output directory"
    );

    let path = write(
        &dir,
        "mine.eadl",
        "(eadl-version eadl/1)\n(defblock console.uart (offers (observable-output true)))\n(deffact parity\n  (doc \"a parity bit\")\n  (domain boolean)\n  (role guarantee)\n  (direction exact))\n",
    );
    let (status, _, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert!(
        err.contains(
            "mine.eadl:3:1 declares a fact of the capability vocabulary, `(deffact parity …)`"
        ),
        "{err}"
    );
}

#[test]
fn a_module_file_is_classified_first_and_its_deffact_refused() {
    // Record §1.1: a `deffact` inside a module's body is `schema-unknown-kind`, no description's registry holding the
    // kind; one beside its `defmodule` is `module-multiple-forms`. Both exit 10, both refusals.
    let dir = scratch("module-deffact");
    let entry = "(deffact parity (doc \"a parity bit\") (domain boolean) (role guarantee) (direction exact))";
    for (name, text, code) in [
        (
            "inside.eadl",
            format!("(eadl-version eadl/1)\n(defmodule lib.parity\n  (version 1 0)\n  (export parity)\n  {entry})\n"),
            "schema-unknown-kind",
        ),
        (
            "beside.eadl",
            format!("(eadl-version eadl/1)\n(defmodule lib.parity\n  (version 1 0)\n  (export uart)\n  (defblock uart (offers (observable-output true))))\n{entry}\n"),
            "module-multiple-forms",
        ),
    ] {
        let path = write(&dir, name, &text);
        let (status, _, err) = invoke(&["check", &path]);
        assert_eq!(status, Status::InvalidDescription, "{name}: {err}");
        let codes: Vec<&str> = err
            .lines()
            .filter_map(|l| l.strip_prefix("error[")?.split(']').next())
            .collect();
        assert_eq!(codes, [code], "{name}: {err}");
        assert!(!err.contains("capability vocabulary"), "{name}: {err}");
    }
}
