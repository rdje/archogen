# `semulith` — a sibling repository that names archogen as its consumer

- **Type:** `reference`
- **Date:** `2026-09-29`
- **Status:** `active`
- **Owner / source:** leaf `PROGRAM.25`. Filed because the fact was raised in conversation and recorded
  nowhere in the tracked tree, which §9 does not allow: a finding that lives only in a reply dies with
  the session.

⛔ **This record states that the project exists and where the seam is. It is deliberately not an
analysis of it** — that was offered and declined, and the decline is respected here. Read
`semulith`'s own documents for its state; this file is a pointer, and a pointer to another repository
goes stale the moment that repository moves.

## What it is

`../semulith` — a **sibling of this repository's parent directory**, so it moves with the checkout
rather than being pinned to one filesystem (§12: a repository may be moved, including to another
volume, and nothing may break). It is a separate git repository on the same discipline spine as this
one (`bedrock`), building **CPU and DSP software models in Rust** and composing validated processor
profiles into boards and complete computers. Its own `README.md` and `ROADMAP.md` are the authority;
nothing here restates their figures.

⛔ **Read-only, in both directions.** [[decision_repository-boundary-read-only]] binds exactly as it
does for `chipdoc`: read freely, never write, never change a pin there. The seam is operator-relayed —
anything archogen needs to tell `semulith` goes into an archogen tracked file and the director relays
it, which is the route `chipdoc`'s `catalog/REQUESTS.md` established.

## Why archogen's tree needs to know

`semulith` names archogen as a **concrete consumer**, and has written down its side of the seam:

- `docs/ARCHOGEN_INTEGRATION.md` — an ownership table for the facts the two projects share (who owns
  hardware/OS feature descriptions, who owns modelled CPU and device behaviour, who owns the selected
  addresses and wiring for a generated platform), a platform-contract content list, and an
  `ARCHOGEN-OS` gate.
- `docs/tasks/AG-OS.md` — its integration tree, whose leaves wait on archogen's *real* typed eADL
  interface rather than a promised one.

The census that makes the asymmetry checkable, both directions:

```text
$ git grep -il "semulith" -- ':!vendor' | wc -l
0        ← archogen's own tracked files named it nowhere before this record
$ grep -ril "semulith" vendor/ | wc -l
40       ← the vendored LinkedSpec submodule names it, as a fellow consumer
```

⛔ **The census trap this records, because it nearly produced a false claim:** `vendor/linkedspec` is a
**submodule** (`.gitmodules`), so `git grep` skips it entirely. A `git grep` over the whole repository
returns nothing at all and would have supported "the two projects have never met". They have — through
the vendor's issue ledger, where `semulith` filed and independently closed its own LinkedSpec reports
while archogen was verifying its five (`M1.20`). A census that cannot see part of the population is
worse than no census, and `-- ':!vendor'` plus a filesystem walk of `vendor/` is what makes this one
whole.

## One boundary, quoted rather than argued

`semulith`'s own integration document says: *"Semulith does not become independent merely because it is
a separate project"*, and tells archogen to keep the QEMU and physical-board paths and to record shared
models in the F30 trust inventory. That agrees with
[[decision_emulator-independence-retained]] and with
`docs/knowledge/an-oracle-is-independent-by-construction.md`, so the two projects hold the same
principle about each other. It is quoted here because it is the one fact a future archogen leaf would
most need and least expect to find already written down on the other side of a read-only boundary.

## Which archogen leaves would need this

Named so the pointer is findable from the work, not only from the index — none of them is changed by
this record, and none owes it anything yet:

- **`M2.8`** (`riscv-virt-up`, the §3.2 agreement check) and **`M4`** (generated system + simulator) —
  the leaves that decide what a platform description contains and what executes it.
- **`API`** — its wasm binding and `semulith`'s browser-first target are the same destination reached
  from two directions.
- **`M5`** — a software model is not physical execution evidence, and `M5` stays blocked on
  procurement regardless of what any model can do.

## Related

[[decision_repository-boundary-read-only]] — the rule that makes this a pointer and not a dependency ·
[[reference_external-document-source-chipdoc]] — the same seam shape, for documents rather than models ·
[[decision_emulator-independence-retained]] — why a sibling project is not an independent oracle
