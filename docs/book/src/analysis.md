# What the scheduling checker establishes

The first analysis in the toolchain is the **idealized zero-overhead response-time baseline** of
`ROADMAP.md` §7.4. For a fixed-priority task set it computes, for each task, a bound on how long
a job can take to finish:

```text
R_i^(0)   = C_i
R_i^(k+1) = C_i + Σ_{j ∈ hp(i)} ⌈ R_i^(k) / T_j ⌉ · C_j
```

It is small, it is exact, and — this is the part that matters — it establishes much less than it
looks like it does.

## What it may be used for, and what it may not

§7.4 is unusually direct about its own limits:

> This baseline is for validating arithmetic and theorem implementation. **It must not be
> selected for a physical runtime whose nonzero overhead and jitter are omitted.**

So the analysis is wired so that a positive answer cannot be detached from the conditions that
make it true. There is no function that returns "schedulable". The only positive conclusion
available is §7.1's *conditional analysis* form, which will not exist without a named model and a
non-empty assumption list:

```text
deadlines follow in idealized-zero-overhead/1 (ROADMAP.md §7.4) under:
  one processor
  independent preemptible tasks
  distinct fixed priorities
  no release jitter
  no blocking
  no overhead — no context-switch, interrupt, or dispatch cost is charged
  constrained deadlines (D ≤ T)
  declared worst-case computation times, whose own evidence is separate
```

A reader who quotes the conclusion quotes "no overhead" with it. That is the best a type system
can do about a misuse that is fundamentally about what the numbers *mean*.

The second guard is earlier: a task set the model does not cover is **refused at construction**.
Duplicate priorities, a deadline longer than the separation, a zero computation bound — each is a
condition of §7.4, and each produces a refusal that says which one and why, rather than a number
for a system the model does not describe.

## Three outcomes, and one that is easy to get wrong

| Outcome | Means |
| --- | --- |
| converged, within the deadline | the property holds **in this model** |
| converged, past the deadline | a concrete violation witness in this model |
| did not converge, or overflowed | nothing is established either way |

The third is the trap. §7.4:

> Conservative analysis failure is `not-established` unless an exact test or validated
> counterexample establishes failure.

An implementation that reported a resource limit as "unschedulable" would be inventing a
counterexample nobody found. So non-convergence and overflow are `analysis-inconclusive`, they
name which of the two explicit limits stopped them, and they never become a deadline miss.

## The witness

§7.4 asks for "the recurrence sequence recorded as a checkable witness", so every result carries
the iterates that produced it. For the §13.2 baseline task `C`:

```text
C: 2 → 4 → 4
```

which is exactly what the roadmap's prose describes — "the iteration starts at 2 and reaches 4,
then remains 4". Change only `C`'s deadline to 3 and the same sequence becomes a counterexample:

```text
in idealized-zero-overhead/1 (ROADMAP.md §7.4), task `C` has response-time bound 4 against a
deadline of 3; recurrence C: 2 → 4 → 4
```

A reader can re-derive that by hand in a minute, which is the point of recording it.

## The expected answers live in the roadmap

§13.2 publishes the baseline and its expected bounds — A `1`, B `2`, C `4`. The F18 test does not
copy those numbers; it **parses them out of `ROADMAP.md`**.

That is a rule from §14.1: *"implementation changes cannot silently weaken requirements, adjust
expected oracle results, or drop failing scenarios."* A test holding its own copy makes adjusting
the oracle a one-line edit that looks like a fix. Reading the specification makes the expectation
and the requirement the same object, so changing the answer means changing a requirement — in a
diff a reviewer recognises as one.

## How time is charged

The idealized baseline charges **nothing** for overhead, which is its declared assumption. Any
analysis that does charge overhead has to say how, and §7.4.1 requires that to be a **versioned
contract** rather than an implementation detail: `cost-accounting/1`, published in
`docs/analysis/cost-accounting-v1.md` and declared as data beside the code, with a test that
fails if the two ever diverge.

The rule at its centre is one sentence:

