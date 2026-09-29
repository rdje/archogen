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
- **Active tree:** `M1` → frontier `M1.13.5`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.13.5`** — the freeze gate, and the last child of `M1.13`:
  `scripts/check_language_freeze.sh` recomputes `docs/semantics/BASELINE.txt` and fails on any difference
  no migration note covers, keeping **moved** / **added** / **removed** apart (the comparator already
  classifies them). Two rules pre-decided: *any* movement needs a note including a correction, and
  regeneration must be an explicit act that **fails without a note**. Handed to it: the gate needs a Rust
  entry point (canonical form is only printable by the frontend) and `--emit` costs **0.8 s** warm against
  a pre-commit path that must stay cheap. Then `.25`, `.26`, `.27`, `.10`, `.21`, `.22`.
  ⚠️ Open in `## Open Questions`: `M1.13.1`'s invisible character, and the language's **name**.
- ⛔ **`M1.13.2` settled F-F, and both measurements it was routed on were false** — a census figure with
  an unstated scope and a radix-scoped maximum, restated in **7** places, and an address argument true
  of *physical* addresses only. Details: `docs/decisions/decision_eadl1-value-domain.md`.
- **Closed, and named rather than restated:** `M1.13.4` (all five children: one manifest with **no count**,
  one reader, four two-sided rules, and a **70**-digest baseline enumerated at run time — plus a
  production defect, a false normative sentence and five false book figures found on the way);
  `M1.13.3`; `PROGRAM.21`; `M1.13.1`; `M1.20`; `M1.12`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a
  **wasm** binding and an **MCP server**, post-build only. `API.3`–`.7` wait on `M1.13`.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions (`docs/decisions/decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), and it does
  **not** compile — `crates/rt-core/tests/differential.rs` is unadapted to the reference's new API.
- **Also open:** `M1.25`, `M1.26` (sequenced *before* any value-domain widening), `M1.10`, `M1.21`,
  `M1.22`; `S0.8`; `M2.6`, `M2.8`; `PROGRAM.11`, `.18`, `.24`, `.13`, `.15`, `.17`, `.20`, then `.5`,
  `.6`, `.9`, `.10`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **535 passed, 0 failed** over 39 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass.
  **Push cadence:** at `400` ahead of `origin/main`, measured not carried — `git rev-list --count
  origin/main..HEAD`, `decision_push-cadence.md`. ⚠️ `make integration` is **red** on `emulator`: QEMU
  is **pinned** (`M2.8.1`) but `TARGET_VERIFIED=no` until the §3.2 agreement check exists (`M2.8.2`).
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged
  code change is **refused** unless the pending message's subject carries `(leaf <ID>)`, or
  `TASK_ACCEPTANCE_LEAF=<ID>` is set. It no longer falls back to the first checklist in the file.
