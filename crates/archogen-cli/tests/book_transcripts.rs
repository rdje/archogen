//! Every diagnostic the book shows is what the command prints (leaf `PROGRAM.20.2`).
//!
//! ⭐ **Why.** The book is the director's only view of the project, and a rendered diagnostic in it reads as a
//! measurement: "run this, you get this". Nothing compared one with a run, so a chapter could keep showing output the
//! tool no longer produces — and did: measured before this test, 4 of the 12 transcripts over a reproducible input
//! differed from a run, two of them silently dropping a secondary label the renderer prints.
//!
//! **The rule.** A block holding an `error[` line is a transcript.
//! - Opened by `$ archogen check <file>` (optionally `; echo $?`): the lines under the command must **equal** the
//!   command's output — the exit code appended when the command echoes it. Nothing abridged, nothing re-wrapped.
//! - With no command line, keyed by its first `-->`: an **excerpt**, which must appear verbatim in the output of
//!   `archogen check` on that file.
//! - A transcript over an input no reader can rebuild — a file that is not in the repository, or none named at all —
//!   cannot be checked. Those are a **backlog**, listed here, which may shrink and may not grow. An entry names the
//!   block by what it *is* — its chapter, its first `error[` line and its first `-->` location — and never by the line
//!   it sits on, so an edit above a block does not move it (leaf `PROGRAM.33`: the line key did, `S0.8` measured it).

use std::fs;
use std::path::{Path, PathBuf};

use archogen_cli::run;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The transcripts that name no reproducible input, each as [`Transcript::key`] — measured `2026-09-30`. Fixing one
/// means deleting its entry; adding one is refused.
const BACKLOG: &[&str] = &[
    "boundary.md: error[boundary-implementation-in-description]: `wcet` is implementation, and eADL contains no implementation @ examples/sensor.eadl:4:5",
    "presence.md: error[missing-fact]: `wrap-behavior` is required by this system and nothing describes it",
    "presence.md: error[infeasible-configuration]: `low-power-timer` is required by this system but declared absent",
    "presence.md: error[invalid-description]: `low-power-timer` is declared both offered and absent @ t.eadl:5:9",
    "reading.md: error[read-malformed-number]: `3ms` is not a number @ examples/sensor.eadl:4:13",
    "reading.md: error[read-unclosed-list]: this list is never closed @ examples/time.eadl:3:1",
    "refinement.md: error[refinement-violated]: `soc.concrete` does not offer `wrap-behavior`, which `soc.abstract` guarantees",
    "refinement.md: error[refinement-violated]: `soc.concrete` offers `dma`, which `soc.abstract` declares absent",
    "s0.md: error[read-unclosed-list]: this list is never closed @ build/s0/malformed.eadl:42:36",
    "workload.md: error[boundary-implementation-in-description]: `wcet` is implementation, and eADL contains no implementation",
    "workload.md: error[unsupported-profile]: tasks `beat` and `chime` share priority 1 @ system.eadl:40:15",
];

/// Directories whose files are generated or belong to another repository: a path there is not reproducible.
const NOT_OURS: &[&str] = &["target/", "build/", "vendor/"];

fn reproducible(root: &Path, relative: &str) -> bool {
    !relative.starts_with('/')
        && !NOT_OURS.iter().any(|dir| relative.starts_with(dir))
        && root.join(relative).is_file()
}

/// `archogen check <file>` run in-process from the repository root: everything it prints, and its exit code.
fn check(relative: &str) -> (String, i32) {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        ["check".to_string(), relative.to_string()],
        &mut out,
        &mut err,
    );
    let mut text = String::from_utf8(err).expect("utf-8");
    text.push_str(&String::from_utf8(out).expect("utf-8"));
    (text.trim_end_matches('\n').to_string(), status.code())
}

struct Transcript {
    at: String,
    /// What the block is, independent of where it sits: its chapter, first `error[` line and first `-->` location.
    key: String,
    command: Option<(String, bool)>,
    location: Option<String>,
    body: Vec<String>,
}

/// Every transcript in the book's chapters.
fn transcripts(root: &Path) -> Vec<Transcript> {
    let mut chapters: Vec<PathBuf> = fs::read_dir(root.join("docs/book/src"))
        .expect("the book's sources")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    chapters.sort();
    chapters
        .iter()
        .flat_map(|chapter| {
            let name = chapter
                .file_name()
                .expect("a name")
                .to_string_lossy()
                .to_string();
            transcripts_in(&name, &fs::read_to_string(chapter).expect("readable"))
        })
        .collect()
}

