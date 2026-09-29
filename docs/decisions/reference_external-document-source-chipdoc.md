# The external document source: `chipdoc`, read-only, mapped by its own `ARCHOGEN.md`

- **Type:** `reference`
- **Date:** `2026-09-27`
- **Status:** `active`
- **Owner / source:** director directive, `2026-09-27` — "if you ever need additional ISA / RISC-V /
  … just shout"; the mapping file is maintained on the `chipdoc` side and updated as material that
  would interest archogen is added
- **External sources:** [chipdoc](../book/src/ledger.md#chipdoc) · [QEMU](../book/src/ledger.md#qemu) — version, scope and limits in the ledger

## The fact

A separate repository, **`chipdoc`**, curates primary-source chip, ISA, interface and protocol
documentation, and maintains **`ARCHOGEN.md` at its root**: a one-way map from archogen's document
needs to the material it holds. Its checkout location is deliberately **not recorded here** — §12
forbids checkout-specific absolute paths in tracked files, and §13 forbids project data living
off-volume. Supply it at runtime as `ARCHOGEN_CHIPDOC_ROOT`; every path in `ARCHOGEN.md` is relative
to that.

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

- **Query the index, not the page's memory.**
  `python3 "$ARCHOGEN_CHIPDOC_ROOT/scripts/find_docs.py" --help`, then `--text` / `--path` /
  `--category` / `--family` / `--id`. Read `ARCHOGEN.md` first for the need → material mapping; read
  `catalog/index.jsonl` through the tool for what is actually there today.
- ⛔ **Read-only, in both directions.** [[decision_repository-boundary-read-only]] binds: never
  write into that repository, and never change a pin there. A document archogen needs and `chipdoc`
  lacks is **requested**, not fetched around: the need is stated in an archogen task leaf, the
  operator relays it, and a `chipdoc` session records it in `catalog/REQUESTS.md` with a status
  (`requested` / `in-progress` / `fulfilled` / `partial` / `blocked`, where `blocked` records
  everything tried). Two requests already exist on archogen's behalf, both measured in that ledger
  on `2026-09-27`:
  - `REQ-006` — SiFive FE310 / HiFive1 Rev B board documents: **fulfilled**. This is the first
    `board-first` candidate whose §3.2 criteria can be checked against a datasheet rather than a
    memory. The *selection and procurement* remain the director's, and remain blocked.
  - `REQ-007` — QEMU `virt` machine documentation and its device-tree bindings: **requested**. A
    dependency of `M2.8`'s §3.2 agreement check, which must compare QEMU's generated device tree
    against the eADL platform fixture. The device contracts it needs (PLIC 1.0.0, ACLINT 1.0-rc4,
    TL16C550C, Devicetree v0.4) are held; the machine's own bindings are not.
- ⛔ **Never a build dependency.** §12's exception for external references is explicit: read to
  adopt or check for updates, never write, and never make project code or builds depend on it.
  Anything adopted is **copied into this repository** under a repository-relative path, the way
  `docs/CLAIM_VERIFICATION.md` was.
- ⭐ **A document is a source, not a fact.** §9: extraction output is "a proposal with a source
  location, not automatically an accepted fact". Cite the exact revision and the digest from the
  family's `SHA256SUMS` or manifest. Do not restate an ISA or device figure from memory or from a
  summary page — `M1.23` and `M1.24` measured what a restated figure costs, and
  [[a-moved-measurement-needs-a-census-of-its-copies]] is the lesson.
- Copy the shape `chipdoc` uses for an unobtainable document rather than smoothing it over: it
  records that only `v1.0-rc4` of ACLINT was ever published, as a fact. That is what §3.2 wants for
  the `board-first` rows too.
