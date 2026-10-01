- ID: `PROGRAM.32`
  Status: `done` — unblocked `2026-09-30` by the director's ruling on §10 of
  `docs/decisions/decision_findings-for-director-review.md`: the hold covers only the 17 template files archogen has
  not changed. The new check, and the change to `TASK-ACCEPTANCE` (`check_task_acceptance.sh`, changed by
  `PROGRAM.21`), are archogen's. The held checks that read leaves must keep working unchanged: `WAIVER-ROUTING`,
  `GAP-CLAIM-CENSUS`, `TASK-TREE-OWNERSHIP` and `LIVE-DOC-CURRENCY`.
  Goal: seal closed leaves out of the task trees — each `done` leaf's body moved byte for byte to a sealed per-subtree
  file with its digest, the tree keeping one line per closed leaf with its commit and a link; `M1` and `PROGRAM` first.
  Unblocked `2026-09-30` by the ruling on §8 of `docs/decisions/decision_findings-for-director-review.md` (option C),
  which follows `PROGRAM.31`.
  Acceptance (if accepted): `TASK-ACCEPTANCE` and every gate that reads a leaf still finds it (sealed text included);
  a check proving sealed bodies unchanged; the tree's live graph complete; no line lost.
  Children: `PROGRAM.32.1` … `PROGRAM.32.4` — decomposed `2026-09-30`, after a read-only audit of every check that
  reads a leaf.
  Verification: through its children — the design, the tool, the seal of `M1` and `PROGRAM`, and an independent
  review whose findings hardened the tool.
  Commit: `ARCHOGEN-PROGRAM-0228` closes it; its children's commits are in the log below.

- ID: `PROGRAM.32.1`
  Status: `done`
  Goal: the design, decided before any code: `docs/decisions/decision_task-tree-sealing.md`.
  - The unit is a closed subtree.
  - Its leaves move byte for byte to `docs/task-history/<TREE>/<SUBTREE>.md`, each leaving a two-line stub.
  - An append-only index carries each file's digest.
  - A reconstruction proof runs before anything is written, and the gate `TASK-HISTORY` runs after.
  - The destination is outside `docs/tasks/` so that no held check mistakes moved text for new text.
  Verification: the audit of `2026-09-30`, quoted in the record's "Why", measured each check against a sealing
  commit, including `M1` and `PROGRAM`'s closed subtrees: 37 of 39 and 32 of 39.
  Commit: `ARCHOGEN-PROGRAM-0224 (leaf PROGRAM.32.1)`

