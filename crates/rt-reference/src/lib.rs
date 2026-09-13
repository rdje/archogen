//! An **independent reference model** of the `rt-static-up-v1` task-lifecycle state machine
//! (`ROADMAP.md` §8 "Runtime and architecture substrate", §3.1 the profile table, §8.1 the
//! unsafe/synchronization boundaries).
//!
//! §12 M2's exit gate asks for "a separate simple reference model" alongside the engine
//! realization, and states the condition that makes it worth having:
//!
//! > A checker sharing the same erroneous recurrence with its reference does not qualify as
//! > independent.
//!
//! ⭐ **So this crate was derived from the contract text and nothing else.** Its author did not
//! read the implementation it is a reference for, has no dependency on it, and deliberately does
//! not share its vocabulary: agreement between the two is evidence only if it was reached twice
//! from `ROADMAP.md`, not once from `ROADMAP.md` and once by transcription. Where the two
//! disagree, the contract decides — and where the contract turns out not to decide, a
//! `⚠️ CONTRACT SILENT` note below names the gap. Those notes are the point of the exercise as
//! much as the model is: a silence found by two readers disagreeing is a silence the contract
//! can be amended to close.
//!
//! # What is modelled, and what is deliberately not
//!
//! §8 lists the initial mechanisms: "static task creation at boot, a fixed-priority ready
//! structure, release/timer management, interrupt dispatch, context switching, static memory
//! layout, and a bounded fault path". This model covers the **policy state transitions** of that
//! list, which §8 asks to be kept separate from the execution substrate:
//!
//! > Separate policy state transitions from the execution substrate.
//!
//! It therefore has no context layouts, no registers, no MMIO, no time. It reports *which*
//! context transition each event forces, in the vocabulary §7.4.1 requires an analysis to
//! identify ("the number and kind of context transitions"), but it charges nothing: costs belong
//! to a ledger under `docs/analysis/cost-accounting-v1.md`, and a reference model that also
//! priced transitions would be two models in a trench coat, each able to hide the other's error.
//!
//! # Shape
//!
//! [`Runtime<N>`] is the whole model. `N` is a const generic because §3.1 admits a "finite static
//! task set" and the profile excludes `dynamic-task-creation` and `runtime-heap`: the task count
//! is known at compile time, so it is a type parameter and every internal structure is a fixed
//! array. There is no `Vec`, no `Box`, no trait object and no allocation of any kind; the crate
//! is `no_std` outside its own test harness, and every operation is a total function returning
//! either a named effect or a named refusal.
#![cfg_attr(not(test), no_std)]
// §8.1: "Deny unsafe code by default; permit it in explicitly reviewed architecture, MMIO, and
// narrowly justified low-level modules." A pure policy model is none of those three, so it does
// not get the permission — `forbid`, not `deny`, because there is no justified exception to
// grant later and a reviewer should not have to check whether one was granted locally.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

use core::fmt;

// ---------------------------------------------------------------------------------------------
// Identities. §8.1: "Use distinct types for physical addresses, virtual addresses where relevant,
// MMIO regions, task IDs, priorities, and time units." Task ids and priorities are both small
// integers and are both indices into the same task set; a bare `usize` for each would make them
// interchangeable at a call site, which is exactly the confusion the clause forbids.
// ---------------------------------------------------------------------------------------------

/// The identity of one task in the static set.
///
/// It is an index into the boot-time task array, not an eADL logical id: §7.3 gives every task a
/// "stable logical ID" in the description, and [`TaskSpec::name`] carries it for diagnostics. The
/// index exists because the profile's task set is fixed at boot, so a position in it is a
/// permanent identity — §3.1's exclusion of `dynamic-task-creation` is what makes that true.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(usize);

impl TaskId {
    /// Names the task at `index` in the boot-time task array.
    ///
    /// The index is not validated here — [`Runtime`] refuses an out-of-range id with
    /// [`Refused::UnknownTask`], so an adapter driving this model from a foreign trace cannot
    /// smuggle a fabricated task into the set.
    #[must_use]
    pub const fn from_index(index: usize) -> Self {
        Self(index)
    }

    /// The task's position in the boot-time task array.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
}

/// A static task priority: a **rank**, where a numerically lower value is a higher priority.
///
/// The direction is not a preference. `docs/decisions/decision_priority-comparison-direction.md`
/// fixes it — "`N` is a rank, not a weight: a numerically lower `N` is a higher priority, and `1`
/// is the highest" — and §15 puts a comparison direction under migration discipline precisely
/// because reversing it is invisible: "every description stays byte-identical while every
/// ordering result inverts".
///
/// ⭐ **`Ord` is deliberately not derived.** A derived `Ord` on a newtype over `u16` would make
/// `a < b` mean "`a` is a *lower* priority", which is the opposite of what every reader of
/// `priority.cmp(&other)` will assume, and the mistake compiles. The only comparison this type
/// offers is [`Priority::outranks`], whose name cannot be read backwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Priority(u16);

impl Priority {
    /// The highest rank the decision record admits.
    pub const HIGHEST: Self = Self(1);

    /// A priority of the given rank.
    ///
    /// Rank `0` is constructible and is refused at boot ([`BootError::PriorityRankZero`]) rather
    /// than here, so that every admissibility rule about the task set lives in one place.
    #[must_use]
    pub const fn new(rank: u16) -> Self {
        Self(rank)
    }

    /// The numeric rank, as an eADL `(priority N)` clause carries it.
    #[must_use]
    pub const fn rank(self) -> u16 {
        self.0
    }

    /// Whether `self` is a **higher** priority than `other` — that is, a numerically smaller rank.
    ///
    /// This is the `hp(i)` relation of §7.4's recurrence, read the way the decision record fixes
    /// it: "`hp(i)` is `{ j : N_j < N_i }`".
    #[must_use]
    pub const fn outranks(self, other: Self) -> bool {
        self.0 < other.0
    }
}

// ---------------------------------------------------------------------------------------------
// Static configuration: what boot is given.
// ---------------------------------------------------------------------------------------------

/// What a task declares before boot.
///
/// This is the subset of §7.3's per-task record that the *lifecycle* state machine can act on.
/// §7.3 requires far more — "period or minimum inter-arrival time, relative deadline and its
/// reference event, maximum release jitter, execution bound, stack allocation, resource use,
/// allowed OS calls" — but every one of those is an input to analysis or to layout, not to a
/// state transition. Carrying them here would invite the model to look like it checked them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskSpec {
    /// The task's stable logical id (§7.3), used only in diagnostics.
    pub name: &'static str,
    /// Its static priority. §3.1: "Static unique task priorities".
    pub priority: Priority,
    /// What the runtime does when this task overruns.
    ///
    /// ⭐ This is **per task**, not a runtime-wide rule, because §7.3 puts it in the per-task
    /// record: the engine assembles "… allowed OS calls, and **overrun behavior**" for every
    /// task. A single global overrun policy would contradict a field the description is required
    /// to carry.
    pub on_overrun: OverrunAction,
}

/// What the runtime does to a task that overruns (§3.1 "Defined overrun … policy", §7.3
/// "overrun behavior").
///
/// ⚠️ **CONTRACT SILENT:** §7.3 names the field and §3.1 requires the policy to be "defined", but
/// neither states the domain of values. These three are chosen because they are the only
/// responses a profile with no memory isolation, no restart service and no dynamic task creation
/// can actually perform: abandon this job, abandon this task, or stop. Anything richer (restart
/// the task, degrade to a backup job, raise criticality) is either `mixed-criticality-scheduling`
/// or `dynamic-task-creation`, both of which the profile excludes by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverrunAction {
    /// Abandon the current job. The task stays in the schedule and its next release starts a
    /// fresh job. This is the only *containing* response, and it is sound only because §3.1
    /// admits "no self-suspension within a job" and no application mutexes: an abandoned job
    /// cannot be holding anything another task is waiting for.
    TerminateJob,
    /// Remove the task from the schedule permanently.
    ///
    /// Permanently is not an exaggeration: §3.1 excludes `dynamic-task-creation`, so nothing can
    /// ever put it back. The task set that remains is *not* the analyzed workload, which is why
    /// [`TaskState::Stopped`] is observable — a timing claim must not be quoted for a system
    /// running a strict subset of the tasks it was established for.
    StopTask,
    /// Treat the overrun as fatal and enter the fatal handler.
    Fatal,
}

