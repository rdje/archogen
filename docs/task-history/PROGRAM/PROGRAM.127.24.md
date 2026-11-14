- ID: `PROGRAM.127.24`
  Status: `done` — `2026-11-07`: round 24, a fresh reader over round 23's answer: 6 findings, 3 defects, each narrow. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `bash target/p12724/work/nested.sh` → `nested run --wait 0: STARTED-BESIDE-THIS-ROOTS-BUILD`; `bash target/p12724/work/makegate.sh` → `run` alone `rc=0`, `make gate` `rc=143`; `python3 -I -B target/p12724/work/zero.py` → the hook `rc=0` on `kill -0 -s KILL -1`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n "this root's lines and turn hold it" 612fd4ac -- docs/decisions/decision_host-guard.md` → a sentence the code never held; `git grep -n 'command.words\[1\] == "-0"' 612fd4ac -- scripts/host_guard.py` → an exception read from one flag.
  - [x] **FIX** — `scripts/host_guard.py`: a later flag undoes `kill -0` and `bash -n`. `scripts/check_host_guard.sh`: local settings. `scripts/selftest_host_guard.py`: each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 878 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `kill -0 -s KILL -1` and `bash -n +n -c 'pkill y'` refused, `kill -0 -1` and `bash -n -c 'pkill y'` not; local settings turning every hook off refused, and any others not.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1226 (leaf PROGRAM.127.24)`
