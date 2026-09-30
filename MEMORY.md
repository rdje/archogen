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
- **Active tree:** `M2` → frontier `M2.8.3.3`. `API`'s frontier is `API.5`; `M1`'s only open leaf, `M1.29.4`, and
  `PROGRAM`'s frontier are blocked (on a push, and on the director). Every other tree's frontier head is in
  `docs/TASK_TREE.md`; it is not copied here.
- **Next action:** **`M2.8.3.3`** — the triple and ISA set into the `.env`, read by `xtask`; then `M2.8.3.4` (flip
  `TARGET_VERIFIED`, lift the quarantine, correct the stale surfaces) and `M2.7`.
- **In flight:** branch **`wip/m2.9`** (`758cbcd`) is a checkpoint, not a finished leaf — `M2.9` records it as
  not compiling (`crates/rt-core/tests/differential.rs` unadapted).
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M2.9` — §6 (a)
  and (b), two rulings on `ROADMAP` §3.1.1; `PROGRAM.31`/`.32` — §8; `M5` — no board procured.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; `integration` is
  `incomplete`, its emulator step quarantined under `M2.8` (`make integration`).
