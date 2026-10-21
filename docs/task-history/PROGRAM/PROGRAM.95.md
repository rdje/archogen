- ID: `PROGRAM.95`
  Status: `done` — a sealed leaf's log rows follow it; every tree's sealed-leaf rows moved, `2026-10-20`
  Goal: a sealed leaf's log rows follow it. `bash scripts/check_task_history.sh --seal <TREE>` moves every
    Verification Log and Commit Log row that names a stub and that the committed tree holds into numbered log-row
    files under `docs/task-history/<TREE>/`, byte for byte and in the tree's order, under the leaf seal's proof,
    gate, rollback and signal rules; the gate's seventh leg proves each file against its row, its form and the
    tree's logs just before its seal; the census reports what a seal would take
    (`docs/decisions/decision_task-log-row-sealing.md`).
  Verification: `bash scripts/check_task_history.sh --self-test`: 170 pass / 0 fail (170 arms), the 24 new arms
    failing first against the tool as it was (147 pass / 24 fail of 171); seven trees sealed, 1122 rows into seven files,
    every seal exit code 0 and the gate green after each; `docs/tasks/` from 815 600 to 465 813 bytes; the broken
    Verification Log tables of `M3` and `PROGRAM` repaired (18 stray blank lines), whose 35 rows the next seal takes;
    `docs/decisions/` made room by sealing the findings register's ruled §6 (`DECISION-HISTORY`).
  Acceptance:
    - [x] ROOT CAUSE — `bash scripts/check_task_history.sh --census HEAD`: exit code 0 — 271 229 byte(s) of log rows
      naming sealed leaves at `HEAD`, which no seal could take: the leaf seal moves leaves and a row is not a leaf, so
      the ceiling was being met by relocating rows into the book.
    - [x] ADDRESSED — `bash scripts/check_task_history.sh --seal <TREE>`, exit code 0 for `M3`, `PROGRAM`, `M1`, `M2`,
      `API`, `S0` and `M0`: 1122 rows into seven log-row files, the tree aggregate 815 600 → 465 813 bytes, the gate
      proving each file against its row, its form and its tree before the seal.
    - [x] NO REGRESSION — `bash scripts/check_task_history.sh --self-test`: 170 pass / 0 fail (170 arms); `make gate` →
      === all doctrines green ===; `make focused`, exit code 0 on the staged tree.
  Commit: `ARCHOGEN-PROGRAM-0827`

