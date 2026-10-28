# The runtime plan: what a checked assignment is lowered into

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — the design for leaf `M4.1`, under independent review (`M4.1.1`) before any code relies on it;
  round 3 answered on `2026-10-28` by narrowing format `/1` to the hosted playground, its fourth round next
- **Owner / source:** leaf `M4.1.1` (`docs/tasks/M4.md`); `ROADMAP.md` §4.2's `archogen-plan`, §7.3, §7.5, §7.6, §10.1
  step 5 and §10.3; model §8 rules 5, 8, 10 and 12; the obligations `M3.3.8`, `M3.5`, `M3.5.5.7.3` and `M3.2.1` wrote
  into `M4.1`. Its review history is `docs/reviews/decision_runtime-plan-reviews.md`.

## The fact / decision

In plain words: once the search has found an assignment and the configuration checker has accepted it, the engine
lowers it into one more artifact for that system, the **runtime plan**. The plan says, in the form the code generator
and the build take, what the description does not say in that form:
- each task's parameters, and what releases it;
- the timer and the interrupt sources, and the order the controller takes the sources in;
- in which context each part of the system runs;
- what is initialised before what, and why;
- what the plan owes, and which leaf discharges it.

A fact no input supplies yet, such as a task's execution bound or a primitive's cost, is never invented. The plan
names it as **owed**, with its owner, and says it is incomplete.

The first format, `archogen-runtime-plan/1`, is for a system run on the **hosted playground**, whose clock and memory
the simulator provides (`M4.3`). Everything in it is determined by the description, the checked assignment and the
joint rules as they stand. Running on a target's memory and timer needs three designs that are not this record's:
which of a target's devices a system's device is (`M4.4`), how timer releases and an adapter share one alarm
(`M4.1.6`), and the sizes only catalog records state. They come with a second format, `/2` (§4, `.7`).

### 1. What it is lowered from, and when it refuses

The lowering is §10.1's step 5, "Lower to a runtime/build plan with initialization dependencies and memory
allocation", in the crate §4.2 names, **`crates/archogen-plan`**. That crate is the generator's, does no I/O, and takes
no external dependency (`decision_zero-dependency-engine-core.md` names "the planner"). Its inputs:
1. the elaborated program the description check accepted, as the typed `Program`, with its digest
   (`eadl_resolve::plan::program_digest`);
2. the assignment record, `archogen-assignment/2`, as the text written;
3. the configuration checker's result record, `archogen-result/1`, for that assignment and that program, as text.

It lowers **the record the checker accepted**, read back by the generator's own reader of `/2` (`M4.1.3`), and never
the search's memory. So the bindings built are the bindings checked. The plan's routes, multiplexing and initialisation
are its own to derive and the build's to verify (`M4.10`), as `M3.5`'s design says of them.

**Refusals, before anything is lowered**, each with the status vocabulary's code and a repair:
- the assignment record does not read, or names another format: `tool-failure`;
- its program digest is not the program's: `stale-input`, "resolve again";
- its `(adapters …)` names a catalog other than this engine's: `stale-input`, "resolve again". Obligations and costs
  are derived from the catalog, never written (`docs/specs/adapters/decision_engine-adapters.md` §5), so a plan
  resolved with another catalog would lower another's;
- the result record does not read; its producer is not `archogen-plan-check`; or its subject is a build, not a plan:
  `tool-failure`;
- its subject is not this assignment's sha256 and this program's digest, or its profile is not the assignment's:
  `stale-input`, "resolve again". The `Program` carries no profile, so the two records' profiles are compared;
- its status is not `ok`: `tool-failure`, naming both verdicts, the generator having accepted what the checker refused
  (`M3.5.1`'s contract).

**Refusals of what `/1` cannot lower**, each `unsupported-profile`, naming what is missing and the leaf that adds it:
- the system has placement memory (model §8 rule 10): laying an image out in memory is `/2`'s (`M4.1.7`);
- an initialisation component whose members' records state that one's initialisation calls another's code (§2.7),
  once a record can be a provider (`M3.7`).

**Provenance.** The lowering cannot see whether the result was produced by an installed, verified checker, nor its exit
code: those are the command line's to check (`M3.5.1`'s contract). So beside the plan it returns its **provenance**:
- the engine's version and the generator executable's sha256 (`TI-H14`);
- the result record's sha256, and the checker program's sha256 that record names;
- how the acceptance was obtained: *checked by the front*, the command line having run and verified the checker; or
  *taken as text*, which `TI-H14` makes experimental.

The provenance and the plan's own sha256 are the build's metadata (`RTP-H7`), never part of the plan's text (§3). The
emitter accepts a runtime plan only by lowering it again from the same inputs and comparing bytes (`RTP-H3`). A plan
*checked by the front* is then built only as lowered from a verified checker's acceptance. A plan *taken as text* is
built only marked experimental.

### 2. What the plan holds, and from whom

