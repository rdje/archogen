//! **F04** — a relevant capability is undescribed → `missing-fact`.
//! **F05** — an irrelevant capability is undescribed → the unrelated system stays admissible.
//! **F06** — contradictory offered/absent declarations → `invalid-description`.
//!
//! `ROADMAP.md` §13.1, first gate M1.
//!
//! F04 and F05 are two halves of one decision, and that is why they are tested together in one
//! file against the **same** description. §2's correction table records the revision:
//!
//! > | Unknown capability anywhere blocks generation | Required facts in the selected dependency
//! > closure must be known | Irrelevant unknown facts do not invalidate unrelated systems |
//!
//! A checker that blocks on every unknown passes F04 and fails F05. A checker that blocks on
//! nothing passes F05 and fails F04. Only relevance passes both, so the fixture is built so that
//! *one description* contains both an unknown fact that matters and an unknown fact that does
//! not — a suite that used two different descriptions could be passed by two different bugs.

use eadl_front::{read, SourceMap};
use eadl_model::presence::{FactMap, PresenceReport};

fn check(text: &str) -> (PresenceReport, String) {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "fixture did not read:\n{}",
        diagnostics.render(&sources)
    );
    let mut map = FactMap::new();
    for form in &document.forms {
        map.collect(form);
    }
    let report = map.check();
    let rendered = report
        .diagnostics
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    (report, rendered)
}

/// One platform, one workload. `counter-width` is needed and offered. `wrap-behavior` is needed
/// and **undescribed** — F04. `dma-channels` is undescribed and irrelevant — F05.
const MIXED: &str = r"
(defsystem app.rt
  (uses time.monotonic))

(defservice time.monotonic
  (needs counter-width wrap-behavior))

(defblock timer.counter
  (offers counter-width))

(defblock unrelated.dma
  (needs dma-channels))
";

#[test]
fn f04_an_undescribed_fact_inside_the_closure_is_a_missing_fact() {
    let (report, rendered) = check(MIXED);
    assert!(
        report.missing.contains(&"wrap-behavior".to_string()),
        "missing = {:?}",
        report.missing
    );
    assert!(rendered.contains("missing-fact"), "{rendered}");
    assert!(
        rendered.contains("`wrap-behavior` is required by this system"),
        "{rendered}"
    );
    assert!(
        rendered.contains("inside the dependency closure"),
        "the refusal must say WHY this one matters:\n{rendered}"
    );
}

#[test]
fn f05_an_undescribed_fact_outside_the_closure_does_not_fail_the_system() {
    // ⭐ Same description as F04. `dma-channels` is just as undescribed as `wrap-behavior`, and
    // must produce no diagnostic at all — only a metadata entry.
    let (report, rendered) = check(MIXED);
    assert!(
        !report.missing.contains(&"dma-channels".to_string()),
        "an irrelevant unknown was reported as missing: {:?}",
        report.missing
    );
    assert!(
        !rendered.contains("dma-channels"),
        "an irrelevant unknown produced a diagnostic:\n{rendered}"
    );
    assert!(
        report.outside_closure.contains(&"dma-channels".to_string()),
        "§5.3 requires it to remain VISIBLE in metadata: outside = {:?}",
        report.outside_closure
    );
}

#[test]
fn f05_a_system_touching_nothing_unknown_is_admissible() {
    let (report, rendered) = check(
        r"
        (defsystem app.rt (uses time.monotonic))
        (defservice time.monotonic (needs counter-width))
        (defblock timer.counter (offers counter-width))
        (defblock unrelated.dma (needs dma-channels))
        ",
    );
    assert!(
        report.is_admissible(),
        "an unrelated unknown invalidated the system:\n{rendered}"
    );
    assert!(report.outside_closure.contains(&"dma-channels".to_string()));
}

#[test]
fn f04_and_f05_cannot_both_be_satisfied_by_a_constant_answer() {
    // The property that makes the pair meaningful: on ONE description, one unknown is reported
    // and another is not. A checker that always blocks, or never blocks, fails this.
    let (report, _) = check(MIXED);
    assert_eq!(report.missing, vec!["wrap-behavior".to_string()]);
    assert!(report.outside_closure.contains(&"dma-channels".to_string()));
    assert!(!report.is_admissible());
}

