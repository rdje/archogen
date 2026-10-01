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
| 3 | priority rank `0` | **refused**; rank is `N ≥ 1`, and the runtime index is stated — `rank − 1` then, `\|hp(i)\|` since `2026-10-01` (the priority record's item 3) | `decision_priority-comparison-direction.md`; `rt-core` |
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

## The amended §3.1.1 reviewed (`2026-10-01`)

`M2.9`'s plan step 6: a new context, instructed not to read `crates/`, the book or the task trees, reviewed
`ROADMAP.md` §3.1.1 as amended and item 3 of [[decision_priority-comparison-direction]]. It disclosed what it saw
outside its brief — some lines of `docs/tasks/` through a grep whose filter missed, and the Knowledge Map's summary
of `fault.rs` — and none of its findings rests on them. **Its verdict: not yet fit to be relied on from the text
alone**, because several decisions both models share live in this record and findings §6 rather than in §3.1.1, and
some are in neither. Twenty-five findings: thirteen defects, eight drafting, four nits.

| # | Kind | Finding | Answer | Step |
| --- | --- | --- | --- | --- |
| 1 | defect | rule 3's "while interrupts are masked", read literally, makes every overrun found at arrival fatal, since a release service runs with the hardware masked | accepted: rule 3 concerns the runtime's masked region, the mask depth above zero, defined; a service's own interrupt disable is not one — as both models implement | 6b |
| 2 | defect | neither policy is defined | accepted: stated as both models implement — `SkipLateJob` abandons the owed job at once, wherever it stands; `Fault` takes the task out of the schedule for good, its later releases discarded, every other task continuing | 6b |
| 3 | defect | the policies' domain, their eADL spelling and an absent clause are undecided | accepted, and measured: `archogen check` accepts `(on-overrun banana)` and a task with no clause. The domain is `fault` and `skip-late-job`; an absent clause means `fault`, the profile's default; any other symbol is refused | `M2.14` |
| 4 | defect | rule 2 says the overrunning task is never the running one; the table, that it need not be | accepted: the overrunning task, whichever context holds the processor | 6b |
| 5 | defect | an overrun raised without a release is not in the text | accepted: a rule 1a, as both models implement it. Raised inside a masked region it escalates: a monitor is an interrupt the port masks with the region, so only the mask holder's own check can raise one there, and ground 1 holds | 6b |
| 6 | defect | a doubled latch at a completion, three arrivals, and the surviving job's instant | accepted: a task's latched arrivals are judged one at a time in arrival order against its state then, so a completing task's own pair is a fresh job and an overrun (both models); later arrivals are the same overrun; under `SkipLateJob` the surviving job is the last arrival's | 6b |
| 7 | defect | §13.1 F26's "missed deadline" has no defined behaviour | decided, **for the director's review**: `rt-static-up-v1`'s runtime detects no deadline miss. Rule 1's overrun is its only timing fault, F26's miss is exercised as one with `D = T`, and a miss with `D < T` is the timing analysis's to exclude and a trace's to observe | 6b, F26's row |
| 8 | defect | attribution outside task context | accepted: the executing context; a service, the trap path, a transition or idle is no task, and the evidence names the interrupted task as interrupted. `rt-core` attributes a fault in a service to the interrupted task today | 6b, 6c |
| 9 | defect | escalation's terminal state and its evidence are unspecified | accepted: halt; nothing runs, is delivered or is latched afterwards; the first fault is kept, with its kind, class, attribution and whether rule 3 escalated it; the task table afterwards is not evidence. `rt-core` still edits its table on a release after a halt | 6b, 6c |
| 10 | defect | refusing a `mask` past the bound leaves the matching `unmask` to close the section early; an unbalanced `unmask` is open | accepted: both are assertion failures, which halt; the bound is the runtime record's to declare | 6b, 6c |
| 11 | defect | containment can abandon a job inside a primitive's unmasked part | accepted: a job is abandoned only in its own code or at a primitive's entry or return, so a policy applied inside a primitive takes effect at its return — a port obligation the runtime records state | 6b, `M2.7.4` |
| 12 | defect | the section's header says nothing changed, and the 2026-10-01 amendment changed behaviour | accepted: marked as a correction | 6b |
| 13 | defect (minor) | the observation events an overrun, a skip, a delivery or an escalation produce are undefined | accepted: a leaf of its own, beside §6.3's | `M2.15` |
| 14 | drafting | rule 1 says only a *second* in-region release is observed at delivery | accepted: every in-region release | 6b |
| 15 | drafting | six names for two concepts; "latch" undefined | accepted: one definitions paragraph | 6b |
| 16 | drafting | "a masked region it opened" | accepted: every job starts at depth zero and nothing else changes it, so every open section at a completion is the job's | 6b |
| 17 | drafting | "Unexpected trap → Deliberate fatal trap" mixes cause and response | accepted | 6b |
| 18 | drafting | the composition record's `releases-never-latched` row says `complete` leaves the depth | accepted: one sentence corrected | 6b |
| 19 | drafting | the priority record's item 2 still gives `rank − 1`; so do gap 3's row here and findings §5 | accepted | 6b |
| 20 | drafting | (a) the index is unstable across builds; (b) duplicate ranks refused nowhere; (c) item 3's review status unmarked | (a) and (c) accepted. (b) **rejected**: duplicates are refused twice, by the checker (`unsupported-profile`) and by the lowering (`BootError::DuplicateRank`), which the reviewer was told not to read; the record will say so | 6b |
| 21 | drafting | §14.1: the rulings were in force before an independent review of the text | agreed: this is that review, and a second round follows the answers | 6d |
| 22 | nit | a coincident completion and release | accepted: §13.4's order lifted into rule 1 | 6b |
| 23 | nit | "each settles a question" overclaims | accepted | 6b |
| 24 | nit | rule 3's grounds read as both needed in every case | accepted | 6b |
| 25 | nit | timing claims after a contained overrun | accepted: a run's claims end at its first contained overrun | 6b |

⭐ **§5's lesson, a third time.** The reference model was derived from the text, and the two models agree, but six
of the decisions they share were written only here and in findings §6. Agreement between models shows the text is
implementable; only a reader of the text alone shows it is *sufficient*.

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
