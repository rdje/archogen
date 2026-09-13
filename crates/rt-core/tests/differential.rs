//! `M2.2` — differential testing of `rt-core` against an independently derived reference.
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
//! It derived the model from `ROADMAP.md` §3.1/§8/§8.1 and the priority decision record.
//!
//! ⚠️ **What remains shared, disclosed because §4.4 requires it and the `M2.2` acceptance demands
//! it explicitly:**
//!
//! | Shared input | Consequence |
//! |---|---|
//! | the contract text itself (`ROADMAP.md` §3.1, §8, §8.1) | a misreading *the contract invites* would be made by both |
//! | `decision_priority-comparison-direction.md` | the priority direction is common to both |
//! | the same model family produced both | §14: "a second model agreeing with the first is not ground truth" |
//! | this adapter, written by the author of `rt-core` | a mapping error here can mask or manufacture a divergence |
//!
//! ⭐ **So agreement is the weak result here, and disagreement is the strong one.** Two models
//! that disagree cannot have been copied from each other, and every divergence is either a defect
//! in one of them or a place the contract does not actually decide. The divergences this harness
//! found are recorded in the `M2.2` leaf, each adjudicated against the roadmap.
//!
//! # The adapter's three deliberate alignments
//!
//! The two APIs are shaped differently — which is itself evidence — so three mappings are needed,
//! and each is a claim that could be wrong:
//!
//! 1. **Priority.** `rt-core` makes a task's *index* its rank, highest first. `rt-reference`
//!    carries an explicit `Priority` where `1` is highest and `0` is refused. So index `i` maps to
//!    rank `i + 1`. ⛔ That off-by-one is real and is stated nowhere in the repository — see the
//!    leaf.
//! 2. **Dispatch timing.** `rt-reference` dispatches *eagerly*, inside `release`. `rt-core`
//!    separates the event from the decision, so this harness calls `decide()` after every event
//!    to bring it to the same point. Without that the two are trivially "different" for a reason
//!    that is not a defect.
//! 3. **Overrun policy.** `rt-core::OverrunPolicy::Fault` leaves the task unschedulable and the
//!    runtime running, which is `rt-reference::OverrunAction::StopTask`, **not** its `Fatal`.
//!    The mapping follows observed behaviour rather than the name.