> Every physical execution interval in a fixed trace has **one primary ledger category**. […]
> Charge only mutually disjoint intervals when asserting exact totals.

⭐ **That is enforced by construction, not by review.** A ledger is a set of half-open intervals,
and sealing one as an *exact trace* refuses two things:

- an **overlap** — time charged twice. This is §7.4.1's own example: counting the same interrupt
  entry once in an ISR term and again inside a context-switch term.
- a **gap** — time charged to nothing. An omitted cost *is* a hole; the trace says the processor
  was busy and the ledger says nothing was spent.

Both matter for the same reason: when they happen, **the total still looks plausible**. Nothing
about the number invites suspicion, so a check that depends on suspicion never fires.

### Three kinds of total, deliberately not interchangeable

| Kind | May over-count | Supports |
| --- | --- | --- |
| `exact-trace` | no | an exact statement about one fixed trace |
| `safe-envelope` | **yes, declared** | an analytical upper bound |
| `observed-maximum` | n/a | an empirical observation, and nothing more |

§7.4.1 permits deliberate pessimism and forbids hiding it — "the requirement is no *undocumented*
omission or duplicate charge, not a ban on sound pessimism" — so an envelope is allowed to charge
the same interval twice, and has to say that is what it is.

## F29: the fixture built so an omission cannot hide

§13.4 specifies a repeated-preemption scenario completely — costs, transitions, preemptibility,
and the instant-by-instant trace it must produce — and says why:

> It deliberately creates two preemptions of one low-priority job, so **omitted interrupt or
> resume costs can turn a real miss in the fixture into a false pass**.

The correct trace runs from 0 to 23, in twelve intervals, and its ledger totals
`8 (L) + 4 (H) + 1 (initial dispatch) + 2 (ISRs) + 8 (four switches) = 23`. `H`'s two jobs finish
at 9 and 19 — five units after their *nominal* releases, because the release interrupt's delay is
inside the response time. `L` finishes at 23 and misses a deadline of 22 by one unit.

It is checked **two ways that are not derived from each other**: the roadmap's own expected-trace
table, parsed out of `ROADMAP.md`, and a simulator written from the operational rules in the prose
above it. They agree interval for interval.

### The controls, and why they re-simulate

| Control | Correct answer | What it exposes |
| --- | ---: | --- |
| the specified model | `L` at 23 | the exact ledger, every unit charged once |
| omit the timer ISR cost | `L` at **21** | a false pass at deadline 22 |
| omit the `H`→`L` resume switch | `L` at **14** | a whole interfering job disappears |
| charge the ISR intervals again inside task cost | total **25** | duplicate interval ownership |

⛔ **The first three re-run the model; they do not do arithmetic on the answer.** §13.4:

> Re-simulate mutations that change execution timing: they can change the number of interfering
> releases, so subtracting a fixed number from the original response is not generally valid.

The third control is the proof. Deleting four units of resume cost does not give `19`. It lets
`L` finish at exactly 14 — the instant of the second nominal release — and the rule "record
completion before processing the new release" then removes that release's interference
altogether. One interfering job vanishes. A control implemented by subtraction would have agreed
with the roadmap on the second row by luck and been wrong here.

The fourth control is the odd one out, and deliberately: it works on the *original* fixed trace,
because the mistake it models is an accounting error rather than a timing one. It needs no
detection logic at all — the ledger simply will not seal.

⚠️ F29 is a **synthetic accounting fixture**. It establishes concrete cost coverage and detects
three known mistakes. It is not a benchmark, not a claim about any board, and not a substitute
for reviewing the runtime accounting model and its theorem conditions.

## What is still owed

⚠️ **Nothing here may be cited for a claim about a running system.** §7.4 requires a
runtime-applicable variant accounting for bounded critical sections, release jitter, timer and
other interrupt interference, and context-switch costs, before any `rt-static-up-v1` timing
result is acceptable. That is leaf `M2.6`, and its control is fixture **F29** (§13.4) — a fully
specified repeated-preemption trace whose ledger totals 23, built so that an omitted interrupt or
resume cost turns a real miss into a false pass.
