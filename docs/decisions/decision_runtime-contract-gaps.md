# Five runtime-semantics questions the contract does not decide

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `resolved` 2026-09-13 by leaf `M2.9`, under authority delegated by the director; **amended
  `2026-10-01`** after the independent review of `ROADMAP.md` §3.1.1 — see *Review and amendment* below
- **Owner / source:** found by leaf `M2.2`, by two independent routes: an independently derived
  reference model flagged them while being written, and a differential comparison hit the first
  of them on its opening sequence.

## Resolution

All five are decided. The director delegated the decision on 2026-09-13 with the instruction that
it be signoff-grade; §14.1's requirement that such a change carry "a separate rationale and
independent review of the underlying requirement or source" is met by recording the rationale
here and in `ROADMAP.md` §3.1.1, and by re-deriving the reference model **from the amended
contract, by the context that has still never read `crates/rt-core`** — so the agreement that
follows is evidence rather than an author agreeing with himself.

| # | Question | Decision | Changed |
| --- | --- | --- | --- |
| 1 | overrun attribution and escalation | detection applies the policy, to the **overrunning** task even when it is not running; the other three faults attribute to the running context | `ROADMAP.md` §3.1.1 (new) |
| 2 | the empty task set | **refused** | `ROADMAP.md` §3.1.1; `rt-core` |
| 3 | priority rank `0` | **refused**; rank is `N ≥ 1`, and `runtime index = rank − 1` is stated | `decision_priority-comparison-direction.md`; `rt-core` |
| 4 | containable fault inside a masked region | **escalates to fatal** | `ROADMAP.md` §3.1.1; `rt-core` |
| 5 | the bound on mask nesting | **declared, and exceeding it is refused** — neither wrapping nor saturating | `ROADMAP.md` §3.1.1; `rt-core` |

⭐ **Four of the five went against the implementation**, which is the outcome that makes the
exercise worth its cost: an author's reading lost to a contract reading four times out of five,
and gap 1 — where the implementation was right — was right for a reason the contract had never
stated, so it needed writing down just as much.

## Review and amendment (`2026-10-01`)

The reference model, re-derived from the amended §3.1.1 by the context that had never read
`crates/rt-core`, reviewed the amendment as it went and returned five criticisms. They are items (a)–(e) of §6 in
[[decision_findings-for-director-review]], ruled `2026-10-01` under the director's delegation, and each is now a
marked amendment in §3.1.1:

| Item | The question | Ruling | Where |
| --- | --- | --- | --- |
| (b) | a second release while the task's latch already holds one | an overrun, kept **beside** the latched release and judged at **delivery**, outside the masked region, by the task's policy; the triggering release is the policy's (`SkipLateJob`: the next job; `Fault`: goes with the task) | rule 1 |
| (a) | a job completing inside a masked region it opened | allowed; the completion closes the job's sections (depth to zero), delivers what was latched as at the outermost unmask, then the schedule is decided | rule 4 (new) |
| (c) | the unexpected trap's class | the deliberate fatal trap | the table |
| (d) | attribution of the synchronous faults | one rule: the executing context, which for a stack guard is the task whose guard was breached | rule 2 |
| (e) | ground 2 of rule 3 | containment means *resuming the schedule* from an inconsistent state, whichever task the fault is attributed to | rule 3 |

⚠️ **(b) changed gap 1's answer at one edge, and (a) changed `rt-core`.** Before (b), a doubled latch was an
overrun detected inside the masked region, so rule 3 made it fatal: a periodic task whose second release landed one
instruction inside a critical section killed the system, where one instruction later it would have been contained.
Before (a), `rt-core` let a job complete with the mask held and left the depth above zero with no owner — the
condition ground 1 of rule 3 calls fatal, reached without a fault. The grounds are in the findings record: FreeRTOS's
pended ticks, AUTOSAR OS re-enabling interrupts for a task that ends with them disabled, and the accepted
composition's `CS_i`, the masked run that ends at completion.

## Carried into both models (`2026-10-01`)

Both models carry the five resolutions and the amendment, and each test in
`crates/rt-core/tests/differential.rs` that recorded a disagreement now records the agreement on both sides (`d1`–`d5`,
with `d6` and `d7` for rulings (b) and (a)). The randomised comparison now reaches what `M2.2` had to exclude:
overrunning releases, at once or latched beside a held one; completions inside a masked region; overruns a monitor
raises, masked or not; and the synchronous faults, with their attribution compared. Measured `2026-10-01`: 400
sequences of up to 60 events across four policy patterns, no divergence, coverage floors about half of what was
reached. A mutation run reverting each ruling, one at a time, turns the tests red in both models.

Rewriting the tests found five things more:

| Found | Kind | Outcome |
| --- | --- | --- |
| a trap or an assertion reported against no task | `rt-core` departing from §3.1.1's table | attributed to the running task; to no task only while none runs |
| an overrun raised without a release started a job, reported as a release | `rt-core` departing from rule 1, whose triggering release is the policy's | the late job abandoned, no job started; a skipped job reported as `JobSkipped`, on both paths |
| a later fault overwrote the one that halted the runtime | `rt-core` departing from §8.1's "preserve … diagnostic evidence" | the first fault is kept |
| ranks with a gap refused | **a new question** — the language admits them | decided `2026-10-01`, for the director's review: ranks lower by their order, `runtime index = \|hp(i)\|` ([[decision_priority-comparison-direction]], item 3) |
| what a halted runtime leaves in its task table | **a new question** — `rt-core` marks the attributed task, the reference freezes the table | **left to the implementation**: §8.1 asks for evidence, not a format; asserted on both sides (`d9`) |

