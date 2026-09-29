//! The frozen-construct baseline: its shape, and the comparator `M1.13.5`'s gate will use.
//!
//! ⛔ **This file writes no gate, deliberately.** `M1.13.4.5` creates `docs/semantics/BASELINE.txt` and
//! the instrument that derives it; `M1.13.5` compares a fresh run against the tracked file, requires a
//! migration note for any movement, and registers the check as a doctrine. A gate written in the same
//! commit as its baseline has no prior state to differ from, so no RED arm can fire against the real
//! tree — which is why the two are separate leaves and not one.
//!
//! What is checked here is therefore the **shape** of the baseline and the **classification** the gate
//! will need: that the tracked file parses, that every digest is a digest, that every construct id is
//! one of the three classes the instrument emits, that each class is present, and that the header names
//! the command that rewrites the file — because amending a baseline must stay an explicit act. The
//! comparator's three classifications each carry an arm over a synthetic pair, which is the part `M1.13.5`
//! cannot test until a real movement exists to differ from.

mod common;

use common::baseline::{
    classes, compare, id_shape_violations, parse, Baseline, Difference, BASELINE, BASELINE_PATH,
};

/// The tracked baseline, parsed — or a panic naming every problem with it.
fn tracked() -> Baseline {
    parse(BASELINE).unwrap_or_else(|problems| {
        panic!(
            "{BASELINE_PATH} is not a readable baseline:\n\n{}",
            problems.join("\n")
        )
    })
}

// ── the green legs: shape, not movement ─────────────────────────────────────────────────────────

