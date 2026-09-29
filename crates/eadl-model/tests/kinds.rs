//! The core kinds, loaded from eADL, against the real corpus.
//!
//! Two properties are under test, and the second is the one that makes the first honest:
//!
//! 1. The core kinds **parse and register** from `docs/semantics/kinds/core.eadl` — so the
//!    claim that only `defkind` is a trusted primitive is a fact about the files, not a
//!    statement of intent.
//! 2. Every **accepted** boundary case validates against them, and every **rejected** one is
//!    refused. The corpus was written before the schema existed, for a different purpose, which
//!    is what makes it evidence rather than confirmation.

use std::path::{Path, PathBuf};

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::kind::{read_kind, validate, Cardinality, NameRule, Registry};

/// The two kind modules the toolchain ships.
const KIND_MODULES: &[&str] = &[
    "docs/semantics/kinds/core.eadl",
    "docs/semantics/kinds/os-rt.eadl",
];

/// The declarations of a parsed file: every top-level form except the language-version identifier.
///
/// §8 of `docs/semantics/reference.md` — the identifier is a statement about the document, not a
/// declaration. ⛔ A call through to the frontend's accessor, not a second filter here: every
/// description in the corpus states its version, so a leg that indexed `forms[0]` was reading the
/// *identifier* as the file's first declaration, and a leg that iterated `forms` was asking the schema
/// to validate it. `M1.13.4.2` measured both shapes: `error[schema-unknown-kind]` on every accepted
/// case, `schema-not-a-kind` on the first kind of `core.eadl`, and a reach census that counted the
/// identifier as a declaration the schema could not reach.
fn declarations(forms: &[Form]) -> Vec<&Form> {
    eadl_front::language_version::declarations(forms).collect()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn parse_file(relative: &str) -> (Vec<Form>, SourceMap) {
    let path = repo_root().join(relative);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let mut sources = SourceMap::new();
    let id = sources.add(relative, text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{relative} did not read:\n{}",
        diagnostics.render(&sources)
    );
    (document.forms, sources)
}

/// The registry built from the shipped kind modules.
///
/// `core.eadl` holds the five §5.1 surface kinds; `os-rt.eadl` is the workload feature module
/// §5.1 calls for, and supplies the `task` kind that `defsystem` references. Loading both is
/// what a real invocation does — and a registry missing `os-rt` says so rather than silently
/// accepting anything inside a task, which `a_missing_kind_module_is_reported_not_ignored`
/// checks.
fn core_registry() -> Registry {
    registry_from(KIND_MODULES)
}

/// Read kind modules from disk into the `(name, text)` pairs [`shipped_registry`] takes.
fn kind_files(files: &[&str]) -> Vec<(String, String)> {
    files
        .iter()
        .map(|relative| {
            let path = repo_root().join(relative);
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
            ((*relative).to_string(), text)
        })
        .collect()
}

/// Build a registry from the named kind modules, **through the production loader**.
///
/// ⭐ `shipped_registry` and not a loop over `read_kind`, for the reason
/// `the_language_version_identifier_survives_the_model_layer` gives for running `check` rather than
/// `validate`: a test that bypasses the layer under test measures the wrong thing, confidently. This
/// helper *was* a second loader, and a second loader is a second place for §8's rule to be missing —
/// which is how the production one came to be the third consumer nobody had told
/// (`M1.13.4.1`). Going through the real path means a kind module that states its language version is
/// read here the way `archogen` reads it, and a loader regression fails these tests instead of only
/// the CLI's.
fn registry_from(files: &[&str]) -> Registry {
    let mut sources = SourceMap::new();
    let kind_files = kind_files(files);
    shipped_registry(&mut sources, &kind_files).unwrap_or_else(|errors| {
        panic!(
            "the kind modules are malformed:\n{}",
            errors
                .iter()
                .map(|d| d.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })
}

#[test]
fn the_core_kinds_are_declared_in_eadl_not_in_rust() {
    // ⭐ §2: "a small trusted semantic foundation remains explicit. The registry cannot
    // silently introduce new trusted axioms." Exactly one primitive is trusted, and this test
    // is what keeps that sentence true as the language grows.
    let registry = core_registry();
    assert_eq!(
        registry.heads(),
        vec![
            "defblock",
            "defplatform",
            "defpolicy",
            "defservice",
            "defsystem",
            "task"
        ],
        "the five §5.1 surface kinds plus the os/rt `task` kind, all declared in eADL"
    );
    assert!(
        registry.kind("defkind").is_none(),
        "`defkind` must NOT be a registry entry — it is the one trusted primitive, implemented \
         in Rust, and an entry would make it look like just another declared kind"
    );
}

#[test]
fn every_core_kind_explains_itself_and_names_its_clauses() {
    let registry = core_registry();
    for head in registry.heads() {
        let kind = registry.kind(head).expect("registered");
        assert!(
            kind.doc.len() > 20,
            "`{head}` has no usable doc: {:?}",
            kind.doc
        );
        assert_eq!(
            kind.name,
            NameRule::Required,
            "`{head}` should carry a name"
        );
        assert!(!kind.clauses.is_empty(), "`{head}` admits no clauses");
    }
}

#[test]
fn every_accepted_boundary_case_validates_against_the_core_kinds() {
    let registry = core_registry();
    let root = repo_root().join("docs/semantics/boundary/accept");
    let mut checked = 0;
    for entry in std::fs::read_dir(&root).expect("the accept corpus exists") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|e| e != "eadl") {
            continue;
        }
        let relative = format!(
            "docs/semantics/boundary/accept/{}",
            path.file_name().expect("name").to_string_lossy()
        );
        let (forms, sources) = parse_file(&relative);
        for form in declarations(&forms) {
            let errors = validate(&registry, form);
            assert!(
                errors.is_empty(),
                "{relative} is an ACCEPTED case but the schema refused it:\n{}",
                errors
                    .iter()
                    .map(|d| d.render(&sources))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
        checked += 1;
    }
    assert_eq!(checked, 10, "the accept corpus changed size");
}

/// Walk the reject corpus and measure how far the schema reaches into it.
///
/// Returns `(refused_by_schema, out_of_reach)`, both sorted.
///
/// ⭐ **One implementation, deliberately.** [`the_schema_now_reaches_every_rejected_case`] asserts
/// the reach and [`the_live_surfaces_publish_the_measured_reach`] compares the published prose
/// against it. Two ways of measuring would reintroduce exactly the defect that pair exists to end:
/// a number in prose that nothing re-derives.
///
/// The classifier assertion lives inside the walk rather than in a caller, because "two
/// independent refusals for the same content" is a property of *each* case, and putting it here
/// means no caller can measure the reach without also checking it.
fn measure_reach() -> (Vec<String>, Vec<String>) {
    let registry = core_registry();
    let root = repo_root().join("docs/semantics/boundary/reject");

    let mut refused_by_schema: Vec<String> = Vec::new();
    let mut out_of_reach: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(&root).expect("the reject corpus exists") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|e| e != "eadl") {
            continue;
        }
        let file = path
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();
        let relative = format!("docs/semantics/boundary/reject/{file}");
        let (forms, _) = parse_file(&relative);
        for form in declarations(&forms) {
            if validate(&registry, form).is_empty() {
                out_of_reach.push(file.clone());
            } else {
                refused_by_schema.push(file.clone());
            }
            // Whatever the schema sees, F27's classifier must refuse every one of them. Two
            // independent refusals for the same content is a feature, not redundancy.
            assert!(
                !eadl_model::boundary::classify(form).is_accepted(),
                "{relative}: neither mechanism refused it"
            );
        }
    }

    refused_by_schema.sort();
    out_of_reach.sort();
    (refused_by_schema, out_of_reach)
}

#[test]
fn the_schema_now_reaches_every_rejected_case() {
    // ⭐ THE GAP, CLOSED AND RE-PINNED. This test previously asserted a measured 10-of-11 split:
    // `execution-bound` hid `wcet` inside `(task …)`, which was declared `(holds forms)` and
    // therefore opaque to the schema, so only the boundary classifier caught it.
    //
    // `M1.7` gave `task` a real kind and changed the clause to `(holds kind task)`, which makes
    // the schema recurse into it. Keeping the assertion exact rather than loosening it to "at
    // least one" is the point: if a later clause moves back to `(holds forms)`, or a new corpus
    // case hides a construct somewhere else opaque, this fails and someone has to say so
    // deliberately.
    let (refused_by_schema, out_of_reach) = measure_reach();

    assert_eq!(
        refused_by_schema.len() + out_of_reach.len(),
        13,
        "the reject corpus changed size"
    );
    assert!(
        out_of_reach.is_empty(),
        "the schema no longer reaches every rejected case: {out_of_reach:?} — if that is \
         deliberate, say so here and name the leaf that closes it again"
    );
}

/// The three live surfaces that talk about the schema's reach.
///
/// `include_str!` rather than `fs::read_to_string`, for the reason `profile.rs` gives for the
/// published profile page: if one of these moves or is deleted, this crate **stops compiling**
/// instead of silently gating nothing.
const KIND_HEADER: &str = include_str!("../src/kind.rs");
const BOOK_KINDS: &str = include_str!("../../../docs/book/src/kinds.md");
const BOOK_WORKLOAD: &str = include_str!("../../../docs/book/src/workload.md");

/// The lines that may carry a reach figure **other than** the current measurement, quoted verbatim.
///
/// ⛔ An explicit list, and not a scan for past-tense words. The scan this replaced excused any
/// non-current figure on a line containing `was`, `were`, `previously`, `before` or `used to`, and
/// `M1.24` built the same escape for a sibling gate and **measured it passing on its own defect**: the
/// corpus index's stale line reads `…worked classification case before adoption", and the five ambiguous
/// cases`, where the `before` belongs to a quotation of `ROADMAP.md` §4.3 seven words ahead of the wrong
/// number. A marker scan cannot tell "this figure is history" from "this line happens to contain a
/// past-tense word", and the coincidence is readily available in exactly the surfaces this gate reads,
/// because each of them discusses what the reach *was*.
///
/// ⭐ What the list buys: history stays legal **and visible**. Adding a historical figure is a change to
/// this file, so a reader of the gate sees every stale number the repository keeps on purpose; and
/// rewording one of those lines fails the gate, which is the point — the line is quoted here exactly as
/// it appears, so the two cannot drift.
///
/// ⚠️ Both entries, not one: `M1.25`'s leaf named `kinds.md`'s line, and the census found the same
/// historical figure in `workload.md` too (`grep -n ' of ' docs/book/src/kinds.md docs/book/src/workload.md`).
const HISTORICAL_REACH_LINES: &[&str] = &[
    "tree. The measured reach was **10 of 11** rejected cases.",
    "was **10 of 11** rejected corpus cases.",
];

/// Every way the live surfaces disagree with the measured reach.
///
/// A function rather than inline assertions so an arm can feed it a mutation: a gate whose only
/// exercise is the real tree has never been seen to fire, which is the shape
/// `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` is about.
fn reach_violations(
    header: &str,
    chapters: &[(&str, &str)],
    measured: (usize, usize),
    historical: &[&str],
) -> Vec<String> {
    let mut out = Vec::new();

    // ── 1. The module header carries NO figure, and routes the reader to the measurement. ──
    // A header that states a count is a header that can go stale; a header that names the test
    // that counts is a header that cannot.
    for (_, n, m, line) in reach_figures(header) {
        out.push(format!(
            "kind.rs's module header carries a reach figure (`{n} of {m}`) — state the rule and name \
             `the_schema_now_reaches_every_rejected_case` instead, or the figure will outlive the \
             measurement\n  {line}"
        ));
    }
    if !header.contains("the_schema_now_reaches_every_rejected_case") {
        out.push(
            "kind.rs's module header no longer names the test that measures the reach, so a reader has \
             nowhere to go to check it"
                .to_string(),
        );
    }

    for (name, text) in chapters {
        let figures = reach_figures(text);

        // ── 2. Each book chapter publishes the measured pair as its CURRENT figure … ──
        if !figures.iter().any(|(_, n, m, _)| (*n, *m) == measured) {
            out.push(format!(
                "{name} does not publish the measured reach {} of {} anywhere; the figures it carries \
                 are {:?}",
                measured.0,
                measured.1,
                figures
                    .iter()
                    .map(|(line, n, m, _)| format!("{line}: {n} of {m}"))
                    .collect::<Vec<_>>()
            ));
        }

        // ── 3. … and every OTHER figure in it is one this file lists as history. ──
        for (line_no, n, m, line) in &figures {
            if (*n, *m) == measured {
                continue;
            }
            if historical.iter().any(|listed| *listed == line.trim()) {
                continue;
            }
            out.push(format!(
                "{name}:{line_no} states `{n} of {m}` as though it were current, but the corpus \
                 measures {} of {}. Either update the figure, or — if it is deliberately describing an \
                 older measurement — add the line verbatim to `HISTORICAL_REACH_LINES`, where a reader \
                 of the gate can see it\n  {}",
                measured.0, measured.1, line
            ));
        }
    }
    out
}

/// Every `<n> of <m>` figure in `text`, as `(line_number, n, m, the_line)`.
///
/// Handles the `of the` spelling too, so a surface that writes "13 of the 13" is still read as a
/// figure rather than skipped — a gate that cannot see a variant of the thing it guards is a gate
/// that passes for the wrong reason.
fn reach_figures(text: &str) -> Vec<(usize, usize, usize, String)> {
    let mut found = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let mut rest = line;
        while let Some(at) = rest.find(" of ") {
            let (before, after) = rest.split_at(at);
            let after = &after[" of ".len()..];
            let digits = |s: &str| {
                s.chars()
                    .rev()
                    .take_while(char::is_ascii_digit)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .collect::<String>()
            };
            let numerator: usize = digits(before).parse().unwrap_or(0);
            let denominator_text = after.strip_prefix("the ").unwrap_or(after);
            let denominator: usize = denominator_text
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
                .parse()
                .unwrap_or(0);
            if numerator > 0 && denominator > 0 {
                found.push((index + 1, numerator, denominator, line.to_string()));
            }
            rest = &after[1.min(after.len())..];
        }
    }
    found
}

