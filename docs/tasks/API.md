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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `API` | `ARCHOGEN-API-0078 (leaf API)` | tree seeded: `ROADMAP.md` §10.4, the decision record and its index row, seven leaves, three of the four open questions routed to a named owner |
| `API` | `ARCHOGEN-API-0354 (leaf API)` | API.1 to API.7 sealed into `docs/task-history/API/`, 24 leaves, each with its stub; `docs/tasks/` was past its ceiling |

## Changelog

- `2026-09-28`: Created task tree from the director's ruling of the same date and `ROADMAP.md` §10.4.
- `2026-09-30`: `API.3` decomposed into `API.3.1`–`API.3.4` after its design was recorded.
- `2026-09-30`: `API.4` decomposed into `API.4.1` (the instance) and `API.4.2` (a request budget), after `M1.38` and
  `M1.39` bounded the language.
