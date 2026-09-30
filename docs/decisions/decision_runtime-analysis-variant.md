# The runtime analysis variant: what it charges, when it applies, and what it refuses

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger
- **Owner / source:** leaf `M2.6.1` (`docs/tasks/M2.md`), deciding what `ROADMAP.md` §7.4 requires before a
  `rt-static-up-v1` timing result may be accepted: "an analysis variant that accounts for its bounded critical
  sections, release jitter, timer/other interrupt interference, and context-switch costs". The review §7.4 asks
  for, and the answer to each of its findings, is the last section.

## The fact / decision

The variant is a fixed-priority preemptive response-time analysis with release jitter, blocking by
non-preemptible sections, interrupt and timer interference, context-switch costs and a preemption-delay term. Its
model identifier is `fixed-priority-with-overheads/1`, apart from the baseline's `idealized-zero-overhead/1`, so a
conclusion of one is never read as the other's.

**It is assembled from published terms, and says where it deviates.** The jitter and blocking form is Tindell,
Burns and Wellings, "An extendible approach for analysing fixed priority hard real-time tasks" (*Real-Time
Systems* 6(2), 1994), which extends the recurrence of Audsley et al. (1993) that §7.4 cites. The per-release
timer cost and the switch charging follow the system-overheads treatment of Burns, Tindell and Wellings,
"Effective analysis for engineering real-time fixed priority schedulers" (*IEEE TSE* 21(5), 1995), and of Burns
and Wellings' textbook. The preemption-delay term follows the cache-related preemption delay literature
(Busquets-Mataix et al., 1996). The deviations are listed in §3. ⚠️ Equation numbers were not checked against the
papers here; `M2.6.3`'s independent derivation is where the combination is checked.

**Version `/1` is deliberately narrow.** It admits one timer kind (event-driven, one interrupt per due release,
with level compare semantics, as §13.4's fixture and the target's `sifive,clint0` behave), interrupts that do not
nest, and sections that mask every interrupt. Each of those is a **declared platform fact**, an input the analysis
checks. Anything else is refused as outside the model (§4), not modelled loosely.

### 1. What it takes

All in one integer time unit. A **cost or delay** (`C`, `C_rel`, `C_s`, `S`, `CS`, `W_wake`, `γ`, `ρ`, `δ`, every
`J`) is rounded **up** into it, and a **separation or deadline** (`T`, `T_s`, `D`) is rounded **down**. **No input
defaults.** A zero is declared, like any other value, with its evidence.

