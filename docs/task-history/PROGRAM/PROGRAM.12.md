- ID: `PROGRAM.12`
  Status: `done`
  Goal: keep **harness-local scratch** out of the tracked tree, so that "handoff-ready" stays
  decidable: an agent session's own permission and state files must not read as unfinished
  project work, and must not be committed as if they were a project decision.
  Reproduce / issue: after the first commit of the `2026-09-27` session, `git status --short`
  reported `?? .qwen/` — a directory the harness created to record two command approvals
  (`Bash(sed *)`, `Bash(git ls-files *)`). Nothing in the repository ignored it, so the tree could
  not be reported clean, and the alternative — committing it — would have exported one session's
  approval decisions to every future reader as project config.
  Acceptance: `.qwen/` is ignored with the reason stated where the rule lives; a deliberately
  shared harness config can still be tracked and the file says how; `git status --porcelain` is
  empty after a session that created harness state; no tracked file is removed or altered by the
  change; the ignore is scoped to the measured harness rather than speculatively listing harnesses
  this repository has never seen.
  Priority: **low effort, medium impact** — it does not change behaviour, but an undecidable
  cleanliness test corrupts every handoff banner that depends on it.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0050 (leaf PROGRAM.12)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — `git status --short` after commit `e95f5e3` reported `?? .qwen/`,
    and `cat .qwen/settings.json` showed harness-recorded approvals
    (`"allow": ["Bash(sed *)", "Bash(git ls-files *)"]`, `"$version": 4`) — state belonging to one
    session, sitting in a tree whose handoff test is "no modified or untracked files".
  - [x] **ROOT CAUSE (WHY + WHERE)** — `.gitignore` had no arm for harness-local directories.
    WHERE: `git check-ignore -v .qwen/settings.json` → no match, `rc=1` at the parent commit, i.e.
    the file was untracked *and* unignored, so it appeared in every status. The two available
    resolutions are both wrong on their own: committing it publishes one session's approvals as
    project config, and leaving it makes "is the tree handoff-ready?" unanswerable.
  - [x] **ADDRESSED (verified)** — after adding `/.qwen/`: `git check-ignore -v
    .qwen/settings.json` → `.gitignore:31:/.qwen/   .qwen/settings.json`, `rc=0`;
    `git status --porcelain` lists only the two files this leaf edits (`.gitignore`,
    `docs/tasks/PROGRAM.md`) and no `??` row; the state itself is preserved on disk
    (`ls -1 .qwen` → `settings.json`), so nothing the harness needs was deleted.
  - [x] **NO REGRESSION** — `git ls-files .qwen | wc -l` → `0`, so no tracked path became ignored
    (the failure mode that silently stops shipping a file); `git diff --stat` → `2 files changed,
    72 insertions(+), 1 deletion(-)`, both of them this leaf's; no build input is touched, so the
    Rust tiers are unaffected — `make gate` → `=== all doctrines green ===` on the staged set.
  - [x] **FIX** — one anchored ignore arm at the repository root, with the reasoning beside it: why
    the state is neither committed nor left visible, and how a deliberately shared harness config
    would still be tracked (`git add -f`). Scoped to the harness actually measured here rather than
    a speculative list of harnesses this repository has never seen — a guess-list is a rule with no
    measurement behind it.
    ⭐ **Drive-by, measured and recorded rather than slipped in:** the adjacent comment pointed at
    `docs/feedback/linkedspec/SETUP.md`, which does not exist — `ls -1 docs/feedback/linkedspec`
    → `INDEX.md`, `README.md`, `issues/`. The tracker was restructured into one self-contained
    sub-tree per issue (`M1.16`, `M1.17`), each carrying its own `SETUP.md`, and this pointer was
    not carried along. Corrected to name the per-issue files.
  - [x] **LOCKSTEP** — this leaf, the `PROGRAM` frontier and both logs; `PROGRAM.13` and
    `PROGRAM.14` logged from the census that this commit's own status check exposed. `README.md`,
    the book and `LIVE_STATUS.md` are unchanged: no user-visible toolchain surface moved, and
    `PROGRAM`'s row already reads `In Progress`.
