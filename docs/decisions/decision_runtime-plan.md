# The runtime plan: what a checked assignment is lowered into

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — the design for leaf `M4.1`, under independent review (`M4.1.1`) before any code relies on it;
  round 3's findings recorded and their answers pending, so no code relies on this text yet
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
- what is initialised before what, and when the first release is;
- where in memory the image, each stack and the runtime's tables go, and what they must avoid;
- what the plan owes, and which leaf discharges it.

A fact no input supplies yet, such as a task's execution bound or a primitive's cost, is never invented. The plan
names it as **owed**, with its owner, and says it is incomplete.

### 1. What it is lowered from, and when it refuses

The lowering is §10.1's step 5, "Lower to a runtime/build plan with initialization dependencies and memory
allocation", in the crate §4.2 names, **`crates/archogen-plan`**. That crate is the generator's, does no I/O, and takes
no external dependency (`decision_zero-dependency-engine-core.md` names "the planner"). Its inputs:
1. the elaborated program the description check accepted, as the typed `Program`, with its digest
   (`eadl_resolve::plan::program_digest`);
2. the assignment record, `archogen-assignment/2`, as the text written;
3. the configuration checker's result record, `archogen-result/1`, for that assignment and that program, as text;
4. optionally, a **target**: the elaborated program of a target description such as `targets/riscv-virt-up.eadl`,
   whose memory and timer the image will run on. Without one, the system is **hosted**: the playground provides its
   memory and its clock (model §8 rule 10, `M4.3`). How a command line names a target is written into `M4.2`.

It lowers **the record the checker accepted**, read back by the generator's own reader of `/2` (`M4.1.3`), and never
the search's memory. So the bindings built are the bindings checked. The plan's routes, multiplexing and initialisation
are its own to derive and the build's to verify (`M4.4`, `M4.10`), as `M3.5`'s design says of them.

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

**Refusals the target decides.** The description check, the search and the checker judge a description without a
target. A refusal that rests on the target is therefore the lowering's, and its message says the target decided it:
- the target given, a task released by the timer, and no device bound for its time (§2.5): `missing-fact`, asking for
  the time service;
- the target given, and stacks, tables and guards whose sizes are all known not fitting its free memory (§2.8):
  `infeasible-configuration`, naming the allocation that found no room.

Every refusal the description alone decides is the joint rules', so `archogen resolve` and the lowering never
disagree about it (`M4.1.2`, §2.5, §2.8).

**Provenance.** The lowering cannot see whether the result was produced by an installed, verified checker, nor its exit
code: those are the command line's to check (`M3.5.1`'s contract). So beside the plan it returns its **provenance**:
- the engine's version and the generator executable's sha256 (`TI-H14`);
- the result record's sha256, and the checker program's sha256 that record names;
- how the acceptance was obtained: *checked by the front*, the command line having run and verified the checker; or
  *taken as text*, which `TI-H14` makes experimental.

The provenance is the build's metadata (`M4.7`), never part of the plan's text (§3). The emitter accepts a runtime plan
only by lowering it again from the same inputs and comparing bytes (`RTP-H3`). A plan *checked by the front* is then
built only as lowered from a verified checker's acceptance. A plan *taken as text* is built only marked experimental.

### 2. What the plan holds, and from whom

A runtime plan is an engine structure (`decision_eadl-engine-boundary.md`). Each section names its input.

1. **Identity**, the plan's semantic inputs: the format, the profile, the system, the program's digest, the
   assignment's sha256, the adapter catalog's version, and the target's digest where one is given. Nothing that changes
   when only the engine or the checker is rebuilt is part of it (§10.3, "identical semantic plans").

