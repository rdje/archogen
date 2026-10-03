//! **F03** — zero clock frequency or incompatible units.
//!
//! `ROADMAP.md` §13.1: *"Zero clock frequency or incompatible units → type/constraint error
//! **before arithmetic**."* First gate M1.
//!
//! The phrase that carries the weight is "before arithmetic". It is not enough to produce an
//! error eventually — a division by a zero clock that happens to be caught downstream has
//! already been performed, and a comparison between a frequency and a duration that returns a
//! number has already produced a wrong answer that something may have acted on.
//!
//! So the fixture tests the *structure* of the refusal as well as its existence:
//!
//! * a non-positive frequency is refused at **construction**, so no such quantity exists to
//!   divide by;
//! * a cross-dimension comparison is refused **without reading either magnitude**, which is
//!   demonstrated by refusing a pair whose magnitudes would overflow if they were touched.

use eadl_front::{read, SourceMap};
use eadl_model::quantity::{unit, ComparisonDirection, Dimension, Quantity, QuantityError};
use eadl_model::rational::Rational;

fn q(value: i64, unit_name: &str) -> Quantity {
    Quantity::new(
        Rational::integer(value),
        unit(unit_name).unwrap_or_else(|| panic!("`{unit_name}` should be a known unit")),
    )
    .unwrap_or_else(|e| panic!("{value} {unit_name} should be valid: {e}"))
}

/// Read `(x <number> <unit>)` and return the quantity or the rendered diagnostic.
fn read_quantity(text: &str) -> Result<Quantity, String> {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", format!("(x {text})")).expect("small");
    let (document, diagnostics) = read(&sources, id);
    if diagnostics.has_errors() {
        return Err(diagnostics.render(&sources));
    }
    let items = document.forms[0].items().to_vec();
    Quantity::read(items.get(1), items.get(2), document.forms[0].span())
        .map_err(|d| d.render(&sources))
}

#[test]
fn f03_a_zero_clock_frequency_is_refused_at_construction() {
    // ⭐ Not "caught later": there is no zero-frequency Quantity to divide by, because the
    // constructor never produces one.
    let error = Quantity::new(Rational::ZERO, unit("MHz").unwrap()).expect_err("must refuse");
    assert_eq!(error, QuantityError::NonPositiveFrequency);

    let rendered = read_quantity("0 MHz").expect_err("must refuse");
    assert!(
        rendered.contains("quantity-non-positive-frequency"),
        "{rendered}"
    );
    assert!(
        rendered.contains("a frequency must be strictly positive"),
        "{rendered}"
    );
    // The repair must say why, not just what.
    assert!(
        rendered.contains("divides by it"),
        "the refusal should explain the consequence: {rendered}"
    );
}

#[test]
fn f03_a_negative_frequency_is_refused_too() {
    assert_eq!(
        Quantity::new(Rational::integer(-1), unit("Hz").unwrap()).expect_err("must refuse"),
        QuantityError::NonPositiveFrequency
    );
    assert!(read_quantity("-5 kHz").is_err());
}

#[test]
fn f03_incompatible_units_are_refused_before_any_magnitude_is_read() {
    // ⭐ The "before arithmetic" property, demonstrated rather than asserted. Both magnitudes
    // are at the edge of the representable range: converting either to its base unit would
    // overflow. The comparison still returns the DIMENSION error, which is only possible if the
    // dimension check happens first.
    let huge_time = Quantity::new(Rational::new(i128::MAX, 1).unwrap(), unit("s").unwrap())
        .expect("a large duration is legal");
    let huge_frequency = Quantity::new(Rational::new(i128::MAX, 1).unwrap(), unit("GHz").unwrap())
        .expect("a large frequency is legal");

    // Sanity: touching the magnitudes really would overflow.
    assert_eq!(huge_frequency.in_base(), Err(QuantityError::Overflow));

    match huge_time.compare(huge_frequency) {
        Err(QuantityError::IncompatibleDimensions { left, right, .. }) => {
            assert_eq!(left, Dimension::Time);
            assert_eq!(right, Dimension::Frequency);
        }
        other => panic!("expected a dimension error before arithmetic, got {other:?}"),
    }
}

#[test]
fn f03_the_incompatibility_message_names_both_sides() {
    let error = q(10, "ms").compare(q(1, "KiB")).expect_err("must refuse");
    let text = error.to_string();
    assert!(text.contains("`ms` measures time"), "{text}");
    assert!(text.contains("`KiB` measures information"), "{text}");
    assert!(text.contains("cannot be compared"), "{text}");
}

#[test]
fn f03_conversion_across_dimensions_is_refused() {
    let error = q(10, "ms")
        .convert_to(unit("MHz").unwrap())
        .expect_err("must refuse");
    assert!(matches!(
        error,
        QuantityError::IncompatibleDimensions { .. }
    ));
}

