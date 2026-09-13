//! Properties, evidence categories, permitted conclusions, and the report that holds them.

/// A named property a report carries a status for (`ROADMAP.md` §7.1).
///
/// The list is closed on purpose: a report must address every one of these. Adding a property
/// is a deliberate change that makes every existing report incomplete until it is answered —
/// which is the correct consequence, because a new property is a new question nobody has
/// answered yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {
    /// The resolved configuration satisfies the submitted requirements.
    ConfigurationValidity,
    /// The runtime behaves as its contracts say.
    RuntimeFunctionalBehavior,
    /// Deadlines and response bounds.
    Timing,
    /// Memory and stack bounds.
    MemoryBounds,
    /// Reset, initialization order, and reaching the first task release.
    StartupBehavior,
}

impl Property {
    /// Every property a complete report must address.
    pub const ALL: &'static [Self] = &[
        Self::ConfigurationValidity,
        Self::RuntimeFunctionalBehavior,
        Self::Timing,
        Self::MemoryBounds,
        Self::StartupBehavior,
    ];

    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::ConfigurationValidity => "configuration-validity",
            Self::RuntimeFunctionalBehavior => "runtime-functional-behavior",
            Self::Timing => "timing",
            Self::MemoryBounds => "memory-bounds",
            Self::StartupBehavior => "startup-behavior",
        }
    }
}

/// Where a conclusion's authority comes from (`ROADMAP.md` §7.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceCategory {
    /// A structural check over the resolved configuration.
    StructuralCheck,
    /// An analysis valid under listed assumptions, in a declared model.
    ConditionalAnalysis,
    /// Observed agreement with a reference, over recorded coverage.
    TestedConformance,
    /// A machine-checked property of a stated formal model.
    ModelProof,
    /// A named implementation preserving a specified model property.
    ImplementationRefinement,
    /// Measurement or a binary-specific bound on a documented target.
    TargetEvidence,
}

impl EvidenceCategory {
    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::StructuralCheck => "structural-check",
            Self::ConditionalAnalysis => "conditional-analysis",
            Self::TestedConformance => "tested-conformance",
            Self::ModelProof => "model-proof",
            Self::ImplementationRefinement => "implementation-refinement",
            Self::TargetEvidence => "target-evidence",
        }
    }

    /// Whether this category can ever support a positive statement about the **real system**,
    /// as opposed to about a model, a configuration, or an observation.
    ///
    /// Nothing here returns `true`, and that is the point rather than an oversight. §7.2 is
    /// explicit that a conditional analysis, a model proof and tested conformance are each one
    /// link of a three-link chain, and that the first practical release "must list the unproved
    /// links". The method exists so that a future refinement track has somewhere honest to
    /// change, and so that today's answer is written down instead of assumed.
    #[must_use]
    pub const fn establishes_real_system_property(self) -> bool {
        false
    }
}

/// What a claim actually says.
///
/// Every positive variant carries the qualifier that makes it true. There is deliberately no
/// `Verified`, no `Passed`, and no `Holds` without a named scope: §7.1's permitted conclusions
/// are conclusions *about something*, and dropping the something is exactly how a report comes
/// to overstate its evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conclusion {
    /// Structural check — "the checked configuration satisfies those constraints".
    ConfigurationSatisfies {
        /// The constraint set that was checked.
        constraints: String,
    },
    /// Conditional analysis — "deadlines follow in the declared model under listed assumptions".
    HoldsUnderAssumptions {
        /// The declared model the conclusion is stated in.
        model: String,
        /// The assumptions the conclusion depends on. Never empty.
        assumptions: Vec<String>,
    },
    /// Tested conformance — "no violation was observed within recorded test coverage".
    ///
    /// Note what this variant cannot say. There is no amount of testing that promotes it to a
    /// stronger variant, because §7.1 says plainly: "Testing does not become proof through
    /// repetition."
    NoViolationObserved {
        /// What was covered, precisely enough to bound the claim.
        coverage: String,
    },
    /// Model proof — "the invariant holds for the stated formal model".
    HoldsForModel {
        /// The formal model the proof is about.
        model: String,
        /// The invariant that was checked.
        invariant: String,
    },
    /// Implementation refinement — "that property transfers through the proved refinement and
    /// its assumptions".
    TransfersByRefinement {
        /// The refinement proof relied upon.
        refinement: String,
        /// The assumptions the refinement carries. Never empty.
        assumptions: Vec<String>,
    },
    /// Target evidence — "only the documented bound or observation applies to that target
    /// configuration".
    AppliesToTarget {
        /// The target configuration identity.
        target: String,
        /// The binary identity the observation belongs to.
        binary: String,
        /// What was observed or bounded.
        observation: String,
    },
    /// A sufficient analysis did not establish the property (§5.5 `not-established`).
    NotEstablished {
        /// Why, concretely.
        why: String,
    },
    /// A resource limit or unresolved bound prevented a conclusion (§5.5 `analysis-inconclusive`).
    Inconclusive {
        /// The limit that was hit.
        limit: String,
    },
    /// A validated witness violates the property (§5.5 `counterexample`).
    Counterexample {
        /// The witness.
        witness: String,
    },
    /// The property does not apply to this system, with the reason.
    ///
    /// This is the only way a report may say nothing about a property, and it is still saying
    /// something: it names why. Absence is not available.
    NotApplicable {
        /// Why this property does not apply here.
        why: String,
    },
}

