- ID: `PROGRAM.1`
  Status: `done`
  Goal: convert `ROADMAP.md` into the milestone task-tree set and register it.
  Acceptance: a tree exists for `M0`, `S0`, `M1`–`M7`; every F01–F30 fixture and every §18
  work package names an owning tree/leaf; `docs/TASK_TREE.md` lists them all.
  Verification: `scripts/check_doctrines.sh` green; coverage table in this file.
  Commit: `ARCHOGEN-PROGRAM-0002`

- ID: `PROGRAM.1.1`
  Status: `done`
  Goal: declare this project's code-path seam so the neutral `TASK-ACCEPTANCE` default stops
  classifying `docs/book/src/**` as a code change.
  Acceptance: the book page no longer matches the code-path set; the gate goes red→green with
  no edit to any spine check.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0002`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the built-in default in `scripts/check_task_acceptance.sh`
    is `default_code_re='(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$'`; its `(^|/)src/`
    arm matches a `src` segment at ANY depth, and this repository's mdBook sources live under
    `docs/book/src/`. Measured on this commit's staging set:
    `git diff --cached --name-only --diff-filter=ACM | grep -E '(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$'`
    → `docs/book/src/introduction.md`, `rc=0`. With one "code" file staged, the check then
    demanded a ticked checklist on all ten staged trees.
  - [x] **ADDRESSED (verified)** — before: `scripts/check_doctrines.sh` on the staged set printed
    `TASK-ACCEPTANCE: docs/tasks/S0.md has no 'ROOT CAUSE' box …` (30 such lines, ten trees × three
    boxes) and `=== 1 doctrine breach(es) — commit blocked ===`. After declaring
    `.doctrine/code_paths.txt`: the same command prints `=== all doctrines green ===`, `rc=0`,
    and `git diff --cached --name-only | grep -Ef .doctrine/code_paths.txt` matches nothing.
  - [x] **NO REGRESSION** — the seam narrows nothing that is really code: `\.(rs|sh)$` still
    covers every Rust and shell source wherever it lives, and `crates/`, `scripts/`, `xtask/`,
    `catalog/`, `Makefile`, `Cargo.toml`/`Cargo.lock` and `rust-toolchain.toml` are named
    explicitly. Full enforcer re-run: `=== all doctrines green ===` (13 checks), `rc=0`.
  - [x] **FIX** — added `.doctrine/code_paths.txt`, the seam `.doctrine/README.md` documents.
    No spine check was edited.
  - [x] **LOCKSTEP** — recorded here and in `DEV_NOTES.md`; the seam file carries its own
    measured rationale so the next reader does not have to rediscover it.
