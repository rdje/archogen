- ID: `PROGRAM.68`
  Status: `done` — filed and closed `2026-10-10`
  Goal: the sixth artifact cleanup the standing instruction asks for about every 24 hours, a leaf of its own as
  `PROGRAM.57` was; `docs/ARTIFACT_CLEANUP.md` names it.
  Acceptance: the trigger read off the record's commit; an inventory before any deletion; each deletion either has a
  tracked regeneration path or is a closed leaf's output that nothing tracked cites; a residue census; the tiers re-run
  cold; anything unexpected investigated rather than removed and, if it is a defect, logged and owned.

  - **Trigger.** `git log -1 --format=%ci -- docs/ARTIFACT_CLEANUP.md` → `2026-10-06 02:01:12 +0200`; the session
    clock read `2026-10-10 02:26 +0200`, four days later.
  - **Inventory before any deletion.** `target` **5 886 556 KB** (5.6 GB): `debug` 3.4 GB, of it `incremental` 2.8 GB
    and `deps` 636 MB in **789** files — so `PROGRAM.58`'s leak has not come back; `wasm32-unknown-unknown` 582 MB, its
    `incremental` 386 MB; `ci` 515 MB; `miri` 475 MB, its `incremental` 473 MB; `mutation-sweep` 293 MB; `trust-tests`
    99 MB; `target/tmp` 42 MB in 25 entries. **2 648** `.bin` and **58** `.log` files. Every top-level entry and every
    entry of `target/tmp` was checked with `git grep -F` for an owner that regenerates it (`scripts`, `crates`, `xtask`,
    `Makefile`, `.githooks`, `.github`) and for a citation anywhere tracked outside `vendor/`.
  - **Unexpected, investigated.** `M3.6.3.4` says of `target/m3634/` *"since removed"*, yet it was present, its four
    files dated `2026-10-06 10:27`: the record was false about the tree. The leaf quotes the lines it rests on, so
    deleting the directory makes the record true, and nothing else names it.
  - **Deleted.**
    1. The compiler's incremental caches — `debug`, `wasm32-unknown-unknown`, `riscv64imac-unknown-none-elf` and
       Miri's `aarch64-apple-darwin/debug` — which `cargo` rebuilds; the `deps` and Miri's sysroot are kept.
    2. The closed design review's loose output under `target/`: `mutate-full*.log` (5), `mutation-sweep-run*.log` (7),
       `round{9,11..15}_brief.txt` (6) and `trust-inventory-run.log`. Nothing tracked names any of them; `M3.6.6.1`
       is `done` and quotes what it needed.
    3. `target/mutation-sweep`, 293 MB, the scratch `scripts/mutation_sweep.sh` writes and recreates; and
       `target/recovery-round15`, the `--out` of the command the review history cites, which that command rebuilds.
    4. Uncited probe output of closed leaves: `trust-staged` (an inventory of `2026-10-06`), `m366` (two probe
       scripts), `cleanup-probe` (the fifth run's own scratch), and `m3634` above.
    5. Test scratch its owners recreate: `trust-tests`, `trust-verify-tests`, `catalog-build-tests`,
       `catalog-check-tests`, `pin-premises`, `generated-header-rustc`, `doctrine_scratch`, and every entry of
       `target/tmp` but the seven below.

    `target` 5 886 556 KB → **1 634 204 KB** (≈4.0 GB released); `.bin` 2 648 → **29**; `.log` 58 → **43**, all in
    retained entries. **Residue census: twelve of twelve sampled paths `gone`**, and `git status --porcelain` showed
    only this tree file.
  - **Retained, each on evidence.**
    1. `target/tmp/m29`, `p411`, `p42`, `p7331`, `p7332`, `p7333` — cited by `done` leaves as their evidence, as the
       fifth run found — and `m129`, `M1.29`'s, still open.
    2. `target/m3121` (37 logs) and `target/m3635`: `PROGRAM.10`'s and `M3.6.3.5`'s evidence, cited by path
       (`PROGRAM.10.md:321,379`; `M3.md`'s `trust-verify target/m3635/package`), with no tracked command that
       rebuilds them.
    3. `target/trust`, the trust tools' output and their `cargo-home`; `ci`, `miri`, `miri-sysroot`, `release`,
       `spike`, `doc`, `s0-demo`, the `wasm32` and `riscv64` products; `build/`, `.app-data` and `.qwen/`, for the
       reasons the earlier runs recorded, which still hold.
  - **Verified cold.** `bash scripts/ci_provision.sh` → both tools `already in place`, exit `0`. `make focused` →
    `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, exit `0`, in 38 s.
    `cargo test --all -q` → **1 304 passed, 0 failed over 97 suites**. `bash scripts/check_doctrines.sh` →
    `=== all doctrines green ===`; `target/doctrine_scratch`, `target/tmp/f28` and `target/trust-tests` exist again
    afterwards.
  - **Adopted policies re-checked (§14, §17, §18 of the standing instructions).** pgen's `docs/CLAIM_VERIFICATION.md`
    is unchanged since `178251cce`, and its 285 lines are `diff`-identical to the end of ours. fsmgen's
    `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` lines 190–529 still hash `af130de4…`, the adopted body; its adoption guide is
    unchanged since `727e0d086`, `8f77fa39…`; `README_POLICY.md` is unchanged since `1f0443b3a`, its lines 29–187
    hashing `77a1e934…`, the adopted body. So there is nothing to apply.
  - **Lockstep.** `docs/ARTIFACT_CLEANUP.md` overwritten with this run only; this section and both logs;
    `CHANGELOG.md`. No snapshot changes: no status, frontier head or blocker moved.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0508 (leaf PROGRAM.68)`
