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
