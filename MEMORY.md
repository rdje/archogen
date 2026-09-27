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
- **Active tree:** `M1` → frontier `M1.20`. `PROGRAM` frontier `PROGRAM.11`; `M2` frontier `M2.8`.
- **Next action:** **`M1.20`** — re-measure the five `fixed-upstream` LinkedSpec reports at the
  adopted pin, through LinkedSpec's public route (`sexpr_file` / `SExprDocumentV1.spec` for the
  document requirements; RGX's public bootstrap for LS-004). Set `verified` only where archogen's
  own rerun shows the defect gone; a report that still fails stays `fixed-upstream`.
- **LinkedSpec pin adopted** `fd3e328d5dd5c80981a1c3b8496a27270291f7b8` (leaf `M1.19`,
  `2026-09-27`), nested RGX `f6e5acdc9`. ⛔ Adoption is **not** acceptance: nothing was re-run, so
  no issue state changed. See `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` is high priority:** the never-write-into-another-repository rule lived only in a
  session prompt, and upstream published a boundary-violation disclosure. Rule and root cause:
  `docs/decisions/decision_repository-boundary-read-only.md`.
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 —
  items (a) and (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`.
  `M2.9` cannot finish without them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), resumed
  with `git checkout wip/m2.9`. It does **not** compile: `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API, and ratchets `d1`–`d5` still assert a disagreement that
  `M2.9` resolved — each must be rewritten to assert agreement, never deleted (§14.1). That
  branch's commit message carries the full remaining list.
- **Also open:** `M1.12` (the language reference) → `M1.13` (freeze `eadl/1`), `M1.10`, `M2.8`
  (pin the installed QEMU), `PROGRAM.5`, `.8`, `.9`, `.10`.
- **Last application verification:** 421 tests, 0 failed (`M1.11`). The pin move touches no crate,
  so it re-ran no application test and claims none.
- **Run checks as tiers:** `make focused` per commit, `make integration` before a push. Exit
  **20 = incomplete** is not a pass — read what it names.
- **Blockers and missing tools** are named on every `make integration` run and owned by
  `docs/targets/first-target.md` and tree `M5`.
