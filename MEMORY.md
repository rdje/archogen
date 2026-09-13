# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `PROGRAM` → frontier leaf `PROGRAM.2` (`pending`).
- **Next action:** replace the bedrock starter crate with the `osgen` CLI shell and the
  crate boundaries S0/M1 need (`docs/tasks/PROGRAM.md`, leaf `PROGRAM.2`).
- **Latest commit:** `ARCHOGEN-PROGRAM-0002 (leaf PROGRAM.1)` — roadmap seeded into ten trees.
- **In-flight uncommitted work:** none.
- **Blockers:** none. Absent tooling: QEMU RISC-V (needed by `M4.9`, `M2.8`).
