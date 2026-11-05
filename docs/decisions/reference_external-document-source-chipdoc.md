# The external document source: `chipdoc`, read-only, entered by its `AGENTS.md`

- **Type:** `reference`
- **Date:** `2026-09-27`
- **Status:** `active`
- **Owner / source:** director directive, `2026-09-27` — "if you ever need additional ISA / RISC-V /
  … just shout"; and again, `2026-11-05`, naming the corpus's `AGENTS.md` — "If you ever need new CPU,
  DSP, MCU, BOARD, ISA or in general digital chip PDF just look here" (leaf `PROGRAM.128`); the mapping
  file is maintained on the `chipdoc` side and updated as material that would interest archogen is added
- **External sources:** [chipdoc](../book/src/ledger.md#chipdoc) · [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger

## The fact

A separate repository, **`chipdoc`**, curates primary-source CPU, DSP, MCU, board, ISA, interface and
protocol documentation. **A need for any digital chip's document starts there**, before the web and
before memory. **`AGENTS.md` at its root is the single entry point**: what exists, how to query it,
how to open it, how to cite it, and what to do when a document is not there. **`ARCHOGEN.md`** beside
it is a one-way map from archogen's document needs to the material it holds.

Its checkout location is deliberately **not recorded here**, since §12 forbids checkout-specific
absolute paths in tracked files. It stands outside this repository and is no data of this project's:
nothing here is written there. Supply it at run time as `ARCHOGEN_CHIPDOC_ROOT`, which a session
without it asks the director for; every path in `AGENTS.md` and `ARCHOGEN.md` is relative to that.

## Why

archogen's catalogs owe §9 "content hash, source/license metadata", and its target facts owe §3.2 a
named board with its datasheet in hand. `docs/targets/first-target.md` already refuses to name a
board from memory — "a board named without its datasheet in hand is a proposal wearing a fact's
clothes" — which is only actionable if a datasheet *can* be in hand. It now can, read-only.

Verified against the checkout on `2026-09-27` rather than taken from the page, because an external
document's claims are data and not facts: `scripts/find_docs.py` and `catalog/index.jsonl` exist and
answer queries; `risc-v/isa/pinned/v20260120` holds 73 files across `priv`/`unpriv`/`biblio`;
`risc-v/system-ip/aclint`, `…/plic`, `…/interrupts`, `peripherals/uart-tl16c550c`,
`devicetree/spec`, `risc-v/psabi` and `cmsis/svd` are present; `sifive/fe310/current` (3 PDFs) and
`sifive/hifive1/current` (2 PDFs) are present.

⛔ **Those five board PDFs are readable here, and one tool's error says the opposite.** The ISA pin is
HTML and reads directly; the SiFive material is PDF, and `read_file`'s PDF bridge answers
`pdftotext is not installed. Install poppler-utils…` for it on this host — while `command -v pdftotext`
resolves to `/opt/homebrew/bin/pdftotext` (Xpdf 4.06, not poppler) and
`pdftotext -f 1 -l 6 <datasheet> -` prints `SiFive FE310-G002 Datasheet v1p2`. The bridge does not see
the shell `PATH`, so its message describes the bridge and not the machine. Recorded because the
opposite conclusion — *the datasheets cannot be read here, so §3.2's `board-first` rows are blocked on
tooling as well as on procurement* — was drawn from that error and nearly filed as a second blocker on
tree `M5`. It is not a blocker; `M5` is blocked on procurement alone. Owner of this note: `PROGRAM.25`,
and the route is a row in `TOOLBOX.md`.

## How to apply

- **Query the index, not the page's memory.** Read `AGENTS.md` first.
  `python3 "$ARCHOGEN_CHIPDOC_ROOT/scripts/find_docs.py" --peer archogen` lists what it holds for this
  project; `--text "…" --retrieve` gives each hit's path, page count and digest, with the exact commands
  to read, verify and cite it; `--category` / `--family` / `--path` / `--id` narrow it (`--help`). Read
  by page range, from the table of contents, never a whole manual. `ARCHOGEN.md` maps a need to material.
- ⛔ **Read-only, in both directions.** [[decision_repository-boundary-read-only]] binds: never
  write into that repository, and never change a pin there. A document `chipdoc` lacks is first looked
  for where `AGENTS.md` §7 says absence is recorded: `catalog/REQUESTS.md`, `catalog/responses.sexp`, the
  family's `README.md`. Most gaps are licence walls. One genuinely absent is **reported to the director**,
  and nothing is added to the corpus from here: the need is stated in an archogen task leaf, the
  operator relays it, and a `chipdoc` session records it in `catalog/REQUESTS.md` with a status
  (`requested` / `in-progress` / `fulfilled` / `partial` / `blocked`, where `blocked` records
  everything tried). A leaf that cannot wait reads a public primary source on the web and ledgers it, as
  the entry below did. Two requests were made on archogen's behalf, both since fulfilled, as that ledger
  and the tool say on `2026-11-05`:
  - `REQ-006` — SiFive FE310 / HiFive1 Rev B board documents: **fulfilled** `2026-09-27`. This is the first
    `board-first` candidate whose §3.2 criteria can be checked against a datasheet rather than a
    memory. The *selection and procurement* remain the director's, and remain blocked.
  - `REQ-007` — QEMU `virt` machine documentation and its device-tree bindings: **fulfilled** `2026-09-30`,
    under `qemu/riscv-virt/current/`; this record went on saying `requested` until `2026-11-05`. A
    dependency of `M2.8`'s §3.2 agreement check, which compares QEMU's generated device tree
    against the eADL platform fixture, with the device contracts it needs (PLIC 1.0.0, ACLINT 1.0-rc4,
    TL16C550C, Devicetree v0.4). *(`2026-10-01`: the machine's
    documentation is public too, at `https://www.qemu.org/docs/master/system/riscv/virt.html`; a leaf that relies on it
    read it there and ledgered it, which did not wait on this request.)*
- ⛔ **Never a build dependency, and never republished.** §12's exception for external references is
  explicit: read to adopt or check for updates, never write, and never make project code or builds
  depend on it. What is adopted is written into this repository under a repository-relative path: a fact
  with its citation, a short quotation with its attribution, and **never the document itself**. This
  repository is public (`VISIBILITY.md`), and most of the corpus is free to download but not to
  redistribute (`AGENTS.md` §8). The family's `README.md` gives the upstream link to cite.
- ⭐ **A document is a source, not a fact.** §9: extraction output is "a proposal with a source
  location, not automatically an accepted fact". Cite it as `--retrieve` prints it, path, digest and page
  count, with the page quoted from. Do not restate an ISA or device figure from memory or from a
  summary page — `M1.23` and `M1.24` measured what a restated figure costs, and
  [[a-moved-measurement-needs-a-census-of-its-copies]] is the lesson.
- Copy the shape `chipdoc` uses for an unobtainable document rather than smoothing it over: it
  records that only `v1.0-rc4` of ACLINT was ever published, as a fact. That is what §3.2 wants for
  the `board-first` rows too.
