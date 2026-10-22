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
- **Active tree:** `M3` → frontier `M3.5`; `PROGRAM` resumes at `PROGRAM.71`.
- **Next action:** `M3.5.1` (P2), role programs reached without a process, with `M2.21`: its design first, reviewed;
  then `M3.5.2`–`.6`, `PROGRAM.103`. The mutation campaign is `M7.6`'s, at completion. `M3.3.1` and `.2` hold F08 and F09 (`crates/eadl-resolve/src/joint.rs`),
  focused green at `eaf71286`; `archogen check` wiring is `M3.4`'s. The next artifact cleanup is due after
  `2026-10-23T15:54`. A test's deadline is one only a hang reaches (`PROGRAM.99`).
  **Mutation testing is stopped** until the functionality is locked
  (`decision_mutation-testing-after-completion.md`, `2026-10-21`): every pending `M3.6.9` leaf is `blocked`;
  every decision advances the project first. `M3.6.9.6.3.2.2.15.2.1.8.2` stays owned, a suspected defect, P2.
  Disposable project-owned Git fixtures under ignored `target/` are approved;
  maintained external repositories and submodules stay read-only.
  Corrected renderer adoption stays owned in `.4.3.4.1`.
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
