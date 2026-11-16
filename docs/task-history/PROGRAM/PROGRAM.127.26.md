- ID: `PROGRAM.127.26`
  Status: `done` — `2026-11-07`: round 26, a fresh reader over round 25's answer: 7 findings, 2 defects, after 3, 3, 3, 3 and 5, each a false sentence, mild, neither in ordinary work on this host. Nothing failed open in ordinary work, and round 25's exact forms held. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `python3 -I -B target/p12726/work/spared.py` → `bash -lc cd /x/codex && cargo build` spared as `codex`; `target/p12726/work/once.py` → `python3 prog /x/claude` left at a line that stops every job.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'if word in ("-m", "-c")' b8160d0b -- scripts/host_guard.py` → a search that ran on past what runs, with two exact flags to stop it, the shape round 25 answered in the reader's exceptions.
  - [x] **FIX** — `scripts/host_guard.py`: `programs_named`, the hook's reading of `kills_by_name`, `unseen`. `scripts/check_host_guard.sh`: what stands read as what is staged. `scripts/selftest_host_guard.py`: rows for each. The record's sentences.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 905 passed, 0 failed, 0 not run`, on these bytes, the stamp holding them; among them each form of `G26-1` a row, a too-deep command through a `run` refused, bytes that are no text in local settings, and a line of 200,000 `[` read past by the hook and `status`.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0; `bash scripts/run_self_tests.sh --self-test` → `run-self-tests self-test: 18 pass / 0 fail (18 arms)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1236 (leaf PROGRAM.127.26)`
