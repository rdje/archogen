# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-02T07:36:05+00:00** (`PROGRAM.122`, twentieth run) — removed only idle, regenerable incremental compiler caches: the main debug one (13286 files / 1934882863 bytes, 912 cached bin files, no logs); the wasm one was absent
