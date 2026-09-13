# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `PROGRAM` → frontier `PROGRAM.4`. `S0` **closed** (F28 green); `M0` done; `M1`
  done except the non-gate leaf `M1.10`. `M2` is the next milestone tree (frontier `M2.3`).
- **Next action:** either **`PROGRAM.4`** (the book-structure pass — chapters have accreted one
  per leaf and nothing has checked that the whole mirrors the programme) or **`M2.3`** (the §7.4
  idealized response-time analysis, F18 — the first real engine knowledge, and the §13.2
  numerical baseline is already written down to check it against). `M2.3` is the milestone
  frontier and carries more value; `PROGRAM.4` is cheap. Also open and non-blocking: `M1.10`,
  `PROGRAM.5`, `PROGRAM.8`, `PROGRAM.9`, `PROGRAM.10`.
- **Run checks as tiers now:** `make focused` per commit, `make integration` before a push.
  Exit **20 = incomplete** is not a pass.
- **Latest commit:** `ARCHOGEN-PROGRAM-0029 (leaf PROGRAM.3)` — the §14.3 tier runner. Four of
  five tiers report **incomplete**, each naming its owning leaf; that is the honest picture, not
  a regression. ⛔ Carry the habit that found the last three defects: **mutate the subject
  at every gate**, and assert the mutation applied
  (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
