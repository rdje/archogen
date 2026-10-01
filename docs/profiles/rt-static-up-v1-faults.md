# `rt-static-up-v1` — the fault contract: classification, attribution and containment

> **Moved here `2026-10-01` from `ROADMAP.md` §3.1.1**, on the director's ruling that a contract still under review
> belongs in the profile's specification rather than in the roadmap (`M2.19`). The text and its rule numbers are
> unchanged by the move; "§3.1.1" in older records means this file. It is normative as part of the profile
> [`rt-static-up-v1`](rt-static-up-v1.md), and `ROADMAP.md` §3.1.1 keeps the requirement and the table. A bare section
> number below — §3.1, §7.3, §8.1, §13.4, §15 — is `ROADMAP.md`'s.

**Amendment, 2026-09-13 (leaf `M2.9`).** §3.1's Fault response row names four faults and §8.1 names
three classes, and nothing connected them. Two independent implementations of §8 read the gap
differently — recorded, with both readings, in
`docs/decisions/decision_runtime-contract-gaps.md` — so the mapping is fixed here rather than left
to each implementer. This is an addition, not a correction: no earlier decision changes, and no
existing description or result is invalidated.

**Amendments, 2026-10-01 (leaves `M2.9`, `M2.17`, `M2.19`).** They answer the director's rulings (findings §6) and
five independent reviews of this text, *R1*–*R5*, whose findings and answers are tabled in
`docs/decisions/decision_runtime-contract-gaps.md`; the rule each answer touched is marked below.
- **Corrections of the 2026-09-13 text** (§14.1): a second release latched in a masked region is contained, not
  fatal (rule 1); a completion closes its region (rule 4); a synchronous fault is the executing context's, and a
  service's is no task's, where the table said "the running task" (rule 2); the mask bound and an unbalanced
  `unmask` are assertion failures, where they were refusals (the second smaller decision).
- **Narrowings**, each a claim the earlier text did not limit, for the director's review: no overrun is raised
  without a release (rule 1a); a missed deadline is reported only as rule 6 says, and §13.1 F26 with it; an
  externally released task's second arrival during a pending request can be lost where its source does not count
  arrivals (rule 1); a `mask` or `unmask` outside a job is an assertion failure (Terms) — initialisation's
  included, which the composition's `leaves-interrupt-hardware-alone` had admitted; and a broken port obligation the
  trap path or a service can see — an empty first claim, a timer trap with no release due, a claim returning an
  undeclared source, an API entry naming no primitive — is an unexpected trap, and halts (Terms).
- **What moved an implementation**, change by change: the record's *What changed an implementation*. `M2.17`
  located the empty set's refusal in `archogen build` and boot; `M2.19` moved this text out of `ROADMAP.md`.
- **Evidence invalidated** (§15): none. No version of `rt-static-up-v1` is released or locked, no released result
  rests on the earlier answers, and both models and their tests moved with each change; so no version is advanced.

**Terms.**
- A **masked region** is the interval in which the runtime's mask nesting depth is above zero: opened by a job's
  outermost `mask` and closed by the matching `unmask` or by that job's completion (rule 4). A **section** is one
  `mask`…`unmask` pair inside a region. §3.1's bounded kernel critical sections are both the regions — bounded in
  depth by `M` (the second smaller decision) and in duration by `CS_i` — and the processor's own interrupt disable,
  bounded by the composition's `masked.p`, `masked.completion` and `L`'s service, transition and idle-wake terms,
  and outside `L` by initialisation's preceding every release and by rule 7's bound on the fatal handler. A task's
  masked runs are regions. That disable — in a primitive, a service, the trap path, a transition, the completion
  path, idle, initialisation or the fatal handler — is **not** a masked region, and §13.4's "latched" means pending
  in hardware there.