// ---------------------------------------------------------------------------------------------
// Observable state.
// ---------------------------------------------------------------------------------------------

/// Where a task is in its lifecycle.
///
/// §9's catalog of "Interface and workload semantics" names "task lifecycle" as a thing that must
/// have a versioned API and model tests; §8 names "static task creation at boot" and "a bounded
/// fault path" as the mechanisms that bound it at each end. The states below are what is left
/// once the profile's exclusions are applied: with no application mutexes, no general IPC and no
/// self-suspension within a job, **there is no blocked state** — a task is never waiting for
/// anything except its own next release.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    /// Created at boot (§8 "static task creation at boot") and never yet released. No job exists.
    ///
    /// It behaves exactly like [`TaskState::Completed`] under every event, and is kept distinct
    /// anyway: it is the only state that proves a task has never run, which is what makes "the
    /// runtime dispatched something that was never released" a detectable error rather than an
    /// indistinguishable one.
    Created,
    /// A job exists and can run, but the processor is held by a higher-priority task.
    Ready,
    /// A job exists and holds the processor. §3.1: "one execution context runs at a time", so at
    /// most one task is ever in this state.
    Running,
    /// Its most recent job finished; it awaits its next release. **Not terminal** — the profile's
    /// workload is "periodic or sporadic releases", so this is the resting state of a healthy
    /// periodic task, reached once per period.
    Completed,
    /// A fault policy removed it from the schedule ([`OverrunAction::StopTask`]). Terminal.
    Stopped,
}

/// What the single processor is doing. §3.1: "One active core; one execution context runs at a
/// time."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Processor {
    /// No task is ready. F29 (§13.4) names this state directly: "a following switch to another
    /// task **or idle**".
    Idle,
    /// The named task holds the processor.
    Running(TaskId),
    /// A fault reached the fatal handler and scheduling has stopped (§8.1: "preserve a defined
    /// fatal handler and diagnostic evidence").
    Halted,
}

// ---------------------------------------------------------------------------------------------
// Context transitions. §7.4.1 requires an analysis to identify "the number and kind of context
// transitions"; the cost-accounting contract spells the kinds out as "one initial dispatch from
// idle, and one two-part switch (save outgoing, restore incoming) per task-to-task transition,
// charged in both directions". This model reports them and prices none of them.
// ---------------------------------------------------------------------------------------------

/// The context transition an event forced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transition {
    /// Idle → task. There is no outgoing context to preserve, which is why the contract gives it
    /// its own name and its own (smaller) cost rather than folding it into a switch: F29 charges
    /// "Initial dispatch from idle to L" 1 unit against 2 for every task-to-task switch.
    ///
    /// ⚠️ **CONTRACT SILENT:** F29 says "*Initial* dispatch", and its trace never returns to idle,
    /// so the contract never says what a *later* idle → task dispatch is called or costs. This
    /// model reports every idle → task transition as `Dispatch`, on the cost-accounting
    /// contract's structural reason — there is no outgoing context — rather than on the temporal
    /// one, since "first in the trace" is not a property of a transition.
    Dispatch {
        /// The task that took the processor.
        incoming: TaskId,
    },
    /// Task → task: save the outgoing context, restore the incoming one.
    ///
    /// Charged in both directions, including when the outgoing job has already *finished*: F29
    /// bills "[9, 11) Switch H to L" a full 2 units after H's job completed at 9. This model
    /// therefore emits a `Switch`, never a `ToIdle` followed by a `Dispatch`, whenever a task
    /// hands the processor straight to another task.
    Switch {
        /// The task that lost the processor.
        outgoing: TaskId,
        /// The task that took it.
        incoming: TaskId,
    },
    /// Task → idle: nothing is ready to run.
    ToIdle {
        /// The task that lost the processor.
        outgoing: TaskId,
    },
}

// ---------------------------------------------------------------------------------------------
// Event effects.
// ---------------------------------------------------------------------------------------------

/// What a release did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseEffect {
    /// The task became ready and took an idle processor.
    Dispatched(Transition),
    /// The task became ready, **outranked the running task, and took the processor from it**.
    ///
    /// This is the preemption half of §3.1's "preemption at the target's supported interrupt
    /// points": the answer to "does a newly-runnable higher-priority task take the processor" is
    /// yes, and it is immediate at any point where releases are being delivered at all. The
    /// "supported interrupt points" qualifier is modelled by interrupt masking, not by a
    /// separate notion of preemptibility — see [`Runtime::mask`].
    Preempted(Transition),
    /// The task became ready and waits: the running task outranks it. Fixed priority, and §3.1 is
    /// emphatic that it stays fixed — "The scheduling model is fixed-priority even if the test
    /// harness randomizes event ordering at permitted boundaries."
    Ready,
    /// Interrupts were masked, so the release was **latched** and will take effect when masking
    /// is fully lifted (§8.1 "Model synchronization and interrupt masking explicitly"; F15
    /// "Interrupt pending while masked → Correct delivery and acknowledgment behavior").
    Latched,
    /// The task already owed a job, so this release had nowhere to go.
    ///
    /// Either its current job had not finished, or an earlier release was still latched and
    /// undelivered. §3.1 excludes `general-ipc` — "including task-to-task queues" — and there is
    /// no per-task job queue either, so the runtime has exactly one slot per task. The release is
    /// therefore **neither stored nor silently dropped**: it is reported, because under a
    /// declared minimum separation `T` and a constrained deadline `D ≤ T` (§3.1 Workload) this
    /// cannot happen in the analyzed workload. Its occurrence means the separation was violated
    /// or the job overran, and either way §13.1 F17 requires the runtime to refuse timing
    /// assurance rather than to absorb it.
    ///
    /// ⚠️ **CONTRACT SILENT:** nothing says whether detecting this must *itself* trigger the §3.1
    /// overrun policy. This model reports and does not escalate, so that an adapter can drive an
    /// implementation that does either; [`Runtime::raise`] with [`Fault::Overrun`] is the
    /// escalation, and it is the caller's to make.
    Overrun,
    /// The task was stopped by a fault policy and can never be released again
    /// ([`OverrunAction::StopTask`]).
    Stopped,
}

/// What a fault did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultEffect {
    /// The job was abandoned; the task awaits its next release
    /// ([`OverrunAction::TerminateJob`]).
    JobTerminated(Transition),
    /// The task was removed from the schedule for good ([`OverrunAction::StopTask`]).
    TaskStopped(Transition),
    /// The runtime entered the fatal handler. Nothing is scheduled again, and the state is frozen
    /// as evidence (§8.1: "preserve a defined fatal handler and diagnostic evidence").
    Fatal,
}

