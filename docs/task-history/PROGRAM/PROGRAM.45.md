- ID: `PROGRAM.45`
  Status: `done`
  Goal: the tour says how a reader interacts with a generated system, and answers the questions the director asked
  on `2026-10-01` — "will it possible to interact with ARCHOGEN OSes? if so, how?", "logging into that OS and have a
  minimal shell", "a kind of filesystem? what kind?" — in the same honest terms ("this sort of concrete, honest
  information will help people understand what ARCHOGEN will produce").
  Acceptance: inputs, outputs and inspection as the profile admits them, with an example the checker accepts; what
  runs today kept apart from what is ahead; no login, shell or filesystem stated with the roadmap's reasons; every
  shown file held to its file by a test.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git show HEAD:docs/book/src/tour.md | grep -c "interact"` → 0.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `PROGRAM.44`'s tour said what a system is for, not how one uses it; WHERE:
    `docs/book/src/tour.md`: `git show HEAD:docs/book/src/tour.md | grep -c "## 6. How you interact"` → 0. A claim made
    in conversation was corrected on the way: a debugger is not yet attached to the emulator — `grep -c "gdb"
    scripts/target_emulator.sh` → 0 — so the chapter says QEMU *can* host one.
  - [x] **FIX** — the tour's section 6, *How you interact with it*, with `positive-sporadic-release.eadl` as an
    `excerpt:`-marked block; `crates/archogen-cli/tests/book_tour.rs` learns excerpts (each shown line in its file, in
    order, an elision starting `…`) and indented blocks, with an arm for a drifted excerpt.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --test book_tour` → `2 passed`; the first run of the
    excerpt check reported the chapter's copy as differing, an elision the check had not yet read, fixed in the check;
    `bash scripts/build_book.sh` → 0 warnings.
  - [x] **NO REGRESSION** — `cargo test --all -q` → 956 passed, 0 failed; `cargo clippy -q -p archogen-cli --tests --
    -D warnings` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the book; this leaf and its log; `CHANGELOG.md`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0282 (leaf PROGRAM.45)`
