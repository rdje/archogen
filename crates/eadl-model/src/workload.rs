//! The admitted workload model of `ROADMAP.md` §3.1, enforced.
//!
//! `rt-static-up-v1` states its **Workload** row as:
//!
//! > Finite static task set; periodic or sporadic releases with declared minimum separation;
//! > constrained deadlines; bounded release jitter.
//!
//! ⛔ **THIS ROW WAS PROSE.** [`crate::profile::Profile`] carries two lists, and only one of them
//! was ever consulted: `exclusions` drives profile admission in [`crate::check`], while
//! `decisions` — all thirteen rows, this one included — was read by exactly one place, the test
//! that compares it against the published profile page. So the checker knew which capabilities
//! the profile *refuses* and nothing at all about the task model it *admits*, and
//! `archogen check` accepted two tasks at the same priority against a profile whose own table
//! says priorities are unique.
//!
//! That is worse than an unimplemented check. A description accepted here is one a later stage
//! is entitled to trust: the S0 realization orders coincident releases by priority rank, which
//! is a total order only because priorities are unique. A rule stated in the profile and
//! enforced nowhere is a rule every consumer has to re-implement, and the copies drift.
//!
//! # The four rules
//!
//! | Rule | Verdict when broken | Why that verdict |
//! |---|---|---|
//! | a release model is declared | `missing-fact` | §5.3 — the arrival model is a relevant fact, and it is simply undescribed |
//! | not both `period` and `min-separation` | `invalid-description` | §5.5 — two release models is contradictory, not merely unsupported |
//! | priorities are unique | `unsupported-profile` | the profile admits one task per priority; another profile could admit more |
//! | deadlines are constrained (`D ≤ T`) | `unsupported-profile` | an arbitrary-deadline model needs a different analysis, not a wider version of this one |
//!
//! The last two are `unsupported-profile` rather than `invalid-description` for the reason §3.1
//! gives: the request is refused **rather than silently reduced to a weaker guarantee**, and the
//! refusal says what admitting it would cost. Neither description is wrong about anything; both
//! describe a system this profile does not analyze.
//!
//! # What is deliberately *not* here
//!
//! "Finite static task set" and "bounded release jitter" need no check: the task set is whatever
//! the description lists, and `dynamic-task-creation` is already an exclusion; an absent `jitter`
//! clause means zero, which is bounded. The remaining twelve `decisions` rows are **counted** by
//! this module's `how_many_profile_decisions_are_still_prose` test and **classified** by leaf
//! `M1.10`, which decides for each one whether it is enforceable here, enforceable at a later
//! stage, or not a checkable rule at all. Twelve unenforced rows is not twelve defects — but
//! twelve rows nobody had counted was the state this module exists to leave behind.

use std::collections::BTreeMap;

use eadl_front::{Diagnostic, Form, Label, Span};

use crate::quantity::Quantity;

/// One task, as the workload rules see it.
struct Task<'a> {
    name: &'a str,
    span: Span,
    /// `(period …)` or `(min-separation …)`, whichever is declared.
    period: Option<&'a Form>,
    min_separation: Option<&'a Form>,
    deadline: Option<&'a Form>,
    priority: Option<(i64, Span)>,
}

/// Check every system's task set against the admitted workload model.
///
/// Returns one diagnostic per violation, in source order. A clause the schema already rejected
/// is skipped rather than reported twice: "`priority` is missing" is the schema's message, and
/// repeating it here would make one mistake look like two.
#[must_use]
pub fn check(forms: &[Form]) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    for system in forms.iter().filter(|f| f.head() == Some("defsystem")) {
        let tasks = collect(system);
        for task in &tasks {
            release_model(task, &mut diagnostics);
            constrained_deadline(task, &mut diagnostics);
        }
        unique_priorities(&tasks, &mut diagnostics);
    }
    diagnostics
}

fn collect(system: &Form) -> Vec<Task<'_>> {
    system
        .items()
        .iter()
        .filter(|clause| clause.head() == Some("task"))
        .map(|task| Task {
            name: task.items().get(1).and_then(Form::as_symbol).unwrap_or("?"),
            span: task.span(),
            period: clause(task, "period"),
            min_separation: clause(task, "min-separation"),
            deadline: clause(task, "deadline"),
            priority: match clause(task, "priority").and_then(|c| c.items().get(1)) {
                Some(Form::Integer { value, span }) => Some((*value, *span)),
                _ => None,
            },
        })
        .collect()
}

fn clause<'a>(task: &'a Form, name: &str) -> Option<&'a Form> {
    task.items().iter().find(|item| item.head() == Some(name))
}

