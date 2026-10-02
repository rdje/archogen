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
