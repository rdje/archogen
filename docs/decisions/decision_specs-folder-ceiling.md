# The specifications folder's ceiling: raised to 384 KiB as a second accepted design moves in

- **Type:** `decision`
- **Date:** `2026-10-05`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.56` (`docs/tasks/PROGRAM.md`), under `README_POLICY.md`'s rule that a routed
  destination's ceiling rises only with a decision record, and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s, that the record
  shows the surface's role grew. `PROGRAM.43` set the ceiling on `2026-10-01`; [[decision_specifications-home]] is the
  ruling that sends accepted designs here.

## The fact / decision

`docs/specs/`' total-bytes ceiling in `README_POLICY.md` rises from 262 144 (256 KiB) to **393 216 (384 KiB)**, and
the trust gate's design, its review closed by `M3.6.1` on `2026-10-05`, moves to `docs/specs/trust/`. The folder's
other ceilings stay: 16 files; 1 200 lines, 98 304 bytes and 1 536-byte lines a file. `docs/decisions/`' ceiling,
the director's one-time raise, is untouched.

**Raised `2026-10-11` to 524 288 (512 KiB)** (`PROGRAM.79`). Measured at `2b66082`,
`git ls-files docs/specs | xargs cat | wc -c` → 389 283 of 393 216, 3 933 to spare; at `b91358a`, this record's
raise, 300 288; 46 commits between them, 88 995 bytes, about 1.9 KB a commit. The role grew again: the trust subject's
three generated-sources designs moved in — the instrument's (52 069 bytes), its refusals' (25 495) and the adequacy
criterion's (5 695), kept here while its review is open — and the adequacy design's first round answered alone takes
the folder to 396 402. At 512 KiB the folder holds that review's rounds and, at the measured rate, about 70 commits
more. Weighed and refused: compacting — a record's review table is its own account of how it was decided, not a
history kept beside it, and nothing here is terminal; a total by subject — the per-file caps already bound each
record, and a total for each sub-folder would move the same decision without changing it.

**Raised `2026-10-22` to 589 824 (576 KiB)** (`PROGRAM.102`). Measured at `757c49a6`, `git ls-tree -r -l HEAD --
docs/specs`: 524 246 of 524 288, 42 bytes to spare, 134 963 bytes in the 264 commits since `e2d11228`, nearly all the
trust subject's mutation-adequacy records, a line of work stopped on `2026-10-21` until the functionality is locked.
`docs/decisions/` held 392 950 of the director's 393 216, and `M3.3.6` needs rows there. The role grew by a subject: the
runtime analysis variant's design, accepted with `M2.11`'s fifth review and unchanged since `2026-10-03`, moves to
`docs/specs/analysis/` (38 794 bytes), taking the folder to about 563 000. One 64 KiB step holds it, with room for
about 50 commits at the overall rate. Weighed and refused: moving a record that is still amended, or whose citations
cannot all follow it (`PROGRAM.102` names each).

**Raised `2026-10-23` to 655 360 (640 KiB)** (`PROGRAM.108`). Measured at `b9a070d4`, `git ls-files docs/specs | xargs
cat | wc -c`: 589 151 of 589 824, 673 to spare; at `efedf6d6`, the last raise, 563 064 — 26 087 bytes in 41
commits, 13 of them to the folder, nearly all the checkers' trust record and the two it amends, read against the
gate's code from its fifth review round and amended at each. Its review is open and each round's answer needs a few
hundred bytes there; one 64 KiB step holds it and the records still to move in. Weighed and refused: trimming the
record's settled prose to fit, which edits what a review has read.

## Why

**Measured `2026-10-05`**, `git ls-files <folder> | xargs cat | wc -c`: `docs/decisions/` held 393 061 bytes of its
393 216, 155 to spare, with the substitutability record's review still open and each round's answers needing about
half a kilobyte; `docs/specs/` held 253 792 of its 262 144. The ruling of `2026-10-01` answers the first: an accepted
design moves to `docs/specs/` when the decisions folder needs the room. The trust record, 46 125 bytes, is one, and
would put `docs/specs/` at 300 288, over its ceiling. The surface's role grew: `PROGRAM.43` sized it for one subject,
the catalog's six records at 186 667 bytes; it now receives every design whose review closes, and the next, the
substitutability relation's 65 KB record, follows when its own review does. At 384 KiB the folder holds both.

The alternatives were holding the decisions folder at its last 155 bytes, which would have stopped the open review's
answers, and raising `docs/decisions/`, which only the director may reopen.

## How to apply

- `README_POLICY.md`: the `docs/specs/` row's total is 393 216, and "Ceilings a decision fixes" names this record for
  it; `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s two rows are re-measured.
- A later raise is a dated paragraph here with the measurement that asks for it.
- Related: [[decision_specifications-home]], [[decision_decisions-folder-ceiling]], [[decision_trust-inventory]].