/// §3.1: "periodic **or** sporadic releases with declared minimum separation".
fn release_model(task: &Task<'_>, out: &mut Vec<Diagnostic>) {
    match (task.period, task.min_separation) {
        (None, None) => out.push(Diagnostic::error(
            "missing-fact",
            format!("task `{}` declares no release model", task.name),
            Label::new(task.span, "neither `period` nor `min-separation`"),
            "declare `(period <t>)` for a periodic task or `(min-separation <t>)` for a \
             sporadic one. §3.1 admits both, and the analysis needs one of them: without an \
             arrival model there is no interval over which interference can be bounded",
        )),
        (Some(period), Some(separation)) => out.push(
            Diagnostic::error(
                "invalid-description",
                format!("task `{}` declares two release models", task.name),
                Label::new(separation.span(), "and a minimum separation here"),
                "keep one. A periodic task's period *is* its minimum separation, so declaring \
                 both leaves it ambiguous which one the analysis should use — and §5.3 requires \
                 contradictory declarations to be rejected rather than one of them chosen",
            )
            .with_secondary(Label::new(period.span(), "a period is declared here")),
        ),
        _ => {}
    }
}

/// §3.1: "constrained deadlines" — the relative deadline may not exceed the release separation.
fn constrained_deadline(task: &Task<'_>, out: &mut Vec<Diagnostic>) {
    let Some(deadline) = task.deadline else {
        return;
    };
    let Some(separation) = task.period.or(task.min_separation) else {
        // No release model: `release_model` already said so, and comparing against nothing
        // would turn one mistake into two.
        return;
    };
    let (Ok(deadline_q), Ok(separation_q)) = (quantity(deadline), quantity(separation)) else {
        // A malformed quantity is F03's business and is reported by the schema/unit pass.
        return;
    };
    let Ok(ordering) = deadline_q.compare(separation_q) else {
        return;
    };
    if ordering.is_gt() {
        let separation_kind = if task.period.is_some() {
            "period"
        } else {
            "minimum separation"
        };
        out.push(
            Diagnostic::error(
                "unsupported-profile",
                format!(
                    "task `{}` has a deadline of {deadline_q}, longer than its {separation_kind} of {separation_q}",
                    task.name
                ),
                Label::new(deadline.span(), "deadline exceeds the release separation"),
                format!(
                    "`rt-static-up-v1` admits **constrained** deadlines (D ≤ T). Shorten the \
                     deadline to at most {separation_q}, or lengthen the {separation_kind}. \
                     Admitting D > T is not a wider version of this profile: with more than one \
                     job of a task alive at once, the response-time recurrence no longer applies \
                     and a busy-period analysis is required instead"
                ),
            )
            .with_secondary(Label::new(
                separation.span(),
                format!("the {separation_kind} is declared here"),
            )),
        );
    }
}

fn quantity(clause: &Form) -> Result<Quantity, ()> {
    Quantity::read(clause.items().get(1), clause.items().get(2)).map_err(|_| ())
}

