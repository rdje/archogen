# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M0` → frontier leaf `M0.2` (`pending`).
- **Next action:** write the boundary corpus — paired accepted/rejected declarations,
  including ambiguous cases — under `docs/semantics/boundary/` (`docs/tasks/M0.md`, `M0.2`).
- **Latest commit:** `ARCHOGEN-M0-0005 (leaf M0.4)` — `rt-static-up-v1` as checked data.
- **In-flight uncommitted work:** none.
- **Blockers:** none. Absent tooling: QEMU RISC-V (needed by `M4.9`, `M2.8`).
