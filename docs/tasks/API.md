# API: the programmatic interface — one engine API, a wasm binding, and an MCP server

## Metadata

- Tree ID: `API`
- Status: `active`
- Roadmap lane: `ROADMAP.md` §10.4 (added by director ruling `2026-09-28`); §4.4 trust; §5.5 verdicts;
  §7.1 report completeness; §14.3 compile targets; §15 versioning
- Created: `2026-09-28`
- Owner: repo-local workflow
- Decision record: [`decision_programmatic-interface.md`](../decisions/decision_programmatic-interface.md)

## Goal

One declared, versioned, transport-neutral **engine API** — a description as text plus a profile in, a
structured result carrying §5.5's verdicts out — with the CLI as a consumer of it rather than a parallel
implementation, and two bindings over it: a `wasm32-unknown-unknown` build and an **MCP server** that any
agent can drive. The server is a capability of the built binary, spawned per instance.

## Non-Goals

- **Neither build is controllable programmatically.** Not archogen's own compilation, and not
  `archogen build <description>` (system generation, §10.3). Generation writes a crate tree and stays a
  human or CI action. Ruled, not deferred.
- No web UI, no editor plugin, no hosted service. The deliverable is an API and two transports; what
  consumes them is somebody else's work.
- No new language feature and no change to the description format. The freeze (`M1.13`) settles the
  language; this tree exposes it.
- No LLM in the loop. §10.3 already forbids a build calling one, and nothing here changes that: the
  agent is a *consumer* of archogen, never a component of it.

## Acceptance Criteria

- The engine API is declared, versioned, documented in the mdBook, and the CLI is a consumer of it — a
  capability cannot exist behind one and not the other.
- Every programmatic response carries §5.5's verdict. A result without one is a contract violation, not
  a degraded result.
- The pure engine crates compile for `wasm32-unknown-unknown` as a tier step, so the feasibility claim is
  a measurement rather than an opinion, and stays one.
- An MCP server exposes the built operations, derives its tool list from
  `crates/archogen-cli/src/spec.rs` rather than a second list, and reports an unimplemented operation's
  owning leaf instead of failing at runtime.
- The invariant that makes agent control safe — **no product code spawns a subprocess or executes
  anything** — is stated in a durable record and gated, not merely observed.
- Resource limits exist for a description supplied by an untrusted consumer.
- The book has a chapter for the programmatic interface. The director reads the book; a capability that
  is not in it does not exist as far as the only reader of it is concerned.
- Focused validation passes per slice; broader validation when the blast radius warrants it.
- Live docs and roadmap status updated where project state changed.
- Each completed leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `API`
  Status: `active`
  Goal: the programmatic interface — one engine API, two bindings
  Children: `API.1` … `API.7`

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

