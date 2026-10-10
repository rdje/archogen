# The decisions folder's ceiling: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.38` (`docs/tasks/PROGRAM.md`). This is the review history of
  [[decision_decisions-folder-ceiling]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`README_POLICY.md` requires an explicit reviewed decision before a routed destination's threshold rises. The round
below is that review, by a context that did not write the record, of its first draft.

**Round 1**, `2026-09-30`. The reviewer re-measured the folder at `HEAD` `7a628a0`: 28 files, 324 165 bytes. With
the draft untracked, it was 327 974, already over the old ceiling. It traced the ceiling to `38d8634` at 12:50
(23 files, 250 243 bytes), and its breaches to `4cba2d6` and `cea5776`. It confirmed that both held template checks
read the folder flat, and that `README-ROUTES` counts a sub-folder into its parent. It found 4 defects, 6 gaps, 5
ambiguities and 3 nits, 18 in all. The verdict was "cannot land as written", with the ceilings 40 and 393 216 judged
"modest and defensible". The answering context re-measured what grew: the catalog design was one record of 72 269
bytes at 12:50, and the catalog design with the composition design is now four records of 136 093 bytes.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| D1 | defect | the record argued placement and a blocked partition, not the policy's test that the contract expanded; ~5 KB for the next answers is landing new content | both stated plainly: the role grew, from one 72 269-byte design under review to four records of 136 093 bytes; and the raise is the director's time-boxed exception, since the immediate need alone would not pass the clause |
| D2 | defect | "no other append-only content is left" was false: the findings register holds 9 749 bytes of settled items | corrected, and the compaction (about 12.6 KB with index trimming) weighed among the alternatives and handed to `PROGRAM.39` |
| D3 | defect | the trigger committed to the partition it rejected as losing enforcement | the partition keeps enforcement with a project gate and curated map links (allowed by findings §10); the rejection now rests on cost and timing |
| D4 | defect | a partition with new rows would add capacity, a second raise | the conservation rule: folder and sub-folders together within 40 files and 393 216 bytes; any net increase needs a new reviewed ruling |
| G1 | gap | "only once" was a promise; no leaf owned the partition; the row's owner was the whole tree | `PROGRAM.39` opened now and named in the row's owner cell; `README-ROUTES` to refuse a ceiling above the one a decision names is part of it |
| G2 | gap | no warning before the ceiling | 36 files or 360 000 bytes starts `PROGRAM.39` before anything else adds to the folder |
| G3 | gap | the owning leaf `PROGRAM.38` was not declared | declared, with its checklist |
| G4 | gap | the inventory row, the index row and the knowledge map were not in the same change | all land in the raise's commit |
| G5 | gap | a sub-folder row nothing routes to is refused as stale | the plan routes each through the parent's "Overflows to" |
| G6 | gap | "the content is where it belongs" was asserted for the catalog design | shown: `M2.7.1` pins the design as a decision record; specifications live elsewhere |
| A1 | ambiguity | "the catalog design's four records" unnamed | named |
| A2 | ambiguity | a file count given as a rate | a time horizon from the measured pace: the last 77 437 bytes lasted two and a half hours |
| A3 | ambiguity | who can reopen the ceiling | only a new explicit ruling by the director |
| A4 | ambiguity | "pause" rejected on the aggregate's purpose | rejected on its cost to `M2.7.1` and `M2.10`; debt with growth stopped named as the policy's route |
| A5 | ambiguity | `README-ROUTES` presented as an obstacle | said to be archogen's own, which `PROGRAM.39` changes |
| N1 | nit | 324 162 against a measured 324 165 | 324 165; 69 051 of headroom at `HEAD`, and 61 187 measured with this change staged, 11 files |
| N2 | nit | "every file is a durable decision" | 23 decisions, two references and one register |
| N3 | nit | the ruling recorded without its question | the question and the options quoted |

## Why

The record states the raise as it stands, and this file keeps how it got there.

## How to apply

A later review of the record appends here, and adds a row to the record's `## Review`.
