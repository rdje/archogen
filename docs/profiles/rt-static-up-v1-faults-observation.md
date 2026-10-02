# `rt-static-up-v1` — the fault paths' observation events, and when a hosted trace and a target trace agree

> Part of the profile's fault contract, [`rt-static-up-v1-faults.md`](rt-static-up-v1-faults.md), normative and
> reviewed with it: the contract's *Still open* routes here "each path's observation events and hosted/target trace
> compatibility, a discarded release's and a completion that closes a region included". Drafted by leaf `M2.15`
> (`docs/tasks/M2.md`), filed by `M2.9`'s review, finding 13. A bare rule number is the contract's; a bare section
> number — §6.3, §7.5, §8.1, §13.4 — is `ROADMAP.md`'s.

- **Status:** `active` — drafted `2026-10-02` by `M2.15`, under review; its rounds are in
  [`docs/reviews/rt-static-up-v1-faults-observation-reviews.md`](../reviews/rt-static-up-v1-faults-observation-reviews.md)

## In plain words

When something goes wrong — a task still busy when its next round is due, a round skipped, a task taken out of the
schedule, the whole system halted — a trace should show it, and show it the same way whether the runtime ran on the
host, in a test harness, or on the emulated or real processor. They cannot show it identically: on the host nothing
interrupts anything, the runtime keeps one "a release came while you were busy" mark where the processor sees every
release, and it decides what runs next at a slightly different moment. So this record says, for each way a fault
shows, which events a trace holds and in what order, and then what may differ between a host trace and a target
trace of the same run, and what must not.

## 1. What this adds to §6.3

§6.3 lists the observation events — "task release/start/preempt/resume/complete, timer programming, interrupt
pending/entry/exit, fault, and selected MMIO interactions" — and asks for "compatible observable behavior, not
unconditional byte-for-byte trace identity across all targets". It names no fields and no fault path. This record
fixes, for the contract's fault paths only, which of those events each path produces, the fields that tell them apart,
their order, and the rule by which a hosted trace and a target trace of one run agree on them. The event set in
general, its fields elsewhere and the orderings it allows elsewhere are `M4.3`'s; the comparison that applies this
record is `M4.9`'s, and the replay manifest's ordering rules (`M4.7`) name it.

## 2. The events, and when each is stamped

| Event | Stamped at | Fields |
| --- | --- | --- |
| `release` | the release's observation (rule 1): its arrival where no masked region is open, its delivery otherwise; a timer release, computed at delivery, there | the task, by its stable logical ID; for a timer-released task, the release's nominal instant |
| `complete` | the job's completion — its last instruction before its completion path's first (Terms) — recorded before a release at the same instant (§13.4) | the task |
| `start`, `resume` | the incoming context's first instruction; a transition preempted before it — by a trap taken inside the transition after it unmasks (rule 2) — stamps neither | the task |
| `fault` | the raising: for an overrun, the release that observes it (rule 1); for a fatal fault, the fault's raising (rule 7) | its §3.1 kind; for an overrun, the task and the policy's outcome, `skip-late-job` or `fault` (rule 5); for a fatal fault, the kept record's fields (rule 7) |

A release that finds its task owing a job produces one `fault` event, kind `overrun`, and no `release` event. Under
`SkipLateJob` that release is the task's next job, with its nominal instant (rule 1), whose later `start` is an
ordinary one. A release arriving inside a masked region produces no event at its arrival: it is observed only at its
delivery (rule 1). A discarded release — any release of a faulted task (rule 5) — produces no event. An event names
a task by its stable logical ID, never an index (§7.5).

## 3. Each path's events, in order

| Path | Its events |
| --- | --- |
| An overrun at arrival | the release arrives with no masked region open and finds its task owing a job: `fault` (overrun, the task, `skip-late-job` or `fault`), at that release. Under `SkipLateJob` the abandoned job stamps no `complete`; the task's next `start` is the new job's. Under `Fault` nothing of the task follows. |
| A skipped job | the `fault` event with outcome `skip-late-job`, at arrival or at delivery: it is the skip. |
| A latched delivery | when a masked region closes — at its outermost `unmask`, or at its job's completion (below) — each task's latched arrivals are judged in arrival order against the task's state then (rule 1): the first is `release` if the task owes no job, else `fault` (overrun); each later one, under `SkipLateJob`, is `fault` (overrun, `skip-late-job`); under `Fault` the first overrun's `fault` (overrun, `fault`) is the last of the task's events, its later arrivals discarded. The order across tasks is the hardware's, the plan's, the port's and each service's (rule 1). |
| A completion that closes a region | `complete` of the job, then its delivery's events, then the next `start` or `resume`: the delivery precedes any task's first instruction (rule 4). The completing task, owing no job, is released afresh, its first latched arrival a `release`. |
| An escalation | a fault that is not containable produces its `fault` event at its raising, with the kept record's fields, and is the trace's last event: no release, transition, `start`, `resume` or `complete` follows (rule 7). A fault taken in the fatal handler produces none, since it never replaces the kept record. In this profile rule 3 escalates nothing (rule 3), so the record's escalation mark is `no` in every trace. |

