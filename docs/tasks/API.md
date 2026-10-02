# API: the programmatic interface — one engine API, a wasm binding, and an MCP server

## Metadata

- Tree ID: `API`
- Status: `done` — `2026-10-02`, with `API.7`
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
  Status: `done` — `2026-10-02`; every child done. Findings §9, how "every programmatic response carries §5.5's
  verdict" is read, stays with the director: a stricter ruling would open a leaf
  Goal: the programmatic interface — one engine API, two bindings
  Children: `API.1` … `API.7`

- ID: `API.1`
  Status: `done` — sealed in [`API/API.1.md`](../task-history/API/API.1.md); commit `ARCHOGEN-API-0157`

- ID: `API.2`
  Status: `done` — sealed in [`API/API.2.md`](../task-history/API/API.2.md); commit `ARCHOGEN-API-0158`

- ID: `API.3`
  Status: `done` — sealed in [`API/API.3.md`](../task-history/API/API.3.md); commit `ARCHOGEN-API-0174`

- ID: `API.3.1`
  Status: `done` — sealed in [`API/API.3.md`](../task-history/API/API.3.md); commit `ARCHOGEN-API-0174`

- ID: `API.3.2`
  Status: `done` — sealed in [`API/API.3.md`](../task-history/API/API.3.md); commit `ARCHOGEN-API-0175`

- ID: `API.3.3`
  Status: `done` — sealed in [`API/API.3.md`](../task-history/API/API.3.md); commit `ARCHOGEN-API-0176`

- ID: `API.3.4`
  Status: `done` — sealed in [`API/API.3.md`](../task-history/API/API.3.md); commit `ARCHOGEN-API-0177`

- ID: `API.4`
  Status: `done` — sealed in [`API/API.4.md`](../task-history/API/API.4.md); commit `ARCHOGEN-API-0182`

- ID: `API.4.1`
  Status: `done` — sealed in [`API/API.4.md`](../task-history/API/API.4.md); commit `ARCHOGEN-API-0182`

- ID: `API.4.2`
  Status: `done` — sealed in [`API/API.4.md`](../task-history/API/API.4.md); commit `ARCHOGEN-API-0183`

- ID: `API.5`
  Status: `done` — sealed in [`API/API.5.md`](../task-history/API/API.5.md); commit `ARCHOGEN-API-0220`

- ID: `API.5.1`
  Status: `done` — sealed in [`API/API.5.md`](../task-history/API/API.5.md); commit `ARCHOGEN-API-0201`

- ID: `API.5.2`
  Status: `done` — sealed in [`API/API.5.md`](../task-history/API/API.5.md); commit `ARCHOGEN-API-0202`

- ID: `API.5.3`
  Status: `done` — sealed in [`API/API.5.md`](../task-history/API/API.5.md); commit `ARCHOGEN-API-0203`

- ID: `API.5.4`
  Status: `done` — sealed in [`API/API.5.md`](../task-history/API/API.5.md); commit `ARCHOGEN-API-0205`

- ID: `API.5.5`
  Status: `done` — sealed in [`API/API.5.md`](../task-history/API/API.5.md); commit `ARCHOGEN-API-0209`

- ID: `API.6`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0341`

- ID: `API.6.1`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0293`

- ID: `API.6.2`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0294`

- ID: `API.6.3`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0332`

- ID: `API.6.4`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0333`

- ID: `API.6.5`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0335`

- ID: `API.6.6`
  Status: `done` — sealed in [`API/API.6.md`](../task-history/API/API.6.md); commit `ARCHOGEN-API-0338`

- ID: `API.7`
  Status: `done` — sealed in [`API/API.7.md`](../task-history/API/API.7.md); commit `ARCHOGEN-API-0353`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | none | `done` | the tree is closed: `API.1`–`API.7` done `2026-10-02`; findings §9 stays with the director |

