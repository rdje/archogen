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
  Status: `pending`
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
  Verification: `pending`
  Commit: `pending`

- ID: `API.4`
  Status: `pending`
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
  Verification: `pending`
  Commit: `pending`

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
| 1 | `API.3` | `pending` | **the load-bearing leaf; its prerequisite, `M1.13`'s freeze, is `done`.** Declaring an API over an unfrozen language would have meant declaring it twice: the freeze settles the integer domain and the escape set, which are the API's numeric types and its string encoding |
| 2 | `API.4` | `pending` | the instance model and the resource limits an untrusted consumer makes necessary. New architecture, not a binding — archogen is stateless over files today |
| 3 | `API.5` | `pending` | the wasm binding, behind `API.1` and `API.3` |
| 4 | `API.6` | `pending` | the MCP server — the point of the tree, and last because everything above is what makes it safe to hand to an arbitrary agent |
| 7 | `API.7` | `pending` | the book chapter. Not optional, and not foldable into `API.6` |

⛔ **This tree does not displace the project's main line.** `M1.13` is the frontier in
[`M1.md`](M1.md), and `API.3`–`API.7` are sequenced behind it by the director's ruling. `API.1` and
`API.2` are the only unblocked slices, and both are measurements or gates rather than features — taking
them early is cheap and makes the rest estimable.

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
  answered or explicitly accepted.

## Blockers

- `API.3`–`API.7` are sequenced behind `M1.13` (the language freeze) by director ruling. `API.1` and
  `API.2` are not blocked.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-28` | `API` | tree seeded from the director's ruling and a feasibility census; no code | the census is recorded in `docs/decisions/decision_programmatic-interface.md`: six of eight crates I/O-free in production, two already `no_std`, zero third-party dependencies, no `Command::new` in any production half |
| `2026-09-30` | `API.1` | the five I/O-free crates built for wasm32; the derivation's arms; the integration tier | all compile; the first cut of the detection matched nothing and the arms caught it; tier `incomplete`, 8 passed, the emulator quarantined |
| `2026-09-30` | `API.2` | the gate on the product; nine arms and two mutations; `wasm_build.sh` on the same rule | 44 production files, none spawns; the naive test-half rule shown leaking and replaced in both scripts |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `API` | `ARCHOGEN-API-0078 (leaf API)` | tree seeded: `ROADMAP.md` §10.4, the decision record and its index row, seven leaves, three of the four open questions routed to a named owner |
| `API.1` | `ARCHOGEN-API-0157 (leaf API.1)` | **the engine compiles for the browser** — measured by a derived `wasm-build` step; four members excluded by the I/O they do, each named |
| `API.2` | `ARCHOGEN-API-0158 (leaf API.2)` | **the product runs nothing, and a gate says so** — `NO-SUBPROCESS` |

## Changelog

- `2026-09-28`: Created task tree from the director's ruling of the same date and `ROADMAP.md` §10.4.
