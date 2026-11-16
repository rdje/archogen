- ID: `PROGRAM.125`
  Status: `done` — P3, filed `2026-11-04` by `M4.2.2`, which found the map had no row for its crate, and closed the same day:
    eleven crates had none in `knowledge-map/subsystems.md`, whose head says a crate's row appears when the crate does, and
    nothing asked for one.
  Built: a row saying what each is for: `archogen-api`, `archogen-catalog`, `archogen-digest`, `archogen-install`,
    `archogen-json`, `archogen-outcome`, `archogen-plan-check`, `archogen-plan-check-wasm`, `archogen-transport`,
    `archogen-wasm` and `eadl-resolve`. `BOOK-COVERAGE`, whose population is already the workspace's, also refuses a member
    with no row: it must begin a line, so a mention in another row's prose or a longer crate's name is none.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git grep -c '^- .crates/eadl-resolve/' 64c39016 -- knowledge-map/subsystems.md` → no match, exit 1: the search, the engine's largest crate, had no row, and neither had ten others.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `git grep -c 'subsystems' 64c39016 -- scripts/check_book_coverage.sh` → no match, exit 1, and `KNOWLEDGE-MAP` holds the map to its sources alone: nothing compared those sources with the workspace.
  - [x] **FIX** — `scripts/check_book_coverage.sh` and its arms; the rows in `knowledge-map/subsystems.md` and the map regenerated; the doctrine's lines in `scripts/check_doctrines.project.sh`, `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md`; `docs/book/src/annex-repository.md`.
  - [x] **ADDRESSED (verified)** — before the rows, `bash scripts/check_book_coverage.sh` → `BOOK-COVERAGE: 11 of 23 workspace member(s) with no row`, exit 1; after them `book-coverage: OK (23 workspace member(s), …)`, exit 0; `bash scripts/check_book_coverage.sh --self-test` → `book-coverage self-test: 12 pass / 0 fail (12 arms)`.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`, on the staged tree; no Rust source changed.
  - [x] **LOCKSTEP** — the book's annex, the doctrine's register, the map, this leaf and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1135 (leaf PROGRAM.125)`
