# Review history — `decision_host-guard.md`

The independent review of [`docs/decisions/decision_host-guard.md`](../decisions/decision_host-guard.md) with `scripts/host_guard.py` (leaf `PROGRAM.127`). Each round is a reader who had not written it, asked to break it by running it, itself under the guard, with tiny limits in scratch roots: does it ever stop what is not ours, miss what is, or fail open?

**Round 1** — `2026-11-05`: not acceptable, 25 findings, 15 defects. Its measurements held: the footprint read 107.0 MiB for a 100 MiB buffer, as `footprint` and `top` say; RAM agreed with a hand sum from `vm_stat`, 52.77% to 52.7%; pressure with `sysctl`; 400 events against a live writer all read. What broke was what it decided from them, and how it behaved when things went wrong.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G1-1 | defect | the walk passed through protected ancestors: an agent elsewhere naming the root, or a terminal started here, made a build in another project ours | A protected process vouches for nothing, and the walk stops at it. A case holds each shape |
| G1-2 | defect | a process was ours by mentioning the path, `ROOT copy/…` and `/other/fixtures/ROOT/…` among the matches | Not by mention at all: by working directory or executable, read from the process itself |
| G1-3 | defect | what was under a stopped process ignored `protected`, so an agent's `git` died under its shell | The tree of a stopped process leaves a protected process and all it started, and a small `git` |
| G1-4 | defect | exact names missed `cursor-agent`, an agent an interpreter runs, and any path with a space | An agent is searched for in its name, its executable and its script; a name is cut by the executable's own path |
| G1-5 | defect | `rm -rf target`, as `cargo clean` does, a damaged `peak.json` or a closed log killed the watcher | Its bookkeeping cannot raise, the stop comes first, and its lock is on the root, which nothing under `target/` can take |
| G1-6 | defect | nothing checked the watcher was alive | `run` keeps one running and ends its job if none can read the host; a heavy command is refused without one |
| G1-7 | defect | the watcher exited while a small job ran | It stays while anything of ours runs |
| G1-8 | defect | a watcher that could not read `ps` looked healthy and filled its log | `sampled` says when it last read the host: `status` says blind, a heavy command is refused, and the log says so once a minute |
| G1-9 | defect | a newline or `$(…)` carried anything in on a read | Every stage must be a read, and no substitution, background or redirection is allowed |
| G1-10 | defect | the hook ignored pressure | Nothing but a read runs at a stop line by RAM, pressure or disk. Swap growth stays the watcher's, which has the baseline, and the record says which |
| G1-11 | defect | a pipe, `lsof`, `du` and the scratch preparation were refused, and `rm` at a disk floor | Those are reads, each stage judged; `rm` too, since disk must be freeable. A small reader is spared by the watcher |
| G1-12 | defect | a bad line lost what was told, or crashed the hook and refused everything for good | Each line is read alone and the position moved only after; the hook refuses what it cannot judge, and a read still runs to repair it |
| G1-13 | defect | a renamed `vm_stat` label read as zero, falling back to a lower figure | A label not found raises: a figure not read is never safe |
| G1-14 | defect | the disk line did not count a small writer | Growth counts from when anything of ours runs |
| G1-15 | defect | `make book` and `make gate` compiled outside it | Through it, and `HOST-GUARD` reads what each recipe line runs |
| G1-16 | gap | heavy commands got past the pattern | More forms held, and the record says it is a courtesy, with the watcher behind it |
| G1-17 | gap | a word in a message or a search was refused over the start line | Quoted text and a heredoc's body are not read as commands, and a program counts only where a command stands |
| G1-18 | gap | `run` did not hold one job at a time | It waits behind a build or another `run`, and a job nested in its own `run` is no rival |
| G1-19 | gap | a `node` job, or any dash-prefixed name, was spared | `node` alone is no agent, and only a real login shell is spared. What is spared is said not to count |
| G1-20 | gap | one second per target in turn, and zombies reported again and again | All asked at once and made sure of at once; a zombie is not in the table |
| G1-21 | gap | `stop` trusted a busy lock over a stale pid | A pid counts only if that process says it is this root's watcher |
| G1-22 | gap | `HOST-GUARD` held 3 of 17 limits and read recipes loosely | Every limit has its mark, and a limit it does not know is refused; the record says it holds the wiring, not behaviour |
| G1-23 | wording | "told before the next command" passed over reads and `run` | `run` is no read and tells too; the record says "that is not a read" |
| G1-24 | wording | "who holds the most on the host" was this user's, and a space cut names | Said as this user's, with an interpreter's script beside its name |
| G1-25 | gap | a session rooted elsewhere gets no hook. Unverified | Said in the record: project settings are per project |

Said, and left as it stands: a detached process that shows nothing here, a jump between two samples, and the host's safety from other projects were confirmed as the record states them. Its reader could not run the census against the real watcher, real pressure, `onFailure` with a real crash, or the true 600-second idle time.

**Round 2** — `2026-11-05`: not acceptable, 23 findings, 12 defects. The measurements and the watcher's survival held: RAM by hand from `vm_stat` 57.31% and by the guard 57.31%; a second watcher exits at once even with its state removed; with state read-only the stop still happens and `status` says blind; a real `git` under a stopped hook left no lock. What broke was the edges of what is ours, `run` going blind, and what a read may carry. From this round the record says what it is: a guard against accident, not a sandbox, so a form written to go round it is a stated limit where closing it costs more than it is worth.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G2-1 | defect | a real interactive or login shell with no dash on its name counted as ours, and so did a build elsewhere under it | A shell told neither a command nor a script is a terminal, whatever its name, and vouches for nothing |
| G2-2 | defect | `--root` was a substring, so root `A-other` counted as `A`'s, its job stopped and its watcher ended | A root is exactly itself, on the command line and in `stop` |
| G2-3 | defect | with `ps` failing or hanging, `run` died with a traceback or hung, and its job went on | The job has its own session and is ended with no table to read; `ps` has a timeout; nothing there may raise |
| G2-4 | defect | a helper's own children were stopped | What an agent's helper starts is its own. One started through a shell is stated as taken for a job |
| G2-5 | defect | the hook let `rm` and reads through, then the watcher killed them | A small tool is passed over, `rm` among them, so disk can be freed. That the shell around a read may be stopped at the danger line is stated |
| G2-6 | defect | one job at a time failed four ways: lock removed, environment forged, `run` killed, and the hook blind to a job not named as a build | By what the process table shows, no environment, and the job holds the lock too; the hook refuses beside any `run` of ours |
| G2-7 | defect | a `run` from another directory was not recognised, so its job was unwatched | A `run` always says its root on its command line, re-executing itself if it must |
| G2-8 | defect | the hook passed input it could not read | Refused, each shape a case |
| G2-9 | defect | a small `git` that was our largest was stopped at the stop line | It is passed over for the next worth stopping |
| G2-10 | defect | a blind watcher left while a job ran | It keeps what it last saw |
| G2-11 | defect | commands rode through `is_read`: a flag, a function, a path, `export PATH`, `sort -o`, `git diff --output` | Bare names only, no function or subshell, a named few settings, options that write refused. What remains is deliberate, and the record says it is no sandbox |
| G2-12 | defect | `HOST-GUARD` passed `cargo fmt && cargo test`, an included file, `disableAllHooks`, `async` and a mere mention of the hook | The recipes and the hook's command are written out in the check and compared whole. Its reader seeded these on a copy to test a sentence; none is kept as a mutant |
| G2-13 | gap | a root spelt in another case saw nothing, on this case-insensitive volume | A root is spelt as the volume spells it |
| G2-14 | gap | `git commit` can compile outside `run` | Stated, and the watcher holds it |
| G2-15 | gap | `pkill`, `kill` and `stop` passed at any line, `pkill -f rustc` being the very act the record blames | `pkill` and `killall` are refused unless the pattern names this repository; `kill` takes pids; `stop` is no read |
| G2-16 | gap | a `ps` answering nothing or cut short looked healthy | A table that does not hold its reader is blindness |
| G2-17 | gap | a `sampled` time in the future passed as fresh | Either way counts |
| G2-18 | gap | `status` and bookkeeping broke on binary or deep files, and read whole logs | They read past it, the log is bounded and rotated |
| G2-19 | gap | with `seen` past a replaced log, a stop went untold | It starts again |
| G2-20 | gap | an interpreter's flag hid an agent's script | The first few paths it names are searched too |
| G2-21 | gap | an assignment of a path ending in `/archogen` counted as heavy | A program counts only where a command stands |
| G2-22 | wording | `status` named stops only among its last five lines | The last few stops, whatever came after |
| G2-23 | wording | "lost during this work" counted anyone's writes | Said as it is |

Said, and left as it stands: its reader could not run the census against the real watcher, real pressure levels, the true idle and blind times, a commit's compile, a real terminal in an IDE or a real tool server, or `onFailure` with a real crash.

