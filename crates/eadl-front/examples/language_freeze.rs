//! `language_freeze` — emit every construct `eadl/1` freezes, as text, for hashing.
//!
//! The baseline `docs/semantics/BASELINE.txt` holds one digest per frozen construct, and
//! `scripts/language_baseline.sh` writes it by piping this instrument into `sha256`. The split is not
//! stylistic: the workspace carries **zero dependencies**
//! (`docs/decisions/decision_zero-dependency-engine-core.md`), so there is no hasher to call from Rust,
//! and the portable `sha256sum`/`shasum` helper `scripts/check_frozen_evaluation.sh` already uses is the
//! one this repository has. Rust owns the part only Rust can do — printing a description's **canonical
//! form** — and the shell owns the part it already does portably.
//!
//! # What is frozen, and why the list is enumerated rather than written down
//!
//! * `grammar.md#ebnf` — the EBNF fence of `docs/semantics/grammar.md`, which is the normative surface
//!   syntax. The prose around it is not frozen: editing a rationale is not a language change.
//! * `reference.md#<table>` — **every** machine-read table `docs/semantics/reference.md` carries, found
//!   by scanning for its markers. ⛔ Enumerated at run time on purpose. `M1.13`'s decomposition censused
//!   "five machine-read tables" and the reference now carries **six**, because `M1.13.3` added
//!   `language-version`; a list written into this file would be the same stale figure with a compiler
//!   behind it.
//! * `suite/<path>` — the **canonical form** of every description in the population
//!   `docs/semantics/conformance.md` declares, which is the same artifact §12 M4 hashes. Not the file's
//!   bytes: a corpus file's comment header carries its `case:`, `why:` and `rationale:` prose, and
//!   editing a rationale is not a language change, while canonical form drops comments — so the digest
//!   moves exactly when the described system moves.
//!
//! ⚠️ **Honest limit, stated rather than discovered later.** Canonical form retains no comment, so a
//! change to a corpus file's *header* is invisible to this baseline. That is deliberate — headers are
//! prose about a case, and the ones that carry data are pinned elsewhere, by `corpus.rs` against
//! `docs/semantics/boundary/README.md`'s counts and by `reference.rs` against the reference's
//! `comment-headers` table — but it means the baseline proves the described systems did not move, not
//! that the files did not.
//!
//! ```console
//! $ cargo run -q -p eadl-front --example language_freeze -- --list
//! $ scripts/language_baseline.sh --print
//! ```
//!
//! Exit codes: `0` emitted · `2` usage, I/O or a construct that could not be read.
//!
//! ⛔ A closed pipe is not a failure: `language_freeze … | head -3` closes stdout early and Rust's
//! `println!` panics on the resulting `BrokenPipe`, printing a backtrace where three lines were
//! expected. Every write goes through [`emit`], the convention `examples/diagnose.rs` set after being
//! piped while in use.

use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use eadl_front::{read, SourceMap};

// ⭐ One reader for the suite's population, shared with the legs. Cargo gives an example no other route
// into `tests/common/`, so this is the seam and `#[path]` is how it is spelled: the alternative is a
// second manifest reader, and two readers for one manifest are two things that can disagree about what
// the suite *is* — the drift `docs/semantics/conformance.md` exists to end.
#[path = "../tests/common/mod.rs"]
mod common;

use common::suite::{population, MANIFEST_PATH};

/// The documents whose machine-read tables are frozen constructs.
///
/// ⛔ Not `docs/semantics/conformance.md`: the manifest declares the *population*, and changing it
/// changes which `suite/…` constructs exist, so the baseline moves anyway. Freezing the manifest's own
/// tables as well would make adding a corpus root a language change requiring a migration note, which is
/// a decision for `M1.13.5`'s director and not something to slip in here.
///
/// ⛔ Not `docs/semantics/model.md` either, and this is `F-I`'s decision, recorded here because this is where
/// the next reader looks (leaf `M1.26.1`). The reference's codes are the **language's** refusals: what a
/// description *is* and what its literals are *worth*, which is exactly what `eadl/1` promises not to move.
/// The model document's codes are the **engine's** verdicts on a description that reads. They grow with every
/// profile and knowledge base `ROADMAP.md` §12 admits later. Freezing them at `eadl/1` would make adding a
/// profile a language migration. Their tables are still executed, both directions, by
/// `crates/eadl-front/tests/reference.rs`: not frozen does not mean unchecked.
const TABLE_DOCUMENTS: &[&str] = &["docs/semantics/reference.md"];

/// The document whose EBNF fence is frozen.
const GRAMMAR: &str = "docs/semantics/grammar.md";

/// The record separator: a byte canonical text cannot contain, because §3 rule 3 of the reference
/// forbids a control character in canonical form and `M1.13.1` made that total.
const NUL: u8 = 0;

fn main() -> ExitCode {
    match run() {
        Ok(code) | Err(code) => code,
    }
}

