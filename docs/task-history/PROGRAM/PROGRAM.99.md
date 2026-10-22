- ID: `PROGRAM.99`
  Status: `done` — P1: the `xtask` suite red under host load: deadlines that read slowness as a hang
  Reproduce / issue: `2026-10-21`, `make focused` on `M3.3.1`'s staged tree — its first run passed every test in 240 s;
    the next two, on the same code formatted, failed 73 and then 31 `xtask` tests in 2731 s and 2168 s, beside other
    projects' builds; three failing tests passed alone. Impact: the gate a commit cites is red for reasons outside the
    commit, so it either blocks product work or is waived; a shared CI runner can meet the same load. The tier runner
    kept only the last 40 lines of the failed step, so no failure message survived (owned: `PROGRAM.100`).
  Goal: first, the focused tier on a quiet host for `M3.3.1` and `M3.3.2`, recorded there; then, tools first, which limit
    fires and why, repaired with a control failing first.
  Acceptance:
    - [x] ROOT CAUSE — `make focused` at `eaf71286`, `2026-10-22`, host quieter (syspolicyd mean 17.5 % over 19
      samples): exit code 0, `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`,
      184 s, recorded in `M3.3.1` and `M3.3.2`. WIDEN — `ARCHOGEN_PARENT_TEST_TIMINGS=1 cargo test -p xtask --
      --nocapture`: `test result: ok. 338 passed; 0 failed; 2 ignored`, 162.50 s; the 200 bounded children median 1.49 s,
      p90 7.03 s, slowest 11.78 s, 34 of them at 5 s or more, against `const LIMIT: Duration = Duration::from_secs(30)`
      (`xtask/src/trust_test_process.rs:11`), set at 6.23 s measured (`M3.6.9.6.3.2.1.2`). Of `2026-10-21`'s 31 named
      failures 28 are bounded children; the other three, and `trust_test_process`'s own 10 s probe control, carry
      deadlines of their own. A census of every wall-clock limit in `xtask` (24 sites) classed each as the side that
      must finish or the side that must be stopped. NARROW — `scripts/exec_pressure.sh` (new) re-creates a loaded
      host; its probe refused 12 and 24 workers (×1.0, ×1.6 — exit code 1) and accepted 64: `quiet 174804 µs, under 64
      workers 685380 µs (×3.9); syspolicyd 65.8 % CPU`, exit code 0. PINPOINT, failing first — under 64 workers
      `test result: FAILED. 335 passed; 3 failed`, each `bounded test command: test process group exceeded its limit`
      at `trust_test_process.rs:56`, the three slowest quiet children, no assertion of theirs failed; under 160 workers
      the timing controls alone `test result: FAILED. 9 passed; 2 failed`: `the unmutated copy does not pass (timeout,
      1s)` (`mutation.rs:1062`, the hang case's 1 s also bounding the baseline that must finish) and `'timeout' !=
      'killed'` (`mutation_sweep_tests.py:239`, `--timeout 1` for every kind of `failing_baseline`).
    - [x] FIX — one rule, stated in the book's verification chapter: what must finish gets a deadline only a hang
      reaches; what must be stopped keeps a tight one, against a hang that outlasts every bound the test asserts. A
      bounded child's limit is `ARCHOGEN_PARENT_TEST_LIMIT` seconds or ten minutes; `cargo xtask mutate` and
      `scripts/mutation_sweep.sh` set 30 unless their caller sets one, the limit their own were sized for (on a line
      of its own beside the sweep's catalogued `env` line). The runner's finishing controls get 120 s, their hangs
      120 s against 60 s bounds; the sample's hang 10 s, its finishing kinds 120 s; `failing_baseline` 10 s for the
      kind that hangs and 120 s for the rest; every sweep a control runs `RUN_LIMIT = 600`; the observer writers 120 s.
      The census's other sites hold already: `a_run_within_its_limit_…`'s 60 s, and hang-side limits of 1 s and 300 ms.
    - [x] ADDRESSED — under the same 64 workers, `test result: ok. 339 passed; 0 failed; 2 ignored`, 325.11 s, the
      slowest child 36.07 s, past the old 30 s; under 160 workers the timing controls `test result: ok. 12 passed; 0
      failed`; `the_limit_is_the_default_or_a_positive_whole_number_of_seconds` holds the variable's reading.
    - [x] NO REGRESSION — quiet host, `cargo test -p xtask`: `test result: ok. 339 passed; 0 failed; 2 ignored`,
      188.08 s (the stopped hangs now wait 10 s, not 1); `cargo fmt --all -- --check` and `cargo clippy -p xtask
      --all-targets -- -D warnings`: exit code 0; `make focused` on this staged tree: exit code 0, `tier focused:
      passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, which also holds every catalogued `from` text.
    - [x] LOCKSTEP — the book's verification chapter (the rule, the measurements, the tool), `TOOLBOX.md`'s row,
      `M3.3.1` and `M3.3.2`'s records, `M3.6.9.7` filed, `PROGRAM.100` filed, the snapshots, the changelog, and a note promoted to
      `docs/knowledge/a-deadline-is-a-safety-net-not-the-property.md`.
  Found and owned here: `PROGRAM.100` (the tier runner discards a failed step's output); `M3.6.9.7` (a harness's 30 s
    child limit can count a slow child as a kill on a loaded host; blocked by the director's postponement).
  Verification: `target/program99/` (untracked): the focused log and host samples, child timings quiet and loaded,
    both pressured runs before and after. The cause of `2026-10-21`'s ×10 is not established — the re-created load
    measured ×3.9 — and the repair does not depend on it.
  Commit: `ARCHOGEN-PROGRAM-0845 (leaf PROGRAM.99)`
