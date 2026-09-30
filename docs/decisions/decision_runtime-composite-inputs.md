# The runtime variant's composite inputs: each part, its owner, and how the parts add up

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the RISC-V privileged specification](../book/src/ledger.md#riscv-privileged) — its version,
  the two sentences relied on, and its limits are in the ledger
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
| the order among sources | `ahead(x)` for two sources | plan | a strict order over the declared sources, the interrupt controller's as the plan configures it: its priorities, with its own rule for ties |
| where the timer stands | `ahead(x)` between the timer and a source | catalog: hardware fact `external-before-timer` (§2) | `yes`: every source is taken before the timer when both are pending; `no`: the timer before every source |

`ahead(x)` is every interrupt taken before `x` when both are pending.

The runtime API a task calls today (`crates/rt-core`, `Scheduler`) is `mask` and `unmask`, and its completion
path is `complete` followed by `decide`. The primitives are whatever the runtime API record's contract lists.

The parts the catalog already supplies keep their names: `ρ` `compare-rounding`, `δ` `delivery`, `S` `switch`,
`W_wake` `wake`, `C_rel` `timer-service`, and `C_s` `service.<source>` with its `no-application-code` fact.

### 2. What the composition assumes

The composition is sound only on a platform where the following hold. Each catalog fact below is a behavioral
fact (the behavior-model facet), with a `code` locator for a code fact. It comes from the record that the groups
in the catalog record's §12 name, checked per selection. A fact declared `no` puts the platform outside what the
composition covers: `unsupported-profile`, naming it. A fact that cannot be read leaves the composites undeclared:
`analysis-inconclusive`, naming it.

| Fact | Kind | Says | Why the composition needs it |
| --- | --- | --- | --- |
| `reprograms-only-in-service` | code | the compare is written only by the timer service, after its due check, and during initialisation | a late write then lands inside a service, whose time `L` already holds |
| `releases-after-initialisation` | code | every nominal release is at or after the end of initialisation, the first unmask of the running system | initialisation's masked run, and its compare write, are then over before any release they could delay |
| `pending-taken-after-unmask` | hardware and code | a pending interrupt is taken before the instruction that follows any unmask, a task's included | two masked runs separated by an unmask stay separate. RISC-V's privileged specification requires interrupt conditions to be evaluated immediately after an explicit write to `mstatus` or `mie` |
| `releases-never-latched` | code | no service runs while the runtime's mask depth is raised: the port masks the hardware before raising it, and lowers it before unmasking the hardware. A completion entered with the depth raised lowers it before the transition that follows unmasks, and that work is inside `completion`. `rt-core`'s `complete` leaves the depth as it is, so this is the port's to do and the runtime record's review to check | `rt-core` latches a release that arrives while the depth is raised and delivers it on `unmask`. On the target that path is then never taken, so no `unmask` makes a task ready and no part has to charge the decision and switch that would follow |
| `primitives-out-of-line` | code | each primitive is compiled out of line, and its masking and unmasking instructions are compiler barriers | a call's boundaries, and a run's, are then instructions. The compiler cannot move the task's own code into a primitive or across a masking instruction |
| `external-before-timer` | hardware | whether the hardware takes a pending external interrupt before a pending timer interrupt, however the controller is configured | the timer's place in the order is the hardware's, not the plan's. RISC-V takes machine external interrupts before machine timer interrupts |
| `external.<source>` | hardware, one per source, from the record that supplies `service.<source>` | the source reaches the processor as an external interrupt through the controller whose priorities the plan sets | the plan's order holds only for such sources. A source with `no`, such as a platform-defined local interrupt, a software interrupt, or one a priority scheme places among the timer, is outside the composition. A catalog reviewer sees the source's own wiring, never a description's list of sources |
| `leaves-interrupt-hardware-alone` | application task fact, and the caller's declaration for a service's application code | no task, and no application code in a service, writes the timer's compare or the interrupt controller's configuration, and no service calls a runtime primitive | `reprograms-only-in-service`, the plan's order and `releases-never-latched` are statements about the whole image, and a code fact's locator reaches only catalog code. Each holds for the image only with this one, as `sections-mask-every-interrupt` holds only with the fourth task fact |

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

B_timer = ρ + L + 2δ         B_s = L + 2δ
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

**The iteration** starts at `B_x`. Each step either reaches the fixed point or grows by at least one unit. A fixed
point reached is the value, wherever it lies, and the variant judges it: past a source's no-loss limit its
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

1. **The compare matches by `t + ρ`**, because it rounds up and has level semantics. `releases-after-initialisation`
   puts `t` after initialisation, and `reprograms-only-in-service` puts every later compare write inside the timer
   service.
2. **From the compare match, at most `2δ + L` passes before the pending interrupts start to be taken.** The
   interrupt is pending within the first part of `δ`. While it waits for the trap, the running code can retire
   instructions (§2, "delivery is time"), a masking one among them, for less than the second part of `δ`. So at
   most one masked stretch is in progress or begins before the trap, and it lasts at most `L`. Once it ends,
   delivery is paid again, at most `δ`. No second stretch can begin:
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
   - For the timer ahead of a source, each timer service maps to a release that was due when its interrupt was
     raised (raised only when due) and that the service performs. No release is performed twice, so the mapping is
     one to one. That release's nominal instant lies at most `J_k` before the service starts. So there are no more
     services than such releases, `Σ_k ⌈(Δ + J_k)/T_k⌉`.
5. **A compare written late is written inside the timer service**, so its lateness is inside the `C_rel + S` that
   `L` holds.

The least fixed point is therefore an upper bound on how late the service can start. The same argument without
`ρ`, from the arrival, gives `J_s`. Condition 7's floors hold by construction, since `Δ_x ≥ B_x`.

**Charged twice, and declared conservative** (the variant's §3 records its own the same way):

- services queued ahead are counted in `J` and again in `w`, the variant's last listed deviation;
- `mask` and `unmask` are charged whole inside a run;
- a transition `S` is charged after every queued service, whether it switched or not;
- the timer is counted once per release, not once per service;
- `δ` includes the latency before an interrupt is pending, which the variant's own `δ` does not;
- `δ` is charged twice in `B_x`, although a stretch that begins after the interrupt is pending can only overlap the
  first `δ`'s later part.

### 5. What the catalog's §12 gains

- **Timing costs** `api.<p>` and `masked.<p>` for each primitive a task can call, and `completion` and
  `masked.completion`. Each comes from the runtime API record, the one that supplies `completion`, and each is
  image-specific, like `switch`.
- **Code facts** `reprograms-only-in-service` and `releases-after-initialisation`, from the record that supplies
  `timer-service`; and `releases-never-latched` and `primitives-out-of-line`, from the runtime API record. Every
  fact is behavioral, and every pairing is one of the catalog record's §12 groups, checked per selection.
- **The fact `pending-taken-after-unmask`**, in the group anchored on `pending-taken-and-transitions-unmasked`.
- **The application's task fact `leaves-interrupt-hardware-alone`**, and the caller's declaration of it for a
  service's application code.
- **Hardware facts** `external-before-timer`, and `external.<source>` in each `service.<source>` group.
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
    configuration, not evidence. They carry no category and lower none. A source's `T_s` is different: it is an
    arrival assumption about the environment (§7.3), and the conclusion names it as one.
- **A part or fact that cannot be read** makes the composites that need it undeclared. That covers a name no record
  supplies, an `unknown`, a cost outside its `holds-for`, a primitive the catalog does not cost, and a plan with no
  order among its sources. It is the variant's `analysis-inconclusive`, with the part named. §12's rule for a value
  it could not read applies unchanged.
- **A declared value outside what the composition covers** is `unsupported-profile`, as the variant's §5 treats one
  outside `/1`. That covers a §2 fact declared `no`, and a plan whose order among sources is not strict.
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
- **Production claims.** Every application part is an application input, so a claim that rests on a composite
  stays `not-established` for a production claim until `M4` gives application inputs their own evidence rules
  ([[decision_catalog-records]] §7). What composition changes is who supplies what: the caller then states only the
  task's own figures, never a kernel or platform figure, and every kernel figure is a reviewed catalog cost.

### 7. What stays whole in `/1`

- **`C_s` of a service that runs application code**, with no `no-application-code` fact, stays the caller's, as §12
  says. `rt-static-up-v1` runs deferred work as a declared task, so the runtime offers no way to run application
  code inside a service yet. When it does, that part is a later leaf's.
- **`ρ` below its worst case when releases fall on ticks** is not composed in `/1`. It needs the timer's resolution
  as a number and the release epoch's alignment, which the catalog's yes-or-no facts cannot state. Taking the
  worst case is conservative, and is recorded here as conservatism, not as a gap.

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
    - RISC-V's privileged specification takes machine external interrupts before timer interrupts;
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
- **A new kind of release, nested interrupts, or a source whose `external.<source>` is `no`** is outside `/1`, in the
  variant as here.
- Related:
  - [[decision_runtime-analysis-variant]]: its §1 names each composite and points here;
  - [[decision_catalog-records]]: its §12 owns the catalog's parts and facts.

## Review

`M2.10.1`'s acceptance is a review by a context that did not write this record, finding no composite that
under-charges against the variant's §1 definitions. The findings, and the answer to each, are in
[`decision_runtime-composite-inputs-reviews.md`](../reviews/decision_runtime-composite-inputs-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 19 | 3 (K1, a job completing masked; K3, a compare written in initialisation; K5, inconsistent stop verdicts) | "Not acceptable as it stands" |
| 2 | 11 | 1 (L1, a masked stretch beginning while delivery is under way, which a simulation reproduced) | "cannot be accepted as it stands"; nothing else under-charges once L1 is fixed |