/// The four faults §3.1 requires a defined policy for: "Defined overrun, unexpected-trap,
/// stack-guard, and assertion failure policy; bounded diagnostic handling."
///
/// ⭐ **Only the first is containable, and the contract is what decides that** — see each variant.
/// §8.1 asks for exactly this triage: "Distinguish expected errors, violated internal invariants,
/// and deliberate fatal traps."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// A job exceeded its declared execution bound.
    ///
    /// An **expected error**, in §8.1's sense: the description declared a bound (§7.3 "execution
    /// bound") and reality exceeded it. The runtime's own invariants are untouched — nothing was
    /// corrupted, one number was wrong — so containment is possible, and §7.3 makes the response
    /// a per-task declaration ([`TaskSpec::on_overrun`]) rather than a runtime-wide rule.
    Overrun,
    /// The architecture reported a trap the runtime did not plan for.
    ///
    /// **Fatal.** §8.1 separates "deliberate fatal traps" from this; an *unexpected* trap means
    /// the runtime's model of the machine is wrong, and a runtime that cannot say why the machine
    /// trapped cannot argue that the damage stops at one task. §3.1 removes the one mechanism
    /// that could bound it — "Trusted application components in one address space; no claim of
    /// isolation" — and the profile excludes `memory-isolation` by name. Continuing to schedule
    /// would be an unchecked path of exactly the kind §8.1 forbids.
    UnexpectedTrap,
    /// A task's stack guard was hit (§3.1 "stack-guard … policy", §7.6 "retain explicit
    /// guard/fault behavior").
    ///
    /// **Fatal**, for the same reason and one more. Task stacks are "separate statically
    /// allocated" (§3.1) in a single address space, so whatever lies beyond a guard belongs to
    /// something else with no protection boundary in between.
    ///
    /// ⚠️ **CONTRACT SILENT:** §3.1 and §7.6 say "stack guard" without saying whether it is a
    /// trapping guard region (which prevents the overflowing write) or a checked canary (which
    /// discovers it afterwards). Only the first reading would permit containment, and the
    /// contract does not promise it, so this model takes the fatal reading. A profile that
    /// pinned the mechanism could justify the other.
    StackGuard,
    /// An assertion failed (§3.1 "assertion failure policy").
    ///
    /// **Fatal.** An assertion is a written claim that an invariant holds; its failure is §8.1's
    /// "violated internal invariant", and scheduling onward from a state the program has just
    /// declared impossible is the "unchecked panic path in normal runtime operation" that §8.1
    /// tells the runtime to avoid.
    ///
    /// ⚠️ **CONTRACT SILENT:** §3.1 does not say whose assertion — the runtime's or the
    /// application's. This model treats both as fatal, because §3.1's Isolation row makes
    /// application components *trusted*: a trusted component's invariant being false is a
    /// statement about the whole address space, not about one task.
    AssertionFailure,
}

/// The single fault record the runtime preserves.
///
/// §3.1 requires "bounded diagnostic handling" and F26 requires a "bounded diagnostic path". The
/// bound here is one record: the **first** fault is kept and later ones never overwrite it, so
/// the fault path cannot re-enter itself and the evidence describes the cause rather than the
/// last consequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultRecord {
    /// The task that was running when the fault was raised. §3.1's single execution context is
    /// what makes this attribution unambiguous.
    pub task: TaskId,
    /// Which fault.
    pub fault: Fault,
    /// Whether it reached the fatal handler.
    pub fatal: bool,
    /// Whether interrupts were masked when it was raised.
    ///
    /// Recorded because it changes the outcome: see [`Runtime::raise`].
    pub masked: bool,
}

/// What lifting a mask did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unmasked {
    /// The mask depth that remains. Non-zero means this was an inner `unmask` of a nested pair
    /// and nothing was delivered.
    pub depth: u8,
    /// How many latched releases became ready.
    pub delivered: usize,
    /// How many latched releases found the task already owing a job
    /// ([`ReleaseEffect::Overrun`]).
    pub overruns: usize,
    /// How many latched releases named a stopped task ([`ReleaseEffect::Stopped`]).
    pub stopped: usize,
    /// The **single** context transition the delivery caused, if any.
    ///
    /// ⭐ Single, on purpose. Every latched release is applied to the ready set *before* the
    /// scheduler runs once, so delivering `n` releases costs at most one transition rather than
    /// `n`. Two clauses force this: §3.1's "bounded kernel critical sections", which an
    /// n-switch delivery would not be, and §7.4.1's demand that "the number and kind of context
    /// transitions" be identifiable — a count that depends on the order flags happened to be
    /// examined in is not identifiable. F29 agrees at the trace level: latched arrivals are
    /// "serviced before the next task computation interval", one servicing point, not one per
    /// arrival.
    pub transition: Option<Transition>,
}

// ---------------------------------------------------------------------------------------------
// Refusals. §8.1: "Distinguish expected errors, violated internal invariants, and deliberate
// fatal traps." These are the first kind — a caller asked for something the model will not do,
// and gets told which thing and why.
// ---------------------------------------------------------------------------------------------

/// Why an operation was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The runtime is in its fatal handler. Every operation is refused from then on; the state is
    /// evidence, not a schedule.
    Halted,
    /// The operation needs a running task and the processor is idle.
    NoTaskRunning,
    /// `unmask` was called with interrupts already enabled.
    ///
    /// ⚠️ **CONTRACT SILENT:** an unbalanced unmask is §8.1's "violated internal invariant", and
    /// §8.1 does not say whether a violated internal invariant must reach the fatal handler. This
    /// model refuses and does not halt, because the *model* is a checker being driven by an
    /// adapter: halting here would destroy the rest of a comparison run over a mistake in the
    /// harness. A real runtime detecting the same thing in its own kernel would have the stronger
    /// case for [`Fault::AssertionFailure`].
    NotMasked,
    /// The mask nesting depth is exhausted.
    ///
    /// ⚠️ **CONTRACT SILENT:** §3.1 requires "bounded kernel critical sections" and §7.3 requires
    /// every interrupt source to declare its "masking constraints", but no maximum nesting depth
    /// is stated. This model uses [`Runtime::MAX_MASK_DEPTH`] and **refuses rather than wraps**,
    /// which is the part that is not a free choice: a depth counter that wraps to zero re-enables
    /// interrupts in the middle of a critical section and reports success while doing it.
    MaskDepthExhausted,
    /// The id does not name a task in this static set.
    UnknownTask,
}

/// A violated structural invariant of the ready structure (see [`Runtime::check_invariants`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Violation {
    /// A ready task outranks the running one — fixed-priority scheduling has been abandoned.
    ReadyOutranksRunning {
        /// The ready task that should hold the processor.
        ready: TaskId,
        /// The task that holds it instead.
        running: TaskId,
    },
    /// The processor is idle while a task is ready.
    IdleWithReadyTask {
        /// A task that is ready and not running.
        ready: TaskId,
    },
    /// Two tasks are in [`TaskState::Running`]. §3.1: "one execution context runs at a time".
    MultipleRunning {
        /// The first task found running.
        first: TaskId,
        /// The second.
        second: TaskId,
    },
    /// The processor's idea of who is running disagrees with the task states.
    RunningDisagreesWithState {
        /// The task the processor believes is running, if any.
        processor: Option<TaskId>,
        /// The task whose state says it is running, if any.
        states: Option<TaskId>,
    },
}

/// Why a task set cannot boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    /// The task set is empty.
    ///
    /// ⚠️ **CONTRACT SILENT:** §3.1 says "finite static task set", and the empty set is finite.
    /// This model refuses it: a system with no task has no workload, so §7.2's second timing
    /// obligation ("the runtime, selected services, and generated configuration conform to that
    /// model") is vacuous, and a vacuously passing schedulability result is the kind of claim
    /// §7.1 exists to prevent. A profile that wants a task-free image should say so explicitly.
    NoTasks,
    /// Two tasks share a priority. §3.1: "Static **unique** task priorities".
    DuplicatePriority {
        /// The rank they share.
        rank: u16,
        /// The lower-indexed task.
        first: TaskId,
        /// The higher-indexed task.
        second: TaskId,
    },
    /// A task declares rank `0`.
    ///
    /// ⚠️ **CONTRACT SILENT:** the decision record says "`1` is the highest" and says nothing
    /// about `0`. This model refuses it rather than treating it as a still-higher priority,
    /// because admitting it would silently move the top of the range and §15 forbids changing a
    /// comparison direction or a parameter's meaning by inference. Admitting `0` is a language
    /// change, so it needs the decision record amended, not a model that tolerates it.
    PriorityRankZero {
        /// The offending task.
        task: TaskId,
    },
}

