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
- **Active tree:** `M3` → frontier `M3.6`. `M2`'s frontier is `M2.7.4`, on the director; `PROGRAM`'s frontier is `PROGRAM.34`, the director's; `API` is
  closed; `M1`'s open leaf is `M1.29.4`, blocked on the director. Every other tree's frontier head is in
  `docs/TASK_TREE.md`.
- **Next action:** `M3.6.3.3`, `cargo xtask trust-gate` and its two-part report — third of the trust gate's slices
  (F30); `.2.1`, the runner's baseline, waits on the CI job `.4` adds. `M2`'s open leaves all wait on the director:
  `M2.7.4.5` and `M2.7.6`'s review and hosting half. This project uses no branches.
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `PROGRAM.65` —
  where the CI rehearsal's checkout may live; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M5` — no board procured; `M2.7.4` and `M2.7.6.4` — findings §11, `main`'s protection and a reviewer;
  `TEMPLATE-REFS` — postponed. **The 17 template files archogen has not changed are never edited** (findings §10,
  ruled `2026-09-30`); every other script is archogen's.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; the integration tier is
  `cargo xtask verify --tier integration`.
