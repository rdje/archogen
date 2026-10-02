# Catalog records: what the runtime variant takes, and from whom

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `M2.7.1` (`docs/tasks/M2.md`). This is §12 of [[decision_catalog-records]], moved out of it
  verbatim on `2026-09-30` because round 8's answers took that record to 1 167 of the 1 200 lines `README-ROUTES`
  allows a file under `docs/decisions/`. It is part of that record: normative, numbered as its §12, and reviewed
  with it. `M2.10.1` added the parts and facts the composition of the variant's composite inputs needs, in the
  same change, so the catalog record's round 9 reviews them.

## The fact / decision

### 12. What the runtime variant takes, and from whom

`decision_runtime-analysis-variant.md` §1 lists every input, and each has one owner here:

- **description:** the eADL description;
- **application:** the separately supplied inputs of `ROADMAP.md` §10.3 ("separately supplied application inputs");
- **the plan:** `ROADMAP.md` §7.5's resolved plan, which `M4` produces;
- **the caller:** stands in for the application and the plan until they exist. Everything the caller supplies is
  named in the conclusion, and none of it can back a production claim (§7).

| Variant input | Owner | Name, kind and facet |
| --- | --- | --- |
| `T_i`, `D_i`, `J_i^event`, priority, what releases the task, whether every arrival does | description | — |
| `T_s` | description | the minimum separation of a source's arrivals is a property of its environment |
| `C_i`, `CS_i` | **composite**: the application's parts (the task's own code, its calls to each primitive, its masked runs and each way each can end) and the catalog's (the primitives' and the completion path's costs below) | composed from those parts as `decision_runtime-composite-inputs.md` §3 says; the caller supplies each whole until `M2.10.2` implements it |
| `J_i^release`, `J_s` | **composite**: the description's periods, jitters and what releases each task; the catalog's costs and facts; the application's masked runs through `CS`; and the plan's order among sources | composed as a least fixed point, as `decision_runtime-composite-inputs.md` §3 says; the caller supplies each whole until `M2.10.2` |
| task facts | application | whether a task suspends, locks the scheduler, shares data outside its sections, or masks other than through the runtime API, plus condition 8 for its own figures, which for the composition also says they hold from any entry state; each way each masked run can end, at an `unmask` or at the job's completion; and `leaves-interrupt-hardware-alone`, covering the application's initialisation too, as `decision_runtime-composite-inputs.md` §2 words it. The caller declares the same of any application code in a service, and `releases-after-initialisation`. The runtime API record's behavioral facts `no-suspension-primitive` and `no-scheduler-lock-primitive` support the first two for a task the application declares uses only that API |
| `C_rel` | catalog | `timer-service`, a timing cost |
| `C_s` | catalog, only with the code fact `no-application-code.<source>` `yes` from the same record | `service.<source>`, a timing cost. The record that supplies it supplies the source's behavioral code facts too: `no-application-code`, `acknowledge-at-entry`, `defers-nothing` and `one-request-per-arrival`, each suffixed `.<source>`. So a change to that record's code, which is what could add application code to the service, makes the fact's review stale. Without the fact `yes`, the service runs application code, so `C_s` is composite, and the caller's. Every declared source needs a record that supplies `service.<source>`, with its cost `unknown` when it has none, and that states the source's facts and `external.<source>`; without one the source's facts cannot be read, and its inputs are undeclared |
| acknowledge point, deferred work | catalog | behavioral code facts `acknowledge-at-entry.<source>` (`no` means at exit; a PLIC source states `no`, since its claim is a load some time after the trap, an arrival before it merges into the pending request, and the gateway holds a later one until the completion) and `defers-nothing.<source>`, from the record that supplies `service.<source>`. A task that runs deferred work is named by the description |
| interrupt priority, the enabled set | the plan | — |
| `S`, `W_wake`, `γ`, `ρ`, `δ` | catalog | timing costs `switch`, `wake`, `preemption-delay`, `compare-rounding` and `delivery` |
| the composition's parts | catalog | timing costs `api.<p>` and `masked.<p>` for each primitive `p` a task can call, and `completion` and `masked.completion`, all from the runtime API record: the one record that supplies `completion` under the selection (the groups below; `decision_runtime-composite-inputs.md` §1) |
| the composition's preconditions | catalog | behavioral facts, each with its group in the facts table below: the code facts `reprograms-only-in-service`, `releases-never-latched`, `primitives-out-of-line` and `pending-taken-after-unmask`; the hardware facts `external-before-timer` and `one-external-controller`; for each source, the hardware fact `external.<source>` and the code fact `one-request-per-arrival.<source>`; `no-empty-claim`; `one-claim-per-trap`, read when `external-before-timer` is `no`, and for the variant's condition 5 by the paragraph after the behavioral code facts below; `starts-by-transition`; and `runtime-discipline.<id>`, from every record whose implementation facet, not `none`, is in the claim's closure, and from `M4` in the image's, about every package its implementation's own and reached sets hold. That set is computed to a fixed point, since reading a statement adds its record's behavioral model to the closure (`decision_runtime-composite-inputs.md` §2) |
| the order among sources | the plan | the controller's priorities, a strict order over the sources whose `external.<source>` is `yes`, each deliverable to the hart's machine-mode context, its priority above that context's threshold; the timer's place is `external-before-timer`'s. A plan that omits either statement leaves every `J` undeclared |
| the platform facts | catalog | behavioral facts, one per condition of the variant's `PlatformFacts`, but for one with no fact of its own (listed below). **Code facts** carry a `code` locator into the code they are about (§2). **Hardware facts** are `one-processor` and `compare-level`. The timing fact `eager-switching` must come from the record that supplies `switch`: `yes` means the variant's condition holds for that `switch`, that switching is eager or that `S` includes every deferred save and restore, and its basis says which |

