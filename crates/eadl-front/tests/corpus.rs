//! The reader against the real boundary corpus.
//!
//! Unit tests use inputs chosen to exercise a branch. This suite uses the corpus that was written
//! for a different purpose entirely — before the reader existed — which is what makes it evidence
//! rather than confirmation. Every case in it must read cleanly, round-trip semantically, and
//! yield its metadata block.
//!
//! ⛔ **No case count is written here, deliberately.** This header carried one, and it was still
//! asserting it 49 commits after the corpus grew past it — because a number copied into prose is
//! copied out of the reach of the test that took it, and nothing fails when it goes stale. The
//! count is a property of the corpus, so the corpus is where it is measured:
//! `the_corpus_is_the_size_the_index_claims` takes the census that keeps every assertion below it
//! honest, and `the_live_surfaces_publish_the_measured_corpus_size` requires this header to name
//! that test instead of quoting a count — and requires the book chapter that *does* publish one to
//! publish the measured one.

use std::path::{Path, PathBuf};

use eadl_front::{read, Document, SourceMap};

/// The repository root, derived from this crate's manifest directory.
///
/// Repo-root-relative by construction: the repository can be moved or renamed and this still
/// resolves, which is the project's path policy.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Every corpus case, as `(repo-relative path, text)`.
fn corpus() -> Vec<(String, String)> {
    let root = repo_root();
    let mut cases = Vec::new();
    for verdict in ["accept", "reject"] {
        let dir = root.join("docs/semantics/boundary").join(verdict);
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("readable entry").path();
            if path.extension().is_some_and(|e| e == "eadl") {
                let text = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
                let name = format!(
                    "docs/semantics/boundary/{verdict}/{}",
                    path.file_name().expect("a file name").to_string_lossy()
                );
                cases.push((name, text));
            }
        }
    }
    cases.sort();
    cases
}

fn parse(name: &str, text: &str) -> Document {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text).expect("corpus files are small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{name} did not read cleanly:\n{}",
        diagnostics.render(&sources)
    );
    document
}

/// What the corpus measures about itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CorpusFigures {
    /// Every case.
    total: usize,
    /// Cases under `accept/`.
    accepted: usize,
    /// Cases under `reject/`.
    rejected: usize,
    /// Cases whose own header says `; ambiguous: yes`.
    ambiguous: usize,
}

/// Walk the corpus and measure it.
///
/// ⭐ **One implementation, deliberately.** `the_corpus_is_the_size_the_index_claims` pins the
/// quadruple to absolute numbers so this suite cannot silently iterate over nothing, and
/// `the_live_surfaces_publish_the_measured_corpus_size` compares every surface that *states* one
/// against the same walk. Two ways of measuring would reintroduce exactly the defect that pair
/// exists to end: a figure in prose that nothing re-derives.
///
/// The ambiguous count comes from each case's own `; ambiguous:` header rather than from a list
/// kept beside it, because the header is what F27 and the index are both answerable to.
fn measure_corpus() -> CorpusFigures {
    let cases = corpus();
    let accepted = cases
        .iter()
        .filter(|(name, _)| name.contains("/accept/"))
        .count();
    let ambiguous = cases
        .iter()
        .filter(|(name, text)| {
            parse(name, text)
                .comment_headers()
                .iter()
                .any(|(key, value)| key == "ambiguous" && value == "yes")
        })
        .count();
    CorpusFigures {
        total: cases.len(),
        accepted,
        rejected: cases.len() - accepted,
        ambiguous,
    }
}

#[test]
fn the_corpus_is_the_size_the_index_claims() {
    // A suite that silently iterates over zero files passes forever. This is the census that
    // makes every assertion below mean something.
    //
    // ⚠️ Half a name, and honestly so: this test pins the absolute numbers, while the index it is
    // named after is read and compared by `the_live_surfaces_publish_the_measured_corpus_size`.
    // Before that gate existed the name claimed a comparison nothing performed.
    let figures = measure_corpus();
    assert_eq!(
        figures.total, 23,
        "expected 23 corpus cases, found {}",
        figures.total
    );
    assert_eq!(figures.accepted, 10);
    assert_eq!(figures.rejected, 13);
    assert_eq!(
        figures.ambiguous, 7,
        "the ambiguous cases are the corpus's point — if this moved, the index's summary and its \
         `the … ambiguous cases` sentence both moved with it, and the gate below says so"
    );
}

