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
- **Active tree:** `M1` → frontier `M1.28`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.28`, high — a live production defect.** `archogen check` **accepts** a
  description `archogen build` cannot realize: `(period 10 parsec)` passes `check` and is refused as
  `quantity-unknown-unit` by `build`, and a zero tick rate (§13.1's **F03**, an M1 gate fixture) draws no
  diagnostic from `check` at all. Three consumers of `Quantity::read` discard what it finds, the only one
  that propagates is the S0 prototype, and the schema cannot ask — `period` is `(holds values number
  symbol)` and `ValueType` has no quantity. Costs a migration note; sequenced **ahead of `M1.26.1`**,
  which would otherwise be born stating a rule the pipeline does not enforce. Then `M1.26.1`, `.2`.
- ⛔ **Every gate stayed green because all of them are existence censuses** — promoted to
  `docs/knowledge/an-existence-census-cannot-see-a-discarded-result.md` with three more instances of the
  shape: **F-H** (`M1.26.2`) a §4 row naming a mechanism `grep` cannot find; **F-J** (`M1.26.1`) a code
  whose only site is a totality arm; **F-K** (`M1.28`) the F03 case passing on a clause-name typo.
  **F-I** (`M1.26.1`): the frozen population is scoped by *file*, and nothing says why.
- ⭐ **`M1.13` is closed: `eadl/1` is frozen and gated** — one manifest (**no count** in it), one reader,
  four two-sided rules, `BASELINE.txt` (**70** digests enumerated at run time), and `LANGUAGE-FREEZE`,
  whose **second** leg compares the tracked baseline with `HEAD`'s, so `--emit` needs a pending migration
  note and is not the waiver. Also closed: `M1.25`, `M1.13.4`, `.3`, `PROGRAM.21`, `M1.13.1`, `M1.20`,
  `M1.12`; F-F by `M1.13.2`, `decision_eadl1-value-domain.md`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a **wasm**
  binding and an **MCP server**, post-build only; `API.3`–`.7` waited on `M1.13`'s freeze, now closed, so
  the frontier order is a director's call and not a dependency. **`PROGRAM.11`** is the other active
  frontier: the repository-boundary rule in **both** directions (`decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and (b)
  of §6 in `decision_findings-for-director-review.md`; `M2.9` needs them. That branch (**`wip/m2.9`**,
  `758cbcdd`) is checkpointed, not finished, and does **not** compile — `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API.
- **Also open:** `M1.27`, `.10`, `.21`, `.22`; `S0.8`; `M2.6`, `.8`; `PROGRAM.18`, `.24`, `.26`, `.13`,
  `.15`, `.17`, `.20`, `.5`, `.6`, `.9`, `.10`. ⚠️ `M1`'s open questions: `M1.13.1`'s invisible
  character, and the language's **name**.
- **Baseline to beat:** `make focused` exit `0` at the pin — **541 passed, 0 failed** over 39 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass. Push
  cadence at `400` ahead of `origin/main`, measured (`decision_push-cadence.md`). ⚠️ `integration` is
  **red** on `emulator`: QEMU **pinned** (`M2.8.1`), `TARGET_VERIFIED=no` until `M2.8.2`.
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged code
  change is **refused** unless the subject carries `(leaf <ID>)` or `TASK_ACCEPTANCE_LEAF=<ID>` is set.