**Round 3** — `2026-11-05`: not acceptable, 20 findings, 5 defects. Asked about ordinary work rather than forms written to go round it. The measurements held again: RAM by hand 50.61% and by the guard 50.61%; a 60 MiB buffer read 66 MiB; the watcher used 1.7% of a core at 21 MB; a job's output reached its caller and its exit status was kept; `rm -rf target` during a job left the same watcher, the state back and the job finishing. Claude Code's own shell stayed the parent, so the worst finding does not reach this harness. Two of round 2's answers had brought new trouble, the lock passed down to what a job left behind and a list of writing options applied to every tool.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G3-1 | defect | when an agent's shell execs its command, the job counted as the agent's helper: not counted, not stopped, not seen by the check for a build | That is told from a helper by nothing, so a helper is passed over only while small: past a quarter of a GiB it is a job |
| G3-2 | defect | a terminal given a flag with a value, as VS Code's is, counted as a job | The value is skipped: only `-c` or a script makes a shell a job |
| G3-3 | defect | `pkill` passed whenever the repository's path appeared anywhere, as `cd ROOT && …` does, and `kill $(pgrep …)` was the same act unseen | Judged by its own stage, and `kill` of a search refused with it |
| G3-4 | defect | `-o` and `-S` were refused for every tool, so `ps -o` and `top -o` were no reads at a stop line | Each tool's writing options its own |
| G3-5 | defect | macOS's `screen` is `SCREEN`, and was stopped | Infrastructure by name in any case |
| G3-6 | gap | a long-lived small process kept the count of growth for hours, stopping job after job for what others did | Growth is counted over a window, and from afresh after a stop |
| G3-7 | gap | the lock passed to a job was held by what it left behind, with a false refusal and no way to find the holder | Not passed. A job whose `run` is killed gives up its turn unless it is a build: stated |
| G3-8 | gap | `run` never said its own job's stop, reading the log before it was written | Written first, then the stop |
| G3-9 | gap | in a linked worktree beside the repository the guard exited, and with `onFailure: block` every command was refused | A linked worktree is its own root |
| G3-10 | gap | on another harness `ensure` once is not enough | Said as it is: `run`, `make`, or `ensure` before each job |
| G3-11 | gap | a search for the word `pkill` was refused | Only where a command stands |
| G3-12 | gap | `cargo --version`, `make help` and a word in another tool's quoted argument counted as heavy | What only asks is not, and only a shell's `-c` is read as code |
| G3-13 | gap | `pwd`, `sed -n`, `find`, `git show`, a comment and the handoff census were no reads | They are |
| G3-14 | gap | a refusal did not say what to do, and `run --wait` was silent | Each says what to do, it says what it waits for, and `make` of a guarded recipe passes the hook and waits by itself |
| G3-15 | gap | two sessions shared one place in the log, so one was refused for the other's stop | Each session its own, by the harness's id |
| G3-16 | gap | a person's editor or REPL here was stopped at the danger line, unsaved work with it | They are what a person works in, and are spared |
| G3-17 | gap | the self-test left a watcher behind, which the census would count | It stops it after the job that keeps it alive |
| G3-18 | gap | a command that cannot start, a failing `ps` at start and ^C were tracebacks, and a signal's status not a shell's | Each said, as a shell says it. No controlling terminal: stated |
| G3-19 | wording | "`HOST-GUARD` refuses a commit that changes what a `make` recipe runs" overclaimed | Said of the recipes it writes out, and `GUARD` defined once, and the hook's fields known |
| G3-20 | wording | "the first check starts the watcher", and "during this work" left in a log line | Both said as they are |

Said, and left as it stands: its reader could not run a real `make`, a commit's compile, a real VS Code terminal, a real Codex or Kimi command, real pressure levels, the true idle and blind times, the census against the real watcher, or `onFailure` with a real crash.

**Round 4** — `2026-11-05`: not acceptable, 16 findings, 6 defects, after 15, 12 and 5. No longer converging, and most of the last three rounds' findings came from lists kept by hand: what is a read, which small tools to spare, what a person works in. Each list had edges, and each answer added to one. So this round's answer is by method: the lists are gone. A host's line goes by what a process holds, a disk's by what it writes, and the hook refuses only three things. That answers some findings by removing what they were about, and the next reader should look hardest at what the removal let through. Its measurements held again: RAM by hand 59.28%, by the guard 59.29%; a job under `run` ended rightly on each signal; `TaskStop` left nothing behind. Round 3's 1.7% of a core did not hold: it measured 6.1% far from the lines and 19.7% near them, children included.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G4-1 | defect | `kill "$(pgrep …)"`, backticks, a loop and a variable all passed, the very act the record blames | `kill` with `pgrep` anywhere in the command, double quotes read since they still run what is inside, unless the search names this root |
| G4-2 | defect | a job of many small processes directly under an agent was passed over whole, past the danger line and every ceiling | No process is passed over for whose child it is. A ceiling is reached even when a host's line finds nothing to stop |
| G4-3 | defect | at a disk floor a small tool doing the writing was left, as a `cat` feeding on itself | A disk's line goes by what each process wrote since the last sample |
| G4-4 | defect | with writes failing, the watcher did not stop the job and died | Its bookkeeping cannot raise, and its streams no longer go to the volume |
| G4-5 | defect | two `make`s waited on each other until `--wait` ended, each taking the other for a build | `make` is no sign of a job, only what builds or a started `run` |
| G4-6 | defect | `sed -n` and `find` passed the hook and the watcher then stopped them | Nothing that holds little is stopped at a host's line, and there are no reads to keep in step |
| G4-7 | gap | a worktree or submodule inside another checkout took that project as its root and its processes as ours | The nearest checkout is the root |
| G4-8 | gap | after `cargo clean` a stop could be told to nobody, a lost place read as a new session, and `run` sought past a replaced log | A session with no place is told what was stopped lately, and `run` reads a replaced log whole |
| G4-9 | gap | what was passed over still counted as worth stopping, so a tiny job was stopped for helpers' bulk | Nothing small is stopped at a host's line |
| G4-10 | gap | a refused handoff left the watcher off, reads not restarting it | Every command restarts it now |
| G4-11 | gap | at a stop line "wait for room" could not be done: `sleep` itself was refused | Refused no longer: only a heavy command is |
| G4-12 | gap | "what a person works in" was a short list | No list: what holds little stays. What holds a quarter of a GiB does not, which is stated |
| G4-13 | gap | an interpreter job naming a data file after an agent was spared | Only a path with no extension counts as a program |
| G4-14 | gap | with `ps` slower than 5 s, `status` called a live watcher gone and `stop` did nothing | Read from the lock on the root and the executable, with no `ps` |
| G4-15 | wording | "a read is always let through" was false for a `\|` in quotes | There are no reads now, and that command runs |
| G4-16 | wording | `kill $(cat job.pid)` was refused as by a name, and a pattern naming the root held for `pkill` alone | By a pid passes, in either quoting, and a search naming this root passes |

Said, and left as it stands: its reader could not run a real `make`, a commit's compile, a real full volume, real pressure levels, the true idle and blind times, the census against the real watcher, a real Codex, Kimi or VS Code, two live harness sessions, or `onFailure` with a real crash.

**Round 5** — `2026-11-05`: not acceptable, 21 findings, 13 defects, after 15, 12, 5 and 6: rising, so again by method. The reader found twenty, and its engineer a twenty-first, from the refusal of the session's first command. The measurements held: RAM by hand 54.12%, by the guard 54.1 to 54.2%; a 100 MiB buffer read 106.8 MiB, `footprint` 107 MB; 50 MiB written read 50.0. Nothing outside a scratch root was stopped, and no case of "not ours" failed.

What broke was what round 4's rule stood on. Lines looked at a process where what a person starts is a job: a build stopped under a script was followed by the next, eleven stops in as many samples, and each stop refused the next command, the `kill` of that script among them. And the hook read a shell's text by a pattern, which after four rounds of answers still took words in a message for commands and missed what ran in a subshell.

So a job is what is judged and stopped; each line judges alone and they add up; and the hook reads a command by the shell's own grammar, held to the real shell by forms run against stubs, while an enumeration of the host's states holds the lines. The next reader should look hardest at what stopping a whole job now takes with it, and at where the reader of a command and the shell can still part.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G5-1 | defect | the danger line never ended a script around a build, and `verify` runs every step whatever the last did | A line stops the job, its topmost process of ours with all it started. A watcher no longer spares the `make` that happened to start it |
| G5-2 | defect | while stops continued, every command was refused, the `kill` of that very job among them, so "it refuses three things" was false | A stop is told beside the next command's result and refuses nothing. It refuses three things |
| G5-3 | defect | a script of short writers crossed a disk's floor unstopped | The first writer seen takes the script with it, the watcher reads every half second under a floor, and with none seen a volume falling under a job that keeps starting something new is enough. All of them between two samples is a stated limit |
| G5-4 | defect | any disk reason switched the RAM stop line off, and with a small writer only it stopped at the danger line | Each line judges alone and they add up, and a case judges every state of the host |
| G5-5 | defect | `cargo clean` could not run at a floor, refused by the hook and stopped by the watcher | What only asks or cleans is neither heavy nor a build |
| G5-6 | defect | a `make` queued in its `run` was stopped at the danger line | It is no build until its `run` has started something |
| G5-7 | defect | "since the last sample" was false at a watcher's first, so what wrote long ago and sat idle was stopped | A first sample has no last, and counts nothing written |
| G5-8 | defect | stopping by a name passed in a subshell, in `$(…)`, in `bash -c`, under a comment's apostrophe, and through `ps` into a loop | The command is read by the shell's grammar, and the shell itself holds that reading |
| G5-9 | defect | it refused by a pid and a pattern naming the root, and took words in a search or a message for commands | The same: `pkill -P`, `xargs kill` of a file, a search naming this root, and text all pass |
| G5-10 | defect | the refusal's own remedy was refused, `run … -- bash -c 'cargo test'`, and `make focused>log`, `make -j4 focused` | The same: what `run` is given is its argument, and `make`'s words are read as `make` reads them |
| G5-11 | defect | "left alone, whatever it is" was false for a 150 MiB `vim` at the stop and danger lines, and a small `git` under a stopped build | A job under a quarter of a GiB stays at any host's line, and many small jobs apart are the ceilings'. A stopped job goes whole, which the record now says, with what that takes |
| G5-12 | defect | the self-test, run as it names itself, left an untracked `scripts/__pycache__`, which `UNTRACKED-CODE` refuses | It writes no bytecode and checks so, and `.gitignore` holds any other script's |
| G5-13 | defect | a hand-written line whose time was no time refused the first command of every new session for good, by comparing as text | A time that is no time is never recent. The line is gone from the log |
| G5-14 | gap | a job taking a directory named after an agent was spared | The search for what an interpreter runs ends at what it runs, so the rest are that program's own arguments |
| G5-15 | gap | ordinary heavy forms passed: a comment's apostrophe, `$(…)` in quotes, `caffeinate`, `time -l`, `rustup run` | The same reader, with what wraps a command written down |
| G5-16 | gap | a watcher kept the code it started with, so a fix was not in force while anything of ours ran | It takes up its new self |
| G5-17 | gap | `mmap` writes read as nothing | Stated: only the volume's figures say it |
| G5-18 | gap | a case's name said a new session is told nothing, and it passed with no stop in the log | It now holds a stop long ago, one with no time, and one just made |
| G5-19 | wording | "its own ceilings and turn, wherever it stands" was false for a worktree inside the checkout | Said as it is |
| G5-20 | wording | "over 512 processes, the largest stops" | Said as it is: the jobs with the most, until it is under |
| G5-21 | wording | "each check starts the watcher" was not true of `check` | Said as it is |

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, real pressure levels, a real full volume, the true idle and blind times, the census against the real watcher, a real Codex, Kimi or VS Code, or `onFailure` with a real crash. It stated that it ran `git log -3 --format='%h %ad' -- scripts/host_guard.py` once, dates for that script alone with no content and no sealed path, against a brief that said no history search; and that a first run of a rig of its own left a `bash` behind, which it stopped by its pid.