The behavioral code facts, one per condition of the variant's `PlatformFacts`:

- `preemptive-everywhere`
- `interrupts-do-not-nest`
- `sections-mask-every-interrupt`, which covers the kernel's sections. The variant's condition is the
  conjunction of this fact and the application's task fact that no task masks other than through the runtime API.
  `M2.7.5` composes them so, and the conclusion names both
- `services-preempt-every-task`
- `pending-taken-and-transitions-unmasked`
- `timer-event-driven`
- `compare-rounds-up`
- `due-check-matches-compare`
- `no-early-release`
- `raised-only-when-due`, which states both "raised only when a release is due" and "releases every due task", the
  variant's condition 6, which since `M2.11` says *taken*. "Raised" here covers an interrupt still pending after a
  service has moved the compare on, so it means what *taken* means there, and
  its basis establishes that neither the timer service's return nor initialisation's first enabling of interrupts
  comes while the interrupt still reflects a compare value that was replaced (`decision_runtime-composite-inputs.md`
  §4, step 4)
- `only-timer-releases-timer-tasks`

The variant's `PlatformFacts::services_paid_by_arrivals`, its condition 5 that every interrupt taken runs one
service, paid for by a due release or an arrival (added by leaf `M2.11`), has no fact of its own. It is `yes` when
each of these is `yes`:
- `no-empty-claim`: every external trap is taken while a request an arrival made is pending at the controller that the
  trap's own claim, read at that instant, would take, and that claim, whenever it reads, takes a request an arrival
  made;
- `one-claim-per-trap`, read for this whatever `external-before-timer` states: each trap taken for an interrupt runs
  one service, a timer trap claiming nothing;
- for each declared source, `one-request-per-arrival.<source>` and `external.<source>`, so its interrupts reach the
  hart only through the controller those facts are about;
- `one-external-controller`;
- the application's `leaves-interrupt-hardware-alone`, with the caller's declaration of it for a service's
  application code, and each `runtime-discipline.<id>`, without which no fact about the whole image holds
  (`decision_runtime-composite-inputs.md` §2): code that reads the claim register empties a request a trap was
  taken for.

It is `no` when `no-empty-claim`, `one-claim-per-trap` or any `one-request-per-arrival.<source>` is `no`, each of
which contradicts a clause of the condition. Otherwise, when any conjunct is not `yes`, the fact reaches the variant
undeclared, as a value this section could not read does (below), naming each such conjunct. The condition's timer
half is condition 6's own fact, `raised-only-when-due`. `M2.7.5` composes it so, and the conclusion names each conjunct.

The variant's condition 8 for catalog costs is each cost's own `holds-under-preemption`.

**Every fact the variant and its composition read, and the cost `compare-rounding`**, which its group places
among them, with the facet a lookup finds each in, what it is about, and the group, if any, that fixes which record
supplies it:

