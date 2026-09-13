# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `S0` → frontier leaf `S0.5`. `M0` done; `M1` reopened for the non-gate leaf
  `M1.10` and is otherwise done; `S0.1`–`S0.4` done — **F28 is green**.
- **Next action:** `S0.5` — **source provenance**, the one F28 clause still unmet ("failure points
  have useful diagnostics and **basic source provenance**"). Each generated declaration must map
  back to the source form and the engine record that produced it; the spans are already carried on
  `archogen_s0::interpret::Task` and `Plan` for exactly this.
- **Latest commit:** `ARCHOGEN-S0-0026 (leaf S0.4)` — F28 automated. ⛔ Mutation testing found the
  gate was **blind to a hyperperiod/longest-period swap** because both fixtures are harmonic;
  closed with a unit test and `system-non-harmonic.eadl`. Carry the habit: mutate the subject at
  every gate (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
