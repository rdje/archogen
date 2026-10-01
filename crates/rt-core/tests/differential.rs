//! `M2.2` — differential testing of `rt-core` against an independently derived reference; and
//! `M2.9` — the same comparison once the contract decided what the first one found it did not.
//!
//! `ROADMAP.md` §12 M2 states the requirement and the trap in one sentence:
//!
//! > Use exact fixtures, an independent implementation, and an external analysis source/tool
//! > where semantics agree. **A checker sharing the same erroneous recurrence with its reference
//! > does not qualify as independent.**
//!
//! # How independence was obtained, and what is still shared
//!
//! `crates/rt-reference` was written by an agent working in a separate context that was
//! instructed not to read `crates/rt-core/**`, `docs/book/src/runtime.md` or `docs/tasks/M2.md`.
//! It derived the model from `ROADMAP.md` §3.1/§8/§8.1 and the priority decision record. When
//! `M2.9` resolved the gaps this harness found, writing `ROADMAP.md` §3.1.1 on 2026-09-13 and
//! amending it on 2026-10-01, the reference was re-derived from the amended text each time, by a
//! context under the same instruction.
//!
//! ⚠️ **What remains shared, disclosed because §4.4 requires it and the `M2.2` acceptance demands
//! it explicitly:**
//!
//! | Shared input | Consequence |
//! |---|---|
//! | the contract text itself (`ROADMAP.md` §3.1, §3.1.1, §8, §8.1) | a misreading *the contract invites* would be made by both |
//! | `decision_priority-comparison-direction.md` | the priority direction is common to both |
//! | §3.1.1 was written by the author of `rt-core` | the re-derivation shows the text says what was meant; it cannot show that what was meant is right |
//! | the same model family produced both | §14: "a second model agreeing with the first is not ground truth" |
//! | this adapter, written by the author of `rt-core` | a mapping error here can mask or manufacture a divergence |
//! | near misses in the 2026-10-01 re-derivations | repository-wide searches showed the first author one line of `docs/book/src/runtime.md` and the three lines of `crates/rt-core/Cargo.toml` naming the reference; a `git status` showed the second the names, not the contents, of the `rt-core` files being changed beside it; no implementation or test text |
//!
//! ⭐ **So agreement is the weak result here, and disagreement is the strong one.** Two models
//! that disagree cannot have been copied from each other, and every divergence is either a defect
//! in one of them or a place the contract does not actually decide. The `M2.2` divergences were
//! all of the second kind; `M2.9` decided them, and the tests that recorded each disagreement now
//! record the agreement — rewritten, not deleted, because §14.1 forbids dropping the evidence.
//!
//! # The adapter's three deliberate alignments
//!
//! The two APIs are shaped differently — which is itself evidence — so three mappings are needed,
//! and each is a claim that could be wrong:
//!
//! 1. **Priority.** `rt-core` makes a task's *index* its rank, highest first. `rt-reference`
//!    carries an explicit `Priority` where `1` is highest and `0` is refused. So index `i` maps to
//!    rank `i + 1`, the relation `decision_priority-comparison-direction.md` now states.
//! 2. **Dispatch timing.** `rt-reference` dispatches *eagerly*, inside `release`. `rt-core`
//!    separates the event from the decision, so this harness calls `decide()` after every event
//!    to bring it to the same point. Without that the two are trivially "different" for a reason
//!    that is not a defect.
//! 3. **Overrun policy.** `rt-core::OverrunPolicy::Fault` leaves the task unschedulable and the
//!    runtime running, which is `rt-reference::OverrunAction::StopTask`, **not** its `Fatal`;
//!    `SkipLateJob` is its `TerminateJob`. The mapping follows observed behaviour, not the name.
//!
//! # What the comparison leaves out, and why
//!
//! Only events legal in both models' preconditions are generated, so a divergence is about
//! semantics rather than about one model refusing what the other accepts. The reference refuses,
//! and `rt-core` accepts:
//!
//! - a fault raised in a task's job while no task runs (`rt-core` attributes it to no task), or
//!   in the idle context while one does;
//! - an overrun raised for a task that owes no job;
//! - any event after a halt. Both answer it and change nothing: `rt-core` with the halt it keeps,
//!   the reference with a refusal.
//!
//! And once both have halted, what each leaves in its task table is left to the implementation:
//! the comparison checks that both halted and what each kept — the fault, whom it is attributed
//! to, whom it interrupted, whether rule 3 escalated it — and `d9` asserts the tables on both
//! sides.

use rt_core::{
    Context as CoreContext, Decision, Fatal, OverrunPolicy, Scheduler, TaskState as CoreState,
    Transition as CoreTransition,
};
use rt_reference::{
    Attribution, Context as RefContext, FaultEffect, FaultRecord, Guard, MaskEffect, OverrunAction,
    Priority, Processor, Refused, ReleaseEffect, Runtime, TaskId, TaskSpec, TaskState as RefState,
    UnmaskEffect,
};

const N: usize = 3;

/// What both models are compared on. Deliberately coarse: it is the state a *user* of either
/// model can observe, not either one's internal bookkeeping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seen {
    /// Alive, owes no job — `rt-core`'s `Suspended`, the reference's `Created`/`Completed`.
    Waiting,
    Ready,
    Running,
    /// Permanently out of the schedule.
    Gone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Snapshot {
    tasks: [Seen; N],
    /// `None` = idle, `Some(i)` = task `i` has the processor.
    processor: Option<usize>,
    halted: bool,
}