// ---------------------------------------------------------------------------------------------
// The model.
// ---------------------------------------------------------------------------------------------

/// The task-lifecycle state machine of `rt-static-up-v1`.
///
/// `N` is the size of the static task set. Every field is a fixed array of length `N`: §3.1
/// excludes `runtime-heap` ("Static task and kernel objects; no runtime heap allocation") and
/// `dynamic-task-creation` ("the task set is an input to the schedulability argument; a changing
/// set has no single analyzed workload"), and a const generic is how both of those become
/// unrepresentable rather than merely unused.
#[derive(Debug, Clone)]
pub struct Runtime<const N: usize> {
    spec: [TaskSpec; N],
    state: [TaskState; N],
    /// One pending-release flag per task — §8's "simplest bounded structures adequate for the
    /// profile". A flag, not a counter and not a queue: §3.1 excludes `general-ipc` "including
    /// task-to-task queues", so a second undelivered release has nowhere to be stored and must be
    /// reported instead (see [`ReleaseEffect::Overrun`]).
    latched: [bool; N],
    running: Option<TaskId>,
    mask_depth: u8,
    halted: bool,
    first_fault: Option<FaultRecord>,
}

impl<const N: usize> Runtime<N> {
    /// The deepest interrupt-mask nesting this model will accept.
    ///
    /// See [`Refused::MaskDepthExhausted`] for why there is a limit and why exceeding it is a
    /// refusal rather than a wrap. The value itself is arbitrary; its finiteness is not.
    pub const MAX_MASK_DEPTH: u8 = u8::MAX;

    /// Create the static task set. This is §8's "static task creation at boot", and it is the
    /// only way a task ever comes into existence.
    ///
    /// Every task starts in [`TaskState::Created`] with interrupts unmasked and the processor
    /// idle.
    ///
    /// ⚠️ **CONTRACT SILENT:** whether a periodic task's *first* job is released at boot or at its
    /// first timer tick is not stated anywhere. §3.1 describes "periodic or sporadic releases
    /// with declared minimum separation" without fixing the phase of the first one, while F29
    /// simply asserts "The initial state already contains L's ready job" for its own fixture.
    /// This model takes no position: boot releases nobody, and a caller that wants F29's initial
    /// state calls [`Runtime::release`] to establish it. That keeps the choice visible in the
    /// trace instead of buried in the constructor, where a disagreement about it would look like
    /// a disagreement about scheduling.
    ///
    /// # Errors
    ///
    /// [`BootError`] if the set is empty, shares a priority, or declares rank `0`.
    pub fn boot(spec: [TaskSpec; N]) -> Result<Self, BootError> {
        if N == 0 {
            return Err(BootError::NoTasks);
        }
        for (i, task) in spec.iter().enumerate() {
            if task.priority.rank() == 0 {
                return Err(BootError::PriorityRankZero { task: TaskId(i) });
            }
            // O(N²) over a set fixed at compile time, checked once. §3.1's uniqueness requirement
            // is what makes ascending rank a *total* order, which is what lets the scheduler below
            // pick a winner with no tie-break rule at all — so this check is load-bearing for the
            // rest of the model, not a formality.
            for (j, other) in spec.iter().enumerate().skip(i + 1) {
                if task.priority == other.priority {
                    return Err(BootError::DuplicatePriority {
                        rank: task.priority.rank(),
                        first: TaskId(i),
                        second: TaskId(j),
                    });
                }
            }
        }
        Ok(Self {
            spec,
            state: [TaskState::Created; N],
            latched: [false; N],
            running: None,
            mask_depth: 0,
            halted: false,
            first_fault: None,
        })
    }

    // -- observation ---------------------------------------------------------------------------

    /// How many tasks the static set holds.
    #[must_use]
    pub const fn task_count(&self) -> usize {
        N
    }

    /// Every task id, in declaration order.
    pub fn tasks(&self) -> impl Iterator<Item = TaskId> {
        (0..N).map(TaskId::from_index)
    }

    /// What the processor is doing.
    #[must_use]
    pub fn processor(&self) -> Processor {
        if self.halted {
            return Processor::Halted;
        }
        match self.running {
            Some(task) => Processor::Running(task),
            None => Processor::Idle,
        }
    }

    /// A task's lifecycle state, or `None` if the id names no task in this set.
    #[must_use]
    pub fn state(&self, task: TaskId) -> Option<TaskState> {
        self.state.get(task.index()).copied()
    }

    /// A task's static declaration, or `None` if the id names no task in this set.
    #[must_use]
    pub fn spec(&self, task: TaskId) -> Option<&TaskSpec> {
        self.spec.get(task.index())
    }

    /// Whether a release for this task is latched and undelivered (F15's "pending while masked").
    #[must_use]
    pub fn is_latched(&self, task: TaskId) -> Option<bool> {
        self.latched.get(task.index()).copied()
    }

    /// The current interrupt-mask nesting depth; `0` means interrupts are enabled.
    #[must_use]
    pub const fn mask_depth(&self) -> u8 {
        self.mask_depth
    }

    /// Whether releases are currently deferred.
    #[must_use]
    pub const fn is_masked(&self) -> bool {
        self.mask_depth > 0
    }

    /// The one preserved fault record (§3.1 "bounded diagnostic handling").
    #[must_use]
    pub const fn fault_record(&self) -> Option<FaultRecord> {
        self.first_fault
    }

    // -- events --------------------------------------------------------------------------------

    /// A release arrives for `task`.
    ///
    /// This is the moment the task becomes eligible to run — which, per
    /// `docs/analysis/cost-accounting-v1.md`, is *ISR completion*, not interrupt arrival: "a
    /// release is signalled by an interrupt; the task becomes ready at ISR completion". The
    /// arrival itself, if it lands inside a masked region, is what [`Runtime::mask`] latches.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`] if the runtime is in its fatal handler, or [`Refused::UnknownTask`].
    pub fn release(&mut self, task: TaskId) -> Result<ReleaseEffect, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        let index = self.index_of(task)?;

        if self.is_masked() {
            // F29: "arrivals during them are latched and serviced before the next task
            // computation interval". A latch records the arrival and inspects nothing: the ISR
            // that would have examined the task's state has not run yet. Whether the task can
            // actually accept the release is therefore decided at delivery, in `unmask`, which is
            // where an overrun or a stopped task is reported.
            if self.latched[index] {
                return Ok(ReleaseEffect::Overrun);
            }
            self.latched[index] = true;
            return Ok(ReleaseEffect::Latched);
        }

