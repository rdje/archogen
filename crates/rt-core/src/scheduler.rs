//! The fixed-priority task lifecycle, as a state machine that decides and does not act.
//!
//! `ROADMAP.md` §8 lists the initial mechanisms: "static task creation at boot, a fixed-priority
//! ready structure, release/timer management, interrupt dispatch, context switching, static
//! memory layout, and a bounded fault path". This module owns the first three and the last, and
//! the **boundary** of the fourth — it reports that a switch is due; the architecture port
//! performs it.
//!
//! # The five states, and why there are five
//!
//! ```text
//!            release                 dispatch
//!  Suspended ────────▶ Ready ◀─────┬──────────▶ Running
//!      ▲                           │  preempt      │
//!      │                           └───────────────┘
//!      └──────────────── complete ─────────────────┘
//!
//!  any ──── fault ────▶ Faulted   (terminal for that task)
//! ```
//!
//! `Suspended` and `Ready` are separate because a release is an event with a time; collapsing
//! them loses the instant a deadline is measured from. `Ready` and `Running` are separate because
//! on one core exactly one task runs, and the *transition* between them is what costs a context
//! switch — the cost `cost-accounting/1` charges. `Faulted` is terminal for the task: §8.1 keeps
//! "violated internal invariants" apart from expected errors precisely because resuming is not
//! among the options.
//!
//! # Priorities
//!
//! A task's index **is** its priority rank: index 0 is the highest. That is not a shortcut — it
//! is the profile's "static unique task priorities" made structural, the same way
//! `rt_analysis::model::TaskSet` sorts. Ascending is highest-first
//! (`docs/decisions/decision_priority-comparison-direction.md`), so "the highest-priority ready
//! task" is the first set bit, and the ready structure is a bitmask over a fixed capacity.

use crate::fault::{Fault, OverrunPolicy};

/// Where a task is in its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Created at boot, waiting for its next release.
    Suspended,
    /// Released and runnable, not currently running.
    Ready,
    /// Running. On one core, at most one task is in this state.
    Running,
    /// Faulted. Terminal: §8.1 does not offer resumption as an option.
    Faulted,
}

/// A state transition the scheduler made, for a hosted harness or a trace to observe.
///
/// §8: "A hosted harness can inject preemption at modeled boundaries." Those boundaries are these
/// transitions, so they are reported rather than kept internal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    /// A task became ready.
    Released { task: usize },
    /// A release arrived while interrupts were masked and was latched for delivery on unmask.
    Latched { task: usize },
    /// A task finished its job and is waiting for the next release.
    Completed { task: usize },
    /// A task entered the fault path.
    Faulted { task: usize, fault: Fault },
}

/// What the substrate should do next.
///
/// ⭐ **This crate decides; it does not act.** Performing a switch — saving registers, changing
/// stacks, returning from an interrupt — is the architecture port's job, reviewed separately and
/// untestable without a target. Keeping the decision here is what makes the policy testable on a
/// host, and what stops a scheduling bug and a context-save bug from being the same bug.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Nothing is ready; idle until the next event.
    Idle,
    /// Keep running the task that is already running. No switch, and so no cost.
    Continue { task: usize },
    /// Enter `to` from idle. §13.4 charges this as an initial dispatch, not a switch: there is
    /// no outgoing context to save.
    Dispatch { to: usize },
    /// Switch from `from` to `to`. The only decision that costs a save **and** a restore.
    Switch { from: usize, to: usize },
    /// The runtime's own state is no longer trustworthy; run the fatal path.
    Halt { fault: Fault },
}

