# Format `/2`: a runtime plan for a system on a target, and for one with its application's inputs

- **Type:** `decision`
- **Date:** `2026-11-07`
- **Status:** `active` — written by leaf `M4.1.7.1`; nothing here is code yet but the executable model that holds its formulas, `crates/archogen-plan/tests/format2_model.rs`, and it is under review in leaf `M4.1.7.2`, in `docs/reviews/decision_runtime-plan-format-2-reviews.md`
- **External sources:** none: this record decides how the engine lowers its own records
- **Owner / source:** leaf `M4.1.7` (`docs/tasks/M4.md`), from `decision_runtime-plan.md` §4 `.7` and `docs/reviews/decision_runtime-plan-reviews.md`'s "Kept for `M4.1.7`"; the hand-offs `RA-H1`, `RA-H8` (`decision_release-alarm.md`), `DC-H1` to `DC-H4` (`decision_device-correspondence.md`), `RTP-H5` and `AI-H2` (`decision_application-inputs.md`); `ROADMAP.md` §6.2, §7.3 and §10.3.

## The fact / decision

**In plain words.** A runtime plan says what a checked assignment turns into. The first format, `/1`, describes a system on the hosted playground, where the simulator provides the clock, the memory and the devices, and it has no line for a figure about a task's own code, so it is never complete. Format `/2` is the same plan with what a real machine adds: which machine it is, which device releases tasks and which each alarm takes, how each count on those devices becomes a time, where each stack lies in memory, and what the interrupt controller is told. It also carries what the application's inputs and the generated code's statements say, so that what `/1` could only list as owed can be stated.

The rule that keeps it honest is `/1`'s: nothing is invented. A size only a catalog record can state is owed until a record states it, and every allocation after it is owed too. A plan with an owed figure says `(complete no)`.

`/1` stays as it is, frozen, and a command with none of the new inputs still writes it.

### 1. What is given, and on what command

Three files can be named, on `resolve`, `plan-check` and `build`, each by its path and identified by what it holds, never found by a name under a directory, since an installed tool carries none. Naming any asks for `/2`, and `check` takes `--target`. `--realization none` names no file and asks for `/2` all the same, every generated figure owed: it is how the first of the two plans below is asked for where no other input is given.

| Option | The file | Read by |
| --- | --- | --- |
| `--target FILE` | a target, its hardware alone | `Target::read` (`decision_device-correspondence.md`) |
| `--application FILE` | `archogen-application/1` | `archogen_plan::application` (`decision_application-inputs.md`) |
| `--realization FILE`, or `none` | `archogen-realization/1`, what the generated code states of itself, each bound with its record | its reader, `RPF-H3`'s |

The catalog is a fourth input and no option: the tool's own, by the selection §5 names.

**The hosted playground** is `/2`'s name for no target, `(target hosted-playground)`, as an application's file writes it, reached by `--application` or `--realization` with no `--target`. `M4.13` builds it first. What it holds is `/1`'s, with these and no others: `(target hosted-playground)`, `(catalog "HEX" none)`, `(emitter VERSION)`, its code being generated as a target's is, `(epoch simulator)`, `(timer simulator …)`, each alarm's line as §3 writes one `SIMULATED`, the application's and the realization's lines, `(region …)` for each region the description's hardware states, its `use` as §3 gives one, none being `placement`, and none of §5's placement, allocations or image, nor an owed line for a figure only those read, a stack's or a table's, a system with placement memory staying refused as `/1` refuses it. One thing of `/1`'s it changes: its routes are `/2`'s, an alarm's `ITEM` `(alarm NODE)` and the ranks `decision_release-alarm.md` §3's, where `/1` wrote a `timer-route` ranked by its own rule, so the same system's routes can stand in another order in the two formats. The table says the same once, for the model to read: every line the identity keeps is one a hosted plan writes, as `/1` does or by this list, but for what it has none of (`every_line_the_identity_keeps_is_one_a_hosted_plan_writes`; RP5-3 found `(emitter …)` kept and in neither).

<!-- machine-read: hosted-lines -->
| A hosted plan's line that `/1` has not, or writes otherwise | What it writes |
| --- | --- |
| `target` | `(target hosted-playground)` |
| `catalog` | `(catalog "HEX" none)` |
| `emitter` | `(emitter VERSION)`, as on a target |
| `epoch` | `(epoch simulator)` |
| `timer` | `(timer simulator …)` |
| `alarm` | each alarm's line, `SIMULATED` |
| `route` | `/2`'s, an alarm's `ITEM` and the release alarm's ranks, and no `(controller …)` |
| `enabled`, `context`, `obligation`, `owed` | as `/1` writes each, but a route's item in it as `/2` writes one, an alarm's `(alarm NODE)` |
| `application`, `code`, `task-inputs`, `declared`, `initialisation-inputs` | the application's lines |
| `realization`, `realization-figure`, `realization-fact` | the realization's lines |
| `region` | each region the description's hardware states |

A route's item is `/2`'s wherever it stands (§3), so a hosted plan's `enabled`, `context`, `obligation` and `owed` lines hold an alarm's `(alarm NODE)` where `/1`'s hold its `timer-route`: the model holds every kept line that holds an item to be in the table (`every_kept_line_that_holds_an_item_is_one_a_hosted_plan_writes_otherwise`; RP6-1 found three that a row comparing heads alone took for `/1`'s). And a hosted route writes no `(controller …)`: no catalog record states the simulator's controller, which takes the routes in the plan's order (`RA-H3`), so there is nothing to owe and no line ever stated.

**How a build gets figures of its own code.** `/1`'s `RTP-H3` lowers again from the same inputs, and the figures of generated code come from code generated from the plan, so the realization's file cannot be an input of the first plan. It need not be. A first plan with the figures owed is complete in all that shapes code, and generates. It is a `/2` plan, asked for by `--realization none` where nothing else asks: a `/1` plan has no code identity for a file to name, and its routes can stand in another order. What that code states, measured or derived, goes in the file, and a second plan is lowered from the same inputs and that file. Its code is the first's, since no figure moves code, nor where one came from: **the code identity**. The realization's file names the identity its figures are of, and a lowering given one of another code refuses it (`RPF-H3`).

**The code identity** is the sha256 of the plan's canonical text, each line ending in a line feed, with each line kept, left out, or written with its entry point alone, by its head, as this table says and nothing else does. `crates/archogen-plan/tests/format2_model.rs` reads the table, holds it equal to its own, and holds that every line §3's forms write has a row, so a line added to the format cannot go unclassified.

<!-- machine-read: code-identity -->
| Line | The identity |
| --- | --- |
| `format` | kept |
| `profile` | kept |
| `system` | kept |
| `target` | kept |
| `program` | kept |
| `assignment` | kept |
| `adapters` | kept |
| `emitter` | kept |
| `catalog` | kept |
| `application` | left out |
| `realization` | left out |
| `code` | kept |
| `record` | left out |
| `task` | kept |
| `task-inputs` | its entry point |
| `declared` | left out |
| `initialisation-inputs` | its entry point |
| `realization-figure` | left out |
| `realization-fact` | left out |
| `bind` | kept |
| `instance` | kept |
| `epoch` | kept |
| `timer` | kept |
| `alarm` | kept |
| `route` | kept |
| `enabled` | kept |
| `interrupt-model` | kept |
| `region` | kept |
| `placement` | kept |
| `allocation` | left out |
| `image` | left out |
| `node` | kept |
| `context` | kept |
| `depends` | kept |
| `step` | kept |
| `obligation` | kept |
| `owed` | left out |
| `complete` | left out |