/// The measured reach, as the pair every live surface has to publish.
fn measured_reach() -> (usize, usize) {
    let (refused, out_of_reach) = measure_reach();
    (refused.len(), refused.len() + out_of_reach.len())
}

/// `kind.rs`'s module header — the `//!` block only, which is the part a reader meets first.
fn kind_header() -> String {
    KIND_HEADER
        .lines()
        .take_while(|line| line.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The two chapters that publish the reach.
fn reach_chapters() -> [(&'static str, &'static str); 2] {
    [
        ("docs/book/src/kinds.md", BOOK_KINDS),
        ("docs/book/src/workload.md", BOOK_WORKLOAD),
    ]
}

/// Violations against the live surfaces, for the green leg.
fn live_reach_violations() -> Vec<String> {
    reach_violations(
        &kind_header(),
        &reach_chapters(),
        measured_reach(),
        HISTORICAL_REACH_LINES,
    )
}

/// Violations against mutated surfaces, for an arm.
fn violations_with(header: &str, kinds: &str, workload: &str, historical: &[&str]) -> Vec<String> {
    reach_violations(
        header,
        &[
            ("docs/book/src/kinds.md", kinds),
            ("docs/book/src/workload.md", workload),
        ],
        measured_reach(),
        historical,
    )
}

/// Assert the gate reported exactly `expected` violations, with every `needle` among them.
///
/// ⛔ **The count is part of the arm, not decoration.** An arm that only checks "some complaint mentions
/// X" also passes on a gate that started reporting *everything*, which is the vacuous-pass shape
/// `conformance.rs` refuses for the grammar. The same helper and the same reasoning as `corpus.rs`'s.
fn assert_reported(wrong: &[String], expected: usize, needles: &[&str]) {
    assert_eq!(
        wrong.len(),
        expected,
        "expected {expected} violation(s); the gate reported {}:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
    for needle in needles {
        assert!(
            wrong.iter().any(|violation| violation.contains(needle)),
            "no complaint mentions {needle:?}; the gate reported:\n\n{}",
            wrong.join("\n\n")
        );
    }
}

#[test]
fn the_live_surfaces_publish_the_measured_reach() {
    // ⛔ THE DEFECT THIS GATES. `crates/eadl-model/src/kind.rs`'s module header and
    // `docs/book/src/kinds.md` both published "the schema refuses 10 of the 11 rejected cases"
    // for **47 commits** after `M1.7` made it 13 of 13 — and the book thereby contradicted its own
    // sibling chapter, `workload.md`, which had been updated. Nothing failed, because a number
    // copied into prose is copied out of the reach of the test that took it.
    //
    // So the prose is compared against the measurement instead of being trusted. Retyping the
    // fresh number and changing nothing else is explicitly not the fix: `docs/CLAIM_VERIFICATION.md`
    // §5B calls that "correcting a stale constant to a fresh constant".
    let wrong = live_reach_violations();
    assert!(
        wrong.is_empty(),
        "{} way(s) in which a live surface publishes a reach figure the corpus does not measure:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

// ── RED arms ──────────────────────────────────────────────────────────────────────────────────
//
// ⛔ These arms did not exist before `M1.25`. The leaf that scheduled this work said "all five existing
// arms are re-expressed against the new mechanism"; measured, `grep -c '^fn arm_' crates/eadl-model/tests/kinds.rs`
// → **0** before this commit, and the seven arms it was probably thinking of belong to `corpus.rs`'s
// *boundary-corpus* figure gate, a different gate over different surfaces. So the gate that closed
// `M1.23` had never been seen to fire, which is a worse starting point than the leaf recorded and the
// reason arm 1 is written first: an arm that reproduces the original defect is the one that proves the
// gate is a gate.

#[test]
fn arm_1_a_stale_reach_figure_is_reported_in_both_chapters() {
    // The defect that made `M1.23` exist, replayed as a fixture: one chapter retyped to a figure the
    // corpus does not measure. Two violations and not one — the chapter stops publishing the measured
    // pair, *and* the pair it publishes is nobody's history — which is the count this arm pins.
    let (kinds, workload) = (BOOK_KINDS, BOOK_WORKLOAD);
    let stale = kinds.replace("13 of 13", "12 of 13");
    assert_ne!(stale, kinds, "the mutation did not apply — a false green");
    let wrong = violations_with(&kind_header(), &stale, workload, HISTORICAL_REACH_LINES);
    assert_reported(
        &wrong,
        2,
        &["does not publish the measured reach", "12 of 13"],
    );
}

#[test]
fn arm_2_a_historical_figure_is_legal_only_while_it_is_listed() {
    // The arm the replaced mechanism expressed as "strip the past tense off a legitimate historical
    // figure". The same intent with a sound trigger: the line stops being listed, so the gate has to
    // report it. Two figures over two chapters, because `workload.md` carries the same history.
    let wrong = violations_with(&kind_header(), BOOK_KINDS, BOOK_WORKLOAD, &[]);
    assert_reported(&wrong, 2, &["10 of 11", "HISTORICAL_REACH_LINES"]);
}

#[test]
fn arm_3_a_module_header_that_publishes_a_figure_is_reported() {
    // The other half of `M1.23`'s defect: the header is where the stale figure lived for 47 commits.
    let header = format!(
        "{}\n//! The schema refuses 10 of 11 rejected cases.\n",
        kind_header()
    );
    let wrong = violations_with(&header, BOOK_KINDS, BOOK_WORKLOAD, HISTORICAL_REACH_LINES);
    assert_reported(&wrong, 1, &["module header carries a reach figure"]);
}

#[test]
fn arm_4_a_module_header_that_names_no_measurement_is_reported() {
    // A header with no figure and no pointer is a header whose reader cannot check anything, which is
    // the state `M1.23`'s fix exists to end.
    let header = kind_header().replace(
        "the_schema_now_reaches_every_rejected_case",
        "the reach test",
    );
    assert_ne!(
        header,
        kind_header(),
        "the mutation did not apply — a false green"
    );
    let wrong = violations_with(&header, BOOK_KINDS, BOOK_WORKLOAD, HISTORICAL_REACH_LINES);
    assert_reported(&wrong, 1, &["no longer names the test that measures"]);
}

#[test]
fn arm_5_a_chapter_that_publishes_no_figure_is_reported() {
    // Dropping the figure must not be a way to pass: an unquantified claim is the one a reader cannot
    // check at all, which is the reasoning `corpus.rs`'s arm 6 records for the boundary corpus.
    let vague = BOOK_KINDS.replace("13 of 13", "every one of them");
    assert_ne!(
        vague, BOOK_KINDS,
        "the mutation did not apply — a false green"
    );
    let wrong = violations_with(
        &kind_header(),
        &vague,
        BOOK_WORKLOAD,
        HISTORICAL_REACH_LINES,
    );
    assert_reported(&wrong, 1, &["does not publish the measured reach"]);
}

#[test]
fn arm_6_the_past_tense_escape_m1_24_measured_does_not_work_here() {
    // ⭐ THE ARM THIS LEAF EXISTS FOR. `M1.24` built a marker scan for a sibling gate and measured it
    // **passing on its own defect**: a stale figure excused because its line happened to contain
    // `before`, seven words away from the number. This arm writes that exact line — a stale figure on a
    // line carrying `before`, `was` and `used to` all at once — and requires it to be reported. Under
    // the mechanism this leaf replaced, it would have been excused three times over.
    let poisoned =
        format!("{BOOK_KINDS}\nThe reach used to be 10 of 13 and was measured before adoption.\n");
    assert_ne!(
        poisoned, BOOK_KINDS,
        "the mutation did not apply — a false green"
    );
    let wrong = violations_with(
        &kind_header(),
        &poisoned,
        BOOK_WORKLOAD,
        HISTORICAL_REACH_LINES,
    );
    assert_reported(&wrong, 1, &["10 of 13", "as though it were current"]);
}

#[test]
fn an_execution_bound_inside_a_task_is_now_caught_by_the_schema_too() {
    // The specific case the gap was measured on. It must be refused with the BOUNDARY wording,
    // not merely as an unknown clause: `wcet` is not a typo, it is content in the wrong layer.
    let registry = core_registry();
    let relative = "docs/semantics/boundary/reject/execution-bound.eadl";
    let (forms, sources) = parse_file(relative);
    let errors = validate(&registry, declarations(&forms)[0]);
    let rendered = errors
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered.contains("boundary-implementation-in-description"),
        "the schema recursed but used generic wording:\n{rendered}"
    );
    assert!(rendered.contains("build manifest"), "{rendered}");
}

#[test]
fn a_missing_kind_module_is_reported_not_ignored() {
    // A registry without `os-rt.eadl` cannot validate a task. It must SAY so — silently
    // accepting whatever is inside an unvalidatable clause is the failure mode that let the
    // gap exist in the first place.
    let registry = registry_from(&["docs/semantics/kinds/core.eadl"]);
    let (system, sources) = parse_file("docs/semantics/boundary/reject/execution-bound.eadl");
    let errors = validate(&registry, declarations(&system)[0]);
    let rendered = errors
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered.contains("schema-unknown-referenced-kind"),
        "an unvalidatable clause passed silently:\n{rendered}"
    );
    assert!(rendered.contains("os-rt.eadl"), "{rendered}");
}

#[test]
fn a_forbidden_construct_gets_the_boundary_wording_not_unknown_clause() {
    // "`implementation` is not a clause of `defservice`" is true and useless. The refusal must
    // say what the construct is and where it belongs.
    let registry = core_registry();
    let relative = "docs/semantics/boundary/reject/rollover-algorithm.eadl";
    let (forms, sources) = parse_file(relative);
    let errors = validate(&registry, declarations(&forms)[0]);
    let rendered = errors
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        rendered.contains("boundary-implementation-in-description"),
        "the schema used its own generic wording instead of the boundary's:\n{rendered}"
    );
    assert!(rendered.contains("belongs to"), "{rendered}");
}

#[test]
fn cardinality_is_enforced_in_both_directions() {
    let registry = core_registry();
    let defservice = registry.kind("defservice").expect("registered");
    assert_eq!(
        defservice
            .clause("requires")
            .expect("a requires clause")
            .cardinality,
        Cardinality::OneOrMore
    );

    let mut sources = SourceMap::new();
    // Missing the required clause entirely.
    let id = sources.add("t.eadl", "(defservice s)").expect("small");
    let (document, _) = read(&sources, id);
    let errors = validate(&registry, &document.forms[0]);
    assert!(
        errors.iter().any(|d| d.code == "schema-cardinality"),
        "a missing required clause was not reported"
    );

    // `platform` is at-most-one; two is a violation.
    let id = sources
        .add("u.eadl", "(defsystem s (platform (a)) (platform (b)))")
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = validate(&registry, &document.forms[0]);
    assert!(
        errors.iter().any(|d| d.code == "schema-cardinality"),
        "a repeated at-most-one clause was not reported"
    );
}

#[test]
fn an_unknown_clause_suggests_the_near_miss_and_lists_the_real_ones() {
    let registry = core_registry();
    let mut sources = SourceMap::new();
    let id = sources
        .add("t.eadl", "(defservice s (requires (x)) (requries (y)))")
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = validate(&registry, &document.forms[0]);
    let unknown = errors
        .iter()
        .find(|d| d.code == "schema-unknown-clause")
        .expect("the typo must be reported");
    assert!(
        unknown.repair.contains("did you mean `requires`"),
        "no suggestion for an obvious typo: {}",
        unknown.repair
    );
}

#[test]
fn an_unknown_kind_does_not_get_a_nonsense_suggestion() {
    // Suggesting `defsystem` for `implementation` would send the author to rename rather than
    // to reconsider. The edit-distance bound exists for this.
    let registry = core_registry();
    let mut sources = SourceMap::new();
    let id = sources
        .add("t.eadl", "(deftimer x (offers (a)))")
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = validate(&registry, &document.forms[0]);
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].code, "schema-unknown-kind");
    assert!(
        errors[0].repair.contains("the known kinds are"),
        "{}",
        errors[0].repair
    );
}

