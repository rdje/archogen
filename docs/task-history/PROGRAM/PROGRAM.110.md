- ID: `PROGRAM.110`
  Status: `done` — the director's verification doctrine 22 (`2026-10-26`) recorded: small checks per slice, full
    verification after the feature set
  Built: `docs/decisions/decision_small-checks-per-slice.md`, the doctrine word for word and what it means here — the
    changed crate's tests and clippy and, for a slice changing what a root compiles, the trust inventory; the hooks and
    `make focused` kept as commit integrity; the full xtask suite, the proofs and their pins, workspace clippy and CI
    deferred — indexed; `M3.5.9`, `M3.5`'s deferred verification, filed pending after its feature leaves.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the doctrine existed only in the session; WHERE, `git grep -n "doctrine 22" -- docs`: no line before this commit.
    - [x] FIX — the record, its index row, `M3.5.9`.
    - [x] ADDRESSED — `bash scripts/check_doctrines.sh`: green, the record indexed and inside the folder's ceiling.
    - [x] NO REGRESSION — a records-only change; no code changed.
    - [x] LOCKSTEP — the decisions index and the tree.
  Verification: the doctrines.
  Commit: `ARCHOGEN-PROGRAM-0952 (leaf PROGRAM.110)`
