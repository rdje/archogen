- ID: `PROGRAM.66`
  Status: `done` — filed and closed `2026-10-06`, after `M3.6.3.5`'s correction
  Goal: no commit is made while a file in a code path is untracked, so a commit cannot record a tree nobody ran.
  Acceptance: `UNTRACKED-CODE` refuses a commit beside an untracked file `.doctrine/code_paths.txt` matches, names it,
  and passes an ignored or a non-code one; its self-test's arms each fail first; on the real tree it passes.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `ARCHOGEN-M3-0485` (`948b1b6`) was made with `git commit -a` beside an untracked
    `xtask/src/trust_verify.rs` that its own `main.rs:53` declared: `git ls-tree --name-only 948b1b6 -- xtask/src/`
    lists no `trust_verify.rs`, and that tree fails, `error[E0583]: file not found for module \`trust_verify\``.
    Every check before it — `make focused`, the doctrine gate — read the working tree, where the file was, and passed.
    `M3.6.3.5`'s correction, `0486`, committed it.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the commit path. `git commit -a` stages modified tracked files and no
    new one, and `COMMIT.md` step 8's `git status --short` after the commit is a reading nothing enforces; every gate
    the pre-commit hook runs judges the working tree or the staged changes, so none sees a file that is in neither the
    index nor the commit. `git grep -n -e 'ls-files --others' HEAD -- scripts/check_doctrines.sh
    scripts/check_doctrines.project.sh` → nothing: no gate asked git for untracked files.
  - [x] **FIX** — `scripts/check_untracked_code.sh`, `UNTRACKED-CODE`: `git ls-files --others --exclude-standard`
    filtered by `.doctrine/code_paths.txt`'s patterns, each such path a breach named with what to do; no statement of
    what is code is a breach. Registered in `scripts/check_doctrines.project.sh`; its self-test found by
    `scripts/run_self_tests.sh`'s glob.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_untracked_code.sh --self-test` → `untracked-code self-test: 7
    pass / 0 fail (7 arms)`; with the code-path match removed → `5 pass / 2 fail`, the two refusal arms; with the
    missing-statement check removed → `6 pass / 1 fail`. On the real tree before the script was staged → *"`scripts/
    check_untracked_code.sh` is in a code path and untracked"*, exit 1 — the case it exists for, on itself; staged,
    `untracked-code: OK`.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` with the script staged;
    `DOCTRINE_ENFORCEMENT.md` within its 36 864-byte ceiling after two closure narratives became pointers to their
    leaves (`PROGRAM.27`, `PROGRAM.35`) and a third shortened.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`'s row; the book's Annex B, *A commit holds what was run*, and its
    index regenerated; this leaf and both logs; `CHANGELOG.md`.
  Verification: `2026-10-06` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0490 (leaf PROGRAM.66)`
