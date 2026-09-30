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
- **Active tree:** `M2` → frontier `M2.7.1`. `PROGRAM`'s frontier is `PROGRAM.32`, `API`'s `API.5`; `M1`'s only
  open leaf, `M1.29.4`, is blocked on the director. Every other tree's frontier head is in `docs/TASK_TREE.md`.
- **Next action:** **`M2.7.1`**. Run round 8 of the independent review of `docs/decisions/decision_catalog-records.md`
  and its `-example.md` in a new read-only context, judged against its §0 threat model (the director's ruling);
  rounds 1–7 are answered in `docs/reviews/decision_catalog-records-reviews.md`. Close the leaf when a round finds
  no defect. `M2.10.1`'s composition record awaits its own independent review. Then `M2.7.3`–`M2.7.5`, then `API.6`.
- **In flight:** branch **`wip/m2.9`** (`758cbcd`) is a checkpoint, not a finished leaf — `M2.9` records it as
  not compiling (`crates/rt-core/tests/differential.rs` unadapted).
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M2.9` — §6 (a)
  and (b), two rulings on `ROADMAP` §3.1.1; `M5` — no board procured;
  `API.5.5` — the director saw the Check answer match; the answer on load and the browser's name and version remain;
  `TEMPLATE-REFS` — postponed; **nothing under `scripts/` is touched** until the director says so, which also holds
  `PROGRAM.32`.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; the integration tier is
  `cargo xtask verify --tier integration`.