| Input | For | Meaning |
| --- | --- | --- |
| `C_i` | each task | everything the job executes from its transition in up to the decided switch that follows its completion, services excluded: the ledger's *task execution* for it, plus the *critical sections* (its completion path included), kernel services and *instrumentation* run on its behalf |
| `T_i` | each task | minimum separation between nominal releases |
| `D_i` | each task | relative deadline from the nominal release, `D_i ≤ T_i` |
| `J_i^event` | each task | the release event's own jitter, from the description's `jitter` clause: how far the description lets a release vary from its nominal instant. The analysis always receives it: the language gives an absent clause the meaning `0`, which is the description's statement, not a default of the analysis. `J_k = J_k^event + J_k^release` in every ceiling, the timer term included |
| `J_i^release` | each task | engine knowledge: the most by which the service that releases the task can start after its nominal release, everything included — timer resolution, reprogramming that lands after the due instant, hardware delivery, masked activity in progress, and services queued ahead |
| `CS_i` | each task | the longest section the task executes with interrupts masked, the kernel's run on its behalf (its completion path included) and the application's alike |
| priority, released by | each task | a distinct fixed rank, ascending highest-first (`decision_priority-comparison-direction.md`); the timer, or one named interrupt source, and for a source whether **every** arrival releases the task. A source-released task's nominal release is the arrival |
| task facts | each task | whether it suspends itself; whether it locks the scheduler or defers preemption; whether it shares data outside its masked sections |
| `C_rel` | the timer | one timer service: entry, releasing every due task, reprogramming, the scheduling decision up to the switch point whichever way it goes, exit |
| `C_s`, `T_s`, `J_s` | each other interrupt source | one service, entry to exit, excluding work deferred to a task; minimum separation between arrivals; the most by which its service can start after an arrival, everything included as for `J^release` |
| acknowledge point, interrupt priority, deferred work | each other interrupt source | at entry or at exit; its rank among interrupts; the declared task, if any, that runs work the service defers beyond releasing the tasks declared as `released by` it, whose work is analysed through them |
| `S` | the platform | one context transition: from a decided switch, saving the outgoing context beyond what interrupt entry saved, to the incoming one running. It bounds the ledger's *task switch*, *initial dispatch* and *idle/wakeup* alike |
| `W_wake` | the platform | from an interrupt becoming pending while the idle context waits, to interrupts being unmasked: the idle check plus the wake latency. The ledger's *idle/wakeup* is split here: the wake is `W_wake`, before the service; the switch out of idle is `S` |
| `γ` | the platform | the preemption delay: the most one preemption or service adds to the preempted execution through lost cache, pipeline, predictor or TLB state. `0` only with evidence that the target keeps no such state |
| `ρ`, `δ` | the platform | the largest delay the compare's rounding adds to a release (at most one resolution step less one unit; `0` when releases fall on ticks); the hardware's delivery latency from an interrupt becoming pending, unmasked, to its entry |
| the platform facts | the platform | each a declared yes or no, with its evidence: <br>• there is one processor; <br>• the timer is event-driven, its compare has level semantics and rounds up, and the due-check uses the compare's counter and rounding; <br>• **a service releases a task only once its nominal release has passed** (no early release), releases every due one, and is raised only when one is due; <br>• only the timer service releases timer-released tasks; <br>• interrupts do not nest; every masked section masks every interrupt; every service preempts every task; <br>• preemption happens at every point outside a masked section, a service or a transition (no scheduler lock, no deferred preemption); <br>• a pending interrupt is taken before the resumed context executes an instruction, and every transition, into idle included, ends unmasked; <br>• context switching is eager, or `S` includes every deferred save and restore a transition causes (lazy floating-point state, for instance); <br>• each cost bound holds under any preemption pattern |
| enabled sources | the build | the interrupts the build enables (§7.5's resolved plan), non-maskable interrupts and firmware traps included, which the declared sources plus the timer must equal |

Every numerical input carries its §7.3 evidence category: assumed, observed maximum, externally supplied bound, or
analytically established bound.

⛔ *Ownership amended `2026-09-30` by leaf `M2.7.1`.* Where this record calls an input "engine knowledge", "the
engine's" or "the platform's" (`J^release` above, condition 7 in §4, and "Why"), the input's **owner** is the one
`decision_catalog-records.md` §12 assigns. `C_i`, `CS_i`, `J^release` and `J_s` are composite. How they are composed
from parts is decided in `decision_runtime-composite-inputs.md` (`M2.10.1`), and the caller supplies them whole until
`M2.10.2` implements it. So does `C_s` without its `no-application-code` fact. The enabled set and interrupt
priorities are the plan's. Condition 8 is each catalog cost's `holds-under-preemption`, plus the application's
declaration for its own figures. The platform fact that sections mask every interrupt is the conjunction of the catalog's
fact for the kernel's sections and a fourth task fact, that no task masks other than through the runtime API
(`decision_catalog-records.md` §12); `M2.7.5` composes them. What each input **means** is unchanged, and so is the model, `/1`. A conclusion
names every input the caller supplied, whatever its category, and none of them can back a production claim
(`decision_catalog-records.md` §7). `M2.7.5` implements that naming.

### 2. The recurrence

The response interval runs from the task's **nominal release** to the completion of its last instruction, before
its transition out (the observation boundary §7.4.1 asks for). For the analysis a task becomes ready at the
**later** of the start of the service that releases it and its nominal release. A service started for another
release can release a task whose nominal release passes during it, and ready must never precede the nominal
release.

```text
J_i        = J_i^event + J_i^release
w_i^(0)    = C_i + S
w_i^(n+1)  = C_i + S
             + Σ_{j ∈ hp(i)}   ⌈ (w_i^(n) + J_j) / T_j ⌉ · (C_j + 2S + γ)
             + Σ_{k ∈ tasks}   ⌈ (w_i^(n) + J_k) / T_k ⌉ · (C_rel + γ)
             + Σ_{s ∈ sources} ⌈ (w_i^(n) + J_s) / T_s ⌉ · (C_s + γ)
R_i        = J_i + w_i          (w_i the fixed point)
```

- **No blocking term.** Every masked activity is non-preemptible, and an interrupt is taken only when nothing
  masked is in progress. So from the start of the releasing service to the job's completion, no lower-priority
  code, idle window or lower-priority transition runs. Blocking by such activity happens before readiness, and
  `J_i^release` carries it, bounded below by §4 condition 7. The published forms charge it as `B_i` after
  readiness instead. That is a deviation, stated in §3 and checked by `M2.6.3`.
- The timer term runs over **every** task, lower-priority ones included, because each release is served by the
  timer and every service preempts whatever runs. For `k = i` it counts the task's own release.
- **Before iterating**, if the interference utilisation reaches one, there is no fixed point and the result is
  `not-established` with no iteration. That is `Σ_{hp} (C_j + 2S + γ)/T_j + Σ_{tasks} (C_rel + γ)/T_k +
  Σ_{sources} (C_s + γ)/T_s ≥ 1`, compared exactly in reduced rationals. It is exact in both directions: at or
  above one every iterate exceeds the last, because `C_i > 0`; below one the right-hand side is at most
  `c + U·w`, so a least fixed point exists. Whether it lies within the busy-period stop is decided only by
  iterating. If an
  exact sum does not fit 128-bit numerator and denominator, the result is `analysis-inconclusive`, a named
  resource limit, and there is no iteration.
- **Otherwise** the iteration runs to its fixed point. It stops, not converged, when `J_i + w_i > T_i` (strictly),
  where more than one job can be in the busy period and this single-job form no longer bounds it. It also stops at
  once if `J_i + w_i^(0) > T_i`. Every non-final iterate grows by at least one unit, so it ends within
  `max(0, T_i − J_i − w_i^(0)) + 1` steps. That bound is pseudo-polynomial in `T_i`. So there is also a budget of
  1 000 000 iterations, `analysis-inconclusive` when reached and named in the result. The derived bound says when
  it cannot be reached.
- Arithmetic is exact and checked, and rounds up only in `⌈ ⌉`. The implementation forms each numerator so that a
  small true quotient never overflows. Any overflow that remains means a value above what `T_i` bounds, so it is
  the busy-period stop, `not-established`.

### 3. How every cost is charged (`ROADMAP.md` §7.4.1, against `cost-accounting/1`'s categories)

| Ledger category | Charged as |
| --- | --- |
| task execution, and the critical sections, kernel services and instrumentation run for a job | inside `C` of that job |
| instrumentation inside a service or a transition | inside `C_rel`, `C_s` or `S` respectively, never a fourth place |
| task switch, initial dispatch, idle/wakeup | `S` for the analysed job's transition in; `2S` per interfering job (its transition in, and the return to what it preempted) |
| interrupt service | `C_rel` per release of any task, `C_s` per arrival of source `s`, never inside `S` or `C`. The boundary is the decision to switch: a scheduling decision that switches nothing is the service's, and everything from a decided switch on is `S` |
| critical sections, transitions and the idle wake of lower-priority work | before readiness, inside `J_i^release`. After readiness none runs: no scheduler lock or deferred preemption exists (a platform fact), so the proof in the third review below holds |
| lost preemption-dependent state | `γ` per interfering job and per service |

**Where this deviates from the published forms, and what is declared conservative** (§7.4.1 asks that
conservatism be recorded, not implied):

- Blocking is charged before readiness, in `J_i^release`, and not as `B_i` after it (§2).
- Transitions are counted **at least once**. A job's completion that switches straight to another waiting job is
  one physical transition, charged as the first job's return and the second's transition in. `S` also bounds
  three different transitions.
- `C_rel` is charged for every task's releases, including a task released by an interrupt source, which has no
  timer service. It is also charged for `k = i` when the analysed job was released by a service triggered for
  another task. Releases served by one timer interrupt are each charged.
- `γ` is charged per interfering job and per service, so one preemption episode, a service followed by the job it
  releases, is charged `2γ`.
- The interference terms count `⌈ ⌉` arrivals, which may include one whose service fell before the window.
- When the engine's `J^release` also counts services queued ahead, which `w` counts again, those are charged
  twice.

### 4. When it applies: the admission conditions

A task set is admitted only if every condition holds, each checked against a §1 input. Otherwise it is refused
before any arithmetic. **Every condition is checked**, and the refusal carries the verdict of highest precedence
among all the failures (§5), with every reason of that verdict. It does not stop at the first failure.

1. There is one processor.
2. Priorities are distinct and fixed. Preemption happens at every point outside a masked section, a service, or a
   transition: no scheduler lock and no deferred preemption (a platform fact).
3. Deadlines are constrained, `D_i ≤ T_i`, and `C_i > 0`, `T_i > 0`, `T_s > 0`. The set holds at least one task: an
   empty set would hold vacuously, which is what §7.1 exists to prevent (`decision_runtime-contract-gaps.md`, gap 2;
   added by `M2.6.2`, which found the record silent on it).
4. No task suspends itself, locks the scheduler or defers preemption, or shares data outside masked sections,
   kernel or application, each bounded and declared in `CS`. These are §1's task facts. There is no resource
   protocol in the model.
5. **Interrupts**, all declared platform facts:
   - Every interrupt the build enables other than the timer is declared, with `C_s`, `T_s`, `J_s`, its
     acknowledge point and its priority, and the declared sources plus the timer equal the build's enabled set.
     Counter-wrap, timekeeping, overrun and watchdog interrupts, non-maskable interrupts and firmware traps are
     sources like any other.
   - Interrupts do not nest.
   - Every masked section masks every interrupt.
   - Every service preempts every task.
   - A pending interrupt is taken before the resumed context executes an instruction, and every transition, into
     idle included, ends unmasked.
   - Context switching is eager, or `S` includes every deferred save and restore.
   - Work a service defers runs as a declared task.
   - A task released by source `s` on every arrival has `T_i ≤ T_s`. This compares a nominal separation with an
     arrival separation, which is safe and may refuse a set that jittered arrivals would admit. One released on
     some arrivals only has its rate limit named as an assumption.
6. **The timer**, all declared platform facts:
   - The timer is event-driven, and its compare has level semantics: it fires whenever the counter has reached
     the compare value, including one written in the past.
   - The compare value is rounded up, so the timer never fires before a nominal release.
   - The due-check and the compare use one counter in one unit, and round alike.
   - **No early release.** A service releases a task only once its nominal release has passed. The third review
     showed that a kernel releasing tasks due soon breaks the ceilings, with a deadline missed by 3 in a set this
     analysis would admit.
   - A timer interrupt is raised only when a release is due, and its service releases every due task.
   - Only the timer service releases timer-released tasks.
7. **Floors and no loss.** Write `L` for the longest contiguous masked run:
   `L = max(max_k (CS_k + S), C_rel + S, max_s (C_s + S), W_wake)`.
   - A task released by the timer has `J_i^release ≥ ρ + δ + L`.
   - A source has `J_s ≥ δ + L`.
   - A task released by source `s` has `J_i^release ≥ J_s`.
   - A source acknowledged at entry has `J_s < T_s`. One acknowledged at exit has `J_s + C_s + S < T_s`.

   The floors are **necessary, not sufficient**. They refuse a delay that cannot be true. The true delay adds
   late reprogramming and services queued ahead, and its evidence is the engine's. The conclusion names each
   `J^release` and `J_s` as an assumption with its evidence category. `L` counts on every transition ending
   unmasked (condition 5).
8. The platform declares that `γ`, `C`, `C_s`, `C_rel` and `S` hold under any preemption pattern. `C`, `C_s`,
   `C_rel` and `S` bound their own execution **excluding** the lost state `γ` charges, so that state is charged
   once.
9. Every input of §1 is present. `CS_i`, `J_s`, the acknowledge point, every task fact, every platform fact, and
   a zero anywhere are all declared, never defaulted.

### 5. What it concludes, and what it refuses

| Situation | Outcome | §5.5 verdict |
| --- | --- | --- |
| every task converged with `R_i ≤ D_i` | the deadlines hold in `fixed-priority-with-overheads/1`, under §4's conditions. Every input whose §7.3 evidence is not analytically established is named as an assumption with its category, and so are the enabled set before `M4` and any rate-limited source release | (`HoldsUnderAssumptions`) |
| a task converged with `R_i > D_i` | not established: the bound is an envelope, so exceeding the deadline shows no miss | `not-established` |
| interference utilisation at or above one, a task's `J_i + w_i` past `T_i`, or an overflow | not established: no single-job bound applies | `not-established` |
| an enabled source undeclared, or declared sources differing from enabled ones (condition 5) | refused (F17) | `unsupported-profile`, the profile's `unmodeled-interrupt-load` |
| a declared fact with a value outside `/1`: nesting, a threshold mask, a tick timer or edge compare, early release, a scheduler lock or deferred preemption, lazy switching outside `S`, self-suspension, data shared outside masked sections, undeclared deferred work (conditions 2, 4–6, 8) | refused (F17) | `unsupported-profile` |
| an input, a task fact or a platform fact missing (condition 9), or a declared delay below its floor (condition 7), which is evidence that cannot be true rather than a system outside the model | refused: an unresolved bound | `analysis-inconclusive` |
| the exact utilisation sum too large for 128 bits, or the iteration budget reached | a named resource limit | `analysis-inconclusive` |
| any other condition failing (1–3, the no-loss inequalities of 7) | refused as outside the model, naming the condition | `unsupported-profile` |

⛔ **This variant never reports a counterexample.** It charges upper bounds and pessimistic terms, so its response
bound is safe and not exact. §7.4 is explicit: "Conservative analysis failure is `not-established` unless an exact
test or validated counterexample establishes failure." A validated witness from elsewhere, such as §13.4's event
timeline, can still make a set's verdict `counterexample`, and that witness is not this analysis's conclusion. The
baseline's own `counterexample` holds only where a synchronous release can occur: for sporadic tasks, for free
offsets, or for fixed offsets that admit a simultaneous release.

For a set, the verdict of highest precedence wins, by the ordering `Verdict::precedence` gives
(`crates/eadl-front/src/diagnostic.rs`, rendered in `docs/book/src/checking.md`): `unsupported-profile`, then
`not-established`, then `analysis-inconclusive`.

### 6. Expected results come from elsewhere

The expected response bounds of the variant's fixtures are derived by a context that reads this record and the
published analysis and never the implementation (`M2.6.3`), as `M2.2` derived the runtime's reference. Every
disagreement is classified as the implementation's, the derivation's, or a gap in this record, and answered here.

**§13.4's fixture under this variant is `M2.6.3`'s to compute, with every input pinned**: `ρ`, `δ`, `W_wake`, `CS`,
`γ`, `J^event`, and what releases L. The second reviewer computed `R_H = 10` and `R_L = 50` by hand. The third
found that those values need a floor of 4, which the record did not pin, so neither number is relied on here.
Settled in advance: the fixture releases L with no service, and the variant charges one, so a variant bound for L
above the fixture's exact timeline is expected and is not a disagreement (§7.4.1: "must not demand an exact
response-time answer").

**`M2.6.3`'s result, `2026-09-30`.** The deriving context read this record, the roadmap and the profile, and
never the implementation. It derived 18 fixtures by hand: 6 refusals, and 12 analysed sets covering every term,
every stop and every verdict. Implementation and derivation agree on every verdict, every bound and every
iterate (`crates/rt-analysis/tests/fixtures/runtime_expected.txt`, compared by
`crates/rt-analysis/tests/runtime_expected.rs`). §13.4's fixture, with every input pinned (`S = 2`, `C_rel = 1`,
`ρ = δ = W_wake = CS = γ = J^event = 0`, both tasks released by the timer at the floor `J^release = 3`), gives:
- `R_H = 9`, which holds;
- `R_L = 49`, `not-established` at deadlines 22 and 23.

