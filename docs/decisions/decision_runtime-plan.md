# The runtime plan: what a checked assignment is lowered into

- **Type:** `decision`
- **Date:** `2026-10-27`
- **Status:** `active` — the design for leaf `M4.1`, under independent review (`M4.1.1`) before any code relies on it;
  format `/1` narrowed to the hosted playground on `2026-10-28` (round 3), rounds 4 to 15 answered; from round 7 it is
  reviewed with its executable form and its probe corpus (`decision_executable-design-reviews.md`)
- **Owner / source:** leaf `M4.1.1` (`docs/tasks/M4.md`); `ROADMAP.md` §4.2's `archogen-plan`, §7.3, §7.5, §7.6, §10.1
  step 5 and §10.3; model §8 rules 4, 5, 7, 8, 10, 12 and 15; the obligations `M3.3.8`, `M3.5`, `M3.5.5.7.3` and
  `M3.2.1` wrote into `M4.1`. Its review history is `docs/reviews/decision_runtime-plan-reviews.md`.

## The fact / decision

In plain words: once the search has found an assignment and the configuration checker has accepted it, the engine
lowers it into one more artifact for that system, the **runtime plan**. The plan says, in the form the code generator
and the build take, what the description does not say in that form:
- each task's parameters, and what releases it;
- the timer and the interrupt routes, and the order the controller takes them in;
- in which context each part of the system runs;
- what is initialised before what, and why;
- what the plan owes, and which leaf discharges it.

A fact the plan needs and no input supplies yet, such as a task's execution bound, is never invented. The plan names
it as **owed**, with its owner, and says it is incomplete.

The first format, `archogen-runtime-plan/1`, is for a system run on the **hosted playground**, whose clock, memory and
devices the simulator provides (`M4.3`). Everything in it is determined by the description, the checked assignment and
the joint rules as `.2` amends them, and three complete worked texts pin it (§3). Running on a target needs three designs
that are not this record's: which of a target's devices a system's device is (`M4.4.1`), how timer releases and an
adapter share one alarm (`M4.1.6`), and the sizes only catalog records state. They come with a second format, `/2`
(§4, `.7`).

A `/1` plan describes a system; it does not make its scheduling analysis conclusive. The analysis reads the platform's
own figures, the cost of a context switch, of the timer's service and of delivering an interrupt among them, and on the
playground those are the simulator's model's (`RTP-H9`), not the plan's.

### 1. What it is lowered from, and when it refuses

The lowering is §10.1's step 5, "Lower to a runtime/build plan with initialization dependencies and memory
allocation", in the crate §4.2 names, **`crates/archogen-plan`**. That crate is the generator's, does no I/O, and takes
no external dependency (`decision_zero-dependency-engine-core.md` names "the planner"). Its inputs:
1. the elaborated program the description check accepted, as the typed `Program`. The lowering reads its
   declarations from it (`eadl_model::check::resolved_declarations`) and computes its digest
   (`eadl_resolve::plan::program_digest`), never taking either from its caller;
2. the assignment record, `archogen-assignment/2`, as the text written;
3. the configuration checker's result record, `archogen-result/1`, for that assignment and that program, as text.

It lowers **the record the checker accepted**, read back by the generator's own reader of `/2` (`M4.1.3`), and never the
search's memory. So the bindings built are the bindings checked. It runs the joint rules as the search runs them: the
system read at the profile's caller level (`System::at`, `machine` for `rt-static-up-v1`), each instance rebuilt from
the adapter catalog the record names, `Joint::check` and `Joint::initialization` over the record's bindings, the beneath
ones included, and `Joint::map` over the system. The plan's routes, multiplexing and initialisation are its own to
derive and the build's to verify (`M4.10`, `RTP-H13`), as `M3.5`'s design says of them.

**Refusals, before anything is lowered**, each with the status vocabulary's code and a repair. They are taken in the
order written, this list and then the next, then §2.4's while lowering, and the first that applies is the one given; a
bullet over tasks is taken task by task in runtime order, each task's clauses in the order the bullet writes them; and a
refusal naming several things orders them as it states, each by its text compared byte by byte (§2), never by its
message's words:
- the assignment record does not read, or names another format: `tool-failure`;
- its program digest is not the program's, or the system it names is not one the program declares: `stale-input`,
  "resolve again";
- its `(adapters …)` names a catalog other than this engine's: `stale-input`, "resolve again". Obligations and costs
  are derived from the catalog, never written (`docs/specs/adapters/decision_engine-adapters.md` §5), so a plan
  resolved with another catalog would lower another's;
- the result record does not read; its producer is not `archogen-plan-check`; or its subject is a build, not a plan:
  `tool-failure`;
- its subject is not this assignment's sha256 and this program's digest, or its profile is not the assignment's:
  `stale-input`, "resolve again". The `Program` carries no profile, so the two records' profiles are compared;