fn core_snapshot(s: &Scheduler<N>, halted: bool, running: Option<usize>) -> Snapshot {
    let mut tasks = [Seen::Waiting; N];
    for (i, slot) in tasks.iter_mut().enumerate() {
        *slot = match s.state(i) {
            CoreState::Suspended => Seen::Waiting,
            CoreState::Ready => Seen::Ready,
            CoreState::Running => Seen::Running,
            CoreState::Faulted => Seen::Gone,
        };
    }
    Snapshot {
        tasks,
        processor: running,
        halted,
    }
}

fn ref_snapshot(r: &Runtime<N>) -> Snapshot {
    let mut tasks = [Seen::Waiting; N];
    for (i, slot) in tasks.iter_mut().enumerate() {
        *slot = match r.state(TaskId::from_index(i)).expect("task exists") {
            RefState::Created | RefState::Completed => Seen::Waiting,
            RefState::Ready => Seen::Ready,
            RefState::Running => Seen::Running,
            RefState::Stopped => Seen::Gone,
        };
    }
    let (processor, halted) = match r.processor() {
        Processor::Idle => (None, false),
        Processor::Running(id) => (Some(id.index()), false),
        Processor::Halted => (None, true),
    };
    Snapshot {
        tasks,
        processor,
        halted,
    }
}

/// The reference, booted with the alignment described in this file's header.
fn reference(policies: [OverrunPolicy; N]) -> Runtime<N> {
    let spec = core::array::from_fn(|i| TaskSpec {
        name: ["a", "b", "c"][i],
        priority: Priority::new(u16::try_from(i).expect("small") + 1),
        on_overrun: match policies[i] {
            // Observed behaviour, not the name: `Fault` stops the task and keeps scheduling.
            OverrunPolicy::Fault => OverrunAction::StopTask,
            OverrunPolicy::SkipLateJob => OverrunAction::TerminateJob,
        },
    });
    Runtime::boot(spec).expect("three distinct ranks, none zero")
}

/// A deterministic generator. No dependencies (see the zero-dependency decision record), and
/// deterministic so a failing seed is a reproducible bug report rather than a story.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*, adequate for choosing among a handful of events.
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, bound: usize) -> usize {
        usize::try_from(self.next() % bound as u64).expect("bound is small")
    }
}

/// A fault synchronous to the executing context (§3.1.1 rule 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Synchronous {
    StackGuard,
    Trap,
    Assertion,
}

/// Where a synchronous fault is raised: in the running task's job, or in kernel code — a service
/// while a task runs, the idle loop while none does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Raised {
    InJob,
    InKernel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    Release(usize),
    Complete,
    Mask,
    Unmask,
    /// An `unmask` with nothing to close: an assertion failure in both (§3.1.1).
    UnbalancedUnmask,
    /// A `mask` with no job running: an assertion failure in both (§3.1.1, Terms: "Only a job
    /// changes the depth").
    MaskWithNoJob,
    /// An overrun found by something other than a release — an execution-budget monitor. Outside
    /// `rt-static-up-v1` (§3.1.1 rule 1a), which detects overruns by releases alone; both models
    /// keep an entry for one, for a later profile, so this compares them there too. Raised only for
    /// a task that owes a job; inside a masked region both escalate it, whoever's it is.
    MonitorOverrun(usize),
    /// A trap, a stack guard or an assertion.
    Synchronous(Synchronous, Raised),
}

/// The same synchronous fault in each model's terms, raised with the processor as `running`
/// shows it. A stack guard hit in a job is that task's; one hit in kernel code is the interrupt
/// stack's.
fn synchronous(
    kind: Synchronous,
    raised: Raised,
    running: Option<usize>,
    reference: &mut Runtime<N>,
) -> (rt_core::Fault, CoreContext, Result<FaultEffect, Refused>) {
    let (core_context, context) = match (raised, running) {
        (Raised::InJob, Some(task)) => {
            (CoreContext::Job, RefContext::Task(TaskId::from_index(task)))
        }
        (Raised::InJob, None) => unreachable!("a job's fault is generated only while a task runs"),
        (Raised::InKernel, Some(_)) => (CoreContext::Kernel, RefContext::Service),
        (Raised::InKernel, None) => (CoreContext::Kernel, RefContext::Idle),
    };
    let guarded = match raised {
        Raised::InJob => running,
        Raised::InKernel => None,
    };
    match kind {
        Synchronous::StackGuard => (
            rt_core::Fault::StackGuard { task: guarded },
            core_context,
            reference.raise_stack_guard(
                context,
                guarded.map_or(Guard::InterruptStack, |task| {
                    Guard::Task(TaskId::from_index(task))
                }),
            ),
        ),
        Synchronous::Trap => (
            rt_core::Fault::UnexpectedTrap { cause: 0 },
            core_context,
            reference.raise_unexpected_trap(context),
        ),
        Synchronous::Assertion => (
            rt_core::Fault::InvariantViolated {
                invariant: "differential",
            },
            core_context,
            reference.raise_assertion_failure(context),
        ),
    }
}

/// Whether two halts kept the same record (§3.1.1 rule 7): the same kind of fault, attributed to
/// the same task or to none, the same interrupted task, and the same answer to whether rule 3
/// escalated it. The two models' records are shaped differently, so this is a mapping, and a
/// claim that could be wrong.
fn same_record(core: Fatal, reference: FaultRecord) -> bool {
    let kind = match (core.fault, reference.fault) {
        (rt_core::Fault::Overrun { .. }, rt_reference::Fault::Overrun)
        | (rt_core::Fault::UnexpectedTrap { .. }, rt_reference::Fault::UnexpectedTrap)
        | (rt_core::Fault::InvariantViolated { .. }, rt_reference::Fault::AssertionFailure) => true,
        (rt_core::Fault::StackGuard { task }, rt_reference::Fault::StackGuard(guard)) => {
            guard
                == task.map_or(Guard::InterruptStack, |task| {
                    Guard::Task(TaskId::from_index(task))
                })
        }
        _ => false,
    };
    let attribution = match reference.attribution {
        Attribution::Task(task) => core.task == Some(task.index()) && core.interrupted.is_none(),
        Attribution::NoTask { interrupted, .. } => {
            core.task.is_none() && core.interrupted == interrupted.map(TaskId::index)
        }
    };
    kind && attribution && core.escalated == reference.escalated
}

