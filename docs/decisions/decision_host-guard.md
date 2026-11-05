# This repository's work never endangers the host it shares

- **Type:** `decision`
- **Date:** `2026-11-05`
- **Status:** `active` — by the director's instruction, "that's critical"; written by leaf `PROGRAM.127`, under review in `docs/reviews/decision_host-guard-reviews.md`
- **External sources:** none: it reads macOS's own figures, each named below
- **Owner / source:** leaf `PROGRAM.127` (`docs/tasks/PROGRAM.md`), the director's words of `2026-11-05`: the machine is shared with many other projects, it holds 24 GB, RAM use above 88% is the danger zone, and above 90% the host might crash.

## The fact / decision

**In plain words.** A job from this repository must never be why the machine falls over, and nobody should have to notice for it to stop. So a watcher reads the host every second, and stops this repository's own processes well before the director's lines. Nothing heavy starts when the machine is already full; it waits. Another project's process is never touched, even when it is the cause: this guard can only take its own share out of the way. It guards against accident. It is no sandbox, and an agent set on going round it can.

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
| RAM at 88%, pressure critical, a disk under its floor (100 GiB here, 30 GiB on the boot volume), or 50 GiB lost on the volume while ours runs, whoever wrote it | everything of ours stops |
| ours over 6 GiB, or one process over 3 GiB, or over 512 processes | the largest stops |

A whole `make focused` measured 0.77 GiB of ours on `2026-11-05`, on a third of the cores at lower priority, so the ceilings leave room to work.

**What is ours.** A process that shows it itself: its working directory or its executable is under the repository, or a process that shows so started it, wherever it then runs. A process merely mentioning the path is not ours: another project's build is not made ours by an argument. Spelt as the volume spells it, since this volume ignores case and the kernel's answer does not.

**What is never stopped, nor counted.** An agent, found by searching its name, its executable and what an interpreter runs; a terminal, by a login shell's dash or a shell told neither a command nor a script; an application; the guard; what an agent runs for itself rather than through a shell, such as a server for its tools, and what that starts. None of them vouches for what it starts: a build a terminal here runs elsewhere stays that project's. A small `git`, or a small tool for looking at the host or freeing disk, is passed over, since stopping it would free nothing: a `git` could leave a lock, and an `rm` goes on though the shell around it is stopped. Never by a program's name across the host: stopping the incident that way killed another project's build, and the hook refuses `pkill` or `killall` unless the pattern names this repository.

**How nothing goes around it by accident.**
- `make` runs everything that compiles or tests through `scripts/host_guard.py run`, for any agent or person. `run` is one job at a time: it waits its turn behind a build or another `run` of ours, by what the process table shows, so that no environment can claim to be its job and no removed file can give away its turn; the job holds the lock too, and keeps its turn if `run` is killed. The job has a session of its own, so all of it can be stopped with no table to read, and it is ended if no watcher can read the host.
- `.claude/settings.json` checks every shell command first, a subagent's too (`hook`). Nothing but a read runs at a stop line, by RAM, pressure or disk; a command recognised as heavy is refused at the start line, beside a job of ours, or when no watcher has read the host lately; and what the guard stopped is told before the next command that is not a read. A read, every stage of it, is always let through, so a host in trouble can be looked at, its disk freed and a broken guard repaired. Input it cannot read, and a command it cannot judge, it refuses.
- The first check starts the watcher, which then sees everything of ours however it was started. It survives its own trouble, `cargo clean` removing its state among it, and stays while anything of ours runs. A watcher that cannot read the host is said to be blind rather than taken for one that can, a process table that does not hold its reader among the causes.
- `HOST-GUARD` (`scripts/check_host_guard.sh`) refuses a commit that changes what a `make` recipe runs, takes the hook off, lets it fail open, or moves a line past its mark, each written out in that check. It holds the wiring and the lines. That the guard stops what it should and nothing else is `scripts/selftest_host_guard.py`'s, which does not run on a host over the start line, since it would add to the load.

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
- ⚠️ **What it cannot do.**
  - It cannot make the host safe from other projects. On `2026-11-05` the reading passed 90% with ours at 1 GiB and another project's two compilers at 10 GiB each; the guard stopped ours and said who held it, and that was all it could do.
  - It is no sandbox. It keeps accident out: a program of the same name earlier on the path, or a read spelt to carry something, is not looked for, and the watcher still holds what runs here.
  - A sudden jump can pass a line between two samples.
  - What escapes it: a process that detached itself and shows nothing here, and a command that first leaves the repository's directory, whose shell then shows nothing either. A session rooted in another project that comes here has that project's settings, so no hook; the watcher, if running, still holds what runs here.
  - What it spares it does not count: a job run directly by an agent rather than through a shell, or by an application under `/Applications/`, such as Xcode's linker, can hold memory no limit sees. A helper an agent starts through a shell is taken for a job, and stopped with the rest.
  - What it reads as ours by ancestry may surprise: a tool here, such as an editor, that starts a build elsewhere makes that build ours. The disk line counts what the volume lost while ours ran, whoever wrote it, so another project's writes can stop a job of ours: nothing cheap can say who wrote.
  - The hook's pattern for a heavy command is a courtesy to the host: what gets past it, a program in a variable or a build behind a script, the watcher still holds. `git commit` is not heavy to it, though a commit's hooks can compile.
  - At the danger line the shell around a read may be stopped between its stages, and its output lost, though a small tool itself finishes: a short read reaches its end, and is worth trying again.
  - An agent is found by a search of names, so a script that happens to be called `codex.py` is spared; a new agent with no such name is not. Wrongly sparing costs less than wrongly stopping.
  - It sees this user's processes alone, so "who holds the most" means of this user's.
  - It reads macOS only: elsewhere a job runs unwatched, and says so.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 25 | 15 (G1-1 to G1-15), beside 8 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: a process is ours only by what it shows of itself, so another project's build is left alone; what is under a stopped process is spared as it is at the top; the watcher survives its own trouble, stays while a job runs, and says when it is blind; a read cannot carry anything in and is always let through; a figure not read is never safe |
| 2 | 23 | 12 (G2-1 to G2-12), beside 9 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: a terminal's shell is known by being told no command, whatever its name; a root is exactly itself, and spelt as the volume spells it; one job at a time by what the process table shows; a `run` ends its job with no table to read; nothing by a name across the host; what it is, a guard against accident, said |
