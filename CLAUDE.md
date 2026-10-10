# Agent bootstrap (read this first — whatever AI / harness you are: Claude Code, Codex, Gemini, Cursor, Aider, a custom runner, …)

This repository is built on a **portable discipline spine**: durable memory, task-tree
tracking, a strict commit workflow, mechanical doctrine enforcement, and a claim-verification
standard for any number someone else will act on. The spine is
enforced at the **git level** (hooks + CI), so it holds regardless of which agent or
human is working. Follow it exactly.



1. Read `README.md` — project objective, layout, standard commands.
2. Read `MEMORY_ARCHITECTURE.md` — the durable 4-layer memory model (**MANDATORY**;
   defines how nothing important is lost across sessions, machines, or harness switches).
3. Read `TOOLBOX.md` — the tools-first doctrine. For ANY unknown / failure / surprising
   result: use or build a diagnostic tool FIRST; never guess a root cause.
4. Read `DOCTRINE_ENFORCEMENT.md` — how every mechanizable doctrine is enforced, and the
   task-acceptance checklist a change MUST pass.
5. Read `docs/CLAIM_VERIFICATION.md` — what "checked" means before a number is published:
   **re-derive · falsify · make durable**, three dimensionally different questions, and a
   missing leg is stated rather than hidden. §A restates it in this project's terms.
6. Read `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` — which live documents are bounded snapshots of *now*
   (`MEMORY.md`, `LIVE_STATUS.md`, `docs/TASK_TREE.md`) and which are histories. A snapshot takes current
   state, never a closure narrative: the narrative's home is the tree's Commit Log and `CHANGELOG.md`.
7. Resume from `MEMORY.md` (the bounded layer-A resume pointer) → the active task-tree's
   frontier under `docs/tasks/`.

## The non-negotiables

- **Nothing changes without a task-tree first.** Every code change is owned by a
  task-tree leaf under `docs/tasks/` (index: `docs/TASK_TREE.md`) BEFORE the change is
  made. Track every activity, task, slice, and lane so the project survives a lost session.
- **Record durable facts/decisions** as one-file-per-record notes under `docs/decisions/`
  (index there). Convert relative dates to absolute.
- **Commit per `COMMIT.md`** after each completed leaf, with the work-unit id in the
  subject. A code change must pass the `TOOLBOX.md` / `DOCTRINE_ENFORCEMENT.md` acceptance
  checklist (root cause + addressed + no regression) in its task leaf.
- **Activate the hooks once per clone:** `git config core.hooksPath .githooks`. The
  pre-commit hook runs `scripts/check_doctrines.sh` (the general enforcer); CI runs the
  same. These are git-level and harness-agnostic.
- **Keep the roadmap, the code, and the docs (README + mdBook) aligned** — locked
  together, no drift, for past, present, and future changes.
- **No background job at a handoff point.** Before you end a session (`/exit`, a pause, a
  handover), run `bash scripts/check_no_background_jobs.sh` and make it print `handoff: OK`:
  a job that outlives its session rewrites tracked files under the next one, with its log
  gone. Kill stragglers AND their children — a parent's death does not propagate.
- **A commit message ends with its own last line** — no agent/tool attribution trailers
  (`COMMIT.md`); the `commit-msg` hook refuses them.
- **Every other git repository is read-only — in both directions.** *Outbound:* never create a
  commit, branch, tag or file in any repository but this one, the vendored checkouts under
  `vendor/` included; reading, fetching, checking out a published commit, building by the
  documented route and moving **our own** submodule pin are consumption, not writes. *Inbound:* a
  change another project's agent delivers here lands through a task-tree leaf recording who
  authorized it, what it touched and what it preserved. The `REPOSITORY-BOUNDARY` doctrine checks
  the part visible from here — each vendored checkout at its pin, nothing committed, modified or
  created in it; the rest is this rule
  (`docs/decisions/decision_repository-boundary-read-only.md`).
- **The sealed evaluation set is never read before leaf `M6.5`.** Its text is out of the tree; never `git show` or
  `git blame` a case, diff or grep `docs/evaluation/frozen/` with `-a`/`--text` or an external driver, run git on a
  bare clone or with `--git-dir` from elsewhere, view such a commit in a viewer that diffs blobs itself, or check
  out, clone, archive or add a worktree at a commit older than `6d61f65`, or revert `2f6f331`; fence every delegated
  search away from it
  (`docs/evaluation/README.md`). A case read is recorded.

> One rule above all: **information that exists only in the live conversation is not yet
> saved — route it to a layer and commit it before the turn ends.**

## First time in a fresh clone

Run `scripts/bootstrap.sh` once. It sets the project name, installs the git hooks, and
seeds the first task-tree from `ROADMAP.md`. Then drop your real roadmap into `ROADMAP.md`
and grow the project from there.