/// Write one line, treating a closed pipe as a normal end of output.
fn emit(out: &mut dyn Write, line: &str) -> Result<(), ExitCode> {
    match writeln!(out, "{line}") {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Err(ExitCode::SUCCESS),
        Err(error) => Err(fail(&format!("cannot write output: {error}"))),
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("language_freeze: {message}");
    ExitCode::from(2)
}

fn run() -> Result<ExitCode, ExitCode> {
    let mut stdout = std::io::stdout().lock();
    let args: Vec<String> = std::env::args().skip(1).collect();
    // `--diff` is the gate's classifier: `scripts/check_language_freeze.sh` hashes a fresh baseline in
    // shell and asks this to say *what kind* of movement each difference is, because a moved digest, an
    // added construct and a removed one are three different claims and a migration note has to answer to
    // the right one. One comparator, in one language, used by both the gate and its arms.
    if let [flag, fresh, tracked] = args.as_slice() {
        if flag == "--diff" {
            return diff(fresh, tracked);
        }
    }
    let list = match args.as_slice() {
        [] => false,
        [flag] if flag == "--list" => true,
        [flag] if flag == "--texts" => false,
        _ => {
            eprintln!("usage: language_freeze [--list | --texts | --diff <fresh> <tracked>]");
            eprintln!("  --diff   classify how two baseline files differ; exit 1 if they do");
            eprintln!("  --list   the construct ids, one per line");
            eprintln!(
                "  --texts  NUL-framed `<id>\\0<text>\\0` records, for hashing (the default)"
            );
            return Ok(ExitCode::from(2));
        }
    };

    let root = repo_root();
    let constructs = match constructs(&root) {
        Ok(constructs) => constructs,
        Err(problems) => {
            for problem in &problems {
                eprintln!("language_freeze: {problem}");
            }
            return Ok(ExitCode::from(2));
        }
    };

    if list {
        for (id, _) in &constructs {
            emit(&mut stdout, id)?;
        }
        return Ok(ExitCode::SUCCESS);
    }

    // NUL-framed rather than line-framed, because a construct's text contains newlines and the shell
    // side has to recover the exact bytes: `read -r -d ''` keeps every one of them, and a framing byte
    // that could appear inside a construct would silently merge two of them.
    for (id, text) in &constructs {
        stdout
            .write_all(id.as_bytes())
            .and_then(|()| stdout.write_all(&[NUL]))
            .and_then(|()| stdout.write_all(text.as_bytes()))
            .and_then(|()| stdout.write_all(&[NUL]))
            .map_err(|error| {
                if error.kind() == ErrorKind::BrokenPipe {
                    ExitCode::SUCCESS
                } else {
                    fail(&format!("cannot write output: {error}"))
                }
            })?;
    }
    stdout.flush().map_err(|error| fail(&error.to_string()))?;
    Ok(ExitCode::SUCCESS)
}

/// The repository root, derived from this crate's manifest directory.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Every frozen construct, as `(id, text)`, sorted by id.
///
/// # Errors
///
/// Every problem found: an unreadable document, a grammar with no EBNF fence, a suite that cannot be
/// enumerated, or a description that does not read. A construct that cannot be read is **not** skipped —
/// a baseline silently missing a construct is a freeze that does not cover it.
pub fn constructs(root: &Path) -> Result<Vec<(String, String)>, Vec<String>> {
    let mut problems = Vec::new();
    let mut out: Vec<(String, String)> = Vec::new();

    // ── the EBNF fence ────────────────────────────────────────────────────────────────────────
    match read_document(root, GRAMMAR) {
        Ok(text) => match ebnf_fence(&text) {
            Some(fence) => out.push((format!("{GRAMMAR}#ebnf"), fence)),
            None => problems.push(format!(
                "{GRAMMAR} carries no ```ebnf fence, so the normative surface syntax has nothing to \
                 digest — that is a defect in the document, not a construct to skip"
            )),
        },
        Err(problem) => problems.push(problem),
    }

    // ── every machine-read table, enumerated from the documents rather than listed here ────────
    for document in TABLE_DOCUMENTS {
        match read_document(root, document) {
            Ok(text) => {
                let tables = machine_tables(&text);
                if tables.is_empty() {
                    problems.push(format!(
                        "{document} carries no `<!-- machine-read: … -->` table, so no executed rule of \
                         the language is frozen"
                    ));
                }
                for (marker, rows) in tables {
                    out.push((format!("{document}#{marker}"), rows));
                }
            }
            Err(problem) => problems.push(problem),
        }
    }

    // ── the canonical form of every description in the declared population ────────────────────
    match population(root) {
        Ok(suite) => {
            if suite.is_empty() {
                problems.push(format!(
                    "{MANIFEST_PATH} declares an empty population, so the baseline would freeze no \
                     description at all"
                ));
            }
            for (path, text) in suite {
                let mut sources = SourceMap::new();
                match sources.add(path.clone(), text) {
                    Ok(id) => {
                        let (document, diagnostics) = read(&sources, id);
                        if diagnostics.has_errors() {
                            problems.push(format!(
                                "{path} does not read cleanly, so it has no canonical form to digest:\n{}",
                                diagnostics.render(&sources)
                            ));
                            continue;
                        }
                        out.push((format!("suite/{path}"), document.to_canonical()));
                    }
                    Err(error) => problems.push(format!("cannot register {path}: {error}")),
                }
            }
        }
        Err(more) => problems.extend(more),
    }

    if !problems.is_empty() {
        return Err(problems);
    }
    out.sort_by(|left, right| left.0.cmp(&right.0));
    let ids: Vec<&str> = out.iter().map(|(id, _)| id.as_str()).collect();
    let mut deduplicated = ids.clone();
    deduplicated.sort_unstable();
    deduplicated.dedup();
    if deduplicated.len() != ids.len() {
        return Err(vec![format!(
            "two frozen constructs produced the same id, so one digest would stand for both: {} id(s), \
             {} distinct",
            ids.len(),
            deduplicated.len()
        )]);
    }
    Ok(out)
}

/// Classify how the baseline at `fresh_path` differs from the one at `tracked_path`.
///
/// Exit `0` when they agree, `1` when they differ — printing one line per difference, classified — and
/// `2` when either file is not a baseline at all, which is a defect in the file and not a movement.
fn diff(fresh_path: &str, tracked_path: &str) -> Result<ExitCode, ExitCode> {
    let fresh = parse_baseline(fresh_path)?;
    let tracked = parse_baseline(tracked_path)?;
    let differences = common::baseline::compare(&fresh, &tracked);
    if differences.is_empty() {
        return Ok(ExitCode::SUCCESS);
    }
    let mut stdout = std::io::stdout().lock();
    for difference in &differences {
        emit(
            &mut stdout,
            &format!("{}\t{}", difference.kind(), difference.render()),
        )?;
    }
    let (mut moved, mut added, mut removed) = (0, 0, 0);
    for difference in &differences {
        match difference.kind() {
            "moved" => moved += 1,
            "added" => added += 1,
            _ => removed += 1,
        }
    }
    emit(
        &mut stdout,
        &format!(
            "{moved} moved, {added} added, {removed} removed — a moved digest is a construct that \
             changed, an added one is a language that grew, and a removed one is a construct nothing \
             freezes any more"
        ),
    )?;
    Ok(ExitCode::from(1))
}

/// Read one baseline file, reporting a malformed one as a defect rather than as a movement.
fn parse_baseline(path: &str) -> Result<common::baseline::Baseline, ExitCode> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => return Err(fail(&format!("cannot read {path}: {error}"))),
    };
    match common::baseline::parse_named(path, &text) {
        Ok(baseline) => Ok(baseline),
        Err(problems) => {
            for problem in &problems {
                eprintln!("language_freeze: {problem}");
            }
            Err(ExitCode::from(2))
        }
    }
}

