//! The trust-dependency vocabulary (`ROADMAP.md` §4.4, §14.4).
//!
//! The generator and the independent configuration checker are supposed to be independent. In
//! practice they share code, data and ideas, and §4.4's position is not that sharing is
//! forbidden — it is that sharing must be **visible**:
//!
//! > A checker acquiring a dependency on the generator's constraint-evaluation implementation
//! > must be visible and block automatic acceptance until its loss of independence is
//! > explicitly addressed. **Merely auto-updating the baseline is not acceptance.**
//!
//! This module is the vocabulary that makes such a statement expressible: which roots exist,
//! what an item's role is, and what counts as newly shared. The inventory *producer* — rooted
//! in the real build manifest — is leaf `M3.6`, and the gate that blocks on drift is F30.
//!
//! ⚠️ **Honest limit, stated rather than hidden.** §4.4 is blunt about what this cannot do:
//! "Different crate names do not establish independent derivation. Copied code, shared
//! generated formulas, and common source misinterpretations still require provenance and
//! review. This check enforces disclosure and change control; **it does not prove semantic
//! independence.**" Two separately written implementations of the same misread specification
//! share nothing this module can see.

/// A root whose reachable dependencies are inventoried (`ROADMAP.md` §4.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustRoot {
    /// The generator: elaboration, resolution, planning, emission.
    Generator,
    /// The independent configuration checker.
    ConfigurationChecker,
    /// The scheduling checker.
    SchedulingChecker,
    /// The reference-model build used for behavioral comparison.
    ReferenceModelBuild,
}

impl TrustRoot {
    /// Every inventoried root.
    pub const ALL: &'static [Self] = &[
        Self::Generator,
        Self::ConfigurationChecker,
        Self::SchedulingChecker,
        Self::ReferenceModelBuild,
    ];

    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Generator => "generator",
            Self::ConfigurationChecker => "configuration-checker",
            Self::SchedulingChecker => "scheduling-checker",
            Self::ReferenceModelBuild => "reference-model-build",
        }
    }

    /// Whether this root is supposed to reach its verdict independently of the generator.
    ///
    /// This is what makes sharing *matter*: an item shared between two roots that are both
    /// meant to be independent of the generator is a different event from one shared between
    /// the generator and its own planner.
    #[must_use]
    pub const fn must_be_independent_of_generator(self) -> bool {
        matches!(
            self,
            Self::ConfigurationChecker | Self::SchedulingChecker | Self::ReferenceModelBuild
        )
    }
}

/// What an inventoried item contributes (`ROADMAP.md` §4.4 classification).
///
/// The ordering matters: it is severity. A shared logging crate and a shared constraint
/// evaluator are both "shared", and treating them the same way produces a gate that authors
/// learn to wave through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustRole {
    /// Allocation, formatting, iteration — carries no semantics of the claim.
    Infrastructure,
    /// Parsing, canonicalization, normalization — shapes what both sides *see*.
    InterpretationNormalization,
    /// Constraint evaluation, analysis, decision logic — shapes what both sides *conclude*.
    SemanticAnalysis,
    /// Facts, tables, specifications both sides rely on being right.
    AuthoritativeData,
    /// Code or data from which a reference model is derived.
    ReferenceDerivation,
}

impl TrustRole {
    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Infrastructure => "infrastructure",
            Self::InterpretationNormalization => "interpretation-normalization",
            Self::SemanticAnalysis => "semantic-analysis",
            Self::AuthoritativeData => "authoritative-data",
            Self::ReferenceDerivation => "reference-derivation",
        }
    }

    /// Whether sharing an item in this role between a root and the generator costs
    /// independence, rather than merely being worth recording.
    ///
    /// Infrastructure is the only role that does not: a shared allocator cannot make two
    /// implementations agree on a wrong answer. Everything else can.
    #[must_use]
    pub const fn compromises_independence(self) -> bool {
        !matches!(self, Self::Infrastructure)
    }
}

/// One item in a trust inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustItem {
    /// Stable identity — a package name, a data-file path, a tool name.
    pub id: String,
    /// The exact version or revision inventoried.
    pub version: String,
    /// Content hash of the item as it was actually used.
    ///
    /// §14.4 case 2 requires detecting "a feature or change source content in an already
    /// shared package … even when package names and versions are unchanged". The hash is what
    /// makes that possible; a name and a version are not enough.
    pub source_hash: String,
    /// What it contributes.
    pub role: TrustRole,
    /// Which roots reach it. Sorted and deduplicated by [`TrustItem::new`].
    pub reachable_from: Vec<TrustRoot>,
    /// Any declared over-approximation or known coverage gap for this item.
    ///
    /// §4.4: "declare any conservative over-approximation or missing coverage." An inventory
    /// that silently omits what it could not see is worse than one that says so.
    pub coverage_note: Option<String>,
}

