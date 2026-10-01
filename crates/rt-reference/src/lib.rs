//! An **independent reference model** of the `rt-static-up-v1` task-lifecycle state machine
//! (`ROADMAP.md` §8 "Runtime and architecture substrate", §3.1 the profile table, §8.1 the
//! unsafe/synchronization boundaries), and the profile's **fault contract**,
//! `docs/profiles/rt-static-up-v1-faults.md` (`ROADMAP.md` §3.1.1 until it moved there, verbatim,
//! on 2026-10-01).
//!
//! "The fault contract" below always means that file: its *Terms*, its table, its numbered
//! rules — whose numbers the move kept — its two smaller decisions and its *Still open* list. A
//! bare section number (§3.1, §7.3, §8.1, …) is `ROADMAP.md`'s, in that file as in this crate.
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
//! The fault contract — then `ROADMAP.md` §3.1.1, "Fault classification, attribution and
//! containment", amendment of 2026-09-13 — and the new "The admissible range, and the runtime's
//! index" section of `docs/decisions/decision_priority-comparison-direction.md` now decide them.
//! This model has been brought into line with **the amended text**, still without reading the
//! implementation: every change is derived from the fault contract's table and its rules, so where
//! the two models now agree they agree because both satisfy one contract, not because one was
//! adjusted to the other. Each note an amendment answers is **kept and rewritten** to cite it
//! rather than deleted: that a question was once open is part of the record, and erasing it would
//! erase the evidence that the agreement was earned.
//!
//! ⭐ **The fault contract was then read the same way, and amended in its turn.** Three of this
//! model's notes found gaps that the fault contract itself had opened, and the rulings of
//! 2026-10-01 (`docs/decisions/decision_findings-for-director-review.md` §6) closed them: rule 1
//! now observes a release that finds its task's latch already full **at delivery**, and a new rule
//! 4 lets a job complete inside a masked region, closing it. Both are carried here from the amended
//! text alone, and the three notes are rewritten below in the same `⭐ AMENDED` form as the first
//! five.
//!
//! ⭐ **Then the text itself was reviewed, and rewritten.** Also on 2026-10-01, a reader of the
//! fault contract alone returned twenty-five findings
//! (`docs/decisions/decision_runtime-contract-gaps.md`, *The amended §3.1.1 reviewed*), and the
//! fault contract was rewritten to answer them: a *Terms* paragraph, a rule 1a, rules 5 to 7, and a
//! second smaller decision that changed its answer. Most of the answers state what this model
//! already did. Four move it, each carried here from the rewritten text alone and marked
//! `⭐ AMENDED` where it lands:
//!
//! - a fault raised in a service, the trap path, a transition or idle is **no task's** (rule 2),
//!   so a synchronous fault now names the executing [`Context`] rather than a task, and its
//!   record carries an [`Attribution`] that can say "no task";
//! - an overrun policy is one of **two** (rule 5), so the halting policy this model had added
//!   beside them is gone, and an overrun reaches the fatal handler only by rule 3's escalation;
//! - the halt keeps the first fault that **entered the fatal handler** (rule 7), not the first
//!   fault of any kind — [`Runtime::fatal_record`], with [`Runtime::first_contained_overrun`]
//!   beside it for rule 5's end of a run's timing claims;
//! - a `mask` past the declared depth and an `unmask` at depth zero are **assertion failures**
//!   that halt (the second smaller decision), where this model refused them.
//!
//! That rewrite's rule 1a also narrowed what may be raised inside a masked region to the region
//! holder's own overrun, which this model enforced with a refusal (`Refused::MonitorMasked`).
//!
//! ⭐ **And the rewritten text was reviewed again, and rewritten again.** A second reader of
//! the fault contract alone returned twenty-eight findings more
//! (`decision_runtime-contract-gaps.md`, *The rewritten §3.1.1 reviewed again*), and the fault
//! contract as amended on 2026-10-01 now answers them. Four of the answers move this model, each
//! carried from the text alone and marked `⭐ AMENDED` where it lands:
//!
//! - **only a job changes the depth** (Terms): a `mask` or `unmask` executed with no job running
//!   is an assertion failure of the executing context, so the `mask` with the processor idle that
//!   this model accepted under a `⚠️ CONTRACT SILENT` note now halts ([`MaskEffect::Fatal`],
//!   [`UnmaskEffect::Fatal`]);
//! - **an overrun raised without a release is outside this profile** (rule 1a):
//!   `rt-static-up-v1` has no execution-budget monitor, so [`Runtime::raise_overrun`] is kept
//!   only as the entry a later profile would need, and rule 3 — which "decides nothing
//!   observable in this profile" — is reached through it alone;
//! - with the sentence that made a monitor "masked with the region" gone, **rule 3's second ground
//!   reaches every task again** — "ground 2 extends the rule to every task" — so through that
//!   entry another task's overrun raised inside a region escalates as the holder's does, and
//!   `Refused::MonitorMasked` is removed;
//! - **a task without an `on-overrun` clause has `Fault`** (rule 5), which
//!   [`OverrunAction::default`] now states.
//!
//! The rest of the second rewrite stated what this model already did, in words that quoted
//! differently: the latch keeps the most recent release and a mark for the earlier ones, judged as
//! one (Terms, rule 1); the order across tasks at one delivery, and whether a completion's
//! scheduling decision precedes its delivery, are the port's (rules 1 and 4); a transition
//! interrupted the task the runtime held as running (rule 2). What the text still leaves to an
//! implementer keeps, or newly carries, a `⚠️ CONTRACT SILENT` note.
//!
//! ⭐ **Then a third reader reviewed it, and it was rewritten a third time.** The review is *R3*
//! in the fault contract's amendment header, and the text as amended on 2026-10-01 answers it.
//! **No answer moves this model's behaviour.** The new sentences either state a port's
//! obligations, which a hosted model whose events are atomic cannot see — initialisation "calls no
//! masking primitive", "a port detects the rest from its own state", an external source's arrivals
//! are distinct only where "its source's catalog record states that arrivals while its request is
//! pending or claimed are counted" — or state what this model already did: under `Fault`, a later
//! latched arrival "is discarded" rather than judged (rule 1), and a runtime "that is told no
//! context detects the case it can see, a call with no job running" (Terms). So:
//!
//! - every quotation below is refreshed to the text as it now reads;
//! - two `⚠️ CONTRACT SILENT` notes are answered and rewritten `⭐ AMENDED` — a `mask` executed by
//!   a context that is no job while a job is held as running ([`Runtime::mask`]), and the context
//!   of the decision that would start a job inside a region
//!   ([`Violation::JobStartedInsideMaskedRegion`]);
//! - one statement of this crate's that rule 2 now contradicts is withdrawn: that an empty claim
//!   is raised in the service that made it, where rule 2 raises "an unexpected trap that is an
//!   interrupt" in the trap path ([`Fault::UnexpectedTrap`]) — a statement of the documentation,
//!   since the model takes the context it is given;
//! - three things the text now decides that this model cannot express are marked
//!   `⚠️ DECIDED, NOT EXPRESSED` where they fall: a service named "by its source" in the record
//!   (rules 2 and 7, [`Context::Service`]); a transition "being decided" after its outgoing job
//!   completed or was abandoned, which interrupts no task (rule 2, [`Context::Transition`], whose
//!   `⚠️ CONTRACT SILENT` note it answers); and initialisation as a context that is no task (rule
//!   2, [`Context`]).
//!
//! ⭐ **The priority record moved as well.** Its item 1 now gives a rank's width — "any such integer
//! the language's integer type holds, a 64-bit signed one" — so [`Priority`] holds an `i64`, and
//! boot refuses every rank below `1` ([`BootError::PriorityBelowOne`]), not only `0`.
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
//! # The contract's terms in this model
//!
//! The fault contract opens with a *Terms* paragraph, and each term has one home here:
//!
//! - **masked region** — "the interval in which the runtime's mask nesting depth is above zero":
//!   [`Runtime::is_masked`]. It is "opened by a job's outermost `mask`, kernel or application, and
//!   closed by the matching `unmask` or by that job's completion (rule 4)" — [`Runtime::mask`],
//!   [`Runtime::unmask`], [`Runtime::complete`]. The processor's own interrupt disable is not one:
//!   "That disable — in a service, the trap path, a transition, the completion path, idle,
//!   initialisation or the fatal handler — is **not** a masked region, and §13.4's "latched" means
//!   pending in hardware there." A release service is the one atomic [`Runtime::release`], which
//!   never changes the depth.
//! - **section** — "one `mask`…`unmask` pair inside a region. §3.1's "kernel critical section" is
//!   a region": [`Runtime::mask_depth`] counts the open sections, bounded by
//!   [`Runtime::MAX_MASK_DEPTH`].
//! - **Only a job changes the depth** — "A `mask` or `unmask` is a job's only when the job's own
//!   code, or a primitive it called, executes it. Executed by any other context — a service, the
//!   trap path, a transition, idle, initialisation or the fatal handler — it is an assertion
//!   failure of that context, raised before the depth changes, whether or not a job is preempted
//!   beneath it. A runtime that is told no context detects the case it can see, a call with no job
//!   running". This model is such a runtime: its `mask` and `unmask` take no context, and with no
//!   job running they are [`MaskEffect::Fatal`] and [`UnmaskEffect::Fatal`], the depth unchanged.
//!   "A depth above zero when a job is dispatched or resumed is likewise an assertion failure,
//!   raised by the decision that would do it — the transition's, no task's … So every section open
//!   at a completion is the completing job's." That one is unreachable here by construction: only a
//!   running job opens a region, inside one arrivals latch, a completion closes it before the
//!   scheduler runs, and every fault that can be raised inside one halts. So the model never raises
//!   it, and [`Runtime::check_invariants`] reports [`Violation::JobStartedInsideMaskedRegion`] if
//!   the construction ever breaks.
//! - **initialisation** — "running before the first enabling of interrupts with no job and with
//!   interrupts already disabled, has nothing to mask and calls no masking primitive": here it is
//!   [`Runtime::boot`], one call that touches no depth and raises no fault.
//! - **latch** — "what holds its releases that arrive while a masked region is open", and "in a
//!   hosted model, the runtime's own record: the most recent release, and a mark that an earlier
//!   one also came": [`Runtime::is_latched`] and [`Runtime::is_overrun_latched`].
//! - **completes** — "A job **completes** at its last instruction of its own code (§13.4)":
//!   [`Runtime::complete`].
//! - **owes a job** — "from its release until that job completes or is abandoned":
//!   [`TaskState::Ready`] or [`TaskState::Running`]. "A release observed while the job's completion
//!   path runs, before or after the completion is recorded, finds the task owing no job": an
//!   adapter replays it after [`Runtime::complete`].
//! - **unexpected trap** — "any trap other than the timer's interrupt, a declared source's
//!   interrupt whose claim finds a request, the runtime API's own entry where the port uses one,
//!   and a trap that is another fault's mechanism": [`Fault::UnexpectedTrap`].
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
/// ⛔ **It is not a priority either.** The priority-direction record's amendments warn that a
/// runtime which makes a task's array index its priority ends up with a highest priority of `0`
/// while the language's highest is `1`, so that a mapping — now stated as `runtime index =
/// |hp(i)|`, "which is `rank - 1` exactly when the ranks are `1, 2, …, n`" (item 3) — becomes
/// load-bearing, "which is precisely how an off-by-one survives review: both halves are
/// individually correct and nothing states the relation". This model never makes that
/// identification: [`TaskId`] and [`Priority`] are distinct types carrying unrelated numbers, and
/// declaration order has no scheduling meaning at all.
///
/// The same record adds that "a trace, a plan or a fault record names a task by its stable
/// logical ID (`ROADMAP.md` §7.3), never by its index", and the fault contract's rule 7 asks the
/// same of the fatal handler's evidence — "the attributed task's stable logical ID — its
/// elaborated eADL task name, the instance path of `docs/semantics/reference.md` §6 rule 9, which
/// `archogen check` requires unique (`schema-duplicate-name`)" — and says who supplies it when the
/// runtime holds an index instead: "A runtime that holds tasks by an internal index has the plan
/// supply the ID when the record leaves it (§7.5): an index names no task in a record."
///
/// This model is such a runtime, and the set it booted is its plan. A [`FaultRecord`] holds
/// [`TaskId`]s — the attributed task, an interrupted one, a guard's owner — while it is the
/// model's; an adapter that takes one out through [`Runtime::fatal_record`] supplies each ID from
/// that plan, [`Runtime::spec`]'s [`TaskSpec::name`], before comparing the record with anything,
/// since outside the model "an index names no task". The resolution is lossless because a record
/// lives no longer than the [`Runtime`] that booted the set; the renumbering the priority record
/// warns about happens between builds, which this model never spans.
///
/// Boot does not check that names are unique. Rule 7 puts that check on `archogen check`, and a
/// set that reaches the runtime has passed it; a duplicate here would make two records' names
/// ambiguous, never their ids.
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
/// Ranks need not be contiguous. The record's item 3: "`(priority 1)`, `(priority 5)` and
/// `(priority 9)` is a valid description — and fixed priority uses only their order". This model
/// only ever compares two ranks, so a gap has no meaning here and is accepted.
///
/// ⭐ **AMENDED — a rank's width (the priority record's item 1, leaf `M2.18`).** This note used to
/// read `⚠️ CONTRACT SILENT`: the fault contract's *Still open* list left "the checker's refusals
/// of … a rank the lowering cannot take (`M2.18`)" to a leaf, so no upper end of the range was
/// fixed, and this model held a rank in a `u16`, leaving an adapter to refuse a description whose
/// rank did not fit rather than truncate it. The record's item 1 now gives the range:
///
/// > **A rank is an integer `N ≥ 1`** — any such integer the language's integer type holds, a
/// > 64-bit signed one
///
/// So a rank is an `i64`, the language's own integer, and every rank a description can carry
/// reaches this model unchanged: those from `1` to `i64::MAX` boot, and those below `1` — which
/// the language's integer holds as well, and which `archogen check` "refuses … and any rank below
/// 1, with `priority-below-one`" — are refused at boot ([`BootError::PriorityBelowOne`]). An
/// adapter never narrows a rank, so it has no refusal of its own to make.
///
/// ⭐ **`Ord` is deliberately not derived.** A derived `Ord` on a newtype over `i64` would make
/// `a < b` mean "`a` is a *lower* priority", which is the opposite of what every reader of
/// `priority.cmp(&other)` will assume, and the mistake compiles. The only comparison this type
/// offers is [`Priority::outranks`], whose name cannot be read backwards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Priority(i64);

impl Priority {
    /// The highest rank the decision record admits.
    ///
    /// The range is closed at this end explicitly: "**A rank is an integer `N ≥ 1`**", and "Rank
    /// `0` is **not** admissible in a description" (`decision_priority-comparison-direction.md`,
    /// item 1). Its other end is the integer's own, `i64::MAX`, the lowest priority.
    pub const HIGHEST: Self = Self(1);

    /// A priority of the given rank.
    ///
    /// A rank below `1` is constructible and is refused at boot ([`BootError::PriorityBelowOne`])
    /// rather than here, so that every admissibility rule about the task set lives in one place.
    #[must_use]
    pub const fn new(rank: i64) -> Self {
        Self(rank)
    }

    /// The numeric rank, as an eADL `(priority N)` clause carries it.
    #[must_use]
    pub const fn rank(self) -> i64 {
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
    /// The task's stable logical id (§7.3) — its elaborated eADL task name, the instance path the
    /// fault contract's rule 7 names. It names the task in diagnostics, and it is what a [`TaskId`]
    /// in a [`FaultRecord`] resolves to when the record leaves the model (see [`TaskId`]).
    pub name: &'static str,
    /// Its static priority. §3.1: "Static unique task priorities".
    pub priority: Priority,
    /// What the runtime does when this task overruns.
    ///
    /// ⭐ This is **per task**, not a runtime-wide rule, because §7.3 puts it in the per-task
    /// record: the engine assembles "… allowed OS calls, and **overrun behavior**" for every
    /// task. A single global overrun policy would contradict a field the description is required
    /// to carry.
    ///
    /// It is the resolved policy: a task whose description has no `on-overrun` clause carries
    /// [`OverrunAction::default`], rule 5's `Fault`.
    pub on_overrun: OverrunAction,
}

/// What the runtime does to a task that overruns (§3.1 "Defined overrun … policy", §7.3
/// "overrun behavior"): one of the two policies of the fault contract's rule 5.
///
/// ⭐ **AMENDED — the fault contract's rule 5, "The two overrun policies".** This note used to read
/// `⚠️ CONTRACT SILENT — still, after §3.1.1`: the table settled that an overrun is containable by
/// "its declared per-task policy (§7.3)", and nothing stated the domain that policy ranges over.
/// This model chose three — abandon this job, abandon this task, or stop — as the only responses a
/// profile with no memory isolation, no restart service and no dynamic task creation can perform.
/// Rule 5 now defines two, `SkipLateJob` (eADL `skip-late-job`) and `Fault` (eADL `fault`), and
/// says of the second that "every other task continues, and the runtime does not halt"; the
/// table's overrun row reads, under *Containable*, "Yes — by its declared per-task policy (§7.3,
/// rule 5)". A third
/// policy that halts would make a declaration the thing that refuses containment, which that row
/// does not admit, so **the variant this model called `Fatal` is removed**. In this profile an
/// overrun never reaches the fatal handler: rule 3 "decides nothing observable in this profile",
/// and only the entry a later profile would need ([`Runtime::raise_overrun`], outside the profile
/// by rule 1a) reaches it there.
///
/// (The fault contract's first 2026-10-01 header said the review's answers "state what both
/// implementations already did" apart from three named corrections, and rule 5 was not among the
/// three. For this model that was not quite so: it carried the third, halting policy until this
/// change. The header now defers to the gaps record's *What changed an implementation*, which lists
/// it.)
///
/// ⭐ **AMENDED — rule 5, the eADL clause.** This note used to read `⚠️ CONTRACT SILENT — on the
/// eADL side only`: the fault contract's *Still open* list left "the eADL clause's domain and its
/// default (`M2.14`)" to a leaf, and this model gave [`TaskSpec::on_overrun`] no default. Rule 5
/// now decides both: "A task without an `on-overrun` clause has `Fault`, and `archogen check`
/// refuses a policy other than these two (`docs/semantics/model.md` §4 rule 6, leaf `M2.14`)".
/// The domain is the two variants below, and the default is
/// [`OverrunAction::StopTask`], which `Default` now returns. A [`TaskSpec`] still states its policy
/// — it is the record after that default has been applied — so the default is a name for the
/// adapter building one, not something boot fills in. Anything richer (restart the task, degrade
/// to a backup job, raise criticality) is `mixed-criticality-scheduling` or
/// `dynamic-task-creation`, both of which the profile excludes by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OverrunAction {
    /// Abandon the owed job; the task stays in the schedule. The `SkipLateJob` of the fault
    /// contract's rule 5 (eADL `skip-late-job`):
    ///
    /// > the task's owed job is abandoned — not started, preempted, or the job the release
    /// > interrupted. Its remaining code never runs and no completion is recorded for it, no fault
    /// > halts anything, and the triggering release, if there is one, makes the task ready for its
    /// > next job (rule 1).
    ///
    /// The triggering release is the next job — rule 1: "under `SkipLateJob` it makes the task
    /// ready for its next job, with that release's nominal instant" — so the task is ready again at
    /// once (see [`ReleaseEffect::Overrun`]). This model has no time, so the nominal instant is an
    /// adapter's to check; what the model fixes is that the surviving job is the one the latch held,
    /// the most recent arrival, judged after the earlier ones its mark stands for. (The entry
    /// outside the profile, [`Runtime::raise_overrun`], has no triggering release; see there.)
    ///
    /// It is sound only because §3.1 admits "no self-suspension within a job" and no application
    /// mutexes: an abandoned job cannot be holding anything another task is waiting for. Rule 5
    /// states the limit of the claim — containment "keeps the runtime's state consistent and the
    /// schedule running, and claims nothing about the application state the abandoned job left
    /// (§3.1, Isolation)" — and where the abandonment falls:
    ///
    /// > A job is abandoned only where it holds no masked region: in its own code with the depth
    /// > at zero, or at the entry or return of a primitive that leaves the depth at zero. A policy
    /// > applied while the job is inside a primitive takes effect at the primitive's entry if the
    /// > primitive has not yet changed the runtime's state — for `mask`, before the depth is raised
    /// > — and otherwise at its return, which a policy never reaches while the primitive holds the
    /// > depth raised, since no release is observed inside a region (rule 1).
    ///
    /// This model's events are atomic, so the places it can find an overrun are exactly those: an
    /// unmasked release, where the owed job is in its own code at depth zero or preempted there,
    /// and a delivery, where the depth has already reached zero and the job holding the processor
    /// is at the return of its outermost `unmask` or of its completion. Where inside a primitive a
    /// port defers the policy is the port's obligation, not something this model can express.
    TerminateJob,
    /// Remove the task from the schedule for good. The `Fault` of the fault contract's rule 5 (eADL
    /// `fault`), and the policy of a task whose description has no `on-overrun` clause (the
    /// `Default`):
    ///
    /// > the task is faulted and leaves the schedule until reset. Its owed job is abandoned, the
    /// > triggering release and every later release of it are discarded, and no further
    /// > instruction of its own code runs; every other task continues, and the runtime does not
    /// > halt.
    ///
    /// "Until reset" is for good within a run: §3.1 excludes `dynamic-task-creation` and this
    /// model has no reset, so nothing puts the task back. The task set that remains is *not* the
    /// analyzed workload, which is why [`TaskState::Stopped`] is observable — a timing claim must
    /// not be quoted for a system running a strict subset of the tasks it was established for.
    /// Every later release reports [`ReleaseEffect::Stopped`], and a latched one is counted in
    /// [`Unmasked::stopped`].
    #[default]
    StopTask,
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
///
/// The fault contract's Terms divide them in two: a task "owes a job from its release until that
/// job completes or is abandoned", which is exactly [`TaskState::Ready`] and
/// [`TaskState::Running`].
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
    /// It owes no job and awaits its next release: its most recent job completed. **Not
    /// terminal** — the profile's workload is "periodic or sporadic releases", so this is the
    /// resting state of a healthy periodic task, reached once per period.
    ///
    /// Outside the profile it is also where [`Runtime::raise_overrun`] leaves a task whose job it
    /// abandoned under [`OverrunAction::TerminateJob`] with no release to start the next one. That
    /// job reaches this state without a completion being recorded, as rule 5 requires ("no
    /// completion is recorded for it"): the event reports [`FaultEffect::JobTerminated`], never the
    /// [`Unmasked`] of [`Runtime::complete`]. One state serves both because nothing asked of the
    /// lifecycle afterwards tells them apart: either way the task owes no job and its next release
    /// starts a fresh one.
    Completed,
    /// A fault policy removed it from the schedule ([`OverrunAction::StopTask`], rule 5's
    /// `Fault`). Terminal.
    Stopped,
}

