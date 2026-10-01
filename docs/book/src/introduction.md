# archogen

## The idea, in plain words

Many small devices — a drone's flight controller, a pump, a box of sensors — run a tiny operating system whose
whole job is to make the device's tasks happen on time: read this sensor every 10 milliseconds, answer that
button within 15. Such a system is usually written by hand, or cut down from a general-purpose one, and at the end
nobody can say exactly which of its timing promises were checked and which were only hoped for.

archogen takes another route. You **describe** what your device needs — what the hardware offers, such as a timer
and a serial port; which tasks it runs; how often each one runs and how quickly it must finish — in a small
language called **eADL**. archogen's job is then to **build** an operating system for exactly that description, a
simulator of it, and a report that says what has been checked and what has only been assumed.

The rule that makes it work: a description says **what** is needed, never **how** to do it. How to drive the
timer, how to switch between tasks, how to lay out memory — that is archogen's job, done by its engine from a
catalog of reviewed parts. A description written that way can be built again for other hardware, and the promises
the report makes can be traced to what they rest on.

> **In one minute, for engineers.** eADL (Extended Architecture Description Language) describes platform
> capabilities, required services and a workload; the engine resolves it against a catalog of reviewed components,
> generates a specialized Rust image and a simulator, and emits a report in which every claim — configuration,
> function, timing, memory, start-up — carries its own evidence category. The first profile, `rt-static-up-v1`,
> is one core, a finite static task set with unique fixed priorities, static stacks, no heap, a timer and explicitly
> modelled bounded interrupt sources. The description–implementation boundary is enforced mechanically. Start with
> [the tour](tour.md).

## How it works

1. **Describe** the platform and the tasks in eADL ([Reading a description](reading.md)).
2. **Check** the description against the supported profile: archogen accepts it or says exactly what is wrong
   ([Checking a description](checking.md)).
3. **Build** the system from reviewed parts ([The runtime](runtime.md), [the catalog](catalog.md)).
4. **Analyse** it and **report** what each promise rests on ([What a report may claim](evidence.md),
   [What the scheduling checker establishes](analysis.md)).

## The precise rules

### The controlling boundary

eADL describes **functionality**: what a platform offers, what a system requires, and the constraints that must
hold. It contains **no implementation**. Algorithms, device implementations, register-programming sequences,
simulator models, provider selection, lowering rules and code generation belong to the engine and its versioned
knowledge bases. This is not a stylistic preference: it is the property that makes a description reusable across
implementations, and it is enforced mechanically rather than by review habit ([The boundary](boundary.md)).

| eADL describes | The engine decides and implements |
| --- | --- |
| Available timer functionality and required time-service properties | Counter extension, compare programming, interrupt handling, simulator behavior |
| Task-execution functionality, priority semantics, deadline constraints | Scheduler data structures, dispatch code, context saving, cost analysis |
| Memory capacity and required protection functionality | Concrete layout, region tables, privileged operations |
| Hardware connectivity and supported operating states | Drivers, initialization order, transition sequences, state-dependent models |
| Reusable sub-HW/sub-OS feature compositions | Realization graph, provider bindings, generated Rust, verification execution |

### What is being built first

The first supported family is the `rt-static-up-v1` profile: one active core, static unique task priorities, a
finite static task set, statically allocated stacks, no runtime heap, a timer interrupt plus explicitly modeled
bounded interrupt sources. Requests outside that profile receive an `unsupported-profile` diagnostic — never a
silently weakened guarantee ([The supported profile](profile.md)).

### What this project does not claim

- No global "verified" flag. Configuration validity, functional behavior, timing, memory bounds and startup
  behavior each carry their own status and their own evidence category.
- Testing does not become proof through repetition, a published algorithm proof does not verify its Rust
  implementation, and a declared capability is not hardware evidence.
- The first release may accurately provide *conditional* timing analysis plus *tested* implementation
  conformance, and must list the links that remain unproved.

## Today and ahead

archogen is early. Today it reads and checks descriptions, and an experimental path generates and runs a small
system on your computer; the parts of the real system — the runtime, the scheduling analysis, the catalog — are
being built and checked one at a time, and nothing runs on a board yet. [The tour](tour.md) shows exactly what is
real and what is ahead.

## How to read this book

[The tour](tour.md) follows one small system from its description to a board, and is the best next page. The
chapters after it each open in plain words and then give the precise rules; the technical depths are in the
**annexes**, every abbreviation is spelled out in [the glossary](glossary.md), and [the index](book-index.md) lists
every topic. The book is kept in lockstep with the code: where a chapter describes behavior, it names the code or
fixture that implements it. The direction and its exit gates live in `ROADMAP.md`; the work is tracked as task
trees under `docs/tasks/`.
