# ARTIFACT_CLEANUP.md

The **latest cleanup only** — this file is overwritten, never appended to.

Sessions do not carry a sense of elapsed time, so this is how a resuming one knows whether a cleanup
is due: **if the date below is more than 24 hours old, or this file does not exist, clean during that
session.** Delete only where deletion is 100% safe — an artifact whose regeneration path is a tracked
command — and investigate anything unexpected instead of removing it. Each run is a leaf of its own,
since `PROGRAM.19`, the owner of the first four, was sealed; the leaf named below carries the full
inventory, the retained items and their reasons, in `docs/tasks/PROGRAM.md` or, once sealed, `docs/task-history/`.

- **2026-10-10** (`PROGRAM.68`, sixth run) — released **≈4.0 GB** (`target` 5.6 GB → 1.6 GB, `.bin` 2 648 → 29): the
  compiler's incremental caches (`debug`, `wasm32`, `riscv64`, Miri's); the closed generated-sources review's loose
  logs and briefs, and the mutation sweep's scratch; uncited probe output of closed leaves, `m3634` among them, which
  its leaf already called removed; and test scratch its owners recreate. `PROGRAM.58`'s leak has not come back (`deps`:
  789 files). Retained on evidence: seven `target/tmp` entries a leaf cites or owns, `m3121` and `m3635`, the trust
  tools' output, the CI tools, Miri's sysroot, the tiers' live products, `target/s0-demo`, `build/` and `.app-data`.
  Residue census: twelve of twelve sampled paths `gone`. Verified **cold**: the provisioner → both tools `already in
  place`, `make focused` → `passed — 3 passed, 0 failed`, `cargo test --all` → **1 304 passed, 0 failed over 97
  suites**, `scripts/check_doctrines.sh` → `=== all doctrines green ===`.
