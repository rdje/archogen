#!/usr/bin/env python3
"""host_guard.py: this repository's work never endangers the host it shares (`docs/decisions/decision_host-guard.md`).

The machine holds 24 GiB and many other projects. The director's lines: RAM use above 88% is the danger zone, above 90% the host may
crash. So the limits below hold well inside them, on what the whole host uses, and on what this repository's own processes use.

    host_guard.py check            one reading; exit 0 clear to start a heavy job, 75 not
    host_guard.py run [--wait S] -- CMD...    a heavy job: one at a time, refused unless clear or waiting up to S seconds for room; at
                                   lower priority, watched, its exit status kept
    host_guard.py ensure           the watcher running, for this repository
    host_guard.py watch            the watcher itself; one per repository
    host_guard.py hook             the harness's check before a shell command: exit 2 refuses it
    host_guard.py status           what it reads now, and what it has stopped
    host_guard.py stop             the watcher stopped, for a handoff

It reads macOS, the host this runs on, by macOS's own figures: Activity Monitor's sum for RAM, the kernel's memory pressure level, swap, and
each process's footprint. On any other system it watches nothing and says so.

What it stops is only ever this repository's, by what each process shows of itself: its working directory or its executable under the root,
or having been started by a process that shows so. A process merely mentioning the path is not ours, and an agent or a terminal here vouches
for nothing it started. Never another project's, never by a program's name across the host, never an agent, a terminal, or what an agent runs
for itself. It guards against accident, not against an agent set on going round it.
"""

from __future__ import annotations

import ctypes
import fcntl
import json
import os
import re
import signal
import subprocess
import sys
import time
from dataclasses import asdict, dataclass, fields
from pathlib import Path

GIB = 1 << 30
UNCLEAR = 75  # EX_TEMPFAIL: not now, and nothing was run.


@dataclass
class Limits:
    """Each line the guard holds. An override may only tighten one (`tightened`), so no flag can loosen the guard."""

    # RAM the whole host uses, in percent. The director's danger zone starts at 88.
    start_used_percent: float = 70.0  # no heavy job starts above this
    stop_used_percent: float = 85.0  # our largest process stops
    panic_used_percent: float = 88.0  # everything of ours stops
    # macOS's own judgement, `kern.memorystatus_vm_pressure_level`: 1 normal, 2 warning, 4 critical. With the compressor taking the load, RAM
    # use can sit still while swap climbs, so this says what a percentage cannot.
    start_pressure_level: float = 2  # no heavy job starts at this
    stop_pressure_level: float = 2  # our largest process stops
    panic_pressure_level: float = 4  # everything of ours stops
    # Swap, which on this machine reached 62 GiB before anyone noticed, RSS showing under one.
    start_swap_gib: float = 16.0
    stop_swap_growth_gib: float = 3.0
    # What this repository's own processes may hold: a quarter of the machine, an eighth in one.
    all_processes_gib: float = 6.0
    one_process_gib: float = 3.0
    processes: float = 512
    # Disk: what must stay free where this repository writes and where the host swaps, and what one job may take.
    stop_repo_free_gib: float = 100.0
    stop_boot_free_gib: float = 30.0
    stop_disk_growth_gib: float = 50.0  # lost on the volume while ours runs, whoever wrote it: nothing cheap can say who did
    # Under this much of ours, stopping it would not help the host: the pressure is someone else's.
    worth_stopping_gib: float = 0.25
    # A watcher that has not read the host for this long is not watching, however alive it looks.
    watcher_stale_seconds: float = 60.0


# Those a higher value makes stricter. For all the others a lower one does.
RAISED_TO_TIGHTEN = {"stop_repo_free_gib", "stop_boot_free_gib"}


def tightened(overrides: dict) -> Limits:
    """The default limits with `overrides`, each of which must be stricter than the line it replaces."""
    limits = Limits()
    known = {field.name for field in fields(Limits)}
    for name, value in overrides.items():
        if name not in known:
            raise ValueError(f"no limit is called `{name}`")
        now = getattr(limits, name)
        if not isinstance(value, (int, float)) or isinstance(value, bool):
            raise ValueError(f"`{name}` takes a number")
        stricter = value >= now if name in RAISED_TO_TIGHTEN else value <= now
        if not stricter:
            raise ValueError(f"`{name}` may only be tightened: {value} is looser than {now}")
        setattr(limits, name, value)
    return limits


def repository_root() -> Path:
    """The main checkout holding this file: a worktree under `target/`, whose `.git` is a file, is part of it."""
    for at in Path(__file__).resolve().parents:
        if (at / ".git").is_dir():
            return at
    raise SystemExit("host-guard: this file is in no repository")


def state_path(root: Path) -> Path:
    # A scoped root is its own state; the repository's is under its ignored `target/`.
    return root / "target" / "host-guard" / "state" if (root / ".git").is_dir() else root / "state"


def state_dir(root: Path) -> Path:
    state = state_path(root)
    state.mkdir(parents=True, exist_ok=True)
    return state


# ---- the host


@dataclass
class Reading:
    used_percent: float
    pressure_level: int
    free_percent_says: int
    swap_used_gib: float
    repo_free_gib: float
    boot_free_gib: float


def sysctl(*names: str) -> list[str]:
    done = subprocess.run(["sysctl", "-n", *names], capture_output=True, text=True, errors="replace", timeout=20, check=True)
    return done.stdout.splitlines()


def free_gib(path: str) -> float:
    stats = os.statvfs(path)
    return stats.f_bavail * stats.f_frsize / GIB


