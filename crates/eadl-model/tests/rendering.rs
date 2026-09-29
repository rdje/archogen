//! Leaf `M1.31`: every marker a rendered diagnostic draws stays on the line it is drawn under.
//!
//! The renderer sized a caret by the whole span while printing only the span's first line, so a label
//! over a multi-line declaration — which is how F27 labels a rejected `(implementation …)` body — drew 183
//! carets under a 17-character line in `docs/semantics/boundary/reject/register-programming-sequence.eadl`,
//! and one run reached column 2 474 under a 13-character line of `docs/semantics/kinds/os-rt.eadl`: 14 runs
//! over 10 of the 78 tracked descriptions. `crates/eadl-front/src/diagnostic.rs` pins the rule on
//! hand-built spans; this suite pins it where an author meets it: every diagnostic `check` produces over
//! every tracked description in the conformance suite, rendered exactly as `archogen check` renders it.
//!
//! The population is read from `docs/semantics/conformance.md`'s machine-read `suite-roots` table, not
//! listed here, so a root added there is inside this leg the day it is added.

use std::path::{Path, PathBuf};

use eadl_front::SourceMap;
use eadl_model::check::{check, default_profile, shipped_registry};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn kind_files() -> Vec<(String, String)> {
    [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|relative| {
        let text = std::fs::read_to_string(repo_root().join(relative))
            .unwrap_or_else(|e| panic!("cannot read {relative}: {e}"));
        ((*relative).to_string(), text)
    })
    .collect()
}

/// The suite's roots, from the first backticked cell of each row under the `suite-roots` marker.
fn suite_roots() -> Vec<String> {
    let manifest = std::fs::read_to_string(repo_root().join("docs/semantics/conformance.md"))
        .expect("the conformance manifest is readable");
    let roots: Vec<String> = manifest
        .lines()
        .skip_while(|line| !line.contains("<!-- machine-read: suite-roots -->"))
        .skip(1)
        .take_while(|line| line.starts_with('|'))
        .filter_map(|row| {
            let cell = row.trim_start_matches('|').split('|').next()?.trim();
            Some(cell.strip_prefix('`')?.strip_suffix('`')?.to_string())
        })
        .collect();
    assert!(
        !roots.is_empty(),
        "docs/semantics/conformance.md carries no machine-read suite-roots table"
    );
    roots
}

fn descriptions_under(dir: &Path, found: &mut Vec<PathBuf>) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("a readable entry").path();
        if path.is_dir() {
            descriptions_under(&path, found);
        } else if path.extension().is_some_and(|e| e == "eadl") {
            found.push(path);
        }
    }
}

/// Every marker line in `rendered` whose run of `^` or `-` ends past the line above it.
///
/// A zero-width span at the end of a line puts its one caret at column `len + 1`, just past the last
/// character — pointing *at the end*, which is legitimate — so an overrun is a run ending beyond that.
/// Only the indent and the run are measured, never the label text after them: measuring the whole line
/// is how this leaf's first census over-counted by more than half.
fn overruns(rendered: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut source_line: Option<(&str, &str)> = None;
    for line in rendered.lines() {
        let Some((gutter, content)) = line.split_once(" | ") else {
            source_line = None;
            continue;
        };
        if !gutter.trim().is_empty() {
            source_line = Some((gutter.trim(), content));
            continue;
        }
        let Some((number, text)) = source_line.take() else {
            continue;
        };
        let indent = content.chars().take_while(|c| *c == ' ').count();
        let run = content
            .chars()
            .skip(indent)
            .take_while(|c| matches!(c, '^' | '-'))
            .count();
        let width = text.chars().count();
        if run > 0 && indent + run > width + 1 {
            found.push(format!(
                "line {number}: a {run}-character marker from column {} under a {width}-character line",
                indent + 1
            ));
        }
    }
    found
}

#[test]
fn no_rendered_marker_runs_past_the_line_it_is_drawn_under() {
    let root = repo_root();
    let mut descriptions = Vec::new();
    for suite_root in suite_roots() {
        descriptions_under(&root.join(&suite_root), &mut descriptions);
    }
    descriptions.sort();
    assert!(
        !descriptions.is_empty(),
        "the suite roots hold no description"
    );

    let mut violations = Vec::new();
    let mut continued = 0;
    for path in &descriptions {
        let name = path
            .strip_prefix(&root)
            .expect("under the root")
            .display()
            .to_string();
        let text = std::fs::read_to_string(path).expect("a readable description");
        let mut sources = SourceMap::new();
        let registry = shipped_registry(&mut sources, &kind_files())
            .unwrap_or_else(|_| panic!("the shipped kind modules are malformed"));
        let id = sources.add(name.clone(), text).expect("small");
        let rendered = check(&sources, id, &registry, default_profile()).render(&sources);
        continued += rendered.matches("(continues to line ").count();
        violations.extend(
            overruns(&rendered)
                .into_iter()
                .map(|found| format!("{name}: {found}")),
        );
    }

    assert!(
        violations.is_empty(),
        "{} marker(s) run past their line:\n{}",
        violations.len(),
        violations.join("\n")
    );
    // ⛔ Not vacuous: the population must still render at least one multi-line label, or this leg is
    // green because nothing it could fail on was ever drawn.
    assert!(
        continued > 0,
        "no description in the suite renders a multi-line label, so this leg checks nothing it was \
         written for"
    );
}

#[test]
fn the_overrun_detector_measures_the_run_and_not_the_label() {
    // The instrument's own RED and GREEN arms, on hand-written renderings: a label longer than its line
    // is not an overrun, a caret one past the end is not an overrun, and a run past the end is one.
    let label_longer_than_line = "2 | (a)\n  | ^^^ a label much longer than the line above it\n";
    assert!(overruns(label_longer_than_line).is_empty());
    let caret_just_past_the_end = "2 | (a\n  |   ^ input ends here\n";
    assert!(overruns(caret_just_past_the_end).is_empty());
    let run_past_the_end = "2 | (a\n  | ^^^^^^ too wide\n";
    assert_eq!(overruns(run_past_the_end).len(), 1);
}
