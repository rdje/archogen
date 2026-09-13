# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M2` → frontier `M2.5`. `S0` **closed** (F28 green) and `M2.3` closed (**F18
  green**) and `M2.4` closed; `M0` done; `M1` done except the non-gate leaf `M1.10`; `PROGRAM` frontier `PROGRAM.4`.
- **Next action:** **`M2.5`** — **F29**. §13.4 specifies the whole fixture: a twelve-interval
  trace over `[0, 23)`, a ledger totalling 23, H finishing at 9 and 19, L at 23 (missing a
  deadline of 22 by one), and four required controls. Build the trace as a
  `rt_analysis::cost::Ledger` — the disjointness rule is already structural, so the
  duplicate-charge control is close to free — and read the expected numbers out of §13.4 rather
  than copying them, as F18 does with §13.2. Also open and non-blocking: `M1.10`, `PROGRAM.4`,
  `.5`, `.8`, `.9`, `.10`.
- **Run checks as tiers now:** `make focused` per commit, `make integration` before a push.
  Exit **20 = incomplete** is not a pass.
- **Latest commit:** `ARCHOGEN-M2-0031 (leaf M2.4)` — §7.4.1's accounting contract, published and
  enforced by construction (`Ledger::seal`).
  ⚠️ Nothing in it may be cited for a **runtime** claim until `M2.6` supplies the variant that
  charges overhead; the crate says so in its own module docs. ⛔ Carry the habit that found the last three defects: **mutate the subject
  at every gate**, and assert the mutation applied
  (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
