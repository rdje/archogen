# Sealing closed subtrees: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.32.4` (`docs/tasks/PROGRAM.md`). This is the review history of
  [[decision_task-tree-sealing]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

**Round 1**, `2026-09-30`, of the tool, its gate and the first seal (`940baf1`), by a read-only context that had not
written them. It re-derived both trees with its own code:
- `M1` equals its parent byte for byte;
- `PROGRAM` equals its parent apart from the commit's own closure of `PROGRAM.32.3`;
- all 69 rows matched `shasum`, `wc` and a leaf count.

It confirmed that exactly the closed subtrees were sealed, 62 and 50 leaves, and checked all 112 stubs' commits.
Re-running `--seal` in a clone at `15c61b4` reproduced the seal. It ran a 25-case mutation of the gate and 9 seal
scenarios. Its verdict: the seal "correct and lossless", and the tool to be hardened before another tree is sealed,
P2 and P3 first. The answering context rebuilt the tool's core and took the self-test from 14 arms to 25, each new
leg proven by a mutation that turns its arm red. The real history passes every leg, all 112 sealed leaves proven
against the trees `940baf1`'s parent held.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| P1 | defect | the record said the fallback was `not recorded`; the tool wrote `in the tree's Commit Log`, unchecked; the example stub's commit was invented | the fallback is `in the tree's Commit Log` only when that log has a row for the leaf, else `not recorded`, and the seal warns; the example is the real stub (the record, the tool) |
| P2 | gap | the seal sliced by lines, so a column-0 fence, heading or prose tore a leaf, and one tear passed the gate | the seal refuses a leaf with a line at column 0 after its `ID`, writing nothing, and rolls back if the gate refuses what it wrote; RED arm (the tool) |
| P3 | gap | the gate never re-proved a seal: a hand-made seal from an open subtree, an edited body, a stub moved to another tree all passed | leg 5, provenance: every sealed leaf equals, byte for byte, its tree's leaf just before its sealing commit, `done`, with its whole subtree; stubs are tied to their tree and leaves to their subtree's file; RED arms (the tool) |
| P4 | ambiguity | the HEAD comparison is vacuous in CI, so a committed forgery passed there; the honest limit understated it | leg 3 is history-wide: every row any committed index held, and every sealed file against the commit that added it; RED arm for a committed forgery. `HISTORY-LEDGERS` has the same blind spot, filed as `PROGRAM.40` |
| P5 | gap | 8 of 15 breach paths had no RED arm | an arm for each, 25 in all |
| P6 | gap | reopening a sealed subtree was undecided | forbidden: new work opens a new top-level subtree, and leg 6 refuses a live leaf in a sealed subtree (the record, the tool) |
| P7 | defect | the rule said a sealing commit changes nothing else, and the first one also closed its leaf | the rule reworded: the sealing commit may close the leaf that ran it, and leg 5 proves every sealed leaf against its parent, so other edits do not weaken the proof |
| P8 | defect | figures that did not reproduce at their commit | corrected at the commit: `PROGRAM.md` 384 721 bytes before, 978 lines and 113 467 bytes after; the inventory's aggregate re-measured; the correction noted in `PROGRAM.32.3` and the tree's changelog |
| P9 | nit | the book overstated what the index check covers | "has lost or changed no row it ever held", and "since the commit that sealed it" |
| P10 | nit | a traceback in place of a named breach; the work-unit pattern's missing boundary; a stray file at the folder's root; the folder's size; a silent seal of a `done` leaf with a `pending` commit | named breaches; a boundary; stray files refused; the size stated as unbounded by design; a warning (the tool, the record) |

## Why

The record states the design as it stands, and this file keeps how it got there.

## How to apply

A later review appends here, and adds a row to the record's `## Review`.
