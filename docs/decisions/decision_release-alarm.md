# The release alarm: which device releases tasks on a target, and what a timer route costs

- **Type:** `decision`
- **Date:** `2026-11-06`
- **Status:** `active` — written by leaf `M4.1.6`; nothing here is yet code, and its first review is `M4.1.6.1`
- **External sources:** none: this record decides how the engine reads its own records
- **Owner / source:** leaf `M4.1.6` (`docs/tasks/M4.md`), filed by `M4.1.1`'s round-3 answer and widened by `M2.30`'s review (R10); `ROADMAP.md` §6.2, §7.3 and §7.4; the kept question R3-1 of `docs/reviews/decision_runtime-plan-reviews.md`, and rounds 21 and 26 there.

## The fact / decision

**In plain words.** On the playground the simulator's clock releases tasks, so no device is involved. On a real machine something must raise the interrupt that releases them, and that something is a device like any other. So the kernel becomes one more requester of the system: it asks for an alarm, and the search binds it to whichever device can serve, under the same rules and the same count of channels as anything else that asks. A second alarm, an adapter's say, is then no special case: it takes a second channel, on that device if it has one or on another. Each such alarm's interrupt is charged as any other interrupt source already is, since the analysis was proved for sources and says so of timekeeping interrupts by name. And a job never runs inside an interrupt, so a job gets told of an alarm in the one way a job ever learns anything: a task is released.

### 1. The release timer is a requester

Where a system has a task whose trigger is `timer` and runs on a target, the engine adds one requester, the **release requester**, with these requirements, read and judged as any requester's (model §8):

```text
(needs relative-delay counter-modulus tick-rate observation-coherent)
(unambiguous-horizon (at-least LONGEST))
```

`LONGEST` is the longest `period` of a timer-released task. The horizon is the release timer's own: the analysis's condition 6 takes a timer interrupt "only when a release is due", so no trap may fall between two releases to re-arm it, and one compare must reach from a release to the next. The longest gap between due releases is at most the longest period.

- **It takes a channel** (model §8 rules 2 and 3), so what shares a device with it is judged with no new rule. A device of one channel cannot release tasks and serve an adapter's alarm, and that is the refusal any two requesters get. Its repair is another device or a device with more channels, since `/1`'s catalog has no multiplexer (the adapters record's §2).
- **Which device** is what the search binds: the policy's order, then its stable one. Nothing here prefers a device, for the reason `decision_device-correspondence.md` gives of matching by what is asked. So it is never "the target's tick-rate block", R3-1's rejected fix: a counter may raise no interrupt, and `relative-delay` is what says this one can.
- **Its bound names no description's device** (`decision_device-correspondence.md` rule 2): it is judged on the target, and a plan names the target's device.
- **On the playground there is no release requester.** The release timer stays the simulator's exact clock, written `simulator` (the runtime plan's §2.5, `RTP-H9`). Nothing in `/1` moves.
- **What a target must state.** `relative-delay`, `channels` and the counter's facts, each with what checks it: `DC-H1`'s obligation, and a target stating them passes as any provider does. A device whose compare is not level-triggered is the variant's condition 6, its platform fact, and refused there.

### 2. Each alarm's interrupt is a declared source

A timer route is charged as a source: an entry of the analysis's `Source`, with `C_s`, `T_s`, `J_s`, an acknowledge point and a priority. This is round 21's better decision, and the variant already says it: "Counter-wrap, timekeeping, overrun and watchdog interrupts … are sources like any other". So nothing in the recurrence, the composition or the admission conditions changes, and no proof is reopened.

- **`T_s`** is its arrival bound, owed by the realization with the bound's §7.3 record. For a `wake`, the update period the realization states. For an `expiry`, what the requester's programming allows: at most one arrival per deadline delivered and per re-arm (the adapters record's §2). The plan writes `programmed` until a realization states it, and a route so written leaves the analysis inconclusive and not refused.
- **`C_s`** is one service, entry to exit: the code the route runs, a generated service's cost as any other, owed by the realization. Where the route runs a node (model §8 rule 4), that node's code is inside it.
- **`J_s`** is composed as any source's: `Δ_s` (the composite-inputs record's §3).
- **The acknowledge point** is owed by the realization, at entry or at exit, as for a source.
- **Condition 5's paid-for rule binds it**: a trap taken runs one service, paid for by an arrival, whose one claim takes a request. So an alarm reaches the core as a request at the controller, which a claim takes. An alarm on a line with no claim, as a hart-local timer interrupt has, claims nothing and is refused by that condition. That is no new limit, since condition 5 says so of "a source whose service claims nothing". On the simulator its trap claims the request (`RA-H3`).

