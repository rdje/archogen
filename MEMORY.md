# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `S0` → frontier leaf `S0.2`. `M0` and `M1` are **done**; `S0.1` is **done**.
- **Next action:** close `S0.2` honestly against the frontend `M1` actually built — its
  acceptance ("the three fixtures parse; a malformed fixture reports a span-localized error") is
  a re-verification, not new code. Then `S0.3`, the emitter.
- **Latest commit:** `ARCHOGEN-S0-0022 (leaf S0.1)` — the F28 corpus `examples/s0-heartbeat/` and
  its oracle `crates/archogen-cli/tests/s0_oracle.rs`, both landed **before** any emitter exists,
  which is what makes §12 S0's independence claim a fact of the commit order.
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
