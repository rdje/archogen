---
slug: a-gate-is-only-as-sharp-as-its-fixtures
answers:
  - "My acceptance gate is green — what class of error could it still be blind to?"
  - "How do I know a fixture set is discriminating and not just complete?"
  - "Why did a wrong formula pass every test?"
type: knowledge
date: 2026-09-13
---

# A fixture set can be complete against its specification and blind to a class of error

## The question

An end-to-end gate is green: it generates a system, runs it, and compares the output to an
expectation frozen in advance. What can it still not see?

## The answer

**Anything two different implementations happen to agree on for the inputs you chose.** Find out
which class that is by *mutating the subject*, not by rereading the tests.

Concretely: the S0 gate ran a described system and compared its trace to a frozen observation. It
passed. Replacing the **hyperperiod** (`lcm` of the task periods) with the **longest period**
(`max`) left all twelve of its tests green. Both fixtures were *harmonic* — periods 10 and 30,
then 10 and 20 — so one period divided the other and the two formulas returned the same number
on every input the corpus had.

The fixtures were not wrong. They were complete against the specification they were written from,
and they were picked for readability: harmonic task sets are the easy ones to verify by hand,
which is exactly why they were chosen and exactly why they could not discriminate.

## Why

Two forces push a corpus towards blind spots and neither feels like carelessness:

- **Fixtures are chosen to be checkable by a human.** Round numbers, one period dividing another,
  a trace short enough to read. Those are the same properties that collapse distinct formulas
  onto the same answer.
- **A gate is written against a specification, not against the space of wrong implementations.**
  "The horizon is the hyperperiod" is satisfied by the fixture; "and is not the longest period"
  is a different sentence, and nothing asks for it.

The programme's own acceptance matrix says as much — thirty fixtures described as "a minimum
practical corpus, not a proof of completeness" — but that sentence is easy to nod at and hard to
act on. Mutation testing is what turns it into a measurement.

## How to apply

- **Mutate the subject at every gate you believe in.** The stronger your belief, the more the
  mutation is worth. Swap a formula for a plausible wrong one — `lcm`→`max`, `≤`→`<`, sort key
  reversed — and see whether anything goes red. Verify the mutation applied
  ([[verify-the-mutation-applied]]).
- **Add the discriminating case at the level where it discriminates.** A unit test on the formula
  is cheaper and sharper than a second end-to-end fixture; add the end-to-end one too only when
  the whole path could get it wrong in a way the unit cannot see.
- **Watch the pedigree of what you add afterwards.** An expectation frozen *before* the
  implementation is strong evidence; one added after is two implementations agreeing. Both are
  useful and they are not interchangeable — keep them distinguishable, or a later reader will
  credit the weaker one with the stronger claim
  ([[an-oracle-is-independent-by-construction]]).
- **Record the blind spot where the fixtures live**, not only in a commit message. The next person
  to choose a fixture is the one who needs it.
