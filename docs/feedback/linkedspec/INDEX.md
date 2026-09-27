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
| Toolchain | rustc 1.95.0, Darwin arm64 |
| Reported by | archogen — first consumer of the Rust backend |
| Vendored at | `vendor/linkedspec` (development-time evaluation only; no crate depends on it) |

## Upstream response — 2026-09-27

LinkedSpec published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8` with the LS-004 adoption and all previously named
remedies. The five addressed requirements are `fixed-upstream`, awaiting ARCHOGEN adoption
and measurement; none is marked `verified` by this notice. Original measured revisions and
last-verified dates below are unchanged. See the [LS-004 completion notice](issues/LS-004-bootstrap-false-success/UPSTREAM.md)
and each issue's dated response. LS-006 stays withdrawn; LS-007 stays no-action.

## Register

| ID | Title | Kind | Severity | State | Opened | Last verified |
| --- | --- | --- | --- | --- | --- | --- |
| [LS-001](issues/LS-001-cargo-workspace-collision/) | Documented vendoring layout does not build inside a Cargo workspace | Build | Blocker | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-002](issues/LS-002-multi-form-truncation/) | A multi-form file yields only its first form, exit `0` | Correctness | Blocker | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-003](issues/LS-003-token-kind-erasure/) | Quoted string, bare symbol and number are indistinguishable | Correctness | Major | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-004](issues/LS-004-bootstrap-false-success/) | PGEN bootstrap continues past a failed `cargo` and reports a false seed | Robustness | Moderate | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-005](issues/LS-005-guide-ordering/) | "Add and pin" commands alone never produce a buildable tree | Docs | Minor | `fixed-upstream` | `2026-09-20` | `2026-09-20` |
| [LS-006](issues/LS-006-hex-underscore-withdrawn/) | Hex literal underscores corrupted by atom joining | Correctness | — | `withdrawn` | `2026-09-20` | `2026-09-20` |
| [LS-007](issues/LS-007-adjacent-fragment-join/) | Adjacent fragments join into a single atom | Correctness | Informational | `no-action` | `2026-09-20` | `2026-09-20` |

## Totals by state

| State | Count | IDs |
| --- | --- | --- |
| `open` | 0 | — |
| `acknowledged` | 0 | — |
| `by-design` | 0 | — |
| `fixed-upstream` | 5 | LS-001, LS-002, LS-003, LS-004, LS-005 |
| `verified` | 0 | — |
| `withdrawn` | 1 | LS-006 |
| `no-action` | 1 | LS-007 |

**Consumer verification pending for two reported blockers:** `LS-001` (build) and `LS-002`
(correctness). Both are fixed-upstream; ARCHOGEN has not yet accepted its own rerun.

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