## 4. When a hosted trace and a target trace agree

A **hosted** trace is a hosted model's, `rt-core` among them: it takes no trap, delivers a region's latched releases
inside `unmask` or the completion path before it decides, and keeps for each task the most recent latched release and
a mark that an earlier one came (Terms, rule 4). A **target** trace is the port's image's, on the emulator or a board.
The two traces of one run — one plan, the same external events at the same instants — agree on the fault paths when
each of these holds, every other event being compared as `M4.3` orders it:

1. **A delivery is compared task by task.** The tasks with events in it are the same, and for each: its first event in
   the delivery is the same, `release` or `fault` with the same outcome; the hosted trace holds no more `fault` events
   of outcome `skip-late-job` than the target, and at least one where the target does, a hosted mark judging the
   earlier arrivals it stands for as one (Terms); and the task's state after the delivery is the same — owing a job
   with the same nominal instant, owing none, or faulted. The order of events across tasks within one delivery is not
   compared: it changes no task's state (rule 1).
2. **A completion that closes a region** yields `complete`, its delivery's events, and the next `start` or `resume`
   in both, since a start or a resume is stamped at the incoming context's first instruction (§2): the target's
   decision before its delivery, and a transition preempted inside it, stamp nothing. The delivery is compared by the
   first item, and the next `start` or `resume` by its task.
3. **Interrupt pending, entry and exit, and timer programming,** are not compared between a hosted trace and a target
   trace: the hosted model takes no trap. Between two target traces they are `M4.3`'s and `M4.9`'s.
4. **A fatal fault** is compared by its kind and its §8.1 class. Its attributed task, raising context and interrupted
   task are compared where both traces raise it outside a delivery. During a delivery they differ by construction — the
   hosted model's is the task's, the target's a context's (rule 4) — and the port's fixtures judge each against the
   contract (`M4.6`), not the comparison.
5. **An external source that does not count arrivals.** Where a delivery holds two or more arrivals of a source whose
   catalog record does not state that arrivals while its request is pending or claimed are counted (rule 1), the target
   can lose one the hosted mark keeps, and the task's state can differ. That task's events, from that delivery to its
   next release observed with no region open, are **inconclusive**: neither a disagreement nor an agreement.
6. **Identity and instants.** Tasks are compared by stable logical ID; a hosted model that holds indices has the plan
   supply the IDs (§7.5). Instants are compared within `M4.9`'s tolerances; this record fixes order, not time.

## 5. `rt-core`'s transitions, mapped

| `rt-core` | Event |
| --- | --- |
| `Transition::Released { task }` | `release` |
| `Transition::Latched`, `Transition::OverrunLatched` | none: an arrival inside a region is observed at its delivery |
| `Transition::Completed { task }` | `complete`; the delivered list `complete` returns with it is the delivery of §3 |
| `Transition::JobSkipped { task }` | `fault`: overrun, the task, `skip-late-job` |
| `Transition::Faulted { task, fault }`, the task's first | `fault`: overrun, the task, `fault` |
| `Transition::Faulted { .. }` of a task already faulted | none: a discarded release |
| `Transition::Halted { fatal }`, the first | `fault`, with the kept record's fields; its §8.1 class follows from its kind, and the plan supplies the task's ID |
| `Transition::Halted { .. }` after it | none |
| `mask`, and an inner `unmask` | none; an outermost `unmask`'s delivered list is a delivery of §3 |
| `Decision::{Dispatch, Switch, Continue, Idle}` | `M4.3`'s `start`, `resume` and `preempt`, stamped as §2 says |

`rt-core` judges a task's held release before the earlier arrivals its mark stands for, the reverse of arrival order.
The first event is the same either way, being judged against the task's state before the delivery, and the later ones
are that task's overruns, so the difference reaches no event §4 compares.

## 6. What stays open

- The event set outside the fault paths, its fields there, and the orderings it allows: `M4.3`.
- The trace's format, the comparator, its instants' tolerances and the report of an inconclusive stretch: `M4.9`; the
  replay manifest's ordering rules: `M4.7`.
- The catalog fact by which an external source's record states it counts arrivals (rule 1): the source's record,
  under `M2.7.4`.

## Why

§6.3 leaves the fault paths' events to be defined and says differential comparison is of compatible behaviour, not
identical traces. The contract already decides what each path does and where a hosted model and a port may differ; this
record turns those decisions into the events a trace holds and the differences a comparison forgives, so `M4` compares
traces against a stated rule rather than one invented when the first mismatch appears.

## How to apply

- A change to a fault path's rule in the contract revisits this record's §3 and §4 in the same change.
- `M4.3` takes §2's stamping rules and fields; `M4.9` applies §4; `M4.6`'s fixtures judge what §4 leaves to them.
