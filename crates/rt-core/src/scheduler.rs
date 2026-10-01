//! The fixed-priority task lifecycle, as a state machine that decides and does not act.
//!
//! `ROADMAP.md` §8 lists the initial mechanisms: "static task creation at boot, a fixed-priority
//! ready structure, release/timer management, interrupt dispatch, context switching, static
//! memory layout, and a bounded fault path". This module owns the first three and the last, and
//! the **boundary** of the fourth — it reports that a switch is due; the architecture port
//! performs it.
//!
//! # The four states, and why there are four
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

use crate::fault::{Context, Fault, OverrunPolicy};

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
    /// A release arrived while the task's latch already held one. §3.1.1 rule 1: the overrun is
    /// kept beside the held release and judged at delivery, outside every masked region, where the
    /// task's policy applies. Never lost, and never fatal for landing inside a critical section.
    OverrunLatched { task: usize },
    /// A task finished its job and is waiting for the next release.
    Completed { task: usize },
    /// A task overran and its `SkipLateJob` policy abandoned the late job. When a release detected
    /// the overrun, that release is the task's next job and the task is ready (§3.1.1 rule 1);
    /// when the overrun was raised through [`Scheduler::fault`], the task waits for its next
    /// release. Its own variant, because a skipped job is a missed deadline by another name and
    /// must be told apart from an ordinary release in any trace.
    JobSkipped { task: usize },
    /// A task overran under its `Fault` policy and left the schedule for good; every other task
    /// continues (§3.1.1 rule 5). A fault that halts the runtime is [`Transition::Halted`].
    Faulted { task: usize, fault: Fault },
    /// The runtime entered its fatal handler (§3.1.1 rule 7). Every event after the halt answers
    /// with this same record and changes nothing: no job runs, no release is processed or latched.
    Halted { fatal: Fatal },
}

/// What the fatal handler keeps (§3.1.1 rule 7): the **first** fault that was not containable,
/// or was made so by rule 3. A later fault, one in the handler included, never replaces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fatal {
    /// The fault, which carries its §3.1 kind; [`Fault::runtime_state_is_trustworthy`] separates
    /// §8.1's expected error from the rest.
    pub fault: Fault,
    /// The task it is attributed to (§3.1.1 rule 2), or `None` when no task's code raised it.
    pub task: Option<usize>,
    /// The task whose job a fault raised in kernel context interrupted — recorded as interrupted,
    /// never as attributed.
    pub interrupted: Option<usize>,
    /// Whether rule 3 escalated a containable fault because a masked region was open.
    pub escalated: bool,
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
    /// The runtime's own state is no longer trustworthy; run the fatal path. Stays the decision.
    Halt { fatal: Fatal },
}

/// Why an operation was refused.
///
/// ⛔ One reason only. A `mask` past the bound and an `unmask` with nothing to close were refusals
/// until `2026-10-01`, and §3.1.1 now makes both assertion failures: a refused `mask` leaves its
/// caller's matching `unmask` to close the section early, which is the failure the bound exists
/// to prevent. So both halt, and the refusal a caller sees is the one every operation after a
/// halt sees.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The runtime has entered its fatal path; §8.1 offers no way back. [`Decision::Halt`] says
    /// why.
    Halted,
}

/// Why a task set was not admissible at boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    /// §3.1.1: "A task set must be non-empty." A system with no workload makes §7.2's second
    /// timing obligation vacuous, and a vacuously passing schedulability result is what §7.1
    /// exists to prevent.
    NoTasks,
    /// An eADL priority rank below 1. `decision_priority-comparison-direction.md`: a rank is an
    /// integer `N >= 1`, and admitting 0 would move the top of the range by inference.
    RankBelowOne {
        /// Its position in the supplied list.
        position: usize,
    },
    /// §3.1: "static **unique** task priorities".
    DuplicateRank {
        /// The rank two tasks share.
        rank: u16,
    },
}

/// What a task's latch holds while interrupts are masked: one slot per task, since the profile
/// excludes queues, and a mark for the overrun a second arrival makes (§3.1.1 rule 1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Latch {
    Empty,
    Held,
    /// A release is held, and at least one more arrived: the overrun is judged at delivery.
    Overrun,
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
    latched: [Latch; N],
    running: Option<usize>,
    /// Interrupt masking depth. §8.1 asks for masking to be modelled explicitly; nesting is
    /// counted so a critical section inside another does not unmask early.
    mask_depth: u32,
    /// Set once, by the first fault that is not containable (§3.1.1 rule 7).
    halted: Option<Fatal>,
}

