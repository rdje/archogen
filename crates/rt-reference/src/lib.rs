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
//! ⭐ **And it was.** A differential comparison against an implementation of the same contract
//! found five divergences, every one of them a place the contract genuinely did not decide.
//! `ROADMAP.md` §3.1.1 ("Fault classification, attribution and containment", amendment of
//! 2026-09-13) and the new "The admissible range, and the runtime's index" section of
//! `docs/decisions/decision_priority-comparison-direction.md` now decide them. This model has
//! been brought into line with **the amended text**, still without reading the implementation:
//! every change is derived from §3.1.1's table and its rules, so where the two models now
//! agree they agree because both satisfy one contract, not because one was adjusted to the other.
//! Each note an amendment answers is **kept and rewritten** to cite it rather than deleted: that
//! a question was once open is part of the record, and erasing it would erase the evidence that
//! the agreement was earned.
//!
//! ⭐ **The amendment was then read the same way, and amended in its turn.** Three of this model's
//! notes found gaps that §3.1.1 itself had opened, and the rulings of 2026-10-01
//! (`docs/decisions/decision_findings-for-director-review.md` §6) closed them: rule 1 now observes
//! a release that finds its task's latch already full **at delivery**, and a new rule 4 lets a job
//! complete inside a masked region, closing it. Both are carried here from the amended text alone,
//! and the three notes are rewritten below in the same `⭐ AMENDED` form as the first five.
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
///
/// ⛔ **It is not a priority either.** The priority-direction record's 2026-09-13 amendment warns
/// that a runtime which makes a task's array index its rank ends up with a highest priority of `0`
/// while the language's highest is `1`, so that `runtime index = eADL rank − 1` becomes
/// load-bearing and unwritten — "which is precisely how an off-by-one survives review: both halves
/// are individually correct and nothing states the relation". This model never makes that
/// identification: [`TaskId`] and [`Priority`] are distinct types carrying unrelated numbers, and
/// declaration order has no scheduling meaning at all.
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
    ///
    /// The range is now closed at this end explicitly: "a rank is an integer `N ≥ 1`. Rank `0` is
    /// **not** admissible in a description" (`decision_priority-comparison-direction.md`,
    /// amendment of 2026-09-13).
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
/// ⚠️ **CONTRACT SILENT — still, after §3.1.1:** the amendment settles that an overrun *is*
/// containable and that the containing response is "its declared per-task policy (§7.3)", which is
/// the half this model needed; it still does not state the **domain** of values that policy ranges
/// over. §7.3 names the field and §3.1 requires the policy to be "defined", and neither states
/// what it may say. These three are chosen because they are the only
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
    ///
    /// It is the policy §3.1.1 rule 1 calls `SkipLateJob`: the late job is skipped and the task
    /// keeps its place in the schedule. When a **release** detected the overrun, that release is
    /// the next one — "under `SkipLateJob` it becomes the task's next job" — so the task is ready
    /// again at once (see [`ReleaseEffect::Overrun`]). An overrun raised through [`Runtime::raise`]
    /// has no triggering release, so there the task waits for its next one.
    TerminateJob,
    /// Remove the task from the schedule permanently.
    ///
    /// Permanently is not an exaggeration: §3.1 excludes `dynamic-task-creation`, so nothing can
    /// ever put it back. The task set that remains is *not* the analyzed workload, which is why
    /// [`TaskState::Stopped`] is observable — a timing claim must not be quoted for a system
    /// running a strict subset of the tasks it was established for.
    ///
    /// This and [`OverrunAction::Fatal`] are the two responses that *fault* the task rather than
    /// skip a job, and under both the release that detected the overrun goes with it — §3.1.1 rule
    /// 1: "under `Fault` it goes with the faulted task". A stopped task is never released again.
    StopTask,
    /// Treat the overrun as fatal and enter the fatal handler.
    ///
    /// The release that detected the overrun goes with the faulted task (§3.1.1 rule 1), as under
    /// [`OverrunAction::StopTask`]: a halted runtime schedules nothing.
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
    ///
    /// `outgoing` and `incoming` may name **the same task**. Since §3.1.1's 2026-10-01 rulings a
    /// job can end and be followed at once by the same task's next job — a late job skipped under
    /// `SkipLateJob` while its replacement starts (rule 1), or a job completing inside a masked
    /// region that latched its next release (rule 4). The finished job's context leaves and the
    /// fresh job's arrives, so by the same F29 reasoning it is one switch, not nothing and not a
    /// `ToIdle` plus a `Dispatch`.
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
    ///
    /// This is also what a release reports when the task's latch **already holds one**. §3.1.1
    /// rule 1: such a release "is observed at **delivery**, not at arrival: the latch keeps the
    /// overrun beside the release it holds". Nothing is decided here, so there is nothing else to
    /// report; [`Runtime::is_overrun_latched`] shows what the latch now keeps, and the policy's
    /// effect appears at delivery, in [`Unmasked::overruns`].
    Latched,
    /// The task already owed a job, so this release had nowhere to go — and the task's overrun
    /// policy has **already been applied**, with its effect carried here.
    ///
    /// Its current job had not finished. (A release whose task merely has an earlier release
    /// still *latched* is no longer reported here: §3.1.1 rule 1 observes it at delivery — see
    /// [`ReleaseEffect::Latched`].) §3.1 excludes `general-ipc` — "including task-to-task queues"
    /// — and there is no per-task job queue either, so the runtime has exactly one slot per task.
    /// The release is therefore **neither stored nor silently dropped**: under a declared minimum
    /// separation `T` and a constrained deadline `D ≤ T` (§3.1 Workload) this cannot happen in the
    /// analyzed workload, so its occurrence means the separation was violated or the job overran,
    /// and either way §13.1 F17 requires the runtime to refuse timing assurance rather than absorb
    /// it.
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 1, "Detection applies the policy".** This note used to read
    /// `⚠️ CONTRACT SILENT`, because nothing said whether detecting a second release must itself
    /// trigger the §3.1 overrun policy; this model reported the overrun and left the escalation to
    /// its caller. The amendment decides it the other way:
    ///
    /// > An overrun is a fault the moment the runtime observes a release for a task that still
    /// > owes a job; it does not wait for a separate decision. … it is the system's behaviour
    /// > rather than a caller's option.
    ///
    /// So detection applies [`TaskSpec::on_overrun`] here, and the resulting [`FaultEffect`] is
    /// this variant's payload.
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 1, the triggering release.** This note used to read
    /// `⚠️ CONTRACT SILENT`: the rule settled that the policy applies, not what becomes of the
    /// release that triggered it. Under `TerminateJob` a runtime could abandon the late job and
    /// leave the task at rest, or abandon it and start a new one from this very release, and this
    /// model took the first reading — "the triggering release is consumed by the fault" — because
    /// the second begins a response interval at an instant the analyzed workload never contains.
    /// The ruling of 2026-10-01 takes the second:
    ///
    /// > The release that triggers an overrun is the policy's: under `SkipLateJob` it becomes the
    /// > task's next job, and under `Fault` it goes with the faulted task.
    ///
    /// So under [`OverrunAction::TerminateJob`] — the contract's `SkipLateJob` — the late job is
    /// abandoned and the task is [`TaskState::Ready`] again with this release as its job. If it
    /// held the processor, the payload's transition is that handover, and since nothing can
    /// outrank a task that was running unmasked it is a `Switch` from the task to itself (see
    /// [`Transition::Switch`]); if it did not, nothing moves and the payload's transition is
    /// `None`. Under [`OverrunAction::StopTask`] and [`OverrunAction::Fatal`] the release is
    /// consumed with the task, as before. The objection this model had raised still stands as a
    /// statement about timing, not as a reading of the contract: the job this release starts is
    /// already outside the analyzed workload, which is why the fault is recorded for §13.1 F17.
    Overrun(FaultEffect),
    /// The task was stopped by a fault policy and can never be released again
    /// ([`OverrunAction::StopTask`]).
    Stopped,
}