| Fact | Facet | About | Group anchor |
| --- | --- | --- | --- |
| `timer-event-driven`, `compare-rounds-up`, `due-check-matches-compare`, `no-early-release`, `raised-only-when-due`, `only-timer-releases-timer-tasks` | behavior-model | code: the timer service's, and every write of the compare, initialisation's first included | `timer-service` |
| `compare-rounding`, a timing cost, not a fact | timing-model | the rounding the timer-service record's code does whenever it turns a nominal instant into a compare value, on the target's counter, initialisation's first write included | `timer-service` |
| `preemptive-everywhere`, `sections-mask-every-interrupt` | behavior-model | code: the runtime's scheduler and sections | `completion` |
| `interrupts-do-not-nest`, `services-preempt-every-task`, `pending-taken-and-transitions-unmasked`, `one-claim-per-trap`, `starts-by-transition` | behavior-model | code: the port's trap entry and exit and its transitions, each fact as `decision_runtime-composite-inputs.md` §2 words it; `one-claim-per-trap`'s basis states, when it is `no`, how a trap finds a further pending interrupt and the order among those it serves (§14.4) | `switch` |
| `pending-taken-after-unmask`, `no-empty-claim` | behavior-model | code, with the hardware's half established in its basis; `no-empty-claim` is `yes` on a target with no external source, and like each of the twelve facts §14.2 names it is known only as §14.2 states | `switch` |
| the facts of §14.4's table | behavior-model | code: the port's, or a `describes` record's, as §14.4 says; `panic-strategy-abort` and `generated-guard-check-contexts` not code facts, each with a `file` locator | `switch` |
| `convention-stated.<id>` | behavior-model | not a code fact; a `file` locator to the convention record's copy of its convention (§14.4) | the record `<id>` itself; a statement named with another record's id is refused |
| `guard-check-contexts.<id>` | behavior-model | code: the contexts that run the guard checks the record `<id>`'s code makes (§14.4) | the record `<id>` itself; a statement named with another record's id is refused |
| `fatal-path.entry`, `fatal-path.trap`, timing costs, not facts | timing-model | the port's fault path, as §14.4 says | `switch` |
| `one-processor`, `compare-level`, `external-before-timer`, `one-external-controller` | behavior-model | hardware, about the target; `one-external-controller` is `yes` on a target with no external source; `external-before-timer` `no` needs `one-claim-per-trap` `yes` (`decision_runtime-composite-inputs.md` §2) | — |
| `runtime-discipline.<id>` | behavior-model | code: every package of the stating record's implementation's own and reached sets | the record `<id>` itself; a statement named with another record's id is refused |
| `no-application-code.<source>`, `acknowledge-at-entry.<source>`, `defers-nothing.<source>`, `one-request-per-arrival.<source>` | behavior-model | code; for `one-request-per-arrival`, with the hardware's half, how the target's controller turns the source's line into requests, established in its basis | `service.<source>` |
| `external.<source>` | behavior-model | hardware: the source reaches the hart as a machine external interrupt, through the controller's context for the hart's machine mode, whose priorities the plan sets | `service.<source>` |
| `reprograms-only-in-service` | behavior-model | code | `timer-service` |
| `no-suspension-primitive`, `no-scheduler-lock-primitive`, `releases-never-latched`, `primitives-out-of-line` | behavior-model | code | `completion` |
| `eager-switching` | timing-model | code | `switch` |

A fact in any other facet is refused at load (`catalog-field`), so no author chooses which hash covers it. A
behavioral fact about hardware sits where the record's targets' files enter the bound hash (§3). So an edit to a
target's `.env` or `.eadl` makes its review stale.

- **Image-specific names.** `switch`, `wake`, `preemption-delay`, `timer-service`, every `service.<source>`, and the
  composition's `api.<p>`, `masked.<p>`, `completion` and `masked.completion` include the image's code: generated
  code (`ROADMAP.md` §8.2) and build-selected instrumentation. `independent` is admitted only for `compare-rounding`
  and `delivery`, and only on a board (§2).
- **What each catalog value must bound, independently of any application:**
  - `preemption-delay`: the most one preemption or service adds to **any** preempted execution, as the variant's
    §1 defines γ. A value measured on
    particular code is that code's, and its `scope` says so;
  - `compare-rounding`: the worst case over all release instants. The variant takes ρ as given and derives
    nothing, so the worst case is what it uses, which is conservative. A smaller value because the description's
    releases fall on ticks is not composed in `/1`: it needs the timer's resolution as a number, which no
    yes-or-no fact states (`decision_runtime-composite-inputs.md` §7);
  - `delivery`: the delay for any interrupted code. On an emulator, delivery waits for the end of a translated
    block, so it is image-specific there, and §2 refuses `independent` for it. It bounds two latencies, added
    together: from the arrival or compare match to the interrupt being pending, and from its being pending and
    unmasked to the processor taking the trap. `timer-service` and every `service.<source>` begin when the trap is
    taken, so nothing masked falls between delivery and a service;
  - `switch`: it ends at the incoming context's first instruction, which is the job's first instruction of its own
    code for a transition in, and the resumed instruction for a return to a preempted job;
  - the composition's part costs: `holds-under-preemption` `yes` on each also means the bound holds from any state
    the code before it leaves, since the parts are added (`decision_runtime-composite-inputs.md` §4).
