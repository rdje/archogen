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
- **Active tree:** `M1` → frontier `M1.31`. `PROGRAM` `.27`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.31`, medium — a caret overruns its line** in **23 of 78** tracked descriptions (all
  13 boundary rejects): `diagnostic.rs:264` sizes the caret by the whole span while printing only its first
  line. Clip to the line, mark continuation, re-run the census to 0. Sequenced ahead of `M1.29.2` because
  that leaf's module diagnostics label multi-line `(defmodule …)` forms. Then **`M1.29.2`** (disk loader,
  §6's codes reachable from a command — read its two ⛔ design constraints first: resolve by a *stated* rule
  from the imported name so `module-name-mismatch` stays firable, and a tracked module fixture is a
  `conformance.md` manifest decision + migration note), `M1.29.3` (the name rule §6 lacks), `.4` (params).
- ⭐ **`M1.29.1` closed `M1.29`'s live defect**: `check`/`build` refuse a module file as `unimplemented`
  (exit 20, was 10) through one routing pair in `check_cmd.rs`; `help check` says "type-check"; a leg in
  `crates/archogen-cli/tests/module_files.rs` couples "elaborate" to the capability and another fails if
  `M1.29.2` closes while the refusal still names it. 78 descriptions censused, 0 changed.
- ⛔ **F-O → `PROGRAM.27`, high: `LANGUAGE-FREEZE`'s explicitness leg cannot fail on the real tree** —
  with the baseline amended and **no note at all** it prints `OK`: its notes grep matches the migrations
  `README.md`'s form template and `names_construct`'s `*all*` case reads it as covering every construct.
- ⭐ **`eadl/1` is frozen and gated** (`M1.13`): `BASELINE.txt` (**72** digests) + `LANGUAGE-FREEZE`, whose
  **second** leg is the one **F-O** found inert. Also closed: `M1.28`, `M1.25`, `PROGRAM.21`, `M1.20`, `M1.12`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a **wasm**
  binding and an **MCP server**, post-build only; `API.3`–`.7` waited on `M1.13`'s freeze, now closed, so
  the frontier order is a director's call and not a dependency. **`PROGRAM.11`** is the other active
  frontier: the repository-boundary rule in **both** directions (`decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and (b)
  of §6 in `decision_findings-for-director-review.md`; `M2.9` needs them. That branch (**`wip/m2.9`**,
  `758cbcdd`) is checkpointed, not finished, and does **not** compile — `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API.
- **Also open:** `M1.26.1`, `.26.2`, `.30`, `.27`, `.10`, `.21`, `.22`; `S0.8`; `M2.6`, `.8`; `PROGRAM.18`, `.24`, `.26`, `.13`,
  `.15`, `.17`, `.20`, `.5`, `.6`, `.9`, `.10`. ⚠️ `M1`'s open questions: `M1.13.1`'s invisible
  character, and the language's **name**.
- **Baseline to beat:** `make focused` exit `0` at the pin — **569 passed, 0 failed** over 40 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass. Push
  cadence at `400` ahead of `origin/main`, measured (`decision_push-cadence.md`). ⚠️ `integration` is
  **red** on `emulator`: QEMU **pinned** (`M2.8.1`), `TARGET_VERIFIED=no` until `M2.8.2`.
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged code
  change is **refused** unless the subject carries `(leaf <ID>)` or `TASK_ACCEPTANCE_LEAF=<ID>` is set.
