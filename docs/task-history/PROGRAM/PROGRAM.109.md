- ID: `PROGRAM.109`
  Status: `done` — fifteenth daily artifact cleanup; the record's last run `2026-10-22T15:54:45+00:00`, over 24 h old
    at the director's prompt (`2026-10-24`)
  Goal: remove only idle, regenerable compiler caches — the main and the wasm debug incremental directories, the
    owned native-sweep cache being absent — after a physical and native inventory, a successful handoff census and an
    identity recheck; retain every dependency store, final product, cited evidence directory and Git workspace; prove
    each removed directory gone, every other entry under `target/` identity-equal or accounted, the tracked inputs
    restored, the focused tier and `scripts/wasm_build.sh` regenerating; overwrite `docs/ARTIFACT_CLEANUP.md` and add
    the run's section to the repository annex. The commit also seals what closed at `HEAD`: `PROGRAM.100`–`.102` and
    `.104`–`.108`.
    [Full evidence and adapters](../book/src/annex-repository.md#daily-artifact-cleanup-on-2026-10-24).
  Built otherwise in one part: the wasm debug cache, empty on earlier runs, held 462 files after `M3.5.2`'s builds, so
    it was admitted beside the main one, the same kind of regenerable compiler cache, its regeneration proven by the
    documented `scripts/wasm_build.sh`; and the audit, which assumed every surveyed location present, now requires the
    one absent, the native-sweep cache, to be empty.
  Acceptance:
  - [x] ROOT CAUSE: `python3 -I -B target/artifact-cleanup/2026-10-24/census.py`, exit code 0: five locations, physical
    and native find/stat inventories agree; no link, nested Git boundary or cross-volume entry; the handoff census
    `handoff: OK` before the inventory and again immediately before the deletion. No `.log` file in any location; the
    `.bin` files are the caches' own.
  - [x] ADDRESSED: `python3 -I -B target/artifact-cleanup/2026-10-24/audit.py`, exit code 0: only 29847 idle cache
    files / 4245385971 logical bytes removed, 2232 bin, no logs; both directories gone by Python and native stat;
    850169 of 850172 other `target/` entries identical, the three directory rows the strict comparison refused
    accounted (two parents, the census's scratch); 120995 evidence digests agree; 4 of 4 falsifying arms refused.
  - [x] NO REGRESSION: `bash scripts/trust_adequacy.sh --check-inputs`: `restored 1277 regular inputs`; production
    source unchanged; `bash scripts/check_task_history.sh --seal PROGRAM`: `sealed 8 subtree(s), 8 leaves`, the
    reconstruction byte for byte; `bash scripts/wasm_build.sh`: exit code 0, the wasm cache regenerated; `make focused` on this staged tree: exit code 0, `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 260 s — cargo's fingerprints held every crate fresh, so nothing recompiled and the main cache, its directory recreated empty, refills on the next change; the wasm build recompiled and refilled its own.
  Verification: `target/artifact-cleanup/2026-10-24/`; the external policies of §14, §17 and §18 re-read at the prompt,
    each body still hashing as adopted (`README_POLICY.md`'s lines 29–187 `77a1e934…`, the adoption guide
    `8f77fa39…`, `pgen`'s claim verification, 285 lines and 18166 bytes, verbatim in the adopted file), so nothing to
    apply.
  Commit: `ARCHOGEN-PROGRAM-0928 (leaf PROGRAM.109)`
