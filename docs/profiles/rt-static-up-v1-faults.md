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
three independent reviews of this text, *R1*, *R2* and *R3*, whose findings and answers are tabled in
`docs/decisions/decision_runtime-contract-gaps.md`; the rule each answer touched is marked below.
- **Corrections of the 2026-09-13 text** (§14.1): a second release latched in a masked region is contained, not
  fatal (rule 1); a completion closes its region (rule 4); a synchronous fault is the executing context's, and a
  service's is no task's, where the table said "the running task" (rule 2); the mask bound and an unbalanced
  `unmask` are assertion failures, where they were refusals (the second smaller decision).
- **Narrowings**, each a claim the earlier text did not limit, for the director's review: no overrun is raised
  without a release (rule 1a); a missed deadline is reported only as rule 6 says, and §13.1 F26 with it; an
  externally released task's second arrival during a pending request can be lost where its source does not count
  arrivals (rule 1).
- **What moved an implementation**, change by change: the record's *What changed an implementation*. `M2.17`
  located the empty set's refusal in `archogen build` and boot; `M2.19` moved this text out of `ROADMAP.md`.
- **Evidence invalidated** (§15): none. No version of `rt-static-up-v1` is released or locked, no released result
  rests on the earlier answers, and both models and their tests moved with each change; so no version is advanced.

**Terms.**
- A **masked region** is the interval in which the runtime's mask nesting depth is above zero: opened by a job's
  outermost `mask`, kernel or application, and closed by the matching `unmask` or by that job's completion (rule 4).
  A **section** is one `mask`…`unmask` pair inside a region. §3.1's "kernel critical section" is a region; the
  composition's task runs (`CS_{i,r,e}^app`) are regions' extents, while its `masked.p` and `masked.completion` are
  the processor's own interrupt disable. That disable — in a service, the trap path, a transition, the completion
  path, idle, initialisation or the fatal handler — is **not** a masked region, and §13.4's "latched" means pending
  in hardware there.
- **Only a job changes the depth.** A `mask` or `unmask` is a job's only when the job's own code, or a primitive it
  called, executes it. Executed by any other context — a service, the trap path, a transition, idle, initialisation
  or the fatal handler — it is an assertion failure of that context, raised before the depth changes, whether or not
  a job is preempted beneath it. A runtime that is told no context detects the case it can see, a call with no job
  running; a port detects the rest from its own state — on the target a job's code runs at depth zero only with
  interrupts enabled, so a `mask` entered at depth zero with interrupts disabled, or any call made while the port's
  trap nesting is above zero, is not a job's. **Initialisation**, running before the first enabling of interrupts
  with no job and with interrupts already disabled, has nothing to mask and calls no masking primitive: the
  composition's `leaves-interrupt-hardware-alone` lets application code write the hart's interrupt state only
  through those primitives, which for initialisation means not at all. A depth above zero when a job is dispatched
  or resumed is likewise an assertion failure, raised by the decision that would do it — the transition's, no
  task's, even where a port makes that decision inside the completion path — and it interrupts the task rule 2 names
  for a transition. So every section open at a completion is the completing job's.
- A task's **latch** is what holds its releases that arrive while a masked region is open. On the target that is
  the interrupt pending in hardware, or the timer's releases due, which the timer service computes at delivery (the
  composition's `releases-never-latched`); in a hosted model, the runtime's own record: the most recent release, and
  a mark that an earlier one also came.
- A job **completes** at its last instruction of its own code (§13.4). A task **owes a job** from its release until
  that job completes or is abandoned. A release observed while the job's completion path runs, before or after the
  completion is recorded, finds the task owing no job; a port that cannot order them so disables interrupts from
  the completion path's first instruction.
- An **unexpected trap** is any trap other than the timer's interrupt, a declared source's interrupt whose claim
  finds a request, the runtime API's own entry where the port uses one, and a trap that is another fault's mechanism
  — an access fault on a stack's guard is a stack guard, and a trap the runtime raises deliberately on a failed check
  is an assertion failure. An external trap whose first claim finds no request is one, since the composition's
  `no-empty-claim` makes it a port's broken obligation; a later claim in the same trap that finds none ends its
  service loop and is no fault.

*(Terms: R2 33, 35, 37, 45, 47; R3 54–56, 58, 60–62, 64, 70.)*

| §3.1 fault | §8.1 class | Attributed to | Containable |
|---|---|---|---|
| Overrun | Expected error | the **overrunning** task, whichever context holds the processor (rule 2) | Yes — by its declared per-task policy (§7.3, rule 5) |
| Stack guard | Violated internal invariant | the executing context (rule 2) | No |
| Unexpected trap | Deliberate fatal trap (its cause is outside the model) | the executing context (rule 2) | No |
| Assertion failure | Violated internal invariant | the executing context (rule 2) | No |

