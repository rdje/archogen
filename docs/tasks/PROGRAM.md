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
  Children: `PROGRAM.1` … `PROGRAM.21`, plus `PROGRAM.1.1` and `PROGRAM.2.1`

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
  Status: `done`
  Goal: establish the workspace skeleton and crate boundaries actually needed by S0/M1,
  replacing the bedrock starter crate with the `archogen` CLI shell.
  Acceptance: `cargo test --all` green; `archogen --help` lists the §10.2 command surface as
  implemented-or-unimplemented, with unimplemented commands exiting with a clear diagnostic.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0003`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the workspace still held only the template's starter
    crate, so the project had no entry point and the §10.2 interface target existed nowhere in
    code. Measured at the parent commit: `git ls-tree --name-only HEAD crates/` → `crates/app`
    (one crate); `git grep -c 'bin name = "archogen"' HEAD -- crates/` → no match, `grep rc=1`;
    `git show HEAD:crates/app/src/main.rs | grep -n 'println!'` →
    `7:    println!("bedrock: replace this crate with your project — start from ROADMAP.md.");`.
    WHERE: `crates/app/src/main.rs:7` — the only executable behavior in the repository was a
    template placeholder.
  - [x] **ADDRESSED (verified)** — after: `cargo run --quiet --bin archogen -- --help` prints all
    seven §10.2 commands with their owning leaves and the eleven-row exit-code table,
    `exit=0`; `cargo run --quiet --bin archogen -- check examples/periodic-three/system.eadl
    --profile rt-static-up-v1` prints
    `archogen: unimplemented: \`archogen check\` is not implemented yet` with
    `hint: … tracked by task-tree leaf M1.8`, `exit=20`. Before, the same binary name did not
    resolve at all (evidence B above, `grep rc=1`).
  - [x] **NO REGRESSION** — `cargo fmt --all -- --check` → `fmt rc=0`; `cargo clippy
    --all-targets --all-features -- -D warnings` → `Finished \`dev\` profile`, no warnings;
    `cargo test --all` → `test result: ok. 28 passed; 0 failed; 0 ignored; 0 measured; 0
    filtered out`; `scripts/check_doctrines.sh` → `=== all doctrines green ===`, `rc=0`;
    `mdbook build docs/book` → `INFO HTML book written to`.
  - [x] **FIX** — removed `crates/app`; added `crates/archogen-cli` (lib `archogen_cli` + bin
    `archogen`) with the §10.2 surface declared once as data in `src/spec.rs`, a hand-written
    parser in `src/cli.rs`, and the §5.5 outcome vocabulary with stable exit codes in
    `src/status.rs`. Zero external dependencies.
  - [x] **LOCKSTEP** — `docs/book/src/cli.md` (new chapter) and `SUMMARY.md`;
    `docs/decisions/decision_zero-dependency-engine-core.md` + its index row;
    `knowledge-map/subsystems.md`; `MEMORY.md`; `LIVE_STATUS.md`; `CHANGELOG.md`.

- ID: `PROGRAM.2.1`
  Status: `done`
  Goal: rename the toolchain's command and crate family from `archogen` to `archogen`, across the
  roadmap, the mdBook, the task-trees, the examples and the code.
  Acceptance: the old name survives **only** in the documents that record the rename — this
  leaf, `ROADMAP.md`'s migration note, the `CHANGELOG.md` entry and `MEMORY.md`'s latest-commit
  line — and nowhere else in tracked files; `archogen check`
  behaves exactly as the old command did; every test, gate and book build stays green.

  ⛔ The first draft of this criterion said "`git grep -ci osgen` returns nothing on tracked
  files", which is **self-defeating**: the record of a rename necessarily names the old thing,
  so writing the evidence down would have broken the criterion it was evidence for. The
  criterion is scoped to exclude those records instead, and the exclusion is verified rather
  than assumed.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0021`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the project is `archogen` and its command announced
    itself as `osgen`, so the toolchain carried two names. Measured before the change:
    `git grep -o -i 'osgen' | wc -l` → `169` across 33 tracked files, and
    `git grep -oh -E '[A-Za-z_-]*osgen[A-Za-z_-]*' | sort | uniq -c` → six distinct forms
    (`osgen` 128, `osgen-cli` 14, `osgen-evidence` 12, `osgen_cli` 6, and
    `osgen-plan`/`osgen-emit`/`osgen-check` 3 each). WHERE: `crates/osgen-cli/Cargo.toml`'s
    `[[bin]] name = "osgen"` was the source, and `ROADMAP.md` §4.2 and §10.2 propagated it.
  - [x] **ADDRESSED (verified)** — after the change, the scoped census

    ```
    git grep -c -i 'osgen' -- . ':(exclude)Cargo.lock' ':(exclude)docs/tasks/PROGRAM.md' \
      ':(exclude)ROADMAP.md' ':(exclude)CHANGELOG.md' ':(exclude)MEMORY.md'
    ```

    returns **no output**, `rc=1`. The four excluded documents are exactly the four that *record*
    the rename: this leaf, the §15 migration note, the changelog entry, and the resume pointer's
    "latest commit" line. `Cargo.lock` is excluded because it was regenerated rather than edited:
    `grep -c 'osgen' Cargo.lock` → `0`, `rc=1`.
    ⛔ An earlier version of this box pasted the same command with only **three** exclusions,
    which does not reproduce — it returns `MEMORY.md:1`. A pasted command that does not reproduce
    is the failure `TOOLBOX.md` exists to prevent, so it was corrected rather than left as a
    near-miss.
    `crates/archogen-cli/Cargo.toml` now carries `[[bin]] name = "archogen"`, and the command
    behaves identically: `archogen check examples/periodic-three/system.eadl` →
    `accepted against profile \`rt-static-up-v1\` (8 declaration(s))`, `exit=0`;
    `archogen check examples/bounded-queue/system.eadl` →
    `archogen: unsupported-profile: 1 diagnostic(s)`, `exit=12` — the same verdicts and the same
    exit codes as before, with only the program name changed.
  - [x] **NO REGRESSION** — `cargo fmt --all -- --check` → `fmt clean`; `cargo clippy
    --all-targets --all-features -- -D warnings` → 0 errors; `cargo test --all` → **246** tests
    passing, `0 failed` — the same count as before the rename, so no test was lost to a renamed
    path; `scripts/check_doctrines.sh` → `=== all doctrines green ===`; `mdbook build docs/book`
    → `INFO HTML book written to`.
  - [x] **FIX** — `git mv` on both crate directories so history follows the files, then one
    substitution across every tracked text file except `Cargo.lock`. All six spellings share the
    `osgen` prefix, so a single replacement covers the hyphenated, underscored and bare forms
    without a per-form rule.
    ⛔ `cargo fmt` was **not** optional here: the four extra characters pushed several lines past
    the width limit, and `--check` caught it before the commit rather than CI catching it after.
  - [x] **LOCKSTEP** — `ROADMAP.md` gains a dated **migration note**, which §15 requires of a
    rename ("maintain migrations for renamed fields and kinds"), stating that nothing but the
    name changed; `README.md`, all 15 book chapters, every task-tree, `examples/README.md`,
    `TOOLBOX.md`, `knowledge-map/subsystems.md`, `MEMORY.md`, `LIVE_STATUS.md` and `CHANGELOG.md`
    carry the new name.

  ### Scope decision, recorded because it was a judgement call

  The director's words were "rename the CLI from archogen to archogen **in the entire project**".
  Read narrowly that is the binary alone; read broadly it is every `archogen`-prefixed name. The
  broad reading was taken, because the narrow one **preserves the inconsistency that prompted
  the request**: a project called `archogen` whose evidence crate is `archogen-evidence` and whose
  §4.2 component names are `archogen-plan`, `archogen-emit`, `archogen-check` is still a project with two
  names in it.

  So the rename covers, in one substitution:

  | Form | Count | Becomes |
  | --- | --- | --- |
  | `archogen` (the command, and prose) | 128 | `archogen` |
  | `archogen-cli` (crate) | 14 | `archogen-cli` |
  | `archogen-evidence` (crate) | 12 | `archogen-evidence` |
  | `archogen_cli` (Rust identifier) | 6 | `archogen_cli` |
  | `archogen-plan` / `archogen-emit` / `archogen-check` (§4.2 names) | 9 | `archogen-*` |

  Narrowing it back to the binary alone is a small, mechanical revert of the crate directories
  and the §4.2 table; nothing depends on the broad reading being right.

