# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here. It answers one question,
> *what is next?*, and nothing else: warnings, lessons, measurements and completed work
> belong in the layer that owns them, which this file may name but not restate.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `PROGRAM` → frontier `PROGRAM.10`. Every other tree's frontier head is in `docs/TASK_TREE.md`;
  it is not copied here.
- **Next action:** **`PROGRAM.10.4`** — the `integration` job in CI, provisioning `mdbook` and QEMU at their pins and
  calling `scripts/ci_integration.sh`.
- **In flight:** branch **`wip/m2.9`** (`758cbcd`) is a checkpoint, not a finished leaf — `M2.9` records it as
  not compiling (`crates/rt-core/tests/differential.rs` unadapted).
- **⏳ Blockers (the director's):** `M1.29.4` — §7 of `decision_findings-for-director-review.md`; `M2.9` — §6 (a)
  and (b), two rulings on `ROADMAP` §3.1.1; `PROGRAM.31`/`.32` — §8; `M5` — no board procured.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance is
  `git rev-list --count origin/main..HEAD` against the cadence in `decision_push-cadence.md`; `integration` is
  `incomplete`, its emulator step quarantined under `M2.8` (`make integration`).