fn read_document(root: &Path, relative: &str) -> Result<String, String> {
    std::fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

/// The contents of a document's ```ebnf fence, fences excluded.
fn ebnf_fence(document: &str) -> Option<String> {
    let lines: Vec<&str> = document.lines().collect();
    let start = lines.iter().position(|line| line.trim() == "```ebnf")?;
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.trim() == "```")?;
    Some(lines[start + 1..start + 1 + end].join("\n"))
}

/// Every machine-read table in a document, as `(marker, the table's own lines)`.
///
/// ⭐ Enumerated by scanning for markers, so a table added to the reference is frozen by the next run
/// and nothing has to be told. The digest is of the **raw lines**, marker excluded: the table as the
/// document states it, so re-wording a cell moves the digest and re-flowing the prose around it does not.
fn machine_tables(document: &str) -> Vec<(String, String)> {
    let lines: Vec<&str> = document.lines().collect();
    let mut out = Vec::new();
    let mut at = 0;
    while at < lines.len() {
        let Some(marker) = marker_on(lines[at]) else {
            at += 1;
            continue;
        };
        let mut end = at + 1;
        let mut saw_row = false;
        while end < lines.len() {
            let trimmed = lines[end].trim();
            if trimmed.starts_with('|') {
                saw_row = true;
                end += 1;
            } else if trimmed.is_empty() && !saw_row {
                // A blank line between the marker and its table is layout; anything else ends it.
                end += 1;
            } else {
                break;
            }
        }
        if saw_row {
            out.push((marker, lines[at + 1..end].join("\n")));
        }
        at = end.max(at + 1);
    }
    out
}

/// The marker a line opens, when the line *is* a machine-read marker.
///
/// ⛔ Matched at the start of the line and not merely contained in it, which is what keeps the
/// reference's own prose mention of the convention — "a table introduced by a
/// `<!-- machine-read: … -->` comment is **executed**" — out of the frozen population. A marker inside a
/// sentence is a description of the notation, not a table.
fn marker_on(line: &str) -> Option<String> {
    line.strip_prefix("<!-- machine-read: ")
        .and_then(|rest| rest.strip_suffix(" -->"))
        .map(str::to_string)
}
