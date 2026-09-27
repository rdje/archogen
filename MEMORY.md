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
- **Active tree:** `M1` → frontier `M1.20.3`, third of seven sub-leaves under `M1.20`.
  `PROGRAM` frontier `PROGRAM.11`; `M2` frontier `M2.8`.
- **Next action:** **`M1.20.3`** — re-measure **LS-004** through RGX's **public** `make bootstrap`
  (never by reconstructing PGEN's internals): two empty-store offline failure arms, one fresh
  success arm, one prepared-reuse arm. ⚠️ The generated parser sources in the checkout are dated
  `2026-09-20`, i.e. from the `8763a0e6`-era build, and PGEN has since moved to `d9d41c28` — the
  guide at the pin warns that stale generated files can make a bootstrap a silent no-op, so
  regenerate rather than reuse. Then `.4` (build `sexpr_file` + `lispish_file` into a **fresh**
  app-local target dir), `.5` (LS-002), `.6` (LS-003), `.7` (register).
- **LinkedSpec pin = latest published head** `2ac834913d85c32f532be9b0aab63644838a577a` (leaves
  `M1.19` → `M1.19.1`, `2026-09-27`), nested RGX `f6e5acdc9`; it contains every remedy commit.
  Re-measurement runs one row per sub-leaf: **LS-005 and LS-001 are `verified`** on archogen's own
  reruns; LS-002 … LS-004 stay `fixed-upstream` until theirs land. The root `Cargo.toml` now carries
  `exclude = ["vendor/linkedspec"]` — required by the vendor's guide and load-bearing for the nested
  PGEN manifest, so do not remove it.
  ⚠️ The Rust integration guide changed between `fd3e328d5` and head — `M1.20` follows the guide
  **at `2ac834913`**. See `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions — `docs/decisions/decision_repository-boundary-read-only.md`.
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 —
  items (a) and (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`.
  `M2.9` cannot finish without them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), resumed
  with `git checkout wip/m2.9`. It does **not** compile: `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API, and ratchets `d1`–`d5` still assert a disagreement that
  `M2.9` resolved — each must be rewritten to assert agreement, never deleted (§14.1). That
  branch's commit message carries the full remaining list.
- **Also open:** `M1.12` (the language reference) → `M1.13` (freeze `eadl/1`), `M1.10`, `M2.8`
  (pin the installed QEMU), `PROGRAM.5`, `.8`, `.9`, `.10`, `.13`, `.14`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **421 passed, 0 failed** over 36 suites.
  Run checks as tiers: `make focused` per commit, `make integration` before a push; exit
  **20 = incomplete** is not a pass — read what it names, and the missing tools it names are owned
  by `docs/targets/first-target.md` and tree `M5`.
