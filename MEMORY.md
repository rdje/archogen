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
- **Active tree:** `PROGRAM` → frontier `PROGRAM.6` (child `.6.3` format goldens left). `M1`
  `.29.4` (**waiting on the director**); `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`PROGRAM.6.3`** — a golden sample per evidence format, so a shape change under the same identifier is
  refused. ⏳ **`M1.29.4` waits on the director** — §7 of `decision_findings-for-director-review.md`. A new
  version constant or format needs an entry in `docs/book/src/versions.md` (`VERSION-REGISTER`).
- ⭐ **Closed today:** `M1.29.1`–`.3`, `M1.31`, `M1.33`, `PROGRAM.27`, `PROGRAM.11`, `PROGRAM.18`, `PROGRAM.24`, `PROGRAM.28` (gate arms run in a tier), `PROGRAM.29` (scratch on this volume, gated), `PROGRAM.5` (§15 source ledger), `PROGRAM.9.1` (Miri step armed), `PROGRAM.9.2` (fuzz step: found 3 defects), `PROGRAM.9.3` (mutation catalog; `extended` passes), `PROGRAM.6.1` (version register), `PROGRAM.6.2` (frozen verdicts), `M1.34` (`Rational` ordering exact), `M1.35` (reader escape panic), `M1.36` (exact printing never overflows) — `archogen check` elaborates **and
  type-checks** a module tree (§6 rules 7–10); a name is declared once (§7 rule 6), for a file and a tree.
  30 cases in `docs/semantics/modules/`. A migration is **two commits** (`pending`, then `applied` — enforced).
- ⭐ **`eadl/1` is frozen and gated** (`M1.13`, `PROGRAM.27`): `BASELINE.txt` (**114** digests) +
  `LANGUAGE-FREEZE`, three legs. Also closed: `M1.28`, `M1.25`, `PROGRAM.21`, `M1.20`, `M1.12`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a **wasm**
  binding and an **MCP server**, post-build only; `API.3`–`.7` waited on `M1.13`'s freeze, now closed, so
  the frontier order is a director's call and not a dependency. **`PROGRAM.11`** is the other active
  frontier: the repository-boundary rule in **both** directions (`decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and (b)
  of §6 in `decision_findings-for-director-review.md`; `M2.9` needs them. That branch (**`wip/m2.9`**,
  `758cbcdd`) is checkpointed, not finished, and does **not** compile — `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API.
- **Also open:** `M1.26.1`, `.26.2`, `.30`, `.27`, `.32`, `.10`, `.21`, `.22`; `S0.8`; `M2.6`, `.8`;
  `PROGRAM.18`, `.24`, `.28`, `.26`, `.13`, `.15`, `.17`, `.20`, `.5`, `.6`, `.9`, `.10`. ⚠️ `M1`'s open
  questions: `M1.13.1`'s invisible character, and the language's **name**.
- **Baseline to beat:** `make focused` exit `0` at the pin — **602 passed, 0 failed** over 42 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass. Push
  cadence at `400` ahead of `origin/main`, measured (`decision_push-cadence.md`). ⚠️ `integration` is
  **red** on `emulator`: QEMU **pinned** (`M2.8.1`), `TARGET_VERIFIED=no` until `M2.8.2`.
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged code
  change is **refused** unless the subject carries `(leaf <ID>)` or `TASK_ACCEPTANCE_LEAF=<ID>` is set.
