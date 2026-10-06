# ARTIFACT_CLEANUP.md

The **latest cleanup only** — this file is overwritten, never appended to.

Sessions do not carry a sense of elapsed time, so this is how a resuming one knows whether a cleanup
is due: **if the date below is more than 24 hours old, or this file does not exist, clean during that
session.** Delete only where deletion is 100% safe — an artifact whose regeneration path is a tracked
command — and investigate anything unexpected instead of removing it. Each run is a leaf of its own,
since `PROGRAM.19`, the owner of the first four, was sealed; the leaf named below carries the full
inventory, the retained items and their reasons, in `docs/tasks/PROGRAM.md` or, once sealed, `docs/task-history/`.

- **2026-10-06** (`PROGRAM.57`, fifth run) — released **≈9.4 GB** (`target` 11 GB → 1.6 GB): all of `target/debug`,
  whose `deps` held 1 454 953 object files that a macOS build leaves behind and never deletes (a defect, owned by
  `PROGRAM.58`); the output of the trust design's review rounds 8–11; `API`'s closing-review scratch; and the test
  scratch and uncited probe output of closed leaves in `target/tmp`. Retained on evidence: six `target/tmp` entries
  whose mutation scripts or scratch a `done` leaf cites, `m129` (an open leaf), the CI tools, Miri's cache, the tiers'
  live products, `target/s0-demo`, `build/` and `.app-data`. Residue census: twelve of twelve sampled paths `gone`.
  Verified **cold**: the provisioner → both tools `already in place`, `make focused` → `passed — 3 passed, 0 failed`,
  `cargo test --all` → **1 218 passed, 0 failed over 91 suites**, `scripts/check_doctrines.sh` → `=== all doctrines
  green ===`.