def read_memory() -> tuple[float, int, int, float]:
    """RAM used in percent, as Activity Monitor adds it or as the kernel's free percentage has it if that is higher; the kernel's pressure
    level; its free percentage; and swap used in GiB. What does not read as expected raises: a figure not read is never a figure of zero."""
    memsize, level, pressure, swap = sysctl("hw.memsize", "kern.memorystatus_level", "kern.memorystatus_vm_pressure_level", "vm.swapusage")
    pages = subprocess.run(["vm_stat"], capture_output=True, text=True, errors="replace", timeout=20, check=True).stdout
    size = re.search(r"page size of (\d+) bytes", pages)
    if not size:
        raise ValueError("`vm_stat` states no page size")
    page = int(size.group(1))

    def count(label: str) -> int:
        found = re.search(rf"^{label}:\s+(\d+)\.", pages, re.M)
        if not found:
            raise ValueError(f"`vm_stat` no longer says `{label}`")
        return int(found.group(1))

    # App memory, wired and compressed. Cached files are not use: macOS gives them back.
    used = count("Anonymous pages") - count("Pages purgeable") + count("Pages wired down")
    used += count("Pages occupied by compressor")
    by_pages = 100.0 * used * page / int(memsize)
    swap_used = re.search(r"used = ([\d.]+)M", swap)
    if not swap_used:
        raise ValueError(f"`vm.swapusage` reads `{swap}`")
    return max(by_pages, 100.0 - float(level)), int(pressure), int(level), float(swap_used.group(1)) / 1024.0


def read_host(root: Path) -> Reading:
    used, pressure, free_says, swap = read_memory()
    return Reading(used, pressure, free_says, swap, free_gib(str(root)), free_gib("/"))


def disk_reasons(reading: Reading, limits: Limits) -> list[str]:
    reasons = []
    if reading.repo_free_gib < limits.stop_repo_free_gib:
        reasons.append(f"the repository's volume has {reading.repo_free_gib:.0f} GiB free, under {limits.stop_repo_free_gib:.0f}")
    if reading.boot_free_gib < limits.stop_boot_free_gib:
        reasons.append(f"the boot volume, where the host swaps, has {reading.boot_free_gib:.0f} GiB free, under {limits.stop_boot_free_gib:.0f}")
    return reasons


def start_reasons(reading: Reading, limits: Limits) -> list[str]:
    """Why a heavy job should not start now."""
    reasons = []
    if reading.used_percent > limits.start_used_percent:
        reasons.append(f"RAM use is {reading.used_percent:.0f}%, over the {limits.start_used_percent:.0f}% a heavy job starts under")
    if reading.pressure_level >= limits.start_pressure_level:
        reasons.append(f"macOS reports memory pressure at level {reading.pressure_level}, where 1 is normal")
    if reading.swap_used_gib > limits.start_swap_gib:
        reasons.append(f"swap holds {reading.swap_used_gib:.1f} GiB, over {limits.start_swap_gib:.0f}")
    reasons.extend(disk_reasons(reading, limits))
    return reasons


def stop_reasons(reading: Reading, limits: Limits) -> list[str]:
    """Why nothing but a read should run now."""
    reasons = []
    if reading.used_percent >= limits.stop_used_percent:
        reasons.append(f"RAM use is {reading.used_percent:.0f}%, at the {limits.stop_used_percent:.0f}% stop line")
    if reading.pressure_level >= limits.stop_pressure_level:
        reasons.append(f"macOS reports memory pressure at level {reading.pressure_level}, where 1 is normal")
    reasons.extend(disk_reasons(reading, limits))
    return reasons


# ---- processes

INTERPRETERS = {"python", "python3", "Python", "node", "ruby", "perl", "bash", "sh", "zsh"}
SHELLS = {"bash", "sh", "zsh", "fish", "dash", "ksh"}


@dataclass
class Process:
    pid: int
    ppid: int
    command: str
    cwd: str
    exe: str
    bytes: int

    @property
    def name(self) -> str:
        # `command` may hold spaces in its first word, `/Library/Application Support/…`: the executable's own path settles where it ends.
        # A process that renamed itself, `exec -a`, does not begin with it, and goes by the name it chose.
        if self.exe and self.command.startswith(self.exe):
            return os.path.basename(self.exe)
        return os.path.basename(self.command.split(" ", 1)[0]) if self.command else ""

    @property
    def script(self) -> str:
        """What an interpreter runs, by its file's name; nothing otherwise."""
        if not (self.name in INTERPRETERS or self.name.startswith("python")):
            return ""
        rest = self.command[len(self.exe) :] if self.exe and self.command.startswith(self.exe) else self.command.partition(" ")[2]
        for word in rest.split():
            if not word.startswith("-"):
                return os.path.basename(word)
        return ""

    @property
    def paths_named(self) -> list[str]:
        """The files an interpreter's first arguments point at, by name: a flag may take a value, and hide its script behind it."""
        if not self.script:
            return []
        return [os.path.basename(word) for word in self.command.split()[1:7] if "/" in word]

    @property
    def label(self) -> str:
        return f"{self.name} {self.script}" if self.script else self.name


class Darwin:
    """What `ps` cannot say: a process's working directory and executable, and its footprint, which counts what is compressed and swapped. RSS
    does not: on `2026-11-05` the host's swap held 62 GiB while the largest process showed 0.9."""

    def __init__(self) -> None:
        self.lib = ctypes.CDLL("libproc.dylib", use_errno=True)

    def footprint(self, pid: int) -> int | None:
        buffer = ctypes.create_string_buffer(96)  # rusage_info_v0
        if self.lib.proc_pid_rusage(pid, 0, buffer) != 0:
            return None
        return int.from_bytes(buffer.raw[72:80], sys.byteorder)  # ri_phys_footprint

    def cwd(self, pid: int) -> str:
        buffer = ctypes.create_string_buffer(2352)  # proc_vnodepathinfo
        if self.lib.proc_pidinfo(pid, 9, 0, buffer, 2352) <= 0:  # PROC_PIDVNODEPATHINFO
            return ""
        return buffer.raw[152:152 + 1024].split(b"\0", 1)[0].decode("utf-8", "replace")

    def exe(self, pid: int) -> str:
        buffer = ctypes.create_string_buffer(4096)
        if self.lib.proc_pidpath(pid, buffer, 4096) <= 0:
            return ""
        return buffer.value.decode("utf-8", "replace")


