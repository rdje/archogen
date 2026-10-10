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
  `done`. A subtree with an open leaf stays whole in the tree, its closed leaves included, until it closes — *amended
  `2026-10-10`: but for its closed parts, which seal on their own (next).* So a sealed file never has to change again.
- **Amended `2026-10-10` (`PROGRAM.69`): the unit is the outermost closed subtree.** A top-level subtree whose every
  leaf is `done` seals as before. Below a top-level subtree that is still open, a leaf whose own subtree — itself and
  every leaf below it — is `done`, and whose nearest ancestor leaf's is not, seals too, whole, into
  `docs/task-history/<TREE>/<LEAF>.md`: `M3.6.2` with `M3.6.2.1` while `M3.6` waits on `M3.6.5`. Below the top level
  a subtree is a leaf of the tree, so two closed leaves with no leaf between them and an open subtree — `X.1.2.1` and
  `X.1.2.2` under an open `X.1`, with no leaf `X.1.2` — seal apart. A sealed file still never changes, since its subtree
  is closed. When the parent closes later, the parent's remaining leaves seal into the parent's own file, beside it. A
  seal takes the outermost closed subtree, whole: the gate refuses a part sealed apart from a subtree that was closed,
  and, below the top level, parts sealed together that no one leaf holds (leg 5), and a live leaf under a sealed subtree at any depth and in
  any tree file (leg 6).
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
- **What stays live:** the tree's root leaf, every open leaf, every subtree with an open leaf — *amended `2026-10-10`:
  every leaf with an open leaf below it; a subtree with an open leaf no longer stays whole* — and the sections around
  the leaves: the goal, the Current Frontier, the decisions, the logs and the changelog.
- **`docs/task-history/INDEX.md`** has one table per tree, with one row per sealed file: the subtree, its leaf
  count, lines, bytes, sha256, and the day it was sealed. Rows are only ever appended.
- **Reading a closed leaf** is following its stub's link, a file read that depends on no tool.

**Sealing is done by a tool**, `bash scripts/check_task_history.sh --seal <TREE>`. It proves that the tree, with every
stub replaced by its body from its sealed file — *amended `2026-10-10`: every new stub, those of earlier seals proven by
their own (review R11 P1)* — reconstructs the tree as it stood, byte for byte. If it does not, it
writes nothing. Then it runs the gate on what it wrote, and rolls everything back if the gate refuses — *amended
`2026-10-10`: or if any exception stops it, the folders a first seal made included; the tree and the index are written
whole or not at all, a sealed file's path already taken is refused before any write, and the rollback undoes what was
written alone, each step on its own, keeping the sealed files whenever a restore fails, since the tree or the index
may still name them, and naming what it could not undo (review R9-1, worded by R11 R2); each write is noted before it
is made, so an interrupt just after one is undone too, any entry at a sealed file's path is refused, and so is a tree
or an index that is a link (review R10), or a history folder (R11 P2) — *by review R12: any path the seal writes that
a link lies on, the file or a folder above it, below the root; by R13 D3, its temporary file created new; by R14 D3,
any entry at a rewritten file's temporary path refused before the first write* —; and a stop — any signal but
SIGKILL, SIGSTOP, the six fault signals (SIGSEGV, SIGBUS, SIGFPE, SIGILL, SIGTRAP, SIGSYS), whoever sends them, and,
on Linux, the C library's own signals, which no mask holds —
is held by the signal mask from before the first write through the proof and any rollback, taking effect once the
seal is done: proven and kept, refused and rolled back, or stopped by an error, said, and rolled back, its outcome
said where its output can be written; an interrupt held alone then says which — after an outcome said, or after one
that could not be said, and one that came before the mask that nothing was written — where its error output can be
written, and exits 130 whatever became of
its outputs; so the seal is whole either way, a signal ignored on entry staying ignored, and a git the proof runs,
which inherits the mask, not ended by a stop; an abort the process raises on itself ends it whatever the mask, and a
C library signal sent from outside may; and only a rollback that undid everything is said to have
rolled the seal back (reviews R11 D1, R12 D3, R13: a stop recorded and answered later left moments between a write and
its record, which holding closes; R14 D1, D2; R15 D1, D2; R16 D1, D5, D6; R17 D1 to D4; R18 D1).*
`--census <COMMIT>` measures, at a commit, the bytes of `done` leaves under open top-level subtrees and of those a seal
would take (`PROGRAM.69`).

