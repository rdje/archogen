- ID: `PROGRAM.58`
  Status: `done` — filed by `PROGRAM.57` and closed `2026-10-06`
  Goal: on macOS, a build of the workspace stops leaving every object file it compiles in `target/debug/deps`.
  Reproduce / issue: found by `PROGRAM.57`'s inventory. `target/debug` was 8.1 GB, and 5.8 GB of it was `deps`, which
  held **1 454 953** `*.rcgu.o` names over 208 crate hashes (`find target/debug/deps -maxdepth 1 -name '*.rcgu.o' |
  wc -l`). Many are hard links, up to 1 462 names on one inode, because an incremental build links a cached object
  into `deps` under a new per-session name. Reproduced in a scratch crate on the pinned 1.95.0: four builds and one
  touch-only rebuild left 6, 12, 18, 24 and 30 objects under Cargo's macOS default, and 0 under `packed` and `off`. On
  the workspace (`cargo test --all --no-run` into a scratch target directory): 7 215 objects cold, then 13 313 and
  19 411 after two one-line edits to `eadl-front`, so each relink leaves about 6 100 names behind.
  Root cause, from tools: Cargo's profile reference says *"The default value for this option is `unpacked` on macOS
  for profiles that have debug information otherwise enabled"*. Under `unpacked` rustc keeps each object file so a
  debugger can find its debug information, and `nm -ap` on a test executable shows its `OSO` entries naming those
  objects by absolute path and session suffix. A later session's objects carry new names, nothing ever deletes the
  old ones, and no executable references them. Linux is not affected: rustc's default there is `off`, which deletes
  the objects.
  Measured options, one sample each: `off` — 0 objects, a 4 s relink, Linux unchanged; macOS backtraces keep the
  function names but lose file and line, and the panic location is still printed. `packed` — 0 objects, keeps line
  numbers through a `.dSYM`, a 6 s relink and a 2.5 GB target against 1.3 GB, and on Linux it would switch to split
  DWARF. A macOS-only flag in `.cargo/config.toml` works (Cargo passes `unpacked`, then the flag, and the last one
  wins), but `crates/archogen-catalog/src/package.rs` refuses `rustflags` and a configuration may hold only `alias`
  keys, so that route breaks the catalog's contract.
  Direction: `[profile.dev] split-debuginfo = "off"` in the workspace manifest, with the comment saying why;
  `CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=unpacked` brings line numbers back for one debugging session, written into
  `TOOLBOX.md`. Acceptance: a workspace relink leaves 0 objects, measured; the tiers green, the `wasm32`, `no_std`
  and Miri steps included, because the profile reaches their builds.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `find target/debug/deps -maxdepth 1 -name '*.rcgu.o' | wc -l` → 1 454 953 at
    `PROGRAM.57`'s inventory. In the scratch crate, five builds on 1.95.0 left 6 → 12 → 18 → 24 → 30 objects.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: Cargo's profile default, *"`unpacked` on macOS for profiles that have
    debug information otherwise enabled"* (Cargo's profile reference), which the workspace manifest, holding no
    `[profile.dev]`, inherited. WHY: under `unpacked` rustc keeps each object file for its debug information.
    `nm -ap` on `archogen_cli-58307b0c11ca3660` lists 473 `OSO` entries naming objects by absolute path and the
    session suffix `00we9kb`, so a later session's names are new and the old ones are referenced by nothing.
    `CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=off` and `=packed` each left 0 objects over the same five `cargo build`
    runs, which isolates the setting. rustc's codegen-options page: `off` is *"the default for platforms with ELF
    binaries"*, and *"all other platforms support `off`"*, `packed` being supported on Linux, Apple and Windows
    MSVC alone.
  - [x] **FIX** — `[profile.dev] split-debuginfo = "off"` in the workspace `Cargo.toml`, with a comment giving the
    measurement, the macOS cost, the way back and the Windows MSVC limit. The test profile inherits it. No
    `.cargo/config.toml` flag, which the catalog's rules refuse (`package.rs`, a configuration holds only `alias`
    keys).
  - [x] **ADDRESSED (verified)** — on the real `target/debug`, one one-line `eadl-front` edit then `cargo test --all
    --no-run`. Before: 7 394 → 13 492 objects (+6 098, rc=0). After the change, and after removing the leftovers once:
    0 → 0 → 0 over two such edits (rc=0). The first build after the change added nothing (13 492 → 13 492). The
    way back works: with `off` in a scratch manifest, `CARGO_PROFILE_DEV_SPLIT_DEBUGINFO=unpacked` into its own
    target directory turns `5: hatch::inner` into `6: hatch::inner at ./src/main.rs:1:52` (cargo 1.95.0).
  - [x] **NO REGRESSION** — `make integration` → `tier integration: passed — 12 passed, 0 failed, 0 unavailable, 0
    not built, 0 quarantined`, rc=0 in 186 s, its `no-std-build`, `wasm-build`, `wasm-binding` and `pin-premises`
    steps included. `bash scripts/extended_miri.sh --arm-only` → `✓ arm: Miri refused the seeded dangling-pointer
    read`, rc=0. After the documentation edits, `make focused` and the doctrine gate were re-run on the tree as
    committed (the Commit Log row).
  - [x] **LOCKSTEP** — `Cargo.toml`'s comment; `TOOLBOX.md`, a row for the way back; the book's ledger, the
    `rust-toolchain` entry's known limitations, where claims about Cargo live; `CHANGELOG.md`. No snapshot moves.
    No decision record: the setting, its reason and its measured alternatives live beside it and in this leaf.
  - **Hand-off.** rustc refuses `off` on Windows MSVC, and `ROADMAP.md` §3.1 schedules Windows hosts after the primary
    pipeline. No leaf owns that yet; the comment in `Cargo.toml` is the note that leaf will meet.
