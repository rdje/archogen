- ID: `PROGRAM.4`
  Status: `done`
  Goal: stand up the mdBook structure that mirrors the program: mission, boundary, profile,
  language, engine, analysis, generation, evidence, CLI.
  Acceptance: `make book` builds; every chapter that describes behavior points at the code or
  fixture that implements it.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0033`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the book had accreted one chapter per leaf into a **flat
    list of eighteen**, with no structure and nothing checking the second half of this leaf's own
    acceptance. Measured at the parent commit: `grep -c '^- \[' docs/book/src/SUMMARY.md` → `18`
    and `grep -c '^# ' docs/book/src/SUMMARY.md` → `1` (the title) — no parts at all. And the
    census that found the real defect, run per chapter over repository-path citations in code
    spans: **`presence.md` cited zero**. A chapter with no anchor cannot be checked against
    anything, by a script or by a reader; it is an essay about a system rather than a description
    of one. WHERE this costs most: `docs/book/` is the director's **only** window into the
    project — the code is not read, the book is.
  - [x] **ADDRESSED (verified)** — `SUMMARY.md` now carries five parts that mirror the programme
    (what eADL describes · writing a description · what the engine may claim · generating and
    running a system · using the toolchain), and `presence.md` names
    `crates/eadl-model/src/presence.rs`, its F04–F06 test file and the worked-case directory.
    The acceptance is now **mechanical**: `scripts/check_book_anchors.sh` →
    `book-anchors: OK (18 chapter(s); every cited repository path resolves)`, registered as a
    project doctrine, so `scripts/check_doctrines.sh` → `=== all doctrines green ===` includes it.
    `mdbook build docs/book` → `INFO HTML book written to`.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed`; `cargo test --all` → **357**
    passed, `0 failed`, unchanged — this leaf touches no Rust; `scripts/check_doctrines.sh` →
    `=== all doctrines green ===`; `mdbook build docs/book` → OK with the new part structure. No
    chapter's prose was rewritten: the only content change is the "Where it lives" section added
    to `presence.md`.
  - [x] **FIX** — `scripts/check_book_anchors.sh`, with two legs that fail for different reasons.
    **Anchored**: a behavior chapter must cite a repository path. **Resolvable**: every cited path
    must exist — the leg that catches a renamed module or a deleted fixture, where the book keeps
    reading perfectly, which is the problem.
  - [x] **REPRODUCE / ISSUE** — three RED arms, run:
    `scripts/check_book_anchors.sh --self-test` → `book-anchors self-test: 3 pass / 0 fail`. They
    seed an unanchored chapter, a chapter citing `crates/no-such-crate/src/gone.rs`, and — the arm
    that matters as much — a **well-formed** chapter, because a check that always fails is not
    discriminating either.
    ⛔ The false positive was anticipated from `scripts/check_s0_retirement.sh`'s first run and
    designed out: chapters legitimately name `src/main.rs` of a **generated** crate and
    `os-rt.eadl` by basename, and requiring those to exist at the repository root would make
    writing about generated output cost a doctrine breach. The check matches what is
    unambiguously a claim about *this* repository — a path under a tracked top-level directory, or
    a named root document.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` (the registry mirror) and
    `scripts/check_doctrines.project.sh`; `TOOLBOX.md` gains the check as a diagnostic tool;
    `docs/book/src/SUMMARY.md` and `presence.md`; `knowledge-map/subsystems.md`; `MEMORY.md`;
    `LIVE_STATUS.md`; `CHANGELOG.md`; `docs/TASK_TREE.md`. No new `DEV_NOTES.md` heading: the
    transferable rule — match a claim about this repository, not any string that looks like one —
    is the same false positive already recorded for `S0-RETIREMENT`, and designing it out in
    advance is the lesson being *applied* rather than a new one.