#[test]
fn a_missing_name_is_reported_with_the_kinds_own_doc() {
    let registry = core_registry();
    let mut sources = SourceMap::new();
    let id = sources
        .add("t.eadl", "(defservice (requires (x)))")
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = validate(&registry, &document.forms[0]);
    let missing = errors
        .iter()
        .find(|d| d.code == "schema-missing-name")
        .expect("a missing name must be reported");
    assert!(
        missing.repair.contains("required OS service"),
        "the repair should quote the kind's own doc: {}",
        missing.repair
    );
}

#[test]
fn a_kind_definition_carrying_implementation_is_refused() {
    // ⛔ §5.6: `defkind` must not become an implementation template language. The same
    // classifier that refuses implementation in a declaration refuses it in the facility that
    // declares the language — there is no back door.
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            "t.eadl",
            "(defkind deftimer (doc \"a timer\") (name required) (implementation (emit-template \"t.rs\")))",
        )
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = read_kind(&document.forms[0]).expect_err("must refuse");
    assert!(
        errors
            .iter()
            .any(|d| d.code == "boundary-implementation-in-description"),
        "a kind definition smuggled implementation through: {:?}",
        errors.iter().map(|d| d.code).collect::<Vec<_>>()
    );
}

#[test]
fn a_kind_definition_must_explain_itself() {
    let mut sources = SourceMap::new();
    let id = sources
        .add("t.eadl", "(defkind deftimer (name required))")
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = read_kind(&document.forms[0]).expect_err("must refuse");
    assert!(errors.iter().any(|d| d.code == "schema-missing-doc"));
}

