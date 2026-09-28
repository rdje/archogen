# ARTIFACT_CLEANUP.md

The **latest cleanup only** — this file is overwritten, never appended to.

Sessions do not carry a sense of elapsed time, so this is how a resuming one knows whether a cleanup
is due: **if the date below is more than 24 hours old, or this file does not exist, clean during that
session.** Delete only where deletion is 100% safe — an artifact whose regeneration path is a tracked
command — and investigate anything unexpected instead of removing it. The owning leaf carries the
full inventory, the retained items and their reasons: `PROGRAM.19` in `docs/tasks/PROGRAM.md`.

- **2026-09-28** (`PROGRAM.19`, second run) — released ≈18 MB of regenerable residue and **retained
  2.2 GB on evidence**. Deleted: the test scratch under `target/tmp` (`f28`, `s0-build`, `s0-oracle`,
  `s0-provenance`, `s0-reader`, `s0-build-library.eadl`), each recreated by `cargo test`; and one stale
  `-working` incremental directory left by an interrupted build. `target` 873 MB → 855 MB; residue
  census reports all seven paths `gone`. ⛔ **Two deletions this run did *not* make, both after
  investigation rather than by policy.** `.app-data/pgen-generated-before-remeasure` (18 MB) looked like
  a superseded snapshot, but `LS-004`'s `remeasure.sh` treats an existing backup as a reason to keep
  it — deleting it would silently change a frozen instrument's behaviour. And `target/debug/incremental`
  (614 MB, most of the repository's `.bin` files) is not residue: it holds at most four generations per
  crate, which is cargo's own retention, and deleting it would slow every subsequent build rather than
  remove anything stale. Verified **cold**: `make focused` → `passed — 3 passed, 0 failed, 0
  unavailable` with the scratch it consumes already deleted, which is the `S0.7` lesson applied rather
  than remembered.
