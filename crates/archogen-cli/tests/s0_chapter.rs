//! The S0 chapter's figures, measured rather than carried (leaf `S0.8`).
//!
//! ⛔ **The defect this gates, and it was live.** `docs/book/src/s0.md` said "Three descriptions in
//! `examples/s0-heartbeat/`" for 38 commits after `699a068` (`S0.4`) added the fourth, while the same chapter's
//! retirement section said "The four descriptions" — the book contradicting itself — and its corpus table had no
//! row for the fourth. It is the fourth measured instance of one defect class (`M1.23`, `M1.24`, `PROGRAM.20`): a
//! count copied into prose, out of reach of whatever measured it.
//!
//! What is compared, each with a measurement taken here:
//! - every "<count> descriptions" in the chapter, spelled out or in digits, with the `.eadl` files in the
//!   directory — except the sentences in [`HISTORY`], which are true of the commit they name;
//! - the corpus table, one row per description, by name;
//! - the "<count> files" figure and the table beneath it, with what `archogen build` actually writes.
//!
//! The arms are the `M1.24` shape: each feeds a violation-returning function the prose that was actually wrong,
//! and pins the violation **count**, so a gate that over-reports fails its own arms.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use archogen_cli::run;

/// `include_str!`, so a chapter that moves or is deleted stops this crate compiling instead of gating nothing.
const CHAPTER: &str = include_str!("../../../docs/book/src/s0.md");
const CHAPTER_PATH: &str = "docs/book/src/s0.md";
const CORPUS: &str = "examples/s0-heartbeat";

/// Sentences that state a figure true of the commit they name, not of today. Listed, never inferred from the
/// tense: a past-tense scan would also excuse a stale present-tense sentence that happened to contain "landed".
const HISTORY: &[&str] =
    &["the three descriptions and their frozen observations landed in leaf `S0.1`"];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The descriptions in the corpus directory, by file name.
fn measured_descriptions() -> BTreeSet<String> {
    std::fs::read_dir(repo_root().join(CORPUS))
        .expect("the corpus directory")
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name.ends_with(".eadl"))
        .collect()
}

/// Every file `archogen build` writes for the base description, relative to its output directory.
///
/// ⛔ Built **once** per test binary. Two tests called this, and tests run in parallel: while each removed and
/// rebuilt the same directory, one walked it as the other deleted it — `arm_5` failed 4 runs in 40, measured
/// after `S0.8`'s commit. A shared measurement belongs in a `OnceLock`, not in a directory both reset.
fn measured_emitted_files() -> &'static BTreeSet<String> {
    static EMITTED: OnceLock<BTreeSet<String>> = OnceLock::new();
    EMITTED.get_or_init(build_and_list)
}

