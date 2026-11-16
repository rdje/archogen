# ARTIFACT_CLEANUP.md

Latest cleanup only, overwritten after each run. If this date is more than 24 hours old, clean during the session.
The owning task leaf records the full inventory, retained items, regeneration paths and verification.

- **2026-11-16T01:20:56+00:00** (`PROGRAM.148`, twenty-ninth run) — removed 16.5 GB of regenerable, idle output, as `PROGRAM.145` did: 8,755 directories, the nested cargo target directories in leaves' and readers' folders (5.2 GB) and the test scratch in `target/tmp` and `target/trust-tests` (11.3 GB); `target/` 36.9 GB to 22.9 GB. The main build, the stores, the guard's state and every report, brief, rig and log stay. The handoff census was clear, no reader was at work, and nothing removed was open.
