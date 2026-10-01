# Annex A: The runtime's rules in detail

This annex is for implementers and reviewers. [The runtime](runtime.md) chapter gives the idea and the rules a
reader needs; here are the mechanics behind them, case by case, and how they were settled. The normative text
is the profile's fault contract, `docs/profiles/rt-static-up-v1-faults.md`; where this annex and the contract
differ, the contract wins, and the difference is a defect of this page.

## Latched releases and completions inside a region

When a second release arrives **while the first is still latched**, both wait, and at delivery — outside the
masked region — the task's latched arrivals are judged in the order they came, against the task's state then:
the first is fresh if the task owes no job and an overrun if it does, and each later one meets the state the
earlier ones left (the fault contract's rule 1). Under `SkipLateJob` each overrun renews the job in turn; under
`Fault` the first overrun takes the task out of the schedule and later arrivals are discarded. `rt-core`, a
hosted model, keeps the latest release and a mark that an earlier one came, so it can report fewer overruns
than a board, which judges every due release; for a timer-released task the state after delivery is the same.
The alternative, judging a release on arrival, would have made an ordinary contained overrun fatal for landing
one instruction inside a critical section rather than one instruction after it.

A job may **complete inside a masked region it opened** (rule 4). Its completion closes the region — the depth
returns to zero with the job — and what was latched is delivered before any task runs its own code again. On a
board the next scheduling decision comes first, in the completion path, and the latched interrupts are taken
when the transition that follows unmasks; `rt-core`, a hosted model, delivers inside `complete`. Both reach the
same schedule. So a task that completes inside its own region and finds its own release latched is released
afresh, not overrun, which is why `complete` returns the deliveries beside the completion.

The nesting bound exists because each alternative fails silently: a counter that wrapped would re-enable
interrupts inside a critical section while reporting success; one that saturated would stop counting; one that
refused — the answer until the contract's second review — would leave its caller's matching `unmask` to close
the section early.

## Faults in detail

**Whose guard.** A stack guard says separately whose guard was hit — a task's, or the interrupt stack's —
because the stack and the culprit can differ.

**Panics.** A panic in the runtime's or the port's own code is an assertion failure — unless its check found a stack's guard reached or one of the unexpected traps the contract names, which it then is. A check in application code is classified by how it ends: a panic is an assertion failure, any other trap an unexpected trap; no port could tell what an application's own check meant. It reaches the port's
one fatal handler, which the port supplies; an application's own handler is refused at build. On a board a few
instructions run between a failed check and the handler's first one, and an interrupt can land there. What may
happen in that window depends on how a port builds its panic path, so the contract leaves it to the port to
state, and to be reviewed with the port's design, rather than legislating for a port that does not exist yet.

**The record a board keeps.** Beside the fault, its task and the task it interrupted, a board's record carries a
mark — empty from boot, begun by its first write, complete by its last — so a record cut short by a fault in the
handler says so, and an empty mark on a halted system says the fault came before the record began.

⛔ **A containable fault raised while interrupts are masked would halt** (rule 3). In this profile the case never
arises — no release is observed inside a masked region, so no overrun is raised there — and the rule stands for a
later profile that could raise one; `rt-core` still escalates an overrun raised through `fault` while masked, the
entry it keeps for such a profile. The grounds: terminating a job that holds the mask leaves the depth above zero
with no owner, so interrupts never return; forcing it to zero re-enables them inside a region whose invariants
were half-restored. §8.1 offers no third option, so the conservative direction wins: a fatal path that was not
strictly needed costs availability, a containment that was not safe costs correctness silently.

**An overrun with no release.** `rt-core` keeps an entry for an overrun raised some other way, through `fault`,
for a later profile with an execution-budget monitor: with no triggering release, it starts nothing, and the task
waits for its next one.

## How the two models were made to agree

The first comparison of `rt-core` with the independently derived `rt-reference` found **five** disagreements, and
every one was a question the roadmap did not answer:

| # | The question the contract left open | Decided |
| --- | --- | --- |
| 1 | does *detecting* an overrun apply its policy, and may a fault attach to a task that is not running? | yes, and yes: an overrun belongs to the task that overran |
| 2 | is an empty task set admissible? | no |
| 3 | what does priority rank `0` mean — and which end of the range is the runtime's index? | rank `0` is refused; a task's index is the number of tasks that outrank it |
| 4 | what happens when a containable fault is raised inside a masked region? | it halts — a rule that, since the contract's later rounds, this profile never reaches |
| 5 | what bounds mask nesting, and what happens at the bound? | a declared bound, and `mask` beyond it is refused — since the contract's second review, an assertion failure |

Deciding them changed the contract — written first as `ROADMAP.md` §3.1.1, now the profile's fault contract —
and the priority decision record, which §14.1 makes a reviewed decision rather than an implementer's. The
reference was then re-derived from the amended text by a context that still had not read `rt-core`, and its two
behaviour questions became the masking rules above. The contract has since been reviewed independently round after
round, each round's answers checked against both models and, from the seventh, read by a separate context before
they land (`docs/decisions/decision_runtime-contract-gaps.md`).

The tests that once asserted each disagreement now assert the agreement, on both sides. They were **rewritten, not
deleted**: §14.1 forbids dropping the only evidence that a gap was ever closed. The randomised comparison also
generates overrunning releases, doubled latches, completions inside a masked region, overruns a monitor raises and
the synchronous faults, checking whom each fault is attributed to, and a mutation run that reverts each ruling in
either model, one at a time, turns the tests red.

Rewriting those tests found more again. Three were places where `rt-core` had departed from text the contract
already had — the trap's attribution, the monitor's overrun and the kept first fault — and were fixed. Two were new
questions: ranks with gaps, `1, 5, 9`, which the implementation refused and the reference accepted, so ranks now
lower by their order; and what a halted runtime leaves in its task table, which the contract leaves to the
implementation, so a test asserts both sides.
