# Closed subtrees are sealed out of the task trees, byte for byte, one file per subtree

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.32` (`docs/tasks/PROGRAM.md`), carrying out option (C) of §8 of
  [[decision_findings-for-director-review]], ruled `2026-09-30` by delegation ("(C) as recommended … `M1` and
  `PROGRAM` first"). §10 of the same record, ruled the same day, holds the template's 17 unchanged files, and this
  design leaves them as they are. It applies `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s `archive_terminal` class.

## The fact / decision

A task tree keeps its live work readable by sealing out what is finished.

- **The unit is a closed subtree.** A subtree is one of the tree's top-level children with every leaf under it, for
  example `M1.13` with `M1.13.1`, `M1.13.2` and the rest. It is closed when every one of its leaves has status
  `done`. A subtree with an open leaf stays whole in the tree, its closed leaves included, until it closes. So a
  sealed file never has to change again.
- **Its leaves move byte for byte**, in the order the tree holds them, into
  `docs/task-history/<TREE>/<SUBTREE>.md`. The file holds exactly those bytes, with no header. A leaf is its
  `- ID:` line and every line up to the next `- ID:` line or `## ` heading.
- **Each leaf leaves a stub where it stood**, two lines:

  ```text
  - ID: `M1.13.2`
    Status: `done` — sealed in [`M1/M1.13.md`](../task-history/M1/M1.13.md); commit `ARCHOGEN-M1-0082`
  ```

  The `ID` and `Status` lines are what every check that reads a leaf needs. The commit is the first work-unit id
  the leaf's `Commit:` field names. Failing that, it is `in the tree's Commit Log` when that log has a row for the
  leaf, and `not recorded` otherwise; the seal warns about a `done` leaf whose field names no commit.
- **A sealed body is its `ID` line and indented or blank lines only.** The seal refuses a leaf with a line at
  column 0 after its `ID`, a fence, a heading or prose, because the line slicing every check shares would tear it.
  It writes nothing, and names each such line.
- **What stays live:** the tree's root leaf, every open leaf, every subtree with an open leaf, and the sections
  around the leaves: the goal, the Current Frontier, the decisions, the logs and the changelog.
- **`docs/task-history/INDEX.md`** has one table per tree, with one row per sealed file: the subtree, its leaf
  count, lines, bytes, sha256, and the day it was sealed. Rows are only ever appended.
- **Reading a closed leaf** is following its stub's link, a file read that depends on no tool.

**Sealing is done by a tool**, `bash scripts/check_task_history.sh --seal <TREE>`. It proves that the tree, with
every stub replaced by its body from its sealed file, reconstructs the tree as it stood, byte for byte. If it does
not, it writes nothing. Then it runs the gate on what it wrote, and rolls everything back if the gate refuses.

**The gate**, `TASK-HISTORY`, runs on every commit, in CI as in the pre-commit hook. It checks:
1. every sealed file's leaves, lines, bytes and sha256 against its row;
2. that sealed files and rows correspond one to one, and that nothing else is under `docs/task-history/`;
3. **across history**: that every row any committed version of the index held is still there, unchanged, and that
   every sealed file is byte for byte what the commit that added it wrote. So CI, where `HEAD` is the commit under
   test, catches a forged file and row as surely as the hook does;
4. that every leaf in a sealed file has exactly one stub, in its own tree, with status `done` and a link to that
   file; that every stub links a file that holds its leaf; and that a leaf sits in its own subtree's file;
5. **provenance**: that every sealed leaf is, byte for byte, the leaf its tree held just before the commit that
   sealed it (`HEAD`, for a seal not yet committed), `done` there, and sealed with the rest of its subtree. So a
   seal made by hand, or a body edited on its way in, is refused;
6. that no live leaf sits in a subtree that is sealed.

**What else changes:**
- **`TASK-ACCEPTANCE`**, archogen's since `PROGRAM.21`: when a commit names a sealed leaf as its owner, the
  refusal says the leaf is closed and sealed, and that a change needs an open leaf. It no longer says only that
  there is no checklist.
- **`README_POLICY.md`**:
  - `docs/task-history/` gets a row of its own, reached through `docs/tasks/`' "Overflows to";
  - `docs/tasks/`' debt cells, which this leaf owns, become measured ceilings.

## Why

- **The pressure was measured on `2026-09-30`.**
  - `M1.md` is 717 804 bytes over 7 633 lines. 70 of its 74 leaves are done, and their bodies are 77% of it.
  - `PROGRAM.md` is 378 708 bytes, with 54 of 63 leaves done, at 79%.
  - Of `M1`'s 39 subtrees, 37 are closed, 465 944 bytes; of `PROGRAM`'s 39, 32 are closed, 281 453 bytes.
- **A closed subtree, not a closed leaf.** Sealing leaf by leaf would either make a sealed file change as its
  siblings close, or scatter one subtree over many files. A subtree seals once, whole, and a reader finds a
  feature's history in one file.
- **Byte-exact files and a reconstruction proof**, as for the changelog (`decision_history-ledgers.md`), because
  evidence that is edited on the way into an archive is no longer evidence.
- **`docs/task-history/`, not `docs/tasks/sealed/` or `docs/history/`.**
  - The template's held checks read `docs/tasks/` and its sub-folders, and archogen may not change them. Among
    them, `LESSON-PROMOTION` would take the moved `promotion: declined` lines as new ones, and `WAIVER-ROUTING`,
    `TASK-ACCEPTANCE` and `TABLE-ARITY` would judge moved text as added.
  - Outside `docs/tasks/`, moved text is not seen as new, and `docs/tasks/`' 20-file ceiling holds.
  - `docs/history/` is the rolling ledgers' home, with a 256-byte line ceiling. Eight closed `M1` leaves have
    longer lines.
- **Measured against the checks that read leaves** (a read-only audit on `2026-09-30`):
  - `WAIVER-ROUTING`, `GAP-CLAIM-CENSUS`, `TASK-TREE-OWNERSHIP` and `LIVE-DOC-CURRENCY` pass a sealing commit;
  - `STATED-ORDER` and `README-ROUTES` read a stub's `ID` and `Status` lines;
  - `S0-RETIREMENT` finds a stub's `ID` line.
- **The checks grow stricter, and that is accepted.** `LESSON-PROMOTION` and `ROUTING-EVIDENCE` then no longer
  find their markers in leaves that are closed. A new lesson or routing line in a tree needs its own marker, which
  is what those checks ask.

## How to apply

- **Sealing:**
  - after a subtree closes, run `bash scripts/check_task_history.sh --seal <TREE>` and commit its result;
  - the sealing commit may also close the leaf that ran it, as the first one did. Every sealed leaf is proven
    against its tree as it stood just before that commit (leg 5), so other edits in the same commit do not weaken
    the proof;
  - a leaf in a sealed file is never edited. A correction is a new entry in the tree's changelog.
- **Reopening:** a sealed subtree is never reopened. New work under its heading opens a new top-level subtree, and
  a live leaf inside a sealed subtree is refused (leg 6).
- **Size:** each sealed file is bounded by `README-ROUTES`. The folder's file count and total are not, by design:
  an `archive_terminal` grows with what is finished, and no mandatory read includes it.
- **Reading:** follow the stub, or open `docs/task-history/INDEX.md`.
- **A change needs an open leaf.** A sealed leaf cannot own one.
- **Trees:** `M1` and `PROGRAM` first, as ruled. Then any tree whose closed subtrees are the larger part of it, by
  the same tool.
- Related: [[decision_history-ledgers]], [[decision_findings-for-director-review]] §8 and §10,
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`.

## Review

An independent read-only context reviewed the tool, its gate and the first seal (`PROGRAM.32.4`). It accepted the
seal: lossless, exactly the closed subtrees, every digest matching. It found that the tool needed hardening before
another tree is sealed. The findings, and the answer to each, are in
[`decision_task-tree-sealing-reviews.md`](../reviews/decision_task-tree-sealing-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 10 | 3 (the record's fallback text unlike the build's; a sealing commit that was not seal-only; figures that did not reproduce), with the seal not fail-closed and the gate not re-proving a seal as the gaps to fix first | the seal "correct and lossless"; the tool to be hardened before another tree is sealed |