- ID: `PROGRAM.32.2`
  Status: `done`
  Goal: `scripts/check_task_history.sh`: the gate, `--seal <TREE>` with its reconstruction proof, and `--self-test`
  with RED arms for each of the gate's four checks. It is registered as `TASK-HISTORY` in
  `scripts/check_doctrines.project.sh`. `TASK-ACCEPTANCE`'s refusal names a sealed owner as closed.
  Acceptance: every arm refused and then passed; the spine's own self-tests green; no held file changed.
  Verification: see the checklist — 14 arms, a dry run on real copies of `M1` and `PROGRAM`, and a mutation
  that turns each new rule's arm red.
  Commit: `ARCHOGEN-PROGRAM-0225 (leaf PROGRAM.32.2)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — the need, measured before the tool: 70 of `M1`'s 74 leaves and 54 of `PROGRAM`'s 63 were
    `done`, 77% and 79% of the files (`PROGRAM.32.1`'s audit). No check held a sealed file to its digest, because
    none existed.
  - [x] **ROOT CAUSE** — nothing moved finished work out of a tree, and a hand move would lose a byte unseen. The
    audit (`git grep -n "docs/tasks" scripts/`, each hit read) showed what a sealing tool must respect:
    - `TASK-ACCEPTANCE` read a stub as a leaf lacking boxes;
    - `LESSON-PROMOTION` and `WAIVER-ROUTING` judge anything under `docs/tasks/` as added;
    - `docs/tasks/`' 20-file ceiling.
  - [x] **FIX** — `scripts/check_task_history.sh`: the gate's four legs; `--seal <TREE>`, which proves the
    reconstruction before it writes; and `--self-test`. It is registered as `TASK-HISTORY` in
    `scripts/check_doctrines.project.sh`. `scripts/check_task_acceptance.sh` names a sealed owner as closed, with
    arm 10. The rows are in `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md`.
  - [x] **ADDRESSED** — the self-test and the dry run:
    - `bash scripts/check_task_history.sh --self-test` → rc=0, `task-history self-test: 14 pass / 0 fail (14 arms)`;
    - `bash scripts/check_task_acceptance.sh --self-test` → `task-acceptance self-test: 10 pass / 0 fail`;
    - a dry run on copies of the real trees in `target/doctrine_scratch/task_history/dryrun`:
      - `--seal M1` → "sealed 37 subtree(s), 62 leaves … the reconstruction is byte for byte";
      - `--seal PROGRAM` → "sealed 32 subtree(s), 50 leaves …";
      - the gate → `task-history: OK (69 sealed file(s) … 112 stub(s) …)`, rc=0;
      - `M1.md` went from 717 804 to 259 334 bytes, `PROGRAM.md` from 381 216 to 107 157;
      - a real code change owned by a sealed leaf was refused there with "is closed and sealed".
  - [x] **NO REGRESSION** — the self-tests, the enforcer and two mutations:
    - `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 35 self-test(s) passed`;
    - `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` with this leaf staged;
    - `bash -n` on both scripts → syntax OK;
    - mutations: with the stub-to-file check disabled, the self-test goes to `13 pass / 1 fail`; with the sealed
      branch removed from `TASK-ACCEPTANCE`, arm 10 goes red, `9 pass / 1 fail`.

    No held file changed: `git diff --cached --name-only` names none of the 17 in findings §10.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `scripts/check_doctrines.project.sh`; this leaf,
    the frontier and both logs; `CHANGELOG.md`. The book's chapter follows with the real seal (`PROGRAM.32.3`).

- ID: `PROGRAM.32.3`
  Status: `done`
  Goal: `M1` and `PROGRAM` sealed. `docs/task-history/` gets its row, and `docs/tasks/`' debt cells become measured
  ceilings. The inventory, the book's verification chapter and `docs/TASK_TREE.md` are updated.
  Acceptance: the reconstruction proven byte for byte for both trees; every check green in the sealing commit; the
  trees' live leaves, frontier and logs unchanged.
  Verification: see the checklist — the seal's own proof, then an independent re-derivation of both trees and of all
  69 digests.
  Commit: `ARCHOGEN-PROGRAM-0226 (leaf PROGRAM.32.3)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — at `HEAD` `15c61b4`, `wc -lc docs/tasks/M1.md docs/tasks/PROGRAM.md` → 7 633 lines and
    717 804 bytes, and 381 216 bytes. `README-ROUTES` carried four `debt: PROGRAM.32` cells for `docs/tasks/`.
  - [x] **ROOT CAUSE** — closed work was never moved out of a tree, the finding of §8 of the findings record.
  - [x] **FIX** — the seal:
    - `bash scripts/check_task_history.sh --seal M1` → "sealed 37 subtree(s), 62 leaves … the reconstruction is
      byte for byte";
    - `--seal PROGRAM` → "sealed 32 subtree(s), 50 leaves …";
    - this leaf's own subtree, `PROGRAM.32`, stays live, since `.32.4` is open.

    Registration:
    - `README_POLICY.md`'s `docs/tasks/` row takes measured ceilings in place of its debt, and overflows to
      `docs/task-history/`, which has its own `archive_terminal` row;
    - the inventory rows, the book's new section, and `docs/TASK_TREE.md`'s header are updated.
  - [x] **ADDRESSED** — the counts and an independent re-derivation:
    - `wc -lc` → `M1.md` 2 278 lines and 259 334 bytes; `PROGRAM.md` 110 662 bytes;
    - an independent re-derivation, separate code over `git show HEAD:docs/tasks/<TREE>.md`, expanded every stub
      from its sealed file: "M1 independent reconstruction equals HEAD: True", and the same for `PROGRAM`;
    - `shasum -a 256` over all 69 sealed files against their rows: "69 rows, 0 mismatched";
    - `bash scripts/check_readme_routes.sh` → rc=0, `readme-routes: OK (21 destination(s) governed)`, with no debt
      left.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` in the sealing commit,
    the four held checks that read leaves among them. The frontier tables, the verification and commit logs and every
    open leaf are untouched, since the reconstruction is byte for byte. `STATED-ORDER` reads the stubs' status lines.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, `docs/book/src/verification.md`,
    `docs/TASK_TREE.md`; this leaf, the frontier and both logs; `LIVE_STATUS.md`, `MEMORY.md`, `CHANGELOG.md`.
  ⚠️ *Corrected by `PROGRAM.32.4`.* Some figures above were not taken at the commits they name:
  - `PROGRAM.md` was 384 721 bytes and 4 280 lines at `15c61b4`; 381 216 had been measured at `8b9a770`;
  - at `940baf1` it was 978 lines and 113 467 bytes, since the 948 lines and 110 662 bytes were measured before
    this leaf's own closure edits;
  - the commit changed neither `KNOWLEDGE_MAP.md` nor `MEMORY.md`, which its message and the list above name.

- ID: `PROGRAM.32.4`
  Status: `done`
  Goal: an independent read-only review of the tool and the sealed result, every finding answered.
  Verification: see the checklist. The review accepted the seal, "correct and lossless", and asked for the tool to
  be hardened before another tree is sealed. That is done: 25 RED arms, and the real history passes every leg.
  Commit: `ARCHOGEN-PROGRAM-0228 (leaf PROGRAM.32.4)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — the reviewer's constructions, each rebuilt as an arm in `scripts/check_task_history.sh`'s
    self-test, refused none of them before this change:
    - a seal made by hand of `T.2.1` from its open subtree;
    - a body edited together with its row;
    - a stub moved into another tree;
    - a forged file and row, committed.

    A leaf holding a column-0 fence was sealed torn.
  - [x] **ROOT CAUSE** — three designs of the first tool:
    - its gate compared with `HEAD` only, which is the commit itself in CI;
    - it never re-derived a sealed leaf from the tree it came from;
    - its seal sliced by lines without refusing what the slicing cannot hold.

    The first, shown by `git show 940baf1:scripts/check_task_history.sh | grep -n head_index`:
    ```text
    296:  if git cat-file -e "HEAD:docs/task-history/INDEX.md" 2>/dev/null; then
    297:    git show "HEAD:docs/task-history/INDEX.md" > "$SCRATCH/head-index.md" && printf '%s' "$SCRATCH/head-index.md"
    ```
    It read only `HEAD`'s index. The other two, by the review's 25-case mutation run, rebuilt here as arms.
  - [x] **FIX** — the gate's legs:
    - leg 3 is history-wide: every row any committed index held, and every sealed file against the commit that
      added it;
    - leg 4 ties each stub to its tree and each leaf to its subtree's file;
    - leg 5 re-proves every sealed leaf against its tree just before its seal;
    - leg 6 refuses a live leaf in a sealed subtree.

    The seal refuses a leaf with a column-0 line after its `ID`, and rolls back if the gate refuses what it wrote.
    Unreadable input is a named breach. The fallback commit text is checked against the Commit Log. The decision
    record, the book and the inventory now match the build and the commits, and the review history is in
    `docs/reviews/`.
  - [x] **ADDRESSED** — `bash scripts/check_task_history.sh --self-test` → rc=0, `task-history self-test: 25 pass /
    0 fail (25 arms)`. `bash scripts/check_task_history.sh` on the real repository → rc=0, "69 sealed file(s), each
    against its row and its sealing commit … 112 stub(s) … every sealed leaf proven against its tree before its
    seal".
  - [x] **NO REGRESSION** — the mutations, the enforcer and the self-tests:
    - with leg 5 disabled, the self-test goes to `23 pass / 2 fail`;
    - with leg 3's file check disabled, it goes to `24 pass / 1 fail`;
    - `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`;
    - `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 35 self-test(s) passed`.

    No held file changed. `HISTORY-LEDGERS` shares leg 3's old blind spot, and `PROGRAM.40` owns it.
  - [x] **LOCKSTEP** — `docs/decisions/decision_task-tree-sealing.md`, `docs/reviews/`, `DOCTRINE_ENFORCEMENT.md`,
    `docs/book/src/verification.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`; `PROGRAM.32.3`'s corrected figures; the
    frontier and both logs; `LIVE_STATUS.md`, `docs/TASK_TREE.md`, `MEMORY.md`, `CHANGELOG.md`.
