# Format `/2`: a runtime plan for a system on a target, and for one with its application's inputs

- **Type:** `decision`
- **Date:** `2026-11-07`
- **Status:** `active` — written by leaf `M4.1.7.1`; nothing here is code yet, and it is under review in leaf `M4.1.7.2`
- **External sources:** none: this record decides how the engine lowers its own records
- **Owner / source:** leaf `M4.1.7` (`docs/tasks/M4.md`), from `decision_runtime-plan.md` §4 `.7` and `docs/reviews/decision_runtime-plan-reviews.md`'s "Kept for `M4.1.7`"; the hand-offs `RA-H1`, `RA-H8` (`decision_release-alarm.md`), `DC-H1` to `DC-H4` (`decision_device-correspondence.md`) and `RTP-H5` (`decision_application-inputs.md`); `ROADMAP.md` §6.2, §7.3 and §10.3.

## The fact / decision

**In plain words.** A runtime plan says what a checked assignment turns into. The first format, `/1`, describes a system on the hosted playground, where the simulator provides the clock, the memory and the devices, and it has no line for a figure about a task's own code, so it is never complete. Format `/2` is the same plan with what a real machine adds: which machine it is, which device releases tasks and which each alarm takes, how each count on those devices becomes a time, where each stack lies in memory, and what the interrupt controller is told. It also carries what the application's inputs say, so that what `/1` could only list as owed can be stated.

The rule that keeps it honest is `/1`'s: nothing is invented. A size only a catalog record can state is owed until a record states it, and every allocation after it is owed too. A plan with an owed figure says `(complete no)`.

`/1` stays as it is, frozen, and a command with neither of the two new inputs still writes it.

### 1. What is given, and on what command

`--target FILE` names a target's file, read by `Target::read` as `decision_device-correspondence.md` says, and `--application FILE` the application's inputs, read by `archogen_plan::application` (`decision_application-inputs.md`). Either on `resolve`, `plan-check` or `build` asks for `/2`, and `check` takes `--target`. A file is named by its path and identified by what it holds, never found by a name under a directory of targets, since an installed tool carries none. The hosted playground is `/2`'s name for no target, written `(target hosted-playground)`, as an application's file writes it, and reached by `--application` alone: `M4.13` builds it first.

- **The target in the identity** (R3-4, R5-R4). A plan on a target writes `(target NAME "HEX")`, `NAME` the file's one platform and `HEX` the sha256 of its forms' program text as `eadl_resolve::plan::program_text` writes a program's, so a comment moves no plan. An assignment on a target is `archogen-assignment/3`, which adds that same line after `(program …)`; `/2` stays the hosted one. A plan's application is `(application "HEX")`, the sha256 of the file's bytes, since a figure's evidence and its basis are part of what was said. A file's own `(target …)` is held against the plan's: an application's bounds are of one target (`decision_application-inputs.md`).
- **What `check` does with a target** is `DC-H3`, as that hand-off writes it, and a target with no placement memory, or a machine the profile refuses, is refused there and at `resolve` (`DC-H4`). The profile's "one active core" reads a target's `core-count`.
- **The checker** reads the target itself (`DC-H2`): `archogen-plan-check` gains `--target` and a reader of its own, puts the system on it as that record's rule 1 says, and judges the assignment there, the digest in the record compared with the file it was given. A record whose target differs is `stale-input`, "resolve again", as a program's is. It holds what `RA-H1` adds, the release requester among it.
- **A locked build.** Each digest above is lock data (`RTP-H7`).

### 2. What the machine states

A target states more than `riscv-virt-up` says today, each with what checks it (`DC-H1`). This record adds to what `RA-H8` lists:

