# The release alarm: which device releases tasks on a target, and what a timer route costs

- **Type:** `decision`
- **Date:** `2026-11-06`
- **Status:** `active` — written by leaf `M4.1.6`; nothing here is yet code but the executable model that holds its claims, `crates/rt-analysis/tests/alarm_model.rs`, and it is under review in `docs/reviews/decision_release-alarm-reviews.md`
- **External sources:** none: this record decides how the engine reads its own records
- **Owner / source:** leaf `M4.1.6` (`docs/tasks/M4.md`), filed by `M4.1.1`'s round-3 answer and widened by `M2.30`'s review (R10); `ROADMAP.md` §6.2, §7.3 and §7.4; the kept question R3-1 of `docs/reviews/decision_runtime-plan-reviews.md`, and rounds 21 and 26 there.

## The fact / decision

**In plain words.** On the playground the simulator's clock releases tasks, so no device is involved. On a real machine something must raise the interrupt that releases them, and that something is a device like any other. So the kernel becomes one more requester of the system: it asks for an alarm, and the search binds it to a device that can serve, under the same rules and the same count of channels as anything else that asks. A second alarm, an adapter's say, takes a channel of its own, on another device than the kernel's, since the two reach the core differently. Each such alarm's interrupt is charged as any other interrupt source is, since the analysis was proved for sources and says so of timekeeping interrupts by name. What it must be told is how close two arrivals can come, and no job's choice of deadlines is left to decide that: the code programming an alarm holds its compare a least distance past the last arrival, so the figure is true whatever a job does. And a job never runs inside an interrupt, so a job hears of an alarm when a task is released.

### 1. The release timer is a requester

Where a system runs on a target, and has a task whose trigger is `timer`, the engine adds one requester, the **release requester**, named by the system, with these requirements:

```text
(needs relative-delay counter-modulus tick-rate observation-coherent)
(available-in-state idle)
(alarm-wiring core)
(unambiguous-horizon (at-least SHORTEST))
```

- **`SHORTEST`** is the shortest `period` of a timer-released task, rounded up to a tick wherever it enters a sum of lags, where the variant rounds a separation down. The analysis's condition 6 takes a timer interrupt "only when a release is due", so no trap may fall between two releases to re-arm it, and one compare must reach from a release to the next. The next due release is at most the shortest period away, whichever task it is, since every timer task is released from time zero at its own nominal releases. A long period does not ask more: the service runs at the shortest, and counts wraps there.
- **That is necessary, not sufficient.** A compare over a modular counter cannot tell a value it has passed from one a whole wrap ahead, so with `λ` the most from a due-check to the compare's write landing, `SHORTEST + λ` must lie within the horizon: `a_modular_counter_tells_a_deadline_past_from_one_ahead_only_within_its_modulus` holds that, over each modulus, reach and lag it tries, as exactly the condition. `λ` is bounded by `J^release + C_rel`, which only the analysis knows, so the scheduling checker holds it (`RA-H2`). A device that passes the search and fails there is refused there, naming that.
- **`available-in-state idle`.** The idle context waits for the next interrupt, so a timer that stops in idle loses every release after it (`ROADMAP.md` §6.2's "clock-state dependencies").
- **`alarm-wiring`** is a fact this record adds: how a device's alarm reaches the core, `core`, a hart-local timer line a trap takes with no claim, or `controller`, a request at the interrupt controller that a claim takes. Condition 5 asks exactly this of the two: "a timer trap the timer's, claiming nothing", and any other trap's one claim. One block states one wiring, so a machine with both describes two blocks. Without this fact a second channel repairs nothing where the wiring is uniform, and the search's first pick could give the release timer the device an alarm needs.
- **It takes a channel** (model §8 rules 2 and 3), and so the release timer and an alarm are on two blocks by their wiring, whatever each one's channels. What does share a device, two alarms on one wired to the controller, is judged with no new rule. There `channels` counts lines to the controller, so a device of two comparators on one line states `channels 1`, and a second alarm's repair is another device.
- **Which device** is what the search binds, by the policy's order and then its stable one. So it is never "the target's tick-rate block", R3-1's rejected fix: a counter may raise no interrupt, and `relative-delay` is what says this one can.
- **What the joint rules need said.** Rule 1 gains this requester, rules 7 and 12 name it among what a system serves, rule 8 gives no route for its binding, since the release timer is no route, and rule 9 reads it as a caller at the system's level. Those lines land with the code (`RA-H1`), as do the assignment's format and the checker's own reading of it (`DC-H2`). The record supplying `C_rel` is the one whose code programs the device bound, and the analysis reads the compare's facts of that device.
- **On the playground there is no release requester.** The release timer stays the simulator's exact clock, written `simulator` (the runtime plan's §2.5, `RTP-H9`). Nothing in `/1` moves.
- **With no timer-released task on a target** there is no release requester and no device: `/2` writes `(timer none)`, and the timer's interrupt is never enabled. "Bound and never due" does not hold of a modular compare, whose largest value comes round within a wrap, and a trap finding no release due is an unexpected trap (the fault contract). So the variant owes this case its own reading, since it now declares a timer of every set: no `C_rel`, `ρ` or condition 6's facts are asked, and the timer term is zero, which it is, no task being released by it (`RA-H2`).
- **What a target must state:** those facts and `channels`, each with what checks it, which is `DC-H1`'s obligation. A compare that is not level-triggered is the variant's condition 6, its platform fact, and refused there.

