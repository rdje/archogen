# The runtime: decisions, not actions

## The idea, in plain words

Every system archogen generates runs a few **tasks**: small pieces of work done over and over, such as
reading a sensor every 10 milliseconds or refreshing a display every 30. A processor core does one thing at a
time, so something inside the system must keep deciding which task runs now. That something is the
**runtime**, and every generated system carries one.

Picture a kitchen with one cook and a row of orders, each with a time it must be ready by. The cook works on
the most urgent order; when a more urgent one comes in, the current one is set aside and picked up again
later. And when something goes wrong — an order taking far longer than it should, an oven failing — someone
must decide what happens next: give up on that one order and carry on, or stop the kitchen before things get
worse.

archogen splits that job in two. The runtime only **decides**: which task should run, and what a fault means.
A separate part, the **port**, written once for each kind of chip, **does**: it saves one task's registers and
restores another's. Because of the split, every decision can be tested on an ordinary computer, and a wrong
decision and a wrong register save are found by different tests rather than tangled into one bug.

> **In one minute, for engineers.** Fixed-priority preemptive scheduling of a static task set on one core.
> `crates/rt-core` is a `no_std`, allocation-free state machine that returns a decision — `Idle`, `Continue`,
> `Dispatch`, `Switch` or `Halt` — and never switches context itself; the architecture port does. Critical
> sections nest to a declared bound, and a release that arrives inside one is latched and judged when the
> section closes. An overrun is contained by the task's declared policy, `SkipLateJob` or `Fault`; every other
> fault halts the system and keeps the first one's record. The rules are the profile's fault contract,
> `docs/profiles/rt-static-up-v1-faults.md`, reviewed independently round after round, and a model derived
> independently from that text agrees with `rt-core` over randomised event sequences. The case-by-case
> mechanics are in [Annex A](annex-runtime.md).

## How it works

### A task's life

```text
           release                 dispatch
 Suspended ────────▶ Ready ◀─────┬──────────▶ Running
     ▲                           │  preempt      │
     │                           └───────────────┘
     └──────────────── complete ─────────────────┘

 any ──── fault ────▶ Faulted   (terminal for that task)
```

A task sleeps (`Suspended`) until its next **release**, the moment its next round of work becomes due. It is
then `Ready`, waiting for the processor. When the runtime picks it, it is `Running`. When it finishes the
round — it **completes** — it sleeps again until the next release. A more urgent task can take the processor
from it, which is **preemption**: it goes back to `Ready` and resumes later from where it stopped. A task that
fails is `Faulted` and stays out until the system is reset.

### Who runs next

Every task has a **priority**, `1` for the most urgent, and no two tasks share one. The rule is simple: the
most urgent ready task runs. When a release makes a more urgent task ready, it runs at once.

### Holding interrupts back

An **interrupt** is a signal — from the timer, or from a device — that makes the processor drop what it is
doing and run the system's own code for a moment. Sometimes a task must do a few steps without that happening:
updating two values that must always change together, say. So it **masks** interrupts: it asks the processor to
hold them back. A release that arrives while they are held waits — it is **latched** — and is dealt with the
moment the task unmasks. Masks can nest, a section inside a section, up to a declared depth.

### When something goes wrong

The profile names four faults:

- an **overrun** — a task released again before it finished its previous round;
- a **stack guard** — a task's stack ran into the guard placed below it;
- an **unexpected trap** — the processor stopped for a reason the system was not built to expect;
- an **assertion failure** — a check found something impossible, a failed `assert!` or a panic among them.

Only an overrun is contained. The task's declared policy says what happens to it — `SkipLateJob` drops the
late round and starts the next, `Fault` takes that task out — and every other task carries on, because the
runtime still knows exactly where it stands. Every other fault **halts** the whole system and keeps a record of
the first fault: after it, nothing the timing analysis promised is known to hold, and running on would mean
running a system nobody analysed.

## The precise rules

### Why the split

`ROADMAP.md` §8 states the constraint before the feature list:

> **Separate policy state transitions from the execution substrate.** A hosted harness can inject
> preemption at modeled boundaries; the architecture port must also validate real asynchronous
> interrupt entry, context preservation, and return.

So `rt-core` decides *that* a switch is due and returns a decision; saving registers, changing stacks and
returning from an interrupt belong to the architecture port, which is reviewed separately and cannot be tested
without a target. The policy is testable on a host, which is what `M2.1`'s acceptance asks for.

| Decision | Means | Cost under `cost-accounting/1` |
| --- | --- | --- |
| `Idle` | nothing is ready | — |
| `Continue { task }` | the running task keeps the processor | none: no switch happened |
| `Dispatch { to }` | enter from idle | an initial dispatch — there is no outgoing context to save |
| `Switch { from, to }` | preempt or resume | a save **and** a restore |
| `Halt { fault }` | the runtime's own state is no longer trustworthy | the fatal path |

### States and priorities

`Suspended` and `Ready` are distinct because a release is an event with a *time*, and collapsing them loses the
instant a deadline is measured from. `Ready` and `Running` are distinct because on one core exactly one task
runs, and the transition between them is what costs a switch.