/// What the single processor is doing. §3.1: "One active core; one execution context runs at a
/// time".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Processor {
    /// No task is ready. F29 (§13.4) names this state directly: "a following switch to another
    /// task **or idle**".
    Idle,
    /// The named task holds the processor.
    Running(TaskId),
    /// A fault entered the fatal handler and the runtime has stopped for good. The fault contract's
    /// rule 7: "From then no job runs, no release is processed or latched, and no transition
    /// occurs"; §8.1: "preserve a defined fatal handler and diagnostic evidence".
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
    /// ⚠️ **CONTRACT SILENT — and now listed as open.** F29 says "*Initial* dispatch", and its
    /// trace never returns to idle, so the contract never says what a *later* idle → task
    /// dispatch is called or costs; the fault contract's *Still open* list names it: "needed by no
    /// fixture yet, a later idle-to-task dispatch's cost". This model reports every
    /// idle → task transition as `Dispatch`, on the cost-accounting contract's structural reason —
    /// there is no outgoing context — rather than on the temporal one, since "first in the trace"
    /// is not a property of a transition.
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
    /// `outgoing` and `incoming` may name **the same task**. Since the fault contract's 2026-10-01
    /// rulings a job can end and be followed at once by the same task's next job — a late job
    /// skipped under `SkipLateJob` while its replacement starts (rule 1), or a job completing
    /// inside a masked region that latched its next release (rule 4). The finished job's context
    /// leaves and the fresh job's arrives, so by the same F29 reasoning it is one switch, not
    /// nothing and not a `ToIdle` plus a `Dispatch`.
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
    /// "supported interrupt points" qualifier is modelled by masked regions, not by a separate
    /// notion of preemptibility — see [`Runtime::mask`].
    Preempted(Transition),
    /// The task became ready and waits: the running task outranks it. Fixed priority, and §3.1 is
    /// emphatic that it stays fixed — "The scheduling model is fixed-priority even if the test
    /// harness randomizes event ordering at permitted boundaries."
    Ready,
    /// A masked region was open, so the release was **latched** and will take effect when the
    /// region closes (§8.1 "Model synchronization and interrupt masking explicitly"; F15
    /// "Interrupt pending while masked → Correct delivery and acknowledgment behavior").
    ///
    /// The fault contract's rule 1: "Every release that arrives while a masked region is open is
    /// latched, and is observed — for this rule and every other — only at its **delivery**, when
    /// the region closes." So this is what *every* release inside a region reports, including one
    /// for a task whose latch already holds one: a hosted model's latch is "the runtime's own
    /// record: the most recent release, and a mark that an earlier one also came" (Terms), so a
    /// later arrival takes the held place and the one it displaces is kept as the mark. Nothing is
    /// judged here, so there is nothing else to report; [`Runtime::is_overrun_latched`] shows the
    /// mark, and the policy's effect appears at delivery, in [`Unmasked::overruns`].
    ///
    /// §13.4's "arrivals during them are latched" is not this. The Terms read it as "pending in
    /// hardware" during a service, a transition or the like — the processor's own interrupt
    /// disable, which is not a masked region — and this model, whose services and transitions are
    /// atomic, has no state for it: such an arrival reaches the model as the next
    /// [`Runtime::release`].
    Latched,
    /// The task already owed a job, so this release had nowhere to go — and the task's overrun
    /// policy has **already been applied**, with its effect carried here.
    ///
    /// Its current job had not finished. (A release arriving inside a masked region is never
    /// reported here: the fault contract's rule 1 observes it at delivery — see
    /// [`ReleaseEffect::Latched`].) §3.1 excludes `general-ipc` — "including task-to-task queues" —
    /// and there is no per-task job queue either, so the runtime has exactly one slot per task. The
    /// release is therefore **neither stored nor silently dropped**: under a declared minimum
    /// separation `T` and a constrained deadline `D ≤ T` (§3.1 Workload) this cannot happen in the
    /// analyzed workload, so its occurrence means the separation was violated or the job overran,
    /// and either way §13.1 F17 requires the runtime to refuse timing assurance rather than absorb
    /// it — which the fault contract's rule 5 now states for itself: "A run's timing claims end at
    /// its first contained overrun" ([`Runtime::first_contained_overrun`]).
    ///
    /// ⭐ **AMENDED — the fault contract's rule 1, "Detection applies the policy".** This note used
    /// to read `⚠️ CONTRACT SILENT`, because nothing said whether detecting a second release must
    /// itself trigger the §3.1 overrun policy; this model reported the overrun and left the
    /// escalation to its caller. The fault contract decides it the other way:
    ///
    /// > An overrun is a fault the moment the runtime observes a release for a task that still
    /// > owes a job; it does not wait for a separate decision. … it is the system's behaviour
    /// > rather than a caller's option.
    ///
    /// So detection applies [`TaskSpec::on_overrun`] here, and the resulting [`FaultEffect`] is
    /// this variant's payload.
    ///
    /// ⭐ **AMENDED — the fault contract's rule 1, the triggering release.** This note used to read
    /// `⚠️ CONTRACT SILENT`: the rule settled that the policy applies, not what becomes of the
    /// release that triggered it. Under `TerminateJob` a runtime could abandon the late job and
    /// leave the task at rest, or abandon it and start a new one from this very release, and this
    /// model took the first reading — "the triggering release is consumed by the fault" — because
    /// the second begins a response interval at an instant the analyzed workload never contains.
    /// The ruling of 2026-10-01 takes the second, and the rewritten rule 1 says which release
    /// survives:
    ///
    /// > The release that triggers an overrun is the policy's: under `SkipLateJob` it makes the
    /// > task ready for its next job, with that release's nominal instant; under `Fault` it goes
    /// > with the faulted task.
    ///
    /// So under [`OverrunAction::TerminateJob`] — the contract's `SkipLateJob` — the late job is
    /// abandoned and the task is [`TaskState::Ready`] again with this release as its job. If it
    /// held the processor, the payload's transition is that handover, and since nothing can
    /// outrank a task that was running unmasked it is a `Switch` from the task to itself (see
    /// [`Transition::Switch`]); if it did not, nothing moves and the payload's transition is
    /// `None`. Under [`OverrunAction::StopTask`] — the contract's `Fault` — the release is
    /// consumed with the task. The objection this model had raised still stands as a statement
    /// about timing, not as a reading of the contract: the job this release starts is already
    /// outside the analyzed workload, which is why rule 5 ends the run's timing claims here.
    ///
    /// The payload is **never [`FaultEffect::Fatal`]**: an unmasked release meets only a declared
    /// policy, and rule 5's two policies never halt; a release inside a masked region is latched,
    /// so rule 3 cannot reach it.
    Overrun(FaultEffect),
    /// The task was stopped by a fault policy ([`OverrunAction::StopTask`]) and can never be
    /// released again: rule 5, "the triggering release and every later release of it are
    /// discarded".
    Stopped,
}

/// What a fault did.
///
/// ⭐ The context transition became an [`Option`] with the fault contract's rule 2. An overrun is
/// attributed to "the **overrunning** task, whichever context holds the processor", and containing
/// a fault in a task that does not hold the processor moves no context at all. `None` is therefore
/// not an absence of information: it is the positive statement that the schedule's occupant did not
/// change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaultEffect {
    /// The job was abandoned ([`OverrunAction::TerminateJob`], the contract's `SkipLateJob`).
    /// The release that detected the overrun is the task's next job, so the task is ready again
    /// (the fault contract's rule 1). Outside the profile, an overrun raised without a release
    /// leaves the task awaiting its next one ([`Runtime::raise_overrun`]). `Some` only if the
    /// faulting task held the processor.
    JobTerminated(Option<Transition>),
    /// The task was removed from the schedule for good ([`OverrunAction::StopTask`], the
    /// contract's `Fault`). `Some` only if the faulting task held the processor.
    TaskStopped(Option<Transition>),
    /// The runtime entered the fatal handler and halted (the fault contract's rule 7). Nothing is
    /// scheduled again, the state is frozen, and [`Runtime::fatal_record`] is the evidence (§8.1:
    /// "preserve a defined fatal handler and diagnostic evidence").
    Fatal,
}

/// The four faults §3.1 requires a defined policy for: "Defined overrun, unexpected-trap,
/// stack-guard, and assertion failure policy; bounded diagnostic handling".
///
/// ⭐ **Only the first is containable, and the contract is what decides that** — see each variant.
/// §8.1 asks for exactly this triage: "Distinguish expected errors, violated internal invariants,
/// and deliberate fatal traps." [`Fault::class`] gives each fault its class.
///
/// ⭐ **AMENDED — the fault contract's table.** §3.1's four faults and §8.1's three classes were two
/// lists with nothing joining them, and this model joined them by inference. The fault contract
/// states the join, and states attribution and containment with it; as last amended on 2026-10-01
/// it reads:
///
/// | §3.1 fault | §8.1 class | Attributed to | Containable |
/// |---|---|---|---|
/// | Overrun | Expected error | the **overrunning** task, whichever context holds the processor (rule 2) | Yes — by its declared per-task policy (§7.3, rule 5) |
/// | Stack guard | Violated internal invariant | the executing context (rule 2) | No |
/// | Unexpected trap | Deliberate fatal trap (its cause is outside the model) | the executing context (rule 2) | No |
/// | Assertion failure | Violated internal invariant | the executing context (rule 2) | No |
///
/// Earlier wordings read "which need not be the running one" in the overrun row and, in its last
/// column, "unless raised inside a masked region (rule 3)" — dropped because in this profile no
/// overrun is raised inside one (rules 1a and 3). The trap row read "Deliberate fatal trap, or a
/// surprise outside the model", then "Deliberate fatal trap", then "Outside the model, and so taken
/// by the deliberate fatal trap". The three synchronous rows read "the running task" or "the
/// executing task, whose guard was breached"; that last change is the one that moved this model:
/// they now name the **executing context**, which may be no task at all — see [`Context`] and
/// [`Runtime::raise_unexpected_trap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    /// A job exceeded its declared execution bound.
    ///
    /// An **expected error**, in §8.1's sense: the description declared a bound (§7.3 "execution
    /// bound") and reality exceeded it. The runtime's own invariants are untouched — nothing was
    /// corrupted, one number was wrong — so containment is possible, and §7.3 makes the response
    /// a per-task declaration ([`TaskSpec::on_overrun`]) rather than a runtime-wide rule.
    ///
    /// It is the only one of the four whose row in the fault contract's table is containable —
    /// "Yes — by its declared per-task policy" — and the only one attributed to a task rather than
    /// to the executing context. It is also the only timing fault: rule 6, "The runtime detects no
    /// missed deadline. Rule 1's overrun is its only timing fault" — which this model, having no
    /// deadlines and no time, satisfies by construction. A miss is observed only as rule 6 says:
    /// "A job that misses its deadline is reported only if it is still owed when the runtime
    /// observes the task's next release, which needs `D = T`."
    ///
    /// In this profile it has one source: "an overrun is detected by rule 1 alone" (rule 1a), at a
    /// release ([`ReleaseEffect::Overrun`]) or a delivery ([`Unmasked::overruns`]).
    Overrun,
    /// The architecture reported a trap the runtime did not plan for.
    ///
    /// **Fatal.** An *unexpected* trap means the runtime's model of the machine is wrong, and a
    /// runtime that cannot say why the machine trapped cannot argue that the damage stops at one
    /// task. §3.1 removes the one mechanism that could bound it — "Trusted application components
    /// in one address space; no claim of isolation" — and the profile excludes `memory-isolation`
    /// by name. Continuing to schedule would be an unchecked path of exactly the kind §8.1 forbids.
    ///
    /// The fault contract's row confirms it and picks the class: "Deliberate fatal trap (its cause
    /// is outside the model)", attributed to "the executing context (rule 2)", and "No" under
    /// *Containable*. The fatal path is the deliberate response; what is unexpected is what the
    /// machine did to reach it.
    ///
    /// The Terms say which traps are unexpected: "any trap other than the timer's interrupt, a
    /// declared source's interrupt whose claim finds a request, the runtime API's own entry where
    /// the port uses one, and a trap that is another fault's mechanism — an access fault on a
    /// stack's guard is a stack guard, and a trap the runtime raises deliberately on a failed check
    /// is an assertion failure. An external trap whose first claim finds no request is one, since
    /// the composition's `no-empty-claim` makes it a port's broken obligation; a later claim in the
    /// same trap that finds none ends its service loop and is no fault." This model sees no trap
    /// that is expected — a timer or source interrupt reaches it as a [`Runtime::release`], an API
    /// entry as the call itself — so what it is told is unexpected is.
    ///
    /// ⭐ **AMENDED — rule 2, who raises it (R3).** This note used to say that an empty claim "is
    /// raised in the context that made the claim ([`Context::Service`] or [`Context::TrapPath`])".
    /// Rule 2 now separates the two kinds: "An exception — a guard's access fault, an assertion's
    /// trap, any other faulting instruction — is raised by the context that executed the
    /// instruction; an unexpected trap that is an interrupt — a cause the port does not serve, an
    /// undeclared source, an empty claim — is raised by the trap path." So an empty claim is
    /// [`Context::TrapPath`]'s, never a service's; an unexpected trap that is an exception is the
    /// executing context's, which may be any [`Context`]. The model took the context it was given
    /// and still does — it cannot tell an interrupt from an exception, so it cannot refuse a
    /// service for one — so only this documentation changed.
    UnexpectedTrap,
    /// A stack guard was hit (§3.1 "stack-guard … policy", §7.6 "retain explicit guard/fault
    /// behavior"), and the [`Guard`] says whose.
    ///
    /// **Fatal**, for the same reason and one more. Task stacks are "separate statically
    /// allocated" (§3.1) in a single address space, so whatever lies beyond a guard belongs to
    /// something else with no protection boundary in between.
    ///
    /// ⭐ **AMENDED — the fault contract's table.** This note used to read `⚠️ CONTRACT SILENT`,
    /// because §3.1 and §7.6 say "stack guard" without saying whether it is a trapping guard region
    /// (which prevents the overflowing write) or a checked canary (which discovers it afterwards),
    /// and only the first reading could ever have permitted containment. The fault contract decides
    /// the *outcome* without pinning the mechanism — "Violated internal invariant … Containable:
    /// No" — so the question no longer reaches this model: whichever mechanism a target implements,
    /// the response is the same one.
    ///
    /// ⭐ **AMENDED — the fault contract's rule 2, whose guard.** This variant carried nothing, and
    /// the fault was attributed to the task whose guard was breached. Rule 2 now separates the two:
    /// "A stack guard is attributed the same way [to the executing context], and the evidence names
    /// whose guard was hit — a task's, or the interrupt stack's." So the guard travels with the
    /// fault into the [`FaultRecord`], and the attribution is the [`Context`]'s.
    StackGuard(Guard),
    /// An assertion failed (§3.1 "assertion failure policy").
    ///
    /// **Fatal.** An assertion is a written claim that an invariant holds; its failure is §8.1's
    /// "violated internal invariant", and scheduling onward from a state the program has just
    /// declared impossible is one of the "unchecked panic paths in normal runtime operation" that
    /// §8.1 tells the runtime to avoid.
    ///
    /// ⭐ **AMENDED — the fault contract's table.** This note used to read `⚠️ CONTRACT SILENT`,
    /// because §3.1 does not say whose assertion — the runtime's or the application's — and this
    /// model inferred that both must be fatal from §3.1's Isolation row making application
    /// components *trusted*. The fault contract reaches the same place without the inference:
    /// "Violated internal invariant … the executing context (rule 2) … No", with no distinction
    /// drawn by whose assertion it was.
    ///
    /// The model raises this fault **itself**, too, wherever the fault contract names an assertion
    /// failure it can detect: a `mask` past the declared depth ([`MaskEffect::Fatal`]), an `unmask`
    /// at depth zero ([`UnmaskEffect::Fatal`]), and a `mask` or `unmask` called with no job running
    /// (both). The fourth the Terms name, "A depth above zero when a job is dispatched or resumed",
    /// this model cannot reach ([`Violation::JobStartedInsideMaskedRegion`]).
    AssertionFailure,
}

impl Fault {
    /// The §8.1 class the fault contract's table gives this fault. Rule 7 preserves it with the
    /// fault.
    #[must_use]
    pub const fn class(self) -> Class {
        match self {
            Self::Overrun => Class::ExpectedError,
            Self::StackGuard(_) | Self::AssertionFailure => Class::ViolatedInternalInvariant,
            Self::UnexpectedTrap => Class::DeliberateFatalTrap,
        }
    }
}

/// §8.1's three classes: "Distinguish expected errors, violated internal invariants, and
/// deliberate fatal traps." The fault contract's table gives one to each §3.1 fault
/// ([`Fault::class`]), and rule 7 preserves it in the evidence ([`FaultRecord::class`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Class {
    /// An expected error — the overrun's: a declared bound was wrong, and the runtime's own
    /// invariants are intact.
    ExpectedError,
    /// A violated internal invariant — the stack guard's and the assertion failure's.
    ViolatedInternalInvariant,
    /// A deliberate fatal trap — the unexpected trap's: "Deliberate fatal trap (its cause is
    /// outside the model)".
    DeliberateFatalTrap,
}

/// Whose stack guard was hit. The fault contract's rule 2: "the evidence names whose guard was hit
/// — a task's, or the interrupt stack's."
///
/// ⚠️ **CONTRACT SILENT — which guard a context can hit.** Rule 2 attributes a stack guard to the
/// executing context and names the guard separately, and states no relation between the two. Which
/// stack each context runs on is a port's layout (§7.6 lists "task stacks, interrupt stack" and
/// fixes no assignment): a service may run on the interrupt stack or on the stack of the task it
/// interrupted, and a wild write from one task's job can reach another's guard. So this model
/// admits **every** pairing of [`Context`] and [`Guard`], and checks only that a guarded task
/// exists. A port that fixes the layout could refuse pairings this model accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Guard {
    /// A task's stack guard. §3.1: "Separate statically allocated task stacks".
    Task(TaskId),
    /// The interrupt stack's guard (§7.6: "task stacks, interrupt stack").
    InterruptStack,
}