impl Conclusion {
    /// The evidence category this conclusion draws on, or `None` for a negative or
    /// not-applicable outcome, which draws on none.
    #[must_use]
    pub const fn category(&self) -> Option<EvidenceCategory> {
        match self {
            Self::ConfigurationSatisfies { .. } => Some(EvidenceCategory::StructuralCheck),
            Self::HoldsUnderAssumptions { .. } => Some(EvidenceCategory::ConditionalAnalysis),
            Self::NoViolationObserved { .. } => Some(EvidenceCategory::TestedConformance),
            Self::HoldsForModel { .. } => Some(EvidenceCategory::ModelProof),
            Self::TransfersByRefinement { .. } => Some(EvidenceCategory::ImplementationRefinement),
            Self::AppliesToTarget { .. } => Some(EvidenceCategory::TargetEvidence),
            Self::NotEstablished { .. }
            | Self::Inconclusive { .. }
            | Self::Counterexample { .. }
            | Self::NotApplicable { .. } => None,
        }
    }

    /// Whether this conclusion asserts the property in any positive sense.
    #[must_use]
    pub const fn is_positive(&self) -> bool {
        self.category().is_some()
    }

    /// The rendered statement, always carrying its qualifier.
    #[must_use]
    pub fn statement(&self) -> String {
        match self {
            Self::ConfigurationSatisfies { constraints } => {
                format!("the checked configuration satisfies {constraints}")
            }
            Self::HoldsUnderAssumptions { model, assumptions } => format!(
                "holds in model `{model}` under {} listed assumption(s): {}",
                assumptions.len(),
                assumptions.join("; ")
            ),
            Self::NoViolationObserved { coverage } => {
                format!("no violation was observed within recorded coverage: {coverage}")
            }
            Self::HoldsForModel { model, invariant } => {
                format!("invariant `{invariant}` holds for the stated formal model `{model}`")
            }
            Self::TransfersByRefinement {
                refinement,
                assumptions,
            } => format!(
                "transfers through refinement `{refinement}` and its {} assumption(s): {}",
                assumptions.len(),
                assumptions.join("; ")
            ),
            Self::AppliesToTarget {
                target,
                binary,
                observation,
            } => format!("{observation} — applies only to target `{target}`, binary `{binary}`"),
            Self::NotEstablished { why } => format!("not established: {why}"),
            Self::Inconclusive { limit } => format!("inconclusive: {limit}"),
            Self::Counterexample { witness } => format!("counterexample: {witness}"),
            Self::NotApplicable { why } => format!("not applicable: {why}"),
        }
    }

    /// Why this conclusion is malformed, if it is.
    ///
    /// A qualifier that is present but empty is the same overstatement wearing a different
    /// hat: `HoldsUnderAssumptions { assumptions: vec![] }` renders as "holds", full stop.
    fn defect(&self) -> Option<&'static str> {
        let blank = |s: &String| s.trim().is_empty();
        match self {
            Self::ConfigurationSatisfies { constraints } if blank(constraints) => {
                Some("a structural check must name the constraint set it checked")
            }
            Self::HoldsUnderAssumptions { model, assumptions } => {
                if blank(model) {
                    Some("a conditional analysis must name the model its conclusion is stated in")
                } else if assumptions.is_empty() || assumptions.iter().all(blank) {
                    Some("a conditional analysis with no listed assumptions is an unconditional claim")
                } else {
                    None
                }
            }
            Self::NoViolationObserved { coverage } if blank(coverage) => {
                Some("tested conformance must state the coverage that bounds it")
            }
            Self::HoldsForModel { model, invariant } if blank(model) || blank(invariant) => {
                Some("a model proof must name both the model and the invariant")
            }
            Self::TransfersByRefinement {
                refinement,
                assumptions,
            } => {
                if blank(refinement) {
                    Some("a refinement claim must name the refinement it relies on")
                } else if assumptions.is_empty() || assumptions.iter().all(blank) {
                    Some("a refinement transfers its assumptions; an empty list hides them")
                } else {
                    None
                }
            }
            Self::AppliesToTarget {
                target,
                binary,
                observation,
            } if blank(target) || blank(binary) || blank(observation) => {
                Some("target evidence must name the target, the binary, and what was observed")
            }
            Self::NotEstablished { why } if blank(why) => {
                Some("`not-established` must say what was missing")
            }
            Self::Inconclusive { limit } if blank(limit) => {
                Some("`inconclusive` must name the limit that was hit")
            }
            Self::Counterexample { witness } if blank(witness) => {
                Some("a counterexample must carry its witness")
            }
            Self::NotApplicable { why } if blank(why) => {
                Some("`not applicable` must say why — silence about a property is not an option")
            }
            _ => None,
        }
    }
}

