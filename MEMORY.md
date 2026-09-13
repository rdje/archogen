# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M2` → frontier `M2.8`. `S0` **closed** (F28 green); `M2.1`, `M2.3` (**F18
  green**), `M2.4` and `M2.5` (**F29 green**) closed; `M0` done; `M1` done except the non-gate
  leaf `M1.10`; `PROGRAM` frontier `PROGRAM.5`.
- **Next action:** **`M2.8`** — QEMU 11.1.1 is now installed, so `targets/riscv-virt-up.env`'s
  `QEMU_VERSION_PINNED=none-yet` / `TARGET_VERIFIED=no` make the integration tier **fail** rather
  than skip. Pin the release, run `scripts/target_emulator.sh --check` and `--dump-dtb`, compare
  the generated device tree against the eADL platform fixture, and flip `TARGET_VERIFIED` only on
  that evidence. ⛔ `M2.9` is **blocked on a director decision** — four of the five contract gaps
  in `docs/decisions/decision_runtime-contract-gaps.md` change the roadmap. Also open: `M1.10`,
  `PROGRAM.5`, `.8`, `.9`, `.10`.
- **Run checks as tiers:** `make focused` per commit, `make integration` before a push. Exit
  **20 = incomplete** is not a pass — read what it names.
- **Latest commit:** `ARCHOGEN-M2-0035 (leaf M2.2)` — an independent reference model and a
  differential harness. 16 000 events of exact agreement, and **five disagreements that are all
  gaps in the contract**. ⚠️ Nothing in `rt-analysis` may be cited for a runtime claim until
  `M2.6`.
- **Carry the habit that found the last four defects:** mutate the subject at every gate, and
  assert the mutation applied (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers / missing tools** — `make integration` and `cargo xtask verify --tier
  extended` name these on every run: no physical board procured → tree `M5` blocked
  (director decision, `docs/targets/first-target.md`). `qemu-system-riscv64` absent → the
  integration tier's emulator step, `M2.8`'s emulator half, `M4.9`. The
  `riscv64imac-unknown-none-elf` rustup target absent → the bare-metal compile step.
  `cargo-miri` absent → the extended tier's Miri step.
