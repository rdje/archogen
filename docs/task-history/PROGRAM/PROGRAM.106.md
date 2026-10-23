- ID: `PROGRAM.106`
  Status: `done` — P2, asked by the director `2026-10-23`: task-tree sealing written as a doctrine any project, on any
    harness, can adopt
  Reproduce / issue: the director asked what led to the task history and sealed task trees, what it solved, and for a
    git-tracked document under `docs/` stating the approach project- and harness-agnostically. The approach is spread
    over three local records (`decision_task-tree-sealing.md`, `decision_task-log-row-sealing.md`,
    `decision_task-history-line-ceiling.md`), its trigger over a sealed finding (§8 of the findings record), its
    mechanics over a tool's header (`scripts/check_task_history.sh`): each written in this project's terms, none of them
    adoptable as it stands.
  Goal: `docs/TASK_TREE_SEALING.md` — the problem and its measured trigger, the principles, the vocabulary, the rules, the
    seal's procedure and the gate's legs, an adoption path from a minimal version to the full one, the lessons this
    project paid for, what it does not do, a checklist; every figure cited to the record that measured it; registered in
    `README_POLICY.md`, linked from `README.md`'s spine and the book's annex.
  Verification: every figure re-derived at `b9d52e1c` — `git ls-files docs/task-history` (394 files, 3 248 964 bytes),
    `git ls-files docs/tasks` (554 725 bytes), each tree's lines and stubs — and each against the record it cites;
    two claims and a count corrected on that read; every link resolving; the doctrines green.
  Commit: `ARCHOGEN-PROGRAM-0900 (leaf PROGRAM.106)`
