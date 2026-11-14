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
- **Active tree:** `M4` → frontier `M4.1`, the accepted runtime plan, the model's amendments done (`M4.1.2`): `/1` for
  the hosted playground; `PROGRAM` → `PROGRAM.34`, the director's.
- **The host is shared:** every heavy job goes through `scripts/host_guard.py`, one at a time (`docs/decisions/decision_host-guard.md`). A reader's brief says so.
- **Next action:** answer `M4.1.7.2.22`'s report (`target/m4365/report.md`: RP22-1, a source route's `external` owed on a
  target; RP22-2, a called node's `primitives-out-of-line` and balanced masking owed), then round 23. Closed reviews: the
  application inputs' (`M4.12.1`, round 20) and the host guard's (`PROGRAM.127`, round 49).
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M5` — no board procured; `M2.7.4.5` — the catalog records; `M2.7.6.4` — the director's review round;
  `TEMPLATE-REFS` — postponed. **The 17 template files archogen has not changed are never edited** (findings §10,
  ruled `2026-09-30`); every other script is archogen's.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; the integration tier is
  `cargo xtask verify --tier integration`.
