- ID: `PROGRAM.124`
  Status: `done` — P1, filed `2026-11-03` by `M4.1.19`'s commit, which the gate refused for `docs/tasks/M3.md` at 3008
    lines, over its 3000, with nothing closed to seal; closed the same day.
  Reproduce / issue: `bash scripts/check_task_history.sh --seal M3` → `nothing to seal`, while `M3.md`'s Verification Log
    held rows whose leaf cell names one sealed leaf with a note, `` `M3.1.1` (step 1) ``: the seal's pattern, `LOGS`, took a
    cell that is the leaf alone, so those rows stayed live. The director ruled `2026-10-20` that the ceiling is met by
    sealing, not by relocating text.
  Built: `LOGS` reading a note in parentheses after the leaf as no second leaf, in both logs; a note holding a code span
    stays live, the seal not reading its prose; two rows in the arm's fixture, one that goes and one that stays; the
    record's amendment and `docs/TASK_TREE_SEALING.md`. Then, on the committed trees: `--seal M4`, 16 subtrees, 21 leaves
    and 46 log rows; `--seal M2`, 8 subtrees, 8 leaves and 47 rows; `--seal M3`, 71 rows; `--seal PROGRAM`, 2 subtrees, 2
    leaves and 55 rows; each `the reconstruction is byte for byte`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash scripts/check_task_history.sh --seal M3` at `8a767597`, before the change → `nothing to seal`, `M3.md` at 2999 of its 3000 lines.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git show 8a767597:scripts/check_task_history.sh | grep -c 'no second leaf'` → `0`: `LOGS` matched a closing backtick directly before the cell's end, so a note after it named no leaf.
  - [x] **FIX** — `scripts/check_task_history.sh`'s `LOGS`, its arm's fixture and its expected rows; `docs/decisions/decision_task-log-row-sealing.md`; `docs/TASK_TREE_SEALING.md`; the four seals.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_task_history.sh --self-test` → `174 pass / 0 fail`, the new rows among them; `wc -l docs/tasks/M3.md` → `2928`, 72 lines of room; `docs/tasks/` 785285 → 614992 bytes before this leaf's own text.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree; the gate's `task-history` leg holds every seal across history.
  - [x] **LOCKSTEP** — the record, the sealing guide, this leaf and the Commit Log; `MEMORY.md`.
  Commit: `ARCHOGEN-PROGRAM-1122 (leaf PROGRAM.124)`
