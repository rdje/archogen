- ID: `PROGRAM.127.19`
  Status: `done` — `2026-11-06`: round 19, a fresh reader over round 18's answer: 4 findings, 2 defects, both mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4, 1 and 2. The self-test is spared by its own path, a space in it or not; what a floor counts of a process newly ours is said. The findings and each answer are in `docs/reviews/decision_host-guard-reviews.md`.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reader's rigs on `e9818964`, `target/p12719/work/`: `rig_spared.py`, `sp ace` → `ours: 1 process(es)`, stopped, `rc=-15`; `rig_arrives.py` → a process that writes nothing now, stopped 0.1 s after it came here.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -n 'SELF_TEST_PROCESS = ' e9818964 -- scripts/host_guard.py` → a pattern with no space in it, where `GUARD_PROCESS` carries its own path; `git grep -n 'self.written.get(p.pid, 0)' e9818964 -- scripts/host_guard.py` → a first sight counts its whole life.
  - [x] **FIX** — `scripts/host_guard.py`: `SELF_TEST_PROCESS` by its path, a comment. `scripts/selftest_host_guard.py`: the case. The record, the review history and its index.
  - [x] **ADDRESSED (verified)** — `python3 -I -B scripts/selftest_host_guard.py` → `host-guard self-test: 765 passed, 0 failed, 0 not run`, exit 0, its stamp matching.
  - [x] **NO REGRESSION** — `bash scripts/check_host_guard.sh` → `host-guard: OK`, exit 0, on the staged tree; `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree, through the guard.
  - [x] **LOCKSTEP** — the record, the review history and its index, these leaves and the Commit Log; `MEMORY.md`'s next action.
  Commit: `ARCHOGEN-PROGRAM-1211 (leaf PROGRAM.127.19)`