2. **Tasks**, from the description, in runtime order. A task's line holds:
   - its **runtime index**, `|hp(i)|`, the number of tasks of smaller rank, and its **rank**. The emitter still
     builds the scheduler with `rt_core::Scheduler::from_eadl_ranks`, "the only supported way to lower a description
     onto a scheduler", and a test holds the plan's index equal to the scheduler's slot (`RTP-H3`);
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
       through `uses` and `needs`, at any depth (model §8 rule 8), as `(source NAME every)` or `(source NAME some)`.
       The analysis asks "whether every arrival releases the task" (`crates/rt-analysis/src/runtime.rs`). The plan
       writes `every` when the task's minimum separation equals the source's stated separation, and `some` otherwise,
       the task's own separation then bounding its releases;
     - `unknown` where no route reaches it, where several sources do, or where the source states no separation. The
       language states no trigger, and the analysis refuses a release it cannot bound.

   **Durations** are exact. Each is one reduced rational number of seconds, `N s` or `N/D s`, with `D > 1` and `N` and
   `D` coprime, so that `1.5 ms`, `1.50 ms` and `1500 us` are one text. Converting a duration to timer ticks is the
   generator's, where the timer is known (`RTP-H1`): nominal releases stay exact, each compare is set to the ceiling of
   its release, and only a period under one tick or past the timer's horizon is refused.

   **Owed for each task** to the application, the rest of §7.3's task record and what the analysis reads of the
   task's own code (`docs/specs/catalog/decision_runtime-composite-inputs.md` §1 and §2), which the boundary forbids a
   description to state (`docs/semantics/boundary/reject/`'s `application-entry-point.eadl`, `execution-bound.eadl`,
   `stack-allocation.eadl`):
   - its entry point;
   - its execution bound, `C_i^app`;
   - each OS call it may make, with the most one job makes, `n_{i,p}`;
   - its masked runs, `CS_{i,r,e}^app` and `n_{r,e,p}`;
   - its task facts, among them `leaves-interrupt-hardware-alone`;
   - its code's stack bound, and that bound's evidence category, kept apart from the bound as
     `stack-allocation.eadl` asks and as `M4.8`'s F21 reads it.

   The application inputs' format is `RTP-H5`'s. `archogen-runtime-plan/1` has no line for a supplied application
   fact, so `/1` is never complete. `RTP-H5`'s format brings `/2`, which carries them.

3. **Bindings**, the assignment's `bind` lines, copied as written, so a consumer reads one artifact.

