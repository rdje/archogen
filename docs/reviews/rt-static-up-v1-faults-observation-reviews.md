# The fault paths' observation events: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — round 1 answered; round 2 next; the review closes on the first round that finds no defect
- **Owner / source:** leaf `M2.15` (`docs/tasks/M2.md`). The design under review is
  [`rt-static-up-v1-faults-observation.md`](../profiles/rt-static-up-v1-faults-observation.md), part of the profile's
  fault contract, with the routing it makes in the contract's *Still open* and the pointers in `M4.3` and `M4.9`.

## The draft

Step 1, `2026-10-02`: drafted from a quoted inventory a read-only context made of `ROADMAP.md` §6.3, the fault
contract, `M2.9`'s review findings (`docs/decisions/decision_runtime-contract-gaps.md`), `rt-core`'s and
`rt-reference`'s transitions, and the repository's traces — 161 entries, 21 ambiguities and 12 open questions. It
answers them by refusing what can be refused and delegating what another leaf owns: an arrival inside a region and a
discarded release produce no event; a start or a resume is stamped at the incoming context's first instruction, which
takes the target's decision-before-delivery out of the comparison; a delivery is compared task by task, the hosted
latch's coarser count forgiven within the target's; a fatal fault raised during a delivery is compared by its kind and
class, its attribution left to the port's fixtures; and a source that does not count arrivals makes its stretch
inconclusive.

## Round 1

`2026-10-02`, of commit `51e9663`, by a new context, with probes driving `rt-core` and `rt-reference` side by side
(P1–P11). The core held: stamping a start or a resume at the incoming context's first instruction takes the target's
decision before its delivery out of the comparison, and `rt-core`'s delivered lists match §3 for both policies and one
to three arrivals. 23 findings: 9 defects, 5 gaps, 6 drafting points, 3 nits. The defects shared one cause, a hosted
run fed by its own clock; the answer drives the hosted run by the target's observations.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | `release` stamped at arrival, which the contract keeps apart from observation | stamped where its service judges it, which can follow the arrival by its latency (rule 6) |
| 2 | defect | "one run" ignored release latency: a release due just after a region closes, or arriving before a completion and observed after, made correct pairs disagree | the hosted run is driven by the target's observations, a delivery's releases together at the region's close before the hosted model decides |
| 3 | defect | the count rule accepted a target that miscounted a timer task's overruns | the target's releases checked against the plan and the recorded events, not the hosted trace (§4 item 3) |
| 4 | defect | a delivery a fatal fault cuts short made correct pairs disagree | the externally raised fault given to the hosted model where the target raised it |
| 5 | defect | a lost arrival of a source that does not count arrivals, outside a delivery, disagreed; the inconclusive stretch ended too early | driving gives the hosted model only what the target observed; the plan check allows at most one per recorded arrival, the fixtures judging a loss |
| 6 | defect | §2's and §3's absolutes contradicted the Terms' window | "save in the Terms' window"; under `Fault`, "no later job of the task starts"; a job's own fatal fault's stretch not compared (§4 item 6) |
| 7 | defect | `rt-core`'s halts from `mask`, `unmask` and `decide` unmapped; a non-job `complete` panics | the halt's first report mapped, whichever call gives it; the panic noted, left to the fixtures |
| 8 | defect | §5 claimed fields `rt-core` does not hold: the raising context | the fields `rt-core` holds named; the raising context not compared |
| 9 | defect | the interrupted task depends on the port's trap structure, so comparing it flagged correct pairs | attribution not compared; the fixtures judge the target's, `rt-core`'s tests the hosted model's |
| 10 | gap | `preempt` unstamped; start and resume indistinguishable in `rt-core` after a skip | `preempt` stamped; the harness says which after a `JobSkipped`; a `preempt` compared by its task and what follows it |
| 11 | gap | nothing delimits a delivery | events compared task by task between consecutive transitions, no delimiter needed |
| 12 | gap | no event carried a skipped job's successor's nominal instant | the overrun `fault` carries it under `skip-late-job` |
| 13 | gap | the fatal event's fields before the mark is complete; a trap-path fault before the handler | the fields fixed at the raising; a fault on the trap path before the handler's first act produces none |
| 14 | gap | rule 4's "`M2.15` maps the two records" handed to `M4.6`, which did not point here; nor did `M4.7` | the records agree on kind, class, guard and mark, their attribution differing by construction; `M4.6` and `M4.7` point here |
| 15 | drafting | "the trace's last event" forbade diagnostic events after a halt | dropped; the list of what does not follow kept |
| 16 | drafting | §4's exclusion of interrupt events read generally | "on the fault paths" |
| 17 | drafting | a completion followed by idle | "or idle" |
| 18 | drafting | the guard owner and escalation mark neither compared nor excluded | compared |
| 19 | drafting | the hosted latch's held release's instant | driven as §4 says, no mark forms; the undriven latch described, not compared |
| 20 | drafting | a faulted task's latched arrival read as `release`; "no event" against §6.3's interrupt pending | "and is not faulted"; "no `release` or `fault` event" |
| 21 | nit | §3.1 used bare | added to the header's list |
| 22 | nit | the plan supplies other IDs too; `InvariantViolated` | "the tasks' IDs"; `InvariantViolated` is the assertion failure |
| 23 | nit | the acceptance named a home the record does not use; the annex said "board" | the acceptance notes the record; the annex says "a target's" |

## Why

The fault paths' traces are what `M4` compares between the hosted playground and the emulator; a rule stated after the
first mismatch would be fitted to it.
