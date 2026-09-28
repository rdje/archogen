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
- **Active tree:** `M1` → frontier `M1.13.2`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.13.2`** — state the value domain as a property of `eadl/1`, not an
  implementation limit. Already decided and recorded in the tree's Decisions: **it stays 64-bit**,
  because the widest standardized RISC-V address is 56-bit physical / 57-bit virtual against 2^63−1 and
  the corpus's largest literal of 27 is 2^28. So: write the `docs/decisions/` record, replace §1's ⚠️
  implementation-shaped marker with a stated rule, and name the compatibility-corpus row a later
  widening would add. Then `.3` (the version identifier) `.4` (the suite and baseline) `.5` (the gate).
- ⛔ **`M1.13.1` closed F-G as three defects, not one** — a printer escaping four characters and copying
  the rest, a reader refusing a control byte between forms but copying it inside a string, and a grammar
  accepting a raw line feed the reader refused. Closed by executed rows: `\u{…}` in the language,
  `- control` in the grammar, and a `<0xNN>` source notation that made a raw **NUL** a table row for the
  first time — which is also the notation `M1.26` gap (b) was missing. The fix lives in the printer,
  because `Form::Str` is constructible outside the frontend and need never pass a reader.
- **⭐ Tree `API`** (ruled `2026-09-28`, `ROADMAP.md` §10.4,
  `docs/decisions/decision_programmatic-interface.md`): one engine API, a **wasm** binding and an **MCP
  server**, post-build only. `API.3`–`.7` wait on `M1.13`; only `API.1` and `API.2` are unblocked.
- **Closed, and named rather than restated:** the LinkedSpec evaluation (`M1.20`, all five defects
  `verified` at pin `2ac834913` → `docs/feedback/linkedspec/INDEX.md`); `M1.12`, the normative language
  reference; the claim-verification spine (`docs/CLAIM_VERIFICATION.md`).
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions (`docs/decisions/decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), and it does
  **not** compile — `crates/rt-core/tests/differential.rs` is unadapted to the reference's new API.
- **Also open:** `M1.25`, `M1.26`, `M1.10`, `M1.21`, `M1.22`; `S0.8`; `M2.6`, `M2.8`; `PROGRAM.21`
  (**high**), `.18`, `.24`, `.13`, `.15`, `.17`, `.20`, then `.5`, `.6`, `.9`, `.10` — each with its
  census on its own leaf.
- **Baseline to beat:** `make focused` exit `0` at the pin — **474 passed, 0 failed** over 37 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass —
  read what it names (missing tools: `docs/targets/first-target.md`, tree `M5`; ISA/target specs:
  read-only `chipdoc` via `ARCHOGEN_CHIPDOC_ROOT`). **Push cadence: `400` commits ahead of
  `origin/main`** (`docs/decisions/decision_push-cadence.md`; `PROGRAM.23` makes it enforced).
  ⚠️ `make integration` is **red** on `emulator`: QEMU 11.1.1 is **pinned** (`M2.8.1`) but
  `TARGET_VERIFIED=no` until the §3.2 agreement check exists, so `M2.8.2` then `M2.8.3` own it.
