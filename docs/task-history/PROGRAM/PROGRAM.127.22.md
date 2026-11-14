- ID: `PROGRAM.127.22`
  Status: `done` — `2026-11-07`: round 22, a fresh reader over round 21's answer: 11 findings, 3 defects, 3 after 3, so answered by removing what kept failing. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `target/p12722/work/mk/try.sh a 'focused: SHELL := /usr/bin/true'` → `HOST-GUARD rc=0`, `make focused rc=0`, `ran.txt: ABSENT`; `target/p12722/work/mk/load.py`, a line set in `tightened` → `HOST-GUARD rc=0`, `status` saying 99.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'make_says' 15177496 -- scripts/check_host_guard.sh` → a tool asked of a text it can be told anything by; `git grep -n 'module.Limits()' 15177496 -- scripts/check_host_guard.sh` → the class asked, where `main` takes `tightened`.
  - [x] **FIX** — `scripts/check_host_guard.sh`: `MAKEFILE_SHA256` in place of asking `make`, `GNUmakefile` and `makefile` refused, `tightened({})` asked, any value at a call of `Limits` refused. `scripts/host_guard.py`: `taskpolicy`, `sandbox-exec`, `xcrun`'s asking flags, `PYTHON_FLAGS_WITH_VALUES`. `scripts/selftest_host_guard.py`: each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 858 passed, 0 failed, 0 not run`, the stamp equal to what is staged: `case_a_commit_holds_what_make_and_the_limits_can_be_told`, a target's own `SHELL`, a `BASH_ENV`, a `$(shell)` at parse that did not run, a comment, a `GNUmakefile`, a Makefile moved with its digest passing, and what `tightened` gives.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1218 (leaf PROGRAM.127.22)`
