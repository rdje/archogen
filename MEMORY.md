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
- **Active tree:** `M1` → frontier `M1.12`. `M2` frontier `M2.8`; `PROGRAM` frontier `PROGRAM.5`.
- **Last application verification:** 421 tests, 0 failed; this upstream documentation notice does not re-run application tests.

- **LinkedSpec is published:** `fd3e328d5dd5c80981a1c3b8496a27270291f7b8` includes LS-004.
  `M1.18` records the upstream notice; five reports are fixed-upstream, awaiting ARCHOGEN
  adoption and verification. Follow `docs/feedback/linkedspec/INDEX.md` and the issue responses;
  document requirements use `sexpr_file` / `SExprDocumentV1.spec`, and bootstrap uses RGX's public route.
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 —
  items (a) and (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`.
  `M2.9` cannot finish without them.

- **Next action if unblocked:** **`M1.12`** — the **language reference**: the normative rules a
  grammar cannot carry (exactness, comment retention, canonical form, module/import semantics,
  `defkind`'s limits). ⛔ `M1.11` proved the grammar and the reader agree on the language and on
  token boundaries; **values are still checked only against the reader**. `M1.13` then freezes
  the result as `eadl/1`.

- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), resumed
  with `git checkout wip/m2.9`. It does **not** compile: `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API, and ratchets `d1`–`d5` still assert a disagreement
  that `M2.9` resolved — each must be rewritten to assert agreement, never deleted (§14.1).
  That branch's commit message carries the full remaining list.

- **Also open:** `M2.8` (pin the installed QEMU), `M1.10`, `PROGRAM.5`, `.8`, `.9`, `.10`.
- **Run checks as tiers:** `make focused` per commit, `make integration` before a push. Exit
  **20 = incomplete** is not a pass — read what it names.
- **Blockers and missing tools** are named on every `make integration` run and owned by
  `docs/targets/first-target.md` and tree `M5`.
