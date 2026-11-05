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
for nothing it started. Never another project's, never by a program's name across the host, never an agent, a terminal or the harness's helpers.
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
    stop_disk_growth_gib: float = 50.0
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
    return table


# Who may be working here and must never be stopped. A name is searched for, not matched whole: `cursor-agent`, and an agent run by an interpreter
# goes by its script. Wrongly sparing a process costs less than wrongly stopping an agent, and the record says so.
AGENT = re.compile(r"claude|codex|kimi|gemini|aider|cursor|copilot|opencode|goose", re.I)
INFRASTRUCTURE = {"login", "sshd", "tmux", "screen", "iTerm2", "Terminal", "Code", "caffeinate"}
APPLICATION = ("/Applications/", "/System/", "/usr/libexec/", "/usr/sbin/")
GUARD_PROCESS = re.compile(r"^\S*[Pp]ython\S*\s+(?:-\S+\s+)*\S*host_guard\.py(?:\s|$)")
# Small, and stopping them frees nothing: a `git` could leave a lock or a half-written index behind, and these others are how a host in trouble is looked at.
SPARED_WHEN_SMALL = {"git", "ps", "top", "sysctl", "vm_stat", "df", "du", "lsof", "memory_pressure"}


def is_agent(process: Process) -> bool:
    return any(AGENT.search(word) for word in (process.name, os.path.basename(process.exe), process.script))


def protected(process: Process, by_pid: dict[int, Process]) -> bool:
    if process.name in INFRASTRUCTURE or is_agent(process) or GUARD_PROCESS.search(process.command):
        return True
    if process.name.startswith("-") and process.name[1:] in SHELLS:  # a login shell
        return True
    if process.command.startswith(APPLICATION) or process.exe.startswith(APPLICATION):
        return True
    # What an agent runs for itself, an MCP server, is not a job. What it runs as a command goes through a shell.
    parent = by_pid.get(process.ppid)
    return parent is not None and is_agent(parent) and process.name not in SHELLS


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
    runs = [
        p.pid
        for p in table
        if GUARD_PROCESS.search(p.command) and re.search(r"host_guard\.py\s+run(?:\s|$)", p.command) and (under(p.cwd) or f"--root {text}" in p.command)
    ]
    todo = list(found) + runs
    while todo:
        for child in children.get(todo.pop(), []):
            if child not in found and child not in spared:
                found.add(child)
                todo.append(child)
    line = set()
    at = me
    while at in by_pid and at not in line:
        line.add(at)
        at = by_pid[at].ppid
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


