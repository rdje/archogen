# LinkedSpec — bug index

Every defect archogen has reported to LinkedSpec, with its current state. This file is the
register; each row's sub-tree holds the issue, its inputs, its reproducer and its frozen
observation.

## Vendor

| Field | Value |
| --- | --- |
| Vendor | LinkedSpec |
| Upstream | `https://github.com/rdje/linkedspec` |
| Component under test | Rust backend (`linkedspec-runtime`) and the shipped `specs/Lispish.spec` |
| Revision measured | LinkedSpec `ad290bdb4`, RGX `8763a0e6`, PGEN `db6f8c68` |
| Revision adopted | LinkedSpec `2ac834913d85c32f532be9b0aab63644838a577a` — the **latest published head** — with RGX `f6e5acdc99720349d1e3ecef9f821f365c4db19c`. Pinned `2026-09-27` by leaves `M1.19` → `M1.19.1`, and measured **documentation-only** ahead of the notice's `fd3e328d5` (no `.rs`, `Cargo.*` or `specs/` path differs), so the fixes under test are the same code. The original observations below were taken at the **measured** revision; a row re-measured at this one says so in its own sub-tree |
| Toolchain | rustc 1.95.0, Darwin arm64 |
| Reported by | archogen — first consumer of the Rust backend |
| Vendored at | `vendor/linkedspec` (development-time evaluation only; no crate depends on it) |
| Consumer workspace | this repository's root manifest carries `exclude = ["vendor/linkedspec"]`, adopted `2026-09-27` — the vendor's documented requirement of a workspace consumer before any metadata or build command, and load-bearing for the nested PGEN manifest (see `LS-001`) |

## Upstream response — 2026-09-27

LinkedSpec published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8` with the LS-004 adoption and all previously named
remedies. The five addressed requirements are `fixed-upstream`, awaiting ARCHOGEN adoption
and measurement; none is marked `verified` by this notice. Original measured revisions and
last-verified dates below are unchanged. See the [LS-004 completion notice](issues/LS-004-bootstrap-false-success/UPSTREAM.md)
and each issue's dated response. LS-006 stays withdrawn; LS-007 stays no-action.

**Adopted `2026-09-27`** (leaves `M1.19` → `M1.19.1`): `vendor/linkedspec` is pinned at LinkedSpec's
**latest published head** `2ac834913`, with its nested RGX submodule at `f6e5acdc9`, through the
vendor's published adoption route. The notice named `fd3e328d5`; the director ruled that the latest
pushed work should be pinned, and the delta between the two was measured path by path before moving
— nine root documents and sixteen files under `docs/`, with **no** `.rs`, `Cargo.*` or `specs/` path
among them. The code under test is therefore identical at both revisions, and `fd3e328d5` remains an
ancestor, so the evidence the notice cites still resolves.
⚠️ One consequence: the pinned Rust integration guide
(`docs/linkedspec-book/src/public-api/integration-rust.md`) *is* among the changed documents, so
`M1.20` must follow the guide **at `2ac834913`**, not the one the notice links at `fd3e328d5`.
Adoption is **not** acceptance: moving the pin changed no state by itself. Re-measurement is owned
by leaf `M1.20` — the only thing that may move a row to `verified` — and runs one report per
sub-leaf. `LS-005` and `LS-001` were re-measured at this pin on `2026-09-27` and are now
`verified`; `LS-002`, `LS-003` and `LS-004` stay `fixed-upstream` until their own reruns land.

## Register

| ID | Title | Kind | Severity | State | Opened | Last verified |
| --- | --- | --- | --- | --- | --- | --- |
| [LS-001](issues/LS-001-cargo-workspace-collision/) | Documented vendoring layout does not build inside a Cargo workspace | Build | Blocker | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-002](issues/LS-002-multi-form-truncation/) | A multi-form file yields only its first form, exit `0` | Correctness | Blocker | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-003](issues/LS-003-token-kind-erasure/) | Quoted string, bare symbol and number are indistinguishable | Correctness | Major | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-004](issues/LS-004-bootstrap-false-success/) | PGEN bootstrap continues past a failed `cargo` and reports a false seed | Robustness | Moderate | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-005](issues/LS-005-guide-ordering/) | "Add and pin" commands alone never produce a buildable tree | Docs | Minor | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-006](issues/LS-006-hex-underscore-withdrawn/) | Hex literal underscores corrupted by atom joining | Correctness | — | `withdrawn` | `2026-09-20` | `2026-09-20` |
| [LS-007](issues/LS-007-adjacent-fragment-join/) | Adjacent fragments join into a single atom | Correctness | Informational | `no-action` | `2026-09-20` | `2026-09-20` |

## Totals by state

| State | Count | IDs |
| --- | --- | --- |
| `open` | 0 | — |
| `acknowledged` | 0 | — |
| `by-design` | 0 | — |
| `fixed-upstream` | 3 | LS-002, LS-003, LS-004 |
| `verified` | 2 | LS-001, LS-005 |
| `withdrawn` | 1 | LS-006 |
| `no-action` | 1 | LS-007 |

**Consumer verification pending for one reported blocker:** `LS-002` (correctness — a multi-form
file yields only its first form). `LS-001`, the other reported blocker, was re-measured at the
adopted pin on `2026-09-27` and is `verified`: the integration example now carries its own
`[workspace]` boundary, and the nested PGEN manifest resolves once the consuming workspace root
carries the documented `exclude = ["vendor/linkedspec"]` — which this repository now does, so the
exclusion is a standing consumer requirement, not a one-off fix. `LS-003` and `LS-004` are still
awaiting their own reruns.

## Totals by severity, open only

| Severity | Count |
| --- | --- |
| Blocker | 0 |
| Major | 0 |
| Moderate | 0 |
| Minor | 0 |

## Reproducibility

Every row has a runnable reproducer whose exit code is its verdict, and every one of the seven
was executed on the revision above. No row rests on reading upstream documentation.

A row that has been **re-measured** also carries a `remeasure.sh`, whose exit code answers the
opposite question — *is the reported defect gone at this revision?* — and whose `--self-test`
proves the instrument still reports the original revision as defective. A frozen reproducer cannot
do that job: it reports *change*, and a renamed heading is a change too. Rows with one today:
`LS-001`, `LS-005`.

Each issue directory is **self-contained**: it carries its own setup instructions, inputs,
reproducer and frozen observation, and references nothing outside itself. One row can be handed
over on its own.

| Reproducer needs | IDs |
| --- | --- |
| A LinkedSpec checkout only | LS-001, LS-004, LS-005 |
| A built `lispish_file` (each issue's own `SETUP.md` describes the build) | LS-002, LS-003, LS-006, LS-007 |

## Keeping this index true

This register is a **pointer**; the issue sub-trees are the source. When a state changes, change
it in the issue's `README.md` **and** in this table in the same commit, and add a dated line to
that issue's History. A stale index is worse than none: it is the file a returning reader trusts
first.