NATIVE = Darwin() if sys.platform == "darwin" else None


def processes() -> list[Process]:
    """This user's processes, nobody else's. A zombie holds nothing and cannot be stopped again, so it is not among them."""
    me = os.getuid()
    # A command line is bytes, and any process may hold ones that are no text. One such process must not blind the watcher.
    listed = subprocess.run(["ps", "-axww", "-o", "pid=,ppid=,uid=,stat=,rss=,command="], capture_output=True, text=True, errors="replace", timeout=30, check=True)
    table = []
    for line in listed.stdout.splitlines():
        parts = line.split(None, 5)
        if len(parts) < 6 or int(parts[2]) != me or parts[3].startswith("Z"):
            continue
        pid = int(parts[0])
        held = NATIVE.footprint(pid)
        table.append(Process(pid, int(parts[1]), parts[5], NATIVE.cwd(pid), NATIVE.exe(pid), int(parts[4]) * 1024 if held is None else held))
    # A table that does not hold the one reading it is not the table: `ps` answering nothing, or cut short, must read as blind and never as calm.
    if not any(process.pid == os.getpid() for process in table):
        raise ValueError("`ps` does not list the process reading it")
    return table


# Who may be working here and must never be stopped. A name is searched for, not matched whole: `cursor-agent`, and an agent run by an interpreter
# goes by its script. Wrongly sparing a process costs less than wrongly stopping an agent, and the record says so.
AGENT = re.compile(r"claude|codex|kimi|gemini|aider|cursor|copilot|opencode|goose", re.I)
INFRASTRUCTURE = {"login", "sshd", "tmux", "screen", "iTerm2", "Terminal", "Code", "caffeinate"}
APPLICATION = ("/Applications/", "/System/", "/usr/libexec/", "/usr/sbin/")
GUARD_PROCESS = re.compile(r"^\S*[Pp]ython\S*\s+(?:-\S+\s+)*\S*host_guard\.py(?:\s|$)")
# Small, and stopping them frees nothing: a `git` could leave a lock or a half-written index behind, and these others are how a host in trouble is looked at.
# `rm` so that disk can be freed: its shell may be stopped around it, and it finishes.
SPARED_WHEN_SMALL = {
    "git", "ps", "top", "sysctl", "vm_stat", "df", "du", "lsof", "memory_pressure", "rm", "cat", "grep", "ugrep", "sort", "uniq", "head", "tail", "cut", "wc", "tr", "ls",
    "stat", "date", "uptime", "pgrep", "kill",
}


def is_agent(process: Process) -> bool:
    return any(AGENT.search(word) for word in (process.name, os.path.basename(process.exe), process.script, *process.paths_named))


def is_terminal_shell(process: Process) -> bool:
    """A shell someone or something types into, which vouches for nothing it starts: a login shell by its dash, or any shell told neither a command
    (`-c`, `-lc`) nor a script. A shell running a command or a script is a job."""
    name = process.name
    if name.startswith("-"):
        return name[1:] in SHELLS
    if name not in SHELLS:
        return False
    rest = process.command[len(process.exe) :] if process.exe and process.command.startswith(process.exe) else process.command.partition(" ")[2]
    for word in rest.split():
        if not word.startswith("-") or re.fullmatch(r"-\w*c\w*", word):
            return False
    return True


def names_root(command: str, root: str) -> bool:
    """Whether a guard's command line says `--root` is exactly this root, and not one whose name merely begins the same."""
    return re.search(r"--root\s+" + re.escape(root) + r"(?:\s|$)", command) is not None


def is_run_of(process: Process, root: str) -> bool:
    return bool(GUARD_PROCESS.search(process.command)) and re.search(r"host_guard\.py\s+run(?:\s|$)", process.command) is not None and names_root(process.command, root)


def line_of(table: list[Process], pid: int) -> set[int]:
    """`pid` and its parents, up the table."""
    by_pid = {process.pid: process for process in table}
    line: set[int] = set()
    while pid in by_pid and pid not in line:
        line.add(pid)
        pid = by_pid[pid].ppid
    return line


def protected(process: Process, by_pid: dict[int, Process]) -> bool:
    if process.name in INFRASTRUCTURE or is_agent(process) or is_terminal_shell(process) or GUARD_PROCESS.search(process.command):
        return True
    if process.command.startswith(APPLICATION) or process.exe.startswith(APPLICATION):
        return True
    # What an agent runs for itself, a server for its tools and all that server starts, is not a job. What it runs as a command goes through a shell.
    at, steps = process, 0
    while steps < 64:
        parent = by_pid.get(at.ppid)
        if parent is None:
            return False
        if is_agent(parent):
            return at.name not in SHELLS
        at, steps = parent, steps + 1
    return False


def children_of(table: list[Process]) -> dict[int, list[int]]:
    children: dict[int, list[int]] = {}
    for process in table:
        children.setdefault(process.ppid, []).append(process.pid)
    return children


def ours(table: list[Process], root: Path, me: int) -> list[Process]:
    """This repository's processes that may be stopped, largest first: under its root by working directory or executable, and what those started,
    wherever it runs. A protected process is never one of them and vouches for nothing: the walk does not pass through it, so a terminal started here
    gives no claim on a build it runs elsewhere. A guard's `run` of this root is the one exception, since it says itself that its job is ours."""
    text = str(root)
    by_pid = {process.pid: process for process in table}
    children = children_of(table)
    under = lambda path: path == text or path.startswith(text + os.sep)
    spared = {process.pid for process in table if protected(process, by_pid)}
    found = {p.pid for p in table if p.pid not in spared and (under(p.cwd) or under(p.exe))}
    todo = list(found) + [p.pid for p in table if is_run_of(p, text)]
    while todo:
        for child in children.get(todo.pop(), []):
            if child not in found and child not in spared:
                found.add(child)
                todo.append(child)
    line = line_of(table, me)
    return sorted((by_pid[pid] for pid in found if pid not in line), key=lambda process: process.bytes, reverse=True)


