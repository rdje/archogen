# The runtime variant's composite inputs: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `M2.10.1` (`docs/tasks/M2.md`). This is the review history of
  [[decision_runtime-composite-inputs]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`M2.10.1`'s acceptance is a review, by a context that did not write the record, that finds no composite
under-charging against the runtime variant's §1 definitions. Each round below is one such review, with every
finding and the answer the record gives it. The section numbers are the record's as answered.

Each round is a new context, read-only, which had not written the record. It is given:
- the record;
- `decision_runtime-analysis-variant.md`;
- the catalog record's §2, §7 and §12;
- `ROADMAP.md` §7.3–§7.5 and §10.3;
- `crates/rt-core`'s scheduler.

It is barred from other implementation, and may check external specifications on the web.

**Round 1**, `2026-09-30`: the `C_i` sum, the max-of-sums form of `CS_i` and the `J` fixed point were judged sound
within the model. The reviewer checked the fixed point numerically against a simulation of the record's own model,
on 1 920 admitted random sets. There were no violations when the interrupt is chosen at the moment the trap is
taken, and 156 violations, each at most `δ − 1`, when delivery commits at its start (K2). The reviewer also
fetched the ratified privileged specification and confirmed the record's M-mode interrupt order. There were 19
findings, 3 of them defects, and the verdict was "Not acceptable as it stands". The answers keep the variant's
record and its code unchanged. Each new requirement is a precondition of the composition (§2), with a verdict when
it is declared false. The specification's two sentences were re-read by the answering context before the ledger
entry `riscv-privileged` was written, and that settled the variant's own open question about `mret`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| K1 | defect | a job that completes while still masked: a run ended by completion was in no term, so `CS_i` and every `J` built on it under-charged (a true 30 against a composed 7 or 27) | a run ends at its `unmask` or at the job's completion, and the application says which. A run ending at completion adds `completion` in place of `unmask` (§1, §3, §4) |
| K2 | gap | where delivery ends and a service begins was not pinned, and masked time after the processor commits to a trap belonged to no part | `δ` covers arrival or compare match to pending, plus pending-and-unmasked to the trap being taken, and every service begins at the trap. At least the variant's `δ`, so its floors stay safe (§2) |
| K3 | defect | a compare written late during initialisation, whose masked run `L` does not hold | the code fact `releases-after-initialisation`: every nominal release is at or after the first unmask of the running system (§2, §4) |
| K4 | gap | nothing said that a task's own `unmask` lets a pending interrupt in, so two runs could fuse | the fact `pending-taken-after-unmask`. The privileged specification requires evaluation "immediately following … an explicit write to" `mstatus` or `mie` (§2, §4) |
| K5 | defect | a source's stops gave `not-established` for no fixed point and overflow, but `unsupported-profile` past `T_s` | every source stop means `J_s` cannot be bounded below its no-loss limit: condition 7's `unsupported-profile`. The timer's stops stay `not-established`; limits are `analysis-inconclusive`. The limit at exit is `T_s − C_s − S` (§3, §6) |
| K6 | gap | interrupts behind a stopped one had no `J_q`, and an iterate past a refusal bound was handed on | a stopped interrupt has no value; everything behind it is not composed and is named under the stop. No iterate is passed on (§3, §6, Why) |
| K7 | gap | three verdicts unstated: the pre-check's rational outside 128 bits, the fact declared `no`, a non-strict order | `analysis-inconclusive`, `unsupported-profile` and `unsupported-profile` (§3, §6) |
| K8 | gap | the timer's place in the order is fixed by hardware, not configured by the plan | the hardware facts `external-before-timer` and `every-source-external`; the plan orders only the sources (§1, §2) |
| K9 | ambiguity | `rt-core`'s `unmask` does deliver releases latched while its mask depth is raised | the code fact `releases-never-latched`: the port's ordering means no service runs while the depth is raised, so the path is never taken on the target (§2, §4) |
| K10 | ambiguity | "from the call to its return" is undefined once a primitive is inlined | the code fact `primitives-out-of-line`: out-of-line primitives with barrier masking instructions, so boundaries are instructions (§1, §2, §4) |
| K11 | nit | the category order is the record's choice, not §7.3's; §7.3's "unless a valid argument" dropped; categoryless parts | all three stated (§6) |
| K12 | nit | the timer count cited the weaker fact | each service maps to the release that was due when its interrupt was raised (§4) |
| K13 | ambiguity | whether a caller-supplied `C_s` stops the `J` composites | it does not; the composites built on it are composed and the figure is named (§6) |
| K14 | nit | the application's entry-state declaration was not among what §12 gains | listed, with each run's ending (§5) |
| K15 | nit | "switch ends at the job's first instruction" did not fit a return | a return ends at the resumed instruction (§2) |
| K16 | nit | the timer's refusal bound is undefined with no timer-released task | `Δ_timer` is then not computed (§3) |
| K17 | nit | "the most by which" overstated a bound | "an upper bound on how late" (§4) |
| K18 | nit | platform-local interrupts and the Advanced Interrupt Architecture can reorder the timer | `every-source-external` puts them outside; the ledger entry quotes AIA §4.1 (§2) |
| K19 | nit | a future primitive that returns masked or unmasks would be missed | the new-primitive rule classifies each before it is costed (How to apply) |

## Why

The record states the composition as it stands, and this file keeps how it got there, as for the catalog record.

## How to apply

- A new round appends its paragraph and table here, and one row to the summary table in the record's `## Review`.
- A finding is answered in the record first, and its row here names the sections that answer it.
