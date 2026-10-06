- ID: `PROGRAM.64`
  Status: `done` — filed and closed `2026-10-06`; a whole rehearsal passes since `PROGRAM.65`
  Goal: the CI rehearsal (`scripts/ci_rehearse.sh`, `PROGRAM.10.4`) gives every process it runs the runner's want of
  a git identity and of a global configuration — those that clear their environment and keep `HOME` included.
  Acceptance: in the rehearsal's job environment, `a3c0cbd`, the commit the first CI run failed on, fails the same six
  `catalog_check` tests the runner failed, and the commit after `.10.5.1` passes them. Narrowed `2026-10-06` from a
  whole rehearsal of each commit: a whole rehearsal fails on any commit since `M2.7.4.3`, for the reason `PROGRAM.65`
  owns.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `PROGRAM.10.5.1`'s six failures reached the runner although `PROGRAM.10.4`'s
    rehearsal promises *"no global or system git configuration and no identity the job did not set"* and passed on
    `2026-09-30`. Its job environment says so with variables alone:
    ```text
    $ git grep -n -e 'GIT_CONFIG_GLOBAL=/dev/null' -e 'GIT_CONFIG_KEY_0=user.useConfigOnly' HEAD -- scripts/ci_rehearse.sh
    HEAD:scripts/ci_rehearse.sh:53:    GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
    HEAD:scripts/ci_rehearse.sh:54:    GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=user.useConfigOnly GIT_CONFIG_VALUE_0=true \
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the rehearsal's `job` says "no global configuration, no identity" with
    `GIT_CONFIG_GLOBAL`, `GIT_CONFIG_COUNT` and `GIT_CONFIG_NOSYSTEM`, and keeps this machine's `HOME`. WHY that is not
    the runner's: a process that clears its environment and passes back an allowlist keeps `HOME` and drops every
    `GIT_CONFIG_*`, so its git reads `~/.gitconfig` — here, an identity; on the runner, none.
    ```text
    $ git grep -n -e '"HOME"' HEAD -- xtask/src
    HEAD:xtask/src/catalog_build.rs:187:    for key in ["PATH", "HOME"] {
    HEAD:xtask/src/catalog_check.rs:152:        for key in ["PATH", "HOME", "GIT_DIR", "GIT_INDEX_FILE", "GIT_WORK_TREE"] {
    HEAD:xtask/src/catalog_check.rs:499:    for key in ["PATH", "HOME", "RUSTUP_HOME"] {
    ```
    `catalog_check.rs:152` is the history readers' `Git`, which `pending_date` used until `.10.5.1`; the builds of
    `catalog_build.rs`, which the trust instrument shares (`environment`), pass `RUSTUP_HOME` back as well, or derive it
    from `HOME`.
  - [x] **FIX** — `scripts/ci_rehearse.sh`: the job runs with `HOME` a directory of its own under
    `target/ci/rehearsal/`, whose `.gitconfig` holds `user.useConfigOnly = true` and nothing else, `XDG_CONFIG_HOME`
    unset, and the toolchain named by `RUSTUP_HOME` and `CARGO_HOME`, the machine's, so nothing is installed again.
    The two `GIT_CONFIG_*` settings the file now carries go; `GIT_CONFIG_NOSYSTEM` stays.
  - [x] **ADDRESSED (verified)** — in the rehearsal's checkout of `a3c0cbd`, `cargo test -p xtask catalog_check` in the
    old job environment (the `GIT_CONFIG_*` variables, the machine's `HOME`) → `test result: ok. 8 passed; 0 failed`,
    the miss; in the new one → `test result: FAILED. 2 passed; 6 failed`, each `git var GIT_COMMITTER_IDENT` failed:
    Committer identity unknown, the runner's six. On `e2baf65`, the new one → `test result: ok. 9 passed`. The job's
    home holds its `.gitconfig` and nothing else afterwards: `ls -a target/ci/rehearsal/home` → `.gitconfig`, so the
    toolchain was the machine's and nothing was installed. A whole rehearsal of `a3c0cbd` with the fix, `bash
    scripts/ci_rehearse.sh a3c0cbd`, failed its `tests` step with `test result: FAILED. 57 passed; 49 failed`. Run
    alone in the same environment, `cargo test -p xtask --bin xtask` → the same 49, `grep -E '^test .* FAILED$'` by
    module: 6 `catalog_check`, the six; 5 `catalog_build` and 38 `trust`, each refusing *"a cargo configuration above
    the repository, on a build's path (§3)"*, which the machine's own `HOME` gives too: `PROGRAM.65`.
  - [x] **NO REGRESSION** — `bash -n scripts/ci_rehearse.sh` → clean; the job's other variables unchanged; the
    doctrine gate at commit. ⚠️ Not verified: a whole rehearsal passing, which no commit since `M2.7.4.3` can until
    `PROGRAM.65`.
  - [x] **LOCKSTEP** — the script's header and comment; the book's verification chapter; `TOOLBOX.md`'s row; this leaf,
    `PROGRAM.65` and both logs; `CHANGELOG.md`.
  Verification: `2026-10-06` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0482 (leaf PROGRAM.64)`