impl<const N: usize> Scheduler<N> {
    /// The declared bound on critical-section nesting (§3.1 "bounded kernel critical sections",
    /// fixed by §3.1.1).
    ///
    /// The *value* is a judgement — 255 is far beyond any real kernel's nesting, and the runtime's
    /// catalog record declares it — but what happens at the bound is not: §3.1.1 makes exceeding it
    /// an assertion failure, because wrapping, saturating and refusing each fail silently.
    pub const MASK_DEPTH_LIMIT: u8 = u8::MAX;

    /// Build a scheduler from eADL priority ranks, in description order.
    ///
    /// ⛔ **This is the only supported way to lower a description onto a scheduler**, because the
    /// mapping it performs is a load-bearing off-by-one that was written down nowhere until
    /// `decision_priority-comparison-direction.md` was amended: a task's array index here *is* its
    /// priority and indices start at **0**, while the language's highest rank is **1**. So
    ///
    /// ```text
    /// runtime index = |hp(i)| = the number of tasks whose rank is smaller
    /// ```
    ///
    /// which is `eADL rank - 1` exactly when the ranks run contiguously from 1, as every example
    /// does. The language does not require that, and fixed priority uses only the order — the
    /// record's `hp(i) = { j : N_j < N_i }` — so ranks `1, 5, 9` lower to indices `0, 1, 2`.
    ///
    /// Performing it in one named, tested place is the difference between a relation and an
    /// assumption. `ranks[i]` is the rank of the task described at position `i`; the returned
    /// scheduler holds them in rank order.
    ///
    /// # Errors
    ///
    /// [`BootError`] when the task set is empty, a rank is below 1, or two ranks collide.
    pub fn from_eadl_ranks(
        ranks: [u16; N],
        policies: [OverrunPolicy; N],
    ) -> Result<Self, BootError> {
        if N == 0 {
            return Err(BootError::NoTasks);
        }
        for (position, rank) in ranks.iter().enumerate() {
            if *rank < 1 {
                return Err(BootError::RankBelowOne { position });
            }
        }
        // §3.1: "static **unique** task priorities". Distinct ranks also make every index below
        // distinct, so each task lands on its own slot. Quadratic in N, once, at boot.
        for (position, rank) in ranks.iter().enumerate() {
            if ranks[..position].contains(rank) {
                return Err(BootError::DuplicateRank { rank: *rank });
            }
        }

        let mut by_rank = [OverrunPolicy::Fault; N];
        for (position, rank) in ranks.iter().enumerate() {
            let index = ranks.iter().filter(|other| *other < rank).count();
            by_rank[index] = policies[position];
        }
        Ok(Self::new(by_rank))
    }

