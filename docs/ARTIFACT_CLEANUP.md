# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-10-28T10:37:12+00:00** (`PROGRAM.120`, eighteenth run) — removed only idle, regenerable incremental compiler caches: the main and wasm debug ones (15647 files / 1926076171 bytes, 1068 cached bin files, no logs)