A runtime plan is an engine structure (`decision_eadl-engine-boundary.md`). Each section names its input. A **node**
below is a node of the joint rules' initialization graph (§2.7): a provider an active binding names (a block, a
platform or an adapter's instance), a service, or a policy: the three kinds of node
`crates/eadl-resolve/src/joint.rs` gives, `Provider`, `Service` and `Other`. A node's **text** is its name as the plan writes it, an instance's being its
`(adapter …)` text, and texts are compared byte by byte.

1. **Identity**, the plan's semantic inputs: the format, the profile, the system, the program's digest, the
   assignment's sha256 and the adapter catalog's version. Nothing that changes when only the engine or the checker is
   rebuilt is part of it (§10.3, "identical semantic plans").

2. **Tasks**, from the description, in runtime order. A task's line holds:
   - its **runtime index**, `|hp(i)|`, the number of tasks of smaller rank, and its **rank**. The emitter still
     builds the scheduler with `rt_core::Scheduler::from_eadl_ranks`, "the only supported way to lower a description
     onto a scheduler", and a test holds the plan's index equal to the scheduler's slot (`RTP-H1`);
   - its stable logical identity, `(task SYSTEM TASK)`;
   - its period, or its minimum separation;
   - its relative deadline and its reference event (`deadline-from`). The language gives no default, so where none is
     written the plan writes `unstated` and owes it to the description: §7.3 makes the reference event part of the
     record;
   - its release jitter, written as zero where none is stated: "Absent means zero, which is a claim the analysis
     relies on" (`docs/semantics/kinds/os-rt.eadl`);
   - its overrun policy, `fault` where none is stated, the profile's default (the same kind);
   - the services it names by `uses`. §7.3's "resource use" is these and what the bindings give them;
   - **what releases it**:
     - `timer` for a periodic task;
     - for a task with a minimum separation, the external source whose route names the task or a service it reaches
       through `uses` and `needs`, at any depth (model §8 rule 8). The plan writes `(source NAME every)` when the
       source's stated separation is at least the task's. Every arrival then releases the task, and the releases are
       as far apart as the task states. The analysis asks "whether every arrival releases the task"
       (`crates/rt-analysis/src/runtime.rs`);
     - `unknown` otherwise: where no route reaches the task, where several do, where the source states no separation,
       or where it states one shorter than the task's. In the last case the source can arrive faster than the task
       says it is released, and nothing says which arrivals release it, so the trigger is owed to the description. The
       analysis refuses a release it cannot bound.

   **Durations** are exact. Each is one reduced rational number of seconds, `N s` or `N/D s`, with `D > 1` and `N` and
   `D` coprime, so that `1.5 ms`, `1.50 ms` and `1500 us` are one text. The simulator's clock is exact, so no duration
   is converted (`RTP-H9`). Converting to a target timer's ticks is `/2`'s.

   **Owed for each task** to the application, the rest of §7.3's task record and what the analysis reads of the
   task's own code (`docs/specs/catalog/decision_runtime-composite-inputs.md` §1 and §2), which the boundary forbids a
   description to state (`docs/semantics/boundary/reject/`'s `application-entry-point.eadl` and
   `execution-bound.eadl`):
   - its entry point;
   - its execution bound, `C_i^app`;
   - each OS call it may make, with the most one job makes, `n_{i,p}`;
   - its masked runs, `CS_{i,r,e}^app` and `n_{r,e,p}`;
   - its task facts, among them `leaves-interrupt-hardware-alone`.

   Its stack bound is owed by `/2`, which lays the stacks out. The application inputs' format is `RTP-H5`'s.
   `archogen-runtime-plan/1` has no line for a supplied application fact, so `/1` is never complete.

3. **Bindings**, the assignment's `bind` lines, copied as written, so a consumer reads one artifact.