4. **Engine adapter instances**, each named as the plan writes it, `(adapter KIND OVER)` or
   `(adapter KIND OVER REQUESTER)`. Each line carries:
   - the value of each fact its kind offers `obliged`, which is the requirement's value at that exclusive instance,
     model §8 rule 15's ordinary value, such as `deadline-over-delay`'s `delivery-bound`;
   - its kind's obligations and costs, in the kind's written order, as the engine API's `Plan.obligations` lists them
     (the adapters record's §9).

   The values a cost's terms need, such as the wake's update period and each interrupt's service cost, are owed by the
   adapter's realization (`RTP-H2`).

5. **Interrupts.**
   - **The release timer** releases every task whose trigger is `timer`. It is the device the assignment binds for
     `tick-rate` to the tasks with that trigger, or to a requester they reach through `uses` and `needs`. A task's own
     bindings count. An adapter's instance counts as the provider it programs, as model §8 rule 14 counts it:
     - **one device:** that device is the release timer;
     - **several devices:** the plan writes the release timer as `unknown`. Model §8 rule 8 leaves a second timer to
       the analysis to refuse, and the emitter cannot program two (`RTP-H1`);
     - **none, hosted:** the simulator's clock releases these tasks (`M4.3`), and the plan writes the timer as
       `simulator`;
     - **none, with a target:** refused by the lowering, `missing-fact`, asking for the time service (§1).
   - **Timer routes**: each other device the joint rules report a route with a timer interrupt for (model §8 rule 8),
     so a second timer is written, never dropped.
   - **External sources**: each provider the joint rules report a route with an external interrupt for. Each has its
     minimum separation, or `unknown` where the description states none, the requesters relying on it, and the level
     it is delivered at.
   - **An adapter's cost interrupt** is no interrupt of its own. `expiry` and `wake` are "one interrupt at its
     provider" (the adapters record's §2). The `/1` kinds program `relative-delay`, a consumed fact, so that provider's
     route is a timer route (model §8 rules 8 and 15). Each cost is written on that route's line, the release timer's
     or a timer route's, with the instance that takes it. A shared instance's cost counts once.

     A cost at the release timer's device puts timer traps there that are not releases. The variant's condition 6
     assumes the timer is "taken only when a release is due", so the analysis refuses such a system under this profile
     (`unsupported-profile`). The plan carries the cost, and the analysis says so. `M4.1.2` amends the adapters record's
     §9 to read this way, with the engine API's documentation and the book: "charged as a source" is replaced by "a
     cost on its provider's route".
   - **The order among external sources**, `ahead(x)`, is the plan's to set (the composite-inputs record §1). The
     timer's place against them is the hardware's, by the catalog's `external-before-timer`, and the plan does not
     order the timer. Each external source gets a distinct **rank**, rank `1` taken first, so the order is strict by
     construction:
     - ascending by stated minimum separation, a source with `unknown` last;
     - ties broken on the provider's name.

     This is rate-monotonic over the sources' separations. It equals deadline-monotonic over each source's no-loss
     limit when each source is acknowledged at entry, where that limit is its separation (the composite-inputs record
     §3). Interrupts do not nest here (the variant's condition 5), so service is non-preemptive among sources. For
     fixed-priority non-preemptive scheduling, "unlike in the pre-emptive case, DMPO is not optimal for
     constrained-deadline task sets", and "the Optimal Priority Assignment algorithm given by Audsley (1991) is
     applicable" ([Davis](../book/src/ledger.md#davis-speedup-factors), citing George et al., 1996). Audsley's
     algorithm needs each source's service cost and a schedulability test, which are the catalog's and the scheduling
     checker's, not the generator's. So this order is a **heuristic**: an analysis refusal under it says that another
     order may admit the set (`RTP-H4`).

     Soundness does not rest on the order. `crates/rt-analysis/src/compose.rs` refuses an order that is not strict, a
     source with no rank and a source not deliverable, and otherwise takes the order given. It does rest on the emitted
     controller configuration realising the ranks. The emitter maps them to distinct priorities above the threshold it
     sets, and refuses `unsupported-profile`, naming the controller's level count, when they do not fit (`RTP-H1`).
     `M4.10` checks what the image writes against the plan.
   - **The enabled set**: the timer, if any task's trigger is `timer` or a timer route exists; each external source;
     and, where a target is given, the platform's non-maskable interrupts and firmware traps (`runtime.rs`: "Every
     enabled interrupt by name, `timer` included, non-maskable interrupts and firmware traps included"). The last are
     owed to the catalog (`RTP-H6`) until a record states them. Each external source is declared **deliverable**, its priority above the
     controller's threshold, as the composite-inputs record asks of "every declared source"; no threshold governs the
     timer, nor the non-maskable interrupts and traps.
   - **The interrupt model**, §7.3's per-source "masking constraints, and whether interrupt nesting is possible":
     interrupts do not nest, and every masked section masks every interrupt. These are the variant's conditions, which
     the plan states. Each external source's service cost, `service.<source>`, is owed to the catalog.

   `M4.1.2` amends model §8 rule 8, whose plan "enables each such interrupt and no other", to this enabled set: the
   timer, every route's interrupt, timer routes included, and the platform's non-maskable interrupts and traps.

6. **Execution contexts** (`M3.3.8`'s obligation). One line for each requester the system serves but its own level:
   tasks, services, policies and instances. Each node of the initialization graph that is no requester, a bound block
   or platform, has one too, with `initialisation` alone. The contexts are:
   - `job`, for a task, and for a service or instance a task reaches through `uses` and `needs`;
   - `(interrupt NAME)` for each route naming the requester or a service it reaches, the release timer's and the timer
     routes' included, so a service receiving `expiry` runs in that interrupt;
   - `initialisation`, for every node of the initialization graph: a bound provider, an instance, a service or a
     policy.

   A record states the contexts it supports. Each line is judged against it once the record exists (`RTP-H6`); until
   then it is owed. **No step of initialisation may wait on an interrupt**, an alarm's or a source's, since
   initialisation runs masked (`M3.3.8`). For a record that is an obligation on its initialisation entry, owed to the
   catalog (`RTP-H6`). For generated code, such as an epoch extender arming its wake, it is the emitter's (`RTP-H2`).
   `M4.10` checks the image.

7. **Initialisation.**
   - The graph is the joint rules' initialization graph over the system and the assignment's bindings (model §8 rule
     12). `M4.1.3` adds its production computation: only the tests call `Joint::initialization` today.
   - Its steps are its strongly connected components, in a topological order taken by repeatedly choosing, among the
     components whose predecessors have all been taken, the one whose least member's canonical name is least.
   - A step names its members in canonical-name order. Nodes are named as the bindings name them: a block, a platform,
     an instance's `(adapter …)` text, a service or a policy.

   **A cycle stays as rule 12 has it.** Two services that call each other at run time can each initialise without the
   other (`M3.3.7`, held by `two_services_using_each_other_are_one_component_not_a_refusal`). So a component of several
   members is one step: the members are initialised in canonical-name order, all before any successor. This carries
   an obligation, owed to the catalog for records and to the emitter for generated code (`RTP-H2`, `RTP-H6`), and
   checked by `M4.10`: no member's initialisation calls another member's code. Only where a record states such a call
   does the lowering refuse, `unsupported-profile`, naming the component.

   **The release epoch** is the counter value `E` that every timer-released task's first nominal release is at; each
   later release follows from it exactly. Initialisation fixes `E` before it writes the first compare: the counter read
   then, plus a catalog-stated bound on the path from that read to the first enabling of interrupts, rounded up to a
   tick. So `E` is at or after the first enabling, which is the plan's half of `releases-after-initialisation` (the
   composite-inputs record §2). The bound is owed to the catalog (`RTP-H6`), and the code that computes `E` is the
   emitter's (`RTP-H2`). `M4.10` checks the image's half, the first enabling coming after the last step. A task with a
   minimum separation is first released by an arrival, after the first enabling. A hosted system's epoch is the
   simulator's time zero (`M4.3`).

8. **Memory.** Where the system has placement memory (model §8 rule 10, "M3 chooses the region, `M4.1` the offsets"),
   or else where the target has some, the plan writes:
   - the placement region, from the assignment's `(placement …)` or the target's, with its base and size;
   - each **hole** in it the joint rules report (`Joint::map`, which the search already runs per system, over its
     hardware or the target's), with its base and size, or `unknown`;
   - the **free ranges** the holes leave, ascending by base;
   - the **image's start**, the lowest free range's base. The image's linked sections run from there and must end
     within that range, which `M4.4` checks of the linked image (F20). The image's size is no fact the plan owes;
   - the **allocations**, each a unit of a guard and the stack above it, or a table, with its base, its size, its
     guard's size and its alignment:
     - the boot stack, which initialisation and the idle loop run on (`__stack_top`, the image's set-up,
       `docs/specs/catalog/decision_catalog-records-port.md`);
     - each task's stack, in runtime order;
     - the interrupt stack, which every trap runs on, faults included;
     - the runtime's tables.

   Addresses are absolute. Units are allocated in that order, from the top of the highest free range downward. A unit
   that does not fit what remains of a range goes to the top of the next lower range, so a guard always lies directly
   below its stack. A unit whose size, guard or alignment is owed has its base owed, and so does every unit placed
   after it in the same range. Where every size is known and a unit fits no range, the lowering refuses (§1).

   **Sizes** are the engine's derivation (§7.3: "the engine derives concrete allocations"; `stack-allocation.eadl`:
   "the derived allocation, checked against the linked image"):
   - a task's stack is its code's stack bound, plus the runtime's and the trap path's frames;
   - the boot stack's size is the catalog's;
   - the interrupt stack is the largest stack use of any service the enabled set runs, plus the trap frame;
   - the guard's size and every alignment are the catalog's (§7.6).

   The frames, the guard, the boot stack, the tables and the alignments are owed to the catalog (`RTP-H6`); the stack
   bounds to the application (`RTP-H5`); a generated service's stack use to the emitter (`RTP-H2`). `M4.4` checks the
   linked image against these allocations, which is what M4's "Linked memory accounting matches the plan" compares.

   `M4.1.2` amends rule 10, so the search, the checker and the lowering agree over the description:
   - **placement memory with no size** is `missing-fact`, naming it, since nothing can be laid out in it;
   - **a hole of no size** whose base lies inside placement memory (`M3.5.5.7.3`'s obligation) is taken to reach the
     region's end. That is the conservative extent, the largest the region allows. If it leaves no free byte, the
     system is `missing-fact`, naming the hole and asking for its size. `M4.4` names the hole, and asks for its size,
     when an image does not fit a range it truncates.

   A hosted system has no memory section: the playground provides memory.

9. **Regions**: each region of the hardware, as `Joint::map` reads it, the system's or the target's, with its base,
   size, provider, and the requesters bound to that provider, or none. A device's registers on a platform nothing binds,
   such as `periodic-three`'s `device.timer` on `soc.playground`, are still the build's to map (§7.6, §10.3's
   "resource ownership").

10. **Obligations**, each with the leaves that discharge it:
    - the adapters', by the adapter's realization in the generated code (`RTP-H2`), reported by `M4.8`;
    - the joint rules' multiplex obligations, by the service's realization (`RTP-H2`);
    - their route obligations: the plan enables the interrupt (item 5), while its path, line and latency are the
      catalog's (`RTP-H6`), and the image's configuration is checked by `M4.10`;
    - their placement obligations, by the allocations (item 8), verified by `M4.4`;
    - initialisation's, by item 6 and item 7, checked by `M4.10`.

11. **Owed facts**, in one list, each a subject, a fact and its owner. Exactly these are written:

    | Subject | Facts | Owner | Where |
    | --- | --- | --- | --- |
    | each task | `entry-point`, `execution-bound`, `masked-runs`, `os-calls`, `task-facts` | `application` | always |
    | each task | `stack-bound`, `stack-category` | `application` | where there is a memory section |
    | each task with no `deadline-from` | `deadline-from` | `description` | always |
    | each service and policy that is a node | `contexts`, `no-wait` | `catalog` | always |
    | each bound block or platform that is a node | `no-wait` | `catalog` | always |
    | each instance | `contexts`, `no-wait`, and `cost-terms` where its kind has costs | `realization` | always |
    | each external source | `service-cost` | `catalog` | always |
    | the system | `nmi-and-traps`, `epoch-bound` | `catalog` | where a target is given |
    | the system | `frame-bytes`, `guard-bytes`, `alignment`, `boot-stack-bytes`, `table-bytes` | `catalog` | where there is a memory section |

    The plan's last line says whether it is complete, and `/1`'s never is (§2.2).

§7.5's "selected provider IDs" are the bindings' providers. Selected catalog records join them when a record can be a
provider (`M3.7`), and the plan then carries each record's identity.

### 3. The format, `archogen-runtime-plan/1`

Canonical text, fixed here so that both writers and every reader derive it from this record alone:
- each line is one form and ends with a single newline, the last line included;
- tokens are separated by a single space, with no space after `(` or before `)`;
- nothing else is written: no comment, and no blank line.

The forms come in the order below. `NAME` is a declaration's name; `REQ` is a requester as `/2` writes one; `DUR` is
§2.2's duration; `HEX` is 64 lowercase hexadecimal digits; `INT` is a decimal integer with no sign and no leading zero.

```text
(format "archogen-runtime-plan/1")
(profile PROFILE)
(system NAME)
(program "HEX")
(assignment "HEX")
(adapters VERSION)
(target "HEX")
(task INDEX (task SYSTEM NAME) (rank INT) RELEASE (deadline DUR) (deadline-from EVENT|unstated) (jitter DUR) (overrun POLICY) (released-by TRIGGER) (uses NAME …))
(bind REQ FACT PROVIDER)
(instance INSTANCE (obliged FACT DUR) … (obligations NAME …) (costs NAME …))
(timer NAME|unknown|simulator (requesters REQ …) (costs (INSTANCE NAME) …))
(timer-route NAME (requesters REQ …) (costs (INSTANCE NAME) …))
(source INT NAME (separation DUR|unknown) (level LEVEL|none) (requesters REQ …))
(enabled ITEM …)
(interrupt-model (nesting no) (masking all))
(context SUBJECT CONTEXT …)
(step INT NODE …)
(epoch (bound owed|DUR))
(placement NAME REGION (base INT) (bytes INT))
(hole NAME REGION (base INT) (bytes INT|unknown))
(free (base INT) (bytes INT))
(image (base INT))
(allocate WHAT (base INT|owed) (bytes INT|owed) (guard INT|owed) (align INT|owed))
(region NAME REGION (base INT) (bytes INT|unknown) (requesters REQ …))
(obligation KIND SUBJECT (discharged-by LEAF …))
(owed SUBJECT FACT (by OWNER))
(complete yes|no)
```

**Values and absent forms:**
- `RELEASE` is `(period DUR)` or `(min-separation DUR)`.
- `EVENT` and `POLICY` are written as the description writes them; `(deadline-from unstated)` where none is.
- `TRIGGER` is `timer`, `(source NAME every)`, `(source NAME some)` or `unknown`.
- A list with no members is written with its head alone: `(uses)`, `(requesters)`, `(costs)`.
- `(level none)` where the route states no level.
- `INSTANCE` is the instance's `(adapter …)` text. A cost is `(INSTANCE NAME)`, `NAME` the cost's name in the kind.
- `ITEM` is `timer`, an external source's provider name, or `nmi-and-traps` where a target is given.
- `SUBJECT` is a `REQ`, a provider's name, an instance's text, or the system's name.
- `CONTEXT` is `job`, `(interrupt NAME)` or `initialisation`. For the timer, `NAME` is `timer`; otherwise it is the
  source's or the timer route's provider name.
- `NODE` is a member's name, an instance as its `(adapter …)` text.
- `WHAT` is `boot-stack`, `(stack (task SYSTEM NAME))`, `interrupt-stack` or `tables`.
- `KIND` is an adapter kind's obligation name, or `multiplex`, `route`, `placement` or `initialisation`.
- `LEAF` is a task-tree identifier. `OWNER` is `application`, `catalog`, `description` or `realization`.
- `FACT` in `owed` is one of: `entry-point`, `execution-bound`, `os-calls`, `masked-runs`, `task-facts`,
  `stack-bound`, `stack-category`, `deadline-from`, `service-cost`, `cost-terms`, `contexts`, `nmi-and-traps`,
  `epoch-bound`, `frame-bytes`, `guard-bytes`, `alignment`, `boot-stack-bytes`, `table-bytes`, `no-wait`.

**Forms written only where §2 calls for them:**
- `(target …)`, where a target is given;
- `(timer …)`, where any task's trigger is `timer`: the device's name, `unknown` where several devices are, or
  `simulator` where the system is hosted. Its `requesters` are the tasks it releases and its route's requesters;
- `(enabled …)`, where any interrupt is enabled;
- `(epoch …)`, where a target is given and a timer releases a task;
- the memory forms, `(placement …)` through `(allocate …)`, where there is placement memory;
- `(region …)`, where the hardware has regions.

`(interrupt-model …)` is always written.

**Orders:**
- `task` lines by index, starting at `0`;
- `bind` lines in `/2`'s order;
- `instance` lines by their text; within a line, `obliged` items in the kind's offer order, then the kind's
  obligations, then its costs, each in the kind's written order;
- `timer-route` lines by provider name; `source` lines by rank, starting at `1`;
- `costs` and `requesters` items by their text; `uses` items by name;
- `enabled` items: `timer`, then sources by rank, then `nmi-and-traps`;
- `context` lines by subject's text; within a line, `job`, then each `(interrupt …)` by name, then `initialisation`;
- `step` lines by index, starting at `1`;
- `hole`, `free` and `region` lines by base, then by provider, then by region;
- `allocate` lines in allocation order (§2.8);
- `obligation` and `owed` lines by their text.

**The heartbeat, worked.** `examples/s0-heartbeat/system.eadl`, hosted, its assignment the one line
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
(timer simulator (requesters (task heartbeat beat) (task heartbeat chime)) (costs))
(enabled timer)
(interrupt-model (nesting no) (masking all))
(context (task heartbeat beat) job)
(context (task heartbeat chime) job)
(context console.uart initialisation)
(context console.write job initialisation)
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

Its platform, `host.playground`, is bound by nothing and is no node of the initialization graph, so it has no line. A
hosted system has no target, no memory section and no epoch line.

The plan's identity is the sha256 of its text. It has one writer and one reader in `crates/archogen-plan`, and a plan
the reader accepts writes back the same bytes, as `archogen-result/1`'s rule asks. The scheduling checker reads it with
a reader of its own, held to the same golden, as `/2` has two readers: the generator's reader is generator code, which
the scheduling checker must not share (`RTP-H4`). The golden is lowered from frozen fixture inputs, its result record a
fixed text, and it is never rewritten. **The rules of §2 are part of the format**: a change to any of them — an order,
the allocation, the topological choice — is a new format number. `docs/book/src/versions.md` registers the format
(`VERSION-REGISTER`). Lowering the same inputs twice writes the same bytes (M4's "Repeated generation produces
identical canonical plans").

### 4. What `M4.1` builds, and what it hands on

**Built by `M4.1`'s children:**
- `.2`: model §8 rules 8 and 10 amended as §2.5 and §2.8 say, in the model, the generator's joint rules and the
  checker's, the checker's by a fenced context; the adapters record's §9 and its faces' documentation; the book; a
  migration note for any verdict that moves;
- `.3`: the generator's reader of `/2`, its round trip held against its writer, and the typed record it returns; the
  initialization graph computed for a record's bindings;
- `.4`: the crate, the typed plan, its writer and reader and golden, the lowering, its refusals and its provenance; the
  heartbeat's plan, the first to lower whole but for what it owes;
- `.5`: the register entry; the crate in the generator root's `(role-packages …)` in `trust/roots.eadl`, each item it
  shares with another role classified by its commit (F30); the book's chapter; and every other place that names `M4.1`
  as owner, moved as below.

**What else names `M4.1`**, discharged or moved by `.5`:
- `crates/eadl-model/src/profile.rs`'s rule "one execution context runs at a time", owner `M4.1`. The plan states it:
  no nesting, one core, and each requester's contexts. The runtime realizes it, so its owner moves to `M4.2`.
- S0's `whole-millisecond-periods` moves to `M4.2`, which the plan's exact durations let end. That covers its row in
  `decision_s0-retirement.md`, its marker and hint in `crates/archogen-s0/src/interpret.rs` and that file's test,
  `docs/book/src/s0.md`, and `examples/s0-heartbeat/README.md`.
- `no-lock-data` and `--locked` move to `M4.7`, which writes the whole lock data. That covers the row, the comments,
  hint and test in `crates/archogen-cli/src/build_cmd.rs` and `tests/s0_build.rs`, `resolve_cmd.rs`'s refusal of
  `--locked`, and `docs/book/src/cli.md`.
- The remarks naming `M4.1` in `crates/eadl-resolve/src/joint.rs` and its tests stand: they state this leaf's role
  truly.

**Written into other leaves with this record**, quoted word for word by each receiving leaf (`HANDOFF-LEDGER`):

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RTP-H1` | `M4.2` | The emitter moves `archogen build` onto the runtime plan, its faces in step on the command line, the engine API and the [MCP](../book/src/ledger.md#mcp-specification) tool, a target named on the command line, durations converted to ticks with nominal releases kept exact and each compare at the ceiling of its release, source ranks mapped to distinct controller priorities above its threshold or `unsupported-profile` naming the level count, a release timer written `unknown` refused, the scheduler built with `from_eadl_ranks` and its slot held to the plan's index, and S0's `whole-millisecond-periods` and the profile's "one execution context runs at a time" discharged. |
| `RTP-H2` | `M4.2` | The generated code realizes the adapters' and the multiplexed services' obligations, computes the release epoch, initialises no component member by calling another's code, never waits on an interrupt in initialisation, and states each cost's terms, each generated service's stack use and each value the runtime plan owes to the realization. |
| `RTP-H3` | `M4.2` | The emitter accepts a runtime plan only by lowering it again from the same inputs and comparing bytes, and marks a plan whose acceptance was taken as text experimental. |
| `RTP-H4` | `M2.21.2` | The scheduling checker reads the runtime plan with a reader of its own held to the plan's golden, maps it onto the analysis's inputs, and words a refusal under the plan's source order as one another order may admit. |
| `RTP-H5` | `M4.12` | The application inputs have a format — each task's entry point, execution bound, OS calls with counts, masked runs, task facts, stack bound and its evidence category — that `archogen-runtime-plan/2` carries. |
| `RTP-H6` | `M4.10` | The catalog records state what the runtime plan owes them — supported contexts, an initialisation entry that never waits on an interrupt, frame, guard, boot-stack and table sizes, alignments, the platform's non-maskable interrupts and traps, each source's service cost and route latency, and the bound from the epoch's read to the first enabling — amending the catalog record's §12 and §14.4. |
| `RTP-H7` | `M4.7` | The build's metadata carries the runtime plan's provenance, the whole lock data of `ROADMAP.md` §10.3 is written, `--locked` is honoured, and S0's `no-lock-data` is discharged. |

`M4.12` is filed with this record, for the application inputs.

## Why

- §10.1 asks every pass to have "a typed input/output contract and validation rules", and §7.5 asks that the build
  "records the exact plan hash in generated metadata". A plan read from the checked record, with one canonical text
  and its sha256, is both. Its provenance beside it keeps that hash a property of the plan alone.
- Lowering the checked record, never the search's memory, keeps `M3.5`'s guarantee: the bindings built are the
  bindings checked.
- Owed facts instead of defaults are the project's rule for what the engine cannot establish. A default stack or
  execution bound would be a claim no one made.
- Exact durations keep the plan free of any one timer, and end S0's whole-millisecond restriction. The one lossy step,
  to ticks, stays where the tick is known.
- A target as an input lets a system that describes no memory of its own, as every example is, be laid out where it will
  run, and keeps that target in the plan's identity.
- The plan writes canonical S-expression text because it quotes eADL values and `/2`'s lines word for word: one form
  for both keeps them as written.

**Alternatives weighed.**
- **Lowering from `Resolution` in memory** is quicker, and would build what the search found rather than what the
  checker accepted.
- **Making a cycle a conflict of the assignment** (round 1's answer) contradicts `M3.3.7`'s reviewed reading: run-time
  calls are no initialisation order. Its edges named by `uses` do not depend on the assignment, so no other assignment
  helps.
- **Ordering sources by the most urgent task they release** has no key for a source that releases none, and
  disregards the source's own loss limit.
- **Audsley's optimal assignment in the lowering** needs the sources' costs and a schedulability test, the catalog's
  and the scheduling checker's; the generator running that test would share the checker's role.
- **A source for each adapter cost** duplicates its provider's route, and the analysis refuses two sources of one name.
- **Treating an unsized hole as zero bytes** assumes an extent, which the obligation forbids.

## How to apply

- `M4.1`'s children are `.1` this record, reviewed, then `.2` to `.5` as §4 names them.
- A consumer reads the plan with a reader held to the golden, never by matching text.
- Related: [[decision_s0-retirement]], [[decision_eadl-engine-boundary]], [[decision_priority-comparison-direction]],
  [[decision_zero-dependency-engine-core]], `docs/specs/adapters/decision_engine-adapters.md`,
  `docs/specs/catalog/decision_runtime-composite-inputs.md`.
