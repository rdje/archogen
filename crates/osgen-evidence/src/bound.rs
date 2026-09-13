//! Numerical bounds and where they came from (`ROADMAP.md` §7.3).
//!
//! > Every numerical bound records units, origin, scope, target and binary identity, and
//! > evidence category: assumed, observed maximum, externally supplied bound, or analytically
//! > established bound. **An observed maximum with a safety multiplier remains an empirical
//! > assumption unless a valid argument establishes a bound.**
//!
//! That last sentence is the one this module exists to enforce. Multiplying an observation by
//! 1.5 and calling the result a bound is the single most common way a timing claim becomes
//! untrue, and it is untrue in a way that looks like diligence. So the safety factor lives
//! *inside* [`BoundOrigin::ObservedMaximum`], where it cannot change the origin, and
//! [`Bound::is_established`] stays `false` no matter how large it is.

/// A padding multiplier applied to an observation, held as an exact rational.
///
/// Not an `f64`, for two reasons that point the same way. §7.4 requires "exact integer time
/// units or checked rational arithmetic" throughout, and a safety factor multiplies a timing
/// number — so binary floating point would introduce a rounding question in the one place
/// nobody would think to look for one. And `f64` is not `Eq`, which would quietly deny the
/// whole evidence vocabulary the ability to compare two records for equality; a bound that
/// cannot be compared to its baseline cannot be checked for drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SafetyFactor {
    /// The numerator, e.g. `3` for a factor of 3/2.
    pub numerator: u32,
    /// The denominator, e.g. `2` for a factor of 3/2. Never zero in a valid factor.
    pub denominator: u32,
}

impl SafetyFactor {
    /// A factor of `numerator / denominator`.
    #[must_use]
    pub const fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// Whether the factor is well-formed and pads rather than shrinks.
    #[must_use]
    pub const fn pads(self) -> bool {
        self.denominator != 0 && self.numerator >= self.denominator
    }
}

impl core::fmt::Display for SafetyFactor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

/// Where a numerical bound's authority comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundOrigin {
    /// Chosen, not derived. Honest and often necessary; never evidence.
    Assumed {
        /// Who chose it and on what basis.
        rationale: String,
    },
    /// The largest value seen, under stated conditions.
    ///
    /// A safety factor may be recorded, and changes nothing about what this is.
    ObservedMaximum {
        /// How many observations, under what conditions.
        conditions: String,
        /// An optional multiplier applied to the observation. Does **not** promote it.
        safety_factor: Option<SafetyFactor>,
    },
    /// Supplied by a named external party or tool, with its own scope.
    ExternallySupplied {
        /// The source, exactly enough to re-obtain it.
        source: String,
    },
    /// Derived by an argument that establishes it as an upper bound.
    AnalyticallyEstablished {
        /// The argument, and where it is recorded.
        argument: String,
    },
}

impl BoundOrigin {
    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(&self) -> &'static str {
        match self {
            Self::Assumed { .. } => "assumed",
            Self::ObservedMaximum { .. } => "observed-maximum",
            Self::ExternallySupplied { .. } => "externally-supplied",
            Self::AnalyticallyEstablished { .. } => "analytically-established",
        }
    }

    /// Whether this origin makes the value an established upper bound.
    ///
    /// `ObservedMaximum` returns `false` **with or without** a safety factor. §7.3 is explicit,
    /// and the alternative — letting a multiplier promote an observation — is how an empirical
    /// number acquires the authority of a proof without anyone deciding that it should.
    #[must_use]
    pub const fn is_established(&self) -> bool {
        matches!(
            self,
            Self::ExternallySupplied { .. } | Self::AnalyticallyEstablished { .. }
        )
    }

    /// Whether this origin is empirical — a report of what happened, not of what cannot happen.
    #[must_use]
    pub const fn is_empirical(&self) -> bool {
        matches!(self, Self::ObservedMaximum { .. })
    }
}

/// A numerical bound with everything §7.3 requires it to carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    /// What the number is a bound on.
    pub quantity: String,
    /// The value, in `unit`.
    pub value: u64,
    /// The unit. §7.3 requires units on every bound; a bare number is not a bound.
    pub unit: String,
    /// What the bound applies to — a task, an ISR, a critical section.
    pub scope: String,
    /// The target configuration identity this bound belongs to.
    pub target: String,
    /// The binary identity this bound belongs to.
    ///
    /// §15: "a compiler flag change can invalidate a timing bound even when the eADL
    /// description is identical". A bound not tied to a binary is not tied to anything.
    pub binary: String,
    /// Where the number came from.
    pub origin: BoundOrigin,
}

/// Why a bound was refused.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundDefect {
    /// The field at fault.
    pub field: &'static str,
    /// What is wrong with it.
    pub why: &'static str,
}

impl core::fmt::Display for BoundDefect {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "`{}`: {}", self.field, self.why)
    }
}