- ID: `PROGRAM.3`
  Status: `done`
  Goal: implement the tiered verification runner (`xtask`) for the §14.3 tiers
  (focused / integration / extended / hardware / assurance), including the CI-policy split
  between per-commit focused checks and the pre-push full gate.
  Acceptance: each tier is a named command; a skipped or unavailable required tool is
  reported as skipped, never as a pass.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0029`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — §14.3's five tiers existed only as a table in the roadmap,
    and the director's CI policy (run the full gate before a push, a selected set for ordinary
    commits) had no mechanism at all. Measured at the parent commit:
    `git grep -ci 'tier' HEAD -- Makefile .github/` printed nothing, `rc=1`, and
    `git ls-tree --name-only HEAD` → no `xtask`, though §4.2 names it and
    `.doctrine/code_paths.txt` already reserves `(^|/)xtask/`. WHERE the cost landed: every
    verification decision was a judgement made per commit and recorded nowhere, so the three
    tiers nobody can run today — extended, hardware, assurance — were **invisible** rather than
    incomplete.
  - [x] **ADDRESSED (verified)** — five named commands, and the honest picture they produce:
    `cargo xtask verify --tier focused` → `tier focused: passed — 3 passed, 0 failed, 0
    unavailable, 0 not built`, `exit=0`;
    `--tier integration` → `incomplete — 5 passed, 0 failed, 1 unavailable, 0 not built`,
    `exit=20`, naming QEMU;
    `--tier extended` → `incomplete — 0 passed, 0 failed, 1 unavailable, 2 not built`;
    `--tier hardware` → `incomplete — … 1 not built`, naming `M5.1` and the absent board;
    `--tier assurance` → `incomplete — … 3 not built`, naming `M3.6`, `M4.8`, `M4.7`.
    `cargo test -p xtask` → `test result: ok. 8 passed; 0 failed`.
  - [x] **NO REGRESSION** — `cargo fmt --all -- --check` → `fmt rc=0`;
    `cargo clippy --all-targets --all-features -- -D warnings` → no warnings;
    `cargo test --all` → **317** passed, `0 failed` (309 before this leaf, `+8`);
    `scripts/check_doctrines.sh` → `=== all doctrines green ===`; `mdbook build docs/book` →
    `INFO HTML book written to`. `make check` and `make gate` are unchanged, so nothing that
    referenced them broke.
  - [x] **FIX** — `xtask/` (a workspace member, per §4.2), reached through the committed
    `.cargo/config.toml` alias, because a tier that works only if each developer remembers an
    incantation is a tier nobody runs. Tiers are declared **as data**, the idiom `spec.rs`
    already uses here. The verdict type has **three** states — `passed`, `failed`,
    `incomplete` — and `incomplete` is the whole point: §14.3 says "a required tool skipped or
    unavailable is reported as such, **not a passed check**", which two states cannot express.
    Two kinds of "cannot run" are kept apart because the response differs: **unavailable** (a
    tool is missing from this machine — install it) and **not built** (the step does not exist,
    and names the leaf that owns building it).
  - [x] **REPRODUCE / ISSUE** — the runner found two real defects on its first two runs, which is
    the argument for it. (1) It reported `❌ fmt … FAILED` **with no reason**: `cargo fmt
    --check` writes its diff to stdout while the runner captured only stderr. A failing step
    that cannot say why is one a developer re-runs by hand, which is the same as not having a
    runner — both streams are now captured, the tail is printed, and the exact command to re-run
    is echoed. (2) `every_named_leaf_is_declared_by_a_task_tree` failed on first run —
    `step \`fuzz\` names leaf \`PROGRAM.9\`, which no tree under docs/tasks/ declares` — because
    the shape test above it cannot tell `M9.9` from `M4.8`. `PROGRAM.9` now exists, and the
    existence check stays.
  - [x] **LOCKSTEP** — `Makefile` gains `make focused` / `make integration` / `make tiers`;
    `COMMIT.md` step 2 adopts the director's CI policy explicitly; `TOOLBOX.md` gains the runner;
    `docs/book/src/verification.md` (new chapter) + `SUMMARY.md`;
    `.github/workflows/rust.yml` runs the integration tier so CI and the local gate are the same
    object; `knowledge-map/subsystems.md`; `MEMORY.md`; `LIVE_STATUS.md`; `CHANGELOG.md`;
    `DEV_NOTES.md`; `docs/TASK_TREE.md`.

  ### Lesson promotion

`a verification runner's most useful output is what it cannot run` →
  `promotion: declined (already docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md, applied there to fixtures and here to tiers; a second entry for the same question makes the retrievable layer harder to search)`

  ⛔ **Written on one line deliberately.** `scripts/check_lesson_promotion.sh` matches
  `promotion: declined \(..*\)` with a **line-oriented** grep, so a decline whose parentheses
  wrap across lines is invisible to the gate — which is how a decline can look recorded and not
  be. Found by this very commit: the first draft wrapped, and the gate reported
  `1 new lesson entry in DEV_NOTES.md with NO promotion and NO explicit decline`. The same wrap
  exists in `S0.3`'s decline, where it went unnoticed because that commit also promoted a lesson
  and satisfied the check the other way; it is corrected in its own docs-only commit, since
  staging a second tree beside code is what `PROGRAM.8` tracks.

  ### The measurement that settled `focused`'s contents, recorded because it will expire

  §14.3 defines focused as "format/type checks and **affected** contract tests", and selecting
  affected tests needs change-impact machinery. Measured warm on this tree:
  `cargo fmt` 0.14 s, `cargo clippy` 0.08 s, `cargo test --all` **2.63 s**,
  `scripts/check_doctrines.sh` 1.36 s, `mdbook build` 0.08 s. At 2.9 s for the whole focused
  tier, impact analysis would cost more than it saves and would be one more thing to be wrong,
  so `focused` runs the entire suite. The number is written into the runner's own source so the
  decision can be re-taken against it rather than re-argued from memory.