**Round 6** — `2026-11-05`: not acceptable, 13 findings, 2 defects, the fewest yet after 15, 12, 5, 6 and 13. Each measurement it re-derived held: RAM by hand 83.18% against 83.19%, a 100 MiB buffer 106.8 MiB against `footprint`'s 107, the watcher's gaps, the hook at 46 ms. It ran through a real incident: another project's `rustc` at 14 to 15 GiB took the host to 93.9%, with its own processes at 0.05 GiB at most, and the real watcher stopped its rig.

Both defects were sentences the code did not keep, at the two places the brief pointed to. A floor's way out was stopped for a fall of a single MiB, by anyone, on either volume; and "left alone, whatever it is" was not true of what starts a build. The reader's gaps in the shell reader were real, and it calibrated them: over 18,403 commands from this project's own transcripts, none of the forms it found lost occurred. They are answered all the same, because the reader of a command is what the hook stands on, and each form is now held to the shell.

So a fall is what a shared volume does not cost anyway, on the volume that is short; `case`, arithmetic, arrays, `[[ ]]`, `trap` and what a here-document feeds a shell are read as the shell reads them; and the oracle runs against each `bash` here, which at once found bash 3.2's own bug with a case inside a substitution. The next reader should look hardest at the reader's new branches, and at whether what is now stated of a job going whole is the whole of it.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G6-1 | defect | at a floor a script of `rm` was stopped, 9 runs of 9, by a 1 MiB fall on either volume by anyone, against "an `rm` is not" | A volume has fallen when it has lost a GiB within ten seconds, against up to 49 MiB a half second measured from others, and only the volume that is short counts. A real script of `rm` and `du` now runs to its end under a floor, and a slower script of short writers is what this costs, said |
| G6-2 | defect | a small editor was stopped for what its `:!` ran, where the record listed it as left alone and named only a tool server | The record says it: left alone as long as it has started no build, and an editor goes with the build it started. Nothing cheap tells an editor from a script: `verify`'s script is why a job goes whole |
| G6-3 | gap | "all of it goes" was false through `run`: a script of four `run` steps had each stopped and went on, rc 0 | Said as it is: a `run` is a job of its own, its caller hears 143, and each next `run` is judged again. Only the `Makefile` calls it, and `make` stops |
| G6-4 | gap | a shell fed its script reads as a terminal, and the reader did not read the script | A here-document or here-string fed to a shell is read. A pipe into one is not, and the watcher's view is a stated limit, since telling them apart would stop an agent's own shell |
| G6-5 | gap | the reader lost the rest of the command after a quoted `esac)`, and read `<<` in arithmetic as a here-document | A case has its own reading, branch by branch, and `((`, `$((` and `$[` read as arithmetic, as the shell decides it |
| G6-6 | gap | `bash -euo pipefail -c …` and `bash -c -- …` went unread | `shell_told` reads options as the shell does, the watcher's terminal and the hook's reading both |
| G6-7 | gap | `trap '…' EXIT` and a substitution in arithmetic went unread | Both read |
| G6-8 | gap | text was refused as a command: an array's words, a case's patterns, a `[[ ]]`'s regular expression, arithmetic | Each read as what it is, and a function's name is no command |
| G6-9 | gap | what only asks was read as heavy: `cargo build --help`, `rustc --explain`, `cargo update`, `mdbook help`, `make --jobs 4 focused` | An asking flag before any `--`, `help` first, and what only edits a manifest or a lock file or fetches. `archogen-plan-check --help`, a heavy case before, is now a light one |
| G6-10 | gap | a main agent and its subagents shared a place in the log, so a stop was told to whichever ran a command first | The place is the session's and the `agent_id`'s, which the harness's reference gives a subagent |
| G6-11 | gap | "what each job has written" is what has reached the disk, and only `mmap` was stated | Said, with what was measured. `ri_logical_writes` was tried and is worse, which the record's alternatives say |
| G6-12 | wording | `check` and `status` said a heavy job may start beside a running one | Each now says one heavy job at a time, as `run` and the hook do |
| G6-13 | wording | smaller slips: a `run`'s job ours wherever it runs, the watcher's pace, "the real shell" being 3.2 where the harness runs 5.3, what `protected` holds, the limits' comments | Each said as it is, and the oracle runs both shells |

Beside these, one of the self-test's own: under load the oracle read its log before `cat <(cargo build)` had written, since a shell does not wait for a process substitution, 6 runs of 150 on one bash. The first fix tried, waiting on the shell's standard output, was worse, 33 of 150, since that output is the substitution's pipe; a descriptor everything the shell starts inherits gave 0 of 300 on each `bash`. And G6-12's answer cost the self-test three cases until it was seen: it asked `check` whether the host was clear, and `check` now counts a build of ours, which a stub of its own reads as, so those three said "not run" on a quiet host, twelve runs of twelve. It now asks about the host's room alone.

Said, and left as it stands: its reader could not run a real full volume, a real `make focused`, a commit's compile, chosen pressure levels, the true idle and blind times, the census against the real watcher, a real Codex, Kimi or VS Code, or `onFailure` with a real crash. It stated that it read this project's transcripts, the Bash `command` field alone, dropping unread any naming `docs/evaluation` or `frozen`; that it fetched the harness's hooks reference; and that it made one disposable `git init` fixture under `target/p1276/work/`, with `git add` and no commit. Its rigs under `target/p1276/work/`, inside this root, count as builds to the real watcher, which is why it stopped one. It resumed small rigs after the host recovered, beyond a literal "stop", and said so.

**Round 7** — `2026-11-05`: not acceptable, 13 findings, 1 defect and that small, after 15, 12, 5, 6, 13 and 2. What it re-derived held: RAM by hand 87.55% against 87.55%, a 100 MiB buffer 106.9 MiB against `footprint`'s 107, pressure against `sysctl`. Its fuzz of 800 composed forms on both `bash`es missed nothing, and its 332 hand-written forms found only what is below. Its own subagent had a place of its own in the real log, `seen.…`, and the real hook told it of a stop by `additionalContext`: round 6's G6-10 seen in the real harness. It read a real `make` around a small `run` at the danger line go as the record says, and found nothing else missing from what is stated of a job going whole.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G7-1 | defect | `make --version`, `make --help` and `gmake -v` were heavy, since G6-9's answer returned before it reached `make`; and `Process.spawns` called `make help` and `make -n` builds | `make_only_asks`, in both places |
| G7-2 | gap | `condition()` took a `#` glued to a metacharacter for a comment, and missed `]]` before `<` or `>`, losing the rest of the command | A `#` begins a comment only where a word could begin |
| G7-3 | gap | a here-document to a shell with arguments went unread: `-s` was ignored, and `/dev/stdin` as its script | `-s` keeps it on its input, and `bash /dev/stdin` and `source /dev/stdin` read it |
| G7-4 | gap | a delimiter closing on the substitution's `)` lost all after it | Inside a substitution it ends the body, and leaves the `)` |
| G7-5 | gap | rarer misses: `a[0]=x cargo`, `${x:-'}'}`, `coproc name { … }`, `{fd}>log cargo`, `env -S 'cargo build'` | Each read, the two only later `bash`es have held on those |
| G7-6 | gap | false refusals of a `kill` by a pid: a pipeline across newlines, `ps -fp`, `pkill -F`, and `kill "$(cat job.pid)"` after an unrelated `ps` | A newline after `|` continues it; `-p` in any cluster; `-F` is by a pid; a substitution alone is judged by what it ran, which is why `Command.own_from` exists |
| G7-7 | gap | "names this repository's path" was a substring, so `archogen-other` passed | Exactly the root, as G2-2 answered for `--root` |
| G7-8 | gap | the hook told only the last five stops, silently | It says how many came before them and where they are |
| G7-9 | wording | "asked first" left out that what is still there a second later is stopped outright, and that under pressure it may not run its handler | Said |
| G7-10 | wording | "1.2 MiB/s read nothing for ten seconds" and "up to 49 MiB" did not hold: the figure reads nothing until about 8 MiB has gone, 1.2 MiB/s stopping at 6.7 s, and a fall reached 67.1 MiB | Each as measured, with the reader's rates; the largest ten-second fall, 355 MiB, leaves the GiB its margin |
| G7-11 | wording | line 79 read as if a large job went only at the danger line, where one that is our largest goes at the stop line | Said |
| G7-12 | wording | "`Limits`, in one place" was not true of the fall, the windows and what counts as writing, which no mark held | Said, and `HOST-GUARD` holds marks for each |
| G7-13 | gap | text read as a command: a function after a keyword, `bash -n`, `command -pv` | Each read as what it is |

Said, and left as it stands: its reader could not run a real full volume, a real `make focused`, a commit's compile, chosen pressure levels, the true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, or an executable run by a path in another case. It stated that RAM reached 87.6% twice from another project; that it paused the first time until 47%, and that the second time `rig_measure.py`, a 100 MiB buffer, ran in the same command as the status reading, against its brief, after which it paused; that its scratch watcher stopped the first runs of a rig whose shell stood inside its scratch root; and that it used `git grep` and `git ls-files` on the working tree alone. It also said the brief's `--root … VERB` shape is refused, the verb coming first: a slip of the brief, which the record does not state.