A task's **index is its priority rank** — index 0 is highest — so "the highest-priority ready task" is a scan for
the first ready index. ⛔ The language counts from the other end: eADL's highest rank is `(priority 1)`. A
description reaches the scheduler through `Scheduler::from_eadl_ranks`, the one place that performs
`runtime index = |hp(i)|`, the number of tasks that outrank it, and refuses at boot what the fault contract and
`decision_priority-comparison-direction.md` rule out: an empty task set, a rank below `1`, or two tasks sharing
a rank. Ranks with gaps are admitted, because only their order matters.

### Masking

§8.1: *"Model synchronization and interrupt masking explicitly. Banning a mutex type does not eliminate races or
blocking."* A release arriving while interrupts are masked is latched, not lost, and delivered when the region
closes. The nesting is bounded: a `mask` beyond `Scheduler::MASK_DEPTH_LIMIT`, an `unmask` with nothing to close,
a `mask` or `unmask` with no job running, and a job dispatched or resumed, or idle entered, while a region is open
are each an assertion failure, which halts. A counter that wrapped, saturated or refused would each fail
silently, so none is allowed.

⛔ **A second release while one is still owed is an overrun, not a second pending job.** There is nowhere to put
it: somewhere would be a queue, in a profile that excludes queues, and the analysis would describe a different
system from the one running. How latched arrivals are judged, and what a completion inside a masked region
does, are in [Annex A](annex-runtime.md#latched-releases-and-completions-inside-a-region).

### The fault path is a value

A fault is returned, not panicked. That lets a hosted test observe the whole path without the process dying,
and lets the port route it to its one fatal handler — the same one a panic enters. §8.1 asks for three things to
be kept apart, and the distinction the runtime acts on is whether its **own state is still trustworthy**:

| Fault | §8.1 class | Attributed to | Trustworthy after? | Because |
| --- | --- | --- | --- | --- |
| `Overrun` | expected error | the overrunning task, which need not be running | yes | a workload event the scheduler fully understands; a declared policy applies |
| `StackGuard` | violated internal invariant | the context that raises it | no | memory the runtime relies on may already be wrong |
| `UnexpectedTrap` | outside the model, taken by the deliberate fatal trap | the context that raises it | no | the cause is outside what was modelled |
| `InvariantViolated`, §3.1's assertion failure | violated internal invariant | the context that raises it | no | the runtime's own bookkeeping, or a checked assumption, is wrong |

The executing context is told, not guessed: a fault from outside arrives with a `Context` — the running task's
`Job`, which includes a primitive it called and its completion path, or `Kernel` code: a service, the trap path,
a transition, idle. Kernel code is no task's, so a trap there blames nobody and records the task it interrupted
as interrupted.

The halt keeps one record, `Fatal`: the **first** fault, the task it is attributed to or none, the task it
interrupted, and whether rule 3 escalated it. One the fatal path raises afterwards is a consequence, not the
cause, so it never replaces the record, and a halted runtime changes nothing: every event after the halt answers
with the same record.

⛔ There is **no** "ignore" overrun policy. §3.1 requires a *defined* one, and silently dropping the late job
would make the running task set differ from the analysed one with no record. `SkipLateJob` reports the
abandonment as its own transition, `JobSkipped`, because a skipped job is a missed deadline by another name. A
release is the only overrun detector: the profile has no execution-budget monitor and no deadline monitor (the
fault contract's rules 1a and 6).

Panics, the masked-overrun rule and the record a board keeps are in [Annex A](annex-runtime.md#faults-in-detail).

### What is deliberately absent

§8: *"Use the simplest bounded structures adequate for the profile."* No heap, no `Vec`, no trait object, no
allocation at all: the task set is a fixed-capacity array sized by a const parameter, because the profile admits
a finite static task set created at boot. A test asserts the whole scheduler's size is a function of the task
count alone, so a pointer indirection that crept in would fail it.

## How we know it is right

§12 M2 asks for an independent reference, and says what would make one worthless: *"A checker sharing the same
erroneous recurrence with its reference does not qualify as independent."* So `crates/rt-reference` was derived
by a separate context that never read `crates/rt-core`, this chapter or the task tree, from the roadmap and the
written contract alone. Its API came out visibly different, which is evidence, not friction.

`crates/rt-core/tests/differential.rs` drives both through randomised event sequences and compares what a user of
either could observe, with coverage floors so the agreement cannot be vacuous. ⭐ **The agreement is the weak
result**: every disagreement it found was a question the roadmap had not answered, and deciding each one is how
the fault contract came to exist. That history is in [Annex A](annex-runtime.md#how-the-two-models-were-made-to-agree).

⚠️ What the two still **share** — the contract text, the priority record, the adapter that drives both, and the
model family that wrote each — could still produce a common error. §14 is explicit that "a second model agreeing
with the first is not ground truth", so this is evidence with a stated scope, not a proof.

## Today and ahead

- **Today:** `rt-core` is built and tested on a host, and compiles for the pinned bare-metal target,
  `riscv64imac-unknown-none-elf`, in the integration tier's `no-std-build` step (`scripts/no_std_build.sh`). The
  fault contract is settled: its fifteenth independent review found no defect, and the director approved the claims it
  narrows (`M2.9`).
- **Ahead:** no port exists yet, so `rt-core` has decided for no real interrupt. The port's catalog record comes
  next (`M2.12`): its assembly format is decided, and what the port must state of the fault contract's open points is
  under review — on the road to [the emulated machine and a board](targets.md).
