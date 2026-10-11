- ID: `PROGRAM.67`
  Status: `done` — filed `2026-10-06` by `M3.6.6.1`'s round 10; started and closed `2026-10-10`
  Goal: `cargo xtask mutate` runs each entry under a timeout that kills the test's whole process group and says
  which entry hung, so a mutation that makes a loop never end is reported, not waited on.
  Acceptance: an entry that hangs is reported within its timeout as a hang the tests reach, and the file is restored;
  a self-test arm holds it; the `extended` tier's `mutation` step finishes whatever the catalogue holds.
  Why: measured `2026-10-06`, a catalogue entry of `M3.6.6.1`'s, `gh-header-reads-past-code`, made the header's loop
  step past the end of a file forever once a redundant end test was removed: the whole-catalogue run waited 1 507.5 s
  until the test was killed by hand, and only then counted it killed. `scripts/mutation_sweep.sh` has such a timeout;
  the harness has none (`grep -n timeout xtask/src/mutation.rs` → nothing). The entry was rewritten to end, so nothing
  hangs today.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git show b3c14de:xtask/src/mutation.rs | grep -n '.output()'` → `248`: the tests ran by
    `Command::output`, which waits without limit; `git show b3c14de:xtask/src/mutation.rs | grep -c timeout` → `0`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: `git show b3c14de:xtask/src/mutation.rs | grep -n 'Command::new("cargo")'`
    → `245`, `run_one`'s call, ending in `.output()` at `250`. WHY: the harness
    assumed a mutation fails or passes; one that makes a loop endless does neither, and cargo's test binary runs in
    cargo's process group, so killing cargo alone would leave it running.
  - [x] **FIX** — `xtask/src/mutation.rs`: `run_bounded`, the command in a process group of its own, its output read
    by two threads, polled against `limit()` — `ARCHOGEN_MUTATE_TIMEOUT` seconds, 600 unless set — and past it the
    group killed with `kill -KILL -<pgid>`; `Seen::Hung`, reported as a kill *"by a hang"*; the file restored by the
    same guard as ever. Two unit tests; two catalogue entries breaking the limit and the group kill. ⚠️ A process the
    tests start with `setsid` leaves the group, and while it holds the output the run waits: stated beside the code.
  - [x] **ADDRESSED (verified)** — `cargo test -p xtask mutation::tests` → `8 passed`; `cargo xtask mutate --only
    mutate-a-hang-kills-the-command-alone mutate-a-hang-is-waited-on` → *"2 mutation(s), each killed"*, each by
    `a_run_past_its_limit_is_stopped_with_everything_it_started`. End to end, a temporary entry making a test spin
    forever, `ARCHOGEN_MUTATE_TIMEOUT=30 cargo xtask mutate --only demo-a-hang` → *"killed by a hang: the tests ran
    past the 30 s limit, their process group killed (30.1s)"*, the file restored byte for byte (`shasum` equal), no
    test process left (`pgrep` → none) — the entry removed after, its run untracked and so not durable.
  - [x] **NO REGRESSION** — `cargo test -p xtask` → `165 passed; 0 failed`; `cargo clippy -p xtask --all-targets -- -D
    warnings` → clean; `cargo fmt -p xtask -- --check` → clean; `make focused` → `passed — 3 passed, 0 failed`; the doctrine gate at commit.
  - [x] **LOCKSTEP** — the module's header; the book's harness section; `TOOLBOX.md`'s row; this leaf and both logs;
    `docs/TASK_TREE.md`; `CHANGELOG.md`.
  Verification: `2026-10-10` — the Verification Log's row
  Commit: `ARCHOGEN-PROGRAM-0530 (leaf PROGRAM.67)`
