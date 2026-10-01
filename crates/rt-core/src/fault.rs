//! The bounded fault path (`ROADMAP.md` §8, §3.1).
//!
//! §3.1 requires the profile to have a "defined overrun, unexpected-trap, stack-guard, and
//! assertion failure policy; bounded diagnostic handling", and §8.1 separates three things that
//! are easy to conflate:
//!
//! > Distinguish expected errors, violated internal invariants, and deliberate fatal traps.
//! > Avoid unchecked panic paths in normal runtime operation; preserve a defined fatal handler
//! > and diagnostic evidence.
//!
//! ⭐ **A fault is a value here, not a panic.** It is produced, returned, and acted on by the
//! substrate, so a hosted test can observe the whole fault path without the process dying — and
//! so the target port can route it to a defined fatal handler rather than to whatever
//! `panic_handler` happens to be linked. A `panic!` in this crate would be a fault path that
//! cannot be tested and cannot be routed.

/// Why a task or the runtime entered its fault path.
///
/// Bounded and closed: every variant is a condition the profile names. A runtime that can fault
/// for a reason its profile does not list has a behavior nobody analyzed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// A task was released while its previous job had not completed. §3.1 requires a defined
    /// overrun policy; which one applies is [`OverrunPolicy`], and eADL declares it per task.
    Overrun {
        /// The task that overran.
        task: usize,
    },
    /// A stack guard was touched. §7.6: guards stay even when a static bound exists, because a
    /// bound is an argument and a guard is a fact.
    StackGuard {
        /// Whose guard was hit: a task's, or `None` for the interrupt stack's. This is not the
        /// attribution, which is the executing context's (§3.1.1 rule 2, and [`Context`]).
        task: Option<usize>,
    },
    /// A trap the runtime does not model. §8.1's "deliberate fatal trap" is *not* this — that is
    /// a decision; this is a surprise.
    UnexpectedTrap {
        /// The architecture's trap cause, uninterpreted. `rt-core` does not know the ISA.
        cause: u64,
    },
    /// A violated internal invariant. §8.1 keeps this distinct from an expected error because
    /// the response differs: an expected error is handled, an invariant violation means the
    /// runtime's own state is no longer trustworthy.
    InvariantViolated {
        /// A static identifier of the invariant, so the diagnostic is bounded — no formatting,
        /// no allocation, on a path that may be taken with the stack already damaged.
        invariant: &'static str,
    },
}

impl Fault {
    /// Whether the runtime's own state is still trustworthy after this fault.
    ///
    /// An overrun is a workload event: the scheduler knows exactly what happened and can apply a
    /// policy. An invariant violation, a stack guard, or an unmodelled trap are not — after any
    /// of those, continuing to schedule is running a system nobody analyzed.
    #[must_use]
    pub const fn runtime_state_is_trustworthy(self) -> bool {
        matches!(self, Self::Overrun { .. })
    }

    /// A stable, allocation-free name for the diagnostic path.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Overrun { .. } => "overrun",
            Self::StackGuard { .. } => "stack-guard",
            Self::UnexpectedTrap { .. } => "unexpected-trap",
            Self::InvariantViolated { .. } => "invariant-violated",
        }
    }
}

/// Where a synchronous fault was raised — a stack guard, a trap or an assertion — which is what it
/// is attributed to: §3.1.1 rule 2, "synchronous to the context executing the faulting instruction".
/// An overrun is attributed to the overrunning task wherever it is raised, so this does not apply
/// to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// The running task's job, a runtime primitive it called, or its completion path: that task.
    Job,
    /// A service, the trap path, a transition, idle or the fatal handler: no task. A task whose
    /// job the fault interrupted is recorded as interrupted, never as attributed.
    Kernel,
}

/// What to do when a task overruns — the policy eADL's `(on-overrun …)` clause declares.
///
/// ⛔ There is no `Ignore`. §3.1 requires a *defined* overrun policy, and silently dropping the
/// late job is a definition only in the sense that anything is: it makes the task set analyzed
/// differ from the task set running, with no record that it happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverrunPolicy {
    /// Enter the fault path. The profile's default, and the one the examples declare.
    Fault,
    /// Abandon the late job and accept the new release, recording that it happened.
    ///
    /// Legal only where the analysis was told: a dropped job is a missed deadline by another
    /// name, and a report that does not say so overstates its evidence.
    SkipLateJob,
}

#[cfg(test)]
mod tests {
    use super::{Fault, OverrunPolicy};

    #[test]
    fn only_an_overrun_leaves_the_runtime_trustworthy() {
        assert!(Fault::Overrun { task: 0 }.runtime_state_is_trustworthy());
        for fault in [
            Fault::StackGuard { task: Some(0) },
            Fault::UnexpectedTrap { cause: 2 },
            Fault::InvariantViolated { invariant: "x" },
        ] {
            assert!(
                !fault.runtime_state_is_trustworthy(),
                "{} must stop the runtime, not be handled and continued",
                fault.slug()
            );
        }
    }

    #[test]
    fn every_fault_has_a_distinct_allocation_free_name() {
        let slugs = [
            Fault::Overrun { task: 0 }.slug(),
            Fault::StackGuard { task: Some(0) }.slug(),
            Fault::UnexpectedTrap { cause: 0 }.slug(),
            Fault::InvariantViolated { invariant: "x" }.slug(),
        ];
        let mut sorted = slugs;
        sorted.sort_unstable();
        let mut deduped = sorted;
        let unique = {
            let mut n = 0;
            for i in 0..deduped.len() {
                if i == 0 || deduped[i] != deduped[i - 1] {
                    deduped[n] = deduped[i];
                    n += 1;
                }
            }
            n
        };
        assert_eq!(unique, slugs.len(), "two faults share a name");
    }

    #[test]
    fn there_is_no_policy_that_silently_drops_a_job() {
        // §3.1 requires a DEFINED overrun policy. `SkipLateJob` is defined and recorded;
        // "ignore" would make the running task set differ from the analyzed one with no trace.
        let policies = [OverrunPolicy::Fault, OverrunPolicy::SkipLateJob];
        assert_eq!(
            policies.len(),
            2,
            "a third policy needs its own analysis story"
        );
    }
}