#[test]
fn f06_a_fact_both_offered_and_absent_is_an_invalid_description() {
    let (report, rendered) = check(
        r"
        (defsystem app.rt (uses time.monotonic))
        (defservice time.monotonic (needs low-power-timer))
        (defblock timer.a (offers low-power-timer))
        (defblock timer.b (absent low-power-timer))
        ",
    );
    assert!(!report.is_admissible());
    assert!(rendered.contains("invalid-description"), "{rendered}");
    assert!(
        rendered.contains("both offered and absent"),
        "{rendered}"
    );
    // Both sides must be named: the author looking at one cannot see the other.
    assert!(rendered.contains("declared absent by `timer.b`"), "{rendered}");
    assert!(rendered.contains("declared offered by `timer.a`"), "{rendered}");
    assert!(
        rendered.contains("only the author knows which half was meant"),
        "the refusal must say why it is not resolved by preference:\n{rendered}"
    );
}

#[test]
fn f06_a_contradiction_outside_the_closure_is_still_a_contradiction() {
    // A contradictory description is invalid whether or not the contradiction is reachable.
    // §5.3 says to reject contradictory declarations, without a relevance qualifier — unlike
    // unknown facts, where relevance is explicitly the rule.
    let (report, rendered) = check(
        r"
        (defsystem app.rt (uses time.monotonic))
        (defservice time.monotonic (needs counter-width))
        (defblock timer.counter (offers counter-width))
        (defblock a (offers never-used))
        (defblock b (absent never-used))
        ",
    );
    assert!(!report.is_admissible());
    assert!(rendered.contains("invalid-description"), "{rendered}");
    assert!(rendered.contains("never-used"), "{rendered}");
}

#[test]
fn an_explicitly_absent_required_fact_is_infeasible_not_missing() {
    // ⭐ The distinction absent-vs-undescribed exists for exactly this. An absent fact is a
    // definite answer that the requirement contradicts; an undescribed one is a gap. Reporting
    // both as `missing-fact` would tell an author to go and describe something the platform has
    // already said it does not have.
    let (report, rendered) = check(
        r"
        (defsystem app.rt (uses time.monotonic))
        (defservice time.monotonic (needs low-power-timer))
        (defblock timer.counter (absent low-power-timer))
        ",
    );
    assert!(!report.is_admissible());
    assert!(rendered.contains("infeasible-configuration"), "{rendered}");
    assert!(!rendered.contains("missing-fact"), "{rendered}");
    assert!(
        rendered.contains("not a gap to be filled in"),
        "{rendered}"
    );
}

#[test]
fn the_closure_is_transitive_and_the_report_lists_it() {
    let (report, _) = check(
        r"
        (defsystem app.rt (uses time.deadline))
        (defservice time.deadline (needs time.monotonic compare-unit))
        (defservice time.monotonic (needs counter-width))
        (defblock t (offers compare-unit counter-width))
        ",
    );
    assert!(report.is_admissible(), "{:?}", report.diagnostics);
    for name in ["time.deadline", "time.monotonic", "compare-unit", "counter-width"] {
        assert!(
            report.closure.contains(&name.to_string()),
            "`{name}` missing from closure {:?}",
            report.closure
        );
    }
}

#[test]
fn a_needs_edge_nested_inside_a_requires_clause_is_still_followed() {
    // Authors write `needs` where it reads best. A closure that only looked at top-level
    // clauses would miss most real descriptions and would fail OPEN — the dangerous direction.
    let (report, _) = check(
        r"
        (defsystem app.rt (requires (uses time.monotonic)))
        (defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s)) (needs wrap-behavior)))
        ",
    );
    assert!(
        report.missing.contains(&"wrap-behavior".to_string()),
        "a nested needs edge was not followed: closure = {:?}",
        report.closure
    );
}

#[test]
fn declaring_a_fact_offered_is_a_claim_and_not_evidence() {
    // §5.3: "'Offered' records a claim whose evidential status is separate; declaring it does
    // not make it proven." This module's job ends at presence; nothing here may be read as
    // saying the platform really has the capability.
    let (report, _) = check(
        r"
        (defsystem app.rt (uses time.monotonic))
        (defservice time.monotonic (needs counter-width))
        (defblock t (offers counter-width))
        ",
    );
    assert!(report.is_admissible());
    // The report carries no evidence field at all — by construction, there is nothing here to
    // mistake for one.
    assert!(report.missing.is_empty());
}