| Fact | Of | What it is | What checks it on `riscv-virt-up` |
| --- | --- | --- | --- |
| `counter-modulus`, `counter-width`, `observation-coherent`, `relative-delay`, `alarm-wiring`, `alarm-kind`, `compare-step`, `compare-window` or `compare-magnitude`, `supported-horizon` | the timer | `RA-H8`, `DC-H1` | the [privileged specification](../book/src/ledger.md#riscv-privileged)'s `mtime` and `mtimecmp`, cited by the agreement: 64 bits, `core`, `level-compare`, a step of 1, `compare-magnitude` |
| `controller-levels`, a count | a controller | how many priorities it distinguishes above its threshold | none on `riscv-virt-up`, which claims no controller, and so no external source |
| `controller-line`, a count | a source, or an alarm wired to a controller | its line at the controller, which is also its identifier for a priority write | none, as above |
| `non-maskable-interrupts`, a count | the platform | how many sources no mask stops | the device tree's `riscv,isa` holding no `smrnmi` |
| `firmware-traps`, a count | the platform | how many traps firmware beneath the image takes | `targets/riscv-virt-up.env`'s `-bios none` |

**Silence gives no value.** A target that does not state a fact a system needs is refused `missing-fact` by what asks, as `RA-H1` has it. `targets/riscv-virt-up.eadl` keeps saying nothing its agreement does not check, so these arrive with the agreement's rules (`M4.1.7.3`).

**Non-maskable interrupts and firmware traps** (R4-D15). The variant's condition 5 masks every interrupt in a masked section and reads each of these as a source. A source no mask stops breaks that condition, so a target stating either above zero is refused `unsupported-profile`, naming the count, and `/2`'s enabled set holds neither. Model §8 rule 8's "on a target, the platform's non-maskable interrupts and traps" is then true of `/2` by having none. A format that admits one names it, which a count cannot, so it comes with a variant that charges it.

### 3. The forms

`/2`'s text is `/1`'s, its canonical rules and its orders with it (`decision_runtime-plan.md` §3), with these lines added or changed, in this order. `DUR` is a duration, `HEX` a digest, `INT` an integer, each as there.

```text
(format "archogen-runtime-plan/2")
(profile PROFILE)
(system NAME)
(target hosted-playground) | (target NAME "HEX")
(program "HEX")
(assignment "HEX")
(adapters VERSION)
(application "HEX")                     ; where inputs are given
(record ID "HEX")                       ; each catalog record a figure here comes from, by its id
(task …)                                ; as /1
(task-inputs (task SYSTEM NAME) …)      ; each task, where inputs are given
(declared leaves-interrupt-hardware-alone (clause "…") (basis "…"))  ; where inputs are given
(initialisation-inputs (stack-bound …))  ; where inputs are given
(bind …)                                ; as /1, `(release SYSTEM)` a requester
(instance …)                            ; as /1
(epoch simulator) | (epoch first-enabling)
(timer simulator (releases …)) | (timer none) | (timer (device NODE) COUNTER (rounding FIGURE) (first-compare transition) (releases …))
(alarm ITEM (device NODE) COUNTER (kind KIND) (time-base BASE) (horizon DUR|none) (delivery-bound DUR|none))   ; each alarm
(route INT ITEM …)                      ; /1's, with (controller LINE PRIORITY) where it reaches one
(enabled ITEM …)
(interrupt-model (nesting no) (masking all))
(region NAME PROVIDER (base INT) (size INT|unsized) (use USE) (requesters REQ …))  ; a target's
(placement PROVIDER REGION)             ; a target's
(allocation UNIT (base INT) (guard INT) (bytes INT) (align INT) (origin ORIGIN …))
(allocation UNIT owed (needs NEED …))
(image (floor INT) (limit INT|owed) (truncated-by NAME …))
(node …) (context …) (depends …) (step …) (obligation …) (owed …) (complete yes|no)
```

- `COUNTER` is `(tick-rate R) (modulus M) (reach DUR) (step S) (window W)`: `R` a whole number of hertz, `M` and `S` whole numbers of ticks, `W` a count, or `magnitude` where the device states `compare-magnitude` (`RA-H8`). An alarm on a delay writes no `(window …)`. These are exactly what `decision_release-alarm.md` §2's inputs table says reaches the checker by a plan's line, `tick-rate` and the two counts unconverted, so the checker rounds each as that table says and the plan writes no converted figure. `M` of `2^64` is written whole. `reach` is a duration where a target can state one, and `(reach none)` where the search asked none of it, which the checker reads as owed.
- `FIGURE` is a catalog figure, `(value DUR) (record ID) (category CATEGORY)`, or `owed`. It is `compare-rounding` here.
- `KIND` is `level-compare` or `delay`. `BASE` is `counter`, the time base being the counter itself, or `epoch`, read off the plan's bindings as §2's table says.
- `USE` is `placement`, `executable`, `reserved` or `device`. A region `(reserved true)` is `reserved`, one executable and not reserved `placement` where it is the one chosen and otherwise `executable`, and any other `device`.
- `UNIT` is `(stack boot)`, `(stack interrupt)`, `(stack (task SYSTEM NAME))`, or `(table NAME)`, a table being what the catalog says a generated system holds in memory.
- `ORIGIN` names where a figure came from: `(application NAME)`, `(record ID NAME)` or `(realization NODE NAME)`. `NEED` is one such, for a figure not yet stated.
- `(task-inputs …)` writes `decision_application-inputs.md`'s clause for a task, canonically and sorted by each list's own key, since a file's order is the person's, each bound with its `ROADMAP.md` §7.3 record and category, so a consumer reads one artifact. `(initialisation-inputs …)` writes the initialisation's. `(owed … (by application))` is written only for what the file leaves unstated, and the format's reader refuses a file leaving anything unstated, so with inputs given none remains.
- **`RA-H7`.** A task writing an alarm whose `BASE` is `counter` must name it in its `deadlines-within-reach`, or that alarm's `room` is owed (`RA-H2`), which the plan writes `(owed (task SYSTEM NAME) deadlines-within-reach (by application))`.

**An alarm and the release timer each have one line**, as `RA-H1` says, and ranks follow `decision_release-alarm.md` §3. The requester is `(release SYSTEM)` in a `bind` line, in the record's order for a name (`eadl_resolve::plan`). `(release SYSTEM)` is no node: the search binds it and nothing it does is code the plan places, as the system's own level's is not.

### 4. Time on a machine

**The epoch** (R3-8, `RA-H8`) is the first enabling. The port's transition reads the counter ahead of it by at most its own length, and the release timer's first compare is written for that read as the last act before the transition, which `reprograms-only-in-service` counts as initialisation's, so `(first-compare transition)` states what the generated code does and `/2` writes no count for it. A hosted plan writes `(epoch simulator)`, which is `RTP-H9`'s.

**Each compare at the ceiling of its release** (R5-R4). A task's nominal release is `k·T`, an exact duration, and the code writes the first value on the compare's step at or past it, counted from the epoch's read. `/2` writes the exact durations, so the generated code does this, never the plan: `RA-H6` for an alarm, and the catalog's `timer-service` record for the release timer, whose rounding is `compare-rounding`. **The horizon over consecutive releases** is what the release requester asks, `(unambiguous-horizon (at-least SHORTEST))`, with `decision_release-alarm.md` §1's sums, so no other test is written.

**The checker's sums** are `RA-H2`'s, from `(timer …)` and `(alarm …)`, and this record adds none.

### 5. Memory

**Which regions** (R3-2, R5-R4). `Joint::map` over the target's hardware (`decision_device-correspondence.md`, whose "every block and platform its file declares" is that hardware) gives the map, and its findings refuse: `MapContradiction`, `MemoryAllHoles`, `PlacementUnsized`, each `invalid-description` where it is, naming the target's file. One `(region …)` line for each, by base, then name. Its `requesters` are those bound to its provider, for §10.3's "resource ownership".

**Which is placement memory** (R3-2, R15-R9). The search's: an assignment's `(placement PROVIDER REGION)` names both, and `eadl_resolve::plan::rebuild` keeps both, which R15-R9 found it did not. Where a target has several regions that could serve, the search takes the lowest base, then the provider's and the region's text, so a rename moves nothing else. `(placement …)` is written beside the map.

**The image lies at the bottom, every allocation at the top** (R3-9). The image starts at the placement region's base, as the port's reset entry requires of a machine that starts there, and grows up. Every allocation lies above it, in a fixed order, so nothing the image's size changes moves an allocation, and a rebuild of the code moves no plan. `(image (floor BASE) (limit LOWEST) …)` states where the image may lie: `LOWEST` the lowest allocation's base, or `owed` while one is. That the linked image stays under the limit is `M4.4`'s to check, since no plan knows its size (`RPF-H2`).

**The formula** (R3-10). A unit is `[base, base + guard + bytes)`, its guard at the low end, so that a stack growing down reaches its guard first. Tables carry `(guard 0)`. Units are laid from the top of the region down, by a cursor that only descends:

```text
cursor := the placement region's end
for each unit, in order:
    top  := cursor rounded down to the unit's alignment
    base := top - bytes - guard
    if [base, top) meets a hole: cursor := that hole's base, and lay this unit again
    if base < the placement region's base: refuse, "no room"
    cursor := base
```

- **The order:** the boot stack, the interrupt stack, each task's stack by runtime index, then each table by name, so a task added at the bottom moves only what lies below it.
- **An owed unit makes every later unit owed**, since no base below it is known, and `(allocation UNIT owed (needs …))` says what each lacks. Nothing is written as a guess.
- **A hole with no size** reaches the region's end (model §8 rule 10, `M4.1.2.5`), so it takes the region's top, and the cursor starts at its base. `M4.4` names one that truncates an image, with a request for its size (`RPF-H2`). `(truncated-by …)` lists them.
- **No room** is `infeasible-configuration`, naming the unit, its size and what the region holds, with the repair to give the system a larger region or to lower a bound.

**The stacks** (R3-9, R3-11, R3-R9, `RTP-H5`):

| Unit | `bytes` | Whose figures |
| --- | --- | --- |
| a task's | its `stack-bound`, plus the largest node stack of any node its `node-calls` name, plus one trap frame | the application, the realization (`RPF-H3`), the catalog (`RPF-H1`) |
| the interrupt's | the largest service stack among the routes, plus the fault path's use, plus one trap frame: interrupts do not nest, a service's fault is the one more thing that can arrive, and the fault path runs last | the realization, the catalog |
| the boot's | the largest of what each initialisation step uses, plus the application's initialisation, its `initialisation` clause, and at least what the idle context uses once the port's transition has run: this stack holds all three in turn | the realization, the application, the catalog |

So a stack is a sum of three owners' figures, each with its record of a bound. `(origin …)` names each one, the weakest category among them being what the composition's §6 reads. The application's figure is the bound of its own code alone (`stack-allocation.eadl`), and the allocation here is the engine's, as that boundary asks.

**The catalog's numbers** (R3-R6). The trap frame, the guard, each alignment, each table's size, the fault path's use and the idle context's are the records' (`RPF-H1`), each read by `archogen_catalog::selection` for this profile and this target, each record named by a `(record …)` line and its category written in an `ORIGIN`. Until a record states one, it is owed `(by catalog)`, a fourth owner in §3's `owed` lines, and every allocation it enters is owed.

### 6. The controller

**Ranks onto levels** (R4-D15, R5-R4). Each route that reaches a controller gets `(controller LINE PRIORITY)`: `LINE` the source's `controller-line`, and `PRIORITY` the priority a write gives it. Rank 1 takes the highest level, and each rank after it the next, so the order the plan set is the order the controller takes them in. Levels count up from just above the threshold, which `/2` holds at its lowest, so `PRIORITY` is `controller-levels` less the number of such routes ahead of it. Where the routes outnumber the levels the system is refused `unsupported-profile`, naming the count and the levels, since two on one level would take an order `/2` does not set. A route wired to the core has no line and no level: `alarm-wiring core`, which the release timer alone may be (`decision_release-alarm.md` §1).

**What this does not do:** it writes no enable. The generated code reads `(enabled …)` and `(route …)` and writes them (`RTP-H13`).

### 7. The rest of what was kept

- **The release requester, the alarms' sums and their ranks** are `decision_release-alarm.md`'s, and `RA-H1`'s lines land here.
- **A multiplexed node's users stay as `/1` counts them**, by what reaches it (R18-R6, round 29's better decision). A job's declared calls into nodes are `node-calls`, and counting by them would remove a user who never calls, whose whole effect is to drop an overhead. A set the over-count refuses is one the analysis refuses for its charge, and a sound refusal is what is in hand, so no format moves for it: it is a change to what `Joint::check` counts, for when a set needs it, and asks no line of `/2`.
- **The target's own timer where a system binds its own** (R3-1). `decision_device-correspondence.md` answers it: a device of the description is the target device bound for the same requirement.
- **Refusals `/1` makes that `/2` does not:** placement memory, and `(by application)` facts left owed when inputs are given. **Refusals `/1` makes that stay:** a system with no task, a deadline from another event, what a job would receive of an interrupt. A plan on a target adds those above, each before anything is lowered and in this record's order: a target's own refusals, `missing-fact` from the joint rules, then no room and too few levels while lowering.

