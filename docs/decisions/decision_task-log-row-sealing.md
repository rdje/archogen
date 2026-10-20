# A sealed leaf's log rows follow it: the seal moves them, byte for byte, into numbered log-row files

- **Type:** `decision`
- **Date:** `2026-10-20`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.95` (`docs/tasks/PROGRAM.md`), on the director's instruction of `2026-10-20` that
  `docs/tasks/`' ceiling be kept and met by sealing rather than by relocating text. It extends
  [[decision_task-tree-sealing]], option (C) of [[decision_findings-for-director-review]] §8, to the two logs a tree
  keeps, and applies `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s `archive_terminal` class as that record does.

## The fact / decision

A task tree's Verification Log and Commit Log rows that name a sealed leaf are history, and
`bash scripts/check_task_history.sh --seal <TREE>` moves them out under the guarantees the leaves have.

- **What moves.** Every row of the tree's `## Verification Log` and `## Commit Log` tables whose leaf cell — the
  Commit Log's first cell, the Verification Log's second — names one leaf that stands in the tree as a stub, sealed
  by this run or by an earlier one, and that the tree as committed holds in the same log. A row naming an open
  leaf, a `done` leaf not yet sealed, no leaf or several stays live, and so does a row not yet committed, until the
  next seal, as a leaf closed only in the working tree does; the tables' headings and headers stay.
- **Where to.** `docs/task-history/<TREE>/rows-NNNN.md`, numbered from `0001` per tree in the order of its seals. A
  file holds, for each log it carries, the log's heading, a blank line, the tree's two header lines and the rows,
  byte for byte and in the tree's order, the Verification Log first, the logs parted by a blank line. One seal
  writes as many files as its rows need under a budget of 131 072 bytes and 1 500 lines a file, inside
  `README_POLICY.md`'s ceilings for the folder (163 840 bytes and 1 800 lines).
- **The index.** `docs/task-history/INDEX.md` gains, per tree, a second table under `` ## `<TREE>` log rows ``, one
  row a file: its name, its row count, lines, bytes, sha256 and the day. Rows are only ever appended.
- **Nothing is left where a row stood.** A sealed leaf's rows are found through the index; its stub names its commit
  already.
- **The proof.** Before a write, the seal checks that putting every row back at its line reconstructs the tree as
  it stood, byte for byte, and that the files' rows are the rows taken, in order. After a write, the gate runs, and
  a refusal rolls everything back, under the signal mask and the rollback rules the leaf seal has: the row files
  are created new beside the leaf files, the tree and the index are rewritten whole or not at all, each write
  noted before it is made.

**The gate**, `TASK-HISTORY`, gains a seventh leg and extends the others:
1. a log-row file's rows, lines, bytes and sha256 are its index row's, and it is exactly the form the seal writes;
2. files and rows correspond one to one, and a tree's log-row files are numbered 1 to n without a gap;
3. across history, every log-row table row any committed index held is still there, unchanged, and every log-row
   file is byte for byte what the commit that added it wrote;
7. **provenance**: for the log-row files one commit added to a tree (HEAD, for a seal not yet committed), taken
   together, every row of those files was a row of the tree's same log just before that commit, in that order and
   under that log's header; every row the log held then that names a sealed leaf is kept in the tree at that commit,
   byte for byte and in order, or sealed in those files — none dropped and none invented, other edits in the sealing
   commit being no more the seal's than they are leg 5's; and every sealed row names one leaf of that tree that is a
   stub at that commit and was `done` before it.

## Why

- **Measured `2026-10-20`.** `docs/tasks/` stood at 818 469 bytes under its 819 200 ceiling after `M3`'s latest
  seal. Of its bytes, 433 254 were log rows naming sealed leaves: `M3` 152 225, `PROGRAM` 123 533, `M1` 78 065,
  `M2` 65 473, `API` 8 600, `S0` 4 185, `M0` 1 539. The leaf seal cannot reach them: a row is not a leaf.
- **What the ceiling was being met with.** `M3.6.9.6.3.2.2.15.2.1.6.1.1.1` freed 4 740 bytes by moving 21 sealed
  commit notes into the book and leaving pointer rows; the leaf before it moved four contracts the same way. The
  bytes were relocated into an unbounded file, not contained, and each move was audited by hand. The ceiling's
  purpose — a bounded, browsable resume surface — is met by sealing what is finished, as the leaf seal does.
- **The rows of sealed leaves only.** A leaf's rows and its body leave together: until a subtree seals, a reader of
  it still finds its finished leaves' closures in the logs. The rows of `done` leaves not yet sealed weighed 3 742
  bytes.
- **Numbered files, not the leaf's file.** A sealed file never changes, its row's digest proving it, so the rows of
  leaves sealed earlier cannot join their file. One immutable file per seal keeps the index append-only and the
  folder's file count a function of seals, not of leaves. The budget makes a seal that moves `M3`'s 152 225 bytes
  at once meet the folder's 163 840-byte file ceiling by construction.
- **No stub for a row.** A pointer row would keep most of a row's bytes and say nothing the leaf's stub does not.

## How to apply

- `--seal <TREE>` moves rows whenever it runs, with or without a subtree to seal; the first run after this record
  moves the rows of every leaf sealed so far.
- A sealed row is never edited. A correction is a new row in the tree, naming the leaf; the next seal moves it too.
- `COMMIT-LOG-ROWS` reads `docs/task-history/` as it reads the trees, so a sealed commit row still counts.
- `--census <COMMIT>` reports, per tree and in total, the bytes of rows a seal would take.
- Related: [[decision_task-tree-sealing]], [[decision_task-history-line-ceiling]], [[decision_history-ledgers]].
