- ID: `PROGRAM.44`
  Status: `done`
  Goal: the book opens with a tour of what archogen is for, at the director's request of `2026-10-01` ("it would
  really help users, students, see the real potential of ARCHOGEN … seeing what they will be able to do with ARCHOGEN
  will keep them engaged"), on the condition the director endorsed — "scrupulously honest about today versus
  tomorrow".
  Acceptance: a chapter after the introduction that follows one description to its running program and on to the
  board, every claim about today checked against the repository, every copy of a file held to the file by a test.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0278 (leaf PROGRAM.44)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git show HEAD:docs/book/src/SUMMARY.md | grep -c "tour"` → 0: the book explained
    each part and showed no newcomer what the whole is for.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the chapters were written as each part landed, so the book had no chapter
    whose subject is the destination: `git show HEAD:docs/book/src/introduction.md | grep -c "archogen build"` → 0,
    and the summary goes from the introduction straight to "What eADL describes"; WHERE:
    `docs/book/src/SUMMARY.md`'s first section.
  - [x] **FIX** — `docs/book/src/tour.md`, listed after the introduction: `examples/s0-heartbeat`'s description and
    frozen output, verbatim; what is real today, each item checked — `rt-core`'s bare-metal build and agreement, the
    fault contract still in review, the spike a hand-written measurement and nothing generated on the emulator yet;
    what it becomes; what it is for and what it is not; the road to a board. `crates/archogen-cli/tests/book_tour.rs`
    holds each marked copy to its file. The ledger's `<TaskID>`, which rendered as an unclosed HTML tag, put in code.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --test book_tour` → `2 passed`, its arm reporting a
    drifted copy and a missing file; `bash scripts/build_book.sh` → the book written, 0 warnings;
    `book-anchors: OK (24 chapter(s), 4 normative document(s); every cited repository path resolves)`.
  - [x] **NO REGRESSION** — `cargo test -q --workspace --exclude rt-core --exclude rt-reference` → 832 passed, 0
    failed (the two crates were being edited by `M2.9`'s step 6i and are not touched here); `cargo clippy -q -p
    archogen-cli --tests -- -D warnings` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the book's summary and ledger; this leaf and its log; `CHANGELOG.md`.