### 8. What it hands on

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RPF-H1` | `M4.10` | The catalog's records state the trap frame, the stack guard, each stack's and table's alignment, the fault path's stack use, the idle context's stack use and each table a generated system holds, each with its bound's record, read for a profile and target by `archogen_catalog::selection`. |
| `RPF-H2` | `M4.4` | The linked image is checked against `/2`'s `(image …)` line: it starts at `floor`, ends at or below `limit`, and is refused naming the size it would need where a hole with no size truncates it. |
| `RPF-H3` | `M4.2` | The generated code states each node's stack use in a job, each service's and each initialisation step's, each with its bound's record, as what `RTP-H2` lists. |

## Why

**A second format and not a growing one.** `/1` is frozen, and its goldens hold a hosted plan's every byte. A target's lines change what a plan is, so a reader must know which it reads.

**Two inputs, one format.** Application inputs and a target are independent: a hosted system has a task's code before any board does. `(target playground)` lets one format hold both, so `M4.13` is not a third.

**Every count unconverted.** The inputs table's rounding is the checker's: a modulus rounds down by `tick-rate`, a step up, and a plan's own conversion would leave one direction for two readers to disagree on.

**Allocations from the top.** A rebuild of the code then moves no allocation, so a plan stays the plan through a change of a task's code, and `M4.4` checks one number.

**Rejected: `--target NAME` finding a file in a directory.** An installed tool holds no such directory, and a name is no identity: the digest is.

**Rejected: a target's file holding the catalog's figures.** A cost is a record's, with its evidence, and the file says nothing its agreement does not check (`decision_target-platform-description.md`).

**Rejected: writing a compare's tick values in the plan.** The sequence has no end, and each value depends on the epoch's read at run time.

## How to apply

- `M4.1.7.2` is this record's review, a fresh reader each round. It closes on a round that finds no defect, and then `M4.1.7.3` to `M4.1.7.7` build it: the vocabulary, each target's statements and the agreement's rules; `--target` and `check`; the search's release requester, with the joint rules and the checker; the lowering's lines and their goldens; and `build`'s and the checker's readers. Each is filed when this closes.
- Consumers read `/2` as they read `/1`: by a reader held to its goldens, never by matching text.
- Related: [[decision_runtime-plan]], [[decision_release-alarm]], [[decision_device-correspondence]], [[decision_application-inputs]], [[decision_target-platform-description]].

## Review

Not yet reviewed: leaf `M4.1.7.2`, whose rounds will be in `docs/reviews/decision_runtime-plan-format-2-reviews.md`.
