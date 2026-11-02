- ID: `PROGRAM.112`
  Status: `done` — doctrine 22's record corrected: what `make focused` runs is not deferred
  Built: `docs/decisions/decision_small-checks-per-slice.md`'s deferred bullet split in two — what `make focused` runs,
    its tier `cargo fmt --all -- --check`, workspace clippy and `cargo test --all`, stays at every commit, a pin a slice
    moves re-pinned by that slice; what it does not run — the integration tier and CI, the sweeps themselves, the
    oracle, audits — deferred to `M3.5.9`; its index row likewise.
  Acceptance:
    - [x] ROOT CAUSE — WHY: the record listed the whole xtask suite, the sweep's pinned premises and workspace clippy as deferred, and `make focused`, mandatory at every commit, runs all three; WHERE, `git show d8153b33:docs/decisions/decision_small-checks-per-slice.md`, its *Deferred* bullet, against `xtask/src/main.rs`'s `focused` tier, `steps: &[FMT, CLIPPY, TESTS]` — found when `M3.5.4.5`'s focused run refused the lockfile pin the record had deferred.
    - [x] FIX — the record's two bullets and its index row.
    - [x] ADDRESSED — `bash scripts/check_doctrines.sh` through the hooks: the record indexed and within the folder's ceiling.
    - [x] NO REGRESSION — a records-only change; no code changed.
    - [x] LOCKSTEP — the record, its index and the tree.
  Verification: the doctrines.
  Commit: `ARCHOGEN-PROGRAM-0959 (leaf PROGRAM.112)`
