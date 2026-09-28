# archogen is wasm-runnable and MCP-controllable — post-build only, and after the language freeze

- **Type:** `decision`
- **Date:** `2026-09-28`
- **Status:** `active`
- **Owner / source:** director ruling, `2026-09-28`, in conversation. Roadmap text: §10.4. Owning tree:
  [`API`](../tasks/API.md).

## The fact / decision

archogen gains a **programmatic interface**: one declared, versioned, transport-neutral engine API, of
which a `wasm32-unknown-unknown` build and an **MCP server** are two bindings. Any agent — an LLM, a
swarm, an orchestrator — may drive a running archogen instance through that server.

Three rulings came with it, and each is a constraint rather than a preference:

1. **It enters the plan as a new task tree plus a §10 amendment**, not as a numbered milestone. §10 is
   the user contract; the programmatic surface is a second face of the same contract.
2. **It is declared after the `M1` language freeze** (`M1.13`). An API over an unfrozen language is an
   API that will break, and the freeze is what makes declaring one worth the cost.
3. **MCP controls an instance post-build only, and both builds are excluded** — archogen's own
   compilation, and `archogen build <description>` (system generation). The server is a capability of
   the *built binary*: build archogen normally, then spawn the server and listen to it.

## Why

**The feasibility is measured, not assumed.** A census over every crate's production half — the half
before its first `#[cfg(test)]`:

| Crate | `std::fs` | `std::process` | `std::env` |
| --- | --- | --- | --- |
| `eadl-front`, `eadl-model`, `rt-analysis`, `rt-core`, `rt-reference`, `archogen-evidence` | 0 | 0 | 0 |
| `archogen-cli` | 2 | 0 | 1 |
| `archogen-s0` | 1 | 0 | 0 |

Six of the eight crates touch no filesystem, no subprocess and no environment. `rt-core` and
`rt-reference` are already `#![cfg_attr(not(test), no_std)]`, and the integration tier already compiles
for a bare-metal target — the same constraint wasm imposes, already paid for. The workspace has **zero**
third-party dependencies: every `[dependencies]` entry is a `path =` sibling. And every crate already
has a `lib.rs` with public items, so a de facto API exists; it was simply never declared, versioned or
documented as one.

**`archogen-cli`'s one `std::process` hit is not a subprocess.** It is `use std::process::ExitCode;`.
`Command::new` appears in no production half anywhere in the workspace; the only one that compiles a
generated artifact is `crates/archogen-cli/tests/s0_oracle.rs`, a test. So `archogen build` writes a
crate tree and stops. This matters twice over: it is why the interface can be a pure function, and it is
an **undocumented, ungated invariant** — see "How to apply".

**Ruling 3 is what keeps the surface safe, and it is a design consequence rather than caution.** With
generation excluded, the whole programmatic surface is computation over an in-memory description: no
filesystem, no subprocess, no ambient authority. That is also why wasm and MCP stop being two projects —
they are one API with two transports, describing the same pure surface, and a server compiled to wasm
could run in a worker as well as on a host.

**The build phase cannot be MCP-controlled anyway, on logic rather than policy.** A server must exist
before it can be controlled; "being compiled" has no runtime to serve MCP. Ruling 3 restates a
bootstrapping fixed point, and putting it in the roadmap stops a future session from designing around it.

**Why the freeze first.** `M1.13` settles the integer domain and the escape set (findings F-F and F-G).
Both are exactly the things an API's types and its string encoding depend on.

## How to apply

- **Every programmatic response carries §5.5's verdict.** This is the single most important rule and it
  is not a nicety: a result returned without its verdict is how `tool-failure` or
  `analysis-inconclusive` becomes `established` in a consumer that never saw the distinction. MCP would
  otherwise be the cheapest way to overstate evidence in a project whose whole design is about not doing
  that. §7.1 applies to a machine-readable response exactly as to a printed one.
- **A spawned server taking descriptions from arbitrary agents is not a trusted local CLI.** It needs
  input limits. With generation excluded, resource exhaustion from a pathological description is the main
  abuse vector, and there are no limits today because none were needed.
- **"Instance" needs a definition before it needs an implementation** — lifecycle, identity, what a
  server is bound to, and what a response is reproducible against. archogen is stateless over files
  today and has no such concept.
- **Capability discovery comes from `crates/archogen-cli/src/spec.rs`**, which already declares the
  command surface as data with a three-state `Maturity` (built / experimental / unimplemented), renders
  help from it, validates the parser against it, asserts that property in a test, and names the owning
  leaf for every non-built state. The MCP tool list is a fourth consumer of that one table, not a second
  list. Consequence to state in the handshake: the capability set is frozen at build time, so a missing
  capability means "rebuild archogen", never "ask the server".
- ⛔ **The zero-dependency rule is keyed on `crates/`, not on a list of engine crates.**
  [[zero-dependency-engine-core]] enumerates the engine crates in its statement and then says, in "How to
  apply", *do not add a `[dependencies]` entry to any crate under `crates/`* without a decision record
  naming the §4.4 trust category and the claims its compromise would invalidate. A transport crate is
  therefore **not** exempt for being non-core, and an earlier framing in conversation that said otherwise
  was wrong. The shape that satisfies both: the dependency lives in the transport crate, and the
  dependence runs one way — transport → engine, never engine → transport — so the generator and the
  checker share nothing new and F30's independence argument is untouched. A serializer reachable from
  both the engine and its reporting surface is precisely the shared transitive helper F30 exists to
  surface.
- ⭐ **"The product spawns no subprocess and executes nothing" must be written down and gated, not
  observed.** Measured `2026-09-28`: `git grep -niE 'spawns? no|no subprocess|does not execute|never
  executes|no child process'` over tracked files returns **nothing**. It is the property that makes
  handing archogen to an arbitrary agent safe, and a `Command::new` added to a product crate tomorrow
  would pass every gate in the tree. `API.2` owns it; the scanner shape already exists in
  `crates/eadl-front/tests/reference.rs`'s legs 4 and 5.
- **The surface is thin today and must not be advertised otherwise.** Per `docs/book/src/cli.md`, only
  `check` is built; `build` is experimental over the S0 path; `resolve`, `analyze`, `verify`, `explain`
  and `replay` are unimplemented (`M3.4`, `M2.6`, `PROGRAM.3`, `M3.4`, `M4.7`). An MCP server shipping
  now would expose one real capability, and the three-state maturity is what says so honestly.
- **`archogen verify` needs its own ruling before it is exposed.** It runs verification tiers, which
  invoke a toolchain and an emulator through `xtask` — side-effecting and subprocess-spawning, unlike
  everything else on this surface. It is recorded as an open question on the `API` tree rather than
  decided by analogy to `build`.

Related: [[zero-dependency-engine-core]] — the dependency rule this record must satisfy rather than
sidestep. [[eadl-engine-boundary]] — why an agent cannot inject implementation content through a
description, which is the other half of why this surface is safe to expose.