        match self.state[index] {
            TaskState::Stopped => Ok(ReleaseEffect::Stopped),
            // The task already owes a job. One slot per task, so there is nowhere to put a
            // second and nothing to silently drop it into.
            TaskState::Ready | TaskState::Running => Ok(ReleaseEffect::Overrun),
            TaskState::Created | TaskState::Completed => {
                self.state[index] = TaskState::Ready;
                Ok(match self.take_processor() {
                    None => ReleaseEffect::Ready,
                    Some((None, incoming)) => {
                        ReleaseEffect::Dispatched(Transition::Dispatch { incoming })
                    }
                    Some((Some(outgoing), incoming)) => {
                        ReleaseEffect::Preempted(Transition::Switch { outgoing, incoming })
                    }
                })
            }
        }
    }

    /// The running task's current job ends.
    ///
    /// §3.1 admits "bounded jobs; no self-suspension within a job", so this is the *only* way a
    /// task gives up the processor voluntarily — there is no yield, no block and no wait. The job
    /// boundary is also the timing observation boundary: "a job's response interval ends at the
    /// end of its useful computation; the switch away from it is outside that job's interval"
    /// (`docs/analysis/cost-accounting-v1.md`), which is why the returned [`Transition`] is
    /// reported separately rather than folded into the completion.
    ///
    /// Masking does **not** defer this. F29 latches *arrivals*; a completion is the running task
    /// reaching the end of its own computation, not an asynchronous event, and deferring it would
    /// leave a finished job holding the processor.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], or [`Refused::NoTaskRunning`] if the processor is idle.
    pub fn complete(&mut self) -> Result<Transition, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        let outgoing = self.running.ok_or(Refused::NoTaskRunning)?;
        self.state[outgoing.index()] = TaskState::Completed;
        Ok(self.vacate(outgoing))
    }

    /// Mask interrupts: releases arriving from now on are latched rather than delivered.
    ///
    /// §8.1 requires this to exist and to be explicit: "Model synchronization and interrupt
    /// masking explicitly. Banning a mutex type does not eliminate races or blocking." It is also
    /// how §3.1's "preemption at the target's supported interrupt points" is expressed — a masked
    /// region *is* the absence of an interrupt point, so a higher-priority release inside one
    /// cannot preempt until the region ends.
    ///
    /// Nesting is counted, not flagged. §7.3 requires every interrupt source to declare its
    /// "masking constraints" and §3.1 requires "bounded kernel critical sections"; a boolean
    /// would let an inner critical section's exit re-enable interrupts inside an outer one, which
    /// is a race the contract asks to be modelled rather than assumed away.
    ///
    /// Returns the new nesting depth.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], or [`Refused::MaskDepthExhausted`] at [`Runtime::MAX_MASK_DEPTH`].
    pub fn mask(&mut self) -> Result<u8, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        if self.mask_depth == Self::MAX_MASK_DEPTH {
            return Err(Refused::MaskDepthExhausted);
        }
        self.mask_depth += 1;
        Ok(self.mask_depth)
    }

    /// Unmask interrupts one level.
    ///
    /// At depth `1 → 0` every latched release is delivered and **then** the scheduler runs once;
    /// see [`Unmasked::transition`] for why the order is forced rather than chosen. Deliveries are
    /// processed in ascending rank, so the report is deterministic and matches the decision
    /// record's rule for coincident releases: "A release trace emits coincident releases in
    /// ascending `N`."
    ///
    /// An inner unmask of a nested pair delivers nothing.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], or [`Refused::NotMasked`] if interrupts are already enabled.
    pub fn unmask(&mut self) -> Result<Unmasked, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        if self.mask_depth == 0 {
            return Err(Refused::NotMasked);
        }
        self.mask_depth -= 1;
        if self.mask_depth > 0 {
            return Ok(Unmasked {
                depth: self.mask_depth,
                delivered: 0,
                overruns: 0,
                stopped: 0,
                transition: None,
            });
        }

        let mut delivered = 0;
        let mut overruns = 0;
        let mut stopped = 0;
        while let Some(task) = self.next_latched_by_rank() {
            self.latched[task.index()] = false;
            match self.state[task.index()] {
                TaskState::Stopped => stopped += 1,
                TaskState::Ready | TaskState::Running => overruns += 1,
                TaskState::Created | TaskState::Completed => {
                    self.state[task.index()] = TaskState::Ready;
                    delivered += 1;
                }
            }
        }
        let transition = self
            .take_processor()
            .map(|(outgoing, incoming)| match outgoing {
                None => Transition::Dispatch { incoming },
                Some(outgoing) => Transition::Switch { outgoing, incoming },
            });
        Ok(Unmasked {
            depth: 0,
            delivered,
            overruns,
            stopped,
            transition,
        })
    }

    /// Raise a fault against the running task.
    ///
    /// §3.1's single execution context is what makes the attribution unambiguous: one core, one
    /// running context, so a fault has exactly one task to belong to. That is also why there is no
    /// `task` parameter — a fault attributed to a task that was not running would be a claim about
    /// concurrency the profile does not admit.
    ///
    /// ⚠️ **CONTRACT SILENT:** nothing says what happens when a *containable* fault is raised
    /// inside a masked region. This model **escalates it to fatal**, and the reasoning is the
    /// contract's own: terminating a job that holds the mask leaves the depth counter above zero
    /// with no owner, so interrupts never come back and the runtime dies silently; forcing the
    /// depth to zero instead re-enables interrupts in the middle of a region whose invariants the
    /// faulting job was halfway through restoring. §8.1 offers no third option — "Avoid unchecked
    /// panic paths in normal runtime operation; preserve a defined fatal handler and diagnostic
    /// evidence" — so the defined fatal handler is where this goes.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], or [`Refused::NoTaskRunning`] if the processor is idle.
    pub fn raise(&mut self, fault: Fault) -> Result<FaultEffect, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        let task = self.running.ok_or(Refused::NoTaskRunning)?;
        let masked = self.is_masked();

        let action = match fault {
            Fault::Overrun => self.spec[task.index()].on_overrun,
            // The other three are fatal by the reasoning recorded on each `Fault` variant; the
            // per-task declaration of §7.3 covers overrun behaviour and nothing else.
            Fault::UnexpectedTrap | Fault::StackGuard | Fault::AssertionFailure => {
                OverrunAction::Fatal
            }
        };
        let action = if masked && !matches!(action, OverrunAction::Fatal) {
            OverrunAction::Fatal
        } else {
            action
        };

        let fatal = matches!(action, OverrunAction::Fatal);
        self.record_fault(FaultRecord {
            task,
            fault,
            fatal,
            masked,
        });

        Ok(match action {
            OverrunAction::Fatal => {
                // Freeze everything. The task states, the mask depth and the running task are
                // diagnostic evidence now (§8.1), and a fatal handler that tidied up would be
                // destroying the record it exists to preserve.
                self.halted = true;
                FaultEffect::Fatal
            }
            OverrunAction::TerminateJob => {
                self.state[task.index()] = TaskState::Completed;
                FaultEffect::JobTerminated(self.vacate(task))
            }
            OverrunAction::StopTask => {
                self.state[task.index()] = TaskState::Stopped;
                FaultEffect::TaskStopped(self.vacate(task))
            }
        })
    }

    // -- invariants ----------------------------------------------------------------------------

    /// Check the structural invariants of the fixed-priority ready structure.
    ///
    /// These are the properties §7.1 calls a "model proof" candidate — "A state-machine invariant
    /// is machine checked" — and they are the ones an adapter should assert after *every* event on
    /// both this model and the implementation it drives. A model that agrees event-by-event on
    /// which task runs but silently loses the invariant in between is agreeing by luck.
    ///
    /// # Errors
    ///
    /// The first [`Violation`] found.
    pub fn check_invariants(&self) -> Result<(), Violation> {
        let mut running_by_state: Option<TaskId> = None;
        for (i, &state) in self.state.iter().enumerate() {
            if state == TaskState::Running {
                if let Some(first) = running_by_state {
                    return Err(Violation::MultipleRunning {
                        first,
                        second: TaskId(i),
                    });
                }
                running_by_state = Some(TaskId(i));
            }
        }
        if running_by_state != self.running {
            return Err(Violation::RunningDisagreesWithState {
                processor: self.running,
                states: running_by_state,
            });
        }
        let Some(ready) = self.highest_ready() else {
            return Ok(());
        };
        match self.running {
            None => Err(Violation::IdleWithReadyTask { ready }),
            Some(running) => {
                if self.spec[ready.index()]
                    .priority
                    .outranks(self.spec[running.index()].priority)
                {
                    Err(Violation::ReadyOutranksRunning { ready, running })
                } else {
                    Ok(())
                }
            }
        }
    }

    // -- internals -----------------------------------------------------------------------------

    fn index_of(&self, task: TaskId) -> Result<usize, Refused> {
        if task.index() < N {
            Ok(task.index())
        } else {
            Err(Refused::UnknownTask)
        }
    }

    /// The highest-priority task in [`TaskState::Ready`].
    ///
    /// A linear scan over a compile-time-sized array is §8's "simplest bounded structures adequate
    /// for the profile": its cost is a constant of the task set, which is exactly what a bounded
    /// kernel critical section needs, and §8 warns against more ("Do not add a generic object
    /// manager, reference counting, or an asynchronous service framework without a workload that
    /// needs them"). Priority uniqueness (§3.1) means the winner is unique and no tie-break rule
    /// exists to get wrong.
    fn highest_ready(&self) -> Option<TaskId> {
        let mut best: Option<TaskId> = None;
        for (i, &state) in self.state.iter().enumerate() {
            if state != TaskState::Ready {
                continue;
            }
            let outranks_best = best.is_none_or(|b| {
                self.spec[i]
                    .priority
                    .outranks(self.spec[b.index()].priority)
            });
            if outranks_best {
                best = Some(TaskId(i));
            }
        }
        best
    }

    fn next_latched_by_rank(&self) -> Option<TaskId> {
        let mut best: Option<TaskId> = None;
        for (i, &latched) in self.latched.iter().enumerate() {
            if !latched {
                continue;
            }
            let outranks_best = best.is_none_or(|b| {
                self.spec[i]
                    .priority
                    .outranks(self.spec[b.index()].priority)
            });
            if outranks_best {
                best = Some(TaskId(i));
            }
        }
        best
    }

    /// Run the scheduler once, preemptively.
    ///
    /// Returns `(outgoing, incoming)` if a task took the processor. The running task keeps it
    /// unless a ready task **strictly** outranks it: fixed priority with no round-robin and no
    /// time slice, because §3.1 fixes the policy and the profile excludes every alternative
    /// (`mixed-criticality-scheduling`, `arbitrary-async-executors`) by name.
    fn take_processor(&mut self) -> Option<(Option<TaskId>, TaskId)> {
        let candidate = self.highest_ready()?;
        match self.running {
            Some(running) => {
                if !self.spec[candidate.index()]
                    .priority
                    .outranks(self.spec[running.index()].priority)
                {
                    return None;
                }
                self.state[running.index()] = TaskState::Ready;
                self.state[candidate.index()] = TaskState::Running;
                self.running = Some(candidate);
                Some((Some(running), candidate))
            }
            None => {
                self.state[candidate.index()] = TaskState::Running;
                self.running = Some(candidate);
                Some((None, candidate))
            }
        }
    }

    /// The running task has given up the processor (completed, terminated or stopped); hand it to
    /// whoever is next.
    ///
    /// This always yields a transition, and a task-to-task handover is one `Switch` rather than a
    /// `ToIdle` plus a `Dispatch` — F29 bills the post-completion handover "[9, 11) Switch H to L"
    /// as a single two-part switch.
    fn vacate(&mut self, outgoing: TaskId) -> Transition {
        self.running = None;
        match self.take_processor() {
            Some((_, incoming)) => Transition::Switch { outgoing, incoming },
            None => Transition::ToIdle { outgoing },
        }
    }

    /// Keep the first fault and only the first (§3.1 "bounded diagnostic handling", F26's
    /// "bounded diagnostic path"). A record that the newest fault overwrote would describe the
    /// consequence and lose the cause.
    fn record_fault(&mut self, record: FaultRecord) {
        if self.first_fault.is_none() {
            self.first_fault = Some(record);
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Diagnostics. §5.5 and §8.1 both ask for errors that name the violated obligation rather than a
// code; these sentences are what an adapter prints when this model and an implementation part
// company, so they say which clause was relied on.
// ---------------------------------------------------------------------------------------------

impl fmt::Display for BootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoTasks => write!(
                f,
                "the static task set is empty; a system with no task has no workload, so no \
                 schedulability statement about it would mean anything (§3.1 Workload, §7.2)"
            ),
            Self::DuplicatePriority {
                rank,
                first,
                second,
            } => write!(
                f,
                "tasks at index {} and {} both declare priority {rank}; §3.1 admits static \
                 *unique* task priorities, because uniqueness is what makes ascending rank a \
                 total order and removes the need for any tie-break rule",
                first.index(),
                second.index()
            ),
            Self::PriorityRankZero { task } => write!(
                f,
                "the task at index {} declares priority rank 0; the priority-direction decision \
                 record fixes 1 as the highest rank, and admitting 0 would move the top of the \
                 range by inference, which §15 forbids",
                task.index()
            ),
        }
    }
}