### 2. Each alarm's interrupt is a declared source

Each requester that takes a channel for an alarm has a declared source of its own: an entry of the analysis's `Source`, with `C_s`, `T_s`, `J_s`, an acknowledge point and a priority. This is round 21's better decision, and the variant already says it: "Counter-wrap, timekeeping, overrun and watchdog interrupts … are sources like any other". So the recurrence, the ceilings and the admission conditions do not change, and what the sources' proofs rest on is what an alarm must then be shown to meet.

**One source per alarm, not per route.** Model §8 rule 8 reports one route for each provider and kind, and a device of two channels gives one route two alarms, whose arrivals can coincide: no one separation describes them, and a merged one could be zero, which condition 3 refuses. So `/2` writes one line per alarm, which rule 8 and the runtime plan's §2.5 and §2.9 say of each alarm where they speak of a route (`RA-H1`), and an alarm needs a line of its own, as a request of its own: `one-request-per-arrival` is owed of each, and `channels` counts those lines (§1).

**On a target an alarm also asks what §1's reasons ask**, the engine adding it, so an adapter's text does not change and neither does anything on the simulator, which has no idle and no wiring. `(alarm-wiring controller)` joins the requirements of the requester that takes the channel, with `counter-modulus tick-rate observation-coherent` where it does not state them, since the code that writes a compare reads the time; those and `relative-delay` are facts of the function `counter`, so model §8 rule 13 binds them to one block, and the wiring is that block's. That its device runs in idle, and what kind of alarm it is, are asked of the device bound and of nothing else, by the joint rules (`RA-H1`): `available-in-state` as a requirement's qualifier is judged at everything the requester reaches (model §8 rule 13), which would refuse an alarm's service over its console. A device's kind is a level compare or a delay. An edge compare is refused, since "raised at the write and never lost" rests on a level, and an edge raises nothing until the count comes round.

**What each needs, and who owes it:**

| Input | For a `wake` | For an `expiry` | Owed by |
| --- | --- | --- | --- |
| `T_s`, the **least** separation between two arrivals | the floor below, the update period | the floor below | the realization, which holds it in the code that writes the compare |
| `P`, the step its service programs from one point to the next, rounded up | the update period | none | the realization |
| `R`, the farthest ahead of a read a deadline stands, chained or not, where the time base is the counter itself | `P` | at least what its requirement asks by `supported-horizon` | the realization |
| `ρ`, and its device's kind | of its own device | of its own device | the target's statement (`RA-H1`): the variant's `ρ` and condition 6 are the release timer's, and an alarm is on another device |
| `λ`, the longer of two: from reading the time to the write landing, and from the landing to the read after it | the re-arm's | the program's | the realization |
| `C_s`, the acknowledge point, deferred work, `no-application-code`, `one-request-per-arrival`, whether it reaches the controller | as a source's | as a source's | the realization, with the bound's §7.3 record |
| `J_s` | `Δ_s` | `Δ_s` | the composition (§3 there) |

