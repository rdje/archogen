//! The §7.4 response-time recurrence, with a checkable witness.
//!
//! ```text
//! R_i^(0)   = C_i
//! R_i^(k+1) = C_i + Σ_{j ∈ hp(i)} ⌈ R_i^(k) / T_j ⌉ · C_j
//! ```
//!
//! §7.4 attaches four requirements to implementing it, and each shows up in the code:
//!
//! | Requirement | Where |
//! |---|---|
//! | "exact integer time units or checked rational arithmetic" | `u64`, every operation `checked_*` |
//! | "upward rounding where needed" | [`ceil_div`], which is the `⌈ ⌉` |
//! | "overflow detection" | [`Bound::Overflow`], a distinct outcome, never a wrapped number |
//! | "explicit convergence/deadline limits" | [`Limit`], two of them, each named in the result |
//! | "record the recurrence sequence as a checkable witness" | [`Witness`], every iterate |
//!
//! # Three outcomes that are easy to collapse and must not be
//!
//! * **converged, within the deadline** — the property holds *in this model*.
//! * **converged, past the deadline** — a concrete witness of violation in this model.
//! * **did not converge, or overflowed** — nothing is established either way.
//!
//! The third is the one a careless implementation turns into "unschedulable", and §7.4 is
//! explicit that it must not: *"Conservative analysis failure is `not-established` unless an
//! exact test or validated counterexample establishes failure."* Reporting a resource limit as a
//! deadline miss invents a counterexample that was never found.
//!
//! # Convergence is bounded by the busy period, not by the deadline
//!
//! A natural optimisation stops the iteration as soon as an iterate exceeds `D`. It is sound for
//! a yes/no answer and it destroys the witness: §13.2 asks for the deadline-3 variant to produce
//! "a concrete deadline-violation witness", and the useful witness is the **converged** response
//! `4`, not the first iterate above `3`. So the iteration runs to its fixed point, bounded by the
//! task's own separation `T` — beyond which the level-`i` busy period does not close — and the
//! comparison with `D` happens afterwards.

use archogen_evidence::claim::Conclusion;

use crate::model::TaskSet;

/// The model this analysis reasons in. Versioned, because a changed model invalidates every
/// conclusion previously stated in it (§15).
pub const MODEL: &str = "idealized-zero-overhead/1 (ROADMAP.md §7.4)";

/// The assumptions every conclusion from this analysis carries. Taken verbatim from §7.4's list,
/// because a paraphrase is where an assumption quietly weakens.
pub const ASSUMPTIONS: &[&str] = &[
    "one processor",
    "independent preemptible tasks",
    "distinct fixed priorities",
    "no release jitter",
    "no blocking",
    "no overhead — no context-switch, interrupt, or dispatch cost is charged",
    "constrained deadlines (D ≤ T)",
    "declared worst-case computation times, whose own evidence is separate",
];

/// A defensive cap on iterations, distinct from the busy-period limit.
///
/// The iteration is non-decreasing and bounded above by `T`, so it terminates in at most `T`
/// steps — but `T` may be large, and a bug that made the sequence stall would otherwise hang
/// rather than report. §7.4 asks for explicit limits; this is the second one.
pub const MAX_ITERATIONS: usize = 10_000;

/// `⌈ numerator / denominator ⌉` in exact integer arithmetic.
///
/// ⛔ Written as `(n + d - 1) / d` in most textbooks, which overflows for large `n`. This form
/// cannot: it never constructs a value larger than `numerator`.
const fn ceil_div(numerator: u64, denominator: u64) -> Option<u64> {
    if denominator == 0 {
        return None;
    }
    let whole = numerator / denominator;
    if numerator.is_multiple_of(denominator) {
        Some(whole)
    } else {
        whole.checked_add(1)
    }
}

/// The recurrence sequence for one task — the witness §7.4 asks to be recorded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    /// Whose response this is.
    pub task: String,
    /// `R^(0), R^(1), …`, in order. The last two are equal exactly when it converged.
    pub sequence: Vec<u64>,
}

impl Witness {
    /// Render the sequence so a reader can check it by hand.
    #[must_use]
    pub fn render(&self) -> String {
        let steps: Vec<String> = self.sequence.iter().map(u64::to_string).collect();
        format!("{}: {}", self.task, steps.join(" → "))
    }
}

