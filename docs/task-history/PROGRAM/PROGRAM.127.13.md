- ID: `PROGRAM.127.13`
  Status: `done` — `2026-11-06`: round 13, a fresh reader over round 12's answer: 11 findings, 5 defects. Round 12's leaving opened a moment with nothing watching. It is closed: a watcher takes itself up from the file as it stands, unless a save can be seen to have broken it, with no proof. The sentences on a broken guard, two minutes and blindness now say what happens. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `ee72c8e0`, `target/p12713/work/`: `e8_clean.py` → `save, then the next command at once: hook rc 0`, then `T+30: job alive True | watcher: not running`; `e1_saves.py` → guard emptied: `watcher left after 0.93 s`, `hook on 'pkill -x rustc': rc 0`; `e2_broken.py` → `run` ended at T+122; `e4_blind.py` → `status`: `BLIND`, `check`: `clear`, hook on `cargo build` rc 2.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'if code_stamp() not in (None, running)' ee72c8e0 -- scripts/host_guard.py` → a watcher that leaves, with nothing to bring the next but a command that may already have looked.
  - [x] **FIX** — `scripts/host_guard.py`: `file_is_whole`, a watcher taking itself up, `ensure`'s `PATH`, `run` starting one as it waits and with its job, `kill -1`, a plain pid to `ps`. `scripts/run_self_tests.sh`. `scripts/selftest_host_guard.py`: `case_a_guard_changed_holds_from_then`, the forms. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 657 passed, 0 failed, 0 not run`, exit 0, no `scripts/__pycache__`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1189 (leaf PROGRAM.127.13)`