**The floor: `T_s` is enforced, not assumed.** A rate is no separation: "one arrival per deadline delivered" says how many, and the variant's `T_s` is how close. A job of period 100 arming one deadline per job can place two arrivals 50 apart, or 10, by which deadlines it picks and when it runs. Nor can an owner state the least one: the last segment of a chained delay can arrive just after the re-arm, a deadline due at its write arrives at that write, and each hangs on a response time. So the code that writes a compare holds it past the last arrival:

```text
compare := max(deadline, last_arrival + T_s)
```

each write, a job's and a service's, a segment of a chain and a wake's next point, `P` past the one before, among them. Two arrivals are then never closer than `T_s`, whatever a job programs, with no input from the application. What it costs is a deadline's lateness, which `delivery` holds. `T_s` is the realization's figure, judged at that value: too small and the load fails the analysis, too large and `delivery` fails. Choosing it is no search here.

**What the code can know.** `last_arrival` is no value any code holds at a write: an arrival happens when it happens, the read of the time is `λ` before the landing, and a compare standing may match in between. So the code bounds it from above, in a masked run, by what it read and what it wrote:

- **In a service**, the compare standing, if at or before the time it read, is the one that raised it, spent now, and the last arrival is at most that value.
- With `r` the time it read, a compare standing that may match by `r + λ` is **left**, and the service it raises picks up what is pending. Rewriting it could put two arrivals closer than the floor.
- Otherwise it writes the earliest deadline pending, floored and then rounded up to the compare's resolution, `v`, and after the landing, still masked and within `λ` of it, reads the time again. A `v` at or before that read has arrived or will at once, so the last arrival is at most that read; a later `v` arrives at `v`, on a level compare. With nothing pending it disarms the compare, or where it cannot, stands a value far enough ahead, whose arrival is paid for as any.
- A delay-only device is programmed a count, `value − r` rounded up to its resolution, from the read and never early, and lands late by its own lag, so its arrival is at most `r + count + λ`, which is the bound the code holds. It knows the count it wrote, so `ρ` does not enter that bound, and is charged once, in the sums below.
- **Where the time base is the counter itself** each of these reads is modular, and each window is its own. A deadline stands ahead of a read within `R`, and past it beyond. The floor stands ahead only within `T_s + λ` of the read, and beyond that it is past and holds nothing back: an alarm may idle for wraps, and judged within `R` an arrival long gone would hold a deadline back by up to `R`. An arrival a whole wrap gone that reads as recent holds one back within that window, which `delivery` pays.

`Know::WhatItReadAndWrote` is that protocol in the model, as it reads here. The read after the landing is drawn within `λ`, and a read farther than that breaks `delivery`, which is why `λ` covers it. A floor read off the value last written, the first thing one would write, does not hold: arrivals 4 apart, and a response of 104 against a bound of 63, where the protocol holds the same set at 47, arrivals 17 apart.

**This is held by a model, and not by reading.** `crates/rt-analysis/tests/alarm_model.rs` runs the variant's timeline, unit by unit, and `compose` and `analyze` as they stand: masked runs contiguous with their decided switch as `L` reads them, `δ`, `γ` and a compare's resolution, `W_wake`, several alarms, tasks an alarm releases, and a read apart from a landing. Over 1,400 timelines of seeded random sets, each job programming as it likes, from past due to two periods ahead: arrivals are never closer than the floor, none is lost, no response passes the analysis's bound and no deadline passes `delivery`. With the floor off, the review's chain holds at a bound of 2,406 and misses on the timeline at 2,989, arrivals 10 apart. A timeline is no realization, and has no initialisation and no modular hardware of its own.

**Condition 5's paid-for rule binds it**: a trap taken runs one service, paid for by an arrival, whose one claim takes a request. So on a device wired to the core an alarm is refused by the search. On the simulator its trap claims the request (`RA-H3`).

