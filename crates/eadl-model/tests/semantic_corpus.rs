//! The semantic corpus — `ROADMAP.md` §12 M1's twenty worked cases, driven end to end.
//!
//! > Adopt at least **twenty semantic examples**, including positive matches, relevant missing
//! > facts, conflicts, and unsupported cases.
//!
//! Each case declares the §5.5 verdict it expects, and the driver runs the whole frontend
//! pipeline against it. A case is evidence only if the expectation is written *in the case* —
//! a driver that computed the expectation would agree with itself forever.
//!
//! The suite also enforces the §5.5 diagnostic contract on every diagnostic the corpus produces:
//! a span, and a concrete repair direction.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use eadl_front::{SourceMap, Verdict};
use eadl_model::check::{check, default_profile, shipped_registry};
use eadl_model::kind::Registry;

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
        let path = repo_root().join(relative);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        ((*relative).to_string(), text)
    })
    .collect()
}

fn registry(sources: &mut SourceMap) -> Registry {
    shipped_registry(sources, &kind_files()).unwrap_or_else(|errors| {
        panic!(
            "the shipped kind modules are malformed: {}",
            errors
                .iter()
                .map(|d| d.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        )
    })
}

struct Case {
    name: String,
    expect: Verdict,
    why: String,
    text: String,
}

fn corpus() -> Vec<Case> {
    let dir = repo_root().join("docs/semantics/cases");
    let mut cases = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("the semantic corpus exists") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|e| e != "eadl") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("readable case");
        let name = path
            .file_stem()
            .expect("a stem")
            .to_string_lossy()
            .to_string();

        let header = |key: &str| -> Option<String> {
            let mut value: Option<String> = None;
            for line in text.lines() {
                let Some(body) = line.strip_prefix("; ") else {
                    if line.starts_with(";   ") {
                        if let Some(acc) = value.as_mut() {
                            acc.push(' ');
                            acc.push_str(line.trim_start_matches(';').trim());
                        }
                    }
                    continue;
                };
                if let Some((k, v)) = body.split_once(": ") {
                    if k == key {
                        value = Some(v.trim().to_string());
                    } else if value.is_some() {
                        break;
                    }
                }
            }
            value
        };

        let expect = header("expect").unwrap_or_else(|| panic!("{name}: no `expect` header"));
        cases.push(Case {
            expect: Verdict::parse(&expect)
                .unwrap_or_else(|| panic!("{name}: `{expect}` is not a §5.5 verdict")),
            why: header("why").unwrap_or_else(|| panic!("{name}: no `why` header")),
            name,
            text,
        });
    }
    cases.sort_by(|a, b| a.name.cmp(&b.name));
    cases
}

#[test]
fn the_corpus_meets_the_roadmap_minimum_and_covers_every_category() {
    // §12 M1: "at least twenty semantic examples, including positive matches, relevant missing
    // facts, conflicts, and unsupported cases." A corpus of twenty rejections would meet the
    // count and cover nothing.
    let cases = corpus();
    assert!(cases.len() >= 20, "only {} cases", cases.len());

    let count = |verdict: Verdict| cases.iter().filter(|c| c.expect == verdict).count();
    assert!(count(Verdict::Ok) >= 3, "too few positive matches");
    assert!(count(Verdict::MissingFact) >= 1, "no missing-fact case");
    assert!(
        count(Verdict::InvalidDescription) >= 3,
        "too few conflict cases"
    );
    assert!(
        count(Verdict::UnsupportedProfile) >= 3,
        "too few unsupported cases"
    );
    assert!(
        count(Verdict::InfeasibleConfiguration) >= 1,
        "no infeasible case"
    );

    for case in &cases {
        assert!(
            case.why.len() > 30,
            "{}: the `why` header does not explain the case: {}",
            case.name,
            case.why
        );
    }
}

#[test]
fn every_case_produces_the_verdict_it_declares() {
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for case in corpus() {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text.clone())
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert_eq!(
            outcome.verdict,
            case.expect,
            "{}: expected `{}`, got `{}`\n{}\n--- diagnostics ---\n{}",
            case.name,
            case.expect.slug(),
            outcome.verdict.slug(),
            case.why,
            outcome.render(&sources)
        );
    }
}

