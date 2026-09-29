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
- **Active tree:** `M1` → frontier `M1.29.3`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** ⛔ **first, flip `docs/semantics/migrations/eadl-1-module-path-and-cases.md` to
  `status: applied`** — the freeze gate refuses every commit until it does (spent-note leg); do it in the
  docs-only `PROGRAM.20` note (two `modules.md` transcripts it listed as unverifiable are now verified).
  Then **`M1.29.3`, high — the name rule** an elaborated program needs: how a reference inside an instance
  resolves and what an `export` hides — written into `docs/semantics/reference.md` §6 first — then the model
  passes over the resolved program, F01/F02 type-checked through `archogen`, and "elaborate and type-check a
  description" back in `crates/archogen-cli/src/spec.rs` (two coupling legs in `module_files.rs` enforce the
  wording). The 26 cases are in `docs/semantics/modules/`; `module_cases.rs` expects `ok` cases to answer
  `unimplemented` naming `M1.29.3` and must change with it. Then `M1.29.4` (parameters).
- ⭐ **Closed today:** `M1.29.1`, `M1.31`, `PROGRAM.27`, and **`M1.29.2`** — `check`/`build` elaborate a module
  tree from its module path (§6 rules 7–8), 23 of 24 `module-` codes reached from a command.
- ⭐ **`PROGRAM.27` closed: `LANGUAGE-FREEZE` can fail on the real tree**, and a migration is **two
  commits** — the note `pending` with its movement, then flipped to `applied` (the gate refuses a note `HEAD`
  already carries as pending). `M1.29.2`'s fixtures will be its first client: read the migrations README.
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
- **Also open:** `M1.26.1`, `.26.2`, `.30`, `.27`, `.32`, `.10`, `.21`, `.22`; `S0.8`; `M2.6`, `.8`; `PROGRAM.18`, `.24`, `.26`, `.13`,
  `.15`, `.17`, `.20`, `.5`, `.6`, `.9`, `.10`. ⚠️ `M1`'s open questions: `M1.13.1`'s invisible
  character, and the language's **name**.
- **Baseline to beat:** `make focused` exit `0` at the pin — **590 passed, 0 failed** over 42 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass. Push
  cadence at `400` ahead of `origin/main`, measured (`decision_push-cadence.md`). ⚠️ `integration` is
  **red** on `emulator`: QEMU **pinned** (`M2.8.1`), `TARGET_VERIFIED=no` until `M2.8.2`.
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged code
  change is **refused** unless the subject carries `(leaf <ID>)` or `TASK_ACCEPTANCE_LEAF=<ID>` is set.
