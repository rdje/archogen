- ID: `PROGRAM.127.17`
  Status: `done` — `2026-11-06`: round 17, a fresh reader over round 16's answer: 8 findings, 1 defect, mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4 and 4. A script whose name merely ends in this file's is a job like any, so `check` and `status` say what `run` would beside it. `-d` selects and `-M` takes no value. What a commit holds says where the check itself stands. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rig on `86b587c4`, `target/p12717/work/`: `S/my_host_guard.py` sleeping under `run --root S` → `check` says `clear`, `run --wait 0` refuses, rc 75, hook on `cargo build` rc 0; `ps -dp 1 | wc -l` → 91, hook rc 0; `ps -M "$PID"; kill "$PID"` → hook rc 2; `imp.py` → a copy whose repository has no `scripts/` raises `FileNotFoundError`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'S\*host_guard' 86b587c4 -- scripts/host_guard.py` → any name ending in this file's matched, the self-test's among them.
  - [x] **FIX** — `scripts/host_guard.py`: this file's name by a path's last part, `-d`, `-M`. `scripts/selftest_host_guard.py`: `case_a_job_named_like_the_guard_is_a_job`, the forms, `leave_the_stamp`'s `try`. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 756 passed, 0 failed, 0 not run`, exit 0, its stamp matching.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1202 (leaf PROGRAM.127.17)`
