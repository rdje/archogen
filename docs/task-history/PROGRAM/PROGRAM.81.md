- ID: `PROGRAM.81`
  Status: `done` — filed, cleaned and verified `2026-10-11`, the seventh daily artifact cleanup
  Goal: remove only proven regenerable project-owned artifacts, preserve cited evidence and other Git repositories,
  and overwrite `docs/ARTIFACT_CLEANUP.md` with this run's date and result.
  Acceptance: metadata census before deletion; exact deletion list with tracked regeneration path; no active compiler
  or mutation job; residue census; cold focused validation; cleanup record, task logs and book kept aligned.
  Inventory: before deletion, a metadata-only census of `target/`, excluding `.git` and links, measures 25 908 165 945
  logical file bytes in 296 411 files. Most growth since the sixth run is retained historical review workspaces;
  the twelve largest review/work directories each contain compiled products. No sealed case text was read.
  Exact safe candidates: `target/debug/incremental` — 1 896 571 227 bytes, 17 811 files, 1 218 `.bin`, no `.log`;
  `target/wasm32-unknown-unknown/debug/incremental` — 126 290 198 bytes, 729 files, 27 `.bin`, no `.log`.
  Cargo regenerates both through the tracked focused and wasm build commands. `target/debug/deps` has 789 files,
  no `.bin` or `.log`; `target/release` has 117 files, no `.bin` or `.log`, so neither needs stray-file deletion.
  Direction: remove only those two main-repository incremental caches after a process census, then verify cold.
  Retain historical review containers and their reports/probes/logs: they carry cited evidence and nested Git
  repositories, whose read-only boundary this cleanup will not cross. Retain active round-15 and round-32 evidence,
  CI tools, dependency products, Miri's sysroot and the other retained products of the sixth run.
  Trigger: the sixth run's record is dated `2026-10-10`, committed at `2026-10-10 00:31:07 UTC`; this daily run is
  started just before the next 24-hour boundary and will complete in this session. No off-volume data is touched.
  Deleted: exactly those two incremental directories — 2 022 861 425 logical file bytes, 18 540 files, 1 245 `.bin`.
  Residue census immediately after removal: both paths `gone`. The preceding OS-access handoff census printed
  `handoff: OK`; the active independent reviewer uses synthetic Python probes and no compiler or shared fixtures.
  Cold focused verification: `make focused` → `passed — 3 passed, 0 failed`, including all contract tests; no stamp
  was written because the restricted process could not name the Git tree. Final staged-tree stamping follows after
  the documentation is complete, with Git access; the hook requires its passing stamp.
  `bash scripts/wasm_build.sh` → exit 0, all nine derived I/O-free crates built for `wasm32-unknown-unknown` after
  removal of that target's incremental cache. Dependency products were retained, so these are cold-cache checks,
  not claims of a from-source rebuild of all dependencies. Book anchors/glossary/index → OK (316 index entries).
  Lockstep: the latest-only cleanup record, the book's maintenance procedure, this leaf and both logs, changelog;
  MEMORY names the received review reports and the next owned repair. Product milestones unchanged. The first
  commit attempt required the standard changelog rollover at 40 entries; `bash scripts/check_history_ledgers.sh
  --seal` → segment `0022` sealed byte for byte, 20 live entries kept. It also exposed the registry initialization
  defect, owned as `PROGRAM.82`, next; no repair to that code is mixed into this cleanup.
  Verification: `2026-10-11` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0610 (leaf PROGRAM.81)`

