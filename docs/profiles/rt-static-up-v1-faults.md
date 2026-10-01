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

**Amendments, 2026-10-01 and 2026-10-02 (leaves `M2.9`, `M2.17`, `M2.19`).** They answer the director's rulings
(findings §6) and nine independent reviews of this text, *R1*–*R9*, whose findings and answers are tabled in
`docs/decisions/decision_runtime-contract-gaps.md`; the rule each answer touched is marked below.
- **Corrections of the 2026-09-13 text** (§14.1): a second release latched in a masked region is contained, not fatal
  (rule 1); a completion closes its region (rule 4); a fault other than an overrun is the context's that raises it, and
  a service's is no task's, where the table said "the running task" (rule 2); the mask bound and an unbalanced `unmask`
  are assertion failures, where they were refusals (the second smaller decision).
- **Narrowings**, each a claim the earlier text did not limit, for the director's review: no overrun is raised without a
  release (rule 1a); a missed deadline is reported only as rule 6 says, and §13.1 F26 with it; an externally released
  task's second arrival during a pending request can be lost where its source does not count arrivals (rule 1); a `mask`
  or `unmask` outside a job is an assertion failure (Terms) — initialisation's included, which the composition's
  `leaves-interrupt-hardware-alone` had admitted; and a broken port obligation the trap path, a service or the runtime
  API's entry can see — an external trap's empty first claim, a timer trap with no release due, a claim returning an
  undeclared source, an API entry naming neither a primitive nor the completion path — is an unexpected trap, and halts
  (Terms); a port's runtime-API trap, a primitive's or the completion path's, serves no interrupt (rule 2,
  `2026-10-02`); and the image's panic handler is the port's, an application's own refused at build, and the panic
  strategy the port's, another refused at build, and a panic an assertion failure unless its check finds a guard or an
  unexpected trap; what can happen between a failed check and the handler's first act is the port's to state, whole, in
  its catalog record (Terms, `2026-10-02`).
- **What moved an implementation**, change by change: the record's *What changed an implementation*. `M2.17`
  located the empty set's refusal in `archogen build` and boot; `M2.19` moved this text out of `ROADMAP.md`.
- **Evidence invalidated** (§15): none. No version of `rt-static-up-v1` is released or locked, no released result
  rests on the earlier answers, and both models and their tests moved with each change; so no version is advanced.

