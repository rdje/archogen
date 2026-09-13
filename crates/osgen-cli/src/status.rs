//! The outcome vocabulary and its process exit codes.
//!
//! Two families live here and they are deliberately not mixed:
//!
//! * **Diagnostic results** — the `ROADMAP.md` §5.5 vocabulary. These describe what the
//!   toolchain concluded *about the submitted system*. Their names and meanings are part of
//!   the user contract and may not be redefined by implementation convenience.
//! * **Process-level statuses** — `Usage`, `Unimplemented`, `ToolFailure`. These describe
//!   what happened to *the invocation*, never to the system under description. §5.5 is
//!   explicit that a tool failure is never reported as a valid system, so it must not share
//!   a code with any diagnostic result.
//!
//! `Unimplemented` is process-level and temporary: it exists only while a command named in
//! the §10.2 interface target has no implementation behind it, and each occurrence names the
//! task-tree leaf that removes it.

/// A complete outcome of one `osgen` invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The command completed and its result was affirmative.
    Ok,
    /// The invocation itself was malformed: unknown command, unknown option, missing value.
    Usage,
    /// §5.5 — malformed, contradictory, or ill-typed input.
    InvalidDescription,
    /// §5.5 — relevant contract information is unavailable.
    MissingFact,
    /// §5.5 — requested behavior or analysis lies outside implemented semantics.
    UnsupportedProfile,
    /// §5.5 — supported constraints have no satisfying assignment.
    InfeasibleConfiguration,
    /// §5.5 — a resource limit or unresolved bound prevented a conclusion.
    AnalysisInconclusive,
    /// §5.5 — a sufficient analysis did not establish the requested property.
    NotEstablished,
    /// §5.5 — a validated witness violates a named property.
    Counterexample,
    /// The command is part of the §10.2 interface target but is not built yet.
    Unimplemented,
    /// §5.5 — internal failure or unavailable required tool. Never a valid system.
    ToolFailure,
}

impl Status {
    /// The process exit code. Stable: scripts and CI tiers branch on these.
    #[must_use]
    pub const fn code(self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::Usage => 2,
            Self::InvalidDescription => 10,
            Self::MissingFact => 11,
            Self::UnsupportedProfile => 12,
            Self::InfeasibleConfiguration => 13,
            Self::AnalysisInconclusive => 14,
            Self::NotEstablished => 15,
            Self::Counterexample => 16,
            Self::Unimplemented => 20,
            Self::ToolFailure => 70,
        }
    }

    /// The stable machine-readable name, as it appears in reports and diagnostics.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Usage => "usage",
            Self::InvalidDescription => "invalid-description",
            Self::MissingFact => "missing-fact",
            Self::UnsupportedProfile => "unsupported-profile",
            Self::InfeasibleConfiguration => "infeasible-configuration",
            Self::AnalysisInconclusive => "analysis-inconclusive",
            Self::NotEstablished => "not-established",
            Self::Counterexample => "counterexample",
            Self::Unimplemented => "unimplemented",
            Self::ToolFailure => "tool-failure",
        }
    }

    /// Whether this status is one of the `ROADMAP.md` §5.5 diagnostic results, as opposed to
    /// a statement about the invocation. Report generation may only cite the former as a
    /// conclusion about the submitted system.
    #[must_use]
    pub const fn is_diagnostic_result(self) -> bool {
        matches!(
            self,
            Self::InvalidDescription
                | Self::MissingFact
                | Self::UnsupportedProfile
                | Self::InfeasibleConfiguration
                | Self::AnalysisInconclusive
                | Self::NotEstablished
                | Self::Counterexample
                | Self::ToolFailure
        )
    }

    /// The exit code for a `ROADMAP.md` §5.5 verdict.
    ///
    /// `Verdict` is the single source of truth for what a check concluded; this maps it to the
    /// process contract. `verdict_mapping_is_total` asserts every verdict has a code, so the
    /// two cannot drift apart.
    #[must_use]
    pub const fn from_verdict(verdict: eadl_front::Verdict) -> Self {
        use eadl_front::Verdict;
        match verdict {
            Verdict::Ok => Self::Ok,
            Verdict::InvalidDescription => Self::InvalidDescription,
            Verdict::MissingFact => Self::MissingFact,
            Verdict::UnsupportedProfile => Self::UnsupportedProfile,
            Verdict::InfeasibleConfiguration => Self::InfeasibleConfiguration,
            Verdict::AnalysisInconclusive => Self::AnalysisInconclusive,
            Verdict::NotEstablished => Self::NotEstablished,
            Verdict::Counterexample => Self::Counterexample,
            Verdict::ToolFailure => Self::ToolFailure,
        }
    }

    /// Every status, in exit-code order. The single source for the contract table.
    pub const ALL: &'static [Self] = &[
        Self::Ok,
        Self::Usage,
        Self::InvalidDescription,
        Self::MissingFact,
        Self::UnsupportedProfile,
        Self::InfeasibleConfiguration,
        Self::AnalysisInconclusive,
        Self::NotEstablished,
        Self::Counterexample,
        Self::Unimplemented,
        Self::ToolFailure,
    ];
}

#[cfg(test)]
mod tests {
    use super::Status;

    #[test]
    fn exit_codes_are_unique() {
        let mut codes: Vec<i32> = Status::ALL.iter().map(|s| s.code()).collect();
        let total = codes.len();
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(
            codes.len(),
            total,
            "two statuses share an exit code: {codes:?}"
        );
    }

    #[test]
    fn slugs_are_unique() {
        let mut slugs: Vec<&str> = Status::ALL.iter().map(|s| s.slug()).collect();
        let total = slugs.len();
        slugs.sort_unstable();
        slugs.dedup();
        assert_eq!(slugs.len(), total, "two statuses share a slug");
    }

    #[test]
    fn only_ok_succeeds() {
        for status in Status::ALL {
            assert_eq!(
                status.code() == 0,
                *status == Status::Ok,
                "{}",
                status.slug()
            );
        }
    }

    #[test]
    fn tool_failure_is_never_confused_with_a_verdict_about_the_system() {
        // §5.5: a tool failure is "never reported as a valid system". It is a diagnostic
        // result, but it must not share a code with any other outcome, and it must not be Ok.
        assert!(Status::ToolFailure.is_diagnostic_result());
        assert_ne!(Status::ToolFailure.code(), Status::Ok.code());
    }

    #[test]
    fn verdict_mapping_is_total_and_slugs_agree() {
        // ⭐ The check that keeps two vocabularies from becoming two vocabularies. Every §5.5
        // verdict maps to a status, and the two spell it identically — so a new result cannot
        // be added on one side and forgotten on the other.
        for verdict in eadl_front::Verdict::ALL {
            let status = Status::from_verdict(*verdict);
            assert_eq!(
                status.slug(),
                verdict.slug(),
                "verdict `{}` maps to status `{}`",
                verdict.slug(),
                status.slug()
            );
        }
        // And every diagnostic-result status is reachable from some verdict.
        for status in Status::ALL.iter().filter(|s| s.is_diagnostic_result()) {
            assert!(
                eadl_front::Verdict::ALL
                    .iter()
                    .any(|v| Status::from_verdict(*v) == *status),
                "status `{}` has no verdict",
                status.slug()
            );
        }
    }

    #[test]
    fn process_level_statuses_are_not_diagnostic_results() {
        // A malformed command line and an unbuilt command say nothing about the description.
        for status in [Status::Ok, Status::Usage, Status::Unimplemented] {
            assert!(!status.is_diagnostic_result(), "{}", status.slug());
        }
    }
}