- its status is not `ok`: `tool-failure`, naming both verdicts, the generator having accepted what the checker refused
  (`M3.5.1`'s contract);
- its profile is not `rt-static-up-v1`, the one whose rules `/1` lowers: `unsupported-profile`, naming the profile.
  It comes before the rules run, since they run at its caller level;
- the record does not rebuild over the program and the catalog (`eadl_resolve::plan::rebuild`): `tool-failure`,
  naming the rebuild's verdict, the checker having accepted bindings the generator cannot rebuild;
- the generator's own joint rules, run over the record the checker accepted, find a conflict, an unknown or a map
  finding: `tool-failure`, naming both verdicts, the checker's acceptance and each of the generator's findings — its
  conflicts, then its unknowns, then the map's refusals and the map's unknowns, each group in the order the joint rules
  give it — the two roles disagreeing;
- a task's release or deadline the description check should have refused: none stated, which it refuses already
  (`missing-fact` for no release model, a deadline's cardinality one); a period, minimum separation, deadline or jitter
  that is not a time; a period or minimum separation of zero, which no run can release (`RTP-H9`); or a periodic task
  bound itself for `interrupt-source`, its requirement met by `true` as model §8 rule 2 reads one, which states a second
  release model, the source "a source of interrupts a service can be released by" (the vocabulary): `tool-failure`,
  naming the task and the clause — for the last, its `period` — the check having accepted an ill-formed description.
  `M1.42` and `M1.43` make the language refuse the last three (`RTP-H14`, `RTP-H15`), `invalid-description`;
- a declaration or a task the system serves naming a system by `uses` or `needs`, what a system it names writes at its
  own level — its `platform` clause included — being served too (model §8 rules 1 and 7): a system is no thing to use —
  its own `uses` says what it includes (model §8 rule 1) — and its own level would become a node §2 does not give:
  `tool-failure`, naming each such name, by the namer's text and then the system's, the check having accepted it;
  `M1.46` makes the language refuse it, `invalid-description` (`RTP-H17`).
- a block or a platform the system serves naming, by `uses` or `needs`, a declaration that is neither a block nor a
  platform: this record's reading, which `M1.47` writes into the model (`RTP-H18`), is that hardware names only hardware
  — model §8 rule 1 makes a block's or a platform's `uses` composition and rule 7 reads its `needs` as bringing a block
  into the system's hardware, neither giving an operand of another kind a meaning — and such an operand would be read by
  some rules and not others, serving what it names and dropping an edge across "composition" to a service:
  `tool-failure`, naming each, by the block's or platform's text and then the operand's, the check having accepted it;
  `M1.47` makes the language refuse it, `invalid-description`. This refusal and the one above take only what the system
  serves, as the bullet over tasks takes its tasks: nothing else reaches its plan, and the language's refusal, once it
  lands, takes the description whole.

**Refusals of what `/1` cannot lower**, each `unsupported-profile`, naming what is missing and, where one does, the
leaf that adds it:
- a duration the plan would write past the engine's exact arithmetic (§2.2): a task's period, minimum separation,
  deadline or jitter, a source's separation, or an obliged bound, naming each — the tasks' in runtime order, then the
  sources' by their interrupt's text, then the bounds by their instance's text and then the fact's; `M1.45` has the
  language refuse such a quantity where it is read (`RTP-H16`);
- the system has placement memory (model §8 rule 10): laying an image out in memory is `/2`'s (`M4.1.7`). The repair
  is to describe a hosted platform, or to wait for `/2`;
- the system declares no task: the scheduler a build runs (`RTP-H1`) has no empty form,
  `rt_core::Scheduler::from_eadl_ranks` refusing one (`BootError::NoTasks`), and a run would release nothing. The
  repair is to describe a task;
- a task whose deadline is measured from an event other than its release: the analysis measures every relative
  deadline "from the nominal release" (`docs/specs/analysis/decision_runtime-analysis-variant.md`), and no other event
  has a meaning `/1` can state;
- a task, or the system's own level, that would receive an interrupt's code: a timer route that names it, bound to its
  provider itself by a binding that takes a channel (model §8 rule 2) — an alarm it programs, as §2.6 reads that binding
  — or an adapter instance with a cost running its requester's code (`expiry`) whose requester it is, the one an
  exclusive instance serves or one bound to a shared instance. No context runs a job's code in an interrupt, and how a
  job receives an alarm it programs itself or an expiry, and what the system's own level's requirement for either means
  are `M4.1.6`'s. It names every such receiver — each task in runtime order, then the system's own level — with what it
  would receive: first the alarms it programs, each naming the provider, then its expiries, each naming the instance
  that delivers it, each kind by those texts. So no order among them decides the message, though one receiver receives
  at most one today, the search binding a requester's `counter` facts to one provider whole;
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
below is a node of the joint rules' initialization graph (§2.7): a provider an active binding names (a block, a platform
or an adapter's instance), a block or a platform the system serves that is bound for requirements of its own — a
requester like any other (model §8 rule 1, `M4.1.14`), whose code an interrupt can run (§2.6) — a service, or a policy:
the three kinds of node `crates/eadl-resolve/src/joint.rs` gives, `Provider` (hardware and instances), `Service` and
`Other`, the last a policy's, a description naming a system being refused (§1). Its tasks and its own level, requesters
too, are never nodes; a block or a platform the system serves that is bound for nothing is a node only as a provider an
active binding names, since as a requester it runs no code the plan places; the plan derives the context of each task
and each node (§2.6), the requesters whose code it places, as `M3.3.8` asks. A node's **text** is its name as the plan
writes it, an instance's being its `(adapter …)` text, and texts are compared byte by byte. A requester **reaches** what
it names by `uses` or `needs`, and what it is bound to as its provider, at any depth, as the joint rules take a
requester's names (model §8 rule 7) — but for a block's or a platform's `uses` or `needs`, which names only hardware
(§1) and is never a call — its `uses` composition (rule 1; rule 5's "`uses` alone is composition"), its `needs` a block
it brings into the system's hardware (rule 7): hardware reaches only what its own requirements are bound to.

An **interrupt** is named by its kind and its provider, never by the provider alone, since one provider can raise a
timer route and a source, and a block may be named `timer`:
- `timer`, the release timer;
- `(timer-route NAME)`, a timer route at provider `NAME`;
- `(source NAME)`, an external source at provider `NAME`.

1. **Identity**, the plan's semantic inputs: the format, the profile, the system, the program's digest, the
   assignment's sha256 and the adapter catalog's version. Nothing that changes when only the engine or the checker is
   rebuilt is part of it (§10.3, "identical semantic plans").

2. **Tasks**, from the description, in runtime order. A task's line holds:
   - its **runtime index**, `|hp(i)|`, the number of tasks of smaller rank, and its **rank**. The emitter still
     builds the scheduler with `rt_core::Scheduler::from_eadl_ranks`, "the only supported way to lower a description
     onto a scheduler", and a test holds the plan's index equal to the scheduler's slot (`RTP-H1`);
   - its stable logical identity, `(task SYSTEM TASK)`;
   - its period, or its minimum separation;
   - its relative deadline, measured from its release. A task whose description writes `(deadline-from release)` has
     that written; one that writes none has the reference owed to the description, since §7.3 makes it part of the
     record and the language gives no default. Any other event is refused (§1);
   - its release jitter, written as zero where none is stated: "Absent means zero, which is a claim the analysis
     relies on" (`docs/semantics/kinds/os-rt.eadl`);
   - its overrun policy, `fault` where none is stated, the profile's default (the same kind);
   - what it names by `uses` or `needs` that is a service or a policy, each once: a fact or a block it names is no
     member, its `bind` line saying what it is bound to. §7.3's "resource use" is these and what the bindings give
     them;
   - **what releases it**:
     - `timer` for a periodic task;
     - for a task with a minimum separation, the external source whose route names the task or a requester it
       reaches (model §8 rule 8). The plan writes `(source NAME every)` when the source's stated separation is at
       least the task's. Every arrival then releases the task, and the releases are as far apart as the task states.
       The analysis asks "whether every arrival releases the task" (`crates/rt-analysis/src/runtime.rs`). The trigger
       is an inference from the routes, which the description does not state, so the build reports it (`RTP-H1`);
     - `unknown` otherwise: where no source's route reaches the task, where several sources' routes do, where the
       source states no separation or declares it absent, or where it states one shorter than the task's. A timer
       route releases no task. In the last case the source can arrive
       faster than the task says it is released, and nothing says which arrivals release it, so the trigger is owed to
       the description. The analysis refuses a release it cannot bound, and the build refuses the plan (`RTP-H1`).

   **Durations** are exact. Each is one reduced rational number of seconds, `N s` or `N/D s`, with `D > 1` and `N` and
   `D` coprime, so that `1.5 ms`, `1.50 ms` and `1500 us` are one text. `N` and `D` lie within the engine's exact
   arithmetic, signed 128-bit integers (`eadl_model::rational`): a duration whose seconds lie past it is refused
   `unsupported-profile`, naming it, never read as no duration (§1). A task's period, minimum separation, deadline and
   jitter are each a time, a quantity of another dimension refused (§1). The simulator's clock is exact, so no duration
   is converted (`RTP-H9`). Converting to a target timer's ticks is `/2`'s.

   **Owed for each task** to the application, the rest of §7.3's task record and what the analysis reads of the
   task's own code (`docs/specs/catalog/decision_runtime-composite-inputs.md` §1 and §2), which the boundary forbids a
   description to state (`docs/semantics/boundary/reject/`'s `application-entry-point.eadl`, `execution-bound.eadl` and
   `stack-allocation.eadl`):
   - its entry point;
   - its execution bound, `C_i^app`, and that bound's evidence category, as `execution-bound.eadl` asks;
   - each OS call it may make, with the most one job makes, `n_{i,p}`;
   - its masked runs, `CS_{i,r,e}^app` and `n_{r,e,p}`, and their bounds' evidence category, as the composition reads
     one for each run's bound (`compose.rs`);
   - its task facts;
   - the application's two declarations for it that are no task fact (`decision_catalog-records-variant-inputs.md`'s
     task-facts row, `M2.27`): that it masks only through the runtime API, and that its own figures hold under any
     preemption and from any entry state, condition 8's half;
   - its code's stack bound, and that bound's evidence category, kept apart from the bound as `stack-allocation.eadl`
     asks. The allocation itself is `/2`'s.

   `leaves-interrupt-hardware-alone` is owed once, for the system: it covers the application's tasks and its
   initialisation (the composite-inputs record §2); any further fact of the application's the composition is found
   to need is owed so too (`RTP-H12`). The application inputs' format is `RTP-H5`'s.
   `archogen-runtime-plan/1` has no line for a supplied application fact, so `/1` is never complete. Until those
   inputs exist, a `/1` build runs each job as an engine-owned empty body that the release trace alone observes, as
   S0's executable does, and says so (`RTP-H1`).

3. **Bindings**, the assignment's `bind` lines, copied as written, so a consumer reads one artifact.

4. **Engine adapter instances**, each named as the plan writes it, `(adapter KIND OVER)` or
   `(adapter KIND OVER REQUESTER)`, a task requester written `(task SYSTEM TASK)` (`eadl_resolve::plan::instance_text`).
   Each line carries:
   - each fact its kind offers `obliged`: its value, the requirement's value at that exclusive instance, the
     "ordinary value" of the adapters record's §1, item 5, such as `deadline-over-delay`'s `delivery-bound`; or, where
     the requirement writes no bound, the fact alone. "A requirement writing no bound on it leaves the fact offered
     bare" (that item). Either way the fact is owed by the realization: that its code keeps the bound, with that
     evidence's category, as `ROADMAP.md` §7.3 asks of "every numerical bound". A value that is not a duration is
     refused `tool-failure`: `adapters/1`'s one obliged fact, `delivery-bound`, is a time;
   - its kind's obligations, then its costs, each in the kind's written order, as the engine API's `Plan.obligations`
     lists them (the adapters record's §9).

   The values a cost's terms need, such as the wake's update period, are owed by the adapter's realization (`RTP-H2`).

5. **Interrupts.**
   - **The release timer**, `timer`, releases every task whose trigger is `timer`. In `/1` it is the simulator's clock,
     written `simulator`, and it is always enabled. The analysis declares it on every system and refuses a build that
     does not enable it ("is declared and the build does not enable it", `runtime.rs`, condition 5). With no task to
     release it is never due, so never taken; the composition then skips `Δ_timer`, though the timer service's cost
     still enters every source's blocking and jitter (the composite-inputs record §3). Releases start at the
     simulator's time zero, after initialisation (`RTP-H9`). Choosing a device's alarm on a target, and sharing it
     with an adapter's, is `M4.1.6`'s design.
   - **Routes**: each route `Joint::check` reports, one for each provider and kind (model §8 rule 8's "for each
     provider", by kind as `joint.rs` keys it, which `M4.1.2` writes into the rule; `Obligation::Route`), its interrupt
     a timer route or a source, with the requesters the route names:
     - a **timer route**, `Interrupt::Timer`: an alarm the requester programs. Its arrivals are as its code programs
       them, written `programmed`, and owed by the realization with its service cost. An adapter's cost interrupt is
       no interrupt of its own: `expiry` and `wake` are "one interrupt at its provider" (the adapters record's §2).
       The `/1` kinds program `relative-delay`, a consumed fact, so their provider's route is a timer route. Each cost
       is written on that route's line, with the instance that takes it. A shared instance's cost counts once;
     - an **external source**, `Interrupt::Source`: its minimum separation, the duration the provider states, zero
       included, which the analysis refuses (its condition 3) and `M1.45` has the language refuse; `absent` where it
       declares the fact absent; or `unknown` where it states none, owed to the description. On the playground a
       stated separation is held, not only assumed: the simulator injects no arrival sooner (`RTP-H9`), so its
       evidence is the run's; a target's is `/2`'s to owe. Its service
       cost, its acknowledge point and the work it defers are the variant's other inputs for a source, owed by the
       realization that serves it, with the code facts the composition reads of that service: whether it runs
       application code, and that each arrival makes one request (the composite-inputs record §1 and §2). Whether the
       source reaches the processor through the controller, its `external`, is the simulator's (`RTP-H9`).

     Each route has the level it is delivered at, the profile's caller level, and is declared **deliverable**, as the
     composite-inputs record asks of "every declared source" (`crates/rt-analysis/src/compose.rs` reads it).
   - **The order among routes** is the plan's to set (the composite-inputs record §1's `ahead(x)` among sources). Each
     route gets a distinct **rank**, rank `1` taken first, so the order is strict by construction:
     - the sources with a stated separation, ascending by it, ties broken on their interrupt's text;
     - then the sources whose separation is `unknown` or `absent`, by their interrupt's text;
     - then the timer routes, by their interrupt's text.

     The timer routes come last because `/1` has no key for them: the analysis refuses them until `M4.1.6` gives them a
     charge, and that design gives their place too, such as deadline-monotonic over an expiry's delivery bound. The
     release timer's place against the sources is the hardware's fact `external-before-timer`, which speaks of external
     interrupts; the simulator states it as `yes`, every source before the release timer, and takes a timer route before
     it too until `.6` places one (`RTP-H9`). The emulator target, on [QEMU](../book/src/ledger.md#qemu), states `no`
     (the composite-inputs record §2), so a comparison of hosted and emulated runs (`M4.9`) reads each against its own
     order (`RTP-H11`).

     Among the sources this is rate-monotonic over their separations. It is a **heuristic**, and no optimality is
     claimed for it. Service is non-preemptive among sources, interrupts not nesting (the variant's condition 5), and
     for fixed-priority non-preemptive scheduling of tasks "DMPO is not optimal for constrained-deadline task sets"
     ([Davis](../book/src/ledger.md#davis-speedup-factors), citing George et al., 1996). Audsley's algorithm is
     optimal for any schedulability test meeting three conditions, the first that "The schedulability of a task k
     may, according to test S, depend on any independent properties of tasks with priorities higher than k, but not
     on any properties of those tasks that depend on their relative priority ordering" ([A Review of Priority Assignment
     in Real-Time Systems](../book/src/ledger.md#davis-2016-priority-assignment), §V); for a test that fails them,
     "optimal priority assignment remains an open problem" ([Davis and
     Burns](../book/src/ledger.md#davis-burns-priority-assignment), §4.1). The
     composition's bound for a source, `Δ_x`, reads the jitter `J_q = Δ_q` of each source `q` ahead of it, and `Δ_q`
     depends on the order of the sources ahead of `q` (the composite-inputs record §3): the test fails Condition 1.
     A bound that counts each source ahead at its no-loss limit instead depends on no order, so it meets Condition 1;
     Audsley's algorithm is optimal for it only if it meets the other two conditions of §V as well, which this record
     does not show. The scheduling checker may search with it all the same, and confirms any order it finds with the
     composition before naming it (`RTP-H4`), so soundness never rests on the search's optimality.

     Soundness does not rest on the order. `compose.rs` refuses an order that is not strict, a source with no rank and
     a source not deliverable, and otherwise takes the order given. It does rest on the run realising the ranks: the
     simulator takes the routes in rank order (`RTP-H9`).
   - **The enabled set**: `timer`; each route. Each is an item of its own kind, so no two interrupts share a name. The
     simulator raises no other interrupt (`RTP-H9`), so there is no platform's non-maskable interrupt or firmware
     trap to add.

     The analysis variant analyses one timer, the release timer, and model §8 rule 8 leaves a second timer to it. Its
     condition 5 compares the enabled set with the declared one, the sources and `timer`, and refuses an enabled
     interrupt no source declares: "unmodeled-interrupt-load" (`runtime.rs`). The scheduling checker gives each
     enabled item an identifier of its own kind and declares no source for a timer route (`RTP-H4`). So a system with
     a timer route, one binding an adapter or a multiplexed alarm among them, is refused `unsupported-profile` by the
     analysis until `M4.1.6` gives that interrupt a charge. The plan carries the route, its costs and its requesters,
     and the analysis says why it cannot judge them.
   - **The interrupt model**, §7.3's per-source "masking constraints, and whether interrupt nesting is possible":
     interrupts do not nest, and every masked section masks every interrupt. These are the variant's conditions; the
     plan states them and the simulator realises them (`RTP-H9`).

   `M4.1.2` amends model §8 rule 8, whose plan "enables each such interrupt and no other", to: the release timer,
   every route's interrupt, timer routes included, and, on a target, the platform's non-maskable interrupts and traps
   (`/2`); no other.

6. **Execution contexts** (`M3.3.8`'s obligation). One line for each task and each node. The contexts are:
   - `job` for a task, and for every node a task reaches. A task released by an interrupt runs as a job after it, so
     a task's line is `job` alone;
   - `(interrupt I)`, for interrupt `I` at provider `X`, a timer route or a source, for the code its interrupt runs:
     - `X` itself, whose code serves its interrupt;
     - a node the route names that is bound to `X` itself, for the interrupt fact where the route is a source, or for
       a consumed fact where it is a timer route: its handler, or the alarm it programs, is its own code;
     - for a timer route at `X`, an instance bound to `X` for a consumed fact, which takes its cost there;
     - a node the route names through such an instance, when the instance's kind runs its requester's code in that
       interrupt. The adapters record states no kind's either way, so the plan reads `expiry`, a delivery to its
       requester, as running it, and `wake`, an epoch advance no requester observes but by reading, as running the
       instance's alone (`M4.1.2` has the record state it);
     - every node one of these reaches.

     A task or the system's own level that a route names adds no code: the interrupt runs `X`'s code and what these
     bullets add, and releases the task only where its trigger names `X`. A timer route or an expiry that would
     deliver to either is refused (§1);
   - `initialisation`, for every node.

   A provider's code runs in the contexts of the requesters bound to it, which is why reaching passes through bindings.
   That over-approximates: a provider takes every context any of its requesters runs in, which can refuse a record
   that would have served, never accept one that cannot. The code that serves each node states the contexts it
   supports (`M3.3.8`: "judged by inclusion, as a level is"). On the playground that code is generated, so each node's
   `contexts` is owed by the realization. Once a record can be a provider (`M3.7`), the lowering judges the record's
   supported contexts against the context line of each node it serves, and refuses `unsupported-profile` a context the
   record does not include (`RTP-H10`). **No step of initialisation may wait on an interrupt**, an alarm's or a
   source's, since initialisation runs masked (`M3.3.8`). For generated code it is the emitter's (`RTP-H2`); for a
   record, an obligation on its initialisation entry (`RTP-H6`). `M4.10` checks the image (`RTP-H6`).

7. **Initialisation.**
   - The graph is `Joint::initialization` over the system and the record's bindings (model §8 rule 12), which the
     lowering computes (`M4.1.4`).
   - Its edges are written with their reasons, as §10.1's "initialization dependencies" and `M4.1`'s Goal ask: one line
     for each ordered pair of nodes an edge joins, the node the edge orders earlier first — so a component's mutual
     edges are two lines — naming each reason `Joint::initialization` gives, a binding's fact or `named` for a `uses` or
     `needs` — a block's or a platform's `uses` or `needs` giving none, hardware's names never a call (§2); an edge
     across composition, a `uses`, is none either, while one to a block a `needs` names stands (rule 12).
   - Its steps are its strongly connected components, in a topological order taken by repeatedly choosing, among the
     components whose predecessors have all been taken, the one whose least member's text is least.
   - A step names its members by their text, in order.
   - This engine initialises each node in **one phase**. `M4.1.2` writes that answer into rule 12, which leaves it to
     `M4.1`.

   **A cycle stays as rule 12 has it.** Two services that call each other at run time can each initialise without the
   other (`M3.3.7`, held by `two_services_using_each_other_are_one_component_not_a_refusal`). So a component of several
   members is one step: the members are initialised in their texts' order, all before any successor. This carries an
   obligation: no member's initialisation calls another member's code. Since `M4.1.14` and `M4.1.15` a component can
   hold hardware and instances too, a block bound to the timer whose alarm it programs: the answer holds for them,
   re-derived. A member's initialisation sets its own state; programming an alarm is a call of the timer's code, work
   for run time, and initialisation, run masked, waits on no interrupt (§2.6); so the obligation binds hardware as it
   binds services. For generated code it is the emitter's (`RTP-H2`), which generates no such call; for a record it is
   stated by its initialisation entry (`RTP-H6`); `M4.10` checks the image (`RTP-H6`). Where a record states such a
   call, the lowering refuses (§1).

   **The epoch**, the instant of every timer-released task's first nominal release, is the simulator's time zero. The
   simulator runs initialisation to its last step with interrupts masked and enables them at time zero (`RTP-H9`), so
   every nominal release is at or after the first enabling: `releases-after-initialisation` (the composite-inputs
   record §2) on the playground. A task with a minimum separation is first released by an arrival, after the first
   enabling. On a target the epoch is `/2`'s.

8. **Memory and regions.** `/1` writes neither. A system with placement memory is refused (§1); a hosted system's
   memory is the playground's, and the simulator maps each device the plan names (`RTP-H9`). The regions of a target's
   memory map, with which of them are executable and which reserved, are `/2`'s (`.7`).

9. **Obligations**, each written with its kind, its subject and the leaves that discharge it, exactly as this table
   gives them:

   | Kind | Subject | Discharged by | From |
   | --- | --- | --- | --- |
   | an adapter kind's obligation, such as `update` | the instance's text | `M4.2 M4.8`: the generated code realizes it (`RTP-H2`), and the report lists it until it is discharged | the instance's kind (the adapters record's §9) |
   | `multiplex` | the node, an instance aside | `M4.2 M2.21.2`: its realization serves every user through its one channel and states its overhead (`RTP-H2`), and the scheduling checker charges it with the term `M4.1.6` designs (`RTP-H4`), as model §8 rule 4 asks of both | the rule below, over the system's tasks and interrupts |
   | `route` | the interrupt, `(timer-route NAME)` or `(source NAME)` | `M4.2 M4.3`: the emitter configures the enabled set (`RTP-H1`), and the simulator delivers it (`RTP-H9`) | `Obligation::Route`, one for each provider and kind |

   `Obligation::Placement` is `Joint::map`'s, and comes only with placement memory, which `/1` refuses. A node that
   takes a channel — a service, a policy, or a block or platform with requirements of its own; an instance aside, whose
   kind's obligations say how it shares — is **multiplexed** where it has two or more **users**: each of the system's
   tasks that reaches it (§2), directly or through services, policies and bindings; and each interrupt whose context
   line (§2.6) holds it, but for its own timer route — at the provider whose channel it takes, or that the instance
   whose channel it takes programs — where nothing else that route runs reaches it: that route's own code delivering to
   it is no caller, but another node's code there calling it is. Its users are the jobs and the interrupt routines that
   can call it, whatever declaration names it; the system's own level is none (model §8 rule 4). So
   [OSEK](../book/src/ledger.md#osek-os) counts a shared resource's users once resources are used at interrupt level:
   "The ceiling priority shall be set at least to the highest priority of all tasks and interrupt routines that access a
   resource or any of the resources linked to this resource" (§8.6, "OSEK Priority Ceiling Protocol with extensions for
   interrupt levels"). A channel taken on a consumed fact makes a timer route, so a multiplexed node's system is one the
   analysis refuses until `M4.1.6`. The heartbeat's `console.write`, used by both tasks, takes no channel and owes no
   `multiplex`. The lowering derives the obligation by this rule. `Joint::check` reports its own today: a service whose
   users, the tasks and services naming it by `uses` over every system of the description, number two or more; `M4.1.2`
   makes rule 4's users, and so `Joint::check`, count as this rule does, system by system as rule 7 resolves.

10. **Owed facts**, in one list, each a subject, a fact and its owner. Exactly these are written:

    | Subject | Facts | Owner |
    | --- | --- | --- |
    | each task | `entry-point`, `execution-bound`, `execution-category`, `figures-under-preemption`, `masked-category`, `masked-runs`, `masks-only-through-api`, `os-calls`, `stack-bound`, `stack-category`, `task-facts` | `application` |
    | the system | `leaves-interrupt-hardware-alone` | `application` |
    | each task that writes no `deadline-from` | `deadline-from` | `description` |
    | each task whose trigger is `unknown` | `trigger` | `description` |
    | each source whose separation is `unknown` | `separation` | `description` |
    | each node | `contexts`, `no-wait` | `realization` |
    | each node whose context line holds `job` | `call-cost`, the time one call of its code takes in a job | `realization` |
    | each instance whose kind has costs | `cost-terms` | `realization` |
    | each instance's obliged fact | that fact | `realization` |
    | each node a `multiplex` obligation names | `overhead` | `realization` |
    | each source | `service-cost`, `acknowledge-point`, `deferred-work`, `no-application-code`, `one-request-per-arrival` | `realization` |
    | each timer route | `service-cost`, `arrival-bound` | `realization` |

    The owners are the application's inputs (`RTP-H5`), the description, and the generated code (`RTP-H2`). No
    catalog record describes the playground, so `/1` owes the catalog nothing, and the platform's own figures are the
    simulator's (`RTP-H9`). The plan's last line says whether it is complete, and `/1`'s never is (§2.2).

§7.5's "selected provider IDs" are the bindings' providers. Selected catalog records join them when a record can be a
provider (`M3.7`), and the plan then carries each record's identity.

### 3. The format, `archogen-runtime-plan/1`

Canonical text, fixed here so that its writer and every reader derive it from this record alone:
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
(task INDEX (task SYSTEM NAME) (rank INT) RELEASE (deadline DUR) REFERENCE (jitter DUR) (overrun POLICY) (released-by TRIGGER) (uses NAME …))
(bind REQ FACT PROVIDER)
(instance INSTANCE OBLIGED … (obligations NAME …) (costs NAME …))
(timer simulator (releases (task SYSTEM NAME) …))
(route INT ITEM (separation DUR|unknown|absent|programmed) (level LEVEL) (deliverable yes) (requesters REQ …) (costs (INSTANCE NAME) …))
(enabled ITEM …)
(interrupt-model (nesting no) (masking all))
(context SUBJECT CONTEXT …)
(depends NODE NODE (because REASON …))
(step INT NODE …)
(obligation KIND SUBJECT (discharged-by LEAF …))
(owed SUBJECT FACT (by OWNER))
(complete yes|no)
```

**Values:**
- `INDEX` is the task's runtime index (§2.2), an `INT`; `SYSTEM` is a system's name.
- `PROFILE` is the profile's identifier and `VERSION` the adapter catalog's version, each as `/2` writes it.
- On a `bind` line, `REQ`, `FACT` and `PROVIDER` are `/2`'s, copied as written (§2.3).
- `RELEASE` is `(period DUR)` or `(min-separation DUR)`, its duration never zero (§1).
- `REFERENCE` is `(deadline-from release)`, or `(deadline-from)` where the description writes none, a form no
  description can write.
- `POLICY` is written as the description writes it.
- `TRIGGER` is `timer`, `(source NAME every)` or `unknown`.
- `OBLIGED` is `(obliged FACT DUR)`, or `(obliged FACT)` for a fact offered bare.
- A list with no members is written with its head alone: `(uses)`, `(releases)`, `(requesters)`, `(costs)`.
- `LEVEL` is the profile's caller level.
- `INSTANCE` is the instance's `(adapter …)` text. A cost on a route is `(INSTANCE NAME)`, `NAME` the cost's name in
  the kind.
- `ITEM` is an interrupt (§2): `timer`, `(timer-route NAME)` or `(source NAME)`. A `route` line's is a timer route or a
  source; `programmed` is a timer route's separation, and a source's is never `programmed`.
- `SUBJECT` is a task's `(task SYSTEM NAME)` or a node's text in a `context` line; as §2.9's and §2.10's tables give it
  in an `obligation` or `owed` line: a task, a node, an instance, an interrupt, or the system's name.
- `CONTEXT` is `job`, `(interrupt ITEM)` with a timer route or a source, or `initialisation`.
- `REASON` is `(binding FACT)` or `named`.
- `KIND` is an adapter kind's obligation name, `multiplex` or `route`; `LEAF` a task-tree identifier.
- `OWNER` is `application`, `description` or `realization`.
- `FACT` in `owed` is one of: `entry-point`, `execution-bound`, `execution-category`, `figures-under-preemption`,
  `masked-category`, `masked-runs`, `masks-only-through-api`, `os-calls`, `task-facts`, `stack-bound`,
  `stack-category`, `leaves-interrupt-hardware-alone`, `deadline-from`, `trigger`, `separation`,
  `contexts`, `no-wait`, `call-cost`, `cost-terms`, `overhead`, `service-cost`, `acknowledge-point`, `deferred-work`,
  `no-application-code`, `one-request-per-arrival`,
  `arrival-bound`, or an obliged fact's name.

**Forms written only where §2 calls for them**, none where there is none:
- `(task …)`, one for each task, and at least one (§1);
- `(bind …)`, one for each binding of the record;
- `(instance …)`, one for each instance;
- `(route …)`, one for each route;
- `(context …)`, one for each task and each node;
- `(depends …)`, one for each ordered pair of nodes an edge joins (§2.7);
- `(step …)`, one for each component of the initialisation graph;
- `(obligation …)`, one for each obligation §2.9 lists;
- `(owed …)`, one for each owed fact §2.10 lists.

`(timer …)`, `(enabled …)` and `(interrupt-model …)` are always written.

**Orders:**
- `task` lines by index, starting at `0`; `releases` items by index;
- `bind` lines in `/2`'s order;
- `instance` lines by their text. Within a line come the `obliged` items in the kind's offer order, then the
  `obligations` names in the kind's written order, then the `costs` names in the kind's written order;
- `route` lines by rank, starting at `1`; on a route's line, `requesters` items and `costs` items each by their text;
- `uses` items by name;
- `enabled` items: `timer`, then the routes by rank;
- `context` lines by subject's text; within a line, `job`, then each `(interrupt …)` by its text, then
  `initialisation`;
- `depends` lines by the earlier node's text, then the later's; within a line, `(binding …)` reasons by fact, then
  `named`;
- `step` lines by index, starting at `1`;
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
(context console.uart job initialisation)
(context console.write job initialisation)
(depends console.uart console.write (because (binding observable-output)))
(step 1 console.uart)
(step 2 console.write)
(owed (task heartbeat beat) entry-point (by application))
(owed (task heartbeat beat) execution-bound (by application))
(owed (task heartbeat beat) execution-category (by application))
(owed (task heartbeat beat) figures-under-preemption (by application))
(owed (task heartbeat beat) masked-category (by application))
(owed (task heartbeat beat) masked-runs (by application))
(owed (task heartbeat beat) masks-only-through-api (by application))
(owed (task heartbeat beat) os-calls (by application))
(owed (task heartbeat beat) stack-bound (by application))
(owed (task heartbeat beat) stack-category (by application))
(owed (task heartbeat beat) task-facts (by application))
(owed (task heartbeat chime) entry-point (by application))
(owed (task heartbeat chime) execution-bound (by application))
(owed (task heartbeat chime) execution-category (by application))
(owed (task heartbeat chime) figures-under-preemption (by application))
(owed (task heartbeat chime) masked-category (by application))
(owed (task heartbeat chime) masked-runs (by application))
(owed (task heartbeat chime) masks-only-through-api (by application))
(owed (task heartbeat chime) os-calls (by application))
(owed (task heartbeat chime) stack-bound (by application))
(owed (task heartbeat chime) stack-category (by application))
(owed (task heartbeat chime) task-facts (by application))
(owed console.uart call-cost (by realization))
(owed console.uart contexts (by realization))
(owed console.uart no-wait (by realization))
(owed console.write call-cost (by realization))
(owed console.write contexts (by realization))
(owed console.write no-wait (by realization))
(owed heartbeat leaves-interrupt-hardware-alone (by application))
(complete no)
```

Its platform, `host.playground`, is bound by nothing and is no node, so it has no line. `console.uart` runs in the
tasks' jobs, since `console.write` is bound to it. `console.write` takes no channel, so it owes no `multiplex` (§2.9).

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
(route 1 (timer-route timer.short) (separation programmed) (level machine) (deliverable yes) (requesters time.monotonic) (costs ((adapter epoch-extender timer.short) wake)))
(enabled timer (timer-route timer.short))
(interrupt-model (nesting no) (masking all))
(context (adapter epoch-extender timer.short) job (interrupt (timer-route timer.short)) initialisation)
(context (task short-counter beat) job)
(context time.monotonic job initialisation)
(context timer.short job (interrupt (timer-route timer.short)) initialisation)
(depends (adapter epoch-extender timer.short) time.monotonic (because (binding counter-width) (binding observation-coherent) (binding tick-rate) (binding unambiguous-horizon)))
(depends timer.short (adapter epoch-extender timer.short) (because (binding counter-modulus) (binding observation-coherent) (binding relative-delay) (binding tick-rate) (binding wrap-behavior)))
(step 1 timer.short)
(step 2 (adapter epoch-extender timer.short))
(step 3 time.monotonic)
(obligation coherent (adapter epoch-extender timer.short) (discharged-by M4.2 M4.8))
(obligation route (timer-route timer.short) (discharged-by M4.2 M4.3))
(obligation update (adapter epoch-extender timer.short) (discharged-by M4.2 M4.8))
(owed (adapter epoch-extender timer.short) call-cost (by realization))
(owed (adapter epoch-extender timer.short) contexts (by realization))
(owed (adapter epoch-extender timer.short) cost-terms (by realization))
(owed (adapter epoch-extender timer.short) no-wait (by realization))
(owed (task short-counter beat) entry-point (by application))
(owed (task short-counter beat) execution-bound (by application))
(owed (task short-counter beat) execution-category (by application))
(owed (task short-counter beat) figures-under-preemption (by application))
(owed (task short-counter beat) masked-category (by application))
(owed (task short-counter beat) masked-runs (by application))
(owed (task short-counter beat) masks-only-through-api (by application))
(owed (task short-counter beat) os-calls (by application))
(owed (task short-counter beat) stack-bound (by application))
(owed (task short-counter beat) stack-category (by application))
(owed (task short-counter beat) task-facts (by application))
(owed (timer-route timer.short) arrival-bound (by realization))
(owed (timer-route timer.short) service-cost (by realization))
(owed short-counter leaves-interrupt-hardware-alone (by application))
(owed time.monotonic call-cost (by realization))
(owed time.monotonic contexts (by realization))
(owed time.monotonic no-wait (by realization))
(owed timer.short call-cost (by realization))
(owed timer.short contexts (by realization))
(owed timer.short no-wait (by realization))
(complete no)
```

How it is derived:
- the timer route is `Joint::check`'s. The instance's `relative-delay` binding takes `timer.short`'s channel, and the
  route names the requester bound to the instance, `time.monotonic` (`joint.rs`, `routes` and `relying`);
- `timer.short` runs in its own interrupt, and the instance with it, which takes its wake there. Both also run in
  `beat`'s job, which reaches them through `time.monotonic`. `time.monotonic` runs in the job alone, since a wake runs
  only the instance's code;
- `soc.short` is bound by nothing and is no node;
- the analysis refuses this plan under condition 5, `(timer-route timer.short)` being enabled and no source declaring
  it (§2.5).

**A source, worked.** A system whose one task is released by an external source, with no `deadline-from`:

```text
(defblock dev.e
  (offers (interrupt-source true) (min-arrival-separation 20 ms) (reachable-at-privilege machine)))
(defplatform soc.e
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses dev.e)))
(defservice evt
  (requires (needs interrupt-source)))
(defsystem sporadic
  (task s (min-separation 10 ms) (deadline 10 ms) (priority 1) (uses evt))
  (requires (uses evt))
  (platform (uses soc.e)))
```

Its `/2` record is the one line `(bind evt interrupt-source dev.e)`, and it lowers to exactly this text, but for its two
digests:

```text
(format "archogen-runtime-plan/1")
(profile rt-static-up-v1)
(system sporadic)
(program "…")
(assignment "…")
(adapters adapters/1)
(task 0 (task sporadic s) (rank 1) (min-separation 1/100 s) (deadline 1/100 s) (deadline-from) (jitter 0 s) (overrun fault) (released-by (source dev.e every)) (uses evt))
(bind evt interrupt-source dev.e)
(timer simulator (releases))
(route 1 (source dev.e) (separation 1/50 s) (level machine) (deliverable yes) (requesters evt) (costs))
(enabled timer (source dev.e))
(interrupt-model (nesting no) (masking all))
(context (task sporadic s) job)
(context dev.e job (interrupt (source dev.e)) initialisation)
(context evt job (interrupt (source dev.e)) initialisation)
(depends dev.e evt (because (binding interrupt-source)))
(step 1 dev.e)
(step 2 evt)
(obligation route (source dev.e) (discharged-by M4.2 M4.3))
(owed (source dev.e) acknowledge-point (by realization))
(owed (source dev.e) deferred-work (by realization))
(owed (source dev.e) no-application-code (by realization))
(owed (source dev.e) one-request-per-arrival (by realization))
(owed (source dev.e) service-cost (by realization))
(owed (task sporadic s) deadline-from (by description))
(owed (task sporadic s) entry-point (by application))
(owed (task sporadic s) execution-bound (by application))
(owed (task sporadic s) execution-category (by application))
(owed (task sporadic s) figures-under-preemption (by application))
(owed (task sporadic s) masked-category (by application))
(owed (task sporadic s) masked-runs (by application))
(owed (task sporadic s) masks-only-through-api (by application))
(owed (task sporadic s) os-calls (by application))
(owed (task sporadic s) stack-bound (by application))
(owed (task sporadic s) stack-category (by application))
(owed (task sporadic s) task-facts (by application))
(owed dev.e call-cost (by realization))
(owed dev.e contexts (by realization))
(owed dev.e no-wait (by realization))
(owed evt call-cost (by realization))
(owed evt contexts (by realization))
(owed evt no-wait (by realization))
(owed sporadic leaves-interrupt-hardware-alone (by application))
(complete no)
```

How it is derived:
- the route is `Joint::check`'s, at `dev.e` with `dev.e`'s own separation. `s` reaches `evt`, which the route names, and
  20 ms is at least `s`'s 10 ms, so every arrival releases `s`;
- `dev.e` runs in its own interrupt, and `evt` with it, bound to `dev.e` for the interrupt fact;
- the release timer is enabled though it releases nothing, as the analysis requires (§2.5).

The three texts are `/1`'s goldens, held by `M4.1.4` with the systems that pin the rest of §2: a provider raising both a
timer route and a source; a block named `timer`; a `deadline-over-delay` instance with and without a bound; a
multiplexed service, and one its users name by `needs`; a task binding `interrupt-source` itself; a task naming its
service by `needs`; a task naming a policy; a task bound to a device directly; the system's own level binding a source;
a sporadic task that also reaches an alarm; two sources of one separation; an extender over a provider raising both
kinds; a service two systems use; and the refused: a task serving `deadline-over-delay` directly, a task and the
system's level each programming an alarm themselves, a deadline measured from another event, and a periodic task bound
to a source itself. Round 7 adds a task naming a block; a task reaching a channel through a service, and one through a
policy; a frequency where a duration is due; a system with no task; a service bound to one provider for a source and,
through an extender, for its counter; a release jitter and `skip-late-job`; every rank class in one system; two
instances' costs on one route; the system's own level served by a shared extender; contexts spreading two services deep;
placement memory; an expiry to the system's own level; and each of §1's refusals of a record. Round 8 adds a source's
handler and a task sharing a channel, two handlers sharing one, and a policy taking one; the tightest obliged bound; a
sporadic task reaching only an alarm; a zero release separation; a module tree; and records that bind an instance no
requester is bound to, or another requester's exclusive instance, which do not rebuild. Round 9 adds services named with
`/`; callers on a node's own route; a lone user; policies handling a source and taking an expiry; a platform raising a
source; a zero deadline and a jitter past the period; co-applying refusals; a record the relation does not satisfy; and
a plan with no node. Round 10 adds `uses` cycles through a node's own route, durations past the exact arithmetic, a zero
source separation, and two ill-formed tasks. Round 11 adds blocks requiring an alarm and an output, several receivers of
interrupt code, and systems named by `uses`. Round 12 adds blocks and platforms with requirements nothing binds to, an
alarm, an expiry and a source handled by hardware, composition that is no call, a separation whose spellings overflow,
several durations past the arithmetic at once, and a bare separation. Round 13 adds hardware naming a service by `uses`
or `needs`, and a block by `needs`; a block bound to itself, and a deadline adapter over its own block; a block whose
one requirement is its level's; a block only a service names, and a service only composed; and the naming order. Round
14 adds names one of which prefixes another, in both naming refusals; hardware on another system's platform naming a
system; and a provider whose own requirement is bound to the node that takes its channel, calling it on that node's own
route. A description the language's check refuses is no input (§1), and its probe is held at that check's verdict.

The plan's identity is the sha256 of its text. It has one writer and one reader in `crates/archogen-plan`, and a plan
the reader accepts writes back the same bytes, as `archogen-result/1`'s rule asks. The scheduling checker reads it with
a reader of its own, held to the same goldens, as `/2` has two readers: the generator's reader is generator code, which
the scheduling checker must not share (`RTP-H4`). Each golden is lowered from frozen inputs, the crate's own
fixtures and never another suite's file, its result record a fixed text, and it is never rewritten. **The rules of §2
are part of the format**: a change to any of them, an order or the topological choice among them, is a new format
number. So is a change to what §2 reads, the joint rules of model §8 (rules 4, 7, 8, 12 and 15 among them), the
vocabulary or the adapter catalog, that moves a plan for the same inputs. `docs/book/src/versions.md` registers the
format (`VERSION-REGISTER`). Lowering the same inputs twice writes the same bytes (M4's "Repeated generation produces
identical canonical plans").

### 4. What `M4.1` builds, and what it hands on

**Built by `M4.1`'s children:**
- `.2`: model §8 rules 4, 8, 10 and 12 amended, in the model, the generator's joint rules and the checker's, the
  checker's by a fenced context. Rule 4's users are §2.9's: of a node that takes a channel, an instance aside, the
  system's tasks that reach it, through `uses`, `needs` and bindings at any depth, and the interrupts whose context
  holds it, but for its own timer route — at the provider whose channel it takes, or that the instance whose channel it
  takes programs — where nothing else that route runs reaches it, never another system's; rule 7 still resolves system
  by system.
  Rule 8's enabled set is §2.5's. Rule 12 records this engine's answer, one phase, and counts among its nodes a block or
  a platform the system serves that is bound for requirements of its own (§2). Rule 10's are below. The adapters
  record's §9 reads "each cost a cost on the route of the provider it programs", and it states, for each cost, whether
  the interrupt runs the requester's code (§2.6). The engine API's `crates/archogen-api/src/resolve.rs` and
  `docs/book/src/engine-api.md`, which read "a cost the scheduling analysis charges as a source", read so too. The book
  follows, with a migration note for any verdict that moves. Rule 10's amendments keep the search, the checker and any
  later layout agreed over a description:
  - **placement memory with no size** is `missing-fact`, naming it, since nothing can be laid out in it;
  - **a hole of no size** whose base lies inside placement memory (`M3.5.5.7.3`'s obligation) is taken to reach the
    region's end. That is the conservative extent, the largest the region allows. If it leaves no free byte, the system
    is `missing-fact`, naming the hole and asking for its size;
- `.3`: the generator's reader of `/2`, its round trip held against its writer, and the typed record it returns. The
  record's bindings are read back with each instance rebuilt from the catalog and each beneath binding, which the
  search's `Resolution.bindings` omits. `Joint::check` and `Joint::initialization` run over them, and `Joint::map` over
  the system, at the profile's caller level, as the search runs them;
- `.4`: the crate, the typed plan, its writer and reader, the lowering, its refusals and its provenance; the format's
  register entry; the goldens of §3, the three worked texts and the systems named there, their inputs frozen as the
  crate's own fixtures;
- `.5`: the crate in the generator root's `(role-packages …)` in `trust/roots.eadl`, each item it
  shares with another role classified by its commit (F30); the book's chapter; and every other place that names `M4.1`
  as owner, moved as below;
- `.6`: **the release alarm**, a design of its own, reviewed. It decides which device's alarm releases timer tasks on a
  target, and how an adapter's alarm shares a device with releases, each taking a channel (model §8 rule 3). It decides
  how the analysis charges a timer route's interrupts, its service cost and arrival bound, such as a wake taken as a
  timer event of the release timer under an amended condition 6; the term for a multiplexed node's overhead and for a
  node's call cost in a job; how a job receives an alarm it programs itself or an expiry, and what the system's own
  level's requirement for either means; and the timer routes' place among the routes and against the release timer.
  That design is the joint rules', the analysis variant's and the composite-inputs record's;
- `.7`: **format `/2`**, after `.6`, `M4.4.1`'s device correspondence (`RTP-H8`) and `M4.12`'s application inputs
  (`RTP-H5`), designed and reviewed as this record was. Its questions are kept in the review history, each with the
  fix its reviewer proposed:
  - a target as an input, in the plan's identity, and how the command line names one; the target's hardware, every
    block and platform its file declares; `Joint::map` over it, its refusals, and a placement region chosen by rule;
  - the memory map's regions, each with whether it is executable or reserved, and the requesters bound to its
    provider, a hosted system's device regions among them, for §10.3's "resource ownership";
  - the image's place below every allocation in its range; the allocation's formula; the "no room" refusal; an
    unsized hole that truncates an image, named with a request for its size;
  - the stacks: the boot stack and its owners, each task's stack from `RTP-H5`'s bound, the interrupt stack with the
    fault path's use, a generated service's stack use;
  - the catalog's sizes, the frames, the guard, the alignments and the tables, owed to the records by a hand-off `.7`
    writes, with their identity and evidence category in the plan, and the platform's figures the analysis reads;
  - the platform's non-maskable interrupts and firmware traps in the enabled set;
  - the route ranks mapped onto the controller's levels above its threshold, or `unsupported-profile` naming the
    level count, each source's controller line and identifier written for its priority;
  - the epoch on a target, read and its first compare written as the last act before the transition into idle, so
    that its bound is the switch record's;
  - ticks, each compare at the ceiling of its release, and the horizon tested over consecutive releases;
  - a step of `M4.4`'s, the linked image checked against the allocations, ordered after `.7`.

**What else names `M4.1`**, discharged or moved by `.5`:
- `crates/eadl-model/src/profile.rs`'s rule "one execution context runs at a time", owner `M4.1`. The plan states it:
  no nesting, one core, and each requester's contexts. The runtime realizes it, so its owner moves to `M4.2`.
- S0's `whole-millisecond-periods` moves to `M4.2`, which the plan's exact durations let end. That covers its row in
  `decision_s0-retirement.md`, its marker and hint in `crates/archogen-s0/src/interpret.rs` and that file's test,
  `docs/book/src/s0.md`, and `examples/s0-heartbeat/README.md`.
- `no-lock-data` and `--locked` move to `M4.7`, which writes the whole lock data. That covers the row, the comments,
  hint and test in `crates/archogen-cli/src/build_cmd.rs` and `tests/s0_build.rs`, `resolve_cmd.rs`'s refusal of
  `--locked`, `docs/book/src/cli.md`, and `docs/book/src/s0.md`'s lock-data passages, which its transcript test reads.
- `crates/archogen-api/src/resolve.rs`'s and `docs/book/src/engine-api.md`'s "charges as a source (`M4.1`)" are
  reworded by `.2`.
- The remarks naming `M4.1` in `crates/eadl-resolve/src/joint.rs` and its tests stand: they state this leaf's role
  truly.

**Written into other leaves with this record**, quoted word for word by each receiving leaf (`HANDOFF-LEDGER`):

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RTP-H1` | `M4.2` | The emitter moves `archogen build` onto the runtime plan for the hosted playground, its faces in step on the command line, the engine API and the [MCP](../book/src/ledger.md#mcp-specification) tool, the enabled set configured as the plan writes it and the routes' ranks realized as the order the simulated controller takes them in, each source trigger the plan infers reported, a plan holding a task released by `unknown` refused naming the task (`missing-fact` where no source's route reaches it or its one source states no separation; otherwise `unsupported-profile`, since `eadl/1` cannot say which arrivals release it), each job an engine-owned empty body that the release trace alone observes until `M4.12`'s inputs give it code, the build saying it runs no application code, the scheduler built with `from_eadl_ranks` and its slot held to the plan's index, and S0's `whole-millisecond-periods` and the profile's "one execution context runs at a time" discharged. |
| `RTP-H2` | `M4.2` | The generated code realizes the adapters' and the multiplexed nodes' obligations, initialises no component member by calling another's code, never waits on an interrupt in initialisation, and states each cost's terms, each multiplexed node's overhead, each bound an obliged fact offered bare leaves to it, the contexts each node's code supports, and each other value the runtime plan owes to the realization. |
| `RTP-H3` | `M4.2` | The emitter accepts a runtime plan only by lowering it again from the same inputs and comparing bytes, and marks a plan whose acceptance was taken as text experimental. |
| `RTP-H4` | `M2.21.2` | After `M4.1.4` has written the runtime plan's goldens, the scheduling checker reads the plan with a reader of its own held to them and maps it onto the analysis's inputs: each enabled interrupt given an identifier of its own kind, so that no two share one, and no source declared for a timer route, so that its interrupt is refused as unmodeled load; where an input the analysis needs is unknown, absent, unstated or owed, the analysis reported inconclusive, naming it, never run on a default; each term `M4.1.6` designs, a timer route's charge and a multiplexed node's overhead among them, charged once it exists; a refusal under the plan's source order worded as one another order may admit, and an order searched with Audsley's algorithm over a bound counting each source ahead at its no-loss limit, one it finds confirmed with the composition and named as the repair. |
| `RTP-H5` | `M4.12` | The application inputs have a format — each task's entry point, execution bound and its evidence category, OS calls with counts, masked runs and their bounds' evidence category, task facts, declarations that it masks only through the runtime API and that its own figures hold under any preemption, stack bound and its evidence category, and the application's `leaves-interrupt-hardware-alone` — that `archogen-runtime-plan/2` carries. |
| `RTP-H6` | `M4.10` | The catalog records state what the runtime plan owes them — supported contexts, an initialisation entry that never waits on an interrupt and names any other component member's code it calls, and each source's service cost — amending the catalog record's §12 and §14.4, and the image is checked: no initialisation step waits on an interrupt, and no component member's initialisation calls another member's code. |
| `RTP-H7` | `M4.7` | The build's metadata carries the runtime plan's sha256 and its provenance, the whole lock data of `ROADMAP.md` §10.3 is written, `--locked` is honoured, and S0's `no-lock-data` is discharged. |
| `RTP-H8` | `M4.4.1` | A system's devices are put in correspondence with a target's — which target device each system device is, by a rule the lowering of `archogen-runtime-plan/2` reads — before `M4.1.7` designs that format. |
| `RTP-H9` | `M4.3` | The hosted simulator runs initialisation to its last step with interrupts masked and enables them at time zero, then releases each timer-released task from its own exact clock at its nominal releases from time zero and each task released by `(source X every)` at each arrival of X and at no other time; takes its timer only when a release is due; takes every route before the release timer when both are pending — its `external-before-timer` for a source's, and for a timer route its own order until `M4.1.6` places one — and the routes in the plan's rank order; nests no interrupt and masks every interrupt in a masked section; raises no interrupt the plan does not enable; models each device the plan names, a timer route's alarm included; lets no source arrive before the first enabling, nor sooner after its last arrival than the separation the plan states for it, a source whose separation it states as zero, `unknown` or `absent` arriving only as the run's scenario states, which the run names beside its result; and states every platform input and fact the analysis variant and the composition read, `external-before-timer`, `releases-after-initialisation` and each source's `external` among them, in the form the scheduling checker reads. |
| `RTP-H11` | `M4.9` | A comparison of a hosted run and an emulated one reads each run against its own platform's `external-before-timer`, the simulator's `yes` and the emulator's `no`, never one run's order against the other's. |
| `RTP-H12` | `M2.30` | If the composition's soundness needs a fact of the application's beyond `leaves-interrupt-hardware-alone`, the runtime plan owes it with the others, its §2.2, §2.10 and `RTP-H5` moving in the same change. |
| `RTP-H10` | `M4.10` | Once a catalog record can be a provider (`M3.7`), the runtime plan's lowering judges the record's supported contexts against the context line of each node it serves, by inclusion, and refuses `unsupported-profile` a context the record does not include. |
| `RTP-H13` | `M4.10` | The build verifies the image against the runtime plan: the interrupts it enables and the controller's priorities as the plan's enabled set and route ranks give them, each multiplexed node realized through its one channel, and initialisation run in the plan's steps, a component's members in their texts' order and every step before its successors. |
| `RTP-H14` | `M1.42` | The description check refuses `invalid-description`, naming the clause, a task's `period`, `min-separation`, `deadline` or `jitter` that is not a time, and a `period` or `min-separation` of zero. |
| `RTP-H15` | `M1.43` | The description check refuses `invalid-description` a periodic task that needs `interrupt-source`, a second release model, naming both. |
| `RTP-H16` | `M1.45` | The description check refuses, where the description states it, a quantity whose value in its dimension's base unit lies past the engine's exact arithmetic, `unsupported-profile`, a task clause's and a block's or a service's fact's alike, and a source's `min-arrival-separation` of zero, `invalid-description`. |
| `RTP-H17` | `M1.46` | The description check refuses `invalid-description` a `uses` or `needs` naming a system, naming both. |
| `RTP-H18` | `M1.47` | Model §8 rules 1 and 7 state that a block's or a platform's `uses` and `needs` name only hardware, and the description check refuses `invalid-description` an operand that is neither a block nor a platform, naming both. |

`M4.12` is filed with this record, for the application inputs, and `M4.4.1` for the correspondence.

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
  another leaf's question: how a system's device corresponds to a target's (`M4.4.1`), how releases and an adapter
  share an alarm (the joint rules'), and sizes only records state. A plan for the playground needs none of them, and
  every rule in it is determined today, as its worked texts show. Those questions get designs of their own (`.6`, `.7`).
- **Why interrupts are named by kind, and every route ranked.** One provider can raise a timer route and a source, and
  a block may be named `timer`. Named by provider alone, the analysis would read a timer route as the release timer or
  a source, and charge nothing for it. Ranked after the sources, a device's alarm has a place in the simulator's order
  until `.6` gives it its own.
- The plan writes canonical S-expression text because it quotes eADL values and `/2`'s lines word for word: one form
  for both keeps them as written.

**Alternatives weighed.**
- **Lowering from `Resolution` in memory** is quicker, and would build what the search found rather than what the
  checker accepted.
- **A target in `/1`** (rounds 1 to 3): it needs the device correspondence and the release alarm, which `/1` would
  have decided for other leaves; round 3 showed that a target lowered by its rules needed choices in six places.
- **The release timer as the device bound for `tick-rate`** (rounds 2 and 3): `tick-rate` belongs to a counter, which
  may raise no interrupt, and the rule ignored the channels an adapter takes. That question is `.6`'s.
- **Enabling the release timer only where a task is timer-released** (round 3): the analysis declares it on every
  system and refuses a build that does not enable it, and a timer with nothing due is never taken.
- **`some` for a source faster than its task** (rounds 2 and 3): the analysis would assume a release at most once per
  task separation, and nothing would make the runtime keep it. An enforced inter-arrival time would make it sound, as
  in "The Operating System module shall limit the inter-arrival time of Tasks to one per OsTaskTimeFrame"
  ([AUTOSAR](../book/src/ledger.md#autosar-os), SWS_Os_00465). But dropping an arrival is the description's choice to
  make, which `eadl/1` cannot write, so the trigger stays `unknown` and owed.
- **Making a cycle a conflict of the assignment** (round 1's answer) contradicts `M3.3.7`'s reviewed reading: run-time
  calls are no initialisation order. Its edges named by `uses` do not depend on the assignment, so no other assignment
  helps.
- **Ordering sources by the most urgent task they release** has no key for a source that releases none, and
  disregards the source's own loss limit. **Deadline-monotonic over each source's no-loss limit** needs the
  acknowledge point, which the playground owes the realization; once it is stated, the scheduling checker's search
  covers it.
- **Audsley's assignment in the lowering** needs the sources' costs and a schedulability test, the realization's and
  the scheduling checker's, and the composition's test is not one it is proven optimal for (§2.5).
- **Ranking a timer route among the sources** (round 5's wording, by separation) ranked it after every source
  anyway, its separation being `programmed`; and a device's alarm needs a key of its own, such as an expiry's delivery
  bound, which `.6` decides.
- **A source for each adapter cost** duplicates its provider's route, and the analysis refuses two sources of one name.

## How to apply

- `M4.1`'s children are `.1` this record, reviewed, then `.2` to `.7` as §4 names them, and from `.8` on the leaves its
  rounds file for what they find, each named in its round's answer. `.3` and `.4`'s lowering are built ahead, as this
  record's executable form, each rule citing its section, with every probe of the review a fixture; from round 7 a round
  reviews the record, the code and the corpus together (`decision_executable-design-reviews.md`). A corpus outcome is
  what the code writes, blessed: a review judges it against these rules, and it is never a rule. Beside the corpus, a
  bounded exhaustive checker (`M4.1.13`) holds every plan of generated universes to a reading of §2 written by a context
  that did not read the lowering's code, as `decision_executable-design-reviews.md` item 1 asks.
- A change to what the lowering reads — the search, the record's reader, the joint rules — is re-derived against this
  record in the same change, its rules restated where they move: round 12 found three defects where `M4.1.14` made
  hardware a requester and nothing here followed.
- A consumer reads the plan with a reader held to the goldens, never by matching text.
- Related: [[decision_s0-retirement]], [[decision_eadl-engine-boundary]], [[decision_priority-comparison-direction]],
  [[decision_zero-dependency-engine-core]], `docs/specs/adapters/decision_engine-adapters.md`,
  `docs/specs/catalog/decision_runtime-composite-inputs.md`.
