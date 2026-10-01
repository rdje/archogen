//! The book's tour shows real files, not copies that drift (leaf `PROGRAM.44`).
//!
//! `docs/book/src/tour.md` is the first chapter a newcomer reads, and it shows a description and the output its
//! generated program prints. A copy in prose is a figure out of reach of what produced it — the defect class
//! `crates/archogen-cli/tests/s0_chapter.rs` records four times — so each block the chapter marks with
//! `<!-- verbatim: <path> -->` must equal that file with its comment lines (`;`) and blank lines removed. The output
//! file is itself what F28 compares the running program with (`s0_oracle.rs`), so the chapter's output is the
//! program's.

use std::path::{Path, PathBuf};

/// `include_str!`, so a chapter that moves or is deleted stops this crate compiling instead of gating nothing.
const CHAPTER: &str = include_str!("../../../docs/book/src/tour.md");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// A file's content as the chapter shows it: no comment lines, no blank lines.
fn shown(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| !line.trim().is_empty() && !line.trim_start().starts_with(';'))
        .map(str::to_string)
        .collect()
}

/// Every marked block in `chapter` that differs from its file, read through `read`; and every marker with no block.
fn violations(chapter: &str, read: &dyn Fn(&str) -> Option<String>) -> Vec<String> {
    let lines: Vec<&str> = chapter.lines().collect();
    let mut found = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        let Some(path) = line
            .trim()
            .strip_prefix("<!-- verbatim: ")
            .and_then(|rest| rest.strip_suffix(" -->"))
        else {
            continue;
        };
        if !lines.get(index + 1).is_some_and(|l| l.starts_with("```")) {
            found.push(format!(
                "`{path}`: the marker is not followed by a fenced block"
            ));
            continue;
        }
        let block: Vec<String> = lines[index + 2..]
            .iter()
            .take_while(|l| !l.starts_with("```"))
            .map(|l| (*l).to_string())
            .collect();
        match read(path) {
            None => found.push(format!("`{path}`: no such file")),
            Some(text) if shown(&text) != block => {
                found.push(format!(
                    "`{path}`: the chapter's copy differs from the file"
                ));
            }
            Some(_) => {}
        }
    }
    found
}

#[test]
fn the_tour_shows_each_file_as_it_is() {
    let root = repo_root();
    let read = |path: &str| std::fs::read_to_string(root.join(path)).ok();
    let found = violations(CHAPTER, &read);
    assert!(found.is_empty(), "{found:#?}");
    assert_eq!(
        CHAPTER.matches("<!-- verbatim: ").count(),
        2,
        "the tour marks the description and its frozen output"
    );
}

#[test]
fn a_copy_that_drifted_is_reported() {
    // The RED arm: the chapter as it would read had the frozen output gained a release the file does not have.
    let root = repo_root();
    let read = |path: &str| std::fs::read_to_string(root.join(path)).ok();
    let drifted = CHAPTER.replacen(
        "release 20 ms beat\n",
        "release 20 ms beat\nrelease 25 ms chime\n",
        1,
    );
    assert_ne!(drifted, CHAPTER, "the arm must change the chapter");
    let found = violations(&drifted, &read);
    assert_eq!(found.len(), 1, "{found:#?}");
    assert!(found[0].contains("expected/system.txt"), "{found:#?}");

    let missing = CHAPTER.replacen(
        "examples/s0-heartbeat/system.eadl -->",
        "examples/s0-heartbeat/gone.eadl -->",
        1,
    );
    assert_eq!(
        violations(&missing, &read),
        vec!["`examples/s0-heartbeat/gone.eadl`: no such file".to_string()]
    );
}
