- ID: `PROGRAM.127.18`
  Status: `done` — `2026-11-06`: round 18, a fresh reader over round 17's answer: 7 findings, 2 defects, one mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4 and 1. Round 17 took from the self-test what spared it from a line. It is spared again by its own name, and is a job only to what `check` and `status` say. Dashless `ps` is read as dashed. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `ecd22a0b`, `target/p12718/work/`: `g.protected(Process(1,0,"python3 -I -B scripts/selftest_host_guard.py",…))` → `False`; `e1/driver.py … '{"panic_used_percent":0}' run case_what_holds_little_is_left_alone` → `rc=143`, no summary; `psforms2.py` → `ps U 501`, 519 lines, passes.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'GUARD_PROCESS.search' ecd22a0b -- scripts/host_guard.py` → one reading of a name served what spares a process and what tells a job, and round 17 moved it for the second alone.
  - [x] **FIX** — `scripts/host_guard.py`: `SELF_TEST_PROCESS`, dashless `ps`. `scripts/selftest_host_guard.py`: `case_the_self_test_is_spared_and_a_job`, `jobs_of`, the forms. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 764 passed, 0 failed, 0 not run`, exit 0, its stamp matching.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1206 (leaf PROGRAM.127.18)`