#[test]
fn an_accepted_case_produces_no_diagnostics_at_all() {
    // "Accepted with three warnings" is a shape this pipeline does not have: a case that is ok
    // is silent, so an author never has to judge which messages mattered.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for case in corpus().into_iter().filter(|c| c.expect == Verdict::Ok) {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text)
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert!(
            outcome.diagnostics.is_empty(),
            "{} is accepted but still said something:\n{}",
            case.name,
            outcome.render(&sources)
        );
    }
}

#[test]
fn a_refused_case_always_says_why() {
    // A verdict with no diagnostic is a refusal with no explanation, which is worse than no
    // refusal: the author has nothing to act on.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for case in corpus().into_iter().filter(|c| c.expect != Verdict::Ok) {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text)
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert!(
            !outcome.diagnostics.is_empty(),
            "{} was refused with no diagnostic",
            case.name
        );
    }
}

#[test]
fn every_diagnostic_the_corpus_produces_meets_the_5_5_contract() {
    // ⭐ §5.5: "Each diagnostic carries source spans … and a concrete repair direction."
    // Checked over every diagnostic the whole corpus produces, rather than on the handful a
    // unit test happens to construct.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();
    let mut checked = 0;

    for case in corpus() {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text)
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        for diagnostic in &outcome.diagnostics {
            assert!(
                !diagnostic.code.is_empty(),
                "{}: a diagnostic with no code",
                case.name
            );
            assert!(
                !diagnostic.message.is_empty(),
                "{}: `{}` has no message",
                case.name,
                diagnostic.code
            );
            assert!(
                diagnostic.repair.len() > 15,
                "{}: `{}` has no usable repair direction: {:?}",
                case.name,
                diagnostic.code,
                diagnostic.repair
            );
            // The span must point into a real source, and the rendered form must locate it.
            let rendered = diagnostic.render(&sources);
            assert!(
                rendered.contains("-->") && !rendered.contains("<unknown source>"),
                "{}: `{}` does not locate itself:\n{rendered}",
                case.name,
                diagnostic.code
            );
            assert!(
                rendered.contains("= hint:"),
                "{}: `{}` renders without its repair direction",
                case.name,
                diagnostic.code
            );
            checked += 1;
        }
    }
    assert!(checked >= 20, "only {checked} diagnostics exercised");
}

#[test]
fn an_unsupported_request_names_the_capability_and_the_obligation() {
    // §3.1: refused "rather than silently reducing the requested guarantee". A refusal that
    // does not say what admitting it would cost reads as a wall rather than as work.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    let case = corpus()
        .into_iter()
        .find(|c| c.name == "unsupported-general-ipc")
        .expect("the uc4 case");
    let id = sources.add("uc4", case.text).expect("small");
    let outcome = check(&sources, id, &registry, profile);
    assert_eq!(outcome.verdict, Verdict::UnsupportedProfile);
    let rendered = outcome.render(&sources);
    assert!(
        rendered.contains("`general-ipc` is not admitted"),
        "{rendered}"
    );
    assert!(rendered.contains("rt-static-up-v1"), "{rendered}");
    assert!(
        rendered.contains("admitting it would add"),
        "the refusal must name the obligation:\n{rendered}"
    );
    assert!(rendered.contains("never silently reduced"), "{rendered}");
}

#[test]
fn the_verdict_is_what_to_fix_first_not_what_was_found_first() {
    // ⭐ A description with an out-of-profile request AND an undescribed fact reports
    // `unsupported-profile`: describing the fact would be wasted work on a system the profile
    // refuses anyway. Both diagnostics are still present — precedence chooses the headline, not
    // what the author gets to see.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    let case = corpus()
        .into_iter()
        .find(|c| c.name == "unsupported-precedence-over-missing-fact")
        .expect("the precedence case");
    let id = sources.add("precedence", case.text).expect("small");
    let outcome = check(&sources, id, &registry, profile);
    assert_eq!(outcome.verdict, Verdict::UnsupportedProfile);

    let codes: Vec<&str> = outcome.diagnostics.iter().map(|d| d.code).collect();
    assert!(codes.contains(&"unsupported-profile"), "{codes:?}");
    assert!(
        codes.contains(&"missing-fact"),
        "precedence must not hide the other problem: {codes:?}"
    );
}

