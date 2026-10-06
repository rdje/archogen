- ID: `PROGRAM.63`
  Status: `done` — filed and closed `2026-10-06`
  Goal: a closed subtree that carries ledger hand-offs, or lines a tree may hold, can be sealed: `HANDOFF-LEDGER` reads a
  sealed leaf's quotes in its sealed file, and `docs/task-history/` admits every line `docs/tasks/` does.
  Acceptance: `M3.1`, the first such subtree, seals with every gate green; each fix proven by an arm that failed first.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — after `ARCHOGEN-M3-0470` closed `M3.1`, `bash scripts/check_task_history.sh --seal M3`
    sealed it byte for byte, and the commit was refused twice: `HANDOFF-LEDGER: 7 breach(es)`, *"`SR-H1`'s obligation is
    not quoted word for word in `M3.1.2` (docs/tasks/M3.md)"*, one per `SR-H1` … `SR-H7`; and *"docs/task-history/:
    2871 bytes on its longest line, over its ceiling of 2048"*. The seal was undone, the tree left clean.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE, the ledger: `scripts/check_handoff_ledger.sh` collects leaf blocks "open
    trees first, then sealed history" and keeps the first block per leaf (`if cur is not None and cur not in blocks`),
    so a sealed leaf's two-line stub in its tree shadowed the sealed body holding its quotes; its arm *"a hand-off quoted
    by a sealed leaf passes"* wrote the sealed file and no stub, so it never met the real shape. WHY it surfaced now:
    `M3.1` is the first sealed subtree whose leaves quote hand-offs. WHERE, the ceiling: `README_POLICY.md`'s
    `docs/task-history/` row, 2048, beside `docs/tasks/`' 3072 — a seal copies bytes, so the archive must admit every
    line the tree may hold; `M3.1.1`'s two lines, 2 771 and 2 871 bytes, were within the tree's ceiling.
  - [x] **FIX** — the ledger: a stub (`Status: \`done\` — sealed in [`) never shadows a sealed body; two arms, a stub
    beside the body that quotes it, and a sealed body that drops the quote, refused naming its sealed file. The
    ceiling: `docs/task-history/`' longest line 3 072, by `docs/decisions/decision_task-history-line-ceiling.md`, named
    in "Ceilings a decision fixes".
  - [x] **ADDRESSED (verified)** — `bash scripts/check_handoff_ledger.sh --self-test` → `12 pass / 2 fail (14 arms)`
    before the fix, both new arms failing, and `14 pass / 0 fail (14 arms)` after; a trial seal of `M3.1` on the fixed
    gate → `handoff-ledger: OK (46 hand-off(s) in 2 ledger(s) …)`, undone after; `readme-routes: OK (24
    destination(s) governed)` with the new ceiling, its self-test `20 pass / 0 fail`.
  - [x] **NO REGRESSION** — `bash scripts/check_handoff_ledger.sh` on the real tree → `OK`; `decision-index: OK (32
    record(s) …)`; `make focused` → `tier focused: passed — 3 passed, 0 failed`; the doctrine gate at commit. The seal
    itself is the next commit, under leaf `M3`.
  - [x] **LOCKSTEP** — the script's own header; the book's annex, *Finished work leaves the task trees*; the decision
    record and its index row; `CHANGELOG.md`. `DOCTRINE_ENFORCEMENT.md`'s row is unchanged and still true — a sealed
    leaf is the leaf — and the file stands at its ceiling.
