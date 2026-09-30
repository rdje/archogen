# The runtime variant's composite inputs: each part, its owner, and how the parts add up

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `M2.10.1` (`docs/tasks/M2.md`). [[decision_catalog-records]] §12 left four of the
  runtime variant's inputs whole, `C_i`, `CS_i`, `J_i^release` and `J_s`, because each includes what no single
  owner knows. It also left them to the caller until this leaf, because a composition that took the larger of two
  parts where the definition needs their sum would under-charge (that record's second review, findings B2 and B3).
  This record decides the composition. The review it needs, by a context that did not write it, is the last section.

## The fact / decision

The engine composes each of the four inputs from parts, and each part has one owner:

- **the catalog:** kernel code and the platform, as reviewed timing costs and behavioral facts;
- **the application:** the task's own code, one of `ROADMAP.md` §10.3's "separately supplied application inputs";
- **the plan:** `ROADMAP.md` §7.5's resolved plan, which orders the interrupts;
- **the description:** its periods, jitters and what releases each task, as before.

The variant's §1 definitions are unchanged, and so is its model, `fixed-priority-with-overheads/1`. This record is
how a value that meets each definition is put together.

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
| a primitive's cost | `api.p`, for each primitive `p` a task can call | catalog: timing cost `api.<p>` of the record that supplies the runtime API | one call, from the call to its return, on the caller's behalf. It excludes any service taken during the call or right after it. A release latched while masked is delivered by the pending interrupt's service, which is `C_rel`, not by `unmask` |
| a primitive's masked run | `masked.p` | catalog: timing cost `masked.<p>` | the longest contiguous masked stretch inside one call of `p` made with interrupts unmasked |
| the completion path | `completion` | catalog: timing cost `completion` | from the job's last instruction of its own code to the decided switch that follows, the scheduling decision included |
| the completion path's masked run | `masked.completion` | catalog: timing cost `masked.completion` | the longest contiguous masked stretch in the completion path, up to the decided switch. The transition that follows it is added by `L`, as it is today |
| where the compare is written | — | catalog: behavioral code fact `reprograms-only-in-service` | the compare is written only by the timer service, after its due check, and at initialisation |
| the task's own code | `C_i^app` | application, per task | the most one job executes of the task's own code, from its first instruction after the transition in to its last. It excludes every primitive's execution and every service, and includes the application's own instrumentation |
| how often it calls each primitive | `n_{i,p}` | application, per task | the most calls to `p` one job makes: §7.3's "allowed OS calls", with a count |
| the task's masked runs | for each run `r`: `CS_{i,r}^app`, and `n_{r,p}` | application, per task | a run is a stretch from a `mask` call to the `unmask` call that ends it. `CS_{i,r}^app` is the most the task's own code executes inside it, and `n_{r,p}` is the calls to each primitive inside it, other than the `mask` and `unmask` that delimit it |
| the order pending interrupts are taken in | `ahead(x)` | plan | a strict order over the timer and every declared source. `ahead(x)` is every interrupt taken before `x` when both are pending |

The runtime API a task calls today (`crates/rt-core`, `Scheduler`) is `mask` and `unmask`, and its completion
path is `complete` followed by `decide`. The primitives are whatever the runtime API record's contract lists. A
task that calls one the catalog does not cost cannot be composed (§5).

The parts the catalog already supplies keep their names: `ρ` `compare-rounding`, `δ` `delivery`, `S` `switch`,
`W_wake` `wake`, `C_rel` `timer-service`, and `C_s` `service.<source>` with its `no-application-code` fact.

### 2. The composition

Exact integers, in the variant's unit, rounded as its §1 rounds. Every sum and product is checked.

```text
C_i  = C_i^app + Σ_p n_{i,p} · api.p + completion

CS_i = max( max_r ( CS_{i,r}^app + api.mask + api.unmask + Σ_p n_{r,p} · api.p ),
            max_{p : n_{i,p} > 0} masked.p,
            masked.completion )

L    = max( max_k (CS_k + S), C_rel + S, max_s (C_s + S), W_wake )      the variant's §4, condition 7, unchanged
```

**Waiting for a service.** For an interrupt `x`, the timer or a source, `Δ_x` is the least fixed point of

```text
Δ = B_x + Σ_{q ∈ ahead(x)} n_q(Δ) · (δ + C_q + S)

B_timer = ρ + L + δ          B_s = L + δ
n_q(Δ)  = ⌈(Δ + J_q) / T_q⌉                                   for a source q, with C_q = C_s of q
n_q(Δ)  = Σ_{k released by the timer} ⌈(Δ + J_k) / T_k⌉      for the timer, with C_q = C_rel and J_k = J_k^event + J_k^release
```

and then:

- `J_i^release = Δ_timer` for every task the timer releases;
- `J_s = Δ_s` for every source;
- `J_i^release = J_s` for a task released by source `s`, whose nominal release is the arrival.

`Δ_x` needs only `J_q` for `q` ahead of `x`. So computing the interrupts in the plan's order, first taken first,
never needs a value that is not yet known.

**The iteration** starts at `B_x`. Each step either reaches the fixed point or grows by at least one unit. It stops
in four ways:

- when `Σ_{q ∈ ahead(x)} (δ + C_q + S) · r_q ≥ 1`, compared exactly in reduced rationals, where `r_q` is `1/T_q`
  for a source and `Σ_k 1/T_k` for the timer. There is then no fixed point, since what queues ahead of `x` can
  arrive at least as fast as it is served. This is checked before iterating;
- when `Δ` passes the value past which the variant must refuse anyway. For a source that is `T_x`, since
  condition 7 needs `J_s < T_s`. For the timer it is the largest `T_k` of a timer-released task, past which every
  such task meets the variant's busy-period stop;
- when a value overflows;
- after 1 000 000 steps.

§5 gives each stop its verdict.

### 3. Why none of it under-charges

**`C_i`.** The variant's `C_i` is everything the job executes from its transition in up to the decided switch that
follows its completion, services excluded.

- **The parts partition it.** Every instruction in that span is the task's own code, is inside a primitive call
  (at most `n_{i,p}` calls to each `p`), or is on the completion path, and the parts meet at points with nothing
  between them. Those points are each call and return, the job's last instruction of its own code, and the end of
  the transition in. The catalog defines `switch` to end at the job's first instruction of its own code (§4 below).
- **Instrumentation** lands in whichever part runs it.
- **So the sum bounds it:** a sum of each part's maximum is at least the maximum of the sum.
- **The sum is only a bound if each part's figure holds from whatever state the code before it left.** `γ`
  charges state lost to a preemption, not state that the job's own earlier code disturbed. So this is each
  part's own obligation:
  - the catalog meets it through the part cost's `holds-under-preemption yes`, whose meaning §12 extends to "and
    from any state the code before it leaves" for these costs;
  - the application meets it for its own figures through its condition-8 declaration, extended the same way.

**`CS_i`.** The variant's `CS_i` is the longest section the task executes masked. That is a maximum over sections,
so the composition takes a maximum over the task's masked stretches, and each stretch is a sum of everything that
runs inside it:

- **An application run** holds its own code, the rest of `mask` after it masks and the start of `unmask` before it
  unmasks, and every primitive called inside the run, each counted whole. Charging `mask` and `unmask` whole
  over-covers.
- **The runs the application declares are all its runs.** The fourth task fact, that no task masks except through
  the runtime API, makes its masked runs exactly its `mask`…`unmask` stretches.
- **A kernel section the task runs while unmasked** is inside one call, and is bounded by `masked.p`, or by
  `masked.completion` on the completion path.
- **A primitive that unmasks inside an application's run** would split the run into shorter ones. The composition
  still counts the whole, which over-covers.

**`J_i^release` for a timer-released task, and `J_s`.** Take nominal release `t`.

1. **The compare fires by `t + ρ`**, because it rounds up and has level semantics. From then the timer interrupt is
   pending.
2. **At that instant at most one masked stretch is in progress, and what remains of it is at most `L`.** Condition
   5 makes the stretches separate: a pending interrupt is taken before the resumed context executes an instruction,
   and every transition ends unmasked.
3. **When that stretch ends, pending interrupts are taken back to back, in the plan's order, before any task runs
   an instruction.** So until the timer's service starts, only these run:
   - the services of interrupts ahead of the timer, each preceded by its delivery `δ` and followed by at most one
     transition `S`;
   - then the timer's own delivery `δ`.
4. **Services of `q` that start in the window come from arrivals at most `J_q` before it,** so there are at most
   `⌈(Δ + J_q)/T_q⌉` of them.
   - When the timer is ahead of a source, each timer service releases at least one task (no early release; raised
     only when due). So there are no more timer services than releases, `Σ_k ⌈(Δ + J_k)/T_k⌉`.
5. **A compare written late is written inside the timer service** (`reprograms-only-in-service`). So its lateness
   is inside the `C_rel + S` that `L` already holds.

The least fixed point is therefore the most by which the service can start. The same argument without `ρ` gives
`J_s`. Condition 7's floors hold by construction, since `Δ_x ≥ B_x`.

**Charged twice, and declared conservative** (the variant's §3 records its own the same way):

- services queued ahead are counted in `J` and again in `w`, the variant's last listed deviation;
- `mask` and `unmask` are charged whole inside a run;
- a transition `S` is charged after every queued service, whether it switched or not;
- the timer is counted once per release, not once per service.

### 4. What the catalog's §12 gains

- The timing costs `api.<p>` and `masked.<p>` for each primitive a task can call, and `completion` and
  `masked.completion`. Each comes from the record that supplies the runtime API, and is image-specific, like
  `switch`.
- The behavioral code fact `reprograms-only-in-service`, from the record that supplies `timer-service`.
- **Boundaries.** `switch` ends at the job's first instruction of its own code, and `completion` begins after its
  last.
- **What these costs' `holds-under-preemption yes` means:** the bound also holds from any state the code before
  it leaves.

Until §12 names them, nothing supplies them, and the composite stays the caller's. This record names what §12
gains; that record's own review checks the wording it takes.

### 5. What the variant receives, and when composition fails

- **The variant receives the composed values, never the parts.** Its §1, §2 and §4 read them unchanged.
- **The conclusion names each composed input with its parts, their owners and their evidence categories.**
  - The composed value's category is the weakest of its parts', in §7.3's order: analytically established, then
    externally supplied, then observed maximum, then assumed.
  - An observed maximum with a safety factor remains an empirical assumption (§7.3).
- **A part that cannot be read makes its composite undeclared.** That covers a name no record supplies, an
  `unknown`, a cost outside its `holds-for`, a primitive the catalog does not cost, and no strict order from the
  plan. It is the variant's `analysis-inconclusive`, with the part named. §12's rule for a value it could not read
  applies unchanged.
- **The four stops of §2:**
  - no fixed point, or an overflow: `not-established`, naming the interrupt. These are the variant's own verdicts
    for utilisation at or above one and for overflow;
  - passing the refusal bound: the value that passed it is handed to the variant, whose condition 7 or
    busy-period stop refuses it with its own verdict;
  - the step budget: `analysis-inconclusive`, a named resource limit.
- **Production claims.** Every application part is an application input, so a claim that rests on a composite
  stays `not-established` for a production claim until `M4` gives application inputs their own evidence rules
  ([[decision_catalog-records]] §7). What composition changes is who supplies what: the caller then states only
  the task's own figures, never a kernel or platform figure, and every kernel figure is a reviewed catalog cost.

### 6. What stays whole in `/1`

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
  catalog supplied would claim what no reviewer of the kernel saw.
- **Queued services as a fixed point, not one arrival each.** A source that arrives faster than the window is long
  would otherwise be under-counted.
- **Rejected alternatives:**
  - **`J^release` as its floor `ρ + δ + L`.** It omits queued services. The variant itself says the floors are
    necessary and not sufficient.
  - **`CS_i` as the larger of the application's longest section and the kernel's.** It under-charges a run with a
    primitive call inside it: B3's under-charge.
  - **An extra `γ` at every primitive boundary instead of the entry-state obligation.** `γ` is defined per
    preemption, and its evidence would not cover a boundary.
  - **A joint fixed point over interrupts with no order.** A strict order exists on the targets `/1` admits.
    RISC-V's privileged specification takes machine external interrupts before timer interrupts, and the external
    controller orders its sources. Without an order, the plan's input is missing, which is named rather than
    guessed.

## How to apply

- **The caller**, until `M2.10.2` implements this, still supplies the four inputs whole, as the variant's ownership
  note says. After it, the caller supplies only the application's parts, and the engine composes them.
- **A new primitive** in the runtime API needs its `api.<p>` and `masked.<p>` costs in the catalog before any task
  that calls it can be composed.
- **A new kind of release, or nested interrupts,** is outside `/1`, in the variant as here.
- Related:
  - [[decision_runtime-analysis-variant]]: its §1 names each composite and points here;
  - [[decision_catalog-records]]: its §12 owns the catalog's parts.

## Review

Pending: `M2.10.1`'s acceptance is a review by a context that did not write this record. It must show, or refute
by counterexample, that no composite under-charges against the variant's §1 definitions.
