- ID: `PROGRAM.84`
  Status: `done` — eighth daily artifact cleanup
  Goal: remove only proven regenerable project-owned compiler artifacts, preserve cited evidence and other Git
    repositories, overwrite the latest-only cleanup record and resume M3.6.9.6.3.2.2.10.3.3 from a clean commit.
  Startup: clean main 0e20af7, brief empty; seventh cleanup and maintenance/book contracts read. Seventh cleanup
    commit 6e4ec8a is 2026-10-11 00:36:06 UTC; begin this daily pass shortly before its 24-hour boundary, before
    longer domain-proof work. Relevant roadmap §11/§14, Rust cache regeneration commands and locality/boundary
    rules reviewed. No source or submodule implementation change.
  Plan: metadata census only of the main target/debug/incremental and wasm32 debug incremental caches and the
    requested target/debug/deps and target/release stray-bin/log locations. Validate real on-volume non-link
    directories/files without nested Git markers; independent system find/stat counts and logical sizes agree.
    Preserve all cited logs/probes/reports, tools, dependency stores, historical containers and Git workspaces.
    Before deletion, clean-process census and maintained indexed-input snapshot; remove only explicitly proven
    Cargo incremental cache directories, verify residue and every original tracked input, then cold focused/wasm
    checks, book/live/history sync and commit. Fix this root's child-range pointer when adding the new child.
  Inventory: targeted metadata census agrees with independent system find/stat:
    target/debug/incremental — 610779360 logical bytes, 4867 files, 300 .bin, zero .log;
    target/wasm32-unknown-unknown/debug/incremental — empty, zero files/bytes;
    target/debug/deps — 789 files/667977594 bytes, zero .bin/.log;
    target/release — 117 files/17080485 bytes, zero .bin/.log.
    All scanned entries are real regular files/directories on the repository volume, with no nested Git markers.
    Delete only the two explicitly listed incremental directories; Cargo regenerates them through make focused
    and bash scripts/wasm_build.sh. Preserve other target trees, cited evidence, dependency stores and Git workspaces.
    The metadata adapter/manifest are untracked, supplementary and qualified, with independent system census;
    target/artifact-cleanup/2026-10-12/before.json SHA-256
    5e1af46e740270ef315c1136b787b23d46d194c7087e26b1c0dfa3de14dfde02.
  Deleted: exactly those two validated incremental directories; 610779360 logical file bytes in 4867 files,
    including 300 .bin. The wasm incremental directory was empty. Immediate residue census: both paths gone.
    This count is logical file size, not a claim about physical allocated space. No release/deps strays needed removal.
    Metadata/deletion adapters and manifests are untracked supplementary producers, qualified as such; independent
    system find/stat agrees before deletion. Recipe for that independent census on macOS:
    `find <cache> -type f -exec /usr/bin/stat -f '%z' {} +`, count and sum the emitted sizes.
    Snapshot/census and regeneration commands are maintained; no historical auxiliary producer is relabelled durable.
  Verification: `2026-10-12` — OS-access `bash scripts/handoff_census.sh` exits 0, handoff: OK before deletion.
    `python3 -I -B scripts/trust_adequacy.py --snapshot-inputs --output target/artifact-cleanup/2026-10-12/inputs.json`
    exits 0: 1003 regular inputs, independent Python/system hashes agree, one opaque gitlink metadata identity.
    After deletion and again after BOTH cold-cache checks, `python3 -I -B scripts/trust_adequacy.py --check-inputs
    target/artifact-cleanup/2026-10-12/inputs.json --output target/artifact-cleanup/2026-10-12/<case>-restoration.json`
    exits 0: complete input restoration, gitlink unchanged. Snapshot SHA-256
    c511f8886f38e19f50b2bb8407458b28e6ab364e743404063825b0045030801f;
    producer 8d11a12788b158741d533caaa994eadab9c7c55a18190017c488dc25dfd83515.
    `make focused` exits 0: fmt 0.58s, clippy 0.16s, whole suite 43.26s; 3 passed/0 failed, zero
    unavailable/not-built/quarantined. Tree stamp 037155dbf5cabcf9ba37060511711167ee59ce1e binds the frozen
    cleanup activation/production tree before closure documentation. `bash scripts/wasm_build.sh` exits 0:
    all nine derived I/O-free crates build for wasm32-unknown-unknown.
    Existing build/dependency products retained; these checks run with incremental caches removed, not a claim
    that every dependency was rebuilt from source. Logs/receipts retained under target/artifact-cleanup/2026-10-12.
    No tracked Rust/script change, no hardware/full-integration/adequacy claim. Book anchors/index/glossary,
    pinned build, live snapshots/currency, task history, history ledgers, commit-log rows, source ledger,
    README routes and whitespace checks pass, exit code 0. The first commit hook refuses the mistakenly
    per-area 0627 work-unit number and the new book count's missing figure classification. The enforced ledger
    sequence is project-wide: correct to 0687 after 0686; add a dated record for the 300-bin-file phrase.
    History/order/figure gates re-run and pass, exit code 0; final staged/process checks before commit.
  Lockstep: latest-only cleanup record and book example reflect actual scope; root child-range pointer corrected
    from PROGRAM.63 to PROGRAM.84 when adding the new child. Resume snapshot returns to committed M3 helper domain
    judgments; PROGRAM's ordinary next frontier remains PROGRAM.71. Product status categories unchanged.
  Commit: `ARCHOGEN-PROGRAM-0687 (leaf PROGRAM.84)`