#[test]
fn a_kind_definition_reports_every_problem_not_just_the_first() {
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            "t.eadl",
            "(defkind deftimer (doc \"t\") (name maybe) (clause a (cardinality lots)) (nonsense x))",
        )
        .expect("small");
    let (document, _) = read(&sources, id);
    let errors = read_kind(&document.forms[0]).expect_err("must refuse");
    let codes: Vec<&str> = errors.iter().map(|d| d.code).collect();
    assert!(codes.contains(&"schema-bad-name-rule"), "{codes:?}");
    assert!(codes.contains(&"schema-bad-cardinality"), "{codes:?}");
    assert!(codes.contains(&"schema-unknown-kind-field"), "{codes:?}");
}

#[test]
fn registering_a_kind_twice_is_refused() {
    // §15: a source description retains its meaning under its locked semantic version.
    // Silently redefining a kind is how that stops being true.
    let mut registry = core_registry();
    let (forms, _) = parse_file("docs/semantics/kinds/core.eadl");
    // The first **kind**, which is not the first form any more: `core.eadl` states its language
    // version, and §8 says that form is not a declaration.
    let kind = read_kind(declarations(&forms)[0]).expect("well-formed");
    let error = registry
        .register(kind)
        .expect_err("a duplicate must be refused");
    assert_eq!(error.code, "schema-duplicate-kind");
    assert!(error.repair.contains("silently change"), "{}", error.repair);
}