**The gate**, `TASK-HISTORY`, runs on every commit, in CI as in the pre-commit hook. It checks:
1. every sealed file's leaves, lines, bytes and sha256 against its row — *amended `2026-10-10`: and that it holds a leaf*;
2. that sealed files and rows correspond one to one, and that nothing else is under `docs/task-history/`;
3. **across history**: that every row any committed version of the index held is still there, unchanged, and that
   every sealed file is byte for byte what the commit that added it wrote. So CI, where `HEAD` is the commit under
   test, catches a forged file and row as surely as the hook does. History is read with `--full-history`, so a
   merge that keeps only a side that never sealed cannot hide a seal, and a shallow repository is refused
   (`PROGRAM.42`);
4. that every leaf in a sealed file has exactly one stub, in its own tree, with status `done` and a link to that
   file; that every stub links a file that holds its leaf; and that a leaf sits in its own subtree's file;
5. **provenance**: that every sealed leaf is, byte for byte, the leaf its tree held just before the commit that
   sealed it (`HEAD`, for a seal not yet committed), `done` there, and sealed with the rest of its subtree. So a
   seal made by hand, or a body edited on its way in, is refused — *amended `2026-10-10`: when it differs from the
   tool's in its files, its stubs' links or its units; one byte for byte the tool's passes, however made, and a stub's
   commit text, a row's date and the column-0 rule stay the seal's alone, which the gate does not check, and so do where
   a stub stands among its tree's lines and text after the name on its ID line (review R9-5), a seal of only some of a
   tree's closed units, each the tool's — the rest seal later as units of their own — and prose in the index outside
   its tables (review R10).* *Amended
   `2026-10-10`:* and that, in that tree, the
   outermost closed subtree holding each of its leaves was its file's subtree, so a part of a closed subtree sealed
   apart from it, or, below the top level, parts sealed together that no one leaf holds, is refused; and that the
   file is exactly the bytes
   those leaves' spans made, in that tree's order, so a file reordered or padded is refused too;
6. that no live leaf sits in a subtree that is sealed — *amended `2026-10-10`: at any depth, and in any tree file of
   `docs/tasks/`, one in a sub-folder being `PROGRAM.72`'s;
   and a tree file holds only leaves under its own name, each named once, which a seal checks before it writes; and the
   index keeps one table per tree, its header first, its rows under it, one row a sealed file.*

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
  feature's history in one file — *amended `2026-10-10`: in one file per closed part, below a subtree that stays
  open (next).*
