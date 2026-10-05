# The reviews folder's ceiling: raised to 384 KiB, with the measurement that asks for it

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.49` (`docs/tasks/PROGRAM.md`), under `README_POLICY.md`'s rule that a routed
  destination's ceiling rises only with a decision record, and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s, that the record
  shows the surface's role grew. `PROGRAM.36` set the ceiling on `2026-09-30`.

## The fact / decision

`docs/reviews/`'s total-bytes ceiling in `README_POLICY.md` rises from 262 144 (256 KiB) to **393 216 (384 KiB)**.
The folder's other ceilings stay: 16 files; 1 200 lines, 131 072 bytes and 1 024-byte lines a file.

## Why

**Measured `2026-10-03`**, `git ls-files docs/reviews | xargs cat | wc -c`: 264 219 bytes over 2 111 lines in 11 review
histories and the index, 2 075 bytes past the ceiling, which `README-ROUTES` refused with the commit that answered
the substitutability record's fourth round. `PROGRAM.36` set that ceiling from 6 histories at 117 623 bytes, with
room for one design's review. The surface's role has grown since in two ways. There are nearly twice the designs
under review. And each review runs under the closure rule — a design closes on the first round that finds no defect
— so a history holds as many rounds as it takes: the catalog's record took 16 (75 383 bytes), the composite inputs
11, the runtime variant 9, the port's statement and the faults observation 7 each, the substitutability relation 4
so far (16 756 bytes), and a round with its findings and answers is 3 to 5 KB. A history is frozen when its review
closes and is never edited (`docs/reviews/INDEX.md`), so the folder cannot be compacted, and `docs/reviews/` has no
overflow destination of its own. The one other remedy — a terminal archive for closed histories, as
`docs/decision-history/` is for settled sections — is a mechanism no leaf owns yet, and 2 KB over is not what calls
for one.

At 384 KiB the folder has room for some thirty rounds at the measured rate. If the rate holds, the next step is that
archive, with the closed histories' bytes proven against an index as `TASK-HISTORY` and `DECISION-HISTORY` prove
theirs — not another raise.

**`2026-10-05`.** The rate held (393 216 reached in two days), so the archive came, not a raise: `PROGRAM.55`.

## How to apply

- `README_POLICY.md`: the `docs/reviews/` row's total is 393 216, and "Ceilings a decision fixes" names this record
  for it; `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s inventory row is re-measured.
- A later raise is a dated paragraph here with the measurement that asks for it; a closed history that must leave the
  folder is a new mechanism, filed as a `PROGRAM` leaf.
- Related: [[decision_decisions-folder-ceiling]], [[decision_book-in-layers]], [[decision_specifications-home]].