/// The context that raised a synchronous fault — the fault contract's rule 2:
///
/// > The other three faults are attributed to the context that raises them. An exception — a
/// > guard's access fault, an assertion's trap, any other faulting instruction — is raised by the
/// > context that executed the instruction; an unexpected trap that is an interrupt — a cause the
/// > port does not serve, an undeclared source, an empty claim — is raised by the trap path. A
/// > task's job is that task, including a runtime primitive it called however the port enters it
/// > — where the port enters the runtime API by a trap, the code from that trap's entry to its
/// > return is the primitive, not the trap path — and the completion path it entered. A service,
/// > the trap path, a transition, idle, initialisation or the fatal handler is no task: the
/// > evidence names that context, a service by its source, and records the task it interrupted as
/// > interrupted, never as attributed. A service or the trap path interrupts the task whose job it
/// > preempted, or none if it preempted idle. A transition interrupts the incoming task once the
/// > switch is decided; while it is being decided, the outgoing task if its job is still owed, and
/// > none if the outgoing context was idle or its job has completed or been abandoned.
///
/// ⭐ **AMENDED — the fault contract's rule 2 (findings §6 (d); the first review's findings 4 and
/// 8).** Attribution has moved twice. Before 2026-09-13 a fault took no task at all, on the
/// reasoning that "§3.1's single execution context is what makes the attribution unambiguous: one
/// core, one running context, so a fault has exactly one task to belong to". The 2026-09-13
/// amendment separated attribution from execution — "The single-core rule of §3.1 governs
/// *execution*, not *attribution*" — and the model's `raise` took a task, refusing any but the
/// running one for the three synchronous faults, and refusing them outright with the processor
/// idle, as having "no context to attribute it to". The rewrite gives such a fault a context that
/// is no task, so this type replaced the task parameter: [`Context::Task`] for a job, the other
/// variants for the contexts that are none, and the evidence ([`Attribution::NoTask`]) names that
/// context and keeps the interrupted task apart from the attributed one.
///
/// The fatal handler is deliberately **not** a variant. A fault in the handler can only be raised
/// into a runtime that has already halted, where every operation is refused with
/// [`Refused::Halted`] and the record is untouched — which is rule 7's "A fault taken in the
/// handler ends it at once in its terminal state and never replaces the preserved one", with
/// nothing left over for a variant to do.
///
/// ⚠️ **DECIDED, NOT EXPRESSED — initialisation (R3).** Rule 2 now lists initialisation among the
/// contexts that are no task, and the Terms say what it is: "running before the first enabling of
/// interrupts with no job and with interrupts already disabled". A fault raised there is no task's,
/// interrupts none — it preempted nothing — and its record names initialisation. This model has no
/// state inside initialisation: [`Runtime::boot`] is one call that refuses or returns the model
/// idle, and its first event may already be a release, which an enabled interrupt delivers. So
/// there is no variant for it, and a port's fault in initialisation cannot be replayed here. The
/// nearest state is the one just after boot, every task still [`TaskState::Created`]; admitting a
/// variant there, with no task interrupted, is a choice the text does not make for a model whose
/// initialisation is atomic, and this model does not make it either.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Context {
    /// A task's job — "including a runtime primitive it called however the port enters it", and
    /// "the completion path it entered". It must name the task holding the processor
    /// ([`Refused::NotTheRunningTask`], [`Refused::NoTaskRunning`]). A fault in the completion path
    /// is raised before [`Runtime::complete`], while the job still holds the processor; so is one
    /// in a primitive the port enters by a trap, since "the code from that trap's entry to its
    /// return is the primitive, not the trap path".
    Task(TaskId),
    /// An interrupt service: a release service, or another declared source's (§3.1 Interrupts).
    /// It "interrupts the task whose job it preempted, or none if it preempted idle", which is
    /// the task holding the processor when the fault is raised. (Rule 1a: this profile has no
    /// execution-budget monitor, so no service here is one.)
    ///
    /// ⚠️ **DECIDED, NOT EXPRESSED — a service by its source (R3).** Rule 2: "the evidence names
    /// that context, a service by its source"; rule 7 keeps it, "or no task, with the executing
    /// context named, a service by its source". This model has no sources — a release names its
    /// task, and boot declares no interrupt source — so this variant carries none, and a
    /// [`FaultRecord`] says "a service" without saying which. An adapter compares a port's source
    /// outside the model. Carrying one would need an identity for a source and a declared set to
    /// check it against; the text names a source's catalog record but no identifier the fault
    /// record would carry, so this model does not choose one.
    Service,
    /// The runtime's trap path (§8.2 "trap entry/exit"). Like a service, it interrupts the task
    /// whose job it preempted, or none if it preempted idle. Rule 2 raises here "an unexpected trap
    /// that is an interrupt — a cause the port does not serve, an undeclared source, an empty
    /// claim"; a fault in the job's own instruction, or in a primitive it entered by a trap, is
    /// [`Context::Task`].
    ///
    /// Rule 4 adds one more: in a hosted model, which decides after the delivery a completion
    /// makes, "a fault raised during the delivery is … the trap path's". This model's
    /// [`Runtime::complete`] keeps that order and is one atomic call, so such a fault is raised
    /// before or after it; which task the trap path then interrupts — the completed job's, or none
    /// — the text does not say (see [`Runtime::complete`]).
    TrapPath,
    /// A context transition ([`Transition`]; §7.4.1). The task the processor shows when the fault
    /// is raised, if any, is recorded as interrupted.
    ///
    /// ⭐ **AMENDED — rule 2, which task a transition interrupts (the second review's finding 34,
    /// then R3).** This note used to read `⚠️ CONTRACT SILENT`: rule 2 said only that "an
    /// interrupted task" was recorded. The second rewrite said "the task the runtime holds as
    /// running when the fault is raised", which is what this model recorded. Rule 2 now names the
    /// task by the transition's phase: "A transition interrupts the incoming task once the switch
    /// is decided; while it is being decided, the outgoing task if its job is still owed, and none
    /// if the outgoing context was idle or its job has completed or been abandoned." This model has
    /// no state between a transition's decision and its end: every event completes its transition.
    /// So an adapter raises a fault "once the switch is decided" after the event, where the
    /// processor shows the incoming task, or none after a `ToIdle`; and one raised "while it is
    /// being decided" in a preemption — a release, or a delivery, taking the processor from a job
    /// still owed — before the event, where the processor shows that outgoing task, or none from
    /// idle. Both give the task rule 2 names.
    ///
    /// ⚠️ **DECIDED, NOT EXPRESSED — a transition being decided after its outgoing job completed
    /// or was abandoned (R3).** This note used to read `⚠️ CONTRACT SILENT — at what point a
    /// completed job stops being held as running`, and said that a fault in the transition that
    /// follows a completion "names the completed task before the event and the incoming one after
    /// it". Rule 2 now decides that point: while the switch is being decided, the transition
    /// interrupts none. This model has no state there. Before [`Runtime::complete`] — or before the
    /// release or delivery that abandons a job under rule 5 — the job is still owed and no
    /// transition has begun; after it, the switch is decided. So the reading quoted above is
    /// withdrawn: the state before the event is not inside that transition. A fault a port raises
    /// there cannot be replayed here, except where the incoming context is idle — after a `ToIdle`
    /// the processor shows none, which is rule 2's answer in both phases.
    Transition,
    /// Idle: no task holds the processor. Admissible only while the processor is idle
    /// ([`Refused::NotIdle`]); nothing is interrupted.
    Idle,
}

/// Whom a fault is attributed to: the fault contract's "Attributed to" column, preserved by rule 7
/// as "the attributed task's stable logical ID … — or no task, with the executing context named, a
/// service by its source; the task it interrupted, if any".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attribution {
    /// A task: for an overrun, the overrunning task (rule 2: "the task a release, or a delivered
    /// latched release, belongs to"); for the other three, the task whose job — or a primitive it
    /// called, or the completion path it entered — executed the faulting instruction. Its stable
    /// logical ID is [`TaskSpec::name`], which an adapter supplies through [`Runtime::spec`] when
    /// the record leaves the model (see [`TaskId`]).
    ///
    /// It carries no interrupted task. A synchronous fault attributed to a task was executed by
    /// that task's own job, which interrupted nothing. An overrun is preserved only when rule 3
    /// escalates it, which in this profile never happens (rule 1a); see
    /// [`Runtime::raise_overrun`] for the entry outside it.
    Task(TaskId),
    /// No task: the fault was raised in a context that is no task (rule 2).
    NoTask {
        /// The executing context. Never [`Context::Task`]. A service is named without its source
        /// (see [`Context::Service`]).
        context: Context,
        /// The task that context interrupted, if any — recorded "as interrupted, never as
        /// attributed" (rule 2), and preserved as "the task it interrupted, if any" (rule 7).
        interrupted: Option<TaskId>,
    },
}

/// The fault the fatal handler preserves — the fault contract's rule 7:
///
/// > It preserves until reset the **first** such fault only, written before anything else the
/// > handler does: its §3.1 kind; its §8.1 class; the attributed task's stable logical ID — its
/// > elaborated eADL task name, the instance path of `docs/semantics/reference.md` §6 rule 9, which
/// > `archogen check` requires unique (`schema-duplicate-name`) — or no task, with the executing
/// > context named, a service by its source; the task it interrupted, if any; for a stack guard,
/// > whose guard was hit; and whether rule 3 escalated it, which in this profile it never does. A
/// > runtime that holds tasks by an internal index has the plan supply the ID when the record
/// > leaves it (§7.5): an index names no task in a record. A fault taken in the handler ends it at
/// > once in its terminal state and never replaces the preserved one.
///
/// Field by field: the kind is [`FaultRecord::fault`], which for a stack guard carries whose guard
/// was hit; the class is [`FaultRecord::class`]; the attributed task, or no task with the executing
/// context named and the task it interrupted, is [`FaultRecord::attribution`]; and rule 3's
/// escalation is [`FaultRecord::escalated`]. The record is the first thing the handler writes and
/// the only thing it does. Two of rule 7's demands this record does not meet by itself: its tasks
/// are [`TaskId`]s, which the plan — here the booted set — names when the record leaves the model
/// (see [`TaskId`]); and a service is named without its source, which this model cannot express
/// (see [`Context::Service`]).
///
/// §3.1 requires "bounded diagnostic handling" and F26 a "bounded diagnostic path"; the bound here
/// is one record, kept by the first fault that halted the runtime, which therefore describes the
/// cause rather than the last consequence. Rule 7 also bounds the handler's *duration*, "within a
/// bound the runtime's catalog record declares beside the nesting bound `M`", which this model,
/// having no time, does not see.
///
/// ⭐ **AMENDED — the fault contract's rule 7 (the first review's finding 9).** This record used to
/// be the first fault *of any kind*, contained or not, with fields `task`, `fault`, `fatal` and
/// `masked`. Rule 7 makes the preserved fault the first that **entered the fatal handler** — "the
/// first such fault", such as "is not containable, or is made so by rule 3". A contained overrun
/// before it is not such a fault, and a record showing one would describe an episode the runtime
/// survived rather than the one that stopped it. So the record now exists only once the runtime has
/// halted ([`Runtime::fatal_record`]); `fatal` went, since every such record is; `task` became
/// [`FaultRecord::attribution`], since rule 2 admits no task; and `masked` became
/// [`FaultRecord::escalated`], since rule 7 asks whether rule 3 escalated the fault, not whether
/// the depth was above zero — an assertion failure inside a region was masked and was not
/// escalated, because it never was containable. The first *contained* overrun, which rule 5 makes
/// the end of a run's timing claims, is kept apart: [`Runtime::first_contained_overrun`].
///
/// ⭐ **AMENDED — rule 7, the interrupted task (the second review's finding 34).** Rule 7 did not
/// list the interrupted task, though rule 2 told the evidence to record it; it now keeps "the task
/// it interrupted, if any". This model already kept it, inside [`Attribution::NoTask`], so the
/// record's shape is unchanged.
///
/// ⭐ **AMENDED — rule 7's fields, one by one (R3).** Rule 7 now lists the record's fields
/// separately, adding "for a stack guard, whose guard was hit", which [`Fault::StackGuard`]
/// already carried by rule 2, and "a service by its source", which this model cannot (above). The
/// record's shape is unchanged again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FaultRecord {
    /// Its §3.1 kind; for a stack guard, with whose guard was hit (rule 2).
    pub fault: Fault,
    /// Its §8.1 class, per the fault contract's table — always `fault.class()`.
    pub class: Class,
    /// The attributed task, or no task with the executing context and the task it interrupted
    /// (rules 2 and 7).
    pub attribution: Attribution,
    /// Whether rule 3 escalated it: a containable fault — an overrun — raised inside a masked
    /// region. False for every fault that was never containable, wherever it was raised. In
    /// `rt-static-up-v1` it is never true — rule 3 "decides nothing observable in this profile" —
    /// and only [`Runtime::raise_overrun`], the entry outside the profile, sets it.
    pub escalated: bool,
}

/// What a masked region's closing did: the outermost [`Runtime::unmask`], or a completion that
/// closed the region ([`Runtime::complete`]). An inner `unmask` reports it too, with nothing
/// delivered.
///
/// The two share one record because the fault contract's rule 4 makes a completion's closing a
/// delivery too: "releases latched in the region are delivered after the completion is recorded and
/// before any task executes an instruction of its own code". A completion outside any region
/// reports the same record with nothing delivered and the completion's own transition.
///
/// Every latched arrival processed lands in exactly one of `delivered`, `overruns` and `stopped`,
/// and a delivery processes every one: it never halts. It is outside every masked region, so rule
/// 3 does not reach it, and rule 5's two policies never halt — under `Fault`, "every other task
/// continues, and the runtime does not halt". (Before rule 5 this model had a third, halting
/// policy, and a delivery could stop partway at it.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unmasked {
    /// The mask depth that remains. Non-zero means this was an inner `unmask` of a nested pair
    /// and nothing was delivered. Always `0` after a completion, which closes every section.
    pub depth: u8,
    /// How many latched arrivals found their task owing no job, and made it ready.
    pub delivered: usize,
    /// How many latched arrivals found the task already owing a job, **each of which had that
    /// task's overrun policy applied** (the fault contract's rule 1).
    ///
    /// The policy applied is the declared one, not an escalation. The fault contract's rule 3 is
    /// explicit: "Delivery is **not** inside a masked region, so an overrun found at delivery
    /// applies its ordinary policy — the difference between a profile that can contain an overrun
    /// and one that cannot."
    ///
    /// ⭐ This includes an overrun a latch's **mark** stands for. Rule 1: "At delivery a task's
    /// latched arrivals are judged in arrival order against the task's state then: the first is
    /// fresh if the task owes no job (one that completed inside the region included) and an
    /// overrun if it does; each later one is an overrun under `SkipLateJob`, and is discarded under
    /// `Fault`, whose first overrun took the task out; the earlier arrivals a hosted latch's mark
    /// stands for are judged as one." So a marked latch is two arrivals — the mark's, earliest,
    /// then the held one, most recent — and counts here once or twice under `SkipLateJob`, exactly
    /// as the same two arrivals would be judged one instruction after the region closed. Under
    /// `Fault` it counts here at most once: a held release after the task's overrun is counted in
    /// [`Unmasked::stopped`].
    ///
    /// ⚠️ **CONTRACT SILENT — a later arrival under `Fault` when the first was fresh.** Read word
    /// by word, "each later one … is discarded under `Fault`" also discards the second arrival of
    /// a task that owed no job, whose first arrival was fresh; but the clause's own reason, "whose
    /// first overrun took the task out", has not happened then, and the same rule judges every
    /// arrival "against the task's state then" and says an overrun is "not lost … for landing
    /// inside a region rather than one instruction after it". One instruction after the region,
    /// the second of two releases to a task owing the job the first started is an overrun, and
    /// `Fault` takes the task out. This model judges it so: the held release is the task's first
    /// overrun, counted here, and only an arrival after that overrun is discarded. A reader taking
    /// the clause literally would leave the task ready with the first arrival's job and count the
    /// second in [`Unmasked::stopped`].
    pub overruns: usize,
    /// How many latched arrivals named a stopped task, and were discarded
    /// ([`ReleaseEffect::Stopped`]) — whether the task was stopped before the region closed or by
    /// an earlier arrival of the same delivery. Rule 1: a later arrival "is discarded under
    /// `Fault`, whose first overrun took the task out"; rule 5: "the triggering release and every
    /// later release of it are discarded".
    pub stopped: usize,
    /// The **single** context transition the delivery caused, if any.
    ///
    /// ⭐ Single, on purpose. Every latched release is applied to the ready set *before* the
    /// scheduler runs once, so delivering `n` releases costs at most one transition rather than
    /// `n`. Two clauses force this: §3.1's "bounded kernel critical sections", which an
    /// n-switch delivery would not be, and §7.4.1's demand that "the number and kind of context
    /// transitions" be identifiable — a count that depends on the order flags happened to be
    /// examined in is not identifiable. (F29's "arrivals during them are latched and serviced
    /// before the next task computation interval" used to be cited here as agreeing. The Terms now
    /// read §13.4's "latched" as "pending in hardware" during a service or a transition, not as a
    /// task's latch, so F29 says nothing about this delivery.)
    ///
    /// An overrun policy that vacates the processor during delivery is folded into the same single
    /// transition rather than causing one of its own: the faulting task stands down, the scheduler
    /// still runs exactly once at the end, and the result is one `Switch` (or one `ToIdle`) that
    /// names it as the outgoing task. A completion is folded in the same way, so after
    /// [`Runtime::complete`] this is always `Some` and names the completing task as outgoing. That
    /// is the order rule 4 calls "a hosted model" — the delivery, then the decision — which it
    /// leaves to the port beside the target's opposite order: "Both reach the same schedule"; see
    /// [`Runtime::complete`].
    pub transition: Option<Transition>,
}

/// What [`Runtime::mask`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaskEffect {
    /// The depth rose by one, to this value. At `0 → 1` a masked region opened.
    Masked(u8),
    /// An **assertion failure**, so the runtime entered the fatal handler and halted (the fault
    /// contract's rule 7): the `mask` was executed with no job running, or would have raised the
    /// depth past [`Runtime::MAX_MASK_DEPTH`]. The depth is left where it was, as evidence.
    ///
    /// ⭐ **AMENDED twice — the fault contract's second smaller decision.** This model first refused
    /// a `mask` past the bound under a `⚠️ CONTRACT SILENT` note, because §3.1 requires "bounded
    /// kernel critical sections" and §7.3 requires every interrupt source to declare its "masking
    /// constraints", while no maximum nesting depth was stated anywhere. The 2026-09-13 amendment
    /// stated one and agreed with the refusal — "Kernel critical sections are bounded by a
    /// declared nesting depth, and exceeding it is refused" — and the refusal was
    /// `Refused::MaskDepthExhausted`. The first review's finding 10 then found the hole in
    /// refusing, and the rewritten decision closes it:
    ///
    /// > A `mask` that would raise the depth past `M`, and an `unmask` at depth zero, are
    /// > **assertion failures**: a counter that *wraps* re-enables interrupts inside a critical
    /// > section while reporting success, one that *saturates* stops counting, and one that
    /// > *refuses* leaves its caller's matching `unmask` to close the section early.
    ///
    /// The caller of a refused `mask` still issues the `unmask` that matches it, and that `unmask`
    /// closes the *enclosing* section instead — the wrap's failure by a slower route. So the
    /// refusal is gone, `Refused::MaskDepthExhausted` with it, and the model halts as the contract
    /// now requires. Past the bound, a job is running, so the assertion failure is attributed to
    /// it (rule 2): its job, whose primitive `mask` is.
    ///
    /// ⭐ **AMENDED — the fault contract's Terms, "Only a job changes the depth" (the second
    /// review's finding 35, restated after R3).** A `mask` executed by a context that is no job "is
    /// an assertion failure of that context, raised before the depth changes", and "A runtime that
    /// is told no context detects the case it can see, a call with no job running". See
    /// [`Runtime::mask`]: this model used to accept a `mask` with the processor idle. It is now
    /// this variant, attributed to the executing context, which with no job running this model
    /// takes to be idle.
    Fatal,
}

