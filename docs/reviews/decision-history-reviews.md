# Sealing settled sections: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.41.1` (`docs/tasks/PROGRAM.md`). This is the review history of
  `DECISION-HISTORY`, whose design is the header of `scripts/check_decision_history.sh` and whose rule is the
  sealing bullet of [[decision_decisions-folder-ceiling]], kept apart as `docs/reviews/INDEX.md` describes.

## The fact / decision

**Round 1**, `2026-09-30`, of the gate and the first seal (`4973e33`), by a read-only context that had not written
them. It extracted §2, §4, §8 and §10 from the register at `4973e33^` with its own code: each sealed file was
byte-identical to its section, the rows' lines, bytes and digests reproduced, and the record rebuilt from its stubs
equalled its parent but for the intended line in its introduction. It ran constructions and a mutation matrix in
scratch repositories. There were 18 findings, 4 of them defects, and the verdict was that the first seal is "correct
and lossless", and the gate "not yet sound enough to seal more sections". The answering context (`PROGRAM.41.1`)
rewrote the gate. It armed every construction the review gave, and ran its own matrix of 42 mutations on a copy: 41
turn an arm red. The one that does not drops `--full-history` from the read of which commit added each file; the
only construction that hides a seal's side, a `-s ours` merge, is refused first by the rows leg, and no arm isolates
the read. The self-test holds 54 arms.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| D1 | defect | history simplification hid a seal behind a `git merge -s ours`, and an edit to the settled text then passed | both history reads take `--full-history`; a RED arm for the merge. `TASK-HISTORY` and `HISTORY-LEDGERS` read history the same way, and `PROGRAM.42` tries the construction on them |
| D2 | defect | a fence model toggling on backticks alone let a seal swallow a live section, or tear one | CommonMark fences, backticks and tildes, closed by a run as long; the seal refuses a section holding another `## ` line, fenced or not, or ending inside a fence; a sealed file holding a second `## ` line is refused; RED arms for both constructions |
| D3 | defect | the inventory's figure for the folder was measured before two late edits | re-measured, with the partition's and the reviews' rows (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`) |
| D4 | defect, low | no rollback when the gate met an unreadable file | the seal's gate call rolls back on any refusal, an unreadable file included; a RED arm |
| D5 | gap | a shallow clone passed, and a failed git read counted as empty history | a shallow repository is refused, and a failed history read is a breach; CI's `fetch-depth: 0` named in the header; a RED arm |
| D6 | gap | several legs had no arm | an arm for each: rows, headings, the missing file, a duplicate row, the deleted record, two stubs, each provenance branch, the record's name held twice, the proof (through a self-test seam that can only make a seal fail) and the rollback |
| D7 | gap | a stub in a record's preamble was not checked | every line of every record is scanned, the preamble included; a RED arm |
| D8 | gap | the index's header was not checked | the index must begin with its header, byte for byte; a RED arm |
| D9 | ambiguity | a stand-in record of the same name could supply the sealed section | stated in the header's honest limits: it equals an edit made live and then sealed, which only review sees |
| D10 | ambiguity | a breach committed past the hook stays red for good | stated: repaired by rewriting unpushed commits, never `main`'s, or in a leaf that owns the exception |
| D11 | ambiguity | "a move breaks nothing" hid two constraints | stated: a move needs every stub's link rewritten, and a rename is refused for good (the header, `DOCTRINE_ENFORCEMENT.md`) |
| D12 | ambiguity | nothing bounds the folder, and nothing checks that a section is settled | stated in the inventory ("unbounded by design") and in the ceiling record: sealing is a route around the ceiling that only review guards |
| D13 | nit | tracebacks and "section None" | a section number that is not a number, or given twice, is refused up front; a stub under an unnumbered heading is a named breach; RED arms |
| D14 | nit | line endings | the seal refuses a record with a carriage return or no final newline; `.gitattributes` marks `docs/decision-history/` `-text`; RED arms |
| D15 | nit | an ignored `.DS_Store` failed the gate | the folder is listed through `git ls-files -co --exclude-standard`; a RED arm, and a GREEN one for the ignored file |
| D16 | nit | a live section quoting the stub's words was refused | only an exact stub line outside a fence counts; a GREEN arm |
| D17 | nit | the row's date was never checked | it must lie between the sealing commit's parent and the sealing commit, in UTC; a RED arm |
| D18 | nit | "20 RED arms" counted 6 arms that expect a pass | corrected here: the first self-test held 14 arms expecting a refusal and 6 expecting a pass |

## Why

The gate states the design as it stands, and this file keeps how it got there, as for the task trees' sealing.

## How to apply

A later review appends here, and the header of `scripts/check_decision_history.sh` states what changed.