/// What one event did, as far as the coverage count needs to know.
#[derive(Debug, Default)]
struct Stepped {
    /// `rt-core`'s transitions.
    core: Vec<CoreTransition>,
    /// Overruns the reference found by a release, at arrival or at delivery.
    reference_overruns: usize,
}

/// Drive both models through one event, keeping `rt-core` at the same decision point.
///
/// `running` and `halted` are what `rt-core`'s `decide()` reported, because neither is on its
/// public surface otherwise. An `Err` is a divergence no snapshot shows: either model refusing an
/// event both should accept, or the two answering it differently.
fn step(
    core: &mut Scheduler<N>,
    reference: &mut Runtime<N>,
    event: Event,
    running: &mut Option<usize>,
    halted: &mut bool,
) -> Result<Stepped, String> {
    let refused =
        |who: &str, why: &dyn core::fmt::Debug| format!("{who} refused {event:?}: {why:?}");
    let mut stepped = Stepped::default();
    match event {
        Event::Release(i) => {
            stepped.core.push(core.release(i));
            let effect = reference
                .release(TaskId::from_index(i))
                .map_err(|why| refused("the reference", &why))?;
            if matches!(effect, ReleaseEffect::Overrun(_)) {
                stepped.reference_overruns += 1;
            }
        }
        Event::Complete => {
            let task = running.expect("generated only while a task runs");
            let (completed, delivered) = core.complete(task);
            stepped.core.push(completed);
            stepped.core.extend(delivered.as_slice());
            let closed = reference
                .complete()
                .map_err(|why| refused("the reference", &why))?;
            stepped.reference_overruns += closed.overruns;
        }
        Event::Mask => {
            core.mask().map_err(|why| refused("rt-core", &why))?;
            match reference.mask() {
                Ok(MaskEffect::Masked(_)) => {}
                other => return Err(format!("the reference answered {event:?} with {other:?}")),
            }
        }
        Event::Unmask => {
            let delivered = core.unmask().map_err(|why| refused("rt-core", &why))?;
            stepped.core.extend(delivered.as_slice());
            match reference.unmask() {
                Ok(UnmaskEffect::Unmasked(closed)) => stepped.reference_overruns += closed.overruns,
                other => return Err(format!("the reference answered {event:?} with {other:?}")),
            }
        }
        Event::MaskWithNoJob => {
            if core.mask().is_ok() {
                return Err("rt-core accepted a mask with no job running".into());
            }
            match reference.mask() {
                Ok(MaskEffect::Fatal) => {}
                other => return Err(format!("the reference answered {event:?} with {other:?}")),
            }
        }
        Event::UnbalancedUnmask => {
            if core.unmask().is_ok() {
                return Err("rt-core accepted an unmask with nothing to close".into());
            }
            match reference.unmask() {
                Ok(UnmaskEffect::Fatal) => {}
                other => return Err(format!("the reference answered {event:?} with {other:?}")),
            }
        }
        Event::MonitorOverrun(i) => {
            stepped
                .core
                .push(core.fault(rt_core::Fault::Overrun { task: i }, CoreContext::Kernel));
            reference
                .raise_overrun(TaskId::from_index(i))
                .map_err(|why| refused("the reference", &why))?;
        }
        Event::Synchronous(kind, raised) => {
            let (core_fault, context, effect) = synchronous(kind, raised, *running, reference);
            let effect = effect.map_err(|why| refused("the reference", &why))?;
            if effect != FaultEffect::Fatal {
                return Err(format!("the reference contained {kind:?}: {effect:?}"));
            }
            let transition = core.fault(core_fault, context);
            if !matches!(transition, CoreTransition::Halted { .. }) {
                return Err(format!("rt-core contained {kind:?}: {transition:?}"));
            }
            stepped.core.push(transition);
        }
    }
    // Alignment 2: bring `rt-core` to the decision point the reference is already at.
    match core.decide() {
        Decision::Idle => *running = None,
        Decision::Continue { task } | Decision::Dispatch { to: task } => *running = Some(task),
        Decision::Switch { to, .. } => *running = Some(to),
        Decision::Halt { .. } => {
            *halted = true;
            *running = None;
        }
    }
    Ok(stepped)
}

/// What the generator actually exercised. ⭐ Without this, "400 sequences agreed" could be true
/// of a generator that never left the idle state — the failure mode recorded in
/// `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`, where a corpus is complete
/// against its own specification and blind to the behaviour that matters.
#[derive(Debug, Default, Clone, Copy)]
struct Coverage {
    preemptions: usize,
    latched_deliveries: usize,
    completions: usize,
    idle_periods: usize,
    /// Overruns a release found, at arrival or at delivery, counted by both models.
    overruns_by_release: usize,
    /// Releases into a full latch (findings §6 (b)).
    doubled_latches: usize,
    /// Late jobs abandoned under `SkipLateJob`, a release's or a monitor's.
    skipped_jobs: usize,
    /// Completions inside a masked region (§3.1.1 rule 4).
    masked_completions: usize,
    /// Overruns raised by a monitor while unmasked, contained by the task's policy.
    contained_monitor_overruns: usize,
    /// Overruns a monitor raised inside a masked region, which halt
    /// (§3.1.1 rules 1a and 3).
    masked_escalations: usize,
    /// Traps, stack guards and assertions raised in a task's job, which halt.
    synchronous_halts: usize,
    /// The same raised in kernel code — a service or the idle loop — which halt and blame no task.
    kernel_faults: usize,
    /// `unmask` with nothing to close, or `mask` with no job running: assertion failures.
    unbalanced_unmasks: usize,
}