#[test]
fn an_unknown_unit_lists_the_known_ones() {
    let rendered = read_quantity("10 jiffies").expect_err("must refuse");
    assert!(rendered.contains("quantity-unknown-unit"), "{rendered}");
    assert!(rendered.contains("`ms`"), "{rendered}");
}

#[test]
fn a_bare_number_is_not_a_quantity() {
    let rendered = read_quantity("10").expect_err("must refuse");
    assert!(rendered.contains("quantity-missing-unit"), "{rendered}");
    assert!(
        rendered.contains("nothing says what it measures"),
        "the refusal should explain why a bare number cannot be compared: {rendered}"
    );
}

#[test]
fn conversion_within_a_dimension_is_exact() {
    assert!(q(1, "s").equals(q(1000, "ms")).expect("same dimension"));
    assert!(q(1, "ms")
        .equals(q(1_000_000, "ns"))
        .expect("same dimension"));
    assert!(q(1, "MHz")
        .equals(q(1_000_000, "Hz"))
        .expect("same dimension"));
    assert!(q(1, "KiB").equals(q(8192, "bit")).expect("same dimension"));
    assert!(q(1, "byte").equals(q(8, "bit")).expect("same dimension"));
}

#[test]
fn conversion_round_trips_without_loss() {
    // A round trip through a smaller unit and back must be the identity. With floating point
    // it would not be, and the loss would be invisible.
    let original = read_quantity("1.5 ms").expect("valid");
    let converted = original
        .convert_to(unit("ns").unwrap())
        .expect("same dimension");
    assert_eq!(converted.value, Rational::integer(1_500_000));
    let back = converted
        .convert_to(unit("ms").unwrap())
        .expect("same dimension");
    assert_eq!(back.value, original.value);
}

#[test]
fn a_fractional_conversion_stays_exact_rather_than_rounding() {
    // 1 ns expressed in ms is 1/1_000_000, which has no short decimal form. It must stay
    // exact, because a rounded-to-zero duration is a duration that disappears.
    let nanosecond = q(1, "ns");
    let in_ms = nanosecond
        .convert_to(unit("ms").unwrap())
        .expect("same dimension");
    assert_eq!(in_ms.value, Rational::new(1, 1_000_000).unwrap());
    assert!(!in_ms.value.is_zero());
    assert!(in_ms.equals(nanosecond).expect("same dimension"));
}

#[test]
fn comparison_direction_decides_what_satisfies_what() {
    // §5.2: "More bits or a faster clock is not universally better." A 60 s horizon satisfies a
    // 30 s requirement; a 60 us delivery bound does NOT satisfy a 30 us one.
    let offered_horizon = q(60, "s");
    let required_horizon = q(30, "s");
    assert!(ComparisonDirection::AtLeast
        .satisfied_by(offered_horizon, required_horizon)
        .expect("same dimension"));
    assert!(!ComparisonDirection::AtMost
        .satisfied_by(offered_horizon, required_horizon)
        .expect("same dimension"));

    let offered_bound = q(60, "us");
    let required_bound = q(30, "us");
    assert!(!ComparisonDirection::AtLeast
        .satisfied_by(required_bound, offered_bound)
        .expect("same dimension"));
    assert!(ComparisonDirection::AtMost
        .satisfied_by(required_bound, offered_bound)
        .expect("same dimension"));
}

#[test]
fn an_exact_direction_refuses_a_merely_better_value() {
    // A tick unit or a counter modulus is not improved by being larger; it has to match.
    assert!(!ComparisonDirection::Exact
        .satisfied_by(q(20, "MHz"), q(10, "MHz"))
        .expect("same dimension"));
    assert!(ComparisonDirection::Exact
        .satisfied_by(q(10_000_000, "Hz"), q(10, "MHz"))
        .expect("same dimension, different spelling"));
}

#[test]
fn a_direction_comparison_across_dimensions_is_refused_not_answered() {
    let error = ComparisonDirection::AtLeast
        .satisfied_by(q(10, "MHz"), q(10, "ms"))
        .expect_err("must refuse");
    assert!(matches!(
        error,
        QuantityError::IncompatibleDimensions { .. }
    ));
}

#[test]
fn the_corpus_quantities_all_read() {
    // The units the boundary corpus actually writes must all be known: `s`, `us`, `MHz`,
    // `KiB`, `bit`, `ns`, `ms`. A unit table that the corpus cannot use is the wrong table.
    for text in [
        "60 s", "50 us", "10 MHz", "64 KiB", "32 bit", "10 ms", "16 byte", "1 tick",
    ] {
        assert!(read_quantity(text).is_ok(), "`{text}` should read");
    }
}