/// What [`Runtime::unmask`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnmaskEffect {
    /// The depth fell by one. At `1 → 0` the masked region closed and its latched releases were
    /// delivered; an inner `unmask` delivers nothing.
    Unmasked(Unmasked),
    /// An **assertion failure**, so the runtime entered the fatal handler and halted (the fault
    /// contract's rule 7): the `unmask` was executed with no job running, or at depth zero.
    ///
    /// ⭐ **AMENDED — the fault contract's second smaller decision (the first review's finding
    /// 10).** This model used to refuse an `unmask` at depth zero, with `Refused::NotMasked`, under
    /// a note that read `⚠️ CONTRACT SILENT — narrowed by §3.1.1, not closed`: an unbalanced unmask
    /// was §8.1's "violated internal invariant", and the table settled that such a fault, once
    /// raised, is not containable, but the table classified only the four faults §3.1 names, and
    /// nothing said a runtime detecting an unbalanced unmask must raise one. The model refused and
    /// did not halt "because the *model* is a checker being driven by an adapter: halting here
    /// would destroy the rest of a comparison run over a mistake in the harness", while conceding
    /// that "a real runtime detecting the same thing in its own kernel would have the stronger case
    /// for `Fault::AssertionFailure`, and §3.1.1 would then make it fatal".
    ///
    /// The contract has now decided it: "A `mask` that would raise the depth past `M`, and an
    /// `unmask` at depth zero, are **assertion failures**". So the model halts too, and
    /// `Refused::NotMasked` is gone. What changed is not the harness argument but its weight: an
    /// unbalanced `unmask` is now a contract-defined fatal fault that the implementation must halt
    /// on as well, so a comparison run that continued past it would be comparing two runtimes the
    /// contract says have stopped. Refusing would also hide the imbalance from the caller, who
    /// would go on believing a section was open — rule 4 makes that easy to reach, since a job that
    /// completes inside a region has closed it, and an `unmask` meant for that region then arrives
    /// at depth zero.
    ///
    /// ⭐ **AMENDED — the fault contract's Terms, "Only a job changes the depth".** An `unmask` made
    /// as "a call with no job running" is an assertion failure too, whatever the depth. In this
    /// model the two conditions coincide on every reachable state — only a job opens a region and
    /// its completion closes it, so with no job running the depth is zero — but the model checks
    /// the Terms' condition first, and attributes the failure to the executing context (rule 2):
    /// the running task's job, or idle.
    Fatal,
}

// ---------------------------------------------------------------------------------------------
// Refusals. A refusal says the caller described an event the model will not take: one that names
// something outside the static set, or one the model's state contradicts. A refusal changes
// nothing. It is not a fault: the faults the contract defines are raised and recorded, never
// refused.
// ---------------------------------------------------------------------------------------------

/// Why an operation was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The runtime is in its fatal handler. Every operation is refused from then on; the state is
    /// evidence, not a schedule. The fault contract's rule 7: "From then no job runs, no release is
    /// processed or latched, and no transition occurs", and "A fault taken in the handler ends it
    /// at once in its terminal state and never replaces the preserved one."
    Halted,
    /// The operation needs a running task and the processor is idle: [`Runtime::complete`], or a
    /// synchronous fault raised in a [`Context::Task`].
    NoTaskRunning,
    /// A synchronous fault named a [`Context::Task`] that is not the one holding the processor.
    ///
    /// The fault contract's rule 2: "The other three faults are attributed to the context that
    /// raises them", and "An exception … is raised by the context that executed the instruction". A
    /// task that does not hold the processor is not executing anything, so attributing the fault to
    /// it is the concurrency claim the profile does not admit, and it is refused rather than
    /// recorded.
    NotTheRunningTask,
    /// A synchronous fault named [`Context::Idle`] while a task holds the processor: idle is not
    /// executing (rule 2, as for [`Refused::NotTheRunningTask`]).
    NotIdle,
    /// [`Runtime::raise_overrun`] — the entry outside the profile — named a task that owes no job,
    /// so there is nothing for it to have overrun.
    ///
    /// The fault contract's Terms: a task "owes a job from its release until that job completes or
    /// is abandoned", which is exactly [`TaskState::Ready`] or [`TaskState::Running`]; and rule 1
    /// makes an overrun a statement about "a task that still owes a job". A task that has never
    /// been released, has completed or abandoned its last job, or has been stopped is not making
    /// late progress on anything.
    NoJobOwed,
    /// The id does not name a task in this static set.
    UnknownTask,
}

/// A violated structural invariant (see [`Runtime::check_invariants`]).
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
    /// A masked region is open and is not the running job's: the processor is held by a task
    /// other than the job that opened it — so a job started, or resumed, at a depth above zero —
    /// or no job opened it, or no job holds the processor.
    ///
    /// The fault contract's Terms: "**Only a job changes the depth.**" A `mask` or `unmask`
    /// executed by any other context "is an assertion failure of that context, raised before the
    /// depth changes", and "A depth above zero when a job is dispatched or resumed is likewise an
    /// assertion failure, raised by the decision that would do it … So every section open at a
    /// completion is the completing job's." This model cannot reach either state: a `mask` with no
    /// job running halts ([`MaskEffect::Fatal`]), inside a region arrivals latch, a completion
    /// closes the region before any job starts, and every fault inside one halts. So finding one
    /// means the model itself is broken, which is what a [`Violation`] reports, rather than the
    /// assertion failure a runtime would raise.
    ///
    /// ⭐ **AMENDED — the context of "the decision that would do it" (R3).** This note used to read
    /// `⚠️ CONTRACT SILENT`: the Terms said the job-start assertion was "raised by that decision,
    /// which is no task's", rule 2's list of contexts that are no task named no decision, and rule
    /// 4 let a port decide in the completion path, which rule 2 makes the completing task's. The
    /// Terms now say whose it is — "the transition's, no task's, even where a port makes that
    /// decision inside the completion path — and it interrupts the task rule 2 names for a
    /// transition". Were this model to raise it, it would be [`Context::Transition`]'s; it never
    /// does, so nothing here moved.
    JobStartedInsideMaskedRegion {
        /// The job that opened the region, if any. Since only a job opens one, `None` with a region
        /// open is itself the violation.
        holder: Option<TaskId>,
        /// The task that holds the processor now, if any.
        running: Option<TaskId>,
    },
}

/// Why a task set cannot boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootError {
    /// The task set is empty.
    ///
    /// ⭐ **AMENDED — the fault contract's first smaller decision.** This note used to read
    /// `⚠️ CONTRACT SILENT`: §3.1 says "finite static task set", and the empty set is finite. The
    /// fault contract refuses it, and refuses it for the reason this model had given —
    ///
    /// > **A task set must be non-empty** — no workload makes §7.2's second timing obligation
    /// > vacuous. `archogen build` and boot refuse one; a refused boot is no fault: the runtime
    /// > never starts, and nothing is kept but the refusal. `archogen check` admits one, a system
    /// > describing composition alone (F01) being valid, and makes no timing claim for it
    /// > (`M2.17`; R3 79).
    ///
    /// — so the behaviour is unchanged and only its authority has: it is now the contract's
    /// reasoning rather than this model's inference from it, and boot is where the contract puts
    /// the refusal, beside `archogen build`. A refused [`Runtime::boot`] returns this error and no
    /// [`Runtime`], so no fault is raised and nothing is kept but the refusal — as for every
    /// [`BootError`].
    NoTasks,
    /// Two tasks share a priority. §3.1: "Static **unique** task priorities".
    DuplicatePriority {
        /// The rank they share.
        rank: i64,
        /// The lower-indexed task.
        first: TaskId,
        /// The higher-indexed task.
        second: TaskId,
    },
    /// A task declares a rank below `1`: `0`, or a negative rank.
    ///
    /// ⭐ **AMENDED — `decision_priority-comparison-direction.md`, "The admissible range, and the
    /// runtime's index".** This note used to read `⚠️ CONTRACT SILENT`: the record said "`1` is the
    /// highest" and said nothing about `0`. Its item 1 closes the range —
    ///
    /// > **A rank is an integer `N ≥ 1`** — any such integer the language's integer type holds, a
    /// > 64-bit signed one … Rank `0` is **not** admissible in a description. … A description
    /// > carrying `(priority 0)` is refused.
    ///
    /// — on this model's own ground, that "admitting it would move the top of the range by
    /// inference, and §15 makes a change to a parameter's meaning a versioned language change
    /// rather than a tolerance".
    ///
    /// ⭐ **AMENDED — every rank below `1`, not only `0` (item 1, leaf `M2.18`).** This variant was
    /// `PriorityRankZero { task }`, because a rank held in a `u16` could fall below `1` only at
    /// `0`. Item 1 makes a rank the language's 64-bit signed integer, which [`Priority`] now holds,
    /// so a negative rank reaches boot too; and `archogen check` "refuses it, and any rank below 1,
    /// with `priority-below-one`". Boot refuses them all, and names the rank it refused.
    PriorityBelowOne {
        /// The offending task.
        task: TaskId,
        /// The rank it declares, below `1`.
        rank: i64,
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
    /// task-to-task queues". The fault contract's Terms give a hosted model's latch exactly its
    /// shape: "the runtime's own record: the most recent release, and a mark that an earlier one
    /// also came".
    latch: [Latch; N],
    running: Option<TaskId>,
    mask_depth: u8,
    /// The job whose outermost `mask` opened the open masked region — the region's holder, which
    /// rule 3's first ground and [`Runtime::raise_overrun`] distinguish. Meaningful only while
    /// [`Runtime::is_masked`], and then always `Some`: only a job changes the depth (Terms).
    holder: Option<TaskId>,
    /// Rule 7's record. `Some` exactly when the runtime has halted.
    fatal: Option<FaultRecord>,
    /// Rule 5's end of a run's timing claims: the task of the first contained overrun.
    first_contained_overrun: Option<TaskId>,
}

impl<const N: usize> Runtime<N> {
    /// The deepest mask nesting this model admits: the fault contract's `M`.
    ///
    /// The second smaller decision: "Each runtime's catalog record declares a maximum depth
    /// `M ≥ 1`. A `mask` that would raise the depth past `M`, and an `unmask` at depth zero, are
    /// **assertion failures**". The value is this model's declaration of it; its finiteness, its
    /// being declared, and the assertion failure past it are the contract's — see
    /// [`MaskEffect::Fatal`]. "This bounds the depth; the duration is bounded separately, by the
    /// `CS_i` the timing analysis charges", which this model, having no time, does not see.
    pub const MAX_MASK_DEPTH: u8 = u8::MAX;