#[test]
fn the_examples_directory_agrees_with_the_pipeline() {
    // The three shipped use cases run through the same pipeline as the corpus. uc1 and uc2 are
    // accepted; uc3 is `infeasible-configuration` today, because its platform declares
    // `absolute-deadline` absent and its service requires it — the gap M3.2's indirect
    // realization must close, without the description changing.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for (relative, expected) in [
        ("examples/periodic-three/system.eadl", Verdict::Ok),
        ("examples/high-interference/system.eadl", Verdict::Ok),
        (
            "examples/alternative-timer/system.eadl",
            Verdict::InfeasibleConfiguration,
        ),
        // ⭐ uc3 and uc4 are both refused, for reasons that must not be confused. uc3's refusal
        // is TEMPORARY — the same description must build once M3.2 supplies indirect
        // realization. uc4's is PERMANENT for this profile: admitting it requires a new named
        // profile with its own analysis obligations, not a wider `rt-static-up-v1`.
        (
            "examples/bounded-queue/system.eadl",
            Verdict::UnsupportedProfile,
        ),
    ] {
        let text = std::fs::read_to_string(repo_root().join(relative)).expect("readable");
        let id = sources.add(relative, text).expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert_eq!(
            outcome.verdict,
            expected,
            "{relative}: expected `{}`, got `{}`\n{}",
            expected.slug(),
            outcome.verdict.slug(),
            outcome.render(&sources)
        );
    }
}

// ── the figure the book publishes, compared with the measurement ────────────────────────────────

/// `docs/book/src/checking.md`, the chapter that publishes this corpus's size to the reader.
///
/// `include_str!` for the reason `crates/eadl-front/tests/corpus.rs` gives for its live surfaces: if
/// the chapter moves or is deleted, this crate **stops compiling** instead of silently gating nothing.
const BOOK_CHECKING: &str = include_str!("../../../docs/book/src/checking.md");

/// The corpus size the chapter publishes, or `None` when it publishes none.
///
/// Read out of the sentence rather than listed beside it: the figure is the last number before the
/// phrase "worked cases", so re-wording the sentence around it does not move the extraction, and a
/// chapter that stops quantifying returns `None` — which [`figure_violations`] makes a breach rather
/// than a pass, because an unquantified claim is the one a reader cannot check at all.
fn published_size(chapter: &str) -> Option<usize> {
    for line in chapter.lines() {
        let Some(before) = line.split_once("worked cases").map(|(head, _)| head) else {
            continue;
        };
        let digits = before
            .split(|character: char| !character.is_ascii_digit())
            .rfind(|token| !token.is_empty())?;
        return digits.parse().ok();
    }
    None
}

/// Compare the published figure with the measured size of the corpus.
fn figure_violations(chapter: &str, measured: usize) -> Vec<String> {
    match published_size(chapter) {
        None => vec![
            "docs/book/src/checking.md publishes no size for this corpus, and an unquantified claim is \
             the one a reader cannot check at all — publish the measured count, or name the test that \
             measures it"
                .to_string(),
        ],
        Some(published) if published != measured => vec![format!(
            "docs/book/src/checking.md says this corpus holds {published} worked cases and the walk \
             finds {measured} — the prose is compared against the measurement rather than trusted, \
             because a number copied into a chapter is copied out of the reach of the test that took it"
        )],
        Some(_) => Vec::new(),
    }
}

