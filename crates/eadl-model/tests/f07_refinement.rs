//! **F07** — an invalid functional refinement names the violated obligation.
//!
//! `ROADMAP.md` §13.1, first gate M1. The word doing the work is **named**: "this refinement is
//! invalid" sends an author to re-read two descriptions and guess; "it drops the guarantee
//! `counter-width`, which `soc.abstract` offers" sends them to one line.
//!
//! Two roadmap sentences pull against each other here, and the fixture tests both directions:
//!
//! * §5.1.1 — "a refinement declaration is an obligation to check, not permission to trust a
//!   claim blindly";
//! * §5.3 — "adding an unused device is **not** automatically an invalid refinement".
//!
//! A checker that demanded equality would reject every real refinement. A checker that only
//! compared what both mention would miss the case that matters most: an abstract description
//! that declares something absent *because* a guarantee depends on its absence.

use eadl_front::{read, SourceMap};
use eadl_model::refinement::{check, Facets, Obligation, RefinementReport};

fn refine(text: &str) -> (RefinementReport, String) {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "fixture did not read:\n{}",
        diagnostics.render(&sources)
    );
    let facets: Vec<Facets> = document.forms.iter().map(Facets::of).collect();
    assert_eq!(facets.len(), 2, "a refinement fixture is two declarations");
    let report = check(&facets[0], &facets[1]);
    let rendered = report
        .diagnostics()
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    (report, rendered)
}

fn violated(report: &RefinementReport) -> Vec<Obligation> {
    report.violations.iter().map(|(o, _)| *o).collect()
}

#[test]
fn f07_a_refinement_that_drops_a_guarantee_names_the_guarantee_obligation() {
    let (report, rendered) = refine(
        "(defplatform soc.abstract (offers counter-width wrap-behavior))\n\
         (defplatform soc.concrete (refines soc.abstract) (offers counter-width))",
    );
    assert!(!report.is_valid());
    assert_eq!(violated(&report), vec![Obligation::Guarantee]);
    assert!(rendered.contains("refinement-violated"), "{rendered}");
    assert!(
        rendered.contains("does not offer `wrap-behavior`"),
        "the message must name the dropped fact:\n{rendered}"
    );
    assert!(
        rendered.contains("violated obligation `guarantee`"),
        "the obligation must be named:\n{rendered}"
    );
    assert!(rendered.contains("guaranteed here"), "both sites:\n{rendered}");
}

#[test]
fn f07_a_refinement_that_violates_a_bound_names_the_constraint_obligation() {
    let (report, rendered) = refine(
        "(defplatform soc.abstract (offers (counter-width (at-least 32 bit))))\n\
         (defplatform soc.concrete (refines soc.abstract) (offers (counter-width 16 bit)))",
    );
    assert!(!report.is_valid());
    assert_eq!(violated(&report), vec![Obligation::Constraint]);
    assert!(
        rendered.contains("does not satisfy `at-least 32 bit`"),
        "{rendered}"
    );
    assert!(rendered.contains("violated obligation `constraint`"), "{rendered}");
}

#[test]
fn f07_a_refinement_that_adds_an_excluded_fact_names_the_exclusion_obligation() {
    // ⭐ The obligation a naive "the concrete may say more" rule silently drops. An abstract
    // description declares a fact absent *because something depends on its absence*.
    let (report, rendered) = refine(
        "(defplatform soc.abstract (offers counter-width) (absent dma))\n\
         (defplatform soc.concrete (refines soc.abstract) (offers counter-width dma))",
    );
    assert!(!report.is_valid());
    assert_eq!(violated(&report), vec![Obligation::Exclusion]);
    assert!(rendered.contains("which `soc.abstract` declares absent"), "{rendered}");
    assert!(rendered.contains("violated obligation `exclusion`"), "{rendered}");
    assert!(
        rendered.contains("not an omission to be filled in"),
        "the refusal must say why an absence is a constraint:\n{rendered}"
    );
}

#[test]
fn adding_an_unused_device_is_not_an_invalid_refinement() {
    // ⭐ §5.3, in as many words. The counterpart to the exclusion test: a checker that refused
    // any addition would pass the exclusion arm and reject every real refinement.
    let (report, rendered) = refine(
        "(defplatform soc.abstract (offers counter-width))\n\
         (defplatform soc.concrete (refines soc.abstract) (offers counter-width uart spi))",
    );
    assert!(report.is_valid(), "an addition was refused:\n{rendered}");
    assert_eq!(report.additions, vec!["spi".to_string(), "uart".to_string()]);
}

