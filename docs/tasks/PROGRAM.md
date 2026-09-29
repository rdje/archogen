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
  Children: `PROGRAM.1` … `PROGRAM.26`, plus `PROGRAM.1.1` and `PROGRAM.2.1`

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

  ### Run `2026-09-28` — the second cleanup, under this leaf because the obligation is recurring

  A new leaf per run would flood this tree with one entry per day forever, and the mechanism this leaf
  built is precisely a *latest-only* record with a standing owner. So runs append here and
  `docs/ARTIFACT_CLEANUP.md` stays the answer to "is one due?". Status stays `done`: the leaf's goal was
  the mechanism, and it works — this run is the mechanism running.

  - **Trigger, measured rather than assumed.** `docs/ARTIFACT_CLEANUP.md` recorded `2026-09-27` and the
    session date is `2026-09-28`; the record carries a date and not a time, so "more than 24 hours old"
    is undecidable at that granularity. Cleaned, because the instruction's own tie-break is to clean.
  - **Inventory before touching anything.** `target` 873 MB, of which `target/tmp` 17 MB and
    `target/debug/incremental` 614 MB; `.app-data` 2.2 GB (`target-2ac834913` 2.0 GB,
    `cargo-home` 132 MB, `pgen-generated-before-remeasure` 18 MB); `docs/book/book` 2.3 MB; `build/`
    empty; **715** `.bin` files and **0** `.log` files under `target`.
  - **Deleted, each with a tracked regeneration path.** The test scratch under `target/tmp` — `f28`
    (14 MB), `s0-build` (2.8 MB), `s0-oracle`, `s0-provenance`, `s0-reader`, `s0-build-library.eadl` —
    all recreated by `cargo test`; and one stale `…/archogen_cli-…/s-…-working` incremental directory,
    the residue of an interrupted build. `target` 873 MB → 855 MB, `.bin` count 715 → 693. **Residue
    census: all seven paths report `gone`, none `STILL PRESENT`.** `target/tmp/m112` (44 KB) was
    retained because it is the *active* literal-probe instrument of the session doing the cleanup, not
    residue.
  - ⛔ **Two deletions this run did not make, both after investigation rather than by policy.**
    1. `.app-data/pgen-generated-before-remeasure` — independently re-investigated and re-retained.
       `git grep -rn 'pgen-generated-before-remeasure'` → `LS-004`'s `remeasure.sh:225-227`, which
       treats an existing backup as a reason to **keep** it and prints `a backup already exists … —
       kept`. Deleting it would not merely lose the "before" digest this leaf's checklist already
       recorded; it would change what a *frozen* instrument prints on its next run.
    2. `target/debug/incremental`, 614 MB and 684 of the 693 remaining `.bin` files. Not residue:
       measured at most **four** `s-*` generations per crate across 121 crate directories, which is
       cargo's own retention, and its "regeneration path" is a full rebuild of a 37-suite workspace.
       The standing instruction says to *check* that directory, and checking it produced a reason to
       keep it — deleting live cache to satisfy a word count would slow every subsequent edit loop
       without removing anything stale.
  - **Verified cold, which is the only verification that means anything here.** `make focused` →
    `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built` with the scratch it
    consumes already deleted, so the F28/S0 suites recreated what they need. That is `S0.7`'s lesson
    applied rather than remembered: the first run of the previous cleanup *failed* on exactly this, and
    a warm re-run would have hidden it again. `bash scripts/check_doctrines.sh` → `=== all doctrines
    green ===`. `git status --porcelain` after the deletions → empty, so nothing tracked was touched.
  - **Lockstep.** `docs/ARTIFACT_CLEANUP.md` overwritten with this run only; this section;
    `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`. No frontier move — the frontier stays where `M1`
    left it — and no book change, for the reason recorded above.

  ### Run `2026-09-29` — the third cleanup

  Same standing owner, same mechanism. One difference worth recording: **the unexpected item was
  identified rather than merely retained**, and identifying it is what made it deletable — and is also
  what exposed a hazard this leaf does not own.

  - **Trigger, read off the record rather than assumed.** `docs/ARTIFACT_CLEANUP.md` recorded
    `2026-09-28`, and `git log -1 --format='%ci' -- docs/ARTIFACT_CLEANUP.md` →
    `2026-09-28 03:57:18 +0200` against a session clock of `2026-09-29 10:38 CEST` — more than 24 hours
    on the commit's own timestamp, so the date-only granularity that made the previous run undecidable
    did not have to be guessed at this time.
  - **Inventory before touching anything.** `target` 957 MB, of which `target/tmp` 9 MB,
    `target/doctrine_scratch` 260 KB, `target/sync-backup-2026-09-21` 24 KB, `target/s0-demo` 20 KB, and
    **696** `.bin` files under `target/debug/incremental` plus 9 more under the `no_std` target's; a
    further 15 appeared under `target/tmp/f28` once the suite had run. `.app-data` 2.2 GB
    (`target-2ac834913` 2.0 GB, `cargo-home` 132 MB, `pgen-generated-before-remeasure` 18 MB, `ls004`
    32 KB). `build/` 16 KB. `.log` files: **4** outside the submodule, all under `.app-data/ls004/`, and
    **27** inside `vendor/linkedspec`, which §20 and §21 put out of reach and out of scope.
  - **Deleted, each with a tracked regeneration path.** The twelve scratch directories under
    `target/tmp` — `f28`, `m112`, `m1125`, `m113`, `m1132`, `m1134`, `p21`, `s0-build`,
    `s0-build-library.eadl`, `s0-oracle`, `s0-provenance`, `s0-reader` — of which six are recreated by
    `cargo test` and the `m*`/`p21` ones are prior leaves' probe scratch whose measurements are
    recorded on their leaves (`M1.12`, `M1.13`, `M1.13.2`, `M1.13.4`, `PROGRAM.21`); and
    `target/doctrine_scratch`, which `scripts/check_gap_claims.sh:40` recreates on the next commit.
    `target` 957 MB → 949 MB. **Residue census: all five sampled paths report `gone`, none `STILL
    PRESENT`.** Unlike the previous run, no leaf's *active* scratch was spared: `M1.13.4`'s probe
    directory `m1134` was deleted only after its measurements were written onto the leaf and committed
    in `ARCHOGEN-M1-0089`, so the record outlives the scratch it was taken from.
  - ⛔ **The unexpected item, identified before it was deleted.** `target/sync-backup-2026-09-21/`
    held four spine files — `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `TASK_TREE.md` — that
    match **no** committed state of this repository: `diff` against both `a4cbab5^:COMMIT.md` and
    `a4cbab5:COMMIT.md` differs, and likewise for the other three (`DIFFER` on 4 of 4 both ways). The
    first instinct — retain it, because it is not reproducible from here — was the wrong stopping
    point, and §21 permits reading a sibling repository. The `bedrock` checkout settles it:
    `git -C ../bedrock show HEAD:<path>` is **byte-identical** to all four (`MATCH` on 4 of 4, including
    `docs/TASK_TREE.md` for the backup's `TASK_TREE.md`). So the directory is a copy of the *incoming*
    scaffold and not a backup of archogen's own spine, and its regeneration path is a read of a
    repository that exists on this volume. Deleted, with that evidence.
  - ⭐ **And identifying it exposed a hazard that is not this leaf's, so it is filed rather than fixed
    here.** archogen is on `bedrock-scaffold 0.8.1` (`cat DOCTRINE_VERSION`) where upstream is `0.10.0`,
    and this repository's `scripts/update_scaffold.sh` still `cp`s every NEUTRAL file straight over the
    project's copy — including `docs/TASK_TREE.md`, whose Active Task Trees table *is* project content,
    and `COMMIT.md`, which carries this project's tier paragraph and the no-agent-trailer ruling.
    Upstream fixed exactly that shape (`BEDROCK-MAINTENANCE-0015` and `-0016`: "never overwrites
    anything", "the merge is asked for and never applied"). Owner: **`PROGRAM.26`**.
  - **Two retentions, both new and both investigations rather than policy.**
    1. `build/riscv-virt.dtb` and `build/riscv-virt.dts`, 16 KB. Their regeneration path is
       `scripts/target_emulator.sh --dump-dtb`, which needs the **pinned emulator** to be present, and
       `M2.8.2` is the leaf whose whole subject is comparing a device-tree fixture against them.
       Deleting 16 KB to satisfy a word in the instruction, one leaf before the leaf that needs the
       file, is a bad trade and is recorded as declined.
    2. `target/s0-demo/base`, 20 KB. A **closed** leaf cites it as verification evidence
       (`docs/tasks/S0.md:181`: `archogen build examples/s0-heartbeat/system.eadl --out
       target/s0-demo/base` → `exit=0`), and recreating it is a full `archogen build` rather than a
       `cargo test` — so it does not meet the bar the six `s0-*` scratch directories meet.
    The previous run's two retentions were re-checked and stand: `.app-data/pgen-generated-before-remeasure`
    (18 MB) is still read by `LS-004`'s `remeasure.sh`, whose own behaviour changes if the backup is
    absent, and `target/debug/incremental` is still cargo's retention rather than residue.
    `.app-data/ls004/`'s four `.log` files are retained with it: they are a frozen instrument's RED-arm
    output, and no document cites them, so nothing here can prove they are regenerable.
  - **Verified cold, which is the only verification that means anything here.** `make focused` →
    `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built`, exit `0`, with the scratch
    it consumes already deleted; `cargo test --all` → **492 passed, 0 failed**, matching the recorded
    baseline; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`, and
    `target/doctrine_scratch/gap_claim_census` exists again afterwards, which is the proof that the
    deleted directory was scratch and not state. `target` measured 965 MB after these runs — **larger**
    than before the deletion — because the suite recreated its scratch and cargo its cache, which is
    the same observation the first run made and the reason deleting them was safe.
  - **Lockstep.** `docs/ARTIFACT_CLEANUP.md` overwritten with this run only; this section;
    `LIVE_STATUS.md`'s `PROGRAM` row; `CHANGELOG.md`. No frontier move and no book change, for the
    reason the first run recorded: `git grep -ln 'app-data' -- docs/book` → no match, and the book
    documents eADL and the engine, not the repository's scratch directories.

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
  looked at the index that restates it. Corrected by `M1.24`'s routing edit rather than left. A fifth
  shape, `2026-09-28` (`M1.12.2`): `docs/semantics/grammar.md` headed a list **"Three rules the
  productions above imply"** above *four* numbered items — a **normative document**, and not one of
  the surfaces this leaf's acceptance enumerates (book chapters, corpus indexes, crate module headers),
  so no existing gate would have caught it and no sweep pattern aimed at it. Fixed by deleting the
  count rather than retyping it, which is the register's own prescription.
  A sixth shape, `2026-09-29` — found while recovering a force-quit session, and ⛔ **it is not a
  figure at all**, so a register scoped to figure-shaped text would not see it. Three stale copies of
  an *ordered sequence* and of a *list head*, left by the two commits that closed `PROGRAM.21` and
  `M1.13.3`: `docs/TASK_TREE.md`'s `PROGRAM` row still named `PROGRAM.21` as the frontier one commit
  after that leaf's own status became `done`, and both that row and `LIVE_STATUS.md`'s `M1` row named
  the frontier as its own successor ("Then `.13.4` … `.13.5`"), because a closure rewrote the frontier
  sentence and left the old successor clause behind. Census, re-runnable both ways:

  ```text
  census:   awk -F'|' '/^\| \[`/{print $2, substr($4,1,14)}' docs/TASK_TREE.md, against each
            docs/tasks/<TREE>.md "## Current Frontier" order-1 row
            -> 10 of 12 rows AGREE, pre-correction; PROGRAM DISAGREE (index `PROGRAM.21`,
               tree `PROGRAM.11`). After this commit's correction: 11 of 12, and the only
               DISAGREE left is the artifact below.
               M5's cell carries no head at all, so its DISAGREE is the pattern reading the
               blocker the cell names (`M0.5`) — recorded, not quietly dropped
  census:   git ls-files '*.md' | xargs grep -n -o "[Tt]hen `\.13\.4\`"
            -> LIVE_STATUS.md:15 and docs/TASK_TREE.md:57 only; no other tree's row repeats
               its own head, and MEMORY.md / docs/tasks/M1.md already read `.13.4` then `.13.5`
  evidence: git show --stat 7c59b0b -- docs/TASK_TREE.md | wc -l  -> 0
            the commit that closed PROGRAM.21 staged no line of the index that restates its
            frontier — the same evidence shape as the S0 row above
            (git show --stat 91f329d -- docs/TASK_TREE.md -> 1 insertion, the M1 row it rewrote)
  ```

  Corrected in `ARCHOGEN-PROGRAM-0088`, docs only, no code staged. ⚠️ The paragraph above ended
  "classifying **five** shapes by hand", which made the register's own description carry a figure — the
  class, restated inside the leaf that exists to end it. ⛔ It went stale one shape later, so the count
  is **deleted** here rather than retyped, which is this leaf's own prescription for exactly this.

  A seventh shape, `2026-09-29` — found by `M1.13.4`'s measurement (its M-I: the identifier retrofit
  executed on all 62 suite files and reverted), and ⛔ **not a figure about the repository at all**: a
  line number **inside a quoted transcript**. `grep -rn '^[0-9]\+ |' docs/book/src/*.md` → 14 lines over
  8 chapters (`boundary.md:88`, `checking.md:66` and `:70`, `modules.md:73` and `:77`, `presence.md:67`
  and `:71`, `quantities.md:62`, `reading.md:84`, `:99` and `:103`, `s0.md:218` and `:222`,
  `workload.md:94` and `:98`), each one a diagnostic rendering copied out of a real run. Editing the
  description a transcript quotes moves every line number in it, and nothing compares a transcript to
  the run it claims to be — so the shape is invisible to both existing sweep patterns (`N of M`, digits
  followed by a size noun) and to every gate in the repository. It is the `M1.23`/`M1.24` class in the
  one live surface the register's acceptance already names (book chapters) but no sweep has aimed at.
  ⚠️ **Recorded, not fixed here**: `M1.13.4.2` re-renders the transcripts its own insertion moves and
  states on its leaf that the class stays open. Whether the register should require a transcript to be
  *generated* rather than copied — which is the only fix that cannot rot — is this leaf's design work.
  Each sweep is a population bounded by its pattern, so "nothing else found" has never been a result
  this repository could rely on — and a third sweep, adding `leaves|arms|checks|doctrines|productions|rows`,
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
  ⚠️ **Widened `2026-09-29` on the sixth shape.** Everything above enumerates *figure-shaped text*, and
  the shape that just fired is an ordered **sequence**: an index row naming a leaf whose own status is
  `done`, and a successor clause naming the frontier as its own successor. Neither contains a number
  that moved, so a digits-or-spelled-out census passes the commit that produces them. The registered
  check must therefore also compare a restated list head against the tree file it indexes (and a
  successor clause against the head it follows), and `--self-test` must carry an arm whose subject is a
  sequence rather than a count — priced here rather than discovered by the seventh shape.
  Priority: **medium** — it is the structural fix for the most frequently recurring defect class in
  this repository, and it is what stops `M1.23`/`M1.24`/`S0.8` from being followed by an `M2.x`. It
  gates no milestone and blocks nothing, which is why it is scheduled behind `PROGRAM.11` and
  `PROGRAM.18` rather than ahead of them; `PROGRAM.18` (repeatable RED arms for registered controls)
  should land first, because this check arrives with arms and the older ten do not.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.21`
  Status: `done`
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
  ⭐ **Severity as first measured: latent, not active.** Auditing every leaf that records a
  commit found five with no ticked ROOT CAUSE box — `M0.1`, `M0.2`, `M1.18`, `M1.20`, `PROGRAM.1` — and
  `git show --stat` on each of their commits reports **0** code files: `M1.18` staged twelve `.md`
  files, `M0.1`/`M0.2`/`PROGRAM.1` none, and `M1.20` is an aggregation node whose seven sub-leaves
  each carry their own checklist. So no code change had landed unboxed. The defect was that the gate
  **could not have stopped one** on any leaf but the first in its file, while telling the author it had.

  ⛔ **That severity claim is SUPERSEDED `2026-09-28`: the defect is active, and this is measured rather
  than inferred.** Commit `cd355ef` (`ARCHOGEN-M1-0080`, leaf `M1.13.1`) staged five Rust files —
  `crates/eadl-front/src/form.rs`, `crates/eadl-front/src/reader.rs` and three test files — and the gate
  printed `task-acceptance: OK (every staged code-change leaf carries a ticked, evidence-backed
  checklist)` with `exit=0`. What it actually read was **`docs/tasks/M1.md` line 53**, inside leaf
  **`M1.1`**, written `2026-09-13`. Three measurements, all re-runnable:

  ```text
  census: grep -cE '^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*\*\*ROOT CAUSE' docs/tasks/M1.md
          → 31 ticked ROOT CAUSE boxes in the file (24 when this leaf was written); exactly one is read
  census: the check's own awk over docs/tasks/M1.md with kw="root.?cause"
          → captures line 53, ticked=1 — leaf M1.1's box, not M1.13.1's at line ~1470
  mutation: M1.13.1's ROOT CAUSE box unticked in a scratch copy, the same awk re-run over both files
          → identical capture, line 53 ticked=1 in BOTH, so M1.13.1's boxes cannot affect the verdict
  ```

  ⚠️ **The mutation is the finding, so read it before reading the reassurance.** Nothing bad happened in
  `cd355ef`: `M1.13.1`'s checklist was written, ticked and evidence-backed, so the commit was honest and
  the verdict was right **for the wrong reason**. Had those boxes been empty the verdict would have been
  byte-identical, which means the gate supplied that commit no protection and told its author it had.
  Every code commit on every leaf of `M1.md` after `M1.1` sits in the same position — 30 of the file's
  31 boxes belong to leaves the gate never reads — and `M1.md` is the tree the project is working in.
  ⛔ Interim mitigation, stated so it is not mistaken for a fix: there is none inside the gate. The only
  thing standing between this and an unboxed code commit is the author's own discipline, which is
  precisely what a gate exists to not depend on.
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

  ### Design pinned by measurement `2026-09-29` — implementation NOT started, and this is the resume point

  ⛔ **The obvious design is dead, measured rather than argued.** Identifying the owning leaf from
  *which leaf sections a commit's tree-file diff touches* was priced against the last seven code
  commits, comparing the touched set with the `(leaf X)` token in each subject:
  ```text
  3a6bbb9 M1.13.2 → touched M1.13,M1.13.2                  DISAGREE
  cd355ef M1.13.1 → touched M1.13.1,M1.26                   DISAGREE
  e4212c4 M1.12.5 → touched M1,M1.12,M1.12.5,M1.26,PROGRAM.24  DISAGREE
  64e35c4 M1.12.4 → touched M1.12.4                         agree
  951f6c0 M1.12.3 → touched M1.12.3,M1.12.4,M1.12.5         DISAGREE
  690f5c6 M1.12.2 → touched M1.12.2,PROGRAM.20              DISAGREE
  29154da M1.12.1 → touched M1.12.1,M1.12.2                 DISAGREE
  → agree 1 / disagree 6 / no-map 0
  ```
  A commit legitimately touches its own leaf, its parent, the frontier, and every leaf it routes a
  finding to, so the touched set is not the owner and narrowing it (e.g. "touched sections that carry a
  checklist") still leaves `M1.13.2` and `M1.13` both live. **Do not re-derive this; it is measured.**

  ⭐ **The chosen identification chain, and why it fails closed:**
  1. `TASK_ACCEPTANCE_LEAF=<ID>` — for a commit made with no message file (an IDE dialog, `-m`).
  2. the **subject** of the pending commit message, via a new seam `.doctrine/commit_message_file`
     (one line, default `git_message_brief.txt`) so the template stays project-neutral. `COMMIT.md`
     step 4 already writes that file before step 6's `git commit -F`, so it exists at `pre-commit` time.
  3. ⛔ **otherwise REFUSE.** Do not fall back to the first checklist in the file — that fallback *is*
     the defect. A green verdict about a leaf nobody claimed is worse than no verdict, because the
     author reads it as protection. The refusal must name what it tried and both ways to satisfy it.
     Consequence to state in the header: `make gate` with staged code and no message file now fails,
     which enforces the mandated workflow rather than quietly excusing its absence.

  **Scoping mechanism:** slice the leaf by *exact string* match on ``- ID: `<owner>` `` (not a regular
  expression — an id's `.` would match another leaf's), running to the next flush-left `- ID: ` or
  `## `, then run the existing three-box awk over that slice alone. **Success message must name what it
  read** — `leaf <ID> in <file>` — because criterion 2 is that the message be true of what was examined.
  Also refuse when the named leaf is in **no staged** tree file, which catches a stale message file
  left over from a previous commit.

  **`--self-test` arms, each in a throwaway `git init` repo under `mktemp -d` so the real index is never
  touched** (the house pattern in `check_book_anchors.sh` cannot be copied directly, because this check
  reads `git diff --cached`): (1) first leaf complete, second leaf owns the change and has no checklist
  — the measured defect — must **fail** naming the second; (2) same fixture, complete leaf named, must
  **pass**, proving arm 1 fails for the right reason; (3) named leaf with unticked boxes fails;
  (4) no owner identifiable fails closed; (5) a stale message file naming an unstaged leaf fails;
  (6) the env override beats the message file; (7) `TEMPLATE.md` staged beside a complete leaf still
  passes; (8) code with no leaf staged still fails with `NO owning task-tree leaf`; (9) the OK message
  names the leaf it read.

  ⚠️ **Two traps for whoever resumes.** (a) The new check gates **its own** commit: the hook runs the
  working-tree script, so this leaf must carry ticked, evidence-backed `ROOT CAUSE` / `ADDRESSED` /
  `NO REGRESSION` boxes before the commit can land — which is the right self-test, not an obstacle.
  (b) `ADDRESSED` should re-run the two historical scenarios (`cd355ef`, `3a6bbb9`) as fixtures rather
  than asserting from memory, since those are the commits the defect was measured on.

  ⛔ **A full draft was written and then REVERTED, deliberately, rather than committed unverified.**
  It parsed (`bash -n` clean) but its `--self-test` was never run, and committing an untested gate that
  fails closed would have blocked every subsequent code commit. A copy sits at
  `target/tmp/p21/check_task_acceptance.draft.sh` — **scratch, and the next artifact cleanup will
  delete it**, so the design above is the durable artifact and the draft is only a head start.
  `scripts/check_task_acceptance.sh` is byte-identical to `HEAD` (`diff -q` against
  `git show HEAD:…` silent). Lockstep owed with the fix: `DOCTRINE_ENFORCEMENT.md` §4's description of
  this control, `.doctrine/README.md` for the new seam, `TOOLBOX.md`, and `PROGRAM.18`'s note that ten
  controls lack a RED arm (this one gains nine).
  ✅ **That checkpoint is superseded the same day: the draft was installed, its arms were run, and the
  fix landed.** The lockstep listed above is discharged in this commit, and the design was followed
  except where measurement corrected it — see the NO REGRESSION box, where the arms' own oracle turned
  out to be unsound in the same way the check was.

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE` — which, from this commit, enforces it *here*)

  - [x] **ROOT CAUSE (WHY + WHERE)** — **WHERE:** `scripts/check_task_acceptance.sh`, the per-keyword
    `awk` that extracts a box's bullet: `if (inbox) exit` stops at the *next* box bullet, and the awk is
    run once over the whole tree **file**, so a file holding N leaves yields exactly one box per keyword
    — whichever leaf comes first. **WHY it survived:** the check had already been hardened twice, for
    cross-**file** and incidental-**prose** leakage, and its own header claimed box-scoping was "the
    soundness property"; cross-**leaf** leakage is the same hole one directory level down, and the
    success message asserted the opposite of what was examined, so a green run taught the author the
    box they skipped did not matter. Measured at `HEAD`, not recalled:
    ```text
    $ git show HEAD:scripts/check_task_acceptance.sh | grep -n "if (inbox) exit"
    111:          if (inbox) exit
    $ grep -cE '^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*\*\*ROOT CAUSE' docs/tasks/M1.md
    32                                        ← 32 ticked boxes in the file; the awk reads ONE
    $ <HEAD's own awk, kw="root.?cause", over docs/tasks/M1.md>
    53:   - [x] **ROOT CAUSE (WHY + WHERE)** — §12 M1's exit gate is that "invalid examples produce
                                                ↑ leaf M1.1, written 2026-09-13
    ```
    ⛔ **And the fix's premise was priced before it was chosen.** Identifying the owner from the leaf
    sections a commit touches — the only signal inside the staged paths — was measured against seven
    real code commits and agrees with the `(leaf X)` subject token on **1 of 7** (`3a6bbb9` touched
    `M1.13`+`M1.13.2`; `e4212c4` touched five leaves across two trees). So the owner comes from the
    author's own declaration and the check refuses when there is none; the table is in the design
    section above.
  - [x] **FIX** — `scripts/check_task_acceptance.sh` rewritten leaf-scoped, plus the new seam
    `.doctrine/commit_message_file`. (1) The owner is `TASK_ACCEPTANCE_LEAF`, else the `(leaf <ID>)`
    token in the pending message's **subject**, else **refuse** — no fallback to the first checklist,
    because that fallback *is* the defect. (2) The leaf's section is sliced by **exact string** match on
    ``- ID: `<owner>` `` (not a regular expression, so an id's `.` cannot match another leaf's) running
    to the next flush-left `- ID: ` or `## `, and the three-box awk now runs over that slice alone.
    (3) A named leaf found in **no staged** tree file is refused, which catches a stale message file.
    (4) The OK message names the leaf and the file it read. (5) `--self-test` with nine RED arms, each in
    a throwaway `git init` repo under `mktemp -d`, because the house pattern in `check_book_anchors.sh`
    cannot be copied by a check that reads `git diff --cached`.
  - [x] **ADDRESSED (verified)** — both commits the defect was measured on, **replayed as fixtures**
    rather than asserted from memory: each commit's own `docs/tasks/M1.md` and one of its staged source
    files extracted with `git show <commit>:<path>` into a throwaway repo, its `.doctrine/` seams copied,
    and a brief naming the leaf that commit claimed. Then the same fixture with that leaf's ROOT CAUSE
    box unticked. `HEAD`'s check and the new one run over both:
    ```text
    cd355ef  pristine  OLD exit=0 OK | NEW exit=0  → "OK (leaf M1.13.1 in docs/tasks/M1.md — …)"
    cd355ef  mutated   OLD exit=0 OK | NEW exit=1  → "leaf M1.13.1 — the 'ROOT CAUSE' box is present
                                                      but NOT ticked."
    3a6bbb9  pristine  OLD exit=0 OK | NEW exit=0  → "OK (leaf M1.13.2 in docs/tasks/M1.md — …)"
    3a6bbb9  mutated   OLD exit=0 OK | NEW exit=1  → "leaf M1.13.2 — the 'ROOT CAUSE' box is present
                                                      but NOT ticked."
    ```
    ⭐ Read the `mutated` rows as the finding: the OLD verdict is **byte-identical** pristine and
    mutated, so those two commits' own boxes provably could not change it, while the NEW verdict moves
    and names the right leaf. Both leaves were honest, which is why `pristine` passes on both — the
    defect was never a bad commit, it was a gate that supplied no protection and said it had.
    `bash scripts/check_task_acceptance.sh --self-test` → **9 pass / 0 fail**, `exit=0`.
    `bash -n` clean; `make gate` → `=== all doctrines green ===` over 13 checks; `make focused` run
    below. ⚠️ This commit is the new check's first real exercise: it gates itself, so the verdict it
    prints is about *this* leaf's boxes and not `PROGRAM.1`'s.
    ⭐ **And it refused this very commit before the brief was written** — the fail-closed path exercised
    live rather than only in a fixture, with the real staged set (11 files, `scripts/…` among them):
    ```text
    $ git add <the 11 files> && bash scripts/check_doctrines.sh      # git_message_brief.txt still empty
      ❌ TASK-ACCEPTANCE
       TASK-ACCEPTANCE: a CODE change is staged but the check CANNOT TELL WHICH LEAF owns it.
         It does not guess. Falling back to the first checklist in the file is how this check
         came to print OK having read a different leaf's boxes, and a green verdict about a leaf
         nobody claimed is worse than no verdict: the author reads it as protection.
         Name the owning leaf either way:
           • write the commit message to 'git_message_brief.txt' with '(leaf <ID>)' in its SUBJECT, or
           • TASK_ACCEPTANCE_LEAF=<ID> <your commit command>
         tried: TASK_ACCEPTANCE_LEAF (unset), 'git_message_brief.txt' (exists but its subject names no leaf)
    ```
    `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built`.
  - [x] **NO REGRESSION** — three mutations, each restoration proven byte-identical with `diff -q`
    against the saved copy (`target/tmp/p21/new.sh.orig`), per
    `docs/knowledge/verify-the-mutation-applied.md`:
    ```text
    mutation A: replace the fail-closed refusal with the OLD behaviour — take the first leaf in the file
      → self-test `8 pass / 1 fail`: arm 4 (no owner identifiable) goes green, so the refusal is
        load-bearing and an arm proves it
    mutation B: point the arms' invocation at a file that does not exist, exact oracle kept
      → self-test `0 pass / 9 fail`: every arm reports that the check never ran
    mutation B2: the same break, with the oracle weakened from `rc -eq 1 && grep TASK-ACCEPTANCE` to
      `rc -ne 0`
      → self-test `4 pass / 5 fail`, and the four ✅ are arms 1/3/4/5 **reported as passes on
        `exit 127`** — the false green reproduced deliberately
    restored: `diff -q` silent; self-test `9 pass / 0 fail`, `exit=0`
    ```
    ⛔ **B2 is the reason this leaf found a second defect, in its own new code.** The arms as first
    written used `rc -ne 0` as the pass condition, and their first run scored **4 passes on `exit 127`**
    — `$0` was a relative path and every arm `cd`s into a throwaway repo, so the check was never found.
    A refusal and a failure to exec are different claims, and an oracle that accepts "not success"
    cannot tell them apart; the tally still read like partial success. Fixed by requiring the subject's
    **own** exit code and its **own** identifying output, and by resolving the invocation path against
    the directory the caller stood in. Promoted into `docs/knowledge/verify-the-mutation-applied.md`,
    which now names all three parts of an arm that can be weaker than the property: the needle, the
    mutation, and the oracle.
  - [x] **LOCKSTEP** — `scripts/check_task_acceptance.sh` (header now names three leakage holes, the
    leaf-scoping rationale, the fail-closed rule and its three honest limits);
    `.doctrine/commit_message_file` (new seam, with the 1-of-7 measurement as its reason);
    `.doctrine/README.md` (the seam table, "in both" → "in all three"); `DOCTRINE_ENFORCEMENT.md` §4's
    `TASK-ACCEPTANCE` row (two holes → three, the identification rule, the refusal, nine arms);
    `TOOLBOX.md` (a row for the check and its `--self-test`);
    `docs/knowledge/verify-the-mutation-applied.md` (new section + `answers:` line + a How-to-apply
    bullet); this leaf, this tree's frontier and both logs; `MEMORY.md`, `LIVE_STATUS.md`,
    `CHANGELOG.md`, `DEV_NOTES.md`. ⛔ `KNOWLEDGE_MAP.md` was regenerated and came out **unchanged** —
    `git diff --stat HEAD -- KNOWLEDGE_MAP.md` empty, and `grep -c` for the new `answers:` line in the
    map returns 0, so the map does not index a card's `answers:` and adding one moves nothing. Recorded
    rather than claimed, because "regenerated the map" reads as "the map changed". ⭐ `PROGRAM.18`'s
    population moves: it counted ten registered controls with no repeatable RED arm,
    `check_task_acceptance.sh` among them — this one now has nine, so `PROGRAM.18` keeps the other nine
    and must re-run its census rather than reuse the figure.
  Verification: see the acceptance checklist above.
  Commit: `ARCHOGEN-PROGRAM-0086 (leaf PROGRAM.21)`

- ID: `PROGRAM.22`
  Status: `done`
  Goal: the roadmap's own worked example dirties the working tree. `ROADMAP.md` §10.1 writes to
  `build/` (`archogen resolve … --out build/plan.json`, `archogen build … --out
  build/periodic-three`), and so do `docs/book/src/targets.md`, `docs/book/src/s0.md`,
  `docs/book/src/verification.md`, `docs/targets/first-target.md` and
  `scripts/target_emulator.sh --dump-dtb` — but `build/` was **not** in `.gitignore`, so following
  the documentation left an untracked directory and broke the meaning of a clean `git status`.
  Reproduce / issue: measured while taking the device-tree dump for
  [[decision_emulator-independence-retained]] — `mkdir -p build && scripts/target_emulator.sh
  --dump-dtb build/riscv-virt.dtb`, then `git status --short` → `?? build/`. `cat .gitignore`
  listed `/target`, `/generated`, `/docs/book/book`, `/.app-data/`, `docs/feedback/**/.repro-work/`
  and `/.qwen/`, and no `build`. Impact: `PROGRAM.12` established that scratch is "never committed —
  and never left untracked either, because 'handoff-ready' means a clean `git status`"; this was the
  same defect one level up, in a path the *roadmap itself* tells a reader to write to. It also makes
  §8's artifact-cleanup question ambiguous, since a cleanup cannot tell ignored output from stray
  output.
  Fix: `/build` added to `.gitignore` with the reason and this leaf named beside it, following
  `PROGRAM.12`'s precedent exactly. The dumped device tree stays on disk — it is `M2.8`'s input and
  is regenerated by one command — but no longer dirties the tree.
  Priority: **low**, fixed immediately because it blocked handoff-readiness in the same session it
  was found, and because `.gitignore` is not a code path under `.doctrine/code_paths.txt`
  (`printf '.gitignore\n' | grep -Ef <(grep -vE '^\s*(#|$)' .doctrine/code_paths.txt)` → no match),
  so no acceptance checklist was owed.
  Verification: `git status --short` after the change → empty, with `build/riscv-virt.dtb` and
  `build/riscv-virt.dts` still present on disk (`git check-ignore -v build/riscv-virt.dtb` names the
  new rule); `make gate` → `=== all doctrines green ===` over 13 checks, `exit=0`.
  Commit: `ARCHOGEN-PROGRAM-0067 (leaf PROGRAM.22)`

- ID: `PROGRAM.23`
  Status: `pending`
  Goal: make the ruled push cadence **enforced rather than prose** — one machine-readable threshold, a
  check that reports the live count against it, and `MEMORY.md`'s layer-A field filled in. Ruled
  `2026-09-28` at `N = 400` commits ahead of `origin/main`; see [[decision_push-cadence]].
  Reproduce / issue: `MEMORY_ARCHITECTURE.md:193` provides the slot — `(ahead of origin: <N>; push at
  ~<threshold>)` — and archogen never filled it. `grep -rniE 'rev-list --count|origin/main|
  push.threshold|ahead of origin' scripts/*.sh xtask/src/main.rs` → **no match**, so nothing mechanical
  has ever read a threshold. The cadence existed only as an operator batch instruction (BWFSC, default
  100 slices) that the PNT loop can never trigger, because PNT has no fixed BWFSC by definition; the
  branch went unpushed from `32e6b14` (`2026-09-13`) at ≈4.9 commits/day. Fourth instance in a week of
  a rule that lives only in prose — [[a-rule-only-in-the-prompt-is-enforced-nowhere]].
  Acceptance: the threshold has **one** producer, a machine-readable location the check reads, with
  `MEMORY.md` *reporting* it rather than restating it and no document carrying a second copy of the
  number; the check reports the live `git rev-list --count origin/main..HEAD` against the threshold and
  is registered in `TOOLBOX.md` and `DOCTRINE_ENFORCEMENT.md`'s E1 list; it arrives with repeatable
  `--self-test` RED arms to `PROGRAM.18`'s standard rather than joining its backlog, including an arm
  at exactly the threshold and one above it; the **live count appears in no tracked document**, since
  it moves with every commit; and ⛔ it **cannot deadlock the repository** — a blocking verdict at `N`
  while `make integration` fails would leave the tree able neither to commit nor to push, because
  `COMMIT.md` step 2 requires `make integration` before a push and that tier currently fails on the
  emulator step. So this leaf is sequenced **after `PROGRAM.10`** reclassifies that step, or ships as a
  loud non-blocking report until it has.
  Priority: **medium** — it makes a director ruling durable and observable, and it is the mechanism
  that stops the cadence becoming the next rule that exists only in a prompt. Behind `PROGRAM.10` for
  the deadlock reason above.
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

  ⛔ **Measured `2026-09-28`: the verdict this leaf must rule on has already changed shape, and the
  change is what blocks pushing.** When this leaf was written the emulator step reported
  `Unavailable` → tier `incomplete` → exit `20`, because the tool was absent. QEMU is now installed,
  so `scripts/target_emulator.sh --check` reaches the unpinned-config branch and exits `1`, and the
  runner maps "ran, nonzero" to `Failed` → tier `failed` → `make integration` exit `1`. That matters
  because `COMMIT.md` step 2 treats the two differently: it explicitly permits proceeding past
  **incomplete** after reading what it names ("*not* a pass and *not* a failure: nothing broke, and
  something could not be run"), and it does **not** permit proceeding past **failed**. So the branch
  has been unpushed since `origin/main`'s `32e6b14` (`2026-09-13`) over a *classification*, not over
  a broken build — `git rev-list --count origin/main..HEAD` for the live number, which moves with
  every commit and is therefore deliberately not written down here.
  ⭐ **And §14.3 already supplies the mechanism, unused.** "A required tool skipped or unavailable is
  reported as such, not a passed check. Quarantine requires a named issue, owner, affected claim, and
  bounded scope." The runner already has the vocabulary — `Outcome::{Passed, Failed, Unavailable,
  NotBuilt}` and `Action::NotBuilt { owner, note }`, which five steps use under `PROGRAM.9`. "QEMU is
  present; the release is unpinned and `DEVICE_TREE_FIXTURE` does not exist, so the §3.2 agreement
  check could not be run" is *incomplete*, quarantined with owner `M2.8`, affected claim
  `target-verification`, bounded to that one step. This leaf's deliverable is therefore not only the
  CI provisioning and the blocking policy — it is also making the step report the verdict §14.3's own
  vocabulary already has a word for.
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

- ID: `PROGRAM.24`
  Status: `pending`
  Goal: the **mirror direction** of `BOOK-ANCHORS`. That doctrine walks the book and asks whether what
  it cites exists; nothing walks the codebase and asks whether the book describes it. The standing
  instruction is that the roadmap, the codebase and the mdBook stay in lockstep, and the book is the
  director's only window into the project — so an undocumented capability is not a documentation nit.
  It is a feature that, as far as the only reader of it is concerned, does not exist.
  Reproduce / issue: **one live instance, measured `2026-09-28` by `M1.12.5`.** The `rt-analysis`
  crate is named nowhere in `docs/book/`, and `docs/book/src/analysis.md` — the chapter titled *"What
  the scheduling checker establishes"*, which is precisely what that crate does — cites only
  `ROADMAP.md` and `docs/analysis/cost-accounting-v1.md`. It passes `BOOK-ANCHORS` on both legs, and
  that is the finding rather than an excuse: leg 1 asks for *a* repository path and the chapter has
  two, leg 2 asks whether they exist and they do. A chapter can be perfectly anchored and still never
  tell its reader where the thing it describes is implemented.

  ```text
  census: for c in crates/*/; do n=$(basename "$c"); printf '%-22s %s\n' "$n" \
            "$(grep -roF "$n" docs/book/src/ | wc -l | tr -d ' ')"; done
          → archogen-cli 2 · archogen-evidence 1 · archogen-s0 2 · eadl-front 1 · eadl-model 3 ·
            rt-analysis **0** · rt-core 4 · rt-reference 1     (7 of 8 workspace crates named)
  census: grep -rn "rt-analysis" docs/book/                     → no match, exit 1
  census: grep -noE '`(crates|docs|scripts)/[A-Za-z0-9_./-]+`|`ROADMAP\.md`' docs/book/src/analysis.md
          → 4 citations, none inside the crate the chapter is about
  census: grep -rln "docs/book" scripts/ crates/*/tests/*.rs xtask/src/
          → check_readme_stability.sh · check_book_anchors.sh · corpus.rs · reference.rs · kinds.rs ·
            differential.rs · xtask/src/main.rs. Every one starts FROM the book and asks about the
            code; none starts from the code and asks about the book.
  census: grep -n '^members' Cargo.toml                         → members = ["crates/*", "xtask"]
  ```

  Acceptance: the population is **derived from the root manifest's workspace members**, never listed
  inside the check, so a crate added tomorrow is in scope without anyone editing a gate; the rule is
  the stronger of the two shapes below, or the weaker one with the reason for stopping there recorded
  in this leaf —
  1. *weak*: every workspace member is named in at least one chapter. Cheap, and it catches the live
     instance, but a list of crate names in an appendix satisfies it and is worth nothing — the same
     token-citation trap `check_book_anchors.sh`'s own header refuses to set.
  2. *strong*: every workspace member is named in a chapter that **also cites a repository path inside
     that member**. Mechanical, and not satisfiable by an appendix, because a bare name carries no path
     with it. ⭐ This is the shape to build unless measurement says it cannot be met honestly.

  Plus: the `rt-analysis` instance fixed in the same commit, with `analysis.md` naming the crate and
  the fixtures that establish its claims; RED arms in the `--self-test` idiom, each pinning its
  violation count, including one that proves a name-without-a-path does **not** satisfy the strong
  shape; `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md` and `scripts/check_doctrines.project.sh` updated if
  this becomes a doctrine rather than a test; `make focused` exit `0`.

  ⛔ **Deliberately out of scope: `ROADMAP.md` → book.** Nothing gates that direction either, and this
  leaf must not "complete" itself by adding one. The roadmap is direction and exit criteria; the book
  is a description of what the toolchain does for a user. Requiring every roadmap section to appear in
  the book would produce a chapter per milestone and teach exactly the token-mention habit the strong
  shape exists to refuse. Recorded so the absence stays a decision. Where the roadmap *is* load-bearing
  as data, it is already read as data and gated: `crates/rt-analysis/tests/f18_baseline.rs` and
  `f29_preemption.rs` parse §13.2's table out of `ROADMAP.md` and fail loudly if it will not parse,
  because a baseline that quietly shrank would still be green.
  Priority: **medium-high** — a live drift in the one artifact the director reads, and no mechanism in
  the tree that could have found it. Sequenced behind `PROGRAM.21` and `PROGRAM.18` only because those
  are about gates reporting that they checked something they did not, which corrupts every other
  gate's evidence including this one's.
  Verification: `pending`
  Commit: `pending`

- ID: `PROGRAM.25`
  Status: `done`
  Goal: **own the two findings raised in conversation on `2026-09-28`/`29` and left unowned.** §15 is
  explicit that reporting an issue for the director to review, or mentioning it in a summary, is
  incomplete — an issue raised and not owned is a complaint. Both of these were raised in a reply and
  recorded nowhere in the tracked tree, so if the session ended they would have ended with it.
  Reproduce / issue: two separate failures of the same rule, and the second one is the worse shape.
  **(1)** Answering a question about the target ISA, `read_file` on
  `$ARCHOGEN_CHIPDOC_ROOT/sifive/fe310/current/FE310-G002_datasheet_v1p2.pdf` returned
  `pdftotext is not installed. Install poppler-utils…`, and that was reported onward as "this machine
  has no PDF text extractor, so §3.2's board facts cannot be read here" — with a promise to log it as
  its own commit. ⛔ **The premise is false, and measuring before filing is what caught it:**
  ```text
  $ command -v pdftotext pdftk mutool qpdf gs
  /opt/homebrew/bin/pdftotext
  /opt/homebrew/bin/pdftk
  $ pdftotext -v | head -1
  pdftotext version 4.06 [www.xpdfreader.com]
  $ pdftotext -f 1 -l 6 <that datasheet> - | head -1
  SiFive FE310-G002 Datasheet v1p2            ← the route works
  ```
  So the host has an extractor (Xpdf's, not poppler's) and the **`read_file` PDF bridge cannot see the
  host `PATH`**. The consequence is the *opposite* of the one reported: chipdoc's board PDFs **are**
  readable here via `run_shell_command`, and §3.2's `board-first` rows are **not** blocked on tooling.
  Census of what that unblocks: `sifive/fe310/current` holds **3** PDFs and `sifive/hifive1/current`
  **2**, all named as *present* by `docs/decisions/reference_external-document-source-chipdoc.md`, which
  says nothing about whether they can be read — so the record needs the route as well as the inventory.
  **(2)** Told that a sibling repository `../semulith` exists, the reply analysed it and then, on being
  told the analysis was not what was asked for, deleted it and reported "nothing filed, nothing changed
  in the repo". Correct response to "don't analyse it"; wrong response to §9, which requires a novel
  finding to be recorded in a **durable, tracked file** so it survives the message. Census, both
  directions, so the claim is checkable:
  ```text
  $ git grep -il "semulith" -- ':!vendor' | wc -l
  0                     ← archogen's own tracked files name it nowhere
  $ grep -ril "semulith" vendor/ | wc -l
  40                    ← the vendored LinkedSpec submodule names it, as a fellow consumer
  ```
  ⛔ Note the census trap this records: `vendor/linkedspec` is a **submodule** (`.gitmodules`), so
  `git grep` skips it entirely and a `git grep` alone would have reported "nothing anywhere mentions
  it" — the two projects have already met, through the vendor's own issue ledger, and archogen's side
  of that meeting never says so.
  Acceptance: `TOOLBOX.md` carries the PDF route with the misreport named, so the next session does not
  re-conclude the datasheets are unreadable; `reference_external-document-source-chipdoc.md` gains the
  readability fact beside its inventory; a `docs/decisions/reference_*` record states what `semulith`
  is, that it is **read-only** under §21, and which archogen leaves would need to know it exists —
  with no analysis of it, which is what was declined; the record is indexed; `KNOWLEDGE_MAP.md`
  regenerated; `make gate` green.
  Priority: **medium** — neither finding blocks the current frontier. Filed ahead of it anyway, because
  the alternative is a false statement ("no PDF extractor here") standing as the last word in a
  conversation nobody can search, and a sibling project that names archogen as its consumer while
  archogen's tree has never heard of it.
  Verification: docs-only, so no tier step governs the change itself; what was measured is in the
  Reproduce block above and was re-run rather than recalled — `command -v pdftotext` →
  `/opt/homebrew/bin/pdftotext`, `pdftotext -v` → `version 4.06 [www.xpdfreader.com]`, and
  `pdftotext -f 1 -l 6 <the FE310-G002 datasheet> -` → its first line, so the route is proven and not
  assumed. Both censuses re-run: `git grep -il semulith -- ':!vendor' | wc -l` → **0**,
  `grep -ril semulith vendor/ | wc -l` → **40**, and `.gitmodules` confirms `vendor/linkedspec` is a
  submodule, which is why the first census alone would have been a false negative. Every citation in
  the new record resolves (`decision_repository-boundary-read-only`,
  `decision_emulator-independence-retained`, `reference_external-document-source-chipdoc`,
  `an-oracle-is-independent-by-construction`), and the semulith path is written as `../semulith` rather
  than an absolute one so §12 holds if the repository moves volume. `make gate` → `13/13 green`;
  `KNOWLEDGE_MAP.md` regenerated for the new record.
  Commit: `ARCHOGEN-PROGRAM-0083 (leaf PROGRAM.25)`

- ID: `PROGRAM.26`
  Status: `pending`
  Goal: make `make update-scaffold` unable to destroy project content, by adopting the upstream fix
  rather than inventing one — this repository's `scripts/update_scaffold.sh` is two minor versions
  behind `bedrock`, and the version it has `cp`s every neutral spine file straight over the project's
  copy.
  Reproduce / issue: **measured `2026-09-29` by `PROGRAM.19`'s third cleanup run**, which found
  `target/sync-backup-2026-09-21/` — four spine files matching no committed state of this repository —
  and identified them as byte-identical to `bedrock`'s `HEAD` copies. Identifying them is what exposed
  the hazard beside them.

  ```text
  census: cat DOCTRINE_VERSION                                -> bedrock-scaffold 0.8.1
          cat ../bedrock/DOCTRINE_VERSION   (read-only, §21)  -> bedrock-scaffold 0.10.0
  census: grep -n 'cp "\$tmp/bedrock/\$f"' scripts/update_scaffold.sh
          -> one unconditional `cp` per NEUTRAL file, 25 entries in the array, no comparison and no
             refusal; the script's own header calls them "safe to overwrite because it never carries
             project content"
  census: the array includes docs/TASK_TREE.md, COMMIT.md, DOCTRINE_ENFORCEMENT.md, TOOLBOX.md and
          .doctrine/README.md — and this repository's copies of at least the first two DO carry project
          content: docs/TASK_TREE.md's Active Task Trees table is the index a resuming session reads
          first, and COMMIT.md carries the §14.3 tier paragraph and the ⛔ no-agent-trailer ruling
          (maintainer ruling 2026-08-22, ported by BEDROCK-MAINTENANCE.2.5)
  census: git -C ../bedrock log --oneline -8 | grep -i overwrite
          -> BEDROCK-MAINTENANCE-0015 (.2.10) "update_scaffold.sh never overwrites anything"
             BEDROCK-MAINTENANCE-0016 (.2.11) "never overwrite is now auditable, and the merge is
             asked for and never applied"
             BEDROCK-MAINTENANCE-0014 (.2.9)  "a spine file that carries a project decision can now
             reach a project that predates it"
  ```

  Impact: running `make update-scaffold` today would replace this project's task-tree **index** with the
  template's empty one and its commit workflow with the neutral text, and the only thing standing
  between that and a commit is the script's closing advice to "review `git diff`". Nothing mechanical
  refuses it. The `2026-09-21` sync survived because whoever ran it kept a copy of the incoming files
  in `target/` — a hand-made mitigation, in a scratch directory that any `cargo clean` deletes, which
  is how the cleanup run found it.
  Acceptance: `scripts/update_scaffold.sh` is replaced by **upstream's** version at a named `bedrock`
  revision (copied in, per §12's exception for read-only external sources — never depended on at build
  time, and `bedrock` is not a submodule), so the fix is adopted rather than re-derived; the adoption
  is recorded in a `docs/decisions/` record naming the revision and what it changes; a **dry run** on
  this repository is measured and its output recorded on this leaf — what it would touch, what it
  refuses to touch, and what it asks for; `DOCTRINE_VERSION` states the adopted revision truthfully
  afterwards, whether or not the rest of `0.10.0` is adopted in the same commit; the other neutral files
  are **censused rather than assumed** — each one diffed against upstream and classified as
  *identical*, *project-carrying* or *behind*, so adopting the script is not confused with adopting the
  whole spine; RED arm: a seeded project-carrying spine file survives a dry run and is named in its
  output; `make gate` green.
  Priority: **medium** — it fires only on a deliberate sync, and the last one was `2026-09-21`. Filed
  now rather than then-because the cost of being wrong is the task-tree index, and because the
  mitigation currently in use is a directory inside `target/`.
  ⚠️ **Interim mitigation, until this leaf lands:** do not run `make update-scaffold`. If a sync becomes
  necessary before then, run it on a clean tree, `git diff` every one of the 25 files before staging,
  and keep the incoming copies somewhere tracked rather than under `target/`.
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
| §10.4 programmatic interface — engine API, wasm binding, MCP server | [`API`](API.md) | added by director ruling `2026-09-28`; §10.2's CLI stays the human interface and becomes a consumer of the same contract |
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
| 1 | `PROGRAM.11` | `active` | **the frontier leaf** — the repository-boundary doctrine, **in both directions**. The rule was nowhere in the committed tree, which is how an *inbound* crossing got recorded backwards in a durable record. Priority medium: the outbound half is preventive, the inbound half fired once and was handled correctly |
| 2 | `PROGRAM.18` | `pending` | **medium-high** — **nine** of eighteen registered controls carry no repeatable `--self-test` RED arm. ⛔ The figure this row carried was **ten**, and `PROGRAM.21` moved it: `check_task_acceptance.sh` now ships nine arms, so `PROGRAM.18` must **re-run its census** rather than reuse the number — the exact defect class `PROGRAM.20` exists to catch. Its own reason for being sequenced behind `PROGRAM.21` is discharged: arming the boxes is now meaningful because the check reads the right ones |
| 3 | `PROGRAM.24` | `pending` | **medium-high** — the mirror direction of `BOOK-ANCHORS`, which nobody checks: does a capability the codebase has get described in the book? One live instance measured `2026-09-28`: the `rt-analysis` crate is named nowhere in `docs/book/`, and `analysis.md` — the chapter about what it establishes — cites only `ROADMAP.md` and a cost-accounting doc, passing both `BOOK-ANCHORS` legs while never telling its reader where the code is. Filed by `M1.12.5` on the director's lockstep instruction |
| 4 | `PROGRAM.5` | `pending` | the §15/§19 dependency and evidence ledger — every external source claim in the book should resolve to a row, and `BOOK-ANCHORS` now checks the *internal* ones. The director has offered a read-only external document source (ISA / RISC-V / devicetree / peripheral specifications) reachable by operator-relayed request; the ledger is where that seam gets a row |
| 5 | `PROGRAM.9` | `pending` | the extended tier reports `incomplete` on every run until its three steps exist |
| 6 | `PROGRAM.6` | `pending` | semantic versioning separation (§15); `cost-accounting/1` and `archogen-provenance/1` are already versioned artifacts waiting for the discipline around them |
| 7 | `PROGRAM.13` | `pending` | twelve closed leaves in `BOOTSTRAP`, `M2` and this tree do not name their own commit — backfill both logs from git, then `PROGRAM.14` gates it so the gap cannot reopen |
| 8 | `PROGRAM.15` | `pending` | a feedback register row that contradicts its own issue sub-tree passes every gate today; five state transitions in six commits held only by hand-editing and a manual census |
| 9 | `PROGRAM.17` | `pending` | the §18 size-containment guide is only partly adopted: `README.md` and `MEMORY.md` are capped and enforced, while `CHANGELOG.md` (1 464 lines), `ROADMAP.md` and `DEV_NOTES.md` have no recorded budget at all |
| 10 | `PROGRAM.20` | `pending` | a **carried-figure register** — the defect class `M1.23`, `M1.24` and `S0.8` are three separate findings of, found by three sweeps whose patterns each missed what the next one caught. ⭐ A **near-miss** worth pricing in, recorded honestly as a near-miss and not as a fourth instance: `PROGRAM.18`'s frontier row still read "ten of eighteen controls" while this leaf was being written, and was corrected in the same commit — it would have gone stale the moment the fix landed, and nothing in the tree compares a control count against the controls. Behind `PROGRAM.18`, which gives the older controls the repeatable arms this one arrives with |
| 11 | `PROGRAM.10` | `pending` | run the integration tier in CI **and reclassify the emulator step's verdict** — it moved from `incomplete` (exit `20`, tool absent) to `failed` (exit `1`, config unpinned) when QEMU was installed, and `COMMIT.md` step 2 permits proceeding past the first but not the second. §14.3's quarantine clause and the runner's existing `NotBuilt { owner, note }` vocabulary already supply the mechanism; this is what the push precondition is actually waiting on |
| 12 | `PROGRAM.23` | `pending` | make the ruled push cadence (`400` commits ahead, `2026-09-28`) enforced rather than prose — one machine-readable threshold, a check reporting the live count against it, `MEMORY.md`'s layer-A field filled. **Behind `PROGRAM.10`**: a blocking verdict at N while `make integration` fails would leave the tree able neither to commit nor to push |
| 13 | `PROGRAM.26` | `pending` | **medium** — `make update-scaffold` can currently destroy project content: this repository's `scripts/update_scaffold.sh` is `bedrock-scaffold 0.8.1` where upstream is `0.10.0`, and it `cp`s all 25 neutral spine files over the project's copies with no comparison and no refusal — including `docs/TASK_TREE.md`, whose Active Task Trees table is the index a resuming session reads first, and `COMMIT.md`, which carries this project's tier workflow. Upstream fixed exactly that shape (`BEDROCK-MAINTENANCE-0015`/`-0016`), so the fix is an adoption and not an invention. Found by `PROGRAM.19`'s third cleanup identifying a hand-made backup of the incoming files parked in `target/`. Sequenced last because it fires only on a deliberate sync and the last one was `2026-09-21`; the interim mitigation is on the leaf |

**`PROGRAM.21` is closed: `TASK-ACCEPTANCE` verifies the leaf that owns the change, and refuses when it
cannot tell which one that is.** The hole was cross-**leaf** leakage — one awk over the whole tree file,
stopping at the first box, so a file of N leaves verified whichever came first and then printed that it
had checked the staged change. Measured active on two real commits, and replayed here as fixtures rather
than recalled: `cd355ef` and `3a6bbb9` each return a **byte-identical** verdict from `HEAD`'s check
pristine and with the committing leaf's own ROOT CAUSE box unticked, while the new check moves and names
the right leaf. The owner is taken from the author's declaration — `TASK_ACCEPTANCE_LEAF`, else the
`(leaf <ID>)` token in the pending message's subject — and **never inferred from the staged paths**,
because that signal was priced against seven real code commits and agrees on **1 of 7**. Nine `--self-test`
RED arms, each in a throwaway repository. ⛔ The arms' own first oracle was unsound in the same way the
check was: `rc -ne 0` scored **four passes on `exit 127`**, a check that was never found, and the tally
still read like partial success — reproduced deliberately as mutation B2 and promoted into
`docs/knowledge/verify-the-mutation-applied.md`, which now names all three parts of an arm that can be
weaker than the property.

`PROGRAM.16` is closed: the claim-verification policy is adopted as `docs/CLAIM_VERIFICATION.md`,
copied verbatim and diff-verified against its read-only source, restated in this project's terms as
its own §7.6 requires, and registered in all three entrypoints — `CLAUDE.md`, `AGENTS.md` and
`DOCTRINE_ENFORCEMENT.md`'s E1 list. The spine is five portable architectures, not four.

`M0` is closed; `S0` is open again for `S0.8` (a book figure `M1.24`'s census found — F28's evidence
is unchanged, so the seven original leaves stay closed); `M1` is open with its frontier at `M1.13`,
the language freeze — `M1.12` closed `2026-09-28` when its fifth child delivered the reference's
lockstep; and `M2` is in progress, so `PROGRAM` carries the substrate work those trees lean on.
`PROGRAM.8` remains open and unblocking. `PROGRAM.12` is closed: harness-local scratch is ignored, so
a clean `git status` means what the handoff rule says it means.

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

- **What is the push cadence?** The layer-A template provides the field — `MEMORY_ARCHITECTURE.md:193`,
  `(ahead of origin: <N>; push at ~<threshold>)` — and `MEMORY.md` has never filled it in. Nothing
  mechanical reads it: `grep -rniE 'rev-list --count|origin/main|push.threshold|ahead of origin'
  scripts/*.sh xtask/src/main.rs` → no match. The doctrine is qualitative only: "**Push regularly** —
  the remote is your crash insurance; an unpushed commit dies with the machine" (`:239`), "The single
  point of failure is **not committing / not pushing**" (`:432`). The only numeric threshold anywhere
  is the operator's *batch* rule (BWFSC, default 100 slices) — and the PNT loop has no fixed BWFSC by
  definition, so under PNT that trigger can never fire. That is the structural reason the branch went
  unpushed from `origin/main`'s `32e6b14` (`2026-09-13`) at ≈4.9 commits/day — `git rev-list --count
  origin/main..HEAD` for the live number, which is deliberately not written down because it moves
  with every commit.
  **Ruled `2026-09-28`: `N = 400` commits.** See Decisions below and
  [[decision_push-cadence]]. The recommendation put to the director was `25` commits or `7` days,
  whichever came first; `400` was chosen, and the consequence at the measured rate is recorded in the
  decision rather than argued again here. Enforcement is `PROGRAM.23`, because an ungated threshold
  is the unenforced-prose class `docs/knowledge/a-rule-only-in-the-prompt-is-enforced-nowhere.md`
  describes — and `MEMORY.md`'s template field now carries the number.

## Blockers

- **The branch cannot be pushed under `COMMIT.md`'s own precondition**, and the cause is a verdict
  *classification*, not a broken build: `make integration` exits `1` (`failed`) on the emulator step
  because `QEMU_VERSION_PINNED=none-yet`, while `COMMIT.md` step 2 permits proceeding past
  `incomplete` (exit `20`) after reading what it names, and does not permit proceeding past `failed`.
  `PROGRAM.10` owns the reclassification (§14.3's quarantine clause, with the runner's existing
  `NotBuilt { owner, note }` vocabulary); `M2.8` owns removing the cause. **Not urgent**: the cadence
  is now ruled at `400` commits and the branch is far below it, so no push is due — but it must be
  cleared before the threshold is reached, or the cadence becomes unpayable exactly when it fires.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-13` | `PROGRAM.1` | `scripts/check_doctrines.sh` | `13/13 green` |
| `2026-09-13` | `PROGRAM.1.1` | `scripts/check_doctrines.sh` staged | `red → green` |
| `2026-09-13` | `PROGRAM.2` | `make check` + `make gate` + `mdbook build` | `28 tests pass; 13/13 green` |
| `2026-09-27` | `PROGRAM.12` | `git check-ignore -v`, `git status --porcelain`, `git ls-files .qwen`, `make gate` | `ignore matches at .gitignore:31; status carries no untracked row; 0 tracked paths ignored; 13/13 green` |
| `2026-09-27` | `PROGRAM.16` | the policy copied and the copy **diff-verified** against its read-only source rather than read; the §7 adoption sweep run against this repository; all three entrypoints re-grepped afterwards; tiers and the gate re-run | body `diff -q` identical, digest `9f99df25209c43af` on both sides; 406 lines / 27 263 bytes; sweep found 18 checks / 8 with RED arms / **10 without** → `PROGRAM.18`; `AGENTS.md` line 11 carries an explicit list, so it was edited after the leaf's first draft claimed otherwise; 13 doctrines green; `make focused` exit `0`, 421 passed / 0 failed |
| `2026-09-27` | `PROGRAM.19` | the artifact inventory measured before and after; a residue census over every deleted path; the retained vendor build, one instrument self-test, the focused tier and the doctrine gate all re-run afterwards | ≈1.4 GB released: `.app-data` 3.5 GB → 2.2 GB, `target` 815 MB → 799 MB (then 816 MB once the suite recreated its scratch); all 7 deleted paths `gone`; `bins` → both binaries resolve; `reference` → the documented two-form result; `LS-002 --self-test` → `9/9`; 13 doctrines green; `make focused` exit `0`, 421 passed / 0 failed. ⛔ The first post-cleanup tier run **failed** and was reproduced, not dismissed → `S0.7` |
| `2026-09-28` | `PROGRAM.19` (second run) | the trigger read off `docs/ARTIFACT_CLEANUP.md` rather than assumed; a full inventory taken before any deletion; a residue census over every deleted path; `git grep` for each retention candidate's consumers; the focused tier re-run **cold**, with the scratch it consumes already deleted | ≈18 MB released: `target` 873 MB → 855 MB, `.bin` files 715 → 693, all 7 deleted paths `gone`; **2.2 GB retained on evidence** — `pgen-generated-before-remeasure` is read by `LS-004`'s `remeasure.sh:225-227`, and `target/debug/incremental` holds at most four generations per crate across 121 directories, which is cargo's retention and not residue; `make focused` → `passed — 3 / 0 / 0` cold; 13 doctrines green; `git status --porcelain` empty after the deletions |
| `2026-09-28` | `PROGRAM.21` | **no code changed — this is the finding's severity re-measured, not its fix.** Three instruments over the commit that had just landed: a `grep -c` census of ticked ROOT CAUSE boxes in `docs/tasks/M1.md`; the check's **own awk**, extracted from `scripts/check_task_acceptance.sh` and run over the file with `kw="root.?cause"`, printing the line it captures and whether that box is ticked; and a **mutation** — `M1.13.1`'s ROOT CAUSE box unticked in a scratch copy under `target/tmp/m113/`, with the same awk re-run over both files and the captures compared | the census returns **31** ticked boxes where this leaf recorded 24; the awk captures **line 53, ticked=1**, which is leaf `M1.1`'s box from `2026-09-13`, while `M1.13.1`'s sits near line 1470; and the mutation gives an **identical capture for both files**, so `M1.13.1`'s boxes provably cannot affect the verdict for commit `cd355ef` — five Rust files staged, `task-acceptance: OK`, `exit=0`. ⛔ The severity claim on this leaf is therefore superseded from *latent* to **active**: the gate read a thirteen-day-old checklist belonging to a different leaf and reported that it had checked the staged change |
| `2026-09-29` | `PROGRAM.25` | **docs-only, and the point of the run was to falsify the finding before filing it.** `command -v pdftotext pdftk mutool qpdf gs`; `pdftotext -v`; `pdftotext -f 1 -l 6` on the FE310-G002 datasheet to stdout; both semulith censuses (`git grep` excluding `vendor/`, and a filesystem walk of `vendor/`); `.gitmodules`; and a resolution check on every citation the new record makes | ⛔ **The finding as raised was false and the measurement is what caught it**: `pdftotext` resolves to `/opt/homebrew/bin/pdftotext` (Xpdf **4.06**), and the datasheet's first page extracts, so chipdoc's board PDFs are readable here and §3.2's `board-first` rows are **not** blocked on tooling — `read_file`'s bridge cannot see the host `PATH`, and its "not installed" message describes the bridge. A blocker on tree `M5` was drafted on that false premise and is **not** filed; what is filed is the route, in `TOOLBOX.md` and beside the chipdoc record's inventory. Semulith census: **0** archogen tracked files name it, **40** files inside the vendored submodule do, and `.gitmodules` confirms `vendor/linkedspec` is a submodule — so `git grep` alone returns a false negative and the census needs both halves. All four citations resolve; `make gate` → `13/13 green` |
| `2026-09-29` | `PROGRAM.21` | the check rewritten leaf-scoped, then **its own RED arms run before anything was claimed** — and the arms' first oracle found unsound, so three mutations were run over the finished script (`bash -n` first, then `--self-test`); both historical commits **replayed as fixtures** built from `git show <commit>:<path>` in throwaway repos, each run pristine and then with the committing leaf's ROOT CAUSE box unticked, against `HEAD`'s check and the new one side by side; `make focused`; `make gate`; the knowledge map regenerated | ⛔ **The arms' first run scored `4 pass / 5 fail` and the four passes were on `exit 127`** — the check was never found, because `$0` was relative and every arm `cd`s into a throwaway repo, and the oracle was `rc -ne 0`. Mutation B2 reproduces that false green deliberately (`4 pass / 5 fail`, arms 1/3/4/5 ✅ on 127) while mutation B, with the exact oracle, gives `0 pass / 9 fail`. Restored: `diff -q` silent, `9 pass / 0 fail`, `exit=0`. Mutation A (restore the first-leaf fallback) → `8 pass / 1 fail`, arm 4 only, so the refusal is load-bearing. **Replay, the finding:** `cd355ef` and `3a6bbb9` both give OLD `exit=0 OK` **pristine and mutated** — byte-identical, so neither commit's own boxes could affect the verdict — while NEW gives `exit=0` pristine naming `M1.13.1`/`M1.13.2` and `exit=1` mutated naming the same leaf. Root cause confirmed at `HEAD`: `if (inbox) exit` at line 111, **32** ticked ROOT CAUSE boxes in `docs/tasks/M1.md`, and that awk captures **line 53** — leaf `M1.1`, `2026-09-13`. `make focused` → `passed — 3 / 0 / 0`; `make gate` → `13 doctrines green`; ⚠️ the draft had also silently **widened `DEFAULT_SIG`** with a token that does not exist (`\bspindb\b`) and mis-dated a historical comment — both caught by diffing the preserved blocks against `HEAD` and restored byte-identical |
| `2026-09-29` | `PROGRAM.20` | **docs only, no code staged — a sixth shape of the class recorded, and its three copies corrected.** A census over the frontier surfaces rather than the two lines spotted by reading: an `awk` over `docs/TASK_TREE.md`'s row heads against each `docs/tasks/<TREE>.md` `## Current Frontier` order-1 row; `git ls-files '*.md'` piped to `grep -n -o` for successor clauses; `git show --stat 7c59b0b -- docs/TASK_TREE.md` for the copy the closing commit never touched. Recovery post-conditions after the force-quit measured rather than assumed: `git status --short` empty, `git rev-parse HEAD` = `91f329d`, `wc -c git_message_brief.txt` = `0` and untracked, `git config core.hooksPath` = `.githooks`. Then the baseline re-derived rather than carried: `cargo test --all` → **492 passed, 0 failed, 37 suites**; `make focused` → `passed — 3 / 0 / 0`; `scripts/check_doctrines.sh` → `13/13 green`; `scripts/check_no_background_jobs.sh` → `handoff: OK` | ⛔ The class fired a **sixth** time and the shape is new: not a figure but an ordered **sequence**. `docs/TASK_TREE.md`'s `PROGRAM` row named `PROGRAM.21` as the frontier one commit after that leaf's status became `done` — `7c59b0b` staged **0** lines of the index that restates it, the same evidence shape as the `S0` instance above — and both that row and `LIVE_STATUS.md`'s `M1` row named `.13.4` as its own successor, because a closure rewrote the frontier sentence and left the old clause behind. **10 of 12** index rows agreed with their tree file before the correction and **11 of 12** after; `M5`'s `DISAGREE` is the census pattern reading the blocker its cell names, recorded rather than dropped. Nothing in the tree compares an index row against the tree it indexes, and this leaf's acceptance enumerated figure-shaped text only — so it is widened on the leaf rather than left to miss the shape that just fired |
| `2026-09-29` | `PROGRAM.19` (third run) | the trigger read off the record's own commit timestamp rather than its date-only line; a full inventory before any deletion; a residue census over the deleted paths; the unexpected item **identified against a read-only sibling repository** before deletion; the focused tier, the whole suite and the doctrine gate all re-run **cold** with the scratch they consume already deleted | ≈11 MB released: `target` 957 MB → 949 MB, then 965 MB once verification recreated the scratch; all five sampled paths `gone`. `target/sync-backup-2026-09-21` matched **no** committed state here (`DIFFER` on 4 of 4 against both `a4cbab5^:` and `a4cbab5:`) and was proven **byte-identical to `bedrock`'s `HEAD`** copies (`MATCH` on 4 of 4), so it was the incoming scaffold and not a backup — deleted on that evidence, and identifying it exposed `PROGRAM.26`. **2.2 GB retained on evidence**, with two new retentions: `build/riscv-virt.dtb` needs the pinned emulator to regenerate and `M2.8.2` compares against it, and `target/s0-demo/base` is cited by a closed leaf and needs a full `archogen build`. `make focused` → `passed — 3 / 0 / 0`; `cargo test --all` → **492 passed, 0 failed**; 13 doctrines green, with `target/doctrine_scratch` recreated by the run |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROGRAM.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | roadmap seeded into ten trees |
| `PROGRAM.1.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | code-path seam, same commit |
| `PROGRAM.2` | `ARCHOGEN-PROGRAM-0003 (leaf PROGRAM.2)` | `archogen` CLI shell, §5.5 exit codes |
| `PROGRAM.12` | `ARCHOGEN-PROGRAM-0050 (leaf PROGRAM.12)` | harness-local scratch ignored; the stale `.gitignore` pointer to a nonexistent shared `SETUP.md` corrected; `PROGRAM.13`/`.14` logged from the census |
| `PROGRAM.16` | `ARCHOGEN-PROGRAM-0058 (leaf PROGRAM.16)` | the claim-verification policy adopted as `docs/CLAIM_VERIFICATION.md` — copied verbatim and diff-verified, restated locally per its own §7.6, registered in all three entrypoints; its §7.4 sweep found ten controls with no RED arm → `PROGRAM.18` |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0060 (leaf PROGRAM.19)` | ≈1.4 GB of regenerable artifacts released and `docs/ARTIFACT_CLEANUP.md` started, so "is a cleanup due?" is answerable; the cleanup's own verification exposed `S0.7`; one unexpected item investigated and flagged rather than deleted |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0074 (leaf PROGRAM.19, second run)` | ≈18 MB more released, and **2.2 GB retained on evidence**: a frozen instrument's backup, which `remeasure.sh` treats an existing copy of as a reason to keep it, and cargo's own incremental cache. Runs append to the standing owner rather than becoming one leaf per day |
| `PROGRAM.21` | `ARCHOGEN-PROGRAM-0081 (leaf PROGRAM.21)` | **filed, not fixed** — the severity re-measured from *latent* to **active** on commit `cd355ef`: five Rust files staged, `task-acceptance: OK`, and the box it read was leaf `M1.1`'s from `2026-09-13`. A mutation proves `M1.13.1`'s own boxes could not have changed the verdict. No code in this commit, so the gate stays unsound and the fix is still owed |
| `PROGRAM.25` | `ARCHOGEN-PROGRAM-0083 (leaf PROGRAM.25)` | two findings raised in conversation and owned by nothing are now tracked — census, both halves, because `git grep` alone is a false negative here: `git grep -il semulith -- ':!vendor' \| wc -l` → **0** archogen tracked files, `grep -ril semulith vendor/ \| wc -l` → **40** files inside the submodule, which `.gitmodules` confirms `git grep` skips. And one of the two findings was **false as raised**: `pdftotext` is on the host (`/opt/homebrew/bin`, Xpdf 4.06) and chipdoc's board datasheets extract fine, so `read_file`'s "not installed" is a bridge limitation and §3.2's `board-first` rows are not tooling-blocked. The route is a `TOOLBOX.md` row and a note beside the chipdoc inventory; an `M5` blocker drafted on the false premise is **not** filed. `docs/decisions/reference_sibling-project-semulith.md` records that `../semulith` exists, is read-only, and names archogen as its consumer — a pointer with no analysis, which is what was declined |
| `PROGRAM.21` | `ARCHOGEN-PROGRAM-0086 (leaf PROGRAM.21)` | **fixed, not filed**: `TASK-ACCEPTANCE` is leaf-scoped. The owner comes from `TASK_ACCEPTANCE_LEAF` or the `(leaf <ID>)` token in the pending message's subject through the new `.doctrine/commit_message_file` seam, and the check **refuses** when it cannot tell — never falling back to the first checklist, because that fallback *was* the defect. The staged-paths signal was priced first and is dead: **1 of 7** real code commits. Both historical commits replayed as fixtures, pristine and mutated, old check against new: the old verdict is byte-identical either way, the new one moves and names the right leaf. Nine `--self-test` arms; ⛔ their first oracle scored **4 passes on `exit 127`** and the false green is reproduced deliberately as mutation B2, promoted into `verify-the-mutation-applied`. This commit is the new check's first real exercise — it gated itself |
| `PROGRAM.20` | `ARCHOGEN-PROGRAM-0088 (docs)` | **an instance recorded, not the register built** — three stale frontier copies corrected, in `docs/TASK_TREE.md` (the `PROGRAM` head, which named a `done` leaf, and the `M1` successor clause) and `LIVE_STATUS.md` (the `M1` successor clause), and the sixth shape filed on the leaf with its census and its acceptance widened: an ordered sequence, which a register scoped to figure-shaped text could not have seen |
| `PROGRAM.19` | `ARCHOGEN-PROGRAM-0090 (leaf PROGRAM.19, third run)` | ≈11 MB more released and **2.2 GB retained on evidence**. The run's finding is not a deletion but an identification: a directory that matched no committed state here was proven byte-identical to the upstream scaffold's, so it was deleted on evidence — and reading it exposed that this repository's `update_scaffold.sh` is two minor versions behind a fix upstream made to exactly that hazard → `PROGRAM.26` |

## Changelog

- `2026-09-13`: Created task tree; seeded the milestone trees from `ROADMAP.md` revision 2.0.
