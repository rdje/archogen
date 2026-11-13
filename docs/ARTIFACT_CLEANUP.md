# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-13T21:32:55+00:00** (`PROGRAM.136`, twenty-sixth run) — removed only the idle, regenerable incremental compiler cache: the main debug one (24497 files / 3218773639 bytes, 1632 cached bin files, no logs); the wasm one was absent. Readers' checkouts, every dependency store, product and evidence directory stay. The handoff census was clear, and nothing had the cache open.
