# `rt-static-up-v1` — the fault paths' observation events, and when a hosted trace and a target trace agree

> Part of the profile's fault contract, [`rt-static-up-v1-faults.md`](rt-static-up-v1-faults.md), normative and
> reviewed with it: the contract's *Still open* routes here "each path's observation events and hosted/target trace
> compatibility, a discarded release's and a completion that closes a region included". Drafted by leaf `M2.15`
> (`docs/tasks/M2.md`), filed by `M2.9`'s review, finding 13. A bare rule number is the contract's; §1 to §6 are this
> record's; any other bare section number — §3.1, §6.3, §7.5, §8.1, §13.4 — is `ROADMAP.md`'s.

- **Status:** `active` — drafted `2026-10-02` by `M2.15`, under review; its rounds are in
  [`docs/reviews/rt-static-up-v1-faults-observation-reviews.md`](../reviews/rt-static-up-v1-faults-observation-reviews.md)

## In plain words

When something goes wrong — a task still busy when its next round is due, a round skipped, a task taken out of the
schedule, the whole system halted — a trace should show it, and show it the same way whether the runtime ran on the
host, in a test harness, or on the emulated or real processor. They cannot show it identically: on the host nothing
interrupts anything, the runtime keeps one "a release came while you were busy" mark where the processor sees every
release, and it decides what runs next at a slightly different moment. So this record says, for each way a fault
shows, which events a trace holds and in what order; and, for comparing a host trace with a target trace, it feeds the
host what the target saw, when the target saw it, and says what may still differ and what must not.

## 1. What this adds to §6.3

§6.3 lists the observation events — "task release/start/preempt/resume/complete, timer programming, interrupt
pending/entry/exit, fault, and selected MMIO interactions" — and asks for "compatible observable behavior, not
unconditional byte-for-byte trace identity across all targets". It names no fields and no fault path. This record
fixes, for the contract's fault paths, which of those events each path produces, the fields that tell them apart and
their order; and how a hosted run is driven by a target run for any comparison (§4), with the rule by which the two
traces agree on the fault paths. The event set in
general, its fields elsewhere and the orderings it allows elsewhere are `M4.3`'s; the comparison that applies this
record is `M4.9`'s, and the replay manifest's ordering rules (`M4.7`) name it.

## 2. The events, and when each is stamped

| Event | Stamped at | Fields |
| --- | --- | --- |
| `release` | the release's observation (rule 1): where its service judges it — at once where no masked region is open, at its delivery otherwise — which can follow its arrival by its latency (rule 6) | the task, by its stable logical ID; for a timer-released task, the release's nominal instant |
| `complete` | the job's completion — its last instruction before its completion path's first (Terms) — recorded before a release at the same instant (§13.4) | the task |
| `mask`, `unmask` | the job's call of the primitive | the task, and the depth after it |
| `start`, `resume` | the incoming context's first instruction: `start` a job's first, `resume` a preempted job's later one. A transition preempted before it — by a trap taken inside the transition after it unmasks (rule 2) — stamps neither, and nor does a completion path's context, whose task owes no job (Terms) | the task |
| `preempt` | the outgoing job's last instruction before another context's first; an abandoned job, a context never entered, a completion path's context and a job whose trap ends in a halt stamp none | the task |
| idle entry | idle's first instruction after a transition into it; a trap that returns to idle stamps none | — |
| `fault` | for an overrun, the release that observes it (rule 1); for a fatal fault, its raising (rule 7) | its §3.1 kind; for an overrun, the task, the policy's outcome, `skip-late-job` or `fault` (rule 5), and under `skip-late-job` for a timer-released task the triggering release's nominal instant; for a fatal fault, the fields fixed at its raising — the kept record's (rule 7) where its mark is complete |

On the fault paths, §6.3's interrupt entry names the context it preempted — a job's task, idle, a completion path's
context, or a transition's incoming context, which a trap taken inside it preempted (rule 2) — and its exit the
context it returns to.

Save in the Terms' window, where the port's catalog record states what a release observed there does, a release
that finds its task owing a job produces one `fault` event, kind `overrun`, and no `release` event. Under
`SkipLateJob` that release is the task's next job, with its nominal instant (rule 1), whose later `start` is an
ordinary one. A release arriving inside a masked region produces no `release` or `fault` event at its arrival: it is
observed only at its delivery (rule 1). A discarded release — any release of a faulted task (rule 5) — produces no
event. An event names a task by its stable logical ID, never an index (§7.5). A trace orders its events by their
stamps, not by when a model reports them.