The rules below settle what the text left open; what is still open is listed at the end.
1. **Detection applies the policy.** An overrun is a fault the moment the runtime observes a release for a task
   that still owes a job; it does not wait for a separate decision. §3.1 requires the policy to be *defined* and
   §7.3 makes it part of the task record, so it is the system's behaviour rather than a caller's option. Every
   release that arrives while a masked region is open is latched, and is observed — for this rule and every other —
   only at its **delivery**, when the region closes. At delivery a task's latched arrivals are judged in arrival
   order against the task's state then: the first is fresh if the task owes no job (one that completed inside the
   region included) and an overrun if it does; each later one is an overrun under `SkipLateJob`, and is discarded
   under `Fault`, whose first overrun took the task out; the earlier arrivals a hosted latch's mark stands for are
   judged as one. Across tasks the order changes no task's state and is the port's (`M2.15` traces it). So an
   overrun is not lost, nor fatal for landing inside a region rather than one instruction after it — for arrivals
   the platform delivers as distinct requests. A timer-released task's are computed at delivery, one per nominal
   release due, on every port. An externally released task's are distinct only where its source's catalog record
   states that arrivals while its request is pending or claimed are counted — a fact no record states yet — and
   otherwise the record states that such an arrival, and its overrun, can be lost. The release that triggers an
   overrun is the policy's: under `SkipLateJob` it makes the task ready for its next job, with that release's
   nominal instant; under `Fault` it goes with the faulted task. A completion and a release at one instant: the
   completion first (§13.4). *(2026-10-01: §6 (b); R1 6, 14, 22; R2 31, 37, 42, 49; R3 64, 69, 76.)*

   1a. **An overrun raised without a release is outside this profile.** `rt-static-up-v1` has no execution-budget
   monitor: an overrun is detected by rule 1 alone, as rule 6 says of a deadline. A port that raises one another
   way — a monitor's interrupt, or a bound checked synchronously — is outside the profile. A later profile that
   admits one must say how its source is declared (§3.1, §7.3) and charged, and how its overrun is judged against
   the job it measured. *(2026-10-01: R1 5; narrowed by R2 26, 27, 29, 40, for the director's review.)*
2. **Attribution follows the fault, not the processor.** An overrun is attributed to the overrunning task — the
   task a release, or a delivered latched release, belongs to — whichever context holds the processor: a task that
   is ready and not running, the task whose job the release service interrupted, or the task executing its own
   outermost `unmask` or completion. The single-core rule of §3.1 governs *execution*, not *attribution*. The other
   three faults are attributed to the context that raises them. An exception — a guard's access fault, an
   assertion's trap, any other faulting instruction — is raised by the context that executed the instruction; an
   unexpected trap that is an interrupt — a cause the port does not serve, an undeclared source, an empty claim — is
   raised by the trap path. A task's job is that task, including a runtime primitive it called however the port
   enters it — where the port enters the runtime API by a trap, the code from that trap's entry to its return is the
   primitive, not the trap path — and the completion path it entered. A service, the trap path, a transition, idle,
   initialisation or the fatal handler is no task: the evidence names that context, a service by its source, and
   records the task it interrupted as interrupted, never as attributed. A service or the trap path interrupts the
   task whose job it preempted, or none if it preempted idle. A transition interrupts the incoming task once the
   switch is decided; while it is being decided, the outgoing task if its job is still owed, and none if the
   outgoing context was idle or its job has completed or been abandoned. A stack guard is attributed the same way,
   and the evidence names whose guard was hit — a task's, or the interrupt stack's. *(2026-10-01: §6 (d); R1 4, 8;
   R2 34, 46; R3 57–59.)*
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

4. **A job may complete inside a masked region, and its completion closes it.** The region is the job's (Terms),
   so the nesting depth returns to zero with the job, and releases latched in the region are delivered after the
   completion is recorded and before any task executes an instruction of its own code. Whether the next scheduling
   decision precedes that delivery — the target: decided in the completion path, delivered when the following
   transition unmasks — or follows it — a hosted model — is the port's. Both reach the same schedule; a fault raised
   during the delivery is the completion path's, the completing task's, in the first and the trap path's in the
   second, and `M2.15` states how a trace shows each. A completion is not a fault: the job's own code has finished, so neither ground
   of rule 3 reaches it, and it is the masked run that ends at completion, which the timing analysis charges
   (`docs/specs/catalog/decision_runtime-composite-inputs.md`, `CS_i`). A latched release is judged at that
   delivery (rule 1), so a task that completed inside the region is released afresh, not overrun. *(2026-10-01:
   §6 (a); R1 16; R2 30; R3 73.)*
5. **The two overrun policies.** `SkipLateJob` (eADL `skip-late-job`): the task's owed job is abandoned — not
   started, preempted, or the job the release interrupted. Its remaining code never runs and no completion is
   recorded for it, no fault halts anything, and the triggering release, if there is one, makes the task ready for
   its next job (rule 1).
   `Fault` (eADL `fault`): the task is faulted and leaves the schedule until reset. Its owed job is abandoned, the
   triggering release and every later release of it are discarded, and no further instruction of its own code runs;
   every other task continues, and the runtime does not halt. A task without an `on-overrun` clause has `Fault`, and
   `archogen check` refuses a policy other than these two (`docs/semantics/model.md` §4 rule 6, leaf `M2.14`). A job is
   abandoned only where it holds no masked region: in its own code with the depth at zero, or at the entry or return
   of a primitive that leaves the depth at zero. A policy applied while the job is inside a primitive takes effect at
   the primitive's entry if the primitive has not yet changed the runtime's state — for `mask`, before the depth is
   raised — and otherwise at its return, which a policy never reaches while the primitive holds the depth raised,
   since no release is observed inside a region (rule 1). Under `Fault` the runtime completes the primitive on the
   task's behalf, which is not the task's own code. Containment keeps the runtime's state consistent and the
   schedule running, and claims nothing about the application state the abandoned job left (§3.1, Isolation). A
   run's timing claims end at its first contained overrun, which shows that an assumption of its analysis — an
   execution bound, an arrival bound or the interference it counted — did not hold, or that the claim was not
   established. *(2026-10-01: R1 2, 11, 25; R2 28, 32, 41; R3 66, 72, 76.)*
6. **The runtime detects no missed deadline.** Rule 1's overrun is its only timing fault. A job that misses its
   deadline is reported only if it is still owed when the runtime observes the task's next release, which needs
   `D = T`. Even then a job that completes after its deadline but before that release is observed — within the
   release's delivery latency, or inside a region where the release was latched (rule 4) — is not reported. §13.1
   F26 exercises a miss with `D = T`, strictly periodic releases and zero declared jitter, in which the job is still
   running when the next release is observed; any other miss is the analysis's to exclude and a trace's to observe.
   A deadline monitor would be a new source and a new fault. *(2026-10-01: R1 7, for the director's review; R2 39,
   40; R3 63.)*
7. **Escalation halts, and keeps the first fault.** A fault that is not containable, or is made so by rule 3,
   enters the fatal handler. From then no job runs, no release is processed or latched, and no transition occurs;
   the handler ends, within a bound the runtime's catalog record declares beside the nesting bound `M`, in a
   terminal state with interrupts masked. It preserves until reset the **first** such fault only, written before
   anything else the handler does: its §3.1 kind; its §8.1 class; the attributed task's stable logical ID — its
   elaborated eADL task name, the instance path of `docs/semantics/reference.md` §6 rule 9, which `archogen check`
   requires unique (`schema-duplicate-name`) — or no task, with the executing context named, a service by its
   source; the task it interrupted, if any; for a stack guard, whose guard was hit; and whether rule 3 escalated it,
   which in this profile it never does. A runtime that holds tasks by an internal index has the plan supply the ID
   when the record leaves it (§7.5): an index names no task in a record. A fault taken in the handler ends it at
   once in its terminal state and never replaces the preserved one. What the task table shows afterwards is the
   implementation's, and is not evidence of attribution. *(2026-10-01: R1 9; R2 34, 44, 48; R3 65, 68, 75, 80.)*

Two smaller decisions are fixed at the same time, for the same reason:

- **A task set must be non-empty** — no workload makes §7.2's second timing obligation vacuous. `archogen build` and
  boot refuse one; a refused boot is no fault: the runtime never starts, and nothing is kept but the refusal.
  `archogen check` admits one, a system describing composition alone (F01) being valid, and makes no timing claim
  for it (`M2.17`; R3 79).
- **Kernel critical sections are bounded by a declared nesting depth.** "Bounded kernel critical sections" did not
  say what the bound is or what happens at it. Each runtime's catalog record declares a maximum depth `M ≥ 1`. A
  `mask` that would raise the depth past `M`, and an `unmask` at depth zero, are **assertion failures**: a counter
  that *wraps* re-enables interrupts inside a critical section while reporting success, one that *saturates* stops
  counting, and one that *refuses* leaves its caller's matching `unmask` to close the section early. This bounds
  the depth; the duration is bounded separately, by the `CS_i` the timing analysis charges. *(2026-10-01: R1 10; a
  refusal had been the answer.)*

**Still open:** each path's observation events and hosted/target trace compatibility, a discarded release's and a
completion that closes a region included (`M2.15`); the catalog fact by which an external source's record says
whether arrivals during a pending request are counted (rule 1; the runtime records, `M2.7.4`); and, needed by no
fixture yet, a later idle-to-task dispatch's cost and a periodic task's first release instant.
