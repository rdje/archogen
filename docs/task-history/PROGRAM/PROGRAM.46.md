- ID: `PROGRAM.46`
  Status: `done` — `2026-10-01`, filed and closed the same day
  Goal: a gate that every commit on `main` whose subject names a work unit (`ARCHOGEN-<TREE>-NNNN (leaf …)`) has a row
  in its tree's Commit Log.
  Why: `ARCHOGEN-M2-0263` and `ARCHOGEN-M2-0280` both landed without their rows, each found by hand a commit later; no
  gate checks it, and `HISTORY-LEDGERS` checks only the changelog's order.
  Acceptance: the check derives the rows from `git log` rather than a list, reports a missing row by commit, runs in
  the enforcer, and has RED arms in scratch repositories like the other gates.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git show HEAD:scripts/check_doctrines.project.sh | grep -c "COMMIT-LOG-ROWS"` → 0;
    the census: 18 of the 260 commits whose subject has the `(leaf …)` form had no Commit Log row, `-0263` and
    `-0280` among them.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `COMMIT.md` asks for the row, `git show HEAD:COMMIT.md | grep -c -i "commit log"`
    → 2, and no gate reads it; `HISTORY-LEDGERS` checks the changelog's order only. WHERE: the enforcer's project slot, `scripts/check_doctrines.project.sh`.
  - [x] **FIX** — `scripts/check_commit_log_rows.sh`: ids from `git log` and the pending message file, rows from the
    table lines of `docs/tasks/` and `docs/task-history/`, the measured backlog of 18; registered in the project slot;
    `DOCTRINE_ENFORCEMENT.md`'s row; the book's verification chapter.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_commit_log_rows.sh` → rc=0, `commit-log-rows: OK (242 work-unit
    commit(s), each with a Commit Log row; 18 in the backlog, which may only shrink)`; `--self-test` → `9 pass / 0
    fail (9 arms)`, among them a pending commit with no row refused and a prose mention not counted as a row. Twenty
    older commits name a work unit without `(leaf …)` and are outside the rule; the first run, which counted them,
    refused its own backlog, and the backlog was re-measured by the rule itself.
  - [x] **NO REGRESSION** — scripts and docs only; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`,
    the new gate among them, this commit's own row checked as the pending one.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`; the book's `verification.md`; this leaf and its log; `CHANGELOG.md`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0283 (leaf PROGRAM.46)`
