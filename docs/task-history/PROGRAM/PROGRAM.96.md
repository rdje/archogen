- ID: `PROGRAM.96`
  Status: `done` — thirteenth daily artifact cleanup; the record's last run `2026-10-20T00:58:20+00:00`, over 24 h old
  Goal: remove only idle, regenerable compiler caches — the main and owned native-sweep incremental directories —
    after a physical and native inventory, a successful handoff census and an identity recheck; retain every
    dependency store, final product, cited evidence directory and Git workspace; prove each removed directory gone,
    every other file under `target/` identity-equal, the tracked inputs restored and the focused tier regenerating;
    overwrite `docs/ARTIFACT_CLEANUP.md` and add the run's section to the repository annex. The commit also seals
    what closed at `HEAD`: `M3`'s `.6.1` subtree and `PROGRAM.95`.
    [Full evidence and adapters](../book/src/annex-repository.md#daily-artifact-cleanup-on-2026-10-21).
  Acceptance:
  - [x] ROOT CAUSE: `python3 -I -B target/artifact-cleanup/2026-10-21/census.py`, exit code 0: five locations,
    physical and native find/stat inventories agree; no link, nested Git boundary or cross-volume entry; the handoff
    census `handoff: OK` before the inventory and again immediately before the deletion.
  - [x] ADDRESSED: `python3 -I -B target/artifact-cleanup/2026-10-21/audit.py`, exit code 0: only 2849 idle cache
    files / 1017454565 logical bytes removed, 360 bin, no logs; both directories gone by Python and native stat;
    584886 of 584889 other `target/` entries identical, the three directory rows the strict comparison refused
    accounted (two parents, the census's scratch); 580 evidence digests agree; 4 of 4 falsifying arms refused.
  - [x] NO REGRESSION: `make focused`, exit code 0 on the final staged tree; 1168 inputs restore; production source
    unchanged; `bash scripts/check_task_history.sh` OK on both seals.
  Verification: `target/artifact-cleanup/2026-10-21/`; the external policies of §14, §17 and §18 re-read at session
    start, each body still hashing as adopted (`README_POLICY.md` `77a1e934…`, the containment body `af130de4…`,
    the adoption guide `8f77fa39…`, `pgen`'s claim verification 285 lines / 18 166 bytes), so nothing to apply.
  Commit: `ARCHOGEN-PROGRAM-0829`

