- ID: `PROGRAM.38`
  Status: `done`
  Goal: `docs/decisions/`' ceiling raised once, to 40 files and 393 216 bytes, as the director ruled on
  `2026-09-30` ("Raise, reviewed"), by a decision an independent context reviewed before it landed.
  Why: the folder was at 324 165 of 327 680 bytes, the next catalog answers needed about 5 KB, and a partition waits
  on a project gate the held template checks make necessary (findings §10).
  Acceptance: `docs/decisions/decision_decisions-folder-ceiling.md` reviewed and every finding answered; the row, the
  inventory row, the index rows and the knowledge map in one commit; `PROGRAM.39` opened; `README-ROUTES` and the
  doctrine enforcer green.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0222 (leaf PROGRAM.38)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — at `HEAD` `7a628a0`, `git ls-files -z docs/decisions | xargs -0 cat | wc -c` → 324 165 over
    28 files. With the record untracked in the working tree the reviewer measured 327 974, over the ceiling before
    its index row.
  - [x] **ROOT CAUSE** — measured with `git ls-tree -r -l 38d8634 docs/decisions`: the ceiling was derived at
    `38d8634` from 23 files and 250 243 bytes. Since then two normative designs under open review reached 136 093
    bytes over four records, against one record of 72 269, after their histories moved out. No append-only
    content was left to move except the findings register's settled items (about 9.7 KB), which the reviewer found.
  - [x] **FIX** — the record, rewritten against its review's 18 findings, and the row with 40 files, 393 216 bytes
    and owner `PROGRAM.39`. The inventory row, both index rows, the review history in `docs/reviews/`, and
    `PROGRAM.39` land with it.
  - [x] **ADDRESSED** — `bash scripts/check_readme_routes.sh` → rc=0 with the row at its new ceilings; the folder,
    measured with this change staged, 332 029 bytes over 29 files, 61 187 bytes and 11 files under them.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` (`MEMORY-ARCH` finds the
    new record in the index; `KNOWLEDGE-MAP` after regeneration). No Rust and no script changed.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, `docs/decisions/INDEX.md`,
    `docs/reviews/INDEX.md`; this leaf, `PROGRAM.39`, the frontier and both logs; `LIVE_STATUS.md`,
    `docs/TASK_TREE.md`, `MEMORY.md`, `KNOWLEDGE_MAP.md`, `CHANGELOG.md`.
