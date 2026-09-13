# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M0` → frontier leaf `M0.3` (F27). `M1.1` (the reader) is done.
- **Next action:** implement F27 — the mechanical classifier over the 21-case boundary
  corpus, which the reader now parses (`docs/tasks/M0.md`, leaf `M0.3`). Then `M1.2`.
- **Latest commit:** `ARCHOGEN-M1-0010 (leaf M1.1)` — the eADL reader and its diagnostics.
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