## The fact / decision

`crates/rt-core` and `crates/rt-reference` implement the same runtime semantics from the same
written contract. Over **400 randomised sequences of 40 events each** they agree exactly — in the
region the contract decides. **Five questions it does not decide** produced different, defensible
answers from two readers who each read only the contract.

⛔ **They are recorded, not silently resolved.** §14.1: *"Implementation changes cannot silently
weaken requirements, adjust expected oracle results, or drop failing scenarios. Such changes
require a separate rationale and independent review of the underlying requirement or source."*
Picking a side here is a change to `ROADMAP.md` or to the published profile, which is a reviewed
decision, not an implementer's. Each is asserted on **both** sides in
`crates/rt-core/tests/differential.rs` so the disagreement cannot drift unnoticed.

## The five

### 1. Overrun attribution and escalation — ⚠️ the significant one

§3.1 requires a "defined overrun … policy" and §7.3 puts "overrun behavior" in the per-task
record. Neither says whether **detecting** an overrun applies that policy by itself, nor whether
a fault may be attributed to a task that is **not running**.

| Model | Reading |
| --- | --- |
| `rt-core` | a second release *is* the overrun, so the policy applies at once, to the task that overran — which need not be running |
| `rt-reference` | reports the overrun; escalation is the caller's. Faults attach only to the running task: "a fault attributed to a task that was not running would be a claim about concurrency the profile does not admit" |

**Proposed resolution.** Both are right about different faults, and the contract conflates them.
A *trap* belongs to whoever executed the instruction — the reference is correct there. An
*overrun* is detected by a release interrupt and is precisely a statement about a task that is
**not** making progress — the implementation is correct there. The underlying defect is that §3.1
lists "overrun, unexpected-trap, stack-guard, and assertion failure" in one policy sentence while
§8.1's three-way triage ("expected errors, violated internal invariants, deliberate fatal traps")
does not map onto that list. **Recommend: §3.1 states the mapping explicitly**, and says that an
overrun applies its policy on detection, to the overrunning task.

### 2. The empty task set

§3.1 says "finite static task set", and the empty set is finite. `rt-reference` refuses it at
boot; `rt-core` accepts it and idles. **Recommend: refuse.** A system with no workload makes
§7.2's second timing obligation vacuous, and a vacuously passing schedulability result is what
§7.1 exists to prevent.

### 3. The meaning of priority rank `0`

`decision_priority-comparison-direction.md` says "`1` is the highest" and says nothing about `0`.
`rt-reference` refuses rank 0. `rt-core` has no explicit rank at all: a task's **index** is its
rank, and indices start at **0**.

⛔ **So the runtime's highest priority is `0` while the language's is `1`, and that off-by-one is
written down nowhere.** Anything mapping an eADL `(priority N)` clause onto a runtime task index
must subtract one. **Recommend: state the mapping in the decision record, and say whether rank 0
is admissible in a description** (recommend: no — admitting it moves the top of the range by
inference, which §15 makes a versioned language change).

### 4. A containable fault raised inside a masked region

§8.1 requires masking to be modelled and a defined fatal handler; nothing covers this case.
`rt-reference` escalates to fatal, reasoning from the contract: terminating a job that holds the
mask leaves the depth above zero with no owner, so interrupts never return and the runtime dies
silently; forcing the depth to zero instead re-enables interrupts mid-region with the invariants
half-restored, and §8.1 offers no third option. `rt-core` does not model the interaction at all.

**Recommend: adopt the reference's reading**, and treat this as an `rt-core` defect rather than a
contract gap — the reasoning is drawn entirely from text that already exists.

### 5. The bound on mask nesting

§3.1 requires "bounded kernel critical sections" and §7.3 requires each interrupt source to
declare its "masking constraints"; no maximum depth is stated. `rt-reference` bounds it and
**refuses** beyond; `rt-core` saturates a counter.

**Recommend: refuse.** The bound's value is arbitrary; the refusal is not. A counter that wraps
re-enables interrupts inside a critical section and reports success, and a counter that saturates
stops counting, so the matching unmasks no longer balance — the same failure by a slower route.

## Why

A specification gap is invisible while one person implements it: they resolve it, correctly or
not, and the resolution looks like the specification. It becomes visible the moment a second
reader derives the same thing without seeing the first — which is what §12 M2 is asking for when
it says *"a checker sharing the same erroneous recurrence with its reference does not qualify as
independent."*

⭐ **The agreement was the weak result; the disagreements are the strong one.** Two models that
disagree cannot have been copied from each other, so each of these five is simultaneously
evidence of independence and a real defect in the written contract.

## How to apply

- Do not "fix" either model to make a test pass. Each ratchet in `crates/rt-core/tests/differential.rs`
  that asserted a disagreement now asserts the agreement the resolution produced, on both sides; it is
  rewritten, never deleted, because it is the only evidence the gap was closed (§14.1).
- A further reading difference between the models is a new gap: record it here or in a new record, and
  route it to the director before either model moves.
- Resolving 1, 2, 3 and 5 meant editing `ROADMAP.md` §3.1.1 and [[decision_priority-comparison-direction]];
  the `2026-10-01` amendment went through the director's findings record for the same reason. Gap 4
  needed no contract change, only an `rt-core` fix.