A line left out holds a figure, where a figure came from, or what is said of the code and is no code. `(task-inputs …)` is written `(task-inputs (task SYSTEM NAME) (entry-point "…"))` and `(initialisation-inputs …)` `(initialisation-inputs (entry-point "…"))` or `(initialisation-inputs none)`: an entry point is code, and each bound and each list of calls is said of it. `(catalog …)` and `(code …)` are kept, the identities of the records' code and the application's, a route's `(controller …)`, which the port writes, and `(emitter VERSION)`, which says what generated the code: one plan under two emitters is two codes, so a file of the one is refused by a lowering of the other. `VERSION` is `archogen-emit`'s own, `archogen-emit/N`, which it moves whenever what it writes of one plan moves and its goldens hold (`RPF-H3`); a rebuild that moves no byte it writes moves no plan, as `decision_runtime-plan.md` §2.1 asks. One that moves its version moves each plan's text and digest with its line, as it moves the code, and nothing but those goldens holds that the version moves when the output does: a stated limit (RP5-5). One figure stands on a kept line, the timer's `(rounding FIGURE)`, the catalog's: both plans read one catalog, so it is the same in both, and a first plan writes `(rounding owed)` only where no record states it.
That is what an image holds of its plan, in place of `/1`'s digest of the whole text. **The image identity** is what a build computes of the image it makes: the sha256 of that code identity and of what builds it, the toolchain, its flags and features, the target it builds for, and the linker's input but for the allocations, its preimage the build's to write down (`AI-H3`). It is what a bound's `binary` names an image by, so a change of code, as far as `code`'s digest holds the application's (`decision_application-inputs.md`, "What the digest is not of"), of toolchain, linker, feature or target makes a bound stale, as `ROADMAP.md` §9 asks. So a bound can name the image the plan builds, which it could not if it named the image's own digest, and re-measuring moves the plan and not the code (`AI-H2`). An allocation is written in the linker's input, so two images of one image identity can differ in where each allocation lies, a stack or a table, since a bound measured again moves every allocation laid after its own, and a bound holds of either, a code's cost being taken as independent of where its allocations lie. ⚠️ That is an assumption, stated here and in what the checker names.

- **The target in the identity** (R3-4, R5-R4). A plan on a target writes `(target NAME "HEX")`, `NAME` the file's one platform and `HEX` the sha256 of its forms' program text as `eadl_resolve::plan::program_text` writes a program's, so a comment moves it not. An assignment on a target is `archogen-assignment/3`, which adds that same line after `(program …)`; `/2` stays the hosted one.
- **The application and the realization in the identity.** `(application "HEX")` and `(realization "HEX")` are the sha256 of each file's canonical text, `eadl_front`'s `Document::to_canonical` of the file as read, each of its forms on a line in the file's order: so a comment or spacing moves neither, and a figure's evidence and its basis, which are forms, do, as does the order of a file's clauses, which is the person's. A file is held against the plan by `Application::unmet`, each thing unmet refused `invalid-description`, a file's own `(target …)` against the plan's among them.
- **The catalog in the identity.** `(catalog "HEX" NAME)`. `HEX` is the sha256 of one line for each record the tool's catalog holds, its id, a space and its hash as `archogen_catalog::hash`'s `Catalog::hashes` gives a record's, the lines by id and each ending in a line feed, that module's line grammar: so it is of the whole catalog, and moves with any record. `NAME` is the name its records go by for this target, the platform's `catalog-target` (§2), `none` where a target states none or the plan is hosted, so that no record's figure is read and every figure is owed, a selection with no target reading facts alone, and of records that say `(targets any)` (`archogen_catalog::selection`). A name is no identity and this one claims none: the target's digest and the catalog's are the identities, and `NAME` says only which of that catalog's records were read, which the agreement holds to its `.env`'s stem.
- **What moves the code identity with no code moved.** It errs toward moving. Any record of the catalog moves `(catalog …)`, one for another target among them, the digest being of the whole catalog. A target the tool ships has its file hashed as its bytes into each record naming it (`crates/archogen-catalog/src/hash.rs`), so a comment there moves the catalog's digest, though not `(target …)`'s. And `PLACE` in `(code …)` moves with a directory's name. Each makes a file's figures stale that could have stood, and none lets a stale one stand.
- **What `check` does with a target** is `DC-H3`, as that hand-off writes it, and a target with no placement memory is refused `infeasible-configuration`, or a machine the profile refuses as the profile says, there and at `resolve` (`DC-H4`). The profile's "one active core" reads a target's `core-count`.
- **The checker** reads the target itself (`DC-H2`): `archogen-plan-check` gains `--target` and a reader of its own, puts the system on it as that record's rule 1 says, and judges the assignment there, the digest in the record compared with the file it was given. A record whose target differs is `stale-input`, "resolve again", as a program's is. It holds what `RA-H1` adds, the release requester among it.
- **A locked build.** Each digest above is lock data (`RTP-H7`).

### 2. What the machine states

A target states more than `riscv-virt-up` says today, each with what checks it (`DC-H1`). This record adds to what `RA-H8` lists:

