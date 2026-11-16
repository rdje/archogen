- ID: `PROGRAM.131`
  Status: `done` — `2026-11-07`, the live task trees over their ceiling, `README-ROUTES` refusing a commit.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: 53 closed leaves in `docs/tasks/M4.md`, the review rounds' bodies among them, had grown it to 213,819 bytes. WHERE: `bash scripts/check_readme_routes.sh` on the guard's round 23 → `docs/tasks/: 820406 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree: 44 closed subtrees, 53 leaves, into `docs/task-history/M4/`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (537 sealed file(s), each against its row and its sealing commit`, exit 0; `docs/tasks/M4.md` 66,644 bytes, `docs/tasks/` 670,865 in all.
    - [x] NO REGRESSION — `bash scripts/check_handoff_ledger.sh` → `handoff-ledger: OK`, exit 0, each hand-off's leaf still open; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1221 (leaf PROGRAM.131)`