#[test]
fn the_language_version_identifier_survives_the_model_layer() {
    // §8 of `docs/semantics/reference.md` puts the language version on the surface as a **top-level
    // form**. `crates/eadl-model/src/check.rs` validates every top-level form against the kind
    // registry, so without an exemption `(eadl-version eadl/1)` reads cleanly in the frontend and is
    // then refused *here* as `schema-unknown-kind` — §8 contradicted end-to-end, by the layer furthest
    // from it, and invisibly: no frontend test can see a model-layer refusal.
    //
    // ⛔ This runs the real path — `check`, the entry point S0 and the corpus suite use — rather than
    // calling `validate` directly. The first version of this test called `validate`, which is the
    // function the exemption sits *in front of*, so it kept failing after the fix landed and would have
    // kept failing whatever `check.rs` did. A test that bypasses the layer under test measures the
    // wrong thing, confidently.
    //
    // Measured before the fix: `error[schema-unknown-kind]: `eadl-version` is not a known kind`.
    use eadl_model::check::{check, default_profile};

    let mut sources = SourceMap::new();
    let id = sources
        .add(
            "version.eadl",
            "(eadl-version eadl/1)\n(defsystem s (name n))",
        )
        .expect("small");
    let registry = core_registry();
    let outcome = check(&sources, id, &registry, default_profile());
    let rendered: Vec<String> = outcome
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.render(&sources))
        .collect();
    assert!(
        !rendered
            .iter()
            .any(|item| item.contains("schema-unknown-kind")),
        "the language-version identifier was refused by the schema layer:\n{}",
        rendered.join("\n")
    );
}

