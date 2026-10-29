# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-10-29T11:11:25+00:00** (`PROGRAM.121`, nineteenth run) — removed only idle, regenerable incremental compiler caches: the main debug one (11835 files / 1722517361 bytes, 813 cached bin files, no logs); the wasm one was absent
