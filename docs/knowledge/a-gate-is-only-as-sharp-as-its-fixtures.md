---
slug: a-gate-is-only-as-sharp-as-its-fixtures
answers:
  - "My acceptance gate is green — what class of error could it still be blind to?"
  - "How do I know a fixture set is discriminating and not just complete?"
  - "Why did a wrong formula pass every test?"
  - "Every RED arm of my gate passes — could the gate still be unable to fail on the real tree?"
  - "My arm expects a refusal and gets one — how do I know it was refused for the reason the arm names?"
  - "My new rule's fixtures went green — would they have gone green without the rule?"
  - "I moved a self-test's scratch directory and it still passes — is it still testing anything?"
  - "My test is named for the property it guards — does it contain a case where the property could fail?"
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
- ⛔ **Build a negative control by re-running the model, never by arithmetic on the answer.** A
  control that "omits the interrupt cost" by subtracting two units from the result agrees with
  the specification by luck: removing a cost changes *when* later events happen, and can change
  how many of them happen at all. Measured here — deleting the resume-switch cost from a
  repeated-preemption fixture does not shorten the response by four, it lets the low-priority job
  finish at exactly the next release instant, and one interfering job disappears entirely (23 →
  14, not 23 → 19). The specification this came from says it outright: *"subtracting a fixed
  number from the original response is not generally valid."* The corollary is that the model
  must be a **function you can re-run**, not a table you edit.
- ⛔ **Point at least one arm at the population the gate actually reads.** `LANGUAGE-FREEZE`'s nine arms
  all ran against a scratch notes directory holding only their own fixture, so none of them ever read the
  real directory's `README.md` — whose form template, `- status: pending | applied` and
  `constructs: … — or: all`, the gate took for a pending note covering every construct. The gate printed
  `OK` with the baseline amended and no note at all (`PROGRAM.27`). A fixture you wrote for the arm cannot
  contain the file the directory ships for another purpose; only the deployed directory does.
- ⛔ **Make a refusing arm name what it refuses.** An exit code and the gate's own word prove the gate
  refused — not that it refused for the arm's reason. Two of the same gate's arms passed from the commit
  that wrote them (`2ae744a`, twelve commits earlier) on a fixture the classifier rejected *whole* as
  unsorted, so "a construct nobody declared is refused" never classified a construct. Once each refusing
  arm had to find its construct id in the output, putting the unsorted fixture back turned both red
  (`PROGRAM.27`, mutation P-F). Seed the arm with the file that actually broke, byte for byte — the
  deployed README, not a README-shaped string.
- ⛔ **Remove the rule and see which fixtures notice.** `M1.29.3` gave an elaborated module tree a name rule
  in two halves — declarations renamed by their instance path, and names resolved where they are written —
  and both F01 trees went green at once. With the second half removed they *stayed* green: their root
  writes names that already equal the global ones, so resolution changed no string. A fixture that passes
  with and without a rule is evidence about something else. Only a module naming **its own sibling** by a
  local name, and a name an export **hides**, discriminate; `app.sibling` does the first in all three name
  positions, and dropping any one position fails it.
- ⛔ **Moving a fixture moves what surrounds it.** `PROGRAM.29` moved every gate's scratch from the system
  temporary directory into `target/`, and two self-tests changed meaning without changing a line.
  `FEEDBACK-SELF-CONTAINED`'s fixture had relied on being *outside* the repository: inside `target/`, which git
  ignores entirely, its file lister asked git and got nothing, so two red arms stopped firing. The gate had gone
  partly vacuous, and it still looked healthy until the self-test counted arms. LS-001's re-measurement had
  relied on there being no Cargo workspace above it. Inside this repository there was one, and an arm failed
  for a reason that had nothing to do with the instrument. A fixture encodes its environment. Where the
  environment is the point, **check it before the arms**, and report "could not run" when it does not hold.
- ⛔ **A test's name is a claim about its cases.** `ordering_is_exact_and_total` compared `1/3`, `1/2` and zero,
  values whose cross products fit easily in `i128`, so the saturating comparison beside it passed. The
  comment on that comparison explained why saturation was safe, and the explanation was false in the one case
  it did not consider: both products saturating. `(2^100+1)/2^30` and `(2^100+3)/2^30` compared `Equal`, and a
  description whose deadline exceeded its period in the 17th decimal was admitted (`M1.34`). Read the name as
  the specification and add the case that sits where the implementation's own reasoning stops. For a
  comparison, that means the values at the limits of the representation.
