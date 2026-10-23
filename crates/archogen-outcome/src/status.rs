//! The outcome vocabulary and its process exit codes.
//!
//! ⭐ **One vocabulary for every consumer** (leaf `API.3.2`, `docs/decisions/decision_engine-api.md`). It was
//! the CLI's until the engine API was declared, and it moved into the API whole: every API response carries one,
//! and the CLI's exit code is [`Status::code`] of it. A consumer that had its own vocabulary would need a translation
//! table, and a translation table is where one surface's `ok` and another's `unimplemented` drift apart.
//!
//! ⭐ **No role's** (leaf `M3.5.1.1`). It moved on, out of the engine API, when the checkers became executables of
//! their own (`M3.5.1`): each writes its outcome in a result record and exits with its code, and carries this
//! vocabulary without the engine API, so the vocabulary has a version of its own, [`VOCABULARY`]. What a language
//! verdict is in it stays with the engine API (`archogen_api::verdict_status`), so this package compiles nothing of
//! the reader.
//!
//! Two families live here and they are deliberately not mixed:
//!
//! * **Diagnostic results** — the `ROADMAP.md` §5.5 vocabulary. These describe what the
//!   toolchain concluded *about the submitted system*. Their names and meanings are part of
//!   the user contract and may not be redefined by implementation convenience.
//! * **Process-level statuses** — `Usage`, `StaleInput`, `Unimplemented`, `ToolFailure`. These describe
//!   what happened to *the invocation*, never to the system under description. §5.5 is
//!   explicit that a tool failure is never reported as a valid system, so it must not share
//!   a code with any diagnostic result.
//!
//! `Unimplemented` is process-level and temporary: it exists only while a command named in
//! the §10.2 interface target has no implementation behind it, and each occurrence names the
//! task-tree leaf that removes it. `StaleInput` is process-level and permanent: the inputs of one invocation do not
//! belong together, so nothing about the system was judged, and the repair is to run the step that made them again.

use crate::Version;

/// This vocabulary's version, apart from the engine API's, since a checker carries the vocabulary without the API
/// (leaf `M3.5.1.1`).
///
/// **The promise**, the one the engine API made of it (`docs/decisions/decision_engine-api.md` §5): within a major, a
/// status is never removed, renamed, renumbered, moved between families or given a new meaning, so a reader of an
/// earlier minor refuses only a status it does not know. Adding one is a minor. `1.0` is the eleven statuses the
/// engine API carried from leaf `API.3.2`; `1.1` added [`Status::StaleInput`] (leaf `M3.5.1.1`). Each minor's table
/// is frozen in `tests/goldens/`, and `docs/book/src/versions.md` registers it.
pub const VOCABULARY: Version = Version { major: 1, minor: 1 };

/// A complete outcome of one `archogen` invocation, and of one engine API response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The command completed and its result was affirmative.
    Ok,
    /// The invocation itself was malformed: unknown command, unknown option, missing value.
    Usage,
    /// The inputs of the invocation do not belong together — a plan resolved from another description, a build made
    /// from another plan — so nothing about the system was judged. The repair: resolve, or build, again.
    StaleInput,
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
            Self::StaleInput => 3,
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
            Self::StaleInput => "stale-input",
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

    /// The status a slug names, or `None` for one this vocabulary does not hold: a reader meets one only in a record
    /// written under a later minor, or a damaged one, and refuses it rather than guess.
    #[must_use]
    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|status| status.slug() == slug)
    }

    /// The minor of [`VOCABULARY`] this status entered at: a record that names an earlier one cannot hold it.
    #[must_use]
    pub const fn since(self) -> Version {
        const FIRST: Version = Version { major: 1, minor: 0 };
        match self {
            Self::Ok
            | Self::Usage
            | Self::InvalidDescription
            | Self::MissingFact
            | Self::UnsupportedProfile
            | Self::InfeasibleConfiguration
            | Self::AnalysisInconclusive
            | Self::NotEstablished
            | Self::Counterexample
            | Self::Unimplemented
            | Self::ToolFailure => FIRST,
            Self::StaleInput => Version { major: 1, minor: 1 },
        }
    }

    /// Every status, in exit-code order. The single source for the contract table.
    pub const ALL: &'static [Self] = &[
        Self::Ok,
        Self::Usage,
        Self::StaleInput,
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
    fn process_level_statuses_are_not_diagnostic_results() {
        // A malformed command line, inputs that do not belong together and an unbuilt command say nothing about the
        // description.
        for status in [
            Status::Ok,
            Status::Usage,
            Status::StaleInput,
            Status::Unimplemented,
        ] {
            assert!(!status.is_diagnostic_result(), "{}", status.slug());
        }
    }

    #[test]
    fn stale_input_is_three_and_collides_with_nothing() {
        // The tiers use 0, 1 and 20; `xtask`'s judge reads 3 as failed, never as passed (`M3.5.1`).
        assert_eq!(Status::StaleInput.code(), 3);
        assert_eq!(Status::StaleInput.slug(), "stale-input");
        assert_eq!(
            Status::ALL.iter().filter(|s| s.code() == 3).count(),
            1,
            "one status holds exit 3"
        );
    }

    #[test]
    fn every_status_is_named_by_its_slug_and_nothing_else_is() {
        for status in Status::ALL {
            assert_eq!(Status::from_slug(status.slug()), Some(*status));
        }
        for slug in [
            "",
            "OK",
            "stale",
            "stale_input",
            "tool-failure ",
            "established",
        ] {
            assert_eq!(Status::from_slug(slug), None, "{slug:?}");
        }
    }

    #[test]
    fn every_status_entered_at_a_minor_the_vocabulary_has_reached() {
        for status in Status::ALL {
            assert!(status.since() <= super::VOCABULARY, "{}", status.slug());
            assert_eq!(status.since().major, super::VOCABULARY.major);
        }
    }
}
