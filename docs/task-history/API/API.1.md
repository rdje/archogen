- ID: `API.1`
  Status: `done`
  Goal: **measure** whether the engine compiles for the browser, before anything is promised about it.
  Add a `wasm32-unknown-unknown` compile-target step to the §14.3 tier runner over the crates that are
  I/O-free in production, in the shape of the existing `no-std-build` step.
  Acceptance: the step exists and reports one of the runner's real verdicts — `passed`, `failed`, or
  `not built` naming a leaf — never a silent skip; the crate set it walks is **derived** (from the
  workspace members, minus the two that touch the filesystem) rather than listed, so a new crate is in
  scope without anyone editing the runner; if it fails, the failure is recorded here with the exact
  error and a leaf filed for each distinct cause rather than the step being weakened to pass; the book's
  `verification.md` names the step; `make focused` exit `0`.
  Priority: **high, and the only leaf here that is not sequenced behind the freeze.** It is a
  measurement, not a contract: it converts "should be feasible" into "compiles, or here is exactly what
  breaks", and every later leaf is cheaper to scope once that is known. The director approved it landing
  before `M1.13` for that reason.
  ⚠️ **One premise corrected by measurement:** "minus the two that touch the filesystem" — **four** do, in production
  code: `archogen-cli`, `xtask`, `archogen-s0` (its emitter writes the generated crate) and `eadl-front` (its module
  loader reads files). The last is the one a browser needs most, and its reads already sit behind a `ModuleSource`
  implementation (`DirectoryModules`, `crates/eadl-front/src/module.rs`), so `API.5` can supply another; that is
  recorded on `API.5`. The decision record's census ("six of the eight", `2026-09-28`) was true of its date:
  `git log -S"read_to_string(&path)" -- crates/eadl-front/src/module.rs` → `0a19c36 2026-09-29`, the module loader,
  a day later. The record is amended, not rewritten.
  Verification: see the checklist — the five I/O-free crates compile for `wasm32-unknown-unknown`; the derivation's
  five arms, which found the first cut of the detection matching nothing; the `integration` tier with the new step.
  Commit: `ARCHOGEN-API-0157 (leaf API.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — nothing had measured it, and the pinned toolchain could not have:
    ```text
    $ git show HEAD:xtask/src/main.rs | grep -c "wasm"   → 0
    $ rustup +1.95.0 target list --installed           → aarch64-apple-darwin, riscv64imac-unknown-none-elf (no wasm32)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the binding was promised by decision (`decision_programmatic-interface.md`)
    and no tier step compiled anything for the browser target, so feasibility was an expectation. **WHERE:** the
    `integration` tier's steps in `xtask/src/main.rs` — `git show HEAD:xtask/src/main.rs | grep -c 'name: "no-std-build"'`
    → `1`, and no sibling for wasm32.
  - [x] **FIX** — `scripts/wasm_build.sh`, the `wasm-build` step (requires `target:wasm32-unknown-unknown`, so an
    absent target is *unavailable* locally and a failure under `--provisioned`); the crate set derived from
    `cargo metadata`, excluding members whose production code names `std::fs`/`process`/`net`/`env`, each exclusion
    named with its line; `rust-toolchain.toml` lists the target, so CI installs it.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/wasm_build.sh; echo "exit=$?"
      wasm-build: excluded archogen-cli crates/archogen-cli/src/build_cmd.rs:82: … std::fs::read_to_string(path) …
      wasm-build: excluded archogen-s0  crates/archogen-s0/src/emit.rs:69: std::fs::create_dir_all(&src)?;
      wasm-build: excluded eadl-front   crates/eadl-front/src/module.rs:206: match std::fs::read_to_string(&path) {
      wasm-build: excluded xtask        xtask/src/main.rs:47: use std::process::{Command, Stdio};
      wasm-build: compiling for wasm32-unknown-unknown: eadl-model archogen-evidence rt-analysis rt-core rt-reference
      exit=0
    $ bash scripts/wasm_build.sh --self-test   → wasm-build self-test: 5 pass / 0 fail (5 arms)
    ```
    ⛔ The arms caught the first cut passing for the wrong reason — `wasm-build self-test: 1 pass / 4 fail (5 arms)`:
    `\b` in its `awk` regex is not a word boundary under BSD `awk`, the detection matched nothing, and the real run
    compiled all nine members and exited `0`. Then a grouped `use std::{fs, io};` was missed (`4 pass / 1 fail`).
    Both fixed in the pattern, both pinned by an arm.
  - [x] **NO REGRESSION** — `cargo xtask verify --tier integration` → `incomplete — 8 passed, 0 failed, 0 unavailable,
    0 not built, 1 quarantined`: the new step ✅, every other step as before; `cargo test -q -p xtask` → `test result:
    ok. 25 passed; 0 failed`.
  - [x] **LOCKSTEP** — `verification.md` (a section, the tier table, the transcript re-rendered from the run above),
    `COMMIT.md` step 2, `TOOLBOX.md`, the `rust-toolchain` ledger entry, `docs/figures.md`.
