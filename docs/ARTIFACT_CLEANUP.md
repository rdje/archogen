# ARTIFACT_CLEANUP.md

The **latest cleanup only** — this file is overwritten, never appended to.

Sessions do not carry a sense of elapsed time, so this is how a resuming one knows whether a cleanup
is due: **if the date below is more than 24 hours old, or this file does not exist, clean during that
session.** Delete only where deletion is 100% safe — an artifact whose regeneration path is a tracked
command — and investigate anything unexpected instead of removing it. The owning leaf carries the
full inventory, the retained items and their reasons: `PROGRAM.19` in `docs/tasks/PROGRAM.md`.

- **2026-09-29** (`PROGRAM.19`, third run) — released ≈11 MB of regenerable residue and **retained
  2.2 GB on evidence**, the same two items as the previous run plus two new ones. Deleted: the twelve
  scratch directories under `target/tmp` (`f28`, `m112`, `m1125`, `m113`, `m1132`, `m1134`, `p21`,
  `s0-build`, `s0-build-library.eadl`, `s0-oracle`, `s0-provenance`, `s0-reader`), six of which
  `cargo test` recreated during the cold verification; `target/doctrine_scratch`, which the doctrine
  gate recreates on every commit; and `target/sync-backup-2026-09-21`, which was **identified before
  it was deleted** — all four files are byte-identical to `bedrock`'s `HEAD` copies of the same neutral
  spine files, so the backup duplicated a readable source rather than preserving anything. `target`
  957 MB → 949 MB; the residue census reports all five sampled paths `gone`. ⛔ **Two retentions this
  run are new and both are investigations, not policy.** `build/riscv-virt.dtb` and `.dts` (16 KB) stay:
  their regeneration path is `scripts/target_emulator.sh --dump-dtb`, which needs the pinned emulator,
  and `M2.8.2` is the leaf that compares a fixture against them. `target/s0-demo/base` (20 KB) stays:
  a closed leaf cites it as verification evidence (`docs/tasks/S0.md:181`) and recreating it is a full
  `archogen build`, not a `cargo test`. Verified **cold**: `make focused` → `passed — 3 passed,
  0 failed, 0 unavailable`, `cargo test --all` → **492 passed, 0 failed**, `scripts/check_doctrines.sh`
  → `=== all doctrines green ===`, with the scratch all three consume already deleted.
