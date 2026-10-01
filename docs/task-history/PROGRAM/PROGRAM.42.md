- ID: `PROGRAM.42`
  Status: `done`
  Goal: `TASK-HISTORY` and `HISTORY-LEDGERS` checked for `DECISION-HISTORY`'s D1. Both read history with `git log`
  under its default simplification (`check_task_history.sh`, `check_history_ledgers.sh`), so a merge that drops a
  seal's commit from the simplified history may hide it the same way. The review did not test them.
  Acceptance: the construction reproduced on each gate or shown not to apply, by a scratch repository; where it
  applies, `--full-history` and a RED arm; line endings of the sealed folders pinned in `.gitattributes`, as the
  review's D14 asks of `docs/decision-history/`. Widened while reproducing: both gates also read a failed history
  command as an empty history, D5's class, so a shallow repository and a failed read are refused too.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0239 (leaf PROGRAM.42)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — scratch repositories under `target/tmp/p42/`, the seal on one branch, then
    `git merge -s ours` from a branch that never sealed. `TASK-HISTORY` at the merge → rc=0, `task-history: OK (0
    sealed file(s) …)`, and again after the sealed leaf is edited live. `HISTORY-LEDGERS`, whose other side dropped
    the oldest twenty entries itself so the window leg stays quiet → rc=0, `history-ledgers: OK (2 ledger(s) …)`:
    twenty entries lost, every leg green. Both apply.
  - [x] **ROOT CAUSE** — `git show 3526848:scripts/check_task_history.sh | grep -n 'git("log"'` → lines 232 and
    241, and `git show 3526848:scripts/check_history_ledgers.sh | grep -n 'git log'` → lines 171 and 183: every
    history read uses git's default simplification, which follows one side of a merge that is TREESAME to it and
    never visits the seal. Each also read a failed command as no history (`or b""`, `2>/dev/null`).
  - [x] **FIX** — `--full-history` on all four reads; a shallow repository refused; a failed read a breach. Each
    self-test gains the merge and a shallow clone as RED arms. `.gitattributes` marks `docs/task-history/` and
    `docs/history/` `-text`, all 83 of their files measured `i/lf` first.
  - [x] **ADDRESSED** — the same constructions with the fix → rc=1, "`T.1`, committed in `415761b824ba`, is gone or
    changed" and "row `0001`, committed in `fecbdf577beb`, is gone or changed"; `bash scripts/check_task_history.sh
    --self-test` → `27 pass / 0 fail (27 arms)`; `bash scripts/check_history_ledgers.sh --self-test` → `17 pass / 0
    fail (17 arms)`. Mutations on copies: without `--full-history` the merge arm turns red in each (`26 pass / 1
    fail`, `16 pass / 1 fail`); without the shallow check its arm turns red in each.
  - [x] **NO REGRESSION** — both gates on the real trees → rc=0, `task-history: OK (69 sealed file(s) …)` and
    `history-ledgers: OK (2 ledger(s) …)`; `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 37 self-test(s)
    passed`; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — `docs/decisions/decision_task-tree-sealing.md`, `docs/decisions/decision_history-ledgers.md`,
    `DOCTRINE_ENFORCEMENT.md`, `.gitattributes`; this leaf, the frontier and both logs; `LIVE_STATUS.md`,
    `docs/TASK_TREE.md`, `MEMORY.md`, `CHANGELOG.md`.