4. **Engine adapter instances**, each named as the plan writes it, `(adapter KIND OVER)` or
   `(adapter KIND OVER REQUESTER)`. Each line carries:
   - the value of each fact its kind offers `obliged`, the requirement's value at that exclusive instance, the
     "ordinary value" of the adapters record's §1, item 5, such as `deadline-over-delay`'s `delivery-bound`;
   - its kind's obligations, then its costs, each in the kind's written order, as the engine API's `Plan.obligations`
     lists them (the adapters record's §9).

   The values a cost's terms need, such as the wake's update period, are owed by the adapter's realization (`RTP-H2`).

5. **Interrupts.**
   - **The release timer** releases every task whose trigger is `timer`. In `/1` it is the simulator's clock, written
     `simulator`. Releases start at the simulator's time zero, after the last step of initialisation, and the simulator
     takes its timer only when a release is due (`RTP-H9`). Choosing a device's alarm on a target, and sharing it with
     an adapter's, is `M4.1.6`'s design.
   - **Timer routes**: each provider the joint rules report a route with a timer interrupt for, as
     `Joint::check` reports it (model §8 rule 8; `Obligation::Route` with `Interrupt::Timer`), with the requesters
     the route names. An adapter's cost interrupt is no interrupt of its own: `expiry` and `wake` are "one interrupt at
     its provider" (the adapters record's §2). The `/1` kinds program `relative-delay`, a consumed fact, so their
     provider's route is a timer route. Each cost is written on that route's line, with the instance that takes it. A
     shared instance's cost counts once.
   - **External sources**: each provider the joint rules report a route with an external interrupt for. Each has its
     minimum separation, or `unknown` where the description states none, the level it is delivered at, and the
     requesters the route names. Each is declared **deliverable**, its priority above the controller's threshold, as
     the composite-inputs record asks of "every declared source" (`crates/rt-analysis/src/compose.rs` reads it).
   - **The order among external sources**, `ahead(x)`, is the plan's to set (the composite-inputs record §1). The
     timer's place against them is the hardware's, by the catalog's `external-before-timer`, and the simulator's for
     the playground (`RTP-H9`), so the plan does not order the timer. Each external source gets a distinct **rank**,
     rank `1` taken first, so the order is strict by construction:
     - ascending by stated minimum separation, a source with `unknown` last;
     - ties broken on the provider's name.

     This is rate-monotonic over the sources' separations. It is a **heuristic**, and no optimality is claimed for it.
     Service is non-preemptive among sources, interrupts not nesting (the variant's condition 5), and for
     fixed-priority non-preemptive scheduling of tasks "DMPO is not optimal for constrained-deadline task sets"
     ([Davis](../book/src/ledger.md#davis-speedup-factors), citing George et al., 1996). Audsley's algorithm is
     optimal only for a test under which "The schedulability of a task τk may, according to test S, be dependent on
     the set of higher priority tasks, but not on the relative priority ordering of those tasks" ([Davis and
     Burns](../book/src/ledger.md#davis-burns-priority-assignment), Condition 1). The composition's bound for a
     source, `Δ_x`, reads the jitter `J_q = Δ_q` of each source `q` ahead of it, and `Δ_q` depends on the order of the
     sources ahead of `q` (the composite-inputs record §3). So that algorithm promises nothing here either. An
     analysis refusal under this order says that another order may admit the set (`RTP-H4`).

     Soundness does not rest on the order. `compose.rs` refuses an order that is not strict, a source with no rank and
     a source not deliverable, and otherwise takes the order given. It does rest on the run realising the ranks: the
     simulator takes the sources in rank order (`RTP-H9`), and the emitter configures the order it realises
     (`RTP-H1`).
   - **The enabled set**: `timer`, if any task's trigger is `timer`; each timer route's provider; each external
     source. A timer route is an item of its own, never folded into `timer`. The simulator raises no other interrupt
     (`RTP-H9`), so there is no platform's non-maskable interrupt or firmware trap to add.

     The analysis variant analyses one timer, the release timer, and model §8 rule 8 leaves a second timer to it. Its
     condition 5 compares the enabled set with the declared one, the sources and `timer`, and refuses an enabled
     interrupt no source declares: "unmodeled-interrupt-load" (`runtime.rs`). The scheduling checker declares no source
     for a timer route (`RTP-H4`). So a system with a timer route, one binding an adapter among them, is refused
     `unsupported-profile` by the analysis until `M4.1.6` gives that interrupt a charge. The plan carries the route,
     its costs and its requesters, and the analysis says why it cannot judge them.
   - **The interrupt model**, §7.3's per-source "masking constraints, and whether interrupt nesting is possible":
     interrupts do not nest, and every masked section masks every interrupt. These are the variant's conditions, which
     the plan states. Each external source's service cost, `service.<source>`, and each route's latency are owed to
     the catalog.

   `M4.1.2` amends model §8 rule 8, whose plan "enables each such interrupt and no other", to this enabled set: the
   timer where a task is released by it, and every route's interrupt, timer routes included.

6. **Execution contexts** (`M3.3.8`'s obligation). One line for each task and each node. Say that one requester
   **reaches** another when it names it by `uses` or `needs`, or is bound to it as its provider, at any depth. The
   contexts are:
   - `job` for a task, and for each service, policy or instance a task reaches. A task released by an interrupt runs
     as a job after it, so a task's line is `job` alone;
   - `(interrupt X)`, for a route at provider `X`, for the code its interrupt runs, a task's never:
     - a requester the route names that is bound to `X` itself, for the interrupt fact or a consumed fact: its
       handler, or the alarm it programs, is its own code;
     - an instance bound to `X` for a consumed fact, which takes its cost at `X`;
     - a requester the route names through such an instance, when the instance's kind runs its requester's code in
       that interrupt. The adapters record states no kind's either way, so the plan reads `expiry`, a delivery to its
       requester, as running it, and `wake`, an epoch advance no requester observes but by reading, as running the
       instance's alone (`M4.1.2` has the record state it);
     - each service, policy or instance that one of these reaches;
   - `initialisation`, for every node.

   A record states the contexts it supports (`M3.3.8`: "judged by inclusion, as a level is"). Once a record can be a
   provider (`M3.7`), the lowering judges each line against it and refuses `unsupported-profile` a context the record
   does not include (`RTP-H10`). Until then each node's contexts are owed. **No step of initialisation may wait on an
   interrupt**, an alarm's or a source's, since initialisation runs masked (`M3.3.8`). For a record that is an
   obligation on its initialisation entry, owed to the catalog (`RTP-H6`). For generated code, such as an epoch
   extender arming its wake, it is the emitter's (`RTP-H2`). `M4.10` checks the image (`RTP-H6`).

7. **Initialisation.**
   - The graph is the joint rules' initialization graph over the system and the assignment's bindings (model §8 rule
     12; `Joint::initialization`). `M4.1.3` adds its production computation: only the tests call it today.
   - Its edges are written with their reasons, as §10.1's "initialization dependencies" and `M4.1`'s Goal ask: one line
     for each pair of nodes, the earlier first, naming each reason `Joint::initialization` gives, a binding's fact or
     `named` for a `uses` or `needs`.
   - Its steps are its strongly connected components, in a topological order taken by repeatedly choosing, among the
     components whose predecessors have all been taken, the one whose least member's text is least.
   - A step names its members by their text, in order.
   - This engine initialises each node in **one phase**. Rule 12's "in two phases where it can" is a record's to state,
     with the entry each phase runs (`M3.7`).

   **A cycle stays as rule 12 has it.** Two services that call each other at run time can each initialise without the
   other (`M3.3.7`, held by `two_services_using_each_other_are_one_component_not_a_refusal`). So a component of several
   members is one step: the members are initialised in their texts' order, all before any successor. This carries an
   obligation: no member's initialisation calls another member's code. For a record it is stated by its initialisation
   entry (`RTP-H6`); for generated code it is the emitter's (`RTP-H2`); `M4.10` checks the image (`RTP-H6`). Where a
   record states such a call, the lowering refuses (§1).

   **The epoch**, the instant of every timer-released task's first nominal release, is the simulator's time zero,
   after the last step (`RTP-H9`). That is the plan's half of `releases-after-initialisation` (the composite-inputs
   record §2) on the playground. A task with a minimum separation is first released by an arrival, after the first
   enabling. On a target the epoch is `/2`'s.

8. **Regions**: each region of the system's hardware, as `Joint::map` reads it, with its base, size, provider, and the
   requesters bound to that provider, or none. A device's registers on a platform nothing binds, such as
   `periodic-three`'s `device.timer` on `soc.playground`, are still the build's to map (§7.6, §10.3's "resource
   ownership"). `/1` has no memory section: a system with placement memory is refused (§1), and a hosted one's memory
   is the playground's.

9. **Obligations**, each written with its kind, its subject and the leaves that discharge it, exactly as this table
   gives them:

   | Kind | Subject | Discharged by | From |
   | --- | --- | --- | --- |
   | an adapter kind's obligation, such as `update` | the instance's text | `M4.2 M4.8`: the generated code realizes it (`RTP-H2`), and the report lists it until it is discharged | the instance's kind (the adapters record's §9) |
   | `multiplex` | the service | `M4.2`: the service's realization serves every user through its one channel (`RTP-H2`) | `Obligation::Multiplex`, as `Joint::check` reports it |
   | `route` | the provider | `M4.2 M4.3`: the emitter configures what the plan enables (`RTP-H1`), and the simulator delivers it (`RTP-H9`) | `Obligation::Route`, timer and source alike |

   `Joint::check`'s third kind, `Obligation::Placement`, comes only with placement memory, which `/1` refuses. A
   service is multiplexed only where several requesters reach it and it takes a channel: the heartbeat's
   `console.write`, used by both tasks, takes none, so it owes no `multiplex`.

10. **Owed facts**, in one list, each a subject, a fact and its owner. Exactly these are written:

    | Subject | Facts | Owner | Where |
    | --- | --- | --- | --- |
    | each task | `entry-point`, `execution-bound`, `masked-runs`, `os-calls`, `task-facts` | `application` | always |
    | each task with no `deadline-from` | `deadline-from` | `description` | always |
    | each task whose trigger is `unknown` | `trigger` | `description` | always |
    | each service and policy node | `contexts`, `no-wait` | `catalog` | always |
    | each block or platform node | `no-wait` | `catalog` | always |
    | each instance | `contexts`, `no-wait`, and `cost-terms` where its kind has costs | `realization` | always |
    | each external source's provider | `service-cost`, `route-latency` | `catalog` | always |
    | each timer route's provider | `route-latency` | `catalog` | always |

    A service's `contexts` are the supported contexts of the record that realizes it, a provider's statement as
    `M3.3.8` makes them. The plan's last line says whether it is complete, and `/1`'s never is (§2.2).

§7.5's "selected provider IDs" are the bindings' providers. Selected catalog records join them when a record can be a
provider (`M3.7`), and the plan then carries each record's identity.

### 3. The format, `archogen-runtime-plan/1`

Canonical text, fixed here so that both writers and every reader derive it from this record alone:
- each line is one form and ends with a single newline, the last line included;
- tokens are separated by a single space, with no space after `(` or before `)`;
- nothing else is written: no comment, and no blank line.

The forms come in the order below. `NAME` is a declaration's name; `REQ` is a requester as `/2` writes one; `NODE` is a
node's text (§2); `DUR` is §2.2's duration; `HEX` is 64 lowercase hexadecimal digits; `INT` is a decimal integer with no
sign and no leading zero.

```text
(format "archogen-runtime-plan/1")
(profile PROFILE)
(system NAME)
(program "HEX")
(assignment "HEX")
(adapters VERSION)
(task INDEX (task SYSTEM NAME) (rank INT) RELEASE (deadline DUR) (deadline-from EVENT|unstated) (jitter DUR) (overrun POLICY) (released-by TRIGGER) (uses NAME …))
(bind REQ FACT PROVIDER)
(instance INSTANCE (obliged FACT DUR) … (obligations NAME …) (costs NAME …))
(timer simulator (releases (task SYSTEM NAME) …))
(timer-route NAME (requesters REQ …) (costs (INSTANCE NAME) …))
(source INT NAME (separation DUR|unknown) (level LEVEL|none) (deliverable yes) (requesters REQ …))
(enabled ITEM …)
(interrupt-model (nesting no) (masking all))
(context SUBJECT CONTEXT …)
(depends NODE NODE (because REASON …))
(step INT NODE …)
(region NAME REGION (base INT) (bytes INT|unknown) (requesters REQ …))
(obligation KIND SUBJECT (discharged-by LEAF …))
(owed SUBJECT FACT (by OWNER))
(complete yes|no)
```

**Values:**
- `RELEASE` is `(period DUR)` or `(min-separation DUR)`.
- `EVENT` and `POLICY` are written as the description writes them; `(deadline-from unstated)` where none is.
- `TRIGGER` is `timer`, `(source NAME every)` or `unknown`.
- A list with no members is written with its head alone: `(uses)`, `(requesters)`, `(costs)`.
- `(level none)` where the route states no level.
- `INSTANCE` is the instance's `(adapter …)` text. A cost on a route is `(INSTANCE NAME)`, `NAME` the cost's name in
  the kind.
- `ITEM` is `timer`, a timer route's provider name, or an external source's provider name.
- `SUBJECT` is a task's `(task SYSTEM NAME)` or a node's text in a `context` line; as §2.9's and §2.10's tables give it
  in an `obligation` or `owed` line.
- `CONTEXT` is `job`, `(interrupt NAME)` with a route's provider name, or `initialisation`.
- `REASON` is `(binding FACT)` or `named`.
- `KIND`, `LEAF` and `OWNER` are §2.9's and §2.10's: an adapter kind's obligation name, `multiplex` or `route`; a
  task-tree identifier; `application`, `catalog`, `description` or `realization`.
- `FACT` in `owed` is one of: `entry-point`, `execution-bound`, `os-calls`, `masked-runs`, `task-facts`,
  `deadline-from`, `trigger`, `service-cost`, `route-latency`, `cost-terms`, `contexts`, `no-wait`.

**Forms written only where §2 calls for them:**
- `(timer …)`, where any task's trigger is `timer`. Its `releases` are those tasks;
- `(timer-route …)` and `(source …)`, one for each such route;
- `(enabled …)`, where any interrupt is enabled;
- `(depends …)`, one for each pair of nodes with an edge;
- `(region …)`, where the system's hardware has regions;
- `(obligation …)`, one for each obligation §2.9 lists.

`(interrupt-model …)` is always written.

**Orders:**
- `task` lines by index, starting at `0`; `releases` items by index;
- `bind` lines in `/2`'s order;
- `instance` lines by their text. Within a line come the `obliged` items in the kind's offer order, then the
  `obligations` names in the kind's written order, then the `costs` names in the kind's written order;
- `timer-route` lines by provider name; `source` lines by rank, starting at `1`;
- on a route's line, `requesters` items and `costs` items each by their text; `uses` items by name;
- `enabled` items: `timer`, then timer routes by name, then sources by rank;
- `context` lines by subject's text; within a line, `job`, then each `(interrupt …)` by name, then `initialisation`;
- `depends` lines by the earlier node's text, then the later's; within a line, `(binding …)` reasons by fact, then
  `named`;
- `step` lines by index, starting at `1`;
- `region` lines by base, then by provider, then by region;
- `obligation` and `owed` lines by their text.

**The heartbeat, worked.** `examples/s0-heartbeat/system.eadl`, its assignment the one line
`(bind console.write observable-output console.uart)`, lowers to exactly this text, but for its two digests, written
`…` here and held by the golden:

```text
(format "archogen-runtime-plan/1")
(profile rt-static-up-v1)
(system heartbeat)
(program "…")
(assignment "…")
(adapters adapters/1)
(task 0 (task heartbeat beat) (rank 1) (period 1/100 s) (deadline 1/100 s) (deadline-from release) (jitter 0 s) (overrun fault) (released-by timer) (uses console.write))
(task 1 (task heartbeat chime) (rank 2) (period 3/100 s) (deadline 3/100 s) (deadline-from release) (jitter 0 s) (overrun fault) (released-by timer) (uses console.write))
(bind console.write observable-output console.uart)
(timer simulator (releases (task heartbeat beat) (task heartbeat chime)))
(enabled timer)
(interrupt-model (nesting no) (masking all))
(context (task heartbeat beat) job)
(context (task heartbeat chime) job)
(context console.uart initialisation)
(context console.write job initialisation)
(depends console.uart console.write (because (binding observable-output)))
(step 1 console.uart)
(step 2 console.write)
(owed (task heartbeat beat) entry-point (by application))
(owed (task heartbeat beat) execution-bound (by application))
(owed (task heartbeat beat) masked-runs (by application))
(owed (task heartbeat beat) os-calls (by application))
(owed (task heartbeat beat) task-facts (by application))
(owed (task heartbeat chime) entry-point (by application))
(owed (task heartbeat chime) execution-bound (by application))
(owed (task heartbeat chime) masked-runs (by application))
(owed (task heartbeat chime) os-calls (by application))
(owed (task heartbeat chime) task-facts (by application))
(owed console.uart no-wait (by catalog))
(owed console.write contexts (by catalog))
(owed console.write no-wait (by catalog))
(complete no)
```

Its platform, `host.playground`, is bound by nothing and is no node, so it has no line. `console.write` takes no
channel, so it owes no `multiplex` (§2.9).

**An adapter, worked.** `crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl`, whose `/2` record binds
`time.monotonic`'s four facts to `(adapter epoch-extender timer.short)` and the instance's five to `timer.short`, lowers
to exactly this text, but for its two digests:

```text
(format "archogen-runtime-plan/1")
(profile rt-static-up-v1)
(system short-counter)
(program "…")
(assignment "…")
(adapters adapters/1)
(task 0 (task short-counter beat) (rank 1) (period 1/100 s) (deadline 1/100 s) (deadline-from release) (jitter 0 s) (overrun fault) (released-by timer) (uses time.monotonic))
(bind time.monotonic counter-width (adapter epoch-extender timer.short))
(bind time.monotonic observation-coherent (adapter epoch-extender timer.short))
(bind time.monotonic tick-rate (adapter epoch-extender timer.short))
(bind time.monotonic unambiguous-horizon (adapter epoch-extender timer.short))
(bind (adapter epoch-extender timer.short) counter-modulus timer.short)
(bind (adapter epoch-extender timer.short) observation-coherent timer.short)
(bind (adapter epoch-extender timer.short) relative-delay timer.short)
(bind (adapter epoch-extender timer.short) tick-rate timer.short)
(bind (adapter epoch-extender timer.short) wrap-behavior timer.short)
(instance (adapter epoch-extender timer.short) (obligations update coherent) (costs wake))
(timer simulator (releases (task short-counter beat)))
(timer-route timer.short (requesters time.monotonic) (costs ((adapter epoch-extender timer.short) wake)))
(enabled timer timer.short)
(interrupt-model (nesting no) (masking all))
(context (adapter epoch-extender timer.short) job (interrupt timer.short) initialisation)
(context (task short-counter beat) job)
(context time.monotonic job initialisation)
(context timer.short initialisation)
(depends (adapter epoch-extender timer.short) time.monotonic (because (binding counter-width) (binding observation-coherent) (binding tick-rate) (binding unambiguous-horizon)))
(depends timer.short (adapter epoch-extender timer.short) (because (binding counter-modulus) (binding observation-coherent) (binding relative-delay) (binding tick-rate) (binding wrap-behavior)))
(step 1 timer.short)
(step 2 (adapter epoch-extender timer.short))
(step 3 time.monotonic)
(obligation coherent (adapter epoch-extender timer.short) (discharged-by M4.2 M4.8))
(obligation route timer.short (discharged-by M4.2 M4.3))
(obligation update (adapter epoch-extender timer.short) (discharged-by M4.2 M4.8))
(owed (adapter epoch-extender timer.short) contexts (by realization))
(owed (adapter epoch-extender timer.short) cost-terms (by realization))
(owed (adapter epoch-extender timer.short) no-wait (by realization))
(owed (task short-counter beat) entry-point (by application))
(owed (task short-counter beat) execution-bound (by application))
(owed (task short-counter beat) masked-runs (by application))
(owed (task short-counter beat) os-calls (by application))
(owed (task short-counter beat) task-facts (by application))
(owed time.monotonic contexts (by catalog))
(owed time.monotonic no-wait (by catalog))
(owed timer.short no-wait (by catalog))
(owed timer.short route-latency (by catalog))
(complete no)
```

How it is derived:
- the timer route is `Joint::check`'s. The instance's `relative-delay` binding takes `timer.short`'s channel, and the
  route names the requester bound to the instance, `time.monotonic` (`joint.rs`, `routes` and `relying`);
- the instance runs in `beat`'s job through `time.monotonic` and in the wake's interrupt. `time.monotonic` runs in the
  job alone, since a wake runs only the instance's code;
- `soc.short` is bound by nothing and is no node;
- the analysis refuses this plan under condition 5, `timer.short` being enabled and no source declaring it (§2.5).

The two texts are `/1`'s goldens, held by `M4.1.4`.

The plan's identity is the sha256 of its text. It has one writer and one reader in `crates/archogen-plan`, and a plan
the reader accepts writes back the same bytes, as `archogen-result/1`'s rule asks. The scheduling checker reads it with
a reader of its own, held to the same goldens, as `/2` has two readers: the generator's reader is generator code, which
the scheduling checker must not share (`RTP-H4`). Each golden is lowered from frozen fixture inputs, its result record
a fixed text, and it is never rewritten. **The rules of §2 are part of the format**: a change to any of them, an order or
the topological choice among them, is a new format number. `docs/book/src/versions.md` registers the format
(`VERSION-REGISTER`). Lowering the same inputs twice writes the same bytes (M4's "Repeated generation produces
identical canonical plans").

### 4. What `M4.1` builds, and what it hands on

**Built by `M4.1`'s children:**
- `.2`: model §8 rules 8 and 10 amended, in the model, the generator's joint rules and the checker's, the checker's by a
  fenced context. Rule 8's enabled set is §2.5's. Rule 10's are below. The adapters record's §9 reads "each cost a cost
  on the route of the provider it programs", and it states, for each cost, whether the interrupt runs the requester's
  code (§2.6). The engine API's `crates/archogen-api/src/resolve.rs` and `docs/book/src/engine-api.md`, which read "a
  cost the scheduling analysis charges as a source", read so too. The book follows, with a migration note for any
  verdict that moves. Rule 10's amendments keep the search, the checker and any later layout agreed over a description:
  - **placement memory with no size** is `missing-fact`, naming it, since nothing can be laid out in it;
  - **a hole of no size** whose base lies inside placement memory (`M3.5.5.7.3`'s obligation) is taken to reach the
    region's end. That is the conservative extent, the largest the region allows. If it leaves no free byte, the system
    is `missing-fact`, naming the hole and asking for its size;
- `.3`: the generator's reader of `/2`, its round trip held against its writer, and the typed record it returns; the
  initialization graph computed for a record's bindings;
- `.4`: the crate, the typed plan, its writer and reader, the two goldens, the lowering, its refusals and its
  provenance;
- `.5`: the register entry; the crate in the generator root's `(role-packages …)` in `trust/roots.eadl`, each item it
  shares with another role classified by its commit (F30); the book's chapter; and every other place that names `M4.1`
  as owner, moved as below;
- `.6`: **the release alarm**, a design of its own, reviewed. It decides which device's alarm releases timer tasks on a
  target, and how an adapter's alarm shares a device with releases, each taking a channel (model §8 rule 3). It
  decides how the analysis charges a timer route's interrupts, such as a wake taken as a timer event of the release
  timer under an amended condition 6. That design is the joint rules', the analysis variant's and the composite-inputs
  record's;
- `.7`: **format `/2`**, after `.6`, `M4.4`'s device correspondence (`RTP-H8`) and `M4.12`'s application inputs
  (`RTP-H5`). It is designed and reviewed as this record was. Its known questions are round 3's, kept in the review
  history:
  - a target as an input, and the target's hardware as every block and platform its file declares;
  - `Joint::map` over it;
  - the image's place below every allocation in its range;
  - the allocation's formula;
  - the interrupt stack with the fault path's use;
  - the epoch read and the first compare written as the last act before the transition into idle, so that its
    bound is the switch record's;
  - ticks and the horizon test over consecutive releases;
  - the catalog's numbers in the identity, with their evidence category;
  - the boot stack's owners.

**What else names `M4.1`**, discharged or moved by `.5`:
- `crates/eadl-model/src/profile.rs`'s rule "one execution context runs at a time", owner `M4.1`. The plan states it:
  no nesting, one core, and each requester's contexts. The runtime realizes it, so its owner moves to `M4.2`.
- S0's `whole-millisecond-periods` moves to `M4.2`, which the plan's exact durations let end. That covers its row in
  `decision_s0-retirement.md`, its marker and hint in `crates/archogen-s0/src/interpret.rs` and that file's test,
  `docs/book/src/s0.md`, and `examples/s0-heartbeat/README.md`.
- `no-lock-data` and `--locked` move to `M4.7`, which writes the whole lock data. That covers the row, the comments,
  hint and test in `crates/archogen-cli/src/build_cmd.rs` and `tests/s0_build.rs`, `resolve_cmd.rs`'s refusal of
  `--locked`, and `docs/book/src/cli.md`.
- `crates/archogen-api/src/resolve.rs`'s and `docs/book/src/engine-api.md`'s "charges as a source (`M4.1`)" are
  reworded by `.2`.
- The remarks naming `M4.1` in `crates/eadl-resolve/src/joint.rs` and its tests stand: they state this leaf's role
  truly.

**Written into other leaves with this record**, quoted word for word by each receiving leaf (`HANDOFF-LEDGER`):

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RTP-H1` | `M4.2` | The emitter moves `archogen build` onto the runtime plan for the hosted playground, its faces in step on the command line, the engine API and the [MCP](../book/src/ledger.md#mcp-specification) tool, the sources' ranks realized as the order the simulated controller takes them in, the scheduler built with `from_eadl_ranks` and its slot held to the plan's index, and S0's `whole-millisecond-periods` and the profile's "one execution context runs at a time" discharged. |
| `RTP-H2` | `M4.2` | The generated code realizes the adapters' and the multiplexed services' obligations, initialises no component member by calling another's code, never waits on an interrupt in initialisation, and states each cost's terms and each value the runtime plan owes to the realization. |
| `RTP-H3` | `M4.2` | The emitter accepts a runtime plan only by lowering it again from the same inputs and comparing bytes, and marks a plan whose acceptance was taken as text experimental. |
| `RTP-H4` | `M2.21.2` | After `M4.1.4` has written the runtime plan's goldens, the scheduling checker reads the plan with a reader of its own held to them, maps it onto the analysis's inputs declaring no source for a timer route, so that its interrupt is refused as unmodeled load, and words a refusal under the plan's source order as one another order may admit. |
| `RTP-H5` | `M4.12` | The application inputs have a format — each task's entry point, execution bound, OS calls with counts, masked runs, task facts, stack bound and its evidence category — that `archogen-runtime-plan/2` carries. |
| `RTP-H6` | `M4.10` | The catalog records state what the runtime plan owes them — supported contexts, an initialisation entry that never waits on an interrupt and names any other component member's code it calls, each source's service cost and each route's latency — amending the catalog record's §12 and §14.4, and the image is checked: no initialisation step waits on an interrupt, and no component member's initialisation calls another member's code. |
| `RTP-H7` | `M4.7` | The build's metadata carries the runtime plan's sha256 and its provenance, the whole lock data of `ROADMAP.md` §10.3 is written, `--locked` is honoured, and S0's `no-lock-data` is discharged. |
| `RTP-H8` | `M4.4` | A system's devices are put in correspondence with a target's — which target device each system device is, by a rule the lowering of `archogen-runtime-plan/2` reads — before `M4.1.7` designs that format. |
| `RTP-H9` | `M4.3` | The hosted simulator runs initialisation to its last step, then releases each timer-released task from its own exact clock at its nominal releases from time zero, takes its timer only when a release is due, takes the external sources in the plan's rank order, raises no interrupt the plan does not enable, and models each device the plan names, a timer route's alarm included. |
| `RTP-H10` | `M4.10` | Once a catalog record can be a provider (`M3.7`), the runtime plan's lowering judges each context line against the record's supported contexts by inclusion and refuses `unsupported-profile` a context the record does not include. |

`M4.12` is filed with this record, for the application inputs.

## Why

- §10.1 asks every pass to have "a typed input/output contract and validation rules", and §7.5 asks that the build
  "records the exact plan hash in generated metadata". A plan read from the checked record, with one canonical text
  and its sha256 handed to the build's metadata (`RTP-H7`), is both. Its provenance beside it keeps that hash a
  property of the plan alone.
- Lowering the checked record, never the search's memory, keeps `M3.5`'s guarantee: the bindings built are the
  bindings checked.
- Owed facts instead of defaults are the project's rule for what the engine cannot establish. A default stack or
  execution bound would be a claim no one made.
- Exact durations keep the plan free of any one timer, and end S0's whole-millisecond restriction.
- **Why `/1` is hosted only.** Three review rounds found 18, 17 and 18 defects, each time in the parts that decided
  another leaf's question: how a system's device corresponds to a target's (`M4.4`), how releases and an adapter share
  an alarm (the joint rules'), and sizes only records state. A plan for the playground needs none of them, and every
  rule in it is determined today, as its two worked texts show. Those questions get designs of their own (`.6`, `.7`).
- The plan writes canonical S-expression text because it quotes eADL values and `/2`'s lines word for word: one form
  for both keeps them as written.

**Alternatives weighed.**
- **Lowering from `Resolution` in memory** is quicker, and would build what the search found rather than what the
  checker accepted.
- **A target in `/1`** (rounds 1 to 3): it needs the device correspondence and the release alarm, which `/1` would
  have decided for other leaves; round 3 showed that a target lowered by its rules needed choices in six places.
- **The release timer as the device bound for `tick-rate`** (rounds 2 and 3): `tick-rate` belongs to a counter, which
  may raise no interrupt, and the rule ignored the channels an adapter takes. That question is `.6`'s.
- **`some` for a source faster than its task** (rounds 2 and 3): the analysis would assume a release at most once per
  task separation, and nothing would make the runtime keep it.
- **Making a cycle a conflict of the assignment** (round 1's answer) contradicts `M3.3.7`'s reviewed reading: run-time
  calls are no initialisation order. Its edges named by `uses` do not depend on the assignment, so no other assignment
  helps.
- **Ordering sources by the most urgent task they release** has no key for a source that releases none, and
  disregards the source's own loss limit.
- **Audsley's assignment in the lowering** needs the sources' costs and a schedulability test, the catalog's and the
  scheduling checker's, and the composition's test is not one it is optimal for (§2.5).
- **A source for each adapter cost** duplicates its provider's route, and the analysis refuses two sources of one name.

## How to apply

- `M4.1`'s children are `.1` this record, reviewed, then `.2` to `.7` as §4 names them.
- A consumer reads the plan with a reader held to the goldens, never by matching text.
- Related: [[decision_s0-retirement]], [[decision_eadl-engine-boundary]], [[decision_priority-comparison-direction]],
  [[decision_zero-dependency-engine-core]], `docs/specs/adapters/decision_engine-adapters.md`,
  `docs/specs/catalog/decision_runtime-composite-inputs.md`.