#[test]
fn stating_a_version_does_not_change_how_many_declarations_a_description_has() {
    // ⭐ The half of §8's rule that is user-visible. `archogen check` prints
    // "accepted against profile `…` (N declaration(s))" and `docs/book/src/checking.md` publishes that
    // line for a real example, so the field the count comes from has to hold **declarations** and not
    // top-level forms — or every description that states its language version is reported as one
    // declaration larger than it is, in the sentence a reader is most likely to believe.
    //
    // This is what makes `M1.13.4.2`'s retrofit safe for the book: the eight declarations of
    // `examples/periodic-three/system.eadl` stay eight when the identifier is added above them.
    use eadl_model::check::{check, default_profile};

    let without = "(defblock b (offers (x true)))\n(defsystem s (platform (uses b)))";
    let with = &format!("(eadl-version eadl/1)\n{without}");
    let registry = core_registry();

    let mut sources = SourceMap::new();
    let unstated = sources
        .add("unstated.eadl", without.to_string())
        .expect("small");
    let stated = sources.add("stated.eadl", with.clone()).expect("small");
    let first = check(&sources, unstated, &registry, default_profile());
    let second = check(&sources, stated, &registry, default_profile());

    assert_eq!(
        first.declarations.len(),
        second.declarations.len(),
        "stating the language version changed the reported declaration count: {} vs {}",
        first.declarations.len(),
        second.declarations.len()
    );
    assert_eq!(
        second.declarations.len(),
        2,
        "the identifier was counted as a declaration"
    );
    assert!(
        second
            .declarations
            .iter()
            .all(|form| form.head() != Some("eadl-version")),
        "`Outcome::declarations` holds the identifier, so every consumer of the field inherits it"
    );
}

