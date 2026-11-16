- ID: `PROGRAM.139`
  Status: `done` — `2026-11-14`, the live task trees over their ceiling again, `README-ROUTES` refusing format `/2`'s round 12.
  Goal: bring `docs/tasks/` under its ceiling by the project's own sealing, nothing summarised and nothing lost.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the review rounds' closed leaves in `docs/tasks/M4.md`, format `/2`'s rounds 7 to 11 and the application inputs' 8 to 12, had grown it since `PROGRAM.134` sealed it. WHERE: `bash scripts/check_doctrines.sh` on format `/2`'s round 12, staged → `README-ROUTES: docs/tasks/: 820961 bytes in total, over its ceiling of 819200`.
    - [x] FIX — `bash scripts/check_task_history.sh --seal M4`, on the committed tree, that round's work set aside: 10 closed subtrees, 10 leaves, into `docs/task-history/M4/`, 10 log rows into `rows-0005.md`, each with its row in the index and a stub in the tree.
    - [x] ADDRESSED — `task-history: OK (563 sealed file(s), each against its row and its sealing commit`, "the reconstruction is byte for byte"; `docs/tasks/` 768,673 bytes in all.
    - [x] NO REGRESSION — `bash scripts/check_doctrines.sh` on the staged tree → exit 0. No code changed.
    - [x] LOCKSTEP — the tree, the index and the Commit Log.
  Verification: the seal's own proof, the gate.
  Commit: `ARCHOGEN-PROGRAM-1287 (leaf PROGRAM.139)`