⛔ **This tree does not displace the project's main line.** The director's ruling sequenced `API.3`–`API.7`
behind `M1.13`, the language freeze, which closed on `2026-09-29`; `API.3` and `API.4` followed it. The main
line is now `M2` ([`M2.md`](M2.md); its frontier is in [`docs/TASK_TREE.md`](../TASK_TREE.md)), and a slice of
this tree is taken when it does not delay that.

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
| `2026-09-30` | `API.5.1` | a throwaway `cdylib` for `wasm32-unknown-unknown` on the pinned toolchain, read by `WebAssembly.Module`; `#[no_mangle]` under `deny(unsafe_code)`; the module-file rule read from `eadl-front` | no imports; `memory`, the functions and two linker globals exported; the lint refuses each unmangled export; `<dir>/<module>.eadl` |
| `2026-09-30` | `API.5.2` | the host tests; the same tests under Miri; two catalogued mutations; the shape golden blessed once; the focused tier, the wasm build and the gates | 8 of 8, and 8 of 8 under Miri; both mutations killed; the golden passes unblessed; all green |
| `2026-09-30` | `API.5.3` | the artifact inspected by `WebAssembly.Module`; every tracked description through the loader, against the host build and `archogen check`; six RED arms; a broken loader on the real tree; the integration tier | no imports, the decided exports; 108 of 108 byte-identical, each exit equal; 6 of 6; refused, then green when restored; 11 of 11 |
| `2026-09-30` | `API.5.4` | the book's page transcript through the page's own logic; five new RED arms and a real-tree RED; the page's wiring against a stand-in document; the book's serving command read with `curl`; the integration tier | reproduced; 11 of 11 arms, refused then green; both answers as expected; every file with its right content type; 11 of 11 — no browser run (`API.5.5`) |
| `2026-09-30` | `API.5.5` | the director ran the book's "Opening the page" in a browser and reported the answer after Check | the book's transcript, line for line; the load answer and the browser's name and version not yet reported |
| `2026-09-30` | `API.5.5` | the director's second report: the page's answer on load, and the browser | `invalid-description (exit 10)` as the book says; Chrome `154.0.8037.58` (arm64); the leaf and `API.5` closed |
| `2026-10-02` | `API.6` | each acceptance clause against its test or record; `make integration` | every clause evidenced; 11 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined; `API.6` closed |
| `2026-10-02` | `API.7` | the reference legs 8 and 9; the transcript replay; the doctrines | 55 passed; 3 passed; all green; `API.7` and the tree closed |

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
| `API.5.1` | `ARCHOGEN-API-0201 (leaf API.5.1)` | **the wasm binding decided** — three exports, a framed request, a versioned JSON response, no imports as the authority test |
| `API.5.2` | `ARCHOGEN-API-0202 (leaf API.5.2)` | **the wasm binding, built and tested on the host** — `crates/archogen-wasm`, its framing, its encoding and its three exports |
| `API.5.3` | `ARCHOGEN-API-0203 (leaf API.5.3)` | **the browser module answers as the command line does** — the `wasm-binding` tier step, the loader and the harness |
| `API.5.4` | `ARCHOGEN-API-0205 (leaf API.5.4)` | **the page** — `crates/archogen-wasm/page/`, documented with a transcript the tier reproduces; `API.5.5` filed for the browser run this session could not make |
| `API.5.5` | `ARCHOGEN-API-0209 (leaf API.5.5)` | partial: the Check answer observed in a browser by the director; the leaf stays open for the load answer and the browser |
| `API.5.5` | `ARCHOGEN-API-0220 (leaf API.5.5)` | **the page run in a real browser**, both legs observed by the director in Chrome 154; `API.5` closed |
| `API.6.1` | `ARCHOGEN-API-0293 (leaf API.6.1)` | **the MCP server designed** — both protocol eras read at the source and answered; the specification ledgered by commit and hash |
| `API.6.2` | `ARCHOGEN-API-0294 (leaf API.6.2)` | **each command declares whether it is offered** — `Exposure`, the builds and `verify` excluded citing §10.4, `tools()` |
| `API.6.3` | `ARCHOGEN-API-0332 (leaf API.6.3)` | **the server's JSON reader and writer**: RFC 8259 and nothing else, a size and a nesting bound, a name twice refused, a number kept as its text; `spec.rs` formatted, which `focused` had failed on since `-0294` |
| `API.6.4` | `ARCHOGEN-API-0333 (leaf API.6.4)` | **the MCP server, `archogen mcp`**: both eras over stdio, the command table's tools, `check` answering the wasm binding's bytes, its verdicts the command line's on the conformance cases |
| `API.6.5` | `ARCHOGEN-API-0335 (leaf API.6.5)` | **the book documents the MCP server**: *The server an agent spawns*, its transcript a real run the stdio test replays |
| `API.6.5` | `ARCHOGEN-API-0337 (leaf API.6.5)` | **the MCP server's review answered**: the duplicate check linear, the line bound six times the budget, no `null` ids, integral ids, argument errors as tool errors; D3 filed as `API.6.6` |
| `API.6.6` | `ARCHOGEN-API-0338 (leaf API.6.6)` | **an answer as bounded as its request**: a diagnostic quotes a 160-character window of a long line; 1.5 GB to 0.57 MB for the reviewer's 1 MB line |
| `API.7` | `ARCHOGEN-API-0353 (leaf API.7)` | **the programmatic interface's chapter**: what each consumer can do and receives, today and ahead; the tree closed |
| `API` | `ARCHOGEN-API-0354 (leaf API)` | API.1 to API.7 sealed into `docs/task-history/API/`, 24 leaves, each with its stub; `docs/tasks/` was past its ceiling |
| `API.6` | `ARCHOGEN-API-0341 (leaf API.6)` | **the MCP server closed**: every acceptance clause evidenced, the integration tier 11 of 11; `API.7` next |

## Changelog

- `2026-09-28`: Created task tree from the director's ruling of the same date and `ROADMAP.md` §10.4.
- `2026-09-30`: `API.3` decomposed into `API.3.1`–`API.3.4` after its design was recorded.
- `2026-09-30`: `API.4` decomposed into `API.4.1` (the instance) and `API.4.2` (a request budget), after `M1.38` and
  `M1.39` bounded the language.
