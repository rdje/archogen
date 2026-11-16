- ID: `PROGRAM.113`
  Status: `done` — filed `2026-10-26` from the CI run on `d5123578`
  Goal: `scripts/check_task_history.sh`'s self-test arm "a SIGALRM just after the tree's rename" holds on every run of
    the Linux runner, or the reason it cannot is stated and the arm says so.
  Reproduce / issue: the `doctrines` workflow's run `40323084588`, attempt 1, on `ubuntu-latest`: the arm `rc 142, not
    142, or the seal not finished and proven`, the seal's output empty — the process ended by the signal before the
    seal printed, as though `SIGALRM` were not held; the same run's `SIGQUIT`, `SIGUSR1` and `SIGUSR2` arms passed, and
    attempt 2 of the same commit passed whole; the arm passed on this host (`aarch64-apple-darwin`) and in the base's
    last green run. Intermittent, so one run diagnoses nothing.
  Direction: tools first — a Linux reproduction (a container or the runner) running the arm in a loop under load,
    recording the process's threads and signal masks (`/proc/self/status`'s `SigBlk`) at the rename; then the mask, the
    arm or the record fixed by what it shows. Not patched blind.
  Since (`2026-10-26`): the runs on `e089751f` (`40326236217`) and `a809a826` (`40327569374`) failed four and five of
    the signal arms after the tree's rename — `HUP`, `QUIT`, `USR1`, `USR2`, `ALRM`, never `INT` or `TERM` — each with
    its expected exit code, so the failures are steady on the runner, not one run's. The self-test runner keeps only
    `^SELF-TEST` lines, so the arms' evidence never reached the log: the arm's failure line now carries it — whether the
    seal said it sealed, whether `Q.1.md` exists, the seal's last words and the gate's verdict in the proof — as the
    tool the next run reads (`ARCHOGEN-PROGRAM-0976`).
  Closed (`2026-10-26`): `.1` put the arms' evidence on the line the runner keeps, `.2` traced the seal, which showed
    it sound and the arms' own check at fault, `.3` read every output through a here-string; the runs on `34b6abeb` —
    `doctrines` `40337545873`, `rust` `40337545686`, `trust-gate` `40337546070` — each `success`.
  Verification: the three runs on `34b6abeb`, green.
  Commit: `ARCHOGEN-PROGRAM-0978 (leaf PROGRAM.113.3)`, its last child's

- ID: `PROGRAM.113.1`
  Status: `done` — the signal arms' failure line carries the seal's evidence, the one line the runner keeps
  Acceptance:
    - [x] ROOT CAUSE — WHY: the self-test runner keeps a failing gate's `^SELF-TEST` lines alone, and the arm printed its evidence on the lines after, so the runner's log said only that the arms failed; WHERE, `git show a809a826:scripts/run_self_tests.sh`, its line 110, `grep -E '^SELF-TEST|self-test' | head -8`.
    - [x] FIX — the arm's failure line names whether the seal said it sealed, whether `Q.1.md` exists, the seal's last words and the gate's verdict in the proof.
    - [x] ADDRESSED — the `SIGALRM` arm forced to fail, its wanted code set to `1` and restored after: its line read `sealed line yes, Q.1.md yes; output: …; gate: task-history: OK …`; `bash scripts/check_task_history.sh --self-test`: `171 pass / 0 fail (171 arms)`.
    - [x] NO REGRESSION — `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 337 s; the gate's behaviour unchanged, the arm's message alone longer.
    - [x] LOCKSTEP — the arm and `PROGRAM.113`'s record.
  Commit: `ARCHOGEN-PROGRAM-0976 (leaf PROGRAM.113.1)`

- ID: `PROGRAM.113.2`
  Status: `done` — the signal arms trace the seal: the signal sent, each mask change, each outcome printed
  Acceptance:
    - [x] ROOT CAUSE — WHY: `.1`'s evidence from the run on `d7fa9c69` — `sealed line no, Q.1.md yes`, the gate's proof OK — fits both a line printed and lost and a process ended before it printed, and one run cannot tell them apart; WHERE, `git show d7fa9c69:scripts/check_task_history.sh`, the arm's failure line, no order of events.
    - [x] FIX — the test hook writes, unbuffered and in order, to a trace file the arm names: `fire N` as the signal is sent, `mask HOW COUNT` at each mask change, `print …` before each `task-history:` outcome; the arm's failure line quotes the trace and the output's first lines.
    - [x] ADDRESSED — the `SIGALRM` arm forced to fail, its wanted code `1` and restored after: `trace: mask 1 23|fire 14|print task-history: docs/tasks/Q.md — sealed 1 subtree|mask 3 0|`, the designed order on this host; `bash scripts/check_task_history.sh --self-test`: `171 pass / 0 fail (171 arms)`.
    - [x] NO REGRESSION — test-only code, the gate's own unchanged; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 308 s.
    - [x] LOCKSTEP — the hook, the arm and `PROGRAM.113`'s record.
  Commit: `ARCHOGEN-PROGRAM-0977 (leaf PROGRAM.113.2)`

- ID: `PROGRAM.113.3`
  Status: `done` — the arms read their output without a pipe, so `pipefail` cannot turn a match into a failure
  Acceptance:
    - [x] ROOT CAUSE — WHY: `.2`'s trace on the runner, run `40333831327` on `80719f25`, read `mask 0 54|fire 1|print task-history: docs/tasks/Q.md — sealed 1 subtree|mask 2 0|`, the output's first line the seal's own — the seal held the signal, said its outcome and was proven; the arm failed in its own check, `printf '%s' "$out" | grep -qE '^task-history: … sealed '` under `set -o pipefail`: GNU grep's `-q` exits at its first match while `printf` is still writing an output the size of the gate's Python, which bash's notice of a signalled job repeats (`git show d7fa9c69:scripts/check_task_history.sh` holds 55174 bytes of it, the base `8cdac117` 35958), so `printf` meets a closed pipe and the pipeline fails though `grep` matched — on Linux's pipe, never on this host's larger one or BSD grep, which reads to the end.
    - [x] FIX — every `printf '%s' "$out" | grep -q…` in the script, fifty on forty-five lines, made `grep -q… <<< "$out"`: no pipeline, so no `pipefail`.
    - [x] ADDRESSED — the mechanism alone, GNU grep under `pipefail` on a matching first line: through the pipe `20/20 pass` at 70 kB and `0/20 pass` at 300 kB; through the here-string `20/20 pass` at 1 MB; the arms with each signalled run's output padded by 300 kB, under GNU grep, before the fix `bash scripts/check_task_history.sh --self-test`: `159 pass / 12 fail (171 arms)`, after it `171 pass / 0 fail (171 arms)`; unpadded, with this host's grep and with GNU grep, `171 pass / 0 fail (171 arms)`.
    - [x] NO REGRESSION — `bash scripts/check_task_history.sh`: OK; `make focused` on this staged tree: `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined`, in 358 s.
    - [x] LOCKSTEP — the script; `PROGRAM.114` filed for the idiom elsewhere.
  Commit: `ARCHOGEN-PROGRAM-0978 (leaf PROGRAM.113.3)`