fn build_and_list() -> BTreeSet<String> {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("s0-chapter");
    let _ = std::fs::remove_dir_all(&dir);
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let source = repo_root().join(CORPUS).join("system.eadl");
    let status = run(
        [
            "build".to_string(),
            source.display().to_string(),
            "--out".to_string(),
            dir.display().to_string(),
        ],
        &mut out,
        &mut err,
    );
    assert_eq!(
        status.code(),
        0,
        "the base description must build: {}",
        String::from_utf8_lossy(&err)
    );
    let mut files = BTreeSet::new();
    let mut pending = vec![dir.clone()];
    while let Some(next) = pending.pop() {
        for entry in std::fs::read_dir(&next).expect("readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let relative = path.strip_prefix(&dir).expect("under the output directory");
                files.insert(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    files
}

/// A count written as a word or in digits.
fn count_of(word: &str) -> Option<usize> {
    const WORDS: [&str; 12] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];
    let word = word.trim_matches(|c: char| !c.is_ascii_alphanumeric());
    if let Ok(n) = word.parse() {
        return Some(n);
    }
    WORDS
        .iter()
        .position(|w| w.eq_ignore_ascii_case(word))
        .map(|i| i + 1)
}

/// Every `<count> <noun>` in the chapter, as `(line number, count)`, skipping the listed history.
fn figures(chapter: &str, noun: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    for (index, line) in chapter.lines().enumerate() {
        if HISTORY.iter().any(|history| line.contains(history)) {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        for pair in words.windows(2) {
            let noun_here = pair[1].trim_matches(|c: char| !c.is_ascii_alphabetic());
            if noun_here.eq_ignore_ascii_case(noun) {
                if let Some(n) = count_of(pair[0]) {
                    found.push((index + 1, n));
                }
            }
        }
    }
    found
}

/// The first cell of every body row of the first table after the line containing `marker`.
fn table_after<'a>(chapter: &'a str, marker: &str) -> Vec<&'a str> {
    let mut cells = Vec::new();
    let mut in_table = false;
    for line in chapter
        .lines()
        .skip_while(|line| !line.contains(marker))
        .skip(1)
    {
        if line.starts_with('|') {
            in_table = true;
            cells.push(
                line.trim_start_matches('|')
                    .split('|')
                    .next()
                    .unwrap_or("")
                    .trim(),
            );
        } else if in_table {
            break;
        }
    }
    // the header row and the delimiter row
    cells.into_iter().skip(2).collect()
}

/// What the corpus table's case names mean in the directory: `base` is `system.eadl`, every other case
/// `system-<case>.eadl`.
fn file_of_case(case: &str) -> String {
    if case == "base" {
        "system.eadl".to_string()
    } else {
        format!("system-{case}.eadl")
    }
}

/// Every way the chapter's description figures and corpus table disagree with the directory.
fn description_violations(chapter: &str, measured: &BTreeSet<String>) -> Vec<String> {
    let mut wrong = Vec::new();
    let found = figures(chapter, "descriptions");
    if found.is_empty() {
        wrong.push(format!(
            "{CHAPTER_PATH} states no count of descriptions — an unquantified claim is the one a reader cannot check"
        ));
    }
    for (line, n) in found {
        if n != measured.len() {
            wrong.push(format!(
                "{CHAPTER_PATH}:{line} says {n} descriptions, and {CORPUS}/ holds {} — {:?}",
                measured.len(),
                measured
            ));
        }
    }
    let rows: BTreeSet<String> = table_after(chapter, "## The corpus")
        .into_iter()
        .map(file_of_case)
        .collect();
    for missing in measured.difference(&rows) {
        wrong.push(format!(
            "{CHAPTER_PATH}'s corpus table has no row for {CORPUS}/{missing}"
        ));
    }
    for extra in rows.difference(measured) {
        wrong.push(format!(
            "{CHAPTER_PATH}'s corpus table has a row for {extra}, which {CORPUS}/ does not hold"
        ));
    }
    wrong
}

/// Every way the chapter's "<count> files" figure and file table disagree with what a build writes.
fn file_violations(chapter: &str, emitted: &BTreeSet<String>) -> Vec<String> {
    let mut wrong = Vec::new();
    let found = figures(chapter, "files");
    if found.is_empty() {
        wrong.push(format!("{CHAPTER_PATH} states no count of generated files"));
    }
    for (line, n) in found {
        if n != emitted.len() {
            wrong.push(format!(
                "{CHAPTER_PATH}:{line} says {n} files, and `archogen build` writes {} — {:?}",
                emitted.len(),
                emitted
            ));
        }
    }
    let rows: BTreeSet<String> = table_after(chapter, "files, and only one of them is generated")
        .into_iter()
        .map(|cell| cell.trim_matches('`').to_string())
        .collect();
    if &rows != emitted {
        wrong.push(format!(
            "{CHAPTER_PATH}'s file table lists {rows:?}, and `archogen build` writes {emitted:?}"
        ));
    }
    wrong
}

#[test]
fn the_chapter_states_what_the_corpus_and_the_build_measure() {
    let mut wrong = description_violations(CHAPTER, &measured_descriptions());
    wrong.extend(file_violations(CHAPTER, measured_emitted_files()));
    assert!(
        wrong.is_empty(),
        "{} way(s) the S0 chapter disagrees with what it describes:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

#[test]
fn arm_1_the_figure_that_was_actually_wrong_is_reported_once() {
    let measured = measured_descriptions();
    let stale = CHAPTER.replacen("Four descriptions in", "Three descriptions in", 1);
    assert_ne!(stale, CHAPTER, "the mutation did not apply — a false green");
    let wrong = description_violations(&stale, &measured);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n\n"));
    assert!(
        wrong[0].contains("says 3 descriptions")
            && wrong[0].contains(&format!("holds {}", measured.len())),
        "the complaint must name both numbers: {}",
        wrong[0]
    );
}

#[test]
fn arm_2_history_is_excused_by_the_list_and_only_by_the_list() {
    let measured = measured_descriptions();
    // The listed sentence says three and stays: it is true of `S0.1`.
    assert!(
        CHAPTER.contains(HISTORY[0]),
        "the history sentence moved — update the list, do not drop it"
    );
    assert!(description_violations(CHAPTER, &measured).is_empty());
    // The same figure in a present-tense sentence is not excused, though it sits in the same paragraph.
    let present = CHAPTER.replacen(
        HISTORY[0],
        "the three descriptions and their frozen observations are the corpus",
        1,
    );
    assert_ne!(
        present, CHAPTER,
        "the mutation did not apply — a false green"
    );
    assert_eq!(description_violations(&present, &measured).len(), 1);
}

#[test]
fn arm_3_a_description_with_no_row_is_reported_by_name() {
    // The table as it was: no row for the non-harmonic case.
    let measured = measured_descriptions();
    let row = CHAPTER
        .lines()
        .find(|line| line.starts_with("| non-harmonic |"))
        .expect("the non-harmonic row");
    let without = CHAPTER.replacen(&format!("{row}\n"), "", 1);
    let wrong = description_violations(&without, &measured);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n\n"));
    assert!(
        wrong[0].contains("no row for examples/s0-heartbeat/system-non-harmonic.eadl"),
        "{}",
        wrong[0]
    );
}

#[test]
fn arm_4_a_chapter_that_stops_counting_is_reported_rather_than_skipped() {
    let measured = measured_descriptions();
    let vague = CHAPTER
        .replacen("Four descriptions in", "Descriptions in", 1)
        .replacen(
            "The four descriptions survive",
            "The descriptions survive",
            1,
        );
    assert_eq!(description_violations(&vague, &measured).len(), 1);
    assert!(description_violations(&vague, &measured)[0].contains("states no count"));
}

#[test]
fn arm_5_the_file_figure_and_table_follow_the_build() {
    let emitted = measured_emitted_files();
    assert!(file_violations(CHAPTER, emitted).is_empty());
    let fewer: BTreeSet<String> = emitted
        .iter()
        .filter(|f| *f != "provenance.json")
        .cloned()
        .collect();
    // A build that stopped writing provenance: the figure and the table are both now wrong.
    assert_eq!(
        file_violations(CHAPTER, &fewer).len(),
        2,
        "{:?}",
        file_violations(CHAPTER, &fewer)
    );
}

#[test]
fn arm_6_the_number_reader_reads_words_and_digits_and_nothing_else() {
    assert_eq!(count_of("Four"), Some(4));
    assert_eq!(count_of("**four**"), Some(4));
    assert_eq!(count_of("12"), Some(12));
    assert_eq!(count_of("The"), None);
    assert_eq!(
        figures("Five files, and one of them.", "files"),
        vec![(1, 5)]
    );
    assert_eq!(
        figures("The files here.", "files"),
        Vec::<(usize, usize)>::new()
    );
}
