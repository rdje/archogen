- ID: `PROGRAM.53`
  Status: `done` — started and closed `2026-10-10`
  Goal: a ticked NO REGRESSION box that cites a test run is backed by a run made on the tree it is committed with —
  measured, not remembered.
  Reproduce / issue: found `2026-10-05` by `M3.6.2.1`. `M3.6.2`'s box read `make focused` → `tier focused: passed — 3
  passed, 0 failed`; the session's only run before its commit `ARCHOGEN-M3-0432` was at 08:08, and the file that
  broke `eadl-front`'s population test, `trust/roots.eadl`, was written at 08:27. The doctrines run no test suite, so
  nothing at commit time compares a cited run with the tree. CI runs the suite only after a push.
  Direction, to be designed in this leaf: `make focused` writes a stamp in `target/` naming the tree it ran on (the
  working tree's own tree id, through a temporary index) and its verdict; `TASK-ACCEPTANCE` refuses a ticked box that
  cites `make focused` when the staged tree has no passing stamp. Open: how unstaged changes beside the staged ones are
  handled, and a stamp's relation to a tier other than `focused`.
  Decided: unstaged changes beside the staged ones make the stamped tree another than the staged one, so the commit is
  refused until they are stashed and the tier run again — the rule this project's own notes already keep by hand; a
  stamp is written for any tier that passes, and the gate asks for the focused tier's alone, the one boxes cite. The
  leg joins `UNTRACKED-CODE`, whose doctrine is that no commit records a tree nobody ran; `TASK-ACCEPTANCE` is the
  scaffold's, not this project's to change.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the new arms beside `76d8775`'s gate, `bash scripts/check_untracked_code.sh --self-test`
    → *"9 pass / 2 fail (11 arms)"*: a staged leaf citing a focused run with no stamp, and one with another tree's stamp,
    both passed by the gate as it stood.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git grep -n 'git ls-files --others' -- scripts/check_untracked_code.sh` →
    the gate's one leg, untracked files; `git grep -c verify-stamps 76d8775 -- xtask/src` → nothing: no run left a
    record of the tree it ran on. WHY: every check reads the working tree, and a box's citation is a sentence.
  - [x] **FIX** — `xtask/src/main.rs`: `working_tree_id`, through a temporary index; `stamp`, written after a tier
    passes when the tree did not move during the run. `scripts/check_untracked_code.sh`: the second leg and four arms.
    The doctrine's row, its registry line, the book's *A commit holds what was run*.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_untracked_code.sh --self-test` → *"11 pass / 0 fail (11 arms)"*;
    this commit's own box below is held by the leg, its `make focused` run on the staged tree the stamp it shows.
  - [x] **NO REGRESSION** — `cargo test -p xtask` → `165 passed; 0 failed`; `cargo clippy -p xtask --all-targets -- -D
    warnings` → clean; `bash scripts/run_self_tests.sh` → *"OK — 48 self-test(s) passed"*; `make focused` → `passed —
    3 passed, 0 failed`, stamped; the doctrine gate at commit.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`'s row and the registry's line; the book's annex; this leaf and both
    logs; `docs/TASK_TREE.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0534 (leaf PROGRAM.53)`
