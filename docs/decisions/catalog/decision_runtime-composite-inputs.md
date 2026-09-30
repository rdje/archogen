# The runtime variant's composite inputs: each part, its owner, and how the parts add up

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the RISC-V privileged specification](../../book/src/ledger.md#riscv-privileged), [the PLIC
  specification](../../book/src/ledger.md#riscv-plic) and [QEMU](../../book/src/ledger.md#qemu) — their versions, the
  sentences and the behaviour relied on, and their limits are in the ledger
- **Owner / source:** leaf `M2.10.1` (`docs/tasks/M2.md`). [[decision_catalog-records]] §12 left four of the
  runtime variant's inputs whole, `C_i`, `CS_i`, `J_i^release` and `J_s`, because each includes what no single
  owner knows. It also left them to the caller until this leaf, because a composition that took the larger of two
  parts where the definition needs their sum would under-charge (that record's second review, findings B2 and B3).
  This record decides the composition. Its independent reviews are summarised in the last section.

## The fact / decision

The engine composes each of the four inputs from parts, and each part has one owner:

- **the catalog:** kernel code and the platform, as reviewed timing costs and behavioral facts;
- **the application:** the task's own code, one of `ROADMAP.md` §10.3's "separately supplied application inputs";
- **the plan:** `ROADMAP.md` §7.5's resolved plan, which sets the interrupt controller's priorities;
- **the description:** its periods, jitters and what releases each task, as before.

The variant's §1 definitions, its conditions and its model, `fixed-priority-with-overheads/1`, are unchanged. This
record is how a value that meets each definition is put together, and §2 states what the composition itself
assumes, beyond the variant's conditions.

**The rule is the definition's own shape:**

- Where the definition covers code that runs **one part after another**, the composition **adds** the parts.
- Where the definition asks for the **longest of alternatives**, it takes the **maximum over the alternatives**,
  and each alternative is itself a sum.
- Where it asks for **everything that can run first**, it counts arrivals over the window, as the variant's own
  interference terms do.

No composite ever takes the maximum of things its definition adds.

### 1. The parts

| Part | Symbol | Owner | What it bounds |
| --- | --- | --- | --- |
| a primitive's cost | `api.p`, for each primitive `p` a task can call | catalog: timing cost `api.<p>` of the runtime API record, the one that supplies `completion` (the catalog record's §12 groups) | one call, from its first instruction to its last (§2's `primitives-out-of-line`), on the caller's behalf, excluding any service taken during the call or right after it |
| a primitive's masked run | `masked.p` | catalog: timing cost `masked.<p>` | the longest contiguous masked stretch inside one call of `p` made with interrupts unmasked |
| the completion path | `completion` | catalog: timing cost `completion` | from the job's last instruction of its own code to the decided switch that follows, the scheduling decision included |
| the completion path's masked run | `masked.completion` | catalog: timing cost `masked.completion` | the longest contiguous masked stretch in the completion path, up to the decided switch. The transition that follows it is added by `L`, as it is today |
| the task's own code | `C_i^app` | application, per task | the most one job executes of the task's own code, from its first instruction after the transition in to its last. It excludes every primitive's execution and every service, and includes the application's own instrumentation |
| how often it calls each primitive | `n_{i,p}` | application, per task | the most calls to `p` one job makes: §7.3's "allowed OS calls", with a count |
| the task's masked runs | for each run `r` and each way `e` it can end: `CS_{i,r,e}^app` and `n_{r,e,p}` | application, per task | a run is a stretch from a `mask` call to an `unmask` call that ends it, or to the job's completion when none does. A run that ends at `unmask` on one path and at completion on another has both endings. For each ending, `CS_{i,r,e}^app` is the most the task's own code executes from the `mask` to that ending, and `n_{r,e,p}` counts the calls to each primitive on the way, other than the `mask` and `unmask` that delimit it |
| the order among sources | `ahead(x)` for two sources | plan | a strict order over the declared sources, the interrupt controller's as the plan configures it: its priorities, with its own rule for ties. The plan also states that every declared source is deliverable, its priority above the controller's threshold, since an enabled source that is never taken has no bound |
| where the timer stands | `ahead(x)` between the timer and a source | catalog: hardware fact `external-before-timer` (§2) | `yes`: every source is taken before the timer when both are pending; `no`: the timer before every source, admitted only with the port fact `one-claim-per-trap` (§2) |

`ahead(x)` is every interrupt taken before `x` when both are pending.

The runtime API a task calls today (`crates/rt-core`, `Scheduler`) is `mask` and `unmask`, and its completion
path is `complete` followed by `decide`. The primitives are whatever the runtime API record's contract lists.

The parts the catalog already supplies keep their names: `ρ` `compare-rounding`, `δ` `delivery`, `S` `switch`,
`W_wake` `wake`, `C_rel` `timer-service`, and `C_s` `service.<source>` with its `no-application-code` fact.

### 2. What the composition assumes

The composition is sound only on a platform where the following hold. Each catalog fact below is a behavioral fact
(the behavior-model facet), with a `code` locator for a code fact. It comes from the record that the groups in the
catalog record's §12 name, checked per selection. A fact that is read and declared `no` puts the platform outside
what the composition covers: `unsupported-profile`, naming it. The one exception is `external-before-timer`, whose
`no` is covered when `one-claim-per-trap` is `yes` (its row). A fact that cannot be read leaves undeclared the
composites it gates, `analysis-inconclusive`, naming it:
- the timer's facts gate `J^release` of every timer-released task, and every `J` behind the timer;
- the interrupt and hardware facts, each source's `one-request-per-arrival.<source>` among them, gate every `J`,
  and so does `one-claim-per-trap` when `external-before-timer` is `no`;
- `pending-taken-after-unmask`, `releases-never-latched`, `primitives-out-of-line`, `starts-by-transition` and
  every `runtime-discipline.<id>` gate every composite;
- `leaves-interrupt-hardware-alone` is a task fact, so its absence is the variant's own condition 9.

| Fact | Kind | Says | Why the composition needs it |
| --- | --- | --- | --- |
| `reprograms-only-in-service` | code | the compare is written only by the timer service, after its due check, and by the timer-service record's code during initialisation, before the first enabling of interrupts. Each write sequence leaves the compare at the rounded earliest nominal release not yet performed, or at its largest value when none remains. The timer facts and `compare-rounding` hold of every compare write, initialisation's first included. The counter is written only by this record's code during initialisation, before the compare's first write, and never after | a late write then lands inside a service, and a counter moved back would delay every release. A first compare value rounded otherwise, or set for another instant, would delay the first release or raise a service that no release pays for |
| `releases-after-initialisation` | code | every nominal release is at or after the end of initialisation, the first enabling of interrupts | initialisation's masked run, and its compare write, are then over before any release they could delay |
| `pending-taken-after-unmask` | code, with the hardware's half established in its basis | a pending interrupt is taken before the instruction that follows any unmask, a task's included | two masked runs separated by an unmask stay separate. RISC-V's privileged specification requires interrupt conditions to be evaluated immediately after an explicit write to `mstatus` or `mie` |
| `releases-never-latched` | code | no service runs while the runtime's mask depth is raised: the port masks the hardware before raising it, and lowers it before unmasking the hardware. A completion entered with the depth raised lowers it before the transition that follows unmasks, and that work is inside `completion`. `rt-core`'s `complete` leaves the depth as it is, so this is the port's to do and the runtime record's review to check | `rt-core` latches a release that arrives while the depth is raised and delivers it on `unmask`. On the target that path is then never taken, so no `unmask` makes a task ready and no part has to charge the decision and switch that would follow |
| `primitives-out-of-line` | code | each primitive is compiled out of line, and its masking and unmasking instructions are compiler barriers | a call's boundaries, and a run's, are then instructions. The compiler cannot move the task's own code into a primitive or across a masking instruction |
| `external-before-timer` | hardware | whether the hardware takes a pending machine external interrupt before a pending machine timer interrupt, however the controller is configured | the timer's place in the order is the hardware's, not the plan's. The privileged specification takes machine external interrupts before machine timer interrupts; QEMU 11.1.1, without the Advanced Interrupt Architecture, takes the lowest pending cause first, the timer before external interrupts (the ledger's `qemu`), so the emulator target states `no`. With `no` the timer is ahead of every source (§1), and §3's queue for a source counts it. That holds only if each external trap serves one request and returns, since a port that claims again before returning, as the PLIC specification lets a handler do, would run several services while the timer waits: so `no` needs `one-claim-per-trap` `yes`, and is `unsupported-profile` without it |
| `one-claim-per-trap` | code, in the `switch` group | every trap taken for an interrupt serves exactly that interrupt: a timer trap claims nothing, and an external trap claims one request, runs its service and returns; an exception trap serves no interrupt. So an interrupt taken before external ones is taken between two source services, and nothing else is served between a service and its trap's return | read only when `external-before-timer` is `no`, where a claim loop would put several services ahead of the timer that §3's queue counts behind it |
| `external.<source>` | hardware, one per source, from the record that supplies `service.<source>` | the source reaches the hart as a machine external interrupt, through the controller's context for the hart's machine mode, whose priorities the plan sets. A source routed through a supervisor context is taken below the machine timer, and states `no` | the plan's order holds only for such sources. A source with `no`, such as a platform-defined local interrupt, a software interrupt, or one a priority scheme places among the timer, is outside the composition. A catalog reviewer sees the source's own wiring, never a description's list of sources |
| `no-empty-claim` | code, with the hardware's half established in its basis, in the `switch` group | every external trap finds a pending request to claim. How is the basis's to show: for example, the port's service exit waits until the controller's notification to the hart reflects the last claim, so a notification sent before that claim cannot trap the hart again. It is `yes` on a target with no external source, once `M2.12` lets a port fact be stated | the PLIC specification lets a notification "take some time to be received", holding a value "valid at some point in the past", and a claim then returns zero. Such a trap is a service that no request pays for, and repeated it can keep the timer from ever being taken |
| `one-request-per-arrival.<source>` | code, one per source, with the hardware's half established in its basis, from the record that supplies `service.<source>` | every request a service of the source claims was made by an arrival, and each arrival makes at most one. The basis shows it for the target's controller: for a PLIC gateway, that the source is edge-triggered with one edge per arrival, or that its level is deasserted at the gateway before the completion arrives there; for QEMU's `sifive_plic`, which pends a request on every raise of a source's line, even while the source is claimed, that from the first enabling of interrupts, and from each claim, until the source's next arrival, the line is raised only by that arrival, or while the request it raised is still pending and unclaimed | the PLIC specification forwards a new request on a completion "if the interrupt is level-triggered and the interrupt is still asserted", and QEMU pends one on any raise of the line. Either way a source can be served twice for one arrival, and step 4 counts services by arrivals. How the controller turns the line into requests is the source's wiring, and when its service lowers or raises the line is that service's code, so both are in that record's review |
| `starts-by-transition` | code, in the `switch` group | interrupts are first enabled by the transition into the first task's job or into idle, and no code that ran before it runs again. That transition is "the first enabling of interrupts" every other fact names | an application's `main` that carried on after it, or initialisation that ran on, would mask where no term of `L` counts it |
| `one-external-controller` | hardware, about the target, in no group | every external interrupt of the target reaches the processor through one controller. It is `yes` on a target with no external source | two controllers would each order their own sources, and no strict order the plan gives would be the hardware's. A reviewer of one source's wiring cannot see the others, so this is stated once, of the target |
| `leaves-interrupt-hardware-alone` | application fact, covering its tasks and its initialisation, and the caller's declaration for a service's application code | after the first enabling of interrupts, application code runs only as a declared task's job or as a service's application code; and no application code, a task's, a service's or application initialisation's, writes the timer's counter or compare, the interrupt controller's configuration, or the hart's interrupt or trap state other than through the runtime API's masking primitives, or reads or writes the controller's claim and complete registers, a read of which claims, or a declared source's device registers; no service calls a runtime primitive; and no application code releases a task or calls a runtime function other than the runtime API record's primitives and its completion path. A runtime function is any function of a catalog record's implementation, a driver's included, called after the first enabling of interrupts; before it, application initialisation reaches that hardware only through catalog functions, which `runtime-discipline.<id>` covers | `reprograms-only-in-service`, the plan's order, `releases-never-latched`, `no-empty-claim` and each `one-request-per-arrival.<source>` are statements about the whole image, and a code fact's locator reaches only catalog code. Each holds for the image only with this one, as `sections-mask-every-interrupt` holds only with the fourth task fact |
| `runtime-discipline.<id>` | code, one per record, stated by each record about every package its implementation's own and reached sets hold | that code writes the timer's counter or compare, the controller's configuration, or the hart's interrupt or trap state, reads or writes the controller's claim and complete registers or a declared source's device registers, and releases a task, only in a role listed below this table. No service in it calls a runtime primitive | the image-wide facts above rest on locators into their owners' code. Every other record compiled into the image, a driver or a service among them, states the same about itself, so no compiled code is covered by nothing. It is required `yes` from every record whose implementation facet, not `none`, is in the claim's closure, and from `M4` in the image's. Reading one adds its record's behavioral model to the closure, so the set is computed to a fixed point. Generated code is in no record: from `M4` the plan's generator states the same of it, and until then the conclusion names generated code, and any record compiled but outside the closure, as assumptions |

**The roles `runtime-discipline.<id>` allows.** A function's writes, claims and releases take the role of the
context that calls it, so a runtime function called from a service acts in that service's role, and the port's
dispatch, which claims before it knows the source, acts in the role of the service it dispatches to:
- the timer service writes the compare and releases its due timer tasks;
- a source's service claims and completes its own source, accesses its own device's registers, and releases the
  tasks bound to it;
- the runtime API's primitives, and the port's trap entry and exit, its transitions, its idle wake and its fault
  path, mask and unmask; the port's trap path and its transitions use `mscratch` where it keeps a stack there;
- initialisation, before the first enabling of interrupts, writes the controller's configuration and the hart's
  interrupt and trap state as the plan sets them. It may claim and complete requests left pending from before it,
  and it leaves none pending at the first enabling of interrupts that no arrival made. Two writes are one record's
  alone, so the facts that depend on them are reviewed with the code that makes them: the counter's first value,
  and the compare's, are written only by code of the record that supplies `timer-service`
  (`reprograms-only-in-service`); and a declared source's device is read and written only by code of the record
  that supplies `service.<source>`, in initialisation as at run time, whose `one-request-per-arrival.<source>`
  basis covers that configuration. Which record writes the controller's configuration and the hart's state is not
  fixed in `/1`; checking what the image wrote against the plan is `M4`'s (`M4.10`).

A path that another fact of this section makes unreachable is named in the basis with that fact. `rt-core`'s
`unmask` delivers a latched release, which is unreachable by `releases-never-latched`. Its `release` of a task whose
job is late, under `SkipLateJob`, is a release in the caller's role.

**The hart's interrupt state** is its global enable `mstatus.MIE`, its enables `mie`, its delegation `mideleg`, the
software interrupt `msip`, any hart-local priority, such as the Advanced Interrupt Architecture's `iprio`, and on a
hart with Sstc its `stimecmp` and the `STCE` bit of `menvcfg`. The timer these facts name is the one the
timer-service record's code programs.
`external-before-timer` is about the hart's order however these are set: a hart whose local priorities can place
the timer among external interrupts writes it `unknown`. **Its trap state** is `mtvec`, which decides what code a
trap runs, and `mscratch` where the port keeps a stack pointer there.

**Assumed of the environment**, named in the conclusion as arrival assumptions, like `T_s` (§6): no source arrives
before the first enabling of interrupts, and firmware or a boot loader leaves no request pending that
initialisation does not claim and complete. What code does after that is the facts' to state: requests a device
raises without an arrival are `one-request-per-arrival.<source>`'s, and initialisation's leftovers its role's.
**Assumed of the run:** no fault trap is taken. The fault path's role is there so its code is covered, and its
masked run is in no term of `L`. An arrival
during initialisation would wait out initialisation's masked run, which `L` does not hold, and no catalog fact can
vouch for when the environment's arrivals come.

**Boundaries the catalog's costs keep:**

- **Delivery ends where a service begins.** `δ` bounds two latencies, added together: from the arrival or compare
  match to the interrupt being pending, and from its being pending and unmasked to the processor taking the trap.
  `C_rel` and `C_s` begin when the trap is taken, which is what the variant's "entry" means here. A caller-supplied
  `C_s` begins there too. So everything masked after the processor commits to the trap is inside a service, where
  `L` counts it. The variant's `δ` runs from pending and unmasked to entry, so this `δ` is at least that one, which
  keeps its floors safe.
- **Delivery is time, not a barrier.** While an interrupt is pending and unmasked and the trap is not yet taken,
  the running code can still retire instructions, a masking one among them. The specification bounds that latency
  and forbids instructions only after an explicit write to `mstatus` or `mie`. So a masked stretch can begin after
  the interrupt is pending, and delivery is then paid again after it. §3 charges `δ` twice for that reason.
- **Where `switch` ends.** It ends at the incoming context's first instruction: the job's first instruction of its
  own code for a transition in, and the resumed instruction for a return to a preempted job.
- **From any entry state.** Each part cost's `holds-under-preemption yes` means its bound also holds from any state
  the code before it leaves. The application's condition-8 declaration for its own figures says the same of them.

### 3. The composition

Exact integers, in the variant's unit, rounded as its §1 rounds. Every sum and product is checked, and an overflow
has a verdict (§6).

```text
C_i  = C_i^app + Σ_p n_{i,p} · api.p + completion

CS_i = max( max_{(r, e) : e is an unmask}     ( CS_{i,r,e}^app + api.mask + Σ_p n_{r,e,p} · api.p + api.unmask ),
            max_{(r, e) : e is completion}    ( CS_{i,r,e}^app + api.mask + Σ_p n_{r,e,p} · api.p + completion ),
            max_{p : n_{i,p} > 0} masked.p,
            masked.completion )

L    = max( max_k (CS_k + S), C_rel + S, max_s (C_s + S), W_wake )      the variant's §4, condition 7, unchanged
```

**Waiting for a service.** For an interrupt `x`, the timer or a source, `Δ_x` is the least fixed point of

```text
Δ = B_x + Σ_{q ∈ ahead(x)} n_q(Δ) · (δ + C_q + S)

B_timer = ρ + C_rel + S + L + 2δ         B_s = L + 2δ
n_q(Δ)  = ⌈(Δ + J_q) / T_q⌉                                   for a source q, with C_q = C_s of q
n_q(Δ)  = Σ_{k released by the timer} ⌈(Δ + J_k) / T_k⌉      for the timer, with C_q = C_rel and J_k = J_k^event + J_k^release
```

and then:

- `J_i^release = Δ_timer` for every task the timer releases. With no such task, `Δ_timer` is not computed, and the
  timer counts nothing ahead of any source;
- `J_s = Δ_s` for every source;
- `J_i^release = J_s` for a task released by source `s`, whose nominal release is the arrival.

`Δ_x` needs only `J_q` for `q` ahead of `x`. So computing the interrupts in order, first taken first, never needs a
value that is not yet known.

**The iteration** starts at `B_x`. Each step either reaches the fixed point or grows by at least one unit. At each
iterate the fixed-point test comes first, then the refusal bound. Each ceiling's numerator is formed as the
variant's §2 forms its own, so that a small true quotient never overflows. A fixed point reached is the value,
wherever it lies, and the variant judges it: past a source's no-loss limit its
condition 7 refuses it, and past every timer task's period its busy-period stop does. Short of a fixed point, the
iteration stops without a value in four ways:

- **No fixed point.** When `Σ_{q ∈ ahead(x)} (δ + C_q + S) · r_q ≥ 1`, compared exactly in reduced rationals, where
  `r_q` is `1/T_q` for a source and `Σ_k 1/T_k` for the timer. What queues ahead of `x` can then arrive at least as
  fast as it is served. This is checked before iterating.
- **Past the refusal bound** before a fixed point, beyond which the variant must refuse anyway:
  - for a source, its no-loss limit from condition 7: `T_s` when it is acknowledged at entry, and `T_s − C_s − S`
    when it is acknowledged at exit;
  - for the timer, the largest `T_k` of a timer-released task, past which every such task meets the variant's
    busy-period stop.
- **An overflow.**
- **A limit:** 1 000 000 steps, or an exact sum of the pre-check that does not fit a 128-bit numerator and
  denominator.

§6 gives each stop its verdict. **A stopped interrupt has no value**: no iterate is ever passed on as one. Every
interrupt behind it needs its `J`, so none of them is composed either. Each is named under the stop, and each is
still checked as far as a lower bound allows: `B_y ≤ J_y`, so a source `y` whose `B_y` already passes its no-loss
limit is refused by condition 7 whatever stopped ahead of it. And since the variant's recurrence reads every task's
`J_k` and every source's `J_s`, a stop anywhere leaves the whole set without a bound: no task is reported as
holding.

### 4. Why none of it under-charges

**`C_i`.** The variant's `C_i` is everything the job executes from its transition in up to the decided switch that
follows its completion, services excluded.

- **The parts partition it.** Every instruction in that span is the task's own code, is inside a primitive call
  (at most `n_{i,p}` calls to each `p`), or is on the completion path. The parts meet at instructions with
  nothing between them: each primitive's first and last instruction (`primitives-out-of-line`), the job's last
  instruction of its own code, and the end of the transition in (§2).
- **Instrumentation** lands in whichever part runs it.
- **So the sum bounds it:** a sum of each part's maximum is at least the maximum of the sum.
- **The sum is only a bound if each part's figure holds from whatever state the code before it left.** `γ` charges
  state lost to a preemption, not state that the job's own earlier code disturbed. So this is each part's own
  obligation (§2, "from any entry state").

**`CS_i`.** The variant's `CS_i` is the longest section the task executes masked. That is a maximum over
sections, so the composition takes a maximum over the task's masked stretches, and each stretch is a sum of
everything that runs inside it:

- **A run ending at `unmask`** holds the rest of `mask` after it masks, the task's own code, every primitive called
  inside the run, and the start of `unmask` before it unmasks. `mask` and `unmask` are charged whole, which
  over-covers.
- **A run ending at completion** holds the same, but in place of `unmask` it holds the completion path, masked up
  to the decided switch. `L`'s `+ S` then covers the switch.
- **The runs the application declares are all its runs,** for three reasons:
  - the fourth task fact makes its masked stretches start only at a `mask` call;
  - `pending-taken-after-unmask` keeps stretches split by an `unmask` apart;
  - a stretch not ended by an `unmask` ends at completion.
- **A kernel section the task runs while unmasked** is inside one call, and is bounded by `masked.p`, or by
  `masked.completion` on the completion path.
- **A primitive that unmasks inside an application's run** would split the run into shorter ones. The composition
  still counts the whole, which over-covers.

**`J_i^release` for a timer-released task, and `J_s`.** Take nominal release `t`.

1. **The compare matches by `t + ρ + C_rel + S`.** `releases-after-initialisation` puts `t` after initialisation,
   and `reprograms-only-in-service` puts every later compare write inside the timer service, under the timer facts
   that hold of initialisation's first write too. The due check sees `t` only from `⌈t⌉ ≤ t + ρ`, since it rounds as
   the compare does (`due-check-matches-compare`). After each write, the compare holds the earliest release not yet
   performed (`reprograms-only-in-service`), so apart from a service that starts before `⌈t⌉` and still releases `t`
   on time, two cases cover every timeline:
   - at `⌈t⌉` the compare holds at most `⌈t⌉`, written for `t` or for an earlier release: it has level semantics,
     so it matches by `⌈t⌉`;
   - a timer service that did not release `t`, because its due check came before `⌈t⌉`, writes the compare late.
     A compare written in the past matches at once, and the write lies inside that service, which started before
     `⌈t⌉` and ends, with its transition, within `C_rel + S` of its start.

   A write can also pass through a value above the counter, as RV32's two-store sequence does. The timer interrupt
   then stops being pending until after the service ends, since the specification makes MTIP follow the comparison
   "eventually, but not necessarily immediately". So a task resumed after that service can begin a full masked
   stretch before the interrupt is pending again. That is why the late case adds the service's time to a full `L`
   (step 2) instead of counting it inside `L`.
2. **From the compare match, at most `2δ + L` passes before the pending interrupts start to be taken.** The
   interrupt is pending within the first part of `δ`. Before it is pending, and while it waits for the trap, the
   running code can begin a masked stretch (§2, "delivery is time"). So one masked stretch can be in progress or
   begin between the match and the trap, and it lasts at most `L`. Once it ends, delivery is paid again, at most
   `δ`. No second stretch can begin after that:
   - a pending interrupt is taken before the instruction after any unmask (`pending-taken-after-unmask`);
   - it is taken before a resumed context executes an instruction (condition 5);
   - every transition ends unmasked (condition 5).

   Once the processor commits to a trap, what follows is a service (§2's boundary), so the stretch is a task's, a
   service's with its transition, or the idle wake, each within `L`.
3. **When that stretch ends, pending interrupts are taken back to back, in order, before any task runs an
   instruction.** So until the timer's service starts, only these run: the services of interrupts ahead of the
   timer, each preceded by its delivery `δ` and followed by at most one transition `S`. The timer's own delivery
   after the stretch is step 2's second `δ`.

   No release is latched on the way (`releases-never-latched`), so no `unmask` does a service's work.
4. **Services of `q` that start in the window come from arrivals at most `J_q` before it,** so there are at most
   `⌈(Δ + J_q)/T_q⌉` of them.
   - For a source, every service claims a request (`no-empty-claim`), every request was made by an arrival and each
     arrival makes at most one (`one-request-per-arrival.<q>`), so no more services start than arrivals. A claim
     takes the highest request pending when it is read, so a service can serve a request that arrived after its trap
     was taken. The count still holds: a request it displaces is served next, inside the window, where its arrival
     is counted; and the claim comes within `C_q` of the trap, with `C_q ≤ L` (step 2), so the interrupt's own later
     request adds no service the ceiling misses.
   - For the timer ahead of a source, each timer service maps to a release that was due when its
     interrupt was raised (raised only when due) and that the service performs. "Raised" covers an interrupt still
     pending after a service has moved the compare on, which the specification allows, since MTIP follows the
     comparison "eventually, but not necessarily immediately". So the fact's basis establishes the hardware's half:
     the timer service does not return while MTIP still reflects a compare value it has replaced. No release is
     performed twice, so the mapping is one to one. That release's nominal instant lies at most `J_k` before the
     service starts. So there are no more services than such releases, `Σ_k ⌈(Δ + J_k)/T_k⌉`.
5. **A compare written late is written inside the timer service**, and step 1 charges that service's remaining time
   apart from `L`, because the stretch step 2 counts may follow it.

The least fixed point is therefore an upper bound on how late the service can start. The same argument without
`ρ`, from the arrival, gives `J_s`. Condition 7's floors hold by construction, since `Δ_x ≥ B_x`.

**Charged twice, and declared conservative** (the variant's §3 records its own the same way):

- services queued ahead are counted in `J` and again in `w`, the variant's last listed deviation;
- `mask` and `unmask` are charged whole inside a run;
- a transition `S` is charged after every queued service, whether it switched or not;
- the timer is counted once per release, not once per service: one service can perform several releases, and
  `raised-only-when-due` rules out one that performs none (step 4);
- `δ` includes the latency before an interrupt is pending, which the variant's own `δ` does not;
- `δ` is charged twice in `B_x`, although a stretch that begins after the interrupt is pending can only overlap the
  first `δ`'s later part;
- `B_timer` charges `ρ` and a whole service with its transition, although only a late write needs the second.

### 5. What the catalog's §12 gains

- **Timing costs** `api.<p>` and `masked.<p>` for each primitive a task can call, and `completion` and
  `masked.completion`. Each comes from the runtime API record, the one that supplies `completion`, and each is
  image-specific, like `switch`.
- **Code facts** `reprograms-only-in-service` and `releases-after-initialisation`, in the `timer-service` group;
  `releases-never-latched` and `primitives-out-of-line`, in the `completion` group; and `pending-taken-after-unmask`,
  `one-claim-per-trap` and `starts-by-transition`, in the `switch` group. Every fact is behavioral, and every
  pairing is one of the catalog record's §12 groups, checked per selection.
- **The application's fact `leaves-interrupt-hardware-alone`**, which covers its initialisation too, and the
  caller's declaration of it for a service's application code.
- **`runtime-discipline.<id>`**, which each record states about every package its implementation's own and
  reached sets hold, and `no-empty-claim` in the `switch` group.
- **Hardware facts** `external-before-timer` and `one-external-controller`, and in each `service.<source>` group
  `external.<source>` and the code fact `one-request-per-arrival.<source>`.
- **§2's boundaries:** where delivery ends and a service begins, and where `switch` ends.
- **What these costs' `holds-under-preemption yes` means:** the bound also holds from any state the code before it
  leaves.
- **The application's task facts gain** the same entry-state statement for its own figures, and, for each masked
  run, each way it can end.

This record names what §12 gains; the catalog record's own review checks the wording it takes.

### 6. What the variant receives, and when composition fails

- **The variant receives the composed values, never the parts.** Its §1, §2 and §4 read them unchanged.
- **The conclusion names each composed input with its parts, their owners and their evidence categories.**
  - **The composite's category is the weakest of its parts' categories.** The order runs, from strongest:
    analytically established, externally supplied, observed maximum, assumed. `ROADMAP.md` §7.3 lists the four
    categories without ranking them, so the order is this record's choice. The catalog record leaves evidence
    strength to the analysis to state (its §7).
  - §7.3's rule stands: an observed maximum with a safety factor remains an empirical assumption "unless a valid
    argument establishes a bound".
  - The description's periods, deadlines and release jitters, and the plan's order, are requirements and
    configuration, not evidence. They carry no category and lower none. A source's `T_s`, and the start-up
    assumption of §2, are different: they are arrival assumptions about the environment (§7.3), and the conclusion
    names each as one. It names §2's assumption of the run, that no fault trap is taken, beside them.
- **A part or fact that cannot be read** makes the composites that need it undeclared. That covers a name no record
  supplies, an `unknown`, a cost outside its `holds-for`, a primitive the catalog does not cost, and a plan with no
  order among its sources. It is the variant's `analysis-inconclusive`, with the part named. §12's rule for a value
  it could not read applies unchanged.
- **A declared value outside what the composition covers** is `unsupported-profile`, as the variant's §5 treats one
  outside `/1`. That covers a §2 fact that is read and declared `no`, `external-before-timer` apart when
  `one-claim-per-trap` is `yes`, and a plan whose order among sources is not strict.
- **The stops of §3:**
  - **For a source:** no fixed point, an overflow, or passing its no-loss limit. Each means `J_s` cannot be bounded
    below the limit, which is condition 7's no-loss failure, so the verdict is `unsupported-profile`, naming the
    source;
  - **For the timer:** no fixed point, an overflow, or passing the largest `T_k`. Each is the variant's own
    `not-established`, which it gives for utilisation at or above one and for overflow;
  - **A limit:** the step budget, or the pre-check's sum outside 128 bits. Each is `analysis-inconclusive`, a named
    resource limit, as in the variant;
  - every interrupt behind a stopped one is named under the stop, with no value, and checked against its no-loss
    limit through `B_y` (§3);
  - **An overflow while composing** `C_i` is the variant's busy-period stop, `not-established`: the job's own cost
    is past what any period bounds. An overflow in `CS_i`, `L` or a `B_x` stops every interrupt as its own overflow
    would, so sources get `unsupported-profile` and the timer `not-established`;
  - **The set's verdict** is the highest-precedence verdict among all the stops and every other condition's, by the
    variant's §5. A stop anywhere leaves no task reported as holding.
- **A caller-supplied `C_s`** (§7) does not stop the composition. The composites built on it through `L` and the
  queue are composed, and the conclusion names the caller's figure as such. That figure begins at the trap (§2),
  and the caller declares `leaves-interrupt-hardware-alone` for the service's application code.
- **Every declared source needs a record that anchors `service.<source>`.** A caller's `C_s` exists only when that
  record states `no-application-code.<source>` `no`, with its cost `unknown` if it has none. The record still states
  `acknowledge-at-entry`, `defers-nothing`, `one-request-per-arrival` and `external.<source>`. With no such record,
  the source's facts cannot be read, and every `J` is undeclared, naming it.
- **An enabled source the plan leaves undeliverable**, below the controller's threshold for the hart's machine-mode
  context, is `unsupported-profile`, as the variant's condition 5 reads "enabled". A plan that does not state its
  sources deliverable is `analysis-inconclusive`, like a missing order.
- **A primitive named `completion`** in the application's parts cannot be composed, since `masked.completion` names
  the completion path: `analysis-inconclusive`, naming it.
- **Production claims.** Every application part is an application input, so a claim that rests on a composite
  stays `not-established` for a production claim until `M4` gives application inputs their own evidence rules
  ([[decision_catalog-records]] §7). What composition changes is who supplies what: the caller then states only the
  task's own figures, never a kernel or platform figure, and every kernel figure is a reviewed catalog cost.

### 7. What stays whole in `/1`

- **`C_s` of a service that runs application code**, whose record states `no-application-code.<source>` `no`, stays
  the caller's, as §12 says. `rt-static-up-v1` runs deferred work as a declared task, so the runtime offers no way
  to run application code inside a service yet. When it does, that part is a later leaf's.
- **`ρ` below its worst case when releases fall on ticks** is not composed in `/1`. It needs the timer's resolution
  as a number and the release epoch's alignment, which the catalog's yes-or-no facts cannot state. Taking the
  worst case is conservative, and is recorded here as conservatism, not as a gap.
- **A task's call into another record's code**, a driver's for example. `leaves-interrupt-hardware-alone` counts
  that code as a runtime function (§2), so a task that makes such a call is outside the composition,
  `unsupported-profile`, naming the fact: no part covers that code's execution or its masked runs. Giving such calls
  parts of their own is a later leaf's.

## Why

- **The definitions are the variant's.** Composing at the definition's own joints keeps the composite equal to the
  definition: parts in sequence are added, alternatives are maximised, and everything that runs first is
  counted.
- **One owner per part.** A kernel figure the application supplied would be unreviewed. An application figure the
  catalog supplied would claim what no reviewer of the kernel saw. Where the hardware fixes something, such as the
  timer's place among interrupts, the owner is the catalog's hardware fact, not the plan.
- **The composition's assumptions are its own.** They sit in §2 and are checked when it runs, so the variant's
  record and its code (`PlatformFacts`) stay as reviewed. A caller who supplies the four inputs whole is not held to
  them. The caller's figures carry their own evidence, as before.
- **Queued services as a fixed point, not one arrival each.** A source that arrives faster than the window is long
  would otherwise be under-counted.
- **Rejected alternatives:**
  - **`J^release` as its floor `ρ + δ + L`.** It omits queued services. The variant itself says the floors are
    necessary and not sufficient.
  - **`CS_i` as the larger of the application's longest section and the kernel's.** It under-charges a run with a
    primitive call inside it: B3's under-charge.
  - **An extra `γ` at every primitive boundary instead of the entry-state obligation.** `γ` is defined per
    preemption, and its evidence would not cover a boundary.
  - **A joint fixed point over interrupts with no order.** A strict order exists on the platforms `/1` covers:
    - the hart orders the timer against external interrupts, the specification's way or, as QEMU 11.1.1 does,
      lowest cause first, and `external-before-timer` states which;
    - the external controller orders its sources by the priorities the plan sets.

    Without an order, the plan's input is missing, which is named rather than guessed.
  - **Handing the variant an iterate that passed a refusal bound.** It is not a bound, and interrupts behind it
    would be composed from it.

## How to apply

- **The caller**, until `M2.10.2` implements this, still supplies the four inputs whole, as the variant's ownership
  note says. After it, the caller supplies only the application's parts, and the engine composes them.
- **A new primitive** in the runtime API needs its `api.<p>` and `masked.<p>` costs in the catalog before any task
  that calls it can be composed. It must also be classified:
  - whether it can return with interrupts masked, which would open a run;
  - whether it can unmask, which would close one.

  A primitive that does either needs this record's run definition extended before it is costed.
- **A new kind of release, nested interrupts, a source whose `external.<source>` or
  `one-request-per-arrival.<source>` is `no`, or a platform whose `external-before-timer` and
  `one-claim-per-trap` are both `no`** is outside this composition in `/1`; one of these facts `unknown` leaves
  every `J` undeclared.
- Related:
  - [[decision_runtime-analysis-variant]]: its §1 names each composite and points here;
  - [[decision_catalog-records]]: its §12 owns the catalog's parts and facts.

## Review

`M2.10.1`'s acceptance is a review by a context that did not write this record, finding no composite that
under-charges against the variant's §1 definitions. The findings, and the answer to each, are in
[`decision_runtime-composite-inputs-reviews.md`](../../reviews/decision_runtime-composite-inputs-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 19 | 3 (K1, a job completing masked; K3, a compare written in initialisation; K5, inconsistent stop verdicts) | "Not acceptable as it stands" |
| 2 | 11 | 1 (L1, a masked stretch beginning while delivery is under way, which a simulation reproduced) | "cannot be accepted as it stands"; nothing else under-charges once L1 is fixed |
| 3 | 13 | 1 (M1, a late compare write letting a full masked stretch in before the interrupt is pending again, reproduced by hand and by simulation) | "cannot be accepted as it stands"; nothing else under-charges once M1 is fixed and M2–M5 and M8 are stated |
| 4 | 9 | 2 (N1, a late write by a service that started before the due check could see the release; N2, a trap that claims nothing, which the PLIC specification allows) | "cannot be accepted as it stands"; `C_i` and `CS_i` sound, and nothing else under-charges once N1 and N2 are fixed and N3 is stated |
| 5 | 8 | 1, latent (O1, a level-triggered source completed before its device is cleared, served twice per arrival, which the PLIC specification allows); O3 live, an ambiguity | "cannot be accepted as it stands"; `C_i`, `CS_i` and `B_timer` sound, and nothing else under-charges once O1 is fixed and O2 and O3 are stated |
| 6, the first under the closure rule | 10 | 2 live on the emulator (P1, a claim is a read, which no fact forbade a task; P2, QEMU pends a request on any raise of a claimed source's line) | "does not meet its closure rule"; the model sound once the §2 facts hold, and nothing else under-charges on hosted and emulator targets once P1 and P2 are fixed |
| 7 | 11 | 1 live (Q2, a declared source's device registers ungoverned, which QEMU's UART re-raises on); Q1, the emulator taking the timer before external interrupts, so `/1` composed nothing there | "not met in substance"; `C_i` and `CS_i` sound, `J^release` and `J_s` sound where external interrupts come first once Q2 is fixed |
| 8 | 8 | 3 live on the emulator, all in the facts' wording (R1, a timer trap that drains sources; R3, application initialisation unbound; R4, a request no arrival made) | "does not meet its closure rule"; the model sound under both interrupt orders in simulation, QEMU's timer-first order included |
| 9 | 8 | 2 live on the emulator, in the facts' coverage (S1, application code masking after the first enabling outside any task; S2, another record's initialisation writing the counter or a device) | "not met"; the model sound under both orders in simulation, claims of later arrivals included, and nothing else under-charges once S1 and S2 are fixed and S3 and S4 stated |
