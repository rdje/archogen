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
- **Active tree:** `M1` → frontier `M1.20.7`, the last of seven sub-leaves under `M1.20`.
  `PROGRAM` frontier `PROGRAM.11`; `M2` frontier `M2.8`.
- **Next action:** **`M1.20.7`** — reconcile the register against every issue sub-tree and close
  `M1.20`: states, totals, the tracker `README.md`, this tree's frontier and the `docs/TASK_TREE.md`
  index row (which still names `M1.20.1`). Bookkeeping with a census, not measurement — read each
  sub-tree's State field rather than trusting the index. Then `M1.12` (the language reference),
  `M1.21` (file the 753 MB-log finding as `LS-008`), `M1.22` (third opinion on the normative grammar).
- **LinkedSpec pin = latest published head** `2ac834913d85c32f532be9b0aab63644838a577a` (leaves
  `M1.19` → `M1.19.1`), nested RGX `f6e5acdc9`, PGEN `d9d41c28`. **All five reported defects are
  `verified`** on archogen's own reruns — both blockers included, nothing left on the vendor's word.
  Four standing facts: the root `Cargo.toml` carries `exclude = ["vendor/linkedspec"]` (required,
  load-bearing — do not remove); the checkout's generated parser was **regenerated at this pin**
  (`50eec63c…` → `196db2ee…`) and a bootstrap is idempotent on existence, so regenerate after any pin
  move; LS-002/LS-003's remedy is a **new route** — `sexpr_file` + `SExprDocumentV1.spec` — while
  `lispish_file` + `Lispish.spec` keeps truncating and erasing kinds by design, now guarded; and both
  consumers are built behind `scripts/linkedspec_eval.sh` (`bins`, `run <bin> …`).
  ⚠️ Follow the integration guide **at `2ac834913`**; it changed after `fd3e328d5`. See
  `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions — `docs/decisions/decision_repository-boundary-read-only.md`.
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), resumed with
  `git checkout wip/m2.9`. It does **not** compile: `crates/rt-core/tests/differential.rs` is
  unadapted to the reference's new API, and ratchets `d1`–`d5` still assert a disagreement `M2.9`
  resolved — rewritten to assert agreement, never deleted (§14.1). Its commit message lists the rest.
- **Also open:** `M1.12` (the language reference) → `M1.13` (freeze `eadl/1`), `M1.10`, `M1.21`,
  `M1.22`, `M2.8` (pin the installed QEMU), `PROGRAM.5`, `.8`, `.9`, `.10`, `.13`, `.14`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **421 passed, 0 failed** over 36 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass —
  read what it names (missing tools are owned by `docs/targets/first-target.md`, tree `M5`).
