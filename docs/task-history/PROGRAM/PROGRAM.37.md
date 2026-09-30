- ID: `PROGRAM.37`
  Status: `done`
  Goal: `docs/decisions/` back under its total-bytes ceiling (327 680), by moving the review history still kept
  inside a decision record to `docs/reviews/`, the rule `PROGRAM.36` set.
  Why: on `2026-09-30`, `M2.7.1`'s seventh checkpoint staged §12 of the catalog record as a companion record, and
  `M2.10.1` had added the composition record. That took the directory to 331 050 bytes, measured by the staged
  `README-ROUTES` run that refused the commit. The largest review history still inside a decision is
  `decision_runtime-analysis-variant.md`'s `## Review`, 13 482 bytes, measured by section. Moving it pays the
  overrun with about 9.6 KB to spare. The ceiling is not raised.
  Acceptance: that section moved byte for byte to `docs/reviews/decision_runtime-analysis-variant-reviews.md`,
  indexed there, with a summary and a link left in the record; `docs/decisions/`' debt cell back to 327 680;
  `README-ROUTES` and the doctrine enforcer green.
  Verification: see the checklist — the move checked against `HEAD`; the directory 9 055 bytes under its ceiling.
  Commit: `ARCHOGEN-PROGRAM-0218 (leaf PROGRAM.37)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — the staged run of `ARCHOGEN-M2-0217`: `bash scripts/check_readme_routes.sh` → rc=1 with the
    cell at its ceiling, "`docs/decisions/: 331050 bytes in total, over its ceiling of 327680`", which that commit
    recorded as this leaf's debt.
  - [x] **ROOT CAUSE** — new decisions, not a stray file: the composition record (`M2.10.1`) and §12's companion
    (`M2.7.1`) added their bytes to a directory still holding a review history that `PROGRAM.36`'s rule had
    placed elsewhere. Measured by section with `awk` over every record, the variant's `## Review` was 13 482 bytes,
    the largest left; the next were 1 400 and 540 bytes, both of them summary tables.
  - [x] **FIX** — the section moved to `docs/reviews/decision_runtime-analysis-variant-reviews.md`, unchanged,
    under a header and a closing note like its siblings'. The record keeps a four-row summary and a link. The
    history is indexed as closed with `M2.6`, and the debt cell is 327 680 again.
  - [x] **ADDRESSED** — the moved text compared with `git show HEAD:docs/decisions/decision_runtime-analysis-variant.md`'s
    tail: identical. `bash scripts/check_readme_routes.sh` → rc=0, "`readme-routes: OK (20 destination(s)
    governed; recorded as debt, owned by: PROGRAM.32)`". `docs/decisions/` is 318 625 bytes over 28 files, and
    `docs/reviews/` 65 128 bytes over 4 files, longest line 600, all within their rows.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` (`SOURCE-LEDGER`, since
    the history names QEMU and carries its link; `MEMORY-ARCH`; `KNOWLEDGE-MAP` after regeneration). No Rust and
    no script changed.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` (both rows);
    `docs/reviews/INDEX.md`; this leaf, the frontier and both logs; `LIVE_STATUS.md`, `docs/TASK_TREE.md`,
    `MEMORY.md`, `KNOWLEDGE_MAP.md`, `CHANGELOG.md`.
