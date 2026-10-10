# Archiving closed review histories: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-05`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.55.1` (`docs/tasks/PROGRAM.md`). This is the review history of `REVIEW-HISTORY`,
  whose design is the header of `scripts/check_review_history.sh` and whose rule is the dated line of
  [[decision_reviews-folder-ceiling]], kept apart as `docs/reviews/INDEX.md` describes.

## The fact / decision

**Round 1**, `2026-10-05`, of the gate and the first archiving (`49c44ed`), by a read-only context that had not
written them. It found the archived file byte-identical to the history at `49c44ed^` and at its closing commit, the
row's figures reproduced, every citation resolving through the stub, and the archiving reproduced byte for byte from
`49c44ed^` by the gate itself. It ran constructions in scratch repositories and 30 single mutations of the gate, 14 of
which no arm killed. There were 17 findings, 5 of them defects, and the verdict was that the first archiving is
"correct and lossless", and the gate "not yet sound enough to archive more histories". The answering context
(`PROGRAM.55.1`) rewrote the gate and armed every construction the review gave: a matrix of 38 single mutations, one per refusal and leg, each killed by the self-test (38 of 38 killed). The self-test holds
65 arms.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| E1 | defect | the status was the last cell of any line opening `\| [`, read by its first word: `closedness not reached`, a fifth cell the page drops, and a hidden row in a comment each archived an open review | a review's row is the table's own, exactly four cells, its status matching `closed: `, and the file linked once in the whole index; three RED arms |
| E2 | defect | a side branch's round, merged after the archiving with the stub kept, was lost | every commit touching the stub's path after the archiving, a merged side included, must hold the exact stub; a RED arm |
| E3 | defect | the rollback arm deleted the folder before looking at it, so two rollback faults survived | the arm removes the stray file alone and then requires the folder empty |
| E4 | defect | the reviews folder's figure was measured before the index's note | re-measured, and `PROGRAM.55`'s figure corrected in place |
| E5 | defect | tightening `DECISION-HISTORY`'s row dropped D11's recorded answer | restored: a record moves only with its stubs' links rewritten, and is never renamed |
| E6 | gap | 14 of 30 mutations survived: rollback, refusals, the lines and sha256 checks, a failed history read, a malformed row, a history no commit held | an arm for each; the claim restated as the matrix measured it |
| E7 | gap | a file ignored or left unstaged passed the hook and failed in CI | the gate reads the staged copy of every file it judges, as `RUST-FORMAT` does, and refuses one on disk and not staged, or staged otherwise than on disk; RED arms |
| E8 | gap | no `-text` attribute, so a clone with `core.autocrlf` broke the archive | `.gitattributes` marks `docs/reviews/` and `docs/review-history/` `-text`; the seal writes with `newline=""` |
| E9 | gap | a closed history could still change before its archiving | a commit changing a history while its row already read closed is refused; a RED arm |
| E10 | gap | a history quoting the stub's line was refused, and could never be archived | a stub is a file of three lines or fewer naming the archive; a longer history may quote it; a GREEN arm |
| E11 | gap | a near-stub left where a history stood passed | every stub-shaped file must be an exact stub with its archived file; a RED arm |
| E12 | gap | a stub never leaves, so the archive frees bytes and not files | stated in the inventory: stubs count toward the folder's 16 files by design, and a raise of that one cell, with its measurement, is the ceiling record's dated line when the count binds |
| E13 | gap | the book said nothing of the archive, and still sent the catalog's history to `docs/reviews/` | `annex-repository.md` beside `DECISION-HISTORY`, with its console block; `catalog.md` links the archive |
| E14 | ambiguity | an archiving in a merge commit was refused as "already a stub" | named: an archiving is a commit of its own, never a merge; a RED arm |
| E15 | nit | a directory at a stub's or file's path raised a traceback; a stub in a sub-folder was misnamed | named breaches; RED arms |
| E16 | nit | the gate held less than the seal: a CRLF archive, prose after the rows; `NAME_RE` unstated; unused capture groups; no read-back | the gate refuses both; the header states the name's form; the groups gone; the seal reads its copy back |
| E17 | nit | `PROGRAM.55`'s Commit Log row out of order | moved after `0430` |

## Why

The gate states the design as it stands, and this file keeps how it got there, as for the decision-history seal.

## How to apply

A later review appends here, and the header of `scripts/check_review_history.sh` states what changed.