#[test]
fn a_zero_duration_is_legal_but_a_negative_one_is_not() {
    // Zero is a meaningful duration — an immediate deadline. Negative is not.
    assert!(read_quantity("0 ms").is_ok());
    let rendered = read_quantity("-1 ms").expect_err("must refuse");
    assert!(
        rendered.contains("quantity-negative-duration"),
        "{rendered}"
    );
}

// ── the pipeline, because §13.1's gate reads a fixture and not a type ─────────────────────────────
//
// ⛔ **Leaf `M1.28`, and the reason this half of the file exists.** Everything above drives `Quantity`
// directly, which is the right level for "refused *before arithmetic*" and the wrong level for "a
// description holding one is refused". Three consumers of [`Quantity::read`] discarded what it found —
// `crates/eadl-model/src/workload.rs`'s `.map_err(|_| ())` and two `.ok()`s in
// `crates/eadl-model/src/refinement.rs` — and the only consumer that propagated was
// `crates/archogen-s0/src/interpret.rs`, the prototype `S0-RETIREMENT` exists to delete. So
// `archogen check` printed `accepted against profile rt-static-up-v1` for `(period 10 parsec)` while
// `archogen build` refused the same bytes, and **no leg in this file could see the difference** because
// none of them ran the pipeline. F03 was green at the type and unenforced at the surface.

use std::path::Path;

use eadl_front::Verdict;
use eadl_model::check::{check, default_profile, shipped_registry, Outcome};

/// The kind modules a real invocation loads.
const KIND_MODULES: &[&str] = &[
    "docs/semantics/kinds/core.eadl",
    "docs/semantics/kinds/os-rt.eadl",
];

fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Run the whole `archogen check` pipeline over one description.
///
/// ⭐ Through [`shipped_registry`] and [`check`] — the production loader and the production pipeline —
/// and not through `validate`, for the reason `M1.13.4.1` recorded: a test that bypasses the layer under
/// test measures the wrong thing, confidently. The quantity refusal this file is about lives in the
/// schema *and* in the refinement pass, and only `check` runs both.
fn pipeline(text: &str) -> (Outcome, SourceMap) {
    let mut sources = SourceMap::new();
    let files: Vec<(String, String)> = KIND_MODULES
        .iter()
        .map(|relative| {
            let path = repo_root().join(relative);
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
            ((*relative).to_string(), text)
        })
        .collect();
    let registry = shipped_registry(&mut sources, &files).unwrap_or_else(|errors| {
        panic!(
            "the shipped kind modules are malformed:\n{}",
            errors
                .iter()
                .map(|d| d.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        )
    });
    let id = sources
        .add("pipeline.eadl", text.to_string())
        .expect("small");
    (check(&sources, id, &registry, default_profile()), sources)
}

/// The diagnostic codes a run produced, in pass order.
fn codes(outcome: &Outcome) -> Vec<&'static str> {
    outcome.diagnostics.iter().map(|d| d.code).collect()
}

#[test]
fn f03_a_zero_clock_frequency_is_refused_by_the_pipeline_and_not_only_by_the_type() {
    let (outcome, sources) = pipeline("(defblock timer.counter (offers (tick-rate 0 MHz)))");
    assert_eq!(
        codes(&outcome),
        vec!["quantity-non-positive-frequency"],
        "{}",
        outcome.render(&sources)
    );
    assert_eq!(outcome.verdict, Verdict::InvalidDescription);
}

#[test]
fn f03_an_unknown_unit_is_refused_by_the_schema_before_anything_reads_it() {
    let (outcome, sources) =
        pipeline("(defsystem s (task t (period 10 parsec) (deadline 10 ms) (priority 1)))");
    assert_eq!(
        codes(&outcome),
        vec!["quantity-unknown-unit"],
        "the refusal must be `quantity.rs`'s own, not a second code for the same mistake: {}",
        outcome.render(&sources)
    );
    // ⛔ And not `schema-type`: the clause *holds* the right shapes, it is the unit that is unknown, and
    // a code that said "expected a quantity" would send the author at the whole clause instead of at the
    // one symbol.
    assert_eq!(outcome.verdict, Verdict::InvalidDescription);
}

#[test]
fn f03_a_malformed_quantity_is_reported_exactly_once() {
    // ⭐ The schema pass and the workload pass both read `period`. Reporting from both would hand the
    // author the same refusal twice, so the workload pass relies on the schema — and this leg is what
    // makes that reliance a claim something checks rather than a comment.
    let (outcome, sources) =
        pipeline("(defsystem s (task t (period 10 parsec) (deadline 10 ms) (priority 1)))");
    let mentioning = outcome
        .diagnostics
        .iter()
        .filter(|d| d.message.contains("parsec"))
        .count();
    assert_eq!(
        mentioning,
        1,
        "one mistake, one message: {}",
        outcome.render(&sources)
    );
}