    /// Create the static task set. This is §8's "static task creation at boot", and it is the
    /// only way a task ever comes into existence.
    ///
    /// Every task starts in [`TaskState::Created`] with no masked region open and the processor
    /// idle.
    ///
    /// ⚠️ **CONTRACT SILENT — and now listed as open.** Whether a periodic task's *first* job is
    /// released at boot or at its first timer tick is not stated anywhere. §3.1 describes
    /// "periodic or sporadic releases with declared minimum separation" without fixing the phase
    /// of the first one, while F29 simply asserts "The initial state already contains L's ready
    /// job" for its own fixture; the fault contract's *Still open* list now names it — "needed by
    /// no fixture yet, … a periodic task's first release instant". This model takes no position:
    /// boot releases nobody, and a caller that wants F29's initial state calls [`Runtime::release`]
    /// to establish it. That keeps the choice visible in the trace instead of buried in the
    /// constructor, where a disagreement about it would look like a disagreement about scheduling.
    ///
    /// # Errors
    ///
    /// [`BootError`] if the set is empty, shares a priority, or declares a rank below `1` — the
    /// first found in declaration order, a task's own rank checked before its duplicates. A refusal
    /// is no fault (the first smaller decision): no [`Runtime`] exists to keep anything.
    pub fn boot(spec: [TaskSpec; N]) -> Result<Self, BootError> {
        if N == 0 {
            return Err(BootError::NoTasks);
        }
        for (i, task) in spec.iter().enumerate() {
            // The priority record's item 1: "A rank is an integer `N ≥ 1`".
            let rank = task.priority.rank();
            if rank < 1 {
                return Err(BootError::PriorityBelowOne {
                    task: TaskId(i),
                    rank,
                });
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
            holder: None,
            fatal: None,
            first_contained_overrun: None,
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
        if self.halted() {
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

    /// Whether this task's latch holds an undelivered release (F15's "pending while masked").
    #[must_use]
    pub fn is_latched(&self, task: TaskId) -> Option<bool> {
        self.latch
            .get(task.index())
            .map(|&latch| latch != Latch::Empty)
    }

    /// Whether this task's latch also holds **the mark that an earlier release came** — the fault
    /// contract's Terms: a hosted model's latch is "the runtime's own record: the most recent
    /// release, and a mark that an earlier one also came".
    ///
    /// At delivery the mark is judged first, since its arrivals came first, and "the earlier
    /// arrivals a hosted latch's mark stands for are judged as one" (rule 1); the held release is
    /// judged after it, against the state the mark left — an overrun if the task then owes a job,
    /// discarded if the mark's own overrun took the task out under `Fault`. Until delivery nothing
    /// about the task has changed and no fault is recorded — see [`Unmasked::overruns`].
    #[must_use]
    pub fn is_overrun_latched(&self, task: TaskId) -> Option<bool> {
        self.latch
            .get(task.index())
            .map(|&latch| latch == Latch::ReleaseAndMark)
    }

    /// The current mask nesting depth.
    ///
    /// After a halt it is the depth the fault left, kept as evidence. The fatal handler ends "in a
    /// terminal state with interrupts masked" (rule 7), but that is the processor's own interrupt
    /// disable, which the Terms say "is **not** a masked region", so it is not counted here.
    #[must_use]
    pub const fn mask_depth(&self) -> u8 {
        self.mask_depth
    }

    /// Whether a masked region is open: the fault contract's Terms, "the interval in which the
    /// runtime's mask nesting depth is above zero". Releases arriving now are latched.
    #[must_use]
    pub const fn is_masked(&self) -> bool {
        self.mask_depth > 0
    }

    /// The fault the fatal handler preserved (the fault contract's rule 7), or `None` while the
    /// runtime runs.
    ///
    /// ⭐ **AMENDED — rule 7.** This replaces `fault_record`, which kept the first fault of any
    /// kind; see [`FaultRecord`] for why the halt's evidence is now the first fault that entered
    /// the fatal handler.
    #[must_use]
    pub const fn fatal_record(&self) -> Option<FaultRecord> {
        self.fatal
    }

    /// The task whose overrun was the run's first to be **contained** — by its declared policy,
    /// at a release or at a delivery (or, outside the profile, raised without a release) — or
    /// `None` if none has been.
    ///
    /// The fault contract's rule 5: "A run's timing claims end at its first contained overrun,
    /// which shows that an assumption of its analysis — an execution bound, an arrival bound or the
    /// interference it counted — did not hold, or that the claim was not established." This is
    /// where they ended. An escalated overrun is not contained, and appears in
    /// [`Runtime::fatal_record`] instead.
    ///
    /// ⭐ **AMENDED — rule 1, the order across tasks (the second review's finding 42).** This note
    /// used to read `⚠️ CONTRACT SILENT — which overrun is first at one delivery`: rule 1 ordered a
    /// task's own latched arrivals, not different tasks', and the latch keeps no order between
    /// tasks. Rule 1 now decides it by giving it away: "Across tasks the order changes no task's
    /// state and is the port's (`M2.15` traces it)." This model, as its own port, judges tasks in
    /// ascending rank (see [`Runtime::unmask`]), so of two overruns found at one delivery the
    /// higher-priority task's is named here. It changes no task's state, as rule 1 says; this is
    /// the one place it shows, and an adapter comparing this value against another port's after a
    /// delivery that found two tasks' overruns compares two ports' choices, not one contract.
    #[must_use]
    pub const fn first_contained_overrun(&self) -> Option<TaskId> {
        self.first_contained_overrun
    }

    // -- events --------------------------------------------------------------------------------

    /// A release arrives for `task`.
    ///
    /// This is the moment the task becomes eligible to run — which, per
    /// `docs/analysis/cost-accounting-v1.md`, is *ISR completion*, not interrupt arrival: "a
    /// release is signalled by an interrupt; the task becomes ready at ISR completion". The whole
    /// release service is this one atomic call. Its own interrupt disable is not a masked region
    /// (the fault contract's Terms), so it never changes the depth; it only reads it, and an
    /// arrival that lands inside a masked region is latched instead.
    ///
    /// Each call is one arrival the platform delivered as a distinct request. Rule 1 says which
    /// arrivals those are — "A timer-released task's are computed at delivery, one per nominal
    /// release due, on every port. An externally released task's are distinct only where its
    /// source's catalog record states that arrivals while its request is pending or claimed are
    /// counted — a fact no record states yet — and otherwise the record states that such an
    /// arrival, and its overrun, can be lost" — so an arrival a source coalesced never reaches this
    /// model, and its loss is the source's catalog record's to state, not the model's to detect.
    ///
    /// ⭐ **A release is also where an overrun is detected, and detection applies the policy**
    /// (the fault contract's rule 1) — see [`ReleaseEffect::Overrun`], which carries what the
    /// policy did. For a release that arrives inside a masked region, "detection" happens at its
    /// delivery, by [`Runtime::unmask`] or by a completion that closes the region
    /// ([`Runtime::complete`]). In this profile there is no other detection: "an overrun is
    /// detected by rule 1 alone" (rule 1a).
    ///
    /// ⭐ **AMENDED — the fault contract's rule 1, a release into a full latch.** This note used to
    /// read `⚠️ CONTRACT SILENT`: rule 1 defined detection as "a release for a task that still owes
    /// a **job**", and a task whose latch slot is already full may owe only an undelivered
    /// *release*. This model counted that as owing a job, so rule 3 then escalated it — a doubled
    /// arrival inside a critical section was fatal where the same doubling one instruction later
    /// was contained — and nothing in the fault contract settled which side of the definition a
    /// full latch fell on. The ruling of 2026-10-01 settled it on neither side, and rule 1 as now
    /// written states it for every arrival inside a region, not only the second:
    ///
    /// > Every release that arrives while a masked region is open is latched, and is observed —
    /// > for this rule and every other — only at its **delivery**, when the region closes. At
    /// > delivery a task's latched arrivals are judged in arrival order against the task's state
    /// > then: the first is fresh if the task owes no job (one that completed inside the region
    /// > included) and an overrun if it does; each later one is an overrun under `SkipLateJob`, and
    /// > is discarded under `Fault`, whose first overrun took the task out; the earlier arrivals a
    /// > hosted latch's mark stands for are judged as one. … So an overrun is not lost, nor fatal
    /// > for landing inside a region rather than one instruction after it — for arrivals the
    /// > platform delivers as distinct requests.
    ///
    /// So every release inside a region reports [`ReleaseEffect::Latched`] and changes nothing but
    /// the latch ([`Runtime::is_overrun_latched`]); no policy runs and no fault is recorded at
    /// arrival, and rule 3 never reaches it.
    ///
    /// Rule 1 adds an order this model cannot impose: "A completion and a release at one instant:
    /// the completion first (§13.4)." The Terms add one more: "A release observed while the job's
    /// completion path runs, before or after the completion is recorded, finds the task owing no
    /// job". The model has no instants and no completion path, so an adapter replaying either pair
    /// calls [`Runtime::complete`] first; the other order would be judged an overrun.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`] if the runtime is in its fatal handler — rule 7: "no release is
    /// processed or latched" — or [`Refused::UnknownTask`].
    pub fn release(&mut self, task: TaskId) -> Result<ReleaseEffect, Refused> {
        if self.halted() {
            return Err(Refused::Halted);
        }
        let index = self.index_of(task)?;

        if self.is_masked() {
            // Rule 1: "Every release that arrives while a masked region is open is latched, and
            // is observed … only at its delivery". A latch records the arrival and inspects the
            // task's lifecycle state not at all. A further arrival takes the held place as "the
            // most recent release", and the one it displaces becomes "a mark that an earlier one
            // also came" (Terms); a third or later joins the mark's earlier arrivals, which rule 1
            // judges "as one". The model keeps no identity per release, so the held place needs no
            // rewrite: only the mark is new.
            self.latch[index] = match self.latch[index] {
                Latch::Empty => Latch::Release,
                Latch::Release | Latch::ReleaseAndMark => Latch::ReleaseAndMark,
            };
            return Ok(ReleaseEffect::Latched);
        }

        match self.state[index] {
            TaskState::Stopped => Ok(ReleaseEffect::Stopped),
            // The task already owes a job. One slot per task, so there is nowhere to put a
            // second and nothing to silently drop it into — and the fault contract's rule 1 makes
            // this observation the fault itself, applied to this task whether or not it is running.
            // The release is the policy's: under SkipLateJob it makes the task ready for its next
            // job.
            TaskState::Ready | TaskState::Running => {
                let was_running = self.running == Some(task);
                let contained = self.contain_overrun(task, true);
                Ok(ReleaseEffect::Overrun(self.reschedule_after(
                    task,
                    was_running,
                    contained,
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
    /// Masking does **not** defer this. Rule 1 latches *arrivals*; a completion is the running
    /// task reaching the end of its own computation, not an asynchronous event, and deferring it
    /// would leave a finished job holding the processor.
    ///
    /// ⭐ **AMENDED — the fault contract's rule 4.** This note used to read `⚠️ CONTRACT SILENT`:
    /// whether a job may *complete* inside a masked region it opened was undecided, and rule 3
    /// sharpened the question without answering it. Its first ground — a job that holds the mask
    /// and ends "leaves the nesting depth above zero with no owner" — applied word for word to a
    /// job that merely *ends* there, but the rule scoped itself to "a containable **fault**", so
    /// this model let the completion through and left the mask at its depth, with a finished task
    /// still nominally its owner. The ruling of 2026-10-01 added a rule for it, which now reads:
    ///
    /// > A job may complete inside a masked region, and its completion closes it. The region is
    /// > the job's (Terms), so the nesting depth returns to zero with the job, and releases latched
    /// > in the region are delivered after the completion is recorded and before any task executes
    /// > an instruction of its own code. … A completion is not a fault … A latched release is
    /// > judged at that delivery (rule 1), so a task that completed inside the region is released
    /// > afresh, not overrun.
    ///
    /// So a completion is three steps in this order. The job ends and its task stands down; the
    /// depth goes to `0` however deep the nesting was, closing every section; and the latched
    /// releases are delivered exactly as [`Runtime::unmask`] delivers them — outside every masked
    /// region, so an overrun found there applies its declared policy unescalated — before the
    /// scheduler runs once. The completing task's own latched release therefore finds it
    /// [`TaskState::Completed`] and starts a fresh job instead of overrunning the one that just
    /// ended. Outside a masked region nothing is latched and this is the plain completion it
    /// always was.
    ///
    /// ⭐ **AMENDED — rule 4, the order of decision and delivery (the second review's finding
    /// 30, then R3).** Rule 4 used to say the latched releases are "delivered as at its closing
    /// `unmask`, and the schedule is decided after", which is the order above. It now gives the
    /// order to the port: "Whether the next scheduling decision precedes that delivery — the
    /// target: decided in the completion path, delivered when the following transition unmasks — or
    /// follows it — a hosted model — is the port's. Both reach the same schedule; a fault raised
    /// during the delivery is the completion path's, the completing task's, in the first and the
    /// trap path's in the second, and `M2.15` states how a trace shows each." This model keeps the
    /// hosted order, delivery then decision. What may differ between the two is the trace, which
    /// `M2.15` reads, and whose a fault raised during the delivery is; the schedule the event ends
    /// in does not, and the schedule is what this model is compared on.
    ///
    /// So a fault raised during this delivery is [`Context::TrapPath`]'s. The call is atomic, so an
    /// adapter raises it before the call or after it, and the trap path then records as interrupted
    /// the task the processor shows: the completing task before, the incoming one after.
    /// ⚠️ **CONTRACT SILENT — which task the trap path interrupts during a completion's delivery.**
    /// Rule 2 has it interrupt "the task whose job it preempted, or none if it preempted idle", and
    /// during this delivery the completing job has ended and the next is not yet decided; neither
    /// of the two points this model offers is inside the delivery, and the text names neither.
    ///
    /// ⭐ **This is why the method returns [`Unmasked`].** A completion inside a region can deliver
    /// releases and find overruns, which a bare [`Transition`] cannot express, so it is reported in
    /// the record the closing `unmask` already uses. Outside a region the record is `depth: 0`,
    /// nothing delivered, and `transition: Some(..)` naming the completing task as outgoing. It
    /// never halts: rule 4 says "A completion is not a fault", and the delivery it performs never
    /// halts either (see [`Unmasked`]).
    ///
    /// The Terms say why the region needs no owner check here: "**Only a job changes the depth** …
    /// So every section open at a completion is the completing job's." In this model the
    /// processor's occupant cannot change inside a region by any route — arrivals latch, and every
    /// fault there halts — so whoever completes inside one held the processor when it opened
    /// ([`Violation::JobStartedInsideMaskedRegion`] checks it).
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], or [`Refused::NoTaskRunning`] if the processor is idle.
    pub fn complete(&mut self) -> Result<Unmasked, Refused> {
        if self.halted() {
            return Err(Refused::Halted);
        }
        let outgoing = self.running.ok_or(Refused::NoTaskRunning)?;
        self.state[outgoing.index()] = TaskState::Completed;
        self.running = None;
        // The fault contract's rule 4: "the nesting depth returns to zero with the job" — every
        // section it opened, not one level — and only then are the latched releases delivered, so
        // that delivery is outside every masked region exactly as rule 3's note requires.
        self.mask_depth = 0;
        self.holder = None;
        Ok(self.deliver_latched(Some(outgoing)))
    }

    /// Raise the mask depth by one; at `0 → 1` a masked region opens, and releases arriving from
    /// then on are latched rather than delivered.
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
    /// is a race the contract asks to be modelled rather than assumed away. The count is bounded
    /// by [`Runtime::MAX_MASK_DEPTH`], and a `mask` past it is an assertion failure that halts —
    /// [`MaskEffect::Fatal`], where this model used to refuse.
    ///
    /// The fault contract's Terms make the region a job's: "opened by a job's outermost `mask`,
    /// kernel or application", and "A `mask` or `unmask` is a job's only when the job's own code,
    /// or a primitive it called, executes it". The caller is the running job, directly or through a
    /// primitive it called — "A task's job is that task, including a runtime primitive it called"
    /// (rule 2) — and that job is the region's holder.
    ///
    /// ⭐ **AMENDED — the fault contract's Terms, "Only a job changes the depth" (the second
    /// review's finding 35).** This note used to read
    /// `⚠️ CONTRACT SILENT — a mask with the processor idle`: the Terms defined the region as
    /// opened by "a job's outermost `mask`" and listed the contexts that never change the depth —
    /// "no service, trap path or transition" — and idle was in neither list. This model accepted
    /// it: the depth rose, arrivals latched as in a job's region, and the closing `unmask`
    /// delivered them, from a region with no holder. Refusing was the alternative, and the second
    /// decision's own argument told against it. The Terms decided it the third way, an assertion
    /// failure, and as rewritten after R3 they read:
    ///
    /// > **Only a job changes the depth.** A `mask` or `unmask` is a job's only when the job's own
    /// > code, or a primitive it called, executes it. Executed by any other context — a service,
    /// > the trap path, a transition, idle, initialisation or the fatal handler — it is an
    /// > assertion failure of that context, raised before the depth changes, whether or not a job
    /// > is preempted beneath it. A runtime that is told no context detects the case it can see, a
    /// > call with no job running; a port detects the rest from its own state …
    ///
    /// So with the processor idle this is [`MaskEffect::Fatal`], attributed to idle — the
    /// executing context, "no task" by rule 2 — and the depth is left at zero.
    ///
    /// ⭐ **AMENDED — a `mask` executed by a service, the trap path or a transition while a job is
    /// held as running (R3).** This note used to read `⚠️ CONTRACT SILENT`: "Only a job changes the
    /// depth" excluded it, but the assertion failure the Terms attached was stated only for one
    /// executed "with no job running". The Terms now state it for every context that is no job,
    /// "whether or not a job is preempted beneath it", and divide the detecting: "A runtime that is
    /// told no context detects the case it can see, a call with no job running; a port detects the
    /// rest from its own state". This model's `mask` takes no context — it is always the processor
    /// occupant's, the running job's or idle's — so it is the first kind of runtime, and detects
    /// exactly the case it detected before. An adapter for a port that raises the rest replays the
    /// port's assertion failure with [`Runtime::raise_assertion_failure`] in that port's context,
    /// rather than this `mask`.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`].
    pub fn mask(&mut self) -> Result<MaskEffect, Refused> {
        if self.halted() {
            return Err(Refused::Halted);
        }
        if self.running.is_none() {
            // The fault contract's Terms: a `mask` executed by a context that is no job "is an
            // assertion failure of that context, raised before the depth changes", and this model,
            // "told no context", detects "a call with no job running" — idle's, here. The depth is
            // left at zero.
            self.assertion_failure_in_a_primitive();
            return Ok(MaskEffect::Fatal);
        }
        if self.mask_depth == Self::MAX_MASK_DEPTH {
            // The fault contract's second smaller decision: an assertion failure, not a wrap, a
            // saturation or a refusal. The depth stays at the bound as evidence.
            self.assertion_failure_in_a_primitive();
            return Ok(MaskEffect::Fatal);
        }
        if self.mask_depth == 0 {
            self.holder = self.running;
        }
        self.mask_depth += 1;
        Ok(MaskEffect::Masked(self.mask_depth))
    }

    /// Lower the mask depth by one; at `1 → 0` the masked region closes and its latched releases
    /// are delivered.
    ///
    /// At depth `1 → 0` every latched release is delivered and **then** the scheduler runs once;
    /// see [`Unmasked::transition`] for why. An inner unmask of a nested pair delivers nothing. An
    /// `unmask` executed with no job running, or at depth zero, is an assertion failure that halts
    /// — [`UnmaskEffect::Fatal`], where this model used to refuse the second and had no first.
    ///
    /// ⭐ **Delivery is outside the masked region, and the fault contract's rule 3 says so.** The
    /// depth reaches zero first and the latched releases are serviced after it: "Delivery is
    /// **not** inside a masked region, so an overrun found at delivery applies its ordinary policy
    /// — the difference between a profile that can contain an overrun and one that cannot." A
    /// runtime that delivered *inside* the region would escalate every latched overrun to fatal by
    /// rule 3, and containment would exist only on paper.
    ///
    /// A delivered release that finds its task still owing a job is therefore an overrun whose
    /// declared policy applies here (the fault contract's rule 1). If that policy takes the
    /// processor away from the faulting task, the scheduler still runs only once, at the end: the
    /// whole unmask reports one [`Transition`]. No policy halts (rule 5), so the delivery always
    /// runs to the end.
    ///
    /// A task whose latch also holds the mark ([`Runtime::is_overrun_latched`]) has two arrivals to
    /// judge, and rule 1 orders them: "At delivery a task's latched arrivals are judged in arrival
    /// order against the task's state then". The mark's arrivals came first, so they are judged
    /// first — "the earlier arrivals a hosted latch's mark stands for are judged as one" — and the
    /// held release,
    /// the most recent, after them. Both are judged before the next task, so the pair ends as the
    /// same two arrivals would one instruction after the region closed: "not lost, nor fatal for
    /// landing inside a region rather than one instruction after it". Only the transition count
    /// differs, by the single-transition rule above.
    ///
    /// ⭐ **AMENDED — rule 1, the order across tasks (the second review's finding 42).** This note
    /// used to read `⚠️ CONTRACT SILENT`: rule 1 ordered one task's arrivals, not different tasks'.
    /// It now says: "Across tasks the order changes no task's state and is the port's (`M2.15`
    /// traces it)." This model, as its own port, takes tasks in ascending rank, on the ground it
    /// gave before: every latched release is observed at one instant — the closing — so they are
    /// coincident, which the priority-direction record orders: "A release trace emits coincident
    /// releases in ascending `N`". The order changes no task's state or the transition; it shows
    /// only in which overrun [`Runtime::first_contained_overrun`] names first.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`].
    pub fn unmask(&mut self) -> Result<UnmaskEffect, Refused> {
        if self.halted() {
            return Err(Refused::Halted);
        }
        if self.running.is_none() {
            // The fault contract's Terms: an `unmask` executed by a context that is no job "is an
            // assertion failure of that context, raised before the depth changes", and this model,
            // "told no context", detects "a call with no job running" — whatever the depth. On
            // every state this model reaches, no job running means depth zero, so the check below
            // would halt too; this is the Terms' own condition, checked first so that it does not
            // depend on that.
            self.assertion_failure_in_a_primitive();
            return Ok(UnmaskEffect::Fatal);
        }
        if self.mask_depth == 0 {
            // The fault contract's second smaller decision: "A `mask` that would raise the depth
            // past `M`, and an `unmask` at depth zero, are **assertion failures**".
            self.assertion_failure_in_a_primitive();
            return Ok(UnmaskEffect::Fatal);
        }
        self.mask_depth -= 1;
        if self.mask_depth > 0 {
            return Ok(UnmaskEffect::Unmasked(Unmasked {
                depth: self.mask_depth,
                delivered: 0,
                overruns: 0,
                stopped: 0,
                transition: None,
            }));
        }
        self.holder = None;
        Ok(UnmaskEffect::Unmasked(self.deliver_latched(None)))
    }

    /// Raise an overrun against `task` without a release.
    ///
    /// ⛔ **Outside `rt-static-up-v1`.** The fault contract's rule 1a:
    ///
    /// > An overrun raised without a release is outside this profile. `rt-static-up-v1` has no
    /// > execution-budget monitor: an overrun is detected by rule 1 alone, as rule 6 says of a
    /// > deadline. A port that raises one another way — a monitor's interrupt, or a bound checked
    /// > synchronously — is outside the profile. A later profile that admits one must say how its
    /// > source is declared (§3.1, §7.3) and charged, and how its overrun is judged against the job
    /// > it measured.
    ///
    /// The entry is kept as the one such a later profile would need, so that an adapter for a port
    /// with a monitor can still drive this model. Its behaviour is the one this model gave it under
    /// the earlier text, except inside a masked region (below). Nothing the profile's contract
    /// decides is exercised through it: a comparison that calls it compares two models' choices
    /// outside the contract.
    ///
    /// ⭐ **AMENDED — rule 1a, twice.** This entry was `raise(task, Fault::Overrun)`, kept "for an
    /// overrun detected some other way, an execution-budget monitor being the obvious one (§7.3's
    /// 'execution bound')", and the fault contract said nothing of such an overrun. The first
    /// rewrite gave it a rule 1a stating what this model did — "the task's policy applies with no
    /// triggering release: under `SkipLateJob` the job is abandoned and the task's next job starts
    /// at its next release, and under `Fault` the task is faulted". The second review found the
    /// monitor an undeclared source, whose overruns inside a region two readers handled
    /// differently, and rule 1a now takes the monitor out of the profile.
    ///
    /// ⚠️ **CONTRACT SILENT — outside the profile, by the text's own design.** A later profile is
    /// to say "how its overrun is judged against the job it measured". This model judges it
    /// against whatever job the task owes when the call is made ([`Refused::NoJobOwed`] if none):
    /// under [`OverrunAction::TerminateJob`] that job is abandoned and, with no release to start
    /// the next, the task is left [`TaskState::Completed`] to await one; under
    /// [`OverrunAction::StopTask`] it is [`TaskState::Stopped`]. A stale monitor overrun, measured
    /// against a job that has since completed and been replaced, would here abandon the
    /// replacement.
    ///
    /// The overrun is attributed to `task` "whichever context holds the processor" (rule 2), so
    /// `task` may be ready rather than running, and containing it then moves no context: the
    /// effect's transition is `None`. It ends the run's timing claims if it is the first contained
    /// one ([`Runtime::first_contained_overrun`]).
    ///
    /// **Inside a masked region** it is not containable, whichever task it names: it enters the
    /// fatal handler with [`FaultRecord::escalated`] set. That is rule 3, which this entry reaches
    /// and the profile does not: "In `rt-static-up-v1` no containable fault is raised inside a
    /// masked region … so this rule decides nothing observable in this profile; it fixes the answer
    /// a later profile that admits one starts from."
    ///
    /// ⭐ **AMENDED — rule 3 reaches every task again.** Its grounds: terminating a job that
    /// *holds* the region "leaves the nesting depth above zero with no owner, so interrupts never
    /// return; forcing the depth to zero re-enables them inside a region whose invariants the
    /// faulting job was partway through restoring"; and "containment means **resuming the
    /// schedule** from a state the region had not finished making consistent, which is what a
    /// kernel critical section exists to prevent. That holds whichever task the fault is
    /// attributed to, not only the one holding the region." The rule adds: "Ground 1 covers the
    /// task holding the region; ground 2 extends the rule to every task." Before the first rewrite
    /// this model escalated both cases, and a test called the second
    /// `a_containable_fault_of_a_task_that_holds_no_mask_escalates_too`. The first rewrite's rule 1a
    /// then said "A monitor is an interrupt source, masked with the region, so an overrun of
    /// another task cannot be raised inside a masked region; one raised there is the region
    /// holder's own", and this model refused another task's with `Refused::MonitorMasked`. That
    /// sentence is gone — how a monitor is declared is now a later profile's to say — and ground 2
    /// stands, so the refusal has no text left under it: another task's overrun raised inside a
    /// region escalates as the holder's does, and the variant is removed. The record attributes it
    /// to the overrunning task (rule 2) and names no interrupted task, since the context that
    /// raised it is the later profile's monitor, which the text does not yet describe.
    ///
    /// Delivery is explicitly *outside* the masked region and does not escalate; see
    /// [`Runtime::unmask`]. Nor does delivery at a completion that closes the region, and the
    /// completion itself is not a fault at all (rule 4); see [`Runtime::complete`].
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`], [`Refused::UnknownTask`], or [`Refused::NoJobOwed`].
    pub fn raise_overrun(&mut self, task: TaskId) -> Result<FaultEffect, Refused> {
        if self.halted() {
            return Err(Refused::Halted);
        }
        let index = self.index_of(task)?;
        if !matches!(self.state[index], TaskState::Ready | TaskState::Running) {
            return Err(Refused::NoJobOwed);
        }
        if self.is_masked() {
            // Rule 3: ground 1 for the region's holder, ground 2 for every other task.
            self.enter_fatal_handler(FaultRecord {
                fault: Fault::Overrun,
                class: Fault::Overrun.class(),
                attribution: Attribution::Task(task),
                escalated: true,
            });
            return Ok(FaultEffect::Fatal);
        }
        let was_running = self.running == Some(task);
        let contained = self.contain_overrun(task, false);
        Ok(self.reschedule_after(task, was_running, contained))
    }

    /// Raise an unexpected trap in `context`. It is not containable (the fault contract's table),
    /// so the runtime enters the fatal handler and halts (rule 7).
    ///
    /// The three synchronous faults are raised the same way, and this note is theirs. The fault
    /// contract's rule 2 attributes each to "the context that raises them": a [`Context::Task`]
    /// names the task whose job — or a primitive it called, or the completion path it entered —
    /// raised it, and must be the running task; every other [`Context`] is no task, and the
    /// record names it and keeps "the task it interrupted, if any" (rule 7) as interrupted, "never
    /// as attributed" — the task holding the processor when the fault is raised, or none (see
    /// [`Context`] and [`Attribution`]). The model refuses rather than re-attributes, because a
    /// misattribution is the interesting thing to catch.
    ///
    /// None of the three is ever escalated: rule 3 escalates a *containable* fault, and these
    /// never were. Raised inside a masked region they halt all the same, with
    /// [`FaultRecord::escalated`] false.
    ///
    /// # Errors
    ///
    /// [`Refused::Halted`] — rule 7, "A fault taken in the handler ends it at once in its terminal
    /// state and never replaces the preserved one" — [`Refused::UnknownTask`], or a context the
    /// processor contradicts:
    /// [`Refused::NoTaskRunning`], [`Refused::NotTheRunningTask`], [`Refused::NotIdle`].
    pub fn raise_unexpected_trap(&mut self, context: Context) -> Result<FaultEffect, Refused> {
        self.raise_synchronous(context, Fault::UnexpectedTrap)
    }

    /// Raise a stack-guard fault in `context`; `guard` names whose guard was hit (the fault
    /// contract's rule 2: "a task's, or the interrupt stack's"). Not containable: the runtime halts
    /// (rule 7). See [`Runtime::raise_unexpected_trap`] for attribution, and [`Guard`] for which
    /// pairings are admitted.
    ///
    /// # Errors
    ///
    /// As [`Runtime::raise_unexpected_trap`], and [`Refused::UnknownTask`] if `guard` names a task
    /// outside the set.
    pub fn raise_stack_guard(
        &mut self,
        context: Context,
        guard: Guard,
    ) -> Result<FaultEffect, Refused> {
        if let Guard::Task(owner) = guard {
            if !self.halted() {
                self.index_of(owner)?;
            }
        }
        self.raise_synchronous(context, Fault::StackGuard(guard))
    }

    /// Raise an assertion failure in `context`. Not containable: the runtime halts (rule 7). See
    /// [`Runtime::raise_unexpected_trap`] for attribution. (The model raises this fault itself for
    /// an unbalanced `mask` or `unmask`; see [`MaskEffect::Fatal`] and [`UnmaskEffect::Fatal`].)
    ///
    /// # Errors
    ///
    /// As [`Runtime::raise_unexpected_trap`].
    pub fn raise_assertion_failure(&mut self, context: Context) -> Result<FaultEffect, Refused> {
        self.raise_synchronous(context, Fault::AssertionFailure)
    }

    // -- invariants ----------------------------------------------------------------------------

    /// Check the structural invariants of the fixed-priority ready structure and of the masked
    /// region.
    ///
    /// These are the properties §7.1 calls a "model proof" candidate — "A state-machine invariant
    /// is machine checked" — and they are the ones an adapter should assert after *every* event on
    /// both this model and the implementation it drives. A model that agrees event-by-event on
    /// which task runs but silently loses the invariant in between is agreeing by luck.
    ///
    /// A halted runtime is exempt. Once the fatal handler has run this is no longer a schedule but
    /// evidence, and the fault contract's rule 7 disclaims the table as evidence of anything
    /// scheduling-shaped: "What the task table shows afterwards is the implementation's, and is not
    /// evidence of attribution." (The exemption used to have a second reason — a halt partway
    /// through a delivery, under the halting policy rule 5 removed, could leave a ready task
    /// outranking the one the processor stopped on. This model's halts now all fall between events,
    /// so its frozen state happens to pass; the exemption stays because the contract says the table
    /// proves nothing.)
    ///
    /// # Errors
    ///
    /// The first [`Violation`] found.
    pub fn check_invariants(&self) -> Result<(), Violation> {
        if self.halted() {
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
        // The fault contract's Terms: "Only a job changes the depth", so every section open is the
        // job's that holds the processor — a region with no job holding it, or held by another task
        // than the one running, is a job started inside it or a depth changed by something that is
        // no job.
        if self.is_masked() && (self.holder.is_none() || self.holder != self.running) {
            return Err(Violation::JobStartedInsideMaskedRegion {
                holder: self.holder,
                running: self.running,
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

    const fn halted(&self) -> bool {
        self.fatal.is_some()
    }

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
    ///
    /// Every job starts, or resumes, here — and always at depth zero, so the Terms' assertion
    /// failure, "A depth above zero when a job is dispatched or resumed", is never raised: the
    /// callers are an
    /// unmasked release, a delivery after the region closed, and a contained overrun, which is
    /// contained only outside every region.
    fn take_processor(&mut self) -> Option<(Option<TaskId>, TaskId)> {
        debug_assert!(!self.is_masked(), "no job starts at a depth above zero");
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

    /// The running task has given up the processor (its job skipped or the task stopped); hand
    /// it to whoever is next.
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

    /// Enter the fatal handler (the fault contract's rule 7): keep the first such fault and only
    /// the first, and halt. Everything else is frozen where it stands — the task states, the
    /// latches, the depth and the running task — because a fatal handler that tidied up would be
    /// destroying the evidence §8.1 asks it to preserve.
    fn enter_fatal_handler(&mut self, record: FaultRecord) {
        if self.fatal.is_none() {
            self.fatal = Some(record);
        }
    }

    /// An assertion failure raised by `mask` or `unmask` itself: past the declared depth or at
    /// depth zero (the fault contract's second smaller decision), or with no job running (the
    /// Terms). Rule 2 attributes it to the executing context, which for these two is the running
    /// job, whose primitive they are, or — for "a call with no job running", which the Terms make
    /// "an assertion failure of that context" — idle, the context this model, "told no context",
    /// takes to be executing.
    fn assertion_failure_in_a_primitive(&mut self) {
        let attribution = match self.running {
            Some(task) => Attribution::Task(task),
            None => Attribution::NoTask {
                context: Context::Idle,
                interrupted: None,
            },
        };
        self.enter_fatal_handler(FaultRecord {
            fault: Fault::AssertionFailure,
            class: Fault::AssertionFailure.class(),
            attribution,
            escalated: false,
        });
    }

    /// Attribute a synchronous fault to `context` (the fault contract's rule 2), or refuse a
    /// context the processor contradicts.
    fn attribute(&self, context: Context) -> Result<Attribution, Refused> {
        match context {
            Context::Task(task) => {
                self.index_of(task)?;
                match self.running {
                    None => Err(Refused::NoTaskRunning),
                    Some(running) if running != task => Err(Refused::NotTheRunningTask),
                    Some(_) => Ok(Attribution::Task(task)),
                }
            }
            Context::Idle => match self.running {
                Some(_) => Err(Refused::NotIdle),
                None => Ok(Attribution::NoTask {
                    context,
                    interrupted: None,
                }),
            },
            Context::Service | Context::TrapPath | Context::Transition => Ok(Attribution::NoTask {
                context,
                interrupted: self.running,
            }),
        }
    }

    /// One of the three synchronous faults: attributed by rule 2, never containable, so it enters
    /// the fatal handler (rule 7), unescalated.
    fn raise_synchronous(
        &mut self,
        context: Context,
        fault: Fault,
    ) -> Result<FaultEffect, Refused> {
        if self.halted() {
            return Err(Refused::Halted);
        }
        let attribution = self.attribute(context)?;
        self.enter_fatal_handler(FaultRecord {
            fault,
            class: fault.class(),
            attribution,
            escalated: false,
        });
        Ok(FaultEffect::Fatal)
    }

    /// Apply `task`'s declared policy to an overrun outside every masked region, and move its
    /// lifecycle state (the fault contract's rules 1 and 5). `triggered` says whether a release
    /// detected it, which in this profile it always did; `false` is the entry outside it
    /// ([`Runtime::raise_overrun`]).
    ///
    /// It deliberately does **not** touch the processor. Who runs next is the scheduler's
    /// business, and separating the two is what lets a delivery apply a policy to the running task
    /// partway through and still report one transition for the whole of it.
    ///
    /// Under `SkipLateJob` the owed job is abandoned and the triggering release is the task's next
    /// job — rule 1, "under `SkipLateJob` it makes the task ready for its next job" — so the task
    /// is ready again; without one (outside the profile) the task owes nothing until its next
    /// release.
    /// Under `Fault` the task leaves the schedule and the triggering release goes with it (rule
    /// 5).
    fn contain_overrun(&mut self, task: TaskId, triggered: bool) -> Contained {
        debug_assert!(
            !self.is_masked(),
            "rule 3: no overrun is contained inside a masked region"
        );
        if self.first_contained_overrun.is_none() {
            self.first_contained_overrun = Some(task);
        }
        let index = task.index();
        match self.spec[index].on_overrun {
            OverrunAction::TerminateJob => {
                self.state[index] = if triggered {
                    TaskState::Ready
                } else {
                    TaskState::Completed
                };
                Contained::JobSkipped
            }
            OverrunAction::StopTask => {
                self.state[index] = TaskState::Stopped;
                Contained::TaskFaulted
            }
        }
    }

    /// Turn a contained overrun into its [`FaultEffect`], running the scheduler only if `task`
    /// held the processor before the policy applied.
    ///
    /// If it did not — the case of the fault contract's rule 2 — nothing moves and the effect
    /// carries no transition. A task skipped under `SkipLateJob` by a release is
    /// [`TaskState::Ready`] again by now, so when it held the processor the scheduler hands the
    /// processor back to it: nothing ready can outrank a task that was running with releases being
    /// delivered, and the handover from the late job to the fresh one is a [`Transition::Switch`]
    /// from the task to itself.
    fn reschedule_after(
        &mut self,
        task: TaskId,
        was_running: bool,
        contained: Contained,
    ) -> FaultEffect {
        let transition = if was_running {
            Some(self.vacate(task))
        } else {
            None
        };
        match contained {
            Contained::JobSkipped => FaultEffect::JobTerminated(transition),
            Contained::TaskFaulted => FaultEffect::TaskStopped(transition),
        }
    }

    /// Deliver every latched arrival, then run the scheduler once — the closing half of
    /// [`Runtime::unmask`] at depth `1 → 0`, and of [`Runtime::complete`] by the fault contract's
    /// rule 4.
    ///
    /// The caller has already brought the depth to zero, so everything here is outside every
    /// masked region and no overrun found here escalates (the note to the fault contract's rule 3).
    /// `stood_down` is a task that gave up the processor just before — the completing one — so the
    /// one transition still names it as outgoing.
    ///
    /// Tasks are taken in ascending rank — rule 1 makes the order across tasks "the port's" — and
    /// each task's arrivals in the order they arrived: the earlier arrivals the mark stands for,
    /// "judged as one", then the most recent, which the latch holds. Each is judged against the
    /// task's state at that moment, exactly as [`Runtime::release`] would judge it unmasked. If the
    /// first left the task owing a job, the held release is an overrun, and under `SkipLateJob` it
    /// is the one that "makes the task ready for its next job, with that release's nominal
    /// instant"; if the first was the overrun that took the task out under `Fault`, the held
    /// release "is discarded" (rule 1; see [`Unmasked::overruns`] for the reading).
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
            // Take one arrival off the latch, the earliest first: the mark's, then the held one.
            // A marked latch still holds its most recent release afterwards, so the same task
            // comes round again before any lower rank.
            self.latch[index] = match self.latch[index] {
                Latch::ReleaseAndMark => Latch::Release,
                Latch::Release | Latch::Empty => Latch::Empty,
            };
            match self.state[index] {
                TaskState::Stopped => stopped += 1,
                TaskState::Ready | TaskState::Running => {
                    overruns += 1;
                    let was_running = self.running == Some(task);
                    // The fault contract's rule 1, with the release the policy's. The task stands
                    // down rather than being rescheduled now, so that the scheduler runs once for
                    // the whole delivery and its one transition still names the task that stood
                    // down.
                    self.contain_overrun(task, true);
                    if was_running {
                        self.running = None;
                        stood_down = Some(task);
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

/// Which of rule 5's two policies contained an overrun, before the scheduler has had its say.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Contained {
    /// `SkipLateJob`.
    JobSkipped,
    /// `Fault`.
    TaskFaulted,
}

/// What one task's latch holds.
///
/// Three states and no more, because the fault contract's Terms give a hosted model's latch
/// exactly two things to keep: "the runtime's own record: the most recent release, and a mark that
/// an earlier one also came". A queue would be the `general-ipc` §3.1 excludes, and a counter of
/// arrivals would keep what rule 1 says is not kept: "the earlier arrivals a hosted latch's mark
/// stands for are judged as one". (This model had reached the same shape before the first rewrite,
/// by arguing that under each policy the state after delivering two arrivals is the state after
/// delivering more; the Terms now state it. The second rewrite moved which release the latch
/// *holds* — the most recent rather than the first — which a model with no instants cannot tell
/// apart; it shows only in whose nominal instant a skipped task's next job carries.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Latch {
    /// Nothing pending.
    Empty,
    /// One release, judged at delivery against the task's state then.
    Release,
    /// The most recent release, and the mark that an earlier one came — judged at delivery mark
    /// first, as one arrival, then the release.
    ReleaseAndMark,
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
            Self::PriorityBelowOne { task, rank } => write!(
                f,
                "the task at index {} declares priority rank {rank}; the priority-direction \
                 decision record admits a rank N >= 1 and fixes 1 as the highest, and admitting \
                 a rank below 1 would move the top of the range by inference, which §15 forbids",
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
                "the runtime is in its fatal handler; from then no job runs, no release is \
                 processed or latched, and no later fault replaces the one preserved (the fault \
                 contract's rule 7, §8.1)"
            ),
            Self::NoTaskRunning => write!(
                f,
                "the processor is idle, and this operation needs a running task (§3.1: one \
                 execution context runs at a time)"
            ),
            Self::NotTheRunningTask => write!(
                f,
                "this fault is attributed to the context that raises it (the fault contract's \
                 rule 2), and the named task is not executing"
            ),
            Self::NotIdle => write!(
                f,
                "the fault names idle as the executing context, but a task holds the processor \
                 (the fault contract's rule 2)"
            ),
            Self::NoJobOwed => write!(
                f,
                "the named task owes no job, so it cannot be overrunning one; the fault \
                 contract's rule 1 defines an overrun in terms of a task that still owes a job"
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
            Self::JobStartedInsideMaskedRegion { holder, running } => write!(
                f,
                "a masked region opened by {:?} is open while {:?} holds the processor; only a \
                 job changes the depth, and a depth above zero when a job is dispatched or resumed \
                 is an assertion failure (the fault contract's Terms)",
                holder.map(TaskId::index),
                running.map(TaskId::index)
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

    /// Two tasks, `H` outranking `L`, with the given overrun policies.
    fn boot2(h: OverrunAction, l: OverrunAction) -> Runtime<2> {
        Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::HIGHEST,
                on_overrun: h,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: l,
            },
        ])
        .expect("the fixture is a valid static task set")
    }

    /// Two tasks, `H` outranking `L`, both skipping a late job on overrun.
    fn two_tasks() -> Runtime<2> {
        boot2(OverrunAction::TerminateJob, OverrunAction::TerminateJob)
    }

    fn assert_sound<const N: usize>(rt: &Runtime<N>) {
        rt.check_invariants().expect("invariants hold");
    }

    /// `mask`, expecting it to raise the depth.
    fn mask_ok<const N: usize>(rt: &mut Runtime<N>) -> u8 {
        match rt.mask().expect("the runtime has not halted") {
            MaskEffect::Masked(depth) => depth,
            MaskEffect::Fatal => panic!("the depth was below the bound"),
        }
    }

    /// `unmask`, expecting it to be balanced.
    fn unmask_ok<const N: usize>(rt: &mut Runtime<N>) -> Unmasked {
        match rt.unmask().expect("the runtime has not halted") {
            UnmaskEffect::Unmasked(unmasked) => unmasked,
            UnmaskEffect::Fatal => panic!("the unmask was balanced"),
        }
    }

    fn assertion_failure(attribution: Attribution) -> Option<FaultRecord> {
        Some(FaultRecord {
            fault: Fault::AssertionFailure,
            class: Class::ViolatedInternalInvariant,
            attribution,
            escalated: false,
        })
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

    #[test]
    fn ranks_need_not_be_contiguous() {
        // The priority record's item 3: "`(priority 1)`, `(priority 5)` and `(priority 9)` is a
        // valid description — and fixed priority uses only their order". Declared lowest first,
        // so declaration order and rank order disagree as well.
        const NINE: TaskId = TaskId::from_index(0);
        const FIVE: TaskId = TaskId::from_index(1);
        const ONE: TaskId = TaskId::from_index(2);
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "nine",
                priority: Priority::new(9),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "five",
                priority: Priority::new(5),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "one",
                priority: Priority::new(1),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .expect("a gap between ranks is admissible");
        assert_eq!(
            rt.release(NINE).unwrap(),
            ReleaseEffect::Dispatched(Transition::Dispatch { incoming: NINE })
        );
        assert_eq!(
            rt.release(FIVE).unwrap(),
            ReleaseEffect::Preempted(Transition::Switch {
                outgoing: NINE,
                incoming: FIVE,
            })
        );
        assert_eq!(
            rt.release(ONE).unwrap(),
            ReleaseEffect::Preempted(Transition::Switch {
                outgoing: FIVE,
                incoming: ONE,
            })
        );
        assert_sound(&rt);
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
    fn boot_refuses_every_rank_below_one() {
        // The priority record's item 1: "A rank is an integer `N ≥ 1`", and `archogen check`
        // refuses "any rank below 1". Zero and the negative ranks a 64-bit signed integer holds
        // are all refused, whichever task declares them.
        for rank in [0, -1, i64::MIN] {
            let err = Runtime::boot([TaskSpec {
                name: "a",
                priority: Priority::new(rank),
                on_overrun: OverrunAction::TerminateJob,
            }])
            .unwrap_err();
            assert_eq!(err, BootError::PriorityBelowOne { task: H, rank });

            let err = Runtime::boot([
                TaskSpec {
                    name: "a",
                    priority: Priority::HIGHEST,
                    on_overrun: OverrunAction::TerminateJob,
                },
                TaskSpec {
                    name: "b",
                    priority: Priority::new(rank),
                    on_overrun: OverrunAction::TerminateJob,
                },
            ])
            .unwrap_err();
            assert_eq!(err, BootError::PriorityBelowOne { task: L, rank });
        }
    }

    #[test]
    fn the_admitted_ranks_reach_both_extremes_of_a_64_bit_signed_integer() {
        // Item 1: a rank is "any such integer the language's integer type holds, a 64-bit signed
        // one". The top of the range is 1; its bottom is the integer's largest value, which boots
        // and is outranked by everything else.
        const BOTTOM: TaskId = TaskId::from_index(0);
        const NEXT: TaskId = TaskId::from_index(1);
        const TOP: TaskId = TaskId::from_index(2);
        let spec = |name, rank| TaskSpec {
            name,
            priority: Priority::new(rank),
            on_overrun: OverrunAction::TerminateJob,
        };
        assert_eq!(Priority::new(i64::MAX).rank(), i64::MAX);
        assert!(Priority::HIGHEST.outranks(Priority::new(i64::MAX)));
        assert!(Priority::new(i64::MAX - 1).outranks(Priority::new(i64::MAX)));
        assert!(!Priority::new(i64::MAX).outranks(Priority::new(i64::MAX - 1)));
        assert!(!Priority::new(i64::MAX).outranks(Priority::new(i64::MAX)));

        let mut rt = Runtime::boot([
            spec("bottom", i64::MAX),
            spec("next", i64::MAX - 1),
            spec("top", 1),
        ])
        .expect("every rank from 1 to i64::MAX is admissible");
        assert_eq!(
            rt.release(BOTTOM).unwrap(),
            ReleaseEffect::Dispatched(Transition::Dispatch { incoming: BOTTOM })
        );
        assert_eq!(
            rt.release(NEXT).unwrap(),
            ReleaseEffect::Preempted(Transition::Switch {
                outgoing: BOTTOM,
                incoming: NEXT,
            })
        );
        assert_eq!(
            rt.release(TOP).unwrap(),
            ReleaseEffect::Preempted(Transition::Switch {
                outgoing: NEXT,
                incoming: TOP,
            })
        );
        assert_sound(&rt);

        // Uniqueness holds at the extreme as anywhere: two tasks at i64::MAX are refused, with the
        // rank carried whole.
        assert_eq!(
            Runtime::boot([spec("a", i64::MAX), spec("b", i64::MAX)]).unwrap_err(),
            BootError::DuplicatePriority {
                rank: i64::MAX,
                first: TaskId::from_index(0),
                second: TaskId::from_index(1),
            }
        );
        // And the other extreme of the integer is below one.
        assert_eq!(
            Runtime::boot([spec("a", 1), spec("b", i64::MIN)]).unwrap_err(),
            BootError::PriorityBelowOne {
                task: TaskId::from_index(1),
                rank: i64::MIN,
            }
        );
    }

    #[test]
    fn every_task_starts_created_and_nothing_runs() {
        let rt = two_tasks();
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.state(L), Some(TaskState::Created));
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.mask_depth(), 0);
        assert_eq!(rt.fatal_record(), None);
        assert_eq!(rt.first_contained_overrun(), None);
        assert_sound(&rt);
    }

    #[test]
    fn an_id_outside_the_static_set_is_refused() {
        let mut rt = two_tasks();
        let stranger = TaskId::from_index(7);
        assert_eq!(rt.release(stranger).unwrap_err(), Refused::UnknownTask);
        assert_eq!(
            rt.raise_overrun(stranger).unwrap_err(),
            Refused::UnknownTask
        );
        assert_eq!(
            rt.raise_unexpected_trap(Context::Task(stranger))
                .unwrap_err(),
            Refused::UnknownTask
        );
        rt.release(L).unwrap();
        assert_eq!(
            rt.raise_stack_guard(Context::Task(L), Guard::Task(stranger))
                .unwrap_err(),
            Refused::UnknownTask
        );
        assert_eq!(rt.fatal_record(), None, "a refusal is not a fault");
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
        // The fault contract's rule 1: the observation *is* the fault, and L's declared
        // TerminateJob (the contract's SkipLateJob) applies here rather than waiting for a separate
        // call. The late job is skipped and the triggering release "makes the task ready for its
        // next job", so L hands the processor from its late job to its fresh one: one switch, from
        // L to L.
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::JobTerminated(Some(Transition::Switch {
                outgoing: L,
                incoming: L,
            })))
        );
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.first_contained_overrun(), Some(L));
        assert_eq!(
            rt.fatal_record(),
            None,
            "contained, so the fatal handler never ran"
        );
        assert_sound(&rt);
        // There is exactly one job — the fresh one — so one completion leaves L at rest.
        assert_eq!(
            rt.complete().unwrap().transition,
            Some(Transition::ToIdle { outgoing: L })
        );
        assert_eq!(rt.complete().unwrap_err(), Refused::NoTaskRunning);
    }

    #[test]
    fn under_fault_the_triggering_release_goes_with_the_faulted_task() {
        // The fault contract's rule 1: "under `Fault` it goes with the faulted task"; rule 5: "the
        // triggering release and every later release of it are discarded". Nothing is left ready
        // and nothing runs.
        let mut rt = boot2(OverrunAction::TerminateJob, OverrunAction::StopTask);
        rt.release(L).unwrap();
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::TaskStopped(Some(Transition::ToIdle {
                outgoing: L
            })))
        );
        assert_eq!(rt.state(L), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Stopped);
        assert_eq!(rt.processor(), Processor::Idle);
        assert_sound(&rt);
    }

    #[test]
    fn under_fault_every_other_task_continues_and_the_runtime_does_not_halt() {
        // Rule 5: "every other task continues, and the runtime does not halt". H declares Fault
        // and overruns while running; L, which H had preempted, takes over and carries on.
        let mut rt = boot2(OverrunAction::StopTask, OverrunAction::TerminateJob);
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(
            rt.release(H).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::TaskStopped(Some(Transition::Switch {
                outgoing: H,
                incoming: L,
            })))
        );
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.fatal_record(), None);
        assert_eq!(
            rt.complete().unwrap().transition,
            Some(Transition::ToIdle { outgoing: L })
        );
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Dispatched(Transition::Dispatch { incoming: L })
        );
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Stopped);
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_is_attributed_to_the_overrunning_task_not_the_running_one() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        // The fault contract's rule 2: the overrunning task is "the task a release … belongs to —
        // whichever context holds the processor: a task that is ready and not running" among them.
        // Containing L's fault moves no context, because L was not holding any.
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::JobTerminated(None))
        );
        // Rule 1: L's late job is skipped and this release is its next job, so L is still ready —
        // with a fresh job now, still waiting behind H.
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.first_contained_overrun(), Some(L));
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_policy_of_stop_removes_a_ready_task_without_a_context_switch() {
        let mut rt = boot2(OverrunAction::TerminateJob, OverrunAction::StopTask);
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

    // -- masking -------------------------------------------------------------------------------

    #[test]
    fn a_release_arriving_while_masked_is_latched_and_takes_effect_at_unmask() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(rt.mask().unwrap(), MaskEffect::Masked(1));
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        // F15: pending, not delivered. L keeps the processor because a masked region is not one
        // of §3.1's "supported interrupt points".
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.is_latched(H), Some(true));
        assert_sound(&rt);

        let lifted = unmask_ok(&mut rt);
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
        rt.release(L).unwrap();
        assert_eq!(mask_ok(&mut rt), 1);
        assert_eq!(mask_ok(&mut rt), 2);
        rt.release(H).unwrap();

        let inner = unmask_ok(&mut rt);
        assert_eq!(inner.depth, 1);
        assert_eq!(inner.delivered, 0);
        assert_eq!(inner.transition, None);
        assert_eq!(rt.processor(), Processor::Running(L));
        assert!(rt.is_masked());

        let outer = unmask_ok(&mut rt);
        assert_eq!(outer.depth, 0);
        assert_eq!(outer.delivered, 1);
        assert_eq!(
            outer.transition,
            Some(Transition::Switch {
                outgoing: L,
                incoming: H,
            })
        );
        assert_sound(&rt);
    }

    #[test]
    fn many_latched_releases_cost_one_transition_and_the_winner_is_the_highest_priority() {
        // A third task, C, below both, holds the region. (This test used to let idle open it, the
        // `mask` with no job running that the Terms now make an assertion failure.)
        const C: TaskId = TaskId::from_index(2);
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "H",
                priority: Priority::new(1),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "L",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "C",
                priority: Priority::new(3),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.release(C).unwrap();
        mask_ok(&mut rt);
        rt.release(L).unwrap();
        rt.release(H).unwrap();

        let lifted = unmask_ok(&mut rt);
        assert_eq!(lifted.delivered, 2);
        // One transition, not two: the ready set is completed before the scheduler runs, so the
        // middle task is never switched to only to be switched away from.
        assert_eq!(
            lifted.transition,
            Some(Transition::Switch {
                outgoing: C,
                incoming: H,
            })
        );
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.state(C), Some(TaskState::Ready));
        assert_sound(&rt);
    }

    #[test]
    fn a_latched_release_for_a_task_that_still_owes_a_job_applies_its_policy_at_delivery() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        // L's latch slot was empty, so this arrival is only inspected when it is delivered.
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Latched);
        let lifted = unmask_ok(&mut rt);
        assert_eq!(lifted.delivered, 0);
        assert_eq!(lifted.overruns, 1);
        // The fault contract's rule 3: "Delivery is not inside a masked region", so L's declared
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
        assert_eq!(rt.first_contained_overrun(), Some(L), "contained");
        assert_eq!(rt.fatal_record(), None, "so not escalated");
        assert_sound(&rt);
    }

    // -- a doubled release inside one masked region (the fault contract's rule 1) ---------------

    /// `L` runs, and `H` — never released before — arrives twice inside one masked region that
    /// `L` holds. `H` declares `policy`.
    fn h_doubled_inside_a_region_held_by_l(policy: OverrunAction) -> Runtime<2> {
        let mut rt = boot2(policy, OverrunAction::TerminateJob);
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        assert_eq!(rt.is_overrun_latched(H), Some(false));
        // The latch is already full. Rule 1: observed only at delivery — so this arrival is
        // latched like the first: it is now the held, most recent release, the first is "a mark
        // that an earlier one also came" (Terms), and nothing else happens.
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        assert_eq!(rt.is_latched(H), Some(true));
        assert_eq!(rt.is_overrun_latched(H), Some(true));
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.first_contained_overrun(), None, "nothing judged yet");
        assert_eq!(rt.fatal_record(), None);
        assert!(rt.is_masked());
        assert_sound(&rt);
        rt
    }

    #[test]
    fn a_doubled_release_under_skip_late_job_is_contained_at_delivery() {
        let mut rt = h_doubled_inside_a_region_held_by_l(OverrunAction::TerminateJob);
        let lifted = unmask_ok(&mut rt);
        // The mark's arrival is judged first (H owed nothing, so it becomes ready), then the held
        // release, against the job the first started: an overrun, and H's SkipLateJob skips that
        // job, and the held release "makes the task ready for its next job, with that release's
        // nominal instant". One scheduler run, one transition.
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
        assert_eq!(rt.first_contained_overrun(), Some(H));
        assert_eq!(
            rt.fatal_record(),
            None,
            "not fatal for landing inside a region"
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_doubled_release_under_stop_task_stops_the_task_at_delivery() {
        let mut rt = h_doubled_inside_a_region_held_by_l(OverrunAction::StopTask);
        let lifted = unmask_ok(&mut rt);
        // The mark's arrival is delivered; the held release then finds H owing that job, an
        // overrun, and H's Fault takes the release with it, so L never loses the processor and
        // the unmask moves no context.
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
        assert_eq!(rt.first_contained_overrun(), Some(H));
        assert_eq!(rt.fatal_record(), None);
        assert_sound(&rt);
    }

    /// Rule 1's own test: an overrun is "not lost, nor fatal for landing inside a region rather
    /// than one instruction after it". The same two arrivals are replayed just inside the region
    /// and just after it, for the running task and for a task that has not run, under both
    /// policies, and the resulting task states, processor and evidence must agree. Only the
    /// number of context transitions may differ — one per delivery, by [`Unmasked::transition`]'s
    /// rule.
    #[test]
    fn a_doubled_release_ends_the_same_inside_a_region_as_one_instruction_after_it() {
        type Observed = (
            Processor,
            [Option<TaskState>; 2],
            Option<TaskId>,
            Option<FaultRecord>,
        );
        fn observe(rt: &Runtime<2>) -> Observed {
            (
                rt.processor(),
                [rt.state(H), rt.state(L)],
                rt.first_contained_overrun(),
                rt.fatal_record(),
            )
        }
        for policy in [OverrunAction::TerminateJob, OverrunAction::StopTask] {
            for doubled in [H, L] {
                let mut inside = boot2(policy, policy);
                inside.release(L).unwrap();
                mask_ok(&mut inside);
                inside.release(doubled).unwrap();
                inside.release(doubled).unwrap();
                unmask_ok(&mut inside);

                let mut after = boot2(policy, policy);
                after.release(L).unwrap();
                mask_ok(&mut after);
                unmask_ok(&mut after);
                after.release(doubled).unwrap();
                after.release(doubled).unwrap();

                assert_eq!(
                    observe(&inside),
                    observe(&after),
                    "policy {policy:?}, doubled task {}",
                    doubled.index()
                );
                assert_eq!(inside.fatal_record(), None, "neither side halts");
                inside.check_invariants().unwrap();
            }
        }
    }

    #[test]
    fn the_earlier_arrivals_a_mark_stands_for_are_judged_as_one() {
        // Three arrivals of H inside one region: the latch holds the third, the most recent, and
        // its mark stands for the first two. Rule 1: "the earlier arrivals a hosted latch's mark
        // stands for are judged as one". Judged one by one they would be a fresh release and an
        // overrun, and the held third a second overrun under SkipLateJob, or a discarded release
        // under Fault.
        for (policy, after) in [
            (OverrunAction::TerminateJob, TaskState::Running),
            (OverrunAction::StopTask, TaskState::Stopped),
        ] {
            let mut rt = boot2(policy, OverrunAction::TerminateJob);
            rt.release(L).unwrap();
            mask_ok(&mut rt);
            for _ in 0..3 {
                assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
            }
            assert_eq!(rt.is_overrun_latched(H), Some(true));
            let lifted = unmask_ok(&mut rt);
            // The mark's arrivals, as one: fresh. The held release: the overrun.
            assert_eq!(
                (lifted.delivered, lifted.overruns, lifted.stopped),
                (1, 1, 0),
                "{policy:?}"
            );
            assert_eq!(rt.state(H), Some(after), "{policy:?}");
            assert_sound(&rt);
        }
    }

    #[test]
    fn a_doubled_release_of_the_running_task_is_two_overruns_at_delivery() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        rt.release(L).unwrap();
        rt.release(L).unwrap();
        let lifted = unmask_ok(&mut rt);
        // L already owed its running job, so the mark's arrival is an overrun at delivery and the
        // held release another — as the same two arrivals would be after the unmask. Each skips a
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
    fn under_fault_a_latched_arrival_after_the_tasks_overrun_is_discarded() {
        // Rule 1: "each later one is an overrun under `SkipLateJob`, and is discarded under
        // `Fault`, whose first overrun took the task out". L runs, owing its job, and declares
        // Fault; two arrivals land in its own region. The mark's is the overrun that stops L; the
        // held one, after it, is discarded — counted as stopped, not as a second overrun.
        let mut rt = boot2(OverrunAction::TerminateJob, OverrunAction::StopTask);
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        rt.release(L).unwrap();
        rt.release(L).unwrap();
        assert_eq!(
            unmask_ok(&mut rt),
            Unmasked {
                depth: 0,
                delivered: 0,
                overruns: 1,
                stopped: 1,
                transition: Some(Transition::ToIdle { outgoing: L }),
            }
        );
        assert_eq!(rt.state(L), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.first_contained_overrun(), Some(L));
        assert_eq!(rt.fatal_record(), None, "Fault does not halt");
        assert_sound(&rt);
    }

    #[test]
    fn delivery_takes_tasks_in_rank_order_and_each_tasks_arrivals_in_arrival_order() {
        // Four tasks: D holds the region; A, B and C arrive inside it, in the reverse of rank.
        const A: TaskId = TaskId::from_index(0);
        const B: TaskId = TaskId::from_index(1);
        const C: TaskId = TaskId::from_index(2);
        const D: TaskId = TaskId::from_index(3);
        let spec = |name, rank| TaskSpec {
            name,
            priority: Priority::new(rank),
            on_overrun: OverrunAction::TerminateJob,
        };
        let mut rt =
            Runtime::boot([spec("A", 1), spec("B", 2), spec("C", 3), spec("D", 4)]).unwrap();
        rt.release(D).unwrap();
        mask_ok(&mut rt);
        rt.release(C).unwrap();
        rt.release(C).unwrap(); // C's overrun arrives first …
        rt.release(B).unwrap();
        rt.release(B).unwrap(); // … B's second,
        rt.release(A).unwrap();
        let lifted = unmask_ok(&mut rt);
        // Each doubled task's mark is judged before its held release, so each is delivered once
        // and overruns once.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 3,
                overruns: 2,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: D,
                    incoming: A,
                }),
            }
        );
        // … but B, the higher rank, is judged first. Rule 1 makes the order across tasks "the
        // port's"; this model's is ascending rank, the closing being one instant (the AMENDED
        // note on `Runtime::unmask`).
        assert_eq!(rt.first_contained_overrun(), Some(B));
        for task in [B, C, D] {
            assert_eq!(rt.state(task), Some(TaskState::Ready));
        }
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_found_at_delivery_still_costs_one_transition_for_the_whole_unmask() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        rt.release(L).unwrap(); // latched, and L still owes its job
        rt.release(H).unwrap(); // latched, and a genuine new release
        let lifted = unmask_ok(&mut rt);
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
    fn a_task_faulted_at_delivery_does_not_stop_the_delivery() {
        // Rule 5's `Fault` does not halt, so a delivery that faults one task goes on to the next.
        // (Under the halting policy this model used to have, the delivery stopped at it.)
        const A: TaskId = TaskId::from_index(0);
        const B: TaskId = TaskId::from_index(1);
        const C: TaskId = TaskId::from_index(2);
        let mut rt = Runtime::boot([
            TaskSpec {
                name: "A",
                priority: Priority::new(1),
                on_overrun: OverrunAction::StopTask,
            },
            TaskSpec {
                name: "B",
                priority: Priority::new(2),
                on_overrun: OverrunAction::TerminateJob,
            },
            TaskSpec {
                name: "C",
                priority: Priority::new(3),
                on_overrun: OverrunAction::TerminateJob,
            },
        ])
        .unwrap();
        rt.release(C).unwrap();
        mask_ok(&mut rt);
        rt.release(A).unwrap();
        rt.release(A).unwrap();
        rt.release(B).unwrap();
        let lifted = unmask_ok(&mut rt);
        // A is delivered, then faulted by its own policy; B, ranked below it, is still delivered,
        // and takes the processor from C.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 2,
                overruns: 1,
                stopped: 0,
                transition: Some(Transition::Switch {
                    outgoing: C,
                    incoming: B,
                }),
            }
        );
        assert_eq!(rt.state(A), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Running(B));
        assert_eq!(rt.fatal_record(), None);
        assert_sound(&rt);
    }

    // -- a completion inside a masked region (the fault contract's rule 4) ----------------------

    #[test]
    fn masking_does_not_defer_a_completion_and_the_completion_closes_the_region() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        // A completion is the running task reaching the end of its own computation, not an
        // arrival, so rule 1's latching does not apply to it. Rule 4: "its completion closes it"
        // — the depth returns to zero with the job.
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
        assert_eq!(rt.fatal_record(), None, "a completion is not a fault");
        assert_sound(&rt);
        // The region is closed, so the unmask that would have closed it now arrives with no job
        // running and at depth zero: an assertion failure on both counts (the Terms, and the
        // second smaller decision), attributed to whatever executes it — idle, since L's job is
        // over.
        assert_eq!(rt.unmask().unwrap(), UnmaskEffect::Fatal);
        assert_eq!(
            rt.fatal_record(),
            assertion_failure(Attribution::NoTask {
                context: Context::Idle,
                interrupted: None,
            })
        );
    }

    #[test]
    fn a_completion_closes_every_nested_section_not_one_level() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        mask_ok(&mut rt);
        mask_ok(&mut rt);
        rt.release(H).unwrap();
        let done = rt.complete().unwrap();
        assert_eq!(done.depth, 0);
        assert_eq!(rt.mask_depth(), 0);
        // Delivery happened at once, as the region closed, not at an inner `unmask`: "after the
        // completion is recorded and before any task executes an instruction of its own code".
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
        mask_ok(&mut rt);
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        let done = rt.complete().unwrap();
        // L's job ends, the region closes, H is delivered, and — this model's order, rule 4's
        // "hosted model" — the schedule is decided after: one transition, from the completing
        // task to the delivered one.
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
        mask_ok(&mut rt);
        // At arrival L still owes the job it is running; that is not judged now.
        assert_eq!(rt.release(L).unwrap(), ReleaseEffect::Latched);
        let done = rt.complete().unwrap();
        // Rule 4: "A latched release is judged at that delivery (rule 1), so a task that
        // completed inside the region is released afresh, not overrun." L's next job takes the
        // processor from its finished one.
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
        assert_eq!(
            rt.first_contained_overrun(),
            None,
            "released afresh, not overrun"
        );
        assert_sound(&rt);
    }

    #[test]
    fn a_completion_inside_a_region_with_its_own_release_doubled_overruns_once() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        rt.release(L).unwrap();
        rt.release(L).unwrap();
        assert_eq!(rt.is_overrun_latched(L), Some(true));
        let done = rt.complete().unwrap();
        // Rule 1: "the first is fresh if the task owes no job (one that completed inside the
        // region included) and an overrun if it does; each later one is an overrun under
        // `SkipLateJob`". The first is the mark's; L's SkipLateJob makes the held release, the
        // most recent, the job after.
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
        assert_eq!(rt.first_contained_overrun(), Some(L));
        assert_eq!(rt.fatal_record(), None);
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_found_by_a_completions_delivery_applies_its_policy_unescalated() {
        let mut rt = boot2(OverrunAction::TerminateJob, OverrunAction::StopTask);
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        mask_ok(&mut rt);
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
        assert_eq!(rt.first_contained_overrun(), Some(L));
        assert_eq!(rt.fatal_record(), None);
        assert_sound(&rt);
    }

    // -- the mask bound and an unbalanced unmask (the fault contract's second smaller decision) ---

    #[test]
    fn an_unmask_at_depth_zero_is_an_assertion_failure_and_halts() {
        // Executed by idle: no task's — and "a call with no job running", which the Terms make an
        // assertion failure on their own (see the next section).
        let mut rt = two_tasks();
        assert_eq!(rt.unmask().unwrap(), UnmaskEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(rt.mask_depth(), 0);
        assert_eq!(
            rt.fatal_record(),
            assertion_failure(Attribution::NoTask {
                context: Context::Idle,
                interrupted: None,
            })
        );
        assert_eq!(rt.release(H).unwrap_err(), Refused::Halted);

        // Executed by a running job: "A task's job is that task, including a runtime primitive it
        // called".
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(rt.unmask().unwrap(), UnmaskEffect::Fatal);
        assert_eq!(rt.fatal_record(), assertion_failure(Attribution::Task(L)));
    }

    #[test]
    fn a_mask_past_the_declared_depth_is_an_assertion_failure_and_halts() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        for depth in 1..=Runtime::<2>::MAX_MASK_DEPTH {
            assert_eq!(rt.mask().unwrap(), MaskEffect::Masked(depth));
        }
        assert_eq!(rt.mask().unwrap(), MaskEffect::Fatal);
        // Neither wrapped, nor saturated, nor refused: the depth is the bound, kept as evidence,
        // and the runtime has stopped.
        assert_eq!(rt.mask_depth(), Runtime::<2>::MAX_MASK_DEPTH);
        assert_eq!(rt.processor(), Processor::Halted);
        // Raised inside a region and still not escalated: rule 3 escalates a containable fault,
        // and an assertion failure never was one.
        assert_eq!(rt.fatal_record(), assertion_failure(Attribution::Task(L)));
        assert_eq!(rt.unmask().unwrap_err(), Refused::Halted);
    }

    // -- only a job changes the depth (the fault contract's Terms) ------------------------------

    #[test]
    fn a_mask_with_no_job_running_is_an_assertion_failure_of_idle() {
        // The Terms: "Only a job changes the depth." Executed by any other context, a `mask` "is an
        // assertion failure of that context, raised before the depth changes", and a runtime "told
        // no context detects the case it can see, a call with no job running". This model used to
        // accept it and open a region with no holder.
        let mut rt = two_tasks();
        assert_eq!(rt.mask().unwrap(), MaskEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(rt.mask_depth(), 0, "the depth is left where it was");
        assert_eq!(
            rt.fatal_record(),
            assertion_failure(Attribution::NoTask {
                context: Context::Idle,
                interrupted: None,
            })
        );
        // Halted, so nothing arriving afterwards is latched as it would be in a region.
        assert_eq!(rt.release(L).unwrap_err(), Refused::Halted);
        assert_eq!(rt.is_latched(L), Some(false));

        // Idle again after the last job completes: the same.
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.complete().unwrap();
        assert_eq!(rt.mask().unwrap(), MaskEffect::Fatal);
        assert_eq!(
            rt.fatal_record(),
            assertion_failure(Attribution::NoTask {
                context: Context::Idle,
                interrupted: None,
            })
        );
    }

    /// A masked region open with no job running or holding it — the state the `mask` this model
    /// used to accept with the processor idle left behind. No event reaches it any more, so it is
    /// forced.
    fn a_region_no_job_holds() -> Runtime<2> {
        let mut rt = two_tasks();
        rt.mask_depth = 1;
        rt.holder = None;
        rt
    }

    #[test]
    fn a_region_no_job_holds_is_a_violation() {
        // Only a job opens a region, so one with no job holding it means the model is broken.
        assert_eq!(
            a_region_no_job_holds().check_invariants(),
            Err(Violation::JobStartedInsideMaskedRegion {
                holder: None,
                running: None,
            })
        );
    }

    #[test]
    fn an_unmask_with_no_job_running_is_an_assertion_failure_whatever_the_depth() {
        // The Terms' `unmask` half. At depth zero the second smaller decision halts as well, so
        // the Terms' own condition shows only above it — where an `unmask` that did not check it
        // would close the region and deliver what it latched.
        let mut rt = a_region_no_job_holds();
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        assert_eq!(rt.unmask().unwrap(), UnmaskEffect::Fatal);
        assert_eq!(rt.mask_depth(), 1, "not lowered");
        assert_eq!(rt.is_latched(H), Some(true), "nothing delivered");
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(
            rt.fatal_record(),
            assertion_failure(Attribution::NoTask {
                context: Context::Idle,
                interrupted: None,
            })
        );
    }

    // -- faults --------------------------------------------------------------------------------

    #[test]
    fn each_fault_carries_its_section_8_1_class() {
        // The fault contract's table, second column.
        assert_eq!(Fault::Overrun.class(), Class::ExpectedError);
        assert_eq!(
            Fault::StackGuard(Guard::InterruptStack).class(),
            Class::ViolatedInternalInvariant
        );
        assert_eq!(Fault::UnexpectedTrap.class(), Class::DeliberateFatalTrap);
        assert_eq!(
            Fault::AssertionFailure.class(),
            Class::ViolatedInternalInvariant
        );
    }

    #[test]
    fn a_task_without_an_on_overrun_clause_has_fault() {
        // Rule 5: "A task without an `on-overrun` clause has `Fault`".
        assert_eq!(OverrunAction::default(), OverrunAction::StopTask);
    }

    #[test]
    fn an_unexpected_trap_is_fatal_for_the_whole_runtime() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(
            rt.raise_unexpected_trap(Context::Task(L)).unwrap(),
            FaultEffect::Fatal
        );
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::UnexpectedTrap,
                class: Class::DeliberateFatalTrap,
                attribution: Attribution::Task(L),
                escalated: false,
            })
        );
        // Everything is refused from here, including another release.
        assert_eq!(rt.release(H).unwrap_err(), Refused::Halted);
        assert_eq!(rt.complete().unwrap_err(), Refused::Halted);
        assert_eq!(rt.mask().unwrap_err(), Refused::Halted);
        assert_eq!(
            rt.raise_assertion_failure(Context::Task(L)).unwrap_err(),
            Refused::Halted
        );
    }

    #[test]
    fn a_stack_guard_and_an_assertion_failure_are_fatal_too() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(
            rt.raise_stack_guard(Context::Task(L), Guard::Task(L))
                .unwrap(),
            FaultEffect::Fatal
        );
        assert_eq!(rt.processor(), Processor::Halted);

        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(
            rt.raise_assertion_failure(Context::Task(L)).unwrap(),
            FaultEffect::Fatal
        );
        assert_eq!(rt.processor(), Processor::Halted);
    }

    #[test]
    fn a_stack_guard_names_whose_guard_was_hit() {
        // Rule 2: "the evidence names whose guard was hit — a task's, or the interrupt stack's."
        // A task's own guard, hit by its own job:
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.raise_stack_guard(Context::Task(L), Guard::Task(L))
            .unwrap();
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::StackGuard(Guard::Task(L)),
                class: Class::ViolatedInternalInvariant,
                attribution: Attribution::Task(L),
                escalated: false,
            })
        );
        // The interrupt stack's guard, hit by a service: no task's, and L only interrupted.
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.raise_stack_guard(Context::Service, Guard::InterruptStack)
            .unwrap();
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::StackGuard(Guard::InterruptStack),
                class: Class::ViolatedInternalInvariant,
                attribution: Attribution::NoTask {
                    context: Context::Service,
                    interrupted: Some(L),
                },
                escalated: false,
            })
        );
    }

    #[test]
    fn a_fault_in_a_runtime_context_is_no_tasks_and_records_the_interrupted_task_as_interrupted() {
        // Rule 2: "A service, the trap path, a transition, idle, initialisation or the fatal
        // handler is no task: the evidence names that context, a service by its source, and
        // records the task it interrupted as interrupted, never as attributed." Rule 7 preserves
        // "the task it interrupted, if any". (The source of a service this model cannot name; see
        // `Context::Service`.)
        for context in [Context::Service, Context::TrapPath, Context::Transition] {
            let mut rt = two_tasks();
            rt.release(L).unwrap();
            assert_eq!(
                rt.raise_assertion_failure(context).unwrap(),
                FaultEffect::Fatal
            );
            assert_eq!(
                rt.fatal_record(),
                assertion_failure(Attribution::NoTask {
                    context,
                    interrupted: Some(L),
                }),
                "{context:?}"
            );
        }
    }

    #[test]
    fn a_fault_in_idle_is_no_tasks_and_interrupts_nothing() {
        let mut rt = two_tasks();
        assert_eq!(
            rt.raise_unexpected_trap(Context::Idle).unwrap(),
            FaultEffect::Fatal
        );
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::UnexpectedTrap,
                class: Class::DeliberateFatalTrap,
                attribution: Attribution::NoTask {
                    context: Context::Idle,
                    interrupted: None,
                },
                escalated: false,
            })
        );
        // Rule 2: "A service or the trap path interrupts the task whose job it preempted, or none
        // if it preempted idle."
        for context in [Context::Service, Context::TrapPath] {
            let mut rt = two_tasks();
            rt.raise_assertion_failure(context).unwrap();
            assert_eq!(
                rt.fatal_record(),
                assertion_failure(Attribution::NoTask {
                    context,
                    interrupted: None,
                }),
                "{context:?}"
            );
        }
    }

    #[test]
    fn a_fault_in_a_transition_interrupts_the_task_rule_2_names_for_its_phase() {
        // Rule 2: "A transition interrupts the incoming task once the switch is decided; while it
        // is being decided, the outgoing task if its job is still owed, and none if the outgoing
        // context was idle or its job has completed or been abandoned." Each event here completes
        // its transition, so a fault "once the switch is decided" is raised after the event, and
        // one "while it is being decided" in a preemption before it.
        fn interrupted_by_a_transition_fault(rt: &mut Runtime<2>) -> Option<TaskId> {
            assert_eq!(
                rt.raise_unexpected_trap(Context::Transition).unwrap(),
                FaultEffect::Fatal
            );
            match rt.fatal_record().expect("halted").attribution {
                Attribution::NoTask {
                    context: Context::Transition,
                    interrupted,
                } => interrupted,
                other => panic!("a transition is no task: {other:?}"),
            }
        }
        // Idle to L: none while it is being decided, L once it is.
        let mut rt = two_tasks();
        assert_eq!(interrupted_by_a_transition_fault(&mut rt), None);
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(interrupted_by_a_transition_fault(&mut rt), Some(L));
        // L to H, a preemption: L, the outgoing one whose job is still owed, while it is being
        // decided; H, the incoming, once it is.
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(interrupted_by_a_transition_fault(&mut rt), Some(L));
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        assert_eq!(interrupted_by_a_transition_fault(&mut rt), Some(H));
        // H to L at H's completion, once decided: L. (While it is being decided rule 2 names none,
        // H's job having completed — a point this model has no state for; see the
        // `DECIDED, NOT EXPRESSED` note on `Context::Transition`.)
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.complete().unwrap();
        assert_eq!(interrupted_by_a_transition_fault(&mut rt), Some(L));
        // L to idle at its completion: none in both phases, which the state after the event shows.
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.complete().unwrap();
        assert_eq!(interrupted_by_a_transition_fault(&mut rt), None);
    }

    #[test]
    fn idle_cannot_fault_while_a_task_holds_the_processor() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        assert_eq!(
            rt.raise_unexpected_trap(Context::Idle).unwrap_err(),
            Refused::NotIdle
        );
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.fatal_record(), None, "a refusal is not a fault");
    }

    #[test]
    fn the_fatal_handler_preserves_the_state_that_explains_it() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.raise_unexpected_trap(Context::Task(H)).unwrap();
        // H was running and L was preempted; both readings survive the halt, because a fatal
        // handler that tidied up would be destroying its own evidence.
        assert_eq!(rt.state(H), Some(TaskState::Running));
        assert_eq!(rt.state(L), Some(TaskState::Ready));
    }

    #[test]
    fn the_fatal_record_is_the_first_fault_that_entered_the_handler_not_the_first_contained_one() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        // Contained: L's late job is skipped and this release is its next (rules 1 and 5), and the
        // runtime goes on.
        assert_eq!(
            rt.release(L).unwrap(),
            ReleaseEffect::Overrun(FaultEffect::JobTerminated(Some(Transition::Switch {
                outgoing: L,
                incoming: L,
            })))
        );
        assert_eq!(rt.fatal_record(), None);
        rt.release(H).unwrap();
        rt.raise_unexpected_trap(Context::Task(H)).unwrap();
        // Rule 7 keeps "the first such fault only" — the first to enter the fatal handler — so
        // the evidence of the halt is H's trap, not L's overrun, which the runtime survived.
        let record = rt.fatal_record().expect("halted");
        assert_eq!(record.fault, Fault::UnexpectedTrap);
        assert_eq!(record.attribution, Attribution::Task(H));
        // L's overrun is kept too, for what rule 5 makes of it: the end of the timing claims.
        assert_eq!(rt.first_contained_overrun(), Some(L));
        // "A fault taken in the handler ends it at once in its terminal state and never replaces
        // the preserved one."
        assert_eq!(
            rt.raise_assertion_failure(Context::TrapPath).unwrap_err(),
            Refused::Halted
        );
        assert_eq!(rt.fatal_record(), Some(record));
    }

    #[test]
    fn after_a_halt_no_job_runs_no_release_is_processed_or_latched_and_nothing_moves() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        rt.raise_assertion_failure(Context::Task(L)).unwrap();
        let record = rt.fatal_record();
        // Rule 7: "From then no job runs, no release is processed or latched, and no transition
        // occurs".
        assert_eq!(rt.release(L).unwrap_err(), Refused::Halted);
        assert_eq!(rt.release(H).unwrap_err(), Refused::Halted);
        assert_eq!(rt.is_overrun_latched(H), Some(false), "not latched either");
        assert_eq!(rt.complete().unwrap_err(), Refused::Halted);
        assert_eq!(rt.unmask().unwrap_err(), Refused::Halted);
        assert_eq!(rt.mask().unwrap_err(), Refused::Halted);
        assert_eq!(rt.raise_overrun(L).unwrap_err(), Refused::Halted);
        assert_eq!(
            rt.raise_unexpected_trap(Context::Service).unwrap_err(),
            Refused::Halted
        );
        assert_eq!(
            rt.raise_stack_guard(Context::TrapPath, Guard::InterruptStack)
                .unwrap_err(),
            Refused::Halted
        );
        // Nothing moved: the region, the latch, the states and the record are as the fault left
        // them.
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(rt.mask_depth(), 1);
        assert_eq!(rt.is_latched(H), Some(true));
        assert_eq!(rt.state(L), Some(TaskState::Running));
        assert_eq!(rt.state(H), Some(TaskState::Created));
        assert_eq!(rt.fatal_record(), record);
        assert_sound(&rt);
    }

    #[test]
    fn a_runs_timing_claims_end_at_its_first_contained_overrun() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.complete().unwrap();
        assert_eq!(rt.first_contained_overrun(), None, "no overrun yet");
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        rt.release(L).unwrap(); // L overruns, and its policy contains it
        assert_eq!(rt.first_contained_overrun(), Some(L));
        rt.release(H).unwrap(); // so does H
        assert_eq!(
            rt.first_contained_overrun(),
            Some(L),
            "rule 5: the first, not the latest"
        );
        assert_eq!(rt.fatal_record(), None);
        assert_sound(&rt);
    }

    #[test]
    fn a_stopped_task_is_skipped_by_the_scheduler_even_when_latched() {
        let mut rt = boot2(OverrunAction::StopTask, OverrunAction::TerminateJob);
        rt.release(H).unwrap();
        rt.release(H).unwrap(); // H overruns, and its Fault stops it
        assert_eq!(rt.state(H), Some(TaskState::Stopped));
        // L holds a region, and H — which outranks it — arrives inside it. (This test used to
        // stop H by the entry outside the profile and then let idle open the region, which the
        // Terms now make an assertion failure.)
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Latched);
        let lifted = unmask_ok(&mut rt);
        // Discarded at delivery, so L keeps the processor H would otherwise have taken.
        assert_eq!(
            lifted,
            Unmasked {
                depth: 0,
                delivered: 0,
                overruns: 0,
                stopped: 1,
                transition: None,
            }
        );
        assert_eq!(rt.state(H), Some(TaskState::Stopped));
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_sound(&rt);
    }

    #[test]
    fn a_fault_that_was_never_containable_is_not_escalated_inside_a_region() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        rt.raise_unexpected_trap(Context::TrapPath).unwrap();
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::UnexpectedTrap,
                class: Class::DeliberateFatalTrap,
                attribution: Attribution::NoTask {
                    context: Context::TrapPath,
                    interrupted: Some(L),
                },
                escalated: false,
            }),
            "rule 3 escalates a containable fault, and a trap never was one"
        );
    }

    #[test]
    fn a_synchronous_fault_must_name_the_task_holding_the_processor() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        // The fault contract's rule 2: the three synchronous faults "are attributed to the context
        // that raises them", so naming another task is the concurrency claim the profile does not
        // admit.
        assert_eq!(
            rt.raise_unexpected_trap(Context::Task(H)).unwrap_err(),
            Refused::NotTheRunningTask
        );
        assert_eq!(
            rt.raise_stack_guard(Context::Task(H), Guard::Task(H))
                .unwrap_err(),
            Refused::NotTheRunningTask
        );
        assert_eq!(
            rt.raise_assertion_failure(Context::Task(H)).unwrap_err(),
            Refused::NotTheRunningTask
        );
        assert_eq!(rt.processor(), Processor::Running(L));
        assert_eq!(rt.fatal_record(), None, "a refusal is not a fault");
    }

    #[test]
    fn a_synchronous_fault_naming_a_task_while_the_processor_is_idle_is_refused() {
        let mut rt = two_tasks();
        assert_eq!(
            rt.raise_unexpected_trap(Context::Task(L)).unwrap_err(),
            Refused::NoTaskRunning
        );
        assert_eq!(
            rt.raise_stack_guard(Context::Task(L), Guard::Task(L))
                .unwrap_err(),
            Refused::NoTaskRunning
        );
        assert_eq!(
            rt.raise_assertion_failure(Context::Task(L)).unwrap_err(),
            Refused::NoTaskRunning
        );
        assert_eq!(rt.fatal_record(), None);
    }

    // -- outside the profile: an overrun raised without a release (rule 1a) ----------------------
    //
    // "An overrun raised without a release is outside this profile." These tests pin the
    // behaviour this model gives `Runtime::raise_overrun`, the entry a later profile would need;
    // they test no sentence of `rt-static-up-v1`'s contract except rule 3, which "fixes the answer
    // a later profile that admits one starts from".

    #[test]
    fn an_overrun_terminates_only_the_job_when_the_task_declares_that() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        // With no triggering release, SkipLateJob abandons the job and the task waits for its next
        // release (this model's reading; rule 1a leaves it to a later profile).
        assert_eq!(
            rt.raise_overrun(H).unwrap(),
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
        let mut rt = boot2(OverrunAction::StopTask, OverrunAction::TerminateJob);
        rt.release(H).unwrap();
        // Fault: the task is faulted, with no release to discard.
        assert_eq!(
            rt.raise_overrun(H).unwrap(),
            FaultEffect::TaskStopped(Some(Transition::ToIdle { outgoing: H }))
        );
        assert_eq!(rt.state(H), Some(TaskState::Stopped));
        // §3.1 excludes dynamic task creation, so there is no way back.
        assert_eq!(rt.release(H).unwrap(), ReleaseEffect::Stopped);
        assert_eq!(rt.processor(), Processor::Idle);
        assert_eq!(rt.fatal_record(), None, "and the runtime does not halt");
        assert_sound(&rt);
    }

    #[test]
    fn an_overrun_needs_a_task_that_still_owes_a_job() {
        let mut rt = two_tasks();
        // Created: never released, so there is no job for it to be late with.
        assert_eq!(rt.raise_overrun(L).unwrap_err(), Refused::NoJobOwed);
        rt.release(L).unwrap();
        rt.complete().unwrap();
        // Completed: its job is over, so it is not overrunning anything either.
        assert_eq!(rt.raise_overrun(L).unwrap_err(), Refused::NoJobOwed);
    }

    #[test]
    fn an_overrun_can_be_raised_against_a_ready_task_and_moves_no_context() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        // The fault contract's rule 2: the overrunning task, "whichever context holds the
        // processor".
        assert_eq!(
            rt.raise_overrun(L).unwrap(),
            FaultEffect::JobTerminated(None)
        );
        assert_eq!(rt.state(L), Some(TaskState::Completed));
        assert_eq!(rt.processor(), Processor::Running(H));
        assert_eq!(rt.first_contained_overrun(), Some(L));
        assert_sound(&rt);
    }

    #[test]
    fn a_containable_fault_inside_a_masked_region_escalates_to_fatal() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        // Rule 3, ground 1: L holds the region, and skipping its job would leave the depth above
        // zero with no owner.
        assert_eq!(rt.raise_overrun(L).unwrap(), FaultEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::Overrun,
                class: Class::ExpectedError,
                attribution: Attribution::Task(L),
                escalated: true,
            })
        );
        assert_eq!(
            rt.first_contained_overrun(),
            None,
            "escalated, so never contained"
        );
    }

    #[test]
    fn a_containable_fault_of_a_task_that_holds_no_region_escalates_too() {
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        rt.release(H).unwrap();
        mask_ok(&mut rt);
        // H holds the region; L is ready and owes a job. Rule 3: "Ground 1 covers the task
        // holding the region; ground 2 extends the rule to every task." The first rewrite's rule
        // 1a said such an overrun "cannot be raised inside a masked region", and this model
        // refused it; that sentence is gone, and ground 2 stands.
        assert_eq!(rt.raise_overrun(L).unwrap(), FaultEffect::Fatal);
        assert_eq!(rt.processor(), Processor::Halted);
        assert_eq!(
            rt.fatal_record(),
            Some(FaultRecord {
                fault: Fault::Overrun,
                class: Class::ExpectedError,
                attribution: Attribution::Task(L),
                escalated: true,
            })
        );
        assert_eq!(
            rt.first_contained_overrun(),
            None,
            "escalated, so never contained"
        );
        // Frozen: L's job was not abandoned, and H's region is still open.
        assert_eq!(rt.state(L), Some(TaskState::Ready));
        assert_eq!(rt.mask_depth(), 1);
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
    /// Each timer ISR is the release service, one atomic [`Runtime::release`]: H "becomes ready at
    /// ISR completion", and the switch to H is charged after the ISR interval. The ISR is
    /// nonpreemptible — "Switches and ISR handling are nonpreemptible; arrivals during them are
    /// latched" — but that is the processor's own interrupt disable during a service, which
    /// the fault contract's Terms say "is **not** a masked region, and §13.4's "latched" means
    /// pending in hardware there". This test used to replay each ISR as
    /// `mask` → `release` → `unmask`; under the rewritten Terms that would be L's job opening a
    /// region it never opened, so the ISR no longer touches the depth. The transitions are the same
    /// either way.
    #[test]
    fn f29_produces_one_initial_dispatch_and_four_switches() {
        let mut rt = two_tasks();
        let mut transitions = [None; 6];
        let mut n = 0;
        let record = |t: Transition, transitions: &mut [Option<Transition>; 6], n: &mut usize| {
            transitions[*n] = Some(t);
            *n += 1;
        };

        // "The initial state already contains L's ready job … Initial dispatch from idle to L".
        let ReleaseEffect::Dispatched(t) = rt.release(L).unwrap() else {
            panic!("L must take the idle processor")
        };
        record(t, &mut transitions, &mut n);
        assert_sound(&rt);

        for _ in 0..2 {
            // "[4, 5) Timer ISR for first H release"; "H becomes ready at ISR completion", and
            // "[5, 7) Switch L to H" follows it.
            let ReleaseEffect::Preempted(t) = rt.release(H).unwrap() else {
                panic!("H preempts L")
            };
            record(t, &mut transitions, &mut n);
            assert_eq!(rt.processor(), Processor::Running(H));
            assert!(!rt.is_masked(), "a service is not a masked region");
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

    #[test]
    fn a_job_never_starts_inside_a_masked_region() {
        // The fault contract's Terms: "A depth above zero when a job is dispatched or resumed is
        // likewise an assertion failure". No event reaches it, so the state is forced here, as a
        // runtime that dispatched
        // inside a region would leave it: L opened the region and H holds the processor.
        let mut rt = two_tasks();
        rt.release(L).unwrap();
        mask_ok(&mut rt);
        assert_sound(&rt);
        rt.state[L.index()] = TaskState::Ready;
        rt.state[H.index()] = TaskState::Running;
        rt.running = Some(H);
        assert_eq!(
            rt.check_invariants(),
            Err(Violation::JobStartedInsideMaskedRegion {
                holder: Some(L),
                running: Some(H),
            })
        );
    }
}