**Terms.**
- A **masked region** is the interval in which the runtime's mask nesting depth is above zero: opened by a job's
  outermost `mask` and closed by the matching `unmask` or by that job's completion (rule 4). A **section** is one
  `mask`…`unmask` pair inside a region. §3.1's bounded kernel critical sections are both the regions — bounded in depth
  by `M` (the second smaller decision) and in duration by `CS_i` — and the processor's own interrupt disable, bounded by
  the composition's `masked.p`, `masked.completion` and `L`'s service, transition and idle-wake terms; initialisation's
  is kept out of `L` by its preceding every release (the composition's `releases-after-initialisation`), and the fatal
  handler's is bounded by rule 7. A task's masked runs are regions. That disable — in a primitive, a service, the trap
  path, a transition, the completion path, idle, initialisation or the fatal handler — is **not** a masked region, and
  §13.4's "latched" means pending in hardware there.
- **Only a job, or its completion (rule 4), changes the depth.** A `mask` or `unmask` is a job's only when the job's own
  code, or a primitive it called, executes it. Executed by any other context — the completion path, a service, the trap
  path, a transition, idle, initialisation or the fatal handler — it is an assertion failure of that context (the
  completion path's attributed to its task, as rule 2 says), raised before the depth changes, whether or not a job is
  preempted beneath it. A runtime that is told no context detects the case it can see, a call with no job running. A
  port must detect the rest — a call from any context but a job, whatever its interrupt state or trap nesting at the
  call — and raise it; how it tells a job's call apart, a call entered through the port's own API trap included, it
  states in its catalog record, where it is reviewed with the port (`M2.12`). The completion path, entered by any
  context but a job, is an assertion failure of that context, as a `mask` is. **Initialisation** is no job, so it calls
  no masking primitive — running before the first enabling of interrupts with interrupts already disabled, it has
  nothing to mask. This narrows what the composition's `leaves-interrupt-hardware-alone` admits: its masking primitives
  are a job's only. A depth above zero when a job is dispatched or resumed, or idle is entered, is likewise an assertion
  failure, raised by the transition — no task's, even where a port decides inside the completion path — so it
  interrupts, as rule 2 says, the task whose job the transition starts from while that job is still owed, and none
  otherwise. So every section open at a completion is the completing job's.
- A task's **latch** is what holds its releases that arrive while a masked region is open. On the target that is
  the interrupt pending in hardware, or the timer's releases due, which the timer service computes at delivery (the
  composition's `releases-never-latched`); in a hosted model, the runtime's own record: the most recent release, and
  a mark that an earlier one also came. A hosted model judges the earlier arrivals its mark stands for as one, so
  for one delivery it can report fewer overruns than the target, which judges each due release. For a
  timer-released task, and an external source whose record states it counts arrivals, the task's state after the
  delivery is the same and `M2.15` maps the events; for any other source the target can lose an arrival the hosted
  mark keeps (rule 1), and the states can differ.
- A job **completes** at its last instruction of its own code (§13.4). A task **owes a job** from its release until that
  job completes or is abandoned. A release observed after the job's last instruction of its own code — while its
  completion path runs, before or after the completion is recorded, or in the instant between that instruction and the
  path's first — finds the task owing no job. Until its own decided switch, that job's context — its completion path, or
  the instant before it — runs at its task's priority though the task owes no job. While such a context is preempted,
  every switch, whatever precedes it, resumes the highest-priority preempted context of this kind unless a task of
  higher priority than its task owes a job, and otherwise dispatches the highest-priority task owing a job, a job
  observed meanwhile included; a job of a task whose completion-path context is waiting starts at its entry only when
  that context's own decided switch dispatches it. A decision a service preempted is made again, or the port lets no
  service preempt the decision. A task faulted meanwhile (rule 5) keeps that context, which runs on at the task's
  priority to its decided switch, and that switch dispatches no job of the faulted task; only then has the task left the
  schedule. A port may instead let no service preempt that interval. Which it does, and how it guarantees either, it
  states in its catalog record, reviewed there (`M2.12`).
- An **unexpected trap** is any trap other than the timer's interrupt with a release due, a declared source's interrupt
  whose claim finds a request, the runtime API's own entry, naming a primitive or the completion path, where the port
  uses one, and a trap that is another fault's mechanism: an access fault on a stack's guard is a stack guard, raised by
  the context that executed the access, the trap path's reading of it being no check of its own. Its cases, each with
  the context that raises it (rule 2):
  - an interrupt that is neither the timer's nor a machine external one, whether or not the port serves it; an external
    trap whose first claim finds no request, since the composition's `no-empty-claim` makes it a port's broken
    obligation; a claim in a trap that returns a source the plan does not declare, at that claim; and a timer trap that
    finds no release due, or a timer service a port runs in an external trap for a further pending timer interrupt that
    finds none, since `raised-only-when-due`, the variant's platform fact the timer-service record supplies
    (`docs/specs/catalog/decision_catalog-records-variant-inputs.md`), makes it a port's broken obligation — each raised
    by the trap path, wherever the port's check runs. Any other claim that finds none — a later claim in the same trap,
    or one a port makes in a timer trap to serve a further pending interrupt (rule 2) — ends its service loop and is no
    fault, and a claim initialisation makes of a request left pending from before it is no fault, whatever source it
    returns. A faulted task's later nominal releases are releases due for the timer-trap test: the timer service
    performs each, and rule 5 discards it;
  - an entry of the runtime API that names neither a primitive nor the completion path, raised by the context that
    executed it;
  - any exception the port does not serve, and any other trap code executes, that is no other fault's mechanism —
    application code's included, whatever path the port routes it through — raised by the context that executed it.

  **A check that finds a fault**, made by the runtime's, a port's, a catalog record's or generated code and passing the
  kind it finds as the port's catalog record states, is classified by what it finds, whichever context makes it and
  however it then enters the fatal path: one that finds one of the cases above is that unexpected trap, raised as the
  case says; one that finds a stack's guard reached is a stack guard, raised by the context that made the check, and
  which context runs each such check the port states in its catalog record (`M2.12`). A check in application code is
  classified by how it ends, whatever it finds: a panic is an assertion failure, any other trap an unexpected trap of
  the context that executed it. Any other deliberate trap on a failed check of the runtime's or the port's own
  invariants, and **any other panic** — a Rust `panic!`, `assert!`, `unwrap`, `expect`, `todo!` or `unreachable!`, and a
  bounds or overflow check, in any code of the image: the application's, `core`'s, any catalog record's, generated code
  — is an assertion failure, raised by the context that made the failed check — the one whose failure the panic reports
  — or, for a panic no check precedes (`todo!`, `unimplemented!`, an unconditional `panic!`), the context that executed
  it, attributed as rule 2 says; a job that panics never completes. A panic reaches the fatal path through the image's
  panic handler, the fault path's entry, which the port's catalog record supplies (`M2.12`) — an image whose application
  supplies its own does not build — and whose first act masks interrupts; under a panic strategy that aborts without
  calling it, by the trap at the check. The strategy the image is built with is part of the port's panic path, stated in
  its catalog record; an image built with another does not build, as one whose application supplies its own handler does
  not. What can happen between a failed check and that act or that trap — whether a trap can preempt the panicking
  context there, whether a release then observed can abandon its job, and what runs before it reaches the handler —
  depends on how the port builds its panic path, and is the port's to state, whole, in its catalog record, reviewed
  there with the port's design; this contract fixes the panic's kind, its raiser and its handler, not that window.
  Application code enters the fatal path only by a panic or a trap — an unexpected trap, a guard's access fault, or the
  trap a check executes under an aborting strategy — never by calling the fault path itself (the composition's
  `leaves-interrupt-hardware-alone`). A boot is refused without a panic, so the handler keeps nothing for it. A job that
  never completes is found only as rule 1 finds an overrun: at its task's next observed release, never while it holds a
  masked region, and never for a task released no more. The assertion failures this contract names are not all there
  are: any other check a runtime or a port makes of its own invariants raises one.

*(Terms: 2026-10-01: R2 33, 35, 37, 45, 47; R3 54–56, 58, 60–62, 64, 70; R4 82, 84–87, 89, 90, 95, 96, 100; R5 106, 110,
111, 118–120; R6 124, 126, 127, 130, 131, 138, 139; R7 140, 142, 144, 147–149; 2026-10-02: R8 152, 153, 156, 161, and
its pre-landing checks; R9 163, 165, 166, 168, 171, 172, and its pre-landing check.)*

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
   each is an overrun in turn; under `Fault` the first overrun, whichever arrival it is, faults the task, and every
   arrival after it is discarded. Across tasks the order changes no task's state; across interrupts it is the hardware's
   and the plan's (the composition's `ahead`), among interrupts one trap serves the port's code's, stated with rule 2's
   choice, and within one service that service's code's, stated in its record (`M2.15` traces it). Where a fatal fault
   interrupts a delivery, the order of its interrupts, and whether the port returns between them, decide which job the
   faulting trap preempted and whether it was still owed (rule 2). So no release that finds its task owing a job at
   delivery escapes its policy, nor is fatal for landing inside a region rather than one instruction after it — for
   arrivals the platform delivers as distinct requests. A timer-released task's are computed at delivery, one per
   nominal release due, on every port. An externally released task's are distinct only where its source's catalog record
   (written under `M2.7.4`) states that arrivals while its request is pending or claimed are counted — a fact no record
   states yet — and otherwise the record states that such an arrival, and its overrun, can be lost. The release that
   triggers an overrun is the policy's: under `SkipLateJob` it makes the task ready for its next job, with that
   release's nominal instant; under `Fault` it goes with the faulted task. A completion and a release at one instant:
   the completion first (§13.4). *(2026-10-01: §6 (b); R1 6, 14, 22; R2 31, 37, 42, 49; R3 64, 69, 76; R4 83, 97, 102;
   R5 116; R6 129; R7 141, 143; 2026-10-02: R8 160.)*

   1a. **An overrun raised without a release is outside this profile.** `rt-static-up-v1` has no execution-budget
   monitor: an overrun is detected by rule 1 alone, as rule 6 says of a deadline. A port that raises one another
   way — a monitor's interrupt, or a bound checked synchronously — is outside the profile. A later profile that
   admits one must say how its source is declared (§3.1, §7.3) and charged, and how its overrun is judged against
   the job it measured. *(2026-10-01: R1 5; narrowed by R2 26, 27, 29, 40, for the director's review.)*
2. **Attribution follows the fault, not the processor.** An overrun is attributed to the overrunning task — the task a
   release, or a delivered latched release, belongs to — whichever context holds the processor: a task that is ready and
   not running, the task whose job the release service interrupted, or the task executing its own outermost `unmask` or
   completion. The single-core rule of §3.1 governs *execution*, not *attribution*. The other three faults are
   attributed to the context that raises them. An exception — a guard's access fault, any other faulting instruction —
   is raised by the context that executed the instruction, except a deliberate trap a check of the runtime's, a port's,
   a catalog record's or generated code executes, which is the fault the check finds, raised by the context the Terms
   name for it; an assertion by the context that made the failed check or, for a panic no check precedes, the one that
   executed it (Terms); and an unexpected trap the Terms give the trap path — an interrupt neither the timer's nor a
   machine external one, an external trap's empty first claim, an undeclared source, a timer trap with no release due —
   by the trap path. A task's job is that task, including a runtime primitive it called however the port enters it, and
   the completion path it entered. Where the port enters the runtime API by a trap, a primitive's or the completion
   path's, that trap serves no interrupt. A primitive's trap, from its entry to its return, is the primitive, not the
   trap path; an interrupt pending when it returns is taken as a trap of its own when interrupts are next unmasked — at
   once if the call leaves the depth at zero, preempting the job, otherwise when the region closes: at its outermost
   `unmask`, preempting the job, or at the job's completion, inside the transition that follows (rule 4). The completion
   path's trap is the completion path up to the decision and the transition after it; an interrupt pending when that
   transition unmasks is taken as a trap inside the transition (below). A service, the trap path, a transition, idle,
   initialisation or the fatal handler is no task: the evidence names that context, a service by its source, and records
   the task it interrupted as interrupted, never as attributed. A service or the trap path interrupts the task whose job
   its trap preempted while that job is still owed, and none if the trap preempted idle or that job has completed or
   been abandoned since, even where its task owes a newer one. A transition, from the decision's first instruction to
   the incoming context's first, interrupts the task whose job it starts from — the job the preceding trap preempted, or
   the job whose completion path decided it — while that job is still owed, whether it switches away from that job or
   returns to it, and none if it starts from idle or that job has completed or been abandoned, even where its task owes
   a newer one. It never interrupts a task it switches to and did not start from. For attribution the completion path
   ends where the scheduling decision begins: the decision, and everything after it up to the incoming context's first
   instruction, is the transition's wherever a port places it, whatever the composition's `completion` cost spans. A
   trap taken for an interrupt is the trap path from its entry until it has identified what it serves — the timer by the
   trap's cause, an external source by a claim that returns it — then that interrupt's service to the service's last
   instruction, and the trap path again between services and from the last to the trap's return or the following
   transition. A timer trap with no release due, an external trap's empty first claim and a claim returning an
   undeclared source are the trap path's wherever the port's check runs. Whether a port serves a further pending
   interrupt in the same trap or returns and takes a new one, it states in its catalog record (`M2.12`); the interrupted
   task follows from it. A trap taken inside a transition, after it unmasks and before the incoming context's first
   instruction, preempted the incoming context: the trap path and its services interrupt the task whose job that is,
   while owed, and none if it is idle or a completion path. A trap taken during a decision that a service may preempt
   preempted that completion path's context, and interrupts no task. The composition's timing boundaries are unchanged.
   A stack guard is attributed the same way, and the evidence names whose guard was hit — a task's, the interrupt
   stack's, or another stack the port guards, named; every stack whose bound is not established has a guard (§7.6).
   *(2026-10-01: §6 (d); R1 4, 8; R2 34, 46; R3 57–59; R4 84, 87, 88, 104; R5 108–110, 118, 121; R6 124, 133; R7 141,
   146; 2026-10-02: R8 152, 155; R9 163, 167, 171.)*
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
   fault other than an overrun raised during a delivery is, on a port, the context's that raises it, as rule 2 says —
   the trap path's, a service's, or a transition's between or after its traps — and no task's. One raised by the trap
   path or a service interrupts the task whose job that fault's own trap preempted, while that job is still owed: for
   the delivery's first trap, the task whose job executed the outermost `unmask` or, after a completion, the task of the
   context the completion's transition resumed or dispatched; for a later trap, the task of the context the preceding
   trap's transition resumed or dispatched; and none if that is idle or a completion path, or that job has completed or
   been abandoned since, even where its task owes a newer one. One raised by a transition interrupts the task whose job
   it starts from, while that job is owed (rule 2). In a hosted model that delivers inside `unmask` or the completion
   path, such a fault is that task's; one that delivers after its decision attributes it as a port does. `M2.15` maps
   the two records as it maps their traces. A completion is not a fault: the job's own code has finished, so neither
   ground of rule 3 reaches it, and it is the masked run that ends at completion, which the timing analysis charges
   (`docs/specs/catalog/decision_runtime-composite-inputs.md`, `CS_i`). A latched release is judged at that delivery
   (rule 1), so a task that completed inside the region is released afresh, not overrun. *(2026-10-01: §6 (a); R1 16;
   R2 30; R3 73; R4 81; R5 107; R6 125; R7 141; 2026-10-02: R8 152, 159, 162.)*
5. **The two overrun policies.** `SkipLateJob` (eADL `skip-late-job`): the task's owed job is abandoned — not started,
   preempted, or the job the release interrupted. Its remaining code never runs and no completion is recorded for it, no
   fault halts anything, and the triggering release makes the task ready for its next job (rule 1). `Fault` (eADL
   `fault`): the task is faulted and leaves the schedule until reset. Its owed job is abandoned, the triggering release
   and every later release of it are discarded, and no further instruction of its own code runs; every other task
   continues, and the runtime does not halt. A task without an `on-overrun` clause has `Fault`, and `archogen check`
   refuses a policy other than these two (`docs/semantics/model.md` §4 rule 6, leaf `M2.14`). A job is abandoned only
   where it holds no masked region: in its own code with the depth at zero, or at a primitive's entry or return where
   the depth is zero; for a panicking job, before its handler's first act, as the port's catalog record states (Terms).
   A policy applied while the job is inside a primitive takes effect at the primitive's entry if the primitive has not
   yet changed the runtime's state — for `mask`, before the depth is raised — and otherwise at its return, which a
   policy never reaches while the primitive holds the depth raised, since no release is observed inside a region
   (rule 1). Where the policy takes effect at a primitive's return, the runtime completes the primitive on the task's
   behalf — under `Fault`, which is not the task's own code, and under `SkipLateJob` before the next job starts; where
   it takes effect at the entry, the primitive is not run. For attribution, a job whose policy takes effect at a
   primitive's return is abandoned when the policy is applied. Containment keeps the runtime's state consistent and the
   schedule running, and claims nothing about the application state the abandoned job left (§3.1, Isolation). A run's
   timing claims end at its first contained overrun, which shows that an assumption of its analysis — an execution
   bound, an arrival bound or the interference it counted — did not hold, or that the claim was not established.
   *(2026-10-01: R1 2, 11, 25; R2 28, 32, 41; R3 66, 72, 76; R4 93, 98; R5 112; R6 125; 2026-10-02: R8 153.)*
6. **The runtime detects no missed deadline.** Rule 1's overrun is its only timing fault. A miss shows only as that
   overrun, and only if the job is still owed when the task's next release is observed. That release is never observed
   before the deadline (§3.1's constrained deadlines). Only with `D = T`, strictly periodic releases and zero declared
   jitter must it fall at the deadline, so that a job owed at its deadline is still owed when the release is observed —
   unless it completes within its release's latency (the composition's `J_i^release`), or inside a region where the
   release was latched (rule 4), and then the miss is not reported. Otherwise the release can fall later, and a job that
   misses its deadline and completes before its next release is observed is never reported. §13.1 F26 exercises a miss
   with `D = T`, strictly periodic releases and zero declared jitter, in which the job is still owed when the next
   release is observed; any other miss is the analysis's to exclude and a trace's to observe. A deadline monitor would
   be a new source and a new fault. *(2026-10-01: R1 7, for the director's review; R2 39, 40; R3 63; R4 92, 99; R5 117;
   R6 128; R7 145.)*
7. **Escalation halts, and keeps the first fault.** A fault that is not containable, or is made so by rule 3, enters the
   fatal handler — a panic, from the panic handler's first act, which masks interrupts, or from the trap that aborts at
   its check. From then no job runs, no release is processed, no pending interrupt is taken, and no transition occurs;
   the handler ends, within a bound the port's catalog record declares, the fault path being the port's (the
   composition's role list), in a terminal state with interrupts masked. It preserves until reset the **first** such
   fault only, written, once interrupts are masked, before anything else the handler does but determine its fields: its
   §3.1 kind; its §8.1 class; the attributed task's stable logical ID — its elaborated eADL task name, the instance path
   of `docs/semantics/reference.md` §6 rule 9, which `archogen check` requires unique (`schema-duplicate-name`) — or no
   task, with the raising context named, a service by its source; the task it interrupted, if any; for a stack guard,
   whose guard was hit; and whether rule 3 escalated it, which in this profile it never does. The record carries a mark
   of three values: empty from boot, begun by the record's first write, and complete by its last, which follows every
   field and precedes anything else the handler does. A fault taken after the mark is begun and before it is complete
   ends the handler with the mark at begun and the fields already written kept; one taken at or before that first write
   leaves it empty, so an empty mark on a halted runtime records a fault taken before the record began. A runtime that
   holds tasks by an internal index has the plan supply the ID when the record leaves it (§7.5): an index names no task
   in a record. A fault taken in the handler ends it at once in its terminal state and never replaces the preserved one.
   What the task table shows afterwards is the implementation's, and is not evidence of attribution. *(2026-10-01: R1 9;
   R2 34, 44, 48; R3 65, 68, 75, 80; R4 80, 103; R5 113; R6 133, 135; R7 150; 2026-10-02: R8 153, 154; R9 175.)*

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

**What §13.1 F26 exercises.** The roadmap's F26 row points here, so the list is kept in one place: an answer that
classifies a new case adds it here in the same change. Each is a fixture for `M4`/`M5`, its expected outcome — the
schedule, and for a fatal fault the kept record — the rule's:
- an overrun under both policies, at arrival and at delivery, several latched arrivals of one task judged in arrival
  order among them, and a missed deadline where the job is still owed at its next observed release — `D = T`, strictly
  periodic, zero declared jitter, the narrowing for the director's review (rules 1, 4, 5, 6);
- a policy applied inside a primitive, at its entry and at its return, in a hosted model and on a port whose primitives
  can be preempted (rule 5); a completion and a release at one instant; a task that completed inside the region released
  afresh (rules 1, 4);
- a stack guard, reached by an access and found by a check, trapping or panicking (Terms);
- an unexpected trap: an interrupt neither the timer's nor a machine external one, an external trap's empty first claim,
  a timer trap with no release due, a claim returning an undeclared source, an API entry naming neither a primitive nor
  the completion path — each also found by a check that panics or traps — an exception the port does not serve and any
  other trap code executes, by a job, a service, the trap path, a transition, idle, initialisation and the completion
  path, each raised by its executor; and the cases that are no fault — a later claim that finds none, a claim a port
  makes in a timer trap that finds none, a timer trap whose only due release is a faulted task's (Terms);
- an assertion failure: the mask bound, an unbalanced `unmask`, a `mask` or `unmask` outside a job, the completion path
  entered by a non-job, a job dispatched or resumed or idle entered inside a region, a failed check of the runtime's or
  the port's invariants (Terms; the second smaller decision);
- a release observed after a job's last instruction, and a service in that interval under the port's stated choice, a
  waiting completion-path context resumed ahead of its own task's new job and a task faulted while its context waits
  among them (Terms);
- the interrupted task of a fault raised by a transition during a delivery, of one in a later trap of a delivery
  spanning several traps, of one whose preempted job was abandoned since, of a trap taken inside a transition after it
  unmasks, and of one taken during a decision a service may preempt (rules 2, 4);
- an overrun attributed to a task that is ready and not running, and to one executing its own outermost `unmask` or
  completion; a fault on the trap path between two services of one trap; a fault raised by a service or the trap path
  whose trap preempted idle, which interrupts no task; an unexpected trap found by a service's check, the trap path's
  (rule 2);
- a panic: in application code, `core`, any catalog record's code and generated code, in a job and in a service, the
  trap path, a transition, idle, initialisation and the completion path, each raised by its context; a guard or an
  unexpected trap found by a check that panics; each of these, and any other panic, also on a port whose stated strategy
  aborts; an application check that finds a guard, classified by how it ends; an application's own handler refused at
  build; and, as the port's catalog record states it, a trap and a release observed between a failed check and the
  handler's first act (Terms);
- an interrupt pending when a runtime-API trap returns, taken at once, at the outermost `unmask`, and inside the
  transition after a completion (rule 2); a transition that returns to the job it started from, and one that switches to
  a job it did not start from (rule 2);
- an external task's arrival lost during a pending request where its source does not count arrivals, and a task with no
  `on-overrun` clause taking `Fault` (rules 1, 5); a job that never completes, unseen while it holds a region and for a
  task released no more (Terms);
- a fault in the fatal handler at or before the record's first write, before its mark is complete and after it, and the
  kept record's fields (rule 7); a refused boot, which is no fault (the first smaller decision).
*(2026-10-02: R8 157, and its pre-landing checks, the first of which found the list incomplete in the change that made
it; R9 164, 171, 174.)*

**Still open:** each path's observation events and hosted/target trace compatibility, a discarded release's and a
completion that closes a region included (`M2.15`); the catalog fact by which an external source's record says whether
arrivals during a pending request are counted (rule 1; the source's catalog record, written under `M2.7.4`); how a port
detects a non-job call, what it lets run in the interval after a job's last instruction and which context runs each
guard check (Terms), whether it serves a further pending interrupt in the same trap (rule 2), what can happen between a
failed check and the panic handler's first act, the panic strategy the image is built with, how a check passes the kind
it names, and for a guard whose, to the handler — or, under an aborting strategy, to the trap path, and how that trap is
told from any other trap (Terms), the fatal handler's bound, and how the kept record is read out of a halted runtime
(rule 7) — each the port's catalog record's (`M2.12`); each service's order of the releases it performs, its own
record's (rule 1); and two that are not this contract's and that no fixture needs yet: a later idle-to-task dispatch's
cost, the port's record's (`M2.12`), and a periodic task's first release instant, the timer-service record's (written
under `M2.7.4`).
