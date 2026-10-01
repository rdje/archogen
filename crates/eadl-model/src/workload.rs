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
//! # The rules
//!
//! | Rule | Verdict when broken | Why that verdict |
//! |---|---|---|
//! | a release model is declared | `missing-fact` | §5.3 — the arrival model is a relevant fact, and it is simply undescribed |
//! | not both `period` and `min-separation` | `invalid-description` | §5.5 — two release models is contradictory, not merely unsupported |
//! | a priority is a rank, 1 or more | `priority-below-one`, an `invalid-description` | the language's, not the profile's: a value below 1 names no rank (leaf `M2.13`) |
//! | priorities are unique | `unsupported-profile` | the profile admits one task per priority; another profile could admit more |
//! | the overrun policy is one the profile performs | `unsupported-profile` | `rt-static-up-v1` performs `fault`, its default, and `skip-late-job` (the fault contract's rule 5); another profile could perform more (leaf `M2.14`) |
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
//! clause means zero, which is bounded. Every `decisions` row is now **classified**, part by part, beside
//! its text in [`crate::profile`] (leaf `M1.10`): enforced by this checker, enforced by a verification step,
//! owned by the leaf that builds a later stage, or not a rule at all, with the reason. This module's
//! `every_profile_decision_is_classified_by_the_stage_that_enforces_it` test holds each exclusion and each
//! owner the classification names to something that exists.

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
    /// `(on-overrun <symbol>)`, when declared and a symbol; the schema refuses any other value.
    on_overrun: Option<(&'a str, Span)>,
}

/// The overrun policies `rt-static-up-v1`'s runtime performs (the fault contract's rule 5), in eADL's
/// spelling. A task without the clause has the first, the profile's default.
pub const OVERRUN_POLICIES: [&str; 2] = ["fault", "skip-late-job"];

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
            rank(task, &mut diagnostics);
            overrun_policy(task, &mut diagnostics);
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
            on_overrun: clause(task, "on-overrun")
                .and_then(|c| c.items().get(1))
                .and_then(|value| value.as_symbol().map(|name| (name, value.span()))),
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
        // ⭐ Not a discard, and the difference is that the pass this names now exists. `period`,
        // `min-separation`, `deadline` and `jitter` are declared `(holds values quantity)` in
        // `docs/semantics/kinds/os-rt.eadl`, so the **schema** pass has already refused a malformed one
        // with `crates/eadl-model/src/quantity.rs`'s own diagnostic before this pass runs — reporting it
        // again here would hand the author the same refusal twice. Until leaf `M1.28` this comment named
        // a pass that did not exist, which is why the discard read as safe.
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

/// `docs/decisions/decision_priority-comparison-direction.md`: a priority is a rank, an integer from 1,
/// and 1 is the highest. A value below 1 names no rank — admitting one would move the top of the range
/// by inference, which `ROADMAP.md` §15 makes a versioned language change — and the runtime refuses to
/// build it (`rt_core::Scheduler::from_eadl_ranks`). Ranks need not be contiguous; only their order
/// is used.
fn rank(task: &Task<'_>, out: &mut Vec<Diagnostic>) {
    let Some((priority, span)) = task.priority else {
        return;
    };
    if priority < 1 {
        out.push(Diagnostic::error(
            "priority-below-one",
            format!("task `{}` has priority {priority}, below 1", task.name),
            Label::new(span, "a priority is a rank from 1"),
            "write a rank of 1 or more. 1 is the highest priority and a larger number is a lower \
             one, so the task meant to run first is `(priority 1)`; a value below 1 names no \
             priority, and the runtime refuses to build one",
        ));
    }
}