/// Run one randomised sequence, returning the first divergence with the trace that produced it.
fn run_sequence(
    seed: u64,
    length: usize,
    policies: [OverrunPolicy; N],
    coverage: &mut Coverage,
) -> Result<(), String> {
    let mut rng = Rng(seed);
    let mut core = Scheduler::<N>::new(policies);
    let mut reference = reference(policies);
    let mut running: Option<usize> = None;
    let mut halted = false;
    let mut trace: Vec<Event> = Vec::new();
    let mut depth = 0_u32;

    for _ in 0..length {
        // ⭐ Since `M2.9` the generator goes where `M2.2`'s could not: a release to a task that
        // still owes a job — at once, or latched beside one already held — completions inside a
        // masked region, overruns a monitor raises, and the synchronous faults. Each was excluded
        // while the contract did not decide it; §3.1.1 now does.
        let owes_a_job = |i: usize| {
            matches!(
                reference.state(TaskId::from_index(i)).expect("task exists"),
                RefState::Ready | RefState::Running
            )
        };
        let owing: Vec<usize> = (0..N).filter(|i| owes_a_job(*i)).collect();
        // Most releases go to a task with no job owed or latched, so the schedule stays busy
        // rather than draining into stopped tasks; the rest go to any task and may overrun.
        let free: Vec<usize> = (0..N)
            .filter(|i| {
                !owes_a_job(*i)
                    && !reference
                        .is_latched(TaskId::from_index(*i))
                        .expect("task exists")
            })
            .collect();
        // Outside the profile (§3.1.1 rule 1a); inside a region both models escalate it for any
        // task (rule 3, ground 2), so every task owing a job is a target.
        let monitored = owing.clone();
        let event = match rng.below(100) {
            0 => {
                let kind = match rng.below(3) {
                    0 => Synchronous::StackGuard,
                    1 => Synchronous::Trap,
                    _ => Synchronous::Assertion,
                };
                let raised = if running.is_some() && rng.below(2) == 0 {
                    Raised::InJob
                } else {
                    Raised::InKernel
                };
                Event::Synchronous(kind, raised)
            }
            1..=3 if !monitored.is_empty() => {
                Event::MonitorOverrun(monitored[rng.below(monitored.len())])
            }
            // Rarer than the rest: each ends the sequence, and an early end costs coverage.
            4 if depth == 0 && running.is_some() && rng.below(4) == 0 => Event::UnbalancedUnmask,
            4 if running.is_none() && rng.below(4) == 0 => Event::MaskWithNoJob,
            5..=28 if running.is_some() => Event::Complete,
            // §3.1.1, Terms: only a job changes the depth, so a region is opened by a running job.
            29..=40 if depth < 4 && running.is_some() => Event::Mask,
            41..=52 if depth > 0 => Event::Unmask,
            53..=65 => Event::Release(rng.below(N)),
            _ if !free.is_empty() => Event::Release(free[rng.below(free.len())]),
            _ => Event::Release(rng.below(N)),
        };
        match event {
            Event::Mask => depth += 1,
            Event::Unmask => depth -= 1,
            // §3.1.1 rule 4: a completion closes every section its job opened.
            Event::Complete => depth = 0,
            _ => {}
        }
        trace.push(event);
        let before = running;
        let was_masked = core.is_masked();
        let stepped =
            step(&mut core, &mut reference, event, &mut running, &mut halted).map_err(|why| {
                format!(
                    "seed {seed}, step {}: {why}\n  trace     {trace:?}",
                    trace.len()
                )
            })?;

        let core_overruns = stepped
            .core
            .iter()
            .filter(|t| {
                matches!(
                    t,
                    CoreTransition::JobSkipped { .. }
                        | CoreTransition::Faulted {
                            fault: rt_core::Fault::Overrun { .. },
                            ..
                        }
                )
            })
            .count();
        if !matches!(event, Event::MonitorOverrun(_)) {
            coverage.overruns_by_release += stepped.reference_overruns.min(core_overruns);
        }
        for transition in &stepped.core {
            match transition {
                CoreTransition::OverrunLatched { .. } => coverage.doubled_latches += 1,
                CoreTransition::JobSkipped { .. } => coverage.skipped_jobs += 1,
                _ => {}
            }
        }
        match event {
            Event::Complete if was_masked => coverage.masked_completions += 1,
            Event::MonitorOverrun(_) if was_masked => coverage.masked_escalations += 1,
            Event::MonitorOverrun(_) => coverage.contained_monitor_overruns += 1,
            Event::Synchronous(_, Raised::InJob) => coverage.synchronous_halts += 1,
            Event::Synchronous(_, Raised::InKernel) => coverage.kernel_faults += 1,
            Event::UnbalancedUnmask | Event::MaskWithNoJob => coverage.unbalanced_unmasks += 1,
            _ => {}
        }
        match (event, before, running) {
            (Event::Release(_), Some(from), Some(to)) if from != to => coverage.preemptions += 1,
            (Event::Unmask, _, Some(_)) if was_masked => coverage.latched_deliveries += 1,
            (Event::Complete, _, _) => coverage.completions += 1,
            (_, _, None) => coverage.idle_periods += 1,
            _ => {}
        }

        let reference_halted = matches!(reference.processor(), Processor::Halted);
        if halted || reference_halted {
            if halted != reference_halted {
                return Err(format!(
                    "seed {seed}, step {}: only {} halted\n  trace     {trace:?}",
                    trace.len(),
                    if halted { "rt-core" } else { "the reference" }
                ));
            }
            // Both halted: compare what each kept (§3.1.1 rule 7). The task tables are left to
            // the implementation (`d9`), and neither answers a later event by changing anything.
            let Decision::Halt { fatal } = core.decide() else {
                unreachable!("rt-core reported a halt");
            };
            let Some(record) = reference.fatal_record() else {
                return Err(format!(
                    "seed {seed}, step {}: the reference halted and kept no record",
                    trace.len()
                ));
            };
            if !same_record(fatal, record) {
                return Err(format!(
                    "seed {seed}, step {}: the halts kept different records\n  trace     {trace:?}\n  rt-core   {fatal:?}\n  reference {record:?}",
                    trace.len()
                ));
            }
            return Ok(());
        }
        let got = core_snapshot(&core, halted, running);
        let want = ref_snapshot(&reference);
        if got != want {
            return Err(format!(
                "seed {seed}, step {}: divergence\n  trace     {trace:?}\n  rt-core   {got:?}\n  reference {want:?}",
                trace.len()
            ));
        }
        // The reference carries its own invariants; a violation there is a harness bug or a
        // reference bug, and either way it must not be mistaken for agreement.
        if let Err(violation) = reference.check_invariants() {
            return Err(format!(
                "seed {seed}: the reference violated its own invariant: {violation:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn differential_over_many_randomised_sequences() {
    const SEQUENCES: u64 = 400;
    // Every pattern of the two policies over the three ranks that matters: all of one, all of the
    // other, and each kind above the other.
    let patterns = [
        [OverrunPolicy::Fault; N],
        [
            OverrunPolicy::SkipLateJob,
            OverrunPolicy::Fault,
            OverrunPolicy::Fault,
        ],
        [OverrunPolicy::SkipLateJob; N],
        [
            OverrunPolicy::Fault,
            OverrunPolicy::SkipLateJob,
            OverrunPolicy::SkipLateJob,
        ],
    ];
    let mut failures: Vec<String> = Vec::new();
    let mut coverage = Coverage::default();
    for seed in 1..=SEQUENCES {
        let policies = patterns[usize::try_from(seed).expect("small") % patterns.len()];
        if let Err(why) = run_sequence(seed, 60, policies, &mut coverage) {
            failures.push(why);
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {SEQUENCES} sequences diverged. First three:\n\n{}",
        failures.len(),
        failures
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    );
    eprintln!("{coverage:?}");

    // ⭐ The agreement above means nothing unless the sequences reached the states that matter.
    // These floors are well below what the generator currently produces; they exist to fail if a
    // future change to the event mix quietly stops exercising a behaviour. Measured 2026-10-01
    // over these seeds, again when `M2.9` step 6c added kernel faults and unbalanced unmasks, which
    // end sequences early, and again when 6f let only a running job open a region, which makes
    // regions rarer: each floor is at most about half of what the generator reached.
    assert!(coverage.preemptions >= 500, "{coverage:?}");
    assert!(coverage.latched_deliveries >= 200, "{coverage:?}");
    assert!(coverage.completions >= 1_000, "{coverage:?}");
    assert!(coverage.idle_periods >= 1_400, "{coverage:?}");
    assert!(coverage.overruns_by_release >= 800, "{coverage:?}");
    assert!(coverage.doubled_latches >= 200, "{coverage:?}");
    assert!(coverage.skipped_jobs >= 700, "{coverage:?}");
    assert!(coverage.masked_completions >= 250, "{coverage:?}");
    assert!(coverage.contained_monitor_overruns >= 100, "{coverage:?}");
    assert!(coverage.masked_escalations >= 25, "{coverage:?}");
    assert!(coverage.synchronous_halts >= 20, "{coverage:?}");
    assert!(coverage.kernel_faults >= 50, "{coverage:?}");
    assert!(coverage.unbalanced_unmasks >= 15, "{coverage:?}");
}

#[test]
fn the_two_models_are_not_the_same_model() {
    // ⭐ A sanity check on the whole exercise. If the reference were a copy of `rt-core`, it would
    // share its shape — and it does not: it refuses things `rt-core` accepts, and carries states
    // `rt-core` has no name for. This asserts the *structural* difference, so that a future
    // "simplification" that made the reference mirror the implementation would fail here rather
    // than quietly turn the differential test into a tautology.
    // The reference keeps every admissibility rule in its boot; `rt-core`'s plain constructor
    // takes indices and checks nothing, and only its description route, `from_eadl_ranks`, does.
    let empty: Result<Runtime<0>, _> = Runtime::boot([]);
    assert!(empty.is_err(), "the reference refuses an empty task set");
    let mut none = Scheduler::<0>::new([]);
    assert_eq!(none.decide(), Decision::Idle);

    // The reference takes a synchronous fault's context as a parameter, naming the task or one of
    // four kernel contexts, and refuses one the processor contradicts; `rt-core` is told only job
    // or kernel and reads the task from its own running one.
    let mut r = reference([OverrunPolicy::Fault; N]);
    assert_eq!(
        r.raise_unexpected_trap(RefContext::Task(TaskId::from_index(0))),
        Err(Refused::NoTaskRunning)
    );
    r.release(TaskId::from_index(0))
        .expect("boots idle, so this dispatches");
    assert_eq!(
        r.raise_unexpected_trap(RefContext::Task(TaskId::from_index(1))),
        Err(Refused::NotTheRunningTask)
    );
    assert_eq!(
        r.raise_unexpected_trap(RefContext::Idle),
        Err(Refused::NotIdle)
    );
    assert_eq!(
        r.raise_unexpected_trap(RefContext::Task(TaskId::from_index(0))),
        Ok(FaultEffect::Fatal)
    );
    assert_eq!(r.processor(), Processor::Halted);
    assert_eq!(r.release(TaskId::from_index(0)), Err(Refused::Halted));
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// THE GAPS, RESOLVED — AND WHAT THE RESOLUTION FOUND
//
// `d1`–`d5` once asserted the five places where `rt-core` and an independently derived model of
// the same contract behaved differently, each on both sides, so neither could drift and the list
// could not quietly shrink. `M2.9` resolved them in `ROADMAP.md` §3.1.1 and the priority record
// (`docs/decisions/decision_runtime-contract-gaps.md`), and each now asserts the agreement, on
// both sides, for the same reason. ⛔ Rewritten, never deleted: §14.1 forbids dropping the only
// evidence that a gap was closed.
//
// `d6` and `d7` are the two behaviours the independent review of §3.1.1 sent to the director,
// ruled 2026-10-01. `d8`, and the second half of `d3`, are what rewriting these tests found: two
// places where `rt-core` departed from text the contract already had. `d9` is the one difference
// the contract leaves to the implementation.
// ═════════════════════════════════════════════════════════════════════════════════════════════

#[test]
fn d1_an_overrun_applies_its_policy_to_the_overrunning_task_in_both() {
    // Once undecided, and found twice: `rt-core` applied the policy at once, to the task that
    // overran, which need not be running; the reference reported the overrun, left escalation to
    // its caller, and attributed faults only to the running task. §3.1.1 rule 1 — detection
    // applies the policy — and rule 2 — an overrun belongs to the overrunning task, because the
    // single-core rule governs execution, not attribution — decided it.
    let mut core = Scheduler::<N>::new([OverrunPolicy::Fault; N]);
    let mut reference = reference([OverrunPolicy::Fault; N]);

    // Task 1 is released but task 0 outranks it, so task 1 is Ready and not running.
    for task in [1, 0] {
        core.release(task);
        reference
            .release(TaskId::from_index(task))
            .expect("accepted");
    }
    assert_eq!(core.decide(), Decision::Dispatch { to: 0 });
    assert_eq!(core.state(1), CoreState::Ready);

    // Now task 1 overruns while task 0 holds the processor.
    assert_eq!(
        core.release(1),
        CoreTransition::Faulted {
            task: 1,
            fault: rt_core::Fault::Overrun { task: 1 }
        }
    );
    assert_eq!(
        reference.release(TaskId::from_index(1)),
        Ok(ReleaseEffect::Overrun(FaultEffect::TaskStopped(None))),
        "applied on detection, to the task that is not running, moving no context"
    );
    assert_eq!(core.decide(), Decision::Continue { task: 0 });
    let both = ref_snapshot(&reference);
    assert_eq!(core_snapshot(&core, false, Some(0)), both);
    assert_eq!(both.tasks[1], Seen::Gone);
}

#[test]
fn d2_both_refuse_an_empty_task_set() {
    // Once undecided: "finite static task set" admits the empty set, which the reference refused
    // and `rt-core` accepted. §3.1.1 refuses it: a system with no workload makes §7.2's second
    // timing obligation vacuous.
    assert_eq!(
        Runtime::<0>::boot([]).unwrap_err(),
        rt_reference::BootError::NoTasks
    );
    assert_eq!(
        Scheduler::<0>::from_eadl_ranks([], []).unwrap_err(),
        rt_core::BootError::NoTasks
    );
}

#[test]
fn d3_both_refuse_rank_zero_and_order_tasks_by_rank() {
    // Once undecided: the priority record said "`1` is the highest" and nothing about `0`, while
    // `rt-core`'s index started at 0. The amended record refuses rank 0 and states the relation.
    // ⛔ Rewriting this test found that `rt-core` also refused ranks with a gap — `1, 5, 9` —
    // which the language admits and the reference accepts. Fixed priority uses only the order, so
    // the record now states `runtime index = |hp(i)|`, and `rt-core` lowers by it.
    let spec = |ranks: [u16; N]| {
        core::array::from_fn::<_, N, _>(|i| TaskSpec {
            name: ["a", "b", "c"][i],
            priority: Priority::new(ranks[i]),
            on_overrun: OverrunAction::StopTask,
        })
    };
    assert!(matches!(
        Runtime::boot(spec([0, 1, 2])),
        Err(rt_reference::BootError::PriorityRankZero { .. })
    ));
    assert_eq!(
        Scheduler::from_eadl_ranks([0, 1, 2], [OverrunPolicy::Fault; N]).unwrap_err(),
        rt_core::BootError::RankBelowOne { position: 0 }
    );

    // Ranks with gaps, described out of order: description positions 0, 1, 2 carry ranks 9, 1, 5,
    // so `rt-core` holds them at indices 2, 0, 1. Both run the rank-5 task over the rank-9 one.
    let ranks = [9, 1, 5];
    let mut reference = Runtime::boot(spec(ranks)).expect("distinct ranks, none zero");
    // The reference takes a 16-bit rank and `rt-core` the language's integer (leaf `M2.18`), so a rank
    // beyond `u16::MAX` cannot be compared; here both take the same small ones.
    let mut core = Scheduler::from_eadl_ranks(ranks.map(i64::from), [OverrunPolicy::Fault; N])
        .expect("distinct ranks, none below 1");
    reference
        .release(TaskId::from_index(0))
        .expect("dispatches");
    reference.release(TaskId::from_index(2)).expect("preempts");
    core.release(2);
    assert_eq!(core.decide(), Decision::Dispatch { to: 2 });
    core.release(1);
    assert_eq!(core.decide(), Decision::Switch { from: 2, to: 1 });
    assert_eq!(
        reference.processor(),
        Processor::Running(TaskId::from_index(2))
    );
}

#[test]
fn d4_a_containable_fault_inside_a_masked_region_escalates_in_both() {
    // Once undecided, and the one taken as an `rt-core` defect, its reasoning drawn entirely from
    // existing text: §3.1.1 rule 3. Terminating a job that holds the mask orphans the depth, and
    // containment means resuming the schedule from a state the section had not made consistent.
    let mut reference = reference([OverrunPolicy::SkipLateJob; N]);
    reference
        .release(TaskId::from_index(0))
        .expect("dispatches");
    reference.mask().expect("maskable");
    assert_eq!(
        reference.raise_overrun(TaskId::from_index(0)),
        Ok(FaultEffect::Fatal),
        "SkipLateJob would have contained it outside the region"
    );
    assert_eq!(reference.processor(), Processor::Halted);
    let record = reference.fatal_record().expect("halted");
    assert!(record.escalated, "and the record says rule 3 escalated it");

    let mut core = Scheduler::<N>::new([OverrunPolicy::SkipLateJob; N]);
    core.release(0);
    core.decide();
    core.mask().expect("maskable");
    core.fault(rt_core::Fault::Overrun { task: 0 }, CoreContext::Kernel);
    let Decision::Halt { fatal } = core.decide() else {
        panic!("rt-core escalates it too");
    };
    assert!(same_record(fatal, record), "{fatal:?} against {record:?}");
}

#[test]
fn d5_both_bound_mask_nesting_at_the_same_depth_and_halt_beyond() {
    // Once undecided: the reference refused beyond a bound; `rt-core` saturated a counter, which
    // stops counting, so the matching unmasks no longer balance. §3.1.1 first declared the bound
    // and refused beyond it; its review found that a refusal leaves the caller's matching `unmask`
    // to close the section early (finding 10), so exceeding it, and an `unmask` with nothing to
    // close, are assertion failures, which halt.
    assert_eq!(
        Scheduler::<N>::MASK_DEPTH_LIMIT,
        Runtime::<N>::MAX_MASK_DEPTH
    );
    // Only a job changes the depth (§3.1.1, Terms), so a job holds the processor first.
    let mut reference = reference([OverrunPolicy::Fault; N]);
    let mut core = Scheduler::<N>::new([OverrunPolicy::Fault; N]);
    reference
        .release(TaskId::from_index(0))
        .expect("dispatches");
    core.release(0);
    core.decide();
    for _ in 0..u8::MAX {
        reference.mask().expect("within the bound");
        core.mask().expect("within the bound");
    }
    assert_eq!(reference.mask(), Ok(MaskEffect::Fatal));
    assert_eq!(core.mask(), Err(rt_core::Refused::Halted));
    let Decision::Halt { fatal } = core.decide() else {
        panic!("rt-core halts too");
    };
    let record = reference.fatal_record().expect("halted");
    assert!(same_record(fatal, record), "{fatal:?} against {record:?}");
    assert!(reference.is_masked() && core.is_masked(), "neither wrapped");

    // Neither lost count before it: as many unmasks as masks close the region, and one more halts.
    let mut reference = self::reference([OverrunPolicy::Fault; N]);
    let mut core = Scheduler::<N>::new([OverrunPolicy::Fault; N]);
    reference
        .release(TaskId::from_index(0))
        .expect("dispatches");
    core.release(0);
    core.decide();
    for _ in 0..u8::MAX {
        reference.mask().expect("within the bound");
        core.mask().expect("within the bound");
    }
    for _ in 0..u8::MAX {
        assert!(matches!(reference.unmask(), Ok(UnmaskEffect::Unmasked(_))));
        core.unmask().expect("balanced");
    }
    assert!(!reference.is_masked() && !core.is_masked());
    assert_eq!(reference.unmask(), Ok(UnmaskEffect::Fatal));
    assert_eq!(core.unmask().unwrap_err(), rt_core::Refused::Halted);
    let Decision::Halt { fatal } = core.decide() else {
        panic!("rt-core halts too");
    };
    let record = reference.fatal_record().expect("halted");
    assert!(same_record(fatal, record), "{fatal:?} against {record:?}");
}

#[test]
fn d6_a_doubled_latch_is_judged_at_delivery_in_both() {
    // Findings §6 (b), ruled 2026-10-01 into §3.1.1 rule 1: a release into a full latch keeps the
    // overrun beside the release it holds, judged at delivery under the task's own policy — so the
    // pair ends exactly as it would have landing one instruction after the region closed.
    for policy in [OverrunPolicy::Fault, OverrunPolicy::SkipLateJob] {
        let policies = [OverrunPolicy::Fault, policy, OverrunPolicy::Fault];

        let mut core = Scheduler::<N>::new(policies);
        let mut reference = reference(policies);
        core.release(0);
        core.decide();
        reference
            .release(TaskId::from_index(0))
            .expect("dispatches");
        core.mask().expect("maskable");
        reference.mask().expect("maskable");
        assert_eq!(core.release(1), CoreTransition::Latched { task: 1 });
        assert_eq!(core.release(1), CoreTransition::OverrunLatched { task: 1 });
        for _ in 0..2 {
            assert_eq!(
                reference.release(TaskId::from_index(1)),
                Ok(ReleaseEffect::Latched)
            );
        }
        assert_eq!(
            reference.is_overrun_latched(TaskId::from_index(1)),
            Some(true)
        );
        assert_eq!(
            core.decide(),
            Decision::Continue { task: 0 },
            "nothing is judged inside the region"
        );
        core.unmask().expect("masked");
        reference.unmask().expect("masked");
        assert_eq!(core.decide(), Decision::Continue { task: 0 });
        let inside = core_snapshot(&core, false, Some(0));
        assert_eq!(inside, ref_snapshot(&reference), "{policy:?}");

        let mut after = Scheduler::<N>::new(policies);
        after.release(0);
        after.decide();
        after.release(1);
        after.release(1);
        assert_eq!(after.decide(), Decision::Continue { task: 0 });
        assert_eq!(
            inside,
            core_snapshot(&after, false, Some(0)),
            "{policy:?}: the region changes when the overrun is judged, not how"
        );
        let expected = match policy {
            OverrunPolicy::Fault => Seen::Gone,
            OverrunPolicy::SkipLateJob => Seen::Ready,
        };
        assert_eq!(inside.tasks[1], expected, "{policy:?}");
    }
}

#[test]
fn d7_a_completion_inside_a_masked_region_closes_it_in_both() {
    // Findings §6 (a), ruled 2026-10-01 into §3.1.1 rule 4: the job's sections end with it, nested
    // ones included, what was latched is delivered as at the outermost unmask, and the schedule is
    // decided after — so the completing task's own latched release starts it afresh.
    let policies = [OverrunPolicy::Fault; N];
    let mut core = Scheduler::<N>::new(policies);
    let mut reference = reference(policies);
    core.release(2);
    core.decide();
    reference
        .release(TaskId::from_index(2))
        .expect("dispatches");
    for _ in 0..2 {
        core.mask().expect("maskable");
        reference.mask().expect("maskable");
    }
    for task in [0, 2] {
        core.release(task);
        assert_eq!(
            reference.release(TaskId::from_index(task)),
            Ok(ReleaseEffect::Latched)
        );
    }

    let (completed, delivered) = core.complete(2);
    assert_eq!(completed, CoreTransition::Completed { task: 2 });
    assert_eq!(
        delivered.as_slice().collect::<Vec<_>>(),
        vec![
            CoreTransition::Released { task: 0 },
            CoreTransition::Released { task: 2 }
        ],
        "task 2's own latched release is a new job, not an overrun"
    );
    let closed = reference.complete().expect("task 2 runs");
    assert_eq!((closed.depth, closed.delivered, closed.overruns), (0, 2, 0));
    assert!(!core.is_masked() && !reference.is_masked());
    assert_eq!(core.decide(), Decision::Dispatch { to: 0 });
    assert_eq!(
        core_snapshot(&core, false, Some(0)),
        ref_snapshot(&reference)
    );
}

#[test]
fn d8_an_overrun_raised_without_a_release_starts_no_job_in_either() {
    // Found while rewriting `d1`. §3.1.1 rule 1 makes the *triggering* release the task's next job
    // under `SkipLateJob`; an overrun an execution-budget monitor raises has none. `rt-core` made
    // the task ready anyway, starting a job no release paid for, and reported it as an ordinary
    // release; the reference left it awaiting its next one. The rule decides it, so `rt-core` was
    // corrected, and reports the skip as itself.
    let policies = [OverrunPolicy::SkipLateJob; N];
    let mut core = Scheduler::<N>::new(policies);
    let mut reference = reference(policies);
    core.release(0);
    core.decide();
    reference
        .release(TaskId::from_index(0))
        .expect("dispatches");

    assert_eq!(
        core.fault(rt_core::Fault::Overrun { task: 0 }, CoreContext::Kernel),
        CoreTransition::JobSkipped { task: 0 }
    );
    assert!(matches!(
        reference.raise_overrun(TaskId::from_index(0)),
        Ok(FaultEffect::JobTerminated(Some(_)))
    ));
    assert_eq!(core.decide(), Decision::Idle);
    let both = ref_snapshot(&reference);
    assert_eq!(core_snapshot(&core, false, None), both);
    assert_eq!(both.tasks[0], Seen::Waiting);
}

#[test]
fn d9_what_a_halt_leaves_in_the_task_table_is_left_to_the_implementation() {
    // Both halt on a stack guard and both attribute it to the running task, as §3.1.1's table
    // has it. What each then shows in its task table differs, and the contract does not decide it:
    // §8.1 asks to "preserve a defined fatal handler and diagnostic evidence", not for a format.
    // Recorded as left to the implementation in `decision_runtime-contract-gaps.md`, and asserted
    // on both sides so a change to either is seen.
    let policies = [OverrunPolicy::Fault; N];
    let mut core = Scheduler::<N>::new(policies);
    let mut reference = reference(policies);
    core.release(1);
    core.decide();
    reference
        .release(TaskId::from_index(1))
        .expect("dispatches");

    let fault = rt_core::Fault::StackGuard { task: Some(1) };
    assert!(matches!(
        core.fault(fault, CoreContext::Job),
        CoreTransition::Halted { .. }
    ));
    let one = TaskId::from_index(1);
    assert_eq!(
        reference.raise_stack_guard(RefContext::Task(one), Guard::Task(one)),
        Ok(FaultEffect::Fatal)
    );
    let Decision::Halt { fatal } = core.decide() else {
        panic!("a stack guard halts");
    };
    assert_eq!(reference.processor(), Processor::Halted);
    let record = reference.fatal_record().expect("halted");
    assert_eq!(record.attribution, Attribution::Task(one));
    assert!(same_record(fatal, record), "{fatal:?} against {record:?}");

    // `rt-core` marks the attributed task and takes it off the processor; the reference freezes
    // the table as it stood when the fault was raised.
    assert_eq!(core.state(1), CoreState::Faulted);
    assert_eq!(
        reference.state(TaskId::from_index(1)),
        Some(RefState::Running)
    );
}
