# LinkedSpec — bug index

Every defect archogen has reported to LinkedSpec, with its current state. This file is the
register; each row's sub-tree holds the issue, its inputs, its reproducer and its frozen
observation.

## Vendor

| Field | Value |
| --- | --- |
| Vendor | LinkedSpec |
| Upstream | `https://github.com/rdje/linkedspec` |
| Component under test | Rust backend (`linkedspec-runtime`), the shipped `specs/Lispish.spec` and, since the `2026-09-27` re-measurements, `specs/SExprDocumentV1.spec` with the example's two native adapters `lispish_file` and `sexpr_file` |
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

⚠️ That paragraph records the state **when the notice arrived** and is superseded by the two below:
adoption landed the same day, and all five reports were then re-measured by archogen at the adopted
pin. Nothing is awaiting measurement now.

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
Adoption is **not** acceptance: moving the pin changed no state by itself. Re-measurement was owned
by leaf `M1.20` — the only thing that may move a row to `verified` — and ran one report per sub-leaf.
All five (`LS-005`, `LS-001`, `LS-004`, `LS-002`, `LS-003`) were re-measured at this pin on
`2026-09-27` and are `verified`.

## Register

| ID | Title | Kind | Severity | State | Opened | Last verified |
| --- | --- | --- | --- | --- | --- | --- |
| [LS-001](issues/LS-001-cargo-workspace-collision/) | Documented vendoring layout does not build inside a Cargo workspace | Build | Blocker | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-002](issues/LS-002-multi-form-truncation/) | A multi-form file yields only its first form, exit `0` | Correctness | Blocker | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-003](issues/LS-003-token-kind-erasure/) | Quoted string, bare symbol and number are indistinguishable | Correctness | Major | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-004](issues/LS-004-bootstrap-false-success/) | PGEN bootstrap continues past a failed `cargo` and reports a false seed | Robustness | Moderate | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-005](issues/LS-005-guide-ordering/) | "Add and pin" commands alone never produce a buildable tree | Docs | Minor | `verified` | `2026-09-20` | `2026-09-27` |
| [LS-006](issues/LS-006-hex-underscore-withdrawn/) | Hex literal underscores corrupted by atom joining | Correctness | — | `withdrawn` | `2026-09-20` | `2026-09-20` |
| [LS-007](issues/LS-007-adjacent-fragment-join/) | Adjacent fragments join into a single atom | Correctness | Informational | `no-action` | `2026-09-20` | `2026-09-20` |
| [LS-008](issues/LS-008-preparation-log-volume/) | One successful preparation writes a 784 MB log, and the guide says to keep it | Cost | Minor | `open` | `2026-09-30` | `2026-09-30` |

## Totals by state

| State | Count | IDs |
| --- | --- | --- |
| `open` | 1 | LS-008 |
| `acknowledged` | 0 | — |
| `by-design` | 0 | — |
| `fixed-upstream` | 0 | — |
| `verified` | 5 | LS-001, LS-002, LS-003, LS-004, LS-005 |
| `withdrawn` | 1 | LS-006 |
| `no-action` | 1 | LS-007 |

**Every reported defect has been re-measured by archogen at the adopted pin.** Nothing in this
register rests on the vendor's word any more: all five reports were re-run on `2026-09-27` at
`2ac834913`, each through an instrument carrying RED arms built from the original observation, and
each with its output frozen in that issue's `evidence/REMEASURED.txt`. Both reported blockers are
closed.

Three standing facts a consumer of this pin needs, each recorded where it cannot be missed:

- `LS-001`'s remedy is **split**. The vendored example carries its own `[workspace]` boundary, and
  the nested PGEN manifest resolves only once the consuming workspace root carries the documented
  `exclude = ["vendor/linkedspec"]` — which this repository does, so the exclusion is a standing
  requirement, not a one-off fix.
- `LS-002`'s and `LS-003`'s remedy is a **new route**, not a change to the old one. `Lispish.spec`
  keeps its extraction behaviour by design — one form per file, kinds erased — so a consumer that
  needs every form or the token kinds must select `SExprDocumentV1.spec`; changing only the old
  adapter's grammar argument does not adopt the new contract, and fails with `entry_rule_not_found`.
  Both old-route behaviours are now **guarded** rather than merely reported: each issue re-runs its
  frozen reproducer and requires it to match, so a silent change there is caught.
- `LS-004`'s preparation must be **regenerated** after a pin move. `make bootstrap` is idempotent on
  existence, so a checkout carrying the previous pin's parser exits `0` having done nothing.

⚠️ **One finding is filed as its own row, `LS-008`, and it is not part of any verdict above.**
Re-measuring `LS-004` measured the cost of the published preparation interface as well as its
correctness: one successful run wrote a 784 MB log of 4 008 986 lines, while the guide instructs a
consumer to preserve the full log when the command fails. It was measured again on `2026-09-30` at the
same revisions, with the same figures, and the proposed remedy is LinkedSpec's to choose.

## Totals by severity, open only

"Open" means not yet resolved from archogen's side: `open`, `acknowledged` or `fixed-upstream`.

| Severity | Count |
| --- | --- |
| Blocker | 0 |
| Major | 0 |
| Moderate | 0 |
| Minor | 1 |

## Reproducibility

Every row has a runnable reproducer whose exit code is its verdict, and each was executed on the
revision above. No row rests on reading upstream documentation.

A row that has been **re-measured** also carries a `remeasure.sh`, whose exit code answers the
opposite question — *is the reported defect gone at this revision?* — and whose `--self-test`
proves the instrument still reports the original revision as defective. A frozen reproducer cannot
do that job: it reports *change*, and a renamed heading is a change too. All five re-measured rows
carry one: `LS-001`, `LS-002`, `LS-003`, `LS-004`, `LS-005`.

Each issue directory is **self-contained**: it carries its own setup instructions, inputs,
reproducer and frozen observation, and references nothing outside itself. One row can be handed
over on its own.

| Instrument needs | IDs |
| --- | --- |
| A LinkedSpec checkout only | LS-001, LS-004, LS-005 (`repro.sh`), LS-001 and LS-005 (`remeasure.sh`) |
| A captured preparation log, or a prepared checkout and room for most of a gigabyte of log | LS-008 (`repro.sh --log` or `--run`) |
| A prepared checkout, and network for the fresh-preparation arm | LS-004 (`remeasure.sh`) |
| A built `lispish_file` (each issue's own `SETUP.md` describes the build) | LS-002, LS-003, LS-006, LS-007 (`repro.sh`) |
| Both built consumers, and `python3` to read the tagged result | LS-002 and LS-003 (`remeasure.sh`) |

## Keeping this index true

This register is a **pointer**; the issue sub-trees are the source. When a state changes, change
it in the issue's `README.md` **and** in this table in the same commit, and add a dated line to
that issue's History. A stale index is worse than none: it is the file a returning reader trusts
first.
