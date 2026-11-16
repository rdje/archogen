- ID: `PROGRAM.135`
  Status: `done` — `2026-11-10`, an open review history past its file's ceiling, `README-ROUTES` refusing the guard's round 36.
  Goal: let an open review history outgrow one file with nothing of it edited and its counts still judged whole, raising no ceiling.
  Acceptance:
    - [x] ROOT CAUSE — WHY: a review history only grows while its review is open, and the archive takes a history only once it closes (`REVIEW-HISTORY`), so an open review of many rounds had nowhere to go: the guard's, at 36 rounds, reached 133,177 bytes. WHERE: `bash scripts/check_doctrines.sh` on the guard's round 36, staged → `README-ROUTES: docs/reviews/: 133177 bytes in its largest file, over its ceiling of 131072`; and `git grep -n 'h, r = read(hist), read(rec)' 58b35022 -- scripts/check_review_counts.sh` → the check reading one file per history.
    - [x] FIX — `scripts/check_review_counts.sh`: a history is read with its parts, `<stem>.part-2.md` on, each after the one before it; three arms. `docs/reviews/INDEX.md` says how a history continues, and `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s row for the folder.
    - [x] ADDRESSED — `bash scripts/check_review_counts.sh --self-test` → `review-counts self-test: 159 pass / 0 fail (159 arms)`, 156 before; its three new arms on the committed check's reading → `157 pass / 2 fail`, the two that need a part read.
    - [x] NO REGRESSION — `bash scripts/check_review_counts.sh` → `review-counts: OK`, exit 0, on this tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
    - [x] LOCKSTEP — the check, the index, the containment table, the tree and the Commit Log. No ceiling moves.
  Verification: the check's own self-test, the gate.
  Commit: `ARCHOGEN-PROGRAM-1269 (leaf PROGRAM.135)`