def small_and_spared(process: Process, limits: Limits) -> bool:
    return process.name in SPARED_WHEN_SMALL and process.bytes < limits.worth_stopping_gib * GIB


def tree_of(pid: int, table: list[Process], limits: Limits) -> list[int]:
    """`pid` and what it started, children first: a parent's death does not reach them. A protected process under it is left, with all it started."""
    by_pid = {process.pid: process for process in table}
    children = children_of(table)
    order, todo = [], [pid]
    while todo:
        at = todo.pop()
        process = by_pid.get(at)
        if process is not None and at != pid and protected(process, by_pid):
            continue
        if process is None or at == pid or not small_and_spared(process, limits):
            order.append(at)
        todo.extend(children.get(at, []))
    order.reverse()
    return order


def stop(pids: list[int], seconds: float = 1.0) -> None:
    """Ask them all at once, then make sure of them all at once. Under memory pressure a process may not get to run its handler, and one target
    at a time would cost a second each while the host fills."""
    for pid in pids:
        try:
            os.kill(pid, signal.SIGTERM)
        except OSError:
            pass
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        left = 0
        for pid in pids:
            try:
                os.kill(pid, 0)
                left += 1
            except OSError:
                pass
        if not left:
            return
        time.sleep(0.05)
    for pid in pids:
        try:
            os.kill(pid, signal.SIGKILL)
        except OSError:
            pass


# ---- the watcher


LOG_AT_MOST = 1 << 20  # What it keeps, and the most any read of it can cost.


