# The runtime: decisions, not actions

`crates/rt-core` is the shared state machine every generated system runs on: the task lifecycle,
the fixed-priority ready structure, interrupt masking, and the bounded fault path. It is
`no_std`, allocation-free, and — the part worth understanding first — **it never performs a
context switch**.

## Why the split

`ROADMAP.md` §8 states the constraint before the feature list:

> **Separate policy state transitions from the execution substrate.** A hosted harness can inject
> preemption at modeled boundaries; the architecture port must also validate real asynchronous
> interrupt entry, context preservation, and return.

So `rt-core` decides *that* a switch is due and returns a decision; saving registers, changing
stacks and returning from an interrupt belong to the architecture port, which is reviewed
separately and cannot be tested without a target.

Two things follow, and both matter:

- The **policy is testable on a host**, which is exactly what `M2.1`'s acceptance asks for.
- A scheduling bug and a context-save bug stop being the same bug. When a system misbehaves,
  those are different investigations with different evidence, and a design that merges them
  merges the investigations too.

| Decision | Means | Cost under `cost-accounting/1` |
| --- | --- | --- |
| `Idle` | nothing is ready | — |
| `Continue { task }` | the running task keeps the processor | none: no switch happened |
| `Dispatch { to }` | enter from idle | an initial dispatch — there is no outgoing context to save |
| `Switch { from, to }` | preempt or resume | a save **and** a restore |
| `Halt { fault }` | the runtime's own state is no longer trustworthy | the fatal path |

## Four task states

```text
           release                 dispatch
 Suspended ────────▶ Ready ◀─────┬──────────▶ Running
     ▲                           │  preempt      │
     │                           └───────────────┘
     └──────────────── complete ─────────────────┘

 any ──── fault ────▶ Faulted   (terminal for that task)
```

`Suspended` and `Ready` are distinct because a release is an event with a *time*, and collapsing
them loses the instant a deadline is measured from. `Ready` and `Running` are distinct because on
one core exactly one task runs, and the transition between them is what costs a switch.

A task's **index is its priority rank** — index 0 is highest. That is the profile's "static
unique task priorities" made structural, and it is why "the highest-priority ready task" is a
scan for the first ready index.

⛔ The language counts from the other end of zero: eADL's highest rank is `(priority 1)`. So a
description reaches the scheduler through `Scheduler::from_eadl_ranks`, the one place that
performs `runtime index = |hp(i)|` — the number of tasks that outrank it, which is
`eADL rank − 1` when the ranks are exactly `1, 2, …, n` — and refuses at boot what §3.1.1 and
`decision_priority-comparison-direction.md` rule out: an empty task set, a rank below `1`, or two
tasks sharing a rank. Ranks with gaps are admitted, because only their order matters.

## Masking is modelled, not assumed away

§8.1: *"Model synchronization and interrupt masking explicitly. Banning a mutex type does not
eliminate races or blocking."*

A release arriving while interrupts are masked is **latched**, not lost, and delivered when the
outermost critical section closes — highest priority first. Masking nests, so an inner section
cannot unmask early, and the nesting is **bounded**: `mask` beyond `Scheduler::MASK_DEPTH_LIMIT`
is refused, and so is an `unmask` with nothing to close. A counter that wrapped would re-enable
interrupts inside a critical section while reporting success; one that saturated would stop
counting, so the unmasks would no longer balance. Both fail silently, so neither is allowed.

⛔ **A second release while one is still pending is an overrun, not a second pending job.** There
is nowhere to put it. Inventing somewhere would be a queue, in a profile that excludes queues,
and the analysis would be describing a different system from the one running.

When the second release arrives **while the first is still latched**, the latch keeps the overrun
beside the release it holds, and both are judged at delivery — outside the masked region, under
the task's own policy (§3.1.1 rule 1). `SkipLateJob` makes the later release the task's next job;
`Fault` takes the task out of the schedule. The alternative, judging it on arrival, would have
made an ordinary contained overrun fatal for landing one instruction inside a critical section
rather than one instruction after it.

A job may **complete inside a masked region it opened** (§3.1.1 rule 4). Its completion closes the
region — the depth returns to zero with the job — delivers what was latched as the outermost
`unmask` would, and the schedule is decided after. So a task that completes inside its own region
and finds its own release latched is released afresh, not overrun. That is why `complete` returns
the deliveries beside the completion.

## The fault path is a value

A fault is returned, not panicked. That is what lets a hosted test observe the whole path without
the process dying, and what lets the target port route it to a defined fatal handler instead of
whatever `panic_handler` happens to be linked.

§8.1 asks for three things to be kept apart, and the distinction the runtime acts on is whether
its **own state is still trustworthy**:

| Fault | §8.1 class | Attributed to | Trustworthy after? | Because |
| --- | --- | --- | --- | --- |
| `Overrun` | expected error | the overrunning task, which need not be running | yes | a workload event the scheduler fully understands; a declared policy applies |
| `StackGuard` | violated internal invariant | the task whose guard was breached | no | memory the runtime relies on may already be wrong |
| `UnexpectedTrap` | deliberate fatal trap | the running task, or none while none runs | no | the cause is outside what was modelled |
| `InvariantViolated` | violated internal invariant | the running task, or none while none runs | no | the runtime's own bookkeeping is inconsistent |

The mapping is `ROADMAP.md` §3.1.1's: §3.1 names the faults and §8.1 names the classes, and until
§3.1.1 nothing connected them. Only an overrun leaves the system schedulable. Everything else
halts, because continuing means running a system nobody analyzed — and the halt keeps the
**first** fault, because one the fatal path raises afterwards is a consequence, not the cause.