/// The fixed-priority scheduler over a static set of `N` tasks.
///
/// `N` is a const parameter because the profile admits a "finite static task set" created at boot
/// and excludes `dynamic-task-creation`: there is no allocation, no growth, and no object
/// manager. A task is its index, and the index is its priority rank.
#[derive(Debug, Clone)]
pub struct Scheduler<const N: usize> {
    states: [TaskState; N],
    policies: [OverrunPolicy; N],
    /// Releases that arrived while masked, delivered on unmask.
    latched: [bool; N],
    running: Option<usize>,
    /// Interrupt masking depth. §8.1 asks for masking to be modelled explicitly; nesting is
    /// counted so a critical section inside another does not unmask early.
    mask_depth: u32,
    /// Set once the runtime's own state is no longer trustworthy.
    halted: Option<Fault>,
}

impl<const N: usize> Scheduler<N> {
    /// Create the task set at boot. Every task starts [`TaskState::Suspended`].
    #[must_use]
    pub const fn new(policies: [OverrunPolicy; N]) -> Self {
        Self {
            states: [TaskState::Suspended; N],
            policies,
            latched: [false; N],
            running: None,
            mask_depth: 0,
            halted: None,
        }
    }

    /// A task's current state.
    ///
    /// # Panics
    ///
    /// If `task` is not in the static set — which is a caller bug, not a runtime condition:
    /// indices come from the generated plan, and a plan naming a task that does not exist never
    /// reached the target.
    #[must_use]
    pub fn state(&self, task: usize) -> TaskState {
        self.states[task]
    }

    /// Whether interrupts are masked.
    #[must_use]
    pub const fn is_masked(&self) -> bool {
        self.mask_depth > 0
    }

    /// Enter a critical section. Nests.
    pub fn mask(&mut self) {
        self.mask_depth = self.mask_depth.saturating_add(1);
    }

    /// Leave a critical section, delivering any latched releases when the outermost one closes.
    ///
    /// Returns the transitions the delivery produced, in task order — highest priority first,
    /// because that is the order a re-enabled interrupt controller would present them and the
    /// order the ready structure will read them back in.
    ///
    /// # Panics
    ///
    /// If unmasking without a matching [`Scheduler::mask`]. An unbalanced critical section is a
    /// violated invariant, and §8.1 keeps that distinct from an expected error — but this one is
    /// a programming error in the substrate, caught at the point it happens rather than turned
    /// into a runtime fault that hides it.
    pub fn unmask(&mut self) -> heapless::Transitions<N> {
        assert!(self.mask_depth > 0, "unmask without a matching mask");
        self.mask_depth -= 1;
        let mut delivered = heapless::Transitions::new();
        if self.mask_depth > 0 {
            return delivered;
        }
        for task in 0..N {
            if self.latched[task] {
                self.latched[task] = false;
                if let Some(transition) = self.deliver(task) {
                    delivered.push(transition);
                }
            }
        }
        delivered
    }

    /// A release arrived for `task`.
    ///
    /// While masked it is **latched** rather than lost. A release arriving while an earlier one
    /// is still latched, or while the task has not completed, is an **overrun**: there is nowhere
    /// to put a second pending job, and inventing somewhere would be a queue in a profile that
    /// excludes queues.
    pub fn release(&mut self, task: usize) -> Transition {
        if self.is_masked() {
            if self.latched[task] {
                return self.raise(Fault::Overrun { task });
            }
            self.latched[task] = true;
            return Transition::Latched { task };
        }
        self.deliver(task)
            .unwrap_or_else(|| self.raise(Fault::Overrun { task }))
    }

    /// Deliver a release now. `None` means it was an overrun.
    fn deliver(&mut self, task: usize) -> Option<Transition> {
        match self.states[task] {
            TaskState::Suspended => {
                self.states[task] = TaskState::Ready;
                Some(Transition::Released { task })
            }
            // Ready or Running: the previous job has not completed.
            TaskState::Ready | TaskState::Running => None,
            TaskState::Faulted => None,
        }
    }

    /// The running task finished its job.
    ///
    /// # Panics
    ///
    /// If `task` is not the running one. The substrate reports completion of what it dispatched;
    /// anything else means the two disagree about what is on the processor, which is an invariant
    /// violation worth catching where it happens.
    pub fn complete(&mut self, task: usize) -> Transition {
        assert_eq!(
            self.running,
            Some(task),
            "completion reported for a task that is not running"
        );
        self.states[task] = TaskState::Suspended;
        self.running = None;
        Transition::Completed { task }
    }

