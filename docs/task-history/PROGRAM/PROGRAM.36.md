- ID: `PROGRAM.36`
  Status: `done`
  Goal: `docs/decisions/` back under its total-bytes ceiling (327 680, `README_POLICY.md`'s routed destinations), by
  giving the append-only review histories a home of their own rather than by raising the ceiling.
  Why: on `2026-09-30`, `M2.7.1`'s fifth checkpoint took the directory to 329 428 bytes, measured by the staged
  `README-ROUTES` run that refused that commit. The largest growth there is review history: rounds of an independent
  review, appended and never edited, which the policy's table classes as append-only history, not as a partitioned
  canonical record. The policy's route for a destination over its ceiling is "Record its current measured ceiling
  as debt, stop further growth there, and open a separately owned partition/compaction task". Raising the ceiling
  "needs the same explicit review as raising the README cap". This leaf is that task, and the total is recorded as
  debt it owns.
  Acceptance: the review histories (`decision_catalog-records-reviews.md` first) moved byte for byte to a destination
  with append-only controls, registered in `README_POLICY.md` with its own ceilings and lifecycle; every link to
  them and the knowledge map updated; the debt cell back to a number the directory is under; `README-ROUTES` and the
  doctrine enforcer green. Until it closes, `docs/decisions/` grows only by what `M2.7.1`'s open review adds, and this
  leaf goes before the next round is answered.
  Verification: see the checklist — the breach reproduced and gone; the file moved with its digest unchanged; 20
  destinations governed; the book built.
  Commit: `ARCHOGEN-PROGRAM-0211 (leaf PROGRAM.36)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — with the debt cell set back to the ceiling, `bash scripts/check_readme_routes.sh` → rc=1,
    "`docs/decisions/: 329428 bytes in total, over its ceiling of 327680`"; the file restored after.
  - [x] **ROOT CAUSE** — a design's review history was kept in `docs/decisions/`, a partition of records, though it
    is append-only history that grows a round at a time. `git show HEAD:docs/decisions/decision_catalog-records-reviews.md | wc -c`
    → 36 283 of the directory's 329 428 bytes, the third-largest file there. The example moved out of the design
    record in `ARCHOGEN-M2-0210` was only what tipped it over.
  - [x] **FIX** — the file moved to `docs/reviews/`, unchanged (`git mv`, a 100 % rename). There, an index lists
    each review history with its design record, its rounds and its status, and says how a round is added and when a
    history is frozen. `README_POLICY.md` registers `docs/reviews/` as `docs/decisions/`' overflow, with ceilings
    from the measurement plus headroom (16 files; 1 200 lines, 131 072 bytes and 1 024-byte lines a file; 262 144 in
    total), and `docs/decisions/`' total is a number again. Links from the design record, `M2.7.1` and the decisions
    index follow the file.
  - [x] **ADDRESSED** — `bash scripts/check_readme_routes.sh` → rc=0, "`readme-routes: OK (20 destination(s)
    governed; recorded as debt, owned by: PROGRAM.32)`": `docs/decisions/` at 292 983 bytes, 34 697 under its
    ceiling, and `docs/reviews/` at 37 295. `git show HEAD:docs/decisions/decision_catalog-records-reviews.md | shasum -a 256`
    and `shasum -a 256 docs/reviews/decision_catalog-records-reviews.md` → both `ece719ec…fd5e39`.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` (`MEMORY-ARCH`'s
    index check, `BOOK-ANCHORS`, `KNOWLEDGE-MAP` after regeneration, `LIVE-SNAPSHOTS`); `mdbook build docs/book` →
    rc=0, the edited chapter naming `docs/reviews/`. No Rust and no script changed. Not mechanized: that a review
    history is only appended to. That needs a script, and the director has asked for `scripts/` to be left alone for
    now, so it stays a convention, as it was in `docs/decisions/`.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` (both rows);
    `docs/book/src/verification.md`, whose routing paragraph still called the changelog and the development notes
    unbounded, stale since `PROGRAM.31` and corrected here; `docs/decisions/INDEX.md`, `docs/reviews/INDEX.md`; this
    leaf, the frontier and both logs; `M2.7.1`'s close-out; `LIVE_STATUS.md`, `docs/TASK_TREE.md`, `MEMORY.md`,
    `KNOWLEDGE_MAP.md`, `CHANGELOG.md`.
