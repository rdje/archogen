//! Leaf `M1.29.1`: what `archogen check` and `archogen build` do with a **module file**.
//!
//! No command elaborates a module tree yet — the elaborator is a library (`eadl_front::module`) that only
//! `crates/eadl-front/tests/f01_f02_modules.rs` calls. Before this leaf, both commands handed a module file
//! to the schema pass, which answered the book's own opening example with `schema-unknown-kind` for the
//! `defmodule` and `missing-fact` for a name the module imports: an `invalid-description` verdict about a
//! well-formed description, exit 10. These arms pin the honest answer instead — `unimplemented`, exit 20,
//! naming the module and the leaf that removes the refusal — and hold the `check` summary to it, so the
//! word "elaborate" cannot return to `archogen help check` without the capability behind it.
//!
//! Every module file here is written from this source into `CARGO_TARGET_TMPDIR` — inside `target/`, on the
//! repository's own volume. None is tracked: `docs/semantics/conformance.md` rule 4 puts every tracked
//! `.eadl` into the frozen `eadl/1` suite, and a module file has no §5.5 verdict to declare in its header
//! until a command can elaborate it (`M1.29.2`).

use std::path::{Path, PathBuf};

use archogen_cli::check_cmd::MODULE_ELABORATION_OWNER;
use archogen_cli::{run, spec, Status};

/// `docs/book/src/modules.md`'s opening example, byte for byte: the module file a reader of the book is
/// shown first, and so the first one they would hand to `archogen check`.
const BOOK_OPENING_EXAMPLE: &str = "\
(defmodule app.system
  (version 1 0)
  (import hw.soc  (as platform) (version (at-least 1 1)))
  (import os.time (as clock)    (version (at-least 2 0)))
  (export app.rt)
  (defsystem app.rt
    (requires (uses clock.time.monotonic))))
";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The scratch directory for this suite, created on demand.
///
/// Cargo creates `CARGO_TARGET_TMPDIR` when it *builds* a test binary, not when it runs one, so a cached
/// binary plus a cleaned `target/tmp` would make a bare write panic (`s0_build.rs` measured exactly that).
fn scratch() -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("module-files");
    std::fs::create_dir_all(&dir).expect("the target tmpdir is creatable");
    dir
}

