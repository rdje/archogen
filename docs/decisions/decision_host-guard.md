# This repository's work never endangers the host it shares

- **Type:** `decision`
- **Date:** `2026-11-05`
- **Status:** `active` — by the director's instruction, "that's critical"; written by leaf `PROGRAM.127`, under review in `docs/reviews/decision_host-guard-reviews.md`
- **External sources:** none: it reads macOS's own figures, each named below
- **Owner / source:** leaf `PROGRAM.127` (`docs/tasks/PROGRAM.md`), the director's words of `2026-11-05`: the machine is shared with many other projects, it holds 24 GB, RAM use above 88% is the danger zone, and above 90% the host might crash.

## The fact / decision

**In plain words.** A job from this repository must never be why the machine falls over, and nobody should have to notice for it to stop. So a watcher reads the host every second, and stops this repository's own jobs well before the director's lines. Nothing heavy starts when the machine is already full; it waits. Another project's process is never touched, even when it is the cause: this guard can only take its own share out of the way. It guards against accident. It is no sandbox, and an agent set on going round it can.

**What it reads, on macOS** (`scripts/host_guard.py`, `read_memory`):
- RAM used: app memory, wired and compressed, as Activity Monitor adds them from `vm_stat`, or the kernel's `kern.memorystatus_level` if that is higher. File cache is not use: macOS gives it back. Measured against the director's own reading on `2026-11-05`: 85.6% here, 85% there.
- Memory pressure: `kern.memorystatus_vm_pressure_level`, 1 normal, 2 warning, 4 critical. macOS's own judgement, since the compressor can hold the percentage still while swap climbs.
- Swap: `vm.swapusage`. Disk: free space where this repository writes, and on the boot volume, where the host swaps.
- Each process's **footprint**, `proc_pid_rusage`, which counts what is compressed and swapped. RSS does not: on `2026-11-05` swap held 62 GiB while the largest process showed 0.9 GiB.

**Its lines** (`Limits`, in one place; an override may only tighten one, so no flag can loosen the guard):

| When | What happens |
| --- | --- |
| RAM over 70%, pressure at warning, swap over 16 GiB, or a disk under its floor | no heavy job starts; `run --wait` waits for room |
| RAM at 85%, pressure at warning, or swap grown 3 GiB within a quarter of an hour while ours runs | our largest job stops, if it holds a quarter of a GiB or more |
| RAM at 88%, or pressure critical | every job of ours holding a quarter of a GiB or more stops, and every build |
| a disk under its floor (100 GiB here, 30 GiB on the boot volume), or 50 GiB lost on the volume within a quarter of an hour while ours runs, whoever wrote it | every job of ours that is writing stops, and every build. If none shows writing while the volume falls, a job that has started something new since the last sample |
| ours over 6 GiB | the largest job stops, one each sample |
| one process over 3 GiB | its job stops |
| over 512 processes | the jobs with the most stop, until it is under |

Each line judges alone and they add up, so that none can silence another: a disk's reason once switched the RAM line off. A whole `make focused` measured 0.77 GiB of ours on `2026-11-05`, on a third of the cores at lower priority, so the ceilings leave room to work.

**What is ours.** A process that shows it itself: its working directory or its executable is under the repository, or a process that shows so started it, wherever it then runs. A process merely mentioning the path is not ours: another project's build is not made ours by an argument. Spelt as the volume spells it, since this volume ignores case and the kernel's answer does not.

**A job is what is judged and what is stopped.** A job is the topmost process of ours with all of ours it started: what a person would start again. A compiler stopped under a script would only be followed by the next, and `verify` runs every step whatever the last did, so a line ends the job and its `run` says why. All of it goes, short of an agent or a terminal under it: a small `git` too, which is asked first and takes its lock away when asked.

**What is stopped goes by what a job holds or writes, not by what it is called.** Under a quarter of a GiB, stopping it would not help the host, so at a host's line it is left alone, whatever it is: a `git`, an `rm` freeing disk, an editor with unsaved work, a tool server, a read. A job holds what its processes hold together, so many small ones under one shell count as one. Small jobs apart hold only their own, and are the ceilings' to hold. A build is the exception, since it spawns more for as long as it lives. What only asks or cleans is no build, `cargo clean` among it, which is how disk is freed; nor is a `make` still waiting its turn in its `run`. At a disk floor it goes by what each job has written since the last sample, so a small `cat` feeding on its own output is stopped, and an `rm` is not. A watcher's first sample has no last, so it counts nothing written.

**What is never stopped, nor counted.** An agent, found by searching its name, its executable and what an interpreter runs; a terminal, by name, by a login shell's dash, or a shell told neither a command nor a script; an application; the guard. None of them vouches for what it starts: a build a terminal here runs elsewhere stays that project's.

