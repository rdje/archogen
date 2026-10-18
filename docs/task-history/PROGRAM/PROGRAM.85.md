- ID: `PROGRAM.85`
  Status: `done` — ninth daily artifact cleanup
  Startup: clean main b48b8f3, message empty/untracked. Latest-only record is 2026-10-12, overdue at
    2026-10-17 resume. Prior cleanup, roadmap §11/§14, locality/boundary and book maintenance contracts
    reviewed. This leaf owns targeted main-repository metadata census, safe incremental-cache removal,
    residue/input restoration and regeneration checks; preserve cited diagnostics, stores and Git workspaces.
  Goal: independently inventory main target/debug/incremental, wasm debug incremental, target/debug/deps
    and target/release for generated bins/logs, rejecting links/cross-volume entries/nested Git markers.
    Delete only proven regenerable main incremental cache directories after project process census/input
    capture; retain ambiguous products/evidence. Verify immediate residue, focused and wasm regeneration,
    complete input restoration, latest-only record/book sync and per-leaf commit, then resume M3 .11.2.6.2.
  - [x] **REPRODUCE / ISSUE** — latest record 2026-10-12 is overdue at 2026-10-17 resume. Task-owned
    targeted metadata census finds regenerable main incremental products; cited evidence remains retained.
  - [x] **ROOT CAUSE** — `python3 -I -B target/artifact-cleanup/2026-10-17/census.py`, exit code 0:
    target/debug/incremental contains 343198312 logical bytes in 920 regular files, including 24 .bin;
    wasm debug incremental is empty. Independent system find/stat agrees on file counts and logical bytes.
    Requested target/debug/deps and target/release contain no .bin/.log strays. All entries are real regular
    files/directories on the repository volume, no links or nested Git markers; other target trees not traversed.
  - [x] **FIX** — explicitly delete only those main/wasm incremental directories. Before deletion recheck
    every captured file path/device/inode/mode/size and directory safety; shutil.rmtree then immediate
    os.path.lexists residue checks report both gone. Retain stores, build products, cited diagnostics and
    Git workspaces. Counts are logical file sizes, not allocated disk-space claims. Metadata/deletion
    adapters/manifests are untracked supplementary evidence, qualified; no new maintained cleanup tool.
  - [x] **ADDRESSED** — OS-readable `bash scripts/handoff_census.sh` exits 0 before deletion. Actual
    `make focused` exits 0 after caches removed: format 0.61s, clippy 0.18s, whole suite 59.90s; official
    activation-tree stamp 8f65f5e002dc22dbe579329dee7952d9db6c8987. `bash scripts/wasm_build.sh` exits 0.
    Existing dependency/build products retained; these verify regeneration workflow after incremental
    removal, not rebuilding every dependency from source. Later closure prose is checked separately.
  - [x] **NO REGRESSION** — maintained `python3 -I -B scripts/trust_adequacy.py --snapshot-inputs
    --output target/artifact-cleanup/2026-10-17/inputs.json`, exit code 0 after activation staged. After
    deletion and again after both regeneration checks, maintained `python3 -I -B scripts/trust_adequacy.py
    --check-inputs target/artifact-cleanup/2026-10-17/inputs.json --output
    target/artifact-cleanup/2026-10-17/<case>-restoration.json`, exit code 0: all 1012 regular inputs,
    independent Python/system hashes and indexed metadata restore, one opaque gitlink unchanged. Hold all
    captured inputs until results consumed. No source/script/submodule change, no full integration claim.
  - [x] **LOCKSTEP** — latest-only ARTIFACT_CLEANUP record and book describe actual retained/deleted scope;
    retire the older book example's figure classification with that example. Parent child range extends to
    .85. Ordinary PROGRAM frontier returns .71; live/resume/index return exactly to committed M3 .11.2.6.2.
    Changelog/commit log retain closure; no DEV_NOTES lesson or product status category change.
  Verification: `2026-10-17` — target/artifact-cleanup/2026-10-17 retains supplementary census/deletion
    manifests, maintained capture/restoration and actual focused/wasm logs/exits. Python/system hashes agree:
    before ded2b92d864f9d74930a53956a6dd664a0f673de0a1ed6c5527b7d8f3aca13aa;
    input e146f68fd63c1f5193b568a45bfa31b5c938c08c428a3a64b2e6b602fd87d603;
    removed 4c2a69b1561c3c4c92cb6e6965a2f69a7a34a84677fa98e2984c9f16b593c7c3;
    focused 24c42583a26a5b20d3985754b60e46a4dc9dec58e419dc62c48a5c871f74996e;
    wasm c219aa1617a5b7e1ec163ae106b6021fb8d378214665251d7ad6eccf4a568854;
    both restoration receipts dae5d03cea175ad510163f10b9c9ecbd43e649c9870ae92b07f35ea817ce97da.
    Pinned closure `bash scripts/build_book.sh` exits 0; anchors/index/glossary/source-ledger/routes/history/
    task-history/live/order gates each exit 0. Final staged gates and project process census before commit.
  Commit: `ARCHOGEN-PROGRAM-0707 (leaf PROGRAM.85)`
