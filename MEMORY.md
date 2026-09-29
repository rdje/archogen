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
- **Active tree:** `M1` → frontier `M1.25`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.25`** — a latent unsoundness in the gate that closed `M1.23`: its past-tense
  escape is a substring scan, so a stale figure on a line containing `was`, `were` or `before` is excused
  silently. No live wrong figure behind it; `M1.26` is sequenced behind it. Then `.27`, `.10`, `.21`,
  `.22`. ⚠️ Open in `## Open Questions`: `M1.13.1`'s invisible character, and the language's **name** —
  now free to decide, since `M1.13`'s freeze is closed and a name would land as a migration note.
- ⛔ **`M1.13.2` settled F-F, and both measurements it was routed on were false** — a census figure with
  an unstated scope and a radix-scoped maximum, restated in **7** places, and an address argument true
  of *physical* addresses only. Details: `docs/decisions/decision_eadl1-value-domain.md`.
- ⭐ **`M1.13` is closed: `eadl/1` is frozen and gated.** One manifest (`docs/semantics/conformance.md`,
  **no count** in it), one reader, four two-sided rules, `docs/semantics/BASELINE.txt` (**70** digests,
  enumerated at run time), and `LANGUAGE-FREEZE` — whose **second** leg is the one that matters: it
  compares the tracked baseline with `HEAD`'s, so `--emit` needs a pending note in
  `docs/semantics/migrations/` and is not the waiver a one-legged gate leaves open. Costs **1.5–1.7 s** of
  the pre-commit path, measured and kept with a named trigger for revisiting. Also closed: `M1.13.4`,
  `M1.13.3`, `PROGRAM.21`, `M1.13.1`, `M1.20`, `M1.12`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a
  **wasm** binding and an **MCP server**, post-build only. `API.3`–`.7` waited on `M1.13`'s freeze, which
  is now closed — so they are unblocked and `API`'s frontier order is a director's call, not a dependency.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions (`docs/decisions/decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), and it does
  **not** compile — `crates/rt-core/tests/differential.rs` is unadapted to the reference's new API.
- **Also open:** `M1.26` (before any value-domain widening), `M1.27`, `M1.10`, `M1.21`, `M1.22`; `S0.8`;
  `M2.6`, `M2.8`; `PROGRAM.11`, `.18`, `.24`, `.26`, `.13`, `.15`, `.17`, `.20`, then `.5`, `.6`, `.9`, `.10`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **535 passed, 0 failed** over 39 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass.
  **Push cadence:** at `400` ahead of `origin/main`, measured not carried — `git rev-list --count
  origin/main..HEAD`, `decision_push-cadence.md`. ⚠️ `make integration` is **red** on `emulator`: QEMU
  is **pinned** (`M2.8.1`) but `TARGET_VERIFIED=no` until the §3.2 agreement check exists (`M2.8.2`).
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged
  code change is **refused** unless the pending message's subject carries `(leaf <ID>)`, or
  `TASK_ACCEPTANCE_LEAF=<ID>` is set. It no longer falls back to the first checklist in the file.
