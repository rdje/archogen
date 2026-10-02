# The fault paths' observation events: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — rounds 1 to 4 answered; round 5 next; the review closes on the first round that finds no defect
- **Owner / source:** leaf `M2.15` (`docs/tasks/M2.md`). The design under review is
  [`rt-static-up-v1-faults-observation.md`](../profiles/rt-static-up-v1-faults-observation.md), part of the profile's
  fault contract, with the routing it makes in the contract's *Still open* and the pointers in `M4.3`, `M4.6`, `M4.7`
  and `M4.9`.

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

## Round 2

`2026-10-02`, of commit `5d3f45b`, by a new context asked first for regressions, with probes P1–P8. Round 1's driving
held in principle, and most of its answers held; one regressed (20). 18 findings: 6 defects, 3 gaps, 5 drafting points,
4 nits. The driving was incomplete: it named no source for the hosted model's `mask`, `unmask` and `complete`, so the
hosted model could neither raise its own assertions nor see a release judged inside a region; and it decided only where
the target transitioned, so a missed preemption was accepted. Defects per round: 9, 6.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | the driving named no source for `mask`, `unmask` and `complete`, and no event marked a region | `mask` and `unmask` are events (§2), each driven with `complete`; a fatal fault stamped at a driven call is the hosted model's to raise; a release given above depth zero is latched, so one judged inside a region disagrees |
| 2 | defect, regression of answer 20 | "else `fault`" gave a faulted task's latched arrival a `fault` event | "`release` if it owes no job, `fault` if it owes one, and nothing if it is faulted" |
| 3 | defect | deciding only where the target transitioned accepted a missed preemption | the hosted model decides at each trap's exit and after each `complete`; item 2 compares each decision with the next event naming the context the target ran, an interrupt entry naming the one it preempted |
| 4 | defect | the plan check counted only, missing a late or out-of-order observation; instants equal by construction | in nominal or arrival order, observed within `J_i^release` or at its region's delivery; instants not compared between the traces |
| 5 | defect | the plan check ignored a halt and a faulted external task | "until the task is faulted or the runtime halts, and save in the Terms' window" over every clause |
| 6 | defect | item 6's stretch too short, and blind to other tasks' diverging decisions | every event from that task's job's `start` to the fault, item 4's checks kept |
| 7 | gap | idle had no stamp | an idle-entry row; a trap returning to idle stamps none |
| 8 | gap | a check-found guard cannot be told from a data-access guard | "where either trace's fatal fault is attributed to a task" |
| 9 | gap | the plan check needs a periodic task's first release instant | routed to the timer-service record, `M2.7.4` (§6) |
| 10 | drafting | neither `Dispatch` nor `Switch` tells start from resume; `Continue` unmapped | the harness says which from each task's history; `Continue` no event |
| 11 | drafting | `rt-reference` decides inside its own calls | the sentence names `rt-core`'s |
| 12 | drafting | driven, `complete`'s delivered list is empty | said |
| 13 | drafting | the driving is for any comparison, not only the fault paths | §1 and `M4.9`'s pointer say so |
| 14 | drafting | item 1's "the same fields" against item 5 | "save as item 5 says" |
| 15 | nit | `Fatal` holds the attribution too | named, not compared |
| 16 | nit | `complete` panics for another task's too | "with no job, or another task's, running" |
| 17 | nit | the non-job completion is §5's | `M4.6`'s pointer says §4 and §5 |
| 18 | nit | bare self-references; the review file's owner line | "§1 to §6 are this record's"; `M4.6` and `M4.7` named |

## Round 3

