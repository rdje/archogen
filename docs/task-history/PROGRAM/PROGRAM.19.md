- ID: `PROGRAM.19`
  Status: `done`
  Goal: run the ~24-hour artifact cleanup the standing instructions require, and start the record that
  makes "when was the last one?" answerable — `docs/ARTIFACT_CLEANUP.md` did not exist, so no session
  could tell whether a cleanup was due, which is the mechanism the instruction created the file for.
  Reproduce / issue: `ls docs/ARTIFACT_CLEANUP.md` → no such file. Inventory measured before touching
  anything: `.app-data` ≈ 3.6 GB, of which `.app-data/target` is **1.3 GB** — the *previous* pin's
  LinkedSpec build, superseded by the pin-named `.app-data/target-2ac834913` (2.1 GB) — and
  `.app-data/pgen-generated-2ac834913` is **70 MB** duplicating the checkout's own generated parser,
  digest-verified identical; `target/` is 815 MB, of which `target/tmp` is 17 MB of test scratch
  carrying most of **703** stale incremental `.bin` files; `docs/book/book` is 2.3 MB of built book.
  Impact: nothing is broken, but 1.4 GB of it is unreachable-by-design residue whose regeneration path
  is a tracked command, and one item — a documentation snapshot parked inside a *build* directory — is
  in the wrong place whatever its size.
  Acceptance: only artifacts whose regeneration path is a **tracked command** are deleted; every
  retained item has its reason recorded in the leaf; nothing tracked is touched, so
  `git status --porcelain` shows only this leaf and the new record; the focused tier, the doctrine gate
  and one vendor instrument all still run green afterwards, measured rather than assumed;
  `docs/ARTIFACT_CLEANUP.md` carries the date and a one-line summary with only the latest entry kept;
  anything unexpected found on the way is investigated and reported, not deleted.
  Priority: **low effort, low risk, mandated** — the instruction is explicit that a missing record file
  means a cleanup is due this session.
  Verification: see the acceptance checklist below.
  Commit: `ARCHOGEN-PROGRAM-0060 (leaf PROGRAM.19)`
  promotion: declined (the operative rules now live where the next session must read them —
  `docs/ARTIFACT_CLEANUP.md` states both the trigger and the delete-only-what-regenerates test — and the
  one interesting finding this cleanup produced is recorded twice already: in `S0.7`'s leaf and in the
  `DEV_NOTES` lesson above it. A knowledge card would restate a standing instruction plus a fix that is
  now in the code it concerns.)

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `docs/ARTIFACT_CLEANUP.md` did not exist
    (`ls docs/ARTIFACT_CLEANUP.md` → no such file), so the instruction's own trigger — "if it is more
    than 24 hours old, **or the file does not exist**" — had been firing on every session with no way
    to tell. WHY the residue accumulated: `.app-data` is the vendor guide's application-local data root
    and is deliberately ignored, so nothing ever revisits it; `M1.20.4` added a *pin-named* target
    directory beside the old one (correctly — the guide says a new one preserves the older build for
    comparison), which is exactly the moment the older one stops being needed and starts being 1.3 GB
    of dead weight. The residue was not a mistake; it was a comparison that outlived its purpose.
  - [x] **ADDRESSED (verified)** — released ≈1.4 GB, each item deleted only because its regeneration
    path is a tracked command: `.app-data/target` 1.3 GB (rebuild via `scripts/linkedspec_eval.sh
    build` at whatever pin is checked out), `.app-data/pgen-generated-2ac834913` 70 MB (a
    digest-verified duplicate of the checkout's own `generated/`, and re-derivable via
    `linkedspec_eval.sh prepare`), `.app-data/empty-store-1` and `-2` (the instruments create them),
    `.app-data/reference-check` (`linkedspec_eval.sh reference` recreates it), `.app-data/upstream-notice27`
    (a prior session's scratch capture; the durable content is the tracked `UPSTREAM.md`), and
    `target/tmp` 17 MB (test scratch the suite recreates). Measured: `.app-data` 3.5 GB → 2.2 GB, and
    `target` 815 MB → 799 MB immediately after the deletion — then back to 816 MB once the verification
    runs recreated the test scratch, which is the expected result and the reason deleting it was safe.
    **Residue census after deletion:** all seven paths report `gone`, none `STILL PRESENT`.
    `docs/ARTIFACT_CLEANUP.md` now carries the date and one entry, with the mechanism stated so the
    next session can act on it.
  - [x] **NO REGRESSION** — nothing tracked was touched: after the deletions `git status --porcelain`
    listed only this leaf, and at commit time only this leaf plus the new record. The retained vendor
    build still works, measured rather than assumed: `scripts/linkedspec_eval.sh bins` → both binaries
    resolve under `.app-data/target-2ac834913/debug/`; `linkedspec_eval.sh reference` →
    `REFERENCE CHECK: the published two-form tagged document, exactly as documented`;
    `LS-002 …/remeasure.sh --self-test` → `9/9 arms passed`. `bash scripts/check_doctrines.sh` →
    `=== all doctrines green ===`; `make focused` → exit `0`; `cargo test --all` → **421 passed,
    0 failed** over 36 suites.
    ⛔ **The verification did not pass first, and that is the point of running it.** The first
    post-cleanup `make focused` reported `tier focused: failed — 2 passed, 1 failed`: with the test
    binaries cached and `target/tmp` gone, `a_description_with_no_system_says_there_is_nothing_to_build`
    panicked at `crates/archogen-cli/tests/s0_build.rs:211`. The warm re-run passed, which is how this
    would have been written off as a flake; it was reproduced deliberately instead
    (`rm -rf target/tmp && cargo test --all`, twice) and fixed at the root in leaf **`S0.7`** — cargo
    creates `CARGO_TARGET_TMPDIR` at build time, not run time, and that test was the only one of six
    sites writing into the tmpdir root. This leaf's acceptance is therefore measured *after* `S0.7`,
    and the record file says so.
  - [x] **FIX** — delete only what regenerates from a tracked command, retain everything else with a
    reason, and investigate rather than remove anything unexpected. **Retained, with reasons:**
    `.app-data/target-2ac834913` (the current pin's build; every remaining LinkedSpec measurement uses
    it), `.app-data/cargo-home` 132 MB (the vendor's guide says offline builds depend on the retained
    store), `.app-data/pgen-generated-before-remeasure` 18 MB (the **only** copy of the previous pin's
    parser — it is the "before" side of the regeneration digest frozen in `LS-004`'s evidence, and
    deleting it would make that digest unreproducible), `.app-data/ls004` 32 KB (the primary logs behind
    that same frozen evidence), `target/debug` 794 MB and `target/riscv64imac-unknown-none-elf` 5 MB
    (live products of the focused and integration tiers), and `docs/book/book` 2.3 MB (the built book —
    the director's window; it rebuilds in 0.07 s but costs nothing to keep).
    ⭐ **One item was investigated and deliberately left alone:** `target/sync-backup-2026-09-21`,
    24 KB, holding copies of `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`, `TASK_TREE.md` and `TOOLBOX.md`
    dated `2026-09-21`, and referenced by nothing tracked at the time it was investigated —
    `git grep -ln 'sync-backup' -- .` → no match before this leaf mentioned it, and one match
    afterwards, which is this sentence.
    Its contents are recoverable from git at any revision, so it is redundant — but it is somebody's
    deliberate backup, it is 24 KB, and "unexpected state may be someone's in-progress work" outranks
    tidiness. **Flagged, not deleted:** a documentation snapshot parked inside a *build* directory is in
    the wrong place whatever its size, and either belongs in git or nowhere.
  - [x] **LOCKSTEP** — `docs/ARTIFACT_CLEANUP.md` (new, latest-entry-only); this leaf, the frontier and
    both logs; `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`, `DEV_NOTES.md`. The cross-tree link is
    recorded in both directions: `S0.7` names this leaf as what surfaced it, and this leaf names `S0.7`
    as what its verification found. No book chapter changes — the book documents eADL and the engine,
    not the repository's scratch directories, and `git grep -ln 'app-data' -- docs/book` → no match,
    `rc=1`.

  ### Run `2026-09-28` — the second cleanup, under this leaf because the obligation is recurring

  A new leaf per run would flood this tree with one entry per day forever, and the mechanism this leaf
  built is precisely a *latest-only* record with a standing owner. So runs append here and
  `docs/ARTIFACT_CLEANUP.md` stays the answer to "is one due?". Status stays `done`: the leaf's goal was
  the mechanism, and it works — this run is the mechanism running.

  - **Trigger, measured rather than assumed.** `docs/ARTIFACT_CLEANUP.md` recorded `2026-09-27` and the
    session date is `2026-09-28`; the record carries a date and not a time, so "more than 24 hours old"
    is undecidable at that granularity. Cleaned, because the instruction's own tie-break is to clean.
  - **Inventory before touching anything.** `target` 873 MB, of which `target/tmp` 17 MB and
    `target/debug/incremental` 614 MB; `.app-data` 2.2 GB (`target-2ac834913` 2.0 GB,
    `cargo-home` 132 MB, `pgen-generated-before-remeasure` 18 MB); `docs/book/book` 2.3 MB; `build/`
    empty; **715** `.bin` files and **0** `.log` files under `target`.
  - **Deleted, each with a tracked regeneration path.** The test scratch under `target/tmp` — `f28`
    (14 MB), `s0-build` (2.8 MB), `s0-oracle`, `s0-provenance`, `s0-reader`, `s0-build-library.eadl` —
    all recreated by `cargo test`; and one stale `…/archogen_cli-…/s-…-working` incremental directory,
    the residue of an interrupted build. `target` 873 MB → 855 MB, `.bin` count 715 → 693. **Residue
    census: all seven paths report `gone`, none `STILL PRESENT`.** `target/tmp/m112` (44 KB) was
    retained because it is the *active* literal-probe instrument of the session doing the cleanup, not
    residue.
  - ⛔ **Two deletions this run did not make, both after investigation rather than by policy.**
    1. `.app-data/pgen-generated-before-remeasure` — independently re-investigated and re-retained.
       `git grep -rn 'pgen-generated-before-remeasure'` → `LS-004`'s `remeasure.sh:225-227`, which
       treats an existing backup as a reason to **keep** it and prints `a backup already exists … —
       kept`. Deleting it would not merely lose the "before" digest this leaf's checklist already
       recorded; it would change what a *frozen* instrument prints on its next run.
    2. `target/debug/incremental`, 614 MB and 684 of the 693 remaining `.bin` files. Not residue:
       measured at most **four** `s-*` generations per crate across 121 crate directories, which is
       cargo's own retention, and its "regeneration path" is a full rebuild of a 37-suite workspace.
       The standing instruction says to *check* that directory, and checking it produced a reason to
       keep it — deleting live cache to satisfy a word count would slow every subsequent edit loop
       without removing anything stale.
  - **Verified cold, which is the only verification that means anything here.** `make focused` →
    `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built` with the scratch it
    consumes already deleted, so the F28/S0 suites recreated what they need. That is `S0.7`'s lesson
    applied rather than remembered: the first run of the previous cleanup *failed* on exactly this, and
    a warm re-run would have hidden it again. `bash scripts/check_doctrines.sh` → `=== all doctrines
    green ===`. `git status --porcelain` after the deletions → empty, so nothing tracked was touched.
  - **Lockstep.** `docs/ARTIFACT_CLEANUP.md` overwritten with this run only; this section;
    `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md`. No frontier move — the frontier stays where `M1`
    left it — and no book change, for the reason recorded above.

  ### Run `2026-09-29` — the third cleanup

  Same standing owner, same mechanism. One difference worth recording: **the unexpected item was
  identified rather than merely retained**, and identifying it is what made it deletable — and is also
  what exposed a hazard this leaf does not own.

  - **Trigger, read off the record rather than assumed.** `docs/ARTIFACT_CLEANUP.md` recorded
    `2026-09-28`, and `git log -1 --format='%ci' -- docs/ARTIFACT_CLEANUP.md` →
    `2026-09-28 03:57:18 +0200` against a session clock of `2026-09-29 10:38 CEST` — more than 24 hours
    on the commit's own timestamp, so the date-only granularity that made the previous run undecidable
    did not have to be guessed at this time.
  - **Inventory before touching anything.** `target` 957 MB, of which `target/tmp` 9 MB,
    `target/doctrine_scratch` 260 KB, `target/sync-backup-2026-09-21` 24 KB, `target/s0-demo` 20 KB, and
    **696** `.bin` files under `target/debug/incremental` plus 9 more under the `no_std` target's; a
    further 15 appeared under `target/tmp/f28` once the suite had run. `.app-data` 2.2 GB
    (`target-2ac834913` 2.0 GB, `cargo-home` 132 MB, `pgen-generated-before-remeasure` 18 MB, `ls004`
    32 KB). `build/` 16 KB. `.log` files: **4** outside the submodule, all under `.app-data/ls004/`, and
    **27** inside `vendor/linkedspec`, which §20 and §21 put out of reach and out of scope.
  - **Deleted, each with a tracked regeneration path.** The twelve scratch directories under
    `target/tmp` — `f28`, `m112`, `m1125`, `m113`, `m1132`, `m1134`, `p21`, `s0-build`,
    `s0-build-library.eadl`, `s0-oracle`, `s0-provenance`, `s0-reader` — of which six are recreated by
    `cargo test` and the `m*`/`p21` ones are prior leaves' probe scratch whose measurements are
    recorded on their leaves (`M1.12`, `M1.13`, `M1.13.2`, `M1.13.4`, `PROGRAM.21`); and
    `target/doctrine_scratch`, which `scripts/check_gap_claims.sh:40` recreates on the next commit.
    `target` 957 MB → 949 MB. **Residue census: all five sampled paths report `gone`, none `STILL
    PRESENT`.** Unlike the previous run, no leaf's *active* scratch was spared: `M1.13.4`'s probe
    directory `m1134` was deleted only after its measurements were written onto the leaf and committed
    in `ARCHOGEN-M1-0089`, so the record outlives the scratch it was taken from.
  - ⛔ **The unexpected item, identified before it was deleted.** `target/sync-backup-2026-09-21/`
    held four spine files — `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, `TASK_TREE.md` — that
    match **no** committed state of this repository: `diff` against both `a4cbab5^:COMMIT.md` and
    `a4cbab5:COMMIT.md` differs, and likewise for the other three (`DIFFER` on 4 of 4 both ways). The
    first instinct — retain it, because it is not reproducible from here — was the wrong stopping
    point, and §21 permits reading a sibling repository. The `bedrock` checkout settles it:
    `git -C ../bedrock show HEAD:<path>` is **byte-identical** to all four (`MATCH` on 4 of 4, including
    `docs/TASK_TREE.md` for the backup's `TASK_TREE.md`). So the directory is a copy of the *incoming*
    scaffold and not a backup of archogen's own spine, and its regeneration path is a read of a
    repository that exists on this volume. Deleted, with that evidence.
  - ⭐ **And identifying it exposed a hazard that is not this leaf's, so it is filed rather than fixed
    here.** archogen is on `bedrock-scaffold 0.8.1` (`cat DOCTRINE_VERSION`) where upstream is `0.10.0`,
    and this repository's `scripts/update_scaffold.sh` still `cp`s every NEUTRAL file straight over the
    project's copy — including `docs/TASK_TREE.md`, whose Active Task Trees table *is* project content,
    and `COMMIT.md`, which carries this project's tier paragraph and the no-agent-trailer ruling.
    Upstream fixed exactly that shape (`BEDROCK-MAINTENANCE-0015` and `-0016`: "never overwrites
    anything", "the merge is asked for and never applied"). Owner: **`PROGRAM.26`**.
  - **Two retentions, both new and both investigations rather than policy.**
    1. `build/riscv-virt.dtb` and `build/riscv-virt.dts`, 16 KB. Their regeneration path is
       `scripts/target_emulator.sh --dump-dtb`, which needs the **pinned emulator** to be present, and
       `M2.8.2` is the leaf whose whole subject is comparing a device-tree fixture against them.
       Deleting 16 KB to satisfy a word in the instruction, one leaf before the leaf that needs the
       file, is a bad trade and is recorded as declined.
    2. `target/s0-demo/base`, 20 KB. A **closed** leaf cites it as verification evidence
       (`docs/tasks/S0.md:181`: `archogen build examples/s0-heartbeat/system.eadl --out
       target/s0-demo/base` → `exit=0`), and recreating it is a full `archogen build` rather than a
       `cargo test` — so it does not meet the bar the six `s0-*` scratch directories meet.
    The previous run's two retentions were re-checked and stand: `.app-data/pgen-generated-before-remeasure`
    (18 MB) is still read by `LS-004`'s `remeasure.sh`, whose own behaviour changes if the backup is
    absent, and `target/debug/incremental` is still cargo's retention rather than residue.
    `.app-data/ls004/`'s four `.log` files are retained with it: they are a frozen instrument's RED-arm
    output, and no document cites them, so nothing here can prove they are regenerable.
  - **Verified cold, which is the only verification that means anything here.** `make focused` →
    `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built`, exit `0`, with the scratch
    it consumes already deleted; `cargo test --all` → **492 passed, 0 failed**, matching the recorded
    baseline; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`, and
    `target/doctrine_scratch/gap_claim_census` exists again afterwards, which is the proof that the
    deleted directory was scratch and not state. `target` measured 965 MB after these runs — **larger**
    than before the deletion — because the suite recreated its scratch and cargo its cache, which is
    the same observation the first run made and the reason deleting them was safe.
  - **Lockstep.** `docs/ARTIFACT_CLEANUP.md` overwritten with this run only; this section;
    `LIVE_STATUS.md`'s `PROGRAM` row; `CHANGELOG.md`. No frontier move and no book change, for the
    reason the first run recorded: `git grep -ln 'app-data' -- docs/book` → no match, and the book
    documents eADL and the engine, not the repository's scratch directories.

  ### Run `2026-09-30` — the fourth cleanup

  - **Trigger, read off the commit.** `git log -1 --format=%ci -- docs/ARTIFACT_CLEANUP.md` →
    `2026-09-29 11:29:00 +0200`, against a session clock of `2026-09-30 12:19 +0200`: 24 h 50 min.
  - **Inventory before touching anything.** `target` **6.3 GB**, up from 949 MB a day earlier:
    `target/debug` 2.8 GB, `target/ci` 2.7 GB, `target/miri` 458 MB, `target/wasm32-unknown-unknown`
    144 MB, `target/doctrine_scratch` 112 MB, `target/miri-sysroot` 108 MB, `target/tmp` 40 MB in 192
    entries; **2 766** `.bin` and **25** `.log` files under `target`. `.app-data` 2.2 GB, unchanged. `build/`
    2.7 MB. Each entry of `target/tmp` and `target/doctrine_scratch` was censused against the tracked tree
    (`git grep -l -F <name> -- scripts crates xtask Makefile .githooks` for a regenerating owner,
    `-- docs '*.md'` for a citation), and every leaf its name points at was read for its status.
  - **Deleted, each with a tracked regeneration path or a closed owner.**
    1. `target/ci/build`, 1.7 GB: the QEMU source and build tree `scripts/ci_provision.sh` unpacks to
       compile the pinned emulator. Once the tool is installed under `target/ci/tools/`, the script
       returns `already in place` without reading it, and a rebuild `rm -rf`s it before unpacking the
       digest-checked tarball again. `git grep -n 'ci/build'` → only the provisioner itself.
    2. `target/ci/rehearsal`, 517 MB: `scripts/ci_rehearse.sh` `rm -rf`s it at the start of every run.
       With it, `provision_run.txt` and `rehearse_run.txt`, hand-captured outputs of `PROGRAM.10.4`,
       which is `done` and carries the transcript (`ci_rehearse.sh → exit=0 (18f55e4 …)`).
    3. `target/doctrine_scratch`, 112 MB, of which 100 MB was `api4`, `API.4`'s cost-probe chains.
       Every subdirectory a script uses is recreated by that script; every other entry is a closed
       leaf's captured output (`api-*`, `m1-*`, `m2-*`, `s0b`, `m110*.py`, …), and no tracked file
       cites any of them.
    4. `target/tmp`, all but one entry. The test scratch (`f28`, `fires-on`, `s0-*`, `module-*`, …) is
       recreated by `cargo test`; the rest is probe scratch of leaves that are all `done`: `M1.13.4.1`,
       `.2`, `.3` and `.5`, `M1.13.5`, `M1.25`, `M1.26`–`M1.26.2`, `M1.34`–`M1.36`, `PROGRAM.5`,
       `.6.1`–`.6.3`, `.9`, `.9.2`, `.9.3`, `.13`, `.15`, `.17.1`–`.17.3`, `.20.1`–`.20.3`, `.27`–`.29`. Four
       leaves cite an input there by path (`m1261/s0-badunit.eadl`, `m129/app.system.eadl`,
       `p27/dup-decl.eadl`, `p27/head-before-m1282.txt`), and each leaf states how its input was made
       from tracked sources, so none of those files is the only copy of anything. `m29_*`, from the
       blocked `M2.9`, were two copies of the doctrine driver's output; the work is on `wip/m2.9`.

    `target` 6.3 GB → 4.1 GB; `.bin` 2 766 → 2 212; `.log` 25 → 3. **Residue census: all ten sampled paths
    report `gone`, none `STILL PRESENT`**, and `git status --porcelain` → empty.
  - **Retained, each on evidence.**
    1. `target/tmp/m129`, 744 KB: `M1.29` is still `active`, since `M1.29.4` waits on the director.
    2. `target/ci/tools` 357 MB, `downloads` 145 MB, `venv` 14 MB: the installed pinned tools the
       rehearsal links in; the digest-verified tarball a rebuild would otherwise fetch over the network;
       and the `ninja` a local QEMU build needs (`PATH=<venv ninja>:$PATH bash scripts/ci_provision.sh`,
       `PROGRAM.10.3`), which no tracked script creates. `target/ci/integration.log` is the CI job's kept
       report.
    3. `target/debug`, 2.8 GB. Its growth from 794 MB is measured, and it is cargo's own retention rather
       than residue: 378 incremental crate directories, **none** above four `s-*` generations (127 hold 2,
       1 holds 3, 250 hold 4). `deps` holds 131 executables, up to 8 hash variants of one name, which are
       the build configurations the tiers use.
    4. `target/miri` and `target/miri-sysroot`, 566 MB: the `extended` tier's cache, about 14 minutes
       to rebuild. `target/wasm32-unknown-unknown`, `target/riscv64imac-unknown-none-elf`,
       `target/release` (the fuzz step's build) and `target/spike`: live products of tier steps.
    5. `target/s0-demo` and `build/`, for the reasons the third run recorded, which still stand. `build/`
       also holds `heartbeat` and `s0`, the working files of `docs/book/src/s0.md`'s transcripts.
    6. `.app-data`, unchanged, for the reasons the second and third runs recorded.
  - **Verified cold.** `bash scripts/ci_provision.sh` → `mdbook v0.5.2 already in place`, `QEMU emulator
    version 11.1.1 already in place`, exit `0`, with the build tree gone. `make focused` → `tier focused:
    passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, exit `0`.
    `cargo test --all -q` → **742 passed, 0 failed over 62 suites**, the recorded baseline.
    `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`, and `target/doctrine_scratch`
    exists again afterwards.
  - **Lockstep.** `docs/ARTIFACT_CLEANUP.md` overwritten with this run only; this section; `CHANGELOG.md`.
    No snapshot changes: no status, frontier or blocker moved (`LIVE_DOCUMENT_SIZE_CONTAINMENT.md`).