- ID: `PROGRAM.11`
  Status: `active`
  Goal: state the **repository boundary in both directions** where a resuming agent actually reads
  it, and gate the part that is observable from inside this repository: no archogen agent writes
  into another repository or a vendored submodule's own history (**outbound**), and a change
  delivered into this repository by another project's agent lands through a leaf that records its
  authorization, its file list and what it preserved (**inbound**).
  Acceptance: both directions are stated in `CLAUDE.md`/`AGENTS.md` and `DOCTRINE_ENFORCEMENT.md`,
  not only in a session prompt; a `scripts/check_*.sh` registered in
  `scripts/check_doctrines.project.sh` fails on a seeded local commit and on a seeded local
  modification inside `vendor/`, and passes on a clean pin; the check has RED arms in `--self-test`;
  `docs/decisions/decision_repository-boundary-read-only.md` is linked from it and states the
  incident correctly.
  Priority: **medium** — see the correction below. The outbound half is preventive; the inbound half
  already fired once and was handled correctly.

  ⛔ **The incident, and a correction this leaf owes.** LinkedSpec `8b5b5ffd8` (`2026-09-27`,
  "record publication and repository-boundary violation") discloses "the unauthorized ARCHOGEN
  documentation commit and auxiliary writes". This leaf first read that as an **outbound** crossing
  and called the priority high because "the defect class already fired". **The director corrected it
  on `2026-09-27`: the crossing was inbound** — LinkedSpec's agent modified a few `.md` files *in
  this repository* to deliver its fix notice. LinkedSpec has since made other repositories
  read-only in its own bootstrap; it was a one-time error and is not expected to recur.
  Measured here: every commit carries the single local identity, `git reflog` is linear (no
  `reset`/`rebase`/`amend`), the inbound content entered via `82ee99a` (leaf `M1.18`) confined to
  `docs/feedback/linkedspec/**` plus archogen's own live docs, **no code path was touched**, and
  `make focused` → exit `0` with `cargo test --all` → **421 passed, 0 failed** over 36 suites.
  Full table in `docs/decisions/decision_repository-boundary-read-only.md`.
  ⭐ What survives, and it is the reason this leaf exists at all: `grep -rn 'READ-ONLY' CLAUDE.md
  AGENTS.md` → **no match**. The boundary rule was nowhere in the committed tree, which is why the
  direction was undecidable from inside it — and why the first draft of the durable record got it
  backwards. That wrong record, not a stray write, is the measured damage.

  ⚠️ **Honest limit to state up front:** a gate in *this* repository cannot prevent an outbound write
  into a checkout elsewhere on the filesystem, and cannot prevent an inbound write either — it can
  only require that the inbound one land through a leaf. What it can do is put the rule where every
  agent reads it, and detect the outbound symptom visible from here: a vendored submodule carrying
  local commits or local modifications. The leaf must not claim more than that.
  Verification: `pending`
  Commit: `pending`

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