impl fmt::Display for Refused {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halted => write!(
                f,
                "the runtime is in its fatal handler; its state is preserved as diagnostic \
                 evidence and nothing is scheduled again (§8.1)"
            ),
            Self::NoTaskRunning => write!(
                f,
                "the processor is idle, and this operation needs the running task (§3.1: one \
                 execution context runs at a time)"
            ),
            Self::NotMasked => write!(
                f,
                "unmask with interrupts already enabled; the mask nesting is unbalanced, which \
                 §8.1 classes as a violated internal invariant"
            ),
            Self::MaskDepthExhausted => write!(
                f,
                "interrupt mask nesting depth exhausted; the depth is refused rather than wrapped, \
                 because a wrap re-enables interrupts inside a critical section and reports \
                 success (§3.1 bounded kernel critical sections)"
            ),
            Self::UnknownTask => write!(
                f,
                "the id names no task in the static set; §3.1 excludes dynamic task creation, so \
                 the set is exactly what booted"
            ),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadyOutranksRunning { ready, running } => write!(
                f,
                "task {} is ready and outranks running task {}; fixed-priority scheduling requires \
                 the highest-priority ready task to hold the processor (§3.1 Scheduling)",
                ready.index(),
                running.index()
            ),
            Self::IdleWithReadyTask { ready } => write!(
                f,
                "the processor is idle while task {} is ready",
                ready.index()
            ),
            Self::MultipleRunning { first, second } => write!(
                f,
                "tasks {} and {} are both running; §3.1 admits one execution context at a time",
                first.index(),
                second.index()
            ),
            Self::RunningDisagreesWithState { processor, states } => write!(
                f,
                "the processor believes {:?} is running and the task states say {:?}",
                processor.map(TaskId::index),
                states.map(TaskId::index)
            ),
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Tests. §12 M2 asks the reference to be testable on its own; §3.2's `hosted-playground` is where
// this runs, with the standing caveat that "Host execution speed provides no target WCET
// guarantee" — which costs nothing here, because this model has no notion of time at all.
// ---------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const H: TaskId = TaskId::from_index(0);
    const L: TaskId = TaskId::from_index(1);

    /// Two tasks, `H` outranking `L`, both terminating their job on overrun.
    fn two_tasks() -> Runtime<2> {
        Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .expect("the fixture is a valid static task set")
    }

    fn assert_sound(rt: &Runtime<2>) {
        rt.check_invariants().expect("invariants hold");
    }

    // -- priority direction --------------------------------------------------------------------

    #[test]
    fn a_numerically_lower_rank_is_the_higher_priority() {
        // The whole content of docs/decisions/decision_priority-comparison-direction.md.
        assert!(Priority::new(1).outranks(Priority::new(2)));
        assert!(!Priority::new(2).outranks(Priority::new(1)));
        assert!(!Priority::new(3).outranks(Priority::new(3)));
        assert_eq!(Priority::HIGHEST.rank(), 1);
    }

