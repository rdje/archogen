# Five runtime-semantics questions the contract does not decide

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `resolved` 2026-09-13 by leaf `M2.9`, under authority delegated by the director; **amended
  `2026-10-01`** after the independent review of `ROADMAP.md` §3.1.1 — see *Review and amendment* below
- **Owner / source:** found by leaf `M2.2`, by two independent routes: an independently derived
  reference model flagged them while being written, and a differential comparison hit the first
  of them on its opening sequence.

> **Where the contract lives.** The resolution below was written into `ROADMAP.md` §3.1.1 on 2026-09-13. On
> 2026-10-01 it moved, verbatim and with its rule numbers, to the profile's **fault contract**,
> `docs/profiles/rt-static-up-v1-faults.md` (leaf `M2.19`; *Where the contract lives* below). "§3.1.1" in this
> record means that text wherever it now is.

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
condition ground 1 of rule 3 calls fatal, reached without a fault. The grounds are in the findings record, checked
against their sources on `2026-10-01`: FreeRTOS's pended ticks ([`tasks.c`](../book/src/ledger.md#freertos-kernel)),
AUTOSAR OS re-enabling interrupts for a task that returns with them disabled, as an error it recovers from
([SWS_Os_00239](../book/src/ledger.md#autosar-os)), and the accepted composition's `CS_i`, the masked run that ends
at completion. OSEK, which the record also cited, drops an activation beyond a task's limit and reports it
([OS 2.2.3](../book/src/ledger.md#osek-os)); the record is corrected.

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

**Carried into both models (`2026-10-01`, step 6c).** `rt-core` and the reference, re-derived again by a context
that read neither `rt-core` nor the book nor the trees, carry findings 8, 9, 10 and 16: a fault raised in kernel
code is no task's and names the task it interrupted; a halt keeps one record — the first fatal fault, its
attribution, whom it interrupted, whether rule 3 escalated it — and every event after it changes nothing; the mask
bound, an `unmask` with nothing to close and a job started inside a region are assertion failures. The comparison
now generates kernel-context faults, unbalanced unmasks and the region holder's escalated overrun, and compares
the two halts' records: 400 sequences, 0 diverged. A mutation run of each rule on `rt-core`: 16 of 17 red; the
seventeenth, a later fault overwriting the kept one inside `raise`, is unreachable, because every entry point
answers a halted runtime before reaching it.

The reference's author recorded what the text still leaves to a reader, for step 6d's round: rule 3's ground 2 has
no case left that ground 1 does not cover, since rule 1a says another task's overrun cannot be raised inside a
region; rule 1a does not say how the holder's own overrun is raised there; a `mask` while idle; the order across
tasks at one delivery; which task a fault in a transition interrupts; that records name a stable logical ID while
boot does not check names are unique (the checker does: `schema-duplicate-name`); and that §3.1.1's header claims
the review's answers stated what both models already did, which was not true of the reference — it had a third,
halting overrun policy, now removed, and escalated another task's masked overrun, which it now refuses as
impossible while `rt-core` still escalates it.

## The rewritten §3.1.1 reviewed again (`2026-10-01`, step 6d)

A second new context, reading the text alone, judged the first round's 25 answers — 17 answered, 8 partly, none
missed — and returned 28 findings more, numbered on from 26: 11 defects, 9 drafting, 8 nits. **Verdict: still not
fit to rely on from the text alone, and not only for drafting.** It read the text before `M2.14` landed, so #32 was
answered when it arrived. Triaged:

| # | Kind | Finding | Answer | Step |
| --- | --- | --- | --- | --- |
| 26, 27, 29, 40 | defects, drafting | rule 1a's monitor: masked with the region, the holder's own overrun cannot be raised there; another task's is "cannot" against ground 2's "whichever task", and the models disagree; a stale monitor overrun; a monitor is an undeclared source | **decided, for the director's review: `rt-static-up-v1` has no execution-budget monitor.** An overrun is detected by rule 1 alone, as rule 6 already says of deadlines; a port that raises one another way is outside the profile, and the runtime's entry for one is a later profile's. So no containable fault is raised inside a masked region here, and rule 3 decides nothing observable in this profile — it is the answer a later profile starts from. The models' masked disagreement (#27) falls outside the profile | 6e |
| 28 | defect | abandonment deferred to a primitive's return holds a region after `mask`, and contradicts `Fault`'s "never runs again" | accepted, the reviewer's text: abandoned only where no region is held, at a primitive's entry if it has not yet changed the runtime's state; under `Fault` no further instruction of the task's own code runs | 6e |
| 30 | defect | rule 4's deliver-then-decide is the opposite of the target's decide-then-deliver | accepted: both orders reach the same state and are the port's; the trace is `M2.15`'s | 6e |
| 31 | defect | "never lost" cannot hold for an external source whose gateway drops edges | accepted: it holds for arrivals the platform delivers as distinct requests; a source that cannot state it counts arrivals says a second one in a masked interval can be lost | 6e |
| 32 | defect | the clause's domain and default undecided | answered by `M2.14` (`ARCHOGEN-M2-0270`) | done |
| 33 | defect | "unexpected trap" undefined; an empty claim | accepted: any trap other than the timer, a declared source's interrupt whose claim finds a request, and the runtime API's entry; an empty claim is one — the composition's `no-empty-claim` makes it a violated port obligation | 6e |
| 34 | defect (minor) | the interrupted task of a fault in a transition; rule 7 omits the interrupted task | accepted: a transition has already decided its incoming task, which is the one interrupted; rule 7 keeps it | 6e |
| 35 | defect (minor) | `mask` or `unmask` with no job running | accepted: only a job changes the depth, so either with no job running is an assertion failure. Both models accept it today | 6e, 6f |
| 36 | defect | the header's provenance claim is wrong on three counts | accepted: the header lists every change that moved an implementation | 6e |
| 37 | drafting | which release the latch keeps; how many overruns a task still owing a job raises | accepted: the latch keeps the most recent release and a mark that an earlier one came; a task still owing a job at delivery overruns once for each latched arrival judged, as both models do | 6e |
| 38 | drafting | the version sentence is not §15's criterion | accepted: no version of `rt-static-up-v1` is released or locked — the repository has no release tag | 6e |
| 39 | drafting | rule 6 and F26 overclaim when a miss is observed | accepted | 6e |
| 41, 42, 45 | drafting | the cause of an overrun; the order across tasks at one delivery; leftover terms | accepted | 6e |
| 43 | drafting | the still-open list is incomplete | accepted: re-listed after these answers | 6e |
| 44, 53 | drafting, nit | the stable logical ID's form and uniqueness; a rank's upper range | accepted: the ID is the eADL task name, unique by the checker (`schema-duplicate-name`); a rank is any `N ≥ 1` the language's integer holds — and measured: `archogen check` accepts `(priority 70000)`, which `rt-core`'s lowering, taking 16-bit ranks, cannot represent | 6e, `M2.18` |
| 46–50, 52 | nits | rule 2 and rule 1a; the start-in-region assertion's attribution; rule 7's bound and a fault in the handler; §13.4's general ordering; the trap row's token; two stale lines | accepted | 6e |
| 51 | nit | who refuses an empty task set | accepted, and measured: `archogen check` accepts `(defsystem s)` with no task, rc=0; only the runtime's boot refuses it | `M2.17` |

## What changed an implementation (`2026-10-01`)

`ROADMAP.md` §3.1.1's 2026-10-01 amendments point here. *R1* is the review in *The amended §3.1.1 reviewed*, *R2*
the one in *The rewritten §3.1.1 reviewed again*. Two amendments correct the 2026-09-13 text (findings §6 (a),
(b)). Of the answers to R1 and R2, most state what both implementations already did; these changed one:

| Change | Rule | Changed |
| --- | --- | --- |
| a fault in a service is no task's, and the task it interrupted is recorded as interrupted | 2 | `rt-core` |
| a halted runtime changes nothing afterwards, and keeps the first fault's record | 7 | `rt-core` |
| the mask bound and an unbalanced `unmask` are assertion failures, not refusals | the second decision | both |
| a job started inside a region, and a `mask` or `unmask` with no job running, are assertion failures | Terms | both (step 6f) |
| exactly two overrun policies | 5 | the reference model, which had a third that halted |
| an overrun raised without a release is outside the profile | 1a | neither's behaviour inside the profile; both keep an entry for one, for a later profile. The reference first refused another task's inside a region, then — once rule 1a no longer said it "cannot be raised" — escalated it by rule 3's ground 2, as `rt-core` does (step 6f) |

⭐ **§5's lesson, a third time.** The reference model was derived from the text, and the two models agree, but six
of the decisions they share were written only here and in findings §6. Agreement between models shows the text is
implementable; only a reader of the text alone shows it is *sufficient*.

## The fault contract reviewed a third time (`2026-10-01`, R3)

A third new context, reading the text alone (it disclosed identifiers quoted inside the files it read, and that the
section changed under it with `M2.17`). On R2's answers: 18 answered, 8 partly, 1 not — #39, rule 6's "always",
which had grown worse. **Verdict: not yet fit to rely on from the text alone; 12 defects remain**, each with a
replacement text, several found by reading the contract against the composition record. Their answers are step 6h's.

| # | Kind | Finding |
| --- | --- | --- |
| 54 | defect | a `mask` or `unmask` by a service, the trap path or a transition while a job is preempted beneath it: forbidden in the headline, undetectable by "no job running" |
| 55 | defect | the composition lets initialisation call the masking primitives; the contract halts any `mask` with no job running |
| 56 | defect | the start-in-region assertion's context is "that decision", not one of rule 2's; on the target the decision is in the completion path, the task's |
| 57 | defect (minor) | a fault in a transition "while it is being decided": the outgoing task or none, unchosen |
| 58 | defect (minor) | a primitive entered by a trap: the task's, or the trap path's |
| 59 | defect | an unexpected trap that is an interrupt has no faulting instruction, so "synchronous … attributed to it" splits readers |
| 60 | defect | the unexpected-trap definition swallows a guard's access fault and a deliberate assertion trap |
| 61 | defect | "a claim that finds no request" catches the empty claim that ends a PLIC claim loop |
| 62 | defect | when a job completes is undefined; a release during the completion path is fresh or an overrun |
| 63 | defect | rule 6's "always" contradicts rule 4 and a release's delivery latency (R2 #39, worse) |
| 64 | defect (minor) | the latch coalesces while rule 1 says timer releases never do; "the next is an overrun" is false under `Fault` |
| 65 | defect (minor) | rule 7 records a task by index with the plan; the priority record forbids the index |
| 66–73 | drafting | rule 5's "otherwise at its return" and "rolls back"; the header against §14.1 and §15; rule 7's guard and context fields; rule 1's undefined "masked interval"; "masked run"; F26's row; "any other policy"; rule 4's "same state" |
| 74–80 | nits | the profile page's pointer; the elaborated task name; "triggering release", "ready" not "dispatched"; `M2.18` going stale; findings §6 (a) and (d) not marked superseded; the empty set's boot refusal and check status; rule 7's escalated field and a fault before the record is complete |

**Answered `2026-10-01` (step 6h), in the fault contract's new home.** Every finding is accepted, most in the
reviewer's own replacement text, and none moves either model: they state what the port must do where the hosted
models cannot see — 54's detection from the port's own state, 58's trap-entered primitive, 59–61's trap
classification and claim loop, 62's completion instant — or settle what both models already did: 56 and 57's
transition rule is what `rt-core` records, the decision's assertion naming the outgoing task while its job is owed,
and 64's discard under `Fault` is what both models' states show. **55 is decided the other way from the reviewer's
first option:** initialisation calls no masking primitive, since it runs with interrupts disabled and has nothing to
mask, and the composition's `leaves-interrupt-hardware-alone` — writing the hart's interrupt state only through
those primitives — then means not at all for it; neither record changes meaning. 63 rewrites rule 6 and §13.1 F26's
row, whose narrowing is marked for the director's review; 65 aligns rule 7 with the priority record (a plan
supplies the ID; an index names no task); 69 leaves the counting fact to the runtime records (`M2.7.4`); 74 and 77
were answered by `M2.19` and `M2.18`; 78 marks findings §6 (a) and (d) superseded in detail. The next round reads the
composition record in full beside the contract, as R3 advised.

## The fault contract reviewed a fourth time (`2026-10-01`, R4)

A fourth new context read the contract beside the composition record in full, as R3 advised. On R3's answers: 20
answered, 7 partly, none missed. **Verdict: 9 defects remain — and three of the four substantive ones are in text
written to answer R3** (#81, #83, #84): each answer round has seeded defects of its own, mostly where the contract
prescribed a port's *mechanism*. So the answers below change approach: **the contract states what a port must
achieve, and leaves how to the port's catalog record (`M2.12`), reviewed there with the port's design**; and each
answer text is to be checked against both models before it lands (step 6k). The reference model's own re-derivation
(step 6i) reported #83 independently.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| 81 | defect | rule 4 attributes a delivery fault backwards: on the target delivery is a trap and service after the transition unmasks | accepted: the trap path's or its service's on the target, the completion path's in a hosted model |
| 82 | defect | the port's detection recipe halts every `mask` on a port that enters the API by a trap, and misses idle | the recipe removed: a port must detect a non-job call and raise it; how is its record's (`M2.12`) |
| 83 | defect | rule 1's `Fault` wording discards the overrun that should take a task out after a fresh first arrival | accepted, the reviewer's text: each later arrival judged against the state the earlier ones left |
| 84 | defect | the dispatch-in-region assertion's interrupted task reads incoming or outgoing; "decided" undefined | accepted: raised while the switch is being decided, interrupting the outgoing task if its job is owed; a switch is decided when the incoming context is recorded as current — what `rt-core` records |
| 85 | defect (minor) | a one-instruction window between a job's last instruction and its completion path | a requirement: a release observed after the last instruction finds no job owed; how is the port's record's (`M2.12`) |
| 86 | defect (minor) | a hosted latch judges the mark as one, the target each due release | accepted: stated, with the count's difference left to `M2.15`'s events; the states agree |
| 87 | defect (minor) | the completion path against the transition when the decision sits inside it | accepted: for attribution the completion path ends where the decision begins |
| 88 | defect (minor) | "its job is still owed" after a `SkipLateJob` renewal | accepted, the reviewer's text |
| 89 | defect (minor) | a later claim returning an undeclared source; an API entry naming no primitive; the completion path entered by a non-job | accepted, the reviewer's text |
| 80 | R3, partly | a fault while the first record is being written | accepted: a completeness mark written last |
| 90, 91 | drafting | initialisation's rule credited to the composition; the composition's role list reads as letting kernel contexts call `mask` | accepted: it is this contract's narrowing, listed in the header; the composition record gains the matching sentence |
| 92–97 | drafting | rule 6's "which needs `D = T`"; rule 5's "leaves the depth at zero"; the table's attribution column; §3.1's kernel critical sections; a timer trap with no release due; the counting fact's owner | accepted; a timer trap with no release due is an unexpected trap, like an empty claim, since the composition's `raised-only-when-due` makes it a port's broken obligation; the counting fact belongs to the source's catalog record, written under `M2.7.4` |
| 98–105 | nits | vestigial "if there is one"; "running" for "owed"; `CS^app`; "no workload"; "not lost"; rule 7's "latched"; other guarded stacks; three stale lines in other records | accepted |

## Where the contract lives (`2026-10-01`)

Answering R3 (`M2.9`) needed about 20 lines more than `ROADMAP.md`'s 1 100-line ceiling allowed, which only the
director, the roadmap's owner, raises. The question put: "How should it grow?", with "Raise to 1200", "Move §3.1.1 to
docs/specs/" and "Keep compacting". The first answer was "Raise to 1200". The director then asked why the roadmap
had a ceiling at all, and why it needed modifying; the answer given was that it doubles as the normative contract,
that §3.1.1 had become a 130-line specification under active review, and that such a contract belongs in the
profile's specification. Asked again — "Move it out" or "Keep the 1200 raise" — **the director chose "Move it out"**.

So the contract is `docs/profiles/rt-static-up-v1-faults.md`, normative as part of `rt-static-up-v1` and linked from
its published form; `ROADMAP.md` §3.1.1 keeps the requirement, the four-row table and a pointer, and dropped from
1 099 lines to 968. The text moved verbatim, its rule numbers unchanged. Every live citation follows it — `rt-core`,
the comparison, the checker, the book, the semantics and the composition record name "the fault contract" — while
records of what was done when it lived in the roadmap keep saying "§3.1.1". The independent model's citations were
rewritten by its own context, without a change of behaviour.

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
inference, which §15 makes a versioned language change). *(Superseded `2026-10-01`: "subtract one" holds only for
ranks exactly `1, 2, …, n`; the relation is `runtime index = |hp(i)|`, the priority record's item 3.)*

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
