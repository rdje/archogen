# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-07T03:24:12+00:00** (`PROGRAM.132`, twenty-fourth run) — removed only idle, regenerable incremental compiler caches: the main debug one (26128 files / 3321402601 bytes, 1752 cached bin files, no logs); the wasm one was absent. Readers' checkouts, every dependency store, product and evidence directory stay. The census named a reader's rig, so idleness was shown of the cache itself: nothing had it open.
