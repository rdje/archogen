- ID: `PROGRAM.39`
  Status: `done` — started at 358 185 bytes, before the warning was crossed, since the next review answers would
  cross it
  Goal: `docs/decisions/` partitioned into sub-folders by subject, with no capacity added, as
  `decision_decisions-folder-ceiling.md` requires.
  Acceptance:
  - each sub-folder has a row, reachable through the parent's "Overflows to";
  - `README-ROUTES` leaves out of a parent's figures what a deeper row governs. It refuses a set of rows whose totals
    exceed 40 files or 393 216 bytes, and a ceiling above the one a decision names, with RED arms for each;
  - a project gate extends index completeness to sub-folders, since the held template check reads the folder flat;
  - `knowledge-map/subsystems.md` links each sub-folder's index;
  - the findings register's settled items considered for sealing, with stubs kept for their section numbers;
  - the first candidates are the catalog design's four records.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0230 (leaf PROGRAM.39)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — at `d64dc8d`, `git ls-files -z docs/decisions | xargs -0 cat | wc -c` → 358 185 over 30
    files, 1 815 bytes under the 360 000 warning. Composition round 4 and catalog round 11 were back with answers
    that would add more.
  - [x] **ROOT CAUSE** — the folder holds four records of one design under open review, 151 370 bytes, which grow
    a round at a time. Two held template checks read the folder flat, `git show HEAD:scripts/check_memory_architecture.sh`
    at line 41 (`for f in docs/decisions/*.md`) and the map generator at line 42, and `README-ROUTES` counted a
    sub-folder into its parent, so no partition had been possible without losing a check.
  - [x] **FIX** — the partition, the checks that make it safe, and the records around it:
    - the four records `git mv`d to `docs/decisions/catalog/`, with every path to them rewritten outside the sealed
      histories, found by `git grep`;
    - `scripts/check_decision_index.sh`, registered as `DECISION-INDEX`;
    - `README-ROUTES`: a partition's files meet their own per-file ceilings, the parent's count and total still
      count them, and a new table, `### Ceilings a decision fixes`, is enforced;
    - the partition's row, and the cap table with the folder's 40 files and 393 216 bytes;
    - `knowledge-map/subsystems.md` links the partition;
    - the findings register's settled items considered and left, with the reason in the ceiling record.
  - [x] **ADDRESSED** — the checks, the self-tests and the size:
    - `bash scripts/check_decision_index.sh` → rc=0, "28 record(s)", four of them in the partition;
    - `bash scripts/check_readme_routes.sh` → rc=0, `readme-routes: OK (22 destination(s) governed)`;
    - `--self-test`: `decision-index self-test: 5 pass / 0 fail (5 arms)`, and
      `readme-routes self-test: 20 pass / 0 fail (20 arms)` with its four new arms (a partition's per-file limits,
      no capacity added, a cap above its decision, a cap's missing decision);
    - the folder, the partition included, is 358 642 bytes over 30 files, within the cap.
  - [x] **NO REGRESSION** — `cargo test -q -p archogen-evidence` → `test result: ok. 34 passed; 0 failed`, since a
    comment in `sha256.rs` names the moved path. `bash scripts/run_self_tests.sh` → rc=0,
    `self-tests: OK — 36 self-test(s) passed`. `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`,
    `MEMORY-ARCH`, `KNOWLEDGE-MAP`, `SOURCE-LEDGER` and `BOOK-ANCHORS` among them. No held file changed.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`,
    `docs/book/src/verification.md`, `docs/book/src/versions.md`; `docs/decisions/decision_decisions-folder-ceiling.md`;
    `docs/decisions/INDEX.md`, `docs/reviews/INDEX.md`; `docs/tasks/M2.md`; this leaf, the frontier and both logs;
    `LIVE_STATUS.md`, `docs/TASK_TREE.md`, `MEMORY.md`, `KNOWLEDGE_MAP.md`, `CHANGELOG.md`.