/// One property's status, with the evidence behind it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Claim {
    /// The property this claim is about.
    pub property: Property,
    /// What is being said, and on what authority.
    pub conclusion: Conclusion,
}

impl Claim {
    /// Build a claim, refusing one whose conclusion is missing its qualifier.
    ///
    /// # Errors
    ///
    /// Returns [`ReportError::Malformed`] when the conclusion cannot carry its own weight —
    /// an unnamed model, an empty assumption list, an unbounded coverage statement.
    pub fn new(property: Property, conclusion: Conclusion) -> Result<Self, ReportError> {
        if let Some(defect) = conclusion.defect() {
            return Err(ReportError::Malformed {
                property,
                defect: defect.to_string(),
            });
        }
        Ok(Self {
            property,
            conclusion,
        })
    }

    /// The rendered report line.
    #[must_use]
    pub fn render(&self) -> String {
        let category = self
            .conclusion
            .category()
            .map_or("—", EvidenceCategory::slug);
        format!(
            "{:<28} [{}] {}",
            self.property.slug(),
            category,
            self.conclusion.statement()
        )
    }
}

/// Why a report could not be produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportError {
    /// A property has no claim at all.
    Incomplete {
        /// The properties left unaddressed.
        missing: Vec<Property>,
    },
    /// Two claims address the same property.
    Duplicate {
        /// The property claimed twice.
        property: Property,
    },
    /// A conclusion is missing the qualifier that would make it honest.
    Malformed {
        /// The property whose claim is malformed.
        property: Property,
        /// What is wrong with it.
        defect: String,
    },
}

impl core::fmt::Display for ReportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Incomplete { missing } => {
                let names: Vec<&str> = missing.iter().map(|p| p.slug()).collect();
                write!(
                    f,
                    "report is incomplete: no claim for {} — a property with no claim is not a pass",
                    names.join(", ")
                )
            }
            Self::Duplicate { property } => {
                write!(f, "property `{}` is claimed twice", property.slug())
            }
            Self::Malformed { property, defect } => {
                write!(f, "claim for `{}` is malformed: {defect}", property.slug())
            }
        }
    }
}

/// A per-property assurance report.
///
/// There is no method on this type that reduces it to a single verdict, and none should be
/// added. §7.1: "One global 'verified' flag is prohibited."
#[derive(Debug, Clone, Default)]
pub struct Report {
    claims: Vec<Claim>,
}

impl Report {
    /// An empty report. It cannot be rendered until every property is addressed.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one property's claim.
    ///
    /// # Errors
    ///
    /// Returns [`ReportError::Duplicate`] if that property already has a claim. A second claim
    /// is not an update: it is two answers to one question, and choosing between them silently
    /// is how the weaker one disappears.
    pub fn claim(&mut self, claim: Claim) -> Result<(), ReportError> {
        if self.claims.iter().any(|c| c.property == claim.property) {
            return Err(ReportError::Duplicate {
                property: claim.property,
            });
        }
        self.claims.push(claim);
        Ok(())
    }

    /// The claim for one property, if it has been made.
    #[must_use]
    pub fn claim_for(&self, property: Property) -> Option<&Claim> {
        self.claims.iter().find(|c| c.property == property)
    }

    /// Properties with no claim yet.
    #[must_use]
    pub fn missing(&self) -> Vec<Property> {
        Property::ALL
            .iter()
            .copied()
            .filter(|p| self.claim_for(*p).is_none())
            .collect()
    }

