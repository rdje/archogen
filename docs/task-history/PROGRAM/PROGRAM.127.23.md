- ID: `PROGRAM.127.23`
  Status: `done` — `2026-11-07`: round 23, a fresh reader over round 22's answer: 7 findings, 3 defects, 3 after 3 and 3, each narrow, so answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash target/p12723/work/xcrun_real.sh` → `xcrun -k stubtool arg => RAN stubtool arg`, and the hook passing `xcrun -k kill -9 -1`; `python3 -I -B target/p12723/work/rig_flake.py target/p12723/work/r3 25` → 1 of 25 for each race.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'What these ask of it runs nothing' 893de281 -- scripts/host_guard.py` → round 22's own exception, from what a flag was assumed to do; `git grep -n 'os.listdir(root)' 893de281 -- scripts/check_host_guard.sh` → the tree read at a commit.
  - [x] **FIX** — `scripts/host_guard.py`: the exception gone, `taskpolicy`'s flags, both patterns, `-W` and `-X` Python's alone. `scripts/check_host_guard.sh`: the index read. `scripts/selftest_host_guard.py`: `jobs_of`, `status` waited for, each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 870 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `xcrun -k kill -9 -1` and `xcrun -f pkill` refused, `taskpolicy`'s four flags read, a `GNUmakefile` staged and gone from the tree refused, one in the tree alone refusing nothing, a guard after `-W ignore` the guard, `perl -W codex.pl` its script.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1222 (leaf PROGRAM.127.23)`
