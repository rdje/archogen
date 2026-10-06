- ID: `PROGRAM.65`
  Status: `done` — closed `2026-10-06`, by the director's ruling of the same day: option **A**, a sibling directory on
  the same volume
  Goal: the CI rehearsal's checkout has no cargo configuration above it, as the runner's has none, so a whole
  rehearsal can pass.
  Reproduce / issue: found `2026-10-06` by `PROGRAM.64`. The rehearsal checks out its commit at
  `target/ci/rehearsal/repo`, inside this checkout, whose tracked `.cargo/config.toml` (since `ARCHOGEN-PROGRAM-0029`,
  `2026-09-13`) is then on every build's directory path. Cargo reads it, and the trust instrument's
  `configurations_on_path` (`xtask/src/catalog_build.rs`, §3 of the catalog record) refuses it, rightly: *"a cargo
  configuration above the repository, on a build's path (§3)"*, failing 43 tests in the rehearsal — 38 `trust`, 5
  `catalog_build` — with the machine's `HOME` as with the job's. So no whole rehearsal has passed since those tests
  landed — `catalog_build`'s with `M2.7.4.3` (`f6381c9`, `2026-10-03`), `trust`'s with `M3.6.2` (`1d6415d`,
  `2026-10-05`) — both after the last rehearsal, `2026-09-30`. The runner's checkout has none above it: the same 43
  tests pass there (`.10.5`'s second run).
  Why blocked: the fix moves the rehearsal's checkout outside this repository's tree, and the data-locality rule keeps
  every scratch file under `target/` here. Options for the director: **A** — a sibling directory on the same volume,
  made and removed by the script, recorded in a decision as the rule's one exception (recommended: the simplest, and
  the bytes stay on this volume); **B** — a disk image under `target/`, mounted at a path outside the tree for the run
  (the bytes stay under `target/`, but it is macOS-only and needs a mount); **C** — keep the checkout where it is and
  state the rehearsal unable to run the `trust` tests, which leaves it red on every commit.
  Acceptance: `bash scripts/ci_rehearse.sh` on `HEAD` passes, and on `a3c0cbd` fails at the six `catalog_check` tests
  alone.
  **Ruled `2026-10-06`.** The director first offered a git-ignored `.archogen-data/` at the repository's root as a
  store (*"At the root of the ARCHOGEN repo you can create a git ignored .archogen-data/ where you can store and manage
  stuff there"*); told that a directory inside the repository still has the root's `.cargo/config.toml` above it, chose
  **A**, a sibling directory on the same SSD, made and removed by the script, the one exception to scratch under
  `target/`. Both are `docs/decisions/decision_ci-rehearsal-beside-the-repository.md`.
  Plan: the checkout at `../.archogen-ci-rehearsal/`, beside this repository; the script checks, before anything runs,
  that no cargo configuration lies above it and says "could not run" if one does; removes it on exit unless
  `CI_REHEARSE_KEEP=1`; `.archogen-data/` ignored by git.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `PROGRAM.64`'s whole rehearsal of `a3c0cbd` under `target/ci/rehearsal/`: `test result:
    FAILED. 57 passed; 49 failed`, 43 of them — 38 `trust`, 5 `catalog_build` — refusing the outer configuration.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `scripts/ci_rehearse.sh`'s `WORK="$ROOT/target/ci/rehearsal"`, inside this
    checkout, whose tracked `.cargo/config.toml` (`git ls-files .cargo` → `.cargo/config.toml`) lies on every build's
    path there; `configurations_on_path` refuses a configuration above the repository it judges, rightly, and cargo
    reads it. WHY nothing inside the tree can do: `.archogen-data/` included, every directory under the root has the
    root above it.
  - [x] **FIX** — `WORK="$(dirname "$ROOT")/.archogen-ci-rehearsal"`, a guard that the path is exactly that, the walk
    up its directory path refusing to run beside a cargo configuration, exit 2, and the directory removed on exit;
    `docs/decisions/decision_ci-rehearsal-beside-the-repository.md` and its index row; the scratch decision names the
    exception; `.gitignore` ignores `/.archogen-data`.
  - [x] **ADDRESSED (verified)** — `bash scripts/ci_rehearse.sh HEAD` (`139c85e`) → *"rehearsing 139c85e in
    …/github/.archogen-ci-rehearsal/repo — 869 tracked files, nothing else"*, `tier integration: passed — 12 passed, 0
    failed, 0 unavailable, 0 not built, 0 quarantined`, *"the job would pass"*, `rc=0` — the first whole rehearsal to
    pass since `2026-09-30`. `bash scripts/ci_rehearse.sh a3c0cbd` → `test result: FAILED. 100 passed; 6 failed`, the
    six `catalog_check` tests the runner failed and no other, `rc=1`. `ls -d ../.archogen-ci-rehearsal` afterwards →
    *"No such file or directory"*.
  - [x] **NO REGRESSION** — `bash -n scripts/ci_rehearse.sh` → clean; `bash scripts/check_scratch_locality.sh` →
    `scratch-locality: OK (270 file(s) scanned …)`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the script's header; the decision record, its index row, the scratch decision's exception; the
    book's `verification.md`; `TOOLBOX.md`'s row; `PROGRAM.64`'s status; this leaf and both logs; `CHANGELOG.md`.
  Verification: `2026-10-06` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0488 (leaf PROGRAM.65)`