- **Only a job changes the depth.** A `mask` or `unmask` is a job's only when the job's own code, or a primitive it
  called, executes it. Executed by any other context — a service, the trap path, a transition, idle, initialisation or
  the fatal handler — it is an assertion failure of that context, raised before the depth changes, whether or not a job
  is preempted beneath it. A runtime that is told no context detects the case it can see, a call with no job running. A
  port must detect the rest — a call from any context but a job, whatever its interrupt state or trap nesting at the
  call — and raise it; how it tells a job's call apart, a call entered through the port's own API trap included, it
  states in its catalog record, where it is reviewed with the port (`M2.12`). The completion path, entered by any
  context but a job, is an assertion failure of that context, as a `mask` is. **Initialisation** is no job, so it calls
  no masking primitive — running before the first enabling of interrupts with interrupts already disabled, it has
  nothing to mask. This narrows what the composition's `leaves-interrupt-hardware-alone` admits: its masking primitives
  are a job's only. A depth above zero when a job is dispatched or resumed, or idle is entered, is likewise an assertion
  failure, raised by the transition — no task's, even where a port decides inside the completion path — so
  it interrupts the outgoing task if that task's job is still owed, and none otherwise (rule 2). So every section open
  at a completion is the completing job's.
- A task's **latch** is what holds its releases that arrive while a masked region is open. On the target that is
  the interrupt pending in hardware, or the timer's releases due, which the timer service computes at delivery (the
  composition's `releases-never-latched`); in a hosted model, the runtime's own record: the most recent release, and
  a mark that an earlier one also came. A hosted model judges the earlier arrivals its mark stands for as one, so
  for one delivery it can report fewer overruns than the target, which judges each due release. For a
  timer-released task, and an external source whose record states it counts arrivals, the task's state after the
  delivery is the same and `M2.15` maps the events; for any other source the target can lose an arrival the hosted
  mark keeps (rule 1), and the states can differ.
- A job **completes** at its last instruction of its own code (§13.4). A task **owes a job** from its release until
  that job completes or is abandoned. A release observed after the job's last instruction of its own code — while
  its completion path runs, before or after the completion is recorded, or in the instant between that instruction
  and the path's first — finds the task owing no job. Until its decided switch, that job's context — its completion
  path, or the instant before it — runs at its task's priority though the task owes no job: the transition that
  follows a service which preempted it returns to it unless a task of higher priority owes a job, and a job of that
  task released meanwhile starts at its entry only when that path's decision dispatches it. A port may instead let
  no service preempt that interval. Which it does, and how it guarantees either, it states in its catalog record,
  reviewed there (`M2.12`).
- An **unexpected trap** is any trap other than the timer's interrupt, a declared source's interrupt whose claim
  finds a request, the runtime API's own entry where the port uses one, and a trap that is another fault's mechanism
  — an access fault on a stack's guard is a stack guard, and a trap the runtime raises deliberately on a failed check
  is an assertion failure. An external trap whose first claim finds no request is one, since the composition's
  `no-empty-claim` makes it a port's broken obligation; a later claim in the same trap that finds none ends its
  service loop and is no fault, while one that returns a source the plan does not declare is an unexpected trap at
  that claim, raised by the trap path. A timer trap that finds no release due is one too, since the composition's
  `raised-only-when-due` (the timer-service records' fact,
  `docs/specs/catalog/decision_catalog-records-variant-inputs.md`) makes it a port's broken obligation; so is an entry
  of the runtime API that names no primitive, of the context that executed it. The assertion failures this contract
  names are not all there are: any check a runtime or a port makes of its own invariants raises one.

*(Terms: R2 33, 35, 37, 45, 47; R3 54–56, 58, 60–62, 64, 70; R4 82, 84–87, 89, 90, 95, 96, 100; R5 106, 110, 111,
118–120.)*

| §3.1 fault | §8.1 class | Attributed to | Containable |
|---|---|---|---|
| Overrun | Expected error | the **overrunning** task, whichever context holds the processor (rule 2) | Yes — by its declared per-task policy (§7.3, rule 5) |
| Stack guard | Violated internal invariant | the context that raises it (rule 2) | No |
| Unexpected trap | Deliberate fatal trap (its cause is outside the model) | the context that raises it (rule 2) | No |
| Assertion failure | Violated internal invariant | the context that raises it (rule 2) | No |

