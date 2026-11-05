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
