---
slug: a-leafs-claims-about-the-repository-are-hypotheses
answers:
  - "My task leaf says the mechanism / arms / figure already exists — do I trust it?"
  - "I planned a slice around something the leaf describes. What if the description is wrong?"
  - "Why did a leaf's own premise turn out to be false, and what do I do when it does?"
  - "How much of a task leaf should I re-measure before implementing it?"
type: knowledge
date: 2026-09-29
---

# A leaf is a plan written before the work, so its claims about the repository are hypotheses

## The question

A task-tree leaf states what exists: "the gate has five RED arms", "this is a grammar change", "the
exclusion is a glob", "no description relies on this rule". The leaf was written by someone who had
just measured *something* — but the leaf is not the measurement, and it is read later, by a session
that did not take it.

## The answer

**Census the premise before designing around it.** A leaf's claims about the repository carry the same
status as anyone else's: hypotheses until measured. `docs/CLAIM_VERIFICATION.md` leg 1 says it for
numbers; this is the same rule applied to the sentence a slice is built on.

Four measured instances in this repository, all found by running the census rather than by reading
carefully (`docs/tasks/M1.md`):

| Leaf said | Measured | Cost of not checking |
| --- | --- | --- |
| `M1.13.3`: a new top-level form "is a grammar change, so `grammar.md`'s `document` production and `conformance.rs`'s probe set both move" | the grammar names **no** construct vocabulary — "the language is extended by `defkind`, not by editing this file" — so neither moved | two edits to normative surfaces that did not need making, in the commit that freezes them |
| `M1.13.4`: the exclusion of the frozen evidence can be enforced "so a feedback file cannot become a conformance case by being copied into a walked root" | `shasum -a 256` put the frozen evidence copy and a suite file at the **same digest**, so content identity is false in one direction on the shipped tree | a check that fires RED on a legitimate file, discovered after it was written |
| `M1.13.4.4`: exclusions "matched by path with a stated, minimal **glob** semantics" | every excluded description sits under one directory, so a path **prefix** is the whole matcher | a glob matcher, its honest-limit prose, and its arms — all unnecessary, and a matcher that quietly understood `*` would have made the manifest's stated rule false |
| `M1.25`: "all **five** existing arms are re-expressed against the new mechanism" | `grep -c '^fn arm_' crates/eadl-model/tests/kinds.rs` → **0**; the seven arms it was probably thinking of belong to a different gate over different surfaces | the slice was sized as a refactor and was actually a build-from-nothing — and the gate it was about had **never been seen to fire** |

## Why

- **A leaf is written to justify and sequence work, not to describe the tree.** Its claims are usually
  the measurements that motivated it, which are real — and stale by the time the leaf is picked up, or
  scoped to a different subject than the sentence implies.
- **The error is invisible from inside the slice.** Implementing against a false premise produces code
  that works; what it produces beside the code is a normative surface edited for no reason, a mechanism
  built for a case that does not exist, or a gate believed to be armed.
- **It compounds at a freeze.** Three of the four instances above were in the leaf sequence that ends by
  freezing the language, where an unnecessary edit to a normative document is exactly what the freeze
  exists to make expensive.

## How to apply

- **Run the census the leaf's premise rests on before writing any design.** One command per claim: does
  the mechanism exist (`grep -c`), does the figure hold (re-derive it), is the file what the leaf says
  (`shasum`, `git ls-files`). It costs a minute and is the whole difference between a plan and a guess.
- ⛔ **Record the correction on the leaf; do not silently implement something different.** The leaf is
  the traceability unit, so a premise that was false belongs in its record with the command that showed
  it — that is what stops the next session re-deriving the same wrong plan, and it is how the four rows
  above became usable evidence instead of four private surprises.
- ⭐ **A false premise is often the more interesting finding.** `M1.25`'s "five existing arms" being zero
  meant the gate it was about had never been observed firing, which is a worse starting point than the
  leaf recorded and changed what the slice had to prove. `M1.13.4`'s identical-bytes measurement is why
  the exclusion is enforced by path *and* by a byte-identity leg that is only usable because a later
  child diverged the pair.
- **Correct the leaf's acceptance text in the same commit**, marking what moved and why, so the
  acceptance the checklist is graded against is the one that was actually achievable.

Related: [[enumerate-the-population-from-the-specification]] — the same discipline applied to a test's
inputs rather than to a plan's premises; [[a-moved-measurement-needs-a-census-of-its-copies]] — what to
do once the premise turns out to be a figure with copies; [[verify-the-mutation-applied]] — the arms a
leaf claims to have are worth counting, and worth running.
