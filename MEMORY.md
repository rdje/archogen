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
- **Active tree:** `M1` → frontier `M1.12`. `PROGRAM` `.11`; `S0` reopened for `S0.8`; `M2` `.8.2`.
- **Next action:** **`M1.12`** — the language reference: the rules a grammar cannot carry (exactness,
  no float anywhere; comment retention; canonical form; `defkind`'s limits). `M1.11` proved the tokens
  agree; **values** are still checked only against the reader.
- **The LinkedSpec evaluation is CLOSED** (`M1.20`, `2026-09-27`): all five reported defects are
  `verified` at pin `2ac834913` (RGX `f6e5acdc9`, PGEN `d9d41c28`) on archogen's own reruns. Four
  standing facts: the root `Cargo.toml` carries `exclude = ["vendor/linkedspec"]` (load-bearing — do
  not remove); a bootstrap is idempotent on **existence**, so regenerate the vendored parser after any
  pin move; the document route is `sexpr_file` + `SExprDocumentV1.spec`, while `lispish_file` +
  `Lispish.spec` truncates and erases kinds **by design** and is guarded against drift; both consumers
  are built behind `scripts/linkedspec_eval.sh`. Details: `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions — `docs/decisions/decision_repository-boundary-read-only.md`. The spine is now **five**
  portable architectures: `docs/CLAIM_VERIFICATION.md`, adopted `2026-09-27` (`PROGRAM.16`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), resumed with
  `git checkout wip/m2.9`. It does **not** compile: `crates/rt-core/tests/differential.rs` is
  unadapted to the reference's new API, and ratchets `d1`–`d5` still assert a disagreement `M2.9`
  resolved — rewritten to assert agreement, never deleted (§14.1). Its commit message lists the rest.
- **Also open:** `M1.12` (the language reference) → `M1.13` (freeze `eadl/1`), `M1.10`, `M1.21`,
  `M1.22`, `M1.25`; `S0.8`; `M2.6`, `M2.8`; `PROGRAM.21` (**high** — `TASK-ACCEPTANCE` reads only the
  *first* checklist in a tree file), `.18`, `.13`, `.15`, `.17`, `.20`, then `.5`, `.6`, `.9`, `.10`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **430 passed, 0 failed** over 36 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass —
  read what it names (missing tools: `docs/targets/first-target.md`, tree `M5`; ISA/target specs:
  read-only `chipdoc` via `ARCHOGEN_CHIPDOC_ROOT`, `docs/decisions/`).
  **Push cadence: `400` commits ahead of `origin/main`** — ruled `2026-09-28`,
  `docs/decisions/decision_push-cadence.md`; `git rev-list --count origin/main..HEAD` for the live
  number; `PROGRAM.23` makes it enforced rather than prose.
  ⚠️ `make integration` is **red** on one step, `emulator`: QEMU 11.1.1 is installed and **pinned**
  (`M2.8.1`), but `TARGET_VERIFIED=no` until the §3.2 agreement check exists — and it has **neither**
  side yet, so `M2.8.2` (device-tree fixture) then `M2.8.3` (eADL platform description) own it.
  Artifact cleanup last ran `2026-09-27`; trigger and record in `docs/ARTIFACT_CLEANUP.md`.
