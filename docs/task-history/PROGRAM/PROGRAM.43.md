- ID: `PROGRAM.43`
  Status: `done`
  Goal: the accepted catalog design moved out of `docs/decisions/` to a specification home, on the director's ruling
  of `2026-10-01`. The question put: "`docs/decisions/` is at 388 884 of its 393 216-byte ceiling (about 4 KB left),
  and that ceiling was your one-time raise. The next decision record won't fit. How should the folder make room?"
  The options: "Move accepted designs out", "Raise the cap again, reviewed", and "Hold the ceiling". The answer:
  "Move accepted designs out". The six records of `docs/decisions/catalog/`, the catalog design and the composition
  of the variant's inputs, both accepted (`M2.7.1`, `M2.10.1`), move to `docs/specs/catalog/`, which gets a routed
  row of its own; every reference follows them; the ruling is recorded as a decision.
  Acceptance: every gate green with the move; no reference left to the old folder outside sealed history; the
  decisions folder's room measured before and after; the crate's tests, which read the worked example by its path,
  still pass.
  Verification: see the checklist — the folder from 388 884 to 205 397 bytes with the ruling's own record; every gate
  green; the book built; 872 passed, 0 failed.
  Commit: `ARCHOGEN-PROGRAM-0259 (leaf PROGRAM.43)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git ls-files docs/decisions | xargs cat | wc -c` → 388 884, against a ceiling of
    393 216 that only a new ruling raises: the next decision record would not fit.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the folder held two accepted designs, 186 667 bytes of it, as decisions still
    being made: `git show HEAD:README_POLICY.md | grep -c "docs/decisions/catalog/"` → 2, the partition's row and
    its parent's onward cell. An accepted design is a specification its code is built against.
  - [x] **FIX** — `git mv docs/decisions/catalog docs/specs/catalog`; every live reference rewritten, 50 in 22
    files, the crate's `include_str!` paths among them, while closed narratives (`PROGRAM.39`'s move and its log
    row), the changelog's past entry and a dated note keep the path they describe; `docs/specs/` routed in
    `README_POLICY.md` and listed in `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`; the index lists the moved records by
    their new paths; the ruling recorded in `docs/decisions/decision_specifications-home.md`, and noted in the
    ceiling's record; the knowledge map regenerated.
  - [x] **ADDRESSED (verified)** — `git ls-files docs/decisions | xargs cat | wc -c` → 205 397, the ruling's record
    included; `git grep -n "decisions/catalog"` → only accounts of the past: `CHANGELOG.md`'s entry,
    `.doctrine/code_paths.txt`'s dated note, `PROGRAM.39`'s move and log row, the ceiling record's account of that
    partition, the new record's account of this move, and this leaf's own text; `cargo test -p archogen-catalog` → every result ok;
    `bash scripts/build_book.sh` → the book written.
  - [x] **NO REGRESSION** — `cargo test --all -q` → 872 passed, 0 failed; `bash scripts/check_doctrines.sh` →
    `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, `knowledge-map/subsystems.md` and
    `KNOWLEDGE_MAP.md`, `docs/decisions/INDEX.md`, the book's catalog and versions chapters, `MEMORY.md`,
    `CHANGELOG.md`.
