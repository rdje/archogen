- ID: `PROGRAM.100`
  Status: `done` — P2: a failed tier step's whole output is kept, not its last 40 lines alone
  Reproduce / issue: `2026-10-22`, `target/m331/focused.log` and `focused-capped.log` — `make focused`'s failed `tests`
    step printed `… 2705 earlier line(s) omitted` and `… 2222 earlier line(s) omitted` then 40 lines; nothing else was
    kept, so the 73-test run's failure list and every failure message were lost (`tail_lines`, `xtask/src/main.rs:745`,
    `TAIL_LINES = 40`), and its advice, "re-run it directly", cannot reproduce a failure that needs the host's load.
  Goal: a failed step's whole stdout and stderr kept under `target/` and named in the summary, the printed tail
    unchanged; a control failing first.
  Acceptance:
    - [x] ROOT CAUSE — `grep -n "print_tail(" xtask/src/main.rs` at `58110665`: three call sites, one per failing branch
      of `run_step` (a gate's refusal, a quarantined step, a failed or undeclared one), each printing `tail_lines`
      and nothing else; `git grep -n verify-logs`: 0 lines — the runner wrote no output anywhere.
    - [x] FIX — `run_step` learns its tier; before a step runs, `forget_output` removes its earlier log, saying so when
      it cannot; each failing branch calls `report_failure`, which prints the same tail and writes
      `target/verify-logs/<tier>/<step>.log` — the command, its status, then each stream whole with its byte count —
      and names it.
    - [x] ADDRESSED — `cargo test -p xtask --bin xtask -- tests::a_failing_step_keeps`: `test result: ok. 1 passed; 0
      failed`: a synthetic step writing 100 lines and a stderr cause, failing with status 3, keeps every line, the
      cause and the status, past the 40-line tail; a passing rerun leaves no log. Failing first, each arm restored byte
      for byte: with the keeping removed `test result: FAILED. 0 passed; 1 failed` at `main.rs:1317`; with the stale log
      left `test result: FAILED. 0 passed; 1 failed` at `main.rs:1327`. The chapter's "the last 40 lines" is gated by
      `the_book_states_the_tail_the_runner_prints` (`docs/figures.md`): `test result: ok. 1 passed`; with the chapter
      saying 41, `test result: FAILED. 0 passed; 1 failed`, restored.
    - [x] NO REGRESSION — the first focused run failed one test, read from the log this leaf keeps
      (`target/verify-logs/focused/tests.log`): `caller suffix source premise requires revalidation` — the sweep
      proof's premise pins `xtask/src/main.rs` whole, for its wiring. Revalidated: its `mod`, `use`, `pub` and `cfg`
      lines identical to `HEAD`'s (`diff`: no output), and no changed item named outside the file (`git grep`: 0 lines
      for each of the eight, `verify` only as `trust_verify::verify`); re-pinned to `48e836b0…`, then
      `cargo test -p xtask --bin xtask -- mutation::tests::the_sweep_holds`: `test result: ok. 1 passed; 0 failed`.
      The over-wide premise is filed as `M3.6.9.8`. `cargo clippy -p xtask --all-targets -- -D warnings`: exit code 0;
      `make focused` on this staged tree: exit code 0, `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not
      built, 0 quarantined`.
    - [x] LOCKSTEP — the book's verification chapter (where a failed step's output goes, with an example), `TOOLBOX.md`'s
      tier-runner row, `docs/figures.md`'s row, the snapshots and the changelog.
  Verification: the control above and its two arms; the focused tier on the staged tree.
  Commit: `ARCHOGEN-PROGRAM-0847 (leaf PROGRAM.100)`
