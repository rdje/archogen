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
their order, and the rule by which a hosted trace and a target trace of one run agree on them (§4). The event set in
general, its fields elsewhere and the orderings it allows elsewhere are `M4.3`'s; the comparison that applies this
record is `M4.9`'s, and the replay manifest's ordering rules (`M4.7`) name it.

## 2. The events, and when each is stamped

| Event | Stamped at | Fields |
| --- | --- | --- |
| `release` | the release's observation (rule 1): where its service judges it — at once where no masked region is open, at its delivery otherwise — which can follow its arrival by its latency (rule 6) | the task, by its stable logical ID; for a timer-released task, the release's nominal instant |
| `complete` | the job's completion — its last instruction before its completion path's first (Terms) — recorded before a release at the same instant (§13.4) | the task |
| `start`, `resume` | the incoming context's first instruction: `start` a job's first, `resume` a preempted job's later one. A transition preempted before it — by a trap taken inside the transition after it unmasks (rule 2) — stamps neither, and nor does a completion path's context, whose task owes no job (Terms), nor the context in which the runtime completes a primitive on an abandoned job's behalf (rule 5) | the task |
| `preempt` | the outgoing job's last instruction before another context's first; an abandoned job, a context never entered, a completion path's context and a job whose trap ends in a halt stamp none | the task |
| idle entry | idle's first instruction after a transition into it; a trap that returns to idle stamps none | — |
| `fault` | for an overrun, the release that observes it (rule 1); for a fatal fault, its raising (rule 7) | its §3.1 kind; for an overrun, the task, the policy's outcome, `skip-late-job` or `fault` (rule 5), and for a timer-released task the triggering release's nominal instant; for a fatal fault, the fields fixed at its raising — the kept record's (rule 7) where its mark is complete — and, for an assertion failure, which of §4 item 5's cases it is, or none |

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

A **hosted** trace is a hosted model's, `rt-core`'s among them; a **target** trace is the port's image's, on the
emulator or a board. A comparison gives the hosted run the target's release observations — each release where the
target judged it, a delivery's together after its region closes and before the hosted model decides — each job's
completion where its `complete` is stamped, before a release at the same instant (§2), and each fatal fault the
target raised that the hosted model cannot raise itself — among them a stack guard, an unexpected trap, a panic, a
failed check of the port's, of a catalog record's or of generated code, and a runtime check beyond item 5's — where
the target raised it; how it drives the hosted run, and where the hosted model decides, are `M4.9`'s, taking the port's catalog facts — among them
`decision-placement` and `services-preempt-completion-interval` (the catalog record's §14.4) and `one-claim-per-trap`
(its §12) — as its parameters. For that, the target's trace shows each `mask` and `unmask` that runs — that changes the depth or
raises an assertion failure — and where each trap begins and ends, which `M4.3`'s event set holds. A call whose job
rule 5 abandons first, at its entry or in the Terms' window, raises nothing (the contract's second smaller decision):
it is not recorded and not driven, the release that abandoned the job being given instead, and whether the port let
it be abandoned there is the fixtures' (`M4.6`). The two traces then agree on the fault paths when each of these
holds:

1. **Each judgment.** Every release the target observed is judged the same in both: a `release`, an overrun `fault`
   with the same outcome and fields, or nothing, discarded.
2. **The next task after a completion that closes a region.** After `complete` and its delivery, the next `start` or
   `resume`, or idle entry, is the same in both, though the target decided before its delivery and a hosted model
   after it: both reach the same schedule (rule 4), and a transition preempted before its incoming context's first
   instruction stamps nothing (§2).
3. **No abandoned job runs on.** After an overrun `fault` with outcome `skip-late-job`, the task's next entry is a
   `start`, never a `resume` of the abandoned job; after one with outcome `fault`, the task is never entered again —
   save in the Terms' window.
4. **The target's releases against the plan,** not against the hosted trace, which the target drove: until the task is
   faulted or the runtime halts, and save in the Terms' window, a timer-released task observes its nominal releases
   in nominal order, none twice and none skipped, each a `release` or an overrun `fault` — under `Fault` up to the one
   that faults it, the later ones discarded; a task of an external source whose catalog record states that it counts
   arrivals, its recorded arrivals likewise, in arrival order; one of a source that does not, at most one per recorded
   arrival — whether one was lost only where rule 1 lets it be is the port's fixtures' (`M4.6`). None is observed
   while a masked region is open, or before its nominal instant or recorded arrival. How late one may be observed is
   the timing analysis's to bound and `M4.9`'s to check: the composition's `J_i^release` bounds when the service that
   releases the task starts, not when it judges the release.