The second reviewer's `10` and `50` are these bounds at `J^release = 4`. Both variant bounds lie at or above the
fixture's exact timeline, where H responds in 5 and L in 23.

**The open questions, and what `M2.6.3` settled:**
- whether a `J^release` that excludes services queued ahead suffices for the response bound, which would remove
  the double charge declared in §3. **Settled in the negative, with a residue.** Services queued ahead are counted
  by the ceilings, because their arrivals lie within their jitter before the window. But while the analysed task
  is not yet released, a queued service can decide a switch to lower-priority work, and that masked transition
  runs before the pending timer interrupt is taken, where no term of `w` charges it. Dropping the queue from
  `J^release` would therefore under-count by up to one `S` per queued service. That is sound only with those
  transitions kept in `J^release`, or with a platform fact that no switch is decided while another interrupt is
  pending. `/1` keeps the whole delay in `J^release`, and the double charge stays declared;
- the exact non-preemptive fixed-priority queueing form that would let the analysis compute `J_s` and
  `J^release` from interrupt priorities, instead of taking them as evidence. **Not settled.** The candidate is
  non-preemptive fixed-priority analysis in its corrected CAN form (Davis, Burns, Bril and Lukkien, 2007). It would
  need the masked run as blocking, the timer as a queue member whose arrivals are every release, and the residue
  above as extra items;
