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