// ── the corpus figures are measured, never carried ─────────────────────────────────────────────

/// The live surfaces that state a figure about this corpus.
///
/// `include_str!` rather than `fs::read_to_string`, for the reason `profile.rs` gives for the
/// published profile page: if one of these moves or is deleted, this crate **stops compiling**
/// instead of silently gating nothing.
const SUITE_SOURCE: &str = include_str!("corpus.rs");
const BOOK_READING: &str = include_str!("../../../docs/book/src/reading.md");
const CORPUS_INDEX: &str = include_str!("../../../docs/semantics/boundary/README.md");

/// Lines permitted to carry a figure that is **not** the current measurement.
///
/// ⛔ **An explicit list, not a past-tense scan, and that is a measured correction.** The first
/// version of this gate excused a non-current figure when its own line contained `was`, `before`,
/// `previously` or `used to` — the idiom `the_live_surfaces_publish_the_measured_reach` uses. It
/// thereby excused the very line it existed to catch:
///
/// ```text
/// new fields require a worked classification case before adoption", and the five ambiguous cases
/// ```
///
/// The `before` in that line belongs to a quotation of `ROADMAP.md` §4.3, seven words ahead of the
/// stale figure, and the gate reported green on the defect it was written for. A historical figure
/// is therefore something an author lists here — in review, where the addition is visible — rather
/// than something unrelated prose on the same line can satisfy by accident. Empty today: every
/// figure these surfaces carry is a current one.
const HISTORICAL_FIGURES: &[&str] = &[];

/// Number words, so a figure spelled out is still a figure. `the five ambiguous cases` drifted
/// for 49 commits and a digits-only census could not see it.
const NUMBER_WORDS: &[(&str, usize)] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("ten", 10),
    ("eleven", 11),
    ("twelve", 12),
    ("thirteen", 13),
];

/// The words a line is made of: lowercased, with markdown emphasis and punctuation dropped, so
/// `**10 accepted**,` reads as `10 accepted` and a figure cannot hide behind a bold marker.
fn words(line: &str) -> Vec<String> {
    let spaced: String = line
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    spaced.split_whitespace().map(str::to_string).collect()
}

/// A number, written as digits or spelled out.
fn number(word: &str) -> Option<usize> {
    if let Ok(value) = word.parse::<usize>() {
        return Some(value);
    }
    NUMBER_WORDS
        .iter()
        .find(|(spelled, _)| *spelled == word)
        .map(|(_, value)| *value)
}

/// The value a line states for `noun`: a number within `window` words before it.
///
/// ⛔ **The window is the precision.** `7 are ambiguous` needs two; `the five ambiguous cases`
/// needs one. Widening it to three across a whole chapter reads `§4.3 requires that "ambiguous
/// new fields …"` as stating a figure of 3, and a gate that fires on a section number is a gate
/// an author routes around — the false positive `BOOK-ANCHORS` was written to avoid.
fn stated(line: &[String], noun: &str, window: usize) -> Option<usize> {
    line.iter().enumerate().find_map(|(at, word)| {
        (word == noun).then(|| {
            line[at.saturating_sub(window)..at]
                .iter()
                .rev()
                .find_map(|before| number(before))
        })?
    })
}