    /// Render the report: one line per property, in a stable order.
    ///
    /// # Errors
    ///
    /// Returns [`ReportError::Incomplete`] while any property lacks a claim. This is the
    /// structural half of the §7.1 prohibition: a report cannot be published that is silent
    /// about a property, so there is nothing for a reader to mistake for a global pass.
    pub fn render(&self) -> Result<String, ReportError> {
        let missing = self.missing();
        if !missing.is_empty() {
            return Err(ReportError::Incomplete { missing });
        }
        let mut out = String::from("assurance report — one status per property\n");
        for property in Property::ALL {
            let claim = self
                .claim_for(*property)
                .expect("completeness was just checked");
            out.push_str("  ");
            out.push_str(&claim.render());
            out.push('\n');
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::{Claim, Conclusion, EvidenceCategory, Property, Report, ReportError};

    fn ok(property: Property, conclusion: Conclusion) -> Claim {
        Claim::new(property, conclusion).expect("well-formed")
    }

    fn complete_report() -> Report {
        let mut report = Report::new();
        report
            .claim(ok(
                Property::ConfigurationValidity,
                Conclusion::ConfigurationSatisfies {
                    constraints: "the declared platform resource map".into(),
                },
            ))
            .unwrap();
        report
            .claim(ok(
                Property::RuntimeFunctionalBehavior,
                Conclusion::NoViolationObserved {
                    coverage: "14 hosted scenarios, 2 emulator runs".into(),
                },
            ))
            .unwrap();
        report
            .claim(ok(
                Property::Timing,
                Conclusion::HoldsUnderAssumptions {
                    model: "fixed-priority, bounded overhead".into(),
                    assumptions: vec!["supplied execution bounds are upper bounds".into()],
                },
            ))
            .unwrap();
        report
            .claim(ok(
                Property::MemoryBounds,
                Conclusion::NotEstablished {
                    why: "no static stack bound for two application entry points".into(),
                },
            ))
            .unwrap();
        report
            .claim(ok(
                Property::StartupBehavior,
                Conclusion::NoViolationObserved {
                    coverage: "reset-to-first-release on the hosted harness".into(),
                },
            ))
            .unwrap();
        report
    }

    #[test]
    fn a_report_cannot_be_rendered_while_a_property_is_unanswered() {
        // ⭐ The structural half of the §7.1 prohibition: silence about a property is not a
        // pass, so it is not even publishable.
        let mut report = Report::new();
        report
            .claim(ok(
                Property::Timing,
                Conclusion::NotEstablished {
                    why: "interrupt interference is not modeled".into(),
                },
            ))
            .unwrap();
        let error = report.render().expect_err("incomplete report must refuse");
        match error {
            ReportError::Incomplete { ref missing } => {
                assert_eq!(missing.len(), Property::ALL.len() - 1);
                assert!(!missing.contains(&Property::Timing));
            }
            other => panic!("unexpected error: {other:?}"),
        }
        assert!(error.to_string().contains("is not a pass"));
    }

    #[test]
    fn a_complete_report_renders_one_line_per_property() {
        let rendered = complete_report().render().expect("complete");
        for property in Property::ALL {
            assert!(
                rendered.contains(property.slug()),
                "`{}` missing from the report:\n{rendered}",
                property.slug()
            );
        }
        // One line per property plus the header.
        assert_eq!(rendered.lines().count(), Property::ALL.len() + 1);
    }

    #[test]
    fn there_is_no_aggregate_verdict_in_the_rendered_report() {
        // The words a reader would mistake for a global flag must not appear at all.
        let rendered = complete_report().render().expect("complete").to_lowercase();
        for banned in [
            "verified",
            "all passed",
            "overall",
            "fully proven",
            "guaranteed",
        ] {
            assert!(
                !rendered.contains(banned),
                "report contains the aggregate word `{banned}`:\n{rendered}"
            );
        }
    }

    #[test]
    fn a_conditional_analysis_with_no_assumptions_is_refused() {
        // An empty assumption list renders as "holds", full stop — the overstatement wearing
        // a different hat.
        let error = Claim::new(
            Property::Timing,
            Conclusion::HoldsUnderAssumptions {
                model: "fixed-priority".into(),
                assumptions: vec![],
            },
        )
        .expect_err("must refuse");
        assert!(error.to_string().contains("unconditional claim"), "{error}");
    }

    #[test]
    fn a_refinement_that_hides_its_assumptions_is_refused() {
        let error = Claim::new(
            Property::RuntimeFunctionalBehavior,
            Conclusion::TransfersByRefinement {
                refinement: "dispatch-refines-model".into(),
                assumptions: vec![],
            },
        )
        .expect_err("must refuse");
        assert!(error.to_string().contains("hides them"), "{error}");
    }

    #[test]
    fn tested_conformance_must_bound_itself() {
        let error = Claim::new(
            Property::RuntimeFunctionalBehavior,
            Conclusion::NoViolationObserved {
                coverage: "   ".into(),
            },
        )
        .expect_err("must refuse");
        assert!(error.to_string().contains("coverage"), "{error}");
    }

    #[test]
    fn target_evidence_must_name_its_target_and_binary() {
        // §7.1: "Only the documented bound or observation applies to that target
        // configuration." A target claim without an identity applies to nothing.
        let error = Claim::new(
            Property::Timing,
            Conclusion::AppliesToTarget {
                target: "riscv-virt-up".into(),
                binary: String::new(),
                observation: "worst observed response 3.2 ms".into(),
            },
        )
        .expect_err("must refuse");
        assert!(error.to_string().contains("binary"), "{error}");
    }

    #[test]
    fn not_applicable_must_still_say_why() {
        let error = Claim::new(
            Property::MemoryBounds,
            Conclusion::NotApplicable { why: String::new() },
        )
        .expect_err("must refuse");
        assert!(error.to_string().contains("silence"), "{error}");
    }

    #[test]
    fn a_property_cannot_be_claimed_twice() {
        let mut report = Report::new();
        report
            .claim(ok(
                Property::Timing,
                Conclusion::NotEstablished { why: "a".into() },
            ))
            .unwrap();
        let error = report
            .claim(ok(
                Property::Timing,
                Conclusion::HoldsUnderAssumptions {
                    model: "m".into(),
                    assumptions: vec!["a".into()],
                },
            ))
            .expect_err("two answers to one question");
        assert!(matches!(error, ReportError::Duplicate { .. }));
    }

    #[test]
    fn every_positive_conclusion_carries_a_qualifier_in_its_rendered_text() {
        // §7.1's permitted conclusions are conclusions *about something*. Dropping the
        // something is exactly how a report comes to overstate its evidence.
        let positives = [
            Conclusion::ConfigurationSatisfies {
                constraints: "the declared resource map".into(),
            },
            Conclusion::HoldsUnderAssumptions {
                model: "fixed-priority".into(),
                assumptions: vec!["bounds are upper bounds".into()],
            },
            Conclusion::NoViolationObserved {
                coverage: "14 scenarios".into(),
            },
            Conclusion::HoldsForModel {
                model: "ready-queue".into(),
                invariant: "priority order".into(),
            },
            Conclusion::TransfersByRefinement {
                refinement: "r".into(),
                assumptions: vec!["a".into()],
            },
            Conclusion::AppliesToTarget {
                target: "t".into(),
                binary: "b".into(),
                observation: "o".into(),
            },
        ];
        for conclusion in positives {
            assert!(conclusion.is_positive());
            let text = conclusion.statement();
            let qualified = [
                "configuration",
                "under",
                "within",
                "for the stated",
                "through",
                "applies only to",
            ]
            .iter()
            .any(|marker| text.contains(marker));
            assert!(qualified, "unqualified positive statement: {text}");
        }
    }

    #[test]
    fn no_evidence_category_establishes_a_property_of_the_real_system() {
        // §7.2's three obligations are three separate links. Until a refinement track closes
        // them, the honest answer is the same for every category — and it is written down,
        // not assumed.
        for category in [
            EvidenceCategory::StructuralCheck,
            EvidenceCategory::ConditionalAnalysis,
            EvidenceCategory::TestedConformance,
            EvidenceCategory::ModelProof,
            EvidenceCategory::ImplementationRefinement,
            EvidenceCategory::TargetEvidence,
        ] {
            assert!(
                !category.establishes_real_system_property(),
                "{}",
                category.slug()
            );
        }
    }

    #[test]
    fn negative_outcomes_draw_on_no_evidence_category() {
        for conclusion in [
            Conclusion::NotEstablished { why: "w".into() },
            Conclusion::Inconclusive { limit: "l".into() },
            Conclusion::Counterexample {
                witness: "w".into(),
            },
            Conclusion::NotApplicable { why: "w".into() },
        ] {
            assert!(conclusion.category().is_none());
            assert!(!conclusion.is_positive());
        }
    }

    #[test]
    fn property_slugs_are_unique_and_stable() {
        let mut slugs: Vec<&str> = Property::ALL.iter().map(|p| p.slug()).collect();
        let total = slugs.len();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(slugs.len(), total);
    }
}