**What the composition gains.** The record's roles allow a device's registers only to the service of its own source, so a node programming its alarm from a job has no role today, and that fact gates every composite. It gains one: a node that takes a channel programs that channel's compare, and any node bound to that device reads its counter, from a job inside its own masked run or from a service that calls it, and that alarm's service alone claims and completes it. Rule 14 binds every other requester of a deadline to that device, a `time.monotonic` that takes no channel among them, and rule 4 makes a source's handler a user, so a node is called from a route's service that is not its own. A wake is first armed at the port's transition to run time, the last masked step before the first enabling of interrupts, for a value `P` past what it reads, which that step's own length must not pass: an arrival during initialisation would wait out a masked run `L` does not hold, and no source arrives before that enabling (the composite-inputs record's §2). The runtime plan's §2.7 calls programming an alarm work for run time, which holds of `/1`, whose emitter generates no such call, and `/2` amends it for a wake (`RA-H1`, `RA-H6`), since nothing else would ever arm one. Its `service.<source>` is anchored by the realization and not by a catalog record (`RA-H5`). What else in the record's §1 to §3 this touches is for `RA-H5` to say when it builds them.

**What the variant's `Source` cannot state** is what the alarm was for. So the scheduling checker holds three more, once `J_s` is known (`RA-H2`), each as this table writes it. `alarm_model.rs` reads this table and runs its cells, so a sum here is the sum it holds, and a change here is a change there (`the_record_s_sums_read_as_they_are_written`).

<!-- machine-read: alarm-sums -->
| Sum | Within | On a level compare | On a delay |
| --- | --- | --- | --- |
| `delivery` | the `delivery-bound` the requirement asks, whatever more a device offers | `max(J_s + C_s, T_s + λ) + ρ + J_s + C_s` | `max(J_s + C_s, T_s + 2λ) + ρ + J_s + C_s` |
| `update` | the provider's `unambiguous-horizon` | `max(P, T_s, λ) + ρ + J_s + C_s` | `max(P, T_s + λ) + λ + ρ + J_s + C_s` |
| `room` | less than one modulus, where the time base is the counter itself; a time base with an epoch of its own has no such limit | `max(R, T_s + λ) + delivery` | `max(R, T_s + λ) + delivery` |

- **`delivery`** runs from the programmed time, or the write if later, and `J_s` from the compare matching. What lies between them adds: the code's bound on the last arrival stands up to `λ` past it, the floor stands `T_s` past that, a delay lands late by `λ` again, and the value is then rounded up, by `ρ`. A write in a service, a chain's next segment or a multiplexed alarm's next deadline, lands only inside that service, so a deadline that came due just before that service's read waits for it, which is the first `J_s + C_s`. The sweeps have not reached that term, so it may refuse a bound that would hold; it stands by that derivation. A deadline already due when written (`ROADMAP.md` §6.2) is raised at the write and never lost, the floor apart. The model holds this sum over its sweeps, at resolutions to 16.
- **`update`** runs from one read of the counter in a service to the next, since an epoch is correct only while it is read once in every wrap. A service's read is at or past the point it was raised for, so the next point is at most `P` past that read. On a compare the code's bound is at or before that read, so the floor is at most `T_s` past it, and a write landing with nothing left to wait for arrives within `λ` of it. On a delay the bound stands up to `λ` past the arrival, and the next count lands `λ` late again. `P` is what the service programs, and not the greatest separation of arrivals, which hangs on `J_s` and which no owner could state.
- **`room`**: what stands ahead and what is read past must lie within one modulus. A deadline stands at most `R` past a read, the floor `T_s + λ`, and a deadline is read past by as much as `delivery` lets it wait. `R` is a deadline's and not a chain's segment, since a chain's deadline stands as far ahead as it was asked to. The search can only say that what is asked lies within the provider's `unambiguous-horizon`, which is `supported-horizon`'s offer (the adapters record's §2); only this knows how far a deadline is read past, so this is the rest of it. The model's case is this, on modular arithmetic: the timeline has no modular hardware.

**Why not a timer event of the release timer**, under an amended condition 6. That was the first candidate, for a `wake` above all. Condition 6 reads "A timer interrupt is taken only when a release is due", and the timer term counts releases, so a wake with no release due is charged nowhere: the amendment would reopen what `M2.11`'s review closed.

### 3. The order

An alarm's rank goes by the deadline it has to meet, ascending, among the sources:

- a source's: its stated `min-arrival-separation`, the no-loss limit of a source acknowledged at entry, as today;
- an `expiry`'s, or an alarm on a block offering `absolute-deadline` itself: the `delivery-bound` its requirement asks;
- a `wake`'s: its provider's `unambiguous-horizon`.