/// What a fault did.
///
/// ⭐ The context transition became an [`Option`] with §3.1.1 rule 2. An overrun is attributed to
/// "the **overrunning** task, which need not be the running one", and containing a fault in a task
/// that does not hold the processor moves no context at all. `None` is therefore not an absence of
/// information: it is the positive statement that the schedule's occupant did not change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultEffect {
    /// The job was abandoned ([`OverrunAction::TerminateJob`], the contract's `SkipLateJob`).
    /// If a release detected the overrun, that release is the task's next job and the task is
    /// ready again; if the overrun was raised, the task awaits its next release (§3.1.1 rule 1).
    /// `Some` only if the faulting task held the processor.
    JobTerminated(Option<Transition>),
    /// The task was removed from the schedule for good ([`OverrunAction::StopTask`]). `Some` only
    /// if the faulting task held the processor.
    TaskStopped(Option<Transition>),
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
///
/// ⭐ **AMENDED — §3.1.1's table.** §3.1's four faults and §8.1's three classes were two lists with
/// nothing joining them, and this model joined them by inference. The amendment states the join,
/// and states attribution and containment with it:
///
/// | §3.1 fault | §8.1 class | Attributed to | Containable |
/// |---|---|---|---|
/// | Overrun | Expected error | the **overrunning** task, which need not be the running one | Yes |
/// | Stack guard | Violated internal invariant | the executing task, whose guard was breached | No |
/// | Unexpected trap | Deliberate fatal trap | the running task | No |
/// | Assertion failure | Violated internal invariant | the running task | No |
///
/// (The rows are quoted as amended on 2026-10-01, findings §6 (c) and (d): the trap row once read
/// "Deliberate fatal trap, or a surprise outside the model", which left the four-to-three mapping
/// partial, and the stack-guard row once named "the task whose guard was breached" in words other
/// than rule 2's. Neither change moves an outcome here.)
///
/// Each variant below now cites the row instead of arguing for it. The rows agree with the
/// readings this model had already taken, with one exception that was not a reading at all: the
/// attribution column, which §3.1.1 rule 2 separates from execution — see [`Runtime::raise`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// A job exceeded its declared execution bound.
    ///
    /// An **expected error**, in §8.1's sense: the description declared a bound (§7.3 "execution
    /// bound") and reality exceeded it. The runtime's own invariants are untouched — nothing was
    /// corrupted, one number was wrong — so containment is possible, and §7.3 makes the response
    /// a per-task declaration ([`TaskSpec::on_overrun`]) rather than a runtime-wide rule.
    ///
    /// It is the only one of the four whose §3.1.1 row says "Containable: Yes", and the only one
    /// attributed to a task other than the running one.
    Overrun,
    /// The architecture reported a trap the runtime did not plan for.
    ///
    /// **Fatal.** An *unexpected* trap means the runtime's model of the machine is wrong, and a
    /// runtime that cannot say why the machine trapped cannot argue that the damage stops at one
    /// task. §3.1 removes the one mechanism that could bound it — "Trusted application components
    /// in one address space; no claim of isolation" — and the profile excludes `memory-isolation`
    /// by name. Continuing to schedule would be an unchecked path of exactly the kind §8.1 forbids.
    ///
    /// §3.1.1's row confirms it and picks the class: "Deliberate fatal trap", attributed to the
    /// running task, "Containable: No". The fatal path is the deliberate response; what is
    /// unexpected is what the machine did to reach it.
    UnexpectedTrap,
    /// A task's stack guard was hit (§3.1 "stack-guard … policy", §7.6 "retain explicit
    /// guard/fault behavior").
    ///
    /// **Fatal**, for the same reason and one more. Task stacks are "separate statically
    /// allocated" (§3.1) in a single address space, so whatever lies beyond a guard belongs to
    /// something else with no protection boundary in between.
    ///
    /// ⭐ **AMENDED — §3.1.1's table.** This note used to read `⚠️ CONTRACT SILENT`, because §3.1
    /// and §7.6 say "stack guard" without saying whether it is a trapping guard region (which
    /// prevents the overflowing write) or a checked canary (which discovers it afterwards), and
    /// only the first reading could ever have permitted containment. The amendment decides the
    /// *outcome* without pinning the mechanism — "Violated internal invariant … Containable: No" —
    /// so the question no longer reaches this model: whichever mechanism a target implements, the
    /// response is the same one.
    StackGuard,
    /// An assertion failed (§3.1 "assertion failure policy").
    ///
    /// **Fatal.** An assertion is a written claim that an invariant holds; its failure is §8.1's
    /// "violated internal invariant", and scheduling onward from a state the program has just
    /// declared impossible is the "unchecked panic path in normal runtime operation" that §8.1
    /// tells the runtime to avoid.
    ///
    /// ⭐ **AMENDED — §3.1.1's table.** This note used to read `⚠️ CONTRACT SILENT`, because §3.1
    /// does not say whose assertion — the runtime's or the application's — and this model inferred
    /// that both must be fatal from §3.1's Isolation row making application components *trusted*.
    /// The amendment reaches the same place without the inference: "Violated internal invariant …
    /// attributed to the running task … Containable: No", with no distinction drawn by whose
    /// assertion it was.
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
    /// The task the fault is **attributed to**, per §3.1.1's table.
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 2.** This field used to be documented as "the task that was
    /// running when the fault was raised", on the reasoning that one core means one owner. The
    /// amendment separates the two ideas: "The single-core rule of §3.1 governs *execution*, not
    /// *attribution*." An overrun belongs to the overrunning task, which "is by construction not
    /// the running one"; a trap, a stack-guard breach and an assertion failure are "synchronous to
    /// the executing context and are attributed to it".
    pub task: TaskId,
    /// Which fault.
    pub fault: Fault,
    /// Whether it reached the fatal handler.
    pub fatal: bool,
    /// Whether interrupts were masked when it was raised.
    ///
    /// Recorded because it changes the outcome — §3.1.1 rule 3, see [`Runtime::raise`]. It is
    /// false for an overrun discovered while *delivering* latched releases, because rule 3 puts
    /// that moment outside the masked region — whether the region was closed by
    /// [`Runtime::unmask`] or by a completion inside it ([`Runtime::complete`], rule 4).
    pub masked: bool,
}

