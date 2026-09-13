//! **F18** — the §13.2 numerical baseline, read from `ROADMAP.md` itself.
//!
//! > | Task | Computation C | Minimum separation T | Deadline D | Expected response bound |
//! > | A | 1 | 4 | 4 | 1 |
//! > | B | 1 | 5 | 5 | 2 |
//! > | C | 2 | 10 | 10 | 4 |
//!
//! ⭐ **The oracle is parsed out of the roadmap, not copied into this file**, and that is the
//! whole design of the test. §14.1 forbids one thing in particular:
//!
//! > Implementation changes cannot silently weaken requirements, **adjust expected oracle
//! > results**, or drop failing scenarios.
//!
//! A test carrying its own copy of the numbers makes that failure a one-line edit that looks like
//! a fix. Reading `ROADMAP.md` §13.2 means the expectation and the specification are the same
//! object: changing the answer means editing the roadmap, in a diff a reviewer sees as a changed
//! requirement. It is the same trick `crates/eadl-model/src/profile.rs` uses against the
//! published profile page.
//!
//! §13.2 supplies no priority column — "Higher priority is listed first" — so the row order *is*
//! the priority, which the parser preserves and
//! `docs/decisions/decision_priority-comparison-direction.md` makes unambiguous.

use std::path::{Path, PathBuf};

use archogen_evidence::claim::Conclusion;
use rt_analysis::model::{Task, TaskSet};
use rt_analysis::response::{analyze, conclusion, Outcome};

/// One row of §13.2.
struct Row {
    name: String,
    computation: u64,
    separation: u64,
    deadline: u64,
    expected: u64,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Read §13.2's table out of `ROADMAP.md`.
///
/// Deliberately strict: an unparsable or resized table fails loudly rather than silently
/// analyzing fewer tasks than the baseline specifies. A baseline that quietly shrank would still
/// be green, which is the failure this whole file exists to prevent.
fn baseline() -> Vec<Row> {
    let text = std::fs::read_to_string(repo_root().join("ROADMAP.md")).expect("the roadmap");
    let section = text
        .split_once("### 13.2 Numerical baseline for the scheduling checker")
        .expect("§13.2 exists")
        .1;
    let section = section
        .split_once("\n### ")
        .map_or(section, |(head, _)| head);

    let rows: Vec<Row> = section
        .lines()
        .filter_map(|line| {
            let cells: Vec<&str> = line
                .trim()
                .strip_prefix('|')?
                .strip_suffix('|')?
                .split('|')
                .map(str::trim)
                .collect();
            if cells.len() != 5 {
                return None;
            }
            let numbers: Option<Vec<u64>> = cells[1..].iter().map(|c| c.parse().ok()).collect();
            let numbers = numbers?;
            Some(Row {
                name: cells[0].to_string(),
                computation: numbers[0],
                separation: numbers[1],
                deadline: numbers[2],
                expected: numbers[3],
            })
        })
        .collect();

    assert_eq!(
        rows.len(),
        3,
        "§13.2 declares three tasks; the parser found {} — the table changed shape, and a \
         baseline that quietly shrank would still be green",
        rows.len()
    );
    rows
}

/// The baseline as an admitted task set. Row order is priority order (§13.2).
fn task_set(rows: &[Row], deadline_override: Option<(&str, u64)>) -> TaskSet {
    let tasks = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let deadline = match deadline_override {
                Some((name, value)) if name == row.name => value,
                _ => row.deadline,
            };
            Task {
                id: row.name.clone(),
                priority: i64::try_from(index).expect("three rows") + 1,
                computation: row.computation,
                separation: row.separation,
                deadline,
            }
        })
        .collect();
    TaskSet::admit(tasks).expect("§13.2's baseline is inside the admitted model")
}

#[test]
fn f18_the_baseline_yields_the_response_bounds_the_roadmap_states() {
    let rows = baseline();
    let outcomes = analyze(&task_set(&rows, None));
    assert_eq!(outcomes.len(), rows.len());
    for (row, outcome) in rows.iter().zip(&outcomes) {
        let Outcome::MeetsDeadline { response, .. } = outcome else {
            panic!(
                "§13.2: \"The system passes this model's test\", but `{}` gave {outcome:?}",
                row.name
            );
        };
        assert_eq!(
            *response,
            row.expected,
            "task `{}`: §13.2 states an expected response bound of {}, the recurrence gave {} \
             (witness {})",
            row.name,
            row.expected,
            response,
            outcome.witness().render()
        );
    }
}

