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
- **Active tree:** `M1` → frontier `M1.13`. `PROGRAM` `.11`; `S0` reopened for `S0.8`; `M2` `.8.2`.
- **Next action:** **`M1.13`** — version the language and freeze the first compatibility baseline,
  carrying the two routed value-domain findings **F-F** (nothing at or above 2^63 is writable) and
  **F-G** (a control character other than `\n \t \r` cannot be escaped). **`M1.12` closed** `2026-09-28`.
- **⭐ Ruled `2026-09-28`, filing next:** archogen gains a **programmatic interface** — a declared
  engine API, a wasm binding and an **MCP server** any agent can drive. Rulings: a **new tree +
  `ROADMAP.md` §10 amendment**; **after the `M1.13` freeze**; MCP controls an instance **post-build
  only, both builds excluded** (archogen's compilation *and* `archogen build <description>`), so the
  surface stays pure. The measurements behind it go on that tree, not here.
- **The LinkedSpec evaluation is CLOSED** (`M1.20`, `2026-09-27`): all five reported defects are
  `verified` at pin `2ac834913` on archogen's own reruns, both blockers included. The standing
  consequences, the two consumer routes and the register: `docs/feedback/linkedspec/INDEX.md`.
- **`PROGRAM.11` (medium)** is the other active frontier: the repository-boundary rule in **both**
  directions — `docs/decisions/decision_repository-boundary-read-only.md`. The spine is now **five**
  portable architectures: `docs/CLAIM_VERIFICATION.md`, adopted `2026-09-27` (`PROGRAM.16`).
- **⏳ WAITING ON THE DIRECTOR:** two behaviour-changing rulings on `ROADMAP` §3.1.1 — items (a) and
  (b) of §6 in `docs/decisions/decision_findings-for-director-review.md`; `M2.9` needs them.
- **`M2.9` is checkpointed, not finished:** branch **`wip/m2.9`** (commit `758cbcdd`). It does **not**
  compile: `crates/rt-core/tests/differential.rs` is unadapted to the reference's new API, and ratchets
  `d1`–`d5` still assert a disagreement `M2.9` resolved — rewritten, never deleted (§14.1).
- **Also open:** `M1.25`, `M1.26` (the two gaps `M1.12.5` measured: the model layer's codes are stated
  normatively nowhere, and no input is pinned to any §4 row), `M1.10`, `M1.21`, `M1.22`; `S0.8`;
  `M2.6`, `M2.8`; `PROGRAM.21` (**high** — `TASK-ACCEPTANCE` reads only the *first* checklist in a tree
  file), `.18`, `.24`, `.13`, `.15`, `.17`, `.20`, then `.5`, `.6`, `.9`, `.10`.
- **Baseline to beat:** `make focused` exit `0` at the pin — **471 passed, 0 failed** over 37 suites.
  Tiers: `focused` per commit, `integration` before a push; exit **20 = incomplete** is not a pass —
  read what it names (missing tools: `docs/targets/first-target.md`, tree `M5`; ISA/target specs:
  read-only `chipdoc` via `ARCHOGEN_CHIPDOC_ROOT`, `docs/decisions/`).
  **Push cadence: `400` commits ahead of `origin/main`** (ruled `2026-09-28`,
  `docs/decisions/decision_push-cadence.md`; `git rev-list --count origin/main..HEAD` for the live
  number; `PROGRAM.23` makes it enforced rather than prose).
  ⚠️ `make integration` is **red** on one step, `emulator`: QEMU 11.1.1 is installed and **pinned**
  (`M2.8.1`), but `TARGET_VERIFIED=no` until the §3.2 agreement check exists — it has **neither** side
  yet, so `M2.8.2` then `M2.8.3` own it. Artifact cleanup last ran `2026-09-28`.
