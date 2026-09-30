- ID: `PROGRAM.13`
  Status: `done`
  Goal: backfill the **Verification Log** and **Commit Log** of the three trees whose logs stopped
  after their first slices, transcribing from `git log` and from each leaf's own recorded checks —
  never inventing a row.
  Reproduce / issue: a census over every tree, counting `done` leaves against log rows
  (`for t in docs/tasks/*.md; do … grep -c '^  Status: `done`' … sed -n '/## Verification
  Log/,/## Commit Log/p' | grep -c '^| `' …; done`) returns
  `BOOTSTRAP done=1 verification_rows=0 commit_rows=0`, `M2 done=5 verification_rows=1
  commit_rows=1`, `PROGRAM done=6 verification_rows=3 commit_rows=3`, while `M0 7/8/8`,
  `M1 17/16/17` and `S0 7/21/7` are complete. So **twelve closed leaves do not name their own
  commit in the tree that owns them**, and `BOOTSTRAP`'s only closed leaf records no verification
  at all. Impact: layer B is the route from a leaf to its evidence; with the log empty, a resuming
  session or an auditor must reconstruct it from `git log --grep`, and a leaf can sit `done` with
  no recorded verification — the exact state the acceptance checklist exists to prevent.
  Acceptance: every `done` leaf in those three trees carries a Commit Log row naming its real
  commit subject, derived from git; a Verification Log row carrying the checks that leaf actually
  recorded; a row that cannot be derived is written `not recorded (<why>)` rather than guessed; the
  census command is recorded in the leaf so the result is re-runnable; no leaf's status changes.
  Priority: **medium** — no behaviour depends on it, but the memory architecture does: an unlogged
  leaf is a leaf a crashed session cannot resume from.
  Verification: closed `2026-09-30`. The census re-derived first — the filing counts were a day old and trees had
  grown — by leaf **ID** rather than by row count, since a row count cannot say *which* leaf is unlogged:
  ```text
  for f in docs/tasks/*.md: for each leaf whose Status is `done`, does `## Verification Log` name `<id>` in
  backticks, and does `## Commit Log` (to the end of the file)?
    before: BOOTSTRAP 1 without a verification row (the tree had no Verification Log at all);
            M1 — M1.9 without either row, M1.11 without a commit row; M2 — M2.1–M2.5 without either;
            PROGRAM — PROGRAM.2.1, .3, .22 and .4 without either                          (22 rows owed)
    after:  every tree 0 / 0
  ```
  Each row derived, never guessed: the commit subject, short hash and date from `git log --grep="leaf <id>)"` —
  one match per leaf, each equal to the commit the leaf itself records; each Verification row transcribes the
  leaf's own ADDRESSED box (or, for `PROGRAM.22`, its `Verification:` field), truncated with `…` and marked
  "backfilled `2026-09-30` (`PROGRAM.13`)". No `not recorded` was needed: every leaf had recorded its checks. No
  leaf's status changed. `BOOTSTRAP.md` gained the `## Verification Log` section it never had; its Commit Log
  stays the bullet list it is.
  Commit: `ARCHOGEN-PROGRAM-0136 (leaf PROGRAM.13)`
