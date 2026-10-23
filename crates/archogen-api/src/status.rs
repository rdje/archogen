//! What a verdict of the language's check is in the outcome vocabulary.
//!
//! The vocabulary is `archogen-outcome`'s, no role's (leaf `M3.5.1.1`): a checker writes its outcome in a result
//! record and exits with its code without compiling the engine API. This mapping stays here, with the API that judges
//! descriptions, so the vocabulary's package compiles nothing of the language's reader.

use archogen_outcome::Status;
use eadl_front::Verdict;

/// The status for a `ROADMAP.md` §5.5 verdict.
///
/// `Verdict` is the single source of truth for what a check concluded; this maps it to the process contract.
/// `verdict_mapping_is_total_and_slugs_agree` asserts every verdict has a status that spells it the same way, so the
/// two cannot drift apart.
#[must_use]
pub const fn verdict_status(verdict: Verdict) -> Status {
    match verdict {
        Verdict::Ok => Status::Ok,
        Verdict::InvalidDescription => Status::InvalidDescription,
        Verdict::MissingFact => Status::MissingFact,
        Verdict::UnsupportedProfile => Status::UnsupportedProfile,
        Verdict::InfeasibleConfiguration => Status::InfeasibleConfiguration,
        Verdict::AnalysisInconclusive => Status::AnalysisInconclusive,
        Verdict::NotEstablished => Status::NotEstablished,
        Verdict::Counterexample => Status::Counterexample,
        Verdict::ToolFailure => Status::ToolFailure,
    }
}

#[cfg(test)]
mod tests {
    use super::verdict_status;
    use archogen_outcome::Status;

    #[test]
    fn verdict_mapping_is_total_and_slugs_agree() {
        // ⭐ The check that keeps two vocabularies from becoming two vocabularies. Every §5.5
        // verdict maps to a status, and the two spell it identically — so a new result cannot
        // be added on one side and forgotten on the other.
        for verdict in eadl_front::Verdict::ALL {
            let status = verdict_status(*verdict);
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
                    .any(|v| verdict_status(*v) == *status),
                "status `{}` has no verdict",
                status.slug()
            );
        }
    }

    #[test]
    fn no_verdict_is_a_statement_about_the_invocation() {
        // A verdict says what the check concluded about a description; the process-level statuses — a malformed
        // command line, inputs that do not belong together, an unbuilt command — are never one.
        for verdict in eadl_front::Verdict::ALL {
            let status = verdict_status(*verdict);
            assert!(
                status == Status::Ok || status.is_diagnostic_result(),
                "verdict `{}` maps to the process-level `{}`",
                verdict.slug(),
                status.slug()
            );
        }
    }
}
