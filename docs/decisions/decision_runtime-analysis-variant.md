# The runtime analysis variant: what it charges, when it applies, and what it refuses

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [QEMU](../book/src/ledger.md#qemu), [the PLIC specification](../book/src/ledger.md#riscv-plic)
  and [the RISC-V privileged specification](../book/src/ledger.md#riscv-privileged) — versions, scope and limits in
  the ledger
- **Owner / source:** leaf `M2.6.1` (`docs/tasks/M2.md`), deciding what `ROADMAP.md` §7.4 requires before a
  `rt-static-up-v1` timing result may be accepted: "an analysis variant that accounts for its bounded critical
  sections, release jitter, timer/other interrupt interference, and context-switch costs". The review §7.4 asks
  for, and the answer to each of its findings, is the last section.

## The fact / decision

The variant is a fixed-priority preemptive response-time analysis with release jitter, blocking by
non-preemptible sections, interrupt and timer interference, context-switch costs and a preemption-delay term. Its
model identifier is `fixed-priority-with-overheads/2`, apart from the baseline's `idealized-zero-overhead/1`, so a
conclusion of one is never read as the other's. `/1` is the text before `2026-10-02`, when `M2.11` changed its
admission conditions, and every change from then on is `/2`'s (How to apply).

**It is assembled from published terms, and says where it deviates.** The jitter and blocking form is Tindell,
Burns and Wellings, "An extendible approach for analysing fixed priority hard real-time tasks" (*Real-Time
Systems* 6(2), 1994), which extends the recurrence of Audsley et al. (1993) that §7.4 cites. The per-release
timer cost and the switch charging follow the system-overheads treatment of Burns, Tindell and Wellings,
"Effective analysis for engineering real-time fixed priority schedulers" (*IEEE TSE* 21(5), 1995), and of Burns
and Wellings' textbook. The preemption-delay term follows the cache-related preemption delay literature
(Busquets-Mataix et al., 1996). The deviations are listed in §3. ⚠️ Equation numbers were not checked against the
papers here; `M2.6.3`'s independent derivation is where the combination is checked.

**The model is deliberately narrow.** It admits one timer kind (event-driven, one interrupt per due release,
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
| `C_s`, `T_s`, `J_s` | each other interrupt source | one service, entry to exit, excluding work deferred to a task; minimum separation between arrivals; the most by which its service can start after an arrival, everything included as for `J^release`. A service starts at its trap's entry; each trap runs one service (condition 5) |
| acknowledge point, interrupt priority, deferred work | each other interrupt source | at entry or at exit; its rank among interrupts; the declared task, if any, that runs work the service defers beyond releasing the tasks declared as `released by` it, whose work is analysed through them |
| `S` | the platform | one context transition: from a decided switch, saving the outgoing context beyond what interrupt entry saved, to the incoming one running. It bounds the ledger's *task switch*, *initial dispatch* and *idle/wakeup* alike |
| `W_wake` | the platform | from an interrupt becoming pending while the idle context waits, to interrupts being unmasked: the idle check plus the wake latency. The ledger's *idle/wakeup* is split here: the wake is `W_wake`, before the service; the switch out of idle is `S` |
| `γ` | the platform | the preemption delay: the most one preemption or service adds to the preempted execution through lost cache, pipeline, predictor or TLB state. `0` only with evidence that the target keeps no such state |
| `ρ`, `δ` | the platform | the largest delay the compare's rounding adds to a release (at most one resolution step less one unit; `0` when releases fall on ticks); the hardware's delivery latency from an interrupt becoming pending, unmasked, to its entry |
| the platform facts | the platform | each a declared yes or no, with its evidence: <br>• there is one processor; <br>• the timer is event-driven, its compare has level semantics and rounds up, and the due-check uses the compare's counter and rounding; <br>• **a service releases a task only once its nominal release has passed** (no early release), releases every due one; the timer's interrupt is taken only when a release is due; <br>• only the timer service releases timer-released tasks; <br>• interrupts do not nest; every masked section masks every interrupt; every service preempts every task; <br>• **every interrupt taken runs one service, paid for by a due release or by an arrival**: an interrupt other than the timer's is taken only while a request an arrival made is pending, and its trap's one claim takes a request; an arrival makes at most one request (condition 5); <br>• preemption happens at every point outside a masked section, a service or a transition (no scheduler lock, no deferred preemption); <br>• a pending interrupt is taken before the resumed context executes an instruction, and every transition, into idle included, ends unmasked; <br>• context switching is eager, or `S` includes every deferred save and restore a transition causes (lazy floating-point state, for instance); <br>• each cost bound holds under any preemption pattern |
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
(`decision_catalog-records.md` §12); `M2.7.5` composes them. What each input **means** is unchanged, and so was the model, then `/1`. A conclusion
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
- **The source term charges everything a source's services run in the window** that starts at the analysed job's
  readiness, by condition 5, under which each interrupt trap runs one service. Services do not nest, and the job
  becomes ready inside, or at the start of, the service that releases it, so every other interrupt trap that runs
  in the window starts in it. A source's trap whose claim falls in the window serves a request whose arrival came
  before that claim and at most `J_s` before the trap's entry, so the ceiling counts it. So does the service that
  releases a source-released job: its arrival is that job's release, at or before readiness and at most `J_s`
  before the service started. A source's trap whose claim falls after the window leaves only its entry in it. It was
  taken while some request was pending, whose arrival came before it, and that request is claimed by this trap or a
  later one, so its service starts no earlier than this trap and at most `J_s` after its arrival: the ceiling counts
  that arrival, and its `C_s` covers an entry. Each arrival is charged once: for its own service, or for the entry
  of the one trap the window's end cuts, its own service then that trap or one after it. A timer trap runs the
  timer's service alone, paid for by the release due when it was taken (condition 6), which the timer term counts.
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
   - **Every interrupt taken runs one service, paid for by a due release or by an arrival.** An arrival of a source
     makes a **request** at the interrupt controller, **pending** from then until a **claim**, a read of the
     controller that takes it. Each trap taken for an interrupt runs exactly one service: a timer trap the timer's,
     claiming nothing, taken only when a release is due (condition 6); any other trap a declared source's, taken
     only while a request an arrival made is pending, whose one claim takes a request. A trap taken on a
     notification that a claim has made stale is taken while nothing is pending, even when a request arrives before
     its claim reads. An arrival makes at most one request. §2 shows why the source term then charges every
     service. *Added `2026-10-02` by leaf `M2.11`*, after the composition's review found services that a conforming
     controller, or the emulator's, allows and no arrival pays for
     (`decision_runtime-composite-inputs.md`, rounds 4 to 6, N2, O1 and P2): a trap taken on a notification that
     lags the claim which emptied it, since the [PLIC specification](../book/src/ledger.md#riscv-plic) lets one
     "take some time to be received", whose claim finds nothing or an arrival made after the trap; and a second
     service for one arrival, since the controller forwards a new request on a completion "if the interrupt is
     level-triggered and the interrupt is still asserted", and [QEMU](../book/src/ledger.md#qemu)'s pends one on any
     raise of the line. Each is charged nowhere, and repeated it can keep the timer from being taken at all.
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
   - A timer interrupt is taken only when a release is due, and its service releases every due task. A release is
     **due** once the counter has reached its nominal instant, rounded as the compare rounds, and it has not been
     performed. *Taken*, not *raised* (`M2.11`): an interrupt still pending after a service moved the compare on,
     which the [privileged specification](../book/src/ledger.md#riscv-privileged) allows, since the timer interrupt
     follows the comparison "eventually, but not necessarily immediately", could be served with no release due, and
     the timer term counts releases.
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
| every task converged with `R_i ≤ D_i` | the deadlines hold in `fixed-priority-with-overheads/2`, under §4's conditions. Every input whose §7.3 evidence is not analytically established is named as an assumption with its category, and so are the enabled set before `M4` and any rate-limited source release | (`HoldsUnderAssumptions`) |
| a task converged with `R_i > D_i` | not established: the bound is an envelope, so exceeding the deadline shows no miss | `not-established` |
| interference utilisation at or above one, a task's `J_i + w_i` past `T_i`, or an overflow | not established: no single-job bound applies | `not-established` |
| an enabled source undeclared, or declared sources differing from enabled ones (condition 5) | refused (F17) | `unsupported-profile`, the profile's `unmodeled-interrupt-load` |
| a declared fact with a value outside `/2`: nesting, a threshold mask, a tick timer or edge compare, early release, an interrupt neither a due release nor an arrival pays for, a scheduler lock or deferred preemption, lazy switching outside `S`, self-suspension, data shared outside masked sections, undeclared deferred work (conditions 2, 4–6, 8) | refused (F17) | `unsupported-profile` |
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
  pending. The model keeps the whole delay in `J^release`, and the double charge stays declared;
- the exact non-preemptive fixed-priority queueing form that would let the analysis compute `J_s` and
  `J^release` from interrupt priorities, instead of taking them as evidence. **Not settled.** The candidate is
  non-preemptive fixed-priority analysis in its corrected CAN form (Davis, Burns, Bril and Lukkien, 2007). It would
  need the masked run as blocking, the timer as a queue member whose arrivals are every release, and the residue
  above as extra items;
- whether RISC-V takes an interrupt pending at `mret` before any instruction of the resumed context. **Settled
  `2026-09-30` for a conforming hart** (`M2.10.1`): the [privileged specification](../book/src/ledger.md#riscv-privileged)
  says interrupt-trap conditions "must also be evaluated immediately following the execution of an xRET
  instruction". Whether the target and QEMU conform is a separate fact. The model keeps it a declared platform fact
  (condition 5).

## Why

- **Narrow and exact beats broad and loose.** Each widening, such as nesting, threshold masks or tick timers,
  changes blocking, delivery and the timer term together. The model admits the configuration the target and §13.4
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
  set. The catalog cannot know them, and a catalog figure for any of them would under-charge. The model, then `/1`, was
  unchanged: these are statements of who supplies an input, not of what the input means.
- Deriving `C` and `CS` from a trace needs each *critical section* interval to name the task it ran for.
  `cost-accounting/1` has no such owner, so in the model they are declared inputs. A ledger that derives them is a
  `cost-accounting/2` change, not this record's.
- A change to the recurrence, the charging or the conditions after a conclusion has been stated is a new model
  version, and invalidates every conclusion stated in the one before. The rule is this record's; `ROADMAP.md` §15
  asks that any changed behaviour be explicit. The review revision below came before any conclusion, so it stayed
  `/1`.
- **`/2`, `2026-10-02`, leaf `M2.11`:** condition 5 gained "every interrupt taken runs one service, paid for by a
  due release or by an arrival", with the terms it defines, and condition 6 reads *taken* where it read *raised*
  and defines *due*. `/1` is the text before commit `1c61e70`, which first added to condition 5 in place; that
  addition and every later one are `/2`'s. `/1` had stated conclusions: §6's expected results, the book's account
  of them, and the variant's tests. So every conclusion stated in `/1` is void. The recurrence's formulas are
  unchanged, and the platforms of those sets are stipulated to meet every condition: the expected
  results' `facts=all` is read as every fact `yes`, the new one included, and the tests declare each `yes`. Re-run
  under `/2` they give the same verdicts, bounds and iterates (`crates/rt-analysis/tests/runtime_expected.rs`). No
  command reached the variant under `/1`: `archogen analyze` is `Unimplemented` in
  `crates/archogen-cli/src/spec.rs`, and no crate but `rt-analysis` depends on it.

## Review (`ROADMAP.md` §7.4: "reviewed applicability conditions")

Four readers checked this record on `2026-09-30`, and two more `M2.11`'s change on `2026-10-02`. Each was a new
read-only context that had not written it. Their
findings, and the answer to each, are kept in
[`decision_runtime-analysis-variant-reviews.md`](../reviews/decision_runtime-analysis-variant-reviews.md).

| Reader | Findings | What it turned on | Outcome |
| --- | --- | --- | --- |
| first review | 20 | two unsound inputs: a jitter defaulted to `0`, and a `C_i` that left the job's kernel work charged nowhere | "not yet fit to implement until findings 1–5 were fixed"; all answered |
| second review, of the revision | 16 | no term of the recurrence under-counts, given true inputs | all answered |
| third review, told to break it | 13 | early release, a deadline missed by 3 in a set the draft admitted; the no-blocking bound proved directly | condition 6 gained "no early release"; all answered |
| `M2.6.3`'s derivation | 9 | what could not be derived from the record alone | all answered; the third review's bounds reproduced |
| `M2.11`'s first review, `2026-10-02` | 14 | a trap taken before the arrival it serves escaped the source term: 33 against an admitted 27 | 5 defects; condition 5 stated at the trap, condition 6 *taken*, the model `/2`; all answered |
| `M2.11`'s second review, same day | 15 | the first's defects closed and no schedule found past the bound; a false justification in the composition record's step 4 | 1 defect; one service per trap, the terms defined; all answered |
