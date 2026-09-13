# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M0` → frontier leaf `M0.1` (`pending`). `PROGRAM` yielded the frontier.
- **Next action:** record the controlling eADL/engine boundary (`ROADMAP.md` §4, §4.3) as
  `docs/decisions/decision_eadl-engine-boundary.md` (`docs/tasks/M0.md`, leaf `M0.1`).
- **Latest commit:** `ARCHOGEN-PROGRAM-0003 (leaf PROGRAM.2)` — the `osgen` CLI shell.
- **In-flight uncommitted work:** none.
- **Blockers:** none. Absent tooling: QEMU RISC-V (needed by `M4.9`, `M2.8`).