#[test]
fn f03_a_negative_duration_is_refused_by_the_pipeline_too() {
    let (outcome, sources) =
        pipeline("(defsystem s (task t (period -5 ms) (deadline 10 ms) (priority 1)))");
    assert_eq!(
        codes(&outcome),
        vec!["quantity-negative-duration"],
        "{}",
        outcome.render(&sources)
    );
}

#[test]
fn f03_a_bound_without_a_unit_is_refused_where_it_is_written() {
    // `(holds forms)` is not interpreted by the schema — `docs/semantics/kinds/core.eadl` says so — so
    // this refusal comes from the pass that reads the bound, and it reaches the author because that pass
    // now propagates instead of discarding.
    let (outcome, sources) =
        pipeline("(defplatform soc.abstract (offers (tick-rate (exactly 10))))");
    assert_eq!(
        codes(&outcome),
        vec!["quantity-missing-unit"],
        "{}",
        outcome.render(&sources)
    );
}

#[test]
fn f03_a_lone_number_is_a_count_and_not_a_malformed_quantity() {
    // ⛔ **The false positive this leaf measured, from the shipped corpus.**
    // `docs/semantics/boundary/accept/counter-width-and-rate.eadl` holds
    // `(counter-modulus 4294967296)` and its recorded verdict is `accept`. Reading *every* value-path
    // offer as a quantity turned it into `quantity-missing-unit`, so an accepted boundary case became
    // `invalid-description` — found by a census over all 76 tracked descriptions and not by any leg,
    // because no leg ran the pipeline over that corpus (`f27_boundary.rs` has one now). The rule is a
    // shape: `<number> <symbol>` is a quantity, a lone number is a count.
    let (outcome, sources) = pipeline(
        "(defblock timer.counter
           (offers
             (counter-width 32 bit)
             (counter-modulus 4294967296)
             (tick-rate 10 MHz)))",
    );
    assert!(
        outcome.diagnostics.is_empty(),
        "a count is not a malformed quantity: {}",
        outcome.render(&sources)
    );
    assert_eq!(outcome.verdict, Verdict::Ok);
}

#[test]
fn f03_an_unreadable_facet_reports_the_quantity_and_not_a_violated_obligation() {
    // An obligation whose value could not be read is *unchecked*, not violated: reporting "this
    // refinement drops a guarantee" beside `quantity-non-positive-frequency` would send the author at
    // the wrong half of their own description.
    let (outcome, sources) = pipeline(
        "(defplatform soc.abstract (offers (tick-rate (exactly 10 MHz))))
         (defplatform soc.concrete (refines soc.abstract) (offers (tick-rate 0 MHz)))",
    );
    assert_eq!(
        codes(&outcome),
        vec!["quantity-non-positive-frequency"],
        "{}",
        outcome.render(&sources)
    );
    assert_eq!(outcome.verdict, Verdict::InvalidDescription);
}

#[test]
fn f03_the_suite_s_zero_frequency_case_is_refused_for_the_reason_its_header_claims() {
    // ⛔ **F-K, and the leg that keeps it closed.** The case used to put `(refines …)` on a `defblock`,
    // which is `defplatform`'s clause, so it was refused as `schema-unknown-clause` and got its expected
    // `invalid-description` without ever reaching a zero frequency — a suite case that passes for the
    // wrong reason is worse than one that fails, because the corpus looks like it covers F03.
    let relative = "docs/semantics/cases/invalid-zero-clock-frequency.eadl";
    let text = std::fs::read_to_string(repo_root().join(relative))
        .unwrap_or_else(|e| panic!("cannot read {relative}: {e}"));
    let (outcome, sources) = pipeline(&text);
    assert_eq!(
        codes(&outcome),
        vec!["quantity-non-positive-frequency"],
        "{relative} must exercise a zero frequency and nothing else: {}",
        outcome.render(&sources)
    );
}

#[test]
fn a_missing_quantity_is_labelled_where_it_is_missing_not_in_the_first_file() {
    // ⛔ Leaf `M1.41`. The reader labelled a missing magnitude with `SourceId(0)`, the first file the source map
    // took — under `archogen check`, a shipped kind module. The description here is the second file, so a label
    // built from file zero lands in `first.eadl`.
    let mut sources = SourceMap::new();
    sources
        .add("first.eadl", "(defkind something)")
        .expect("small");
    let id = sources.add("t.eadl", "(x (exactly))").expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    let bound = &document.forms[0].items()[1];
    let refused = Quantity::read(bound.items().get(1), bound.items().get(2), bound.span())
        .expect_err("nothing is written, so nothing is read");
    let rendered = refused.render(&sources);
    assert_eq!(refused.code, "quantity-missing", "{rendered}");
    assert!(rendered.contains("t.eadl:1:4"), "{rendered}");
    assert!(!rendered.contains("first.eadl"), "{rendered}");
}
