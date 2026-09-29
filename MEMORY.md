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
- **Active tree:** `M1` → frontier `M1.29`. `PROGRAM` `.27`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.29`, high — nothing in production elaborates a module tree**, and three surfaces
  claim otherwise. `git grep -n "elaborate(" -- crates` outside `src/module.rs` → two hits, both in
  `tests/f01_f02_modules.rs`; `MemoryModules` is the only `ModuleSource`; `archogen check` on `modules.md`'s
  own opening `(defmodule …)` answers `schema-unknown-kind`, while `spec.rs:151` — what `archogen help
  check` prints — says "elaborate and type-check". So §10.1 step 1 does not run, **none of §6's 24
  `module-*` codes is reachable from any command**, and F01/F02 are green at library level only. Then
  `M1.26.1` (gap (a)), `.2` (gap (b)), `M1.30` (§5.3's metadata, computed and dropped).
- ⭐ **`M1.28` is closed** (both children): `check` and `build` agree about a malformed quantity —
  `ValueType::Quantity`, four `task` clauses onto it, `refinement.rs` propagating not discarding,
  `Verdict::of_code` in one place (exit **70 → 10**). 76 descriptions censused: **1** changed, **0** lost.
- ⛔ **F-O → `PROGRAM.27`, high: `LANGUAGE-FREEZE`'s explicitness leg cannot fail on the real tree** —
  with the baseline amended and **no note at all** it prints `OK`: its notes grep matches the migrations
  `README.md`'s form template and `names_construct`'s `*all*` case reads it as covering every construct.
- ⭐ **`M1.13` is closed: `eadl/1` is frozen and gated** — one manifest (**no count** in it), one reader,
  four two-sided rules, `BASELINE.txt` (**72** digests enumerated at run time), and `LANGUAGE-FREEZE`, whose
  **second** leg is what stops `--emit` being the waiver — and is the leg **F-O** found inert. Also closed:
  `M1.25`, `M1.13.4`, `.3`, `PROGRAM.21`, `M1.13.1`, `M1.20`, `M1.12`; F-F by `M1.13.2`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a **wasm**
  binding and an **MCP server**, post-build only; `API.3`–`.7` waited on `M1.13`'s freeze, now closed, so
  the frontier order is a director's call and not a dependency. **`PROGRAM.11`** is the other active
  frontier: the repository-boundary rule in **both** directions (`decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and (b)
  of §6 in `decision_findings-for-director-review.md`; `M2.9` needs them. That branch (**`wip/m2.9`**,
  `758cbcdd`) is checkpointed, not finished, and does **not** compile — `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API.
- **Also open:** `M1.27`, `.10`, `.21`, `.22`; `S0.8`; `M2.6`, `.8`; `PROGRAM.18`, `.24`, `.26`, `.13`,
  `.15`, `.17`, `.20`, `.5`, `.6`, `.9`, `.10`. ⚠️ `M1`'s open questions: `M1.13.1`'s invisible
  character, and the language's **name**.
- **Baseline to beat:** `make focused` exit `0` at the pin — **541 passed, 0 failed** over 39 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass. Push
  cadence at `400` ahead of `origin/main`, measured (`decision_push-cadence.md`). ⚠️ `integration` is
  **red** on `emulator`: QEMU **pinned** (`M2.8.1`), `TARGET_VERIFIED=no` until `M2.8.2`.
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged code
  change is **refused** unless the subject carries `(leaf <ID>)` or `TASK_ACCEPTANCE_LEAF=<ID>` is set.
