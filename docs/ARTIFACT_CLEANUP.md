# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-06T01:39:13+00:00** (`PROGRAM.130`, twenty-third run) — removed only idle, regenerable incremental compiler caches: the main debug one (19276 files / 2779347408 bytes, 1654 cached bin files, no logs); the wasm one was absent
