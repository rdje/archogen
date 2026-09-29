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
use eadl_model::kind::{read_kind, validate, Cardinality, NameRule, Registry};

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
    let mut registry = Registry::new();
    for file in [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ] {
        let (forms, sources) = parse_file(file);
        for form in &forms {
            let kind = read_kind(form).unwrap_or_else(|errors| {
                panic!(
                    "{file} has a malformed kind:\n{}",
                    errors
                        .iter()
                        .map(|d| d.render(&sources))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            });
            registry
                .register(kind)
                .unwrap_or_else(|e| panic!("{file}: {}", e.message));
        }
    }
    registry
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
        for form in &forms {
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
        for form in &forms {
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

/// A word that marks the figure on its own line as history rather than as the current measurement.
const HISTORY_MARKERS: &[&str] = &["was", "were", "previously", "before", "used to"];

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

#[test]
fn the_live_surfaces_publish_the_measured_reach() {
    // ⛔ THE DEFECT THIS GATES. `crates/eadl-model/src/kind.rs`'s module header and
    // `docs/book/src/kinds.md` both published "the schema refuses 10 of the 11 rejected cases"
    // for **47 commits** after `M1.7` made it 13 of 13 — and the book thereby contradicted its own
    // sibling chapter, `workload.md`, which had been updated. Nothing failed, because a number
    // copied into prose is copied out of the reach of the test that took it.
    //
    // So the prose is now compared against the measurement instead of being trusted. Retyping the
    // fresh number and changing nothing else is explicitly not the fix: `docs/CLAIM_VERIFICATION.md`
    // §5B calls that "correcting a stale constant to a fresh constant".
    let (refused, out_of_reach) = measure_reach();
    let total = refused.len() + out_of_reach.len();
    let measured = (refused.len(), total);

    // ── 1. The module header carries NO figure, and routes the reader to the measurement. ──
    // A header that states a count is a header that can go stale; a header that names the test
    // that counts is a header that cannot.
    let header: Vec<&str> = KIND_HEADER
        .lines()
        .take_while(|line| line.starts_with("//!"))
        .collect();
    let header = header.join("\n");
    assert!(
        reach_figures(&header).is_empty(),
        "kind.rs's module header carries a reach figure again: {:?} — state the rule and name \
         `the_schema_now_reaches_every_rejected_case` instead, or the figure will outlive the \
         measurement",
        reach_figures(&header)
    );
    assert!(
        header.contains("the_schema_now_reaches_every_rejected_case"),
        "kind.rs's module header no longer names the test that measures the reach, so a reader \
         has nowhere to go to check it"
    );

    // ── 2. Each book chapter publishes the measured pair as its CURRENT figure … ──
    for (name, text) in [
        ("docs/book/src/kinds.md", BOOK_KINDS),
        ("docs/book/src/workload.md", BOOK_WORKLOAD),
    ] {
        let figures = reach_figures(text);
        assert!(
            figures.iter().any(|(_, n, m, _)| (*n, *m) == measured),
            "{name} does not publish the measured reach {} of {total} anywhere; the figures it \
             carries are {:?}",
            measured.0,
            figures
                .iter()
                .map(|(line, n, m, _)| format!("{line}: {n} of {m}"))
                .collect::<Vec<_>>()
        );

        // ── 3. … and every OTHER figure in it is marked as history on its own line. ──
        // A figure that is neither the measurement nor marked past tense is a new stale claim.
        for (line_no, n, m, line) in &figures {
            if (*n, *m) == measured {
                continue;
            }
            assert!(
                HISTORY_MARKERS.iter().any(|marker| line.contains(marker)),
                "{name}:{line_no} states `{n} of {m}` as though it were current, but the corpus \
                 measures {} of {total}. Either update the figure, or mark the sentence as \
                 history (`was`, `before`, …) if it is deliberately describing an older \
                 measurement.\n  {line}",
                measured.0
            );
        }
    }
}

#[test]
fn an_execution_bound_inside_a_task_is_now_caught_by_the_schema_too() {
    // The specific case the gap was measured on. It must be refused with the BOUNDARY wording,
    // not merely as an unknown clause: `wcet` is not a typo, it is content in the wrong layer.
    let registry = core_registry();
    let relative = "docs/semantics/boundary/reject/execution-bound.eadl";
    let (forms, sources) = parse_file(relative);
    let errors = validate(&registry, &forms[0]);
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
    let (forms, sources) = parse_file("docs/semantics/kinds/core.eadl");
    let mut registry = Registry::new();
    for form in &forms {
        registry
            .register(read_kind(form).expect("core.eadl is well-formed"))
            .expect("no duplicates");
    }
    let (system, _) = parse_file("docs/semantics/boundary/reject/execution-bound.eadl");
    let errors = validate(&registry, &system[0]);
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
    let errors = validate(&registry, &forms[0]);
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
    let kind = read_kind(&forms[0]).expect("well-formed");
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
