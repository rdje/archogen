//! What `archogen check` and `archogen build` do with a **module file**, and what `archogen help check`
//! is allowed to say about it.
//!
//! Two leaves wrote this file. `M1.29.1` found both commands answering the book's own opening example with
//! `invalid-description` — `schema-unknown-kind` for the `defmodule`, `missing-fact` for a name its
//! unresolved import brings in — while `archogen help check` said "elaborate and type-check", and nothing in
//! production called the elaborator. It made the commands say so, and coupled the summary to the
//! capability. `M1.29.2` wired the elaborator in: a module file is now elaborated from its module path
//! (`docs/semantics/reference.md` §6 rule 7) and every composition rule of §6 is enforced. `M1.29.3` gave the
//! later passes a name rule to read an elaborated tree with (§6 rules 9 and 10), so a tree is now
//! type-checked like a single description, and the summary says "elaborate and type-check" again — truly.
//!
//! The pipeline-level cases — every `module-` code, `check` against `build` — are in `module_cases.rs`,
//! over the tracked fixtures under `docs/semantics/modules/`. This file holds what those cannot: the
//! classification of odd shapes of module file, the book's opening example on its own, and the legs that
//! hold `archogen help check` and the refusal's owner to what is true. Scratch files are written into
//! `CARGO_TARGET_TMPDIR`, on the repository's own volume.

use std::path::{Path, PathBuf};

use archogen_cli::{run, spec, Status};

/// `docs/book/src/modules.md`'s opening example, byte for byte: the module file a reader of the book is
/// shown first. `docs/semantics/modules/app.system.eadl` is the same text under its comment header, so the
/// example can be run where its imports resolve.
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