/// §3.1: a "defined overrun … policy". `rt-static-up-v1`'s runtime performs two (rule 5 of the fault contract,
/// `docs/profiles/rt-static-up-v1-faults.md`), so a policy it cannot perform is refused rather than lowered onto something it is not. An absent clause
/// is `fault`, the profile's default, and needs no check.
fn overrun_policy(task: &Task<'_>, out: &mut Vec<Diagnostic>) {
    let Some((policy, span)) = task.on_overrun else {
        return;
    };
    if !OVERRUN_POLICIES.contains(&policy) {
        out.push(Diagnostic::error(
            "unsupported-profile",
            format!(
                "task `{}` declares the overrun policy `{policy}`, which `rt-static-up-v1` does not perform",
                task.name
            ),
            Label::new(span, "not an overrun policy of this profile"),
            "write `(on-overrun fault)`, the default — the task leaves the schedule and every other task \
             continues — or `(on-overrun skip-late-job)`, which abandons the late job and lets the release \
             that found it start the next. A response this runtime cannot perform would be a policy nobody \
             analyzed, so it is refused rather than mapped onto one of these",
        ));
    }
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
    fn a_priority_below_one_names_no_rank() {
        for below in ["0", "-3"] {
            let found = diagnose(&system(
                OK_A,
                &format!("(period 30 ms) (deadline 30 ms) (priority {below})"),
            ));
            assert_eq!(found.len(), 1, "{found:?}");
            assert!(found[0].starts_with("priority-below-one: "), "{found:?}");
            assert!(
                found[0].contains(&format!("priority {below}, below 1")),
                "{found:?}"
            );
        }
    }

    #[test]
    fn an_overrun_policy_the_profile_does_not_perform_is_refused() {
        let found = diagnose(&system(
            OK_A,
            "(period 30 ms) (deadline 30 ms) (priority 2) (on-overrun banana)",
        ));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].starts_with("unsupported-profile: "), "{found:?}");
        assert!(found[0].contains("`banana`"), "{found:?}");
    }

    #[test]
    fn both_overrun_policies_and_an_absent_clause_are_admitted() {
        for policy in ["(on-overrun fault)", "(on-overrun skip-late-job)", ""] {
            let task = format!("(period 30 ms) (deadline 30 ms) (priority 2) {policy}");
            assert!(diagnose(&system(OK_A, &task)).is_empty(), "{policy}");
        }
    }

    #[test]
    fn ranks_need_not_be_contiguous() {
        assert!(diagnose(&system(
            OK_A,
            "(period 30 ms) (deadline 30 ms) (priority 9)"
        ))
        .is_empty());
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
    fn a_deadline_longer_by_the_seventeenth_decimal_is_still_refused() {
        // Leaf `M1.34`: in base units both are fractions over 10^26 s, so both cross products pass
        // `i128`. The comparison used to saturate them to the same value and call the two equal —
        // and this system, whose deadline exceeds its period, was admitted.
        let found = diagnose(&system(
            "(period 1.00000000000000001 ns) (deadline 1.00000000000000003 ns) (priority 1)",
            OK_B,
        ));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].contains("longer than its period"), "{found:?}");
        // …and at that precision an equal deadline is still admitted, so the fix is not a refusal of
        // long decimals.
        assert!(diagnose(&system(
            "(period 1.00000000000000001 ns) (deadline 1.00000000000000001 ns) (priority 1)",
            OK_B,
        ))
        .is_empty());
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
    fn every_profile_decision_is_classified_by_the_stage_that_enforces_it() {
        // ⭐ Leaf `M1.10`. This test used to count the rows still prose; it now checks the routing that replaced
        // the count. Every rule is quoted from its row, every exclusion it names exists, every stage not yet built
        // names a leaf a tree declares, and a rule that is not a rule says why.
        use crate::profile::{Stage, RT_STATIC_UP_V1};
        let trees = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(std::path::Path::parent)
            .expect("crates/<name>/ is two levels below the root")
            .join("docs/tasks");
        let declared: String = std::fs::read_dir(&trees)
            .expect("the task trees")
            .flatten()
            .filter_map(|entry| std::fs::read_to_string(entry.path()).ok())
            .collect();
        let mut wrong = Vec::new();
        for row in RT_STATIC_UP_V1.decisions {
            if row.parts.is_empty() {
                wrong.push(format!("`{}` is not classified", row.concern));
            }
            for part in row.parts {
                if !row.decision.contains(part.rule) {
                    wrong.push(format!(
                        "`{}`: `{}` is not in the decision's own words",
                        row.concern, part.rule
                    ));
                }
                match &part.stage {
                    Stage::Check(how) => {
                        // Each `slug` named is an exclusion of this profile, or a path into this crate.
                        for slug in how.split('`').skip(1).step_by(2) {
                            if !slug.contains("::") && RT_STATIC_UP_V1.exclusion(slug).is_none() {
                                wrong.push(format!(
                                    "`{}` names `{slug}`, which is no exclusion of this profile",
                                    row.concern
                                ));
                            }
                        }
                    }
                    Stage::Tier(step) => {
                        if !step.contains("step") {
                            wrong.push(format!("`{}`: a tier part names no step", row.concern));
                        }
                    }
                    Stage::Later { owner, .. } => {
                        if !declared.contains(&format!("- ID: `{owner}`")) {
                            wrong.push(format!(
                                "`{}`: `{owner}` is no leaf a tree declares",
                                row.concern
                            ));
                        }
                    }
                    Stage::NotARule(why) => {
                        if why.len() < 20 {
                            wrong.push(format!(
                                "`{}`: `{}` is not a rule, and says too little why",
                                row.concern, part.rule
                            ));
                        }
                    }
                }
            }
        }
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }
}
