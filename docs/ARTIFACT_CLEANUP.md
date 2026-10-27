# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-10-27T10:30:55+00:00** (`PROGRAM.119`, seventeenth run) — removed only idle, regenerable incremental compiler caches: the main and wasm debug ones (52855 files / 5496697254 bytes, 3589 cached bin files, no logs), each checked to hold no link and proven gone; every other entry kept, the handoff census `handoff: OK` first; a light run under doctrine 22, no identity audit of what stayed.
