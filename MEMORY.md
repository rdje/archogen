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
- **Active tree:** `M1` → frontier `M1.29.2`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.29.2`, high — a disk-backed module loader** wired into `check` and `build` through
  the routing pair `M1.29.1` landed in `crates/archogen-cli/src/check_cmd.rs`, so §6's `module-*` codes are
  reachable from a command. ⛔ Read its two design constraints first: resolve the file by a *stated* rule
  from the imported name so `module-name-mismatch` stays firable, and a tracked module fixture is a
  `docs/semantics/conformance.md` manifest decision + migration note. Then `M1.29.3` (the name rule §6
  lacks), `M1.29.4` (parameters consumed by nothing).
- ⭐ **Closed today:** `M1.29.1` — module files refused as `unimplemented` (exit 20, was 10), and "elaborate"
  coupled to the capability by a leg; `M1.31` — markers clipped to their line (**14 runs / 10 files → 0**),
  with a pipeline leg over the whole conformance suite. ⛔ `M1.29.1`'s published "23 of 78" was wrong both
  ways (label text counted; output truncated by `tee | head`) — corrected everywhere to **10**.
- ⭐ **`PROGRAM.27` closed: `LANGUAGE-FREEZE` can fail on the real tree**, and a migration is **two
  commits** — the note `pending` with its movement, then flipped to `applied` (the gate refuses a note `HEAD`
  already carries as pending). `M1.29.2`'s fixtures will be its first client: read the migrations README.
- ⭐ **`eadl/1` is frozen and gated** (`M1.13`): `BASELINE.txt` (**72** digests) + `LANGUAGE-FREEZE`, whose
  **second** leg is the one **F-O** found inert. Also closed: `M1.28`, `M1.25`, `PROGRAM.21`, `M1.20`, `M1.12`.
- **⭐ Tree `API`** (ruled `2026-09-28`, `decision_programmatic-interface.md`): one engine API, a **wasm**
  binding and an **MCP server**, post-build only; `API.3`–`.7` waited on `M1.13`'s freeze, now closed, so
  the frontier order is a director's call and not a dependency. **`PROGRAM.11`** is the other active
  frontier: the repository-boundary rule in **both** directions (`decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and (b)
  of §6 in `decision_findings-for-director-review.md`; `M2.9` needs them. That branch (**`wip/m2.9`**,
  `758cbcdd`) is checkpointed, not finished, and does **not** compile — `crates/rt-core/tests/differential.rs`
  is unadapted to the reference's new API.
- **Also open:** `M1.26.1`, `.26.2`, `.30`, `.27`, `.32`, `.10`, `.21`, `.22`; `S0.8`; `M2.6`, `.8`; `PROGRAM.18`, `.24`, `.26`, `.13`,
  `.15`, `.17`, `.20`, `.5`, `.6`, `.9`, `.10`. ⚠️ `M1`'s open questions: `M1.13.1`'s invisible
  character, and the language's **name**.
- **Baseline to beat:** `make focused` exit `0` at the pin — **575 passed, 0 failed** over 41 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass. Push
  cadence at `400` ahead of `origin/main`, measured (`decision_push-cadence.md`). ⚠️ `integration` is
  **red** on `emulator`: QEMU **pinned** (`M2.8.1`), `TARGET_VERIFIED=no` until `M2.8.2`.
- ⛔ **Committing changed `2026-09-29`** (`PROGRAM.21`): `TASK-ACCEPTANCE` is leaf-scoped — a staged code
  change is **refused** unless the subject carries `(leaf <ID>)` or `TASK_ACCEPTANCE_LEAF=<ID>` is set.