#[test]
fn the_tracked_baseline_is_well_formed() {
    let baseline = tracked();
    for (id, digest) in &baseline {
        assert_eq!(
            digest.len(),
            64,
            "the digest of `{id}` is {} characters, and a sha256 is 64",
            digest.len()
        );
    }
    let wrong = id_shape_violations(&baseline);
    assert!(
        wrong.is_empty(),
        "{} construct id(s) belong to no class this reader knows:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}

#[test]
fn the_baseline_covers_all_three_classes_of_frozen_construct() {
    // ⭐ A shape leg and not a coverage count: it asks whether each class is *present*, so a baseline
    // that silently lost a whole class — the instrument's table scan matching nothing, or the grammar's
    // fence renamed — fails here, while a corpus that legitimately grows does not. The counts themselves
    // are `M1.13.5`'s business, because comparing them to a fresh run *is* the gate.
    let baseline = tracked();
    for (class, count) in classes(&baseline) {
        assert!(count > 0, "the baseline freezes no {class}");
    }
}

#[test]
fn the_baseline_header_names_the_act_that_rewrites_it() {
    // ⛔ "Amending the baseline must be an explicit act, never a side effect" — the freeze's second rule,
    // and the header is where a reader meets it. A generated file that does not say how it is generated
    // gets edited by hand, and the first hand-edit is the one that makes the digest mean nothing.
    //
    // ⭐ The needle is the **gate's path**, not a task-tree leaf id. The first cut of this leg asked for
    // `M1.13.5`, and it was right for exactly one commit: a generated artifact that names the leaf which
    // produced it rots the moment that leaf closes, and a rotting pointer in a file nobody edits by hand
    // is worse than none. The tool that enforces the file is the thing worth naming.
    for needle in [
        "GENERATED",
        "do not edit",
        "scripts/language_baseline.sh --emit",
        "limit:",
        "scripts/check_language_freeze.sh",
    ] {
        assert!(
            BASELINE.contains(needle),
            "{BASELINE_PATH}'s header does not say {needle:?}, so a reader cannot tell it is generated, \
             how to regenerate it, what it cannot see, or who gates it"
        );
    }
}

// ── RED arms: the comparator, over synthetic baselines ─────────────────────────────────────────

/// A minimal baseline of the three classes, for the arms to mutate.
fn synthetic() -> Baseline {
    [
        ("docs/semantics/grammar.md#ebnf".to_string(), "a".repeat(64)),
        (
            "docs/semantics/reference.md#number-values".to_string(),
            "b".repeat(64),
        ),
        (
            "suite/examples/s0-heartbeat/system.eadl".to_string(),
            "c".repeat(64),
        ),
    ]
    .into_iter()
    .collect()
}

#[test]
fn arm_1_a_moved_digest_is_classified_as_moved_and_names_both() {
    // The classification the gate's migration note has to answer to: not "a line differed" but "this
    // construct moved, from this digest to that one".
    let tracked = synthetic();
    let mut fresh = synthetic();
    fresh.insert(
        "suite/examples/s0-heartbeat/system.eadl".to_string(),
        "d".repeat(64),
    );
    let differences = compare(&fresh, &tracked);
    assert_eq!(differences.len(), 1, "{differences:?}");
    assert_eq!(
        differences[0],
        Difference::Moved {
            id: "suite/examples/s0-heartbeat/system.eadl".to_string(),
            tracked: "c".repeat(64),
            fresh: "d".repeat(64),
        }
    );
    let rendered = differences[0].render();
    assert!(
        rendered.contains("moved") && rendered.contains("system.eadl"),
        "the report must name the construct and the class of movement: {rendered}"
    );
    assert_eq!(differences[0].kind(), "moved");
}

#[test]
fn arm_2_an_added_construct_is_classified_as_added() {
    // A language that grew: a new machine-read table, or a new description in the suite.
    let tracked = synthetic();
    let mut fresh = synthetic();
    fresh.insert(
        "docs/semantics/reference.md#language-version".to_string(),
        "e".repeat(64),
    );
    let differences = compare(&fresh, &tracked);
    assert_eq!(differences.len(), 1, "{differences:?}");
    assert_eq!(differences[0].kind(), "added");
    assert!(
        differences[0].render().contains("language-version"),
        "the report must name the construct that appeared: {}",
        differences[0].render()
    );
}

#[test]
fn arm_3_a_removed_construct_is_classified_as_removed() {
    // The class a plain `diff` reads identically to a moved one: a line went away. It is not the same
    // claim — nothing is frozen any more — and the gate has to say so.
    let tracked = synthetic();
    let mut fresh = synthetic();
    fresh.remove("docs/semantics/grammar.md#ebnf");
    let differences = compare(&fresh, &tracked);
    assert_eq!(differences.len(), 1, "{differences:?}");
    assert_eq!(differences[0].kind(), "removed");
    assert!(
        differences[0].render().contains("no longer exists"),
        "the report must say the construct is gone rather than changed: {}",
        differences[0].render()
    );
}

#[test]
fn arm_4_a_row_nothing_can_parse_is_reported_rather_than_skipped() {
    // ⛔ The arm behind "a malformed row is a violation and never a row to skip": a parser that dropped
    // what it could not read would freeze less than the file says it does, and every gate above it would
    // report green.
    for (row, needle) in [
        ("not a row at all", "is not `<sha256>  <construct id>`"),
        (&format!("{}  some#id", "a".repeat(63)), "64-character"),
        (&format!("{}  some#id", "z".repeat(64)), "64-character"),
        (&format!("{}  some#id", "A".repeat(64)), "uppercase"),
        (&format!("{}  ", "a".repeat(64)), "names no construct"),
    ] {
        let problems = parse(row).expect_err("a malformed row must be refused");
        assert!(
            problems.iter().any(|problem| problem.contains(needle)),
            "row {row:?} was refused for the wrong reason:\n{}",
            problems.join("\n")
        );
    }
}

#[test]
fn arm_5_an_unsorted_or_duplicated_baseline_is_reported_as_hand_edited() {
    // The file is generated sorted by id so that two runs are byte-comparable; an unsorted or duplicated
    // row is evidence somebody edited it, which is the act `M1.13.5` requires a migration note for.
    let unsorted = concat!(
        "# header\n",
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb  zzz#id\n",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa  aaa#id\n",
    );
    let problems = parse(unsorted).expect_err("an unsorted baseline");
    assert!(
        problems.iter().any(|problem| problem.contains("sorted")),
        "an unsorted row was accepted: {problems:?}"
    );

    let digest = "a".repeat(64);
    let duplicated = format!("# header\n{digest}  same#id\n{digest}  same#id\n");
    let problems = parse(&duplicated).expect_err("a duplicated id");
    assert!(
        problems.iter().any(|problem| problem.contains("twice")),
        "a duplicated construct id was accepted: {problems:?}"
    );
}

#[test]
fn arm_6_an_empty_baseline_is_reported_rather_than_passed() {
    // A baseline freezing nothing is the vacuous green every other leg here refuses: a gate comparing
    // against it passes on any language at all.
    let problems = parse("# only a header\n").expect_err("an empty baseline");
    assert_eq!(problems.len(), 1, "{}", problems.join("\n"));
    assert!(
        problems[0].contains("freezes no construct"),
        "the complaint must say the baseline is empty: {}",
        problems[0]
    );
}
