- ID: `PROGRAM.71`
  Status: `done` — filed `2026-10-10` by `PROGRAM.69`'s review round 7, its P1
  Goal: the history's index is append-only in order as well as in presence, as its record says ("Rows are only ever
  appended").
  Reproduce / issue: `scripts/check_task_history.sh`'s leg 3 compares each committed index's rows with the current
  index's as sets, so a row inserted above committed rows passes; measured by the round's own runner, `exp/e2`, untracked
  and so not durable → *"OK"*, and a
  mutation of `add_rows` that puts new rows first passes every arm. Present at `fea69ad`, before `PROGRAM.69`.
  Direction: leg 3 also requires each committed index's rows to stand, in their order, as a prefix of the current
  table's — or the record and the gate's line say "present and unchanged" rather than "append-only"; an arm either way.
  Built: the first direction, so the record's "Rows are only ever appended" and the gate's "append-only" stay true:
    leg 3 keeps each table's rows as a list, and refuses a committed table whose rows, all present and unchanged, no
    longer stand first and in their order — a row inserted above them or moved — beside its refusal of a row gone or
    changed; an arm moving a committed row above another.
  Acceptance:
    - [x] ROOT CAUSE — WHY: leg 3 compared rows as sets, so their order was not judged; WHERE, `git show b3266f25:scripts/check_task_history.sh`, `current = {t: set(r) for t, r in index.items()}`.
    - [x] FIX — leg 3's `in_order`, lists for sets; the arm.
    - [x] ADDRESSED — the arm before the fix: `bash scripts/check_task_history.sh --self-test`: `170 pass / 1 fail (171 arms)`, the moved row `expected exit 1, got 0`; after it: `171 pass / 0 fail (171 arms)`; the leaf's mutant, `add_rows` putting new rows first, `161 pass / 11 fail (172 arms)`, where it passed every arm before.
    - [x] NO REGRESSION — `bash scripts/check_task_history.sh`: OK over the real history, every committed table a prefix in order; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 256 s.
    - [x] LOCKSTEP — the gate and its arm; the record and the gate's line, already saying "append-only", now held to it.
  Commit: `ARCHOGEN-PROGRAM-0971 (leaf PROGRAM.71)`
