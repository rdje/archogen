# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-15T00:08:35+00:00** (`PROGRAM.145`, twenty-eighth run) — removed 29.3 GB of regenerable, idle output, as `PROGRAM.138` did: 7,473 directories, the nested cargo target directories in leaves' and readers' folders (19.7 GB) and the test scratch in `target/tmp` and `target/trust-tests` (9.7 GB); `target/` 46.1 GB to 20.2 GB. The main build, the stores, the guard's state and every report, brief, rig and log stay. The handoff census was clear, no reader was at work, and nothing removed was open.
