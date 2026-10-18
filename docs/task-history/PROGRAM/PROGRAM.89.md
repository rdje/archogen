- ID: `PROGRAM.89`
  Status: `done` — caller/native/integration and required closure archival complete
  Goal: prevent catalog/inventory cleared build environments from defaulting temporary data outside
    the repository volume. Supply repository-derived temporary directories, preserve input/admission
    contracts, add permanent effect/failure controls, and revalidate caller premises before more E.
  Reproduction: M3 .13.2.4.1 compiles exact catalog_build::environment() and executes a real env_clear
    child. TMPDIR/TMP/TEMP absent, std::env::temp_dir outside repository and on a different volume.
    Probe reads only OS metadata and creates no off-volume project data; no historical write census.
  Impact: violates the standing same-volume/no-system-temp-default rule in catalog and trust callers.
  Priority/schedule: immediate after committed M3 .13.2.4.1, before any further public inventory run
    or matcher classification. Native six-entry replay transfers to .89.2 after the fix.
  Children: .1 shared API/producer alignment and focused controls; .2 complete caller/E/integration checkpoint.
  Verification: .89.1 permanent API/effect/error controls and focused; .89.2 complete caller/native/integration checkpoint.
  Commit: `ARCHOGEN-PROGRAM-0729 (leaf PROGRAM.89.2)` closes the parent; .89.1 committed separately.

- ID: `PROGRAM.89.2`
  Status: `done` — caller/native/integration and required closure archival complete
  Goal: revalidate existing conditional caller premises, complete old flag and current six matcher
    native replay with original/copied input restoration, operator proofs and warranted full integration.
    Source/caller revision and all failed/incomplete outcomes stay explicit; no further matcher E
    declarations until this checkpoint closes. Then resume M3 .13.2.4.2.1 metadata evidence.
  Startup: catalog/inventory §3, canonical pinned caller domain, unchanged token consumers, shared
    API and all cleared build/tool identity callers; actual source/producer/native/auditor/census and
    book trust/catalog/Annex C contracts reviewed. Preserve code, existing E and prior tests exactly.
  - [x] **ROOT CAUSE / BINDING** — python3 -I -B target/build-temp-caller-checkpoint/caller-audit.py,
    exit 0: exact db574de trust production, other consumers/preflight/modes, E and native bytes hold;
    only shared production delta is local temporary preparation and three process settings. Compiled
    env allowlist stays exact. Python/system hashes recheck28 immutable pinned sources, cargo/rustc
    identities match pin; binary digests are not claimed. Current f0fd3fd operator source and its28
    actual-edit/independent producer/wider/wrong-oracle controls hold. Permanent cleared-child/admission
    controls and exact-API/std-default metadata probe bind the new environment. TMP settings add no
    rustflags/wrapper/profile/config admission; alias/fresh-home/dynamic-target/stable/local-unit and
    visited-token premises still apply. Six matcher edits bind exact current native identities; no E.
  - [x] **ADDRESSED** — python3 -I -B target/build-temp-caller-checkpoint/replay.py, exit 0:
    native flags all14 finish3 named K/10 scoped E/1 compiler refusal, strict0, bool0/13 stmt1/1
    unviable/attempted; matcher all6 finish S, strict1, dir0/3 lit0/3. Baselines26s/22s; no timeout or
    unjudged result. The six S remain P1 in M3 .13.2.4.2; strict1 is the unmet criterion, not a passing
    adequacy claim. Compiler refusal establishes no coverage. Same filters/timeout600/strict E arguments
    in retained command files. Maintained audit_measurement rejects6 flag and5 matcher falsifiers:
    wrong exit, missing/duplicate result, replacement/summary drift, plus unlisted selected E for flags.
    Selected E revalidates only these reasons, not unrelated whole-file stale-E validity.
    First integration reports10 pass/2 fail, both the pending/active frontier mismatch and its real-tree
    self-test. All inputs restore; frontier status alone is corrected and direct/bare self-tests pass.
    Fresh make integration, exit 0: all12 local stages pass with no failure, absence or quarantine. Actual
    OS reads permitted; fmt/clippy/contracts/doctrines/self-tests/book/no-std/pin premises/wasm build/
    binding/emulator/spike all complete. Official activation stamp matches native write-tree at
    22506bfe35d9a0f976c5dd0855109e3e82243d5f. No remote CI, hardware/assurance or whole-production adequacy claim.
  - [x] **NO REGRESSION** — maintained --check-inputs against inputs.json after each native replay,
    exit 0: all1034 original regular inputs restore with bytes/modes/sizes/index/Python-system hashes
    and opaque pin metadata. Maintained regular_input adapter also restores all1034 current copied
    inputs after each replay; borrowed original index fields prove no copied index/gitlink contents.
    After integration, separate staged-epoch integration-inputs.json restores all1034 originals;
    retained native copy still matches the prior native epoch, explicitly separate. Native source/
    catalogue/tests/E equality holds throughout; ignored adapters are supplemental to maintained
    CLI/auditor/census, compiled controls and checked primary source. Integration binds the activation
    tree above; later closure edits are documentation only and receive actual pinned book plus staged
    doctrine checks. No new DEV_NOTES lesson; existing caller revalidation method applies. Book and
    canonical/live/frontier sync return to M3 metadata evidence. Close this leaf/parent before pivot.
  Closure maintenance ownership: before archival, the collection measured822236 bytes against819200.
    Use published --seal PROGRAM for committed closed units only; keep this parent/leaf active while
    sealing, prove old archives/index and native committed bodies unchanged, then close the unit.
    Book remains uncapped. No source/measurement change or new integration activation follows.
    Published --seal PROGRAM exits 0: PROGRAM.87 and PROGRAM.89.1 reconstruct byte for byte,
    TASK-HISTORY243 files/491 stubs pass. All241 old archives and exact old index after removing
    only two new rows are preserved. Native committed core bodies match with terminal interleaf LF
    normalization qualified; the published reconstruction proves complete bytes. seal-audit.json
    SHA256 17611d937765ff59629989b88180d6a4592f2c98ae3ce587d7a15d1dcfc7f4db.
  Verification: `2026-10-18`, target/build-temp-caller-checkpoint retains actual listing/plans/commands,
    raw native exits/results, source/caller audit, false-receipt results, original/current-copy censuses,
    integration log and exact activation/OS evidence. Closure docs have distinct validation scope.
    caller-audit.json SHA256 9bb31756e438ce2a68704644944a2941ec671793326bb3f15d27af80d6d4f540;
    replay-audit.json SHA256 8460630b8e6a6c9807df55518a5800360f1412768fcdc534a59aa491b77ae746;
    integration-audit.json SHA256 efbc1fb15720e21b86c3d27166315b8f26a3bdc39165cdd5b22c308cefcad984.
  Commit: `ARCHOGEN-PROGRAM-0729 (leaf PROGRAM.89.2)`
