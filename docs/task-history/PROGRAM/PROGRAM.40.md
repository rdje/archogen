- ID: `PROGRAM.40`
  Status: `done`
  Goal: `HISTORY-LEDGERS` catches in CI what it catches before a commit. Its leg 3 compares the index with `HEAD`,
  which is the commit under test in CI, so a segment and its row forged together and committed pass there; the
  review of `PROGRAM.32.4` found it (P4), and `TASK-HISTORY` now checks the same across history.
  Acceptance: every row any committed index held still present, and every segment byte for byte what the commit that
  added it wrote, with RED arms for a committed forgery; the book and `DOCTRINE_ENFORCEMENT.md` updated.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0229 (leaf PROGRAM.40)`

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE** — the new arm, "a segment and its row forged together and committed are refused", built a
    forgery in a scratch repository and committed it. Against the old leg 3 it passes, since `HEAD` is the forgery
    itself. With the new segment check mutated off, `bash scripts/check_history_ledgers.sh --self-test` →
    `history-ledgers self-test: 14 pass / 1 fail (15 arms)`, that arm red.
  - [x] **ROOT CAUSE** — leg 3 compared the index only with `HEAD`, as `git show faa9d2b^:scripts/check_history_ledgers.sh`
    shows (`git cat-file -e "HEAD:$INDEX"`). In CI, `HEAD` is the commit under test, so a forged segment and row
    matched themselves.
  - [x] **FIX** — leg 3 reads every committed version of the index (`git log --format=%H -- docs/history/INDEX.md`)
    and requires each of its rows still present and unchanged. Each segment must equal, byte for byte, what the
    commit that added it wrote (`git log --diff-filter=A`). The honest limit is restated.
  - [x] **ADDRESSED** — `bash scripts/check_history_ledgers.sh --self-test` → rc=0,
    `history-ledgers self-test: 15 pass / 0 fail (15 arms)`; `bash scripts/check_history_ledgers.sh` on the real
    ledgers → rc=0, `history-ledgers: OK (2 ledger(s) …)`.
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`;
    `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 35 self-test(s) passed`. No held file changed.
  - [x] **LOCKSTEP** — `docs/decisions/decision_history-ledgers.md`, `DOCTRINE_ENFORCEMENT.md`,
    `docs/book/src/verification.md`; this leaf, the frontier and both logs; `LIVE_STATUS.md`, `docs/TASK_TREE.md`,
    `MEMORY.md`, `CHANGELOG.md`.
