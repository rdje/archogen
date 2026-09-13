# Five runtime-semantics questions the contract does not decide

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `active` — **four of five need a reviewed contract decision (director)**
- **Owner / source:** found by leaf `M2.2`, by two independent routes: an independently derived
  reference model flagged them while being written, and a differential comparison hit the first
  of them on its opening sequence.

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

- Do not "fix" either model to make the list shorter. The ratchet tests in
  `crates/rt-core/tests/differential.rs` assert both sides, so silently resolving one fails.
- Resolving 1, 2, 3 and 5 means editing `ROADMAP.md` §3.1 / the published profile /
  [[decision_priority-comparison-direction]], under review. Leaf `M2.9` owns carrying whatever is
  decided into both models and deleting the corresponding ratchet.
- 4 is the exception: it needs no contract change, only an `rt-core` fix, and `M2.9` may take it
  without review.