/// §3.1: "static **unique** task priorities".
fn unique_priorities(tasks: &[Task<'_>], out: &mut Vec<Diagnostic>) {
    let mut seen: BTreeMap<i64, (&str, Span)> = BTreeMap::new();
    for task in tasks {
        let Some((priority, span)) = task.priority else {
            continue;
        };
        if let Some((first, first_span)) = seen.get(&priority) {
            out.push(
                Diagnostic::error(
                    "unsupported-profile",
                    format!(
                        "tasks `{first}` and `{}` share priority {priority}",
                        task.name
                    ),
                    Label::new(span, "this priority is already taken"),
                    "`rt-static-up-v1` admits **static unique** task priorities. Give one of \
                     them a different number. Equal priorities need a documented tie-break — \
                     FIFO, round-robin — which is a different scheduling policy with its own \
                     analysis, so it belongs to a different profile rather than to a looser \
                     reading of this one",
                )
                .with_secondary(Label::new(*first_span, "first declared here")),
            );
        } else {
            seen.insert(priority, (task.name, span));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::check;
    use eadl_front::{read, SourceMap};

    fn diagnose(text: &str) -> Vec<String> {
        let mut sources = SourceMap::new();
        let id = sources.add("test", text.to_string()).expect("small");
        let (document, diagnostics) = read(&sources, id);
        assert!(!diagnostics.has_errors(), "the fixture must read cleanly");
        check(&document.forms)
            .iter()
            .map(|d| format!("{}: {}", d.code, d.message))
            .collect()
    }

    /// A two-task system, with each task's clauses supplied by the caller.
    fn system(first: &str, second: &str) -> String {
        format!("(defsystem s (task a {first}) (task b {second}))")
    }

    const OK_A: &str = "(period 10 ms) (deadline 10 ms) (priority 1)";
    const OK_B: &str = "(period 30 ms) (deadline 30 ms) (priority 2)";

    #[test]
    fn an_admitted_task_set_produces_nothing() {
        assert!(diagnose(&system(OK_A, OK_B)).is_empty());
    }

    #[test]
    fn two_tasks_at_one_priority_are_refused_by_the_profile() {
        let found = diagnose(&system(
            OK_A,
            "(period 30 ms) (deadline 30 ms) (priority 1)",
        ));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("unsupported-profile: "), "{found:?}");
        assert!(found[0].contains("share priority 1"), "{found:?}");
    }

    #[test]
    fn three_tasks_at_one_priority_report_each_collision_once() {
        // Not once per pair: three tasks at one priority is two mistakes to fix, not three.
        let text = "(defsystem s (task a (period 10 ms) (priority 1)) \
                    (task b (period 10 ms) (priority 1)) (task c (period 10 ms) (priority 1)))";
        let found: Vec<String> = diagnose(text)
            .into_iter()
            .filter(|d| d.contains("share priority"))
            .collect();
        assert_eq!(found.len(), 2, "{found:?}");
    }

    #[test]
    fn a_deadline_longer_than_the_period_is_refused_by_the_profile() {
        let found = diagnose(&system(
            "(period 10 ms) (deadline 20 ms) (priority 1)",
            OK_B,
        ));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("unsupported-profile: "), "{found:?}");
        assert!(found[0].contains("longer than its period"), "{found:?}");
    }

    #[test]
    fn a_deadline_equal_to_the_period_is_admitted() {
        // D ≤ T, not D < T. An implicit-deadline task set is the commonest admitted case, and
        // an off-by-one here would reject every example in the repository.
        assert!(diagnose(&system(OK_A, OK_B)).is_empty());
    }

    #[test]
    fn the_deadline_comparison_crosses_units() {
        // `1 s` is longer than `100 ms`, and a checker that compared magnitudes would miss it.
        let found = diagnose(&system("(period 100 ms) (deadline 1 s) (priority 1)", OK_B));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("longer than its period"), "{found:?}");
        // …and the same relation written the other way round is fine.
        assert!(diagnose(&system("(period 1 s) (deadline 100 ms) (priority 1)", OK_B)).is_empty());
    }

    #[test]
    fn a_sporadic_deadline_is_compared_against_the_minimum_separation() {
        let found = diagnose(&system(
            "(min-separation 10 ms) (deadline 20 ms) (priority 1)",
            OK_B,
        ));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("longer than its minimum separation"),
            "{found:?}"
        );
    }

    #[test]
    fn a_task_with_no_release_model_is_a_missing_fact() {
        let found = diagnose(&system("(deadline 10 ms) (priority 1)", OK_B));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("missing-fact: "), "{found:?}");
    }

    #[test]
    fn a_task_with_no_release_model_is_not_also_reported_for_its_deadline() {
        // One mistake, one message. The deadline cannot be compared against a separation that
        // was never declared, and saying so twice sends the author to fix the wrong thing.
        let found = diagnose(&system("(deadline 10 ms) (priority 1)", OK_B));
        assert_eq!(found.len(), 1, "{found:?}");
    }

    #[test]
    fn declaring_both_release_models_is_contradictory() {
        let found = diagnose(&system(
            "(period 10 ms) (min-separation 30 ms) (deadline 10 ms) (priority 1)",
            OK_B,
        ));
        assert!(
            found.iter().any(|d| d.starts_with("invalid-description: ")),
            "{found:?}"
        );
    }

    #[test]
    fn a_missing_priority_is_left_to_the_schema() {
        // `priority` has cardinality `one`, so the schema already refuses this. Reporting it
        // again here would make one mistake look like two.
        let found = diagnose(&system("(period 10 ms) (deadline 10 ms)", OK_B));
        assert!(found.is_empty(), "{found:?}");
    }

    #[test]
    fn a_description_with_no_system_is_not_this_module_s_business() {
        assert!(diagnose("(defblock b (offers (x true)))").is_empty());
    }

    #[test]
    fn how_many_profile_decisions_are_still_prose() {
        // ⭐ The census this module exists to stop being unknown. `decisions` has thirteen rows;
        // this module enforces the Workload row. If a later leaf enforces another, this number
        // moves and the test says so rather than letting the gap drift silently.
        assert_eq!(
            crate::profile::RT_STATIC_UP_V1.decisions.len(),
            13,
            "the profile's decision table changed size — re-count what is enforced"
        );
        let enforced = ["Workload"];
        let unenforced: Vec<&str> = crate::profile::RT_STATIC_UP_V1
            .decisions
            .iter()
            .map(|row| row.concern)
            .filter(|concern| !enforced.contains(concern))
            .collect();
        assert_eq!(
            unenforced.len(),
            12,
            "still prose, classified by leaf M1.10: {unenforced:?}"
        );
    }
}