/// Which limit stopped an iteration that did not converge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Limit {
    /// The iterate passed the task's own minimum separation, so the level-`i` busy period does
    /// not close and the recurrence has no fixed point below it.
    BusyPeriod(u64),
    /// The defensive iteration cap. Reaching it is a bug or a pathological input, and either way
    /// it is not a schedulability answer.
    Iterations(usize),
}

/// What the recurrence established for one task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Bound {
    /// A fixed point was reached.
    Converged {
        /// The response-time bound in this model.
        response: u64,
        /// The iteration that produced it.
        witness: Witness,
    },
    /// No fixed point was reached within a named limit. Establishes nothing.
    NotConverged {
        /// Which limit stopped it.
        limit: Limit,
        /// How far it got.
        witness: Witness,
    },
    /// Exact arithmetic overflowed. Establishes nothing, and is deliberately not a wrapped number.
    Overflow {
        /// How far it got before the overflow.
        witness: Witness,
    },
}

/// The response-time bound for the task at `index` of `set`.
///
/// # Panics
///
/// If `index` is out of range for the set.
#[must_use]
pub fn response_time(set: &TaskSet, index: usize) -> Bound {
    let task = &set.tasks()[index];
    let interference_from = set.higher_priority_than(index);
    let mut sequence = vec![task.computation];
    let mut current = task.computation;

    for _ in 0..MAX_ITERATIONS {
        let mut next = task.computation;
        let mut overflowed = false;
        for other in interference_from {
            let Some(releases) = ceil_div(current, other.separation) else {
                overflowed = true;
                break;
            };
            let Some(demand) = releases.checked_mul(other.computation) else {
                overflowed = true;
                break;
            };
            let Some(sum) = next.checked_add(demand) else {
                overflowed = true;
                break;
            };
            next = sum;
        }
        if overflowed {
            return Bound::Overflow {
                witness: Witness {
                    task: task.id.clone(),
                    sequence,
                },
            };
        }
        sequence.push(next);
        if next == current {
            return Bound::Converged {
                response: next,
                witness: Witness {
                    task: task.id.clone(),
                    sequence,
                },
            };
        }
        if next > task.separation {
            return Bound::NotConverged {
                limit: Limit::BusyPeriod(task.separation),
                witness: Witness {
                    task: task.id.clone(),
                    sequence,
                },
            };
        }
        current = next;
    }

    Bound::NotConverged {
        limit: Limit::Iterations(MAX_ITERATIONS),
        witness: Witness {
            task: task.id.clone(),
            sequence,
        },
    }
}

/// What the analysis concluded about one task's deadline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// The bound converged within the deadline.
    MeetsDeadline {
        response: u64,
        deadline: u64,
        witness: Witness,
    },
    /// The bound converged past the deadline — a witness of violation in this model.
    MissesDeadline {
        response: u64,
        deadline: u64,
        witness: Witness,
    },
    /// Nothing was established, with the reason.
    Inconclusive { why: String, witness: Witness },
}

impl Outcome {
    /// The task this outcome is about.
    #[must_use]
    pub fn task(&self) -> &str {
        match self {
            Self::MeetsDeadline { witness, .. }
            | Self::MissesDeadline { witness, .. }
            | Self::Inconclusive { witness, .. } => &witness.task,
        }
    }

    /// The recurrence sequence behind it.
    #[must_use]
    pub const fn witness(&self) -> &Witness {
        match self {
            Self::MeetsDeadline { witness, .. }
            | Self::MissesDeadline { witness, .. }
            | Self::Inconclusive { witness, .. } => witness,
        }
    }
}

/// Analyze every task of an admitted set.
#[must_use]
pub fn analyze(set: &TaskSet) -> Vec<Outcome> {
    (0..set.tasks().len())
        .map(|index| {
            let task = &set.tasks()[index];
            match response_time(set, index) {
                Bound::Converged { response, witness } => {
                    if response <= task.deadline {
                        Outcome::MeetsDeadline {
                            response,
                            deadline: task.deadline,
                            witness,
                        }
                    } else {
                        Outcome::MissesDeadline {
                            response,
                            deadline: task.deadline,
                            witness,
                        }
                    }
                }
                Bound::NotConverged { limit, witness } => Outcome::Inconclusive {
                    why: match limit {
                        Limit::BusyPeriod(separation) => format!(
                            "the recurrence passed task `{}`'s minimum separation of {separation} \
                             without reaching a fixed point, so the level-i busy period does not \
                             close",
                            task.id
                        ),
                        Limit::Iterations(cap) => format!(
                            "the recurrence for task `{}` did not converge within {cap} \
                             iterations",
                            task.id
                        ),
                    },
                    witness,
                },
                Bound::Overflow { witness } => Outcome::Inconclusive {
                    why: format!(
                        "exact arithmetic overflowed while computing task `{}`'s response; §7.4 \
                         requires detection rather than a wrapped value, so no bound is reported",
                        task.id
                    ),
                    witness,
                },
            }
        })
        .collect()
}