The rules below settle what the text left open; what is still open is listed at the end.
1. **Detection applies the policy.** An overrun is a fault the moment the runtime observes a release for a task that
   still owes a job; it does not wait for a separate decision. §3.1 requires the policy to be *defined* and §7.3 makes
   it part of the task record, so it is the system's behaviour rather than a caller's option. Every release that arrives
   while a masked region is open is latched, and is observed — for this rule and every other — only at its **delivery**,
   after the region closes — at once at an outermost `unmask`, and after a completion as rule 4 says. At delivery a
   task's latched arrivals are judged in arrival order against the task's state then: the first is fresh if the task
   owes no job (one that completed inside the region included) and an overrun if it does; each later one is judged
   against the state the earlier ones left, in which the task owes the job an earlier one released. Under `SkipLateJob`
   each is an overrun in turn; under `Fault` the first overrun, whichever arrival it is, takes the task out, and every
   arrival after it is discarded. Across tasks the order changes no task's state and is the port's (`M2.15` traces it).
   So no release that finds its task owing a job at delivery escapes its policy, nor is fatal for landing inside a
   region rather than one instruction after it — for arrivals the platform delivers as distinct requests. A
   timer-released task's are computed at delivery, one per nominal release due, on every port. An externally released
   task's are distinct only where its source's catalog record (written under `M2.7.4`) states that arrivals while its
   request is pending or claimed are counted — a fact no record states yet — and otherwise the record states that such
   an arrival, and its overrun, can be lost. The release that triggers an overrun is the policy's: under `SkipLateJob`
   it makes the task ready for its next job, with that release's nominal instant; under `Fault` it goes with the faulted
   task. A completion and a release at one instant: the completion first (§13.4). *(2026-10-01: §6 (b); R1 6, 14, 22;
   R2 31, 37, 42, 49; R3 64, 69, 76; R4 83, 97, 102; R5 116.)*

   1a. **An overrun raised without a release is outside this profile.** `rt-static-up-v1` has no execution-budget
   monitor: an overrun is detected by rule 1 alone, as rule 6 says of a deadline. A port that raises one another
   way — a monitor's interrupt, or a bound checked synchronously — is outside the profile. A later profile that
   admits one must say how its source is declared (§3.1, §7.3) and charged, and how its overrun is judged against
   the job it measured. *(2026-10-01: R1 5; narrowed by R2 26, 27, 29, 40, for the director's review.)*
2. **Attribution follows the fault, not the processor.** An overrun is attributed to the overrunning task — the task a
   release, or a delivered latched release, belongs to — whichever context holds the processor: a task that is ready and
   not running, the task whose job the release service interrupted, or the task executing its own outermost `unmask` or
   completion. The single-core rule of §3.1 governs *execution*, not *attribution*. The other three faults are
   attributed to the context that raises them. An exception — a guard's access fault, an assertion's trap, any other
   faulting instruction — is raised by the context that executed the instruction; an unexpected trap that is an
   interrupt — a cause the port does not serve, a timer trap with no release due, an undeclared source, an empty claim —
   is raised by the trap path. A task's job is that task, including a runtime primitive it called however the port
   enters it — where the port enters the runtime API by a trap, the code from that trap's entry to its return is the
   primitive, not the trap path — and the completion path it entered. A service, the trap path, a transition, idle,
   initialisation or the fatal handler is no task: the evidence names that context, a service by its source, and records
   the task it interrupted as interrupted, never as attributed. A service or the trap path interrupts the task whose job
   its trap preempted while that job is still owed, and none if the trap preempted idle or that job has completed or
   been abandoned since, even where its task owes a newer one. A transition — from the decision's first instruction to
   the incoming context's first — interrupts the outgoing task if the job it switches away from is still owed, and none
   if the outgoing context was idle or that job has completed or been abandoned, even where its task owes a newer one;
   it never interrupts its incoming context. For attribution the completion path ends where the scheduling decision
   begins: the decision, and everything after it up to the incoming context's first instruction, is the transition's
   wherever a port places it, whatever the composition's `completion` cost spans. A trap taken for an interrupt is the
   trap path from its entry until it has identified what it serves — the timer by the trap's cause, an external source
   by a claim that returns it — then that interrupt's service to the service's last instruction, and the trap path again
   between services and from the last to the trap's return or the following transition. A timer trap with no release
   due, an empty first claim and a claim returning an undeclared source are the trap path's wherever the port's check
   runs. A service run inside an API trap is that service, not the primitive. The composition's timing boundaries are
   unchanged. A stack guard is attributed the same way, and the evidence names whose guard was hit — a task's, the
   interrupt stack's, or another stack the port guards, named; every stack whose bound is not established has a guard
   (§7.6). *(2026-10-01: §6 (d); R1 4, 8; R2 34, 46; R3 57–59; R4 84, 87, 88, 104; R5 108–110, 118, 121.)*
3. **A containable fault raised inside a masked region is not containable.** Two grounds:
   - terminating a job that *holds* the region leaves the nesting depth above zero with no owner, so interrupts
     never return; forcing the depth to zero re-enables them inside a region whose invariants the faulting job was
     partway through restoring;
   - and containment means **resuming the schedule** from a state the region had not finished making consistent,
     which is what a kernel critical section exists to prevent. That holds whichever task the fault is attributed
     to, not only the one holding the region.

   Ground 1 covers the task holding the region; ground 2 extends the rule to every task. §8.1 offers no third
   option — "preserve a defined fatal handler" — so it escalates (rule 7). Escalation is the conservative
   direction: a fatal path that was not strictly required costs availability, while a containment that was not
   safe costs correctness silently. In `rt-static-up-v1` no containable fault is raised inside a masked region —
   every release there is latched (rule 1), and there is no other way to raise an overrun (rule 1a) — so this rule
   decides nothing observable in this profile; it fixes the answer a later profile that admits one starts from.
   *(2026-10-01: §6 (e); R1 1, 24; R2 26.)* ⚠️ Delivery is **not** inside a masked region, so an overrun found at
   delivery applies its ordinary policy — the difference between a profile that can contain an overrun and one
   that cannot.

4. **A job may complete inside a masked region, and its completion closes it.** The region is the job's (Terms), so the
   nesting depth returns to zero with the job, and releases latched in the region are delivered after the completion is
   recorded and before any task executes an instruction of its own code. On a port the decision precedes that delivery —
   decided in the completion path, delivered when the following transition unmasks (the composition's
   `releases-never-latched`); a hosted model, which takes no trap, may deliver first. Both reach the same schedule. A
   fault other than an overrun raised during a delivery is, on a port, the trap path's or its service's — no task's,
   interrupting the task executing the outermost `unmask`, or after a completion the task the transition switched to, or
   none if it switched to idle; in a hosted model that delivers inside `unmask` or the completion path, it is that
   task's. `M2.15` maps the two records as it maps their traces. A completion is not a fault: the job's own code has
   finished, so neither ground of rule 3 reaches it, and it is the masked run that ends at completion, which the timing
   analysis charges (`docs/specs/catalog/decision_runtime-composite-inputs.md`, `CS_i`). A latched release is judged at
   that delivery (rule 1), so a task that completed inside the region is released afresh, not overrun. *(2026-10-01: §6
   (a); R1 16; R2 30; R3 73; R4 81; R5 107.)*
5. **The two overrun policies.** `SkipLateJob` (eADL `skip-late-job`): the task's owed job is abandoned — not started,
   preempted, or the job the release interrupted. Its remaining code never runs and no completion is recorded for it, no
   fault halts anything, and the triggering release makes the task ready for its next job (rule 1). `Fault` (eADL
   `fault`): the task is faulted and leaves the schedule until reset. Its owed job is abandoned, the triggering release
   and every later release of it are discarded, and no further instruction of its own code runs; every other task
   continues, and the runtime does not halt. A task without an `on-overrun` clause has `Fault`, and `archogen check`
   refuses a policy other than these two (`docs/semantics/model.md` §4 rule 6, leaf `M2.14`). A job is abandoned only
   where it holds no masked region: in its own code with the depth at zero, or at a primitive's entry or return where
   the depth is zero. A policy applied while the job is inside a primitive takes effect at the primitive's entry if the
   primitive has not yet changed the runtime's state — for `mask`, before the depth is raised — and otherwise at its
   return, which a policy never reaches while the primitive holds the depth raised, since no release is observed inside
   a region (rule 1). Where the policy takes effect at a primitive's return, the runtime completes the primitive on the
   task's behalf — under `Fault`, which is not the task's own code, and under `SkipLateJob` before the next job starts;
   where it takes effect at the entry, the primitive is not run. Containment keeps the runtime's state consistent and
   the schedule running, and claims nothing about the application state the abandoned job left (§3.1, Isolation). A
   run's timing claims end at its first contained overrun, which shows that an assumption of its analysis — an execution
   bound, an arrival bound or the interference it counted — did not hold, or that the claim was not established.
   *(2026-10-01: R1 2, 11, 25; R2 28, 32, 41; R3 66, 72, 76; R4 93, 98; R5 112.)*
6. **The runtime detects no missed deadline.** Rule 1's overrun is its only timing fault. A miss shows only as that
   overrun, and only if the job is still owed when the task's next release is observed; only with `D = T` and strictly
   periodic releases is that release observed no earlier than the deadline, so a job owed at its deadline can still be
   owed then; even so, a job that completes after its deadline but before that release is observed — within the
   release's delivery latency, or inside a region where the release was latched (rule 4) — is not reported. §13.1 F26
   exercises a miss with `D = T`, strictly periodic releases and zero declared jitter, in which the job is still owed
   when the next release is observed; any other miss is the analysis's to exclude and a trace's to observe. A deadline
   monitor would be a new source and a new fault. *(2026-10-01: R1 7, for the director's review; R2 39, 40; R3 63;
   R4 92, 99; R5 117.)*
7. **Escalation halts, and keeps the first fault.** A fault that is not containable, or is made so by rule 3, enters the
   fatal handler. From then no job runs, no release is processed, no pending interrupt is taken, and no transition
   occurs; the handler ends, within a bound the runtime's catalog record declares beside the nesting bound `M`, in a
   terminal state with interrupts masked. It preserves until reset the **first** such fault only, written before
   anything else the handler does: its §3.1 kind; its §8.1 class; the attributed task's stable logical ID — its
   elaborated eADL task name, the instance path of `docs/semantics/reference.md` §6 rule 9, which `archogen check`
   requires unique (`schema-duplicate-name`) — or no task, with the executing context named, a service by its source;
   the task it interrupted, if any; for a stack guard, whose guard was hit; and whether rule 3 escalated it, which in
   this profile it never does. The record carries a completeness mark, which reads incomplete from boot until it is set
   as the record's last write: a fault taken before it is set ends the handler with the record marked incomplete and the
   fields already written kept. A runtime that holds tasks by an internal index has the plan supply the ID when the
   record leaves it (§7.5): an index names no task in a record. A fault taken in the handler ends it at once in its
   terminal state and never replaces the preserved one. What the task table shows afterwards is the implementation's,
   and is not evidence of attribution. *(2026-10-01: R1 9; R2 34, 44, 48; R3 65, 68, 75, 80; R4 80, 103; R5 113.)*

Two smaller decisions are fixed at the same time, for the same reason:

- **A task set must be non-empty** — an empty workload would make §7.2's second timing obligation vacuous.
  `archogen build` and boot refuse one; a refused boot is no fault: the runtime never starts, and nothing is kept but
  the refusal. `archogen check` admits one, a system describing composition alone (F01) being valid, and makes no timing
  claim for it (`M2.17`; R3 79).
- **Kernel critical sections are bounded by a declared nesting depth.** "Bounded kernel critical sections" did not
  say what the bound is or what happens at it. Each runtime's catalog record declares a maximum depth `M ≥ 1`. A
  `mask` that would raise the depth past `M`, and an `unmask` at depth zero, are **assertion failures**: a counter
  that *wraps* re-enables interrupts inside a critical section while reporting success, one that *saturates* stops
  counting, and one that *refuses* leaves its caller's matching `unmask` to close the section early. This bounds
  the depth; the duration is bounded separately, by the `CS_i` the timing analysis charges. *(2026-10-01: R1 10; a
  refusal had been the answer.)*

**Still open:** each path's observation events and hosted/target trace compatibility, a discarded release's and a
completion that closes a region included (`M2.15`); the catalog fact by which an external source's record says whether
arrivals during a pending request are counted (rule 1; the source's catalog record, written under `M2.7.4`); how a port
detects a non-job call and what it lets run in the interval after a job's last instruction (Terms), and how the kept
record is read out of a halted runtime (rule 7) — each the port's catalog record's (`M2.12`); and, needed by no fixture
yet, a later idle-to-task dispatch's cost and a periodic task's first release instant.