`2026-10-03`, of commit `ea2ce10`, by a new context asked first for regressions, with a probe crate driving `rt-core`
exactly as round 2's §4 said, its decisions stamped three ways (P1–P13). 19 findings: 8 defects, 4 gaps, 6 drafting
points, 1 nit. Defects per round: 9, 6, 8 — not converging. Every new defect sat in the comparator's mechanics round 2
had written: how `mask` and `unmask` are replayed, where the hosted model decides, how its decisions are stamped, and
how each is compared with what the target ran next. Those depend on the port's catalog facts (`decision-placement`,
`one-claim-per-trap`, a primitive preempted at its entry) and on scheduling outside the fault paths — a missed
preemption is no fault path — so they are `M4.9`'s and `M4.3`'s. The answer shrinks §4 to what is the fault paths'
own and delegates the rest, naming what `M4.3` must record and what `M4.9` takes as parameters
(`docs/knowledge/an-answer-to-a-review-must-keep-what-earlier-answers-carried.md`, habit 3).

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | deciding after `complete` and at each trap's exit put the hosted decision before the delivery, and §5's stamping rejected rule 4's own case either way | where the hosted model decides is `M4.9`'s; §4 item 2 compares only the next task after a completion's delivery, rule 4's "same schedule" |
| 2 | defect | `mask` and `unmask` stamped at the call, before the depth changes, made correct pairs at a primitive's entry disagree | the `mask` and `unmask` events and their replay withdrawn: `M4.3` records regions and traps, `M4.9` drives with the port's facts |
| 3 | defect | nothing compared start against resume, so a resumed abandoned job was accepted | item 3: after `skip-late-job` the task's next entry is a `start`; after `fault`, none |
| 4 | defect | the latency bound applied after the run's timing claims had ended, and where `J` is undeclared | "where `J_i^release` is declared and the run's timing claims hold (rule 5)" |
| 5 | defect | no lower bound: an early observation passed | "none … before its nominal instant or recorded arrival" |
| 6 | defect, regression of answer 16 | the exclusion of interrupt and timer-programming events was deleted silently | restored, item 7 |
| 7 | defect | `unmask` returns a delivered list, not a depth | the row withdrawn with the events |
| 8 | defect | another task's release judged inside a region was accepted | item 4: "none is observed while a masked region is open" |
| 9 | gap | a decision at each trap's exit is not the contract's | withdrawn with the decision points, `M4.9`'s |
| 10 | gap | a driven panic, or another task's `mask`, had no verdict | §5: a disagreement on what runs, `M4.9`'s to report |
| 11 | gap | `rt-core`'s guard is a task's or the interrupt stack's | another guarded stack's taken from the target's record |
| 12 | gap | the first release instant and the counting fact routed to `M2.7.4`, which named neither; the contract said no fixture needs the first | `M2.7.4` names both; the contract says §4's plan check needs it |
| 13 | drafting | `mask`, `unmask` and idle entry beside §6.3's events | the first two withdrawn; idle entry kept, a stamp |
| 14 | drafting | `Switch` names both tasks | "name the incoming task, not whether it starts or resumes" |
| 15 | drafting | the latch holds none save a release judged inside a region | §5 says which delivery item 1 assumes |
| 16 | drafting | `rt-core`'s primitives take no task | withdrawn with the events |
| 17 | drafting | "a fatal fault stamped at a driven call" | withdrawn: faults raised from outside the runtime's logic are given; the rest are the hosted model's |
| 18 | drafting | "each trap's exit" for a runtime-API trap or several interrupts in one trap | withdrawn with the decision points |
| 19 | nit | "§4" where §5 too leaves to the fixtures | "§4 and §5" |

## Round 4

`2026-10-03`, of commit `a594223`, by a new context asked to judge round 3's scope decision as well as its text, with
probes P1–P9. It found the decision mostly sound — the driving and the decision points depend on the port's catalog
facts, and a missed preemption after an ordinary delivery is no fault path — but cut too deep in three places that
depend on no port fact: completions, the hosted model's own assertion failures, and what follows a fatal fault. 11
findings: 1 defect, 4 gaps, 3 drafting points, 3 nits. Defects per round: 9, 6, 8, 1.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | defect | item 4's upper bound used `J_i^release`, which bounds when the releasing service starts, against a stamp where it judges | the upper bound withdrawn: how late a release may be observed is the timing analysis's to bound and `M4.9`'s to check |
| 2 | gap, round 2's answer 1 partly dropped | nothing said where the hosted run gets each completion, which item 1's judgments depend on | each job's completion given where its `complete` is stamped, before a release at the same instant |
| 3 | gap | the runtime's own assertion failures at calls that open or close no region were neither given nor recorded | every fatal fault the target raised is given to the hosted run; the comparison does not judge fatal faults, the fixtures do (item 5) |
| 4 | gap | nothing checked that nothing runs after a fatal fault | item 5: after it, neither trace shows a `release`, `start`, `resume` or `complete` |
| 5 | gap | the context that completes a primitive for an abandoned job could stamp `resume` | §2: it stamps neither |
| 6 | drafting | "one release per nominal release due" against `Fault`'s discards and a halt | "in nominal order, none twice and none skipped … under `Fault` up to the one that faults it" |
| 7 | drafting | "the run's timing claims hold" undecidable from the trace | withdrawn with the upper bound |
| 8 | drafting, round 2's answer 12 dropped | the delivered lists are empty where the delivery is given after the region closes | restored in both §5 rows |
| 9 | nit | after a halt `complete` returns `Halted` | "before a halt" |
| 10 | nit | `M4.6`'s pointer still named the non-job completion, now `M4.9`'s | dropped; the pointer names every fatal fault |
| 11 | nit | the facts' parenthesis read as exhaustive, and `one-claim-per-trap` is §12's | "among them … (§14.4) and `one-claim-per-trap` (its §12)" |

## Why

The fault paths' traces are what `M4` compares between the hosted playground and the emulator; a rule stated after the
first mismatch would be fitted to it.