- ID: `API.2`
  Status: `done`
  Goal: state and gate the invariant that makes handing archogen to an arbitrary agent safe — **no
  product code spawns a subprocess or executes anything.**
  Reproduce / issue: measured `2026-09-28`. `git grep -niE 'spawns? no|no subprocess|does not
  execute|never executes|no child process'` over tracked files returns **nothing**, so the property is
  written down nowhere. It holds today: `Command::new` appears in no production half in the workspace,
  the only `std::process` use in `crates/archogen-cli/src` is `use std::process::ExitCode;`, and the one
  place that compiles a generated artifact is `crates/archogen-cli/tests/s0_oracle.rs:609` — a test.
  census: `for f in $(git ls-files 'crates/*/src/*.rs'); do awk '/#\[cfg\(test\)\]/{exit} {print
  FILENAME":"NR": "$0}' "$f"; done | grep -E 'Command::new|std::process'` → one hit, `ExitCode`.
  Acceptance: the invariant is stated in a durable record and in the book; a gate fails if a production
  half anywhere in `crates/*/src` spawns a subprocess, with the test half excluded by construction and
  the exclusion itself armed; RED arms proving the gate fires on a real spawn and does **not** fire on
  `ExitCode`, on a test-half `Command::new`, or on the `xtask` and `scripts/` surfaces that legitimately
  drive a toolchain; `make focused` exit `0`.
  Priority: **medium-high** — cheap, unblocked by the freeze, and it is the property §10.4's safety
  argument rests on. A `Command::new` added to a product crate tomorrow passes every gate in the tree
  today.
  Verification: see the checklist — the gate green on 44 production files, nine arms, two mutations; the same
  production-half rule adopted by `wasm_build.sh`, with its own arm.
  Commit: `ARCHOGEN-API-0158 (leaf API.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the property held and nothing held it:
    ```text
    $ git show HEAD:scripts/check_doctrines.project.sh | grep -c "NO-SUBPROCESS"   → 0
    $ git grep -niE 'spawns? no|no subprocess|does not execute|never executes|no child process' HEAD
      → only CHANGELOG.md and the decision record describing its absence — no statement of it, and no gate
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — §10.4's safety argument was reasoned from a measurement
    (`decision_programmatic-interface.md`, `2026-09-28`) and never turned into a check, so it lived at the level of
    "true today". **WHERE the rule's first shape would have leaked:** "production code ends at the first
    `#[cfg(test)]`" — the rule `API.1`'s `wasm_build.sh` shipped with — lets one attribute on an early helper silence
    every line below it (`git show HEAD:scripts/wasm_build.sh | grep -n 'cfg\\(test'` →
    `43:    hit="$(awk '/^[[:space:]]*#\[cfg\(test\)\]/ { exit } …`, an exit at the first match).
  - [x] **FIX** — `scripts/check_no_subprocess.sh` (**`NO-SUBPROCESS`**, registered in the project slot): every
    tracked `crates/*/src/**/*.rs`, derived; the test half starts only at a `#[cfg(test)]` opening a `mod`; refuses
    `Command::new`, `process::Command` (imports too), `.spawn(`, `exec`, `fork`, and passes `ExitCode` and
    `process::exit`. The invariant stated in the decision record and in `verification.md`. `wasm_build.sh` adopts
    the same production-half rule.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_no_subprocess.sh → no-subprocess: OK (44 production source file(s) under crates/*/src; …)
      (44 = git ls-files 'crates/*/src/*.rs' | wc -l = find crates/*/src -name '*.rs' | wc -l)
    $ bash scripts/check_no_subprocess.sh --self-test → no-subprocess self-test: 9 pass / 0 fail (9 arms)
      a real spawn → refused with its line · an import of Command → refused · ExitCode, process::exit → pass
      a spawn in the test module → pass · cfg(test) on a lone fn → the spawn below it refused
      tests/ and xtask/ → outside the population · .spawn( → refused · no product source → a breach
    M1 the naive rule (the first cfg(test) ends production) → 1 refused: "a cfg(test) on a lone function…", restored
    M2 .spawn( dropped from the pattern → 1 refused, restored
    $ bash scripts/wasm_build.sh --self-test → wasm-build self-test: 6 pass / 0 fail (6 arms), the new arm among them
    ```
  - [x] **NO REGRESSION** — `bash scripts/check_doctrines.project.sh` → `no-subprocess: OK …`, slot rc=0; the
    `wasm_build.sh --list` set unchanged (the five crates, the four exclusions with the same lines).
  - [x] **LOCKSTEP** — the decision record, `DOCTRINE_ENFORCEMENT.md`, `verification.md` "The product runs nothing".

- ID: `API.3`
  Status: `done`
  Children: `API.3.1`, `API.3.2`, `API.3.3`, `API.3.4`
  Goal: declare the **engine API** — the transport-neutral contract every binding consumes, and the one
  the CLI becomes a consumer of.
  Acceptance: an in-memory entry point taking a description as text plus a profile and returning a
  structured result; every result carries §5.5's `Verdict` and the diagnostics with their codes, spans
  and repair directions; the API is versioned under §15 with a stated compatibility promise; the CLI's
  `check` path calls it rather than reimplementing it, and a test asserts that a capability cannot exist
  behind one surface and not the other; `docs/book/src/` documents it; `make focused` exit `0`.
  Priority: **the load-bearing leaf** — everything after it is a binding. ⛔ **Sequenced behind `M1.13`.**
  The freeze settles the integer domain (F-F) and the escape set (F-G), which are exactly what the API's
  numeric types and its string encoding depend on. Declaring first means declaring twice.
  **Decomposed `2026-09-30`, measured first.** `archogen check`'s judging path is spread over
  `crates/archogen-cli/src/check_cmd.rs`: the profile lookup, the registry from the kind modules embedded
  there, the module-file and kind-module routing in `frontend`, and the engine's `check`. `archogen build`
  shares `frontend`. Declaring the API means moving all of that behind one entry point, moving the
  outcome vocabulary with it, and gating parity. That is one design record and three slices, each
  reviewable on its own.
  Verification: closed `2026-09-30` by its children, and the acceptance re-checked criterion by criterion:
  - **an in-memory entry point, text and profile in, a structured result out:** `archogen_api::check(&Request)`;
    `crates/archogen-api/tests/check.rs` → `test result: ok. 9 passed`, one of them a module tree held in memory.
  - **every result carries §5.5's verdict and the diagnostics with codes, spans and repairs:** the status is
    always present, a judged response's status is its verdict (asserted on every leg), and the diagnostics are
    the engine's own. The reading of "carries the verdict" for a request not judged is findings §9, for the
    director.
  - **versioned under §15 with a stated promise:** `VERSION` 1.0, registered in `versions.md` and gated by
    `VERSION-REGISTER` (`version-register self-test: 14 pass / 0 fail`).
  - **the CLI's check path calls it, and a test asserts a capability cannot exist behind one surface and not the
    other:** `crates/archogen-cli/tests/api_parity.rs` → `test result: ok. 6 passed`, three legs, each with a
    mutation killed.
  - **the book documents it:** `docs/book/src/engine-api.md`, its example held to a run.
  - **`make focused` exit `0`:** `tier focused: passed — 3 passed, 0 failed`.
  Commit: closed by `ARCHOGEN-API-0174 (leaf API.3.1)`, `ARCHOGEN-API-0175 (leaf API.3.2)`,
  `ARCHOGEN-API-0176 (leaf API.3.3)` and `ARCHOGEN-API-0177 (leaf API.3.4)`

- ID: `API.3.1`
  Status: `done`
  Goal: the design, decided and recorded before any code — where the API lives, what it takes and returns,
  how it is versioned, and how parity with the CLI is gated.
  Acceptance: a durable record under `docs/decisions/`; any reading of the director's ruling it depends on
  flagged in the findings for the director; the children below filed from it.
  **Decided:** [`decision_engine-api.md`](../decisions/decision_engine-api.md). A crate, `archogen-api`, with
  one operation, `check`. Its `Response` carries the exit contract's `Status`, moved into the API, whose
  verdict variants are §5.5's and whose `usage` and `unimplemented` cover a request it did not judge. Notes
  carry what has no span, and diagnostics carry the rest. The version is apart from the language, `1.0`
  fixed when `API.3` closes. Parity is gated three ways. The reading of "carries §5.5's verdict" is findings §9.
  Verification: the record, the index row, findings §9; no code changed.
  Commit: `ARCHOGEN-API-0174 (leaf API.3.1)`

- ID: `API.3.2`
  Status: `done`
  Goal: the crate — `crates/archogen-api` with `Request`, `Response`, `Status`, `VERSION` and `check`, the shipped
  kind modules embedded in it, and `ModuleSource::unreadable`.
  Acceptance: `check` answers every outcome the CLI's check path does today, each with its status: judged
  (every §5.5 verdict the corpus reaches), an unsupported profile, a kind module, an unreadable import (through
  a test `ModuleSource`), and a description too large to address; no response without a status, enforced by the
  type; the crate in `scripts/wasm_build.sh`'s derived pure set and under `NO-SUBPROCESS`; unit tests for each
  outcome; `make focused` exit `0`.
  Priority: **high** — the load-bearing slice.
  **Closed `2026-09-30`.** `crates/archogen-api` exists: `check(&Request) -> Response`, `Status` moved into it
  whole with its tests, `VERSION` (1.0, not yet fixed), `OPERATIONS`, the shipped kind modules, and the routing
  (`is_module_file`, `kind_module`). `ModuleSource::unreadable` is on the trait. The CLI re-exports `Status` and
  takes its kind modules from the API. Its own routing stays until `API.3.3` moves it onto `check`.
  ⛔ One expectation of mine was wrong. I wrote a module tree's instances root first, but the engine lists them
  children before parents, the elaboration order §6 relies on. The test now states the engine's order and why.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0175 (leaf API.3.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — no API, and the judging path lived in the CLI:
    ```text
    $ git ls-tree -r --name-only HEAD crates/archogen-api | wc -l → 0
    $ git grep -n "pub fn frontend\|^const KIND_MODULES\|pub enum Status" HEAD -- crates/
      HEAD:crates/archogen-cli/src/check_cmd.rs:31 (KIND_MODULES) · :294 (frontend) · crates/archogen-cli/src/status.rs:19
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the CLI was the only composer of registry, profile, routing and engine, so
    there was nothing for another consumer to call (`decision_engine-api.md`):
    `git grep -n "shipped_registry(" HEAD -- crates/ | grep /src/` → `archogen-cli/src/build_cmd.rs:96`,
    `archogen-cli/src/check_cmd.rs:106`, and the definition, `eadl-model/src/check.rs:116`.
  - [x] **FIX** — the crate as the decision fixed it, and a `ModuleSource::unreadable` default. `tests/check.rs`
    has eight legs: acceptance, every case's declared verdict, an unsupported profile, a kind module, an unreadable
    import, a module tree in memory, a description that does not read, and the version with the operations. Each
    checks the judged/not-judged invariant. Three mutations are catalogued. The citation in `s0_build.rs` is
    repointed to the moved file.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-api → test result: ok. 6 passed (Status) · test result: ok. 8 passed (check)
    $ bash scripts/wasm_build.sh --list → archogen-api in the derived pure set, no list edited
    $ bash scripts/wasm_build.sh → compiling for wasm32-unknown-unknown: archogen-api eadl-model … rc=0
    $ bash scripts/check_no_subprocess.sh → no-subprocess: OK (45 production source file(s) …)
    $ cargo run -q -p xtask -- mutate --only api-status-not-the-verdict api-kind-module-judged \
        api-unreadable-import-called-missing → mutate: OK — 3 mutation(s), each killed …
    ```
  - [x] **NO REGRESSION** — `cargo test -q --workspace --no-fail-fast` → 690 passed / 0 failed over 55 suites,
    rc=0, from 682: the eight new legs, and the six `Status` tests moved, not added. `cargo clippy -q --all-targets
    --all-features -- -D warnings` exit=0; `cargo fmt --all -- --check` exit=0; `make focused` exit=0 (`tier
    focused: passed — 3 passed, 0 failed`).
  - [x] **LOCKSTEP** — `verification.md`'s wasm paragraph names the API; the frontier, both logs, the changelog and
    the snapshots.

- ID: `API.3.3`
  Status: `done`
  Goal: the CLI becomes the API's first consumer — `check` and `build` judge through `archogen_api::check`
  and nothing else — and parity is gated.
  Acceptance: `check_cmd.rs` and `build_cmd.rs` call the API, and the duplicated routing is gone; the three
  parity legs of the decision (structural, by operation against `spec.rs`, by behaviour over every tracked
  description), each with a RED arm; every CLI transcript the book shows unchanged; `make focused` exit `0`.
  Priority: **high**.
  **Closed `2026-09-30`.** `check_cmd.rs` reads the file, picks the module path (§6 rule 7), asks
  `archogen_api::check`, and prints the `Response`. `build_cmd.rs` asks the same function through the same helper,
  then interprets and emits from the response's judgement. The routing that lived in the CLI is gone: `frontend`,
  `Frontend`, `KindModule`, `kind_module`, `refuse_kind_module` and `embedded_modules`. `archogen_api::refuse_profile`
  keeps the profile refusal before the file is read, with one wording for both commands.
  ⛔ Two things this move broke, each caught. `Response::render_diagnostics` concatenated where `Outcome::render`
  joins, and it now joins the same way. Three catalogue entries were anchored in text that moved (M1.30's closure
  entry, and M1.32's two); they are retargeted and killed again.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0176 (leaf API.3.3)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the CLI judged descriptions itself, beside the API:
    ```text
    $ git grep -n "shipped_registry(\|check_program(\|elaborate_source(" HEAD -- crates/archogen-cli/src
      build_cmd.rs:96 · check_cmd.rs:90 · check_cmd.rs:299 · check_cmd.rs:348
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `API.3.2` built the API beside the CLI's own path, so two
    implementations of one judgement existed: `git grep -n "pub fn frontend" HEAD -- crates/archogen-cli/src` →
    `check_cmd.rs`, shared by both commands.
  - [x] **FIX** — the CLI as consumer (`ask`, `report_not_judged`), and `refuse_profile` in the API.
    `crates/archogen-cli/tests/api_parity.rs` has three legs with a RED arm each: structural (no CLI production
    line names `eadl_model::check`, `shipped_registry`, `check_program` or `elaborate_source`), by operation
    (`OPERATIONS` equals the commands `spec.rs` runs, less `build` with its §10.4 reason), and by behaviour
    (over every description, the CLI's status, codes and notes equal the API's). Three parity mutations are
    catalogued, and three entries retargeted.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-cli --test api_parity → test result: ok. 6 passed
    $ cargo run -q -p xtask -- mutate --only cli-reaches-the-engine-directly cli-drops-the-first-note \
        api-offers-an-excluded-command → each killed by the leg it targets; mutate: OK — 3 mutation(s)
    $ cargo run -q -p xtask -- mutate --only closure-computed-and-dropped kind-module-not-routed \
        kind-module-classified-by-its-first-declaration → mutate: OK — 3 mutation(s)
    $ cargo run -q -p xtask -- mutate → mutate: OK — 38 mutation(s), each killed or surviving exactly as the
      catalog expects; rc=0 (the whole catalogue, after the move)
    ```
  - [x] **NO REGRESSION** — `cargo test -q --workspace --no-fail-fast` → 690 passed / 0 failed over 55 suites
    after the move and before the parity file, rc=0: every book transcript (`book_transcripts`, the kind-module
    transcript), every frozen verdict (`verdicts`) and every CLI test unchanged by it. With the parity file:
    696 passed / 0 failed over 56 suites, rc=0. `cargo clippy -q --all-targets --all-features -- -D warnings`
    exit=0; `cargo fmt --all -- --check` exit=0.
  - [x] **LOCKSTEP** — `build_cmd.rs`'s order-of-operations doc; the frontier, both logs, the changelog and the
    snapshots. The book's API chapter is `API.3.4`.

- ID: `API.3.4`
  Status: `done`
  Goal: the book documents the engine API — what it takes, what it returns, the outcome vocabulary, the version
  and its promise, and what is outside it and why.
  Acceptance: a chapter or section under `docs/book/src/` with an example that runs; `API.7` builds the full
  programmatic-interface chapter on it and does not restate it; `API.3` closes with the version fixed at `1.0`.
  Priority: **high** — the director reads the book.
  **Closed `2026-09-30`.** `docs/book/src/engine-api.md` covers what the API takes and gives, its one outcome
  vocabulary, an example that runs, the version and its promise, how the CLI is held to it, and what is outside it.
  Its transcript is held to the example by `crates/archogen-api/tests/book_example.rs`. The version is fixed at
  `1.0` and registered in `versions.md`.
  ⛔ Found on the way: `VERSION-REGISTER` derives versions from three code shapes, and a `Version { major, minor }`
  constant is none of them. The API's version was a versioned surface the register could not see, so a bump
  would have landed unrecorded. The gate learned the shape, with two RED arms. An exhaustive reading of
  `Response` and `Judgement` in `tests/check.rs` now stops compiling when a field moves.
  ⛔ And one false sentence of my own, caught on rereading. The chapter said every response names the API, the
  language and the profile versions, but a response that was not judged names only the API's.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0177 (leaf API.3.4)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the book had no page for the API, and the register could not see its version:
    ```text
    $ git ls-tree -r --name-only HEAD docs/book/src | grep -c engine-api → 0
    $ bash scripts/check_version_register.sh (before the gate learned the shape) → version-register: OK (7 entries;
      7 declared version(s)) — archogen_api::VERSION among none of them
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_version_register.sh`'s `declared()` knew format identifiers,
    `*_VERSION` strings and profile ids: `git grep -n "sed -nE" HEAD -- scripts/check_version_register.sh` → three
    patterns, none for a struct constant.
  - [x] **FIX** — the chapter and its `SUMMARY.md` entry; `examples/in_memory.rs` and `tests/book_example.rs`;
    the register's fourth shape, with two arms (a minor bumped without its entry, a new API version with none);
    the `engine-api` entry in `versions.md`; the shape test; `VERSION`'s comment and the decision say `1.0` is
    fixed.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo run -q -p archogen-api --example in_memory → the three responses the chapter shows
    $ cargo test -q -p archogen-api → test result: ok (check 9, book_example 1, status 6)
    the chapter's transcript edited by one word → the_books_example_is_what_the_example_prints FAILED (restored)
    $ bash scripts/check_version_register.sh --self-test → version-register self-test: 14 pass / 0 fail (14 arms)
    $ bash scripts/check_version_register.sh → version-register: OK (8 entries; 8 declared version(s) …)
    ```
  - [x] **NO REGRESSION** — `cargo test -q --workspace --no-fail-fast` → 698 passed / 0 failed over 57 suites,
    rc=0, from 696: the two tests this leaf adds. `cargo clippy -q --all-targets --all-features -- -D warnings`
    exit=0; `cargo fmt --all -- --check` exit=0; `make focused` exit=0.
  - [x] **LOCKSTEP** — `engine-api.md`, `versions.md`, `verification.md` (a stray line break), the decision record;
    the frontier, both logs, the changelog and the snapshots.

- ID: `API.4`
  Status: `done`
  Children: `API.4.1`, `API.4.2`
  Goal: define an **instance** — lifecycle, identity, what a server is bound to, and what a response is
  reproducible against — and put resource limits under an untrusted consumer's input.
  Acceptance: "instance" is defined in a durable record rather than implied by an implementation; a
  response is attributable to the description and profile that produced it; a description from an
  untrusted consumer cannot exhaust memory or CPU without a stated limit and a verdict that says so,
  which is `tool-failure` and never a partial result; RED arms for a limit that is exceeded and for one
  that is not; `make focused` exit `0`.
  Priority: **medium** — archogen is stateless over files today and has no such concept, so this is new
  architecture rather than a binding. It is also the precondition for the server being safe to spawn
  per instance, which is the deployment model the director described.
  ⚠️ Measured `2026-09-30` before starting: ten thousand nested parentheses crashed the reader with a stack
  overflow. That was a defect for every consumer, not only an untrusted one, so it was fixed at its source as
  `M1.38` (a nesting limit of 256, `read-nesting-too-deep`). The next measurement found the module system's two:
  a fan-out of 19 small modules held 1.8 GB, and a 3 000-link import chain overflowed the stack. Both are fixed at
  their source as `M1.39` (1 024 instances, 16-module chains). This leaf's limits start from there.
  **Decomposed `2026-09-30`, after measuring what is left.** With `M1.38` and `M1.39`, nesting, instances and
  import chains are bounded in the language. The remaining dimension is input size, and work is linear in it:
  a system with 50 000 required services, 5 MB, checks in 1.97 s against 0.35 s for a fifth of it (debug build). So
  one byte budget per request bounds time and memory. The instance definition is a design act of its own.
  Verification: closed `2026-09-30` by its children and by `M1.38` and `M1.39`, which its measurement filed, with
  the acceptance re-checked criterion by criterion:
  - **"instance" defined in a durable record:** `docs/decisions/decision_api-instance.md`.
  - **a response attributable to the description and profile that produced it:** every response names the API
    and engine versions, a judged one the language and profile, and its `sources` hold the request's texts;
    `the_same_request_answers_the_same_twice_and_names_the_build` → part of `test result: ok. 10 passed`.
  - **no exhaustion without a stated limit and a verdict that says so, `tool-failure` and never partial:** the
    language bounds nesting (`read-nesting-too-deep`), instances and import chains (`module-too-many-instances`,
    `module-import-too-deep`). The request budget bounds the rest, which is linear. Its refusal is
    `tool-failure` with no diagnostic (`budget.rs`, `test result: ok. 5 passed`).
  - **RED arms for a limit exceeded and one not:** both edges of the budget, through the description and the
    modules; both edges of each module limit (`module_limits.rs`, `test result: ok. 4 passed`); the nesting
    limit at 256 and 257.
  - **`make focused` exit `0`:** exit=0.
  Commit: closed by `ARCHOGEN-API-0182 (leaf API.4.1)` and `ARCHOGEN-API-0183 (leaf API.4.2)`, with
  `ARCHOGEN-M1-0178 (leaf M1.38)` and `ARCHOGEN-M1-0180 (leaf M1.39)`

- ID: `API.4.1`
  Status: `done`
  Goal: define an instance in a durable record, and make every response attributable to the build that produced it.
  Acceptance: a record under `docs/decisions/` stating an instance's lifecycle, its identity, what it is bound to,
  and what a response is reproducible against; the response names the engine version beside the API version,
  which is a minor bump under the promise; a test that the same request answers identically twice; the book says
  what an instance is.
  **Closed `2026-09-30`.** [`decision_api-instance.md`](../decisions/decision_api-instance.md) defines an instance:
  one running copy of one build, stateless between requests, bound to its build, and reproducible from its
  request and that build. `Response::engine` (`archogen_api::ENGINE`) names the engine version beside `VERSION`,
  which moves to `1.1` as a minor, the promise's first use. The version register refused the bump until its
  entry moved. A test asks the same request twice and compares the whole response. The book gains "What an
  instance is".
  ⚠️ Stated in the record, found checking it: identity by version numbers holds only for a build made at a release.
  Every crate manifest's `version` line, over the whole history, reads `0.1.0`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0182 (leaf API.4.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — no definition, and a response that named the API but not the build:
    ```text
    $ git grep -n "needs a definition" HEAD -- docs/decisions/decision_programmatic-interface.md → line 78,
      "\"Instance\" needs a definition before it needs an implementation", and no record gives one
    $ git show HEAD:crates/archogen-api/src/lib.rs | grep -c "pub engine" → 0
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `API.3` declared the contract, and nothing yet said what serves it or what a
    response is reproducible against: `git ls-tree -r --name-only HEAD docs/decisions | grep -c instance` → 0.
  - [x] **FIX** — the record and its index row; `ENGINE` and `Response::engine`; `VERSION` 1.1; the shape test
    (`SHAPE_OF` 1.1); the determinism test; the example prints the engine; `engine-api.md` and `versions.md`.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-api → test result: ok (check 10, book_example 1, status 6)
    $ bash scripts/check_version_register.sh (before the entry moved) → the code says '1.1'; move the entry with
      the code — exit 1; after → version-register: OK (8 entries; 8 declared version(s) …)
    $ git log -G '^version = ' -- 'crates/*/Cargo.toml' → every version line ever written: 0.1.0
    ```
  - [x] **NO REGRESSION** — `cargo test -q --workspace --no-fail-fast` → 708 passed / 0 failed over 58 suites,
    rc=0, from 707. `cargo clippy -q --all-targets --all-features -- -D warnings` exit=0; `cargo fmt --all --
    --check` exit=0.
  - [x] **LOCKSTEP** — the records, `engine-api.md`, `versions.md`; the frontier, both logs, the changelog and the
    snapshots.

- ID: `API.4.2`
  Status: `done`
  Goal: a byte budget per request, over the description and every module text elaboration loads, which a consumer
  handing the API text it did not write can rely on.
  Acceptance: a stated default budget; a request over it answers `tool-failure`, with a note naming the budget and
  no diagnostic about the description, never a partial result; RED arms for a request over the budget and one
  exactly at it, through the description and through the modules; the CLI's behaviour stated (it is its own
  trusted consumer); the book says what an untrusted consumer can rely on; `make focused` exit `0`.
  **Closed `2026-09-30`.** `archogen_api::Limits` with `DEFAULT_BYTES` (1 MiB), and `check_with(request, limits)`,
  which `check` calls with the default. The budget counts the description and every module text elaboration
  loads, each instance's reload included, through a counting `ModuleSource` wrapper. Over it, the answer is
  `tool-failure` with a note naming the budget and what went past it, and no diagnostic. Within it, the answer
  is exactly the unbudgeted one. The CLI asks with `Limits::NONE`, trusting its own files, and the language's
  limits still apply to it. API `1.2`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0183 (leaf API.4.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — no budget, and work linear in an unbounded input:
    ```text
    $ git show HEAD:crates/archogen-api/src/lib.rs | grep -c "Limits\|budget" → 0
    1 MB description: 0.36 s, 81 805 312 maximum resident set size · 5 MB: 1.95 s, 382 959 616 (debug);
    release, 5 MB: 0.31 s, 390 070 272 — about 75 bytes held per byte sent
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — `check` judged whatever it was handed:
    `git show HEAD:crates/archogen-cli/src/check_cmd.rs | grep -n "archogen_api::check("` → 89. The only bound was
    `SourceMap::add`'s 4 GiB, a span's width.
  - [x] **FIX** — `DEFAULT_BYTES`, `Limits`, `Budgeted`, `check_with`, `over_budget` and `judge`; the CLI on
    `Limits::NONE`; `crates/archogen-api/tests/budget.rs`, five legs (both edges through the description, both
    through the modules, reloads counted, within budget equal to unbudgeted, `check`'s default); `VERSION` 1.2, the
    shape rule restated ("last changed the shape"); `engine-api.md` "What one request may cost"; `versions.md`;
    three catalogued mutations.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-api --test budget → test result: ok. 5 passed
    $ cargo run -q -p xtask -- mutate --only budget-ignores-the-description budget-ignores-the-modules \
        budget-returns-a-partial-result → mutate: OK — 3 mutation(s), each killed …
    $ bash scripts/check_version_register.sh → version-register: OK (8 entries; 8 declared version(s) …)
    $ bash scripts/wasm_build.sh → rc=0, archogen-api among the compiled set
    ```
  - [x] **NO REGRESSION** — `cargo test -q --workspace --no-fail-fast` → 713 passed / 0 failed over 59 suites,
    rc=0, from 708. `make focused` exit=0; `cargo clippy -q --all-targets --all-features -- -D warnings` exit=0;
    `cargo fmt --all -- --check` exit=0; `bash scripts/check_doctrines.sh` → all doctrines green.
  - [x] **LOCKSTEP** — `engine-api.md`, `versions.md`, the decision record; the frontier, both logs, the changelog
    and the snapshots.

- ID: `API.5`
  Status: `pending`
  ⚠️ From `API.1`'s measurement: the crates this binding needs most do I/O today — `eadl-front`'s module loader reads
  files through `DirectoryModules` (the `ModuleSource` trait is the seam to supply another), and `archogen-s0`'s
  emitter writes the generated crate to a directory. Both compile for wasm32; neither would work there as written.
  Goal: the **wasm binding** over the declared API, so the description-side toolchain runs in a browser
  or a worker.
  Acceptance: the API is reachable from `wasm32-unknown-unknown` as an artifact a page can load; a
  worked example checks a real description in a browser and shows the verdict; no filesystem, no
  subprocess and no ambient authority is required, and a test asserts the binding does not reach for one;
  the book documents it; `make integration` exit `0` or naming what is incomplete.
  Priority: **medium** — behind `API.1` (which measures whether this is possible at all) and `API.3`
  (which decides what it exposes).
  Verification: `pending`
  Commit: `pending`

- ID: `API.6`
  Status: `pending`
  Goal: the **MCP server** — any agent drives a running archogen instance through it.
  Acceptance: the tool list is derived from `crates/archogen-cli/src/spec.rs`, so a documented operation
  is always an offered one and an unimplemented one names its owning leaf instead of failing at runtime;
  the three-state `Maturity` is visible in capability discovery, so a consumer learns at the handshake
  that `resolve` does not exist yet rather than after calling it; every response carries §5.5's verdict;
  neither build is exposed, and a test asserts that rather than leaving it to convention; any external
  dependency arrives with a decision record naming its §4.4 trust category and the claims its compromise
  would invalidate, and is reachable from the transport and **not** from the generator or the checker;
  the book documents it; `make integration` exit `0` or naming what is incomplete.
  Priority: **the point of the tree**, and last because everything above is what makes it safe.
  Verification: `pending`
  Commit: `pending`

- ID: `API.7`
  Status: `pending`
  Goal: the book chapter for the programmatic interface.
  Acceptance: a chapter states what an agent and a browser can do, what they cannot, and which verdicts
  they receive; it cites the API, the bindings and the decision record; `BOOK-ANCHORS` and
  `reference.rs`'s legs 8 and 9 pass over it; `PROGRAM.24`'s coverage question is answered for every
  crate this tree adds, so the tree does not create the drift it was filed to end.
  Priority: **medium, and not optional** — the director reads the book and not the code. Filed as its own
  leaf rather than folded into `API.6` so it cannot be quietly skipped when the server lands.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `API.5` | `pending` | the wasm binding, behind `API.1` and `API.3` |
| 2 | `API.6` | `pending` | the MCP server — the point of the tree, and last because everything above is what makes it safe to hand to an arbitrary agent |
| 3 | `API.7` | `pending` | the book chapter. Not optional, and not foldable into `API.6` |

⛔ **This tree does not displace the project's main line.** The director's ruling sequenced `API.3`–`API.7`
behind `M1.13`, the language freeze, which closed on `2026-09-29`; `API.3` and `API.4` followed it. The main
line is now `M2` ([`M2.md`](M2.md), frontier `M2.6`), and a slice of this tree is taken when it does not
delay that.

## Decisions

- `2026-09-28`: **the programmatic interface is one API with two bindings, not two projects.** The
  director asked for browser runnability and for MCP control in two messages; measured, they describe the
  same pure surface, because both builds are excluded and what remains is computation over an in-memory
  description. Building the API once is what makes that true rather than hoped.
- `2026-09-28`: **post-build only, and both builds excluded.** Ruled by the director. It is also a
  bootstrapping fixed point rather than only a preference — a server must exist before it can be
  controlled — and it is what keeps the surface free of filesystem and subprocess authority.
- `2026-09-28`: **capability discovery comes from `spec.rs`, not from a second list.** That table already
  declares the command surface as data with a three-state `Maturity`, renders help from it, validates the
  parser against it, asserts the property in a test, and names the owning leaf for every non-built state.
  A fourth consumer of one table is this project's idiom; a parallel tool list is the drift
  `docs/semantics/grammar.md` exists to end, in a new place.
- `2026-09-28`: **the zero-dependency rule is keyed on `crates/`, not on a list of engine crates.** An
  early framing in conversation held that a "non-core" transport crate would sit outside
  [[zero-dependency-engine-core]]; reading the record says otherwise — its "How to apply" forbids a
  `[dependencies]` entry in *any* crate under `crates/` without a decision record. The shape that
  satisfies both is a one-way dependence, transport → engine, so the generator and the checker share
  nothing new and F30's independence argument is untouched.
- `2026-09-28`: **the book chapter is its own leaf.** `PROGRAM.24` was filed the same day on a measured
  instance of a crate the book never names. A tree that adds crates and folds their documentation into a
  feature leaf is a tree that reproduces that defect.

## Open Questions

- **May `archogen verify` be exposed programmatically?** It runs verification tiers, which invoke a
  toolchain and an emulator through `xtask` — side-effecting and subprocess-spawning, unlike everything
  else on this surface. It is unimplemented (`PROGRAM.3`), so nothing is blocked, but it needs its own
  ruling and must not be exposed by analogy to `build`. Owner: the director. Recorded here rather than
  decided, because deciding it by analogy is how an exclusion becomes an exception.
- **What is an instance bound to, and for how long?** A description's text, a directory, a lock file, a
  session? `API.4` must answer it before `API.6` can spawn anything. Owner: `API.4`. Does not block
  `API.1` or `API.2`.
- **Does the API surface the model layer's diagnostics, and with what normative statement behind them?**
  `M1.26` owns the fact that those codes are stated normatively nowhere. An API that returns them to an
  agent returns rules no document states. Owner: `M1.26`, and `API.3` must not close before it is
  answered or explicitly accepted. **Answered `2026-09-30` by `M1.26.1`:** `docs/semantics/model.md` states
  every model-layer code but the S0 prototype's `analysis-inconclusive`, whose exclusion it states.

## Blockers

- `API.3`–`API.7` are sequenced behind `M1.13` (the language freeze) by director ruling. `API.1` and
  `API.2` are not blocked.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-28` | `API` | tree seeded from the director's ruling and a feasibility census; no code | the census is recorded in `docs/decisions/decision_programmatic-interface.md`: six of eight crates I/O-free in production, two already `no_std`, zero third-party dependencies, no `Command::new` in any production half |
| `2026-09-30` | `API.1` | the five I/O-free crates built for wasm32; the derivation's arms; the integration tier | all compile; the first cut of the detection matched nothing and the arms caught it; tier `incomplete`, 8 passed, the emulator quarantined |
| `2026-09-30` | `API.2` | the gate on the product; nine arms and two mutations; `wasm_build.sh` on the same rule | 44 production files, none spawns; the naive test-half rule shown leaking and replaced in both scripts |
| `2026-09-30` | `API.3.1` | a read of `archogen check`'s judging path (`check_cmd.rs`, `build_cmd.rs`), `ModuleSource` and `ROADMAP.md` §10.4 and §15 | the path's parts and their one shared routing point, `frontend`; `MemoryModules` already exists; the ruling's wording and §10.4's differ, flagged as findings §9 |
| `2026-09-30` | `API.3.2` | the crate's eight legs; the wasm build and its derived set; the subprocess gate; three mutations; the whole suite; `make focused` | all pass; `archogen-api` compiles for wasm32; 690 passed / 0 failed over 55 suites; each mutation killed |
| `2026-09-30` | `API.3.3` | the three parity legs and their arms; six mutations; the whole catalogue; the whole suite | 6 pass; each mutation killed by its leg; 38 of 38 as expected; every transcript and frozen verdict unchanged |
| `2026-09-30` | `API.3.4` | the example and its transcript test; the register's self-test and real tree; the shape test; the whole suite; `make focused` | the transcript holds and fails when edited; 14 of 14 arms, 8 entries; 698 passed / 0 failed over 57 suites |
| `2026-09-30` | `API.4.1` | the determinism test; the register before and after the bump; the manifests' history; the whole suite | the same request answers identically; the register refused 1.1 until its entry moved; every manifest version ever written is 0.1.0; 708 passed / 0 failed over 58 suites |
| `2026-09-30` | `API.4.2` | the cost per byte measured; the budget's five legs; three mutations; the wasm build; the whole suite; the doctrines | about 75 bytes held per byte sent; both edges hold through the description and the modules; each mutation killed; 713 passed / 0 failed over 59 suites |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `API` | `ARCHOGEN-API-0078 (leaf API)` | tree seeded: `ROADMAP.md` §10.4, the decision record and its index row, seven leaves, three of the four open questions routed to a named owner |
| `API.1` | `ARCHOGEN-API-0157 (leaf API.1)` | **the engine compiles for the browser** — measured by a derived `wasm-build` step; four members excluded by the I/O they do, each named |
| `API.2` | `ARCHOGEN-API-0158 (leaf API.2)` | **the product runs nothing, and a gate says so** — `NO-SUBPROCESS` |
| `API.3.1` | `ARCHOGEN-API-0174 (leaf API.3.1)` | **the engine API's design, recorded** — a crate, one outcome vocabulary, a version promise, parity three ways; `API.3` decomposed |
| `API.3.2` | `ARCHOGEN-API-0175 (leaf API.3.2)` | **the engine API exists** — `archogen-api`: `check`, `Response`, `Status` moved into it, `VERSION`; compiles for wasm32 |
| `API.3.3` | `ARCHOGEN-API-0176 (leaf API.3.3)` | **the CLI checks through the API** — its own routing removed; parity gated structurally, by operation and by behaviour |
| `API.3.4` | `ARCHOGEN-API-0177 (leaf API.3.4)` | **the book documents the engine API** — `engine-api.md`, its example held to a run; version `1.0` fixed and registered; `API.3` closed |
| `API.4.1` | `ARCHOGEN-API-0182 (leaf API.4.1)` | **an instance defined** — one build, no state between requests; every response names its engine; API `1.1` |
| `API.4.2` | `ARCHOGEN-API-0183 (leaf API.4.2)` | **a byte budget per request** — 1 MiB by default, set by the instance; `tool-failure` past it, never partial; `API.4` closed |

## Changelog

- `2026-09-28`: Created task tree from the director's ruling of the same date and `ROADMAP.md` §10.4.
- `2026-09-30`: `API.3` decomposed into `API.3.1`–`API.3.4` after its design was recorded.
- `2026-09-30`: `API.4` decomposed into `API.4.1` (the instance) and `API.4.2` (a request budget), after `M1.38` and
  `M1.39` bounded the language.
