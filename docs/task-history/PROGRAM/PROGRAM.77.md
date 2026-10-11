- ID: `PROGRAM.77`
  Status: `done` — filed, started and closed `2026-10-10`, on CI's red: the push `a543d10` failed the `rust` workflow;
  the fix committed (`ARCHOGEN-PROGRAM-0569`) and pushed, CI green on the runner
  Goal: the mutation runner's group kill reaches its own process group and nothing else, on every host, so the test
  suite cannot end the CI runner it runs on.
  Reproduce / issue: run `38064491086` of `a543d10`, both jobs — `focused` and `integration` — failed in the tier's
  step after about 45 minutes, the step left `in_progress`, each job's annotation *"The hosted runner lost
  communication with the server. Anything in your workflow that terminates the runner process, starves it for
  CPU/Memory, or blocks its network access can cause this error."*, and no log kept (`gh api …/jobs/114249268009/logs`
  → *"BlobNotFound"*). The run before, `37431167419` of `7ac8b8b`, passed in 7 minutes; the same tests here, `cargo
  test --all` on `aarch64-apple-darwin`, pass in 37 s, at most 732 processes, every test process together under 800 MB
  (`target/ci-a543d10/measure.sh`, untracked).
  Direction: the one test new since `7ac8b8b` that signals a process group — `PROGRAM.67`'s
  `a_run_past_its_limit_is_stopped_with_everything_it_started`, through `run_bounded` — and how the `kill` program
  of the runner's image reads its arguments.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the run above; `git grep -c 'Command::new("kill")' HEAD -- xtask/src crates` →
    `HEAD:xtask/src/mutation.rs:2`, none at `7ac8b8b`, both from `b87adbc` (`git log -S`); `run_bounded` ran `kill
    -KILL -<child's pid>`. Not reproduced on Linux: no host here is one (`M3.6.3.2.1`'s Why blocked), so the leg that
    falsifies the cause is the next CI run, stated rather than hidden.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `xtask/src/mutation.rs`'s `run_bounded`, the external `kill` with a
    negative operand and no `--`. WHY: `/usr/bin/kill` on Ubuntu 24.04 is procps-ng's (`packages.ubuntu.com/noble/
    amd64/procps/filelist` → *"/usr/bin/kill"*), whose `src/kill.c` at `v4.0.4` parses with *"getopt_long(argc, argv,
    "l::Ls:hVq:", longopts, &optindex)"* and, for an unknown digit, *"/* Special case for signal digit negative PIDs
    */ pid = (long)('0' - optopt);"* — `-1234` is signalled as pid `-1`, every process the user may signal, the
    runner's agent among them; a group starting 2 to 9 as that group. POSIX's own text (Issue 8, `utilities/kill`):
    *"If the first pid operand is negative, it should be preceded by "--" to keep it from being interpreted as an
    option."* macOS's `kill` reads it as a pid, so the test passed here.
  - [x] **FIX** — `signal`, kill(2) called directly, `0` and `-1` refused before the call; `run_bounded` signals the
    negated group through it, and the test's liveness check waits for the reaping through it, up to 10 s; two tests —
    the refusals and a failed call for a group nobody has, and no Rust source under `xtask/src` or `crates` running
    the `kill` program; the catalogue's `PROGRAM.67` entry moved to the call, two entries for the refusals; a
    knowledge card, `a-negative-pid-is-an-option-to-the-kill-program.md`; the verification chapter's sentence.
  - [x] **ADDRESSED (verified)** — `cargo test -p xtask mutation` → `test result: ok. 12 passed`; `cargo xtask mutate
    --only mutate-a-hang-kills-the-command-alone mutate-a-signal-to-every-process-is-sent
    mutate-a-signal-to-the-own-group-is-sent mutate-a-hang-is-waited-on` → *"mutate: OK — 4 mutation(s), each killed
    or surviving exactly as the catalog expects"*; the scan test fails on `HEAD`'s source, the grep above its count.
  - [x] **NO REGRESSION** — `cargo test -p xtask` → `test result: ok. 183 passed`; clippy at deny-warnings, clean;
    `make focused` → `passed — 3 passed, 0 failed`, on the staged tree; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the verification chapter; the knowledge card and its index row; this leaf, the frontier and
    both logs; `docs/TASK_TREE.md`, `LIVE_STATUS.md`, `MEMORY.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's rows; on the runner, the leg this machine could not run: run
  `38069871798` of `8cdac11`, the push that restored CI — `focused` passed in 1 min 55 s, *"tier focused: passed — 3
  passed, 0 failed"*, `integration` in 3 min 45 s, where both had been lost at about 45 minutes; `doctrines` and
  `trust-gate` passed beside it
  Commit: `ARCHOGEN-PROGRAM-0569 (leaf PROGRAM.77)`; `ARCHOGEN-PROGRAM-0573 (leaf PROGRAM.77)`, closed on the runner
