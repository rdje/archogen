//! The admitted task model of `ROADMAP.md` §7.4, and the conditions that make it apply.
//!
//! §7.4 defines a "deliberately restricted mathematical baseline" and lists its conditions:
//!
//! > one processor, independent preemptible tasks, distinct fixed priorities, no release jitter,
//! > no blocking, no overhead, constrained deadlines, and declared worst-case computation times.
//!
//! ⭐ **The conditions are a constructor precondition, not a comment.** [`TaskSet::admit`] is the
//! only way to build one, and it refuses a set the model does not cover. That ordering matters:
//! an analysis that accepts anything and documents its assumptions in prose will eventually be
//! run on a task set that violates them, and it will return a number rather than a refusal. A
//! number is what gets quoted.
//!
//! ⚠️ What this **cannot** enforce is the misuse §7.4 warns about most loudly:
//!
//! > It must not be selected for a physical runtime whose nonzero overhead and jitter are
//! > omitted.
//!
//! No type can tell whether the caller is describing an idealized model or a real board. That
//! defence lives in the *conclusion*: [`crate::response`] returns a
//! `Conclusion::HoldsUnderAssumptions` carrying the model and its assumptions, and the evidence
//! vocabulary makes a conclusion without its qualifier unconstructible.

use core::fmt;

/// One task, in abstract integer time units (§13.2: "abstract time units … synthetic fixtures").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// A stable logical identity, used in witnesses and diagnostics.
    pub id: String,
    /// The priority rank. Ascending is highest-first
    /// (`docs/decisions/decision_priority-comparison-direction.md`).
    pub priority: i64,
    /// Worst-case computation time `C`. Declared, never measured here.
    pub computation: u64,
    /// Minimum release separation `T` — the period of a periodic task.
    pub separation: u64,
    /// Relative deadline `D`, measured from release.
    pub deadline: u64,
}

/// Why a task set is outside the admitted model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Inadmissible {
    /// §7.4 admits no empty workload; there is nothing to analyze.
    Empty,
    /// §7.4: "distinct fixed priorities". Two tasks at one priority need a tie-break policy the
    /// recurrence does not model.
    DuplicatePriority {
        priority: i64,
        first: String,
        second: String,
    },
    /// §7.4: "constrained deadlines" — `D ≤ T`. With `D > T` more than one job of a task can be
    /// alive at once and the recurrence no longer bounds the response.
    DeadlineExceedsSeparation {
        task: String,
        deadline: u64,
        separation: u64,
    },
    /// A zero separation makes every interference term unbounded.
    ZeroSeparation { task: String },
    /// A zero computation time is not a declared worst case; it is an omission.
    ZeroComputation { task: String },
}

impl fmt::Display for Inadmissible {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "the task set is empty, so there is nothing to analyze"),
            Self::DuplicatePriority {
                priority,
                first,
                second,
            } => write!(
                f,
                "tasks `{first}` and `{second}` share priority {priority}; §7.4's model admits \
                 distinct fixed priorities only, because equal priorities need a tie-break policy \
                 the recurrence does not describe"
            ),
            Self::DeadlineExceedsSeparation {
                task,
                deadline,
                separation,
            } => write!(
                f,
                "task `{task}` has deadline {deadline} and separation {separation}; §7.4's model \
                 admits constrained deadlines (D ≤ T) only, because with D > T more than one job \
                 can be alive at once and the recurrence no longer bounds the response"
            ),
            Self::ZeroSeparation { task } => write!(
                f,
                "task `{task}` has a zero minimum separation, which makes its interference \
                 unbounded"
            ),
            Self::ZeroComputation { task } => write!(
                f,
                "task `{task}` declares zero computation time; that is an omitted bound rather \
                 than a declared worst case, and §7.3 requires every numerical bound to record \
                 its origin"
            ),
        }
    }
}

/// A task set the §7.4 model applies to.
///
/// Constructible only through [`TaskSet::admit`], and held sorted highest-priority-first so
/// `hp(i)` is a prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskSet {
    tasks: Vec<Task>,
}

