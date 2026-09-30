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
- **application:** the separately supplied inputs of §10.3 ("separately supplied application inputs");
- **the plan:** §7.5's resolved plan, which `M4` produces;
- **the caller:** stands in for the application and the plan until they exist. Everything the caller supplies is
  named in the conclusion, and none of it can back a production claim (§7).

| Variant input | Owner | Name, kind and facet |
| --- | --- | --- |
| `T_i`, `D_i`, `J_i^event`, priority, what releases the task, whether every arrival does | description | — |
| `T_s` | description | the minimum separation of a source's arrivals is a property of its environment |
| `C_i`, `CS_i` | **composite**: the application's parts (the task's own code, its calls to each primitive, its masked runs and how each ends) and the catalog's (the primitives' and the completion path's costs below) | composed from those parts as `decision_runtime-composite-inputs.md` §3 says; the caller supplies each whole until `M2.10.2` implements it |
| `J_i^release`, `J_s` | **composite**: the catalog's costs and facts, the application's masked runs through `CS`, and the plan's order among sources | composed as a least fixed point, as `decision_runtime-composite-inputs.md` §3 says; the caller supplies each whole until `M2.10.2` |
| task facts | application | whether a task suspends, locks the scheduler, shares data outside its sections, or masks other than through the runtime API, plus condition 8 for its own figures, which for the composition also says they hold from any entry state; and how each masked run ends, at its `unmask` or at the job's completion. The runtime API record's behavioral facts `no-suspension-primitive` and `no-scheduler-lock-primitive` support the first two for a task the application declares uses only that API |
| `C_rel` | catalog | `timer-service`, a timing cost |
| `C_s` | catalog, only with the code fact `no-application-code.<source>` `yes` from the same record | `service.<source>`, a timing cost. The record that supplies it supplies the source's behavioral code facts too: `no-application-code`, `acknowledge-at-entry` and `defers-nothing`, each suffixed `.<source>`. So a change to that record's code, which is what could add application code to the service, makes the fact's review stale. Without the fact `yes`, the service runs application code, so `C_s` is composite, and the caller's |
| acknowledge point, deferred work | catalog | behavioral code facts `acknowledge-at-entry.<source>` (`no` means at exit) and `defers-nothing.<source>`, from the record that supplies `service.<source>`. A task that runs deferred work is named by the description |
| interrupt priority, the enabled set | the plan | — |
| `S`, `W_wake`, `γ`, `ρ`, `δ` | catalog | timing costs `switch`, `wake`, `preemption-delay`, `compare-rounding` and `delivery` |
| the composition's parts | catalog | timing costs `api.<p>` and `masked.<p>` for each primitive `p` a task can call, and `completion` and `masked.completion`, from the record that supplies the runtime API (`decision_runtime-composite-inputs.md` §1) |
| the composition's preconditions | catalog | code facts `reprograms-only-in-service` and `releases-after-initialisation`, from the record that supplies `timer-service`; `releases-never-latched` and `primitives-out-of-line`, from the record that supplies the runtime API; `pending-taken-after-unmask`, from the record that supplies `pending-taken-and-transitions-unmasked`; hardware facts `external-before-timer` and `every-source-external` (`decision_runtime-composite-inputs.md` §2) |
| the order among sources | the plan | the controller's priorities, a strict order; the timer's place is `external-before-timer`'s |
| the platform facts | catalog | behavioral facts, one per condition of the variant's `PlatformFacts` (listed below). **Code facts** carry a `code` locator into the code they are about (§2). **Hardware facts** are `one-processor` and `compare-level`. The timing fact `eager-switching` must come from the record that supplies `switch`: `yes` means the variant's condition holds for that `switch`, that switching is eager or that `S` includes every deferred save and restore, and its basis says which |

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
- `raised-only-when-due`, which states both "raised only when a release is due" and "releases every due task",
  as the variant's field does
- `only-timer-releases-timer-tasks`

The variant's condition 8 for catalog costs is each cost's own `holds-under-preemption`.

- **Image-specific names.** `switch`, `wake`, `preemption-delay`, `timer-service`, every `service.<source>`, and the
  composition's `api.<p>`, `masked.<p>`, `completion` and `masked.completion` include the image's code: generated
  code (§8.2) and build-selected instrumentation. `independent` is admitted only for `compare-rounding` and
  `delivery`, and only on a board (§2).
- **What each catalog value must bound, independently of any application:**
  - `preemption-delay`: the most one preemption or service adds to **any** preempted execution, as the variant's
    §1 defines γ. A value measured on
    particular code is that code's, and its `scope` says so;
  - `compare-rounding`: the worst case over all release instants. The variant's `/1` takes ρ as given and derives
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

  Lookups are by `(facet, name)`, with the kind and facet this table fixes.
- **`<source>`** is the source's id as the description and the enabled set name it, verbatim. An id outside
  §2's name grammar cannot be supplied by any record, so its inputs reach the variant undeclared, with the reason
  named.
- **Each name comes from exactly one record.** Two records that supply a name under the same selection are
  refused at load, for every profile and target the catalog names and for a claim with no target
  (`catalog-conflict`), rather than reconciled. An `unknown` supplies its name: it is a record's statement, and a
  lookup that meets it returns it, since §9 says "Cross-validation investigates source conflicts
  rather than averaging them". A fact this table co-locates with a cost, in a record that does not supply that
  cost, is refused at load too (`catalog-field`). The pairs are: `eager-switching` with `switch`; each `.<source>`
  fact with `service.<source>`; `reprograms-only-in-service` and `releases-after-initialisation` with
  `timer-service`; and `releases-never-latched` and `primitives-out-of-line` with the `api.<p>` costs.
- **The code facts** are the eleven listed above, the three `.<source>` facts, `no-suspension-primitive`,
  `no-scheduler-lock-primitive`, `eager-switching`, and the composition's `reprograms-only-in-service`,
  `releases-after-initialisation`, `releases-never-latched`, `primitives-out-of-line` and
  `pending-taken-after-unmask`. Each takes a `code` locator (§2). `pending-taken-after-unmask` is also a statement
  about the hardware, and its basis says what establishes that half. The hardware facts are `one-processor`,
  `compare-level`, `external-before-timer` and `every-source-external`.
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

- A section number here is the catalog record's: "§3" is its hash grammar, "§7" its claims.
- A change to what the variant takes, or to how its composites are composed, lands here and in
  [[decision_runtime-analysis-variant]] or [[decision_runtime-composite-inputs]] together.
