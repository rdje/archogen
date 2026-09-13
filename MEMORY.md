# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `S0` → frontier leaf `S0.6`, the last in the tree. `M0` done; `M1` reopened for
  the non-gate leaf `M1.10` and is otherwise done; `S0.1`–`S0.5` done — **every F28 clause green**.
- **Next action:** `S0.6` — the **retirement note**: a decision record listing every hard-coded S0
  assumption with the leaf that removes it, so no special-case generator can be grandfathered
  (§12 S0). The list is already drafted in `examples/s0-heartbeat/README.md`; the leaf turns it
  into a tracked record and checks nothing is missing from it.
- **Latest commit:** `ARCHOGEN-S0-0027 (leaf S0.5)` — `provenance.json`, resolved from both ends.
  ⛔ Carry the habit that found the last two defects: **mutate the subject at every gate**, and
  assert the mutation applied (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
