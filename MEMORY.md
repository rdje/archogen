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
- **Next action:** `PROGRAM.127.24`, the guard's review, round 24; then `M4.2.3.3.23`, round 23 of the hosted build's review, a fresh reader over round 22's answer, the memory ceiling among it; then
  `M4.1.6.12`, round 12 of the release alarm's review (`M4.1.6` done);
  and `M4.5.2`; `M4.3.4` waits on `M2.21`, `M4.3.6` on `M4.1.7`. The runtime plan's design is accepted (`M4.1.1` done).
  `M3.2.7` and `M3.5.9`, full verification, wait for the
  features (doctrine 22).
  After the first batch, runner leaves `M3.5.3.2`, `M3.5.5.7.4`, `M3.5.1.4.5`, `M3.5.1.6.6`; `M3.4.7` waits on `M3.7`.
  `main` is protected: a push is a batch, `scripts/open_batch.sh` (`COMMIT.md` step 2), at the ruled cadence or to
  restore CI; the director merges, and the first merge is premise 3's named commit (`M2.7.6.6.5`). No other branches.
  The next artifact cleanup is due after `2026-11-07T01:39` UTC (`PROGRAM.130` ran the twenty-third).
  **Mutation testing is stopped** until the functionality is locked
  (`decision_mutation-testing-after-completion.md`, `2026-10-21`): every pending `M3.6.9` leaf and `PROGRAM.78` are
  `blocked`; every decision advances the project first. `M3.6.9.6.3.2.2.15.2.1.8.2` stays owned, a suspected defect.
  Disposable project-owned Git fixtures under ignored `target/` are approved;
  maintained external repositories and submodules stay read-only.
  The handoff command is `bash scripts/handoff_census.sh`; failed OS reads refuse.
  `M3.6.3.2.1.1` owns runner-baseline compatibility after key encoding; an authorized cadence run is needed.
- **⏳ Blockers (the director's):** `PROGRAM.34` — a yes to restore nine nested vendored checkouts; `M1.29.4` — §7 of
  `decision_findings-for-director-review.md`; `M5` — no board procured; `M2.7.4.5` — the catalog records; `M2.7.6.4` — the director's review round;
  `TEMPLATE-REFS` — postponed. **The 17 template files archogen has not changed are never edited** (findings §10,
  ruled `2026-09-30`); every other script is archogen's.
- **Derive, don't copy:** the test baseline is `cargo test --all -q` (must be 0 failed); the push distance
  and the ruled threshold are `bash scripts/push_cadence.sh`; the integration tier is
  `cargo xtask verify --tier integration`.
