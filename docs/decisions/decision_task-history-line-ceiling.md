# The task history's line ceiling: raised to the task trees' own, so a seal moves what a tree may hold

- **Type:** `decision`
- **Date:** `2026-10-06`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.63` (`docs/tasks/PROGRAM.md`), under `README_POLICY.md`'s rule that a routed
  destination's ceiling rises only with a decision record. `PROGRAM.32` set both ceilings when it built the seal
  ([[decision_task-tree-sealing]], the director's option C).

## The fact / decision

`docs/task-history/`' longest-line ceiling in `README_POLICY.md` rises from 2 048 bytes to **3 072**, the ceiling
`docs/tasks/` already has. Its other ceilings stay: 1 800 lines and 163 840 bytes a file, its file count and total
governed by `TASK-HISTORY`.

## Why

A seal moves a closed subtree's leaves **byte for byte** into `docs/task-history/` ([[decision_task-tree-sealing]]),
and `TASK-HISTORY` proves the reconstruction. So whatever a tree may hold, the history must hold too, or a subtree that
meets every ceiling of its tree cannot be sealed. The two ceilings disagreed: a tree's line may be 3 072 bytes, the
history's 2 048.

**Measured `2026-10-06`**, when `M3.1` closed and was the first subtree with ledger hand-offs to be sealed: `bash
scripts/check_task_history.sh --seal M3` wrote `docs/task-history/M3/M3.1.md` and `README-ROUTES` refused it —
*"docs/task-history/: 2871 bytes on its longest line, over its ceiling of 2048"*. The two lines, 2 771 and 2 871
bytes, are `M3.1.1`'s, the design's review narrated step by step, and both were within `docs/tasks/`' 3 072 when
written. The history's longest line before was 1 249 bytes, `docs/tasks/`' 2 871.

The alternatives were re-wrapping the leaf before sealing it — a change to a closed leaf made only to fit a ceiling,
leaving the disagreement for the next subtree to meet — or lowering the trees' ceiling, which would refuse lines the
trees already hold. Equal ceilings make the seal's byte-for-byte promise one the policy can keep.

## How to apply

- `README_POLICY.md`: the `docs/task-history/` row's longest line is 3 072, and "Ceilings a decision fixes" names this
  record for it.
- Should `docs/tasks/`' line ceiling ever rise, the history's rises with it, by a dated paragraph here.
- Related: [[decision_task-tree-sealing]], [[decision_decisions-folder-ceiling]].
