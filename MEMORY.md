# MEMORY — resume pointer (layer A; overwrite-only, keep ≤ ~50 lines)

> The bounded layer-A resume pointer (see `MEMORY_ARCHITECTURE.md`). OVERWRITE the
> "Current state" block each update — never append history here.

## How to resume

1. Read `README.md`, `MEMORY_ARCHITECTURE.md`, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`.
2. Open `docs/TASK_TREE.md` → the active tree below → its Current Frontier → next action.

## Current state

- **Project:** archogen — the eADL → OS generation toolchain (`ROADMAP.md` revision 2.0).
- **Active tree:** `M1` → frontier `M1.12`. (`M2` frontier is `M2.8`.) `S0` **closed** (F28 green); `M2.1`, `M2.3` (**F18
  green**), `M2.4` and `M2.5` (**F29 green**) closed; `M0` done; `M1` done except the non-gate
  leaf `M1.10`; `PROGRAM` frontier `PROGRAM.5`.
- **Next action:** **`M1.12`** — the **language reference**: the normative rules a grammar cannot
  carry. Exactness (no float, anywhere — §7.4), comment retention and the `; key: value` header
  convention, canonical form and what it guarantees, module/import semantics, and `defkind`'s
  meaning and limits (§5.6: it must not become a host-code evaluator). ⛔ `M1.11` proved the
  grammar and the reader agree on the *language* and on *token boundaries*; **values are still
  checked only against the reader** — a reader that read `1.5` as three halves would pass every
  conformance test. That is the gap `M1.12` closes, and `M1.13` then freezes the result as
  `eadl/1`. Also open: `M2.8` (pin the installed QEMU), `M2.9` (**blocked on a director
  decision**), `M1.10`, `PROGRAM.5`, `.8`, `.9`, `.10`.
- **Run checks as tiers:** `make focused` per commit, `make integration` before a push. Exit
  **20 = incomplete** is not a pass — read what it names.
- **Latest commit:** `ARCHOGEN-M1-0036 (leaf M1.11)` — the eADL surface syntax is **normative**
  for the first time (`docs/semantics/grammar.md`), with a recognizer derived from it that must
  agree with the reader on language *and* token spans.
- **Carry the habit that found the last four defects:** mutate the subject at every gate, and
  assert the mutation applied (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`,
  `docs/knowledge/verify-the-mutation-applied.md`).
- **In-flight uncommitted work:** none.
- **Blockers / missing tools** — `make integration` and `cargo xtask verify --tier
  extended` name these on every run: no physical board procured → tree `M5` blocked
  (director decision, `docs/targets/first-target.md`). `qemu-system-riscv64` absent → the
  integration tier's emulator step, `M2.8`'s emulator half, `M4.9`. The
  `riscv64imac-unknown-none-elf` rustup target absent → the bare-metal compile step.
  `cargo-miri` absent → the extended tier's Miri step.