⛔ **And an overrun raised while interrupts are masked halts too** (§3.1.1 rule 3). Terminating a
job that holds the mask leaves the depth above zero with no owner, so interrupts never return;
forcing it to zero re-enables them inside a region whose invariants were half-restored. And
containment means resuming the schedule from a state the critical section had not finished
making consistent. §8.1 offers no third option, so the conservative direction wins: a fatal path
that was not strictly needed costs availability, a containment that was not safe costs
correctness silently.

⛔ There is **no** "ignore" overrun policy. §3.1 requires a *defined* one, and silently dropping
the late job would make the running task set differ from the analyzed task set with no record
that it happened. `SkipLateJob` exists, is legal where the analysis was told, and reports the
abandonment as its own transition, `JobSkipped` — never as an ordinary release, because a skipped
job is a missed deadline by another name. The release that found the job late becomes the next
job. An overrun found some other way, by an execution-budget monitor through `fault`, has no
such release, so it starts nothing: the task waits for its next one.

⏳ **What the contract states and the runtime does not carry yet.** The text's second review
(`M2.9`, step 6) made them explicit, and step 6c carries them into both models:
- a `mask` past the declared bound, and an `unmask` with nothing to close, are **assertion
  failures**, which halt — the refusals described above leave the caller's matching `unmask` to
  close a section early;
- a trap or an assertion raised in a service, the trap path, a transition or idle is **no task's**,
  and the interrupted task is recorded as interrupted, not as attributed;
- a halted runtime **changes nothing afterwards** — no release processed or latched — and its kept
  fault says whether rule 3 escalated it.

## It has been checked against a model that never saw it

§12 M2 asks for an independent reference and says exactly what would make one worthless:

> A checker sharing the same erroneous recurrence with its reference does not qualify as
> independent.

So `crates/rt-reference` was derived by a separate context that was instructed not to read
`crates/rt-core`, this chapter, or the task tree — working from `ROADMAP.md` §3.1/§8/§8.1 and the
priority decision record alone. Its API came out visibly different: explicit `Priority` newtypes
instead of index-as-rank, a separate `Processor` state, operations returning `Result`, dispatch
performed eagerly inside `release`. That difference is evidence, not friction.

`crates/rt-core/tests/differential.rs` drives both through randomised event sequences and
compares what a user of either could observe, with coverage floors so the agreement cannot be
vacuous — the sequences must actually reach preemption, latched delivery, completion and idle.

⭐ **And the agreement is the weak result.** Two models that disagree cannot have been copied from
each other, so every divergence is simultaneously proof of independence and a real defect. The
first comparison found **five**, and every one was a question the roadmap did not answer:

| # | The question the contract left open | Decided |
| --- | --- | --- |
| 1 | does *detecting* an overrun apply its policy, and may a fault attach to a task that is not running? | yes, and yes: an overrun belongs to the task that overran |
| 2 | is an empty task set admissible? | no |
| 3 | what does priority rank `0` mean — and which end of the range is the runtime's index? | rank `0` is refused; a task's index is the number of tasks that outrank it |
| 4 | what happens when a containable fault is raised inside a masked region? | it halts |
| 5 | what bounds mask nesting, and what happens at the bound? | a declared bound, and `mask` beyond it is refused |

Deciding them changed the roadmap — `ROADMAP.md` §3.1.1 and the priority decision record — which
§14.1 makes a reviewed decision rather than an implementer's. The reference was then re-derived
from the amended text by a context that still had not read `rt-core`. It reviewed the amendment as
it went, and found more: its two behaviour questions were ruled on `2026-10-01`, and are the
masking rules above — a second release latched beside the first, and a completion that closes its
own critical section.

The tests that once asserted each disagreement now assert the agreement, on both sides. They were
**rewritten, not deleted**: §14.1 forbids dropping the only evidence that a gap was ever closed.
The randomised comparison now also generates overrunning releases, doubled latches, completions
inside a masked region, overruns a monitor raises, and the synchronous faults, checking whom each
fault is attributed to. A mutation run that reverts each ruling in either model, one at a time,
turns the tests red.

Rewriting those tests found more again. Three were places where `rt-core` had departed from text
the contract already had — the trap's attribution, the monitor's overrun and the kept first fault
above — and were fixed. Two were new questions. A description may give its tasks ranks with gaps,
`1, 5, 9`, which the implementation refused and the reference accepted; ranks now lower by their
order. And what a halted runtime leaves in its task table differs — `rt-core` marks the faulted
task, the reference freezes the table — which the contract leaves to the implementation, so a
test asserts both sides. All of it is in `docs/decisions/decision_runtime-contract-gaps.md`.

⚠️ What is still **shared** between the two, and therefore could still produce a common error: the
contract text itself, the priority decision record, the adapter that drives both, and the fact
that the same model family produced each. §14 is explicit that "a second model agreeing with the
first is not ground truth", so this is evidence with a stated scope — not a proof.

## What is deliberately absent

§8: *"Use the simplest bounded structures adequate for the profile. Do not add a generic object
manager, reference counting, or an asynchronous service framework without a workload that needs
them."*

No heap, no `Vec`, no trait object, no allocation at all: the task set is a fixed-capacity array
sized by a const parameter, because the profile admits a finite static task set created at boot.
A test asserts the whole scheduler's size is a function of the task count alone — if a pointer
indirection appeared, it would fail.

⚠️ `#![no_std]` is active and verified on the host, which shows the core *can* run without `std`.
It does not show that it **does** build for a bare-metal target: that needs the target installed,
and the `integration` tier reports it as unavailable until it is. See
[Verifying the toolchain](verification.md).
