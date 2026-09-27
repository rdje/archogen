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
  Children: `PROGRAM.1` (+ `PROGRAM.1.1`) … `PROGRAM.10`, and `PROGRAM.2.1`

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
  Status: `pending`
  Goal: make the **repository boundary** a stated, mechanically checked doctrine: no archogen
  agent writes into another git repository or into a vendored submodule's own history, and a
  vendored checkout that carries local commits or local modifications fails the gate.
  Acceptance: the rule is stated where a resuming agent reads it (`CLAUDE.md`/`AGENTS.md` and
  `DOCTRINE_ENFORCEMENT.md`, not only in a session prompt); a `scripts/check_*.sh` registered in
  `scripts/check_doctrines.project.sh` fails on a seeded local commit and on a seeded local
  modification inside `vendor/`, and passes on a clean pin; the check has RED arms in
  `--self-test`; `docs/decisions/decision_repository-boundary-read-only.md` is linked from it.
  Priority: **high** — this is the defect class that already fired once (§15 obliges the fix, not
  just the record).

  ⛔ **Opened by a published upstream disclosure, not by speculation.** LinkedSpec
  `8b5b5ffd8ea415b9b6d97387da8289e8f18606f3` (`2026-09-27`, "RGX-CONSUMER-BUILD-REPORTS.1.3.1 —
  record publication and repository-boundary violation") discloses "the unauthorized ARCHOGEN
  documentation commit and auxiliary writes". Measured in this repository: `git log --oneline --
  vendor/linkedspec` → two commits, neither of which wrote inside the submodule, and
  `git log --all --grep=LINKEDSPEC` → one commit. So the crossing is not recorded here — which is
  itself the finding: **the rule was nowhere in the committed repository.**
  `grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md` → no match. An agent resuming from git alone would
  never learn the boundary existed. Root cause and the permitted/unpermitted split are in
  `docs/decisions/decision_repository-boundary-read-only.md`.

  ⚠️ **Honest limit to state up front:** a gate in *this* repository cannot prevent a write into a
  checkout elsewhere on the filesystem. What it can do is (a) put the rule where every agent reads
  it, and (b) detect the symptom that is visible from here — a vendored submodule carrying local
  commits or modifications. The leaf must not claim more than that.
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
  `TARGET_VERIFIED=no`, which needs an installed QEMU to confirm it (director finding 2).
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
| 1 | `PROGRAM.11` | `pending` | the repository-boundary doctrine. **High** because the defect class already fired: upstream published a crossing, and the rule that forbids it was nowhere in the committed tree — so every agent resuming from git alone was unaware of it |
| 2 | `PROGRAM.5` | `pending` | the §15/§19 dependency and evidence ledger — every external source claim in the book should resolve to a row, and `BOOK-ANCHORS` now checks the *internal* ones |
| 3 | `PROGRAM.9` | `pending` | the extended tier reports `incomplete` on every run until its three steps exist |
| 4 | `PROGRAM.6` | `pending` | semantic versioning separation (§15); `cost-accounting/1` and `archogen-provenance/1` are already versioned artifacts waiting for the discipline around them |

`M0` and `S0` are closed; `M1` is open again (its frontier is `M1.20`, then `M1.12`) and `M2` is in
progress, so `PROGRAM` carries the substrate work those trees lean on. `PROGRAM.8` remains open and
unblocking.

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `PROGRAM.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | roadmap seeded into ten trees |
| `PROGRAM.1.1` | `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` | code-path seam, same commit |
| `PROGRAM.2` | `ARCHOGEN-PROGRAM-0003 (leaf PROGRAM.2)` | `archogen` CLI shell, §5.5 exit codes |

## Changelog

- `2026-09-13`: Created task tree; seeded the milestone trees from `ROADMAP.md` revision 2.0.