## 3. Each path's events, in order

| Path | Its events |
| --- | --- |
| An overrun at arrival | the release is observed with no masked region open and finds its task owing a job: save in the Terms' window, `fault` (overrun, the task, `skip-late-job` or `fault`), at that release. Under `SkipLateJob` the abandoned job stamps no `complete`; the task's next `start` is the new job's. Under `Fault` no later job of the task starts. |
| A skipped job | the `fault` event with outcome `skip-late-job`, at arrival or at delivery: it is the skip. |
| A latched delivery | when a masked region closes — at its outermost `unmask`, or at its job's completion (below) — each task's latched arrivals are judged in arrival order against the task's state then (rule 1): the first is `release` if the task owes no job, `fault` (overrun) if it owes one, and nothing if it is faulted, its arrivals discarded; each later one, under `SkipLateJob`, is `fault` (overrun, `skip-late-job`); under `Fault` the first overrun's `fault` (overrun, `fault`) is the task's last overrun, its later arrivals discarded — save in the Terms' window, as above. The order across tasks is the hardware's, the plan's, the port's and each service's (rule 1). |
| A completion that closes a region | `complete` of the job, then its delivery's events, then the next `start` or `resume`, or idle entry: the delivery precedes any task's first instruction (rule 4). The completing task, owing no job, is released afresh, its first latched arrival a `release`. |
| An escalation | a fault that is not containable produces its `fault` event at its raising, and no release, transition, `start`, `resume` or `complete` follows it (rule 7). A fault taken in the fatal handler, or on the trap path between a fatal fault's raising and the handler's first act, produces none, since it never replaces the kept record. In this profile rule 3 escalates nothing (rule 3), so the escalation mark is `no` in every trace. |

## 4. When a hosted trace and a target trace agree

A **hosted** trace is a hosted model's; `rt-core`'s takes no trap and decides where its caller asks it to. A
**target** trace is the port's image's, on the emulator or a board. For any comparison, the hosted run is driven by
the target's:

- each `mask`, `unmask` and `complete` is called where the target's trace shows it;
- the releases each trap judged are given together where that trap ran — so a delivery's, after its region's
  `unmask` or `complete` — and a release given where the hosted model's depth is above zero is one the target judged
  inside a region, which the hosted model latches;
- a fatal fault stamped at a driven call is the hosted model's to raise, and any other fatal fault the target raised
  — a stack guard, an unexpected trap, an assertion failure a check or the port found — is given where it was raised;
- the hosted model decides at each trap's exit and after each `complete`.

The two traces then agree on the fault paths when each of these holds, every other event being compared as `M4.3`
orders it:

1. **Task by task between transitions.** Between two consecutive `start`, `resume` or idle entries, each task's
   `release`, `complete`, `mask`, `unmask` and `fault` events are the same, with the same fields save as item 5 says,
   in the same order; the order across tasks is not compared, since it changes no task's state (rule 1). A `preempt`
   is compared by its task and the `start` or `resume` that follows it, not by its place among the releases around it.
2. **Each decision against what the target ran next.** Every event stamped at a job's instruction names the context
   the target ran — `start`, `resume`, `mask`, `unmask`, `complete` — and so does an idle entry, and an interrupt
   entry names the context it preempted. After each hosted decision, the next such event in the target's trace names
   the context the hosted model chose, the running one where it chose to continue. A decision another follows before
   any such event, and an interrupt entry that preempted a completion path's context, are not compared.
3. **A completion that closes a region** yields `complete`, its delivery's events, and the next `start` or `resume`,
   or idle entry, in both: a start or a resume is stamped at the incoming context's first instruction (§2), so the
   target's decision before its delivery, and a transition preempted inside it, stamp nothing.
