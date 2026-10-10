---
slug: a-negative-pid-is-an-option-to-the-kill-program
answers:
  - "How do I kill a whole process group from code — and why not run `kill -KILL -<group>`?"
  - "CI says the hosted runner lost communication with the server, and no log was kept — what could my tests have done?"
  - "A test passes on my machine and ends the CI runner — what differs between the two `kill`s?"
type: knowledge
date: 2026-10-10
---

# A negative pid is an option to the `kill` program: signal a group through kill(2), never through `kill`

## The question

A runner kills a hung command's whole process group by running `kill -KILL -<group>`. On macOS that does what it
says. What does it do elsewhere?

## The answer

**Whatever that program's argument parser decides a leading `-` means.** procps-ng's `kill` is the one at
`/usr/bin/kill` on Ubuntu 24.04, as `packages.ubuntu.com/noble/amd64/procps/filelist` lists it. It takes the signal
first, then hands the rest to `getopt_long(argc, argv, "l::Ls:hVq:", …)`. That reads `-1234` as an unknown option
`1`, and its `case '?'` branch has a "Special case for signal digit negative PIDs":
`pid = (long)('0' - optopt); execute_kill((pid_t) pid, signo, …)` (procps-ng `v4.0.4`, `src/kill.c`, read
`2026-10-10`). So a group whose id starts with `1` gets `kill(-1, SIGKILL)` instead: every process the user may
signal. On a CI runner that includes the runner's own agent. A group starting with 2 to 9 is signalled as the
group 2 to 9, which is not the group the caller meant.

It happened here once. `PROGRAM.67`'s mutation runner killed its group that way, and its test ran on the runner
with the rest of the suite. Both jobs of run `38064491086` (`a543d10`) failed after about 45 minutes with *"The
hosted runner lost communication with the server"*, and no log survived. The same suite passes in 37 s on macOS,
whose BSD `kill` reads `-1234` as a pid.

## How to apply

- **Signal a group through the system call:** `kill(2)` with the negated group id, as `xtask/src/mutation.rs`'s
  `signal` does. No program parses the argument there, and no `PATH` lookup picks which program runs.
- **Refuse `0` and `-1` before the call.** They mean the caller's own group and every process, and a negated id
  that is `0` or `1` turns into one of them.
- **Where only a program will do, write POSIX's form, `kill -s KILL -- -<group>`.** POSIX's `kill` (Issue 8,
  `pubs.opengroup.org/onlinepubs/9799919799/utilities/kill.html`, read `2026-10-10`) says it in its OPERANDS: *"If
  the first pid operand is negative, it should be preceded by "--" to keep it from being interpreted as an
  option."* Treat every `kill` that runs without the `--` as a bug.
- **When a runner "lost communication" and kept no log, suspect a signal before a resource.** Look for a test that
  signals anything but its own child, and a parser that may read the target differently on another host.

Related: [`verify-the-mutation-applied.md`](verify-the-mutation-applied.md).
