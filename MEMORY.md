# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M1` → frontier leaf `M1.1` (`pending`). `M0` is done but for `M0.3`.
- **Next action:** build the reader — S-expressions, source spans, caret diagnostics
  (`docs/tasks/M1.md`, leaf `M1.1`). It unblocks `M0.3` (F27) and `S0.2`.
- **Latest commit:** `ARCHOGEN-M0-0009 (leaf M0.5)` — emulator pinned; no board procured.
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
