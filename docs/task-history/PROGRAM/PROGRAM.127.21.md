- ID: `PROGRAM.127.21`
  Status: `done` — `2026-11-06`: round 21, a fresh reader over round 20's answer: 10 findings, 3 defects, the count up from 2, so answered by method. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12721/work/hg.py` → `HOST-GUARD rc=0` for `focused: GUARD := true` and five others, each `make focused` exiting 0 with nothing compiled; `python3 -I -B target/p12721/work/make_stop_rig.py target/p12721/work/q3 400 0.15` → `makes started: 400; stopped by the watcher: 5`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'found.get(name) != recipe' 640b8c3f -- scripts/check_host_guard.sh` → a reading of the Makefile's text, where make's own reading decides; `git grep -n 'def reads_the_host' 640b8c3f -- scripts/host_guard.py` → a list of what a bracket may be.
  - [x] **FIX** — `scripts/check_host_guard.sh`: `make -n` and `make -pn` on the staged Makefile, the file loaded and asked its limits, a hook's type. `scripts/host_guard.py`: `says_no_job`, `what_a_run_runs` as `main` reads, `is_number`, `words_read` past a flag's value, `arch`, `xcrun` and `script`. `scripts/selftest_host_guard.py`: each form. The record's sentences and limits.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_host_guard.sh --self-test` → `host-guard self-test: 843 passed, 0 failed, 0 not run`, the stamp equal to what is staged; `python3 -I -B target/p12721/work/hg.py` → `HOST-GUARD rc=1` for each form that left `make focused` compiling nothing, `target_specific_guard`, `pattern_specific_guard`, `shell_true_after_whole`, `ignore_special`, `makeflags_i`, `two_targets_after`, `brace_eval` and `settings_type_prompt` among them.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1216 (leaf PROGRAM.127.21)`
