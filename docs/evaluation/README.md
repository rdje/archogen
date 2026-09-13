# The sealed evaluation set

`ROADMAP.md` §12 M0 requires a separate small set of **previously unused** evaluation cases,
frozen before reuse is measured; §16 requires at least three previously unused configurations
to be evaluated after a catalog freeze. This directory holds them, sealed.

Five cases are sealed under [`frozen/`](frozen/), listed with their SHA-256 digests in
`frozen/MANIFEST.txt`.

> **Do not open `frozen/` before leaf `M6.5`.** Not because the content is secret — this
> repository is public — but because the measurement is worthless once the engine has been
> shaped, even unconsciously, by what those cases need.

## What is actually at stake

The program's central hypothesis (§16) is that each additional supported system costs less
than the last. That is a claim about *unseen* systems. Measured on cases that were in view
while the catalog was designed, it measures how well the catalog was fitted to them — which is
a tautology dressed as evidence.

The contamination does not have to be deliberate. It is enough that a case's requirements were
in front of someone choosing what the engine should support.

## The seal is mechanical

`scripts/check_frozen_evaluation.sh` runs in the pre-commit hook and in CI, through the
project doctrine slot. It enforces three properties:

| Property | What it refuses |
| --- | --- |
| **Integrity** | a sealed file whose content no longer matches its recorded digest — the set cannot be quietly rewritten to match what the engine turned out to do |
| **Completeness** | a file added to or removed from `frozen/` without being sealed |
| **Non-contamination** | any tracked file **outside** `frozen/` that names a sealed case — a case written about in a task tree, decision record or design note is no longer unseen |

All three have been exercised in both directions. The RED arms are recorded in leaf `M0.6`.

⚠️ **The honest limit**, stated rather than hidden: this cannot prove nobody *read* the cases.
It proves they were not edited, not silently added to or removed from, and were not written
about anywhere the repository can see. A human who reads them and stays quiet defeats it. The
check raises the cost of accidental contamination, which is the common failure; it is not a
defence against a determined author.

## Unsealing

At leaf `M6.5`, set `seal: unsealed` in `frozen/MANIFEST.txt` and add `unsealed-on:` and
`unsealed-by:` lines. Integrity and completeness keep running — the cases must still be the
ones that were sealed — and non-contamination lifts, because from that point they are supposed
to be discussed.

§16 then requires the measurement to record, per case: description size, generated and runtime
size, new engine implementation, new hardware facts, reuse by component class, review effort,
debugging effort, time to a justified result, target resource costs, and rejection causes —
with a **predeclared adaptation budget**, and with marginal cost reported separately from
accumulated fixed cost.

## What this is not

It is not a test suite and not an acceptance gate. The mandatory fixtures F01–F30 are in
`ROADMAP.md` §13 and are owned by the milestone trees. This set exists for one measurement,
once, and its value is destroyed by early use.
