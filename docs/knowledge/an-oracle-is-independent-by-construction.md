---
slug: an-oracle-is-independent-by-construction
answers:
  - "How do I write an expected-output oracle that is actually independent of the thing it judges?"
  - "The gate says the assertion must predate the generator — how do I make that checkable?"
  - "Why is a test that only ever passes not evidence that anything was checked?"
  - "How do I stop someone quietly adjusting an expected result to make a test pass?"
type: knowledge
date: 2026-09-13
---

# An oracle is independent by construction, or it is not independent

## The question

A gate asks for an *independent* expected-output assertion — one written before, and separately
from, the code that produces the output. Intent is not enough: how do you make the independence
a property a later reader can check, rather than a claim they have to believe?

## The answer

Three mechanisms, none of which rely on anybody remembering:

1. **Order it in git.** Land the fixtures and the expectations in one leaf, and the generator in
   a later one. The independence claim then reduces to `git log`, which nobody can rewrite
   quietly. Asserting "this was written first" in a comment proves nothing; a commit does.
2. **Put it where it cannot be called.** In Rust, an integration test under `tests/` cannot be
   linked into a library. An oracle that lives there *cannot* become the generator's own
   implementation later, however convenient that shortcut looks under deadline. Choose the
   location for what it forbids, not for where tests usually go.
3. **Freeze literal bytes, and re-derive them separately.** Two legs: an expected file a human
   wrote, and a re-derivation from the input through an implementation of the published
   contract. Either one alone is weak — a frozen file goes stale silently, a derivation alone is
   just the implementation asserting itself. Together, an edit on either side fails.
4. **Where a specification already states the answer, read it from there.** The §13.2 scheduling
   baseline publishes a table of expected response bounds, and the F18 test *parses that table
   out of the roadmap* instead of copying the numbers. The rule it enforces is
   §14.1's: "implementation changes cannot silently … **adjust expected oracle results**". A test
   holding its own copy makes that a one-line edit that looks like a fix; reading the
   specification makes the expectation and the requirement the same object, so changing the
   answer means changing a requirement, in a diff a reviewer recognises as one. ⚠️ Parse
   **strictly** — assert the row count — or a table that quietly shrinks leaves a green test
   checking less than it did.

5. **Isolate the derivation, not just the artifact — and treat disagreement as the finding.**
   For a reference *model* the strongest available form is to have it derived by someone who has
   not seen the implementation at all, from the specification alone. Then ⭐ **agreement is the
   weak result and disagreement is the strong one**: two models that disagree cannot have been
   copied from each other, so every divergence is simultaneously proof of independence and a real
   defect — in one of them, or in the specification that failed to decide it. Measured here: two
   models of one runtime agreed over 16 000 randomised events and disagreed in five places, and
   all five turned out to be genuine gaps in the written contract that a single author had
   resolved silently and invisibly.
   ⛔ Assert the disagreements on **both** sides, as a ratchet. Otherwise someone "fixes" one
   model, the list shrinks, and a specification gap closes without anyone deciding anything.

Then **disclose what is still shared**. Ours shares the reader with the toolchain, and says so
in its own header, because a shared parser bug would mislead both sides identically. A named
shared dependency is a reviewable fact; an unnamed one is a false independence claim.

## Why

The failure this prevents is not laziness, it is ordering. Once a generator exists, the cheapest
way to produce an "expected output" is to run it and paste the result — and the resulting test
is a transcript. It passes forever, including on every future version that is wrong in the same
way, because it never encoded an expectation about behavior at all. The roadmap gate this note
comes from says exactly that: *"embedding a prebuilt output without consuming the functional
input does not satisfy the gate."*

## How to apply

- Write the expectation while the implementation still does not exist. That window closes, and
  it does not reopen.
- **Measure the red arm before claiming the oracle works.** Ours was run in both directions:
  mutate the frozen expectation → 2 tests fail; mutate the input description instead → 3 tests
  fail. A check that has only ever been seen green has not been shown to check anything
  ([[doctrine-seams-vs-forking-a-check]] is the same instinct applied to a gate).
- When a gate cannot be asserted yet, leave a **tripwire, not a skip**: pin the current
  behavior so the test fails the moment the real thing lands. A skipped check that reports green
  is the failure mode the tiered-verification rule ("a required tool skipped or unavailable is
  reported as such, not a passed check") exists to prevent.
- Deriving the expectation may need semantics nobody has written down. Record them as a decision
  in the same commit rather than inferring them — that is how the `priority` comparison
  direction came to be recorded.
