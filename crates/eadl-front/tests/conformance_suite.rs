//! The manifest's own four rules, executed.
//!
//! `docs/semantics/conformance.md` declares `eadl/1`'s conformance suite: the roots, what each root
//! proves, the version, and what is excluded and why. This file is the leg that makes the declaration
//! checkable rather than descriptive — every rule the manifest states about its own population is
//! compared against a walk that does not share a root list with it.
//!
//! ⛔ **Why the rules are checked in both directions.** "Everything under a declared root is in the
//! suite" cannot see a root that stopped existing, and "nothing outside the roots is in the suite"
//! cannot see a description nobody walks. A population asserted from one side is a population that can
//! shrink silently, which is the failure `corpus.rs`'s census exists to refuse for the boundary corpus.

use std::path::{Path, PathBuf};

use eadl_front::language_version::EADL_1;

mod common;

use common::suite::{
    enumerate, excluded_by, identity_violations, outside_violations, population,
    repository_descriptions, under_roots, vacuous_exclusion_violations, Manifest, MANIFEST,
    MANIFEST_PATH,
};

/// The repository root, derived from this crate's manifest directory.
///
/// Repo-root-relative by construction: the repository can be moved, renamed or mounted on another
/// volume and this still resolves, which is the project's path policy.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The declared population, or a panic naming every problem with the manifest.
fn suite() -> Vec<(String, String)> {
    population(&repo_root()).unwrap_or_else(|problems| {
        panic!(
            "{} declares a suite that cannot be enumerated:\n\n{}",
            MANIFEST_PATH,
            problems.join("\n")
        )
    })
}

/// The excluded descriptions, read so rule 3 can compare bytes.
fn excluded_files(root: &Path) -> Vec<(String, String)> {
    let manifest = Manifest::declared();
    repository_descriptions(root)
        .into_iter()
        .filter(|path| manifest.excludes(path).is_some())
        .filter_map(|path| {
            std::fs::read_to_string(root.join(&path))
                .ok()
                .map(|text| (path, text))
        })
        .collect()
}

// ── the green legs ─────────────────────────────────────────────────────────────────────────────

