# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here. It answers one question,
> *what is next?*, and nothing else: warnings, lessons, measurements and completed work
> belong in the layer that owns them, which this file may name but not restate.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M1` → frontier `M1.13.3`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.13.3`** — put the version identifier on the surface. It **cannot ride the
  comment-header convention**: §3 drops comments from canonical form and §12 M4 hashes canonical text,
  so it must be a **form** — a grammar change free only before `.4` writes the baseline. Decidable only
  here: what **absence** of the identifier means, and `M1.13.1`'s routed question (an invisible
  character outside Unicode `Cc`). Then `.4` (suite + baseline), `.5` (the freeze gate).
- ⛔ **`M1.13.2` settled F-F, and both measurements it was routed on were false** — a census figure with
  an unstated scope and a radix-scoped maximum, restated in **7** places, and an address argument true
  of *physical* addresses only. The decision stands (exact signed 64-bit, widening deferred as
  compatible) on a reason that survives re-measurement; the argument and its three reversal triggers are
  `docs/decisions/decision_eadl1-value-domain.md`. The census is now a command, not a figure:
  `cargo run -q -p eadl-front --example literals -- docs/semantics examples docs/feedback`.
- **Closed, and named rather than restated:** `M1.13.1` (the escape set closed *and* sufficient, §3
  rule 3 total, in the printer because `Form::Str` is constructible outside the frontend); `M1.20`
  (LinkedSpec, five defects `verified` at pin `2ac834913`); `M1.12`; `docs/CLAIM_VERIFICATION.md`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `ROADMAP.md` §10.4,
  `docs/decisions/decision_programmatic-interface.md`): one engine API, a **wasm** binding and an
  **MCP server**, post-build only. `API.3`–`.7` wait on `M1.13`; only `API.1`/`API.2` are unblocked.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions (`docs/decisions/decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), and it does
  **not** compile — `crates/rt-core/tests/differential.rs` is unadapted to the reference's new API.
- **Also open:** `M1.25`, `M1.26` (sequenced *before* any value-domain widening), `M1.10`, `M1.21`,
  `M1.22`; `S0.8`; `M2.6`, `M2.8`; `PROGRAM.21` (**high, ACTIVE**), `.18`, `.24`, `.13`, `.15`, `.17`,
  `.20`, then `.5`, `.6`, `.9`, `.10`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **476 passed, 0 failed** over 37 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass,
  so read what it names (missing tools: `docs/targets/first-target.md`, tree `M5`; ISA specs:
  read-only `chipdoc` via `ARCHOGEN_CHIPDOC_ROOT`). **Push cadence:** at `400` ahead of `origin/main`,
  measured not carried — `git rev-list --count origin/main..HEAD`, `decision_push-cadence.md`.
  ⚠️ `make integration` is **red** on `emulator`: QEMU is **pinned** (`M2.8.1`) but `TARGET_VERIFIED=no`
  until the §3.2 agreement check exists, so `M2.8.2` then `M2.8.3` own it.