    /// Raise a fault from outside — a stack guard, an unexpected trap, an assertion.
    pub fn fault(&mut self, fault: Fault) -> Transition {
        self.raise(fault)
    }

    fn raise(&mut self, fault: Fault) -> Transition {
        let task = match fault {
            Fault::Overrun { task } | Fault::StackGuard { task } => task,
            // A trap or an invariant violation is not attributable to one task's state; it stops
            // the runtime, and `decide` reports `Halt`.
            _ => {
                self.halted = Some(fault);
                return Transition::Faulted {
                    task: usize::MAX,
                    fault,
                };
            }
        };
        if matches!(fault, Fault::Overrun { .. })
            && self.policies[task] == OverrunPolicy::SkipLateJob
        {
            // The late job is abandoned and the new release accepted — recorded, never silent.
            self.states[task] = TaskState::Ready;
            if self.running == Some(task) {
                self.running = None;
            }
            return Transition::Released { task };
        }
        self.states[task] = TaskState::Faulted;
        if self.running == Some(task) {
            self.running = None;
        }
        if !fault.runtime_state_is_trustworthy() {
            self.halted = Some(fault);
        }
        Transition::Faulted { task, fault }
    }

    /// What the substrate should do next.
    ///
    /// Named `decide` rather than `next` on purpose: it is not an iterator, and a method called
    /// `next` on a non-iterator reads like one at every call site. It also says what this crate
    /// does — it decides, and the substrate acts.
    ///
    /// Idempotent: calling it twice without an intervening event returns the same decision, so a
    /// substrate may consult it at any modelled boundary.
    pub fn decide(&mut self) -> Decision {
        if let Some(fault) = self.halted {
            return Decision::Halt { fault };
        }
        // The highest-priority ready task is the lowest index, because the index IS the rank.
        let mut best: Option<usize> = None;
        for task in 0..N {
            if self.states[task] == TaskState::Ready {
                best = Some(task);
                break;
            }
        }
        match (self.running, best) {
            (Some(current), None) => Decision::Continue { task: current },
            (Some(current), Some(candidate)) if candidate >= current => {
                Decision::Continue { task: current }
            }
            (Some(current), Some(candidate)) => {
                self.states[current] = TaskState::Ready;
                self.states[candidate] = TaskState::Running;
                self.running = Some(candidate);
                Decision::Switch {
                    from: current,
                    to: candidate,
                }
            }
            (None, Some(candidate)) => {
                self.states[candidate] = TaskState::Running;
                self.running = Some(candidate);
                Decision::Dispatch { to: candidate }
            }
            (None, None) => Decision::Idle,
        }
    }
}

/// A fixed-capacity transition list, so unmasking allocates nothing.
pub mod heapless {
    use super::Transition;

    /// At most one delivered transition per task, which is the capacity a latch set can hold.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Transitions<const N: usize> {
        items: [Option<Transition>; N],
        len: usize,
    }

    impl<const N: usize> Transitions<N> {
        /// An empty list.
        #[must_use]
        pub const fn new() -> Self {
            Self {
                items: [None; N],
                len: 0,
            }
        }

        /// Append. Cannot overflow: at most one latched release exists per task.
        pub fn push(&mut self, transition: Transition) {
            self.items[self.len] = Some(transition);
            self.len += 1;
        }

        /// How many were delivered.
        #[must_use]
        pub const fn len(&self) -> usize {
            self.len
        }

        /// Whether none were.
        #[must_use]
        pub const fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// The delivered transitions, in order.
        pub fn as_slice(&self) -> impl Iterator<Item = Transition> + '_ {
            self.items[..self.len].iter().filter_map(|item| *item)
        }
    }

    impl<const N: usize> Default for Transitions<N> {
        fn default() -> Self {
            Self::new()
        }
    }
}