/// This suite's own module header — the surface leg 1 reads.
fn suite_header() -> String {
    SUITE_SOURCE
        .lines()
        .take_while(|line| line.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every way the three surfaces disagree with `figures`, one message per violation.
///
/// ⭐ **A function rather than a test body, so the RED arms are permanent.** A gate that has only
/// ever been seen green has not been shown to check anything (`TOOLBOX.md`), and mutating the
/// working tree to see it fire is a one-off: it proves the gate fired once, for one author, on one
/// day. Returning the violations lets the arms below feed it the prose that was *actually* wrong
/// and assert the specific complaint — on every run, with nothing mutated.
///
/// ⚠️ HONEST LIMIT. This gates the four figures the corpus produces, on the three surfaces that
/// state them. Correct-but-carried figures with other producers — the grammar's probe count, the
/// S0 chapter's emitted-file count — are out of reach here.
fn figure_violations(
    header: &str,
    index: &str,
    reading: &str,
    figures: CorpusFigures,
) -> Vec<String> {
    let CorpusFigures {
        total,
        accepted,
        rejected,
        ambiguous,
    } = figures;
    let mut wrong = Vec::new();

    // ── 1. The suite's module header carries NO figure, and routes the reader to the census. ──
    // A header that states a count is a header that can go stale; a header that names the test
    // that counts is one that cannot. A *correct* count is refused too, on purpose: the defect is
    // that nothing re-derives it, not that it is wrong today.
    for (at, line) in header.lines().enumerate() {
        let tokens = words(line);
        for noun in ["corpus", "cases", "files", "descriptions"] {
            if let Some(value) = stated(&tokens, noun, 3) {
                wrong.push(format!(
                    "corpus.rs's module header:{} carries a corpus figure (`{value} {noun}`) — \
                     state the rule and name `the_corpus_is_the_size_the_index_claims` instead, \
                     or the figure will outlive the measurement\n  {line}",
                    at + 1
                ));
            }
        }
    }
    if !header.contains("the_corpus_is_the_size_the_index_claims") {
        wrong.push(
            "corpus.rs's module header no longer names the test that measures the corpus, so a \
             reader has nowhere to go to check it"
                .to_string(),
        );
    }

    // ── 2. The index's summary line states the measured quadruple. ──
    match index.lines().find(|line| {
        ["cases", "accepted", "rejected", "ambiguous"]
            .iter()
            .all(|noun| line.contains(noun))
    }) {
        None => wrong.push(
            "docs/semantics/boundary/README.md has no summary line stating cases, accepted, \
             rejected and ambiguous together — the gate that keeps those figures honest cannot \
             find them. Keep one, or move this gate to wherever they went."
                .to_string(),
        ),
        Some(summary) => {
            let tokens = words(summary);
            for (noun, measured) in [
                ("cases", total),
                ("accepted", accepted),
                ("rejected", rejected),
                ("ambiguous", ambiguous),
            ] {
                if stated(&tokens, noun, 3) != Some(measured) {
                    wrong.push(format!(
                        "the corpus index's summary line does not state the measured `{noun}` \
                         count ({measured}) — it reads:\n  {summary}"
                    ));
                }
            }

            // ── 3. … and every OTHER line stating an ambiguous total agrees with it. ──
            // The self-contradiction leg: the index said seven here and five further down.
            // Window 1, so a line quoting `§4.3 … "ambiguous new fields"` states no figure at all.
            for (at, line) in index.lines().enumerate() {
                if line == summary {
                    continue;
                }
                let Some(value) = stated(&words(line), "ambiguous", 1) else {
                    continue;
                };
                if value == ambiguous || HISTORICAL_FIGURES.contains(&line) {
                    continue;
                }
                wrong.push(format!(
                    "docs/semantics/boundary/README.md:{} states `{value}` ambiguous cases, but \
                     the corpus measures {ambiguous} — and the same file's summary line says \
                     {ambiguous}, so the index contradicts itself. Update the figure, or list \
                     the line in `HISTORICAL_FIGURES` if it deliberately describes an older \
                     measurement.\n  {line}",
                    at + 1
                ));
            }
        }
    }

    // ── 4. The book chapter publishes the measured size, and only the measured size. ──
    let mut published = false;
    for (at, line) in reading.lines().enumerate() {
        let Some(value) = stated(&words(line), "corpus", 1) else {
            continue;
        };
        if value == total {
            published = true;
            continue;
        }
        if HISTORICAL_FIGURES.contains(&line) {
            continue;
        }
        wrong.push(format!(
            "docs/book/src/reading.md:{} states a corpus of `{value}`, but the corpus measures \
             {total}. The round trip is the §12 M1 exit gate's evidence, so the chapter that \
             cites it must cite how much of the corpus it ran over. Update the figure, or list \
             the line in `HISTORICAL_FIGURES` if it deliberately describes an older \
             measurement.\n  {line}",
            at + 1
        ));
    }
    if !published {
        wrong.push(format!(
            "docs/book/src/reading.md does not publish the measured corpus size ({total}) \
             anywhere — the round-trip claim is the §12 M1 exit gate's evidence, so the chapter \
             that states it must state how much of the corpus it ran over"
        ));
    }

    wrong
}

#[test]
fn the_live_surfaces_publish_the_measured_corpus_size() {
    // ⛔ THE DEFECT THIS GATES. Three live surfaces published a figure about this corpus for
    // **49 commits** after `M1.7` (`9030111`) moved it: this file's module header and
    // `docs/book/src/reading.md` both still said the corpus held 21 cases, and the corpus index
    // still said "the five ambiguous cases" on one line while its own summary line said seven.
    // Nothing failed, because a number copied into prose is copied out of the reach of the test
    // that took it — and the book, which is the director's only view of the project, thereby
    // under-reported the evidence base of the §12 M1 exit gate.
    //
    // So the prose is compared against the measurement instead of being trusted. Retyping the
    // fresh number and changing nothing else is explicitly not the fix:
    // `docs/CLAIM_VERIFICATION.md` §5B calls that "correcting a stale constant to a fresh
    // constant".
    let wrong = figure_violations(
        &suite_header(),
        CORPUS_INDEX,
        BOOK_READING,
        measure_corpus(),
    );
    assert!(
        wrong.is_empty(),
        "{} way(s) in which a live surface publishes a corpus figure the corpus does not \
         measure:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

// ── RED arms ──────────────────────────────────────────────────────────────────────────────────
//
// Each arm feeds `figure_violations` the prose that was actually wrong, or a minimal mutation of
// the prose that is right, and asserts the specific complaint. Every mutation is confirmed applied
// before the verdict is read: `M1.23`'s arm C reported a pass because its `perl` substitution never
// matched, and a mutation that did not apply is a false green.

/// The three surfaces as they stand, for an arm that mutates one of them.
fn surfaces() -> (String, String, String) {
    (
        suite_header(),
        CORPUS_INDEX.to_string(),
        BOOK_READING.to_string(),
    )
}

/// Violations against the live measurement, for an arm's mutated surfaces.
fn violations_with(header: &str, index: &str, reading: &str) -> Vec<String> {
    figure_violations(header, index, reading, measure_corpus())
}

/// Assert the gate reported exactly `expected` violations, with every `needle` among them.
///
/// ⛔ **The count is part of the arm, not decoration.** An arm that only checks "some complaint
/// mentions X" also passes on a gate that started reporting *everything* — the vacuous-pass shape
/// `the_recognizer_is_not_vacuously_permissive` exists to refuse in the grammar suite. Pinning the
/// count makes an over-reporting gate fail its own arms.
fn assert_reported(wrong: &[String], expected: usize, needles: &[&str]) {
    assert_eq!(
        wrong.len(),
        expected,
        "expected {expected} violation(s); the gate reported:\n{}",
        wrong.join("\n\n")
    );
    for needle in needles {
        assert!(
            wrong.iter().any(|v| v.contains(needle)),
            "no violation mentions {needle:?}; the gate reported:\n{}",
            wrong.join("\n\n")
        );
    }
}

#[test]
fn arm_1_a_stale_corpus_size_in_the_book_is_reported() {
    let (header, index, reading) = surfaces();
    // The original defect verbatim: `reading.md` said 21 for 49 commits after the corpus grew.
    let stale = reading.replace("all 23 corpus files", "all 21 corpus files");
    assert_ne!(stale, reading, "the mutation did not apply — a false green");
    let wrong = violations_with(&header, &index, &stale);
    // Two, and both honest: the chapter states a size that is not the measurement, and so it no
    // longer publishes the measurement either.
    assert_reported(
        &wrong,
        2,
        &[
            "reading.md",
            "states a corpus of `21`",
            "does not publish the measured corpus size",
        ],
    );
}

#[test]
fn arm_2_a_stale_ambiguous_count_is_reported_though_its_line_says_before() {
    let (header, index, reading) = surfaces();
    // ⭐ THE FALSE GREEN THIS ARM EXISTS FOR. The defective line reads:
    //   …worked classification case before adoption", and the five ambiguous cases…
    // The first version of this gate excused a non-current figure whose line carried a past-tense
    // marker, and the `before` in that line belongs to a quotation of §4.3 — seven words ahead of
    // the stale figure. It reported green on the defect it was written for, measured on its first
    // run. An explicit `HISTORICAL_FIGURES` list cannot be satisfied by unrelated prose.
    let stale = index.replace("the seven ambiguous cases", "the five ambiguous cases");
    assert_ne!(stale, index, "the mutation did not apply — a false green");
    let wrong = violations_with(&header, &stale, &reading);
    assert_reported(&wrong, 1, &["README.md", "states `5` ambiguous cases"]);
}

#[test]
fn arm_3_a_corpus_figure_put_back_into_the_module_header_is_reported() {
    let (header, index, reading) = surfaces();
    let stale = format!("{header}\n//! This suite uses the 23 files that were written first.\n");
    assert!(
        stale.contains("23 files"),
        "the mutation did not apply — a false green"
    );
    let wrong = violations_with(&stale, &index, &reading);
    // A *correct* count in the header is refused too: the defect is that nothing re-derives it.
    assert_reported(&wrong, 1, &["module header", "`23 files`"]);
}

#[test]
fn arm_4_a_header_that_stops_naming_the_measuring_test_is_reported() {
    let (header, index, reading) = surfaces();
    let renamed = header.replace(
        "the_corpus_is_the_size_the_index_claims",
        "the_corpus_census",
    );
    assert_ne!(
        renamed, header,
        "the mutation did not apply — a false green"
    );
    let wrong = violations_with(&renamed, &index, &reading);
    assert_reported(&wrong, 1, &["no longer names the test"]);
}

#[test]
fn arm_5_an_index_summary_that_disagrees_with_the_corpus_is_reported() {
    let (header, index, reading) = surfaces();
    // The corpus-grows regression in the other direction: the summary retyped, the corpus not.
    let stale = index.replace("23 cases:", "24 cases:");
    assert_ne!(stale, index, "the mutation did not apply — a false green");
    let wrong = violations_with(&header, &stale, &reading);
    assert_reported(&wrong, 1, &["summary line", "`cases` count (23)"]);
}

#[test]
fn arm_6_a_chapter_that_publishes_no_corpus_size_is_reported() {
    let (header, index, reading) = surfaces();
    // Dropping the figure must not be a way to pass: an unquantified claim is the one a reader
    // cannot check at all.
    let vague = reading.replace("on all 23 corpus files", "on the corpus");
    assert_ne!(vague, reading, "the mutation did not apply — a false green");
    let wrong = violations_with(&header, &index, &vague);
    assert_reported(&wrong, 1, &["does not publish the measured corpus size"]);
}

#[test]
fn arm_7_an_index_with_no_summary_line_is_reported_rather_than_skipped() {
    let (header, _, reading) = surfaces();
    let wrong = violations_with(
        &header,
        "# The boundary corpus\n\nCases are listed below, with no counts.\n",
        &reading,
    );
    assert_reported(&wrong, 1, &["has no summary line"]);
}

#[test]
fn every_corpus_case_reads_without_a_single_diagnostic() {
    for (name, text) in corpus() {
        let document = parse(&name, &text);
        assert!(!document.forms.is_empty(), "{name} parsed to no forms");
        assert_eq!(
            document.forms.len(),
            1,
            "{name} should hold exactly one declaration"
        );
    }
}

#[test]
fn every_corpus_case_round_trips_semantically() {
    // ROADMAP.md §12 M1's exit gate, measured on the real corpus rather than on a toy.
    for (name, text) in corpus() {
        let first = parse(&name, &text);
        let canonical = first.to_canonical();
        let second = parse(&format!("{name} (canonical)"), &canonical);
        assert!(
            first.structurally_eq(&second),
            "{name} changed structure through canonical form:\n--- first ---\n{canonical}\n--- second ---\n{}",
            second.to_canonical()
        );
        assert_eq!(
            canonical,
            second.to_canonical(),
            "{name} is not a fixed point of canonicalization"
        );
    }
}

#[test]
fn every_corpus_case_yields_its_metadata_block() {
    // The corpus carries its verdict in `;` comments, which is why the reader keeps them. F27
    // reads exactly this.
    for (name, text) in corpus() {
        let document = parse(&name, &text);
        let headers = document.comment_headers();
        let get = |key: &str| {
            headers
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.as_str())
        };

        assert_eq!(
            get("case"),
            Some(
                name.rsplit('/')
                    .next()
                    .and_then(|f| f.strip_suffix(".eadl"))
                    .expect("a case file name")
            ),
            "{name}: the `case` header must match the file name"
        );

        let verdict = get("verdict").unwrap_or_else(|| panic!("{name} has no `verdict` header"));
        let expected = if name.contains("/accept/") {
            "accept"
        } else {
            "reject"
        };
        assert_eq!(
            verdict, expected,
            "{name}: verdict disagrees with its directory"
        );

        assert!(
            matches!(get("ambiguous"), Some("yes" | "no")),
            "{name}: `ambiguous` must be yes or no, got {:?}",
            get("ambiguous")
        );

        let tests = get("tests").unwrap_or_else(|| panic!("{name} has no `tests` header"));
        for test in [
            "externality",
            "implementation-independence",
            "non-prescription",
        ] {
            assert!(
                tests.contains(test),
                "{name}: the `tests` header omits `{test}`: {tests}"
            );
        }

        let rationale = get("rationale").unwrap_or_else(|| panic!("{name} has no `rationale`"));
        assert!(
            rationale.len() > 40,
            "{name}: the rationale is too short to be an argument: {rationale}"
        );

        if expected == "reject" {
            let failing =
                get("failing-test").unwrap_or_else(|| panic!("{name} has no `failing-test`"));
            assert!(
                tests.contains(&format!("{failing}=fail")),
                "{name}: `failing-test: {failing}` disagrees with `tests: {tests}`"
            );
        }

        // ⭐ The exact key set, for EVERY case — not just the presence of the keys we wanted.
        // The weaker version of this test passed while the `counter-width-and-rate` rationale
        // was being truncated: it wrapped onto a line beginning `implementation-independence:`,
        // which opened a spurious seventh header. Presence checks cannot see an extra key; an
        // exact set can.
        let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
        let expected_keys: Vec<&str> = if expected == "reject" {
            vec![
                "case",
                "verdict",
                "ambiguous",
                "tests",
                "failing-test",
                "rationale",
                "other-side",
            ]
        } else {
            vec![
                "case",
                "verdict",
                "ambiguous",
                "tests",
                "rationale",
                "other-side",
            ]
        };
        assert_eq!(
            keys, expected_keys,
            "{name}: unexpected header set — a wrapped line probably opened a header"
        );
    }
}

#[test]
fn a_rationale_that_wraps_onto_a_key_shaped_line_stays_one_value() {
    // The regression. `counter-width-and-rate` wraps onto a line that begins
    // `implementation-independence: a different timer …` — text that is exactly a bare
    // kebab-case key followed by a colon. Only the indentation distinguishes it.
    let (name, text) = corpus()
        .into_iter()
        .find(|(name, _)| name.ends_with("counter-width-and-rate.eadl"))
        .expect("the counter-width-and-rate case exists");
    let document = parse(&name, &text);
    let headers = document.comment_headers();
    let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
    assert!(
        !keys.contains(&"implementation-independence"),
        "a wrapped line opened a spurious header: {keys:?}"
    );
    let rationale = headers
        .iter()
        .find(|(k, _)| k == "rationale")
        .map(|(_, v)| v.as_str())
        .expect("a rationale");
    assert!(
        rationale.contains("The test that settles it is implementation-independence:"),
        "the rationale was truncated at the wrap: {rationale}"
    );
    assert!(
        rationale.contains("describes the interface, not the part"),
        "the rationale lost its final sentence: {rationale}"
    );
}

#[test]
fn a_wrapped_rationale_is_one_value_not_several() {
    // The corpus wraps long rationales across many comment lines, several of which contain a
    // colon. Every one of them must land in the value, not open a new header.
    let (name, text) = corpus()
        .into_iter()
        .find(|(name, _)| name.ends_with("execution-bound.eadl"))
        .expect("the execution-bound case exists");
    let document = parse(&name, &text);
    let headers = document.comment_headers();
    let rationale = headers
        .iter()
        .find(|(k, _)| k == "rationale")
        .map(|(_, v)| v.as_str())
        .expect("a rationale");
    assert!(
        rationale.contains("AMBIGUOUS") && rationale.contains("observation about one"),
        "the rationale was truncated at a wrap: {rationale}"
    );
    let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        keys,
        vec![
            "case",
            "verdict",
            "ambiguous",
            "tests",
            "failing-test",
            "rationale",
            "other-side"
        ],
        "unexpected header set — a wrapped line opened a header"
    );
}
