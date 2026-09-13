//! The mechanical half of the eADL/engine boundary (fixture **F27**).
//!
//! `docs/decisions/decision_eadl-engine-boundary.md` states the rule; this module enforces the
//! part a machine can. `ROADMAP.md` §4.3 sets the scope precisely, and its last sentence is the
//! one that keeps this module honest:
//!
//! > F27 tests schema rejection of forbidden implementation fields and correct interpretation
//! > of accepted cases; **human review still checks semantic intent because a field can hide an
//! > algorithm behind an innocent name.**
//!
//! So this is a **floor**, not a verdict. It refuses the constructs that are implementation by
//! definition, with the test each one fails and where it belongs instead. It cannot see an
//! algorithm expressed in vocabulary nobody has classified yet, and it does not pretend to.
//!
//! # Why a construct registry rather than a keyword scan
//!
//! The tempting implementation is to look for imperative-sounding words. It is wrong, and the
//! corpus proves it in one case: the **accepted** `required-ordering-guarantee` declaration
//! contains `write` twice —
//!
//! ```text
//! (defplatform soc.bus
//!   (requires (ordering (before (write device.control))
//!                       (after  (write memory.buffer)))))
//! ```
//!
//! — where `write` names an *observable effect* an ordering requirement is stated over, not a
//! step to perform. A keyword scan rejects a legitimate contract. What actually separates the
//! two is the **construct** the content sits inside: everything under an `implementation`,
//! `model`, `provider` or `emit` block is a procedure; the same word elsewhere is a reference.

use eadl_front::{Diagnostic, Form, Label, Span};

/// Which of the three boundary tests a declaration fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoundaryTest {
    /// Test 1 — does it state an offered feature, required functionality, architectural
    /// connection, operating condition, or externally testable guarantee?
    Externality,
    /// Test 2 — would it remain valid for a different implementation with the same relevant
    /// functionality and guarantees?
    ImplementationIndependence,
    /// Test 3 — can its interpretation be stated without prescribing an algorithm, instruction
    /// sequence, code provider, data structure, or executable model body?
    NonPrescription,
}

impl BoundaryTest {
    /// The stable machine-readable name, as the corpus writes it.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Externality => "externality",
            Self::ImplementationIndependence => "implementation-independence",
            Self::NonPrescription => "non-prescription",
        }
    }

    /// The question the test asks.
    #[must_use]
    pub const fn question(self) -> &'static str {
        match self {
            Self::Externality => {
                "does it state an offered feature, required functionality, architectural connection, operating condition, or externally testable guarantee?"
            }
            Self::ImplementationIndependence => {
                "would it remain valid for a different implementation with the same relevant functionality and guarantees?"
            }
            Self::NonPrescription => {
                "can its interpretation be stated without prescribing an algorithm, instruction sequence, code provider, data structure, or executable model body?"
            }
        }
    }

    /// All three, in the order the decision record states them.
    pub const ALL: &'static [Self] = &[
        Self::Externality,
        Self::ImplementationIndependence,
        Self::NonPrescription,
    ];
}

/// A construct eADL may not carry, and why.
pub struct ForbiddenConstruct {
    /// The list head that names it, matched exactly. Prefix matching would reject
    /// `implementation-strategy` for the wrong reason and, worse, would one day reject a
    /// legitimate name nobody anticipated.
    pub head: &'static str,
    /// The first test it fails.
    pub failing_test: BoundaryTest,
    /// What it actually is.
    pub what: &'static str,
    /// Where it belongs instead. A refusal that does not say this is an obstacle rather than
    /// guidance.
    pub belongs: &'static str,
}

