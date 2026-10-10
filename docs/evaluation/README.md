# The sealed evaluation set

`ROADMAP.md` §12 M0 requires a separate small set of **previously unused** evaluation cases,
frozen before reuse is measured; §16 requires at least three previously unused configurations
to be evaluated after a catalog freeze. This directory holds them, sealed.

Five cases are sealed, listed with their SHA-256 digests in `frozen/MANIFEST.txt`. While the set
is sealed, their text is in **no file of the working tree**: each case is the blob the
manifest's `# sealed-in:` commit holds, and the manifest's digest is the commitment to it. Their
paths are marked `-diff` in `.gitattributes`, so in a working tree that carries the mark — at
`6d61f65` or later — a diff, a log patch or a `git grep` of a commit that holds them shows no line
of them.

> **Do not read a case before leaf `M6.5`.** Not because the content is secret — this
> repository is public — but because the measurement is worthless once the engine has been
> shaped, even unconsciously, by what those cases need. Never `git show` or `git blame` a case,
> never diff or grep over `frozen/` with `-a`/`--text` or an external diff driver, never run git on this
> repository without its working tree (a bare clone, `--git-dir` from elsewhere), and never check
> out, clone or add a worktree at a commit older than `6d61f65`, the first that hides the set's
> diffs.

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
honour an exclusion. So the text left the tree (leaf `PROGRAM.76`): no search of the working
tree, under any term, by any tool, reaches a case, and, in a working tree carrying the mark, no
diff of a commit that holds one shows it; reading one takes an act the box above names.

## The seal is mechanical

`scripts/check_frozen_evaluation.sh` runs in the pre-commit hook and in CI, through the
project doctrine slot. It enforces five properties:

| Property | What it refuses |
| --- | --- |
| **Integrity** | manifest entries that are not the sealing commit's own, a sealing commit that is no ancestor of `HEAD`, a digest the case's blob there does not hash to, or, once unsealed, a restored case that differs from it — the set cannot be rewritten, grown or shrunk to match what the engine turned out to do |
| **Custody** | while sealed, a case at its path, a link there included, or in the index; a case's whole text anywhere in the index or the working tree, ignored files aside; a file, the manifest included, staged or on disk, tracked or not, that quotes a long line of one; an unregistered repository nested outside ignored folders, or a file or folder git cannot read or list, since none can be ruled out; a sealed path not marked `-diff`, staged and on disk |
| **Completeness** | anything in `frozen/` but the manifest and, once unsealed, the listed cases — at any depth, hidden or linked; once unsealed, a listed case missing |
| **Non-contamination** | any tracked file **outside** `frozen/` that names a sealed case — a case written about in a task tree, decision record or design note is no longer unseen |
| **Exposure** | an `# exposed:` line naming no listed case — recording an exposure is this project's rule; no check can see a read |

It prints paths and listed names, never a line of a case — no value read from the manifest is echoed, a bad line
is named by its number, a blob is hashed through a pipe, a quote is found by the name of
the file that holds it — and its `--self-test` arms, on synthetic cases in a scratch repository,
check that of every run. The RED arms are recorded in leaves `M0.6`, `PROGRAM.18.1` and
`PROGRAM.76`.

⚠️ **The honest limit**, stated rather than hidden: the text stays in the published history,
which is not rewritten. Any act the box above names puts a case in front of a reader: a blob
shown or blamed by its path, a diff or grep forced to text or handed to an external driver, a
viewer that diffs blobs itself, git run on the repository without this working tree, a checkout, clone or worktree at a commit older than
`6d61f65`, or another clone not yet past it. `git grep <commit>` and `git log -S` over history
tell which file or commit holds a term, not its line. Ignored files, the build output under
`target/`, or a folder an untracked `.gitignore` ignores, are not scanned, nor a folder outside the repository a link
points to, nor a registered submodule, another repository (`REPOSITORY-BOUNDARY` refuses what is created at a vendored
checkout's first level); a quote shorter than a long line, or reworded, is not found; the
check sees what is staged and on disk, and CI what is pushed. The check cannot prove nobody read a
case; a human who reads one and stays quiet defeats it, so an exposure is recorded, never kept
quiet.

## Exposures

A case read before unsealing is counted apart from the unseen ones at the measurement. The
manifest's `# exposed:` line names it — `frozen/` is the one place a case may be named — and
this table says how it happened, without naming or describing it.

| Date | How | Extent | Answer |
| --- | --- | --- | --- |
| `2026-10-10` | a delegated read-only survey for leaf `M3.3`, told to sweep the whole tree, opened one case; a search in the working session printed one of its lines | one case: a few of its lines read by the survey and summed up to the session in one sentence | the text taken out of the tree and its diffs hidden (`PROGRAM.76`); 365 copies of the set that local clones and tool-made trees under `target/` held deleted, and 47 local clones whose index still held the set; the case reported apart at `M6.5`, four unseen cases remaining against §16's three |

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
