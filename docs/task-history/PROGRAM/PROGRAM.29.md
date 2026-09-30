- ID: `PROGRAM.29`
  Status: `done`
  Goal: no gate script puts its scratch in `/tmp` — every scratch directory is derived from the repository
  root, on its own volume, as the director's data-locality rule requires ("never default to `/tmp`").
  Reproduce / issue: found `2026-09-29` by `PROGRAM.18.1`, reading the script it was arming:
  ```text
  census: grep -n mktemp scripts/*.sh knowledge-map/scripts/*.sh
    scripts/check_frozen_evaluation.sh:59          tmp="$(mktemp -d)"            project-owned
    scripts/check_feedback_self_contained.sh:84    tmp="$(mktemp -d)"            project-owned
    scripts/check_s0_retirement.sh:120             tmp="$(mktemp -d)"            project-owned
    scripts/update_scaffold.sh:16                  tmp="$(mktemp -d)"            project-owned
    scripts/check_task_acceptance.sh:184, :358     SELF/tmp="$(mktemp -d)"       scaffold-owned
    scripts/check_waiver_routing.sh:73, :93        "$(mktemp)"                   scaffold-owned
  ```
  `mktemp` with no template writes under `$TMPDIR` or `/tmp`, off the repository's volume. The newer gates
  (`check_language_freeze.sh`, `check_repository_boundary.sh`) already use `target/doctrine_scratch/`.
  Acceptance: the four project-owned sites moved to `target/doctrine_scratch/<gate>/`, removed on exit, and a
  census leg that fails when a project-owned gate calls `mktemp` without a repository-derived template; the
  two scaffold-owned sites recorded with the reason they cannot be fixed here (another repository owns them,
  and the scaffold would erase a local edit) and the exact upstream change written down for whoever owns the
  scaffold — never sent there by an archogen agent; a residue census showing nothing left in `/tmp` by a gate
  run.
  Priority: **medium** — a director directive (§13) broken in six places, none of them losing data today.
  Verification: see the checklist — a new gate over every tracked script (16 arms, 9 mutations), a logging
  `mktemp` on `PATH` before and after, every changed script re-run, and a residue census.
  Commit: `ARCHOGEN-PROGRAM-0124 (leaf PROGRAM.29)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — measured, not read: a logging `mktemp` first on `PATH` (under `target/`)
    during one doctrine-driver run and one self-test run recorded **26 calls from five gates, 26 off the
    volume** (`$TMPDIR` = `/var/folders/…/T/`, so no path says `/tmp` and the rule is still broken). ⛔ The
    filing census **undercounted**: it searched `scripts/` and found 8 sites in 6 files (calling them "six");
    a census over every tracked script finds 18 in 15 — ten under `docs/` (eight LinkedSpec feedback scripts,
    two probe artifacts), which a census that dropped `docs/` to shed prose also missed:
    ```text
    $ git grep -c mktemp 7bf85ba -- '*.sh' '.githooks/*' Makefile
      … 15 files … => 18 line(s) in 15 file(s)          (scripts/ only => 8 in 6)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `mktemp` with no template resolves against `$TMPDIR`; every site was
    written that way, and nothing read scripts for it. Residue was **not** the problem: 0 of 26 paths existed
    after the run — each site cleans up, so the breach is location only. The shape, at three of the 18:
    ```text
    $ git grep -n 'mktemp -d)"' 7bf85ba -- scripts/check_frozen_evaluation.sh scripts/update_scaffold.sh docs/tasks/artifacts/task_acceptance/
      7bf85ba:docs/tasks/artifacts/task_acceptance/run_task_acceptance_probes.sh:22:WORK="$(mktemp -d)"; trap 'rm -rf "$WORK"' EXIT
      7bf85ba:scripts/check_frozen_evaluation.sh:132:tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
      7bf85ba:scripts/update_scaffold.sh:16:tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
    ```
  - [x] **FIX** — the 14 project-owned sites make scratch under `$ROOT/target/`: gates and probes under
    `target/doctrine_scratch/<name>.XXXXXX`; the feedback scripts, which must stand alone in someone else's
    checkout, derive `ROOT` from their own location (the enclosing work tree, else their own directory) and
    write under `target/feedback_scratch/`. New doctrine **`SCRATCH-LOCALITY`**
    (`scripts/check_scratch_locality.sh`): every tracked `*.sh`, `.githooks/*`, `Makefile` and `*.rs`; a
    `mktemp` must be `mktemp [-d] "$ROOT/target/…XXXXXX"`, no line may name the system temporary directory,
    no Rust `temp_dir()`; ownership **derived** from `update_scaffold.sh`'s `NEUTRAL` array, the scaffold's
    four sites reported and not refused; its own source assembles every refused shape from pieces, so it
    scans itself rather than excluding itself. The scaffold's change is written down —
    `docs/decisions/decision_scratch-on-the-repository-volume.md` — and not sent (upstream `bedrock`
    `5af0c1c` still has all of them, read-only).
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_scratch_locality.sh
      scratch-locality: OK (124 file(s) scanned; 4 site(s) in scaffold-owned files reported …)   exit=0
    $ bash scripts/check_scratch_locality.sh --self-test
      scratch-locality self-test: 16 pass / 0 fail (16 arms)
    after, same shim: 58 calls — 25 in project-owned files, all under $ROOT/target/; the 33 off the volume are
      check_task_acceptance.sh (21) and check_waiver_routing.sh (12), scaffold-owned
    residue: 0 of 58 logged scratch paths still exist; target/feedback_scratch: 0 entries
    ```
    Nine mutations, each restored and checked by `cmp`: **S-1** the template check dropped → 7 / 8; **S-2**
    comments read as code → 11 / 4; **S-3** scaffold files refused → 13 / 2; **S-4** scope narrowed to
    `scripts/` → 14 / 1; **S-5** an empty population passing → 14 / 1; **S-6** the Rust leg dropped → 14 / 1;
    **S-7** the system-path leg dropped → 13 / 2; **S-8b** the `$ROOT/target/` requirement dropped → 14 / 2.
    **S-8** (only the accepted options widened) survived, and was right to: the options do not place the file,
    the template does, and GNU `mktemp` rejects `-t`/`-p` with an absolute template. The arm that S-8b needed —
    an off-repository template, a home cache — was missing and was added.
  - [x] **NO REGRESSION** — ⛔ moving scratch changed what two self-tests tested, and both were caught and
    fixed at the cause. **`FEEDBACK-SELF-CONTAINED`'s self-test went partly vacuous** (4/6 red arms fired —
    its lister asked git, which ignores all of `target/`) → a `$ROOT/target/` fixture is listed with `find`,
    6/6. **LS-001's re-measurement false-failed an arm** (4/5 — Cargo walks up past a workspace that
    excludes a package and found this root) → the root `Cargo.toml` excludes `target`, the remedy LS-001 itself
    documents, 5/5; and its self-test now checks that no workspace encloses its scratch, exit 2 "could not
    run" otherwise — falsified by removing the exclusion (`rc=2`, naming the workspace), restored by `cmp`.
    Every changed script re-run against its before-state: LS-002/3/6/7 `repro.sh` `rc=0` ×4 before and after;
    LS-001/4/5 `--self-test` 5/5 · 9/9 · 4/4 before and after; probes `3 pass / 7 fail` and `5 pass / 0 fail`,
    unchanged. `cargo test --all` → `test result: passed=602 failed=0` over 42 suites; `cargo metadata` → 9
    workspace members, unchanged; `make focused` passed; the doctrine driver green at the commit.
  - [x] **LOCKSTEP** — the book's `verification.md` gains "Scratch stays on this volume";
    `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md` rows; the doctrine registered in the project slot; LS-004's
    `SETUP.md` said "a temp dir" and now says where; `PROGRAM.26` told that the adopted updater must keep the
    fix; two lessons promoted — `docs/knowledge/scope-a-census-by-the-rule-not-by-the-folder.md` (new) and
    `a-gate-is-only-as-sharp-as-its-fixtures.md` (a moved fixture moves its surroundings).
  - [x] **ALSO REPAIRED — this leaf's own placement.** `6bf578f` had inserted it at the first `## Current Frontier`
    in the file, which was **inside `PROGRAM.20`'s census block**, splitting the line
    `docs/tasks/<TREE>.md "## Current Frontier" order-1 row` and turning its tail into a false `## ` heading
    (the unanchored-`index` class, fixed once before with an anchored `"\n## …\n"`). Moved to follow
    `PROGRAM.28`; `PROGRAM.20`'s block compared against `61e5f09` with `cmp` → identical. A census of every
    tree for a leaf header not at column 0 found only this one. Nothing new is filed: `TASK-ACCEPTANCE` reads a
    leaf only from a flush-left `- ID:` line, so the displaced header would have been refused when this leaf
    closed.