/// Every construct that is implementation by definition.
///
/// Each entry is drawn from `ROADMAP.md` §4.3's worked cases or from an explicit rule elsewhere
/// in the roadmap, and each is exercised by a case in `docs/semantics/boundary/reject/`.
pub const FORBIDDEN: &[ForbiddenConstruct] = &[
    ForbiddenConstruct {
        head: "implementation",
        failing_test: BoundaryTest::NonPrescription,
        what: "an implementation body — an algorithm, a sequence, or a data structure",
        belongs: "engine knowledge: the provider record selected against this declaration's contract",
    },
    ForbiddenConstruct {
        head: "model",
        failing_test: BoundaryTest::NonPrescription,
        what: "an executable behavioral model body",
        belongs: "engine catalogs: §12 M4 is explicit that no model implementation is added to eADL",
    },
    ForbiddenConstruct {
        head: "provider",
        failing_test: BoundaryTest::Externality,
        what: "a selection of the code that must be used",
        belongs: "engine configuration: §5.4 puts preferred-provider order there, not in eADL",
    },
    ForbiddenConstruct {
        head: "emit",
        failing_test: BoundaryTest::NonPrescription,
        what: "a code-generation instruction",
        belongs: "the engine: lowering rules and the emitter's output layout",
    },
    ForbiddenConstruct {
        head: "permits",
        failing_test: BoundaryTest::NonPrescription,
        what: "a permitted implementation strategy — a prescription wearing a permission's clothes",
        belongs: "nowhere: the contract already admits every strategy that satisfies it, and naming one narrows the engine to the strategies someone thought of",
    },
    ForbiddenConstruct {
        head: "wcet",
        failing_test: BoundaryTest::Externality,
        what: "an execution bound — evidence about a binary, not a property of the system described",
        belongs: "the engine build manifest: §7.3 keeps bounds, their origin, and their target and binary identity outside eADL",
    },
    ForbiddenConstruct {
        head: "init-order",
        failing_test: BoundaryTest::ImplementationIndependence,
        what: "a concrete boot sequence over selected providers",
        belongs: "the engine: §5.4 derives initialization order during joint resolution, from declared dependencies",
    },
    ForbiddenConstruct {
        head: "save-order",
        failing_test: BoundaryTest::NonPrescription,
        what: "a register save order — a context layout",
        belongs: "the architecture substrate: §8.2 generates layouts from the selected ABI and privilege rules",
    },
    ForbiddenConstruct {
        head: "read-sequence",
        failing_test: BoundaryTest::NonPrescription,
        what: "an access protocol",
        belongs: "engine knowledge: the access strategy chosen per platform against the coherence contract",
    },
];

/// Look up a forbidden construct by its head.
#[must_use]
pub fn forbidden(head: &str) -> Option<&'static ForbiddenConstruct> {
    FORBIDDEN.iter().find(|item| item.head == head)
}

/// What the classifier concluded about one declaration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Classification {
    /// Nothing forbidden was found.
    ///
    /// ⚠️ This is **not** "this declaration is sound". It is "this declaration contains no
    /// construct the registry knows to be implementation". §4.3 requires human review for
    /// intent, and the gap between those two sentences is exactly where a field hiding an
    /// algorithm behind an innocent name lives.
    Accepted,
    /// A forbidden construct was found.
    Rejected {
        /// The construct's head.
        head: &'static str,
        /// The first test it fails.
        failing_test: BoundaryTest,
        /// Where in the source.
        span: Span,
    },
}

impl Classification {
    /// Whether the declaration passed the mechanical floor.
    #[must_use]
    pub const fn is_accepted(&self) -> bool {
        matches!(self, Self::Accepted)
    }
}

/// Classify one declaration.
///
/// The walk is outermost-first, then left to right, so a declaration wrapping an
/// `implementation` block is reported at the block rather than at some deeper step inside it —
/// the outer construct is the thing to remove.
#[must_use]
pub fn classify(form: &Form) -> Classification {
    if let Some(head) = form.head() {
        if let Some(construct) = forbidden(head) {
            return Classification::Rejected {
                head: construct.head,
                failing_test: construct.failing_test,
                span: form.span(),
            };
        }
    }
    for item in form.items() {
        let result = classify(item);
        if !result.is_accepted() {
            return result;
        }
    }
    Classification::Accepted
}

