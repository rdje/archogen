# bedrock — issue index

Every report archogen has made to bedrock, with its current state. This file is the register; each row's sub-tree holds
the issue, its reproducer and its frozen observation.

## Vendor

| Field | Value |
| --- | --- |
| Vendor | bedrock |
| Upstream | `https://github.com/rdje/bedrock.git` |
| Component under test | the portable spine's handoff census, `scripts/check_no_background_jobs.sh` |
| Revision measured | bedrock `835547e` |
| Reported by | archogen — a project made from the template |

## Register

| ID | Title | Kind | Severity | State | Opened | Last verified |
| --- | --- | --- | --- | --- | --- | --- |
| [BR-001](issues/BR-001-handoff-census-instant/) | The handoff census judges one instant | Robustness | Minor | `open` | `2026-10-10` | `2026-10-10` |

## Totals by state

| State | Count | IDs |
| --- | --- | --- |
| `open` | 1 | BR-001 |
| `acknowledged` | 0 | — |
| `by-design` | 0 | — |
| `fixed-upstream` | 0 | — |
| `verified` | 0 | — |
| `withdrawn` | 0 | — |
| `no-action` | 0 | — |

## Totals by severity, open only

"Open" means not yet resolved from archogen's side: `open`, `acknowledged` or `fixed-upstream`.

| Severity | Count |
| --- | --- |
| Blocker | 0 |
| Major | 0 |
| Moderate | 0 |
| Minor | 1 |

## Keeping this index true

This register is a **pointer**; the issue sub-trees are the source. When a state changes, change it in the issue's
`README.md` **and** in this table in the same commit, and add a dated line to that issue's History.
