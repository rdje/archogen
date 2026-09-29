---
slug: an-oracle-is-independent-by-construction
answers:
  - "How do I write an expected-output oracle that is actually independent of the thing it judges?"
  - "The gate says the assertion must predate the generator — how do I make that checkable?"
  - "Why is a test that only ever passes not evidence that anything was checked?"
  - "How do I stop someone quietly adjusting an expected result to make a test pass?"
  - "My digest pipeline and an independent re-computation disagree — which one is wrong?"
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

## When the two sides are byte streams, suspect the cross-check first

`M1.13.4.5` (`docs/tasks/M1.md`) built a baseline of digests over constructs extracted from documents,
with the extraction in Rust and the hashing in shell — two processes that have to agree on bytes through
a NUL-framed pipe. Two independent cross-checks were run over the result, and the first **disagreed**:

- the grammar's EBNF fence, re-extracted in another language and hashed there → an identical digest;
- one machine-read table, re-extracted the same way → a **different** digest.

The instrument was right and the cross-check was wrong: the checker kept consuming blank lines after the
table ended, so it hashed trailing newlines the instrument's rule stops before. The rule — contiguous
`|` lines after the marker, blanks allowed only *before* the header — was in the code and not in the
cross-check, and a cross-check that reimplements a rule from memory is not independent. It is a second
guess with a confident output format.

- **State the extraction rule where a cross-check can read it**, then write the check from the statement
  rather than from the code. Re-run from the statement, both constructs agreed; and a third check — a
  suite file's canonical form re-printed by a *different* printer (`diagnose`) and hashed — agreed too,
  which is the one that mattered, because it crossed an implementation boundary rather than a language
  one.
- ⛔ **Read the output; do not trust the pipeline.** The same slice shipped a shell bug the digests hid:
  `records+="$(printf '%s  %s\n' …)"` — command substitution strips the trailing newline, so all 70
  records reached `sort` as one line and `sort -k2` on one line returns one line. Every digest was
  correct and the file was unusable. Nothing but looking at the emitted bytes would have shown it.
- A framing byte has to be one the payload provably cannot contain. Here the payload is canonical text,
  and §3 rule 3 of the language reference forbids a control character in it — which is what makes NUL a
  safe frame and a documented reason rather than a lucky choice.
