- ID: `PROGRAM.127.20`
  Status: `done` — `2026-11-06`: round 20, a fresh reader over round 19's answer: 10 findings, 2 defects. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12720/work/e31/rig.py target/p12720/work/e31 420` → `make ended rc=-15` at 10, 137 and 238 s; `python3 -I -B target/p12720/work/e35/rig.py …` → `717 samples, 5 with the queued make read as a build`; `bash target/p12720/work/e3/rig.sh …` → another root's `run` stopped, `rc=143`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'process.name in READS_THE_HOST' ee53e991 -- scripts/host_guard.py` → `(sysctl)` is no name of a tool; `git grep -n 'def names_root' ee53e991 -- scripts/host_guard.py` → the whole command line searched.
  - [x] **FIX** — `scripts/host_guard.py`: `unread_name`, `reads_the_host`, `names_root`, `words_read`, `what_a_run_runs`, `GUARD_NAME`, the number's spelling, `lsof` and `top`, `env -`. `scripts/check_host_guard.sh`: what make and the limits can be told. `scripts/selftest_host_guard.py`: five cases. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 808 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `case_what_ps_cannot_read_says_no_job`, `case_a_job_naming_a_root_is_not_that_root_s`, `case_a_kill_is_read_through_what_hides_it`, `case_a_path_holding_a_space_is_one_path`, `case_a_commit_holds_what_make_and_the_limits_can_be_told`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1213 (leaf PROGRAM.127.20)`