impl TrustItem {
    /// Build an item, normalizing its root list.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        version: impl Into<String>,
        source_hash: impl Into<String>,
        role: TrustRole,
        reachable_from: &[TrustRoot],
    ) -> Self {
        let mut roots = reachable_from.to_vec();
        roots.sort_unstable();
        roots.dedup();
        Self {
            id: id.into(),
            version: version.into(),
            source_hash: source_hash.into(),
            role,
            reachable_from: roots,
            coverage_note: None,
        }
    }

    /// Whether more than one root reaches this item.
    #[must_use]
    pub fn is_shared(&self) -> bool {
        self.reachable_from.len() > 1
    }

    /// Whether this item costs a root its claimed independence from the generator.
    ///
    /// True when the generator reaches it, a root that must be independent of the generator
    /// also reaches it, and the role is one that can carry a shared mistake.
    #[must_use]
    pub fn compromises_independence(&self) -> bool {
        let generator = self.reachable_from.contains(&TrustRoot::Generator);
        let independent = self
            .reachable_from
            .iter()
            .any(|root| root.must_be_independent_of_generator());
        generator && independent && self.role.compromises_independence()
    }

    /// A one-line inventory rendering.
    #[must_use]
    pub fn render(&self) -> String {
        let roots: Vec<&str> = self.reachable_from.iter().map(|r| r.slug()).collect();
        let flag = if self.compromises_independence() {
            "  ** independence **"
        } else {
            ""
        };
        format!(
            "{} {} [{}] {} <- {}{}",
            self.id,
            self.version,
            self.role.slug(),
            self.source_hash,
            roots.join(", "),
            flag
        )
    }

    /// How this item differs from its reviewed baseline, if it does.
    ///
    /// §14.4: the report "lists both newly shared dependencies and changed approved ones; it
    /// does not hide them behind an unchanged crate name or version".
    #[must_use]
    pub fn drift_from(&self, baseline: Option<&Self>) -> Option<TrustDrift> {
        match baseline {
            None => Some(TrustDrift::NewItem),
            Some(previous) => {
                if previous.source_hash != self.source_hash {
                    Some(TrustDrift::ContentChanged {
                        from: previous.source_hash.clone(),
                        to: self.source_hash.clone(),
                    })
                } else if previous.reachable_from != self.reachable_from {
                    Some(TrustDrift::SharingChanged {
                        from: previous.reachable_from.clone(),
                        to: self.reachable_from.clone(),
                    })
                } else if previous.role != self.role {
                    Some(TrustDrift::RoleChanged {
                        from: previous.role,
                        to: self.role,
                    })
                } else {
                    None
                }
            }
        }
    }
}

/// How an inventoried item differs from its reviewed baseline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrustDrift {
    /// The item was not in the baseline at all.
    NewItem,
    /// Same identity and version, different content.
    ContentChanged {
        /// Baseline hash.
        from: String,
        /// Current hash.
        to: String,
    },
    /// The set of roots that reach it changed.
    SharingChanged {
        /// Baseline roots.
        from: Vec<TrustRoot>,
        /// Current roots.
        to: Vec<TrustRoot>,
    },
    /// Its classification changed.
    RoleChanged {
        /// Baseline role.
        from: TrustRole,
        /// Current role.
        to: TrustRole,
    },
}

#[cfg(test)]
mod tests {
    use super::{TrustDrift, TrustItem, TrustRole, TrustRoot};

    fn item(role: TrustRole, roots: &[TrustRoot]) -> TrustItem {
        TrustItem::new("eadl-normalize", "0.2.0", "sha256:aaaa", role, roots)
    }

    #[test]
    fn a_semantic_helper_reached_by_generator_and_checker_costs_independence() {
        // §14.4 case 1: "a transitive semantic helper used by both generator and checker: the
        // new shared path is reported and applicable acceptance fails pending review".
        let shared = item(
            TrustRole::SemanticAnalysis,
            &[TrustRoot::Generator, TrustRoot::ConfigurationChecker],
        );
        assert!(shared.is_shared());
        assert!(shared.compromises_independence());
        assert!(shared.render().contains("** independence **"));
    }