#[test]
fn the_manifest_declares_one_version_and_the_roots_it_walks() {
    let manifest = Manifest::declared();
    assert_eq!(
        manifest.version, EADL_1,
        "the suite conforms to one language version, and `eadl_front::language_version::EADL_1` is the \
         only one this toolchain reads — a manifest naming another is a manifest for a suite that does \
         not exist yet"
    );
    assert!(
        !manifest.means.is_empty(),
        "the manifest states a version and not what conforming to it means"
    );
    assert!(
        manifest.roots.len() >= 4,
        "the manifest declares {} root(s): {}",
        manifest.roots.len(),
        manifest
            .roots
            .iter()
            .map(|root| root.path.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    for root in &manifest.roots {
        assert!(
            root.proves.len() > 40,
            "the root `{}` says what it proves in {:?}, which is a label and not a claim — a reader \
             cannot tell what a green suite established",
            root.path,
            root.proves
        );
    }
    assert!(
        !manifest.exclusions.is_empty(),
        "the manifest declares no exclusion, so nothing states what is deliberately outside the suite — \
         and the frozen LinkedSpec reproducers under `docs/feedback/` are outside it on purpose"
    );
    for exclusion in &manifest.exclusions {
        assert!(
            exclusion.reason.len() > 40,
            "the exclusion `{}` carries no real reason: {:?}",
            exclusion.prefix,
            exclusion.reason
        );
    }
}

#[test]
fn the_population_is_exactly_what_the_roots_hold_and_nothing_else() {
    // Rule 1, in both directions, against a walk that does not share a root list with the manifest.
    let root = repo_root();
    let manifest = Manifest::declared();
    let suite = suite();
    let repository = repository_descriptions(&root);
    assert!(
        !repository.is_empty(),
        "the repository walk found no description at all, so every leg below it is green on nothing"
    );

    let paths: Vec<String> = suite.iter().map(|(path, _)| path.clone()).collect();
    for path in &paths {
        assert!(
            repository.contains(path),
            "`{path}` is in the suite and the independent repository walk did not find it"
        );
        assert!(
            manifest
                .roots
                .iter()
                .any(|declared| excluded_by(&declared.path, path)),
            "`{path}` is in the suite and is under no declared root"
        );
        assert!(
            manifest.excludes(path).is_none(),
            "`{path}` is in the suite and matches an exclusion"
        );
    }

    // Rule 4: nothing in the repository is silently outside.
    let outside = outside_violations(&manifest, &paths, &repository);
    assert!(
        outside.is_empty(),
        "{} description(s) belong to no root and no exclusion:\n\n{}",
        outside.len(),
        outside.join("\n\n")
    );
}

#[test]
fn the_population_is_the_size_the_census_pins() {
    // ⛔ An absolute number, on purpose, and the reasoning is `corpus.rs`'s: a suite that silently
    // iterates over fewer files than it used to passes forever. The number is a census rather than a
    // promise — when a description is added or removed deliberately, this is the leg that says so out
    // loud, and `M1.13.5`'s freeze gate is what makes the movement require a migration note.
    //
    // 63 → 65 at leaf `M1.28.2`: `invalid-unknown-unit.eadl` and `invalid-quantity-without-unit.eadl`
    // joined, because the schema and the refinement pass learned to refuse a quantity they used to
    // discard, and a new rule with no worked case is a rule nothing in the suite exercises.
    //
    // 65 → 100 at leaf `M1.29.2`: the `docs/semantics/modules` root joined with 35 files — 26 cases, one
    // per `module-` code a command can now reach plus F01's two compositions, and the 9 library modules
    // they import. §6 had rules and 24 codes and no case a command could run.
    //
    // 100 → 105 at leaf `M1.29.3`: two library modules and three cases for §6 rules 9 and 10 — a module naming
    // its own siblings in `uses`, `needs` and `refines`, and `module-not-exported` directly and transitively.
    //
    // 105 → 107 at leaf `M1.33`: §7 rule 6, a name is declared once — one semantic case for a single file and
    // one module case for §6 rule 9's collision.
    //
    // 107 → 108 at leaf `M2.13`: `invalid-priority-below-one.eadl`, the worked case for the model's §4 rule
    // 5 — a priority is a rank from 1 — which the checker did not enforce until then.
    let suite = suite();
    assert_eq!(
        suite.len(),
        108,
        "the conformance suite holds {} descriptions and this census pins 108 — if a case was added or \
         removed deliberately, update the census in the same commit and say why in the leaf; if not, a \
         root stopped being walked",
        suite.len()
    );
}

#[test]
fn no_excluded_path_enters_the_population() {
    // Rule 2. The enumerator refuses a root that reaches an excluded path rather than skipping it, so
    // this leg is the second half: whatever it returned, none of it is excluded.
    let manifest = Manifest::declared();
    for (path, _) in suite() {
        assert!(
            manifest.excludes(&path).is_none(),
            "`{path}` is excluded by the manifest and is in the suite"
        );
    }
}

#[test]
fn no_suite_file_is_byte_identical_to_an_excluded_one() {
    // Rule 3, and the one a path prefix cannot express: a frozen reproducer copied into a walked root
    // has a suite path, so rules 2 and 4 both pass it. Measured usable rather than assumed — at
    // `M1.13.4`'s decomposition `docs/feedback/linkedspec/issues/LS-002-multi-form-truncation/evidence/system.eadl`
    // was byte-identical to `examples/s0-heartbeat/system.eadl`, and `M1.13.4.2`'s identifier diverged
    // them, so the pair that made content identity unusable is the pair that makes this leg sound.
    let root = repo_root();
    let wrong = identity_violations(&suite(), &excluded_files(&root));
    assert!(
        wrong.is_empty(),
        "{} suite file(s) duplicate a frozen reproducer:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

#[test]
fn every_exclusion_matches_a_real_description() {
    // ⛔ Non-vacuity, and the reason it has its own leg: an exclusion whose prefix is mistyped matches
    // nothing, passes every other leg, and lets the excluded area into the suite the first time a root
    // widens. "A registry entry nothing exercises is an assertion, not a rule" —
    // `docs/semantics/boundary/README.md`, about the boundary classifier, and true of a manifest row.
    let root = repo_root();
    let manifest = Manifest::declared();
    let repository = repository_descriptions(&root);
    let matched = vacuous_exclusion_violations(&manifest, &repository);
    assert_eq!(
        matched.len(),
        manifest.exclusions.len(),
        "the census did not account for every exclusion: {matched:?}"
    );
    for (prefix, count) in &matched {
        assert!(
            *count > 0,
            "the exclusion `{prefix}` matches no description in the repository, so it excludes nothing \
             and its reason is prose about a directory that is not there"
        );
    }
}

#[test]
fn the_exclusion_grammar_is_segment_wise_and_nothing_wider() {
    // The matcher's whole grammar, pinned, because a prefix matcher that quietly understood `*` would
    // make the manifest's stated rule false and every pattern in it ambiguous.
    assert!(excluded_by("docs/feedback", "docs/feedback/a/b.eadl"));
    assert!(excluded_by("docs/feedback/", "docs/feedback/a/b.eadl"));
    assert!(excluded_by("docs/feedback", "docs/feedback"));
    assert!(
        !excluded_by("docs/feedback", "docs/feedback-x/a.eadl"),
        "a prefix matched inside a segment, so `docs/feedback-x` would be excluded by `docs/feedback`"
    );
    assert!(
        !excluded_by("docs/feedback/issues", "docs/feedback/a.eadl"),
        "a pattern longer than the path matched"
    );
    assert!(!excluded_by("", "docs/feedback/a.eadl"));
    assert!(
        !excluded_by("docs/*", "docs/feedback/a.eadl"),
        "`*` is not part of the grammar, so a pattern using it matches nothing — which is what makes a \
         glob in the manifest a leg failure rather than a silent widening"
    );
}

// ── RED arms ───────────────────────────────────────────────────────────────────────────────────
//
// Each arm feeds a rule a violation and asserts the specific complaint, with the count pinned: an arm
// that only checks "some complaint mentions X" also passes on a leg that started reporting everything,
// which is the vacuous-pass shape `conformance.rs` refuses for the grammar. Every mutation is confirmed
// applied before the verdict is read.

#[test]
fn arm_1_a_root_that_reaches_an_excluded_path_is_reported_not_skipped() {
    // Rule 2's arm, against the real tree and a synthetic manifest: `docs` is a root that contains both
    // the suite and the excluded feedback area, so it must be refused rather than quietly filtered.
    let manifest = Manifest {
        version: EADL_1.to_string(),
        means: "a synthetic manifest for one arm".to_string(),
        roots: vec![common::suite::Root {
            path: "docs".to_string(),
            proves: "a root wide enough to reach the excluded area, which is the defect"
                .to_string(),
        }],
        exclusions: Manifest::declared().exclusions,
    };
    let (found, problems) = under_roots(&manifest, &repo_root());
    assert!(
        problems.iter().any(|problem| problem.contains("excludes")),
        "a root reaching an excluded path was not reported: {problems:?}"
    );
    assert!(
        found.iter().all(|path| !path.starts_with("docs/feedback/")),
        "the excluded area entered the population anyway: {} file(s)",
        found
            .iter()
            .filter(|path| path.starts_with("docs/feedback/"))
            .count()
    );
    assert!(
        problems.iter().any(|problem| problem.contains("violation")),
        "the complaint must say what the manifest's rule is, not only that something is wrong: \
         {problems:?}"
    );
}

#[test]
fn arm_2_a_manifest_whose_version_table_is_missing_or_doubled_is_reported() {
    // The parse leg's arms: a manifest that declares no version is not a suite for an unstated version,
    // and one that declares two is two suites in one file.
    let no_table = MANIFEST.replace(
        "<!-- machine-read: suite-version -->",
        "<!-- suite-version -->",
    );
    assert_ne!(
        no_table, MANIFEST,
        "the mutation did not apply — a false green"
    );
    let problems = Manifest::parse(&no_table).expect_err("a manifest with no version table");
    assert_eq!(problems.len(), 1, "{}", problems.join("\n"));
    assert!(
        problems[0].contains("suite-version"),
        "the complaint must name the missing table: {}",
        problems[0]
    );

    let doubled = MANIFEST.replace(
        "| `eadl/1` | every description below reads cleanly",
        "| `eadl/2` | a second version row\n| `eadl/1` | every description below reads cleanly",
    );
    assert_ne!(
        doubled, MANIFEST,
        "the mutation did not apply — a false green"
    );
    let problems = Manifest::parse(&doubled).expect_err("a manifest with two version rows");
    assert_eq!(problems.len(), 1, "{}", problems.join("\n"));
    assert!(
        problems[0].contains("2 rows"),
        "the complaint must say how many versions were declared: {}",
        problems[0]
    );
}

#[test]
fn arm_3_a_root_row_that_proves_nothing_is_reported_rather_than_skipped() {
    // A root with an empty second cell is a directory nobody can say why it is in the suite, and a
    // parser that skipped the row would narrow the suite silently. The row is replaced whole rather than
    // partially: an em dash in the middle of the cell means a substring deletion leaves text behind, and
    // an arm whose mutation did not create the defect it tests is a false green.
    let row = MANIFEST
        .lines()
        .find(|line| line.starts_with("| `examples` |"))
        .expect("the manifest declares an `examples` root")
        .to_string();
    let without_proof = MANIFEST.replace(&row, "| `examples` |  |");
    assert_ne!(
        without_proof, MANIFEST,
        "the mutation did not apply — a false green"
    );
    let problems = Manifest::parse(&without_proof).expect_err("a root that proves nothing");
    assert_eq!(problems.len(), 1, "{}", problems.join("\n"));
    assert!(
        problems[0].contains("one of the two is empty"),
        "an empty cell was skipped rather than reported: {problems:?}"
    );
}

#[test]
fn arm_4_an_exclusion_that_matches_nothing_is_visible_as_vacuous() {
    // The non-vacuity leg's arm: a mistyped prefix reports zero matches, and zero is what the leg
    // refuses. Without this arm the leg could pass on a census that never looked.
    let manifest = Manifest {
        version: EADL_1.to_string(),
        means: "a synthetic manifest for one arm".to_string(),
        roots: Manifest::declared().roots,
        exclusions: vec![common::suite::Exclusion {
            prefix: "docs/feeback".to_string(),
            reason: "a mistyped prefix, which is the defect this arm exists to make visible"
                .to_string(),
        }],
    };
    let matched = vacuous_exclusion_violations(&manifest, &repository_descriptions(&repo_root()));
    assert_eq!(
        matched,
        vec![("docs/feeback".to_string(), 0)],
        "{matched:?}"
    );
}

#[test]
fn arm_5_a_description_outside_every_root_is_reported_by_name() {
    // Rule 4's arm: the walk is synthetic, so the arm does not depend on a stray file existing.
    let manifest = Manifest::declared();
    let suite = vec!["docs/semantics/cases/a.eadl".to_string()];
    let repository = vec![
        "docs/semantics/cases/a.eadl".to_string(),
        "docs/feedback/x.eadl".to_string(),
        "catalog/new-family/device.eadl".to_string(),
    ];
    let wrong = outside_violations(&manifest, &suite, &repository);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n"));
    assert!(
        wrong[0].contains("catalog/new-family/device.eadl"),
        "the complaint must name the description nobody walks: {}",
        wrong[0]
    );
}

#[test]
fn arm_6_a_copy_of_a_frozen_reproducer_is_reported_and_a_different_file_is_not() {
    // Rule 3's arm, both halves: an arm that only checks the positive also passes on a leg that reports
    // every pair.
    let suite = vec![
        (
            "examples/copy.eadl".to_string(),
            "(defsystem s)\n".to_string(),
        ),
        (
            "examples/own.eadl".to_string(),
            "(defsystem other)\n".to_string(),
        ),
    ];
    let excluded = vec![(
        "docs/feedback/linkedspec/issues/LS-002/evidence/system.eadl".to_string(),
        "(defsystem s)\n".to_string(),
    )];
    let wrong = identity_violations(&suite, &excluded);
    assert_eq!(wrong.len(), 1, "{}", wrong.join("\n"));
    assert!(
        wrong[0].contains("examples/copy.eadl") && !wrong[0].contains("examples/own.eadl"),
        "the complaint must name the duplicate and only the duplicate: {}",
        wrong[0]
    );
}

#[test]
fn arm_7_a_manifest_that_declares_no_population_is_reported_rather_than_enumerated() {
    // The empty-suite arm: a root that exists and holds nothing is indistinguishable from a root that
    // was never walked, so both are violations rather than an empty green population.
    let manifest = Manifest {
        version: EADL_1.to_string(),
        means: "a synthetic manifest for one arm".to_string(),
        roots: vec![common::suite::Root {
            path: "docs/semantics/kinds".to_string(),
            proves:
                "a real root, so the arm measures the empty-population rule and not a missing one"
                    .to_string(),
        }],
        exclusions: vec![common::suite::Exclusion {
            prefix: "docs/semantics/kinds".to_string(),
            reason: "excluding the only root, which leaves the manifest declaring nothing"
                .to_string(),
        }],
    };
    let problems =
        enumerate(&manifest, &repo_root()).expect_err("a manifest with an empty population");
    assert!(
        problems
            .iter()
            .any(|problem| problem.contains("violation rather than a skip")),
        "the excluded root was skipped quietly instead of reported: {problems:?}"
    );
}
