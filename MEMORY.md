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
- **Active tree:** `M1` → frontier `M1.20.1`, the first of seven sub-leaves under `M1.20` (one
  re-measurement each, plus the preparation, the build and the register reconciliation).
  `PROGRAM` frontier `PROGRAM.11`; `M2` frontier `M2.8`.
- **Next action:** **`M1.20.1`** — re-measure **LS-005** (guide ordering) against the guide **at
  `2ac834913`**: does "Add and pin" now send the reader to workspace setup, local storage and RGX
  preparation before any build? Then `M1.20.2` (LS-001), `.3` (LS-004), `.4` (build), `.5` (LS-002),
  `.6` (LS-003), `.7` (register). Set `verified` only where archogen's own rerun shows the defect
  gone; a report that still fails stays `fixed-upstream`.
- **LinkedSpec pin = latest published head** `2ac834913d85c32f532be9b0aab63644838a577a` (leaves
  `M1.19` → `M1.19.1`, `2026-09-27`), nested RGX `f6e5acdc9`; it contains every remedy commit.
  ⛔ Adoption is **not** acceptance: no LinkedSpec reproducer was re-run, so no issue state changed.
  ⚠️ The Rust integration guide changed between `fd3e328d5` and head — `M1.20` follows the guide
  **at `2ac834913`**. See `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` (medium):** the repository boundary was nowhere in the committed tree, in **either**
  direction. LinkedSpec's agent once wrote `.md` files *into this repo* to deliver its fix notice —
  inbound, handled correctly by `M1.18`, corrected upstream, not expected to recur. Rule + audit:
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
- **Last application verification:** `make focused` exit `0` at the adopted pin — **421 passed,
  0 failed** over 36 suites, unchanged from `M1.11`, as expected: no crate depends on `vendor/`.
- **Run checks as tiers:** `make focused` per commit, `make integration` before a push. Exit
  **20 = incomplete** is not a pass — read what it names.
- **Blockers and missing tools** are named on every `make integration` run and owned by
  `docs/targets/first-target.md` and tree `M5`.