#[test]
fn a_kind_module_may_state_its_language_version() {
    // §8 says a *description* states its language version, and a kind module is a description: the
    // loader reads it with the same `read` and the file is a tracked `.eadl` like any other. So the
    // identifier has to be skipped here too, or the one file kind that declares the language cannot
    // say which version of it it declares.
    //
    // ⛔ Measured, not anticipated. Before this leaf the loader handed **every** top-level form to
    // `read_kind`, which refused the identifier as `schema-not-a-kind` and took the whole registry
    // with it — `archogen: tool-failure: the shipped kind modules could not be loaded`, and 36 of the
    // 45 failures `M1.13.4`'s retrofit measurement produced. It is the third consumer of §8's rule
    // (`is_identifier`'s doc comment names all three) and the one no description-level test could
    // reach, because nothing reads a kind module except this loader.
    //
    // ⭐ The assertion is that stating a version changes **nothing** about what is declared, which is
    // the property a filter has to have: not merely "does not fail".
    let stated = "(eadl-version eadl/1)\n(defkind deftimer (doc \"a timer\") (name required))";
    let unstated = "(defkind deftimer (doc \"a timer\") (name required))";

    let mut sources = SourceMap::new();
    let with = shipped_registry(
        &mut sources,
        &[("stated.eadl".to_string(), stated.to_string())],
    )
    .unwrap_or_else(|errors| {
        panic!(
            "a kind module stating its language version was refused:\n{}",
            errors
                .iter()
                .map(|d| d.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        )
    });
    let without = shipped_registry(
        &mut sources,
        &[("unstated.eadl".to_string(), unstated.to_string())],
    )
    .expect("a kind module with no identifier loads");

    assert_eq!(
        with.heads(),
        without.heads(),
        "stating the language version changed what the module declares"
    );
    assert_eq!(with.heads(), vec!["deftimer"], "the kind is registered");
    assert_eq!(
        with.len(),
        without.len(),
        "the identifier was registered as a kind of its own"
    );
}

#[test]
fn a_malformed_version_in_a_kind_module_is_still_reported() {
    // ⛔ The other half of the fix, and the half a "just skip the form" patch gets wrong: skipping the
    // identifier must not swallow a *problem* with one. `shipped_registry` surfaces the reader's own
    // diagnostics before it iterates anything, so a version this toolchain does not read is still
    // refused with §8's code — a filter that quietly dropped the form would trade a false refusal for
    // a silent one, and a kind module claiming `eadl/2` would load as though it claimed `eadl/1`.
    let text = "(eadl-version eadl/2)\n(defkind deftimer (doc \"a timer\") (name required))";
    let mut sources = SourceMap::new();
    let errors = shipped_registry(&mut sources, &[("bad.eadl".to_string(), text.to_string())])
        .expect_err("a version this toolchain does not read must be refused");
    let codes: Vec<&str> = errors.iter().map(|d| d.code).collect();
    assert!(
        codes.contains(&"language-version-unknown"),
        "§8's own diagnostic was swallowed by the filter: {codes:?}"
    );
    assert!(
        !codes.iter().any(|code| code.starts_with("schema-")),
        "the refusal must be about the version, not about the identifier not being a kind: {codes:?}"
    );
}

// ── `quantity`: the value type that consumes two forms ──────────────────────────────────────────────
//
// ⛔ **Leaf `M1.28`.** `(holds values number symbol)` said `parsec` was a perfectly good unit, because it
// is a perfectly good *symbol* — which is how `archogen check` came to accept `(period 10 parsec)` while
// `archogen build` refused the same bytes. What a quantity is belongs to
// `crates/eadl-model/src/quantity.rs`, and the schema **asks it and propagates its answer** rather than
// re-implementing the question, so the refusal an author sees is `quantity-unknown-unit` and not a second
// code for one mistake.

use eadl_front::Diagnostic;
use eadl_model::kind::{Holds, ValueType};

/// Validate one inline description against the shipped registry.
fn validate_text(text: &str) -> (Vec<Diagnostic>, String) {
    let registry = core_registry();
    let mut sources = SourceMap::new();
    let id = sources.add("inline.eadl", text.to_string()).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "the fixture did not read:\n{}",
        diagnostics.render(&sources)
    );
    let mut errors: Vec<Diagnostic> = Vec::new();
    for form in declarations(&document.forms) {
        errors.extend(validate(&registry, form));
    }
    let rendered = errors
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    (errors, rendered)
}

