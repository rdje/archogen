# PROGRAM: program spine — roadmap ownership, workspace, tiers, and the book

## Metadata

- Tree ID: `PROGRAM`
- Status: `active`
- Roadmap lane: workstream F — engineering operations (`ROADMAP.md` §11, §14, §15)
- Created: `2026-09-13`
- Owner: repo-local workflow

## Goal

Own the cross-cutting engineering substrate that every milestone tree depends on: the
conversion of `ROADMAP.md` into task-trees, the Rust workspace and crate boundaries, the
tiered verification story, the `xtask` runner, the versioning/ledger discipline, and the
mdBook that is the director's window into the project.

## Non-Goals

- This tree does not implement eADL semantics, engine realization, analysis, or generation.
  Those belong to `M0`–`M7` and `S0`.
- This tree does not restate the roadmap. `ROADMAP.md` remains the direction; the trees own
  the disciplined execution of it.

## Acceptance Criteria

- Every roadmap milestone, work package, and mandatory fixture (F01–F30) is owned by a leaf
  in some tree, and the mapping is discoverable from `docs/TASK_TREE.md`.
- The workspace layout matches the responsibility boundaries in `ROADMAP.md` §4.2, with
  crates created only when a real consumer justifies the split.
- `make check` and `make gate` stay green; the tiered verification story of §14.3 is
  implemented as named, runnable commands.
- The mdBook describes what the code actually does, with no drift.

## Task Tree

- ID: `PROGRAM`
  Status: `active`
  Goal: own the program spine
  Children: `PROGRAM.1` (+ `PROGRAM.1.1`) … `PROGRAM.7`

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

