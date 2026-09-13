# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M0` → frontier leaf `M0.5` (`pending`).
- **Next action:** record target selection (`ROADMAP.md` §3.2) — the pinned emulator
  configuration and the board decision or its recorded unavailability (`M0.5`).
- **Latest commit:** `ARCHOGEN-M0-0008 (leaf M0.7)` — the evidence vocabulary, encoded.
- **In-flight uncommitted work:** none.
- **Blockers:** none. Absent tooling: QEMU RISC-V (needed by `M4.9`, `M2.8`).