    // -- boot ----------------------------------------------------------------------------------

    #[test]
    fn boot_refuses_an_empty_task_set() {
        assert_eq!(Runtime::boot([]).unwrap_err(), BootError::NoTasks);
    }

    #[test]
    fn boot_refuses_duplicate_priorities() {
        let err = Runtime::boot([
            TaskSpec {
                name: "a",
                priority: Priority::new(3),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "b",
                priority: Priority::new(3),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap_err();
        assert_eq!(
            err,
            BootError::DuplicatePriority {
                rank: 3,
                first: TaskId::from_index(0),
                second: TaskId::from_index(1),
            }
        );
    }

    #[test]
    fn boot_refuses_priority_rank_zero() {
        let err = Runtime::boot([TaskSpec {
            name: "a",
            priority: Priority::new(0),
            on_overrun: OverrunAction::TerminateJob,
        }])
        .unwrap_err();
        assert_eq!(err, BootError::PriorityRankZero { task: H });
    }

    #[test]
    fn every_task_starts_created_and_nothing_runs() {
        let rt = two_tasks();
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.state(L), Some(TaskState::Created));
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.mask_depth(), 0);
        assert_sound(&rt);
    }

    #[test]
    fn an_id_outside_the_static_set_is_refused() {
        let mut rt = two_tasks();
        assert_eq!(
            rt.release(TaskId::from_index(7)).unwrap_err(),
            Refused::UnknownTask
        );
    }

    // -- lifecycle -----------------------------------------------------------------------------

    #[test]
    fn a_release_on_an_idle_processor_dispatches() {
        let mut rt = two_tasks();
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Dispatched(Transition::Dispatch { incoming: L })
        );
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_sound(&rt);
    }