The condition the variant's `Source` cannot state is what the alarm was for. Two follow from the adapters record's obligations, and the scheduling checker holds them once `J_s` is known (`RA-H2`):
- `delivery`: an expiry's `J_s` does not pass the `delivery-bound` the instance offers;
- `update`: a wake's `T_s + J_s` does not pass the provider's `unambiguous-horizon`, since an epoch is correct only while it is read once in every wrap.

**Why not a timer event of the release timer**, under an amended condition 6. That was the first candidate, for a `wake` above all. Condition 6 reads "A timer interrupt is taken only when a release is due", and the timer term counts releases, so a wake with no release due is charged nowhere: the amendment would reopen what `M2.11`'s review closed, for a saving of one interrupt line.

### 3. The order

A route's rank goes by the deadline it has to meet, ascending:

- a source's: its stated `min-arrival-separation`, the no-loss limit of a source acknowledged at entry, as today;
- an expiry's: the `delivery-bound` its instance offers;
- a wake's: its provider's `unambiguous-horizon`.

Ties go by the interrupt's text, and what states no value goes last, as today. So an expiry that must be delivered soon stands ahead of a slow source, which ranking timer routes last, `/1`'s stopgap, would not allow. It is deadline-monotonic over what the plan already holds, with no fact owed. ⚠️ It is a choice of order and no claim of optimality: the scheduling checker judges whatever the plan orders, and the Audsley search `RTP-H4` asks for stays `M2.21.2`'s.

`/1`'s ranks stand, since `/1` is a frozen format: this is `/2`'s (`RA-H1`).

**Against the release timer** a timer route stands where any source stands, the hardware's fact `external-before-timer` deciding it (the composite-inputs record's §1 and §2), since it is declared as one and reaches the core through the controller. On the simulator that is `yes`, every route before the release timer, which is what `RTP-H9` has it do today. So `RTP-H9`'s "until `M4.1.6` places one" is answered by what it already does.

### 4. A node's call cost

A job's call into a node's generated code is a call of a primitive. The composition's parts already say all it needs: `api.p` is "one call, from its first instruction to its last", `masked.p` "the longest contiguous masked stretch inside one call", and `n_{i,p}` the most one job makes. So a node `N` a job calls gives the composition `api.N` and `masked.N`, stated by the realization and not by the catalog, and `n_{i,N}` from the application's inputs (`RTP-H5`). §3's formulas do not change. The composite-inputs record's §7, which left such a task's composites undeclared until this design, gives way to it.

- **A span.** A node's call cost is a span, so it holds what the node calls in turn. A node a job reaches only through another owes no call cost of its own: no job's count would multiply it, and it is charged inside its caller's. This answers the runtime plan's R26-R2. `call-cost` is owed for each node a job's task line or `bind` lines name, and the realization derives each from what it calls.
- **A multiplexed node's overhead has no term of its own.** A multiplexer's work for a user is inside the call that user makes, its `api.N`, and its work for an alarm is inside that route's `C_s`. A term beside them would count it twice. The runtime plan's `multiplex` row said "the term `M4.1.6` designs": this is that design, and what it owes is the statement of those two bounds (`RTP-H2`).
- **Its masked runs in a job are a job's.** The fault contract has a `mask` a job's when "the job's own code, or a primitive it called, executes it", and a generated node a job calls is a primitive here. So a `mask` in a node called from a job is that job's, and one in a node a route runs is the service's. This answers `M2.30`'s R10.
- **What runs in two contexts states both.** A node a job calls and a route runs gives `api.N` for the job and stands inside `C_s` for the route. Its shared data is condition 4's, inside declared masked sections, whichever runs it.

### 5. How a job receives an alarm

**A job receives an alarm only as a release: a task is released by it.** No context runs a job's code in an interrupt, and the variant's "Work a service defers runs as a declared task". So what `/1` refuses, a task that would receive an interrupt's code, has one meaning, and it is the one any interrupt has:

