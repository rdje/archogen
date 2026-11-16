- ID: `PROGRAM.127.25`
  Status: `done` — `2026-11-07`: round 25, a fresh reader over round 24's answer: 8 findings, 5 defects, after 3, 3, 3 and 3, so answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12725/work/hook_forms.py "$PWD" target/p12725/work/forms1.txt` → `kill -0 -sKILL -1` and its like passed; `target/p12725/work/forms3.txt` → `pkill -P 123 -P 1 -x rustc` and its like passed.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'A later flag is what it sends' 9dda0da8 -- scripts/host_guard.py` → an exception read by a list of what undoes it, which is never whole; and two sentences of round 24's said of more than they hold.
  - [x] **FIX** — `scripts/host_guard.py`: each exception an exact form. `scripts/check_host_guard.sh`: the local settings. `scripts/run_self_tests.sh`: a gate's input. `scripts/selftest_host_guard.py`: each form, a `make` at each line. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 898 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them; among them each exact form's row, a small `make` left at the stop line and stopped at danger, and local settings refused only where they turn the hooks off.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1231 (leaf PROGRAM.127.25)`