def record(root: Path, **event) -> None:
    """Write down what happened. Never raises: a watcher must not die of its own bookkeeping, `target/` removed by `cargo clean` among the causes."""
    event["at"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    try:
        with open(state_dir(root) / "events.log", "a", encoding="utf-8") as log:
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
    if everything:
        return [p for p in mine if not small_and_spared(p, limits)], everything, False
    if len(mine) > limits.processes:
        return mine[: len(mine) - int(limits.processes)], [f"{len(mine)} processes of ours, over {limits.processes:.0f}"], False
    if mine and mine[0].bytes > limits.one_process_gib * GIB:
        return [mine[0]], [f"one process of ours holds {mine[0].bytes / GIB:.2f} GiB, over {limits.one_process_gib:.2f}"], False
    if total > limits.all_processes_gib * GIB:
        return [mine[0]], [f"our processes hold {total / GIB:.2f} GiB, over {limits.all_processes_gib:.2f}"], False
    if reading.used_percent >= limits.stop_used_percent:
        host.append(f"RAM use is {reading.used_percent:.0f}%, at the {limits.stop_used_percent:.0f}% stop line")
    if reading.pressure_level >= limits.stop_pressure_level:
        host.append(f"macOS reports memory pressure at level {reading.pressure_level}, where 1 is normal")
    if baseline and reading.swap_used_gib - baseline.swap_used_gib > limits.stop_swap_growth_gib:
        host.append(f"swap grew {reading.swap_used_gib - baseline.swap_used_gib:.1f} GiB during this work, over {limits.stop_swap_growth_gib:.1f}")
    if not host:
        return [], [], False
    if total < limits.worth_stopping_gib * GIB:
        return [], host, True
    return [mine[0]], host, False


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
        except (OSError, ValueError, TypeError, KeyError):
            peak = dict.fromkeys(now, 0.0)
        if any(now[name] > peak[name] for name in now):
            path.write_text(json.dumps({name: max(now[name], peak[name]) for name in now}))
    except OSError as error:
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
    baseline, said, unread_said, last_seen = None, [0.0], 0.0, time.monotonic()
    while True:
        near, busy = True, False
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
    shown = subprocess.run(["ps", "-ww", "-o", "command=", "-p", str(pid)], capture_output=True, text=True, errors="replace").stdout
    return pid if GUARD_PROCESS.search(shown) and re.search(r"host_guard\.py\s+watch\b", shown) and f"--root {root}" in shown else None


def healthy(root: Path, limits: Limits) -> bool:
    """Whether a watcher has read the host lately: alive is not enough, since one that cannot read the process table stops nothing."""
    try:
        return time.time() - (state_path(root) / "sampled").stat().st_mtime < limits.watcher_stale_seconds
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
    if end <= start:
        return []
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
HEAVY = re.compile(BEFORE + r"(?:\S*/)?(?:cargo(?!\s+fmt\b)[\w-]*|g?make|rustc|mdbook|qemu-system\S*|archogen|archogen-plan-check|build_book\.sh)(?=[\s;&|)'\"`]|$)|/install/archogen")
BUILDERS = {"cargo", "rustc", "make", "gmake"}
# What is safe at any line, since a host in trouble must stay readable and it must be possible to free disk. Each stage of a command must be one.
READERS = {
    "ps", "pgrep", "sysctl", "vm_stat", "df", "du", "lsof", "memory_pressure", "uptime", "sort", "uniq", "head", "tail", "grep", "cut", "wc", "tr",
    "cat", "ls", "stat", "date", "echo", "printf", "true", ":", "cd", "export", "kill", "pkill", "rm",
}
GUARD_READS = {"status", "check", "ensure", "stop"}


def is_read(command: str) -> bool:
    """Whether every stage of `command` only reads the host, frees disk, or stops what is ours. Nothing may ride in: no substitution, no background,
    no redirection but away from the screen. Quotes are not parsed, so a `|` inside one refuses a read, which is the safe way to be wrong."""
    text = re.sub(r"\s*\d?>\s*/dev/null|\s*\d>&\d", " ", command)
    if re.search(r"[<>`]|\$\(", text) or re.search(r"(?<!&)&(?!&)", text):
        return False
    said = False
    for stage in re.split(r"\|\||&&|[;|\n]", text):
        words = stage.split()
        if not words:
            continue
        said = True
        head = os.path.basename(words[0])
        if head in READERS:
            continue
        if head == "top" and "-l" in words:
            continue
        if head == "mkdir" and all(w == "-p" or "target/" in w for w in words[1:]):
            continue
        if head == "git" and len(words) > 1 and words[1] in ("status", "log", "diff"):
            continue
        if head.startswith("python"):
            rest = [w for w in words[1:] if not w.startswith("-")]
            if len(rest) >= 2 and os.path.basename(rest[0]) == "host_guard.py" and rest[1] in GUARD_READS:
                continue
        return False
    return said


def code_of(command: str) -> str:
    """`command` without what it only says: a heredoc's body, and quoted text unless a shell is told to run it (`-c '…'`)."""
    text = re.sub(r"<<-?\s*(['\"]?)(\w+)\1[^\n]*\n.*?\n\s*\2[ \t]*(?:\n|$)", " ", command, flags=re.S)
    return re.sub(r"(-c\s+)?('[^']*'|\"(?:[^\"\\]|\\.)*\")", lambda m: m.group(0) if m.group(1) else " ", text)


def heavy_running(root: Path) -> list[Process]:
    return [p for p in ours(processes(), root, os.getpid()) if p.name in BUILDERS]


def run(root: Path, limits: Limits, command: list[str], wait_seconds: float = 0.0) -> int:
    if not command:
        print("host-guard: `run` takes a command, after `--`", file=sys.stderr)
        return 2
    state = state_dir(root)
    for event in unseen(root):
        print(f"host-guard: {said_of(event)}", file=sys.stderr)
    ensure(root)
    nested = os.environ.get("HOST_GUARD_JOB") == str(root)
    job_lock = open(state / "job.lock", "a")
    begun = time.monotonic()

    def why_not() -> list[str]:
        try:
            reasons = start_reasons(read_host(root), limits)
        except Exception as error:
            return [f"the host cannot be read, so nothing says this is safe: {error}"]
        if nested:
            return reasons
        running = heavy_running(root)
        if running:
            reasons.append(f"one heavy job at a time, and {running[0].name} (pid {running[0].pid}) is running")
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
    env["HOST_GUARD_JOB"] = str(root)
    logged = size_of(state / "events.log")
    # At least this low, however many guards a job is nested in.
    job = subprocess.Popen(command, env=env, preexec_fn=lambda: os.nice(max(0, 10 - os.nice(0))))
    signal.signal(signal.SIGTERM, lambda *_: job.terminate())
    blind_since = None
    while True:
        try:
            status = job.wait(timeout=5)
            break
        except subprocess.TimeoutExpired:
            pass
        # An unwatched job is what this exists to prevent: keep a watcher running, and end the job if none can see.
        try:
            ensure(root)
            state_dir(root)
            (state_path(root) / "heartbeat").touch()
        except OSError:
            pass
        if healthy(root, limits):
            blind_since = None
        else:
            blind_since = blind_since if blind_since is not None else time.monotonic()
            if time.monotonic() - blind_since > limits.watcher_stale_seconds:
                print("host-guard: no watcher can read the host, so this job is unwatched: stopping it", file=sys.stderr)
                table = processes()
                stop(tree_of(job.pid, table, limits))
                job.wait()
                return UNCLEAR
    try:
        with open(state_path(root) / "events.log", "rb") as lines:
            lines.seek(logged)
            text = lines.read()
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
        command = json.load(sys.stdin).get("tool_input", {}).get("command", "")
    except (ValueError, AttributeError):
        command = ""
    if command and is_read(command):
        return 0
    try:
        return judge_command(root, limits, command)
    except Exception as error:
        print(f"host-guard: refused: the guard cannot judge this: {error}. Reads still run, `rm -rf target/host-guard/state` among them.", file=sys.stderr)
        return 2


def judge_command(root: Path, limits: Limits, command: str) -> int:
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
        print(f"peak since its start: {(state / 'peak.json').read_text()}")
    except OSError:
        pass
    try:
        lines = (state / "events.log").read_bytes().splitlines()[-5:]
    except OSError:
        lines = []
    for line in lines:
        try:
            event = json.loads(line)
            print(said_of(event) if event.get("kind") == "stopped" else f"{event.get('at')}: {event.get('kind')}: {event.get('reasons', event.get('error'))}; largest: {[(h['name'], h['gib']) for h in event.get('largest', [])[:3]]}")
        except (ValueError, AttributeError, KeyError, TypeError):
            print("a line of the log does not read")
    reasons = start_reasons(reading, limits)
    print("a heavy job may start" if not reasons else f"a heavy job may not start: {'; '.join(reasons)}")
    return 0


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
    root, overrides, once, wait_seconds = repository_root(), {}, False, 0.0
    while args and args[0] in ("--root", "--limits", "--once", "--wait"):
        flag = args.pop(0)
        if flag == "--once":
            once = True
        elif flag == "--wait":
            wait_seconds = float(args.pop(0))
        elif flag == "--root":
            root = Path(args.pop(0)).resolve()
        else:
            overrides = json.loads(args.pop(0))
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