- A route's service releases the tasks declared `released by` it, as a source's does. A task reading an expiry is a task released by that alarm: another task, or the one that programmed it, its next job. Its condition is the variant's: a task released on every arrival has `T_i ≤ T_s`.
- **`/1`'s refusal stands until it can be said.** A task requester bound to an alarm directly, or to an `expiry` instance, is still refused. What it lacks is a way to write the release: a trigger naming the alarm, as `(source X every)` names a source, and the arrival bound it reads. That is `RA-H4`'s, with `M4.12`, and the refusal's repair then says so.
- **The system's own level has no such meaning, and stays refused.** It has no job, and runs nothing after initialisation, which runs masked and never waits on an interrupt (the runtime plan's §2.7). So a requirement for an alarm or an expiry at the system's own level has nothing to receive it. The repair is to move it to a task or a service.

**Hand-offs.** Each is carried, word for word, by the leaf it names.

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RA-H1` | `M4.1.7` | Format `/2` adds the release requester of `decision_release-alarm.md` §1 to a system on a target, its binding lowered as the release timer, and ranks routes by that record's §3, each timer route's rank by the deadline it has to meet. |
| `RA-H2` | `M2.21.2` | The scheduling checker declares a source for each timer route, an identifier of its own kind, with the realization's `C_s`, `T_s` and acknowledge point and the composition's `J_s`; a route written `programmed` leaves the set inconclusive; and it holds a route's `J_s` within its instance's `delivery-bound` for an `expiry`, and `T_s + J_s` within its provider's `unambiguous-horizon` for a `wake`. |
| `RA-H3` | `M4.3.5` | The simulator raises an alarm as a request at its controller, which the route's trap claims, one request per arrival, and takes each route where a source stands against the release timer. |
| `RA-H4` | `M4.12` | A task can be released by an alarm or an expiry: a trigger naming it and the arrival bound it reads, the runtime plan's refusal of a task requester giving way to it, and a repair saying so until then. |
| `RA-H5` | `M2.21.2` | A node's `api.N` and `masked.N`, stated by the realization, reach the composition as a primitive's do, for each node a job's task line or `bind` lines name; and the composite-inputs record's §6 reads a missing one as it reads a primitive the catalog does not cost. |

## Why

- **One rule for everything that asks.** A release timer with rules of its own would be a second account of channels, device choice and refusal, each to be kept in step. The kernel asking as anything else asks needs no new judgement, and the joint rules, the checker and the correspondence all hold of it unchanged.
- **Reuse a proof rather than amend one.** The variant's source term has been reviewed, fixture by fixture, against the failure this design would otherwise have to rule out again.
- **A deadline, since that is what an alarm has.** A source's rank by separation is deadline-monotonic where its deadline is its no-loss limit, so one key covers both.

**Alternatives weighed.**

- **The target's tick-rate block**, R3-1's. A counter may raise no interrupt, which is why this reads `relative-delay`, and it would ignore channels: R3-3.
- **Release by a named device in the target's file.** A target saying "this one releases" is a fact the target's agreement cannot check, and a target's file holds what its agreement checks (`decision_target-platform-description.md`).
- **A wake as a timer event of the release timer.** §2.
- **Timer routes last, as `/1` has them.** An expiry due soon could wait behind every slow source, past its delivery bound, though the plan holds what would rank it.
- **A term of its own for a multiplexer's overhead.** It runs inside a call or a service, each already charged whole.
- **A node's cost per callee, added up by the composition.** It would need the call graph inside the realization, which a description never states, and a span is what the parts already mean.
- **A job's code run in the interrupt.** It is what §5 rules out, and it would take condition 5's "Work a service defers runs as a declared task" with it.

## How to apply

- Nothing here changes `/1`, its goldens or a simulator run. Each change is in a hand-off above.
- ⚠️ **What this does not decide.** Whether a port can realize an alarm as a request a claim takes is `M4.1.7`'s and the port record's: on `riscv-virt-up` the CLINT's one comparator is one channel on a hart-local line with no claim, so a system there has the release timer alone and no timer route, which is what the joint rules and condition 5 each say.
- ⚠️ A system with a timer route is still refused or inconclusive until `M2.21.2` and a realization exist: this says how it will be charged, and charges nothing.
