# The supported profile

## The idea, in plain words

A bridge carries a sign with its load limit. Within the limit, the engineers who built it stand behind it; beyond
it, they promise nothing, and the sign is what tells a driver which case they are in. archogen's **profile** is that
sign. It says which systems archogen will build and what it will promise about them, and a system outside it is
refused out loud rather than built with its promises quietly weakened.

There is one profile today, `rt-static-up-v1`. Its name reads as what it covers: *real-time* systems that are
*static*, everything decided before the system starts, on a *uniprocessor*, one processor core; *v1* is its first
version. Concretely: a fixed list of tasks, each with its own priority, each started by the clock or by an event
no more often than a declared interval; memory set aside before start-up and none handed out while running; a
timer, and only the interrupts the description declares.

Much is left out: tasks sharing locks, messages between tasks, several cores, a file system, a network. Each
refusal says what admitting the capability would cost, so a "no" is also a to-do list.

> **In one minute, for engineers.** `rt-static-up-v1` (`docs/profiles/rt-static-up-v1.md`, `ROADMAP.md` §3.1): one
> active core; static unique priorities, preemptive, bounded kernel critical sections; a finite static task set,
> periodic or sporadic, constrained deadlines, bounded jitter; static stacks, no heap, no application mutexes, no
> general IPC; a timer plus explicitly modelled bounded interrupt sources; one address space and no isolation
> claim. A request outside it gets `unsupported-profile` (exit `12`), and an unknown capability is a
> `missing-fact`, never admitted. The exclusions are data in `crates/eadl-model/src/profile.rs`, and a test holds
> the published page to them.

## How it works

A **profile** is the contract that decides which systems archogen will admit, and what it
will claim about them. It is not a preference or a default — it is the boundary of every
guarantee the toolchain makes. `rt-static-up-v1` is a small, single-core, statically
configured real-time executive:

- **one active core**; one execution context runs at a time;
- **static unique task priorities**, preemption at the target's supported interrupt points,
  bounded kernel critical sections;
- a **finite static task set** with periodic or sporadic releases, declared minimum
  separation, constrained deadlines, bounded release jitter;
- **statically allocated stacks**, bounded jobs, no self-suspension within a job;
- **no runtime heap**, no application mutexes, no general IPC;
- a **timer interrupt plus explicitly modeled bounded sources** — unknown or uncontrolled
  interrupt load is unsupported for timing assurance;
- one address space, trusted components, **no claim of isolation** from malicious tasks.

The full decision table is in `docs/profiles/rt-static-up-v1.md`.

The scheduling model stays fixed-priority **even when a test harness randomizes event
ordering** at permitted boundaries. Test exploration never changes the policy whose timing is
being analyzed — otherwise the thing measured is not the thing shipped.

## The precise rules

### Refusal is a feature

Ask for something the profile does not admit and you get an `unsupported-profile` diagnostic
(exit code `12`). You do **not** get a system built to a weaker guarantee without being told.

Each exclusion is recorded with the **obligation admitting it would add**, so the refusal
says what the work is rather than just "no":

| Capability | The obligation it would add |
| --- | --- |
| `application-mutexes` | a resource-sharing protocol, a priority-inversion bound, a blocking term in the analysis |
| `general-ipc` | queue capacity, overflow semantics, and their response-time effects |
| `multicore` | an explicit memory model, inter-core interrupts, shared-resource interference analysis |
| `dynamic-clock-scaling` | every execution bound is tied to a clock configuration; scaling invalidates them |
| `unmodeled-interrupt-load` | an omitted source that can run during the analyzed interval invalidates every timing result |

…and thirteen more, all listed on the profile page with their reasons.

### Silence is not admission

A capability the profile has **never heard of** is not admitted by default. An unknown name
is a `missing-fact` for the caller to resolve, not a quiet yes.

### The list you read is the list the engine uses

The exclusions are **data** (`crates/eadl-model/src/profile.rs`), and the published page is
checked against that data by a test. Misspell one slug on the page and the build fails:

```text
assertion `left == right` failed: docs/profiles/rt-static-up-v1.md has drifted from the profile data
  left:  [… "network-stack", "posiks", "power-states" …]
  right: [… "network-stack", "posix",  "power-states" …]
```

A refusal that names a capability is only trustworthy if the name the engine refuses and the
name the documentation publishes cannot drift apart.

## Today and ahead

### Later profiles

New families are added in the order real examples demand them — bounded IPC and one sharing
protocol first, then additional scheduling policies, protected tasks and capabilities, timer
variants and firmware mediation, multicore and weak memory, DMA, power domains, appliances,
and a POSIX personality last.

Every one of them **changes a named profile and its analysis obligations**. No earlier timing
or isolation result transfers automatically: the evidence was produced under the old
profile's assumptions, and those assumptions are exactly what changed.
