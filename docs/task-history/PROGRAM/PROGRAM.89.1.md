- ID: `PROGRAM.89.1`
  Status: `done` — checked shared local-temp environment and permanent controls
  Goal: derive/create a temp directory from each build's project-local target, set TMPDIR/TMP/TEMP
    after clearing inherited environment, propagate creation failures before tools execute. Verify
    effective child paths/volumes, absent/file/error and hostile inherited-temp cases, and actual
    public build callers. Keep compiled env-dependency admissions narrow. Align producer proof envs
    where needed, update canonical/book contracts and run focused/operator checks before commit.
  Startup: roadmap §4.4/§14.4, catalog hashes §3, inventory §3, pinned caller domain,
    shared environment/cleared-command callers and existing catalog/trust public fixtures reviewed.
    Book catalog/trust/Annex C reviewed; source contract currently lists no local temp variables.
  - [x] **ROOT CAUSE** — M3 .13.2.4.1 exact API/cleared-child metadata evidence; permanent
    cargo test -p xtask catalog_build::tests::cleared_builds_use_local_temporary_directories_despite_inherited_overrides
    -- --exact and build_environment_refuses_blocked_storage_and_recovers_after_removal both RED101.
    Missing keys and accepted temp-file destination reproduce before repair, with passive checks before
    any child temp write. No off-volume project data created and no historical write/migration claim.
  - [x] **ADDRESSED** — target/tmp checked creation/canonicalization, TMPDIR/TMP/TEMP explicitly
    supplied after clearing. Permanent children verify absent/hostile inputs, relative target/different
    cwd, explicit child path and Unix volume, actual local file effect; blocked Cargo/target/temp files stay
    exact and recovery succeeds. Catalog test filter passes15 with one internal probe ignored; public
    temporary_process_paths_are_not_admitted_as_compiled_environment_inputs passes all three variables.
    Native cargo xtask mutate --timeouts-fail --only build-environment-local-temp-variables-omitted
    build-environment-temp-creation-error-ignored exit 0, both named K, no refusal/timeout/unjudged.
    Native result: mutate: OK — 2 mutation(s), each killed or surviving exactly as the catalog expects.
    First catalogue loop context omitted its format's four-space escape and failed binding, while the
    ignored-error control killed; exact indented context corrected before authoritative two-entry pass.
  - [x] **NO REGRESSION** — python3 -I -B xtask/mutation_sweep_tests.py <root>, exit 0: all28 controls
    pass after both actual producer environments gain local TMP/TEMP beside TMPDIR. Literal argv and
    compiled candidate/proof controls hold. Native/full-input census restores all1034 regular inputs,
    modes/index/opaque pin identity, independent Python/system hashes. Source reconstruction proves
    every prior body outside shared API/doc and appended controls exact, old catalogue prefix and E
    unchanged. The native catalogue operates/restores originals; no copied-input census claimed.
    Canonical catalog/inventory/domain records and book align. Exact final staged activation
    must pass focused3/3 with native write-tree stamp; actual pinned book and staged gates bind commit.
    No new integration or final caller/E checkpoint claim: PROGRAM.89.2 owns it.
    The locality gate forbids tracked temp_dir() calls. Permanent controls use explicit child settings;
    a separate passive exact-API/std-default probe confirms all five locality/presence booleans true.
    Its read-only metadata scope remains distinct from the permanent child file-effect control.
  Verification: `2026-10-18`, target/build-temp-locality retains RED exits/logs, final mutations-final.log, green public API/
    catalogue results, full28 operator output, original-input restoration and source/audit receipts.
    audit.json SHA256 405506f965b49fa49de116ff66aeb15d3b44760b35f2ca675ba61fcbad50bd69; environment SHA256 8b6fa81a2d715c2e42f8ab75a02b6e0c037c97d2f55e71c44c21e4a5516ee74c.
    Final focused/stamp and book/staged checks required, with no post-activation edits.
  Commit: `ARCHOGEN-PROGRAM-0728 (leaf PROGRAM.89.1)`