Each is a time, and none reads `J`, so nothing is circular. Ties go by the interrupt's text, and what states no value goes last, as today. So an expiry that must be delivered soon stands ahead of a slow source, which ranking timer routes last, `/1`'s stopgap, would not allow. With no alarm present the order is `/1`'s. It is deadline-monotonic over what a description or its provider states. ⚠️ It is a choice of order and no claim of optimality: the scheduling checker judges whatever the plan orders, and the Audsley search `RTP-H4` asks for stays `M2.21.2`'s.

`/1`'s ranks stand, since `/1` is a frozen format: this is `/2`'s (`RA-H1`).

**Against the release timer** an alarm stands where any source stands, the hardware's fact `external-before-timer` deciding it (the composite-inputs record's §1 and §2), since it is declared as one and reaches the core through the controller. On the simulator that is `yes`, every alarm before the release timer, which is what `RTP-H9` has it do.

### 4. A node's call cost

A job's call into a node's generated code is a call of a primitive. The composition's parts already say all it needs: `api.p` is "one call, from its first instruction to its last", `masked.p` "the longest contiguous masked stretch inside one call", and `n_{i,p}` the most one job makes. So a node `N` a job calls gives the composition `api.N` and `masked.N`, stated by the realization and not by the catalog, and `n_{i,N}` from the application's inputs (`RTP-H5`). §3's formulas do not change. The composite-inputs record's §7, which left such a task's composites undeclared until this design, gives way to it. It holds a primitive's other facts too, `primitives-out-of-line` and that a call returns with the masking balanced, each owed by the realization (`RA-H5`).

- **A span.** A node's call cost is a span, so it holds what the node calls in turn. A node a job reaches only through another owes no call cost of its own for a job's count to multiply: it is charged inside its caller's, and the realization derives each from what it calls. This answers the runtime plan's R26-R2, whose `call-cost` row is owed for every node a job runs.
- **A multiplexed node's overhead has no term of its own.** A multiplexer's work for a user is inside the call that user makes, its `api.N`, and its work for an alarm is inside that alarm's `C_s`. A term beside them would count it twice. The runtime plan's `multiplex` row said "the term `M4.1.6` designs": this is that design, and what it owes is the statement of those two bounds (`RTP-H2`), with each alarm's arrivals as §2 has them.
- **Its masked runs in a job are a job's.** The fault contract has a `mask` a job's when "the job's own code, or a primitive it called, executes it", and a generated node a job calls is a primitive here. This answers `M2.30`'s R10.
- **What runs in two contexts has two paths.** A node a job calls and a route runs takes `mask` and `unmask` on the path a job runs, and neither on the path its service runs: interrupts do not nest, so a service needs none, and the fault contract makes one executed by a service an assertion failure, which would halt the system on the route's first run. So the realization generates the two apart. Its shared data is condition 4's, inside the job path's masked runs, and its masked runs reach a route through `CS_k`, `L` and `B_s`, as any job's do.

### 5. How a job receives an alarm

**A job receives an alarm only as a release: a task is released by it.** No context runs a job's code in an interrupt, and the variant's "Work a service defers runs as a declared task". So what `/1` refuses, a task that would receive an interrupt's code, has one meaning, and it is the one any interrupt has:

- An alarm's service releases the tasks declared `released by` it, as a source's does, and their condition is the variant's: a task released on every arrival has `T_i ≤ T_s`.
- **Something else programs it.** A task released only by an alarm it alone programs has no first job, and the analysis would hold of a task that never runs. So does any set of tasks each released only by alarms the set programs, by two tasks or by more. Some task of a set must be released by the timer or a source, and a set with no such member is refused (`RA-H4`).
- **`/1`'s refusal stands until it can be said.** A task requester bound to an alarm directly, or to an `expiry` instance, is still refused. What it lacks is a way to write the release: a trigger naming the alarm, as `(source X every)` names a source, which `M1.44` owes every sporadic task's source. The separation it reads is the alarm's floor. That is `RA-H4`'s, and the refusal's repair then says so.
- **The system's own level has no such meaning, and stays refused.** It has no job, and runs nothing after initialisation, which runs masked and never waits on an interrupt (the runtime plan's §2.7). So a requirement for an alarm or an expiry at the system's own level has nothing to receive it. The repair is to move it to a service, or until `RA-H4` to a task's service.

