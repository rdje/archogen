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
//!   cannot be checked. Those are a **backlog**, listed here, which may shrink and may not grow.

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

/// The transcripts that name no reproducible input, as `chapter:line` — measured `2026-09-30`. Fixing one means
/// deleting its entry; adding one is refused.
const BACKLOG: &[&str] = &[
    "boundary.md:84",
    "presence.md:35",
    "presence.md:52",
    "presence.md:63",
    "reading.md:87",
    "reading.md:102",
    "refinement.md:31",
    "refinement.md:62",
    "s0.md:214",
    "workload.md:67",
    "workload.md:89",
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
    command: Option<(String, bool)>,
    location: Option<String>,
    body: Vec<String>,
}

/// Every transcript in the book's chapters.
fn transcripts(root: &Path) -> Vec<Transcript> {
    let mut found = Vec::new();
    let mut chapters: Vec<PathBuf> = fs::read_dir(root.join("docs/book/src"))
        .expect("the book's sources")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "md"))
        .collect();
    chapters.sort();
    for chapter in chapters {
        let name = chapter
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .to_string();
        let lines: Vec<String> = fs::read_to_string(&chapter)
            .expect("readable")
            .lines()
            .map(str::to_string)
            .collect();
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
            if block.iter().any(|l| l.starts_with("error[")) {
                let command = block.iter().find_map(|l| {
                    let rest = l.strip_prefix("$ archogen check ")?;
                    let echoes = rest.contains("; echo $?");
                    let file = rest.split_whitespace().next()?.to_string();
                    Some((file, echoes))
                });
                let location = block.iter().find_map(|l| {
                    let rest = l.trim_start().strip_prefix("--> ")?;
                    rest.split(':').next().map(str::to_string)
                });
                let body = block
                    .iter()
                    .filter(|l| !l.starts_with("$ "))
                    .cloned()
                    .collect();
                found.push(Transcript {
                    at: format!("{name}:{}", i + 1),
                    command,
                    location,
                    body,
                });
            }
            i = j + 1;
        }
    }
    found
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
    let mut backlog = Vec::new();
    let mut checked = 0;
    for t in &all {
        let input = t
            .command
            .as_ref()
            .map(|(f, _)| f.clone())
            .or_else(|| t.location.clone());
        let Some(input) = input.filter(|f| reproducible(&root, f)) else {
            backlog.push(t.at.clone());
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
    let grown: Vec<&String> = backlog
        .iter()
        .filter(|at| !BACKLOG.contains(&at.as_str()))
        .collect();
    if !grown.is_empty() {
        problems.push(format!(
            "new transcript(s) over an input no reader can rebuild: {grown:?} — name a file in the repository"
        ));
    }
    let fixed: Vec<&&str> = BACKLOG
        .iter()
        .filter(|at| !backlog.iter().any(|b| b == **at))
        .collect();
    if !fixed.is_empty() {
        problems.push(format!(
            "backlog entries that are no longer there: {fixed:?} — delete them from BACKLOG"
        ));
    }
    assert!(
        problems.is_empty(),
        "{} of {} checked transcript(s) disagree with the tool:\n\n{}",
        problems.len(),
        checked,
        problems.join("\n\n")
    );
}
