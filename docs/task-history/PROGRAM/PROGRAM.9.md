- ID: `PROGRAM.9`
  Status: `done`
  Goal: build what the **extended** tier declares but cannot run — a fuzz corpus over the reader
  and the checked arithmetic, a repeatable mutation harness, and Miri wiring (§13.3).
  Acceptance: `cargo xtask verify --tier extended` reports `passed` on a machine with the tools
  installed, and each step fails on a seeded defect; the mutation harness reproduces, by command,
  at least one blind spot that was found by hand (the `lcm`/`max` case in `S0.4` is the
  worked example).

  ⛔ **Opened by `PROGRAM.3`, which is the point of that leaf.** These three gaps existed before
  the runner and were invisible; the runner makes the extended tier report `incomplete` until
  they are closed, so the absence is a routed item rather than a silence.
  Verification: closed `2026-09-30` by its three children. Its own acceptance, each part measured: `cargo xtask
  verify --tier extended` → **`tier extended: passed — 3 passed, 0 failed`** on this machine (fuzz 3.75 s, mutation
  8.00 s, miri 996.69 s); each step fails on a seeded defect (the Miri arm's dangling read, the fuzz harness's
  six arms and F-A–F-D, the mutation catalog's own H-1–H-4); the `lcm`→`max` blind spot is reproduced by
  `cargo xtask mutate --only lcm-to-max-against-the-harmonic-corpus`. The fuzz harness also found three
  engine defects before its own commit (`M1.34`–`M1.36`).
  Commit: `ARCHOGEN-PROGRAM-0127` (`.9.1`), `ARCHOGEN-PROGRAM-0131` (`.9.2`), `ARCHOGEN-PROGRAM-0132` (`.9.3`)
  Children: `PROGRAM.9.1`, `PROGRAM.9.2`, `PROGRAM.9.3` — decomposed `2026-09-29`: three different tools, one
  commit each. Each step **arms itself**: before its real run it proves, on a seeded defect in scratch under
  `target/`, that it can still fail — so `passed` from this tier always means "an instrument that can fail did
  not", never "an instrument ran". ⛔ Found while decomposing: the `miri` step reports **UNAVAILABLE** on a
  machine where Miri **is** installed (`cargo +nightly miri --version` → `miri 0.1.0 (809936eac6
  2026-09-12)`), because its probe runs `cargo miri --version` under this repository's pinned `stable`
  toolchain, which has no Miri — the probe asks the wrong question, so the tier misreports its environment.

- ID: `PROGRAM.9.1`
  Status: `done`
  Goal: the `miri` step finds Miri where it lives (the `nightly` toolchain), runs it over every crate whose
  tests Miri can execute, and proves each run that it still reports undefined behaviour.
  Acceptance: the step's probe asks the nightly toolchain for its `miri` component and says which toolchain
  it asked; the population is measured, not guessed — per crate, what Miri runs and what stops it;
  a seeded use of undefined behaviour in a scratch crate is refused by the same wiring before the real
  run; the `miri` ledger entry records the adopted version.
  Verification: see the checklist — every test target timed under Miri alone, the step end to end through the
  tier, three falsifications and a probe test with its mutation.
  Commit: `ARCHOGEN-PROGRAM-0127 (leaf PROGRAM.9.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — on a machine where Miri is installed:
    ```text
    $ cargo xtask verify --tier extended      (at 39aa2fc)
      ⚠  miri               UNAVAILABLE — `cargo-miri` is not on PATH                    exit=20
    $ cargo +nightly miri --version
      miri 0.1.0 (809936eac6 2026-09-12)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the probe runs `cargo miri --version` from the repository, where
    `rust-toolchain.toml` selects `stable`, which has no Miri component. The question was wrong, so its true
    answer was misread:
    ```text
    $ git grep -n 'requires: Some("cargo-miri")\|if let Some(sub) = tool.strip_prefix("cargo-")' 39aa2fc -- xtask/src/main.rs
      39aa2fc:xtask/src/main.rs:297:                    requires: Some("cargo-miri"),
      39aa2fc:xtask/src/main.rs:380:    if let Some(sub) = tool.strip_prefix("cargo-") {
    $ git grep -n channel 39aa2fc -- rust-toolchain.toml
      39aa2fc:rust-toolchain.toml:2:channel = "stable"
    ```
  - [x] **FIX** — a probe form `component:<toolchain>:<component>` that asks `rustup +<toolchain>` for the
    component and names both when it is missing; a test refusing any requirement the probe does not understand.
    The step now runs `scripts/extended_miri.sh`: (1) Miri's sysroot under `target/miri-sysroot` via
    `MIRI_SYSROOT` + `cargo miri setup` — by default it was `~/Library/Caches/org.rust-lang.miri`, off this
    volume; (2) **the arm** — a scratch crate under `target/` whose one test reads through a dangling pointer
    must be refused with "Undefined Behavior", or the step fails; (3) every workspace member and every test
    target, derived, minus five test targets left out on **measured** cost (> 300 s each under Miri), `--all`
    to include them; a stale exclusion is refused before the run. The four tests that run cargo as a child
    process carry `#[cfg_attr(miri, ignore = …)]` where they are written. ⛔ The first population excluded
    three whole crates on a story; measured, two of the three were wrong.
  - [x] **ADDRESSED (verified)** —
    ```text
    per target, alone, 600 s cap: 34 targets — 541 passed, 4 ignored, 0 failed; over 300 s: archogen-cli/
      module_cases 463 s, eadl-front/corpus 397 s, eadl-model/rendering 473 s; not finished in 600 s:
      eadl-front/conformance (twice), eadl-front/reference
    $ cargo xtask verify --tier extended
      ✅ miri               851.13s  Miri still refuses a seeded dangling-pointer read, …
      tier extended: incomplete — 1 passed, 0 failed, 0 unavailable, 2 not built          exit=20
    ```
    Falsified, each restored by `cmp`: **F-1** the arm without undefined behaviour (the vector kept alive) →
    `EXTENDED-MIRI: the seeded dangling-pointer read was NOT refused`, exit=1; **F-2** a stale exclusion
    `eadl-model/no_such_target` → refused before the run, exit=1; **F-3** the probe asking for
    `no-such-component` → `UNAVAILABLE — the \`no-such-component\` component is not installed for the
    \`nightly\` toolchain`, exit=20. The probe-form test, mutated to `component:miri`, fails:
    `test result: FAILED. 8 passed; 1 failed`. After the run, nothing in the home cache newer than the script.
  - [x] **NO REGRESSION** — `cargo test -q -p xtask` → `test result: ok. 9 passed; 0 failed`; the
    `cfg_attr(miri, …)` ignores change nothing outside Miri (`make focused` passed); the doctrine driver green
    at the commit. ⚠️ Honest limit: `--all` has not been observed to finish — two of its targets were stopped at
    600 s, twice for one of them.
  - [x] **LOCKSTEP** — the book's `verification.md` gains "The `miri` step proves it can fail before it passes";
    the ledger's `miri` entry records the adopted version and scope; `TOOLBOX.md` row; the not-built steps now
    name `PROGRAM.9.2` and `PROGRAM.9.3`; lesson promoted into
    `docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md`.

- ID: `PROGRAM.9.2`
  Status: `done`
  Goal: a fuzz corpus over the reader and the checked arithmetic (§13.3), within the zero-dependency
  decision — a deterministic, seeded generator in-tree rather than `cargo-fuzz` and `libfuzzer-sys`.
  Acceptance: a `fuzz` step that runs a fixed number of seeded cases per property, prints the seed of any
  failure so it replays by command, and fails on a seeded defect before its real run.
  Verification: see the checklist — the harness, its arms, four seeded defects and a generator attack, the step
  on two seeds, the tier end to end.
  Commit: `ARCHOGEN-PROGRAM-0131 (leaf PROGRAM.9.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the `fuzz` step was a declared absence:
    ```text
    $ cargo xtask verify --tier extended      (at 39aa2fc)
      ⚠  fuzz               NOT BUILT — tracked by leaf PROGRAM.9                            exit=20
    $ git grep -ln "fuzz" 104ccb1 -- 'crates/*/tests/*.rs'     -> no match, rc=1
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — §13.3 names fuzzing, and the zero-dependency decision rules out
    `cargo-fuzz` (`libfuzzer-sys`, nightly), so nothing had been built:
    ```text
    $ git grep -n "no fuzz target exists yet" 39aa2fc -- xtask/src/main.rs
      39aa2fc:xtask/src/main.rs:276:                    note: "no fuzz target exists yet; §13.3 asks for property tests and fuzzing \
    ```
  - [x] **FIX** — `crates/eadl-model/tests/fuzz.rs`: a seeded xorshift generator, one generator per
    `(seed, property, case)` so one case replays alone; **six arms** (known-false claims the generator must
    refute — errors happen; multi-byte text; an escaped multi-byte character; a structured document that reads
    cleanly; an overflowing multiply; a pair with both cross products beyond `i128`) and **eight properties**
    over the reader and `Rational`; `fuzz_smoke` (400 cases) in the ordinary suite, `fuzz_extended` (ignored)
    for the step. `scripts/extended_fuzz.sh` runs the fixed seed and a fresh one, 100 000 cases each, with
    `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true`; the `fuzz` step runs it.
  - [x] **ADDRESSED (verified)** — designing and first running the harness found **three engine defects**, each
    fixed in its own commit before this one: `M1.34` (a saturating comparison: a deadline longer than its period
    admitted), `M1.35` (the reader crashed on `"a\\éb"`, exit 101), `M1.36` (`to_exact_string`'s power of ten
    overflowed — found on the first run, every seed). On the fixed code:
    ```text
    $ bash scripts/extended_fuzz.sh
      extended-fuzz: OK — the fixed seed and seed 1790716945, 100000 case(s) per property, overflow checks on   exit=0  (2.6 s)
    $ FUZZ_SEED={1..5} … fuzz_extended (overflow checks)  -> test result: ok. 1 passed, five times
    ```
    Seeded defects, each restored (tracked files by `git checkout`, matching HEAD): **F-A** the saturating
    comparison → exit=1, `ordering-agrees-with-equality-and-is-transitive failed at case 1`; **F-B** the
    byte-wise escape → exit=1, `reader-never-panics-and-every-span-is-on-a-boundary failed at case 2: panicked:
    start byte index 40 is not a char boundary; it is inside 'é'`; **F-C** the unchecked power of ten → exit=1,
    `checked-arithmetic-never-panics…` and `exact-text-reads-back-to-the-value`, and **with overflow checks
    off** `exact-text-reads-back-to-the-value failed at case 26: … printed as 0.000…` — the wrong value caught
    without a panic; **F-D** an ASCII-only generator → exit=1, arms `no-text-contains-a-multi-byte-character`
    and `no-text-escapes-a-multi-byte-character` never refuted. ⚠️ F-D targeted the untracked harness: the loop's
    `git diff` check called it "did not apply" and skipped the restore; found by grep, reverted by exact inverse
    replacement, smoke `test result: ok` after — recorded as a lesson.
  - [x] **NO REGRESSION** — `cargo test -q -p xtask` → `test result: ok. 9 passed`; the smoke test adds 0.01 s to
    the suite; the doctrine driver green at the commit. The tier end to end:
    ```text
    $ cargo xtask verify --tier extended
      ✅ fuzz                 3.53s  the reader and the exact arithmetic hold eight properties …
      ✅ miri               916.73s  Miri still refuses a seeded dangling-pointer read, …   (the new test target in scope)
      tier extended: incomplete — 2 passed, 0 failed, 0 unavailable, 1 not built        exit=20  (mutation: PROGRAM.9.3)
    ```
    After that run, clippy's `explicit_auto_deref` required one type annotation in the harness
    (`rng.pick::<&str>`), which changes no behaviour; `scripts/extended_fuzz.sh` re-run after it → `extended-fuzz:
    OK`, exit=0, and `make focused` passed.
  - [x] **LOCKSTEP** — the book's `verification.md` gains "The `fuzz` step, and what it found on its first run"
    with the three defects; `TOOLBOX.md` row; lessons promoted into `verify-the-mutation-applied.md` and
    `a-gate-is-only-as-sharp-as-its-fixtures.md`.

- ID: `PROGRAM.9.3`
  Status: `done`
  Goal: the mutation controls that each leaf ran by hand become a repeatable harness — a catalog of
  mutations, each applied, verified to have applied, run against the suite and restored.
  Acceptance: a `mutation` step over the catalog that fails on a surviving mutation; it reproduces by
  command the `lcm`→`max` blind spot found by hand in `S0.4` (surviving against the original harmonic
  fixtures, killed by the discriminating one); restoration checked byte for byte.
  Verification: see the checklist — the catalog run twice, four harness arms, the tier end to end.
  Commit: `ARCHOGEN-PROGRAM-0132 (leaf PROGRAM.9.3)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — mutation controls existed only as hand-run evidence inside checklists; no command
    re-ran any of them:
    ```text
    $ git grep -n 'owner: "PROGRAM.9.3"' 0274f41 -- xtask/src/main.rs
      0274f41:xtask/src/main.rs:285:                    owner: "PROGRAM.9.3",          (the step: NOT BUILT)
    $ git grep -c "restored by \`cmp\`\|mutation" 0274f41 -- docs/tasks/M1.md docs/tasks/PROGRAM.md
      0274f41:docs/tasks/M1.md:129     0274f41:docs/tasks/PROGRAM.md:57       (hand-run controls, recorded once)
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — a control run once proves the test caught the defect *then*; the next
    change can make it stop, and nothing asked again. The evidence lived in prose, not in a runnable catalog —
    the step's own note said so:
    ```text
    $ git grep -n "mutation controls are run by hand" 0274f41 -- xtask/src/main.rs
      0274f41:xtask/src/main.rs:286:                    note: "mutation controls are run by hand, per leaf, and recorded in each \
    ```
  - [x] **FIX** — `xtask/src/mutation.rs` (`cargo xtask mutate`, `--only <id>`) over `xtask/mutations.txt`: each
    entry an exact text that must occur once, a replacement, the `cargo test` arguments and `expect
    killed|survives`. Checked at every step: the text occurs once; the mutated file is read back; a mutation that
    does not compile is a **broken entry**, never a kill; a kill **names the failing tests**; the original is
    restored and read back byte for byte, by a `Drop` guard on panic, and a sentinel under `target/` stops the
    next run after an interrupted one. Eight entries: the `lcm`→`max` pair (killed by the full oracle; **surviving**
    the harmonic corpus — S0.4's blind spot, reproduced by command), `M1.34`–`M1.36`, the scheduler's priority
    scan, the response-time ceiling, and §3.1's D ≤ T. The `mutation` step runs it.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo xtask mutate
      ✓ lcm-to-max        killed by f28_a_non_harmonic_task_set_separates_the_hyperperiod_from_the_longest_period (2.1s)
      ✓ lcm-to-max-against-the-harmonic-corpus     survived (1.4s) — … s0_oracle -- --skip non_harmonic
      ✓ saturating-comparison   killed by rational::tests::ordering_agrees_with_equality_on_values_near_the_limits, …
      ✓ byte-wise-escape        killed by reader::tests::an_unknown_escape_before_a_multibyte_character_is_reported_not_a_panic
      ✓ unchecked-power-of-ten  killed by rational::tests::exact_text_survives_a_power_of_ten_beyond_i128
      … (8 of 8)
      mutate: OK — 8 mutation(s), each killed or surviving exactly as the catalog expects        exit=0  (8 s)
    ```
    Harness arms, each through a temporary catalog entry, the catalog restored by `cmp`: **H-1** a text not in
    the file → `occurs 0 time(s)`, exit=2; **H-2** a mutation that does not compile → `a broken catalog entry,
    not a kill`, exit=2; **H-3** an equivalent mutant expected killed → `1 of 1 did not do what the catalog
    expects`, exit=1; **H-4** a leftover sentinel → `an earlier run was interrupted`, refused. After every arm,
    `git status --short -- crates/` → empty. ⚠️ Honest limit: the `Drop` restore on a panic is by construction,
    not exercised; a killed process is what the sentinel covers.
  - [x] **NO REGRESSION** — `cargo test -q -p xtask` → `test result: ok. 14 passed` (5 new: the catalog parser,
    its refusals, indentation, the once-only rule, the failing-test list); the tier end to end:
    ```text
    $ cargo xtask verify --tier extended
      ✅ fuzz                 3.75s   ✅ mutation             8.00s   ✅ miri               996.69s
      tier extended: passed — 3 passed, 0 failed, 0 unavailable, 0 not built                exit=0
    ```
  - [x] **LOCKSTEP** — `verification.md` gains "The `mutation` step: each defect, put back, must still be caught",
    and its "three of the five tiers are incomplete" is now two, with the date `extended` first passed;
    `COMMIT.md`'s tier status said `extended` was incomplete everywhere and is corrected; `TOOLBOX.md` row.
