- ID: `PROGRAM.72`
  Status: `done` — filed `2026-10-10` by `PROGRAM.69`'s review round 8, its P1; done `2026-10-28`
  Goal: a tree file in a sub-folder of `docs/tasks/` is judged as any tree file is, or is no tree file.
  Reproduce / issue: `scripts/check_task_history.sh`'s leg 6, its foreign-name and named-twice checks read
  `docs/tasks/*.md` alone, while `TASK-ACCEPTANCE` takes an owning leaf from any staged `docs/tasks/**/*.md`
  (`scripts/check_task_acceptance.sh`); the round's probe, its own untracked runner and so not durable: a live
  `M1.12.1.9` in `docs/tasks/m1/M1.md` passes the gate, at `docs/tasks/M1x.md` refused. Present at `fea69ad`.
  Direction: the gate walks `docs/tasks/` whole, `TEMPLATE.md` excepted — or `TASK-ACCEPTANCE` takes owners from
  `docs/tasks/*.md` alone; an arm either way.
  Built: the second way, the gate's: a tree file is `docs/tasks/<TREE>.md`, so `TASK-HISTORY`'s leg 6 refuses a
    Markdown file in any sub-folder of `docs/tasks/`, which none of its legs would judge. `TASK-ACCEPTANCE` is held for
    the scaffold (`.doctrine/scaffold_held.txt`) and stays as it is; the only sub-folder today, `artifacts/`, holds
    scripts.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `HEAD`'s gate with a live `M1.12.1.9` planted in `docs/tasks/m1/M1.md`: `old rc=0`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — leg 6 read the folder's own entries alone:
    `git show HEAD:scripts/check_task_history.sh | grep -c 'for name in sorted(os.listdir(TASKS))'` → `1`.
  - [x] **FIX** — a walk of `docs/tasks/`'s sub-folders in leg 6, its header and two arms; the book's Annex B.
  - [x] **ADDRESSED (verified)** — the patched gate on the same plant: `TASK-HISTORY: docs/tasks/m1/M1.md is a tree
    file in a sub-folder of docs/tasks/`, `1 breach(es)`; `bash scripts/check_task_history.sh` → `exit 0` on the real
    tree; `bash scripts/check_task_history.sh --self-test` → `174 pass / 0 fail (174 arms)`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/run_self_tests.sh` →
    `exit 0`, `self-tests: OK — 58 self-test(s) passed`; `bash scripts/check_doctrines.sh` → `exit 0`.
  - [x] **LOCKSTEP** — the gate's header, the book's `annex-repository.md`, this leaf and the Commit Log.
  Commit: `ARCHOGEN-PROGRAM-1040 (leaf PROGRAM.72)`