#[test]
fn the_book_publishes_the_measured_corpus_size() {
    // ⛔ THE DEFECT THIS GATES, and it was live rather than hypothetical. The chapter said **25**
    // worked cases, which was true at `6df022f` (`2026-09-13`) when the directory held 25; `538fe3b`
    // (`M1.9`) added four and did not touch the chapter, so the book under-reported the evidence base of
    // §12 M1's exit gate for **71 commits** — `git rev-list --count 538fe3b..HEAD` at the commit that
    // added this leg. `M1.13.4.3` found it by adding a thirtieth case and looking for surfaces the
    // change moves.
    //
    // Retyping the fresh number is explicitly not the fix, which is why this is a leg and not an edit:
    // `docs/CLAIM_VERIFICATION.md` §5B calls that "correcting a stale constant to a fresh constant", and
    // `crates/eadl-front/tests/corpus.rs` gates the boundary corpus's figures the same way.
    let measured = corpus().len();
    let wrong = figure_violations(BOOK_CHECKING, measured);
    assert!(
        wrong.is_empty(),
        "{} way(s) in which the book publishes a corpus figure the corpus does not measure:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

#[test]
fn arm_1_a_chapter_that_publishes_the_wrong_size_is_reported() {
    // The prose that was actually wrong, replayed as a fixture rather than recalled: the real chapter
    // said 25 while the walk found more, and the complaint must name both numbers.
    let measured = corpus().len();
    let stale = BOOK_CHECKING.replace(
        &format!("holds {measured} worked cases"),
        "holds 25 worked cases",
    );
    assert_ne!(
        stale, BOOK_CHECKING,
        "the mutation did not apply — a false green"
    );
    let wrong = figure_violations(&stale, measured);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n\n"));
    assert!(
        wrong[0].contains("25") && wrong[0].contains(&measured.to_string()),
        "the complaint must name the published figure and the measured one: {}",
        wrong[0]
    );
}

#[test]
fn arm_2_a_chapter_that_publishes_no_size_is_reported_rather_than_skipped() {
    // Dropping the figure must not be a way to pass: an unquantified claim is the one a reader cannot
    // check at all, which is the same reasoning `corpus.rs`'s arm 6 records for the boundary corpus.
    let measured = corpus().len();
    let vague = BOOK_CHECKING.replace(
        &format!("holds {measured} worked cases"),
        "holds worked cases",
    );
    assert_ne!(
        vague, BOOK_CHECKING,
        "the mutation did not apply — a false green"
    );
    let wrong = figure_violations(&vague, measured);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n\n"));
    assert!(
        wrong[0].contains("publishes no size"),
        "the complaint must say what is missing, not only that something is: {}",
        wrong[0]
    );
}

#[test]
fn arm_3_the_extraction_reads_the_figure_and_not_another_number_on_the_line() {
    // ⛔ The arm on the reader itself. The sentence carries a second quantity — "§12 M1 asks for
    // twenty" — spelled out and after the figure, and a chapter number ("## The semantic corpus") above
    // it. An extraction that took the first or the last number on the line would read one of those
    // instead, and the leg would then compare the corpus against a roadmap minimum it can never equal.
    assert_eq!(
        published_size(
            "`docs/semantics/cases/` holds 30 worked cases — §12 M1 asks for twenty — each"
        ),
        Some(30)
    );
    assert_eq!(published_size("no figure on this line"), None);
    assert_eq!(
        published_size("holds 7 worked cases"),
        Some(7),
        "a one-digit size must read as one digit"
    );
}

/// The per-verdict counts a chapter publishes, as `(verdict slug, count)` in the table's own order.
///
/// ⛔ Read strictly, because the chapter carries other tables: a row counts only when it has exactly
/// two cells, the first a single backticked token and the second bare digits. The passes table above it
/// has three cells and its verdicts are in the *last* one, so it cannot be mistaken for a count — and a
/// reader loose enough to match it would compare the corpus against a list of which pass owns what.
fn published_verdict_counts(chapter: &str) -> Vec<(String, usize)> {
    let mut out = Vec::new();
    for line in chapter.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
            continue;
        }
        let cells: Vec<&str> = trimmed
            .trim_start_matches('|')
            .trim_end_matches('|')
            .split('|')
            .map(str::trim)
            .collect();
        let [first, second] = cells.as_slice() else {
            continue;
        };
        let (Some(verdict), Some(count)) = (
            first
                .strip_prefix('`')
                .and_then(|cell| cell.strip_suffix('`')),
            second.parse::<usize>().ok(),
        ) else {
            continue;
        };
        out.push(((*verdict).to_string(), count));
    }
    out
}

/// The verdicts the corpus declares, counted from the cases themselves.
fn measured_verdict_counts() -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    for case in corpus() {
        *counts.entry(case.expect.slug().to_string()).or_insert(0) += 1;
    }
    counts
}