#[test]
fn an_addition_is_reported_even_though_it_is_allowed() {
    // Allowed is not the same as invisible: the author should be able to see what grew.
    let (report, _) = refine(
        "(defplatform a (offers x))\n(defplatform b (refines a) (offers x y))",
    );
    assert!(report.is_valid());
    assert_eq!(report.additions, vec!["y".to_string()]);
}

#[test]
fn a_stronger_value_satisfies_an_at_least_bound() {
    let (report, rendered) = refine(
        "(defplatform a (offers (counter-width (at-least 32 bit))))\n\
         (defplatform b (refines a) (offers (counter-width 64 bit)))",
    );
    assert!(report.is_valid(), "{rendered}");
}

#[test]
fn direction_decides_which_way_is_stronger() {
    // ⭐ §5.2: "more bits or a faster clock is not universally better." The same numeric
    // relationship passes one bound and fails the other, which is exactly why the abstract
    // description has to state a direction rather than a bare value.
    let (larger_against_at_most, _) = refine(
        "(defplatform a (offers (delivery-bound (at-most 50 us))))\n\
         (defplatform b (refines a) (offers (delivery-bound 80 us)))",
    );
    assert!(!larger_against_at_most.is_valid(), "80 us must not satisfy at-most 50 us");

    let (smaller_against_at_most, _) = refine(
        "(defplatform a (offers (delivery-bound (at-most 50 us))))\n\
         (defplatform b (refines a) (offers (delivery-bound 20 us)))",
    );
    assert!(smaller_against_at_most.is_valid());
}

#[test]
fn an_exact_bound_refuses_a_merely_better_value() {
    let (report, rendered) = refine(
        "(defplatform a (offers (tick-rate (exactly 10 MHz))))\n\
         (defplatform b (refines a) (offers (tick-rate 20 MHz)))",
    );
    assert!(!report.is_valid(), "an exact bound accepted a different value");
    assert!(rendered.contains("the direction is `exact`"), "{rendered}");
}

#[test]
fn an_exact_bound_accepts_the_same_amount_written_differently() {
    let (report, rendered) = refine(
        "(defplatform a (offers (tick-rate (exactly 10 MHz))))\n\
         (defplatform b (refines a) (offers (tick-rate 10000000 Hz)))",
    );
    assert!(report.is_valid(), "comparison must be on the amount, not the spelling:\n{rendered}");
}

#[test]
fn a_bound_checked_against_the_wrong_dimension_is_refused_not_answered() {
    // Reuses F03's refusal: a cross-dimension comparison is a type error, not a false.
    let (report, rendered) = refine(
        "(defplatform a (offers (horizon (at-least 60 s))))\n\
         (defplatform b (refines a) (offers (horizon 60 MHz)))",
    );
    assert!(!report.is_valid());
    assert!(rendered.contains("measures something else entirely"), "{rendered}");
}

#[test]
fn a_bounded_fact_offered_without_a_value_is_refused() {
    // Offering the name alone satisfies the guarantee but leaves the bound unchecked, which is
    // the shape a refinement would take if someone "fixed" the guarantee violation by adding a
    // bare name.
    let (report, rendered) = refine(
        "(defplatform a (offers (counter-width (at-least 32 bit))))\n\
         (defplatform b (refines a) (offers counter-width))",
    );
    assert!(!report.is_valid());
    assert_eq!(violated(&report), vec![Obligation::Constraint]);
    assert!(rendered.contains("no value to check the bound against"), "{rendered}");
}

#[test]
fn every_violated_obligation_is_reported_not_only_the_first() {
    let (report, _) = refine(
        "(defplatform a (offers dropped (counter-width (at-least 32 bit))) (absent dma))\n\
         (defplatform b (refines a) (offers (counter-width 8 bit) dma))",
    );
    let obligations = violated(&report);
    assert!(obligations.contains(&Obligation::Guarantee), "{obligations:?}");
    assert!(obligations.contains(&Obligation::Constraint), "{obligations:?}");
    assert!(obligations.contains(&Obligation::Exclusion), "{obligations:?}");
}

#[test]
fn every_obligation_states_itself_in_full() {
    // A named obligation is only useful if the name resolves to a sentence.
    for obligation in [
        Obligation::Guarantee,
        Obligation::Constraint,
        Obligation::Exclusion,
    ] {
        assert!(!obligation.slug().is_empty());
        assert!(
            obligation.statement().starts_with("a refinement"),
            "{}",
            obligation.statement()
        );
    }
}
