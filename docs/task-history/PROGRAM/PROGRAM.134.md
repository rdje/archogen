- ID: `PROGRAM.134`
  Status: `done` — `2026-11-10`, the live task trees over their ceiling again, `README-ROUTES` refusing the guard's round 35.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the review rounds' closed leaves in `docs/tasks/M4.md`, format `/2`'s six and the application inputs' seven among them, had grown it to 155,320 bytes since `PROGRAM.131` sealed it. WHERE: `bash scripts/check_doctrines.sh` on the guard's round 35, staged → `README-ROUTES: docs/tasks/: 819894 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree, that leaf set aside: 16 closed subtrees, 28 leaves, into `docs/task-history/M4/`, 27 log rows into `rows-0004.md`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (553 sealed file(s), each against its row and its sealing commit`, exit 0, "the reconstruction is byte for byte"; `docs/tasks/M4.md` 82,662 bytes, `docs/tasks/` 743,666 in all; `bash scripts/check_readme_routes.sh` → `readme-routes: OK (25 destination(s) governed)`, exit 0.
    - [x] NO REGRESSION — `bash scripts/check_handoff_ledger.sh` → `handoff-ledger: OK`, exit 0, each hand-off's leaf still open; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1265 (leaf PROGRAM.134)`