**Hand-offs.** Each is carried, word for word, by the leaf it names.

<!-- machine-read: handoffs -->
| Id | Leaf | Obligation |
| --- | --- | --- |
| `RA-H1` | `M4.1.7` | Format `/2` and what it reads are given the release requester of `decision_release-alarm.md` §1 on a system on a target with a timer-released task: the search adds it, the assignment names it, the checker reads it, model §8 rules 1, 7, 8, 9 and 12 say so; its binding is lowered as the release timer, or `(timer none)` with no such task; each alarm has a line of its own, which rule 8 and the runtime plan's §2.5 and §2.9 say of each alarm, asks that record's §2 of a target, the joint rules asking idle and a device's kind of the device bound alone, and routes rank by that record's §3. `alarm-wiring`, and an alarm's kind, a level compare, a delay or an edge, join the vocabulary as facts of the function `counter`, where `relative-delay` is, so that model §8 rule 13 binds them to one block, and each target's statement with its `compare-rounding`, and `channels` on a block wired to the controller counts its lines. The runtime plan's §2.7 is amended for a wake's first arm. |
| `RA-H2` | `M2.21.2` | The scheduling checker declares a source for each alarm, an identifier of its own kind, with the inputs `decision_release-alarm.md` §2 lists and the composition's `J_s`, `ρ` being its own device's; an alarm whose inputs are owed leaves the set inconclusive; `(timer none)` is a set with no timer, the variant amended to admit it at a version of its own, with `docs/book/src/versions.md` and what `/2`'s conclusions then say, the composition asking no `C_rel`, `ρ` or condition 6's facts either, `external-before-timer` being no reason to ask them there; and it holds each alarm's `delivery`, `update` and `room` as §2's table writes them, by its device's kind, and the release timer's `SHORTEST + J^release + C_rel` within its provider's `unambiguous-horizon`, each separation that enters a sum rounded up. |
| `RA-H3` | `M4.3.5` | The simulator raises an alarm as a request at its controller, which the alarm's trap claims, one request per arrival, and takes each alarm where a source stands against the release timer. |
| `RA-H4` | `M1.44` | A task can be released by an alarm or an expiry: a trigger naming it, reading that alarm's floor as its separation; the runtime plan's refusal of a task requester giving way to it, a set of tasks each released only by alarms the set programs refused, since none has a first job, and a repair saying so until then. |
| `RA-H5` | `M2.21.2` | A node's `api.N` and `masked.N`, stated by the realization with `primitives-out-of-line` and a call's balanced masking, reach the composition as a primitive's do, for every node a job runs; each alarm's realization states its floor, and its `service.<source>` is anchored by it; the discipline's role and the sentence beside it hold as §2 states them; the composition no longer calls a floor's `T_s` an assumption about the environment; and the composite-inputs record's §6 reads a missing one as it reads a primitive the catalog does not cost. |
| `RA-H6` | `M4.2` | The generated code of each alarm writes each compare by `decision_release-alarm.md` §2's protocol under the floor it states, and each service by the same; a node a job calls and a route runs has that record's §4 two paths; and a wake is first armed at the port's transition as §2 states it. |

## Why

- **One rule for everything that asks.** A release timer with rules of its own would be a second account of channels, device choice and refusal, each to be kept in step. The kernel asking as anything else asks needs one requester in the joint rules and no new judgement of channels.
- **Reuse a proof rather than amend one.** The variant's source term has been reviewed, fixture by fixture, against the failure this design would otherwise have to rule out again. What it needs in return is honest inputs, and §2 says where each comes from.
- **A deadline, since that is what an alarm has.** A source's rank by separation is deadline-monotonic where its deadline is its no-loss limit, so one key covers both.

**Alternatives weighed.**