def record(root: Path, **event) -> None:
    """Write down what happened. Never raises: a watcher must not die of its own bookkeeping, `target/` removed by `cargo clean` among the causes."""
    event["at"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    try:
        path = state_dir(root) / "events.log"
        if size_of(path) > LOG_AT_MOST:
            os.replace(path, path.with_suffix(".log.1"))
        with open(path, "a", encoding="utf-8") as log:
            log.write(json.dumps(event) + "\n")
    except OSError as error:
        print(f"host-guard: cannot record {event.get('kind')}: {error}", file=sys.stderr)


def size_of(path: Path) -> int:
    try:
        return path.stat().st_size
    except OSError:
        return 0


def judged(reading: Reading, mine: list[Process], limits: Limits, baseline: Reading | None) -> tuple[list[Process], list[str], bool]:
    """What to stop now, and why, and whether the pressure was someone else's and nothing of ours is worth stopping."""
    total = sum(process.bytes for process in mine)
    host, everything = [], []
    if reading.used_percent >= limits.panic_used_percent:
        everything.append(f"RAM use is {reading.used_percent:.0f}%, at the {limits.panic_used_percent:.0f}% danger line")
    if reading.pressure_level >= limits.panic_pressure_level:
        everything.append(f"macOS reports critical memory pressure, level {reading.pressure_level}")
    everything.extend(disk_reasons(reading, limits))
    if baseline and baseline.repo_free_gib - reading.repo_free_gib > limits.stop_disk_growth_gib:
        everything.append(f"the repository's volume lost {baseline.repo_free_gib - reading.repo_free_gib:.1f} GiB during this work, over {limits.stop_disk_growth_gib:.1f}")
    able = [p for p in mine if not small_and_spared(p, limits)]
    if everything:
        return able, everything, False
    if len(mine) > limits.processes:
        return able[: len(mine) - int(limits.processes)], [f"{len(mine)} processes of ours, over {limits.processes:.0f}"], False
    if able and able[0].bytes > limits.one_process_gib * GIB:
        return [able[0]], [f"one process of ours holds {able[0].bytes / GIB:.2f} GiB, over {limits.one_process_gib:.2f}"], False
    if able and total > limits.all_processes_gib * GIB:
        return [able[0]], [f"our processes hold {total / GIB:.2f} GiB, over {limits.all_processes_gib:.2f}"], False
    if reading.used_percent >= limits.stop_used_percent:
        host.append(f"RAM use is {reading.used_percent:.0f}%, at the {limits.stop_used_percent:.0f}% stop line")
    if reading.pressure_level >= limits.stop_pressure_level:
        host.append(f"macOS reports memory pressure at level {reading.pressure_level}, where 1 is normal")
    if baseline and reading.swap_used_gib - baseline.swap_used_gib > limits.stop_swap_growth_gib:
        host.append(f"swap grew {reading.swap_used_gib - baseline.swap_used_gib:.1f} GiB during this work, over {limits.stop_swap_growth_gib:.1f}")
    if not host:
        return [], [], False
    if total < limits.worth_stopping_gib * GIB or not able:
        return [], host, True
    return [able[0]], host, False


def largest_on_host(table: list[Process], count: int = 5) -> list[dict]:
    """Who holds the most, of this user's processes in any project: read and said, and never acted on."""
    largest = sorted(table, key=lambda process: process.bytes, reverse=True)[:count]
    return [{"pid": p.pid, "name": p.label, "gib": round(p.bytes / GIB, 2), "command": p.command[:120]} for p in largest]


def note_peak(root: Path, reading: Reading, ours_bytes: int) -> None:
    """The most this watcher has seen, so a limit can be set from what a job really used. Never raises, whatever the file has become."""
    try:
        path = state_dir(root) / "peak.json"
        now = {"used_percent": round(reading.used_percent, 1), "ours_gib": round(ours_bytes / GIB, 2), "swap_gib": round(reading.swap_used_gib, 1)}
        try:
            peak = json.loads(path.read_text())
            peak = {name: float(peak[name]) for name in now}
        except Exception:  # A file in any state, nested too deep to parse among them.
            peak = dict.fromkeys(now, 0.0)
        if any(now[name] > peak[name] for name in now):
            path.write_text(json.dumps({name: max(now[name], peak[name]) for name in now}))
    except Exception as error:
        print(f"host-guard: cannot note the peak: {error}", file=sys.stderr)


def sample(root: Path, limits: Limits, baseline: Reading | None, said: list[float]) -> tuple[Reading, Reading | None, list[Process]]:
    reading = read_host(root)
    table = processes()
    mine = ours(table, root, os.getpid())
    # What this work grew is measured from when it began, however small it is: only when nothing of ours runs does the count start again.
    baseline = (baseline or reading) if mine else None
    targets, reasons, not_ours = judged(reading, mine, limits, baseline)
    if targets:
        # First the stop, then everything else: nothing below may keep it from happening.
        pids: list[int] = []
        for target in targets:
            pids.extend(pid for pid in tree_of(target.pid, table, limits) if pid not in pids)
        stop(pids)
        stopped = [{"pid": t.pid, "name": t.label, "gib": round(t.bytes / GIB, 2), "command": t.command[:160]} for t in targets]
        record(root, kind="stopped", reasons=reasons, stopped=stopped, reading=asdict(reading), largest=largest_on_host(table))
    elif not_ours and time.monotonic() - said[0] > 60:
        said[0] = time.monotonic()
        record(root, kind="pressure-not-ours", reasons=reasons, reading=asdict(reading), largest=largest_on_host(table))
    note_peak(root, reading, sum(process.bytes for process in mine))
    return reading, baseline, mine


def hold_lock(root: Path) -> int | None:
    """One watcher per root, by a lock on the root itself: unlike a file under `target/`, `cargo clean` cannot take it away and leave two."""
    fd = os.open(str(root), os.O_RDONLY)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        os.close(fd)
        return None
    return fd


def write_note(root: Path, name: str, text: str) -> None:
    try:
        (state_dir(root) / name).write_text(text)
    except OSError as error:
        print(f"host-guard: cannot write {name}: {error}", file=sys.stderr)


def watch(root: Path, limits: Limits, once: bool, idle_seconds: float = 600.0) -> int:
    lock = hold_lock(root)
    if lock is None:
        return 0  # One is watching.
    write_note(root, "watch.pid", str(os.getpid()))
    try:
        (state_dir(root) / "peak.json").unlink(missing_ok=True)
    except OSError:
        pass
    os.chdir("/")
    baseline, said, unread_said, last_seen, busy = None, [0.0], 0.0, time.monotonic(), False
    while True:
        near = True
        try:
            reading, baseline, mine = sample(root, limits, baseline, said)
            near = reading.used_percent >= limits.start_used_percent
            busy = bool(mine)
            write_note(root, "sampled", str(time.time()))
        except Exception as error:  # Whatever it is, the watcher goes on trying, and `sampled` going stale says it is blind.
            if time.monotonic() - unread_said > 60:
                unread_said = time.monotonic()
                record(root, kind="unread", error=str(error))
        if once:
            return 0
        if not (state_path(root) / "watch.pid").exists():
            write_note(root, "watch.pid", str(os.getpid()))
        heartbeat = state_path(root) / "heartbeat"
        try:
            fresh = time.time() - heartbeat.stat().st_mtime < idle_seconds
        except OSError:
            fresh = False
        if busy or fresh:
            last_seen = time.monotonic()
        elif time.monotonic() - last_seen > idle_seconds:
            return 0
        time.sleep(0.25 if near else 1.0)


def watcher_pid(root: Path) -> int | None:
    """The watcher of `root`, if one runs: the pid its file names, only if that process says it is one. A stale file names someone else."""
    try:
        pid = int((state_path(root) / "watch.pid").read_text())
    except (OSError, ValueError):
        return None
    try:
        shown = subprocess.run(["ps", "-ww", "-o", "command=", "-p", str(pid)], capture_output=True, text=True, errors="replace", timeout=5).stdout
    except (OSError, subprocess.SubprocessError):
        return None
    return pid if GUARD_PROCESS.search(shown) and re.search(r"host_guard\.py\s+watch\b", shown) and names_root(shown, str(root)) else None


def healthy(root: Path, limits: Limits) -> bool:
    """Whether a watcher has read the host lately: alive is not enough, since one that cannot read the process table stops nothing."""
    try:
        # Either way: a time in the future, a clock set back, is no proof of having looked.
        return abs(time.time() - (state_path(root) / "sampled").stat().st_mtime) < limits.watcher_stale_seconds
    except OSError:
        return False


def ensure(root: Path) -> None:
    """A watcher for `root`, started if none is. It outlives this call and ends itself when the work has been idle. If two race, the lock leaves one."""
    if watcher_pid(root):
        return
    state = state_dir(root)
    with open(state / "watch.out", "a") as out:
        subprocess.Popen(
            [sys.executable, "-I", "-B", str(Path(__file__).resolve()), "watch", "--root", str(root)],
            stdin=subprocess.DEVNULL, stdout=out, stderr=out, start_new_session=True, cwd="/",
        )


def unseen(root: Path) -> list[dict]:
    """What has been stopped since an agent last read it, marking it read only once read. A bad line is skipped and never costs the rest."""
    state = state_path(root)
    log, seen = state / "events.log", state / "seen"
    try:
        start = max(0, int(seen.read_text()))
    except (OSError, ValueError):
        start = 0
    end = size_of(log)
    if end < start:  # A log replaced or rotated: all of it is new.
        start = 0
    if end == start:
        return []
    start = max(start, end - LOG_AT_MOST)
    with open(log, "rb") as lines:
        lines.seek(start)
        text = lines.read()
    events = []
    for line in text.splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if isinstance(event, dict) and event.get("kind") == "stopped" and isinstance(event.get("stopped"), list):
            events.append(event)
    write_note(root, "seen", str(start + len(text)))
    return events


def said_of(event: dict) -> str:
    stopped = [s for s in event.get("stopped", []) if isinstance(s, dict)]
    names = ", ".join(f"{s.get('name')} (pid {s.get('pid')}, {s.get('gib')} GiB)" for s in stopped[:4])
    more = len(stopped) - 4
    holders = ", ".join(f"{h.get('name')} {h.get('gib')} GiB" for h in event.get("largest", [])[:3] if isinstance(h, dict))
    return (
        f"{event.get('at')}: stopped {names}{f' and {more} more' if more > 0 else ''}: {'; '.join(map(str, event.get('reasons', [])))}"
        + (f". Holding the most on the host: {holders}" if holders else "")
    )


# ---- what runs through it

# What may use real memory or write much, where a command stands, so that the word in a message or a search is not taken for one. `cargo fmt` is
# neither. This is a courtesy to the host, not its guard: what reaches past it the watcher still holds, and the record says so.
BEFORE = r"(?:^|[;&|({`'\"\n]|\b(?:do|then|else|if|while|until|env|nohup|xargs|sudo|command|bash|sh)\s|!\s)\s*(?:\w+=\S*\s+)*(?:(?:timeout\s+\S+|nice(?:\s+-n\s*\S+)?|time|exec)\s+)*"
HEAVY = re.compile(BEFORE + r"(?:[^\s=]*/)?(?:cargo(?!\s+fmt\b)[\w-]*|g?make|rustc|mdbook|qemu-system\S*|archogen|archogen-plan-check|build_book\.sh)(?=[\s;&|)'\"`]|$)|/install/archogen")
BUILDERS = {"cargo", "rustc", "make", "gmake"}
KILL_BY_NAME = re.compile(r"(?:^|[;&|(\n]|\s)(?:pkill|killall)(?=\s)")
# What is safe at any line, since a host in trouble must stay readable and it must be possible to free disk. Each stage of a command must be one, by
# its bare name. `pkill` is not among them: killing by a name is the one act this record blames for stopping another project's build.
READERS = {
    "ps", "pgrep", "sysctl", "vm_stat", "df", "du", "lsof", "memory_pressure", "uptime", "sort", "uniq", "head", "tail", "grep", "cut", "wc", "tr",
    "cat", "ls", "stat", "date", "echo", "printf", "true", ":", "cd", "rm",
}
GUARD_READS = {"status", "check", "ensure"}
SCRATCH_SETTINGS = re.compile(r"(?:TMPDIR|TMP|TEMP|CARGO_HOME|XDG_CACHE_HOME|R)=")
WRITES_OR_SPENDS = re.compile(r"-o\b|-S\b|--output|--compress-program|--no-index|--ext-diff")


def is_read(command: str) -> bool:
    """Whether every stage of `command` only reads the host or frees disk. No substitution, function, subshell, background or redirection but away from the
    screen. Quotes are not parsed, so a `|` inside one refuses a read, which is the safe way to be wrong. This keeps accident out, and not an agent set on
    going round it: a program of the same name earlier on the path is not looked for."""
    text = re.sub(r"\s*\d?>\s*/dev/null|\s*\d>&\d", " ", command)
    if re.search(r"[<>`(){}]|\$\(", text) or re.search(r"(?<!&)&(?!&)", text):
        return False
    said = False
    for stage in re.split(r"\|\||&&|[;|\n]", text):
        words = stage.split()
        if not words:
            continue
        said = True
        head, rest = words[0], words[1:]
        if any(WRITES_OR_SPENDS.match(word) for word in rest):
            return False
        if head in READERS:
            continue
        if head == "top" and "-l" in rest:
            continue
        if head == "kill" and all(re.fullmatch(r"-\w+|\d+", word) for word in rest):
            continue
        if head == "export" and all(SCRATCH_SETTINGS.match(word) for word in rest):
            continue
        if head == "mkdir" and all(word == "-p" or "target/" in word for word in rest):
            continue
        if head == "git" and rest and rest[0] in ("status", "log", "diff"):
            continue
        if os.path.basename(head).startswith("python"):
            given = [word for word in rest if word not in ("-I", "-B", "-E", "-s")]
            if len(given) >= 2 and os.path.basename(given[0]) == "host_guard.py" and given[1] in GUARD_READS and not given[0].startswith("-"):
                continue
        return False
    return said


def code_of(command: str) -> str:
    """`command` without what it only says: a heredoc's body, and quoted text unless a shell is told to run it (`-c '…'`)."""
    text = re.sub(r"<<-?\s*(['\"]?)(\w+)\1[^\n]*\n.*?\n\s*\2[ \t]*(?:\n|$)", " ", command, flags=re.S)
    return re.sub(r"(-c\s+)?('[^']*'|\"(?:[^\"\\]|\\.)*\")", lambda m: m.group(0) if m.group(1) else " ", text)


def heavy_running(root: Path) -> list[Process]:
    """What of ours is a heavy job now: a build, or another `run` whose job has started. A `run` still waiting its turn has started nothing, and only looks at the host."""
    table = processes()
    me = os.getpid()
    text = str(root)
    line = line_of(table, me)
    children = children_of(table)
    by_pid = {process.pid: process for process in table}
    running = [p for p in ours(table, root, me) if p.name in BUILDERS]
    for process in table:
        if process.pid not in line and is_run_of(process, text):
            # The watcher it may have started is its child too, and no job.
            if any(by_pid[pid].name not in SPARED_WHEN_SMALL and not GUARD_PROCESS.search(by_pid[pid].command) for pid in children.get(process.pid, []) if pid in by_pid):
                running.append(process)
    return running


def run(root: Path, limits: Limits, command: list[str], wait_seconds: float = 0.0) -> int:
    if not command:
        print("host-guard: `run` takes a command, after `--`", file=sys.stderr)
        return 2
    state = state_dir(root)
    for event in unseen(root):
        print(f"host-guard: {said_of(event)}", file=sys.stderr)
    ensure(root)
    # A job nested in its own `run` is no rival. That is read from the process table, so no environment can claim it.
    table = processes()
    by_pid = {process.pid: process for process in table}
    nested = any(is_run_of(by_pid[pid], str(root)) for pid in line_of(table, os.getpid()) - {os.getpid()})
    job_lock = open(state / "job.lock", "a")
    begun = time.monotonic()

    def why_not() -> list[str]:
        try:
            reasons = start_reasons(read_host(root), limits)
            if nested:
                return reasons
            running = heavy_running(root)
        except Exception as error:
            return [f"the host cannot be read, so nothing says this is safe: {error}"]
        if running:
            reasons.append(f"one heavy job at a time, and {running[0].label} (pid {running[0].pid}) is running")
        try:
            fcntl.flock(job_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError:
            reasons.append("one heavy job at a time, and another `run` holds its turn")
        return reasons

    # The host is shared: when it is full, the polite thing is to wait for room, not to add to it.
    reasons = why_not()
    while reasons and time.monotonic() - begun < wait_seconds:
        time.sleep(min(15.0, max(0.1, wait_seconds - (time.monotonic() - begun))))
        (state / "heartbeat").touch()
        reasons = why_not()
    if reasons:
        print(f"host-guard: not started: {'; '.join(reasons)}.", file=sys.stderr)
        print("  hint: wait for the host to have room, and look at `host_guard.py status`", file=sys.stderr)
        return UNCLEAR
    # A build's cost is its parallelism, and this machine is shared: a third of its cores, and a lower priority.
    env = dict(os.environ)
    jobs = str(max(2, (os.cpu_count() or 4) // 3))
    env.setdefault("CARGO_BUILD_JOBS", jobs)
    env.setdefault("RUST_TEST_THREADS", jobs)
    logged = size_of(state / "events.log")
    # In a session of its own, so that all of it can be stopped with no process table to read. The job holds the lock too, so it keeps its turn if this process
    # is killed. At least this low a priority, however many guards it is nested in.
    job = subprocess.Popen(command, env=env, start_new_session=True, pass_fds=(job_lock.fileno(),), preexec_fn=lambda: os.nice(max(0, 10 - os.nice(0))))

    def to_job(number: int, *_: object) -> None:
        try:
            os.killpg(job.pid, number)
        except OSError:
            pass

    for number in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
        signal.signal(number, to_job)
    blind_since = None
    while True:
        try:
            status = job.wait(timeout=5)
            break
        except subprocess.TimeoutExpired:
            pass
        # An unwatched job is what this exists to prevent: keep a watcher running, and end the job if none can see. Nothing here may raise past that.
        try:
            ensure(root)
            (state_dir(root) / "heartbeat").touch()
        except Exception:
            pass
        if healthy(root, limits):
            blind_since = None
        else:
            blind_since = blind_since if blind_since is not None else time.monotonic()
            if time.monotonic() - blind_since > limits.watcher_stale_seconds:
                print("host-guard: no watcher can read the host, so this job is unwatched: stopping it", file=sys.stderr)
                to_job(signal.SIGTERM)
                try:
                    job.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    pass
                to_job(signal.SIGKILL)
                job.wait()
                return UNCLEAR
    try:
        with open(state_path(root) / "events.log", "rb") as lines:
            lines.seek(logged)
            text = lines.read(LOG_AT_MOST)
    except OSError:
        text = b""
    for line in text.splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if isinstance(event, dict) and event.get("kind") == "stopped":
            print(f"host-guard: {said_of(event)}", file=sys.stderr)
    return status


def hook(root: Path, limits: Limits) -> int:
    """Exit 2 refuses the command, and what it prints goes to the agent. Anything it cannot do refuses, but never a read, so a broken guard can be
    looked at and its state removed."""
    try:
        command = json.load(sys.stdin)["tool_input"]["command"]
        if not isinstance(command, str) or not command.strip():
            raise ValueError("no command")
    except Exception as error:
        print(f"host-guard: refused: this cannot be read as a command to judge: {error!r}", file=sys.stderr)
        return 2
    if is_read(command):
        return 0
    try:
        return judge_command(root, limits, command)
    except Exception as error:
        print(f"host-guard: refused: the guard cannot judge this: {error}. Reads still run, `rm -rf target/host-guard/state` among them.", file=sys.stderr)
        return 2


def judge_command(root: Path, limits: Limits, command: str) -> int:
    if KILL_BY_NAME.search(code_of(command)) and str(root) not in command:
        print("host-guard: refused: never by a program's name across the host, which can stop another project's work. Stop what is ours by its pid, or name this repository's path in the pattern.", file=sys.stderr)
        return 2
    ensure(root)
    state = state_dir(root)
    (state / "heartbeat").touch()
    happened = unseen(root)
    if happened:
        for event in happened:
            print(f"host-guard: {said_of(event)}", file=sys.stderr)
        print("host-guard: this command was not run, so that you see that. Find out why before running it again.", file=sys.stderr)
        return 2
    reading = read_host(root)
    stopping = stop_reasons(reading, limits)
    if stopping:
        print(f"host-guard: refused: {'; '.join(stopping)}. Reads still run, `host_guard.py status` among them.", file=sys.stderr)
        return 2
    if HEAVY.search(code_of(command)):
        reasons = start_reasons(reading, limits)
        if reasons:
            print(f"host-guard: refused, this is a heavy command: {'; '.join(reasons)}.", file=sys.stderr)
            return 2
        # A watcher just started has had no time to read the host, so it gets a moment before it is called blind.
        deadline = time.monotonic() + 3.0
        while not healthy(root, limits) and time.monotonic() < deadline:
            time.sleep(0.1)
        if not healthy(root, limits):
            print("host-guard: refused, this is a heavy command, and no watcher has read the host lately: look at `host_guard.py status`.", file=sys.stderr)
            return 2
        running = heavy_running(root)
        if running:
            print(f"host-guard: refused: one heavy job at a time, and {running[0].name} (pid {running[0].pid}) is running: {running[0].command[:120]}", file=sys.stderr)
            return 2
    return 0


def status(root: Path, limits: Limits) -> int:
    state = state_path(root)
    try:
        reading = read_host(root)
    except Exception as error:
        print(f"the host cannot be read: {error}")
        return 0
    print(f"RAM used {reading.used_percent:.1f}% (start under {limits.start_used_percent:.0f}, stop at {limits.stop_used_percent:.0f}, all stopped at {limits.panic_used_percent:.0f}); macOS: pressure level {reading.pressure_level} (1 normal, 2 warning, 4 critical), {reading.free_percent_says}% free; swap {reading.swap_used_gib:.1f} GiB")
    print(f"free: repository volume {reading.repo_free_gib:.0f} GiB, boot volume {reading.boot_free_gib:.0f} GiB")
    try:
        table = processes()
        mine = ours(table, root, os.getpid())
        print(f"ours: {len(mine)} process(es), {sum(p.bytes for p in mine) / GIB:.2f} GiB; largest: " + (f"{mine[0].label} {mine[0].bytes / GIB:.2f} GiB" if mine else "none"))
        print("largest of this user's on the host: " + ", ".join(f"{h['name']} (pid {h['pid']}) {h['gib']} GiB" for h in largest_on_host(table, 3)))
    except Exception as error:
        print(f"the process table cannot be read: {error}")
    pid = watcher_pid(root)
    print(("watcher: running, and reading the host" if healthy(root, limits) else "watcher: running, and BLIND: it has not read the host lately") if pid else "watcher: not running")
    try:
        print(f"peak since its start: {(state / 'peak.json').read_text(errors='replace')}")
    except OSError:
        pass
    try:
        with open(state / "events.log", "rb") as log:
            log.seek(max(0, size_of(state / "events.log") - LOG_AT_MOST))
            lines = log.read().splitlines()
    except OSError:
        lines = []
    events = []
    for line in lines:
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if isinstance(event, dict):
            events.append(event)
    # What was stopped is what it is asked: the last few of those, however many other lines came after.
    for event in [e for e in events if e.get("kind") == "stopped"][-3:]:
        try:
            print(said_of(event))
        except (AttributeError, KeyError, TypeError):
            print("a line of the log does not read")
    for event in [e for e in events if e.get("kind") != "stopped"][-2:]:
        print(f"{event.get('at')}: {event.get('kind')}: {event.get('reasons', event.get('error'))}")
    reasons = start_reasons(reading, limits)
    print("a heavy job may start" if not reasons else f"a heavy job may not start: {'; '.join(reasons)}")
    return 0


def canonical(root: Path) -> Path:
    """A root as the volume spells it. This volume ignores case, and what the kernel says of a process's directory does not: spelt otherwise, nothing would be ours."""
    resolved = root.resolve()
    try:
        fd = os.open(str(resolved), os.O_RDONLY)
    except OSError:
        return resolved
    try:
        spelt = fcntl.fcntl(fd, fcntl.F_GETPATH, bytes(1024)).split(b"\0", 1)[0].decode()
    except (OSError, ValueError):
        return resolved
    finally:
        os.close(fd)
    return Path(spelt) if spelt.lower() == str(resolved).lower() else resolved


def main(argv: list[str]) -> int:
    args = list(argv)
    verb = args.pop(0) if args else "status"
    if sys.platform != "darwin":
        # This host is macOS. Elsewhere nothing is read, so nothing is claimed: a job runs unwatched, and says it.
        if verb == "run":
            command = args[args.index("--") + 1:] if "--" in args else args
            print("host-guard: this reads macOS only: running unwatched", file=sys.stderr)
            return subprocess.run(command).returncode
        print("host-guard: this reads macOS only", file=sys.stderr)
        return 0 if verb == "hook" else 2
    root, overrides, once, wait_seconds, named = repository_root(), {}, False, 0.0, False
    while args and args[0] in ("--root", "--limits", "--once", "--wait"):
        flag = args.pop(0)
        if flag == "--once":
            once = True
        elif flag == "--wait":
            wait_seconds = float(args.pop(0))
        elif flag == "--root":
            root, named = Path(args.pop(0)), True
        else:
            overrides = json.loads(args.pop(0))
    root = canonical(root)
    # A `run` says whose it is on its command line, which is how the watcher knows its job is ours, wherever it was started from.
    if verb == "run" and not named:
        os.execv(sys.executable, [sys.executable, "-I", "-B", str(Path(__file__).resolve()), "run", "--root", str(root), *argv[1:]])
    try:
        limits = tightened(overrides)
    except ValueError as error:
        print(f"host-guard: {error}", file=sys.stderr)
        return 2
    if verb == "check":
        try:
            reasons = start_reasons(read_host(root), limits)
        except Exception as error:
            print(f"not clear: the host cannot be read, so nothing says this is safe: {error}")
            return UNCLEAR
        print("clear" if not reasons else f"not clear: {'; '.join(reasons)}")
        return UNCLEAR if reasons else 0
    if verb == "run":
        return run(root, limits, args[1:] if args[:1] == ["--"] else args, wait_seconds)
    if verb == "watch":
        return watch(root, limits, once)
    if verb == "ensure":
        ensure(root)
        return 0
    if verb == "hook":
        return hook(root, limits)
    if verb == "status":
        return status(root, limits)
    if verb == "stop":
        pid = watcher_pid(root)
        if pid:
            os.kill(pid, signal.SIGTERM)
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
