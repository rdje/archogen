# ARTIFACT_CLEANUP.md

The **latest cleanup only** — this file is overwritten, never appended to.

Sessions do not carry a sense of elapsed time, so this is how a resuming one knows whether a cleanup
is due: **if the date below is more than 24 hours old, or this file does not exist, clean during that
session.** Delete only where deletion is 100% safe — an artifact whose regeneration path is a tracked
command — and investigate anything unexpected instead of removing it. The owning leaf carries the
full inventory, the retained items and their reasons: `PROGRAM.19` in `docs/tasks/PROGRAM.md`.

- **2026-09-27** (`PROGRAM.19`) — released ≈1.4 GB of regenerable artifacts: the previous pin's
  LinkedSpec build (`.app-data/target`, 1.3 GB), a digest-verified duplicate of the checkout's
  generated parser (`.app-data/pgen-generated-2ac834913`, 70 MB), two empty cargo stores, the
  reference-check scratch, a prior session's notice scratch, and `target/tmp` (17 MB of test scratch
  carrying most of 703 stale incremental `.bin` files). Nothing tracked was touched. ⛔ The cleanup's
  own verification found a real defect rather than passing: one S0 test wrote into
  `CARGO_TARGET_TMPDIR` without creating it, so it failed on a cold run and passed on a warm one —
  fixed and verified cold in leaf `S0.7` before this record was written.
