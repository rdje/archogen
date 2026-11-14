- ID: `PROGRAM.140`
  Status: `done` — `2026-11-14`, the live task trees over their ceiling again, `README-ROUTES` refusing the guard's round 43.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the review rounds' closed leaves in `docs/tasks/M4.md`, format `/2`'s rounds 12 to 14 and the application inputs' 13 to 15, had grown it since `PROGRAM.139` sealed it. WHERE: `bash scripts/check_doctrines.sh` on the guard's round 43, staged → `README-ROUTES: docs/tasks/: 823393 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree, that round's work set aside: 6 closed subtrees, 6 leaves, into `docs/task-history/M4/`, 6 log rows into `rows-0006.md`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (569 sealed file(s), each against its row and its sealing commit`, "the reconstruction is byte for byte"; `docs/tasks/` 782,551 bytes in all.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1296 (leaf PROGRAM.140)`