**Never by a program's name across the host.** Stopping the incident that way killed another project's build, so the hook refuses `pkill` and `killall`, and `kill` of what a search of the process table found, `pgrep` or `ps`, unless that search's pipeline names this repository's path. What is ours is stopped by its pid, and `kill` of numbers, `kill "$(cat job.pid)"`, `pkill -P` and `kill "$(pgrep -P 123)"` pass.

**How nothing goes around it by accident.**
- `make` runs everything that compiles or tests through `scripts/host_guard.py run`, for any agent or person, and waits for room by itself. `run` is one job at a time: it waits its turn behind a build or another `run` of ours, by what the process table shows, so that no environment can claim to be its job and no removed file can give away its turn. It says what it is waiting for, says why its own job was stopped, and gives a job's status as a shell does. The job has a session of its own, so all of it can be stopped with no table to read, and it is ended if no watcher can read the host.
- `.claude/settings.json` checks every shell command first, a subagent's too (`hook`). It reads a command by the shell's own grammar (`ShellReader`), not by a pattern over its text: what a substitution, a subshell, a `bash -c` or a wrapper would run is read, and what a message, a comment or a quoted here-document only says is not. It refuses three things: input it cannot read, stopping by a name, and a command recognised as heavy at the start line, beside a job of ours, or when no watcher has read the host lately, each told how to wait. What the guard stopped is told beside the next command's result (the harness's `additionalContext`) and refuses nothing, since the way out may be that very command. Everything else runs, whatever the host's state: a light command cannot hurt, the watcher holds what turns out otherwise, and a host in trouble must stay readable and repairable.
- Each command the hook reads and each `run` starts the watcher if it is not running, which then sees everything of ours however it was started; `check` only reads. A guard changed under a running watcher holds from then, since the watcher takes up its new self. It survives its own trouble, `cargo clean` or a full disk among it, and stays while anything of ours runs. A watcher that cannot read the host is said to be blind rather than taken for one that can, a process table that does not hold its reader among the causes. It reads every two seconds with nothing of ours running, every second with something, and every half second near the lines or under a disk's floor, since it costs the shared host too.
- `HOST-GUARD` (`scripts/check_host_guard.sh`) refuses a commit that changes what the recipes it writes out run, takes the hook off, lets it fail open or narrows it, or moves a line past its mark. It holds the wiring it names and the lines: a new recipe that runs a build behind a script is the watcher's to hold. That the guard stops what it should and nothing else is `scripts/selftest_host_guard.py`'s. On a host over the start line it runs only the cases that hold nothing, since the rest would add to the load, and it says "not run" of those and of what the host filling mid-test made it unable to judge. Two of its cases hold the rules whole rather than by example: the real shell runs each of some two hundred forms against stubs, and the reader must name exactly what it ran; and every state of the host is judged against one set of jobs.

## Why

- **Every second, not at the end.** The incident's first lesson: a job's notification says when it ended, never what it did to the host meanwhile.
- **Footprint, and the host as a whole.** The lines are about the machine, because that is what crashes, and the ceilings about us, because that is what we control.
- **Stop, then say.** The stop cannot wait for a person, and the result of the next command says it.
- **A job, not a process.** Round 5's reader ran a script around a build at the danger line: eleven stops in as many samples, the script still running, and each stop refusing the session's next command, the `kill` of that script among them. What a person starts is a job, so that is what ends.

**Alternatives weighed.**
- **`ulimit` or a memory limit on each job.** macOS does not enforce a resident or address-space limit on a process.
- **Running each command through a wrapper only.** A command not wrapped would go unwatched. The watcher reads the process table, so it holds whatever was started.
- **Stopping whatever holds the most.** It would be another project's more often than ours, and that is not ours to stop.
- **A Rust tool in `xtask`.** It would have to compile before it could guard the compile.
- **A pattern over a command's text.** Four rounds of answers kept finding both of its edges: a word in a message read as a command, a command in a subshell not read, a comment's apostrophe paired with a quote. The shell's grammar is small, and the shell is an oracle no code here wrote.
- **Refusing the next command until a stop is told.** It refused the way out. The harness can say it beside the command's result.

## How to apply

