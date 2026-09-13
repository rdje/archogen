# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M2` → frontier `M2.4`. `S0` **closed** (F28 green) and `M2.3` closed (**F18
  green**); `M0` done; `M1` done except the non-gate leaf `M1.10`; `PROGRAM` frontier `PROGRAM.4`.
- **Next action:** **`M2.4`** — the §7.4.1 cost-accounting contract: a versioned record of the
  timing observation boundary, what task bounds include, which operations are preemptible, the
  number and kind of context transitions, interrupt assumptions, and dispatch/critical-section
  costs. It must exist before `M2.5` (F29) can mean anything, and §13.4 already specifies F29's
  trace and its ledger total of 23 in full — the same "read the oracle from the specification"
  move F18 used. Also open and non-blocking: `M1.10`, `PROGRAM.4`, `.5`, `.8`, `.9`, `.10`.
- **Run checks as tiers now:** `make focused` per commit, `make integration` before a push.
  Exit **20 = incomplete** is not a pass.
- **Latest commit:** `ARCHOGEN-M2-0030 (leaf M2.3)` — `crates/rt-analysis`, F18 green.
  ⚠️ Nothing in it may be cited for a **runtime** claim until `M2.6` supplies the variant that
  charges overhead; the crate says so in its own module docs. ⛔ Carry the habit that found the last three defects: **mutate the subject
  at every gate**, and assert the mutation applied
  (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers:** no physical board procured → tree `M5` blocked (director decision;
  `docs/targets/first-target.md`). QEMU RISC-V not installed → `M2.8` emulator half, `M4.9`.
