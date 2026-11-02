- ID: `PROGRAM.116`
  Status: `done` — P2, filed and fixed `2026-10-26`: what is held for the scaffold read from the director's ruling, not
    from the updater's NEUTRAL array
  Reproduce / issue: the ruling of `2026-09-30` (findings §10, sealed in
    `docs/decision-history/decision_findings-for-director-review/10.md`): "only [template] inherited script you haven't
    modified. Those you modified are yours now" — seventeen files measured, `check_task_acceptance.sh` named archogen's.
    `SCRATCH-LOCALITY` (`PROGRAM.29`, `2026-09-29`, a day before it) takes every file on `scripts/update_scaffold.sh`'s
    NEUTRAL array as the scaffold's and was never brought to it; `PIPE-READS` (`PROGRAM.114`, this session) copied that
    rule. So `check_task_acceptance.sh`'s two off-volume `mktemp` calls and six pipe reads under `pipefail` — the form
    `PROGRAM.113` found failing on CI — were reported for upstream instead of fixed. Found while reading the ruling for
    `M3.5.1.4`'s `DOCTRINE_ENFORCEMENT.md` edit; `PROGRAM.115`'s premise, that table scaffold-owned, falls with it.
  Built: `.doctrine/scaffold_held.txt`, the ruling's held files with its citation — sixteen, `.githooks/pre-commit`
    having left the hold when `M2.7.4.4` (`e2b034dc`, `2026-10-03`) changed it — read by both gates in place of the
    NEUTRAL array, each gaining an arm: a NEUTRAL file archogen changed is refused. `check_task_acceptance.sh`'s eight
    sites fixed: its scratch under `target/doctrine_scratch/`, its self-test's reads through here-strings. The scratch
    record, `DOCTRINE_ENFORCEMENT.md`'s `SCRATCH-LOCALITY` row (two bytes shorter), `.doctrine/README.md`, `TOOLBOX.md`
    and the book's repository annex and ledger say so; `PROGRAM.115` corrected. Upstream is owed five pipe reads and
    two scratch sites, all in held files; `PROGRAM.114`'s list of eleven stands for those five.
  Acceptance:
    - [x] ROOT CAUSE — WHY: both gates read the updater's NEUTRAL array as the scaffold's files, which the ruling of `2026-09-30` narrowed to those archogen never changed; WHERE, `git show 99d32dd1:scripts/check_scratch_locality.sh`, `scaffold_owned()` reading `NEUTRAL=(`.
    - [x] FIX — the held list; both gates read it; `check_task_acceptance.sh`'s eight sites; the records and the book.
    - [x] ADDRESSED — `bash scripts/check_scratch_locality.sh --self-test`: `scratch-locality self-test: 17 pass / 0 fail (17 arms)`; `bash scripts/check_pipe_reads.sh --self-test`: `pipe-reads self-test: 9 pass / 0 fail (9 arms)`; `bash scripts/check_task_acceptance.sh --self-test`: `task-acceptance self-test: 10 pass / 0 fail`; `bash scripts/check_pipe_reads.sh`: `pipe-reads: OK (74 pipefail script(s) scanned; 5 site(s) in scaffold-owned files reported, …)`. Falsified: before the fix `SCRATCH-LOCALITY` refused `check_task_acceptance.sh:374` under the held list, and a pipe read put back at `check_task_acceptance.sh:306` is refused by `PIPE-READS`.
    - [x] NO REGRESSION — `bash -n` of the three scripts `exit 0`; `bash scripts/check_readme_routes.sh`: `readme-routes: OK (25 destination(s) governed)`; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 258 s.
    - [x] LOCKSTEP — the gates, the list, the records, `DOCTRINE_ENFORCEMENT.md`, the book.
  Commit: `ARCHOGEN-PROGRAM-0989 (leaf PROGRAM.116)`
