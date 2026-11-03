# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-03T21:42:14+00:00** (`PROGRAM.123`, twenty-first run) — removed only idle, regenerable incremental compiler caches: the main debug one (11323 files / 1586327844 bytes, 780 cached bin files, no logs); the wasm one was absent