- **Selection.** An analysis for profile `P` on target `X` draws on these records only:
  - for facts, records whose `profiles` include `P` and whose `targets` admit `X`;
  - for costs, costs whose `target` is `X`, in records whose `profiles` include `P`. The contract admits `X`,
    by §2.

  A claim with no target draws facts only from records with `(targets any)`, and reads no cost.

  Lookups are by `(facet, name)`, with the kind and facet the tables above fix.
- **`<source>`** is the source's id as the description and the enabled set name it, verbatim. An id outside
  §2's name grammar cannot be supplied by any record, so its inputs reach the variant undeclared, with the reason
  named.
- **Each name comes from exactly one record.** Two records that supply a name under the same selection are
  refused at load, for every profile and target the catalog names and for a claim with no target
  (`catalog-conflict`), rather than reconciled. An `unknown` supplies its name: it is a record's statement, and a
  lookup that meets it returns it, since `ROADMAP.md` §9 says "Cross-validation investigates source conflicts
  rather than averaging them".
- **Groups.** A code fact is about the code of the cost it is grouped with, and must come from the record that
  supplies that cost, so one review sees the fact and the code together. Each group has an anchor, given in the
  facts table and below.
  - **Where the rule is checked.** Under every target selection, meaning a profile and a named target under
    `targets/`, a target admitted only through `any` included, in which some record supplies the group's anchor.
    There, each name of the group that some record supplies must be supplied by the anchor's record. Otherwise the
    catalog is refused at load (`catalog-field`).
  - **Where it is not.** Under a selection where no record supplies the anchor, the group's names are not read,
    and whatever needs them is undeclared. A claim with no target reads no cost, so no cost-anchored group is
    checked or read for it.
  - **How the analysis reads them.** Only from the id its anchor's lookup returned, so a fact is never combined
    with a cost from another record.

  The groups are:
  - `switch`: `eager-switching`, `interrupts-do-not-nest`, `services-preempt-every-task`,
    `pending-taken-and-transitions-unmasked`, `pending-taken-after-unmask`, `no-empty-claim`, `one-claim-per-trap`
    and `starts-by-transition`, the port's; and the facts of §14.4's table and the costs `fatal-path.entry` and
    `fatal-path.trap`, the port's statement (`M2.12.3`);
  - `service.<source>`: its four `.<source>` code facts and `external.<source>`;
  - `timer-service`: the six timer facts, `reprograms-only-in-service`, and the cost `compare-rounding`, since the
    rounding is the timer service's code. `independent` stays open to it on a board, and its review cites that code.
    The compare's first write, in initialisation, is that record's code too, and the group's facts and
    `compare-rounding` hold of it;
  - `completion`: every `api.<p>` and `masked.<p>`, `masked.completion`, `no-suspension-primitive`,
    `no-scheduler-lock-primitive`, `preemptive-everywhere`, `sections-mask-every-interrupt`,
    `releases-never-latched` and `primitives-out-of-line`. That record is **the runtime API record**. No
    primitive may be named `completion`: a cost named `api.completion` is refused at load (`catalog-field`), so
    `masked.completion` is always the completion path's.

  A hardware fact is about the target, sits where the target's files enter its bound hash (§3), and is in no group,
  except `external.<source>`, which is about that one source's wiring and so sits in its service's group.
- **The code facts** are those the facts table marks as about code, and each takes a `code` locator (§2). The
  hardware facts are `one-processor`, `compare-level`, `external-before-timer`, `one-external-controller` and each
  `external.<source>`.
- **What the variant does with a value it could not read.** Three cases reach the variant as an undeclared
  input: a name no record supplies, an `unknown`, and a cost outside its `holds-for` (more tasks, or more declared
  sources other than the timer). The variant refuses with its own verdict, `analysis-inconclusive` (the variant's
  §5), with the reason named. It never fills a value in. A cost with `holds-under-preemption` `no` fails condition
  8, which the variant refuses as outside the model.
- **Conversion.** A cost converts into the analysis's unit exactly toward a finer unit, and rounds up toward a
  coarser one, as the variant's §1 rounds every cost. An overflow is refused.

## Why

§12 is the one section of the catalog record that belongs to a consumer: the runtime variant, and now the
composition of its inputs. It grows when they do. Kept apart, it can grow without pushing the catalog record past
its ceiling, and a reviewer of either record is given both.

## How to apply

- A bare section number here is the catalog record's: "§3" is its hash grammar, "§7" its claims. One that is
  `ROADMAP.md`'s says so.
- A change to what the variant takes, or to how its composites are composed, lands here and in
  [[decision_runtime-analysis-variant]] or [[decision_runtime-composite-inputs]] together.