- ID: `PROGRAM.2`
  Status: `pending`
  Goal: establish the workspace skeleton and crate boundaries actually needed by S0/M1,
  replacing the bedrock starter crate with the `osgen` CLI shell.
  Acceptance: `cargo test --all` green; `osgen --help` lists the §10.2 command surface as
  implemented-or-unimplemented, with unimplemented commands exiting with a clear diagnostic.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.3`
  Status: `pending`
  Goal: implement the tiered verification runner (`xtask`) for the §14.3 tiers
  (focused / integration / extended / hardware / assurance), including the CI-policy split
  between per-commit focused checks and the pre-push full gate.
  Acceptance: each tier is a named command; a skipped or unavailable required tool is
  reported as skipped, never as a pass.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.4`
  Status: `pending`
  Goal: stand up the mdBook structure that mirrors the program: mission, boundary, profile,
  language, engine, analysis, generation, evidence, CLI.
  Acceptance: `make book` builds; every chapter that describes behavior points at the code or
  fixture that implements it.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.5`
  Status: `pending`
  Goal: implement the dependency/evidence ledger of §15 and §19 as a tracked, checkable
  record (external source versions, retrieval dates, scope, limitations, revalidation
  triggers).
  Acceptance: every external source claim in the book and decisions resolves to a ledger row.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.6`
  Status: `pending`
  Goal: implement semantic versioning separation (§15) — language/profile semantics, engine
  implementation, catalog entries, model versions, evidence formats — as machine-checked
  version records with a compatibility corpus.
  Acceptance: a locked description retains its meaning across an engine upgrade; F25 has a
  home.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.7`
  Status: `pending`
  Goal: project-specific doctrine checks (`scripts/check_doctrines.project.sh`) that encode
  this program's own invariants — most importantly that no implementation content leaks into
  eADL fixtures, and that assurance wording cannot overstate evidence.
  Acceptance: each check fails on a seeded violation and passes on the clean tree.
  Verification: `pending`
  Commit: `pending`

## Roadmap coverage map

Every roadmap unit has exactly one owning tree. This table is the answer to "where does
roadmap item X live?".

| Roadmap unit | Owning tree | Note |
| --- | --- | --- |
| §12 M0 — charter, boundary, target, examples | [`M0`](M0.md) | queue packages 1, 2, 4 |
| §12 S0 — early executable generation | [`S0`](S0.md) | queue package 3 |
| §12 M1 — eADL description foundation | [`M1`](M1.md) | queue packages 5, 6 |
| §12 M2 — one engine realization + controls | [`M2`](M2.md) | queue packages 7, 8, 9 |
| §12 M3 — joint resolver + checked plan | [`M3`](M3.md) | queue package 10 |
| §12 M4 — generated system + simulator | [`M4`](M4.md) | queue package 11 |
| §12 M5 — physical execution | [`M5`](M5.md) | board/emulator evidence |
| §12 M6 — reuse and extension | [`M6`](M6.md) | catalog reuse measurement |
| §12 M7 — first supported release | [`M7`](M7.md) | release packaging |
| §11 workstream F — engineering operations | `PROGRAM` | this tree |
| §14 agent workflow, review, CI tiers | `PROGRAM` | `PROGRAM.3` |
| §15 versioning and change management | `PROGRAM` | `PROGRAM.5`, `PROGRAM.6` |
| §16 reuse/optimization measurement | [`M6`](M6.md) | baseline recorded at M6 |
| §17 risk decisions and stop/rework criteria | `PROGRAM` | routed per trigger to its tree |
| §19 prior art and source ledger | `PROGRAM` | `PROGRAM.5` |
| §20 definition of completion | [`M7`](M7.md) | release exit gate |

## Fixture ownership map (F01–F30)

| Fixture | First gate | Owning tree | Leaf |
| --- | --- | --- | --- |
| F01 valid sub-HW/sub-OS imports | M1 | [`M1`](M1.md) | `M1.4` |
| F02 circular imports / conflicting exports | M1 | [`M1`](M1.md) | `M1.4` |
| F03 zero clock frequency / incompatible units | M1 | [`M1`](M1.md) | `M1.3` |
| F04 relevant capability undescribed | M1 | [`M1`](M1.md) | `M1.5` |
| F05 irrelevant capability undescribed | M1 | [`M1`](M1.md) | `M1.5` |
| F06 contradictory offered/absent declarations | M1 | [`M1`](M1.md) | `M1.5` |
| F07 invalid functional refinement | M1 | [`M1`](M1.md) | `M1.6` |
| F08 two exclusive requests, one resource | M3 | [`M3`](M3.md) | `M3.3` |
| F09 conflicting ownership | M3 | [`M3`](M3.md) | `M3.3` |
| F10 unsupported implementation path | M3 | [`M3`](M3.md) | `M3.4` |
| F11 solver resource limit | M3 | [`M3`](M3.md) | `M3.4` |
| F12 corrupted plan / changed requirement | M3 | [`M3`](M3.md) | `M3.5` |
| F13 counter rollover, ambiguous horizon | M4 | [`M4`](M4.md) | `M4.5` |
| F14 deadline already expired | M4 | [`M4`](M4.md) | `M4.5` |
| F15 interrupt pending while masked | M4 | [`M4`](M4.md) | `M4.5` |
| F16 context preservation under preemption | M4/M5 | [`M4`](M4.md) | `M4.6`, `M5.3` |
| F17 unknown interference, refuse assurance | M2/M4 | [`M2`](M2.md) | `M2.6` |
| F18 scheduling positive/negative fixtures | M2 | [`M2`](M2.md) | `M2.3` |
| F19 cost changed, old bound retained | M4 | [`M4`](M4.md) | `M4.8` |
| F20 linked image exceeds or overlaps RAM | M4 | [`M4`](M4.md) | `M4.4` |
| F21 stack observation sold as a bound | M4 | [`M4`](M4.md) | `M4.8` |
| F22 replay identity mismatch | M4 | [`M4`](M4.md) | `M4.7` |
| F23 intentional shared misconception | M5 | [`M5`](M5.md) | `M5.4` |
| F24 new composite functional kind | M6 | [`M6`](M6.md) | `M6.3` |
| F25 locked rebuild after catalog update | M6 | [`M6`](M6.md) | `M6.4` |
| F26 missed deadline / stack guard / trap | M4/M5 | [`M4`](M4.md) | `M4.6` |
| F27 boundary classification corpus | M0/M1 | [`M0`](M0.md) | `M0.3` |
| F28 small description to executable | S0 | [`S0`](S0.md) | `S0.4` |
| F29 repeated preemption cost ledger | M2 | [`M2`](M2.md) | `M2.5` |
| F30 trust-dependency drift gate | M3/M4 | [`M3`](M3.md) | `M3.6` |

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `PROGRAM.2` | `pending` | S0 and M0 both need the workspace shell and the `osgen` entry point |
| 2 | `PROGRAM.4` | `pending` | the book must exist before milestone chapters can land in lockstep |
| 3 | `PROGRAM.3` | `pending` | tiers formalize what `make check` already does informally |

## Decisions

- `2026-09-13`: one tree per roadmap milestone, plus this `PROGRAM` tree for the
  cross-cutting substrate. Rationale: a milestone is the roadmap's own unit of exit-gate
  evidence, so a tree per milestone makes the frontier and the gate the same object.
- `2026-09-13`: `docs/book/src/` is NOT a code path here. The neutral `TASK-ACCEPTANCE`
  default treats any `src/` segment as Rust source; the project seam
  `.doctrine/code_paths.txt` states this repository's real shape instead of editing the
  portable check. Found by the gate refusing this tree's own first commit (`PROGRAM.1.1`).
- `2026-09-13`: crates are created when a consumer needs them, per `ROADMAP.md` §4.2, not
  up front as eleven empty shells. The responsibility names in §4.2 are the naming
  convention for when each split happens.

## Open Questions

- None blocking the frontier.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-13` | `PROGRAM.1` | `scripts/check_doctrines.sh` | `13/13 green` |
| `2026-09-13` | `PROGRAM.1.1` | `scripts/check_doctrines.sh` staged | `red → green` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROGRAM.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | roadmap seeded into ten trees |
| `PROGRAM.1.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | code-path seam, same commit |

## Changelog

- `2026-09-13`: Created task tree; seeded the milestone trees from `ROADMAP.md` revision 2.0.