    /// Create the task set at boot, indexed by priority rank. Every task starts
    /// [`TaskState::Suspended`].
    ///
    /// Prefer [`Scheduler::from_eadl_ranks`] when the task set comes from a description: this
    /// constructor takes the runtime's 0-based indexing and cannot check a rank it never sees.
    #[must_use]
    pub const fn new(policies: [OverrunPolicy; N]) -> Self {
        Self {
            states: [TaskState::Suspended; N],
            policies,
            latched: [Latch::Empty; N],
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
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`] once the runtime has halted — including by this call: a `mask` past
    /// [`Scheduler::MASK_DEPTH_LIMIT`] is an assertion failure (§3.1.1). ⛔ It once saturated, and
    /// then was refused; a saturated counter stops counting, and a refused one leaves the caller's
    /// matching `unmask` to close the section early. Both return interrupts one section early.
    pub fn mask(&mut self) -> Result<u8, Refused> {
        if self.halted.is_some() {
            return Err(Refused::Halted);
        }
        if self.mask_depth >= u32::from(Self::MASK_DEPTH_LIMIT) {
            self.assertion("mask-depth-bound");
            return Err(Refused::Halted);
        }
        self.mask_depth += 1;
        Ok(u8::try_from(self.mask_depth).unwrap_or(u8::MAX))
    }

    /// Leave a critical section, delivering any latched releases when the outermost one closes.
    ///
    /// Returns the transitions the delivery produced, in task order — highest priority first,
    /// because that is the order a re-enabled interrupt controller would present them and the
    /// order the ready structure will read them back in.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`] once the runtime has halted — including by this call: an `unmask` with
    /// no matching [`Scheduler::mask`] is an assertion failure (§3.1.1), taken through the fatal
    /// path rather than a panic, which reaches neither of §8.1's "defined fatal handler and
    /// diagnostic evidence".
    pub fn unmask(&mut self) -> Result<heapless::Transitions<N>, Refused> {
        if self.halted.is_some() {
            return Err(Refused::Halted);
        }
        if self.mask_depth == 0 {
            self.assertion("unmask-at-depth-zero");
            return Err(Refused::Halted);
        }
        self.mask_depth -= 1;
        if self.mask_depth > 0 {
            return Ok(heapless::Transitions::new());
        }
        Ok(self.deliver_latched())
    }

    /// Deliver every latched release, in task order, now that no section is open. A latched
    /// release for a task that still owes a job is an overrun the moment it is delivered, and an
    /// overrun latched beside a release is judged right after it: §3.1.1 rule 1, detection applies
    /// the policy, here outside every masked region, so rule 3 does not escalate it.
    fn deliver_latched(&mut self) -> heapless::Transitions<N> {
        let mut delivered = heapless::Transitions::new();
        for task in 0..N {
            let latch = core::mem::replace(&mut self.latched[task], Latch::Empty);
            if latch == Latch::Empty {
                continue;
            }
            let first = match self.deliver(task) {
                Some(transition) => transition,
                None => self.overrun_by_release(task),
            };
            delivered.push(first);
            if latch == Latch::Overrun {
                let second = self.overrun_by_release(task);
                delivered.push(second);
            }
        }
        delivered
    }

    /// A release arrived for `task`.
    ///
    /// While masked it is **latched** rather than lost. A release arriving while one is already
    /// latched is kept as an overrun beside it, judged at delivery (§3.1.1 rule 1): there is no
    /// queue to hold a second job, and the outcome must not depend on which side of an unmask the
    /// interrupt landed. Unmasked, a release while the task has not completed is an **overrun**
    /// at once.
    pub fn release(&mut self, task: usize) -> Transition {
        if let Some(fatal) = self.halted {
            return Transition::Halted { fatal };
        }
        if self.is_masked() {
            return match self.latched[task] {
                Latch::Empty => {
                    self.latched[task] = Latch::Held;
                    Transition::Latched { task }
                }
                Latch::Held | Latch::Overrun => {
                    self.latched[task] = Latch::Overrun;
                    Transition::OverrunLatched { task }
                }
            };
        }
        self.deliver(task)
            .unwrap_or_else(|| self.overrun_by_release(task))
    }

    /// An overrun a release detected: §3.1.1 rule 1, with the release the policy's. Under
    /// `SkipLateJob` that release becomes the task's next job, so the task is ready again; under
    /// `Fault` it goes with the faulted task. An overrun raised through [`Scheduler::fault`] has no
    /// triggering release, which is the whole difference.
    fn overrun_by_release(&mut self, task: usize) -> Transition {
        let transition = self.raise(Fault::Overrun { task }, Context::Kernel);
        if transition == (Transition::JobSkipped { task }) {
            self.states[task] = TaskState::Ready;
        }
        transition
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

    /// The running task finished its job. Returns the completion, and what it delivered when the
    /// job completed inside a masked region it opened (§3.1.1 rule 4), in task order.
    ///
    /// # Panics
    ///
    /// If `task` is not the running one. The substrate reports completion of what it dispatched;
    /// anything else means the two disagree about what is on the processor, which is an invariant
    /// violation worth catching where it happens.
    pub fn complete(&mut self, task: usize) -> (Transition, heapless::Transitions<N>) {
        if let Some(fatal) = self.halted {
            return (Transition::Halted { fatal }, heapless::Transitions::new());
        }
        assert_eq!(
            self.running,
            Some(task),
            "completion reported for a task that is not running"
        );
        self.states[task] = TaskState::Suspended;
        self.running = None;
        // §3.1.1 rule 4: a job may complete inside a masked region it opened, and its completion
        // closes it. The depth is the job's and ends with it; what was latched is delivered as at
        // the outermost unmask, after the completion, so the completing task is released afresh.
        let delivered = if self.mask_depth > 0 {
            self.mask_depth = 0;
            self.deliver_latched()
        } else {
            heapless::Transitions::new()
        };
        (Transition::Completed { task }, delivered)
    }

    /// Raise a fault from outside: a stack guard, an unexpected trap or an assertion, raised in
    /// `context`; or an overrun some mechanism other than a release detected, an execution-budget
    /// monitor being the obvious one (§3.1.1 rule 1a), for which `context` does not matter.
    ///
    /// ⚠️ Such an overrun has no triggering release, so under `SkipLateJob` the late job is
    /// abandoned and the task waits for its next release: §3.1.1 rule 1 makes the *triggering*
    /// release the next job, and there is none here. Making the task ready would start a job no
    /// release paid for.
    pub fn fault(&mut self, fault: Fault, context: Context) -> Transition {
        if let Some(fatal) = self.halted {
            return Transition::Halted { fatal };
        }
        self.raise(fault, context)
    }

    /// An assertion failure the scheduler itself detects, raised by whatever is executing.
    fn assertion(&mut self, invariant: &'static str) {
        self.raise(Fault::InvariantViolated { invariant }, Context::Job);
    }

    /// Apply a fault's policy. An overrun contained under `SkipLateJob` leaves the task
    /// [`TaskState::Suspended`] and reports [`Transition::JobSkipped`], which
    /// [`Scheduler::overrun_by_release`] completes into a new job.
    fn raise(&mut self, fault: Fault, context: Context) -> Transition {
        // §3.1.1 rule 2: an overrun is the overrunning task's, whichever context holds the
        // processor; the other three are the executing context's, and kernel code is no task.
        let (task, interrupted) = match (fault, context) {
            (Fault::Overrun { task }, _) => (Some(task), None),
            (_, Context::Job) => (self.running, None),
            (_, Context::Kernel) => (None, self.running),
        };
        // §3.1.1 rule 3: a containable fault raised inside a masked region is not containable.
        // Terminating a job that holds the region leaves the nesting depth above zero with no
        // owner, so interrupts never return; forcing it to zero re-enables them with the
        // region's invariants half-restored. §8.1 offers no third option.
        let containable = fault.runtime_state_is_trustworthy();
        let escalated = containable && self.is_masked();
        if let (true, false, Some(task)) = (containable, escalated, task) {
            // §3.1.1 rule 5. The late job is abandoned — recorded, never silent.
            if self.running == Some(task) {
                self.running = None;
            }
            if self.policies[task] == OverrunPolicy::SkipLateJob {
                // Whether a new job follows is the triggering release's to say
                // (`overrun_by_release`).
                self.states[task] = TaskState::Suspended;
                return Transition::JobSkipped { task };
            }
            self.states[task] = TaskState::Faulted;
            return Transition::Faulted { task, fault };
        }
        // Rule 7: the fatal handler. The attributed task is marked in the table, which after a
        // halt is this implementation's and not evidence; the record below is.
        if let Some(task) = task {
            self.states[task] = TaskState::Faulted;
            if self.running == Some(task) {
                self.running = None;
            }
        }
        let fatal = *self.halted.get_or_insert(Fatal {
            fault,
            task,
            interrupted,
            escalated,
        });
        Transition::Halted { fatal }
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
        if let Some(fatal) = self.halted {
            return Decision::Halt { fatal };
        }
        // The highest-priority ready task is the lowest index, because the index IS the rank.
        let mut best: Option<usize> = None;
        for task in 0..N {
            if self.states[task] == TaskState::Ready {
                best = Some(task);
                break;
            }
        }
        let starts_a_job = match (self.running, best) {
            (Some(current), Some(candidate)) => candidate < current,
            (None, Some(_)) => true,
            _ => false,
        };
        if starts_a_job && self.is_masked() {
            // §3.1.1, Terms: every job starts at depth zero, which is what makes every section
            // open at a completion the completing job's (rule 4).
            self.assertion("dispatch-while-masked");
            return self.decide();
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

    /// At most two delivered transitions per task: a latched release, and the overrun latched
    /// beside it (§3.1.1 rule 1).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Transitions<const N: usize> {
        items: [[Option<Transition>; 2]; N],
        len: usize,
    }

    impl<const N: usize> Transitions<N> {
        /// An empty list.
        #[must_use]
        pub const fn new() -> Self {
            Self {
                items: [[None; 2]; N],
                len: 0,
            }
        }

        /// Append. Cannot overflow: at most two transitions are delivered per task.
        pub fn push(&mut self, transition: Transition) {
            self.items[self.len / 2][self.len % 2] = Some(transition);
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
            self.items
                .iter()
                .flatten()
                .take(self.len)
                .filter_map(|item| *item)
        }
    }

    impl<const N: usize> Default for Transitions<N> {
        fn default() -> Self {
            Self::new()
        }
    }
}
