# ARTIFACT_CLEANUP.md

The **latest cleanup only** — this file is overwritten, never appended to.

Sessions do not carry a sense of elapsed time, so this is how a resuming one knows whether a cleanup
is due: **if the date below is more than 24 hours old, or this file does not exist, clean during that
session.** Delete only where deletion is 100% safe — an artifact whose regeneration path is a tracked
command — and investigate anything unexpected instead of removing it. The owning leaf carries the
full inventory, the retained items and their reasons: `PROGRAM.19` in `docs/tasks/PROGRAM.md`.

- **2026-09-30** (`PROGRAM.19`, fourth run) — released **≈2.2 GB** (`target` 6.3 GB → 4.1 GB): the QEMU build
  tree the CI provisioner leaves after installing the pinned emulator (`target/ci/build`, 1.7 GB), the CI
  rehearsal's checkout (`target/ci/rehearsal`, 517 MB, wiped by its script on every run), `target/doctrine_scratch`
  (112 MB) and all of `target/tmp` but `m129`, which `M1.29` still owns. Retained on evidence: the installed CI
  tools and their verified tarball, cargo's and Miri's caches (no crate above four incremental generations), the
  tiers' live build products, `target/s0-demo`, `build/` and `.app-data`. Residue census: ten of ten sampled paths
  `gone`. Verified **cold**: the provisioner → both tools `already in place`, `make focused` → `passed — 3 passed,
  0 failed`, `cargo test --all` → **742 passed, 0 failed over 62 suites**, `scripts/check_doctrines.sh` → `=== all
  doctrines green ===`.