/// The transcripts in one chapter's text.
fn transcripts_in(name: &str, text: &str) -> Vec<Transcript> {
    let mut found = Vec::new();
    let lines: Vec<String> = text.lines().map(str::to_string).collect();
    let mut i = 0;
    while i < lines.len() {
        if !(lines[i].starts_with("```") && lines[i].trim() != "```") {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < lines.len() && !lines[j].starts_with("```") {
            j += 1;
        }
        let block = &lines[i + 1..j];
        if let Some(error) = block.iter().find(|l| l.starts_with("error[")) {
            let command = block.iter().find_map(|l| {
                let rest = l.strip_prefix("$ archogen check ")?;
                let echoes = rest.contains("; echo $?");
                let file = rest.split_whitespace().next()?.to_string();
                Some((file, echoes))
            });
            let first_location = block
                .iter()
                .find_map(|l| l.trim_start().strip_prefix("--> "));
            let location =
                first_location.and_then(|rest| rest.split(':').next().map(str::to_string));
            let key = match first_location {
                Some(at) => format!("{name}: {error} @ {}", at.trim()),
                None => format!("{name}: {error}"),
            };
            let body = block
                .iter()
                .filter(|l| !l.starts_with("$ "))
                .cloned()
                .collect();
            found.push(Transcript {
                at: format!("{name}:{}", i + 1),
                key,
                command,
                location,
                body,
            });
        }
        i = j + 1;
    }
    found
}

/// The ratchet: every unreproducible transcript found must have its entry, and every entry must still be found. A
/// multiset comparison, so a second copy of an existing block counts as new.
fn backlog_problems(unreproducible: &[&Transcript], backlog: &[&str]) -> Vec<String> {
    let mut entries: Vec<&str> = backlog.to_vec();
    let mut grown = Vec::new();
    for transcript in unreproducible {
        if let Some(index) = entries.iter().position(|entry| *entry == transcript.key) {
            entries.remove(index);
        } else {
            grown.push(format!("{} ({})", transcript.at, transcript.key));
        }
    }
    let mut problems = Vec::new();
    if !grown.is_empty() {
        problems.push(format!(
            "new transcript(s) over an input no reader can rebuild: {grown:?} — name a file in the repository"
        ));
    }
    if !entries.is_empty() {
        problems.push(format!(
            "backlog entries that are no longer there: {entries:?} — delete them from BACKLOG"
        ));
    }
    problems
}

#[test]
#[cfg_attr(
    miri,
    ignore = "runs the checker over every transcript in the book, as the corpus walks do"
)]
fn every_rendered_diagnostic_in_the_book_is_what_the_command_prints() {
    let root = repo_root();
    std::env::set_current_dir(&root).expect("the repository root");
    let all = transcripts(&root);
    assert!(
        all.len() >= 20,
        "found only {} transcripts — the reader is broken, not the book",
        all.len()
    );
    let mut problems = Vec::new();
    let mut unreproducible: Vec<&Transcript> = Vec::new();
    let mut checked = 0;
    for t in &all {
        let input = t
            .command
            .as_ref()
            .map(|(f, _)| f.clone())
            .or_else(|| t.location.clone());
        let Some(input) = input.filter(|f| reproducible(&root, f)) else {
            unreproducible.push(t);
            continue;
        };
        checked += 1;
        let (printed, code) = check(&input);
        match &t.command {
            Some((_, echoes)) => {
                let want = if *echoes {
                    format!("{printed}\n{code}")
                } else {
                    printed
                };
                let got = t.body.join("\n");
                if got != want {
                    problems.push(format!(
                        "{}: `archogen check {input}` prints something else — re-render the block from a run\n--- book\n{got}\n--- run\n{want}",
                        t.at
                    ));
                }
            }
            None => {
                let excerpt = t.body.join("\n");
                if !printed.contains(excerpt.trim_end()) {
                    problems.push(format!(
                        "{}: this excerpt does not appear in `archogen check {input}`'s output",
                        t.at
                    ));
                }
            }
        }
    }
    problems.extend(backlog_problems(&unreproducible, BACKLOG));
    assert!(
        problems.is_empty(),
        "{} of {} checked transcript(s) disagree with the tool:\n\n{}",
        problems.len(),
        checked,
        problems.join("\n\n")
    );
}