/// Compare the chapter's per-verdict table with the corpus, in both directions.
///
/// ⭐ Both directions, because each alone has a false green: a table checked only against the corpus
/// passes when it omits a verdict entirely, and a table checked only for "every row names a real
/// verdict" passes when its counts are wrong. This is the same two-sided census `reference.rs` runs over
/// §4's diagnostic codes.
fn verdict_table_violations(chapter: &str, measured: &BTreeMap<String, usize>) -> Vec<String> {
    let published = published_verdict_counts(chapter);
    if published.is_empty() {
        return vec![
            "docs/book/src/checking.md publishes no per-verdict table, so the reader cannot tell what \
             the corpus covers and nothing compares the chapter to the walk"
                .to_string(),
        ];
    }
    let mut out = Vec::new();
    for (verdict, count) in &published {
        match measured.get(verdict) {
            None => out.push(format!(
                "docs/book/src/checking.md lists `{verdict}` with {count} case(s) and no case in the \
                 corpus declares it, so the row describes a verdict this corpus does not exercise"
            )),
            Some(want) if want != count => out.push(format!(
                "docs/book/src/checking.md says {count} case(s) expect `{verdict}` and the corpus holds \
                 {want} — the prose is compared against the walk rather than trusted"
            )),
            Some(_) => {}
        }
    }
    for (verdict, count) in measured {
        if !published.iter().any(|(listed, _)| listed == verdict) {
            out.push(format!(
                "the corpus holds {count} case(s) expecting `{verdict}` and \
                 docs/book/src/checking.md's table has no row for it, so a verdict the suite exercises \
                 is invisible to the reader"
            ));
        }
    }
    out
}

#[test]
fn the_book_publishes_the_measured_verdict_counts() {
    // ⛔ THE DEFECT THIS GATES, live rather than hypothetical and older than the size figure beside it.
    // At the commit that added this leg the chapter's table read `ok` 5, `invalid-description` 10,
    // `unsupported-profile` 5, `infeasible-configuration` 4, `missing-fact` 1 — 25 cases — while the
    // walk over `docs/semantics/cases/` found 11 / 7 / 5 / 4 / 2, so **three of the five rows were
    // wrong** and the table's own sum contradicted the sentence above it. `538fe3b` (`M1.9`) added four
    // cases and touched neither figure. Found by `M1.13.4.3` adding a thirtieth case and looking for
    // every surface the change moves.
    let measured = measured_verdict_counts();
    let wrong = verdict_table_violations(BOOK_CHECKING, &measured);
    assert!(
        wrong.is_empty(),
        "{} way(s) in which the book's per-verdict table disagrees with the corpus:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

#[test]
fn arm_4_a_verdict_count_the_corpus_does_not_have_is_reported() {
    // The row that was actually wrong, replayed as a fixture: `invalid-description` published as 10
    // against a measured 11.
    let measured = measured_verdict_counts();
    let want = measured["invalid-description"];
    let stale = BOOK_CHECKING.replace(
        &format!("| `invalid-description` | {want} |"),
        &format!("| `invalid-description` | {} |", want - 1),
    );
    assert_ne!(
        stale, BOOK_CHECKING,
        "the mutation did not apply — a false green"
    );
    let wrong = verdict_table_violations(&stale, &measured);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n\n"));
    assert!(
        wrong[0].contains("invalid-description"),
        "the complaint must name the verdict: {}",
        wrong[0]
    );
}

#[test]
fn arm_5_a_verdict_the_table_omits_is_reported_rather_than_skipped() {
    // The one-sided check's false green: a table missing a row entirely still agrees with every row it
    // has, so the leg has to census the corpus against the table and not only the table against the
    // corpus.
    let measured = measured_verdict_counts();
    let row = "| `missing-fact` | 2 |\n";
    assert!(
        BOOK_CHECKING.contains(row.trim_end()),
        "the fixture row this arm deletes is not in the chapter — the arm would pass on nothing"
    );
    let without = BOOK_CHECKING.replace(row, "");
    assert_ne!(
        without, BOOK_CHECKING,
        "the mutation did not apply — a false green"
    );
    let wrong = verdict_table_violations(&without, &measured);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n\n"));
    assert!(
        wrong[0].contains("no row for it"),
        "the complaint must say the row is missing, not that a count differs: {}",
        wrong[0]
    );
}

#[test]
fn arm_6_the_table_reader_does_not_read_the_chapter_s_other_table() {
    // ⛔ The arm on the reader itself. `checking.md` carries a three-column table of passes whose last
    // column holds verdict names in backticks; a loose reader would take `invalid-description` from it
    // and then report that the corpus has no count for it, or worse, agree with itself forever.
    let chapter = "| Pass | Owns | Verdict on failure |\n\
                   | --- | --- | --- |\n\
                   | read | syntax and spans | `invalid-description` |\n";
    assert!(
        published_verdict_counts(chapter).is_empty(),
        "the passes table was read as a count table: {:?}",
        published_verdict_counts(chapter)
    );
    let counts = published_verdict_counts("| Expected | Cases |\n| --- | --- |\n| `ok` | 6 |\n");
    assert_eq!(counts, vec![("ok".to_string(), 6)]);
}
