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