impl Bound {
    /// Validate a bound.
    ///
    /// # Errors
    ///
    /// Returns a [`BoundDefect`] when a field §7.3 requires is missing, or when a safety factor
    /// does not pad — a zero denominator, or a factor below one, which narrows an observation
    /// rather than padding it and is almost certainly a mistake.
    pub fn validate(&self) -> Result<(), BoundDefect> {
        let blank = |s: &String| s.trim().is_empty();
        let checks: [(&'static str, bool, &'static str); 5] = [
            (
                "quantity",
                blank(&self.quantity),
                "a bound must say what it bounds",
            ),
            ("unit", blank(&self.unit), "a bare number is not a bound"),
            (
                "scope",
                blank(&self.scope),
                "a bound must name what it applies to",
            ),
            (
                "target",
                blank(&self.target),
                "a bound belongs to a target configuration",
            ),
            (
                "binary",
                blank(&self.binary),
                "a bound not tied to a binary is invalidated by any rebuild and nobody can tell",
            ),
        ];
        for (field, bad, why) in checks {
            if bad {
                return Err(BoundDefect { field, why });
            }
        }
        if let BoundOrigin::ObservedMaximum {
            safety_factor: Some(factor),
            ..
        } = self.origin
        {
            if !factor.pads() {
                return Err(BoundDefect {
                    field: "safety_factor",
                    why: "a safety factor below 1 shrinks the observation rather than padding it",
                });
            }
        }
        Ok(())
    }

    /// Whether this bound may be presented as established.
    #[must_use]
    pub const fn is_established(&self) -> bool {
        self.origin.is_established()
    }
}

#[cfg(test)]
mod tests {
    use super::{Bound, BoundOrigin, SafetyFactor};

    fn bound(origin: BoundOrigin) -> Bound {
        Bound {
            quantity: "worst-case execution time".into(),
            value: 850,
            unit: "us".into(),
            scope: "task `sense`".into(),
            target: "riscv-virt-up".into(),
            binary: "sha256:1f3a…".into(),
            origin,
        }
    }

    #[test]
    fn an_observation_is_not_a_bound_however_large_the_safety_factor() {
        // ⭐ §7.3: "An observed maximum with a safety multiplier remains an empirical
        // assumption unless a valid argument establishes a bound." This is the assertion the
        // whole module exists for.
        for factor in [
            None,
            Some(SafetyFactor::new(1, 1)),
            Some(SafetyFactor::new(3, 2)),
            Some(SafetyFactor::new(10, 1)),
            Some(SafetyFactor::new(1000, 1)),
        ] {
            let b = bound(BoundOrigin::ObservedMaximum {
                conditions: "2000 runs, worst observed at maximum interference".into(),
                safety_factor: factor,
            });
            b.validate().expect("well formed");
            assert!(
                !b.is_established(),
                "a safety factor of {factor:?} promoted an observation to a bound"
            );
            assert!(b.origin.is_empirical());
        }
    }

    #[test]
    fn an_assumption_is_never_established() {
        let b = bound(BoundOrigin::Assumed {
            rationale: "vendor datasheet figure, not re-derived".into(),
        });
        assert!(!b.is_established());
        assert!(!b.origin.is_empirical());
    }

    #[test]
    fn supplied_and_derived_bounds_are_established() {
        assert!(bound(BoundOrigin::ExternallySupplied {
            source: "WCET tool report r17".into()
        })
        .is_established());
        assert!(bound(BoundOrigin::AnalyticallyEstablished {
            argument: "loop-bound argument in docs/analysis/sense.md".into()
        })
        .is_established());
    }

    #[test]
    fn a_bound_without_a_binary_identity_is_refused() {
        // §15: a compiler flag change invalidates a timing bound while the description is
        // byte-identical. A bound with no binary cannot be invalidated, which is worse.
        let mut b = bound(BoundOrigin::AnalyticallyEstablished {
            argument: "a".into(),
        });
        b.binary = String::new();
        let defect = b.validate().expect_err("must refuse");
        assert_eq!(defect.field, "binary");
        assert!(defect.to_string().contains("rebuild"), "{defect}");
    }

    #[test]
    fn a_bare_number_is_refused() {
        let mut b = bound(BoundOrigin::Assumed {
            rationale: "r".into(),
        });
        b.unit = "  ".into();
        let defect = b.validate().expect_err("must refuse");
        assert_eq!(defect.field, "unit");
    }

    #[test]
    fn a_shrinking_safety_factor_is_refused() {
        let b = bound(BoundOrigin::ObservedMaximum {
            conditions: "c".into(),
            safety_factor: Some(SafetyFactor::new(9, 10)),
        });
        let defect = b.validate().expect_err("must refuse");
        assert_eq!(defect.field, "safety_factor");
    }

    #[test]
    fn a_zero_denominator_factor_is_refused_rather_than_panicking() {
        let b = bound(BoundOrigin::ObservedMaximum {
            conditions: "c".into(),
            safety_factor: Some(SafetyFactor::new(3, 0)),
        });
        let defect = b.validate().expect_err("must refuse");
        assert_eq!(defect.field, "safety_factor");
    }

    #[test]
    fn a_safety_factor_renders_as_an_exact_rational() {
        assert_eq!(SafetyFactor::new(3, 2).to_string(), "3/2");
    }

    #[test]
    fn origin_slugs_are_unique() {
        let origins = [
            BoundOrigin::Assumed {
                rationale: "r".into(),
            },
            BoundOrigin::ObservedMaximum {
                conditions: "c".into(),
                safety_factor: None,
            },
            BoundOrigin::ExternallySupplied { source: "s".into() },
            BoundOrigin::AnalyticallyEstablished {
                argument: "a".into(),
            },
        ];
        let mut slugs: Vec<&str> = origins.iter().map(BoundOrigin::slug).collect();
        let total = slugs.len();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(slugs.len(), total);
    }
}