// ── the backlog's own arms (leaf `PROGRAM.33`) ───────────────────────────────────────────────────

/// The unreproducible transcripts of one chapter's text, as the ratchet sees them.
fn unreproducible(root: &Path, name: &str, text: &str) -> Vec<Transcript> {
    transcripts_in(name, text)
        .into_iter()
        .filter(|t| {
            let input = t
                .command
                .as_ref()
                .map(|(f, _)| f.clone())
                .or_else(|| t.location.clone());
            !input.is_some_and(|f| reproducible(root, &f))
        })
        .collect()
}

fn s0_chapter() -> (PathBuf, String, Vec<&'static str>) {
    let root = repo_root();
    let text = fs::read_to_string(root.join("docs/book/src/s0.md")).expect("the S0 chapter");
    let entries = BACKLOG
        .iter()
        .filter(|entry| entry.starts_with("s0.md: "))
        .copied()
        .collect();
    (root, text, entries)
}

const MALFORMED: &str = "error[read-unclosed-list]: this list is never closed";

#[test]
fn arm_1_an_edit_above_a_backlog_block_moves_nothing() {
    // ⭐ The failure `S0.8` measured: one paragraph above `s0.md`'s backlog block read as a new transcript and a
    // vanished one, for bytes that had not changed.
    let (root, text, entries) = s0_chapter();
    let edited = format!("# Moved\n\nA paragraph added above.\n\nAnd another.\n\n{text}");
    let before = unreproducible(&root, "s0.md", &text);
    let after = unreproducible(&root, "s0.md", &edited);
    assert!(
        !before.is_empty(),
        "the S0 chapter no longer has a backlog block to test with"
    );
    assert_ne!(
        before[0].at, after[0].at,
        "the edit did not move the block — a false green"
    );
    assert_eq!(before[0].key, after[0].key);
    assert!(backlog_problems(&after.iter().collect::<Vec<_>>(), &entries).is_empty());
}

#[test]
fn arm_2_a_changed_block_no_longer_matches_its_entry() {
    let (root, text, entries) = s0_chapter();
    let changed = text.replacen(
        MALFORMED,
        "error[read-unclosed-list]: this list never closes",
        1,
    );
    assert_ne!(changed, text, "the mutation did not apply — a false green");
    let found = unreproducible(&root, "s0.md", &changed);
    let problems = backlog_problems(&found.iter().collect::<Vec<_>>(), &entries);
    assert_eq!(problems.len(), 2, "{problems:?}");
    assert!(problems[0].contains("new transcript") && problems[1].contains("no longer there"));
}

#[test]
fn arm_3_a_new_unreproducible_block_is_refused() {
    let (root, text, entries) = s0_chapter();
    let grown = format!(
        "{text}\n```console\nerror[read-malformed-number]: `9x` is not a number\n  --> nowhere.eadl:1:1\n```\n"
    );
    let found = unreproducible(&root, "s0.md", &grown);
    let problems = backlog_problems(&found.iter().collect::<Vec<_>>(), &entries);
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert!(problems[0].contains("nowhere.eadl:1:1"), "{}", problems[0]);
}

#[test]
fn arm_4_a_fixed_block_takes_its_entry_with_it() {
    let (root, text, entries) = s0_chapter();
    let fixed = text.replacen(MALFORMED, "a block with no diagnostic in it", 1);
    let found = unreproducible(&root, "s0.md", &fixed);
    let problems = backlog_problems(&found.iter().collect::<Vec<_>>(), &entries);
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert!(problems[0].contains("no longer there"), "{}", problems[0]);
}

#[test]
fn arm_5_a_second_copy_of_a_backlog_block_is_new() {
    // A set would let a copy through: the key is already listed. The comparison is a multiset.
    let (root, text, entries) = s0_chapter();
    let found = unreproducible(&root, "s0.md", &text);
    let twice: Vec<&Transcript> = vec![&found[0], &found[0]];
    let problems = backlog_problems(&twice, &entries);
    assert_eq!(problems.len(), 1, "{problems:?}");
    assert!(problems[0].contains("new transcript"), "{}", problems[0]);
}