/// A scratch directory for one leg, created on demand and emptied first, so that a module path holds only
/// what that leg wrote.
///
/// Cargo creates `CARGO_TARGET_TMPDIR` when it *builds* a test binary, not when it runs one, so a cached
/// binary plus a cleaned `target/tmp` would make a bare write panic (`s0_build.rs` measured exactly that).
/// ⛔ **One directory per leg.** Tests run in parallel and `fs::write` truncates before it writes, so two
/// legs sharing a file race — measured while writing this suite: a concurrent `check` read the empty file,
/// found no declarations, and *accepted* it.
fn scratch(leg: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("module-files")
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

/// Every `error[<code>]` in `rendered`, in order, repeats kept.
fn codes(rendered: &str) -> Vec<String> {
    rendered
        .split("error[")
        .skip(1)
        .filter_map(|rest| rest.split(']').next())
        .map(str::to_string)
        .collect()
}

#[test]
fn the_fixture_is_the_books_opening_example_and_the_tracked_module_carries_it() {
    let chapter = std::fs::read_to_string(repo_root().join("docs/book/src/modules.md"))
        .expect("the modules chapter is readable");
    assert!(
        chapter.contains(BOOK_OPENING_EXAMPLE),
        "docs/book/src/modules.md no longer opens with this module; re-take the fixture from it"
    );
    let tracked =
        std::fs::read_to_string(repo_root().join("docs/semantics/modules/app.system.eadl"))
            .expect("the tracked F01 fixture is readable");
    assert!(
        tracked.contains(BOOK_OPENING_EXAMPLE),
        "docs/semantics/modules/app.system.eadl is no longer the book's opening example"
    );
}

#[test]
fn the_books_opening_module_alone_is_elaborated_and_both_imports_are_reported_where_written() {
    // Without its library beside it the example cannot compose — and it is told exactly that, at the two
    // import lines, instead of `defmodule is not a known kind`.
    let dir = scratch("opening-alone");
    let path = write(&dir, "app.system.eadl", BOOK_OPENING_EXAMPLE);
    let (status, out, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert_eq!(
        codes(&err),
        ["module-not-found", "module-not-found"],
        "{err}"
    );
    assert!(err.contains(&format!("--> {path}:3:3")), "{err}");
    assert!(err.contains(&format!("--> {path}:4:3")), "{err}");
    assert!(
        err.contains("hw.soc.eadl"),
        "the repair must name the file it looked for:\n{err}"
    );
    assert!(out.is_empty(), "{out}");
}

#[test]
fn build_answers_the_opening_module_alone_the_same_way_and_generates_nothing() {
    let dir = scratch("opening-alone-build");
    let path = write(&dir, "app.system.eadl", BOOK_OPENING_EXAMPLE);
    let out_dir = dir.join("out");
    let (status, _, err) = invoke(&["build", &path, "--out", &out_dir.display().to_string()]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert_eq!(
        codes(&err),
        ["module-not-found", "module-not-found"],
        "{err}"
    );
    assert!(err.contains("nothing was generated"), "{err}");
    assert!(
        !out_dir.exists(),
        "a refused build must not create its output directory"
    );
}

#[test]
fn a_module_file_that_does_not_read_is_still_refused_by_the_read_pass() {
    // A syntax error is a verdict about the bytes, whatever they were meant to be — so routing must not
    // pre-empt it, or a truncated module would be elaborated instead of told where it broke.
    let dir = scratch("truncated");
    let path = write(
        &dir,
        "truncated.eadl",
        "(defmodule app.system\n  (version 1 0)\n",
    );
    let (status, _, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert_eq!(codes(&err), ["read-unclosed-list"], "{err}");
    assert!(!err.contains("elaborated"), "{err}");
}

#[test]
fn a_module_is_recognised_after_the_language_version_identifier() {
    // §8: the identifier is a statement about the document, not a declaration, and a module file is the
    // file kind that most needs to carry one. Classifying by the first *form* would miss the module.
    let dir = scratch("versioned");
    let path = write(
        &dir,
        "hw.timer.eadl",
        "(eadl-version eadl/1)\n(defmodule hw.timer (version 1 0))\n",
    );
    let (status, out, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::Ok, "{err}");
    assert!(
        out.contains("elaborated from 1 instance(s): (root) = hw.timer 1.0"),
        "{out}"
    );
}

#[test]
fn a_module_is_recognised_when_it_is_not_the_first_declaration() {
    // A module beside another declaration is still a module file — a malformed one, which the elaborator
    // reports — and not a description whose second form happens to be an unknown kind.
    let dir = scratch("second");
    let path = write(
        &dir,
        "second.eadl",
        "(defblock console.uart (offers (observable-output true)))\n(defmodule app.system (version 1 0))\n",
    );
    let (status, _, err) = invoke(&["check", &path]);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert_eq!(codes(&err), ["module-multiple-forms"], "{err}");
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
fn the_check_summary_says_it_elaborates_exactly_while_a_module_file_is_elaborated() {
    // ⭐ The leg that would have caught `M1.29`: the summary is what `archogen help check` prints, and it
    // said "elaborate" while no production code called the elaborator. The word and the capability move
    // together — restoring one without the other fails here, in either direction.
    let summary = spec::command("check").expect("check exists").summary;
    let dir = scratch("summary-elaborate");
    let path = write(&dir, "app.system.eadl", BOOK_OPENING_EXAMPLE);
    let (_, out, err) = invoke(&["check", &path]);
    let elaborated = err.contains("error[module-") || out.contains("elaborated from");
    assert_eq!(
        summary.contains("elaborat"),
        elaborated,
        "`archogen help check` says {summary:?}, and a module file {} elaborated:\n{err}",
        if elaborated { "is" } else { "is not" }
    );
}

#[test]
fn the_check_summary_claims_to_type_check_a_module_tree_only_once_one_is() {
    // The second claim, coupled the same way: "elaborate and type-check a description" says an elaborated
    // tree is type-checked. `M1.29.2` could not say it — a clean tree was answered `unimplemented` — and stated
    // the two halves separately; `M1.29.3` made it true, and this leg is what keeps it that way.
    let summary = spec::command("check").expect("check exists").summary;
    let path = repo_root()
        .join("docs/semantics/modules/app.system.eadl")
        .display()
        .to_string();
    let (status, _, err) = invoke(&["check", &path]);
    let type_checked = status != Status::Unimplemented;
    assert_eq!(
        summary.contains("elaborate and type-check a description"),
        type_checked,
        "`archogen help check` says {summary:?}, and a clean module tree is {}:\n{err}",
        if type_checked {
            "type-checked"
        } else {
            "refused as unimplemented"
        }
    );
}