| Fact | Of | What it is | What checks it on `riscv-virt-up` |
| --- | --- | --- | --- |
| `counter-modulus`, `counter-width`, `observation-coherent`, `relative-delay`, `alarm-wiring`, `alarm-kind`, `compare-step`, `compare-window` or `compare-magnitude`, `supported-horizon`, `unambiguous-horizon` | the timer | `RA-H8`, `DC-H1` | the [privileged specification](../book/src/ledger.md#riscv-privileged)'s `mtime` and `mtimecmp`, cited by the agreement: `counter-modulus` `(pow2 64)`, `core`, `level-compare`, a step of 1, `compare-magnitude` |
| `available-in-state`, its states | the timer | that it counts in a state, `idle` among them, which the release requester asks (`decision_release-alarm.md` §1): the idle context waits for the next interrupt, and a timer that stops there loses every release after it | the [privileged specification](../book/src/ledger.md#riscv-privileged)'s "`mtime` must increment at constant frequency", cited by the agreement: `run` and `idle` |
| `catalog-target`, a name | the platform | the name a catalog record's `(target …)` goes by for this machine, the selection's | that the `.env`'s stem is that name |
| `non-maskable-interrupts`, a count | the platform | how many sources no mask stops | the device tree's `riscv,isa` holding no `smrnmi`, which says so of the emulated hart alone |
| `firmware-traps`, a count | the platform | how many traps firmware beneath the image takes | `targets/riscv-virt-up.env`'s `-bios none` |

**Silence gives no value.** A target that does not state a fact a system needs is refused `missing-fact` by what asks, as `RA-H1` has it. `targets/riscv-virt-up.eadl` keeps saying nothing its agreement does not check, so these arrive with the agreement's rules (`M4.1.7.3`).

**How a compare is read.** A device states `compare-window` or `(compare-magnitude true)`. `(compare-magnitude false)` says nothing, as an absent fact does, so it needs the other. Both, or a magnitude beside a window, is `invalid-description`, naming the device (`decision_release-alarm.md`, RA16-4).

**A controller's levels and each source's line stay the catalog's** (model §8 rule 8, "its line and context, and the latency are the catalog's"), read by §5's selection (`RPF-H1`), so a target's file says neither.

**Non-maskable interrupts and firmware traps** (R4-D15). The variant's condition 5 masks every interrupt in a masked section and reads each of these as a source. A source no mask stops breaks that condition, so a target stating either above zero is refused `unsupported-profile`, naming the count, and `/2`'s enabled set holds neither. The profile's admission asks both of every target, at `check` and at `resolve`: one silent on either is refused `missing-fact`, naming it, since silence gives no value and a machine with such a source would otherwise be admitted. Model §8 rule 8's "on a target, the platform's non-maskable interrupts and traps" is then true of `/2` by having none. A format that admits one names it, which a count cannot, so it comes with a variant that charges it.

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
(emitter VERSION)                       ; what generates the code, `archogen-emit/N`
(catalog "HEX" NAME)
(application "HEX")                     ; where inputs are given
(realization "HEX")                     ; where that file is given
(code NAME "PLACE" "DIGEST")            ; where inputs are given
(record ID "HEX")                       ; each catalog record a figure here comes from
(task …)                                ; as /1
(task-inputs (task SYSTEM NAME) …)      ; each task, where inputs are given
(declared leaves-interrupt-hardware-alone yes|no (clause "…") (basis "…"))  ; where inputs are given
(initialisation-inputs none) | (initialisation-inputs (entry-point "…") (stack-bound …) (node-calls …))  ; where inputs are given
(realization-figure SUBJECT FACT BOUND) ; each bound the realization's file states
(realization-fact SUBJECT FACT VALUE)   ; each fact it states that is no bound
(bind …)                                ; as /1, `(release SYSTEM)` a requester
(instance …)                            ; as /1
(epoch simulator) | (epoch after-read) | (epoch none)
(timer simulator (releases …)) | (timer none) | (timer (device NODE) COUNTER (horizon DUR) (rounding FIGURE) (first-compare transition) (releases …))
(alarm NODE (device PROVIDER) COUNTER|SIMULATED (kind KIND) (time-base BASE) (horizon DUR|none) (delivery-bound DUR|none) (asked-reach DUR|none))   ; each alarm
(route INT ITEM …)                      ; /1's, with (controller LINE PRIORITY) or (controller owed) last, where it reaches one
(enabled ITEM …)
(interrupt-model (nesting no) (masking all))
(region NAME PROVIDER (base INT) (size INT|unsized) (use USE) (requesters REQ …))
(placement PROVIDER REGION)             ; a target's
(allocation UNIT (base INT) (guard INT) (bytes INT) (align INT) (origin ORIGIN …))
(allocation UNIT owed (needs NEED …))
(image (floor INT) (limit INT|owed) (truncated-by (PROVIDER NAME) …))   ; a target's
(node …) (context …) (depends …) (step …) (obligation …) (owed …) (complete yes|no)
```

**An alarm's identity is its node.** Model §8 rule 8 reports one route for each provider and kind, and a device of two channels gives one route two alarms (`decision_release-alarm.md` §2, "one line per alarm"). So `NODE` is the text of what takes the channel, the requester bound to it, a name or an instance's `(adapter …)` text, as the application's `deadlines-within-reach` names it, and two alarms on one device are two lines. In `/2` a route's `ITEM` for an alarm is `(alarm NODE)`, one route for each, and a source's and the release timer's are as in `/1`. Ranks follow that record's §3, ties by `NODE`'s text.

**Values:**
- `COUNTER` is `(tick-rate R) (modulus M) (reach DUR) (step S) (window W)`: `R` the rate in hertz, exact, a whole number or `N/D` in lowest terms where the device states one that is none, `(tick-rate 3/2)` of `1.5 Hz`, as `decision_runtime-plan.md` §3 writes a duration; `M`, `S` and `W` counts of ticks, and `W` `magnitude` where the device states `compare-magnitude`. A count is written as the language holds one, unconverted: an `INT` where an `INT` holds it, to `2^64 − 1` (`crates/archogen-plan/src/contents.rs`), and `(pow2 P)` past that, which every count there is, the language reading none past its integers but a power of two, to `(pow2 126)` (`crates/eadl-resolve/src/value.rs`). So `riscv-virt-up`'s is `(modulus (pow2 64))`, as its file states `(counter-modulus (pow2 64))`. An alarm on a delay writes no `(window …)`. These are exactly what `decision_release-alarm.md` §2's inputs table says reaches the checker by a plan's line, `tick-rate` and the counts unconverted, so the checker rounds each as that table says and the plan writes no converted figure. `reach` is the device's `supported-horizon`, a duration: a device stating none has its system refused `missing-fact` before any plan (`decision_release-alarm.md` §1), so the line always holds one.
- `SIMULATED` is `(simulated (tick-rate R|unstated) (modulus M|unstated))`, the hosted playground's in place of `COUNTER`: its device's `tick-rate` and `counter-modulus` as the description states them, which the simulator's clock reads from this line (`M4.3.6`) and the checker's line keeps. A description stating no value of one, which `/1` lowers, writes `unstated` there with `(owed (alarm NODE) tick-rate (by description))` or `counter-modulus`, and the checker reads the line as owed one (`crates/rt-analysis/tests/alarm_model.rs`). Its `ρ`, `λ` and window are nothing and its reach its counter's horizon, one below the modulus (`RA-H3`), so it writes none of them. A counter a hosted time base reads and no alarm takes has no such line: what the simulator's clock reads of it is `(timer simulator …)`'s where it is the release timer's device, and otherwise `M4.3.6`'s to carry, which its leaf says (RP6-6).
- `(horizon DUR)` on the timer is the device's `unambiguous-horizon`, which `RA-H2` holds the release timer's sum within, and on an alarm a wake's provider's; `none` on an expiry, whose row says so. `(asked-reach DUR)` is what an expiry's requirement asks by `supported-horizon`, which the checker holds `R` against where the time base is the counter itself, and `none` on a wake. `(delivery-bound DUR)` is what an alarm's requirement asks by `delivery-bound`, whatever more a device offers, and `none` on a wake, which has none. An expiry asking none writes `none` with `(owed (alarm NODE) delivery-bound (by description))`: its set stays inconclusive, as `decision_release-alarm.md` §2's row has it, and that line is what says why (RP5-5).
- `FIGURE` is a catalog figure, `(value DUR) (record ID) (category CATEGORY)`, or `owed`. It is `compare-rounding` here.
- `KIND` is `level-compare` or `delay`, its device's `alarm-kind` on a target, and `level-compare` on a `SIMULATED` line, the simulator's (`RA-H3`). `BASE` is `counter`, the time base being the counter itself, or `epoch`, read off the plan's bindings as `decision_release-alarm.md` §2's table says, by model §8 rule 14's reach. A wake's is `counter`, its instance reading the device beneath it. Any other alarm's is read of its readers of time: the requester bound for its deadline and each requester that one reaches through `needs` and `uses`, where bound for a fact of the function `counter` that is not consumed and is none of those `RA-H8` has rule 14 read as no reader of a time base, `alarm-wiring`, `alarm-kind`, `compare-step`, `compare-window` and `compare-magnitude`, read off that record and held to it (`the_facts_that_are_no_reader_s_are_the_release_alarm_s_own_five`; RP6-2 found `alarm-kind` left out). It is `epoch` where there is such a reader and each is bound at an `epoch-extender` over the alarm's own device, and `counter` otherwise: where any is bound at the device itself, on another device or at an extender over another, or there is none (RP5-1).
  An alarm compares on its own device's counter, so a value written to its compare needs no `R`, no `room` and no `RA-H7` statement only where it was read as that counter's own extended time; one read of another clock, extended or not, says nothing of this counter's wraps. The two facts a compare is programmed by, `relative-delay` and `absolute-deadline`, are consumed, so being bound for a deadline makes no reader. The model holds every set of up to three readers, and the vocabulary's own word on what is consumed (`an_alarm_s_base_is_the_counter_unless_each_reader_is_extended_over_its_own_device`, `what_programs_a_compare_is_consumed_and_what_describes_one_is_named`).
- `USE` is `placement`, `executable`, `reserved` or `device`. A region `(reserved true)` is `reserved`, one executable and not reserved `placement` where it is the one chosen and otherwise `executable`, and any other `device`.
- `UNIT` is `(stack boot)`, `(stack interrupt)`, `(stack (task SYSTEM NAME))`, or `(table NAME)`, a table being what the catalog says a generated system holds in memory.
- `ORIGIN` names where a figure came from and what it rests on: `(application OF FACT CATEGORY)`, `(record ID FACT CATEGORY)` or `(realization SUBJECT FACT CATEGORY)`, `CATEGORY` one of `ROADMAP.md` §7.3's four as the application's format writes them. `NEED` is `(application OF FACT)`, `(record FACT)` or `(realization SUBJECT FACT)`, for a figure not yet stated: no record being named where none states it. `OF` is whom the application's file says it of, as an owed line's subject is written: a task's `(task SYSTEM NAME)`, or the system's name for its initialisation, whose `FACT` is `stack-bound`, or `initialisation` in a `NEED` where no file is given.
- `SUBJECT`, of what the generated code states, is what that code is of: a `NODE` for its `node-stack`, `(route ITEM)` for a route's `service-stack`, `(step INT)` for an initialisation step's `initialisation-stack`, the step's index, `fault` for the generated handler's `fault-entry-stack`, and a stack's `UNIT` for its `call-frame` and, the boot's, its `last-act-stack` (§5); and `epoch` for the last act's `last-act-lead`, a time (§4). Of a fact `decision_runtime-plan.md` §2.10 owes the realization, it is that owed line's subject as `/1` writes it. Of what an alarm's realization states, §2's five and what it owes as a source, it is `(alarm NODE)`.
- `BOUND` is a bound with its `ROADMAP.md` §7.3 record, written as the file states it, in `decision_application-inputs.md`'s forms and nothing converted: `(value …) (unit …) (scope "…") (binary …) (evidence …)`, a `(locator "…")` where the file states one, as all but an assumption must and an assumption may, and `(basis "…")`. What an `(evidence …)` observed is in the bound's own unit, so the value stays in it too, and no reader converts one against the other.
- `(task-inputs …)` writes `decision_application-inputs.md`'s clause for a task in that record's order: `entry-point`, `execution-bound`, `os-calls`, `node-calls`, `masked-runs`, `stack-bound`, then the yes-or-no facts, then `deadlines-within-reach`. Each list is sorted by its callee's text, runs by name and then ending, and each bound is a `BOUND`, as the file has it: `1 KiB` and `1024 byte` are two texts, which the application's digest tells apart already. `(initialisation-inputs …)` writes that record's `initialisation` clause the same way: `none`, or its `entry-point`, its `stack-bound` and its `node-calls`.
- `(realization-figure …)` writes one bound the file states, `FACT` as `decision_runtime-plan.md` §2.10 names it or one this record adds, `node-stack`, `service-stack`, `initialisation-stack`, `fault-entry-stack`, `call-frame` and `last-act-stack`, each of its `SUBJECT`, `last-act-lead` of `epoch`, and of an alarm, its `SUBJECT` `(alarm NODE)`, §2's `floor`, `step`, `reach-ahead`, `lag` and `service-cost`. `(realization-fact …)` writes one fact the file states that is no bound, of those §2.10 owes the realization, `contexts`, `no-wait`, `runtime-discipline`, `acknowledge-point`, `no-application-code` and `one-request-per-arrival` among them: `VALUE` is the file's own forms for it in canonical text, which `archogen-realization/1` defines (`RPF-H3`). An owed line of either kind goes when the file states it, with this line in its place.
- **`RA-H7`.** A task writes an alarm where it reaches the alarm's node, as `decision_runtime-plan.md` §2 reaches one, directly or through services, policies and bindings, whether or not its `node-calls` name it: a call through a service is that node's in turn, which no file names. A wake is written by no task: its own service writes each of its points, so no task signs for it, as the checker's model asks none of one (`crates/rt-analysis/tests/alarm_model.rs`, `on_the_counter`). A task writing any other alarm whose `BASE` is `counter` must name it in its `deadlines-within-reach`, or that alarm's `room` is owed (`RA-H2`), which the plan writes `(owed (task SYSTEM NAME) deadlines-within-reach (by application))`, the one `(by application)` line a read file can leave.

**`(owed …)`, as `/2` writes it.** An owed line's subject gains `(alarm NODE)`, a `UNIT`, `timer`, `epoch`, and each `SUBJECT` a generated figure is of. `OWNER` gains `catalog`. `FACT` gains, of an alarm by `realization`, `floor`, `step`, `reach-ahead`, `lag` and `service-cost`, `decision_release-alarm.md` §2's `T_s`, `P`, `R`, `λ` and `C_s`, with what a source owes as a source, `acknowledge-point`, `deferred-work`, `no-application-code` and `one-request-per-arrival`, each alarm being a source of its own there; of a simulated alarm by `description`, `tick-rate` and `counter-modulus`, where its line says `unstated`; of an expiry by `description`, `delivery-bound`, where its requirement asks none; of a `UNIT` by `catalog`, `trap-frame`, `guard`, `alignment`, `fault-path-stack`, `idle-stack`, `trap-path-stack`, `timer-service-stack`, `primitive-stack`, `completion-stack`, `initialisation-path-stack`, `transition-stack` and `table-size`; of a node, a `(route ITEM)`, a `(step INT)`, `fault` and a stack's `UNIT` by `realization`, its `node-stack`, `service-stack`, `initialisation-stack`, `fault-entry-stack`, and `call-frame` or `last-act-stack`; and of `epoch` by `realization`, `last-act-lead`, on a target with a wake or a compare that cannot be disarmed.
An alarm's line stands in place of `/1`'s timer route, so `/1`'s `arrival-bound` of one is owed of none: an alarm's `floor` is what holds its arrivals apart. Of a route's `ITEM` by `catalog` it gains `controller-line`, where its line says `(controller owed)` (§6). Of `timer` by `catalog` it gains `compare-rounding`, where its line says `(rounding owed)`; of `epoch` by `catalog`, `epoch-lead` (§4); and of the system by `application`, `initialisation`, where no application's file is given, beside what `/1` owes of it, which `Application::unmet` holds of the system from the leaf that lowers `/2`. Only what no input states is written, and a hosted plan, which lays out no memory, owes no stack's or table's figure.

**Orders.** `record` lines by id; `task-inputs` by runtime index; `realization-figure` and `realization-fact` by subject's text, then fact; `alarm` lines by `NODE`'s text; `region` by base, then name, then provider, two blocks' regions of one name being two lines, and its `requesters` in the assignment's order of requesters, below; `allocation` in §5's order, the owed ones among them; `truncated-by` by provider, then name, its holes having one base; on a line, `origin` and `needs` items by owner, `application`, `record`, `realization`, then by text.

**`(obligation …)`** gains `placement`, its subject the placement region's name, discharged by `M4.4`, for `Obligation::Placement`, whose holes the `(region …)` lines name.

`(release SYSTEM)` is no node: the search binds it and nothing it does is code the plan places, as the system's own level's is not. Among requesters it is a class of its own, the last: `archogen-assignment/3` sorts its bindings by requester, every name, then every task, then every instance, then the release requester, and the plan's `bind` lines follow, as `/1`'s follow `/2`'s. The device it is bound to is a node, as model §8 rule 12 has it, "the providers bound for the requesters the system serves", the release requester among those by `decision_release-alarm.md` §1: it has its `(node …)`, its `(step …)` in initialisation and its owed lines as any block bound does, and its `(context …)` holds `initialisation` and `(interrupt timer)`, which `CONTEXT` gains. What runs there is the catalog's timer service, and the realization's facts of that node say so.

### 4. Time on a machine

**The epoch** (R3-8, `RA-H8`). The port's transition is the first enabling, and code cannot know that instant, only what it read before it. So the last act of initialisation reads the counter at `r` and counts nominal releases from `e = r + E`, and writes the first compare at the ceiling of `e`, which `reprograms-only-in-service` counts as initialisation's. `E` is the port's `epoch-lead`, its record's bound on everything from that read to the first enabling: one tick of the counter, a read being up to that old, the first compare's write, and the transition (`RPF-H1`). The switch record's bound on the transition alone would not do: at 10 MHz a read of 1000 taken at 100.09 µs, a write of 0.30 µs and a transition of 2.00 µs enable at 102.39 µs, past `r` and that bound at 102.00 µs. So `e` is at or after the first enabling and by at most `E`, every nominal release is at or after it, as `releases-after-initialisation` asks, and none fires before its nominal time. `(epoch after-read)` says so, the epoch being after a read and never the enabling itself, and `(first-compare transition)` states what the generated code does; `/2` writes no count for either, and `(owed epoch epoch-lead (by catalog))` until a record states the lead. A hosted plan writes `(epoch simulator)`, which is `RTP-H9`'s.
A target's plan with `(timer none)` writes `(epoch none)`: no release is counted, no counter is read for one, and no lead is owed but where the plan has a wake, below. The last act of initialisation is generated code, and what it uses of the boot's stack is the realization's `last-act-stack` (§5).
**The last act's order** (RP4-1). `E` is a port's bound, and no port's record bounds generated code. So the last act does what is its own before it reads: it arms each wake and writes each placeholder first, then reads the counter, and from that read runs nothing of its own but two calls, the timer service's write of the first compare and the port's transition, each the catalog's. `E` then bounds what it says it bounds (`RPF-H3` has the generated code keep this order). The instructions of those two calls are the generated code's own and lie in that span: the port's bound is stated from the read, so it counts a call of each of its two functions among what it bounds, and nothing else of the act's stands there, a stated limit (RP5-5). Armed after the write, one wake's arm of 1.00 µs in the case above puts the enabling at 103.39 µs, past an `e` of 102.40: `what_the_last_act_arms_it_arms_before_its_read` holds both.
**A wake's first point** (RP5-2). A wake's arm reads the counter itself and writes `P` past what it read, and after it the act arms what is left, reads for the epoch, writes the first compare and calls the transition. So `P` must outlast all of that, or a wake arrives in initialisation, a masked run `L` does not hold: `decision_release-alarm.md` §2's "that step's own length must not pass", which no sum held, there or here. The realization states that length's own part, `last-act-lead`: its bound on everything from a wake's read, up to a tick of that wake's counter old, to the act's read for the epoch, a `(realization-figure epoch last-act-lead …)`, owed `(by realization)` on a target with a wake. And the scheduling checker holds each wake's `P` at no less than `last-act-lead` plus the port's `epoch-lead`, reading a set where either is owed as inconclusive (`RPF-H4`). Under `(timer none)` the act reads nothing for an epoch, `last-act-lead` runs to its call of the transition, and `epoch-lead`, which bounds that transition among more, is owed all the same where the plan has a wake. The model runs every small tick, bound and phase: where the condition holds no wake's first point is before the enabling, and it asks one tick of the timer's counter and one unit more than what can happen (`a_wake_s_first_point_stands_at_or_after_the_first_enabling_where_the_condition_holds`).
`P` rounds down in that hold and each lead up, to whatever unit the checker counts in, so that what holds of the rounded figures holds of the stated ones: rounded up, as the inputs' table rounds a `P` a sum is charged with, a wake due before the enabling could pass (RP6-4, `a_wake_s_step_rounds_down_where_a_sum_lies_within_it`). A compare that cannot be disarmed is a wake's like here: its placeholder is written before the read and must not match before the enabling, so its distance past its own read is held as a wake's `P` is, and within what one write reaches on its device, and `last-act-lead` is owed on a target with either (RP6-5). A hosted plan owes neither figure and is held to nothing here: the simulator enables at time zero, where the plan's wakes are armed (`RTP-H9`), so a wake's first point is `P` past the enabling whatever `P` is (RP6-3, `a_hosted_wake_has_nothing_to_outlast`).

**One write reaches the first compare** (RP3-3). The first compare is written at the ceiling of `e` on its step, so it stands up to `E + ρ` past its read, and no sum of `decision_release-alarm.md` holds that: `SHORTEST + 2ρ` covered it only while `E` was the transition's bound alone. So the scheduling checker holds `E + ρ` within what one write reaches on the release timer's device, as it holds `SHORTEST + 2ρ`, and reads a set whose lead is owed as inconclusive (`RPF-H4`). The model runs it over every small tick, lead and step, `the_first_compare_stands_within_one_write_where_the_condition_holds`: at a reach of 2 ms a lead of 3 ms is refused.

**Each compare at the ceiling of its release** (R5-R4). A task's nominal release is `e + k·T`, an exact duration past `e`, and the code writes the first value on the compare's step at or past it. `/2` writes the exact durations, so the generated code does this, never the plan: `RA-H6` for an alarm, and the catalog's `timer-service` record for the release timer, whose rounding is `compare-rounding`. **The horizon over consecutive releases** is what the release requester asks, `(unambiguous-horizon (at-least SHORTEST))`, with `decision_release-alarm.md` §1's sums, so no other test is written.

**The checker's sums** are `RA-H2`'s, from `(timer …)` and `(alarm …)`, and this record adds one, the first compare's above. The platform's timing figures, `S`, `E`, `W_wake`, `γ` and `δ`, are the catalog's, read by the checker through `(catalog …)`'s selection, and a plan writes none of them.

### 5. Memory

**Which regions** (R3-2, R5-R4). `Joint::map` over the target's hardware (`decision_device-correspondence.md`, whose "every block and platform its file declares" is that hardware) gives the map, and its findings refuse, each naming the target's file and by the code the joint rules give it: `MapContradiction` and `MemoryAllHoles` `invalid-description`; `PlacementUnsized`, `MemoryUnsized` and `HoleUnsized` `missing-fact`; and `NamedOutside` `infeasible-configuration`. One `(region …)` line for each, by base, then name. Its `requesters` are those bound to its provider, for §10.3's "resource ownership".

**Which is placement memory** (R3-2, R15-R9). The search's: an assignment's `(placement PROVIDER REGION)` names both, and `eadl_resolve::plan::rebuild` is to keep both, which R15-R9 found it did not and it does not yet: `archogen-assignment/3`'s leaf builds it. Where a target has several regions that could serve, the search takes the lowest base, then the provider's and the region's text, so a rename moves nothing else. `(placement …)` is written beside the map. ⚠️ The lowest is not the largest: a small executable region below the memory a system fits in is the one taken, and the system is then refused for no room, naming that region. The repair is the target's: an executable region that is no place for a system says `(reserved true)`, which makes it a hole and no placement memory (rule 10), and the refusal says so; choosing by size would let a region's size move every address.

**What the catalog is read by.** `archogen_catalog::selection`, `Selection { profile, target }`, `target` the `(catalog …)` line's name.

**The image lies at the bottom, every allocation at the top** (R3-9). The image starts at the floor and grows up, and every allocation lies above it, in a fixed order, so a change in the image's size moves no allocation, and code of one identity is laid out the same however its figures fall. `(image (floor F) (limit L) (truncated-by NAME …))` says where it may lie. `F` is the least address at or above the placement region's base that no hole covers: the base, moved past each hole that covers it, one after another, so past two that touch or overlap. `L` is the less of the lowest allocation's base and the base of the lowest hole above `F`, since a contiguous image crosses no hole, rule 10's "a hole the plan avoids"; `owed` while the lowest allocation is, the lowest hole above `F` then standing in `truncated-by` as what may yet be the limit, an allocation already laid below it or not. `truncated-by` names each hole whose base is `L`. A region whose every byte is surely reserved is `MemoryAllHoles`, a sized hole's bytes and the base of one with no size being sure, as rule 10 says.
Worked: a region at `0x8000_0000` of 128 MiB with holes `[0x8000_0000, +64 KiB)` and `[0x8001_0000, +64 KiB)` has `F` at `0x8002_0000`; with the second hole from `0x8000_8000` it is `0x8001_8000`, where that hole ends, and `0x8002_0000` again where that hole is of 96 KiB. The model holds `F`, `L` and the formula below against a reference that tries each address in turn, over every small region, and each of these cases as a row (`crates/archogen-plan/tests/format2_model.rs`). ⚠️ The image takes only what lies between `F` and the lowest hole above it, however much lies free above that hole: a hole 16 bytes above `F` leaves it 16 bytes, which `M4.4` refuses naming the hole (`RPF-H2`), an image above a hole being no plan `/2` writes. That the linked image stays in `[F, L)` is `M4.4`'s to check, since no plan knows its size (`RPF-H2`).

**The formula** (R3-10). A unit is `[base, base + guard + bytes)`, its guard at the low end, so that a stack growing down reaches its guard first. Tables carry `(guard 0)`. Units are laid from the top of the region down, by a cursor that only descends. `(align A)` is a stack's top's alignment and a table's base's:

```text
cursor := the placement region's end
for each unit, in order:
    top  := a stack's: cursor rounded down to A; a table's: cursor
    base := top - bytes - guard, a table's rounded down to A
    if [base, top) meets a hole: cursor := that hole's base, and lay this unit again
    if base < F: refuse, "no room"
    cursor := base
```

- **The order:** the boot stack, the interrupt stack, each task's stack by runtime index, then each table by name, so a task added at the bottom moves only what lies below it.
- **An owed unit makes every later unit owed**, since no base below it is known, and `(allocation UNIT owed (needs …))` says what each lacks. Nothing is written as a guess.
- **With no application's file the boot's stack is owed**, the first unit and so every unit: the file could name an initialisation, so nothing is laid as if it said `none`. Its `NEED` is `(application SYSTEM initialisation)`, and `(owed SYSTEM initialisation (by application))` says so.
- **A hole with no size** reaches the region's end (model §8 rule 10, `M4.1.2.5`), so it takes the region's top, and the cursor starts at its base; one that so leaves no byte free, where the bytes surely reserved do not, is `HoleUnsized`; and a reserved range of no size based below the region is `PlacementUnsized`, no hole of it. Each is `Joint::map`'s to say, and the model's refusals are held to that code itself over 1,124 small maps, whatever order their ranges are written in (`the_model_refuses_a_map_as_the_joint_rules_do`). Those maps hold one placement region and reserved ranges, and these sentences are of such a map: with another placement region above the chosen one, or a device's registers in the map, `Joint::map` refuses more, an unsized hole inside the chosen region as `PlacementUnsized` and registers as `MapContradiction`, on 133 of 427 and 291 of 486 of its round-5 reader's maps. Each is its to say, and the model's floor is of one region's map: a stated limit (RP5-4). `M4.4` names one that truncates an image, with a request for its size (`RPF-H2`).
- **No room** is `infeasible-configuration`, naming the unit, its size and what the region holds, with the repair to give the system a larger region or to lower a bound.

**The stacks** (R3-9, R3-11, R3-R9, `RTP-H5`). What runs on a stack runs in turn, so a stack holds the largest of them and not their sum, on top of what is under them all. A trap frame is the port's, one for whatever a stack's code is stopped by. **A fault is added, and never taken among the largest.** A failed check calls the fault path from inside a primitive's, a record's or generated code, and a panic reaches the image's handler from wherever it is raised, initialisation among it (the fault contract), so a fault arrives on top of the deepest of what a stack holds. Its use is the generated handler's `fault-entry-stack`, the realization's (`RPF-H3`), and the `fault-path-stack` it calls, the catalog's (`RPF-H1`).

<!-- machine-read: stack-sums -->
| Unit | `bytes` |
| --- | --- |
| a task's | `call-frame + stack-bound + max(node-stack, primitive-stack, completion-stack) + fault-use + trap-frame` |
| the interrupt's | `trap-path-stack + call-frame + max(service-stack, timer-service-stack) + fault-use + trap-frame` |
| the boot's | `call-frame + max(initialisation-stack, initialisation-path-stack, stack-bound + node-stack, last-act-stack + max(transition-stack, idle-stack)) + fault-use + trap-frame` |

A name that is several figures stands for the largest of them, and for nothing where there is none: each node's that a task's `node-calls` name, or the initialisation's, each primitive's its `os-calls` name, each route's, each step's. `fault-use` is the `fault-entry-stack` plus the `fault-path-stack`. A term whose figure is owed makes the allocation owed. The model reads these cells and runs them, so a sum here is the sum it holds, against what can stand on each stack at once by the table below: `each_stack_s_sum_is_the_most_that_can_stand_on_it`, over every small set of figures. Each figure a sum names is bounded there, by one owner, and owed until stated, which `each_figure_of_a_sum_is_bounded_owned_and_owed` holds.

<!-- machine-read: stack-figures -->
| Figure | Of | Owner | What it bounds |
| --- | --- | --- | --- |
| `stack-bound` | a task, or the application's initialisation | the application | its own code from its entry point, and nothing it calls through the system (`stack-allocation.eadl`) |
| `node-stack` | a node | the realization | that node's generated code from a call into it, a job's or the application's initialisation's, and all it calls in turn |
| `primitive-stack` | a primitive | the catalog | that primitive from its call, and all it calls |
| `completion-stack` | the port | the catalog | the completion path, from a job's return or its own call to the switch away |
| `call-frame` | a stack | the realization | what generated code holds on that stack beneath what it calls there: a job's start beneath its entry point, the boot's code beneath each step, the dispatch beneath each service |
| `service-stack` | a route | the realization | that route's generated service from its call, and all it calls |
| `timer-service-stack` | the port | the catalog | the timer service from its call: the due check, each release and the compare's write |
| `trap-path-stack` | the port | the catalog | the trap path but for the service it calls: its entry, the claim, and the decision and the switch after a service |
| `initialisation-stack` | a step | the realization | that step's generated code, and all it calls |
| `initialisation-path-stack` | the port | the catalog | the catalog's own initialisation, the port's and each record's, which runs before the last act and is no step |
| `last-act-stack` | the boot's stack | the realization | the last act of initialisation, §4's read, the first compare's write and each wake's first arm, up to its call of the port's transition |
| `transition-stack` | the port | the catalog | the port's transition, from its call to the first enabling |
| `idle-stack` | the port | the catalog | the idle context, which runs on the boot's stack once the transition has |
| `fault-entry-stack` | the generated handler | the realization | the generated fault handler, to its call of the fault path |
| `fault-path-stack` | the port | the catalog | the fault path, from that call |
| `trap-frame` | the port | the catalog | one trap frame, for whatever a stack's code is stopped by |

Worked, with figures chosen for the sum and stated by no record: a task of `stack-bound` 512 under a start of 48, a node's 380, a primitive's 96, a completion's 64, a fault's 400 and a frame of 272 has 48 + 512 + 380 + 400 + 272 = 1612 bytes, where taking the fault among the largest gave 1232, under the 1340 a check failing inside that node needs with no frame at all. The boot's, with an initialisation of 256 that calls a driver of 300, an idle context of 64, and the same start, fault and frame, has 48 + 556 + 400 + 272 = 1276, where the bound taken bare gave 976, under the 1004 that initialisation needs at its deepest with a check failing there.

`(origin …)` names each figure entered with its category, the weakest among them being what the composition's §6 reads. The application's figure is the bound of its own code alone (`stack-allocation.eadl`), and the allocation here is the engine's, as that boundary asks. A task with a `node-call` in a masked run has it in `node-calls` too, as its reader holds.

**The catalog's numbers** (R3-R6). The trap frame, the guard, each alignment, each table's size, the fault path's use, the trap path's, the timer service's, each primitive's, the completion's, its own initialisation's, the transition's and the idle context's are the records' (`RPF-H1`), each record named by a `(record …)` line and its category written in its `ORIGIN`, whose last item it is. A table's size is a function of a system's counts, its tasks, its sources and its alarms, which a record states as a base and a size per each. Until a record states one, it is owed `(by catalog)`, and every allocation it enters is owed.

### 6. The controller

**Ranks onto levels** (R4-D15, R5-R4). On a target each route that reaches a controller writes `(controller LINE PRIORITY)` last on its line: `LINE` the source's line at the controller and `PRIORITY` the priority a write gives it, each from the catalog's record of that controller, or `(controller owed)` until one states them, with `(owed ITEM controller-line (by catalog))`. Rank 1 takes the highest level, and each rank after it the next, so the order the plan set is the order the controller takes them in. Levels count up from just above the threshold, which `/2` holds at its lowest, so `PRIORITY` is `controller-levels` less the number of such routes ahead of it. Where the routes outnumber the levels the system is refused `unsupported-profile`, naming the count and the levels, since two on one level would take an order `/2` does not set. A device of `channels 2` has two lines, as the record says of it, and its alarms take them in order: by `NODE`'s text, the first the lowest line. A route wired to the core has no line and no level: `alarm-wiring core`, which the release timer alone may be (`decision_release-alarm.md` §1), so it writes no `(controller …)`.

**What this does not do:** it writes no enable. The generated code reads `(enabled …)` and `(route …)` and writes them (`RTP-H13`).

### 7. The rest of what was kept

- **The release requester, the alarms' sums and their ranks** are `decision_release-alarm.md`'s, and `RA-H1`'s lines land here. **The runtime plan's §2.7 is amended** as `RA-H1` says: a wake's first arm and a compare that cannot be disarmed are written in initialisation. **Whether a port can realize an alarm as a request a claim takes** is the port record's, a target's `alarm-wiring` saying what is there: `riscv-virt-up` has the release timer alone.
- **A multiplexed node's users stay as `/1` counts them**, by what reaches it (R18-R6, round 29's better decision). A job's declared calls into nodes are `node-calls`, and counting by them would remove a user who never calls, whose whole effect is to drop an overhead. So a system the over-count charges more is one the analysis judges at that charge, which is sound, and no format moves for it: it is a change to what `Joint::check` counts, for when a set needs it, and asks no line of `/2`.
- **The target's own timer where a system binds its own** (R3-1). `decision_device-correspondence.md` answers it: a device of the description is the target device bound for the same requirement.
- **Refusals `/1` makes that `/2` does not:** placement memory, on a target, and `(by application)` facts left owed when inputs are given. **Refusals `/1` makes that stay:** a system with no task, a deadline from another event, what a job would receive of an interrupt. A plan on a target adds those above, each before anything is lowered and in this record's order: a target's own refusals, `missing-fact` from the joint rules, an application's unmet, a realization of another code identity, which names no code this lowering makes, then no room and too few levels while lowering. A bound or a list of counts whose `binary` names another image is refused after all of these, by the build that computes the image's identity (`AI-H3`): no lowering knows it.
- **A source's separation on a target** (`decision_runtime-plan.md` §2.5's "a target's is `/2`'s to owe"). On a target the provider that states it is the target's device, and a target states only what its agreement checks (`DC-H1`). So a separation stated there has its evidence, the agreement's rule for it, and one no rule can read off its source is not stated: it is `unknown`, the task it would release has an `(unknown (unstated ITEM))` trigger, and `/1`'s `(owed (source NAME) separation (by description))` says what is lacking. `/2` adds no line and assumes none. ⚠️ An environment's promise that no source can check, a debounced line's, has no carrier in `/2`: a system resting on one is not analysed on a target until a format carries it with §7.3's assumption and the checker names it.

### 8. What it hands on

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RPF-H1` | `M4.10` | The catalog's records state the trap frame, the stack guard, each stack's and table's alignment, the fault path's, the trap path's, the timer service's, each primitive's, the completion path's, its own initialisation's, the transition's and the idle context's stack use, each as `/2`'s table of stack figures bounds it, each table a generated system holds as a base and a size per task, source and alarm, and a controller's levels and each source's line, and the port's `epoch-lead`, its bound from the epoch's read to the first enabling of interrupts, each with its bound's record, read for a profile and target by `archogen_catalog::selection`. |
| `RPF-H2` | `M4.4` | The linked image is checked against `/2`'s `(image …)` line: it starts at or past `floor`, ends at or below `limit`, holds the code identity of the plan, and is refused naming the size it would need where a hole truncates it, with a request for that hole's size where it states none. |
| `RPF-H3` | `M4.2` | The generated code states each node's stack use from a call, each service's, each initialisation step's, its last act's, its fault handler's and what it holds on each stack beneath what it calls, each as `/2`'s table of stack figures bounds it and with its bound's record, its last act arming and writing for each alarm before it reads the counter, which is where a wake is first armed, `RA-H6`'s "at the port's transition" giving way, and running nothing of its own from that read but the first compare's write and the transition, its `last-act-lead`, its bound on everything from a wake's read to that read, its emitter stating a version it moves whenever what it writes of one plan moves, as what `RTP-H2` lists, and each fact `/1` owes it that is no bound, in a format of its own, `archogen-realization/1`, with its reader and its version row, the file a second plan is lowered from, which names the code identity its figures are of and is refused by a lowering of another's; and an image holds the plan's code identity and not its digest. |
| `RPF-H4` | `M2.21.2` | The scheduling checker holds the port's `epoch-lead` plus the release timer's `ρ` within what one write reaches on that timer's device, as it holds `SHORTEST + 2ρ`, failing `unsupported-profile` and naming it, and reads a set whose `epoch-lead` is owed as inconclusive; a set with `(timer none)` is asked neither. And it holds each wake's `P`, its `step`, at no less than the realization's `last-act-lead` plus the port's `epoch-lead`, failing `unsupported-profile` and naming the wake, and reads a set where either is owed as inconclusive, under `(timer none)` as under a timer, rounding `P` down and each lead up; it holds each placeholder's distance past its read the same way, and within what one write reaches on its device; and a hosted set is asked none of it, the simulator enabling at time zero. |

## Why

**A second format and not a growing one.** `/1` is frozen, and its goldens hold a hosted plan's every byte. A target's lines change what a plan is, so a reader must know which it reads.

**Three inputs, one format.** Application inputs, what generated code states and a target are independent: a hosted system has a task's code before any board does. `(target hosted-playground)` lets one format hold all three, so `M4.13` is not a fourth.

**Every count unconverted.** The inputs table's rounding is the checker's: a modulus rounds down by `tick-rate`, a step up, and a plan's own conversion would leave one direction for two readers to disagree on.

**Allocations from the top.** A change in the image's size then moves no allocation, and `M4.4` checks a range.

**Two plans, one code.** The realization's figures come of generated code, so asking them of the first plan is a cycle, and a fixed table of them in the emitter would be a guess that nothing measures. The code identity is what lets the second plan say so without moving the first's code.

**Rejected: `--target NAME` finding a file in a directory.** An installed tool holds no such directory, and a name is no identity: the digest is.

**Rejected: a target's file holding the catalog's figures.** A cost is a record's, with its evidence, and the file says nothing its agreement does not check (`decision_target-platform-description.md`).

**Rejected: writing a compare's tick values in the plan.** The sequence has no end, and each value depends on the epoch's read at run time.

**Rejected: a fact of its own for a modulus past an integer, `counter-modulus-power`**, rounds 1 and 2's answer. The language states `(counter-modulus (pow2 64))`, exactly (`decision_substitutability-relation.md` §2), and what derives a horizon reads `counter-modulus` alone, so a second fact refused the target it was for.

**A model beside the record.** Three rounds worked this record's formulas by hand, and each found one wrong, its own worked example among them. So each procedure it states is transcribed in `crates/archogen-plan/tests/format2_model.rs` and held over every small input, and a probe of a round is a row there (`decision_executable-design-reviews.md`).

## How to apply

- `M4.1.7.2` is this record's review, a fresh reader each round. It closes on a round that finds no defect, and then `M4.1.7.3` to `M4.1.7.7` build it: the vocabulary, each target's statements and the agreement's rules; `--target` and `check`; the search's release requester, with the joint rules and the checker; the lowering's lines and their goldens; and `build`'s and the checker's readers. Each is filed when this closes.
- This record amends `decision_release-alarm.md`: its `RA-H8`'s epoch reads as §4 says, a wake is first armed in the last act before its read, as §4 orders it, where that record's §2, §5 and `RA-H6` say "at the port's transition", and the length its `P` must not pass, which that record gave no holder, is `last-act-lead` plus `epoch-lead`, held by `RPF-H4`, the first compare's reach is held by `RPF-H4` where its §1 held it by the transition's bound, and what an expiry's requirement asks by `supported-horizon`, which that record's `R` is held against, reaches the checker by an alarm's `(asked-reach …)`.
- Consumers read `/2` as they read `/1`: by a reader held to its goldens, never by matching text.
- A rule changed here is changed in `crates/archogen-plan/tests/format2_model.rs` first, failing, and `M4.1.7`'s lowering is held against that model over the same inputs.
- Related: [[decision_runtime-plan]], [[decision_release-alarm]], [[decision_device-correspondence]], [[decision_application-inputs]], [[decision_target-platform-description]].

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 17 | 15 (RP1-1 to RP1-15), beside 1 gap and 1 of wording | `2026-11-07`, not acceptable — answered: a stack's sum holds what runs on it, a realization's figures come by a third file and a second plan of one code identity, an alarm has an identity of its own, a modulus past `eadl/1`'s integers can be stated, the epoch is at or after the first enabling, a controller's lines stay the catalog's, and each order and owed line is written. The cursor, the priorities, the alarm's line and each kept question held. |
| 2 | 13 | 10 (RP2-1 to RP2-10), beside 2 gaps and 1 of wording | `2026-11-10`, not acceptable, 10 after 15 — answered: the image's floor is past every hole that covers it; a fault's use is added to each stack and never taken among the largest; a simulated line keeps its rate and modulus; a timer states that it counts in idle; the code identity leaves out where a figure came from, so two plans have one; a hosted plan's routes are `/2`'s; the epoch is a read and the port's whole lead after it; an origin holds its category; a modulus past an `INT` is written as its power; and each line a builder could not write has its words. The cursor held over 20,000 regions, and the priorities, the timer's line and every other kept question. |
| 3 | 11 | 9 (RP3-1 to RP3-9), beside 1 gap and 1 of wording | `2026-11-10`, not acceptable, 9 after 10 and 15, so answered by method: each procedure this record states is a model that runs, `crates/archogen-plan/tests/format2_model.rs`, held over every small input, its probes rows, and the stacks' sums and the identity's lines are tables it reads. The boot's stack holds what the application's initialisation calls, the last act and the transition; a modulus is the language's own `(pow2 64)`; the first compare's reach is held; a fact that is no bound has its line; and the first plan is asked for as `/2`. The cursor held over 38,865 regions, and the sums, the priorities and each kept question. |
| 4 | 8 | 7 (RP4-1 to RP4-7), beside 1 of wording | `2026-11-10`, not acceptable, 7 after 9, 10 and 15, none where the model had a row: it held, its reader laying 153 million more layouts as its reference does. Each defect now has its row first. A map's refusals are held to `Joint::map` itself; the last act arms before it reads, so the port's lead bounds the port's code; the emitter's version is a line the identity keeps; a wake asks no statement of a task; an alarm owes what a source owes; the release timer's device is a node; and three inputs have their one outcome. |
| 5 | 6 | 3 (RP5-1 to RP5-3), beside 2 gaps and 1 of wording | `2026-11-10`, not acceptable, 3 after 7, 9, 10 and 15, each where the model had no row and two in round 4's own answers; the model held, 15 rows and 5,023 more maps beside `Joint::map`. Each defect has its row first. An alarm's time base is the counter unless each reader of its time is extended over its own device, of every set of readers; a wake's first point has what its step must outlast, the realization's `last-act-lead` and the port's lead, held by the checker, where the release alarm's sentence had no holder; and every line the identity keeps is one a hosted plan writes, `(emitter …)` among them. |
| 6 | 7 | 3 (RP6-1 to RP6-3), beside 3 gaps and 1 of wording | `2026-11-10`, not acceptable, 3 after 3, 7, 9, 10 and 15, each in an answer of round 5 and each where a row read heads alone or a list was written from memory; the model held, and its memory formula against a free-byte rig over 5,384,210 layouts. A hosted plan writes a route's item as `/2` does wherever it stands, and no controller line; the facts that are no reader's are read off the release alarm's record, five; a hosted wake is held to nothing, and a wake's step rounds down where a sum lies within it; a placeholder is held as a wake is. |