**Round 8** — `2026-11-06`: not acceptable, 14 findings, 1 defect, after 15, 12, 5, 6, 13, 2 and 1. What it re-derived held: RAM by hand 78.58% against 78.62%, a 64 MiB buffer 70.9 MiB against `footprint`'s 71, `run`'s statuses, the watcher's pace, a script of `rm` under a floor 7 runs of 7 finished. It checked one of round 7's "could not" for the first time: an executable and a working directory reached by a path in another case both read as ours. Round 7's answers held but for what is below.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G8-1 | defect | `kill` of a process group found by name passed, `kill -- -$(pgrep -x rustc)`, since every word beginning `-` was dropped from its targets, and `CLAUDE.md`'s "kill stragglers AND their children" invites that form | `kill_targets` reads the words as `kill` does: a signal before any target, `--` ending them. `-$(…)` is a target, and judged |
| G8-2 | gap | G7-6's `own_from` opened a search through a file or a function, where only a variable was stated | Stated, with the form: a rewrite costs more here than the accident it would catch |
| G8-3 | gap | "checks every shell command" was false of `Monitor`, which the matcher `Bash` missed | The matcher is `Bash|Monitor`, and a `Monitor` with a socket and no shell has nothing to read. `HOST-GUARD` holds the matcher |
| G8-4 | gap | a pipeline "named this repository" by what `grep -v` excludes and where `tee` saves | Neither counts. Any other word still does, stated |
| G8-5 | gap | "exactly" failed where a space follows the root, in `names_root` and `names_this_root` | A flag follows a root or nothing does, as `main` now always writes it; and a space does not end a path |
| G8-6 | gap | in a checkout whose path holds a space no process read as the guard, so its watcher was counted and stopped. This checkout holds none, and it may be moved | This interpreter and this file are known by their own paths |
| G8-7 | gap | "what only asks" was wrong both ways: `make -qp`, `-ns`, `-nj4`, `-j4 help` and `-C sub -n` read as builds, and `cargo xtask --help` not, whose alias compiles first | Clusters read as clusters, an asking flag before where it runs, and an alias's `--help` is the program's |
| G8-8 | gap | rare misses: `env -S'…'`, `env -vS …`, `EOF )`, and two from G7-2's rule, `;;#it's` and `x=(#it's` | Each read: in a `[[ ]]` a word begins after a blank alone, and elsewhere after a metacharacter too. `bash -c 'bash' <<'E'` is stated |
| G8-9 | gap | a guard saved with a syntax error ended the watcher | The watcher compiles it first and goes on as it was, and says so. The hook, being that file, cannot run, and `onFailure` refuses until it is repaired: stated |
| G8-10 | gap | a watcher recreated a root that was removed, which a removed worktree would meet | A root that is gone ends its watcher, and `state_dir` makes none |
| G8-11 | gap | the self-test failed falsely under CPU load, behind fixed waits, and nothing ran it | It waits on what it waits for, and `check_host_guard.sh --self-test` puts it in `integration`'s `self-tests` step |
| G8-12 | wording | "up to 67 MiB", "355 MiB" and the GiB's margin did not hold: 324 MiB in a half second and 1.2 GiB in ten | Each as measured. The line stands, the rare stop it costs stated |
| G8-13 | wording | the slow writer's rates did not reproduce | Said as a range over runs, and that it varies |
| G8-14 | wording | `kill -s TERM 123` after a search was refused where `-TERM` was not; a command refused by name starts no watcher; "an application" held more than it said | The first by `kill_targets`; the others said as they are |

Beside these, what running the self-test repeatedly on a loaded host taught, each found by its own output and answered at its cause, none of them the guard's: `jobs_of` took a `run`'s short-lived helper, or a child gone by the time `ps` looked, for the job; a case could read `run`'s refusal of a filled host as its own failure, which `run_refused` now says as not run; the second of two `make`s could still hold its turn as the next step began; and the real watcher, since these rigs stand under this root, stopped a stub named after a build mid-case, which `run_case` now says as not run, and only where the real log shows it did.

Said, and left as it stands: its reader could not run a real full volume, a real `make focused`, a commit's compile, chosen pressure levels, the true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, or `onFailure` with a real crash, so it could not say whether a broken guard's `hook` blocks. It stated that it compiled a three-line C sleeper under `work/c2`, made one `git init` fixture with no commit, edited scratch copies of the guard, read `target/host-guard/state/`'s listing and one file's time, used `pgrep -fl` for listing alone and `git grep` on the working tree; and that RAM reached 87.8% from another project, under the director's line, so it did not pause.

**Round 9** — `2026-11-06`: not acceptable, 10 findings, 1 defect, after 15, 12, 5, 6, 13, 2, 1 and 1. What it re-derived held: RAM by hand 45.72% against 45.72%, a 64 MiB buffer 70.7 MiB against `footprint`'s 71, 50 MiB written read 50.0, the watcher's pace, `run`'s statuses, and the slow writer first seen at 8.04 MiB at each rate. It read `kill_targets` over 92 forms, `make_targets` over 99, and the reader against each `bash` over 505, the `for` form below its only miss. The space in a path held on a real `Makefile` at the danger line. For the third round running the defect is in the last round's own answer, which is a reason to answer by what is run and not by what is read: a compile says little of a program.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G9-1 | defect | "a guard saved broken ends nothing" held only for a syntax error: a name no longer defined, an import that fails or a raise at its top was taken up and ended the watcher, with nothing logged, and a job under `run` was ended soon after. A half-done rename across two edits is enough. And "the hook cannot run" was false of a file emptied or cut short, which exits 0 | `code_runs`: the file reads the host and judges in a process of its own, and says so, before the watcher takes it up. What an emptied hook lets through is stated: it runs and does nothing |
| G9-2 | gap | a `for` header with no `;` before `do` lost its body's first command: `for ((…)) do pkill -x rustc; done` | A `do` or `{` after a `for` or `select` ends its header |
| G9-3 | gap | `pkill -P 1 -x rustc` passed as by a pid, and every orphan on the host has that parent; and `pkill -v -f ROOT` as naming this root, stopping all that is not it. `CLAUDE.md`'s "kill stragglers" invites the first | `-P 1` is no parent to go by, and what inverts names nothing |
| G9-4 | gap | false refusals: `kill -0` in a wait loop, which stops nothing and which `Monitor` now reads; `pkill -f 'ROOT.*rustc'`, which the refusal asks for; `make -j "$N" focused` | Each passes |
| G9-5 | gap | `real_stops_since` made a real failure "not run" on any stop of ours, another session's among them | Only a stop of one of this test's own |
| G9-6 | gap | `--self-test` turned 20 into 0, so `integration` read as passed with most of it unrun | It carries 20, and `run_self_tests.sh` carries it up as incomplete, which `xtask` reads as never passed |
| G9-7 | gap | a root removed and remade within one sample kept its old watcher beside a new one | The watcher holds the root's inode, and goes when it changes |
| G9-8 | gap | "`main` always writes it" held only for an unnamed root: a trailing slash, a relative path or no `--` made a `run` unknown | `run` writes the root's spelling and the flag in every case |
| G9-9 | wording | the code's comment still said 67 and 355 MiB | It says the record has them |
| G9-10 | wording | every not-run item was said to be the host over the start line | It says why, or that |

Said, and left as it stands: its reader could not run a real full volume, a real `make focused`, a commit's compile, chosen pressure levels, the true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, or `onFailure` with a real crash. It stated that one rig listed two watchers of its own scratch root with `pgrep -f` of that exact path and signalled those pids, against the brief's "never `kill` fed by a search", reaching only its own; that a stub loop of its orphaned for some 40 s, stopped by its number; and that it fetched the harness's hooks reference, grepped the installed binary for `onFailure`, and read `settings.local.json`'s keys and `target/host-guard/state/`'s listing.

**Round 10** — `2026-11-06`: not acceptable, 12 findings, 3 defects, after 15, 12, 5, 6, 13, 2, 1, 1 and 1. What it re-derived held: RAM by hand 60.17% against the guard's 60.09%, a 64 MiB buffer 70.9 MiB against `footprint`'s 71, 50 MiB written read 50.0, `run`'s statuses, the watcher's pace, and the reader against each `bash` over 259 and 265 forms. `run` always writing its root held over 11 spellings, and `kill -0`, `-P 1` and `inverted` over 128 forms but for G10-8 and G10-9. For the fourth round running the main defect is in the last round's own answer, so this one is answered by method and not by more names: a walk of the compiled file, held by a case that takes every function a function calls as `ast` finds them, and the watcher's own loop run in the proof.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G10-1 | defect | "one saved broken leaves the watcher going as it was, and says so" held only for the names `prove` walked. Renaming only a `def`: `hold_lock`, `write_note`, `code_stamp` or `watch` passed the proof and ended the watcher with nothing logged; `sample`, `baseline_of` or `note_peak` left it never reading again; `record`, `stop` or `tree_of` left a job over the line running, and a job under `run` was ended at 125 s | `names_not_found` walks the compiled file: every name a function reads as a global is defined, whichever path reaches it. `prove` also goes round `watch` itself, `sample`, `tree_of`, `stop` and `record` over a root of its own. A pure case renames every function a function calls, as `ast` finds them, and real watchers are saved five such files. A line that goes wrong in another way and only at a stop is stated as what no proof sees, and a watcher ended by its own error now says so |
| G10-2 | defect | "incomplete and never passed" was false of `integration`: `xtask` reads a 20 no quarantine row names as failed, and 4 cases not run at 72–75% of other projects' RAM would turn the tier red | `xtask`'s rule stands, since no step may exempt itself by its exit code, and the record now says what it does. And the cause is answered: the self-test waits for room, up to fifteen minutes in all, before its cases and before saying what the host took away, which goes again with room |
| G10-3 | defect | "another project's process is never touched" failed after `watch --root <relative>` met any save: it went to `/`, then took itself up with its first arguments, so `sub` ended it and `.` made `/` its root, where 11 processes read as ours | `main` names the root as resolved in what the watcher takes itself up with |
| G10-4 | gap | with the guard emptied or cut short, `make` reported a pass having run nothing | `make` asks `prove` before any recipe, and refuses; `HOST-GUARD` holds that line; a case shows the refusal, and that a guard that runs goes on |
| G10-5 | gap | two saves inside one proof went wrong: the stamp was taken after the proof and not looked at again, so a good file after a broken one was never taken up, and a broken file after a good one ended the watcher | The proof is of the stamp it was asked of. Saved again meanwhile it says nothing, and the new file is asked; the stamp is read again at the last moment, and the instant left is stated |
| G10-6 | gap | a proof that failed for a moment called a good file one that does not run, and nothing asked again until the next save | A file is asked again each minute, `proof_again_seconds`, and said once |
| G10-7 | gap | a save that hangs stopped all reading for the proof's 60 s, unstated beside "reads every second" | The proof stands beside the watcher, polled each round, and `sampled` goes on moving. A case holds it |
| G10-8 | gap | round 9's answer held only where `for` stood first among the words: under `if`, `while`, `{` or `time` the body was lost, and `for w in do pkill; do …` was refused for a list word | `a_loop_header` looks past the keywords before it, and after `in` a word is a list's. Each form is in the oracle over each `bash`, and in the hook's cases |
| G10-9 | gap | `-P` in a cluster was not read, where `ps -fp` was: a false refusal of `pgrep -lP "$PID"; kill "$PID"` and `pkill -fP 123 sleep` | `by_pid` reads a cluster. Found in answering, by this writer and not the reader: `pkill -v -P 123`, which stops all that is not a child of 123, had passed by being "by a pid", and `-P 5,1` hid the 1. Each is refused |
| G10-10 | gap | `real_stops_since` knew only the rig's own pids, and a stopped line under a rig's `run` names the job | A stopped line says the job's parent, and a job whose parent is a rig is the rig's |
| G10-11 | gap | on a host not macOS the self-test counted as passed with none of it run | It exits 21: counted apart, never as passed, allowed to this gate alone, and nothing else run is no pass. Arms hold each, and the 20 this runner's contract states |
| G10-12 | wording | round 8's "the self-test waits on what it waits for" is partial: 58 `time.sleep(` remain, `named`'s 0.15 s among them, none seen to fail | Said here. What round 8 replaced were the waits for a job to begin; the rest are a rig's own pauses |

Found in answering, and answered: the case that said "repaired, it holds from then" left its marker at the top of the file, which the proof also runs, so it could pass with the watcher never having taken the file up. A marker now says `watch` is what ran it, and with what root.

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, `xtask`, a real full volume, chosen pressure levels, the true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, or `onFailure` with a real crash. It stated that it listed by `pgrep -P` and `ps | grep -F` and signalled nothing from them, and that it left no process.

**Round 11** — `2026-11-06`: not acceptable, 8 findings, 2 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1 and 3. What it re-derived held: RAM by hand 64.60% against the guard's 64.5%, a 64 MiB buffer 70.9 MiB against `footprint`'s 71, 50 MiB written read 50.0, `run`'s statuses, the watcher's pace, a slow writer first seen at 7.1 s, the reader over 263 and 269 forms, and the self-test whole, `752 passed, 0 failed, 0 not run`. `a_loop_header` held over 74 new forms on both `bash`es, `Saved` over a hung proof, a stopped watcher, twelve saves and a root removed mid-proof, and `run_self_tests.sh` over each mix of its exits. For the fifth round running the main defect is in the last round's answer, and four of these eight are in `make`'s new `prove`: so the answer takes that out, rather than holding it up.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G11-1 | defect | at the danger line or under a disk's floor `make` was stopped during its new `prove`, since a `make` waits only with a `run` child, so "`make` waits for room by itself" was false, and a script around it went too | `make` runs no proof. It looks at the guard's last line with `tail`, which is all that "emptied or cut short" needs, one broken any other way exiting with its error |
| G11-2 | defect | "two things no proof sees" was false: `prove` called `watch` with no arguments and past `main`, so a file that passed ended the watcher, by a field `follow_the_file` reads or a line on `main`'s way in; and a misspelt attribute in `danger_line` passed on a calm host | A proof goes in by `main` as a watcher does, and what its round says as `unread` is its answer. No line of a round ends a watcher now, so `follow_the_file` going wrong is said and reading goes on. The record says what no proof sees as it is: what one calm round does not reach |
| G11-3 | gap | the proof's two readers disagreed: a warning on standard error made a file that runs read as not running, while `make` read standard output alone | The watcher reads standard output for the answer, and what is beside it only to say why. A file defining a name by `globals()` or a star import is stated |
| G11-4 | gap | `make`'s question was wider than "does this file run", refusing for a closed state directory or a failing `ps`, and had no time limit | Gone with G11-1 |
| G11-5 | gap | `HOST-GUARD` held the proof's text and not the question: commented out, `$(warning` or `ifeq` passed | It holds the three lines whole, each at a line's start, and that the guard ends with the line they look for |
| G11-6 | gap | `-P` or `-F` fed by a search by name passed: `pkill -P "$(pgrep -x make)"` stops the children of every `make` on the host; and `-opid` read as `-p` | What a `pkill` goes by is read as `kill`'s target is. Past an `o`, a `p` is what `-o` shows |
| G11-7 | gap | the self-test's wait for room was silent, read the host some 16 times a second, and could outlive its runner | `wait_for_room` says so first, looks every five seconds as `run` does, and stops if its runner has gone |
| G11-8 | gap | the re-run had no pause and repeated a case's own "not run", and the disk-floor case writes gigabytes a go | Only what the host took away goes again, and twice more at most |

Found in answering, and answered: the whole self-test failed on a loaded host, `769 passed, 3 failed, 2 not run`, each failure a `run` refused for load that had gone by the time it was looked at, and two cases' own "not run" went unretried. What the host did in the last half minute now counts as much as what it does now, and each such case says the host took it away. A real failure stays one, since it goes again with room and fails on a clear host too.

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, `xtask`, a real full volume, chosen pressure levels, the true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, or the interpreter-gone path, its symlink resolving past. It stated that seven `prove` runs with no `--root` made and removed `proof-PID` under the real `target/host-guard/state/`, against its brief and leaving nothing, and that a rig signalled a hung proof of its own watcher found by `pgrep -P` of that watcher's pid.

**Round 12** — `2026-11-06`: not acceptable, 12 findings, 3 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3 and 2. What it re-derived held: RAM by hand 75.51% against the guard's 75.66%, a 64 MiB buffer 70.8 MiB against `footprint`'s 71, 50 MiB written read 50.0, `run`'s statuses, the watcher's pace, and the self-test whole, `746 passed, 0 failed, 0 not run`. `goes_by_pid` and `-F` held over 97 forms but for G12-8, and `tail` over each way a file can be whole or not. For the sixth round running the main defect is in the last round's answer, and all but a few of the last four rounds' findings are in a watcher replacing itself. That guarded a moment a save and a next command already cover, so the answer is to take it out, as round 4 took out the lists that kept failing: `names_not_found`, `prove`, `Saved` and `follow_the_file` are gone, with their cases and two limits.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G12-1 | defect | a repair was not taken up while a round raised, `follow_the_file` standing after `sample` in one `try`, so "holds from then" was false where it mattered most, and only `stop` freed the lock | A watcher leaves when its file changes, outside any `try`, so nothing it is doing holds it. The next command or `run` starts one from the file. `case_a_guard_changed_holds_from_the_next_command` and `case_a_job_under_run_outlasts_a_changed_guard` |
| G12-2 | defect | "`make` waits for room by itself" was still false: it read as a build until its `run` had re-executed, over parsing, `tail` and an interpreter's start: 3 of 25 stopped at a danger line of 1% | A `make` that has started at most a shell, the guard and what reads the host or its file holds no build |
| G12-3 | defect | a real failure read as not run: `room()` returned at the first clear look, so a short case's three goes all fell inside the half minute, and cases no host state touches were excused too | Room now means clear for as long as a failure is excused, so a real one fails on its next go. The last check, about bytecode, is pure |
| G12-4 | gap | a blind watcher kept `busy` as last read, so it might never end, and read at its fastest | Blind, it reads at its slowest, and stays for a job it last saw no longer than it would stay idle, since that job may still run |
| G12-5 | gap | a proof did not check it went round | Gone with `prove` |
| G12-6 | gap | what a round says as `unread` reached nobody but the log and `status` | Stated: a heavy command is refused, a job under `run` ended, and nothing tells a light command |
| G12-7 | gap | `make` and `HOST-GUARD` disagreed on a trailing blank line, so a commit could pass and refuse every `make` | `HOST-GUARD` reads the end as `tail -n 1` does |
| G12-8 | gap | false refusals: `pkill -g 123` and `ps -g` by a group, `kill -- -$$` after a `ps`; rare misses: `pkill -PIPE`, `pkill -FPE`, `ps -upeter`. And `ps -ax \| head; pkill -F job.pid` refused, where `kill "$(cat job.pid)"` passes | `-g` goes by a group; `-$$` is plain; a signal's name is no cluster; a flag taking a value is not read past. The last stands, and is stated here: a pid file reads as a variable does, any search before it naming nothing, since what wrote the file cannot be read from the command |
| G12-9 | gap | only the first case waited for room; two lookers made 1.24 looks a second; a re-run inflated "N passed" | Each case waits; the one beside the cases looks every three seconds; a case's count starts again with it |
| G12-10 | gap | under `integration` the runner dropped the names of what failed and what did not run | It prints them. Its silence through a long wait is stated: it holds a gate's output to the end |
| G12-11 | wording | "each line judges alone" and "only at a host's line" undersold a raise, which silences the round | Said in the record |
| G12-12 | wording | "one broken any other way exits with its error" was too wide: `sys.exit(0)` at its top passes | Said in the record |

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, `xtask`, a real full volume, chosen pressure levels, the true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, or the interpreter-gone path. It stated that its self-test went through the real root's `run`, that one rootless `status` was read, and that a loop bug in a rig was ended by its own pids.

**Round 13** — `2026-11-06`: not acceptable, 11 findings, 5 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2 and 3. What it re-derived held: RAM by hand 49.90% against 49.92%, a 64 MiB buffer 71.0 against `footprint`'s 71, 50 MiB read 50.0, a slow writer seen at 7.3 s, `run`'s statuses, the pace, and the self-test whole, `645 passed, 0 failed, 0 not run`. `only_waiting` held, none of 25 `make`s stopped at a 1% danger line, and `A_PLAIN_TARGET`, `-g` and `ps`'s flags over 120 forms but for G13-9 and G13-10; 81 ordinary commands went as before; `wait_for_room` and the excuse held with the host's states stubbed. Round 12's own answer opened the main defect, a moment with nothing watching. What is kept of that answer is that no proof stands in the way; what goes is the leaving.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G13-1 | defect | "this repository's commit checks and the watcher are what stand then", of a guard emptied or cut short, was false: the watcher left at the save and nothing could start another, `pkill -x rustc` and `cargo build` then passing | A watcher does not take up a file that can be seen to be cut short or not to compile: it goes on as it was and says so, and the sentence holds |
| G13-2 | defect | "holds from the next command" failed where that command's hook came before the old watcher's next look: `ensure` saw it alive, it left, and a bare job ran unwatched for its whole life, 6 of 6 | It takes itself up from the file, so the lock passes with nothing unwatched between. `case_a_guard_changed_holds_from_then` |
| G13-3 | defect | "saved broken, what comes next fails or refuses" was false of a break only the watcher's path meets, a half-done rename: hook, `run` and `check` passed | Said as it is: it ends the watcher, a heavy command is refused after a minute, a job ended a minute later, and a light command runs |
| G13-4 | defect | "ended once no watcher has read the host for a minute" is two: 60 s to stale, then 60 s more | Said as it is, some two minutes |
| G13-5 | defect | "`check` and `status` say what `run` and the hook would" was false beside a blind watcher | They say what `run` would, and `status` says blind |
| G13-6 | gap | a `run` waiting its turn started no watcher, and its job's first came 5 s in | It starts one as it waits, and at once with its job |
| G13-7 | gap | the runner's `head -12` counted "waiting for room" lines, and one dying by a traceback printed no reason | Those lines are left out, and what says nothing gets its last lines |
| G13-8 | gap | a watcher kept its starter's `PATH`, so one narrow `run` left it blind to everyone | `ensure` adds the system's to it, what is first staying first |
| G13-9 | gap | `kill -9 -1` and `kill -- -1`, every process of this user's, passed | Refused |
| G13-10 | gap | false refusal of `ps 123` and `ps up 123` before a `kill "$x"` | A pid written plainly is no search |
| G13-11 | wording | self-test leftovers | Gone |

Found in answering, and answered: an image coming up took its stamp after reading its file, so a save in that moment went unseen until the next, and the self-test caught it; what starts a watcher now tells it the stamp from before. `kill -9 -1` read both as signals and passed: one signal comes before a target, and what follows is one.

Said, and left as it stands: its reader could not run what the brief lists, saw hook rc 1 but not what `onFailure` does with it, and ran `run_self_tests.sh`'s seams and not its `--self-test`, which writes outside its work directory. Rootless `status` was read four times.

**Round 14** — `2026-11-06`: not acceptable, 10 findings, 2 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3 and 5. Its reader's rigs are under `target/p12714/work/`; it ran the self-test once, `657 passed, 0 failed, 0 not run`, and 125 ordinary commands through the real hook. Round 13's answer held where it was aimed: whole saves 0 to 120 ms apart ended no watcher and lost no save, and a heavy command was refused and a job ended at the times the record says, on the true clock.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G14-1 | defect | "`HOST-GUARD` and the self-test hold such a file at the commit" and "the rest is what the commit holds" were false: a commit runs `check_host_guard.sh` without `--self-test`, which parses the file and no more. Over `def hold_lock_(`, `host-guard: OK`, rc 0, and that copy's watcher a `NameError` | A commit now holds what it was said to: by the compiler's own table, every name the guard reads is bound. All 15 forms of the reader's rig are refused, and the self-test holds that. What the file does when it runs stays the self-test's, in `integration`, and the record says so and that no commit runs it |
| G14-2 | defect, rare | "is not taken up" and "leaves no moment" failed in one window: a broken save landing between `file_is_whole`'s read and the new image's read ended the watcher, nothing logged; with the saves aligned to the look, 4 or 5 of 5 up to 40 ms | A stated limit. Closing it takes something beside the watcher to read, run and report the new image, and that is the proof five rounds took out. It needs two saves within the start of a Python, the second broken, and then the watcher is gone with what follows as the record says of any watcher gone |
| G14-3 | gap | "this repository's commit checks and the watcher are what stand then": the watcher only if one was running, and only until idle. Nothing starts another | Said so |
| G14-4 | gap | G13-10's `\d+` took any word of `ps` as a pid, so `ps -U 501 … \| xargs kill` and `kill $(ps -u 501 \| grep rustc …)` passed, beside `-G 20`, `-t 1`, `-O 1` | What a flag takes is that flag's. `-w 1` stays a pid, `-w` taking nothing, which is what `ps` does with it |
| G14-5 | gap | false refusal of a pid's ordinary spelling: `ps "$PID" && kill "$PID"`, `ps $PID; kill $PID`, `ps -f "$PID"; kill "$PID"` | A variable where a pid goes is a pid. A search spelt to look so is a way round, as any is, and the record says so |
| G14-6 | gap | G13-7's answer failed after a wait: the second `grep` matched the waiting line and silenced `tail -3`, so a gate that waited and died by a traceback printed no reason, and the first `grep -v` also dropped any `FAILED` line holding those words | The waiting lines come out first, by their own start, and the rest is judged. A new arm of the runner's own self-test holds it |
| G14-7 | gap | after a repair `status` still showed the broken note, and nothing said the file was taken up | A taking up is recorded, `taken-up`, so `status` shows it after the note |
| G14-8 | wording | "a half-done rename ends the watcher" held of 6 of 15 names: 2 left it blind and 7 left it reading on | It says where each goes. A commit refuses all 15 now |
| G14-9 | wording | "no longer than it would stay idle" is about twice, 10.7 s at an idle of 6 | Twice |
| G14-10 | wording | "`kill` of numbers pass" beside `kill -9 -1` refused, which the record never stated; and it is no stop "by a name" | Stated, and the three things say "or all at once" |

Found in answering, and answered: the host's real watcher was 9 h 38 min old and had run an image from before `prove` came out, since `2026-11-06T08:21:01` saying `as saved does not run` of each new file, which had no such verb, and going on as it was. So rounds 12 to 14's watcher had never run here but in the self-test's copies. Its lines were the same, and it stopped what it should at each line through those hours. It was stopped by `stop`, and the next command started one from the file. `status` now says when a watcher runs an older file than the one saved, which nothing did, and the record says a change to the taking up itself cannot reach one older than it.

Said, and left as it stands: its reader could not run real `make focused`, a commit's compile, `xtask`, a real full volume, chosen pressure levels, true idle or true blind life, the census against the real watcher, real Codex, Kimi or VS Code, or `onFailure` with a real crash; it ran `run_self_tests.sh`'s seams and not its `--self-test`. Its rigs left two scratch watchers and a waiting `run`, which it stopped by typed pid; nothing of its own was left, and RAM peaked at 76%. It did not re-derive the oracle's "some two hundred and sixty" forms, counting the two constant lists as 155 + 6, and the case passed.

**Round 15** — `2026-11-06`: not acceptable, 11 findings, 4 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5 and 2. Its rigs are under `target/p12715/work/`; it ran the self-test once, `702 passed, 0 failed, 0 not run`, and re-derived the measurements, all of which held. Plain names hold, 122 of 122 refused. The count rose, and each of the first two was a sentence of mine about what a commit holds, so this answers by method: what a commit holds is run, and what it does not hold is not said.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G15-1 | defect | "holds that the file compiles" was false: `ast.parse` and `symtable`, never `compile`, so a module-level `return`, `break`, `continue` or `yield` passed, rc 0, and the file itself is a `SyntaxError` | It compiles it |
| G15-2 | defect, fails open | "binds every name it reads" was false of attributes, properties, methods, fields and keywords, 66 names read 326 times: 8 of 8 forms passed, and `job.bytess` in `danger_line` left a job alive past a line | "Every plain name" is what it holds. What reads the rest is the self-test, so a commit changing the guard carries one passed on those very bytes, by a stamp that run leaves, as a focused run stamps a staged tree. A line no case runs is still read by nothing, and the record says so |
| G15-3 | defect | `status` said a false thing after `cargo clean`: `watch.pid` was rewritten when lost and `watch.code` was not, and a note that is no text was a traceback | Both are rewritten, as what the image is. A note that is no text reads as not the file |
| G15-4 | defect, rare | one whose round raises rewrote nothing after a clean, so it was "not running", could not be stopped by `stop`, and the hook passed | What names it is rewritten outside the round |
| G15-5 | gap | `ps -ef -p 1` prints the whole table and read as a pid, beside `-Ap`, `-axp`, `-ep`, `-Ao pid= -p`, `-A -g`, `-axg`, `-ag`, `-G 20 1`; and `ps $$`, `ps "$(cat job.pid)"`, `ps "$@"`, `ps "${PIDS[@]}"` were refused | What selects more than a pid makes it a search, whatever stands beside it; a pid is taken however a shell spells one |
| G15-6 | gap | "at a commit" held the working file, so a half rename staged with the tree repaired would pass, and another session's half-saved file would refuse an unrelated commit | The file as staged, in its own repository |
| G15-7 | gap | a taking up forgets growth's baseline and the last sample, unsaid, each save restarting the quarter-hour | Said |
| G15-8 | gap | the runner's "says why": a gate a signal ended printed no status, and `FAILED` above a traceback lost the traceback | The exit is said, and the last lines always |
| G15-9 | wording | "a watcher older than a change to how a watcher takes itself up cannot take it up" is false where the old look accepts the new file | It says where the look refuses |
| G15-10 | wording | "anywhere else it reads on" is not so of what the loop reads outside its round: `code_stamp` ended it, and `file_is_whole` read on until the repair ended it. A commit holds both names now | It says so |
| G15-11 | wording | `kill -1` "whatever its signal", beside `kill -0 -1`, which sends none; the Review table had row 14 before row 13 | Said, and in order |

Said, and left as it stands: its reader could not run real `make focused`, a commit's compile, a real full volume, chosen pressure levels, true idle or blind times, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, the interpreter-gone path, or `run_self_tests.sh --self-test`. A safety check refused one rig command, which it reissued without the removal. It read `watch.code` once, and left nothing running.

**Round 16** — `2026-11-06`: not acceptable, 11 findings, 4 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2 and 4. Its fixtures are under `target/p12716/work/`, each a disposable repository whose pre-commit runs the check, and it committed only there; it re-derived the measurements, all of which held. All four lay in what round 15 made a commit hold, as round 15's lay in round 14's: the method stands, a thing run in place of a thing said, and what it had not run yet is what fell. The stamp is a dozen lines, and it stays: without it a commit holds nothing of what the file does.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G16-1 | defect, fails open | one case by name left the stamp: `job.bytess` staged, `selftest case_what_counts_as_heavy`, 153 passed in 0.12 s, and the commit passed | A case by name leaves none, and says so |
| G16-2 | defect | the stamp hashed the files when the run ended, so a save during it was stamped: a self-test appended to at 14:27:46, committed unrun | It hashes what the run began on, and leaves none if that changed |
| G16-3 | defect | a copy with no `.git` stamped the real repository's bytes, never run: `ROOT` is the nearest parent with `.git` | A copy leaves none in the repository it lies in |
| G16-4 | defect | "refuses a commit that takes the hook off" was false: only the guard was read as staged, so `{"hooks":{}}` staged with the tree restored passed, and `focused` off the guard likewise; and a half-saved `Makefile` refused an unrelated commit | All it reads is read as staged |
| G16-5 | gap | `git rm --cached` of the guard passed, the tree standing in; `git rm` of the self-test was a traceback | What the index no longer holds is gone, and said |
| G16-6 | gap | it compiled text decoded leniently, so `# \xff` compiled where Python refuses | It compiles the bytes |
| G16-7 | gap | false refusals: `-p$$`, `-p$PID`, `` ps `cat job.pid` ``, `ps "$a,$b"`, `-opid= "$PID"`, `-xp 123`, which prints 2 lines | Each is a pid, and `x` beside a pid selects no more |
| G16-8 | gap | a merge or a cherry-pick runs no pre-commit, and CI has nothing staged, so "at a commit" did not say they hold nothing | Said, with how this repository lands |
| G16-9 | gap | by the volume's other spelling the check skipped the stamp, `Path.resolve` keeping it | The same place is asked of the filesystem |
| G16-10 | gap | "last lines always" held of a failure alone: `echo boom; exit 20` said nothing | It says what it said last, and an arm holds it |
| G16-11 | wording | "a table searched" and "`kill -1`": what is refused is `kill` of them, and `kill -1` alone names no target | It says so |

Found in answering: the stamp's first run refused to stamp itself, `what ran changed while it ran`, since a later line of `main` reused the name the starting hash was held in. It failed closed, as it should, and the name is its own now. And round 15's watcher took each of these saves up as they came, `status` saying so, which is the first this host has run of what rounds 12 to 15 said.

Said, and left as it stands: its reader could not run the brief's list, `main` in a copy with no `.git`, which would have written the real stamp and so is by import, or `run_self_tests.sh --self-test`. `ps -w ax 1` passes, and `ps` itself rejects it. An inherited `GIT_DIR` makes a copy read that repository's index, on no commit's path. It read `status` several times at the real root, and left nothing running.

**Round 17** — `2026-11-06`: not acceptable, 8 findings, 1 defect, mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4 and 4. Its fixtures are under `target/p12717/work/`, and it committed only there. Round 16's stamp and what it reads as staged held through `git commit -a`, a path commit, a linked worktree and the volume's other spelling. Of 106 `ps` and `kill` forms only `-M` and `-d` were off, and it re-derived the measurements, all of which held. Its whole self-test ended `746 passed, 0 failed, 2 not run`, rc 20, `case_run` taken away and retried twice under load, and left no stamp, as the record says.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G17-1 | defect, mild | "`check` and `status` say what `run` would" was false beside a `run` whose job is the self-test: `heavy_running` dropped children matching `GUARD_PROCESS`, meant for the watcher, and `selftest_host_guard.py` matched it, as any name ending so did: `check` clear, `run --wait 0` refused | This file's name, by a path's last part, and not any name ending in it |
| G17-2 | gap, fails open by two rare events | all is read as staged but the check itself, which runs from the tree: a hook taken off and staged, the check emptied in the tree, rc 0; and an unstaged edit of it refuses an unrelated commit | Said: something must run from the tree, and this does. It takes that file emptied and a hook taken off in one commit |
| G17-3 | gap | `-d` is "like `-A`, but excludes session leaders", and read as a plain flag: `ps -dp 1` prints 91 lines | It selects |
| G17-4 | gap | false refusal: `ps -M "$PID"; kill "$PID"`, `-M` taking no value here | It takes none |
| G17-5 | gap | a stamp compares the start and the end, so a save and restore during a run is not seen; and a copy whose nearest repository has no `scripts` raised after a passing run | The first is said. The second says why not, as the others do |
| G17-6 | wording | "a merge or a cherry-pick": neither do a revert nor a rebase | All four |
| G17-7 | wording | "`make` and this repository's commit checks are what stand then" no longer said what they hold, the check reading what is staged | It says what each does |
| G17-8 | wording | "where the commit changes it": three files; "beside a job of ours": a build or a `run` | Each says so |

Said, and left as it stands: its reader could not run real `make focused`, a real full volume, chosen pressure levels, true idle and blind times, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, `run_self_tests.sh --self-test`, or re-derive `ri_logical_writes`. Its idle pace read 1.73 s against a stated 2, unexplained. `run` at the real root moved `seen.run`, so a later `run` did not repeat a stop already said. A safety check refused one rig command, which it reissued without the removal.

**Round 18** — `2026-11-06`: not acceptable, 7 findings, 2 defects, one mild, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4 and 1. Its rigs are under `target/p12718/work/`. It timed what earlier readers could not: true idle ended at 600 s; a blind watcher refused a heavy command at 64 s and ended a job at 125 s, on the true clock, and every other measurement held. Round 17's fix did what it said, and the whole self-test passed twice, `756 passed, 0 failed, 0 not run`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G18-1 | defect | "what is never stopped, nor counted" said the guard and its self-test, and round 17 had taken the self-test out: `protected` false of it, and `status` counted it | It is spared by its own name, exactly. A `run` of it is a job for what `check` and `status` say, and that is all |
| G18-2 | defect, mild | so a line that reached the self-test ended it whole, with no summary, no "not run" and no exit 20, against the record: `rc=143`, through the runner too. The danger line reached it while a stub was up, and a disk floor by the code | The same answer |
| G18-3 | gap | a self-test stopped so left rigs behind, a scratch watcher staying until idle | The same answer: it is not stopped. A kill of it by hand still leaves them, and that is a person's |
| G18-4 | gap | dashless `ps` selecting a whole table passed: `ps U 501`, `ps G 20`, `ps dp 1`, of the family of G14-4 and G17-3 | Read as dashed. A `u` is a format, so `ps up 123` stays a pid |
| G18-5 | gap | the self-test's `jobs_of` kept the "ends in" logic round 17 took from the guard, so its own case burned 20 s of a rig's 30 | It asks the guard's own reading |
| G18-6 | wording | "`check` and `status` say what `run` would" has a window of up to five seconds beside a `run` waiting its turn, which holds `job.lock` that `check` does not read | Said |
| G18-7 | wording | "each told how to wait": a refusal with no watcher says to look at `status` | "What to do" |

Left as it stands, said here: "all of it can be stopped with no table to read" misses a child with its own session, which is how the self-test's rigs stand; a `kill` of a `run` leaves such a child, and the watcher sees it as ours by its root. Its reader could not run real `make focused`, a commit's compile, a real full volume, chosen pressure levels, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, or the blind watcher's true 20-minute stay. One `cp` of its wrote a copy of `settings.json` to `/tmp`, and the same command removed it. Both its self-tests rewrote the real stamp for the bytes they ran on.

**Round 19** — `2026-11-06`: not acceptable, 4 findings, 2 defects, both mild and neither in ordinary work on this checkout, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4, 1 and 2. No finding is fail-open. Its rigs are under `target/p12719/work/`: 95 `ps` forms against what real `ps` prints, 221 new forms through the reader against both bashes with none missed, and every measurement re-derived and held, a blind watcher refusing at 63 s and ending a job at 124 s. Its whole self-test passed, `764 passed, 0 failed, 0 not run`.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G19-1 | defect, mild | the self-test was not spared in a checkout whose path holds a space, where `GUARD_PROCESS` carries its own path for that reason: stopped at a line, `rc=-15`, and it failed its own case there | By its own path too, and the case asks that of whatever path it stands on |
| G19-2 | defect, mild and rare | at a floor a process was stopped for what it wrote before it was ours, a first sight counting its whole life, against "since the last sample": 40 MiB written elsewhere, then a `chdir` here and a sleep, stopped in 0.1 s | Said. Telling an old process from a new one takes its start, and what this costs is rare, at a real floor, and in the direction of stopping |
| G19-3 | wording | "all of it goes, short of an agent, a terminal or a `run`" omitted the self-test, which `tree_of` leaves | It names it |
| G19-4 | wording | a comment's two examples print one pid, `ps ax 1` and `ps -U 501 1` | It says so, and that they are refused with the rest, which costs a rewrite |

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, a real full volume, chosen pressure levels, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, the blind watcher's true stay, `run_self_tests.sh --self-test`, or commit fixtures for `HOST-GUARD`. Two self-tests at once share `target/host-guard/selftest/`, which it did not measure. A watcher blind from its very start refuses sooner, heavy at 6 s and a job ended at 70 s, failing closed. A slow writer at a floor stopped at 27 s and 16 MiB, just past the record's "some".

One whole self-test in answering this ended `758 passed, 2 failed, 1 not run`, with the host under other projects' load: `case_a_queued_make_is_no_build`'s `make` was gone at its check, its job's `--wait` being 40 s. The case then passed alone three times of three, and the whole run passed on a host at 50%, `765 passed, 0 failed, 0 not run`. That its 40 s ran out is the likely reading and is not shown, and the next reader may hold it to that.

**Round 20** — `2026-11-06`, leaf `PROGRAM.127.20`: not acceptable, 10 findings, 2 defects, after 15, 12, 5, 6, 13, 2, 1, 1, 1, 3, 2, 3, 5, 2, 4, 4, 1, 2 and 2, on `ee53e991`. No finding fails open in ordinary work. Round 19's answers held: a spaced checkout's self-test spared by its path, a floor's counting, 110 forms of `ps` against the real one.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G20-1 | defect | a `make` queued in its `run` was read as a build when `ps` showed a child of the `run` as `(sysctl)` or `(Python)`, 5 looks in 717, so it was stopped at the danger line at 10, 137 and 238 s, and `heavy_running` counted it. The case held one sample | `reads_the_host` says what `ps` brackets for a moment is what it would be, and a `(cargo)` stays a job. `case_what_ps_cannot_read_says_no_job`. A candidate for round 19's `758 passed, 2 failed`, not shown |
| G20-2 | defect, mild, rare | a `run` of another root whose job says `--root THISROOT` was read as this root's | `names_root` reads a guard's words before its `--`. `case_a_job_naming_a_root_is_not_that_root_s` |
| G20-3 | gap | the reader did not read through the guard's own `run`, where a refusal sends an agent | `kills_by_name` reads what a `run` is told to run. A substitution inside it is a stated limit |
| G20-4 | gap | `kill -9 -01`, `kill -9 $((-1))`, `pkill -P 01` passed as literal spellings | Read as numbers |
| G20-5 | gap | `lsof -c` and `top` listed the table past the search | Read as searches; another program is a stated limit |
| G20-6 | gap | `env -` hid what it wraps | It is `env -i` |
| G20-7 | gap | `override GUARD`, `sinclude`, a plain line in `Limits` and one from outside it passed `HOST-GUARD` | Each is refused, `$(eval)` too. `case_a_commit_holds_what_make_and_the_limits_can_be_told` |
| G20-8 | gap | in a path holding a space a directory's name could make a job an agent | `words_read` takes the path whole, by the file standing there. `case_a_path_holding_a_space_is_one_path` |
| G20-9 | wording | "short of an agent, a terminal, a `run` under it or the guard's self-test" omitted the rest of `protected` | It says what is protected |
| G20-10 | wording | "by a file named…" holds only for flags with no value and a path with no space, unless by this checkout's own path | It says so |

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, a real full volume, chosen pressure levels, true idle time, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, a 1 GiB fall, the blind 20-minute stay, or two self-tests at once. A self-test of its own read `763 passed, 0 failed, 1 not run`, rc 20, when the host took `case_a_job_under_run_outlasts_a_changed_guard` away three times, and `765 passed` on the next. A `watch --once` with its cwd in the scratch root stopped its own tool shell, its rig's error.

**Round 21** — `2026-11-06`, leaf `PROGRAM.127.21`: not acceptable, 10 findings, 3 defects, after 2, 2 and 2 on the rounds before, on `640b8c3f`. The count did not fall, and G21-1 fails open, so this answers by method: what a text-reading check was passed by, ask the tool. Round 20's answers held past a make's first moments: 425 looks, none misread.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G21-1 | defect, fails open | `HOST-GUARD`'s "says anything that could without a recipe changing" was false: `focused: GUARD := true`, `quick focused:`, `${eval}`, `MAKEFLAGS += GUARD=true` and a later `SHELL` each passed, and `make focused` then exited 0 having compiled nothing | The check asks `make -n` of each recipe and `make -pn` of its shell and flags, on the staged Makefile, as the reader asks bash of a command. `.IGNORE` and `-i` read there too |
| G21-2 | defect, mild | a `make` at its first moments was read as a build: `(env)` and `(bash)`, 9 looks in 447 up to 108 ms after its start, 5 stopped in 400 | `says_no_job`: what `ps` brackets is a moment whatever it is, but for a build by its name. At most one look late, a stated limit |
| G21-3 | defect, mild | "reads each through the guard's own `run`" was false of a `run` with no `--`, which `main` accepts, and of a search inside one | `what_a_run_runs` reads as `main` does, and a search through one reads as itself |
| G21-4 | gap | a path holding a space after a flag's value still made a job an agent | `words_read` looks at each of the first words |
| G21-5 | gap | `Limits(…=99.0)`, `__post_init__`, `setattr`, a tuple and `FELL_BYTES *= 1024` passed | The file is loaded and asked, and a value handed at a call is refused |
| G21-6 | gap | a hook of type `prompt` passed | It must be a command |
| G21-7 | gap | "however the number is spelt" was false of `$[-1]`, `0x1` and `+1` | `is_number` reads as the tools do. `$((1234))` refused is a stated cost |
| G21-8 | gap, false refusal | a message naming a `run` was read as one | Only a guard that is run |
| G21-9 | gap | `arch`, `xcrun` and `script` were no wrappers to the reader | They are |
| G21-10 | gap, rare | a sibling root's name going on with ` -`, and a root holding ` -- ` | A stated limit: a flat command line cannot tell |

Said, and left as it stands: its reader could not run a real `make focused`, a commit's compile, commit fixtures, a real full volume, chosen pressure levels, true idle time, the census against the real watcher, real Codex, Kimi or VS Code, `onFailure` with a real crash, or a hung mount under `words_read`'s `isfile`. A slow writer's top sat just past the stated 13 s and 15 MiB, at 13.4 s and 16.0 MiB. `Makefile` stays out of the stamp, since `make` is now asked of it at each commit.

**Round 22** — `2026-11-07`, leaf `PROGRAM.127.22`: not acceptable, 11 findings, 3 defects, after 3 on the round before, on `15177496`. The count did not fall, and round 21's method fell to its own next forms, so this removes it. What `ps` brackets held: 1,221 looks, none misread, and 400 makes at a scratch danger line, none stopped, where round 21's reader saw 5.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G22-1 | defect, fails open | a target's own `SHELL` passed, since `-pn` reads the global one: `make focused` exited 0 having run nothing | `make` is no longer asked. The Makefile is held whole by `MAKEFILE_SHA256`, so any byte changed moves that line in the same commit, which is a change to the check and asks the self-test |
| G22-2 | defect | a guard broken in the tree alone refused an unrelated commit, `make -n`'s `$(shell tail …)` reading the tree | Gone with it: nothing here runs `make` |
| G22-3 | defect | "moves a line past its mark" was false of `tightened`, which is what `main` takes: `status` said 99 | The file is asked what `tightened({})` gives, and any value handed at a call of `Limits` is refused |
| G22-4 | gap | `BASH_ENV`, a `PATH`, a `GNUmakefile`, a condition on `MAKEFLAGS`, `.PHONY` removed and a look inside `ifdef NEVER` each passed | The digest holds them all but the `GNUmakefile` and `makefile`, which are refused beside it |
| G22-5 | gap | a `$(shell)` at parse ran unguarded, inside each check too | Nothing here runs `make` |
| G22-6 | gap, false refusal | `xcrun -f` read as running | What it asks runs nothing |
| G22-7 | gap | `taskpolicy`, `sandbox-exec` and a `run` after `-W ignore` passed | Read. The wrappers stay a list, a stated limit |
| G22-8 | gap, mild | `script` took a flag's value for what runs, sparing a job | `-W` and `-X` take a value |
| G22-9 | wording | "at most one look late" of a process that names itself a bracket | A stated limit |
| G22-10 | wording | `is_number`'s "as `kill` … read" | It says `pgrep` |
| G22-11 | wording | a limit stating what is refused | Gone |

Said, and left as it stands: its reader could not run the brief's list, a real commit, Linux's `make` or make 3.82 and later. **It went outside its brief once:** a `$(shell cargo build >/dev/null)` rig with no stub on the check's path ran a real `cargo build` about 11 times, unguarded, each finishing in 0.02 s with no fingerprint changed and `target/debug/*.d` rewritten. It said so, and nothing was harmed; it is the very hazard this round removes.

**Round 23** — `2026-11-07`, leaf `PROGRAM.127.23`: not acceptable, 7 findings, 3 defects, after 3 and 3, on `893de281`. Each is narrow. The Makefile's digest held in every way it was tried, and the reader missed none of 185 new forms against real bash, with 659,900 fuzz texts through what the hook runs outside its `try` raising nothing. The count did not fall a second time, so this answers by method: the exception that failed open goes rather than being mended, and the record stops claiming of the kill reader what it claims of nothing else it reads.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G23-1 | defect, fails open, round 22's own | `xcrun -k`, `--kill-cache` and `-f -r` run the tool, and were read as asking: `xcrun -k kill -9 -1` passed | The exception is gone. `xcrun -f pkill` is refused though it only asks, a false refusal that costs a rewrite, stated |
| G23-2 | defect, mild | "holds what is staged" was false of a `GNUmakefile`, read by `os.listdir`: staged and gone from the tree it passed, and one in the tree alone refused an unrelated commit | Read from the index. `case_a_commit_holds_what_is_staged` runs both |
| G23-3 | defect, mild | the self-test could exit 1 on a clear host by two races in `case_run`: `until(jobs_of)` true on a `(ps)` or `<defunct>` child, and a fixed sleep before `status`. Each 1 in 25. Mine: round 22 widened one and left the other | `jobs_of` leaves out what `ps` cannot read and what has ended, and `status` is waited for |
| G23-4 | gap | `taskpolicy`'s `-m`, `-j`, `-P` and `-S` take a value | Read |
| G23-5 | gap, mild | `-W` and `-X` reached the reader, not `GUARD_PROCESS` or `SELF_TEST_PROCESS`: a `run` after `-W ignore` read `ours: 0` | Both patterns read them |
| G23-6 | gap, rare | `-W` and `-X` are Python's, and `perl -W codex.pl` lost its script | Python's alone |
| G23-7 | wording | "what runs from the system's own places, such as `caffeinate`", spared by name | It says by name |

Said, and left as it stands: its reader could not run a real `make focused`, a real commit, a real full volume, chosen pressure levels, true idle time, the census against the real watcher, real Codex, Kimi or VS Code, or `onFailure` with a real crash. One of its self-tests ended `608 passed, 0 failed, 5 not run`, rc 20, after the 15-minute bound, which is what that bound says. A copied `run_self_tests.sh` found itself and ran its arms 6 times under its work directory, one committing in its own scratch repository, and nothing in this one.

**Round 24** — `2026-11-07`, leaf `PROGRAM.127.24`: not acceptable, 6 findings, 3 defects, after 3, 3 and 3, on `612fd4ac`. Each is narrow. Round 23's answers held: 281 kill forms and 66 heavy forms read right, a staged `GNUmakefile` refused, `run` reaching its job in 0.14 to 0.22 s 30 times of 30, and the self-test passing whole, 870 of 870. Nothing not ours was stopped.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| G24-1 | defect | "this root's lines and turn hold" a nested checkout was false of its turn: a `run` there started beside this root's build, and this root's list omitted it. Harness worktrees stand there | Said as it is: its lines hold it, and its turn what shows a build by name. Waiting on an enclosing root would hold the self-test's scratch roots too |
| G24-2 | defect | a small job through `make` was stopped at a host's line, where through `run` alone it finished: a `make` counts as a build and hands its signal on | Said as it is |
| G24-3 | defect, mild, fails open | `kill -0 -s KILL -1` passed, a later flag undoing the exception, and `bash -n +n -c 'pkill y'` read as running nothing | Each goes on only where no later flag undoes it |
| G24-4 | gap | a syncing writer under some 2 MiB/s at a floor's pace is never seen, where the record said seen late | Said, with the measure |
| G24-5 | gap | `.claude/settings.local.json` with `disableAllHooks` passed | Refused, staged or standing |
| G24-6 | gap | the hook reads a directory by `-C` and not by a `cd` | A stated limit of the courtesy |

Said, and left as it stands: its reader could not run a real commit's compile, a real full volume, chosen pressure levels, the census against the real watcher, real Codex, Kimi or VS Code, or `onFailure` with a real crash, which the harness documents as blocking. An accented `--root` under `LC_ALL=C` reads as another, and this checkout's path is ASCII.

**Its reader broke a hard limit once.** A fixture's `make focused`, with a staged `gnumakefile` that `make` did not read, ran the fixture's real recipe, and `cargo xtask` climbed to this workspace: `target/debug/xtask verify --tier focused` ran unstubbed for about 2 minutes beside other work. It stopped its own task by `TaskStop`, and said so. The guard's scripts, Makefile and settings were byte-identical to its copies, and `target/.rustc_info.json` and `target/debug/xtask.d` may be its. A reader's brief now says a fixture's `make` needs a stub `cargo` first on its path.