5. **A fatal fault.** The runtime's own assertion failures — the depth bound, the hosted model's `M` being the target
   runtime's declared one; an `unmask` at depth zero; a `mask` or `unmask` with no job running in the runtime's record
   (the Terms' case, a call from another context over a preempted job being the port's check, given); a dispatch or
   resume with a region open — are the hosted model's to raise at the calls and decisions `M4.9` drives it through,
   and one in one trace only is a disagreement, the target's naming its case (§2); for one in both, its
   kind and §8.1 class are compared. Every other fatal fault is given (above), and whether it was due in that run is
   not compared: a known limit, the fixtures judging each kind in its own scenarios (`M4.6`). A fatal fault's
   attributed task and interrupted task are not compared, since they follow from how the port takes its traps, and
   during a delivery from where a model delivers, so differ by construction between a hosted model and a port (rules
   2, 4); nor whose guard of a given fault, which the hosted model copies; the escalation mark is `no` in every trace
   (§3). The fixtures
   judge the target's record against the contract (`M4.6`), and `rt-core`'s tests the hosted model's. After a fatal
   fault, neither trace shows a `release`, `start`, `resume`, idle entry or `complete` (rule 7).
6. **A task's fatal fault.** Where either trace's fatal fault is attributed to a task, every event from that task's
   job's `start` to the fault, save item 4's checks, is not compared: a check's or a panic's window (Terms) may have
   opened anywhere in that stretch, a release observed in it can leave the job running on where the hosted model
   abandons it, and every decision after it can differ; the port's fixtures judge the stretch (`M4.6`).
7. **What is not compared on the fault paths.** Interrupt pending, entry and exit, and timer programming, which drive
   the hosted run, are not compared with it: the hosted model takes no trap. Tasks are compared by stable logical ID,
   a hosted model that holds indices having the plan supply them (§7.5); instants are not compared between the two
   traces, since the hosted run takes the target's, and item 4 checks the target's against the plan. Which context
   runs next outside items 2, 3 and 5 is `M4.9`'s comparison, not a fault path's.

## 5. `rt-core`'s transitions, mapped

| `rt-core` | Event |
| --- | --- |
| `Transition::Released { task }` | `release` |
| `Transition::Latched`, `Transition::OverrunLatched` | none: an arrival inside a region is observed at its delivery |
| `Transition::Completed { task }` | `complete`; the delivered list `complete` returns with it is the delivery of §3, empty where a delivery's releases are given after its region closes (§4) |
| `Transition::JobSkipped { task }` | `fault`: overrun, the task, `skip-late-job` |
| `Transition::Faulted { task, fault }`, the task's first | `fault`: overrun, the task, `fault` |
| `Transition::Faulted { .. }` of a task already faulted | none: a discarded release |
| the halt's first report — `Err(Refused::Halted)` from a `mask` or `unmask` of a runtime not yet halted, `Decision::Halt` from the `decide` that raised it, or `Transition::Halted { fatal }` | `fault`, with the fields `rt-core` holds: the kind, with its `invariant` or `cause` (`InvariantViolated` is the contract's assertion failure, its `invariant` naming §4 item 5's case), whose guard — a task's or the interrupt stack's, another guarded stack's (`guarded-stacks`) taken from the target's record — the escalation mark, and the attribution — §4 gives the fault rather than comparing it; the §8.1 class follows from the kind, the plan supplies the tasks' IDs, and `rt-core` keeps no raising context |
| any report of the halt after the first | none |
| a `mask` or `unmask` that returns `Ok` | none on the fault paths; an outermost `unmask`'s delivered list is a delivery of §3, empty where a delivery's releases are given after its region closes (§4) |
| `Decision::{Dispatch, Switch, Continue, Idle}` | `start`, `resume`, `preempt` and idle entry, as `M4.3` stamps them for a hosted model; `Dispatch` and `Switch` name the incoming task, not whether it starts or resumes, which the harness knows from the task's `Released` and `JobSkipped` and the contexts its trace has shown entered |

Given a delivery's releases after its region closes (§4), the hosted model judges each against the task's state then,
so its latch holds none and its mark stands for no arrival. Given them inside the region, its latch keeps the most
recent release and a mark that an earlier one came, and judges the held release before the arrivals its mark stands
for, as one (rule 1, Terms): §4's first item assumes the former. A `complete` called with no job, or another task's,
running, before a halt, panics; driven as `M4.9` drives it, that is a disagreement on what runs, `M4.9`'s to report.

## 6. What stays open

- The event set outside the fault paths, its fields there, and the orderings it allows: `M4.3`.
- The trace's format, the comparator, how it drives the hosted run and where the hosted model decides, its instants'
  tolerances, and how late a release may be observed — observations matched to releases in order, against a bound
  the timing analysis states: `M4.9`; the events the trace holds besides these, each `mask` and `unmask` that runs and each trap among them, and their
  order: `M4.3`; the replay manifest's ordering rules and recorded external events: `M4.7`; the fixtures §4 leaves to
  them: `M4.6`.
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
- `M4.3` takes §2's stamping rules and fields, and records each `mask` and `unmask` that runs and each trap; `M4.9` applies §4; `M4.6`'s fixtures
  judge what §4 leaves to them.