- **The target's tick-rate block**, R3-1's. A counter may raise no interrupt, which is why this reads `relative-delay`, and it would ignore channels: R3-3.
- **Release by a named device in the target's file.** A target saying "this one releases" is a fact the target's agreement cannot check, and a target's file holds what its agreement checks (`decision_target-platform-description.md`).
- **`LONGEST` for the horizon.** It holds, and refuses a 16-bit counter at 1 MHz for periods of 1 ms and 1 s, which works.
- **One source per route.** Model §8 rule 8's key, and no one `T_s`, rank or bound describes two alarms on it.
- **`T_s` stated by whoever programs an alarm.** The first two rounds' answer. The owner cannot state it: it hangs on response times and on a chain's last segment, and round 2's reader built a set that holds at the stated figure and misses. A floor makes it what the code does.
- **A release timer bound and never due, with no timer-released task.** Round 2's answer. A modular compare's largest value comes round within a wrap, and its trap finds no release due. So `(timer none)` stands, and the variant owes it a reading.
- **A wake as a timer event of the release timer.** §2.
- **Timer routes last, as `/1` has them.** An expiry due soon could wait behind every slow source, past its delivery bound, though what ranks it is stated.
- **A term of its own for a multiplexer's overhead.** It runs inside a call or a service, each already charged whole.
- **A node's cost per callee, added up by the composition.** It would need the call graph inside the realization, which a description never states, and a span is what the parts already mean.
- **A job's code run in the interrupt.** It is what §5 rules out, and it would take condition 5's "Work a service defers runs as a declared task" with it.

## How to apply

- Nothing here changes `/1`, its goldens or a simulator run. Each change is in a hand-off above, but for the composition's added role, which is text until `RA-H5`.
- ⚠️ **What this does not decide.** Whether a port can realize an alarm as a request a claim takes is `M4.1.7`'s and the port record's: `riscv-virt-up`'s CLINT is one comparator wired to the core, so a system there has the release timer alone and no alarm, which is what the joint rules and condition 5 each say.
- ⚠️ A system with an alarm is still refused or inconclusive until `M2.21.2` and a realization exist: this says how it will be charged, and charges nothing. What has run is the model, `alarm_model.rs`, a timeline and no realization.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 13 | 9 (RA1-1 to RA1-9), beside 3 gaps and 1 of wording | `2026-11-06`, not acceptable — answered: a `T_s` is a least separation, owed by whoever programs; one source per alarm and not per route; `delivery` and `update` held on what they mean, the read-to-write lag and the service among it; what the composition and the joint rules gain, said, where "nothing changes" stood; `alarm-wiring`, and `available-in-state idle`; two paths for what runs in two contexts; a receiver needs something else to program it |
| 2 | 15 | 9 (RA2-1 to RA2-9), beside 5 gaps and 1 of wording | `2026-11-06`, not acceptable, the same count as round 1 — answered by method: a model the claims run against, `alarm_model.rs`. It showed the separation cannot be stated, so it is enforced: a floor on every compare, with no input from the application. `delivery` holds the floor, the rounding and a service's lag; an alarm on a target asks what §1 asks; a release requester on every system on a target; a set with no first job refused; a reach leaves its lag within a modulus |
| 3 | 9 | 7 (RA3-1 to RA3-7), beside 1 gap and 1 of wording | `2026-11-06`, not acceptable, falling from 9 — answered with a wider model, its reader's, in the tree: what code can know at a write is what it read and wrote, so the floor bounds the last arrival from above and leaves a compare that may match before the landing; `delivery` by the device's kind; idle and a level compare asked of the device bound; the role as wide as the rules it answers; `(timer none)` back, with a reading owed by the variant; a reach's room holds the floor |
| 5 | 10 | 7 (RA5-1 to RA5-7), beside 2 gaps and 1 of wording | `2026-11-06`, not acceptable, up from 5, so answered by method: the sums are a table the model reads and runs, and the model reads the protocol's sentences as they are written — `update` by its device's kind, `P` the step a service programs, `λ` covering the read after the landing, the rounded value compared, a delay's bound its own count, a floor judged within its own window, `R` a deadline's with an owner, the wiring of one function with the delay |
| 4 | 10 | 5 (RA4-1 to RA4-5), beside 4 gaps and 1 of wording | `2026-11-06`, not acceptable, falling from 7 — answered: what lies between a deadline and its match adds, so `delivery` and `update` hold `ρ` beside the floor; a reach's room holds what a deadline is read past; a delay rounds from its read; what a generated node owes goes to the leaf that generates it, `RA-H6`; a wake is first armed at the port's transition; the record's protocol is the model's, word for word |