#[test]
fn the_shipped_task_clauses_declare_a_quantity_and_not_a_number_and_a_symbol() {
    // ⭐ A leg over the *declaration*, because the declaration is the normative surface: a kind module
    // reverted to `(holds values number symbol)` would leave every mechanism in `kind.rs` correct and
    // the language unenforced again.
    let registry = core_registry();
    let task = registry
        .kind("task")
        .expect("`os-rt.eadl` declares the task kind");
    for clause in ["period", "min-separation", "deadline", "jitter"] {
        let declared = task
            .clause(clause)
            .unwrap_or_else(|| panic!("the task kind has no `{clause}` clause"));
        assert_eq!(
            declared.holds,
            Holds::Values(vec![ValueType::Quantity]),
            "`{clause}` measures something, so its kind has to say so — `number` then `symbol` is a \
             shape, and `parsec` satisfies it"
        );
    }
}

#[test]
fn a_quantity_value_type_consumes_two_forms_and_is_read_by_the_pair() {
    // The width, the spelling, and the fact that no single form is one. `admits` answers about ONE form,
    // so it is `false` for every form here — and a loop that zipped values against types positionally
    // would refuse every quantity in the language. That is the bug `ValueType::width` exists to prevent.
    assert_eq!(ValueType::Quantity.width(), 2);
    assert_eq!(ValueType::Symbol.width(), 1);
    assert_eq!(ValueType::Quantity.slug(), "quantity");
    assert_eq!(ValueType::Quantity.spelling(), "<number> <unit>");

    let (forms, _) = parse_file("docs/semantics/kinds/os-rt.eadl");
    let mut probed = 0;
    for form in &forms {
        for item in form.items() {
            assert!(
                !ValueType::Quantity.admits(item),
                "no single form is a quantity, and `{}` was admitted as one",
                item.kind()
            );
            probed += 1;
        }
    }
    assert!(
        probed > 10,
        "the kind module yielded {probed} forms to probe, so this leg proved almost nothing"
    );
}

#[test]
fn a_clause_declared_to_hold_a_quantity_takes_two_values() {
    let (errors, rendered) =
        validate_text("(defsystem s (task t (period 10) (deadline 10 ms) (priority 1)))");
    let codes: Vec<&str> = errors.iter().map(|d| d.code).collect();
    assert_eq!(codes, vec!["schema-arity"], "{rendered}");
    assert!(
        rendered.contains("takes 2 value(s), found 1"),
        "the arity is the width and not the number of declared types: {rendered}"
    );
    // ⭐ The repair direction is in the AUTHOR's vocabulary: `<number> <unit>`, because "write a
    // `<quantity>` here" would name the kind vocabulary in a message about a description.
    assert!(rendered.contains("(period <number> <unit>)"), "{rendered}");
}

#[test]
fn a_bad_unit_is_refused_with_the_quantity_s_own_code_and_not_a_schema_one() {
    let (errors, rendered) =
        validate_text("(defsystem s (task t (period 10 parsec) (deadline 10 ms) (priority 1)))");
    let codes: Vec<&str> = errors.iter().map(|d| d.code).collect();
    assert_eq!(
        codes,
        vec!["quantity-unknown-unit"],
        "one mistake, one code, and it is `quantity.rs`'s: {rendered}"
    );
    assert!(
        rendered.contains("the known units are"),
        "the refusal has to list the table, which is the repair direction §5.5 requires: {rendered}"
    );
}

#[test]
fn a_value_type_a_kind_definition_does_not_know_lists_every_one_that_exists() {
    // ⭐ The repair direction for `schema-bad-value-type` is a list, and a list inside a message is a
    // copy of the enumeration it describes. It is now built from `ValueType::ALL`, and this leg compares
    // the message against that enumeration rather than against a second list written here — which is the
    // only form of the assertion that fails when a type is added and the message is not.
    let text = "(defkind thing (doc \"d\") (name required) \
                 (clause c (cardinality one) (holds values parsec)))";
    let mut sources = SourceMap::new();
    let id = sources.add("inline.eadl", text.to_string()).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "the fixture did not read:\n{}",
        diagnostics.render(&sources)
    );
    let errors = read_kind(declarations(&document.forms)[0]).expect_err("must refuse");
    let rendered = errors
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    let codes: Vec<&str> = errors.iter().map(|d| d.code).collect();
    assert_eq!(codes, vec!["schema-bad-value-type"], "{rendered}");
    for kind in ValueType::ALL {
        assert!(
            rendered.contains(&format!("`{}`", kind.slug())),
            "the repair direction does not list `{}`, so an author reading it cannot write one: \
             {rendered}",
            kind.slug()
        );
    }
    assert!(
        rendered.contains("quantity"),
        "the type leaf `M1.28` added must be one an author is told about: {rendered}"
    );
}
