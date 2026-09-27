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
- **Active tree:** `M1` → frontier `M1.12`. `PROGRAM` frontier `PROGRAM.11`; `M2` frontier `M2.8`.
- **Next action:** **`M1.12`** — the language reference: the rules a grammar cannot carry (exactness,
  no float anywhere; comment retention; canonical form; `defkind`'s limits). `M1.11` proved the tokens
  agree; **values** are still checked only against the reader. Then `M1.13` (freeze `eadl/1`),
  `M1.10`, `M1.21`, `M1.22`.
- **The LinkedSpec evaluation is CLOSED** (`M1.20`, `2026-09-27`): all five reported defects are
  `verified` at pin `2ac834913d85c32f532be9b0aab63644838a577a` (RGX `f6e5acdc9`, PGEN `d9d41c28`) on
  archogen's own reruns, and nothing in the register rests on the vendor's word. Four standing facts:
  the root `Cargo.toml` carries `exclude = ["vendor/linkedspec"]` (load-bearing — do not remove); a
  bootstrap is idempotent on **existence**, so regenerate the vendored parser after any pin move (it
  was regenerated here, `50eec63c…` → `196db2ee…`); the document route is `sexpr_file` +
  `SExprDocumentV1.spec`, while `lispish_file` + `Lispish.spec` truncates and erases kinds **by
  design** and is guarded against drift; and both consumers are built behind
  `scripts/linkedspec_eval.sh`. See `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions — `docs/decisions/decision_repository-boundary-read-only.md`. The spine is now **five**
  portable architectures: `docs/CLAIM_VERIFICATION.md` was adopted `2026-09-27` (`PROGRAM.16`) —
  re-derive · falsify · make durable — and `PROGRAM.18` (medium-high) owns the ten registered
  controls that have no repeatable RED arm.
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