#[test]
fn f18_the_witness_for_c_is_the_sequence_the_roadmap_describes() {
    // §13.2: "For C, the iteration starts at 2 and reaches 4, then remains 4." The prose names
    // the sequence, not just the answer, so the witness is checked against it — an
    // implementation that arrived at 4 by a different route would be a different theorem.
    let rows = baseline();
    let outcomes = analyze(&task_set(&rows, None));
    let c = outcomes
        .iter()
        .find(|o| o.task() == "C")
        .expect("§13.2 declares a task C");
    assert_eq!(
        c.witness().sequence,
        vec![2, 4, 4],
        "{}",
        c.witness().render()
    );
}

#[test]
fn f18_the_baseline_concludes_conditionally_and_never_bare() {
    // §7.1's permitted conclusion for a conditional analysis is "deadlines follow in the declared
    // model under listed assumptions". There is no variant that says more.
    let outcomes = analyze(&task_set(&baseline(), None));
    let Conclusion::HoldsUnderAssumptions { model, assumptions } = conclusion(&outcomes) else {
        panic!("a passing baseline must conclude conditionally");
    };
    assert!(model.contains("§7.4"), "{model}");
    assert!(
        assumptions.iter().any(|a| a.contains("no overhead")),
        "the assumption §7.4 warns about most loudly must travel with the conclusion"
    );
}

#[test]
fn f18_changing_only_cs_deadline_to_three_produces_a_violation_witness() {
    // §13.2: "Change only C's deadline to 3: under synchronous releases, higher-priority work
    // occupies the first two time units and C finishes at 4, providing a concrete
    // deadline-violation witness in this model."
    let rows = baseline();
    let outcomes = analyze(&task_set(&rows, Some(("C", 3))));

    // A and B are untouched — "change ONLY C's deadline".
    for name in ["A", "B"] {
        let outcome = outcomes.iter().find(|o| o.task() == name).expect("present");
        assert!(
            matches!(outcome, Outcome::MeetsDeadline { .. }),
            "`{name}` should be unaffected: {outcome:?}"
        );
    }

    let c = outcomes.iter().find(|o| o.task() == "C").expect("present");
    let Outcome::MissesDeadline {
        response,
        deadline,
        witness,
    } = c
    else {
        panic!("C must miss a deadline of 3: {c:?}");
    };
    assert_eq!(*response, 4, "C finishes at 4");
    assert_eq!(*deadline, 3);
    assert_eq!(witness.sequence, vec![2, 4, 4]);

    // ⭐ The witness is a counterexample, not a bare failure. §7.4: a conservative failure is
    // `not-established`; only an exact test or a validated witness establishes a violation, and
    // this one carries the sequence a reader can re-derive by hand.
    let Conclusion::Counterexample { witness: rendered } = conclusion(&outcomes) else {
        panic!("a converged bound past the deadline is a counterexample, not an inconclusive");
    };
    assert!(rendered.contains("C: 2 → 4 → 4"), "{rendered}");
    assert!(rendered.contains("deadline of 3"), "{rendered}");
}

#[test]
fn f18_the_expected_bounds_are_read_from_the_roadmap_and_not_from_this_file() {
    // The property that makes the oracle un-adjustable: the numbers this suite checks against are
    // the ones §13.2 publishes. If the table is edited, this test's expectations move with it —
    // in a diff a reviewer reads as a changed requirement, which §14.1 requires.
    let rows = baseline();
    let published: Vec<(String, u64)> = rows
        .iter()
        .map(|row| (row.name.clone(), row.expected))
        .collect();
    assert_eq!(
        published,
        vec![
            ("A".to_string(), 1),
            ("B".to_string(), 2),
            ("C".to_string(), 4)
        ],
        "the roadmap's §13.2 table no longer states the baseline this analysis was built against"
    );
}