/// What closing the outermost masked region did: lifting a mask ([`Runtime::unmask`]), or a
/// completion that closed one ([`Runtime::complete`]).
///
/// The two share one record because §3.1.1 rule 4 makes them one delivery: a completion inside a
/// region delivers its latched releases "as at the outermost unmask". A completion outside any
/// region reports the same record with nothing delivered and the completion's own transition.
///
/// Every latched arrival processed lands in exactly one of `delivered`, `overruns` and `stopped`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unmasked {
    /// The mask depth that remains. Non-zero means this was an inner `unmask` of a nested pair
    /// and nothing was delivered. Always `0` after a completion, which closes every section.
    pub depth: u8,
    /// How many latched releases found their task owing no job, and made it ready.
    pub delivered: usize,
    /// How many latched releases found the task already owing a job, **each of which had that
    /// task's overrun policy applied** (§3.1.1 rule 1).
    ///
    /// The policy applied is the declared one, not an escalation. §3.1.1 rule 3 is explicit that
    /// "Delivery at unmask is **not** inside a masked region. A release latched during a critical
    /// section is delivered once the outermost section closes, so an overrun discovered at that
    /// moment applies its ordinary per-task policy."
    ///
    /// ⭐ This includes the overrun a latch **kept beside the release it held** — a second arrival
    /// for a task whose latch was already full (§3.1.1 rule 1, amended 2026-10-01). That overrun
    /// is counted here, at delivery, and nowhere at arrival: the held release is delivered first
    /// and the kept one is then judged against the job it has just started, exactly as the same
    /// two arrivals would be judged one instruction after the region closed.
    ///
    /// If one of those policies is [`OverrunAction::Fatal`], delivery stops there: the counts
    /// describe what was processed up to the fault, [`Unmasked::transition`] is `None`, and
    /// [`Runtime::processor`] reports [`Processor::Halted`].
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
    ///
    /// An overrun policy that vacates the processor during delivery is folded into the same single
    /// transition rather than causing one of its own: the faulting task stands down, the scheduler
    /// still runs exactly once at the end, and the result is one `Switch` (or one `ToIdle`) that
    /// names it as the outgoing task. A completion is folded in the same way, as rule 4 orders it
    /// — "the schedule is decided after" — so after [`Runtime::complete`] this is always `Some`
    /// and names the completing task as outgoing, unless delivery reached a fatal policy.
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
    /// A fault that is synchronous to the executing context named a task that is not the one
    /// holding the processor.
    ///
    /// §3.1.1 rule 2: a trap, a stack-guard breach and an assertion failure "are synchronous to
    /// the executing context and are attributed to it". Attributing one of them elsewhere is the
    /// concurrency claim the profile does not admit, so it is refused rather than recorded.
    NotTheRunningTask,
    /// [`Fault::Overrun`] named a task that owes no job, so there is nothing for it to have
    /// overrun.
    ///
    /// §3.1.1 rule 1 defines the fault in terms of "a task that still owes a job", which is
    /// exactly [`TaskState::Ready`] or [`TaskState::Running`]. A task that has never been
    /// released, has completed its last job, or has been stopped is not making late progress on
    /// anything.
    NoJobOwed,
    /// `unmask` was called with interrupts already enabled.
    ///
    /// ⚠️ **CONTRACT SILENT — narrowed by §3.1.1, not closed.** An unbalanced unmask is §8.1's
    /// "violated internal invariant", and the amendment's table now settles that such a fault,
    /// *once raised*, is not containable. What it still does not settle is whether a runtime
    /// detecting an unbalanced unmask must raise one: the table classifies the four faults §3.1
    /// names, and an unbalanced unmask is not among them. This model refuses and does not halt,
    /// because the *model* is a checker being driven by an adapter: halting here would destroy the
    /// rest of a comparison run over a mistake in the harness. A real runtime detecting the same
    /// thing in its own kernel would have the stronger case for [`Fault::AssertionFailure`], and
    /// §3.1.1 would then make it fatal.
    NotMasked,
    /// The mask nesting depth is exhausted.
    ///
    /// ⭐ **AMENDED — §3.1.1's second smaller decision.** This note used to read
    /// `⚠️ CONTRACT SILENT`, because §3.1 requires "bounded kernel critical sections" and §7.3
    /// requires every interrupt source to declare its "masking constraints", while no maximum
    /// nesting depth was stated anywhere. The amendment states it:
    ///
    /// > Kernel critical sections are bounded by a declared nesting depth, and exceeding it is
    /// > refused. … A depth counter that *wraps* re-enables interrupts inside a critical section
    /// > while reporting success; one that *saturates* stops counting, so the matching unmasks no
    /// > longer balance. Both are silent failures, so the bound is explicit and exceeding it is an
    /// > error.
    ///
    /// That is the reasoning this model had already given for refusing rather than wrapping, with
    /// saturation ruled out as well. The **value** of the bound remains the model's own —
    /// "declared" is where the amendment leaves it, and [`Runtime::MAX_MASK_DEPTH`] is this
    /// model's declaration of it.
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
    /// ⭐ **AMENDED — §3.1.1's first smaller decision.** This note used to read
    /// `⚠️ CONTRACT SILENT`: §3.1 says "finite static task set", and the empty set is finite. The
    /// amendment refuses it, and refuses it for the reason this model had given —
    ///
    /// > "Finite static task set" admits the empty set, and a system with no workload makes §7.2's
    /// > second timing obligation vacuous — a vacuously passing schedulability result is exactly
    /// > what §7.1 exists to prevent.
    ///
    /// — so the behaviour is unchanged and only its authority has: it is now the contract's
    /// reasoning rather than this model's inference from it.
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
    /// ⭐ **AMENDED — `decision_priority-comparison-direction.md`, "The admissible range, and the
    /// runtime's index".** This note used to read `⚠️ CONTRACT SILENT`: the record said "`1` is the
    /// highest" and said nothing about `0`. The amendment closes the range — "a rank is an integer
    /// `N ≥ 1`. Rank `0` is **not** admissible in a description … A description carrying
    /// `(priority 0)` is refused" — on this model's own ground, that "admitting it would move the
    /// top of the range by inference, and §15 makes a change to a parameter's meaning a versioned
    /// language change rather than a tolerance".
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
    /// One pending-release latch per task — §8's "simplest bounded structures adequate for the
    /// profile". Not a counter and not a queue: §3.1 excludes `general-ipc` "including
    /// task-to-task queues". It holds at most one release, and beside it at most one overrun —
    /// §3.1.1 rule 1: "the latch keeps the overrun beside the release it holds" — see [`Latch`].
    latch: [Latch; N],
    running: Option<TaskId>,
    mask_depth: u8,
    halted: bool,
    first_fault: Option<FaultRecord>,
}

