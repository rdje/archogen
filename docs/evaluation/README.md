# The sealed evaluation set

`ROADMAP.md` §12 M0 requires a separate small set of **previously unused** evaluation cases,
frozen before reuse is measured; §16 requires at least three previously unused configurations
to be evaluated after a catalog freeze. This directory holds them, sealed.

Five cases are sealed, listed with their SHA-256 digests in `frozen/MANIFEST.txt`. While the set
is sealed, their text is in **no file of the working tree**: each case is the blob the
manifest's `# sealed-in:` commit holds, and the manifest's digest is the commitment to it.

> **Do not read a case before leaf `M6.5`.** Not because the content is secret — this
> repository is public — but because the measurement is worthless once the engine has been
> shaped, even unconsciously, by what those cases need. Never search the sealing commit's
> history (`git show`, `git log -p`, a pickaxe search) for them.

## What is actually at stake

The program's central hypothesis (§16) is that each additional supported system costs less
than the last. That is a claim about *unseen* systems. Measured on cases that were in view
while the catalog was designed, it measures how well the catalog was fitted to them — which is
a tautology dressed as evidence.

The contamination does not have to be deliberate. It is enough that a case's requirements were
in front of someone choosing what the engine should support.

## Why the text is out of the tree

Until `2026-10-10` the cases sat in `frozen/` as files, guarded against edits and against being
named elsewhere, but not against being **read**: the only barrier was the line above, which no
agent's bootstrap reaches. The cases are written in the vocabulary the engine work searches
every day, so an ordinary search for other work can land in one — and one did (*Exposures*,
below). Excluding search terms cannot work, and excluding the folder holds only for tools that
honour an exclusion. So the text left the tree (leaf `PROGRAM.76`): no search of it, under any
term, by any tool, reaches a case; reading one takes a deliberate `git show` of the sealing
commit.

## The seal is mechanical

`scripts/check_frozen_evaluation.sh` runs in the pre-commit hook and in CI, through the
project doctrine slot. It enforces five properties:

| Property | What it refuses |
| --- | --- |
| **Integrity** | a digest that the case's blob in the sealing commit does not hash to, or, once unsealed, a restored case that differs from it — the set cannot be quietly rewritten to match what the engine turned out to do |
| **Custody** | while sealed, a case in the working tree or in the index |
| **Completeness** | a file in `frozen/` the manifest does not list, hidden ones included; once unsealed, a listed case missing |
| **Non-contamination** | any tracked file **outside** `frozen/` that names a sealed case — a case written about in a task tree, decision record or design note is no longer unseen |
| **Exposure** | an `# exposed:` line naming no listed case |

It never prints a case's text — a blob is hashed through a pipe — and its `--self-test` arms,
on synthetic cases in a scratch repository, check that of every run. The RED arms are recorded
in leaves `M0.6`, `PROGRAM.18.1` and `PROGRAM.76`.

⚠️ **The honest limit**, stated rather than hidden: the text stays in the repository's history,
which is published and not rewritten — `git show`, `git log -p` or a pickaxe search over it
reaches a case. The check proves the tree holds none, that each commitment still resolves, and
that no case is named where the repository can see; it cannot prove nobody read one. A human
who reads one and stays quiet defeats it, so an exposure is recorded, never kept quiet.

## Exposures

A case read before unsealing is counted apart from the unseen ones at the measurement. The
manifest's `# exposed:` line names it — `frozen/` is the one place a case may be named — and
this table says how it happened, without naming or describing it.

| Date | How | Extent | Answer |
| --- | --- | --- | --- |
| `2026-10-10` | a delegated read-only survey for leaf `M3.3`, told to sweep the whole tree, searched a public identifier that one case's text also holds, and opened the case; a search in the working session printed one of its lines | one case: a few of its lines read by the survey and summed up to the session in one sentence; nothing of it written anywhere | the text taken out of the tree (`PROGRAM.76`); the case reported apart at `M6.5`, four unseen cases remaining against §16's three |

## Unsealing

At leaf `M6.5`, set `seal: unsealed` in `frozen/MANIFEST.txt` and add `unsealed-on:` and
`unsealed-by:` lines, then run `bash scripts/check_frozen_evaluation.sh --restore`: it writes
each case back from the sealing commit, verifying its digest, and refuses to run while the set
is sealed. Integrity and completeness keep running — the cases must still be the ones that were
sealed — and custody and non-contamination lift, because from that point they are supposed to
be read and discussed.

§16 then requires the measurement to record, per case: description size, generated and runtime
size, new engine implementation, new hardware facts, reuse by component class, review effort,
debugging effort, time to a justified result, target resource costs, and rejection causes —
with a **predeclared adaptation budget**, and with marginal cost reported separately from
accumulated fixed cost. A case the manifest marks `exposed` is reported apart from the unseen
ones.

## What this is not

It is not a test suite and not an acceptance gate. The mandatory fixtures F01–F30 are in
`ROADMAP.md` §13 and are owned by the milestone trees. This set exists for one measurement,
once, and its value is destroyed by early use.