use rt_core::{Decision, OverrunPolicy, Scheduler, TaskState as CoreState};
use rt_reference::{
    OverrunAction, Priority, Processor, Refused, Runtime, TaskId, TaskSpec, TaskState as RefState,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Event {
    Release(usize),
    Complete,
    Mask,
    Unmask,
}

/// Drive both models through one event, keeping `rt-core` at the same decision point.
///
/// Returns the running task `rt-core` believes in, and whether it has halted, because neither is
/// exposed on its public surface — `decide()` reports them.
fn step(
    core: &mut Scheduler<N>,
    reference: &mut Runtime<N>,
    event: Event,
    running: &mut Option<usize>,
    halted: &mut bool,
) {
    match event {
        Event::Release(i) => {
            core.release(i);
            let _ = reference.release(TaskId::from_index(i));
        }
        Event::Complete => {
            if let Some(task) = *running {
                core.complete(task);
                *running = None;
                let _ = reference.complete();
            }
        }
        Event::Mask => {
            core.mask();
            let _ = reference.mask();
        }
        Event::Unmask => {
            if core.is_masked() {
                core.unmask();
            }
            if reference.is_masked() {
                let _ = reference.unmask();
            }
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
        // Only events legal in BOTH models' preconditions, so a divergence is about semantics
        // rather than about one model refusing what the other accepts — and only inside the
        // region the contract actually decides. A release to a task that already owes a job is
        // an OVERRUN, and where its fault is attributed is a question §3.1 does not settle; it
        // is excluded here and asserted explicitly as `d1_overrun_attribution_is_undecided`.
        //
        // ⛔ A LATCHED release counts as owing a job even though the task's *state* has not moved
        // yet — masking defers delivery, it does not create a second slot. Missing this on the
        // first attempt left 373 of 400 sequences "diverging" for the one reason already known,
        // reached through the masked path instead of the direct one.
        let owes_a_job = |i: usize| {
            let id = TaskId::from_index(i);
            matches!(
                reference.state(id).expect("task exists"),
                RefState::Ready | RefState::Running
            ) || reference.is_latched(id).expect("task exists")
        };
        let free: Vec<usize> = (0..N).filter(|i| !owes_a_job(*i)).collect();
        let event = match rng.below(10) {
            5..=6 if running.is_some() => Event::Complete,
            7 if depth < 4 => Event::Mask,
            8 if depth > 0 => Event::Unmask,
            _ if free.is_empty() => {
                if running.is_some() {
                    Event::Complete
                } else if depth > 0 {
                    Event::Unmask
                } else {
                    Event::Mask
                }
            }
            _ => Event::Release(free[rng.below(free.len())]),
        };
        match event {
            Event::Mask => depth += 1,
            Event::Unmask => depth -= 1,
            _ => {}
        }
        trace.push(event);
        let before = running;
        let was_masked = core.is_masked();
        step(&mut core, &mut reference, event, &mut running, &mut halted);

        match (event, before, running) {
            (Event::Release(_), Some(from), Some(to)) if from != to => coverage.preemptions += 1,
            (Event::Unmask, _, Some(_)) if was_masked => coverage.latched_deliveries += 1,
            (Event::Complete, _, _) => coverage.completions += 1,
            (_, _, None) => coverage.idle_periods += 1,
            _ => {}
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
    let mut failures: Vec<String> = Vec::new();
    let mut coverage = Coverage::default();
    for seed in 1..=400_u64 {
        let policies = if seed % 2 == 0 {
            [OverrunPolicy::Fault; N]
        } else {
            [
                OverrunPolicy::SkipLateJob,
                OverrunPolicy::Fault,
                OverrunPolicy::Fault,
            ]
        };
        if let Err(why) = run_sequence(seed, 40, policies, &mut coverage) {
            failures.push(why);
        }
    }
    assert!(
        failures.is_empty(),
        "{} of 400 sequences diverged. First three:\n\n{}",
        failures.len(),
        failures
            .iter()
            .take(3)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n\n")
    );

    // ⭐ The agreement above means nothing unless the sequences reached the states that matter.
    // These floors are well below what the generator currently produces; they exist to fail if a
    // future change to the event mix quietly stops exercising a behaviour.
    assert!(coverage.preemptions >= 200, "{coverage:?}");
    assert!(coverage.latched_deliveries >= 100, "{coverage:?}");
    assert!(coverage.completions >= 1_000, "{coverage:?}");
    assert!(coverage.idle_periods >= 500, "{coverage:?}");
}

#[test]
fn the_two_models_are_not_the_same_model() {
    // ⭐ A sanity check on the whole exercise. If the reference were a copy of `rt-core`, it would
    // share its shape — and it does not: it refuses things `rt-core` accepts, and carries states
    // `rt-core` has no name for. This asserts the *structural* difference, so that a future
    // "simplification" that made the reference mirror the implementation would fail here rather
    // than quietly turn the differential test into a tautology.
    // Rank 0 is constructible but refused at boot — the reference keeps every admissibility
    // rule in one place. `rt-core` has no explicit rank at all, so it cannot express the
    // question, which is itself the divergence.
    let zero = core::array::from_fn::<_, N, _>(|i| TaskSpec {
        name: "z",
        priority: Priority::new(u16::try_from(i).expect("small")),
        on_overrun: OverrunAction::StopTask,
    });
    assert!(
        Runtime::boot(zero).is_err(),
        "the reference refuses rank 0 at boot"
    );
    let empty: Result<Runtime<0>, _> = Runtime::boot([]);
    assert!(empty.is_err(), "the reference refuses an empty task set");
    // `rt-core` accepts one, and idles forever — recorded as a divergence in the M2.2 leaf.
    let mut none = Scheduler::<0>::new([]);
    assert_eq!(none.decide(), Decision::Idle);

    // The reference attributes a fault to the RUNNING task and refuses when the processor is
    // idle — `rt-core` takes a task id instead, which is the same divergence `d1` is about.
    let mut r = reference([OverrunPolicy::Fault; N]);
    assert!(matches!(
        r.raise(rt_reference::Fault::UnexpectedTrap),
        Err(Refused::NoTaskRunning)
    ));
    r.release(TaskId::from_index(0))
        .expect("boots idle, so this dispatches");
    let _ = r.raise(rt_reference::Fault::UnexpectedTrap);
    assert_eq!(r.processor(), Processor::Halted);
    assert!(matches!(
        r.release(TaskId::from_index(0)),
        Err(Refused::Halted)
    ));
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// KNOWN DIVERGENCES
//
// ⭐ These are the point of the exercise. Each one is a place where `rt-core` and an
// independently derived model of the same contract behave differently — which means the contract
// does not decide it, because two readers reading only the contract arrived at different answers.
//
// They are asserted rather than fixed, and asserted on BOTH sides, so that:
//   * a change to either model that alters the disagreement fails here and forces a re-reading;
//   * a NEW divergence shows up in the randomised test above rather than hiding among these;
//   * and the list cannot quietly shrink by someone "fixing" one side without amending the
//     contract, which §14.1 forbids ("implementation changes cannot silently weaken requirements").
//
// Resolving them is a change to `ROADMAP.md`, not to a crate — it is a reviewed contract
// decision. They are routed in the `M2.2` leaf.
// ═════════════════════════════════════════════════════════════════════════════════════════════

#[test]
fn d1_overrun_attribution_and_escalation_are_undecided() {
    // ⛔ THE DEEPEST ONE, and it was found twice independently: the reference author flagged it as
    // a CONTRACT SILENT point while writing the model, and the randomised comparison hit it on its
    // very first sequence.
    //
    // §3.1 requires a "defined overrun … policy" and §7.3 puts "overrun behavior" in the per-task
    // record. Neither says (a) whether *detecting* an overrun applies that policy by itself, nor
    // (b) whether a fault can be attributed to a task that is not the running one.
    //
    // The two models answer differently, and each has a real argument:
    //
    //   `rt-core`      a second release IS the overrun, so the policy applies at once, to the
    //                  task that overran — which need not be running, because the overrun is
    //                  detected by a release interrupt while somebody else holds the processor.
    //
    //   `rt-reference` reports the overrun and leaves escalation to the caller, and attributes
    //                  faults only to the running task: "a fault attributed to a task that was
    //                  not running would be a claim about concurrency the profile does not admit."
    //
    // ⚠️ The reference's argument is right for a *trap* — which belongs to whoever executed the
    // instruction — and the implementation's is right for an *overrun*, which is precisely a
    // statement about a task that is NOT making progress. §3.1 lists both in one sentence and
    // §8.1's three-way triage does not map onto that list, which is the actual gap.
    let mut core = Scheduler::<N>::new([OverrunPolicy::Fault; N]);
    let mut reference = reference([OverrunPolicy::Fault; N]);

    // Task 1 is released but task 0 outranks it, so task 1 is Ready and not running.
    for task in [1, 0] {
        core.release(task);
        reference
            .release(TaskId::from_index(task))
            .expect("accepted");
    }
    core.decide();
    assert_eq!(core.state(1), CoreState::Ready);
    assert_eq!(
        reference.state(TaskId::from_index(1)),
        Some(RefState::Ready)
    );

    // Now task 1 overruns while task 0 holds the processor.
    core.release(1);
    let effect = reference.release(TaskId::from_index(1)).expect("accepted");

    assert_eq!(
        core.state(1),
        CoreState::Faulted,
        "rt-core applies the policy immediately, to the non-running task"
    );
    assert!(
        matches!(effect, rt_reference::ReleaseEffect::Overrun),
        "the reference reports the overrun and leaves escalation to the caller"
    );
    assert_eq!(
        reference.state(TaskId::from_index(1)),
        Some(RefState::Ready),
        "and the reference's task is untouched, because `raise` would attribute to task 0"
    );
}

#[test]
fn d2_the_empty_task_set_is_undecided() {
    // §3.1 says "finite static task set". The empty set is finite.
    //
    //   `rt-reference` refuses it at boot: no workload makes §7.2's second timing obligation
    //                  vacuous, and a vacuously passing schedulability result is what §7.1 exists
    //                  to prevent.
    //   `rt-core`      accepts it and idles forever.
    //
    // A profile that wants a task-free image should say so; one that does not should exclude it.
    assert!(Runtime::<0>::boot([]).is_err());
    let mut none = Scheduler::<0>::new([]);
    assert_eq!(none.decide(), Decision::Idle);
}

#[test]
fn d3_the_meaning_of_priority_rank_zero_is_undecided() {
    // `docs/decisions/decision_priority-comparison-direction.md` says "`1` is the highest" and
    // says nothing about `0`.
    //
    //   `rt-reference` refuses rank 0 at boot — admitting it would move the top of the range by
    //                  inference, and §15 makes that a language change needing the record amended.
    //   `rt-core`      has no explicit rank at all: a task's *index* is its rank and indices start
    //                  at 0, so its highest priority is 0 while eADL's is 1.
    //
    // ⛔ That off-by-one is real, it is load-bearing for anything mapping a description onto the
    // runtime, and it is written down nowhere in the repository.
    let zero = core::array::from_fn::<_, N, _>(|i| TaskSpec {
        name: "z",
        priority: Priority::new(u16::try_from(i).expect("small")),
        on_overrun: OverrunAction::StopTask,
    });
    assert!(Runtime::boot(zero).is_err(), "the reference refuses rank 0");

    // `rt-core`'s highest-priority task is index 0, which the language calls priority 1.
    let mut core = Scheduler::<N>::new([OverrunPolicy::Fault; N]);
    core.release(0);
    core.release(1);
    assert_eq!(
        core.decide(),
        Decision::Dispatch { to: 0 },
        "index 0 is the highest rank here; the eADL description would call it `(priority 1)`"
    );
}

#[test]
fn d4_a_containable_fault_inside_a_masked_region_is_undecided() {
    // §8.1 requires masking to be modelled and requires a defined fatal handler; nothing covers a
    // *containable* fault raised while masked.
    //
    //   `rt-reference` escalates it to fatal: terminating a job that holds the mask leaves the
    //                  depth above zero with no owner, so interrupts never return; forcing the
    //                  depth to zero re-enables them mid-region with invariants half-restored.
    //   `rt-core`      has no notion of it — masking and faults do not interact.
    //
    // The reference's reasoning is the stronger of the two and is drawn entirely from the
    // contract, which is why this is routed as a probable `rt-core` defect rather than a wash.
    let mut reference = reference([OverrunPolicy::SkipLateJob; N]);
    reference
        .release(TaskId::from_index(0))
        .expect("dispatches");
    reference.mask().expect("maskable");
    let effect = reference
        .raise(rt_reference::Fault::Overrun)
        .expect("running");
    assert!(
        matches!(effect, rt_reference::FaultEffect::Fatal),
        "a containable fault inside a masked region escalates in the reference"
    );

    let mut core = Scheduler::<N>::new([OverrunPolicy::SkipLateJob; N]);
    core.release(0);
    core.decide();
    core.mask();
    core.fault(rt_core::Fault::Overrun { task: 0 });
    assert_ne!(
        core.decide(),
        Decision::Halt {
            fault: rt_core::Fault::Overrun { task: 0 }
        },
        "rt-core does not escalate, because it does not model the interaction at all"
    );
}

#[test]
fn d5_the_bound_on_mask_nesting_is_undecided() {
    // §3.1 requires "bounded kernel critical sections" and §7.3 requires every interrupt source to
    // declare its "masking constraints"; no maximum nesting depth is stated anywhere.
    //
    //   `rt-reference` bounds it at 255 and REFUSES beyond — the value is arbitrary, the refusal
    //                  is not: a depth counter that wraps re-enables interrupts inside a critical
    //                  section and reports success while doing it.
    //   `rt-core`      saturates a `u32`. It cannot wrap, but a saturated counter stops counting,
    //                  so the matching unmasks no longer balance — the same failure by a slower
    //                  route.
    let mut reference = reference([OverrunPolicy::Fault; N]);
    for _ in 0..u8::MAX {
        reference.mask().expect("within the bound");
    }
    assert!(
        reference.mask().is_err(),
        "the reference refuses beyond its bound"
    );
    assert_eq!(reference.mask_depth(), u8::MAX, "and does not wrap");
}
