---
slug: prove-the-artifact-was-regenerated-not-just-present
answers:
  - "I re-ran the build at a new pin and it passed — how do I know it did not reuse the old artifacts?"
  - "A generator is idempotent. What does that hide when the inputs moved?"
  - "How do I prove a re-measurement measured the revision I think it measured?"
  - "My vendored checkout arrived with build output already in it — is that a problem?"
type: knowledge
date: 2026-09-27
---

# Prove the artifact was regenerated, not merely present

## The question

You move a vendored dependency to a new revision and re-run its preparation step to re-measure a
report. It exits `0`. Did it build the new revision's artifacts — or did it find the old ones already
there and do nothing?

## The answer

**Hash the artifact before and after, and require the digest to move.** An existence check is not a
freshness check, and many generators are deliberately idempotent *on existence*: if the output is
there, they skip. A green exit from such a step says only "an artifact exists".

Then look for the regeneration instruction in the dependency's **published** contract. It usually
exists, because the maintainers hit the same trap, and it usually says: remove the generated
directory, and do not trust a build directory from before the bump.

## Why

Measured on this project, `2026-09-27`. A vendored checkout arrived carrying generated parser
sources dated a week earlier, produced at the *previous* revision of the generator's own dependency.
The published downstream contract said so explicitly:

> `make bootstrap` is idempotent on *existence*, so after bumping the pin you must force a
> regeneration — remove the generated directory first — or you will keep building against the
> previous pin's parser.

Following it produced the evidence that makes the re-measurement worth anything:

```text
removing stale ............... generated (dated 2026-09-20)
removing stale ............... rust/target (1.7G, dated 2026-09-20)
files in generated/ .......... 12
parser sources ............... regenerated: 50eec63c9ba79b16 -> 196db2eefed767ff
```

Two digests, one before and one after. Had the run reused the stale parser, the two would have been
equal — and every measurement built on top of it (a document route, a token-kind route, a consumer's
results) would have described the *previous* revision while claiming the new one. Nothing else in
the output would have looked wrong: the exit status was `0` either way.

⭐ **The reuse arm is what makes the distinction visible.** A second run over the prepared checkout
prints "already generated — nothing to bootstrap", exits `0`, and leaves the digest **unchanged**.
That is correct behaviour, and it is also exactly what a stale first run looks like from the outside.
Only the digest separates "idempotent because it is prepared" from "idempotent because it is old".

⛔ **A related trap in the same run, worth naming because it is easy to ship.** The instrument that
classified the logs counted a seeding message as a symptom of the defect. On a *failing* run that is
right — a seed claim with nothing generated was the original bug. On a *successful* run the seed
really happened, and the classifier printed `symptom PRESENT` over a clean preparation. A symptom
predicate has to be conditioned on the outcome it is read against, and the correction needs its own
test arm or it will regress the first time someone simplifies the condition.

## How to apply

- **Digest, don't inspect.** One aggregate digest over the generated sources is enough to answer
  "did this run produce anything new?", and it costs nothing. Print both sides of it in the evidence.
- **Remove what the contract tells you to remove, and back it up first** if it is the only copy of a
  state you may need to compare against. Copy, verify by count or digest, then delete — never delete
  first.
- **Check the timestamps before you trust the run.** A generated directory dated before the pin move
  is the whole finding, visible in one `ls`.
- **Run the reuse arm deliberately.** It documents what a no-op looks like, so a later reader cannot
  mistake one for a failure — or a stale artifact for a fresh one.
- **Condition symptom predicates on the run's outcome**, and pin each condition with a test arm built
  from a real log rather than an imagined one.
- Related: [[frozen-reproducers-measure-change-not-repair]] — why a re-measurement needs its own
  instrument, and [[pin-the-vendor-head-and-measure-the-delta]] — choosing the revision to measure.
