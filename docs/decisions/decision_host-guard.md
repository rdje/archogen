# This repository's work never endangers the host it shares

- **Type:** `decision`
- **Date:** `2026-11-05`
- **Status:** `active` — by the director's instruction, "that's critical"; written by leaf `PROGRAM.127`, its review `PROGRAM.127.1`
- **External sources:** none: it reads macOS's own figures, each named below
- **Owner / source:** leaf `PROGRAM.127` (`docs/tasks/PROGRAM.md`), the director's words of `2026-11-05`: the machine is shared with many other projects, it holds 24 GB, RAM use above 88% is the danger zone, and above 90% the host might crash.

## The fact / decision

**In plain words.** A job from this repository must never be why the machine falls over, and nobody should have to notice for it to stop. So a watcher reads the host every second, and stops this repository's own processes well before the director's lines. Nothing heavy starts when the machine is already full; it waits. Another project's process is never touched, even when it is the cause: this guard can only take its own share out of the way.

**What it reads, on macOS** (`scripts/host_guard.py`, `read_memory`):
- RAM used: app memory, wired and compressed, as Activity Monitor adds them from `vm_stat`, or the kernel's `kern.memorystatus_level` if that is higher. File cache is not use: macOS gives it back. Measured against the director's own reading on `2026-11-05`: 85.6% here, 85% there.
- Memory pressure: `kern.memorystatus_vm_pressure_level`, 1 normal, 2 warning, 4 critical. macOS's own judgement, since the compressor can hold the percentage still while swap climbs.
- Swap: `vm.swapusage`. Disk: free space where this repository writes, and on the boot volume, where the host swaps.
- Each process's **footprint**, `proc_pid_rusage`, which counts what is compressed and swapped. RSS does not: on `2026-11-05` swap held 62 GiB while the largest process showed 0.9 GiB.

**Its lines** (`Limits`, in one place; an override may only tighten one, so no flag can loosen the guard):

| When | What happens |
| --- | --- |
| RAM over 70%, pressure at warning, swap over 16 GiB, or a disk under its floor | no heavy job starts; `run --wait` waits for room |
| RAM at 85%, pressure at warning, or swap grown 3 GiB during this work | our largest process stops, if ours is worth stopping |
| RAM at 88%, pressure critical, a disk under its floor (100 GiB here, 30 GiB on the boot volume), or 50 GiB lost during this work | everything of ours stops |
| ours over 6 GiB, or one process over 3 GiB, or over 512 processes | the largest stops |

A whole `make focused` measured 0.77 GiB of ours on `2026-11-05`, on a third of the cores at lower priority, so the ceilings leave room to work.

**What is ours.** A process whose working directory or command line is under the repository, what those started, and what an agent working here started. Never an agent or terminal itself, an application, the guard, or a small `git`, which frees nothing and could leave a lock. Never by a program's name across the host: stopping the incident that way killed another project's build.

**How nothing goes around it.**
- `make` runs everything that compiles or tests through `scripts/host_guard.py run`, for any agent or person.
- `.claude/settings.json` checks every shell command first, a subagent's too (`hook`): every command is refused at the stop line, a heavy one at the start line or while a build of ours runs, and what the guard stopped is told before the next command runs. The host can always be read.
- The first check starts the watcher, which then sees everything of ours however it was started.
- `HOST-GUARD` (`scripts/check_host_guard.sh`) refuses a commit that removes any of these or moves the lines past the director's. `scripts/selftest_host_guard.py` shows that it stops what it should and nothing else.

## Why

- **Every second, not at the end.** The incident's first lesson: a job's notification says when it ended, never what it did to the host meanwhile.
- **Footprint, and the host as a whole.** The lines are about the machine, because that is what crashes, and the ceilings about us, because that is what we control.
- **Stop, then say.** The stop cannot wait for a person, and the next command is refused until it has been told.

**Alternatives weighed.**
- **`ulimit` or a memory limit on each job.** macOS does not enforce a resident or address-space limit on a process.
- **Running each command through a wrapper only.** A command not wrapped would go unwatched. The watcher reads the process table, so it holds whatever was started.
- **Stopping whatever holds the most.** It would be another project's more often than ours, and that is not ours to stop.
- **A Rust tool in `xtask`.** It would have to compile before it could guard the compile.

## How to apply

- Heavy work is `python3 scripts/host_guard.py run --wait SECONDS -- CMD`, or `make`. One at a time: a reader never overlaps `make focused`.
- `python3 scripts/host_guard.py status` says what it reads, who holds the most on the host, the peak, and what it has stopped.
- A reader's brief says to test a size limit with a sparse file refused unread, never by loading gigabytes.
- An agent on another harness runs `python3 scripts/host_guard.py ensure` first: only this harness has the hook.
- ⚠️ **What it cannot do.** It cannot make the host safe from other projects. On `2026-11-05` the reading passed 90% with ours at 1 GiB and another project's two compilers at 10 GiB each; the guard stopped ours and said who held it, and that was all it could do. A sudden jump can also pass a line between two samples, and a process that detached itself and names nothing here is not seen. It reads macOS only: elsewhere a job runs unwatched, and says so.
