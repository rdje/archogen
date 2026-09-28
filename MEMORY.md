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
- **Active tree:** `M1` → frontier `M1.13.1`. `PROGRAM` `.11`; `API` `.1`; `S0` `.8`; `M2` `.8.2`.
- **Next action:** **`M1.13.1`** — settle **F-G**, first of the freeze's five children: make canonical
  form total so §3 rule 3 holds for *any* value, and refuse a raw control byte inside a string as the
  reader already refuses one between forms. Measured `2026-09-28`: ESC, BEL and **NUL** each read
  cleanly with no diagnostic and each print raw into the text §12 M4 hashes. `.1` is first because it
  changes the language, which costs a migration note only after `.4` writes the baseline; then `.2`–`.5`.
- ⛔ **Both findings `M1.12` routed to the freeze moved when `M1.13` re-measured them** — evidence and
  re-runnable censuses on the leaf, decisions in this tree's Decisions. **F-F**'s recorded impact is
  **falsified** (the widest standardized RISC-V address is 56-bit physical against a 2^63−1 domain), so
  the domain **stays 64-bit** for `eadl/1`. **F-G** is an inconsistency, not a missing rule, and
  refusing input is not the whole fix: `Form::Str` is constructible outside the frontend.
- **⭐ Tree `API`** (ruled `2026-09-28`, `ROADMAP.md` §10.4,
  `docs/decisions/decision_programmatic-interface.md`): one engine API, a **wasm** binding and an **MCP
  server**, post-build only. `API.3`–`.7` wait on `M1.13`; only `API.1` and `API.2` are unblocked.
- **Closed and named elsewhere, not restated here:** the LinkedSpec evaluation (`M1.20`, all five
  defects `verified` at pin `2ac834913` → `docs/feedback/linkedspec/INDEX.md`); `M1.12`, the normative
  language reference; the claim-verification spine (`docs/CLAIM_VERIFICATION.md`).
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions (`docs/decisions/decision_repository-boundary-read-only.md`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`), and it does
  **not** compile — `crates/rt-core/tests/differential.rs` is unadapted to the reference's new API.
- **Also open:** `M1.25`, `M1.26`, `M1.10`, `M1.21`, `M1.22`; `S0.8`; `M2.6`, `M2.8`; `PROGRAM.21`
  (**high**), `.18`, `.24`, `.13`, `.15`, `.17`, `.20`, then `.5`, `.6`, `.9`, `.10` — each with its
  census on its own leaf.
- **Baseline to beat:** `make focused` exit `0` at the pin — **471 passed, 0 failed** over 37 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass —
  read what it names (missing tools: `docs/targets/first-target.md`, tree `M5`; ISA/target specs:
  read-only `chipdoc` via `ARCHOGEN_CHIPDOC_ROOT`). **Push cadence: `400` commits ahead of
  `origin/main`** (`docs/decisions/decision_push-cadence.md`; `git rev-list --count origin/main..HEAD`
  for the live number; `PROGRAM.23` makes it enforced).
  ⚠️ `make integration` is **red** on one step, `emulator`: QEMU 11.1.1 is **pinned** (`M2.8.1`) but
  `TARGET_VERIFIED=no` until the §3.2 agreement check exists, so `M2.8.2` then `M2.8.3` own it.
  Artifact cleanup last ran `2026-09-28`.
