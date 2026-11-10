# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-10T01:06:41+00:00** (`PROGRAM.133`, twenty-fifth run) — removed only the idle, regenerable incremental compiler cache: the main debug one (17310 files / 2194445572 bytes, 1152 cached bin files, no logs); the wasm one was absent. Readers' checkouts, every dependency store, product and evidence directory stay. The handoff census was clear, and nothing had the cache open.
