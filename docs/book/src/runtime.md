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

## Five states

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

## Masking is modelled, not assumed away

§8.1: *"Model synchronization and interrupt masking explicitly. Banning a mutex type does not
eliminate races or blocking."*

A release arriving while interrupts are masked is **latched**, not lost, and delivered when the
outermost critical section closes — highest priority first. Masking nests, so an inner section
cannot unmask early.

⛔ **A second release while one is still pending is an overrun, not a second pending job.** There
is nowhere to put it. Inventing somewhere would be a queue, in a profile that excludes queues,
and the analysis would be describing a different system from the one running.

## The fault path is a value

A fault is returned, not panicked. That is what lets a hosted test observe the whole path without
the process dying, and what lets the target port route it to a defined fatal handler instead of
whatever `panic_handler` happens to be linked.

§8.1 asks for three things to be kept apart, and the distinction the runtime acts on is whether
its **own state is still trustworthy**:

| Fault | Trustworthy after? | Because |
| --- | --- | --- |
| `Overrun` | yes | a workload event the scheduler fully understands; a declared policy applies |
| `StackGuard` | no | memory the runtime relies on may already be wrong |
| `UnexpectedTrap` | no | the cause is outside what was modelled |
| `InvariantViolated` | no | the runtime's own bookkeeping is inconsistent |

Only an overrun leaves the system schedulable. Everything else halts, because continuing means
running a system nobody analyzed.

⛔ There is **no** "ignore" overrun policy. §3.1 requires a *defined* one, and silently dropping
the late job would make the running task set differ from the analyzed task set with no record
that it happened. `SkipLateJob` exists, is legal where the analysis was told, and reports the
abandonment as an event.

## It has been checked against a model that never saw it

§12 M2 asks for an independent reference and says exactly what would make one worthless:

> A checker sharing the same erroneous recurrence with its reference does not qualify as
> independent.

So `crates/rt-reference` was derived by a separate context that was instructed not to read
`crates/rt-core`, this chapter, or the task tree — working from `ROADMAP.md` §3.1/§8/§8.1 and the
priority decision record alone. Its API came out visibly different: explicit `Priority` newtypes
instead of index-as-rank, a separate `Processor` state, operations returning `Result`, dispatch
performed eagerly inside `release`. That difference is evidence, not friction.

`crates/rt-core/tests/differential.rs` drives both through **400 randomised sequences of 40
events** and compares what a user of either could observe, with coverage floors so the agreement
cannot be vacuous — the sequences must actually reach preemption, latched delivery, completion
and idle. In the region the contract decides, the two agree exactly.

⭐ **And the agreement is the weak result.** Two models that disagree cannot have been copied from
each other, so every divergence is simultaneously proof of independence and a real defect. There
are **five**, and every one turned out to be a question the roadmap does not answer:

| # | The question the contract leaves open |
| --- | --- |
| 1 | does *detecting* an overrun apply its policy, and may a fault attach to a task that is not running? |
| 2 | is an empty task set admissible? |
| 3 | what does priority rank `0` mean — and which end of the range is the runtime's index? |
| 4 | what happens when a containable fault is raised inside a masked region? |
| 5 | what bounds mask nesting, and what happens at the bound? |

They are **asserted on both sides** as ratchet tests, so neither model can drift and the list
cannot quietly shrink by someone "fixing" one side. Resolving four of them changes the roadmap,
which §14.1 makes a reviewed decision rather than an implementer's; they are set out with
recommendations in `docs/decisions/decision_runtime-contract-gaps.md` and owned by leaf `M2.9`.

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