- **Amended `2026-10-10`: why the outermost closed subtree.** The top-level unit assumed a top-level subtree closes
  soon after its parts. It does not when one of its leaves waits on a decision: `M3.6` stays open while `M3.6.5` waits
  on the director, and every finished leaf below it stays live. Measured by `bash scripts/check_task_history.sh
  --census fea69ad`, every leaf's span in bytes: 274 483 bytes of `done` leaves under open top-level subtrees — `M1` 85 113, `M2` 105 946,
  `M3` 83 424 — 261 610 of them in 28 subtrees closed below the top level, with `docs/tasks/` at its 819 200-byte
  ceiling (`README-ROUTES` refused `M3.6.6.2.1`'s commit until two closed `PROGRAM` subtrees were sealed). The seal
  moved 261 582 bytes into those 28 files: one blank line after each subtree's last leaf stays in the tree. The cost is the one weighed above: a
  feature's history can span several files — the parent's own, and one per part that closed before it. Each file is
  still one closed subtree, whole and never changed, and the stubs, in the tree's order, say where each leaf is.
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
    the proof. *Amended `2026-10-10`:* any exception that stops a seal rolls it back first; a signal that kills the process, SIGTERM among them, leaves its writes, which the next gate run proves as any uncommitted seal (review R8-2) — *amended by R13 to R18: every signal but SIGKILL, SIGSTOP, the six fault signals and, on Linux, the C library's own is held until the seal is done, so what leaves the writes is SIGKILL, a fault signal whoever sends it, an abort the process raises on itself, a C library signal sent from outside, or the machine stopping* —; a failure that defeats the rollback's own writes too, a full disk, leaves what the rollback names (R9-1). The seal reads the working tree and leg 5 the tree as committed, so an
    uncommitted edit that changes which subtrees are closed — a new open leaf under a closed one — makes the seal
    refuse itself and roll back; commit such an edit first;
  - a leaf in a sealed file is never edited. A correction is a new entry in the tree's changelog.
- **Reopening:** a sealed subtree is never reopened. New work under its heading opens a new top-level subtree — *amended
  `2026-10-10`: a new subtree outside it, a sibling for a part sealed below an open subtree* — and a live leaf inside a
  sealed subtree is refused (leg 6).
- **Size:** each sealed file is bounded by `README-ROUTES`. The folder's file count and total are not, by design:
  an `archive_terminal` grows with what is finished, and no mandatory read includes it.
- **Reading:** follow the stub, or open `docs/task-history/INDEX.md`.
- **A change needs an open leaf.** A sealed leaf cannot own one.
- **Trees:** `M1` and `PROGRAM` first, as ruled. Then any tree whose closed subtrees are the larger part of it, by
  the same tool — and, amended `2026-10-10`, any tree once `docs/tasks/` reaches its ceiling, since finished leaves
  are what that folder overflows (`README_POLICY.md`).
- Related: [[decision_history-ledgers]], [[decision_findings-for-director-review]] §8 and §10,
  `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`.

## Review

An independent read-only context reviewed the tool, its gate and the first seal (`PROGRAM.32.4`). It accepted the
seal: lossless, exactly the closed subtrees, every digest matching. It found that the tool needed hardening before
another tree is sealed. The amendment of `2026-10-10` and its seal were reviewed the same way, from round 2
(`PROGRAM.69`). The findings, and the answer to each, are in
[`decision_task-tree-sealing-reviews.md`](../reviews/decision_task-tree-sealing-reviews.md).

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 10 | 3 (the record's fallback text unlike the build's; a sealing commit that was not seal-only; figures that did not reproduce), with the seal not fail-closed and the gate not re-proving a seal as the gaps to fix first | the seal "correct and lossless"; the tool to be hardened before another tree is sealed |
| 2 | 9 | 5 (no arm held the unit's rules; leg 5 refusing only a seal too narrow; figures not reproducible as bytes; sentences the amendment contradicted; the doctrine row) | the seal byte for byte and exactly the outermost closed set; every finding answered |
| 3 | 6 | 2 (the leaf's own figures left uncorrected; two corners of the unit's rule with no arm) | the seal, every figure and every round-2 answer but those two re-derived; every finding answered, two that predate the change among them |
| 4 | 8 | 3 (rounds appended to the history while its row read closed; two survivors called equivalent that were not; the census's producer untracked) | the seal reproduced by a clean clone; 840 fuzzed seals matched its oracle, and of 890 hand seals the gate accepted exactly those the tool makes (runners untracked, not durable); every finding answered |
| 5 | 5 | 2 (the census's byte figures held by no arm; a refused first seal leaving its folder behind) | the seal, the figures and R4-1's route re-derived; every finding answered, then a mutation sweep of the tool's core — 182 of 196 mutants killed, the 14 others each reasoned in the history (runner untracked, not durable) |
| 6 | 11 | 5 (a repeated row passing once its one note was silenced; a rollback that a file the gate cannot read escaped; two first-seal stubs "corrected" wrongly; the text claiming more than the gate checks; figures naming no state) | the seal re-derived and re-sealed from `72bd446`; every finding answered, the stub-commit check withdrawn; the review open |
| 7 | 10 | 4 (a sealed file holding no leaf passing; the leaf describing the withdrawn check; counts from untracked runners unmarked; a rollback whole for a file not UTF-8 alone) | the seal re-derived and re-sealed in a clone, 795 fuzzed seals and of 1 551 hand seals the gate accepted exactly those the tool makes (runner untracked, not durable); every finding answered, two pre-existing remarks filed or armed |
| 8 | 10 | 2 (the doctrine row's "a leaf a file"; the rollback claimed for whatever stops a seal, a kill signal leaving its writes) | the seal, the census and every figure reproduced; 960 fuzzed seals matched its oracle and of 1 368 hand seals the gate accepted exactly those the tool makes (runner untracked, not durable); every finding answered |
| 9 | 10 | 1, and 2 arm gaps (a rollback not whole when the failure defeats its own writes; "any exception" and a refused first seal's folder held by no arm) | the seal, the census and every figure reproduced; 480 fuzzed seals matched its oracle, and of 1 903 hand seals the gate accepted exactly the oracle's 555 (runner untracked, not durable); every finding answered |
| 10 | 11 | 4, and 3 arm gaps (an interrupt just after a write, before it was noted; a taken path refused only as a link to nothing; the book's rollback claim; round 9's counts unmarked) | the gate, every self-test, the census and the seal's figures reproduced, `72bd446` re-sealed byte for byte by two tools; every finding answered |
| 11 | 10 | 2, and 2 arm gaps (a first interrupt during the rollback after a refusal escaping it; the arms said to fail first misnamed) | the gate, every self-test, the census and the seal reproduced, `72bd446` re-sealed byte for byte; 753 fuzzed cases matched its oracle (runner untracked, not durable); every finding answered |
| 12 | 12 | 3, and 4 arm gaps (a stop after the last answer dropped; a link above the written folders passing; "rolled back" said of a rollback that was not whole) | the gate, every self-test, the census and the seal reproduced, `72bd446` re-sealed byte for byte (runner untracked, not durable); every finding answered, the claims narrowed to what the arms hold |
| 13 | 9 | 4, and 2 arm gaps (a second stop cutting short the rollback a given-back stop began; the boundary texts; a link at the temporary path written through; the changelog's count) | the gate, every self-test, the census and the seal reproduced, 535 fuzzed seals and 1 503 hand seals matching its oracle (runner untracked, not durable); answered by holding a stop with the signal mask, closing every moment rounds 11 to 13 found |
| 14 | 11 | 4, and 2 arm gaps (the interrupt's message said whatever happened; SIGQUIT and the other maskable signals not held; a link at the temporary path found after the first write; R13's change credited to R12) | the gate, every self-test, the census and the seal reproduced, 300 fuzzed trees and 1 662 hand seals matching its oracle, 445 late live leaves refused (runner untracked, not durable); answered: every signal a mask can hold held from before the first write, the outcome recorded and the message by it, an error said before it goes on, the temporary paths refused first |
| 15 | 9 | 2, and 4 arm gaps (the stop's message when the seal's report could not be written; "every signal a mask can hold" where seven fault signals are left out whoever sends them) | the gate, every self-test, the census and the seal reproduced, 330 fuzzed trees and 1 354 hand seals matching its oracle, 235 late live leaves refused (runner untracked, not durable); answered: the message by whether the seal started and said its outcome; the nine signals left unheld named; arms for the mask at the first write, the index's temporary path, the error path's breach and its rollback's wording |
| 16 | 13 | 6, and 2 arm gaps ("the stop saying so" of every stop where the interrupt alone does; the script's comment; two changelog entries; SIGABRT's reason; a lost report's exit 120) | the gate, every self-test, the census and the seal reproduced, the first seal's 28 files rebuilt from `72bd446` by an oracle, 160 fuzzed trees and 632 hand seals matching it (runner untracked, not durable); answered: the interrupt alone says which outcome, exiting 130 with its output closed; SIGABRT held, six fault signals left out; arms for the mask's eight and for the start before the first write |
| 17 | 11 | 5, and 1 arm gap (a crash with the output closed at the start, `sys.stdout` None; an outcome to a closed output recorded said; the interrupt's exit with its error output broken; an abort left out of what leaves the writes; the index's round count) | the gate, every self-test, the census and the seal reproduced, 200 fuzzed trees, 540 units, 192 late leaves and 99 partial hand seals as its oracle predicts (runner untracked, not durable); answered: an outcome said only to a stream that exists, one exit for every state of the outputs, an abort and the C library's own signals named; an arm for the git that inherits the mask |
| 18 | 7 | 1, and 0 arm gaps (the lists of what the mask leaves out disagreed on the C library's own signals) | the gate, every self-test, the census and the seal reproduced; an independent oracle over the real history found all 172 sealed files the outermost closed unit at their sealing commit's parent; 72 fuzzed trees, 432 hand seals and 66 late leaves as it predicts; 16 output states probed (runners untracked, not durable); answered: one wording in every list; the interrupt's "before any write" said of one before the mask; row 16's "with its output closed" stands as written, row 17 its correction |