- Heavy work is `python3 scripts/host_guard.py run --wait SECONDS -- CMD`, or `make`. One at a time: a reader never overlaps `make focused`.
- `python3 scripts/host_guard.py status` says what it reads, who holds the most on the host, the peak, and what it has stopped.
- A reader's brief says to test a size limit with a sparse file refused unread, never by loading gigabytes.
- On another harness only this one has the hook, and the watcher ends when the work has been idle: heavy work there goes through `run` or `make`, which keep it running, or runs `python3 scripts/host_guard.py ensure` before each job.
- ⚠️ **What it cannot do.**
  - It cannot make the host safe from other projects. On `2026-11-05` the reading passed 90% with ours at 1 GiB and another project's two compilers at 10 GiB each; the guard stopped ours and said who held it, and that was all it could do.
  - It is no sandbox. It keeps accident out, and a command spelt to go round the hook, or a program of the same name earlier on the path, is not looked for. What runs here the watcher still holds.
  - A sudden jump can pass a line between two samples, and a job whose writers all begin and end between two shows no writing.
  - Writing through `mmap` reads as nothing written: only the volume's own figures say it.
  - What escapes it: a process that detached itself and shows nothing here, and a command that first leaves the repository's directory, whose shell then shows nothing either. A session rooted in another project that comes here has that project's settings, so no hook; the watcher, if running, still holds what runs here.
  - What it never stops it does not count: an application under `/Applications/`, such as Xcode's linker, can hold memory no limit sees.
  - A job that holds a quarter of a GiB or more is stopped at the danger line whatever it is: an editor that large, a REPL, or an agent's tool server with its directory here. And a job goes whole: a tool server here whose helper writes at a disk's floor, or starts a build at the danger line, goes with it, and so does what it was doing. What holds less stays at a host's line, so small jobs apart are held by the ceilings, not the lines.
  - What it reads as ours by ancestry may surprise: a tool here that starts a build elsewhere makes that build ours. Growth counts what the host lost within the window while anything of ours ran, whoever caused it, so another project's writes or swapping can stop a job of ours, which may then be started again: nothing cheap can say who did it. A falling volume is read the same way. A job whose `run` is killed outright gives up its turn, unless it is a build, which is seen as one.
  - A worktree or submodule is its own root, with its own watcher, ceilings and turn, wherever it stands. One inside this checkout is under this root too, so this root's lines and turn hold it as well.
  - The hook's judgement of a heavy command is a courtesy to the host: what gets past it, a program in a variable, a script, a `find -exec`, a `python3` that allocates, the watcher still holds. `git commit` is not heavy to it, though a commit's hooks can compile.
  - `kill` of a search it cannot follow is not seen: a variable set by an earlier command, or a search fed in after the loop that uses it (`done < <(pgrep x)`). A search before a `kill` of a variable is read as feeding it, so a `kill "$PID"` after an unrelated search by name is refused, with no harm but a rewrite.
  - An agent is found by a search of names, so a script that happens to be called `codex.py` is spared, though a directory named so among a program's arguments is not; a new agent with no such name is not spared. Wrongly sparing costs less than wrongly stopping.
  - It sees this user's processes alone, so "who holds the most" means of this user's.
  - A job has no controlling terminal, so one that prompts on the terminal fails. That is what a session of its own costs.
  - It reads macOS only: elsewhere a job runs unwatched, and says so.

## Review

| Round | Findings | Defects | Verdict |
| --- | --- | --- | --- |
| 1 | 25 | 15 (G1-1 to G1-15), beside 8 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: a process is ours only by what it shows of itself, so another project's build is left alone; what is under a stopped process is spared as it is at the top; the watcher survives its own trouble, stays while a job runs, and says when it is blind; a read cannot carry anything in and is always let through; a figure not read is never safe |
| 2 | 23 | 12 (G2-1 to G2-12), beside 9 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: a terminal's shell is known by being told no command, whatever its name; a root is exactly itself, and spelt as the volume spells it; one job at a time by what the process table shows; a `run` ends its job with no table to read; nothing by a name across the host; what it is, a guard against accident, said |
| 3 | 20 | 5 (G3-1 to G3-5), beside 13 gaps and 2 of wording | `2026-11-05`, not acceptable — answered: what an agent runs for itself is passed over only while small, so a job its shell execs is held; a terminal given a flag's value is a terminal; `pkill` judged by its own stage, and `kill` of a search with it; each tool's writing options its own, so a read is a read; growth counted over a window, and from afresh after a stop; what a job leaves behind holds no turn |
| 4 | 16 | 6 (G4-1 to G4-6), beside 8 gaps and 2 of wording | `2026-11-05`, not acceptable — answered by removing what kept failing: the lists of reads, of small tools and of what a person works in are gone. A host's line goes by what a process holds and a disk's by what it writes; the hook refuses only what it cannot read, stopping by a name, and a heavy command with no room |
| 5 | 21 | 13 (G5-1 to G5-13), beside 5 gaps and 3 of wording | `2026-11-05`, not acceptable — answered by method again: a job is what is judged and stopped; each line judges alone and they add up; the hook reads a command by the shell's grammar, held to the real shell, and tells a stop without refusing anything |
