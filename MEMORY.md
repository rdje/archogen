# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `S0` → frontier leaf `S0.4`. `M0` done; `M1` reopened for the non-gate leaf
  `M1.10` and is otherwise done; `S0.1`–`S0.3` done.
- **Next action:** `S0.4` — **F28 as an automated test**: from a clean build directory, generate,
  compile, run and compare against `examples/s0-heartbeat/expected/`, for all three variants. The
  comparison has been run by hand and is byte-identical; the leaf is the mechanized version, and
  it belongs in `crates/archogen-cli/tests/s0_oracle.rs` (the last row of its header table).
- **Latest commit:** `ARCHOGEN-S0-0025 (leaf S0.3)` — `crates/archogen-s0` and a real
  `archogen build`, marked **experimental** on the command surface. The F28 corpus and its oracle
  (`ARCHOGEN-S0-0022`) landed **before** the emitter, which is what makes §12 S0's independence
  claim a fact of the commit order.
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