impl TaskSet {
    /// Admit a task set, or say why the model does not cover it.
    ///
    /// # Errors
    ///
    /// The first condition of §7.4 that fails. One reason, not a list: a set outside the model is
    /// outside it, and enumerating every way would suggest the others are worth fixing
    /// independently.
    pub fn admit(mut tasks: Vec<Task>) -> Result<Self, Inadmissible> {
        if tasks.is_empty() {
            return Err(Inadmissible::Empty);
        }
        tasks.sort_by_key(|task| task.priority);
        for window in tasks.windows(2) {
            let (first, second) = (&window[0], &window[1]);
            if first.priority == second.priority {
                return Err(Inadmissible::DuplicatePriority {
                    priority: first.priority,
                    first: first.id.clone(),
                    second: second.id.clone(),
                });
            }
        }
        for task in &tasks {
            if task.separation == 0 {
                return Err(Inadmissible::ZeroSeparation {
                    task: task.id.clone(),
                });
            }
            if task.computation == 0 {
                return Err(Inadmissible::ZeroComputation {
                    task: task.id.clone(),
                });
            }
            if task.deadline > task.separation {
                return Err(Inadmissible::DeadlineExceedsSeparation {
                    task: task.id.clone(),
                    deadline: task.deadline,
                    separation: task.separation,
                });
            }
        }
        Ok(Self { tasks })
    }

    /// The tasks, highest priority first.
    #[must_use]
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// The tasks of higher priority than the one at `index` — `hp(i)` of §7.4.
    ///
    /// A prefix, because the set is held sorted and priorities are distinct.
    #[must_use]
    pub fn higher_priority_than(&self, index: usize) -> &[Task] {
        &self.tasks[..index.min(self.tasks.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::{Inadmissible, Task, TaskSet};

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
    fn admission_sorts_highest_priority_first_so_hp_is_a_prefix() {
        let set = TaskSet::admit(vec![
            task("c", 3, 2, 10, 10),
            task("a", 1, 1, 4, 4),
            task("b", 2, 1, 5, 5),
        ])
        .expect("admitted");
        let ids: Vec<&str> = set.tasks().iter().map(|t| t.id.as_str()).collect();
        assert_eq!(ids, vec!["a", "b", "c"]);
        assert_eq!(set.higher_priority_than(0).len(), 0);
        assert_eq!(set.higher_priority_than(2).len(), 2);
    }

    #[test]
    fn an_empty_set_is_not_a_vacuously_schedulable_one() {
        assert_eq!(TaskSet::admit(vec![]), Err(Inadmissible::Empty));
    }

    #[test]
    fn equal_priorities_are_outside_the_model() {
        let error = TaskSet::admit(vec![task("a", 1, 1, 4, 4), task("b", 1, 1, 5, 5)])
            .expect_err("§7.4 admits distinct priorities only");
        assert!(matches!(error, Inadmissible::DuplicatePriority { .. }));
        assert!(error.to_string().contains("tie-break"), "{error}");
    }

    #[test]
    fn an_arbitrary_deadline_is_outside_the_model() {
        let error = TaskSet::admit(vec![task("a", 1, 1, 4, 5)])
            .expect_err("§7.4 admits constrained deadlines only");
        assert!(matches!(
            error,
            Inadmissible::DeadlineExceedsSeparation { .. }
        ));
        assert!(error.to_string().contains("more than one job"), "{error}");
    }

    #[test]
    fn a_deadline_equal_to_the_separation_is_admitted() {
        // D ≤ T, not D < T: implicit-deadline sets are the commonest admitted case, and an
        // off-by-one here would reject §13.2's own baseline.
        assert!(TaskSet::admit(vec![task("a", 1, 1, 4, 4)]).is_ok());
    }

    #[test]
    fn a_zero_bound_is_an_omission_rather_than_a_worst_case() {
        let error = TaskSet::admit(vec![task("a", 1, 0, 4, 4)]).expect_err("zero C");
        assert!(matches!(error, Inadmissible::ZeroComputation { .. }));
        let error = TaskSet::admit(vec![task("a", 1, 1, 0, 0)]).expect_err("zero T");
        assert!(matches!(error, Inadmissible::ZeroSeparation { .. }));
    }
}
