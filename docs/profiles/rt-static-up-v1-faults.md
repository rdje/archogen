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

**Amendments, 2026-10-01 (leaf `M2.9`).** Two **correct** the 2026-09-13 text (findings §6 (a), (b)): a second
release latched in a masked region is contained, not fatal (rule 1), and a completion closes its region (rule 4).
The rest answer two independent reviews of this text, *R1* and *R2* below; every change that moved an
implementation is listed in `decision_runtime-contract-gaps.md`, *What changed an implementation*. No version of
`rt-static-up-v1` is released or locked (§15), so none is advanced.

**Terms.** A **masked region** is the interval in which the runtime's mask nesting depth is above zero: opened by
a job's outermost `mask`, kernel or application, and closed by the matching `unmask` or by that job's completion
(rule 4). A **section** is one `mask`…`unmask` pair inside a region; §3.1's "kernel critical section" is a region,
and the composition's "masked run" is a region's extent. The processor's own interrupt disable — in a service, the
trap path, a transition, the completion path, idle or the fatal handler — is **not** a masked region, and §13.4's
"latched" means pending in hardware there, not held in a task's latch. **Only a job changes the depth:** a `mask`
or `unmask` executed with no job running is an assertion failure of the executing context, and so is a depth above
zero when a job would start — raised by that decision, which is no task's. So every section open at a completion
is the completing job's. A task's **latch** holds the most recent release that arrived inside a masked region, and
a mark that an earlier one also did. A task **owes a job** from its release until that job completes or is
abandoned. An **unexpected trap** is any trap other than the timer's interrupt, a declared source's interrupt whose
claim finds a request, and the runtime API's own entry where the port uses one; a claim that finds no request is
one, since the composition's `no-empty-claim` makes it a port's broken obligation.

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
   region included) and an overrun if it does; the next is an overrun, because the first left a job owed; the
   earlier arrivals the mark stands for are judged as one. Across tasks the order changes no task's state and is
   the port's (`M2.15` traces it). So an overrun is not lost, nor fatal for landing inside a region rather than one
   instruction after it — for arrivals the platform delivers as distinct requests: a timer-released task's are
   computed at delivery and never coalesce; an externally released task's are distinct only where its source's
   record states that arrivals during a pending request are counted, and otherwise a port records that a second
   one in a masked interval, and its overrun, can be lost. The release that triggers an overrun is the policy's:
   under `SkipLateJob` it becomes the task's next job, the latched release with its nominal instant; under `Fault`
   it goes with the faulted task. A completion and a release at one instant: the completion first (§13.4).
   *(2026-10-01: §6 (b); R1 6, 14, 22; R2 31, 37, 42, 49.)*

   1a. **An overrun raised without a release is outside this profile.** `rt-static-up-v1` has no execution-budget
   monitor: an overrun is detected by rule 1 alone, as rule 6 says of a deadline. A port that raises one another
   way — a monitor's interrupt, or a bound checked synchronously — is outside the profile. A later profile that
   admits one must say how its source is declared (§3.1, §7.3) and charged, and how its overrun is judged against
   the job it measured. *(2026-10-01: R1 5; narrowed by R2 26, 27, 29, 40, for the director's review.)*
2. **Attribution follows the fault, not the processor.** An overrun is attributed to the overrunning task — the
   task a release, or a delivered latched release, belongs to — whichever context holds the processor: a task that
   is ready and not running, the task whose job the release service interrupted, or the task executing its own
   outermost `unmask` or completion. The single-core rule of §3.1 governs *execution*, not *attribution*. The other
   three faults are synchronous to the context executing the faulting instruction and are attributed to it. A
   task's job, including a runtime primitive or the completion path it called, is that task. A service, the trap
   path, a transition, idle or the fatal handler is no task: the evidence names that context, and records the task
   it interrupted as interrupted, never as attributed. A service or the trap path interrupts the task whose job it
   preempted, or none if it preempted idle; a transition interrupts the task the runtime holds as running when the
   fault is raised — the incoming one once the switch is decided, the outgoing one, or none, while it is being
   decided. A stack guard is attributed the same way, and the evidence names whose guard was hit — a task's, or the
   interrupt stack's. *(2026-10-01: §6 (d); R1 4, 8; R2 34, 46.)*
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
   transition unmasks — or follows it — a hosted model — is the port's: both reach the same state, and `M2.15`
   states how a trace shows each. A completion is not a fault: the job's own code has finished, so neither ground
   of rule 3 reaches it, and it is the masked run that ends at completion, which the timing analysis charges
   (`docs/specs/catalog/decision_runtime-composite-inputs.md`, `CS_i`). A latched release is judged at that
   delivery (rule 1), so a task that completed inside the region is released afresh, not overrun. *(2026-10-01:
   §6 (a); R1 16; R2 30.)*
5. **The two overrun policies.** `SkipLateJob` (eADL `skip-late-job`): the task's owed job is abandoned — not
   started, preempted, or the job the release interrupted. Its remaining code never runs and no completion is
   recorded for it, no fault halts anything, and the triggering release starts the task's next job (rule 1).
   `Fault` (eADL `fault`): the task is faulted and leaves the schedule until reset. Its owed job is abandoned, the
   triggering release and every later release of it are discarded, and no further instruction of its own code runs;
   every other task continues, and the runtime does not halt. A task without an `on-overrun` clause has `Fault`, and
   `archogen check` refuses any other policy (`docs/semantics/model.md` §4 rule 6, leaf `M2.14`). A job is
   abandoned only where it holds no masked region: in its own code with the depth at zero, or at the entry or return
   of a primitive that leaves the depth at zero. A policy applied while the job is inside a primitive takes effect at
   the primitive's entry if the primitive has not yet changed the runtime's state — for `mask`, before the depth is
   raised — and otherwise at its return; under `Fault` the runtime completes or rolls back the primitive on the
   task's behalf, which is not the task's own code. Containment keeps the runtime's state consistent and the
   schedule running, and claims nothing about the application state the abandoned job left (§3.1, Isolation). A
   run's timing claims end at its first contained overrun, which shows that an assumption of its analysis — an
   execution bound, an arrival bound or the interference it counted — did not hold, or that the claim was not
   established. *(2026-10-01: R1 2, 11, 25; R2 28, 32, 41.)*
6. **The runtime detects no missed deadline.** Rule 1's overrun is its only timing fault. A missed deadline is
   observed only if the next release finds the job still owed — always with `D = T`, strictly periodic releases and
   zero jitter, the case §13.1 F26 exercises; any other miss is the analysis's to exclude and a trace's to observe.
   A deadline monitor would be a new source and a new fault. *(2026-10-01: R1 7, for the director's review; R2 39,
   40.)*
7. **Escalation halts, and keeps the first fault.** A fault that is not containable, or is made so by rule 3,
   enters the fatal handler. From then no job runs, no release is processed or latched, and no transition occurs;
   the handler ends, within a bound the runtime's catalog record declares beside the nesting bound `M`, in a
   terminal state with interrupts masked. It preserves until reset the **first** such fault only: its §3.1 kind,
   its §8.1 class, the attributed task's stable logical ID — its eADL task name, which `archogen check` requires
   unique (`schema-duplicate-name`) — or no task with the executing context, the task it interrupted, if any, and
   whether rule 3 escalated it. A runtime that records a task by an internal index records it with the plan that
   maps the index to the name (§7.5). A fault taken in the handler ends it at once in its terminal state and never
   replaces the preserved one. What the task table shows afterwards is the implementation's, and is not evidence of
   attribution. *(2026-10-01: R1 9; R2 34, 44, 48.)*

Two smaller decisions are fixed at the same time, for the same reason:

- **A task set must be non-empty** — no workload makes §7.2's second timing obligation vacuous. `archogen build` and
  boot refuse one; `archogen check` admits it, a system describing composition alone (F01) being valid (`M2.17`).
- **Kernel critical sections are bounded by a declared nesting depth.** "Bounded kernel critical sections" did not
  say what the bound is or what happens at it. Each runtime's catalog record declares a maximum depth `M ≥ 1`. A
  `mask` that would raise the depth past `M`, and an `unmask` at depth zero, are **assertion failures**: a counter
  that *wraps* re-enables interrupts inside a critical section while reporting success, one that *saturates* stops
  counting, and one that *refuses* leaves its caller's matching `unmask` to close the section early. This bounds
  the depth; the duration is bounded separately, by the `CS_i` the timing analysis charges. *(2026-10-01: R1 10; a
  refusal had been the answer.)*

**Still open:** each path's observation events and hosted/target trace compatibility (`M2.15`); and, needed by no
fixture yet, a later idle-to-task dispatch's cost and a periodic task's first release instant.
