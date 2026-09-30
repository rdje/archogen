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
- **Active tree:** `M2` → frontier `M2.10.1`. `PROGRAM`'s frontier is `PROGRAM.34`, the director's; `API`'s is
  `API.6`; `M1`'s only open leaf, `M1.29.4`, is blocked on the director. Every other tree's frontier head is in
  `docs/TASK_TREE.md`.
- **Next action:** `M2.10.1`'s round 10 is in review in a read-only context; answer it, and close the leaf on a
  round with no defect live for the `/1` slice. `M2.7.1` is closed. `M2.7.3`, the catalog crate, is under way:
  `M2.7.3.3.2` (the lock over history) next, then `.4`–`.7`, `M2.7.6`, `M2.7.4`, `M2.7.5`, `M2.11`, then `API.6`.
  `docs/decisions/` is near its cap.
- **In flight:** branch **`wip/m2.9`** (`758cbcd`) is a checkpoint, not a finished leaf — `M2.9` records it as
  not compiling (`crates/rt-core/tests/differential.rs` unadapted).
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M2.9` — §6 (a)
  and (b), two rulings on `ROADMAP` §3.1.1; `M5` — no board procured; `M2.7.4` — findings §11, `main`'s protection;
  `TEMPLATE-REFS` — postponed. **The 17 template files archogen has not changed are never edited** (findings §10,
  ruled `2026-09-30`); every other script is archogen's.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; the integration tier is
  `cargo xtask verify --tier integration`.
