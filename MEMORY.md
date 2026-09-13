# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** none — `S0` **closed** (all six leaves, F28 green). `M0` done; `M1` done except
  the non-gate leaf `M1.10`. `PROGRAM` is the substrate tree and `M2` is the next milestone.
- **Next action:** pick the next tree. Two candidates, and the choice is a judgement call:
  **`PROGRAM.3`** (the §14.3 tiered verification runner — `make check` is informal today, and the
  CI policy of running the full gate only before a push wants named tiers), or **`M2.3`** (the
  §7.4 idealized response-time analysis, F18 — the first real engine knowledge, with the §13.2
  numerical baseline already written down to check it against). `M2.3` is the milestone frontier;
  `PROGRAM.3` is substrate that every later tree uses. Also open: `M1.10`, non-blocking.
- **Latest commit:** `ARCHOGEN-S0-0028 (leaf S0.6)` — the S0 retirement contract, enforced by a
  new project doctrine. ⛔ Carry the habit that found the last three defects: **mutate the subject
  at every gate**, and assert the mutation applied
  (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