4. **The target's releases against the plan,** not against the hosted trace, which the target drove: until the task is
   faulted or the runtime halts, and save in the Terms' window, a timer-released task observes one release per
   nominal release due, each a `release` or an overrun `fault`, in nominal order; a task of an external source whose
   catalog record states that it counts arrivals, one per recorded arrival, in arrival order; one of a source that
   does not, at most one per recorded arrival — whether one was lost only where rule 1 lets it be is the port's
   fixtures' (`M4.6`). Each is observed no later than its nominal instant or recorded arrival plus the task's release
   latency (the composition's `J_i^release`), or, where it arrived inside a masked region, at that region's delivery,
   within `M4.9`'s tolerances.
5. **A fatal fault** is compared by its kind, its §8.1 class, whose guard was hit and its escalation mark. Its
   attributed task, raising context and interrupted task are not compared: they follow from how the port takes its
   traps (rule 2), and during a delivery from where a model delivers (rule 4), so the two records differ by
   construction where both are right. The port's fixtures judge the target's against the contract (`M4.6`), and
   `rt-core`'s tests the hosted model's.
6. **A task's fatal fault.** Where either trace's fatal fault is attributed to a task, every event from that task's
   job's `start` to the fault, save item 4's checks, is not compared: a check's or a panic's window (Terms) may have
   opened anywhere in that stretch, a release observed in it can leave the job running on where the hosted model
   abandons it, and the decisions after that differ for every task; the port's fixtures judge the stretch (`M4.6`).
7. **Identity and instants.** Tasks are compared by stable logical ID; a hosted model that holds indices has the plan
   supply the IDs (§7.5). The hosted run takes its instants from the target's, so between the two traces instants are
   not compared; item 4 checks the target's against the plan.

## 5. `rt-core`'s transitions, mapped

| `rt-core` | Event |
| --- | --- |
| `Transition::Released { task }` | `release` |
| `Transition::Latched`, `Transition::OverrunLatched` | none: an arrival inside a region is observed at its delivery |
| `Transition::Completed { task }` | `complete`; undriven, the delivered list `complete` returns with it is the delivery of §3, and driven as §4 says it is empty |
| `Transition::JobSkipped { task }` | `fault`: overrun, the task, `skip-late-job` |
| `Transition::Faulted { task, fault }`, the task's first | `fault`: overrun, the task, `fault` |
| `Transition::Faulted { .. }` of a task already faulted | none: a discarded release |
| the halt's first report — `Err(Refused::Halted)` from a `mask` or `unmask` of a runtime not yet halted, `Decision::Halt` from the `decide` that raised it, or `Transition::Halted { fatal }` | `fault`, stamped at that call, with the fields `rt-core` holds: the kind (`InvariantViolated` is the contract's assertion failure), whose guard, the escalation mark, and the attribution, which §4 does not compare; the §8.1 class follows from the kind, the plan supplies the tasks' IDs, and `rt-core` keeps no raising context |
| any report of the halt after the first | none |
| a `mask` or `unmask` that returns `Ok` | `mask` or `unmask`, with the depth it returns; undriven, an outermost `unmask`'s delivered list is a delivery of §3, and driven it is empty |
| `Decision::{Dispatch, Switch, Continue, Idle}` | `start`, `resume`, `preempt` and idle entry, stamped as §2 says: `Dispatch` and `Switch` name only the incoming task and `Continue` no event, so the harness, from each task's `Released`, `JobSkipped` and earlier decisions, says start or resume; an `Idle` after an `Idle` stamps none |

A `complete` called with no job, or another task's, running panics: the hosted model has no fault for a completion
path entered by a non-job (Terms), which the port's fixtures judge (`M4.6`). Driven as §4 says, the hosted model is
given a delivery's releases after the region's close, so its latch holds none and its mark stands for no arrival.
Driven otherwise, its latch keeps the most recent release and a mark that an earlier one came, and judges the held
release before the arrivals its mark stands for, as one (rule 1, Terms): what §4 compares assumes the driving, not
that.

## 6. What stays open

- The event set outside the fault paths, its fields there, and the orderings it allows: `M4.3`.
- The trace's format, the comparator, the hosted run's driving as §4 states it, and its instants' tolerances:
  `M4.9`; the replay manifest's ordering rules and recorded external events: `M4.7`; the fixtures §4 leaves to them:
  `M4.6`.
- The catalog fact by which an external source's record states it counts arrivals (rule 1): the source's record,
  under `M2.7.4`; and a periodic task's first release instant, which §4's plan check needs: the timer-service record,
  under `M2.7.4`.

## Why

§6.3 leaves the fault paths' events to be defined and says differential comparison is of compatible behaviour, not
identical traces. The contract already decides what each path does and where a hosted model and a port may differ; this
record turns those decisions into the events a trace holds and the differences a comparison forgives, so `M4` compares
traces against a stated rule rather than one invented when the first mismatch appears.

## How to apply

- A change to a fault path's rule in the contract revisits this record's §3 and §4 in the same change.
- `M4.3` takes §2's stamping rules and fields; `M4.9` applies §4; `M4.6`'s fixtures judge what §4 leaves to them.