/// Turn an analysis into a §7.1 conclusion.
///
/// ⭐ **This function is the reason the analysis cannot overstate itself.** A positive answer can
/// only be expressed as `HoldsUnderAssumptions`, which will not exist without a named model and a
/// non-empty assumption list — so the sentence "the deadlines are met", detached from the eight
/// conditions that make it true, is not constructible. §7.4's warning that this baseline "must
/// not be selected for a physical runtime whose nonzero overhead and jitter are omitted" cannot
/// be enforced by a type, but a conclusion that always carries "no overhead" in its own text is
/// the next best thing: a reader who quotes it quotes that too.
///
/// Precedence: a missed deadline outranks an inconclusive result, because a validated witness is
/// a stronger statement than a resource limit — but an inconclusive result **never** becomes a
/// positive conclusion.
#[must_use]
pub fn conclusion(outcomes: &[Outcome]) -> Conclusion {
    if let Some(Outcome::MissesDeadline {
        response,
        deadline,
        witness,
    }) = outcomes
        .iter()
        .find(|o| matches!(o, Outcome::MissesDeadline { .. }))
    {
        return Conclusion::Counterexample {
            witness: format!(
                "in {MODEL}, task `{}` has response-time bound {response} against a deadline of \
                 {deadline}; recurrence {}",
                witness.task,
                witness.render()
            ),
        };
    }
    if let Some(Outcome::Inconclusive { why, .. }) = outcomes
        .iter()
        .find(|o| matches!(o, Outcome::Inconclusive { .. }))
    {
        return Conclusion::Inconclusive { limit: why.clone() };
    }
    Conclusion::HoldsUnderAssumptions {
        model: MODEL.to_string(),
        assumptions: ASSUMPTIONS.iter().map(|a| (*a).to_string()).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::{analyze, ceil_div, conclusion, response_time, Bound, Limit, Outcome, MODEL};
    use crate::model::{Task, TaskSet};
    use archogen_evidence::claim::Conclusion;

    fn task(id: &str, priority: i64, c: u64, t: u64, d: u64) -> Task {
        Task {
            id: id.into(),
            priority,
            computation: c,
            separation: t,
            deadline: d,
        }
    }

    #[test]
    fn ceiling_division_rounds_up_and_does_not_overflow_near_the_top() {
        assert_eq!(ceil_div(0, 4), Some(0));
        assert_eq!(ceil_div(1, 4), Some(1));
        assert_eq!(ceil_div(4, 4), Some(1));
        assert_eq!(ceil_div(5, 4), Some(2));
        assert_eq!(ceil_div(7, 1), Some(7));
        assert_eq!(ceil_div(1, 0), None);
        // ⛔ The textbook `(n + d - 1) / d` overflows here; this form must not.
        assert_eq!(ceil_div(u64::MAX, 2), Some(u64::MAX / 2 + 1));
        assert_eq!(ceil_div(u64::MAX, 1), Some(u64::MAX));
    }

    #[test]
    fn the_highest_priority_task_converges_immediately_to_its_own_computation() {
        let set = TaskSet::admit(vec![task("a", 1, 1, 4, 4)]).expect("admitted");
        let Bound::Converged { response, witness } = response_time(&set, 0) else {
            panic!("a task with no interference converges at once");
        };
        assert_eq!(response, 1);
        assert_eq!(witness.sequence, vec![1, 1]);
    }

    #[test]
    fn a_non_converging_set_is_inconclusive_and_not_a_deadline_miss() {
        // ⭐ §7.4: "Conservative analysis failure is `not-established` unless an exact test or
        // validated counterexample establishes failure." Reporting a resource limit as a miss
        // invents a counterexample nobody found. Two tasks with utilisation above 1.
        let set =
            TaskSet::admit(vec![task("a", 1, 3, 4, 4), task("b", 2, 3, 5, 5)]).expect("admitted");
        let outcomes = analyze(&set);
        assert!(matches!(
            outcomes[0],
            Outcome::MeetsDeadline { response: 3, .. }
        ));
        let Outcome::Inconclusive { ref why, .. } = outcomes[1] else {
            panic!("b's busy period cannot close: {:?}", outcomes[1]);
        };
        assert!(why.contains("busy period"), "{why}");
        assert!(matches!(
            conclusion(&outcomes),
            Conclusion::Inconclusive { .. }
        ));
    }

    #[test]
    fn overflow_is_a_distinct_outcome_and_never_a_wrapped_number() {
        // Both arithmetic paths, because they fail in different places and a test that only
        // covers one leaves the other free to wrap. §7.4 asks for detection; a wrapped response
        // is a plausible small number, which is the worst possible failure here.
        //
        // The SUM path: two interferers of C = MAX/2 each contribute MAX/2, and the low-priority
        // task's own C = 2 pushes the total past MAX.
        let sum = TaskSet::admit(vec![
            task("h1", 1, u64::MAX / 2, u64::MAX, u64::MAX),
            task("h2", 2, u64::MAX / 2, u64::MAX, u64::MAX),
            task("l", 3, 2, u64::MAX, u64::MAX),
        ])
        .expect("admitted");
        let outcomes = analyze(&sum);
        let Outcome::Inconclusive { ref why, .. } = outcomes[2] else {
            panic!("expected an inconclusive outcome, got {:?}", outcomes[2]);
        };
        assert!(why.contains("overflow"), "{why}");

        // The PRODUCT path: four releases of an interferer whose C is half the range.
        let product = TaskSet::admit(vec![
            task("h", 1, u64::MAX / 2, 1, 1),
            task("l", 2, 4, u64::MAX, u64::MAX),
        ])
        .expect("admitted");
        let outcomes = analyze(&product);
        let Outcome::Inconclusive { ref why, .. } = outcomes[1] else {
            panic!("expected an inconclusive outcome, got {:?}", outcomes[1]);
        };
        assert!(why.contains("overflow"), "{why}");
    }

    #[test]
    fn the_iteration_cap_is_reported_as_itself() {
        assert_eq!(
            Limit::Iterations(super::MAX_ITERATIONS),
            Limit::Iterations(10_000)
        );
        assert_ne!(Limit::Iterations(10), Limit::BusyPeriod(10));
    }

    #[test]
    fn a_positive_conclusion_always_carries_the_model_and_its_assumptions() {
        // ⭐ The structural guard. There is no way to state "the deadlines are met" without the
        // eight conditions that make it true, so a reader who quotes the conclusion quotes them.
        let set = TaskSet::admit(vec![task("a", 1, 1, 4, 4)]).expect("admitted");
        let Conclusion::HoldsUnderAssumptions { model, assumptions } = conclusion(&analyze(&set))
        else {
            panic!("a schedulable set concludes conditionally");
        };
        assert_eq!(model, MODEL);
        assert_eq!(assumptions.len(), super::ASSUMPTIONS.len());
        assert!(
            assumptions.iter().any(|a| a.contains("no overhead")),
            "the assumption §7.4 warns about most must be in the text a reader sees: {assumptions:?}"
        );
    }

    #[test]
    fn a_missed_deadline_outranks_an_inconclusive_result_but_never_the_reverse() {
        let miss = Outcome::MissesDeadline {
            response: 4,
            deadline: 3,
            witness: super::Witness {
                task: "c".into(),
                sequence: vec![2, 4, 4],
            },
        };
        let unknown = Outcome::Inconclusive {
            why: "a limit".into(),
            witness: super::Witness {
                task: "d".into(),
                sequence: vec![1],
            },
        };
        assert!(matches!(
            conclusion(&[unknown.clone(), miss.clone()]),
            Conclusion::Counterexample { .. }
        ));
        assert!(matches!(
            conclusion(&[miss, unknown.clone()]),
            Conclusion::Counterexample { .. }
        ));
        // …and an inconclusive result alone never becomes positive.
        assert!(matches!(
            conclusion(&[unknown]),
            Conclusion::Inconclusive { .. }
        ));
    }

    #[test]
    fn the_witness_renders_so_a_reader_can_check_it_by_hand() {
        let witness = super::Witness {
            task: "c".into(),
            sequence: vec![2, 4, 4],
        };
        assert_eq!(witness.render(), "c: 2 → 4 → 4");
    }
}
