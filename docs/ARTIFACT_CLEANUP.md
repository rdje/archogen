# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-05T00:05:20+00:00** (`PROGRAM.126`, twenty-second run) — removed only idle, regenerable incremental compiler caches: the main debug one (30872 files / 3821526631 bytes, 2106 cached bin files, no logs); the wasm one was absent
