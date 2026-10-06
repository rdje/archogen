- ID: `PROGRAM.10`
  Status: `done` — closed `2026-10-06`: the `integration` job green on the runner, `7ac8b8b`, after `.10.5` fixed what
  the first run found
  Goal: run the **integration** tier in CI — provision `mdbook` and `qemu-system-riscv64` on the
  runner — and decide the blocking policy for an `incomplete` verdict.
  Acceptance: CI runs `cargo xtask verify --tier integration`; the repository has a recorded,
  reasoned answer to whether exit 20 blocks a build, and the workflow implements that answer.

  ⛔ **The open question is the deliverable, not the YAML.** Wiring it up before deciding
  produces either a permanently red CI that people learn to ignore, or a green one that hides the
  gap — and both are worse than the comment currently in `.github/workflows/rust.yml` saying so.
  The emulator step also cannot pass anywhere until `targets/riscv-virt-up.env` loses
  `TARGET_VERIFIED=no`, which `M2.8` confirms against an installed QEMU — the *installation* is no
  longer the missing piece (director finding 2 is resolved as of `2026-09-27`); the pin and the §3.2
  device-tree agreement check are.

  ⛔ **Measured `2026-09-28`: the verdict this leaf must rule on has already changed shape, and the
  change is what blocks pushing.** When this leaf was written the emulator step reported
  `Unavailable` → tier `incomplete` → exit `20`, because the tool was absent. QEMU is now installed,
  so `scripts/target_emulator.sh --check` reaches the unpinned-config branch and exits `1`, and the
  runner maps "ran, nonzero" to `Failed` → tier `failed` → `make integration` exit `1`. That matters
  because `COMMIT.md` step 2 treats the two differently: it explicitly permits proceeding past
  **incomplete** after reading what it names ("*not* a pass and *not* a failure: nothing broke, and
  something could not be run"), and it does **not** permit proceeding past **failed**. So the branch
  has been unpushed since `origin/main`'s `32e6b14` (`2026-09-13`) over a *classification*, not over
  a broken build — `git rev-list --count origin/main..HEAD` for the live number, which moves with
  every commit and is therefore deliberately not written down here.
  ⭐ **And §14.3 already supplies the mechanism, unused.** "A required tool skipped or unavailable is
  reported as such, not a passed check. Quarantine requires a named issue, owner, affected claim, and
  bounded scope." The runner already has the vocabulary — `Outcome::{Passed, Failed, Unavailable,
  NotBuilt}` and `Action::NotBuilt { owner, note }`, which five steps use under `PROGRAM.9`. "QEMU is
  present; the release is unpinned and `DEVICE_TREE_FIXTURE` does not exist, so the §3.2 agreement
  check could not be run" is *incomplete*, quarantined with owner `M2.8`, affected claim
  `target-verification`, bounded to that one step. This leaf's deliverable is therefore not only the
  CI provisioning and the blocking policy — it is also making the step report the verdict §14.3's own
  vocabulary already has a word for.
  Verification: `2026-10-06` — `.10.5`'s second run: `rust` and `doctrines` both `success` on `7ac8b8b`
  Commit: `ARCHOGEN-PROGRAM-0481 (leaf PROGRAM.10.5)`, with `.10.5`
  Children: `PROGRAM.10.1` … `PROGRAM.10.5` — decomposed `2026-09-30`, in the order the warning above requires: the
  verdict made the right shape first, the policy decided second, the workflow wired last — and a fifth leaf for the
  one leg no machine here can supply, because this repository's CI has not run since `origin/main`'s `32e6b14`.

- ID: `PROGRAM.10.1`
  Status: `done`
  Goal: the emulator step reports what §14.3 has a word for — a **quarantine** with a named issue, owner, affected
  claim and bounded scope — when the one thing missing is the §3.2 agreement evidence `M2.8` owns, and still reports
  a **failure** for a real mismatch (a release other than the pin, a machine the emulator does not offer).
  Acceptance: `scripts/target_emulator.sh --check` separates *could not be run* (exit `20`) from *ran and disagreed*
  (exit `1`); the runner turns exit `20` into an absence only for a step that declares a quarantine carrying all four
  of §14.3's fields, so an undeclared step exiting `20` is still a failure; a quarantine whose step passes is refused
  as stale; `make integration` reads `incomplete`, naming `M2.8`.
  Verification: see the checklist — the script's `--check` split and armed (7 arms, 3 mutations), the runner's
  judgement a tested table with two catalogued mutations, and `make integration` `incomplete` at exit `20`.
  Commit: `ARCHOGEN-PROGRAM-0145 (leaf PROGRAM.10.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — `HEAD`'s own script, run from scratch against the installed QEMU:
    ```text
    $ git show HEAD:scripts/target_emulator.sh > …/head_target_emulator.sh; bash …/head_target_emulator.sh --check
      target-emulator:   leaf M2.8 owns flipping it, with the evidence that justifies it
      HEAD's --check exit=1
    ```
    and the runner maps "ran, nonzero" to `Failed`, so the tier read `failed` — the verdict `COMMIT.md` step 2
    does not let a push proceed past, over a comparison that never ran.
  - [x] **ROOT CAUSE (WHY + WHERE)** — **WHERE (1):** `scripts/target_emulator.sh --check` folded *could not be
    run* into *disagreed*: the `TARGET_VERIFIED` branch set the same code as the three real mismatches.
    ```text
    $ git show HEAD:scripts/target_emulator.sh | grep -n 'rc=1'
      58:      rc=1      (no pin)          62:      rc=1   (release mismatch)
      68:      rc=1      (machine absent)  74:      rc=1   (TARGET_VERIFIED=no — nothing compared)
    $ git grep -c -i "quarantin" HEAD -- xtask/src/main.rs   → no match
    ```
    **WHY:** the runner had words for *unavailable* and *not built* but none for §14.3's third case, so the
    script had no code that meant "ran, and could not reach a verdict", and nothing could accept one on terms.
    **WHERE (2), found on the first run of the fix:** the runner showed a failing step's stderr whenever it was
    non-empty — `git show HEAD:xtask/src/main.rs | grep -n "stderr.trim().is_empty()"` → `464:` — and the
    doctrine driver writes each breach to stdout, so a failing `doctrines` step printed `=== 1 doctrine
    breach(es) — commit blocked ===` and never which doctrine: 32 stdout lines, 1 stderr line, measured.
  - [x] **FIX** — `--check` exits `20` when the only shortfall is `TARGET_VERIFIED=no` (and names the missing
    `DEVICE_TREE_FIXTURE`), `1` for any mismatch, which outranks it, and `1` for `TARGET_VERIFIED=yes` with no
    fixture behind it; `--self-test` runs the verdicts against a stub QEMU. The runner gains `QUARANTINES` —
    step, issue, owner, claim — and a pure `judge()`: exit `20` from a named step is `Quarantined`, from any other
    step a failure with a hint; a named step passing is a stale quarantine, refused. A failing step shows both
    streams' tails (`TAIL_LINES` 24 → 40 per stream, sized on the driver's 32-line report).
    `scripts/run_self_tests.sh` censuses every `scripts/*.sh` carrying `--self-test`, so the new arms, and the
    runner's own, run in the `self-tests` step.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/target_emulator.sh --self-test      → target-emulator self-test: 7 pass / 0 fail (7 arms)
    $ bash scripts/target_emulator.sh --check          → exit=20, "the §3.2 agreement check could not be run"
      M1 unverified outranks a mismatch               → 3 arm(s) refused, restored (cmp)
      M2 unverified back to exit 1 (the old verdict)   → 1 arm refused, restored (cmp)
      M3 verified with no fixture accepted             → 1 arm refused, restored (cmp)
    $ cargo xtask mutate --only failing-step-shows-one-stream undeclared-could-not-run-counted-as-an-absence
      mutate: OK — 2 mutation(s), each killed or surviving exactly as the catalog expects
    $ cargo xtask verify --tier integration
      tier integration: incomplete — 7 passed, 0 failed, 0 unavailable, 0 not built, 1 quarantined   exit=20
    ```
  - [x] **NO REGRESSION** — `cargo test -p xtask` → `test result: ok. 17 passed; 0 failed`; the same tier run's
    `tests`, `doctrines`, `self-tests` (`self-tests: OK — 23 self-test(s) passed`, was 21: the census adds
    `target_emulator.sh` and the runner itself), `book` and `no-std-build` all ✅. A real mismatch still fails:
    arms 2–5 of the script's self-test pin `exit 1`.
  - [x] **LOCKSTEP** — `verification.md` names the third kind of absence and re-renders its transcript from the
    run above; `targets.md` and `docs/targets/first-target.md` carry the two exits (the latter re-rendered from
    a run); `COMMIT.md` step 2, `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`; `decision_push-cadence.md` amended,
    because its deadlock premise was this leaf's defect.

- ID: `PROGRAM.10.2`
  Status: `done`
  Goal: every gate's self-test runs as a CI runner would run it — with no git identity it did not set itself.
  Acceptance: `scripts/run_self_tests.sh` runs the arms with the caller's global git configuration hidden, so an arm
  that commits without its own identity fails here as it would on a runner; every such arm fixed.
  ⚠️ **Premise falsified `2026-09-30`, and recorded rather than implemented around.** The leaf was written on a grep
  (`selftest_spine.sh:121` commits with no `-c user.name`), and the "every such arm fixed" half has nothing to fix:
  the harness's `repo()` helper sets a local identity at line 31. The first half stands — it is the instrument that
  showed it, kept so the next arm that leans on this machine fails here.
  Verification: see the checklist — 23 of 23 self-tests pass as a bare runner; the runner's two new arms, one killed
  by removing the bare environment.
  Commit: `ARCHOGEN-PROGRAM-0146 (leaf PROGRAM.10.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the suspected defect, reproduced under the runner's condition rather than read:
    ```text
    $ env -u GIT_AUTHOR_NAME … GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
        GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=user.useConfigOnly GIT_CONFIG_VALUE_0=true bash scripts/run_self_tests.sh
      self-tests: OK — 23 self-test(s) passed                                      exit=0
    $ … bash scripts/selftest_spine.sh | grep -cE "Author identity unknown|empty ident"   → 0
      spine self-test: 34 pass / 0 fail (34 arms)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — of the *false alarm*: a line read out of its function. `grep -rnE "commit
    (-q|-m)" scripts/` listed `scripts/selftest_spine.sh:121` without `-c user.name`, and `grep -n "user\.name"
    scripts/selftest_spine.sh` → `31:  git -C "$d" config user.name arm` — the `repo()` helper every scratch repository
    is made by. **Of the gap that remains:** nothing ran the arms without this machine's `~/.gitconfig`, so the only
    way to learn whether an arm leaned on it was to push — `git show HEAD:scripts/run_self_tests.sh | grep -c
    GIT_CONFIG` → `0`.
  - [x] **FIX** — `run_self_tests.sh` runs every self-test through `bare()`: `GIT_CONFIG_NOSYSTEM=1`,
    `GIT_CONFIG_GLOBAL=/dev/null`, `user.useConfigOnly=true`, and the four `GIT_AUTHOR_*`/`GIT_COMMITTER_*` variables
    unset. Two arms: a stub that commits with the ambient identity must fail; one that sets its own must pass.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/run_self_tests.sh --self-test   → run-self-tests self-test: 8 pass / 0 fail (8 arms)
    mutation: `bare bash "$gate"` → `bash "$gate"` (applied: cmp differs)
      SELF-TEST: an arm leaning on this machine's git identity fails, as it would on a runner — expected exit 1, got 0
      run-self-tests self-test: 7 pass / 1 fail (8 arms)                           restored (cmp)
    ```
  - [x] **NO REGRESSION** — every self-test, now under the bare environment:
    ```text
    $ bash scripts/run_self_tests.sh; echo "exit=$?"
      ✓ scripts/selftest_spine.sh   20s  spine self-test: 34 pass / 0 fail (34 arms)
      self-tests: OK — 23 self-test(s) passed
      exit=0
    ```
  - [x] **LOCKSTEP** — `TOOLBOX.md`, `DOCTRINE_ENFORCEMENT.md`, `verification.md`; the instance added to
    `docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md`, because it had been announced as found.

- ID: `PROGRAM.10.3`
  Status: `done`
  Goal: the blocking policy for `incomplete` — a recorded, reasoned answer, and a runner mode that implements it.
  Acceptance: a decision record under `docs/decisions/`; the runner can be told the environment is provisioned, so a
  missing tool is a failure of that claim rather than an absence; what remains `incomplete` is only what a leaf owns.
  Verification: see the checklist — `--provisioned` measured both ways on a PATH without QEMU; the policy script's 7
  arms and four mutations; the real job run locally, which found and fixed a log its own arms were clobbering.
  Commit: `ARCHOGEN-PROGRAM-0147 (leaf PROGRAM.10.3)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the question stood open in the workflow, with nothing that could answer it safely:
    ```text
    $ git grep -n "should an \`incomplete\` verdict" HEAD -- .github/workflows/
      HEAD:.github/workflows/rust.yml:23:  # not answered: should an `incomplete` verdict (exit 20 — nothing failed, …
    $ git show HEAD:xtask/src/main.rs | grep -c provisioned   → 0
    $ git ls-files docs/decisions | grep -c blocking           → 0
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — **WHY no answer was safe:** in CI, exit `20` had two sources the runner did not
    tell apart — a gap a leaf owns (the quarantine, a step not built) and a tool the *workflow* failed to install.
    Blocking on `20` makes CI red until `M2.8` lands; not blocking lets a provisioning mistake pass with a warning.
    **WHERE:** `run_step` returned `Outcome::Unavailable` for a missing tool unconditionally — one site, no condition
    on the environment:
    ```text
    $ git show HEAD:xtask/src/main.rs | grep -n "return Outcome::Unavailable"
      506:                    return Outcome::Unavailable;
    ```
  - [x] **FIX** — `--provisioned` (`missing_tool()`: a missing tool is `Failed` there, `Unavailable` elsewhere);
    `scripts/ci_integration.sh` maps `0` pass, `1` fail, `20` pass with a `::warning::` per owned gap and a job
    summary, any other code fail; `docs/decisions/decision_incomplete-blocking-policy.md` records why, and what was
    considered. **Found by running the real job and fixed here:** the script's arms, run by the tier's own
    `self-tests` step, `tee`d into the real job's log and truncated it under the running `tee` — `od -c` showed a
    hole of `\0` bytes, `grep` read the log as binary, and the annotation fell back to "names no gap". The arms now
    keep their own log (`CI_INTEGRATION_LOG`), and an arm checks the real log's `cksum` is unchanged.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ PATH=<qemu hidden> cargo xtask verify --tier integration --provisioned   → provisioned exit=1
        ❌ emulator  UNAVAILABLE on a provisioned environment — `qemu-system-riscv64` is not on PATH
    $ PATH=<qemu hidden> cargo xtask verify --tier integration                 → unprovisioned exit=20
    $ bash scripts/ci_integration.sh --self-test   → ci-integration self-test: 7 pass / 0 fail (7 arms)
      M1 `20` blocks the job               → 3 arm(s) refused, restored (cmp)
      M3 no job summary                    → 3 arm(s) refused, restored (cmp)
      M2 rc from `$?` not PIPESTATUS       → 0 refused: EQUIVALENT under `set -o pipefail`, which already carries
                                             the tier's code; with pipefail removed too → 1 pass / 5 fail
      M4 the arms' own log seam removed    → 6 pass / 1 fail: "the arms wrote target/ci/integration.log"
    $ bash scripts/ci_integration.sh   → exit=0
      ::warning title=integration: incomplete, not a pass::emulator 0.12s QUARANTINED — could not be run; leaf M2.8 owns the gap
      (the log: 0 NUL bytes, 19 lines)
    ```
  - [x] **NO REGRESSION** — `cargo test -q -p xtask` → `test result: ok. 18 passed; 0 failed`;
    `bash scripts/run_self_tests.sh` → `self-tests: OK — 24 self-test(s) passed`, exit=0 (the census adds
    `ci_integration.sh`); without `--provisioned` the local tier is unchanged — `unprovisioned exit=20` above.
  - [x] **LOCKSTEP** — the decision record and its `INDEX.md` row; `verification.md` "What CI does with an
    `incomplete` tier"; `TOOLBOX.md`. The workflow itself is `.10.4`'s.

- ID: `PROGRAM.10.4`
  Status: `done`
  Goal: the `integration` job in `.github/workflows/rust.yml`, provisioning `mdbook` and `qemu-system-riscv64` at
  their pins, verified against a hash, and implementing `PROGRAM.10.3`'s policy.
  Acceptance: the provisioning lives in one script the workflow calls; each download is checked against a recorded
  sha256; a rehearsal on this machine — a fresh clone, no global git configuration — runs the job's commands.
  Verification: see the checklist — the pinned QEMU built here from its verified tarball; the provisioner's 8 arms,
  three mutations and a real digest refused; the job rehearsed from a fresh checkout, passing.
  Commit: `ARCHOGEN-PROGRAM-0148 (leaf PROGRAM.10.4)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — no job ran the tier, and the runner image cannot supply the pinned emulator:
    ```text
    $ git show HEAD:.github/workflows/rust.yml | grep -c "integration"   → 0
    $ curl -sSfL https://packages.ubuntu.com/noble-updates/qemu-system-misc | grep -oE 'qemu-system-misc \([^)]+\)'
      qemu-system-misc (1:8.2.2+ds-0ubuntu1.18)          — the pin is 11.1.1, and --check refuses another release
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — **WHERE:** `.github/workflows/rust.yml` held a comment where the job would be,
    because the tier needed a policy (`.10.3`) and tools no workflow step installed. **WHY an apt install is not the
    fix:** the pin is the contract (§3.2), so the job must build the pinned release. And the ledger's pin population
    could not see a CI tool pin: `git show HEAD:scripts/check_source_ledger.sh | grep -n "git ls-files -- 'targets"`
    → `65:  done < <(git ls-files -- 'targets/*.env')` — pins were looked for in target files only.
  - [x] **FIX** — `scripts/ci_provision.sh`: mdBook from its release asset, QEMU from its release tarball
    (`--target-list=riscv64-softmmu`), every download kept only if its sha256 equals the one in `.github/ci-tools.env`,
    tools in `target/ci/tools/bin` (appended to `GITHUB_PATH` under Actions); a pin that moves past its recorded digest
    is refused. The job: `ubuntu-24.04`, the riscv target, QEMU's build dependencies, `actions/cache@v4` keyed on the
    pin files, then the provisioner and `scripts/ci_integration.sh`. `SOURCE-LEDGER` reads pins from every tracked
    `*.env`, with an arm; the `mdbook`, `qemu` and `github-actions` entries carry the new pins and digests.
    `scripts/ci_rehearse.sh` runs the job's steps on a checkout made as `actions/checkout` makes it.
  - [x] **ADDRESSED (verified)** —
    ```text
    digests: mdbook x86_64-linux 084e4342…1f6d and aarch64-darwin da2f5565…4222 = GitHub's published asset digests;
             qemu-11.1.1.tar.xz 079ffbff…2482 (141888716 bytes), one derivation, signature not checked (no gpg here)
    $ PATH=<venv ninja>:$PATH bash scripts/ci_provision.sh   → exit=0
      ci-provision: mdbook: mdbook v0.5.2 installed
      ci-provision: qemu: QEMU emulator version 11.1.1 installed      (-machine help: virt  RISC-V VirtIO board)
    $ bash scripts/ci_provision.sh --self-test   → ci-provision self-test: 8 pass / 0 fail (8 arms)
      M1 a mismatched download kept → 2 refused · M2 a file already there trusted → 1 · M3 a truncated digest → 1
    a wrong recorded digest for the real mdBook asset → "DIGEST MISMATCH …", exit=1, no mdbook installed
    $ bash scripts/ci_rehearse.sh   → exit=0   (18f55e4: the staged change, 448 tracked files, submodule not initialised)
      ✅ fmt ✅ clippy ✅ tests ✅ doctrines ✅ self-tests ✅ book ✅ no-std-build   ⚠ emulator QUARANTINED
      ::warning title=integration: incomplete, not a pass::emulator 0.11s QUARANTINED — … leaf M2.8 owns the gap
      ci-rehearse: the job would pass
    ```
  - [x] **NO REGRESSION** — the ledger gate, its new arm among the others, and the real ledger:
    ```text
    $ bash scripts/check_source_ledger.sh --self-test   → source-ledger self-test: 21 pass / 0 fail (21 arms)
    $ bash scripts/check_source_ledger.sh               → source-ledger: OK (12 entries; 11 pin(s) …)
    ```
    11 pins, was 7: `MDBOOK_VERSION_PINNED` and the new job's three `uses:` refs. The other two jobs are unchanged.
  - [x] **LOCKSTEP** — `verification.md` (the job, its pins, the rehearsal and its limit), `ledger.md`,
    `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`. ⚠️ Not verified: a run on the runner itself — `.10.5`.

- ID: `PROGRAM.10.5`
  Status: `done` — closed `2026-10-06`: the second run, after `.5.1` and `.5.2`, green on every job
  Goal: read the first real run of the `integration` job, and fix what the runner's userland finds.
  The first run, `2026-10-06`: the push of `32e6b14..a3c0cbd`, `gh run view 37427933725` (`rust`) and `37427933648`
  (`doctrines`). `enforce` passed; `focused` and `integration` failed at one step each, the same one:
  ```text
  $ gh api --allow-escape-sequences repos/{owner}/{repo}/actions/jobs/112151765745/logs
  info: the active toolchain `1.95.0-x86_64-unknown-linux-gnu` has been installed
  info: it's active because: overridden by '<the runner's checkout>/rust-toolchain.toml'
    ❌ tests               49.55s  FAILED
       test result: FAILED. 99 passed; 7 failed; 0 ignored; 0 measured; 0 filtered out; finished in 16.47s
  tier integration: failed — 11 passed, 1 failed, 0 unavailable, 0 not built, 0 quarantined
  ```
  Both assumptions held: rustup installed the pin from `rust-toolchain.toml`, and every commit-pinned action resolved
  (checkout, cache, the QEMU build and the mdBook provision all `success`). The seven failures are two defects, both
  in `xtask`'s tests, neither in the runner's userland: six `catalog_check` tests (`.5.1`) and one `trust` test
  (`.5.2`).
  Acceptance: the run's verdict and log are recorded here — and two assumptions `PROGRAM.30` could not test from
  here: the runner's rustup installs from `rust-toolchain.toml` (rustup ≥ 1.28), and the commit-pinned actions
  resolve. ⚠️ It cannot happen before the next push, which the ruled
  cadence (`decision_push-cadence.md`; `bash scripts/push_cadence.sh` for the distance) decides; until then the job's evidence is a rehearsal
  on macOS, not a run on the runner's GNU userland, and the parent says so rather than closing on it.
  The second run, `2026-10-06`: the push of `a3c0cbd..7ac8b8b`, `.5.1`'s and `.5.2`'s fixes, `gh run view
  37431167419` (`rust`) and `37431167424` (`doctrines`):
  ```text
  doctrines 7ac8b8b success: enforce=success
  rust 7ac8b8b success: integration=success, focused=success
  $ gh api --allow-escape-sequences repos/{owner}/{repo}/actions/jobs/112162118736/logs
    ✅ tests               51.14s  every contract test passes, F28 and the semantic corpus included
  tier integration: passed — 12 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined
  ```
  The runner's GNU userland met every script of the spine and found nothing: `enforce` ran the doctrine gate, and the
  `integration` job's `self-tests` step every gate's arms, on GNU `sed`, `awk` and `find`, both runs. What the first
  run found was in two tests, and a third defect behind them, in the rehearsal that should have found one of them, is
  `PROGRAM.64`.
  Verification: `2026-10-06` — both runs read from their logs; the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0481 (leaf PROGRAM.10.5)`

- ID: `PROGRAM.10.5.1`
  Status: `done` — closed `2026-10-06`; the runner's verdict is `.5`'s
  Goal: the catalog gate reads its pending commit's date in the hook's own environment, as its record's §4 says, and
  the six `catalog_check` tests give their scratch repositories an identity of their own.
  Acceptance: the six tests pass where no git identity is configured outside the scratch repository; a test fails if
  the date's read loses the hook's environment.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the first CI run (`.5`), six of seven failures:
    ```text
    thread 'catalog_check::tests::an_untracked_file_under_catalog_and_a_missing_pin_are_refused' panicked at
    xtask/src/catalog_check.rs:1086:44:
    `git var GIT_COMMITTER_IDENT` failed: Committer identity unknown … fatal: empty ident name (for
    <runner@runnervm8df0l…>) not allowed
    ```
    Reproduced here with no identity outside the scratch repository — `HOME` an empty directory under
    `target/m3121/` whose `.gitconfig` says `user.useConfigOnly = true`: `cargo test -p xtask catalog_check` →
    `test result: FAILED. 2 passed; 6 failed`, each *"fatal: no email was given and auto-detection is disabled"*.
    Corrected `2026-10-06`: this record first said `CARGO_HOME` and `RUSTUP_HOME` were the machine's. They were not
    — bash expanded `CARGO_HOME=$HOME/.cargo` after `HOME=` had been assigned, so rustup installed the pinned 1.95.0
    into the empty home (2.8 GB, since deleted), as the runner does. Every run below used that same pin.
  - [x] **ROOT CAUSE (WHY + WHERE)** — two, one under the other. WHERE, the code: `pending_date` in
    `xtask/src/catalog_check.rs` ran `git var GIT_COMMITTER_IDENT` through `Git`, whose `command` calls
    `env_clear()` and passes back the history readers' allowlist alone (`PATH`, `HOME`, `GIT_DIR`,
    `GIT_INDEX_FILE`, `GIT_WORK_TREE`). The catalog record's §4 says otherwise: the date is read *"in the hook's own
    environment rather than the history readers' allowlist, since it reads no history and needs `TZ` and
    `GIT_COMMITTER_DATE`"* (`docs/specs/catalog/decision_catalog-records.md`, the gate's pending commit). So a
    committer's `GIT_COMMITTER_DATE`, `TZ` or identity set in the environment never reached `git var`. WHERE, the
    tests: the scratch repository's identity was `-c user.name=t -c user.email=t@t` on the test's own git calls only,
    so the checker's `git var` found an identity only in the machine's `~/.gitconfig` — here, and not on the runner.
    WHY it stayed hidden: every run before this one was on a machine with a global identity.
    ```text
    $ git grep -n -e 'env_clear' -e 'fn pending_date' -e 'git.text(&["var"' HEAD -- xtask/src/catalog_check.rs
    HEAD:xtask/src/catalog_check.rs:170:            .env_clear();
    HEAD:xtask/src/catalog_check.rs:424:fn pending_date(git: &Git) -> Result<CommitterDate, String> {
    HEAD:xtask/src/catalog_check.rs:425:    let ident = git.text(&["var", "GIT_COMMITTER_IDENT"])?;
    $ git grep -n 'own environment rather' HEAD -- docs/specs/catalog/decision_catalog-records.md
    HEAD:docs/specs/catalog/decision_catalog-records.md:323:    are those `git var GIT_COMMITTER_IDENT` gives when …
    ```
  - [x] **FIX** — `pending_date(cwd, env)` runs `git var` with `env` and nothing else, and `judge` gives it
    `std::env::vars_os()`, the hook's environment; the module's header says which calls the allowlist governs.
    `Repo::new` writes `user.name`, `user.email` and `commit.gpgsign` into the scratch repository's own
    configuration, where every git run in it reads them, and the `-c` flags go. A new test,
    `the_pending_date_is_read_in_the_hook_s_environment`, gives `git var` an environment of `PATH` and
    `GIT_COMMITTER_DATE=@1700000000 +0130`; the mutation `catalog-pending-date-under-the-allowlist` puts the
    allowlist back.
  - [x] **ADDRESSED (verified)** — with no identity outside the scratch repository, as above: `cargo test -p xtask
    catalog_check` → `test result: ok. 9 passed; 0 failed`; in the ordinary environment, `ok. 9 passed`. `cargo xtask
    mutate --only catalog-pending-date-under-the-allowlist` → *"killed by
    catalog_check::tests::the_pending_date_is_read_in_the_hook_s_environment (2.7s)"*, the pre-fix behaviour failing
    the new test.
  - [x] **NO REGRESSION** — the whole suite with no identity outside the scratch repositories: `cargo test --all -q`
    → 97 suites, 1274 passed, 0 failed; `cargo clippy -p xtask --all-targets -- -D warnings` → clean; `make focused`
    → `tier focused: passed — 3 passed, 0 failed`; the doctrine gate at commit. ⚠️ Not verified here: the runner
    itself, which `.5` reads after the push.
  - [x] **LOCKSTEP** — the code now does what §4 already said, so neither the record nor the book's catalog chapter
    moves; `xtask/mutations.txt`; this leaf and both logs; `CHANGELOG.md`.
  Verification: `2026-10-06` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0478 (leaf PROGRAM.10.5.1)`

- ID: `PROGRAM.10.5.2`
  Status: `done` — closed `2026-10-06`; the runner's verdict is `.5`'s
  Goal: the `.incbin` fixture of `trust`'s test `an_incbin_under_asm_and_under_naked_asm_is_refused` assembles on an
  ELF host as it does on Mach-O.
  Acceptance: both of the test's packages build for an ELF target; the blob's bytes are in what they build.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the first CI run (`.5`), the seventh failure:
    ```text
    thread 'trust::tests::an_incbin_under_asm_and_under_naked_asm_is_refused' panicked at xtask/src/trust.rs:2088:84:
    called `Result::unwrap()` on an `Err` value: "`cargo build --lib --release --locked --offline
    --no-default-features --message-format=json -v -p b` failed: error: could not compile `b` (lib)"
    ```
    Reproduced here on the one ELF target installed, `riscv64imac-unknown-none-elf`, with the test's two sources in
    a `#![no_std]` package under `target/m3121/elf-repro/` (no `x86_64-unknown-linux-gnu` standard library is
    installed): the `naked_asm!` package → `rustc-LLVM ERROR: Size expression must be absolute.` and `could not
    compile`; the `asm!` package builds; both build for the host, `aarch64-apple-darwin`, a Mach-O target.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the fixture's assembly, `xtask/src/trust.rs`, ended `.data`, `.incbin`,
    then `.text`, naming the section to return to. WHY ELF refuses it: there a function is emitted in a section of its
    own, `.text.<symbol>`, and a naked function's assembly is followed by `.size f, . - f`. After `.text` the location
    counter is in another section, so the size is a difference across sections, which the assembler cannot make
    absolute. Mach-O keeps every function in `__TEXT,__text` and writes no `.size`, so the same text assembled here
    and every earlier run was on Mach-O. `.previous` returns to whatever section was current, on both formats.
    ```text
    $ git grep -n -e 'incbin \\\"crates/b/blob.bin\\\"\\n.text' HEAD -- xtask/src/trust.rs
    HEAD:xtask/src/trust.rs:3199:   … core::arch::asm!(\".data\\n.incbin \\\"crates/b/blob.bin\\\"\\n.text\") …
    HEAD:xtask/src/trust.rs:3203:   … core::arch::naked_asm!(\".data\\n.incbin \\\"crates/b/blob.bin\\\"\\n.text\\nret\") …
    ```
  - [x] **FIX** — both packages end their `.incbin` with `.previous`, and a comment says why. No other fixture
    returns from a section switch: `git grep -n '\\\\n\.text' -- xtask crates` finds these two lines alone; the
    `global_asm!` fixtures leave `.data` current at the end of their module, which no `.size` measures.
  - [x] **ADDRESSED (verified)** — the two sources with `.previous` in the ELF reproduction: `asm
    riscv64imac-unknown-none-elf rc=0`, `naked riscv64imac-unknown-none-elf rc=0`, and for the host `rc=0` twice;
    the blob's bytes in each `.rlib` built (`grep -a -o SECRET-BYTES` → 1 in each). `cargo test -p xtask
    an_incbin_under_asm_and_under_naked_asm_is_refused` → `test result: ok. 1 passed`. ⚠️ Not verified here: an
    x86-64 ELF build, which is the runner's — `.5` reads it after the push.
  - [x] **NO REGRESSION** — `cargo test --all -q` with no identity outside the scratch repositories → 97 suites, 1274
    passed, 0 failed; `make focused` → `tier focused: passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — a test fixture alone, so no chapter or record moves; this leaf and both logs; `CHANGELOG.md`.
  Verification: `2026-10-06` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0479 (leaf PROGRAM.10.5.2)`
