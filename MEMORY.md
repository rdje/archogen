# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `S0` → frontier leaf `S0.1`. `M0` and `M1` are **done**.
- **Next action:** write the tiny S0 description, its changed and unsupported variants, and the
  independent expected-output oracle — **before** any emitter exists (`docs/tasks/S0.md`, `S0.1`).
- **Latest commit:** `ARCHOGEN-PROGRAM-0021 (leaf PROGRAM.2.1)` — CLI renamed `osgen` → `archogen`.
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
