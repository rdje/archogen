# Profile `rt-static-up-v1`

The first supported family: a small, single-core, statically configured real-time executive.
This is a **deliberate initial profile**, not the eventual scope. Broader OS functionality is
admitted later through additional profiles, each with its own contracts and acceptance
evidence (`ROADMAP.md` §3.3).

The profile is the contract that decides **which systems the toolchain admits** and **what it
will claim about them**. It is declared as data in `crates/eadl-model/src/profile.rs`; this
page is its published form, and a test in that module fails if the two drift apart.

## Decisions

| Concern | Decision |
| --- | --- |
| Processor | one active core; one execution context runs at a time |
| Scheduling | static unique task priorities; preemption at the target's supported interrupt points; bounded kernel critical sections |
| Workload | finite static task set; periodic or sporadic releases with declared minimum separation; constrained deadlines; bounded release jitter |
| Task execution | separate statically allocated task stacks; bounded jobs; no self-suspension within a job |
| Resources | static task and kernel objects; no runtime heap allocation; no application mutexes |
| Communication | no general IPC; task-to-task queues require a later profile amendment with bounded semantics |
| Interrupts | timer interrupt plus explicitly modeled bounded sources; unknown or uncontrolled interrupt load is unsupported for timing assurance |
| Time | fixed clock configuration; explicit counter modulus, conversion rules, and interrupt delivery bounds |
| Isolation | trusted application components in one address space; no claim of isolation from malicious tasks |
| Fault response | defined overrun, unexpected-trap, stack-guard, and assertion failure policy; bounded diagnostic handling |
| Devices | timer and minimal observable output first; no filesystem, network stack, DMA driver, or general virtio dependency |
| Runtime | Rust `no_std` core with narrow architecture and MMIO boundaries |
| Host support | Linux and macOS development first; Windows after the primary pipeline works |

The scheduling model is fixed-priority **even when a test harness randomizes event ordering**
at permitted boundaries. Test exploration must never change the policy whose timing is being
analyzed.

## What this profile does not admit

A request for any of these returns an `unsupported-profile` diagnostic (exit code 12). It is
**never** silently reduced to a weaker guarantee — that is the whole point of naming them.
Each carries the obligation admitting it would add, so a refusal is a statement about work,
not a wall.

| Capability | What is excluded | The obligation it would add |
| --- | --- | --- |
| `task-migration` | moving a task between cores | meaningless on one active core, and its analysis requires a multicore interference model |
| `mixed-criticality-scheduling` | criticality levels with mode change on budget overrun | a distinct scheduling policy with its own theorem, applicability conditions, and mode-change semantics |
| `dynamic-clock-scaling` | changing clock frequency at run time | every execution bound is tied to a clock configuration; scaling invalidates them and changes the timer contract |
| `unbounded-blocking` | blocking whose duration has no declared bound | response-time analysis needs a bounded blocking term; an unbounded one makes the recurrence meaningless |
| `arbitrary-async-executors` | general asynchronous task executors and futures runtimes | introduces dynamic scheduling and self-suspension the admitted task model does not cover |
| `dynamic-task-creation` | creating or destroying tasks after boot | the task set is an input to the schedulability argument; a changing set has no single analyzed workload |
| `runtime-loaded-drivers` | loading device implementations at run time | provider selection, resource ownership, and initialization order are resolved and checked before the build |
| `application-mutexes` | application-level mutual exclusion between tasks | requires a resource-sharing protocol, a priority-inversion bound, and a blocking term in the analysis |
| `general-ipc` | general inter-task communication, including task-to-task queues | needs queue capacity, overflow semantics, and their response-time effects; a later profile amendment |
| `runtime-heap` | dynamic memory allocation after boot | static allocation is what makes the memory accounting checkable against the linked image |
| `memory-isolation` | isolation from malicious or faulty application components | all components share one address space here; isolation needs protected tasks and authority semantics |
| `unmodeled-interrupt-load` | interrupt sources whose arrival or cost is not declared | an omitted source that can run during the analyzed interval silently invalidates every timing result |
| `filesystem` | a filesystem service | protocol semantics, fault and recovery behavior, persistence, and bounded resource use are all out of scope |
| `network-stack` | a network protocol stack | unbounded external arrival and protocol state are outside the admitted workload model |
| `dma` | DMA-capable devices and non-coherent transfers | needs buffer ownership, cache maintenance, ordering, addressability, and bus contention analysis |
| `power-states` | dynamic power states and clock domains | feature availability becomes state-dependent, and transitions change timing bounds and time continuity |
| `multicore` | more than one active core | requires an explicit memory model, inter-core interrupts, and shared-resource interference analysis |
| `posix` | a POSIX personality | needs a named compliance subset, conformance tests, and specified divergences |

## Silence is not admission

A capability this profile has **never heard of** is not admitted by default. An unknown name
is a `missing-fact` for the caller to resolve, not a quiet yes. The engine distinguishes the
two: a known exclusion is refused by name with its reason; an unknown one is reported as a
fact the profile cannot evaluate.

## What a later profile costs

`ROADMAP.md` §3.3 and §12 M8+ set the order in which these are reconsidered, when a real
example demands them — bounded IPC and one sharing protocol first, then additional scheduling
policies, protected tasks and capabilities, timer variants and firmware mediation, multicore
and weak memory, DMA, power domains, appliances, and a POSIX personality last.

Every one of them **changes a named profile and its analysis obligations**. No earlier timing
or isolation result transfers automatically to a new profile: the evidence was produced under
the old profile's assumptions, and those assumptions are exactly what changed.