    #[test]
    fn a_lower_priority_release_waits() {
        let mut rt = two_tasks();
        rt.release(H).unwrap();
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Ready);
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_sound(&rt);
    }

    #[test]
    fn a_higher_priority_release_preempts_immediately() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(
            rt.release(H).unwrap(),
            ReleaseEffect::Preempted(Transition::Switch {
                outgoing: L,
                incoming: H,
            })
        );
        assert_eq!(rt.processor(), Processor::Running(H));
        // Preemption does not end L's job: it is still owed and still ready.
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_sound(&rt);
    }

    #[test]
    fn completion_resumes_the_preempted_task() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(
            rt.complete().unwrap(),
            Transition::Switch {
                outgoing: H,
                incoming: L,
            }
        );
        assert_eq!(rt.state(H), Some(TaskState::Completed));
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_sound(&rt);
    }

    #[test]
    fn completion_with_nothing_ready_goes_idle() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(rt.complete().unwrap(), Transition::ToIdle { outgoing: L });
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.state(L), Some(TaskState::Completed));
        assert_sound(&rt);
    }

    #[test]
    fn a_completed_task_can_be_released_again() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.complete().unwrap();
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Dispatched(Transition::Dispatch { incoming: L })
        );
        assert_sound(&rt);
    }

    #[test]
    fn completing_with_an_idle_processor_is_refused() {
        let mut rt = two_tasks();
        assert_eq!(rt.complete().unwrap_err(), Refused::NoTaskRunning);
    }

    // -- the second release ---------------------------------------------------------------------

    #[test]
    fn releasing_a_running_task_again_is_an_overrun_and_is_not_queued() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Overrun);
        // Nothing was stored: one completion returns the task to rest, it does not uncover a
        // second job. §3.1 excludes queues, so there is no second job anywhere.
        rt.complete().unwrap();
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.state(L), Some(TaskState::Completed));
        assert_sound(&rt);
    }

    #[test]
    fn releasing_a_ready_task_again_is_also_an_overrun() {
        let mut rt = two_tasks();
        rt.release(H).unwrap();
        rt.release(L).unwrap();
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Overrun);
        assert_sound(&rt);
    }

    // -- masking -------------------------------------------------------------------------------

    #[test]
    fn a_release_arriving_while_masked_is_latched_and_takes_effect_at_unmask() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        // F15: pending, not delivered. L keeps the processor because a masked region is not one
        // of §3.1's "supported interrupt points".
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.is_latched(H), Some(true));
        assert_sound(&rt);

        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.delivered, 1);
        assert_eq!(
            lifted.transition,
            Some(Transition::Switch {
                outgoing: L,
                incoming: H,
            })
        );
        assert_eq!(rt.is_latched(H), Some(false));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_sound(&rt);
    }

    #[test]
    fn an_inner_unmask_of_a_nested_pair_delivers_nothing() {
        let mut rt = two_tasks();
        assert_eq!(rt.mask().unwrap(), 1);
        assert_eq!(rt.mask().unwrap(), 2);
        rt.release(H).unwrap();

        let inner = rt.unmask().unwrap();
        assert_eq!(inner.depth, 1);
        assert_eq!(inner.delivered, 0);
        assert_eq!(inner.transition, None);
        assert_eq!(rt.processor(), Processor::Idle);
        assert!(rt.is_masked());

        let outer = rt.unmask().unwrap();
        assert_eq!(outer.depth, 0);
        assert_eq!(outer.delivered, 1);
        assert_eq!(outer.transition, Some(Transition::Dispatch { incoming: H }));
        assert_sound(&rt);
    }

    #[test]
    fn many_latched_releases_cost_one_transition_and_the_winner_is_the_highest_priority() {
        let mut rt = two_tasks();
        rt.mask().unwrap();
        rt.release(L).unwrap();
        rt.release(H).unwrap();

        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.delivered, 2);
        // One transition, not two: the ready set is completed before the scheduler runs, so the
        // low-priority task is never dispatched only to be switched away from.
        assert_eq!(
            lifted.transition,
            Some(Transition::Dispatch { incoming: H })
        );
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_sound(&rt);
    }

    #[test]
    fn a_second_release_of_one_task_inside_a_masked_region_is_an_overrun() {
        let mut rt = two_tasks();
        rt.mask().unwrap();
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Overrun);
        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.delivered, 1);
        assert_sound(&rt);
    }

    #[test]
    fn a_latched_release_for_a_task_that_still_owes_a_job_is_reported_at_delivery() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.release(L).unwrap();
        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.delivered, 0);
        assert_eq!(lifted.overruns, 1);
        assert_eq!(lifted.transition, None);
        assert_sound(&rt);
    }

    #[test]
    fn masking_does_not_defer_a_completion() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        // A completion is the running task reaching the end of its own computation, not an
        // arrival, so F29's latching rule does not apply to it.
        assert_eq!(rt.complete().unwrap(), Transition::ToIdle { outgoing: L });
        assert_eq!(rt.processor(), Processor::Idle);
        assert!(rt.is_masked());
        assert_sound(&rt);
    }

    #[test]
    fn an_unbalanced_unmask_is_refused() {
        let mut rt = two_tasks();
        assert_eq!(rt.unmask().unwrap_err(), Refused::NotMasked);
    }

    #[test]
    fn mask_nesting_is_bounded_and_refuses_rather_than_wrapping() {
        let mut rt = two_tasks();
        for _ in 0..Runtime::<2>::MAX_MASK_DEPTH {
            rt.mask().unwrap();
        }
        assert_eq!(rt.mask_depth(), Runtime::<2>::MAX_MASK_DEPTH);
        assert_eq!(rt.mask().unwrap_err(), Refused::MaskDepthExhausted);
        // The critical section is intact: the refusal did not silently re-enable interrupts.
        assert_eq!(rt.mask_depth(), Runtime::<2>::MAX_MASK_DEPTH);
    }

    // -- faults --------------------------------------------------------------------------------

    #[test]
    fn an_unexpected_trap_is_fatal_for_the_whole_runtime() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(rt.raise(Fault::UnexpectedTrap).unwrap(), FaultEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(
            rt.fault_record(),
            Some(FaultRecord {
                task: L,
                fault: Fault::UnexpectedTrap,
                fatal: true,
                masked: false,
            })
        );
        // Everything is refused from here, including another release.
        assert_eq!(rt.release(H).unwrap_err(), Refused::Halted);
        assert_eq!(rt.complete().unwrap_err(), Refused::Halted);
        assert_eq!(rt.mask().unwrap_err(), Refused::Halted);
        assert_eq!(
            rt.raise(Fault::AssertionFailure).unwrap_err(),
            Refused::Halted
        );
    }

    #[test]
    fn a_stack_guard_and_an_assertion_failure_are_fatal_too() {
        for fault in [Fault::StackGuard, Fault::AssertionFailure] {
            let mut rt = two_tasks();
            rt.release(L).unwrap();
            assert_eq!(rt.raise(fault).unwrap(), FaultEffect::Fatal);
            assert_eq!(rt.processor(), Processor::Halted);
        }
    }

    #[test]
    fn the_fatal_handler_preserves_the_state_that_explains_it() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.raise(Fault::UnexpectedTrap).unwrap();
        // H was running and L was preempted; both readings survive the halt, because a fatal
        // handler that tidied up would be destroying its own evidence.
        assert_eq!(rt.state(H), Some(TaskState::Running));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
    }

    #[test]
    fn the_first_fault_is_the_one_kept() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.raise(Fault::Overrun).unwrap();
        rt.release(H).unwrap();
        rt.raise(Fault::UnexpectedTrap).unwrap();
        let record = rt.fault_record().expect("a fault was recorded");
        assert_eq!(record.fault, Fault::Overrun);
        assert_eq!(record.task, L);
        assert!(!record.fatal);
    }

    #[test]
    fn an_overrun_terminates_only_the_job_when_the_task_declares_that() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(
            rt.raise(Fault::Overrun).unwrap(),
            FaultEffect::JobTerminated(Transition::Switch {
                outgoing: H,
                incoming: L,
            })
        );
        assert_eq!(rt.state(H), Some(TaskState::Completed));
        assert_eq!(rt.processor(), Processor::Running(L));
        // The task is still in the schedule and its next release starts a fresh job.
        rt.complete().unwrap();
        assert_eq!(
            rt.release(H).unwrap(),
            ReleaseEffect::Dispatched(Transition::Dispatch { incoming: H })
        );
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_can_remove_a_task_from_the_schedule_for_good() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::StopTask,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.release(H).unwrap();
        assert_eq!(
            rt.raise(Fault::Overrun).unwrap(),
            FaultEffect::TaskStopped(Transition::ToIdle { outgoing: H })
        );
        assert_eq!(rt.state(H), Some(TaskState::Stopped));
        // §3.1 excludes dynamic task creation, so there is no way back.
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Stopped);
        assert_eq!(rt.processor(), Processor::Idle);
        assert_sound(&rt);
    }

    #[test]
    fn a_stopped_task_is_skipped_by_the_scheduler_even_when_latched() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::StopTask,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.release(H).unwrap();
        rt.raise(Fault::Overrun).unwrap();
        rt.mask().unwrap();
        rt.release(H).unwrap();
        rt.release(L).unwrap();
        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.stopped, 1);
        assert_eq!(lifted.delivered, 1);
        assert_eq!(
            lifted.transition,
            Some(Transition::Dispatch { incoming: L })
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_containable_fault_inside_a_masked_region_escalates_to_fatal() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        // TerminateJob would leave the mask depth above zero with no owner; there is no safe
        // continuation, so §8.1's defined fatal handler is where this goes.
        assert_eq!(rt.raise(Fault::Overrun).unwrap(), FaultEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        let record = rt.fault_record().expect("a fault was recorded");
        assert!(record.masked);
        assert!(record.fatal);
    }

    #[test]
    fn a_fault_with_an_idle_processor_has_no_task_to_attribute_it_to() {
        let mut rt = two_tasks();
        assert_eq!(
            rt.raise(Fault::Overrun).unwrap_err(),
            Refused::NoTaskRunning
        );
    }

    // -- the contract's own fixture --------------------------------------------------------------

    /// §13.4's repeated-preemption fixture, replayed as a sequence of lifecycle events.
    ///
    /// F29 is a *cost* fixture and this model prices nothing, so what is checked here is the half
    /// F29 depends on and states separately: the sequence of context transitions. The fixture
    /// bills "one initial dispatch" and four switches, and it is explicit about why that count is
    /// the thing that matters — "It deliberately creates two preemptions of one low-priority job,
    /// so omitted interrupt or resume costs can turn a real miss in the fixture into a false
    /// pass." A model that produced three switches instead of four would make the same false pass
    /// available to a ledger that trusted it.
    ///
    /// Each timer ISR is replayed as `mask` → `release` → `unmask`, because that is what the
    /// fixture describes: the ISR is nonpreemptible, H "becomes ready at ISR completion", and the
    /// switch to H is charged after the ISR interval, not inside it.
    #[test]
    fn f29_produces_one_initial_dispatch_and_four_switches() {
        let mut rt = two_tasks();
        let mut transitions = [None; 6];
        let mut n = 0;
        let record = |t: Transition, transitions: &mut [Option<Transition>; 6], n: &mut usize| {
            transitions[*n] = Some(t);
            *n += 1;
        };

        // "The initial state already contains L's ready job … Initial dispatch from idle to L."
        let ReleaseEffect::Dispatched(t) = rt.release(L).unwrap() else {
            panic!("L must take the idle processor")
        };
        record(t, &mut transitions, &mut n);
        assert_sound(&rt);

        for _ in 0..2 {
            // "[4, 5) Timer ISR for first H release" — nonpreemptible, so masked.
            rt.mask().unwrap();
            assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
            // "H becomes ready at ISR completion", and "[5, 7) Switch L to H" follows it.
            let lifted = rt.unmask().unwrap();
            record(
                lifted.transition.expect("H preempts L"),
                &mut transitions,
                &mut n,
            );
            assert_eq!(rt.processor(), Processor::Running(H));
            assert_sound(&rt);

            // "[7, 9) First H job" then "[9, 11) Switch H to L".
            record(rt.complete().unwrap(), &mut transitions, &mut n);
            assert_eq!(rt.processor(), Processor::Running(L));
            assert_sound(&rt);
        }

        // "[21, 23) Remaining L computation" and L's first job ends. The fixture stops here; the
        // transition away from L is outside it, and outside L's response interval too.
        record(rt.complete().unwrap(), &mut transitions, &mut n);

        assert_eq!(n, 6);
        assert_eq!(
            transitions,
            [
                Some(Transition::Dispatch { incoming: L }),
                Some(Transition::Switch {
                    outgoing: L,
                    incoming: H
                }),
                Some(Transition::Switch {
                    outgoing: H,
                    incoming: L
                }),
                Some(Transition::Switch {
                    outgoing: L,
                    incoming: H
                }),
                Some(Transition::Switch {
                    outgoing: H,
                    incoming: L
                }),
                Some(Transition::ToIdle { outgoing: L }),
            ]
        );
        let switches = transitions
            .iter()
            .flatten()
            .filter(|t| matches!(t, Transition::Switch { .. }))
            .count();
        assert_eq!(switches, 4, "§13.4 bills four task-to-task switches");
    }

    // -- invariants ----------------------------------------------------------------------------

    #[test]
    fn the_highest_priority_ready_task_always_holds_the_processor() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_sound(&rt);
        rt.release(H).unwrap();
        assert_sound(&rt);
        rt.complete().unwrap();
        assert_sound(&rt);
        rt.complete().unwrap();
        assert_sound(&rt);
    }
}