- ID: `PROGRAM.13`
  Status: `pending`
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
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.14`
  Status: `pending`
  Goal: make `PROGRAM.13`'s finding **mechanically** impossible to repeat — a doctrine check that a
  leaf marked `done` names a commit in its own tree's Commit Log, registered in
  `scripts/check_doctrines.project.sh` and mirrored in `DOCTRINE_ENFORCEMENT.md`.
  THE GAP: nothing gates a `done` leaf that names no commit. CENSUS:
  `git grep -ln 'Commit Log' -- scripts/ xtask/ .doctrine/` → exactly one path,
  `scripts/bootstrap.sh`, which **seeds** the section in a new tree and never reads one back;
  the thirteen registered doctrines (`scripts/check_doctrines.sh`, `make gate`) include no
  task-log check.
  Acceptance: a new `scripts/check_*.sh` exits nonzero on a seeded `done` leaf with no commit row
  and zero on the real trees after `PROGRAM.13`; it carries RED arms in `--self-test`; it is scoped
  to staged files, so an unrelated tree cannot fail a commit; `TOOLBOX.md` and
  `DOCTRINE_ENFORCEMENT.md` name it; the honest limit is stated in the script header — it proves a
  row exists and names a subject that exists in git, never that the work was done.
  Priority: **medium**, and it must land **after** `PROGRAM.13` or the gate is red on arrival.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.15`
  Status: `pending`
  Goal: make the outbound feedback register's **states and totals** mechanically consistent with the
  issue sub-trees that are their source — the half of "keeping this index true" that no check
  performs today.
  Reproduce / issue: `FEEDBACK-SELF-CONTAINED` leg 4 checks only that every issue directory is *named*
  in the vendor's `INDEX.md` — `grep -n 'INDEX' scripts/check_feedback_self_contained.sh` → lines 17,
  47, 49, 75, all registration. Nothing compares a register row's `State` cell with the `**State**`
  field in that issue's own `README.md`, and nothing recomputes the totals table from the rows:
  `git grep -ln 'State' -- scripts/` → only `check_waiver_routing.sh`, which is about waivers.
  THE GAP: a register row that contradicts its own sub-tree is undetectable by any gate. Measured
  consequence: five state transitions landed across six commits on `2026-09-27` (leaves `M1.20.1` –
  `M1.20.7`), and consistency held only because each leaf hand-edited both files and the last one ran
  a census by hand — 7 of 7 rows matched, but a commit that got one wrong would have passed every
  check. Impact: the register is the file a vendor reads first, and a row that disagrees with its own
  sub-tree is worse than no register at all.
  Acceptance: a check — a new `scripts/check_*.sh` or a fifth leg of the existing feedback check —
  fails when a row's State differs from its sub-tree's State field, when a `verified` row carries no
  dated archogen re-measurement behind it, or when the totals table does not equal a recount of the
  rows; it is registered in `scripts/check_doctrines.project.sh` and mirrored in
  `DOCTRINE_ENFORCEMENT.md`; it carries RED arms in `--self-test`; it is scoped to staged files, so an
  unrelated vendor's register cannot fail a commit; and its honest limit is stated in the header — it
  proves the two records agree, never that the measurement behind them was right.
  Priority: **medium** — no behaviour depends on it, but this repository's whole claim to a vendor is
  that its records are consistent, and that claim currently rests on hand-editing.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.16`
  Status: `done`
  Goal: adopt the director-mandated **claim-verification policy** (§17 of the standing session
  instructions) into this repository under a repository-relative path, so the rule survives the
  session that carries it — the same failure `PROGRAM.11` was opened for, arriving from the other
  direction.
  Reproduce / issue: the policy is not here. `ls docs/CLAIM_VERIFICATION.md` → no such file, and
  `git grep -lni 'claim verification' -- '*.md'` → only `DEV_NOTES.md`, and only in the note that
  records this gap. THE GAP: a standing instruction is satisfied only inside a session prompt, so it
  is enforced nowhere in the repository and dies with the session. The read-only source is another
  repository's `docs/CLAIM_VERIFICATION.md` (285 lines, 18 166 bytes, read `2026-09-27`); §12's
  exception permits copying it **into** this repository and forbids writing to it. Measured
  consequence on the same day: leaf `M1.20.7` entered its reconciliation carrying an unverified
  premise about `docs/TASK_TREE.md` and caught it only by running the grep — exactly the behaviour
  the policy exists to require.
  Acceptance: the policy is copied to a repository-relative path with its provenance (source
  repository, path, date read) recorded **in the file**; nothing outside this repository is written
  to; the adoption is registered where a reader will find it — the `CLAUDE.md`/`AGENTS.md` pointer set
  and, for any mechanically checkable clause, `DOCTRINE_ENFORCEMENT.md`; the copy is compared against
  the source and any later change is applied or recorded as deliberately not applied with a reason;
  the leaf records which existing practices here already satisfy the policy and which it changes.
  Priority: **high** — it is the "rule enforced nowhere" shape this tree exists to eliminate, and the
  session that must follow it is the one that cannot see whether the last one did.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0058 (leaf PROGRAM.16)`
  promotion: declined (the lesson's canonical home is the adopted standard itself — §7 of
  `docs/CLAIM_VERIFICATION.md` *is* the adoption checklist, and it is now in this repository and in the
  bootstrap reading order, which is more discoverable than a knowledge card restating it. The local
  finding the sweep produced is owned as work, not prose: `PROGRAM.18`.)

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the repository's own bootstrap lists four spine
    documents and the policy is not among them — `grep -n 'CLAIM' CLAUDE.md` → no match at the parent
    commit — and `git grep -ni 'portable architecture' -- '*.md'` → no match, `rc=1`, so nothing here
    even had the vocabulary the policy uses to describe itself as the fifth. WHY it matters: the
    policy is the one that governs whether the *other four* are trustworthy, since a gate built on an
    unverified measurement enforces the wrong thing precisely and forever. It had been followed in
    practice all day — every leaf box cites a command, every new instrument carries RED arms — and
    followed for exactly the reason `PROGRAM.11` was opened for: it lived in a session prompt, so
    nothing in the tree would have noticed if a session stopped.
  - [x] **ADDRESSED (verified)** — `docs/CLAIM_VERIFICATION.md` now exists, 406 lines / 27 263 bytes:
    a 121-line adoption record (provenance, the §A local restatement, the §B sweep) followed by the
    policy body. **The body was copied, not retyped, and the copy was verified rather than read:**
    `tail -n +122 docs/CLAIM_VERIFICATION.md | diff -q - <source>` → identical, `rc=0`, and both sides
    digest to `9f99df25209c43af…`. A hand-copied policy would have been an unverified transcription of
    the document that defines verification. Registered where a reader meets it: `CLAUDE.md`'s spine
    sentence now names it and its reading order gains step 5, and `DOCTRINE_ENFORCEMENT.md`'s E1
    discovery list carries it with the sibling relationship stated. Nothing outside this repository was
    written to — the source was opened read-only, and `git -C <source-repo> status` was never invoked
    with a write intent.
  - [x] **NO REGRESSION** — the adoption adds one document and edits three; it changes no check, no
    gate and no build input. `bash scripts/check_doctrines.sh` → `=== all doctrines green ===` on the
    staged set, still 13 doctrines (the policy is deliberately **not** registered as a doctrine: its
    §5 mechanizations are a separate decision, and adding a gate that nothing needs yet is how a
    registry accumulates checks nobody can explain). `make focused` → exit `0`; `cargo test --all` →
    **421 passed, 0 failed** over 36 suites. The new file carries no checkout-specific absolute path
    (`DOCPATH` green is the proof, and the source's own examples are domain-free), and
    `check_table_arity`'s ratchet accepts its three new tables because every row matches its header.
  - [x] **FIX** — copy plus restate plus sweep, which is what §7 of the policy itself asks of an
    adopter, in its own order. §A restates all three legs in archogen's terms with **this
    repository's** measured instances rather than the source's — the vacuous `xargs sha` digest that
    would have made "regenerated" indistinguishable from "UNCHANGED", the classifier that reported a
    symptom over a clean run, the leaf premise that a grep disproved — because §7.6 says a rule you
    cannot restate in your own terms is under-specified for you. §B records the sweep, and step 5 is
    adopted **by mapping**: the leaf acceptance box *is* this repository's claim tag and is already
    gated, so a second inline tag syntax is deliberately not added.
    ⛔ **The sweep found a real gap and it was filed, not footnoted.** Step 4 — fire every control —
    measured 18 registered checks, 8 with RED arms (all passing) and **10 without**, including
    `check_task_acceptance.sh`, whose box-scoping was validated once during development and is
    re-fired by no arm. `PROGRAM.18` owns it, medium-high, that gate first. Publishing the adoption
    while hiding that result would have been the exact failure the policy describes.
    ⛔ **`GAP-CLAIM-CENSUS` blocked the first commit attempt, correctly.** The frontier row summarising
    `PROGRAM.18` said the property "is re-fired by nothing" — a whole-tree quantifier in a section
    carrying no census command, which is exactly the shape that doctrine exists to stop, and the census
    was one section away in the leaf. Reworded to name the missing `--self-test` arm: the same fact,
    stated as a count rather than as an absolute, and the count is the thing the leaf measured.
  - [x] **LOCKSTEP** — `docs/CLAIM_VERIFICATION.md` (new); `CLAUDE.md` (spine sentence + reading
    order); `AGENTS.md`; `DOCTRINE_ENFORCEMENT.md` (E1 list + the sibling question); this leaf,
    `PROGRAM.18`, the frontier, the Children line and both logs; `MEMORY.md`, `LIVE_STATUS.md`,
    `CHANGELOG.md`, `DEV_NOTES.md`.
    ⛔ **`AGENTS.md` was nearly missed, and the draft of this box is why it is recorded.** The first
    version of this leaf asserted that `AGENTS.md` "names the discipline documents generically, so it
    inherits the addition — verified by reading it rather than assuming". Reading it says the
    opposite: it carries an **explicit** list — `grep -n 'MEMORY_ARCHITECTURE' AGENTS.md` → line 11,
    `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`, and `COMMIT.md` — so a fifth
    spine document that is not added there is invisible to every harness that reads `AGENTS.md`
    instead of `CLAUDE.md`. The list now carries `docs/CLAIM_VERIFICATION.md`. The claim was written
    from the shape of the file remembered at session start, not from the file; leg 1 applies to a
    leaf's own prose about the repository exactly as it applies to a number.
    No book chapter changes: the book documents eADL and the engine, and this is spine documentation —
    `git grep -ln 'CLAIM_VERIFICATION' -- docs/book` → no match, `rc=1`.

- ID: `PROGRAM.17`
  Status: `pending`
  Goal: check the director-mandated **live-document size-containment guide** (§18) against what this
  repository already does and adopt what is missing — the instruction covers a *partial* adoption too,
  and this is one.
  Reproduce / issue: partially adopted, with no record of the comparison. Caps exist and are
  mechanically enforced for two documents — `README.md` (`line_cap=300`, `byte_cap=16384`,
  `scripts/check_readme_stability.sh`; measured 82 lines / 4 062 bytes) and `MEMORY.md` (≤ 50 lines /
  ≤ 7168 bytes, `scripts/check_memory_architecture.sh`; measured 42 / 3 273). No budget at all is
  recorded for the rest, and they are not small: `CHANGELOG.md` 1 464 lines / 109 666 bytes and grows
  one entry per commit, `ROADMAP.md` 912 / 101 260, `DEV_NOTES.md` 687 / 54 492, `LIVE_STATUS.md`
  20 / 3 315, `KNOWLEDGE_MAP.md` 150 / 11 842 (derived). THE GAP: nothing states which live documents
  are under a size budget and which are deliberately allowed to grow — census:
  `git grep -lni 'byte_cap\|line_cap\|size cap\|byte cap\|line cap' -- '*.md' '*.sh'` → `README_POLICY`,
  `MEMORY_ARCHITECTURE`, `DOCTRINE_ENFORCEMENT`, `CHANGELOG` and two check scripts, i.e. the two
  capped documents and their doctrine, not the other five. The read-only source (431 lines,
  21 327 bytes) has not been re-read since adoption, which §18 requires even for a partial one.
  Acceptance: the guide is read at its current source revision and the comparison recorded; every
  live document is listed with its measured size and either a cap or a recorded reason it has none;
  `CHANGELOG.md` and `DEV_NOTES.md` get an explicit decision — cap, rotate into dated segments, or
  grow deliberately — instead of drifting; anything adopted is copied in under a repository-relative
  path with provenance and nothing outside this repository is written to; a mechanical check is added
  only if a cap is actually set, registered like the others and carrying RED arms.
  Priority: **medium** — nothing is unreadable today and the two documents a resuming session reads
  first are both capped and enforced. But "append-only by design" is a decision nobody has written
  down, and an unwritten decision is the one a future session quietly reverses.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.18`
  Status: `pending`
  Goal: give the registered doctrine controls the repeatable RED arms they lack, so that "is this gate
  known to work?" stops depending on a validation somebody ran once while writing it.
  Reproduce / issue: measured by the §7.4 sweep of the claim-verification adoption (`PROGRAM.16`,
  `2026-09-27`): `scripts/check_*.sh` → **18** files, **8** carry `--self-test` with RED arms and all
  8 pass, **10 do not** — `check_docpaths`, `check_doctrines`, `check_doctrines.project`,
  `check_frozen_evaluation`, `check_memory_architecture`, `check_no_background_jobs`,
  `check_readme_stability`, `check_task_acceptance`, `check_task_tree_ownership`,
  `check_waiver_routing`. THE GAP: ten of eighteen controls have never been observed failing by a
  repeatable arm, and they include the most load-bearing gate in the repository.
  `check_task_acceptance.sh`'s own header records that its box-scoping was "priced against a real
  corpus" and validated against two measured leakage holes — real validation, and **one-off**: nothing
  re-fires it, so an edit could silently break the property and every commit would still pass. Census
  command: `for s in scripts/check_*.sh; do grep -q -- '--self-test' "$s" && echo yes || echo "$s"; done`.
  Acceptance: each of the ten either gains a `--self-test` carrying at least one RED arm that fails on
  a seeded breach and passes on the real tree, or is recorded in `DOCTRINE_ENFORCEMENT.md` as needing
  none with the reason — the two drivers are candidates, since they only run the others;
  `check_task_acceptance` is done **first**, and its arms must include the two leakage holes its header
  names, so the property it was written for is the property under test; every arm is run and its output
  recorded in the leaf; the doctrine count and the registry stay accurate; `make gate` is green before
  and after.
  Priority: **medium-high** — a gate nobody has seen fail is a gate whose failure mode is unknown, and
  this repository's whole claim is that its discipline is mechanical rather than remembered.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.19`
  Status: `done`
  Goal: run the ~24-hour artifact cleanup the standing instructions require, and start the record that
  makes "when was the last one?" answerable — `docs/ARTIFACT_CLEANUP.md` did not exist, so no session
  could tell whether a cleanup was due, which is the mechanism the instruction created the file for.
  Reproduce / issue: `ls docs/ARTIFACT_CLEANUP.md` → no such file. Inventory measured before touching
  anything: `.app-data` ≈ 3.6 GB, of which `.app-data/target` is **1.3 GB** — the *previous* pin's
  LinkedSpec build, superseded by the pin-named `.app-data/target-2ac834913` (2.1 GB) — and
  `.app-data/pgen-generated-2ac834913` is **70 MB** duplicating the checkout's own generated parser,
  digest-verified identical; `target/` is 815 MB, of which `target/tmp` is 17 MB of test scratch
  carrying most of **703** stale incremental `.bin` files; `docs/book/book` is 2.3 MB of built book.
  Impact: nothing is broken, but 1.4 GB of it is unreachable-by-design residue whose regeneration path
  is a tracked command, and one item — a documentation snapshot parked inside a *build* directory — is
  in the wrong place whatever its size.
  Acceptance: only artifacts whose regeneration path is a **tracked command** are deleted; every
  retained item has its reason recorded in the leaf; nothing tracked is touched, so
  `git status --porcelain` shows only this leaf and the new record; the focused tier, the doctrine gate
  and one vendor instrument all still run green afterwards, measured rather than assumed;
  `docs/ARTIFACT_CLEANUP.md` carries the date and a one-line summary with only the latest entry kept;
  anything unexpected found on the way is investigated and reported, not deleted.
  Priority: **low effort, low risk, mandated** — the instruction is explicit that a missing record file
  means a cleanup is due this session.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0060 (leaf PROGRAM.19)`
  promotion: declined (the operative rules now live where the next session must read them —
  `docs/ARTIFACT_CLEANUP.md` states both the trigger and the delete-only-what-regenerates test — and the
  one interesting finding this cleanup produced is recorded twice already: in `S0.7`'s leaf and in the
  `DEV_NOTES` lesson above it. A knowledge card would restate a standing instruction plus a fix that is
  now in the code it concerns.)

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `docs/ARTIFACT_CLEANUP.md` did not exist
    (`ls docs/ARTIFACT_CLEANUP.md` → no such file), so the instruction's own trigger — "if it is more
    than 24 hours old, **or the file does not exist**" — had been firing on every session with no way
    to tell. WHY the residue accumulated: `.app-data` is the vendor guide's application-local data root
    and is deliberately ignored, so nothing ever revisits it; `M1.20.4` added a *pin-named* target
    directory beside the old one (correctly — the guide says a new one preserves the older build for
    comparison), which is exactly the moment the older one stops being needed and starts being 1.3 GB
    of dead weight. The residue was not a mistake; it was a comparison that outlived its purpose.
  - [x] **ADDRESSED (verified)** — released ≈1.4 GB, each item deleted only because its regeneration
    path is a tracked command: `.app-data/target` 1.3 GB (rebuild via `scripts/linkedspec_eval.sh
    build` at whatever pin is checked out), `.app-data/pgen-generated-2ac834913` 70 MB (a
    digest-verified duplicate of the checkout's own `generated/`, and re-derivable via
    `linkedspec_eval.sh prepare`), `.app-data/empty-store-1` and `-2` (the instruments create them),
    `.app-data/reference-check` (`linkedspec_eval.sh reference` recreates it), `.app-data/upstream-notice27`
    (a prior session's scratch capture; the durable content is the tracked `UPSTREAM.md`), and
    `target/tmp` 17 MB (test scratch the suite recreates). Measured: `.app-data` 3.5 GB → 2.2 GB, and
    `target` 815 MB → 799 MB immediately after the deletion — then back to 816 MB once the verification
    runs recreated the test scratch, which is the expected result and the reason deleting it was safe.
    **Residue census after deletion:** all seven paths report `gone`, none `STILL PRESENT`.
    `docs/ARTIFACT_CLEANUP.md` now carries the date and one entry, with the mechanism stated so the
    next session can act on it.
  - [x] **NO REGRESSION** — nothing tracked was touched: after the deletions `git status --porcelain`
    listed only this leaf, and at commit time only this leaf plus the new record. The retained vendor
    build still works, measured rather than assumed: `scripts/linkedspec_eval.sh bins` → both binaries
    resolve under `.app-data/target-2ac834913/debug/`; `linkedspec_eval.sh reference` →
    `REFERENCE CHECK: the published two-form tagged document, exactly as documented`;
    `LS-002 …/remeasure.sh --self-test` → `9/9 arms passed`. `bash scripts/check_doctrines.sh` →
    `=== all doctrines green ===`; `make focused` → exit `0`; `cargo test --all` → **421 passed,
    0 failed** over 36 suites.
    ⛔ **The verification did not pass first, and that is the point of running it.** The first
    post-cleanup `make focused` reported `tier focused: failed — 2 passed, 1 failed`: with the test
    binaries cached and `target/tmp` gone, `a_description_with_no_system_says_there_is_nothing_to_build`
    panicked at `crates/archogen-cli/tests/s0_build.rs:211`. The warm re-run passed, which is how this
    would have been written off as a flake; it was reproduced deliberately instead
    (`rm -rf target/tmp && cargo test --all`, twice) and fixed at the root in leaf **`S0.7`** — cargo
    creates `CARGO_TARGET_TMPDIR` at build time, not run time, and that test was the only one of six
    sites writing into the tmpdir root. This leaf's acceptance is therefore measured *after* `S0.7`,
    and the record file says so.
  - [x] **FIX** — delete only what regenerates from a tracked command, retain everything else with a
    reason, and investigate rather than remove anything unexpected. **Retained, with reasons:**
    `.app-data/target-2ac834913` (the current pin's build; every remaining LinkedSpec measurement uses
    it), `.app-data/cargo-home` 132 MB (the vendor's guide says offline builds depend on the retained
    store), `.app-data/pgen-generated-before-remeasure` 18 MB (the **only** copy of the previous pin's
    parser — it is the "before" side of the regeneration digest frozen in `LS-004`'s evidence, and
    deleting it would make that digest unreproducible), `.app-data/ls004` 32 KB (the primary logs behind
    that same frozen evidence), `target/debug` 794 MB and `target/riscv64imac-unknown-none-elf` 5 MB
    (live products of the focused and integration tiers), and `docs/book/book` 2.3 MB (the built book —
    the director's window; it rebuilds in 0.07 s but costs nothing to keep).
    ⭐ **One item was investigated and deliberately left alone:** `target/sync-backup-2026-09-21`,
    24 KB, holding copies of `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`, `TASK_TREE.md` and `TOOLBOX.md`
    dated `2026-09-21`, and referenced by nothing tracked at the time it was investigated —
    `git grep -ln 'sync-backup' -- .` → no match before this leaf mentioned it, and one match
    afterwards, which is this sentence.
    Its contents are recoverable from git at any revision, so it is redundant — but it is somebody's
    deliberate backup, it is 24 KB, and "unexpected state may be someone's in-progress work" outranks
    tidiness. **Flagged, not deleted:** a documentation snapshot parked inside a *build* directory is in
    the wrong place whatever its size, and either belongs in git or nowhere.
  - [x] **LOCKSTEP** — `docs/ARTIFACT_CLEANUP.md` (new, latest-entry-only); this leaf, the frontier and
    both logs; `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`. The cross-tree link is
    recorded in both directions: `S0.7` names this leaf as what surfaced it, and this leaf names `S0.7`
    as what its verification found. No book chapter changes — the book documents eADL and the engine,
    not the repository's scratch directories, and `git grep -ln 'app-data' -- docs/book` → no match,
    `rc=1`.

- ID: `PROGRAM.20`
  Status: `pending`
  Goal: a **carried-figure register**, so a figure no measurement watches is a breach at the commit
  that adds it rather than a defect a later sweep happens to find. Three sweeps with three different
  patterns have now been needed to find one defect class, and a fourth pattern would find a fourth
  spelling: the mechanism, not the sweep, is what is missing.
  Reproduce / issue: the class has fired four times in this repository. `M1.23` found two live
  surfaces publishing a schema reach superseded 47 commits earlier, sweeping
  `grep -rnE '[0-9]+ of (the )?[0-9]+'` → 9 hits, 2 false. `M1.24` found three more live surfaces
  false from the **same commit** (`9030111`), which that pattern could not see because a corpus *size*
  is not an `N of M` figure; its own first replacement pattern (digits followed by a size noun) then
  found 2 of those 3 and could not see `the five ambiguous cases`, because it is spelled out. Two more
  shapes are already measured: `docs/book/src/s0.md` says "Three descriptions" where the directory
  holds four (`S0.8`), and `docs/TASK_TREE.md`'s `S0` row said "all six leaves closed" for one commit
  after `S0.7` added a seventh — `git show 4b7e000^:docs/TASK_TREE.md` and `git show
  4b7e000:docs/TASK_TREE.md` are byte-identical on that row, so the commit that moved the count never
  looked at the index that restates it. Corrected by `M1.24`'s routing edit rather than left. Each
  sweep is a population bounded by its pattern, so "nothing else found" has never been a result this
  repository could rely on — and a third sweep, adding `leaves|arms|checks|doctrines|productions|rows`,
  returned a further backlog of structure counts this leaf deliberately does **not** classify, because
  classifying five shapes by hand is the work the register exists to end.
  Acceptance: a registered check enumerates figure-shaped text in the **live** surfaces (book
  chapters, corpus indexes, crate module headers) and classifies each occurrence as *gated* (a named
  test compares it to a measurement), *registered as a record* (history, changelog, closed leaf), or
  *unregistered*; it exits nonzero on an unregistered figure introduced by the **staged diff**, in the
  `TABLE-ARITY-RATCHET` idiom — a per-file ratchet against `HEAD`, so the pre-existing population is a
  reported backlog rather than a reason to bypass; `--self-test` carries RED arms including the two
  shapes already measured (a digits figure and a spelled-out one); the gates `M1.23` and `M1.24` built
  are registered as consumers so they do not trip it; and the honest limit is stated in the check's own
  header — it proves a figure is *watched or listed*, never that a listed one is correct, which is
  `BOOK-ANCHORS`' limit one level down.
  Priority: **medium** — it is the structural fix for the most frequently recurring defect class in
  this repository, and it is what stops `M1.23`/`M1.24`/`S0.8` from being followed by an `M2.x`. It
  gates no milestone and blocks nothing, which is why it is scheduled behind `PROGRAM.11` and
  `PROGRAM.18` rather than ahead of them; `PROGRAM.18` (repeatable RED arms for registered controls)
  should land first, because this check arrives with arms and the older ten do not.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.21`
  Status: `pending`
  Goal: **`TASK-ACCEPTANCE` examines only the first acceptance checklist in a tree file, so for every
  leaf after the first it verifies nothing** — and prints a success message asserting the opposite.
  Make the check leaf-scoped, and arm it.
  Reproduce / issue: `scripts/check_task_acceptance.sh` runs one `awk` per staged `docs/tasks/*.md`.
  That awk sets `inbox=1` on the first bullet matching the keyword and `exit`s at the **next** box
  bullet, so exactly one box per keyword per file is ever read. Measured rather than inferred from the
  source: `docs/tasks/M1.md` carries **24** ticked ROOT CAUSE boxes
  (`grep -cE '^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*\*\*ROOT CAUSE' docs/tasks/M1.md` → `24`),
  and the check's own awk against that file captures **line 52** — leaf `M1.1`, written `2026-09-13`.
  Then, with `crates/eadl-front/tests/corpus.rs` staged and leaf `M1.24` carrying **zero** boxes
  (`awk '/^- ID: `M1.24`/,/^## Current Frontier/' docs/tasks/M1.md | grep -cE '^\s*- \[[ x]\] …'` →
  `0`), the gate printed
  `task-acceptance: OK (every staged code-change leaf carries a ticked, evidence-backed checklist)`
  and `exit=0`.
  ⭐ **Severity, measured rather than assumed: latent, not active.** Auditing every leaf that records a
  commit found five with no ticked ROOT CAUSE box — `M0.1`, `M0.2`, `M1.18`, `M1.20`, `PROGRAM.1` — and
  `git show --stat` on each of their commits reports **0** code files: `M1.18` staged twelve `.md`
  files, `M0.1`/`M0.2`/`PROGRAM.1` none, and `M1.20` is an aggregation node whose seven sub-leaves
  each carry their own checklist. So no code change has landed unboxed. The defect is that the gate
  **could not have stopped one** on any leaf but the first in its file, while telling the author it had.
  ⛔ That is the same failure mode the check's own header says box-scoping was introduced to end. It
  closed cross-**file** leakage and incidental-**prose** leakage, and left cross-**leaf** leakage open —
  a co-staged tree file supplying another leaf's evidence, one directory level down from the hole it
  was written for.
  Acceptance: the check verifies the boxes of the leaf that **owns** the staged change rather than the
  first leaf in the file; its success message is true of what it actually examined; a staged code change
  whose owning leaf carries no ticked, evidence-backed boxes is refused **with that leaf's ID in the
  message**; `--self-test` carries RED arms including (a) a *second* leaf in a file with no boxes while
  the first is complete — the exact shape measured here, (b) a file whose only leaf is complete, and
  (c) the `TEMPLATE.md` exclusion still holding; and the honest limit is stated in the header,
  including that the owning leaf is **not always identifiable from staged paths alone** (the commit
  message names it, and `pre-commit` does not have one) and what the check does when it cannot tell.
  Priority: **high** — this is the gate enforcing root-cause / effect / no-regression evidence on every
  code change; it is unsound for the majority of leaves in every multi-leaf tree file (`M1.md` 33
  leaves, `PROGRAM.md` 23, `M2.md` 10, `S0.md` 9); and a false success message is worse than a silent
  one, because it teaches an author that the box they skipped did not matter. Scheduled **ahead of
  `PROGRAM.18`**, which would otherwise spend effort arming a control that reads the wrong boxes —
  `PROGRAM.18` keeps the other nine.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.10`
  Status: `pending`
  Goal: run the **integration** tier in CI — provision `mdbook` and `qemu-system-riscv64` on the
  runner — and decide the blocking policy for an `incomplete` verdict.
  Acceptance: CI runs `cargo xtask verify --tier integration`; the repository has a recorded,
  reasoned answer to whether exit 20 blocks a build, and the workflow implements that answer.

  ⛔ **The open question is the deliverable, not the YAML.** Wiring it up before deciding
  produces either a permanently red CI that people learn to ignore, or a green one that hides the
  gap — and both are worse than the comment currently in `.github/workflows/rust.yml` saying so.
  The emulator step also cannot pass anywhere until `targets/riscv-virt-up.env` loses
  `TARGET_VERIFIED=no`, which `M2.8` confirms against an installed QEMU — the *installation* is no
  longer the missing piece (director finding 2 is resolved as of `2026-09-27`); the pin and the §3.2
  device-tree agreement check are.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.9`
  Status: `pending`
  Goal: build what the **extended** tier declares but cannot run — a fuzz corpus over the reader
  and the checked arithmetic, a repeatable mutation harness, and Miri wiring (§13.3).
  Acceptance: `cargo xtask verify --tier extended` reports `passed` on a machine with the tools
  installed, and each step fails on a seeded defect; the mutation harness reproduces, by command,
  at least one blind spot that was found by hand (the `lcm`/`max` case in `S0.4` is the
  worked example).

  ⛔ **Opened by `PROGRAM.3`, which is the point of that leaf.** These three gaps existed before
  the runner and were invisible; the runner makes the extended tier report `incomplete` until
  they are closed, so the absence is a routed item rather than a silence.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.4`
  Status: `done`
  Goal: stand up the mdBook structure that mirrors the program: mission, boundary, profile,
  language, engine, analysis, generation, evidence, CLI.
  Acceptance: `make book` builds; every chapter that describes behavior points at the code or
  fixture that implements it.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0033`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — the book had accreted one chapter per leaf into a **flat
    list of eighteen**, with no structure and nothing checking the second half of this leaf's own
    acceptance. Measured at the parent commit: `grep -c '^- \[' docs/book/src/SUMMARY.md` → `18`
    and `grep -c '^# ' docs/book/src/SUMMARY.md` → `1` (the title) — no parts at all. And the
    census that found the real defect, run per chapter over repository-path citations in code
    spans: **`presence.md` cited zero**. A chapter with no anchor cannot be checked against
    anything, by a script or by a reader; it is an essay about a system rather than a description
    of one. WHERE this costs most: `docs/book/` is the director's **only** window into the
    project — the code is not read, the book is.
  - [x] **ADDRESSED (verified)** — `SUMMARY.md` now carries five parts that mirror the programme
    (what eADL describes · writing a description · what the engine may claim · generating and
    running a system · using the toolchain), and `presence.md` names
    `crates/eadl-model/src/presence.rs`, its F04–F06 test file and the worked-case directory.
    The acceptance is now **mechanical**: `scripts/check_book_anchors.sh` →
    `book-anchors: OK (18 chapter(s); every cited repository path resolves)`, registered as a
    project doctrine, so `scripts/check_doctrines.sh` → `=== all doctrines green ===` includes it.
    `mdbook build docs/book` → `INFO HTML book written to`.
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed`; `cargo test --all` → **357**
    passed, `0 failed`, unchanged — this leaf touches no Rust; `scripts/check_doctrines.sh` →
    `=== all doctrines green ===`; `mdbook build docs/book` → OK with the new part structure. No
    chapter's prose was rewritten: the only content change is the "Where it lives" section added
    to `presence.md`.
  - [x] **FIX** — `scripts/check_book_anchors.sh`, with two legs that fail for different reasons.
    **Anchored**: a behavior chapter must cite a repository path. **Resolvable**: every cited path
    must exist — the leg that catches a renamed module or a deleted fixture, where the book keeps
    reading perfectly, which is the problem.
  - [x] **REPRODUCE / ISSUE** — three RED arms, run:
    `scripts/check_book_anchors.sh --self-test` → `book-anchors self-test: 3 pass / 0 fail`. They
    seed an unanchored chapter, a chapter citing `crates/no-such-crate/src/gone.rs`, and — the arm
    that matters as much — a **well-formed** chapter, because a check that always fails is not
    discriminating either.
    ⛔ The false positive was anticipated from `scripts/check_s0_retirement.sh`'s first run and
    designed out: chapters legitimately name `src/main.rs` of a **generated** crate and
    `os-rt.eadl` by basename, and requiring those to exist at the repository root would make
    writing about generated output cost a doctrine breach. The check matches what is
    unambiguously a claim about *this* repository — a path under a tracked top-level directory, or
    a named root document.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` (the registry mirror) and
    `scripts/check_doctrines.project.sh`; `TOOLBOX.md` gains the check as a diagnostic tool;
    `docs/book/src/SUMMARY.md` and `presence.md`; `knowledge-map/subsystems.md`; `MEMORY.md`;
    `LIVE_STATUS.md`; `CHANGELOG.md`; `docs/TASK_TREE.md`. No new `DEV_NOTES.md` heading: the
    transferable rule — match a claim about this repository, not any string that looks like one —
    is the same false positive already recorded for `S0-RETIREMENT`, and designing it out in
    advance is the lesson being *applied* rather than a new one.

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

- ID: `PROGRAM.8`
  Status: `pending`
  Goal: resolve the cross-tree lockstep friction in `TASK-ACCEPTANCE` — the check requires a
  complete acceptance checklist from **every** staged `docs/tasks/*.md`, not from the leaf that
  owns the staged code, so propagating a blocker into a second tree in the same commit as code
  is refused.
  Acceptance: either a declared seam that lets a commit name its owning leaf, or a documented
  convention with a gate that enforces it; the fix must not reopen the cross-file evidence
  leakage the check was hardened against, and must not be a local edit to the portable check.
  Verification: `pending`
  Commit: `pending`

  ### ROUTING EVIDENCE

  - **Does it reproduce outside this project?** Yes. Nothing in the refusal is
    archogen-specific: it fires for any repository using this spine whenever one commit lands
    code owned by tree A and a documentation edit in tree B. Measured here on
    `ARCHOGEN-M0-0009`: `docs/tasks/M5.md` was staged carrying only a blocker note, and the
    check reported `docs/tasks/M5.md has no 'ROOT CAUSE' box in its acceptance checklist`
    ×3 with `=== 1 doctrine breach(es) — commit blocked ===`, while `docs/tasks/M0.md` — the
    tree that actually owns the change — carried a complete, evidenced checklist.
  - **What was measured:** the refusal is a property of the check's *file scope*, not of the
    content. Unstaging `docs/tasks/M5.md` and changing nothing else returns
    `=== all doctrines green ===`.
  - **What would make this routing wrong:** if the strictness is deliberate — i.e. if requiring
    every touched tree to justify itself is the intended cost of the cross-file hardening
    described in the check's own header. That is plausible, and it is why the acceptance above
    forbids any fix that reopens the leakage. The correct first output of this leaf may be an
    upstream report rather than a change.
  - **A third occurrence, and the one that changes the shape of the problem
    (`ARCHOGEN-PROGRAM-0021`):** a *rename* touches every tree that mentions the old name — here
    `M0`, `M1`, `M3`, `M4` and `S0`, none of which owned the change. In the earlier two cases the
    cross-tree edit was incidental and could plausibly have been deferred; for a rename it is
    **unavoidable**, because leaving the old name in five trees is the drift the change exists to
    remove. That also exposed a second-order trap: `M0` and `M1` happen to carry ticked
    checklists from their own earlier leaves, so staging them alongside code would have passed
    the gate on evidence belonging to unrelated work — precisely the incidental pass the
    box-scoping was hardened against. They were unstaged deliberately rather than relied upon.
  - **Interim convention, in use from 2026-09-13:** split the commit. Code and its owning tree
    land together; a documentation edit to another tree lands as its own docs-only commit. See
    `docs/knowledge/cross-tree-lockstep-and-commit-scope.md`.

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
| 1 | `PROGRAM.11` | `active` | the repository-boundary doctrine, **in both directions**. The rule was nowhere in the committed tree, which is how an *inbound* crossing got recorded backwards in a durable record. Priority medium: the outbound half is preventive, the inbound half fired once and was handled correctly |
| 2 | `PROGRAM.21` | `pending` | **high** — `TASK-ACCEPTANCE` reads only the *first* checklist in a tree file, so for every later leaf it verifies nothing and then says it did. Measured: `M1.md` carries 24 ticked ROOT CAUSE boxes and the check's awk captures line 52 (`M1.1`); a staged `.rs` with `M1.24` carrying zero boxes returned `exit=0`. Latent, not active — all five checklist-less committed leaves staged no code. Ahead of `PROGRAM.18`, which would otherwise arm the wrong boxes |
| 3 | `PROGRAM.18` | `pending` | **medium-high** — ten of eighteen registered controls carry no repeatable `--self-test` RED arm, `check_task_acceptance.sh` among them: its box-scoping was validated once during development and no arm re-fires it. Found by the claim-verification adoption's §7.4 sweep, and `PROGRAM.21` is what an arm would have caught |
| 4 | `PROGRAM.5` | `pending` | the §15/§19 dependency and evidence ledger — every external source claim in the book should resolve to a row, and `BOOK-ANCHORS` now checks the *internal* ones. The director has offered a read-only external document source (ISA / RISC-V / devicetree / peripheral specifications) reachable by operator-relayed request; the ledger is where that seam gets a row |
| 5 | `PROGRAM.9` | `pending` | the extended tier reports `incomplete` on every run until its three steps exist |
| 6 | `PROGRAM.6` | `pending` | semantic versioning separation (§15); `cost-accounting/1` and `archogen-provenance/1` are already versioned artifacts waiting for the discipline around them |
| 7 | `PROGRAM.13` | `pending` | twelve closed leaves in `BOOTSTRAP`, `M2` and this tree do not name their own commit — backfill both logs from git, then `PROGRAM.14` gates it so the gap cannot reopen |
| 8 | `PROGRAM.15` | `pending` | a feedback register row that contradicts its own issue sub-tree passes every gate today; five state transitions in six commits held only by hand-editing and a manual census |
| 9 | `PROGRAM.17` | `pending` | the §18 size-containment guide is only partly adopted: `README.md` and `MEMORY.md` are capped and enforced, while `CHANGELOG.md` (1 464 lines), `ROADMAP.md` and `DEV_NOTES.md` have no recorded budget at all |
| 10 | `PROGRAM.20` | `pending` | a **carried-figure register** — the defect class `M1.23`, `M1.24` and `S0.8` are three separate findings of, found by three sweeps whose patterns each missed what the next one caught. Behind `PROGRAM.18`, which gives the older controls the repeatable arms this one arrives with |

`PROGRAM.16` is closed: the claim-verification policy is adopted as `docs/CLAIM_VERIFICATION.md`,
copied verbatim and diff-verified against its read-only source, restated in this project's terms as
its own §7.6 requires, and registered in all three entrypoints — `CLAUDE.md`, `AGENTS.md` and
`DOCTRINE_ENFORCEMENT.md`'s E1 list. The spine is five portable architectures, not four.

`M0` is closed; `S0` is open again for `S0.8` (a book figure `M1.24`'s census found — F28's evidence
is unchanged, so the seven original leaves stay closed); `M1` is open (its LinkedSpec evaluation is
closed, so its frontier is `M1.12`, the language reference) and `M2` is in
progress, so `PROGRAM` carries the substrate work those trees lean on. `PROGRAM.8` remains open and
unblocking. `PROGRAM.12` is closed: harness-local scratch is ignored, so a clean `git status` means
what the handoff rule says it means.

## Decisions

- `2026-09-13`: one tree per roadmap milestone, plus this `PROGRAM` tree for the
  cross-cutting substrate. Rationale: a milestone is the roadmap's own unit of exit-gate
  evidence, so a tree per milestone makes the frontier and the gate the same object.
- `2026-09-13`: the engine carries no external Rust dependencies
  (`docs/decisions/decision_zero-dependency-engine-core.md`). §4.4 makes every dependency
  shared between generator and checker a reviewable trust event, §10.3 requires locked
  offline builds, and §5.5 makes diagnostic wording part of the user contract.
- `2026-09-13`: the §10.2 command surface is declared **once, as data**
  (`crates/archogen-cli/src/spec.rs`); help text is rendered from it and the parser validates
  against it, so documented and accepted options cannot diverge.
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
| `2026-09-13` | `PROGRAM.2` | `make check` + `make gate` + `mdbook build` | `28 tests pass; 13/13 green` |
| `2026-09-27` | `PROGRAM.12` | `git check-ignore -v`, `git status --porcelain`, `git ls-files .qwen`, `make gate` | `ignore matches at .gitignore:31; status carries no untracked row; 0 tracked paths ignored; 13/13 green` |
| `2026-09-27` | `PROGRAM.16` | the policy copied and the copy **diff-verified** against its read-only source rather than read; the §7 adoption sweep run against this repository; all three entrypoints re-grepped afterwards; tiers and the gate re-run | body `diff -q` identical, digest `9f99df25209c43af` on both sides; 406 lines / 27 263 bytes; sweep found 18 checks / 8 with RED arms / **10 without** → `PROGRAM.18`; `AGENTS.md` line 11 carries an explicit list, so it was edited after the leaf's first draft claimed otherwise; 13 doctrines green; `make focused` exit `0`, 421 passed / 0 failed |
| `2026-09-27` | `PROGRAM.19` | the artifact inventory measured before and after; a residue census over every deleted path; the retained vendor build, one instrument self-test, the focused tier and the doctrine gate all re-run afterwards | ≈1.4 GB released: `.app-data` 3.5 GB → 2.2 GB, `target` 815 MB → 799 MB (then 816 MB once the suite recreated its scratch); all 7 deleted paths `gone`; `bins` → both binaries resolve; `reference` → the documented two-form result; `LS-002 --self-test` → `9/9`; 13 doctrines green; `make focused` exit `0`, 421 passed / 0 failed. ⛔ The first post-cleanup tier run **failed** and was reproduced, not dismissed → `S0.7` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROGRAM.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | roadmap seeded into ten trees |
| `PROGRAM.1.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | code-path seam, same commit |
| `PROGRAM.2` | `ARCHOGEN-PROGRAM-0003 (leaf PROGRAM.2)` | `archogen` CLI shell, §5.5 exit codes |
| `PROGRAM.12` | `ARCHOGEN-PROGRAM-0050 (leaf PROGRAM.12)` | harness-local scratch ignored; the stale `.gitignore` pointer to a nonexistent shared `SETUP.md` corrected; `PROGRAM.13`/`.14` logged from the census |
| `PROGRAM.16` | `ARCHOGEN-PROGRAM-0058 (leaf PROGRAM.16)` | the claim-verification policy adopted as `docs/CLAIM_VERIFICATION.md` — copied verbatim and diff-verified, restated locally per its own §7.6, registered in all three entrypoints; its §7.4 sweep found ten controls with no RED arm → `PROGRAM.18` |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0060 (leaf PROGRAM.19)` | ≈1.4 GB of regenerable artifacts released and `docs/ARTIFACT_CLEANUP.md` started, so "is a cleanup due?" is answerable; the cleanup's own verification exposed `S0.7`; one unexpected item investigated and flagged rather than deleted |

## Changelog

- `2026-09-13`: Created task tree; seeded the milestone trees from `ROADMAP.md` revision 2.0.