- whether RISC-V takes an interrupt pending at `mret` before any instruction of the resumed context. **Settled
  `2026-09-30` for a conforming hart** (`M2.10.1`): the [privileged specification](../book/src/ledger.md#riscv-privileged)
  says interrupt-trap conditions "must also be evaluated immediately following the execution of an xRET
  instruction". Whether the target and QEMU conform is a separate fact. `/1` keeps it a declared platform fact
  (condition 5).

## Why

- **Narrow and exact beats broad and loose.** Each widening, such as nesting, threshold masks or tick timers,
  changes blocking, delivery and the timer term together. `/1` admits the configuration the target and §13.4
  actually have, and refuses the rest by name. A later version widens it deliberately.
- **Admission before arithmetic.** An analysis that returns a number for a set outside its model is how a number
  with no meaning gets quoted. The baseline admits by constructor, and the variant does the same.
- **Every bound is either the engine's or the description's, never defaulted.** The description can state a release
  event's jitter. The timer's resolution, delivery and every cost are the platform's, and a missing one refuses
  rather than reading as zero.

## How to apply

- `M2.6.2` implements exactly this record. Anything it needs that the record does not say comes back here first.
- The description supplies `T`, `D`, `J^event`, the priorities and each source's `T_s`. The catalog (`M2.7`)
  supplies `S`, `W_wake`, `γ`, `C_rel`, `ρ`, `δ`, each source's `C_s`, acknowledge point and deferred-work fact,
  and the platform facts: `decision_catalog-records.md` §12 names each one and its facet. The resolved plan (`M4`)
  supplies the enabled set and the interrupt priorities. Until a plan exists, the caller supplies them, and
  condition 5 is only as good as that declaration. That limit is stated, not hidden.
- `C_i`, `CS_i`, `J^release` and `J_s` are **composite**. Each includes what no single owner knows: kernel code
  run for a job, contiguous masked runs, the application's masked activity in progress, and the services the
  plan's enabled set queues ahead. Until `M2.10` decides and reviews their composition, the caller supplies each
  whole, as §1 defines it, and the conclusion names it as an assumption with its evidence category.
  ⛔ *Amended `2026-09-30` by leaf `M2.7.1`.* This bullet first gave the catalog `CS`, `J^release` and the enabled
  set. The catalog cannot know them, and a catalog figure for any of them would under-charge. The model, `/1`, is
  unchanged: these are statements of who supplies an input, not of what the input means.
- Deriving `C` and `CS` from a trace needs each *critical section* interval to name the task it ran for.
  `cost-accounting/1` has no such owner, so in `/1` they are declared inputs. A ledger that derives them is a
  `cost-accounting/2` change, not this record's.
- A change to the recurrence, the charging or the conditions after a conclusion has been stated in `/1` is a new
  model version, `fixed-priority-with-overheads/2`, and invalidates every conclusion stated in `/1` (§15). The
  revision below came before any conclusion, so it is still `/1`.

## Review (`ROADMAP.md` §7.4: "reviewed applicability conditions")

`2026-09-30`: the first draft was reviewed by an independent context. It had not written the draft, it was told to
read no implementation, and it worked read-only against published theory, `ROADMAP.md` §7.3, §7.4, §7.4.1 and
§13.4, the profile and the ledger's categories. It confirmed several things:

- the recurrence's structure: jitter in the ceilings and added once to the response;
- the initial value;
- the single-job stop, and that `D_i ≤ T_i` needs no `T_i − J_i` companion;
- blocking once;
- the verdict mapping.

It made twenty findings. The table gives each answer as the first revision made it. The second review then
revised several of them: `B_i` is gone, `W_idle` became `W_wake`, and the floors changed. Where the tables differ,
the second table and the text above stand.

| # | Severity | Finding | Answer |
| --- | --- | --- | --- |
| 1 | unsound | `J_i` was taken from the description with `0` as default, so timer granularity and delivery were lost | `J_i = J_i^event + J_i^release`, the second engine knowledge, never defaulted (§1, conditions 7 and 9) |
| 2 | unsound | `C_i` was defined as ledger *task execution*, leaving the critical sections, kernel services and instrumentation run for the job charged nowhere | `C_i` is everything the job executes between its transition in and its completion, services excluded (§1); a ledger mapping needs an owner on *critical section* (How to apply) |
| 3 | gap | transitions and the idle window also block, and were sound only through an extra `S` the draft called exact | `B_i` includes `S` and `W_idle` (§2); stated as a deviation (§3) |
| 4 | gap | "the timer is a source like any other" is wrong for an event-driven timer, whose arrivals are every task's releases | the timer term runs over every task (§2); `/1` admits only the event-driven timer (condition 6) |
| 5 | wording | "exactly once" is false: a direct switch is charged twice, and the analysed job's own transition out lies outside its response | "at least once" (§3); the analysed job is charged `S`, not `2S`; the observation boundary is stated (§2) |
| 6 | gap | nothing prevented lost interrupt arrivals | the acknowledge point is an input, and condition 7 bounds `J_s` against `T_s` |
| 7 | gap | interrupt priority, nesting and masking were not inputs | `/1` admits no nesting and masking of every interrupt, and refuses the rest (conditions 5, §5) |
| 8 | gap | "every source is declared" cannot be checked against the declaration itself | the declared set must equal the build's enabled set (condition 5); the limit before `M4` is stated |
| 9 | gap | the cost of being preempted was in neither `C` nor `S` | `γ` per interfering job and service (§2), and condition 8 |
| 10 | gap | a conclusion built on observed maxima read as established | observed inputs are named as assumptions of the conclusion (§5) |
| 11 | gap | the boundary between a service and a transition was not stated | the decision to switch is the boundary (§3) |
| 12 | gap | the rounding direction was not stated | costs round up, separations and deadlines round down (§1) |
| 13 | minor | overflow is decidable, and an arbitrary iteration cap turns decidable cases inconclusive | overflow is the busy-period stop, so `not-established`; the step bound is derived and there is no other cap (§2) |
| 14 | wording | the double-charged section was misdescribed | "for any one section at most one of the two happens" (§3). The reviewer's tentative alternative, measuring from the level-`i` busy period with no `J_s`, is not adopted: it was not checked against the literature |
| 15 | wording | releases and arrivals | nominal releases; arrivals for sources (§1) |
| 16 | wording | "one formulation, cited" overstated a combination | assembled from named sources, with the deviations listed (§3); equation numbers left to `M2.6.3` |
| 17 | wording | precedence is not in §5.5, and a counterexample can still come from elsewhere | cites `Verdict::precedence`; a validated witness from elsewhere is named (§5) |
| 18 | wording | the baseline is exact only where a synchronous release can occur | said so (§5) |
| 19 | gap | the review the draft claimed to record was missing | this section |
| 20 | wording | whether application-level masked sections are admitted | they are, as `CS` (§1, condition 4) |

The reviewer's overall verdict was that the recurrence was correctly assembled, and that the record was not yet
fit to implement until findings 1–5 were fixed.

**Second review, same day, of the revision.** A new independent context, under the same constraints, found no term
of the recurrence that under-counts, given true inputs. It confirmed:
- the busy-period stop is consistent with the `k = i` ceiling;
- the monotone iteration reaches the least fixed point;
- the precedence cited is the code's.

The weakness was the admission layer, which is supposed to guard the inputs. Its sixteen findings, and where each
is answered:

| # | Severity | Finding | Answer |
| --- | --- | --- | --- |
| 1 | unsound | the floor `M` took the longest single activity, not the longest contiguous masked run, and it left out hardware delivery. A delay is a sum (masked run, queueing, new arrivals), so no-loss could not be established from `M` | `L` is the longest contiguous run with `+ S`, and `δ`, `ρ` are inputs; the floors are stated as necessary only; the full delay is engine evidence, named as an assumption; the queueing form is an open question (condition 7, §6) |
| 2 | gap | "fires" is not the nominal release: resolution, late reprogramming and edge compare were missed, and condition 6 could not be checked | `J^release` is defined from the nominal release, everything included; level compare, round-up, one counter and every-interrupt-releases are declared facts (§1, condition 6) |
| 3 | gap | ready at service start can precede the nominal release | ready at the later of the two (§2) |
| 4 | wording | after readiness nothing of lower priority runs, so `B_i` only repeated what `J_i` charged; two over-charges were undeclared | `B_i` removed, blocking carried in `J^release` (§2); both over-charges declared (§3) |
| 5 | wording | condition 2 did not name transitions as non-preemptible | named (condition 2) |
| 6 | gap | `T_i ≥ T_s` pointed the wrong way | a task released on every arrival has `T_i ≤ T_s`; a rate limit is an assumption (condition 5) |
| 7 | gap | exit acknowledgement ignored a switch, and `≤` admitted a coincident arrival | `J_s + C_s + S < T_s` (condition 7) |
| 8 | gap | the overflow claim was false for intermediate numerators | numerators formed so a small quotient never overflows; any remaining overflow is the busy-period stop (§2) |
| 9 | gap | the step bound was Θ(`T_i`), a hang in practice | an exact utilisation pre-check; the bound stated as pseudo-polynomial (§2) |
| 10 | gap | several conditions had no input to be checked against | each is a declared task or platform fact; a missing one is `analysis-inconclusive`, an unsupported value `unsupported-profile` (§1, §4, §5) |
| 11 | gap | `CS_i`, `J_s` and the acknowledge point defaulted | no input defaults (§1, condition 9) |
| 12 | gap | three evidence kinds, where §7.3 has four | §7.3's four; every input not analytically established is named (§1, §5) |
| 13 | wording | the idle quantity is pending-to-unmasked, not the masked window | `W_wake` (§1) |
| 14 | wording | instrumentation inside services and transitions | charged in `C_rel`, `C_s` or `S` (§3) |
| 15 | wording | fixed offsets can admit a simultaneous release | said so (§5) |
| 16 | information | §13.4 under the variant: `R_H = 10` holds; `R_L = 50` is not established; the fixture's release service is charged | recorded for `M2.6.3` (§6) |

**Third review, same day, of the second revision**, by a third new context told to break it. It proved the
no-blocking bound directly. Inside the level-`i` window only these run:
- services, each mapped to a release or arrival the ceilings count;
- higher-priority jobs and the analysed job;
- at most two transitions per interfering job, and one for the analysed job.

Folding a blocking time into every jitter reproduces the published `B_i` result. It also confirmed:
- the analysed job's single `S`;
- that the utilisation pre-check is exact in both directions.

It broke one claim, and made twelve other findings:

| # | Severity | Finding | Answer |
| --- | --- | --- | --- |
| 1 | unsound | early release: condition 6 required a service to release every due task, but not *only* due ones. With H (C 5, T 20), L (C 6, T 100, D 20) and K (C 1, T 100), a kernel that releases tasks due within 8 finishes L at 23 while the analysis gives `R_L = 20` | no early release is a declared, checked platform fact (condition 6), refused `unsupported-profile` when false and `analysis-inconclusive` when missing |
| 2 | gap | conditions 2, 4, 5 and 6 had no §1 input, and a scheduler lock would add blocking that `J^release` does not carry | §1's task facts and platform facts; conditions 2 and 4 name them (§1, §4, §5) |
| 3 | wording | if `C` ended at "completion", an interfering job's completion path was charged nowhere | `C` runs up to the decided switch, completion path included (§1) |
| 4 | gap | what a pre-check overflow does was not said | `analysis-inconclusive`, a named resource limit; and a stated iteration budget (§2, §5) |
| 5 | gap | §6's fixture values depended on inputs the record did not pin | the values are withdrawn, and `M2.6.3` pins every input (§6) |
| 6 | wording | the floor demanded the whole resolution | `ρ` is the largest delay the rounding adds (§1) |
| 7 | wording | an absent `jitter` clause looked like a default | it is the language's meaning, not the analysis's default (§1) |
| 8 | wording | the timer is enabled but not a declared source | the declared sources plus the timer; non-maskable interrupts and firmware traps included (§1, condition 5) |
| 9 | wording | a source-released task's nominal release was undefined, and `T_i ≤ T_s` mixes nominal and arrival separation | the arrival; the comparison stated as safe and possibly over-refusing (§1, condition 5) |
| 10 | wording | `C_rel`'s decision, idle/wakeup's split, and a delay below its floor mapped to the wrong verdict | the decision up to the switch point; `W_wake` then `S`; a delay below its floor is `analysis-inconclusive` (§1, §5) |
| 11 | gap (low) | a spurious re-fire, and due-checks on other exits | a timer interrupt is raised only when a release is due; only the timer service releases timer tasks (condition 6) |
| 12 | wording (low) | `L` assumes transitions end unmasked; unmaskable interrupts and traps | a platform fact (condition 5); named in the enabled set (§1) |
| 13 | gap | a lazily deferred context save falls outside `S` | eager switching, or `S` includes the deferred work: a platform fact (condition 5) |

Each round has found something that could under-estimate a response: the first two in the inputs, the third in an
admission condition. So the record is implemented as the contract, and its next check is `M2.6.3`. That context
reads the record without the implementation, derives the fixtures' expected values, and reports anything it cannot
derive from the record alone as a gap in the record. Three points of theory no reviewer settled are §6's open
questions, and `/1` relies on none of them.

**`M2.6.3`'s derivation, a fourth reader, same day.** Besides the results in §6, it reported what it could not
derive from the record alone:

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 1 | gap | condition 1 had no §1 input, and a set with both a missing input and two processors turns on it | "there is one processor" is a platform fact (§1); the implementation already declared it |
| 2 | gap | when several conditions fail at once, the verdict was not stated | every condition is checked, and the highest-precedence verdict wins (§4) |
| 3 | gap | whether a task a source releases counts as its deferred work | no: released tasks are analysed through `released by`, and `deferred` names any other task that runs deferred work (§1) |
| 4 | gap | what `J^event` means for a timer-released task, and which ceilings take it | how far the description lets a release vary; it enters every ceiling, the timer term included (§1) |
| 5 | wording | "below one the right-hand side is bounded" | "at most `c + U·w`, so a least fixed point exists" (§2) |
| 6 | gap | condition 8 could make `γ` charge lost state a second time | `C`, `C_s`, `C_rel` and `S` exclude what `γ` charges (condition 8) |
| 7 | confirmation | the third review's `R_L = 20`, and the §13.4 bounds | reproduced (§6) |
| 8 | open questions | Q1, Q2, Q3 | Q1 settled in the negative with its residue; Q2 and Q3 not settled (§6) |
| 9 | not exercised | the iteration budget, 128-bit overflow, a missing task fact, a rate-limited release, the naming of assumptions | a missing task fact and a rate-limited release now have legs in `runtime_variant.rs`, and the naming of evidence and the catalog's enabled set already had one. The budget and the overflow need inputs past `u64` sums and a million iterations; they are stated as unexercised, not claimed |

It also said it had derived "19 fixtures", where the file holds 18, a miscount in its summary. The fixture file
is its Part 1, copied verbatim.
