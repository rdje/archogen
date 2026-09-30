- ID: `PROGRAM.28`
  Status: `done`
  Goal: every gate's `--self-test` is run by a tier, so an arm that breaks — or starts passing for the wrong
  reason — is seen by the next run and not by the next person who happens to invoke it.
  Reproduce / issue: measured `2026-09-29` by `PROGRAM.27`, which ran them all by hand because nothing else
  does.
  ```text
  census: for s in scripts/check_*.sh knowledge-map/scripts/check_knowledge_map.sh; do
            grep -q -- '--self-test' "$s" && bash "$s" --self-test; done
          -> 10 gates carry a --self-test, all 10 pass today
  census: git grep -n "self-test" -- xtask .github Makefile scripts/check_doctrines.sh
          -> no match: no tier (`make focused`, `make integration`), no CI workflow and not the doctrine
             driver runs a single arm
  ```
  Impact: an arm is re-fired only when a person runs it. `PROGRAM.27`'s two vacuous arms show the
  consequence is not hypothetical in kind — they were wrong from the commit that wrote them and every
  commit after was green — though running them in a tier would not have caught *vacuity*, only breakage;
  that half is what `PROGRAM.27`'s subject-naming oracle is for. `PROGRAM.18` gives the ten gates without
  arms their arms; this leaf makes all of them run.
  Acceptance: an `integration`-tier step (and the CI doctrine workflow) that discovers every script
  carrying `--self-test` by census rather than by list — **and `scripts/selftest_spine.sh`**, which arms the
  scaffold-owned gates from outside (`PROGRAM.18.2`, ~30 s) — runs each, and fails on any arm that fails — with
  a RED arm of its own proving a failing arm is reported; `make tiers` lists it; the book's
  `verification.md` says what the step proves.
  Priority: **medium** — nothing is failing today; it is the difference between an arm that was checked
  once and one that is checked. Sequenced after `PROGRAM.18`, whose new arms it would then also run.
  Verification: see the checklist — a runner that discovers 14 self-tests by census, a tier step, a CI step,
  six arms of its own and four mutations.
  Commit: `ARCHOGEN-PROGRAM-0123 (leaf PROGRAM.28)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — nothing ran any gate's arms:
    ```text
    $ git grep -n "self-test" 96636ac -- xtask .github Makefile scripts/check_doctrines.sh   -> no match, rc=1
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — each arm was written as a script flag and wired to nothing; the tier
    table in `xtask/src/main.rs` had no step for them and the CI workflow ran only the driver:
    ```text
    $ git grep -n "name: \"doctrines\"\|run: scripts/" 96636ac -- xtask/src/main.rs .github/workflows/doctrines.yml
      96636ac:xtask/src/main.rs:173:    name: "doctrines",
      96636ac:.github/workflows/doctrines.yml:18:        run: scripts/check_doctrines.sh
    ```
  - [x] **FIX** — `scripts/run_self_tests.sh`: the population is **discovered** — every `scripts/check_*.sh`
    and `knowledge-map/scripts/check_*.sh` that handles `--self-test`, plus `scripts/selftest_spine.sh` — so a
    gate armed tomorrow is run the day it is armed; an empty population exits 2, never 0. A `self-tests` step
    in the `integration` tier (after `doctrines`), and a step in `.github/workflows/doctrines.yml`.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/run_self_tests.sh
      self-tests: OK — 14 self-test(s) passed                                  real 0m41.6s
    $ cargo xtask verify --tier integration
      ✅ self-tests          33.87s  every doctrine gate's RED arms still fire …
      tier integration: failed — 7 passed, 1 failed, 0 unavailable, 0 not built   (the emulator, M2.8)
    $ bash scripts/run_self_tests.sh --self-test
      run-self-tests self-test: 6 pass / 0 fail (6 arms)
    ```
    Four mutations, restored by `cmp`: **U-1** a failed self-test not counted → 4 / 2; **U-2** the outside
    harness left out → 5 / 1; **U-3** nothing to run reported as a pass → 5 / 1; **U-4** every script run,
    armed or not → 4 / 2. ⚠️ Honest limit: the CI step is written and cannot be run from here; it runs the
    same script the tier does.
  - [x] **NO REGRESSION** — `cargo test -q -p xtask` → `test result: ok. 8 passed; 0 failed`; every other
    `integration` step green, the emulator's failure pre-existing and owned by `M2.8`; `cargo fmt --check`
    clean; the doctrine driver green at the commit.
  - [x] **LOCKSTEP** — the book's `verification.md`: its "What that looks like today" transcript was
    **stale** (no `no-std-build` step, the emulator shown *unavailable* after QEMU was pinned) and is
    re-rendered from this run, with the tier table and the "incomplete tiers" count corrected to the
    measured three; `DOCTRINE_ENFORCEMENT.md`, `TOOLBOX.md`, the live docs.
