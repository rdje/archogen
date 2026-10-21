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
- **Active tree:** `M3` → frontier `M3.6`; `PROGRAM` resumes at `PROGRAM.71`.
- **Next action:** the full dependency-block checkpoint `M3.6.9.6.3.2.2.15.2.1.6.2` (P1): re-discover the
  current block and reconcile every historical obligation with exact outcomes; `.6.1` closed `2026-10-21`,
  Cargo's terminal-backslash refusal held as a control and the build error ruled an error, not a refusal.
  Disposable project-owned Git fixtures under ignored `target/` are approved;
  maintained external repositories and submodules stay read-only.
  Corrected renderer adoption stays owned in `.4.3.4.1`.
  Final full sweeps and review precede `M3.3`.
  `M3.6.9.1` is done, strict/attributed outcomes; `.6.1` and `PROGRAM.95` sealed with their log rows.
  The handoff command is `bash scripts/handoff_census.sh`; failed OS reads refuse.
  `M3.6.3.2.1.1` owns runner-baseline compatibility after key encoding; an authorized cadence run is needed. `M2`'s open leaves
  wait on the director: `M2.7.4.5` and `M2.7.6`'s review and hosting half. No branches.
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M5` — no board procured; `M2.7.4` and `M2.7.6.4` — findings §11, `main`'s protection and a reviewer;
  `TEMPLATE-REFS` — postponed. **The 17 template files archogen has not changed are never edited** (findings §10,
  ruled `2026-09-30`); every other script is archogen's.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; the integration tier is
  `cargo xtask verify --tier integration`.