/// Classify, and render a refusal as a diagnostic when it fails.
///
/// §5.5 requires every diagnostic to carry a span and a concrete repair direction; a boundary
/// refusal also names the test that failed and the question that test asks, because an author
/// told only "not allowed" learns nothing transferable.
///
/// # Errors
///
/// Returns the refusal when the declaration carries a forbidden construct. The error is boxed:
/// a [`Diagnostic`] is several strings and vectors wide, and an unboxed error variant would
/// inflate every `Result` on the success path for the benefit of the failure path.
pub fn check(form: &Form) -> Result<(), Box<Diagnostic>> {
    match classify(form) {
        Classification::Accepted => Ok(()),
        Classification::Rejected {
            head,
            failing_test,
            span,
        } => {
            let construct = forbidden(head).expect("a rejection names a registered construct");
            Err(Box::new(Diagnostic::error(
                "boundary-implementation-in-description",
                format!("`{head}` is implementation, and eADL contains no implementation"),
                Label::new(span, format!("this is {}", construct.what)),
                format!(
                    "it fails the `{}` test — {} It belongs to {}.",
                    failing_test.slug(),
                    failing_test.question(),
                    construct.belongs
                ),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{check, classify, forbidden, BoundaryTest, Classification, FORBIDDEN};
    use eadl_front::{read, SourceMap};

    fn parse(text: &str) -> eadl_front::Form {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text).expect("small");
        let (document, diagnostics) = read(&sources, id);
        assert!(
            !diagnostics.has_errors(),
            "fixture did not read:\n{}",
            diagnostics.render(&sources)
        );
        document.forms.into_iter().next().expect("one form")
    }

    #[test]
    fn a_functional_requirement_is_accepted() {
        let form =
            parse("(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s))))");
        assert_eq!(classify(&form), Classification::Accepted);
        assert!(check(&form).is_ok());
    }

    #[test]
    fn an_implementation_block_is_rejected_on_non_prescription() {
        let form = parse(
            "(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s))) (implementation (on-wrap (increment epoch))))",
        );
        match classify(&form) {
            Classification::Rejected {
                head, failing_test, ..
            } => {
                assert_eq!(head, "implementation");
                assert_eq!(failing_test, BoundaryTest::NonPrescription);
            }
            other => panic!("expected a rejection, got {other:?}"),
        }
    }

    #[test]
    fn a_keyword_scan_would_reject_this_accepted_case_and_the_registry_does_not() {
        // ⭐ The design argument, as a test. `write` appears twice in a legitimate ordering
        // contract, naming observable effects rather than steps. A classifier that looked for
        // imperative-sounding words would refuse a declaration the roadmap explicitly permits.
        let form = parse(
            "(defplatform soc.bus (requires (ordering (before (write device.control)) (after (write memory.buffer)))))",
        );
        assert!(form.to_canonical().contains("write"));
        assert_eq!(classify(&form), Classification::Accepted);
    }

    #[test]
    fn the_same_word_inside_a_forbidden_construct_is_rejected() {
        // The other half of the argument: `write` under `implementation` is a step.
        let form = parse(
            "(defblock timer.compare (implementation (program-deadline (write cmp-low value-low))))",
        );
        assert!(!classify(&form).is_accepted());
    }

    #[test]
    fn a_nested_forbidden_field_is_found_however_deep() {
        // `wcet` sits three levels down, inside a task that is otherwise entirely legitimate.
        let form =
            parse("(defsystem app.rt (task sensor (period 10 ms) (deadline 10 ms) (wcet 850 us)))");
        match classify(&form) {
            Classification::Rejected {
                head, failing_test, ..
            } => {
                assert_eq!(head, "wcet");
                assert_eq!(failing_test, BoundaryTest::Externality);
            }
            other => panic!("expected a rejection, got {other:?}"),
        }
    }

    #[test]
    fn the_outermost_forbidden_construct_is_reported_not_the_deepest() {
        // The outer block is the thing to remove; pointing at a step inside it would send the
        // author to fix the symptom.
        let form = parse("(defblock x (implementation (read-sequence (loop (read high)))))");
        match classify(&form) {
            Classification::Rejected { head, .. } => assert_eq!(head, "implementation"),
            other => panic!("expected a rejection, got {other:?}"),
        }
    }

    #[test]
    fn heads_are_matched_exactly_not_by_prefix() {
        // `implementation-strategy` must not be rejected *because it starts with*
        // `implementation`. Its own case is rejected for containing `permits`, which is a
        // different and correct reason.
        let form = parse("(defservice s (offers (implementation-strategy-note whatever)))");
        assert_eq!(classify(&form), Classification::Accepted);
    }

    #[test]
    fn a_forbidden_head_in_a_non_head_position_is_not_a_construct() {
        // `(offers model)` mentions the word as a value. Only a list *head* introduces a block.
        let form = parse("(defblock b (offers model))");
        assert_eq!(classify(&form), Classification::Accepted);
    }

    #[test]
    fn a_refusal_names_the_test_the_question_and_where_it_belongs() {
        let form = parse("(defsystem app.rt (task t (wcet 1 us)))");
        let diagnostic = check(&form).expect_err("must refuse");
        assert_eq!(diagnostic.code, "boundary-implementation-in-description");
        assert!(diagnostic
            .message
            .contains("eADL contains no implementation"));
        assert!(
            diagnostic.repair.contains("externality"),
            "{}",
            diagnostic.repair
        );
        assert!(
            diagnostic
                .repair
                .contains("does it state an offered feature"),
            "the refusal must carry the question, not just its name: {}",
            diagnostic.repair
        );
        assert!(
            diagnostic.repair.contains("build manifest"),
            "the refusal must say where it belongs: {}",
            diagnostic.repair
        );
    }

    #[test]
    fn every_registered_construct_names_a_test_and_a_destination() {
        for construct in FORBIDDEN {
            assert!(!construct.what.is_empty(), "{}", construct.head);
            assert!(
                !construct.belongs.is_empty(),
                "`{}` refuses without saying where it belongs",
                construct.head
            );
            assert!(forbidden(construct.head).is_some());
        }
    }

    #[test]
    fn registered_heads_are_unique() {
        let mut heads: Vec<&str> = FORBIDDEN.iter().map(|c| c.head).collect();
        let total = heads.len();
        heads.sort_unstable();
        heads.dedup();
        assert_eq!(heads.len(), total);
    }

    #[test]
    fn test_slugs_match_what_the_corpus_writes() {
        let slugs: Vec<&str> = BoundaryTest::ALL.iter().map(|t| t.slug()).collect();
        assert_eq!(
            slugs,
            vec![
                "externality",
                "implementation-independence",
                "non-prescription"
            ]
        );
    }
}