impl<const N: usize> Runtime<N> {
    /// The deepest interrupt-mask nesting this model will accept.
    ///
    /// This is the "declared nesting depth" §3.1.1 requires a profile to bound its kernel
    /// critical sections by. See [`Refused::MaskDepthExhausted`] for why exceeding it is a refusal
    /// rather than a wrap or a saturation. The value itself is arbitrary; its finiteness, its
    /// being declared, and the refusal at it are not.
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
            latch: [Latch::Empty; N],
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
        self.latch
            .get(task.index())
            .map(|&latch| latch != Latch::Empty)
    }

    /// Whether this task's latch also keeps an **overrun** beside the release it holds: a second
    /// release arrived while the first was still latched (§3.1.1 rule 1, amended 2026-10-01).
    ///
    /// The overrun is recorded here and judged at delivery, where the task's declared policy
    /// applies outside every masked region — see [`Unmasked::overruns`]. Until then nothing about
    /// the task has changed and no fault is recorded.
    #[must_use]
    pub fn is_overrun_latched(&self, task: TaskId) -> Option<bool> {
        self.latch
            .get(task.index())
            .map(|&latch| latch == Latch::ReleaseAndOverrun)
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
    /// ⭐ **A release is also where an overrun is detected, and detection now applies the policy**
    /// (§3.1.1 rule 1) — see [`ReleaseEffect::Overrun`], which carries what the policy did. For a
    /// release that arrives inside a masked region, "detection" happens at its delivery, by
    /// [`Runtime::unmask`] or by a completion that closes the region ([`Runtime::complete`]).
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 1, a release into a full latch.** This note used to read
    /// `⚠️ CONTRACT SILENT`: rule 1 defined detection as "a release for a task that still owes a
    /// **job**", and a task whose latch slot is already full may owe only an undelivered
    /// *release*. This model counted that as owing a job, so rule 3 then escalated it — a doubled
    /// arrival inside a critical section was fatal where the same doubling one instruction later
    /// was contained — and nothing in §3.1.1 settled which side of the definition a full latch
    /// fell on. The ruling of 2026-10-01 settles it, on neither of the two sides the note offered:
    ///
    /// > A release that arrives while the task's latch already holds one is observed at
    /// > **delivery**, not at arrival: the latch keeps the overrun beside the release it holds, and
    /// > when the outermost section closes the release is delivered and the overrun detected, so the
    /// > task's policy applies outside every masked region. It is never lost, and never fatal for
    /// > landing inside a critical section rather than one instruction after it.
    ///
    /// So such a release now reports [`ReleaseEffect::Latched`] and changes nothing but the latch
    /// ([`Runtime::is_overrun_latched`]); no policy runs and no fault is recorded at arrival, and
    /// rule 3 never reaches it. At delivery the held release is delivered first and the kept
    /// overrun is then judged against the job that release started, so the task's declared policy
    /// applies there — unescalated, [`FaultRecord::masked`] false — and is counted in
    /// [`Unmasked::overruns`].
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
            // computation interval". A latch records the arrival and inspects the task's
            // lifecycle state not at all: the ISR that would have examined it has not run yet.
            // Whether the task can actually accept the release is therefore decided at delivery.
            // That now holds for a second arrival too (§3.1.1 rule 1, amended): the latch keeps
            // the overrun beside the release it holds, and nothing is decided here.
            self.latch[index] = match self.latch[index] {
                Latch::Empty => Latch::Release,
                // A third or later arrival is the same overrun the latch already keeps: the rule
                // keeps "the overrun", and under every policy the state after delivering two
                // arrivals is the state after delivering more, so a count would change a tally
                // and nothing else — see `Latch`.
                Latch::Release | Latch::ReleaseAndOverrun => Latch::ReleaseAndOverrun,
            };
            return Ok(ReleaseEffect::Latched);
        }

        match self.state[index] {
            TaskState::Stopped => Ok(ReleaseEffect::Stopped),
            // The task already owes a job. One slot per task, so there is nowhere to put a
            // second and nothing to silently drop it into — and §3.1.1 rule 1 makes this
            // observation the fault itself, applied to this task whether or not it is running.
            // The release is the policy's: under SkipLateJob it becomes the task's next job.
            TaskState::Ready | TaskState::Running => {
                let was_running = self.running == Some(task);
                let applied = self.overrun_detected_by_release(task);
                Ok(ReleaseEffect::Overrun(self.reschedule_after(
                    task,
                    was_running,
                    applied,
                )))
            }
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
    /// (`docs/analysis/cost-accounting-v1.md`), which is why the resulting [`Transition`] is
    /// reported separately — [`Unmasked::transition`] — rather than folded into the completion.
    ///
    /// Masking does **not** defer this. F29 latches *arrivals*; a completion is the running task
    /// reaching the end of its own computation, not an asynchronous event, and deferring it would
    /// leave a finished job holding the processor.
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 4.** This note used to read `⚠️ CONTRACT SILENT`: whether a job
    /// may *complete* inside a masked region it opened was undecided, and rule 3 sharpened the
    /// question without answering it. Its first ground — a job that holds the mask and ends
    /// "leaves the nesting depth above zero with no owner" — applied word for word to a job that
    /// merely *ends* there, but the rule scoped itself to "a containable **fault**", so this model
    /// let the completion through and left the mask at its depth, with a finished task still
    /// nominally its owner. The ruling of 2026-10-01 adds a rule for it:
    ///
    /// > A job may complete inside a masked region it opened, and its completion closes it. The
    /// > nesting depth returns to zero with the job, releases latched in the region are delivered
    /// > as at the outermost unmask, and the schedule is decided after. … A latched release is
    /// > judged at that delivery, so a task that completed inside the region is released afresh,
    /// > not overrun.
    ///
    /// So a completion is now three steps in that order. The job ends and its task stands down;
    /// the depth goes to `0` however deep the nesting was, closing every section; and the latched
    /// releases are delivered exactly as [`Runtime::unmask`] delivers them — outside every masked
    /// region, so an overrun found there applies its declared policy unescalated — before the
    /// scheduler runs once. The completing task's own latched release therefore finds it
    /// [`TaskState::Completed`] and starts a fresh job instead of overrunning the one that just
    /// ended. Outside a masked region nothing is latched and this is the plain completion it
    /// always was.
    ///
    /// ⭐ **This is why the method returns [`Unmasked`].** A completion inside a region can deliver
    /// releases, find overruns, and — if one of those policies is [`OverrunAction::Fatal`] — halt
    /// the runtime before any schedule is decided, which a bare [`Transition`] cannot express.
    /// Rule 4 makes the delivery "as at the outermost unmask", so it is reported in the record the
    /// outermost unmask already uses. Outside a region the record is `depth: 0`, nothing
    /// delivered, and `transition: Some(..)` naming the completing task as outgoing.
    ///
    /// "A masked region it opened" needs no owner field here. Inside a masked region the
    /// processor's occupant cannot change by any other route — arrivals latch, and rule 3 makes
    /// every fault there fatal — so whoever completes inside one held the processor when its
    /// outermost section opened.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], or [`Refused::NoTaskRunning`] if the processor is idle.
    pub fn complete(&mut self) -> Result<Unmasked, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        let outgoing = self.running.ok_or(Refused::NoTaskRunning)?;
        self.state[outgoing.index()] = TaskState::Completed;
        self.running = None;
        // §3.1.1 rule 4: "The nesting depth returns to zero with the job" — every section it
        // opened, not one level — and only then are the latched releases delivered, so that
        // delivery is outside every masked region exactly as rule 3's note requires of an unmask.
        self.mask_depth = 0;
        Ok(self.deliver_latched(Some(outgoing)))
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
    /// ⭐ **Delivery is outside the masked region, and §3.1.1 rule 3 says so.** The depth reaches
    /// zero first and the latched releases are serviced after it: "A release latched during a
    /// critical section is delivered once the outermost section closes, so an overrun discovered
    /// at that moment applies its ordinary per-task policy." The rule adds that this is not an
    /// implementer's subtlety but "the difference between a profile that can contain an overrun at
    /// all and one that cannot" — a runtime that delivered *inside* the region would escalate
    /// every latched overrun to fatal by rule 3, and containment would exist only on paper.
    ///
    /// A delivered release that finds its task still owing a job is therefore an overrun whose
    /// declared policy applies here (§3.1.1 rule 1). If that policy takes the processor away from
    /// the faulting task, the scheduler still runs only once, at the end: the whole unmask reports
    /// one [`Transition`]. If that policy is [`OverrunAction::Fatal`], delivery stops at it and
    /// the remaining latched releases stay undelivered — the state is evidence from then on.
    ///
    /// A task whose latch also keeps an overrun ([`Runtime::is_overrun_latched`]) has two arrivals
    /// to deliver, and they are delivered in the order they arrived: "the release is delivered and
    /// the overrun detected" (§3.1.1 rule 1). Both are delivered before the next task in rank, so
    /// the pair is judged exactly as the same two arrivals would be one instruction after the
    /// region closed — which is the ruling's own test: the outcome "may not depend on which side of
    /// an unmask an interrupt lands". Only the transition count differs, by the single-transition
    /// rule above.
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
        Ok(self.deliver_latched(None))
    }

    /// Raise `fault` against `task`.
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 2.** This method used to take no `task` parameter, on the
    /// reasoning that "§3.1's single execution context is what makes the attribution unambiguous:
    /// one core, one running context, so a fault has exactly one task to belong to", and that "a
    /// fault attributed to a task that was not running would be a claim about concurrency the
    /// profile does not admit". The amendment addresses that reasoning directly:
    ///
    /// > A release is signalled by an interrupt while some other context holds the processor, so
    /// > the overrunning task is by construction not the running one. The single-core rule of §3.1
    /// > governs *execution*, not *attribution*.
    ///
    /// So attribution is now a parameter, and §3.1.1's table decides what may be passed:
    ///
    /// - [`Fault::Overrun`] may name **any** task that still owes a job — [`TaskState::Ready`] or
    ///   [`TaskState::Running`] — and is attributed to it. Naming a task that owes no job is
    ///   [`Refused::NoJobOwed`], because rule 1 defines the fault in terms of a task that owes one.
    /// - The other three are "synchronous to the executing context and are attributed to it", so
    ///   they must name the running task: [`Refused::NoTaskRunning`] if the processor is idle and
    ///   [`Refused::NotTheRunningTask`] if some other task holds it. The model refuses rather than
    ///   silently re-attributing, because the misattribution is the interesting thing to catch.
    ///
    /// This is not the *only* way an overrun arises — §3.1.1 rule 1 makes a release that finds its
    /// task still owing a job a fault by itself, applied by [`Runtime::release`], and at delivery
    /// by [`Runtime::unmask`] and [`Runtime::complete`], without going through here. This entry
    /// point remains for an overrun detected some other way, an execution-budget monitor being the
    /// obvious one (§7.3's "execution bound"). Such an overrun has no triggering release, so under
    /// [`OverrunAction::TerminateJob`] the task is left [`TaskState::Completed`] to await its next
    /// one, where an overrun detected by a release starts that release as its next job.
    ///
    /// ⭐ **AMENDED — §3.1.1 rule 3.** A containable fault raised while interrupts are masked
    /// escalates to fatal. This note used to read `⚠️ CONTRACT SILENT` and this model escalated
    /// anyway, on the first of the two grounds the amendment now gives: terminating a job that
    /// holds the mask "leaves the nesting depth above zero with no owner, so interrupts never
    /// return", and forcing the depth to zero "re-enables them inside a region whose invariants the
    /// faulting job was partway through restoring". That ground alone would only cover a fault
    /// attributed to the mask holder, which rule 2 has just stopped being the only case; the rule
    /// carries a second, wider ground for exactly that reason (worded as amended on 2026-10-01,
    /// findings §6 (e), which moved the hazard from *changing* the schedule to *resuming* it):
    ///
    /// > containment means **resuming the schedule** from a state the critical section had not
    /// > finished making consistent, which is what a kernel critical section exists to prevent.
    /// > That holds whichever task the fault is attributed to, not only the one holding the mask.
    ///
    /// Delivery at unmask is explicitly *outside* the masked region and does not escalate; see
    /// [`Runtime::unmask`]. Nor does delivery at a completion that closes the region, and the
    /// completion itself is not a fault at all (rule 4); see [`Runtime::complete`].
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], [`Refused::UnknownTask`], and the attribution refusals above.
    pub fn raise(&mut self, task: TaskId, fault: Fault) -> Result<FaultEffect, Refused> {
        if self.halted {
            return Err(Refused::Halted);
        }
        let index = self.index_of(task)?;

        match fault {
            Fault::Overrun => {
                if !matches!(self.state[index], TaskState::Ready | TaskState::Running) {
                    return Err(Refused::NoJobOwed);
                }
            }
            Fault::UnexpectedTrap | Fault::StackGuard | Fault::AssertionFailure => {
                match self.running {
                    None => return Err(Refused::NoTaskRunning),
                    Some(running) if running != task => {
                        return Err(Refused::NotTheRunningTask);
                    }
                    Some(_) => {}
                }
            }
        }

        Ok(self.apply_and_reschedule(task, fault))
    }

    // -- invariants ----------------------------------------------------------------------------

    /// Check the structural invariants of the fixed-priority ready structure.
    ///
    /// These are the properties §7.1 calls a "model proof" candidate — "A state-machine invariant
    /// is machine checked" — and they are the ones an adapter should assert after *every* event on
    /// both this model and the implementation it drives. A model that agrees event-by-event on
    /// which task runs but silently loses the invariant in between is agreeing by luck.
    ///
    /// A halted runtime is exempt, and deliberately so. Once the fatal handler has run this is no
    /// longer a schedule but the frozen evidence §8.1 asks to be preserved, and evidence of a fault
    /// caught partway through delivering latched releases can legitimately show a ready task
    /// outranking the task the processor stopped on. Checking it would be asserting a scheduling
    /// property of something that is no longer scheduling.
    ///
    /// # Errors
    ///
    /// The first [`Violation`] found.
    pub fn check_invariants(&self) -> Result<(), Violation> {
        if self.halted {
            return Ok(());
        }
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
        for (i, &latch) in self.latch.iter().enumerate() {
            if latch == Latch::Empty {
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

    /// Decide the policy §3.1.1 requires for `fault` on `task`, record the fault, and move the
    /// task's lifecycle state.
    ///
    /// It deliberately does **not** touch the processor. Who runs next is the scheduler's
    /// business, and separating the two is what lets [`Runtime::unmask`] apply a policy to the
    /// running task partway through a delivery and still report one transition for the whole of
    /// it.
    fn apply_fault(&mut self, task: TaskId, fault: Fault) -> Applied {
        let index = task.index();
        let masked = self.is_masked();

        let declared = match fault {
            // §3.1.1: the overrun row is the only one whose "Containable" column says yes, and
            // §7.3 is where the response is declared, per task.
            Fault::Overrun => self.spec[index].on_overrun,
            // The other three rows say "Containable: No".
            Fault::UnexpectedTrap | Fault::StackGuard | Fault::AssertionFailure => {
                OverrunAction::Fatal
            }
        };
        // §3.1.1 rule 3: a containable fault raised while interrupts are masked is not containable.
        let action = if masked && !matches!(declared, OverrunAction::Fatal) {
            OverrunAction::Fatal
        } else {
            declared
        };

        let fatal = matches!(action, OverrunAction::Fatal);
        self.record_fault(FaultRecord {
            task,
            fault,
            fatal,
            masked,
        });

        match action {
            OverrunAction::Fatal => {
                // Freeze everything. The task states, the mask depth and the running task are
                // diagnostic evidence now (§8.1), and a fatal handler that tidied up would be
                // destroying the record it exists to preserve.
                self.halted = true;
                Applied::Fatal
            }
            OverrunAction::TerminateJob => {
                self.state[index] = TaskState::Completed;
                Applied::JobTerminated
            }
            OverrunAction::StopTask => {
                self.state[index] = TaskState::Stopped;
                Applied::TaskStopped
            }
        }
    }

    /// An overrun detected by a **release** — at arrival in [`Runtime::release`], or at delivery.
    ///
    /// [`Runtime::apply_fault`], plus the one thing a release-triggered overrun adds: §3.1.1 rule
    /// 1, "The release that triggers an overrun is the policy's: under `SkipLateJob` it becomes the
    /// task's next job, and under `Fault` it goes with the faulted task." Under `TerminateJob`
    /// (the contract's `SkipLateJob`) the task is therefore ready again with that release as its
    /// job; under the other two the release is consumed with the task, which `apply_fault` has
    /// already done.
    fn overrun_detected_by_release(&mut self, task: TaskId) -> Applied {
        let applied = self.apply_fault(task, Fault::Overrun);
        if applied == Applied::JobTerminated {
            self.state[task.index()] = TaskState::Ready;
        }
        applied
    }

    /// [`Runtime::apply_fault`], then run the scheduler if the faulting task was the one holding
    /// the processor.
    ///
    /// If it was not — §3.1.1 rule 2's case — nothing moves and the effect carries no transition.
    fn apply_and_reschedule(&mut self, task: TaskId, fault: Fault) -> FaultEffect {
        let was_running = self.running == Some(task);
        let applied = self.apply_fault(task, fault);
        self.reschedule_after(task, was_running, applied)
    }

    /// Turn an applied policy into its [`FaultEffect`], running the scheduler only if `task` held
    /// the processor before the policy applied.
    ///
    /// A task skipped under `SkipLateJob` by a release is [`TaskState::Ready`] again by now, so
    /// when it held the processor the scheduler hands the processor back to it: nothing ready can
    /// outrank a task that was running with releases being delivered, and the handover from the
    /// late job to the fresh one is a [`Transition::Switch`] from the task to itself.
    fn reschedule_after(
        &mut self,
        task: TaskId,
        was_running: bool,
        applied: Applied,
    ) -> FaultEffect {
        match applied {
            Applied::Fatal => FaultEffect::Fatal,
            Applied::JobTerminated => {
                FaultEffect::JobTerminated(was_running.then(|| self.vacate(task)))
            }
            Applied::TaskStopped => {
                FaultEffect::TaskStopped(was_running.then(|| self.vacate(task)))
            }
        }
    }

    /// Deliver every latched arrival, then run the scheduler once — the closing half of
    /// [`Runtime::unmask`] at depth `1 → 0`, and of [`Runtime::complete`] by §3.1.1 rule 4.
    ///
    /// The caller has already brought the depth to zero, so everything here is outside every
    /// masked region and [`Runtime::apply_fault`] does not escalate (§3.1.1 rule 3's note).
    /// `stood_down` is a task that gave up the processor just before — the completing one — so
    /// the one transition still names it as outgoing.
    ///
    /// Tasks are taken in ascending rank, and each task's arrivals in the order they arrived: the
    /// release its latch holds, then the overrun it kept beside it. Each arrival is judged against
    /// the task's state at that moment, exactly as [`Runtime::release`] would judge it unmasked —
    /// the held release made the task owe a job, so the kept one is the overrun rule 1 says is
    /// "detected" here.
    fn deliver_latched(&mut self, mut stood_down: Option<TaskId>) -> Unmasked {
        debug_assert!(
            !self.is_masked(),
            "delivery happens outside every masked region"
        );
        let mut delivered = 0;
        let mut overruns = 0;
        let mut stopped = 0;
        while let Some(task) = self.next_latched_by_rank() {
            let index = task.index();
            // Take one arrival off the latch. A latch that kept an overrun still holds that
            // arrival afterwards, so the same task comes round again before any lower rank, and a
            // fatal policy below leaves exactly the undelivered arrivals visible as evidence.
            self.latch[index] = match self.latch[index] {
                Latch::ReleaseAndOverrun => Latch::Release,
                Latch::Release | Latch::Empty => Latch::Empty,
            };
            match self.state[index] {
                TaskState::Stopped => stopped += 1,
                TaskState::Ready | TaskState::Running => {
                    overruns += 1;
                    let was_running = self.running == Some(task);
                    // §3.1.1 rule 1, with the release the policy's. The task stands down rather
                    // than being rescheduled now, so that the scheduler runs once for the whole
                    // delivery and its one transition still names the task that stood down.
                    match self.overrun_detected_by_release(task) {
                        Applied::Fatal => {
                            return Unmasked {
                                depth: 0,
                                delivered,
                                overruns,
                                stopped,
                                transition: None,
                            };
                        }
                        Applied::JobTerminated | Applied::TaskStopped => {
                            if was_running {
                                self.running = None;
                                stood_down = Some(task);
                            }
                        }
                    }
                }
                TaskState::Created | TaskState::Completed => {
                    self.state[index] = TaskState::Ready;
                    delivered += 1;
                }
            }
        }
        let transition = match self.take_processor() {
            Some((preempted, incoming)) => Some(match preempted.or(stood_down) {
                Some(outgoing) => Transition::Switch { outgoing, incoming },
                None => Transition::Dispatch { incoming },
            }),
            None => stood_down.map(|outgoing| Transition::ToIdle { outgoing }),
        };
        Unmasked {
            depth: 0,
            delivered,
            overruns,
            stopped,
            transition,
        }
    }
}

/// Which policy [`Runtime::apply_fault`] applied, before the scheduler has had its say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Applied {
    Fatal,
    JobTerminated,
    TaskStopped,
}

/// What one task's interrupt latch holds while interrupts are masked.
///
/// Three states and no more, because §3.1.1 rule 1 (amended 2026-10-01) gives the latch exactly
/// two things to keep: "the latch keeps the overrun beside the release it holds". A queue would be
/// the `general-ipc` §3.1 excludes; a counter of arrivals would keep nothing the policies can use,
/// because under each of them the state after delivering two arrivals is the state after
/// delivering more — `SkipLateJob` leaves the task ready with one fresh job however many it
/// skipped, and a stopped task or a halted runtime takes no further release. So a third arrival is
/// the overrun already kept, and changes nothing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Latch {
    /// Nothing pending.
    Empty,
    /// One release, judged at delivery against the task's state then.
    Release,
    /// One release, and the overrun of a later arrival kept beside it.
    ReleaseAndOverrun,
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
            Self::NotTheRunningTask => write!(
                f,
                "this fault is synchronous to the executing context, so §3.1.1 attributes it to \
                 the task holding the processor; the named task is not that task"
            ),
            Self::NoJobOwed => write!(
                f,
                "the named task owes no job, so it cannot be overrunning one; §3.1.1 defines an \
                 overrun as a fault of a task that still owes a job"
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
            rt.complete().unwrap().transition,
            Some(Transition::Switch {
                outgoing: H,
                incoming: L,
            })
        );
        assert_eq!(rt.state(H), Some(TaskState::Completed));
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_sound(&rt);
    }

    #[test]
    fn completion_with_nothing_ready_goes_idle() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(
            rt.complete().unwrap(),
            Unmasked {
                depth: 0,
                delivered: 0,
                overruns: 0,
                stopped: 0,
                transition: Some(Transition::ToIdle { outgoing: L }),
            },
            "outside a masked region a completion delivers nothing and reports its own transition"
        );
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
    fn a_second_release_of_a_running_task_applies_its_overrun_policy_at_once() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        // §3.1.1 rule 1: the observation *is* the fault, and L's declared TerminateJob (the
        // contract's SkipLateJob) applies here rather than waiting for a separate call. The late
        // job is skipped and the triggering release "becomes the task's next job", so L hands the
        // processor from its late job to its fresh one: one switch, from L to L.
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::JobTerminated(Some(Transition::Switch {
                outgoing: L,
                incoming: L,
            })))
        );
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_eq!(rt.processor(), Processor::Running(L));
        let record = rt.fault_record().expect("the overrun was recorded");
        assert_eq!(record.task, L);
        assert_eq!(record.fault, Fault::Overrun);
        assert!(!record.fatal);
        assert!(!record.masked);
        assert_sound(&rt);
        // There is exactly one job — the fresh one — so one completion leaves L at rest.
        assert_eq!(
            rt.complete().unwrap().transition,
            Some(Transition::ToIdle { outgoing: L })
        );
        assert_eq!(rt.complete().unwrap_err(), Refused::NoTaskRunning);
    }

    #[test]
    fn under_stop_or_fatal_the_triggering_release_goes_with_the_faulted_task() {
        // §3.1.1 rule 1: "under `Fault` it goes with the faulted task". Both of this model's
        // faulting policies consume the release: nothing is left ready and nothing runs.
        for (policy, expected) in [
            (
                OverrunAction::StopTask,
                FaultEffect::TaskStopped(Some(Transition::ToIdle { outgoing: L })),
            ),
            (OverrunAction::Fatal, FaultEffect::Fatal),
        ] {
            let mut rt = Runtime::boot([
                TaskSpec {
                    name: "H",
                    priority: Priority::HIGHEST,
                    on_overrun: OverrunAction::TerminateJob,
                },
                TaskSpec {
                    name: "L",
                    priority: Priority::new(2),
                    on_overrun: policy,
                },
            ])
            .unwrap();
            rt.release(L).unwrap();
            assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Overrun(expected));
            assert_ne!(rt.state(L), Some(TaskState::Ready));
            assert_ne!(rt.processor(), Processor::Running(L));
        }
    }

    #[test]
    fn an_overrun_is_attributed_to_the_overrunning_task_not_the_running_one() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        // §3.1.1 rule 2: "the overrunning task is by construction not the running one". Containing
        // L's fault moves no context, because L was not holding any.
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::JobTerminated(None))
        );
        // Rule 1: L's late job is skipped and this release is its next job, so L is still ready —
        // with a fresh job now, still waiting behind H.
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.fault_record().expect("recorded").task, L);
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_policy_of_stop_removes_a_ready_task_without_a_context_switch() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::StopTask,
            },
        ])
        .unwrap();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::TaskStopped(None))
        );
        assert_eq!(rt.state(L), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_policy_of_fatal_halts_on_the_release_that_detected_it() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::Fatal,
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
            rt.release(H).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::Fatal)
        );
        assert_eq!(rt.processor(), Processor::Halted);
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
    fn a_latched_release_for_a_task_that_still_owes_a_job_applies_its_policy_at_delivery() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        // L's latch slot was empty, so this arrival is only inspected when it is delivered.
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Latched);
        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.delivered, 0);
        assert_eq!(lifted.overruns, 1);
        // §3.1.1 rule 3: "Delivery at unmask is not inside a masked region", so L's declared
        // TerminateJob applies rather than escalating. L held the processor, its late job is
        // skipped, and the delivered release is its next job (rule 1), so the one transition of
        // this unmask hands the processor from L's late job to L's fresh one.
        assert_eq!(
            lifted.transition,
            Some(Transition::Switch {
                outgoing: L,
                incoming: L,
            })
        );
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_eq!(rt.processor(), Processor::Running(L));
        let record = rt.fault_record().expect("recorded");
        assert!(!record.masked, "delivery is outside the region");
        assert!(!record.fatal, "so the containable policy stays containable");
        assert_sound(&rt);
    }

    // -- a doubled release inside one masked region (§3.1.1 rule 1, amended 2026-10-01) --------

    /// `L` runs, and `H` — never released before — arrives twice inside one masked region that
    /// `L` holds. `H` declares `policy`.
    fn h_doubled_inside_a_region_held_by_l(policy: OverrunAction) -> Runtime<2> {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: policy,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        assert_eq!(rt.is_overrun_latched(H), Some(false));
        // The latch is already full. Rule 1: observed at delivery, not at arrival — so this
        // arrival is latched like the first, and nothing else happens.
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        assert_eq!(rt.is_latched(H), Some(true));
        assert_eq!(rt.is_overrun_latched(H), Some(true));
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(
            rt.fault_record(),
            None,
            "no policy has run yet, so no fault yet"
        );
        assert!(rt.is_masked());
        assert_sound(&rt);
        rt
    }

    #[test]
    fn a_doubled_release_under_skip_late_job_is_contained_at_delivery() {
        let mut rt = h_doubled_inside_a_region_held_by_l(OverrunAction::TerminateJob);
        let lifted = rt.unmask().unwrap();
        // The held release is delivered (H owed nothing, so it becomes ready), then the kept
        // overrun is detected against the job that release started, and H's SkipLateJob skips it:
        // the kept release becomes H's next job. One scheduler run, one transition.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 1,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: L,
                    incoming: H,
                }),
            }
        );
        assert_eq!(rt.state(H), Some(TaskState::Running));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.is_latched(H), Some(false));
        assert_eq!(rt.is_overrun_latched(H), Some(false));
        assert_eq!(
            rt.fault_record(),
            Some(FaultRecord {
                task: H,
                fault: Fault::Overrun,
                fatal: false,
                masked: false,
            }),
            "never fatal for landing inside a critical section"
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_doubled_release_under_stop_task_stops_the_task_at_delivery() {
        let mut rt = h_doubled_inside_a_region_held_by_l(OverrunAction::StopTask);
        let lifted = rt.unmask().unwrap();
        // H is delivered, then faulted by its own policy; the kept release goes with it, so L
        // never loses the processor and the unmask moves no context.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 1,
                stopped: 0,
                transition: None,
            }
        );
        assert_eq!(rt.state(H), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Running(L));
        let record = rt.fault_record().expect("recorded");
        assert_eq!(
            (record.task, record.fatal, record.masked),
            (H, false, false)
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_doubled_release_under_fatal_halts_at_delivery_not_at_arrival() {
        let mut rt = h_doubled_inside_a_region_held_by_l(OverrunAction::Fatal);
        let lifted = rt.unmask().unwrap();
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 1,
                stopped: 0,
                transition: None,
            }
        );
        assert_eq!(rt.processor(), Processor::Halted);
        // Fatal because H declared Fatal, not because the arrival landed inside a region: the
        // record says the fault was raised outside every masked region.
        let record = rt.fault_record().expect("recorded");
        assert_eq!((record.task, record.fatal, record.masked), (H, true, false));
    }

    /// The ruling's own test: "the outcome may not depend on which side of an unmask an
    /// interrupt lands". The same two arrivals are replayed just inside the region and just after
    /// it, for the running task and for a task that has not run, under every policy, and the
    /// resulting task states, processor and fault record must agree. Only the number of context
    /// transitions may differ — one per delivery, by [`Unmasked::transition`]'s rule.
    ///
    /// That one difference shows through in a single place: a halt partway through a delivery
    /// freezes the state *before* the delivery's one scheduler run, so evidence that would show a
    /// task dispatched after the region can show it merely ready inside it (see
    /// [`Runtime::check_invariants`] on why halted evidence is exempt). For a halted runtime the
    /// comparison therefore asks which tasks owe a job, not which one the frozen processor shows.
    #[test]
    fn a_doubled_release_ends_the_same_inside_a_region_as_one_instruction_after_it() {
        fn boot(policy: OverrunAction) -> Runtime<2> {
            Runtime::boot([
                TaskSpec {
                    name: "H",
                    priority: Priority::HIGHEST,
                    on_overrun: policy,
                },
                TaskSpec {
                    name: "L",
                    priority: Priority::new(2),
                    on_overrun: policy,
                },
            ])
            .unwrap()
        }
        fn observe(rt: &Runtime<2>) -> (Processor, [Option<TaskState>; 2], Option<FaultRecord>) {
            let halted = rt.processor() == Processor::Halted;
            let state = |task| match rt.state(task) {
                Some(TaskState::Running) if halted => Some(TaskState::Ready),
                other => other,
            };
            (rt.processor(), [state(H), state(L)], rt.fault_record())
        }
        for policy in [
            OverrunAction::TerminateJob,
            OverrunAction::StopTask,
            OverrunAction::Fatal,
        ] {
            for doubled in [H, L] {
                let mut inside = boot(policy);
                inside.release(L).unwrap();
                inside.mask().unwrap();
                inside.release(doubled).unwrap();
                inside.release(doubled).unwrap();
                inside.unmask().unwrap();

                let mut after = boot(policy);
                after.release(L).unwrap();
                after.mask().unwrap();
                after.unmask().unwrap();
                after.release(doubled).unwrap();
                // A fatal first arrival leaves nothing to release into.
                if after.processor() != Processor::Halted {
                    after.release(doubled).unwrap();
                }

                assert_eq!(
                    observe(&inside),
                    observe(&after),
                    "policy {policy:?}, doubled task {}",
                    doubled.index()
                );
                inside.check_invariants().unwrap();
            }
        }
    }

    #[test]
    fn a_third_release_into_a_full_latch_is_the_overrun_already_kept() {
        let mut rt = two_tasks();
        rt.mask().unwrap();
        for _ in 0..3 {
            assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        }
        assert_eq!(rt.is_overrun_latched(H), Some(true));
        let lifted = rt.unmask().unwrap();
        // One release delivered and one overrun detected: the latch keeps "the overrun", and a
        // third arrival would have left H in this same state under every policy.
        assert_eq!((lifted.delivered, lifted.overruns), (1, 1));
        assert_eq!(rt.state(H), Some(TaskState::Running));
        assert_sound(&rt);
    }

    #[test]
    fn a_doubled_release_of_the_running_task_is_two_overruns_at_delivery() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.release(L).unwrap();
        rt.release(L).unwrap();
        let lifted = rt.unmask().unwrap();
        // L already owed its running job, so the held release is an overrun at delivery and the
        // kept one is another — as the same two arrivals would be after the unmask. Each skips a
        // job and becomes L's next, so L ends with one fresh job and the processor.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 0,
                overruns: 2,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: L,
                    incoming: L,
                }),
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_sound(&rt);
    }

    #[test]
    fn delivery_takes_tasks_in_rank_order_and_each_tasks_arrivals_in_arrival_order() {
        // Three tasks so that a fatal policy in the middle rank shows exactly how far delivery
        // got: everything ranked above it, nothing ranked below it.
        const A: TaskId = TaskId::from_index(0);
        const B: TaskId = TaskId::from_index(1);
        const C: TaskId = TaskId::from_index(2);
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "A",
                priority: Priority::new(1),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "B",
                priority: Priority::new(2),
                on_overrun: OverrunAction::Fatal,
            },
            TaskSpec {
                name: "C",
                priority: Priority::new(3),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.mask().unwrap();
        // Arrival order deliberately the reverse of rank.
        rt.release(C).unwrap();
        rt.release(B).unwrap();
        rt.release(B).unwrap();
        rt.release(A).unwrap();
        let lifted = rt.unmask().unwrap();
        // A first; then B's held release (B owed nothing, so it is delivered), then B's kept
        // overrun, which B's Fatal policy turns into a halt. C is never reached.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 2,
                overruns: 1,
                stopped: 0,
                transition: None,
            }
        );
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(rt.state(A), Some(TaskState::Ready));
        assert_eq!(rt.state(B), Some(TaskState::Ready));
        assert_eq!(rt.state(C), Some(TaskState::Created));
        assert_eq!(
            rt.is_latched(C),
            Some(true),
            "the undelivered release is evidence"
        );
        assert_eq!(rt.is_latched(B), Some(false));
        let record = rt.fault_record().expect("recorded");
        assert_eq!((record.task, record.fatal, record.masked), (B, true, false));
    }

    #[test]
    fn an_overrun_found_at_delivery_still_costs_one_transition_for_the_whole_unmask() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.release(L).unwrap(); // latched, and L still owes its job
        rt.release(H).unwrap(); // latched, and a genuine new release
        let lifted = rt.unmask().unwrap();
        assert_eq!(lifted.delivered, 1);
        assert_eq!(lifted.overruns, 1);
        // L's late job is skipped and H takes the processor, but the scheduler still runs exactly
        // once: one Switch naming L as outgoing, not a ToIdle followed by a Dispatch.
        assert_eq!(
            lifted.transition,
            Some(Transition::Switch {
                outgoing: L,
                incoming: H,
            })
        );
        // Rule 1: the release that found L late is L's next job, now waiting behind H.
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_sound(&rt);
    }

    #[test]
    fn a_fatal_overrun_policy_found_at_delivery_stops_the_delivery() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::Fatal,
            },
        ])
        .unwrap();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.release(H).unwrap();
        rt.release(L).unwrap();
        let lifted = rt.unmask().unwrap();
        // Deliveries run in ascending rank, so H lands before L's overrun is reached.
        assert_eq!(lifted.delivered, 1);
        assert_eq!(lifted.overruns, 1);
        assert_eq!(lifted.transition, None);
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(rt.fault_record().expect("recorded").task, L);
    }

    // -- a completion inside a masked region (§3.1.1 rule 4, added 2026-10-01) ------------------

    #[test]
    fn masking_does_not_defer_a_completion_and_the_completion_closes_the_region() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        // A completion is the running task reaching the end of its own computation, not an
        // arrival, so F29's latching rule does not apply to it. Rule 4: "its completion closes
        // it" — the depth returns to zero with the job.
        assert_eq!(
            rt.complete().unwrap(),
            Unmasked {
                depth: 0,
                delivered: 0,
                overruns: 0,
                stopped: 0,
                transition: Some(Transition::ToIdle { outgoing: L }),
            }
        );
        assert_eq!(rt.processor(), Processor::Idle);
        assert!(!rt.is_masked());
        // The region is closed, so the unmask that would have closed it is now unbalanced.
        assert_eq!(rt.unmask().unwrap_err(), Refused::NotMasked);
        assert_eq!(rt.fault_record(), None, "a completion is not a fault");
        assert_sound(&rt);
    }

    #[test]
    fn a_completion_closes_every_nested_section_not_one_level() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.mask().unwrap();
        rt.mask().unwrap();
        rt.release(H).unwrap();
        let done = rt.complete().unwrap();
        assert_eq!(done.depth, 0);
        assert_eq!(rt.mask_depth(), 0);
        // Delivery happened at once, as at the outermost unmask, not at some later inner one.
        assert_eq!(done.delivered, 1);
        assert_eq!(
            done.transition,
            Some(Transition::Switch {
                outgoing: L,
                incoming: H,
            })
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_completion_inside_a_region_delivers_another_tasks_latched_release_then_schedules() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        let done = rt.complete().unwrap();
        // L's job ends, the region closes, H is delivered, and the schedule is decided after:
        // one transition, from the completing task to the delivered one.
        assert_eq!(
            done,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 0,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: L,
                    incoming: H,
                }),
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Completed));
        assert_eq!(rt.state(H), Some(TaskState::Running));
        assert_eq!(rt.is_latched(H), Some(false));
        assert_sound(&rt);
    }

    #[test]
    fn a_completion_inside_a_region_releases_its_own_latched_release_afresh() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        // At arrival L still owes the job it is running; that is not judged now.
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Latched);
        let done = rt.complete().unwrap();
        // Rule 4: "A latched release is judged at that delivery, so a task that completed inside
        // the region is released afresh, not overrun." L's next job takes the processor from its
        // finished one.
        assert_eq!(
            done,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 0,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: L,
                    incoming: L,
                }),
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_eq!(rt.fault_record(), None, "released afresh, not overrun");
        assert_sound(&rt);
    }

    #[test]
    fn a_completion_inside_a_region_with_its_own_release_doubled_overruns_once() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.release(L).unwrap();
        rt.release(L).unwrap();
        assert_eq!(rt.is_overrun_latched(L), Some(true));
        let done = rt.complete().unwrap();
        // The held release finds L completed and starts its next job; the kept overrun is then
        // detected against that job, and L's SkipLateJob makes the kept release the job after.
        assert_eq!(
            done,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 1,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: L,
                    incoming: L,
                }),
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Running));
        let record = rt.fault_record().expect("recorded");
        assert_eq!(
            (record.task, record.fatal, record.masked),
            (L, false, false)
        );
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_found_by_a_completions_delivery_applies_its_policy_unescalated() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::StopTask,
            },
        ])
        .unwrap();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.mask().unwrap();
        // L is preempted and still owes its job; this arrival is judged only at delivery.
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Latched);
        let done = rt.complete().unwrap();
        // H completes, the region closes, L's latched release finds L owing a job: an overrun,
        // outside every masked region, so L's own StopTask applies rather than rule 3's fatal.
        assert_eq!(
            done,
            Unmasked {
                depth: 0,
                delivered: 0,
                overruns: 1,
                stopped: 0,
                transition: Some(Transition::ToIdle { outgoing: H }),
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Idle);
        let record = rt.fault_record().expect("recorded");
        assert_eq!(
            (record.task, record.fatal, record.masked),
            (L, false, false)
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_fatal_policy_reached_by_a_completions_delivery_halts_before_any_schedule() {
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: OverrunAction::Fatal,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.release(L).unwrap();
        rt.mask().unwrap();
        rt.release(H).unwrap();
        rt.release(H).unwrap();
        let done = rt.complete().unwrap();
        // The completion happened — L's job is over — but the delivery reached H's Fatal policy,
        // so no schedule was decided and there is no transition to report.
        assert_eq!(
            done,
            Unmasked {
                depth: 0,
                delivered: 1,
                overruns: 1,
                stopped: 0,
                transition: None,
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Completed));
        assert_eq!(rt.processor(), Processor::Halted);
        let record = rt.fault_record().expect("recorded");
        assert_eq!((record.task, record.fatal, record.masked), (H, true, false));
        assert_eq!(rt.complete().unwrap_err(), Refused::Halted);
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
        assert_eq!(
            rt.raise(L, Fault::UnexpectedTrap).unwrap(),
            FaultEffect::Fatal
        );
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
            rt.raise(L, Fault::AssertionFailure).unwrap_err(),
            Refused::Halted
        );
    }

    #[test]
    fn a_stack_guard_and_an_assertion_failure_are_fatal_too() {
        for fault in [Fault::StackGuard, Fault::AssertionFailure] {
            let mut rt = two_tasks();
            rt.release(L).unwrap();
            assert_eq!(rt.raise(L, fault).unwrap(), FaultEffect::Fatal);
            assert_eq!(rt.processor(), Processor::Halted);
        }
    }

    #[test]
    fn the_fatal_handler_preserves_the_state_that_explains_it() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.raise(H, Fault::UnexpectedTrap).unwrap();
        // H was running and L was preempted; both readings survive the halt, because a fatal
        // handler that tidied up would be destroying its own evidence.
        assert_eq!(rt.state(H), Some(TaskState::Running));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
    }

    #[test]
    fn the_first_fault_is_the_one_kept() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.raise(L, Fault::Overrun).unwrap();
        rt.release(H).unwrap();
        rt.raise(H, Fault::UnexpectedTrap).unwrap();
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
            rt.raise(H, Fault::Overrun).unwrap(),
            FaultEffect::JobTerminated(Some(Transition::Switch {
                outgoing: H,
                incoming: L,
            }))
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
            rt.raise(H, Fault::Overrun).unwrap(),
            FaultEffect::TaskStopped(Some(Transition::ToIdle { outgoing: H }))
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
        rt.raise(H, Fault::Overrun).unwrap();
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
        // §3.1.1 rule 3, first ground: TerminateJob would leave the mask depth above zero with no
        // owner, and forcing it to zero would re-enable interrupts mid-region.
        assert_eq!(rt.raise(L, Fault::Overrun).unwrap(), FaultEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        let record = rt.fault_record().expect("a fault was recorded");
        assert!(record.masked);
        assert!(record.fatal);
    }

    #[test]
    fn a_containable_fault_of_a_task_that_holds_no_mask_escalates_too() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.mask().unwrap();
        // L holds no mask, so rule 3's first ground does not reach it. Its second does:
        // containment "means changing the schedule, which is the structure a kernel critical
        // section exists to protect … whichever task the fault is attributed to".
        assert_eq!(rt.raise(L, Fault::Overrun).unwrap(), FaultEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        let record = rt.fault_record().expect("a fault was recorded");
        assert_eq!(record.task, L);
        assert!(record.masked);
        assert!(record.fatal);
    }

    #[test]
    fn a_synchronous_fault_must_name_the_task_holding_the_processor() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        // §3.1.1 rule 2: a trap, a stack-guard breach and an assertion failure are "synchronous to
        // the executing context and are attributed to it", so naming another task is the
        // concurrency claim the profile does not admit.
        assert_eq!(
            rt.raise(H, Fault::UnexpectedTrap).unwrap_err(),
            Refused::NotTheRunningTask
        );
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.fault_record(), None, "a refusal is not a fault");
    }

    #[test]
    fn a_synchronous_fault_with_an_idle_processor_has_no_context_to_attribute_it_to() {
        let mut rt = two_tasks();
        for fault in [
            Fault::UnexpectedTrap,
            Fault::StackGuard,
            Fault::AssertionFailure,
        ] {
            assert_eq!(rt.raise(L, fault).unwrap_err(), Refused::NoTaskRunning);
        }
    }

    #[test]
    fn an_overrun_needs_a_task_that_still_owes_a_job() {
        let mut rt = two_tasks();
        // Created: never released, so there is no job for it to be late with.
        assert_eq!(rt.raise(L, Fault::Overrun).unwrap_err(), Refused::NoJobOwed);
        rt.release(L).unwrap();
        rt.complete().unwrap();
        // Completed: its job is over, so it is not overrunning anything either.
        assert_eq!(rt.raise(L, Fault::Overrun).unwrap_err(), Refused::NoJobOwed);
    }

    #[test]
    fn an_overrun_can_be_raised_against_a_ready_task_and_moves_no_context() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        // §3.1.1 rule 2 again, by the explicit entry point this time rather than by detection.
        assert_eq!(
            rt.raise(L, Fault::Overrun).unwrap(),
            FaultEffect::JobTerminated(None)
        );
        assert_eq!(rt.state(L), Some(TaskState::Completed));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.fault_record().expect("recorded").task, L);
        assert_sound(&rt);
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
            record(
                rt.complete()
                    .unwrap()
                    .transition
                    .expect("H hands back to L"),
                &mut transitions,
                &mut n,
            );
            assert_eq!(rt.processor(), Processor::Running(L));
            assert_sound(&rt);
        }

        // "[21, 23) Remaining L computation" and L's first job ends. The fixture stops here; the
        // transition away from L is outside it, and outside L's response interval too.
        record(
            rt.complete().unwrap().transition.expect("L goes idle"),
            &mut transitions,
            &mut n,
        );

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