    #[test]
    fn shared_infrastructure_is_recorded_but_does_not_cost_independence() {
        // A gate that treats a shared allocator like a shared constraint evaluator is a gate
        // authors learn to wave through.
        let shared = item(
            TrustRole::Infrastructure,
            &[TrustRoot::Generator, TrustRoot::ConfigurationChecker],
        );
        assert!(shared.is_shared());
        assert!(!shared.compromises_independence());
    }

    #[test]
    fn generator_internal_sharing_is_not_an_independence_event() {
        let internal = item(TrustRole::SemanticAnalysis, &[TrustRoot::Generator]);
        assert!(!internal.is_shared());
        assert!(!internal.compromises_independence());
    }

    #[test]
    fn content_change_is_detected_behind_an_unchanged_name_and_version() {
        // §14.4 case 2: detected "even when package names and versions are unchanged".
        let baseline = item(TrustRole::SemanticAnalysis, &[TrustRoot::Generator]);
        let mut current = baseline.clone();
        current.source_hash = "sha256:bbbb".into();
        assert_eq!(current.id, baseline.id);
        assert_eq!(current.version, baseline.version);
        match current.drift_from(Some(&baseline)) {
            Some(TrustDrift::ContentChanged { from, to }) => {
                assert_eq!(from, "sha256:aaaa");
                assert_eq!(to, "sha256:bbbb");
            }
            other => panic!("content change not detected: {other:?}"),
        }
    }

    #[test]
    fn newly_acquired_sharing_is_detected() {
        let baseline = item(TrustRole::SemanticAnalysis, &[TrustRoot::Generator]);
        let current = item(
            TrustRole::SemanticAnalysis,
            &[TrustRoot::Generator, TrustRoot::SchedulingChecker],
        );
        assert!(matches!(
            current.drift_from(Some(&baseline)),
            Some(TrustDrift::SharingChanged { .. })
        ));
        assert!(current.compromises_independence());
    }

    #[test]
    fn an_unrelated_item_produces_no_drift_and_no_fabricated_warning() {
        // §14.4 case 5: "an unrelated change outside the recorded roots and provenance: the
        // relevant trust graph is unchanged and no fabricated loss-of-independence warning is
        // produced." A gate that cries wolf is a gate that gets disabled.
        let baseline = item(TrustRole::Infrastructure, &[TrustRoot::Generator]);
        let current = baseline.clone();
        assert!(current.drift_from(Some(&baseline)).is_none());
        assert!(!current.compromises_independence());
        assert!(!current.render().contains("independence"));
    }

    #[test]
    fn an_item_absent_from_the_baseline_is_new() {
        let current = item(TrustRole::AuthoritativeData, &[TrustRoot::Generator]);
        assert_eq!(current.drift_from(None), Some(TrustDrift::NewItem));
    }

    #[test]
    fn roots_are_normalized_so_ordering_is_not_mistaken_for_drift() {
        let a = item(
            TrustRole::Infrastructure,
            &[TrustRoot::ConfigurationChecker, TrustRoot::Generator],
        );
        let b = item(
            TrustRole::Infrastructure,
            &[
                TrustRoot::Generator,
                TrustRoot::ConfigurationChecker,
                TrustRoot::Generator,
            ],
        );
        assert_eq!(a.reachable_from, b.reachable_from);
        assert!(a.drift_from(Some(&b)).is_none());
    }

    #[test]
    fn every_root_except_the_generator_must_be_independent_of_it() {
        for root in TrustRoot::ALL {
            assert_eq!(
                root.must_be_independent_of_generator(),
                *root != TrustRoot::Generator,
                "{}",
                root.slug()
            );
        }
    }

    #[test]
    fn role_severity_orders_infrastructure_lowest() {
        assert!(TrustRole::Infrastructure < TrustRole::InterpretationNormalization);
        assert!(TrustRole::InterpretationNormalization < TrustRole::SemanticAnalysis);
    }

    #[test]
    fn slugs_are_unique() {
        let roles = [
            TrustRole::Infrastructure,
            TrustRole::InterpretationNormalization,
            TrustRole::SemanticAnalysis,
            TrustRole::AuthoritativeData,
            TrustRole::ReferenceDerivation,
        ];
        let mut slugs: Vec<&str> = roles.iter().map(|r| r.slug()).collect();
        slugs.extend(TrustRoot::ALL.iter().map(|r| r.slug()));
        let total = slugs.len();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(slugs.len(), total);
    }
}
