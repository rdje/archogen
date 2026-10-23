- ID: `PROGRAM.101`
  Status: `done` — fourteenth daily artifact cleanup; the record's last run `2026-10-21T12:06:25+00:00`, over 24 h old
    at session start (`2026-10-22T14:55`)
  Goal: remove only idle, regenerable compiler caches — the main and owned native-sweep incremental directories —
    after a physical and native inventory, a successful handoff census and an identity recheck; retain every
    dependency store, final product, cited evidence directory and Git workspace; prove each removed directory gone,
    every other entry under `target/` identity-equal or accounted, the tracked inputs restored and the focused tier
    regenerating; overwrite `docs/ARTIFACT_CLEANUP.md` and add the run's section to the repository annex. The commit
    also seals what closed at `HEAD`: `PROGRAM.99`.
    [Full evidence and adapters](../book/src/annex-repository.md#daily-artifact-cleanup-on-2026-10-22).
  Acceptance:
  - [x] ROOT CAUSE: `python3 -I -B target/artifact-cleanup/2026-10-22/census.py`, exit code 0: five locations,
    physical and native find/stat inventories agree; no link, nested Git boundary or cross-volume entry; the handoff
    census `handoff: OK` before the inventory and again immediately before the deletion. Outside `target/` only agent
    configuration, `build/` and the rendered book are ignored, each retained as on every run; no stray log or bin file.
  - [x] ADDRESSED: `python3 -I -B target/artifact-cleanup/2026-10-22/audit.py`, exit code 0: only 5382 idle cache
    files / 1181873277 logical bytes removed, 255 bin, no logs; both directories gone by Python and native stat;
    590423 of 590426 other `target/` entries identical, the three directory rows the strict comparison refused
    accounted (two parents, the census's scratch); 4862 evidence digests agree; 4 of 4 falsifying arms refused.
  - [x] NO REGRESSION: `bash scripts/trust_adequacy.sh --check-inputs`: `restored 1205 regular inputs`; production
    source unchanged; `bash scripts/check_task_history.sh --seal PROGRAM`: `sealed 1 subtree(s), 1 leaves: PROGRAM.99`,
    the reconstruction byte for byte; `make focused` on this staged tree: exit code 0, `tier focused: passed — 3
    passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, the main cache regenerated.
  Verification: `target/artifact-cleanup/2026-10-22/`; the external policies of §14, §17 and §18 re-read at session
    start, each body still hashing as adopted (`README_POLICY.md` `77a1e934…`, the adoption guide `8f77fa39…`,
    `pgen`'s claim verification identical to the adopted body by diff), so nothing to apply.
  Commit: `ARCHOGEN-PROGRAM-0846 (leaf PROGRAM.101)`