/// Write `text` as `name` in the scratch directory and return its path.
///
/// ⛔ **One name per test.** Tests run in parallel and `fs::write` truncates before it writes, so two tests
/// sharing a name race — measured while writing this suite: a concurrent `check` read the empty file,
/// found no declarations, and *accepted* it.
fn module(name: &str, text: &str) -> String {
    let path = scratch().join(name);
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

/// Assert the refusal a module file must get, and nothing else: the process status, the module named
/// where it is, the owning leaf, and **no** diagnostic — every `error[…]` would be a verdict about a system
/// the tool never read.
fn assert_refused_as_a_module(status: Status, out: &str, err: &str, declared: &str, at: &str) {
    assert_eq!(status, Status::Unimplemented, "{err}");
    assert_eq!(status.code(), 20);
    assert!(
        err.contains(&format!("{at} is a module, `{declared}`")),
        "the refusal must name the module and where it is:\n{err}"
    );
    assert!(
        err.contains(&format!("task-tree leaf {MODULE_ELABORATION_OWNER}")),
        "the refusal must name the leaf that removes it:\n{err}"
    );
    assert!(
        err.contains("hint:"),
        "§5.5 requires a repair direction:\n{err}"
    );
    assert!(
        !err.contains("error["),
        "a module file was checked as a description — a verdict about a system the tool never read:\n{err}"
    );
    assert!(
        out.is_empty(),
        "nothing is accepted, so nothing is printed to stdout:\n{out}"
    );
}

#[test]
fn the_fixture_is_the_books_opening_example() {
    // The arms below are about what a reader of the book meets, so the claim above is held to the book.
    let chapter = std::fs::read_to_string(repo_root().join("docs/book/src/modules.md"))
        .expect("the modules chapter is readable");
    assert!(
        chapter.contains(BOOK_OPENING_EXAMPLE),
        "docs/book/src/modules.md no longer opens with this module; re-take the fixture from it"
    );
}

#[test]
fn check_refuses_the_books_opening_module_as_unimplemented_not_as_an_invalid_description() {
    let path = module("check-app.system.eadl", BOOK_OPENING_EXAMPLE);
    let (status, out, err) = invoke(&["check", &path]);
    assert_refused_as_a_module(
        status,
        &out,
        &err,
        "(defmodule app.system …)",
        &format!("{path}:1:1"),
    );
}

#[test]
fn build_refuses_the_same_module_the_same_way_and_generates_nothing() {
    let path = module("build-app.system.eadl", BOOK_OPENING_EXAMPLE);
    let dir = scratch().join("build-out");
    let _ = std::fs::remove_dir_all(&dir);
    let (status, out, err) = invoke(&["build", &path, "--out", &dir.display().to_string()]);
    assert_refused_as_a_module(
        status,
        &out,
        &err,
        "(defmodule app.system …)",
        &format!("{path}:1:1"),
    );
    assert!(
        !dir.exists(),
        "a refused build must not create its output directory"
    );
}

#[test]
fn a_module_file_that_does_not_read_is_still_refused_by_the_read_pass() {
    // A syntax error is a verdict about the bytes, whatever they were meant to be — so routing must not
    // pre-empt it, or a truncated module would be told "not implemented" instead of where it broke.
    let path = module("truncated.eadl", "(defmodule app.system\n  (version 1 0)\n");
    let (status, _, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert!(err.contains("error[read-unclosed-list]"), "{err}");
    assert!(!err.contains("is a module"), "{err}");
}

#[test]
fn a_module_is_recognised_after_the_language_version_identifier() {
    // §8: the identifier is a statement about the document, not a declaration, and a module file is the
    // file kind that most needs to carry one. Classifying by the first *form* would miss the module.
    let path = module(
        "versioned.eadl",
        "(eadl-version eadl/1)\n(defmodule hw.timer (version 1 0))\n",
    );
    let (status, out, err) = invoke(&["check", &path]);
    assert_refused_as_a_module(
        status,
        &out,
        &err,
        "(defmodule hw.timer …)",
        &format!("{path}:2:1"),
    );
}

#[test]
fn a_module_is_recognised_when_it_is_not_the_first_declaration() {
    // A module beside another declaration is still a module file — a malformed one, which is the
    // elaborator's `module-multiple-forms` to report — and not a description whose second form happens to
    // be an unknown kind.
    let path = module(
        "second.eadl",
        "(defblock console.uart (offers (observable-output true)))\n(defmodule app.system (version 1 0))\n",
    );
    let (status, out, err) = invoke(&["check", &path]);
    assert_refused_as_a_module(
        status,
        &out,
        &err,
        "(defmodule app.system …)",
        &format!("{path}:2:1"),
    );
}

#[test]
fn an_unnamed_module_is_still_a_module() {
    let path = module("unnamed.eadl", "(defmodule (version 1 0))\n");
    let (status, out, err) = invoke(&["check", &path]);
    assert_refused_as_a_module(status, &out, &err, "(defmodule …)", &format!("{path}:1:1"));
}

#[test]
fn a_description_with_no_module_is_not_routed() {
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
fn the_check_summary_claims_elaboration_only_while_a_module_file_is_elaborated() {
    // ⭐ The leg that would have caught `M1.29`: the summary is what `archogen help check` prints, and it
    // said "elaborate" while no production code called the elaborator. The word and the capability now
    // move together — restoring the word without wiring the elaborator fails here, and so does wiring the
    // elaborator without saying so.
    let summary = spec::command("check").expect("check exists").summary;
    let path = module("summary-app.system.eadl", BOOK_OPENING_EXAMPLE);
    let (status, _, err) = invoke(&["check", &path]);
    let elaborates = status != Status::Unimplemented;
    assert_eq!(
        summary.contains("elaborat"),
        elaborates,
        "`archogen help check` says {summary:?}, and a module file is {}:\n{err}",
        if elaborates {
            "elaborated"
        } else {
            "refused as unimplemented"
        }
    );
}

#[test]
fn the_module_refusal_names_a_leaf_the_tree_declares_and_has_not_closed() {
    // A refusal that names a closed or absent leaf routes the author to nothing. Held to the task tree
    // rather than to the id's shape, so closing `M1.29.2` without removing the refusal fails here.
    let tree = std::fs::read_to_string(repo_root().join("docs/tasks/M1.md"))
        .expect("the M1 task tree is readable");
    let header = format!("- ID: `{MODULE_ELABORATION_OWNER}`");
    let mut lines = tree.lines().skip_while(|line| *line != header);
    assert_eq!(
        lines.next(),
        Some(header.as_str()),
        "the refusal names {MODULE_ELABORATION_OWNER}, which docs/tasks/M1.md does not declare"
    );
    let status = lines
        .next()
        .and_then(|line| line.trim_start().strip_prefix("Status: "))
        .expect("a leaf's first field is its status");
    assert!(
        !status.starts_with("`done`"),
        "{MODULE_ELABORATION_OWNER} is closed and the refusal still names it: {status}"
    );
}
