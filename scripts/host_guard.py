#!/usr/bin/env python3
"""host_guard.py: this repository's work never endangers the host it shares (`docs/decisions/decision_host-guard.md`).

The machine holds 24 GiB and many other projects. The director's lines: RAM use above 88% is the danger zone, above 90% the host may
crash. So the limits below hold well inside them, on what the whole host uses, and on what this repository's own processes use.

    host_guard.py check            one reading; exit 0 clear to start a heavy job, 75 not
    host_guard.py run [--wait S] -- CMD...    a heavy job: refused unless clear, or waiting up to S seconds for room; at lower priority,
                                   watched, its exit status kept
    host_guard.py ensure           the watcher running, for this repository
    host_guard.py watch            the watcher itself; one per repository
    host_guard.py hook             the harness's check before a shell command: exit 2 refuses it
    host_guard.py status           what it reads now, and what it has stopped
    host_guard.py stop             the watcher stopped, for a handoff

It reads macOS, the host this runs on, by macOS's own figures: Activity Monitor's sum for RAM, the kernel's memory pressure level, swap, and
each process's footprint. On any other system it watches nothing and says so.

What it stops is only ever this repository's: a process whose working directory or command line is under the root, what those started,
and what an agent working here started. Never another project's, never by a program's name across the host, never an agent itself.
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


def state_dir(root: Path) -> Path:
    # A scoped root is its own state; the repository's is under its ignored `target/`.
    state = root / "target" / "host-guard" / "state" if (root / ".git").is_dir() else root / "state"
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
    done = subprocess.run(["sysctl", "-n", *names], capture_output=True, text=True, timeout=20, check=True)
    return done.stdout.splitlines()


def free_gib(path: str) -> float:
    stats = os.statvfs(path)
    return stats.f_bavail * stats.f_frsize / GIB


def read_memory() -> tuple[float, int, int, float]:
    """RAM used in percent, as Activity Monitor adds it or as the kernel's free percentage has it if that is higher; the kernel's pressure
    level; its free percentage; and swap used in GiB."""
    memsize, level, pressure, swap = sysctl("hw.memsize", "kern.memorystatus_level", "kern.memorystatus_vm_pressure_level", "vm.swapusage")
    pages = subprocess.run(["vm_stat"], capture_output=True, text=True, timeout=20, check=True).stdout
    page = int(re.search(r"page size of (\d+) bytes", pages).group(1))

    def count(label: str) -> int:
        found = re.search(rf"^{label}:\s+(\d+)\.", pages, re.M)
        return int(found.group(1)) if found else 0

    # App memory, wired and compressed. Cached files are not use: macOS gives them back.
    used = count("Anonymous pages") - count("Pages purgeable") + count("Pages wired down")
    used += count("Pages occupied by compressor")
    by_pages = 100.0 * used * page / int(memsize)
    swap_used = re.search(r"used = ([\d.]+)M", swap)
    return max(by_pages, 100.0 - float(level)), int(pressure), int(level), float(swap_used.group(1)) / 1024.0


def read_host(root: Path) -> Reading:
    used, pressure, free_says, swap = read_memory()
    return Reading(used, pressure, free_says, swap, free_gib(str(root)), free_gib("/"))


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


def disk_reasons(reading: Reading, limits: Limits) -> list[str]:
    reasons = []
    if reading.repo_free_gib < limits.stop_repo_free_gib:
        reasons.append(f"the repository's volume has {reading.repo_free_gib:.0f} GiB free, under {limits.stop_repo_free_gib:.0f}")
    if reading.boot_free_gib < limits.stop_boot_free_gib:
        reasons.append(f"the boot volume, where the host swaps, has {reading.boot_free_gib:.0f} GiB free, under {limits.stop_boot_free_gib:.0f}")
    return reasons


# ---- processes


@dataclass
class Process:
    pid: int
    ppid: int
    command: str
    cwd: str
    bytes: int

    @property
    def name(self) -> str:
        return os.path.basename(self.command.split(" ", 1)[0]) if self.command else ""


class Darwin:
    """What `ps` cannot say: a process's working directory, and its footprint, which counts what is compressed and swapped. RSS does not:
    on `2026-11-05` the host's swap held 62 GiB while the largest process showed 0.9."""

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


NATIVE = Darwin() if sys.platform == "darwin" else None


def processes() -> list[Process]:
    """This user's processes, nobody else's."""
    me = os.getuid()
    # A command line is bytes, and any process may hold ones that are no text. One such process must not blind the watcher.
    listed = subprocess.run(["ps", "-axww", "-o", "pid=,ppid=,uid=,rss=,command="], capture_output=True, text=True, errors="replace", timeout=30, check=True)
    table = []
    for line in listed.stdout.splitlines():
        parts = line.split(None, 4)
        if len(parts) < 5 or int(parts[2]) != me:
            continue
        pid = int(parts[0])
        held = NATIVE.footprint(pid)
        table.append(Process(pid, int(parts[1]), parts[4], NATIVE.cwd(pid), int(parts[3]) * 1024 if held is None else held))
    return table


# Never stopped: an agent or a terminal may be working here, and a guard that ended them would end the work that could say why.
PROTECTED = {"claude", "node", "codex", "kimi", "gemini", "aider", "cursor", "login", "sshd", "tmux", "screen", "iTerm2", "Code", "caffeinate"}
SHELL_LOGIN = re.compile(r"^-")
APPLICATION = ("/Applications/", "/System/", "/usr/libexec/", "/usr/sbin/")


def protected(process: Process) -> bool:
    return (
        process.name in PROTECTED
        or bool(SHELL_LOGIN.match(process.name))
        or process.command.startswith(APPLICATION)
        or "host_guard.py" in process.command
    )


def ours(table: list[Process], root: Path, me: int) -> list[Process]:
    """This repository's processes that may be stopped, largest first: under its root by working directory or command line, what those
    started, and what an agent working in it started. The guard's own line of parents is never among them."""
    text = str(root)
    named = re.compile(re.escape(text) + r"(?:[/\s\"']|$)")
    by_pid = {process.pid: process for process in table}
    children: dict[int, list[int]] = {}
    for process in table:
        children.setdefault(process.ppid, []).append(process.pid)
    under = lambda path: path == text or path.startswith(text + os.sep)
    found = {p.pid for p in table if under(p.cwd) or named.search(p.command)}
    todo = list(found)
    while todo:
        for child in children.get(todo.pop(), []):
            if child not in found:
                found.add(child)
                todo.append(child)
    line = set()
    at = me
    while at in by_pid and at not in line:
        line.add(at)
        at = by_pid[at].ppid
    return sorted(
        (by_pid[pid] for pid in found if pid not in line and not protected(by_pid[pid])),
        key=lambda process: process.bytes,
        reverse=True,
    )


def tree_of(pid: int, table: list[Process]) -> list[int]:
    """`pid` and what it started, children first: a parent's death does not reach them."""
    children: dict[int, list[int]] = {}
    for process in table:
        children.setdefault(process.ppid, []).append(process.pid)
    order, todo = [], [pid]
    while todo:
        at = todo.pop()
        order.append(at)
        todo.extend(children.get(at, []))
    order.reverse()
    return order


def alive(pid: int) -> bool:
    try:
        os.kill(pid, 0)
    except OSError:
        return False
    # A zombie holds nothing, and answers as if it were running until its parent reaps it.
    state = subprocess.run(["ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return bool(state) and not state.startswith("Z")


def stop(pids: list[int], seconds: float = 1.0) -> None:
    """Ask, then make sure. Under memory pressure a process may not get to run its handler, so the wait is short."""
    for pid in pids:
        try:
            os.kill(pid, signal.SIGTERM)
        except OSError:
            pass
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline and any(alive(pid) for pid in pids):
        time.sleep(0.05)
    for pid in pids:
        try:
            os.kill(pid, signal.SIGKILL)
        except OSError:
            pass


# ---- the watcher


def record(state: Path, **event) -> None:
    event["at"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    with open(state / "events.log", "a", encoding="utf-8") as log:
        log.write(json.dumps(event) + "\n")


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
        everything.append(f"the repository's volume lost {baseline.repo_free_gib - reading.repo_free_gib:.0f} GiB during this work, over {limits.stop_disk_growth_gib:.0f}")
    if everything:
        # A small git frees nothing, and one stopped mid-write leaves a lock or a half-written index for the next command.
        return [p for p in mine if not (p.name == "git" and p.bytes < limits.worth_stopping_gib * GIB)], everything, False
    if len(mine) > limits.processes:
        return mine[: len(mine) - int(limits.processes)], [f"{len(mine)} processes of ours, over {limits.processes:.0f}"], False
    if mine and mine[0].bytes > limits.one_process_gib * GIB:
        return [mine[0]], [f"one process of ours holds {mine[0].bytes / GIB:.1f} GiB, over {limits.one_process_gib:.1f}"], False
    if total > limits.all_processes_gib * GIB:
        return [mine[0]], [f"our processes hold {total / GIB:.1f} GiB, over {limits.all_processes_gib:.1f}"], False
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
    """Who holds the memory, this user's processes of any project: read and said, and never acted on."""
    largest = sorted(table, key=lambda process: process.bytes, reverse=True)[:count]
    return [{"pid": p.pid, "name": p.name, "gib": round(p.bytes / GIB, 2), "command": p.command[:120]} for p in largest]


def note_peak(state: Path, reading: Reading, ours_bytes: int) -> None:
    """The most this watcher has seen, so a limit can be set from what a job really used. `status` says it; a new watcher starts again."""
    path = state / "peak.json"
    try:
        peak = json.loads(path.read_text())
    except (OSError, ValueError):
        peak = {"used_percent": 0.0, "ours_gib": 0.0, "swap_gib": 0.0}
    now = {"used_percent": round(reading.used_percent, 1), "ours_gib": round(ours_bytes / GIB, 2), "swap_gib": round(reading.swap_used_gib, 1)}
    if any(now[name] > peak[name] for name in now):
        path.write_text(json.dumps({name: max(now[name], peak[name]) for name in now}))


def sample(root: Path, limits: Limits, state: Path, baseline: Reading | None, said: list[float]) -> tuple[Reading, Reading | None, list[Process]]:
    reading = read_host(root)
    table = processes()
    mine = ours(table, root, os.getpid())
    # What this work grew is measured from when it began: an idle repository starts the count again.
    worth = sum(process.bytes for process in mine) >= limits.worth_stopping_gib * GIB
    baseline = (baseline or reading) if worth else None
    note_peak(state, reading, sum(process.bytes for process in mine))
    targets, reasons, not_ours = judged(reading, mine, limits, baseline)
    if not_ours:
        if time.monotonic() - said[0] > 60:
            said[0] = time.monotonic()
            record(state, kind="pressure-not-ours", reasons=reasons, reading=asdict(reading), largest=largest_on_host(table))
    elif targets:
        stopped = []
        for target in targets:
            stop(tree_of(target.pid, table))
            stopped.append({"pid": target.pid, "name": target.name, "gib": round(target.bytes / GIB, 2), "command": target.command[:160]})
        record(state, kind="stopped", reasons=reasons, stopped=stopped, reading=asdict(reading), largest=largest_on_host(table))
    return reading, baseline, mine


def watch(root: Path, limits: Limits, once: bool, idle_seconds: float = 600.0) -> int:
    state = state_dir(root)
    lock = open(state / "watch.lock", "w")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        return 0  # One is watching.
    (state / "watch.pid").write_text(str(os.getpid()))
    (state / "peak.json").unlink(missing_ok=True)
    os.chdir("/")
    baseline, said, last_seen = None, [0.0], time.monotonic()
    while True:
        try:
            reading, baseline, mine = sample(root, limits, state, baseline, said)
            near = reading.used_percent >= limits.start_used_percent
            busy = sum(p.bytes for p in mine) >= limits.worth_stopping_gib * GIB
        except (OSError, subprocess.SubprocessError, ValueError, AttributeError) as error:
            record(state, kind="unread", error=str(error))
            near, busy = True, True
        if once:
            return 0
        heartbeat = state / "heartbeat"
        fresh = heartbeat.exists() and time.time() - heartbeat.stat().st_mtime < idle_seconds
        if busy or fresh:
            last_seen = time.monotonic()
        elif time.monotonic() - last_seen > idle_seconds:
            return 0
        time.sleep(0.25 if near else 1.0)


def ensure(root: Path) -> None:
    """A watcher for `root`, started if none holds the lock. It outlives this call and ends itself when the work has been idle."""
    state = state_dir(root)
    lock = open(state / "watch.lock", "a")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        return
    fcntl.flock(lock, fcntl.LOCK_UN)
    lock.close()
    with open(state / "watch.out", "a") as out:
        subprocess.Popen(
            [sys.executable, "-I", "-B", str(Path(__file__).resolve()), "watch", "--root", str(root)],
            stdin=subprocess.DEVNULL, stdout=out, stderr=out, start_new_session=True, cwd="/",
        )


def unseen(state: Path) -> list[dict]:
    """What has been stopped since an agent last read it, marking it read."""
    log, seen = state / "events.log", state / "seen"
    try:
        start = int(seen.read_text())
    except (OSError, ValueError):
        start = 0
    if size_of(log) <= start:
        return []
    with open(log, encoding="utf-8") as lines:
        lines.seek(start)
        text = lines.read()
    seen.write_text(str(start + len(text.encode())))
    return [event for event in map(json.loads, text.splitlines()) if event.get("kind") == "stopped"]


def said_of(event: dict) -> str:
    names = ", ".join(f"{s['name']} (pid {s['pid']}, {s['gib']} GiB)" for s in event["stopped"][:4])
    more = len(event["stopped"]) - 4
    holders = ", ".join(f"{h['name']} {h['gib']} GiB" for h in event.get("largest", [])[:3])
    return (
        f"{event['at']}: stopped {names}{f' and {more} more' if more > 0 else ''}: {'; '.join(event['reasons'])}"
        + (f". Holding the most on the host: {holders}" if holders else "")
    )


# ---- what runs through it

# What may use real memory or write much, where a command stands, so that the word in a message or a search is not taken for one. `cargo fmt` is
# neither. `run` answers for itself, and what reaches past this the watcher still holds.
BEFORE = r"(?:^|[;&|(\n]|\b(?:do|then|else)\s)\s*(?:\w+=\S*\s+)*(?:(?:timeout\s+\S+|nice(?:\s+-n\s*\S+)?|time|exec)\s+)*"
HEAVY = re.compile(BEFORE + r"(?:\S*/)?(?:cargo(?!\s+fmt\b)|make|rustc|mdbook|qemu-system\S*)(?:\s|$)|/install/archogen|archogen-plan-check")
# What only reads the host, or stops what is ours: always let through, so a host in trouble can still be looked at and helped.
READS = re.compile(r"^\s*(?:ps|pgrep|sysctl|vm_stat|df|memory_pressure|top -l 1|kill|pkill|git (?:status|log|diff --stat)|python3? (?:-\S+ )*\S*host_guard\.py)\b[^;&|]*$")
BUILDERS = {"cargo", "rustc", "make"}


def run(root: Path, limits: Limits, command: list[str], wait_seconds: float = 0.0) -> int:
    if not command:
        print("host-guard: `run` takes a command, after `--`", file=sys.stderr)
        return 2
    ensure(root)
    state = state_dir(root)
    reasons = start_reasons(read_host(root), limits)
    # The host is shared: when it is full, the polite thing is to wait for room, not to add to it.
    deadline = time.monotonic() + wait_seconds
    while reasons and time.monotonic() < deadline:
        time.sleep(min(15.0, max(0.1, deadline - time.monotonic())))
        (state / "heartbeat").touch()
        reasons = start_reasons(read_host(root), limits)
    if reasons:
        print(f"host-guard: not started: {'; '.join(reasons)}.", file=sys.stderr)
        print("  hint: wait for the host to have room, and look at `host_guard.py status`", file=sys.stderr)
        return UNCLEAR
    # A build's cost is its parallelism, and this machine is shared: a third of its cores, and a lower priority.
    env = dict(os.environ)
    jobs = str(max(2, (os.cpu_count() or 4) // 3))
    env.setdefault("CARGO_BUILD_JOBS", jobs)
    env.setdefault("RUST_TEST_THREADS", jobs)
    begun = size_of(state / "events.log")
    # At least this low, however many guards a job is nested in.
    done = subprocess.run(command, env=env, preexec_fn=lambda: os.nice(max(0, 10 - os.nice(0))))
    if size_of(state / "events.log") > begun:
        with open(state / "events.log", encoding="utf-8") as lines:
            lines.seek(begun)
            for event in map(json.loads, lines.read().splitlines()):
                if event.get("kind") == "stopped":
                    print(f"host-guard: {said_of(event)}", file=sys.stderr)
    return done.returncode


def hook(root: Path, limits: Limits) -> int:
    """Exit 2 refuses the command, and what it prints goes to the agent."""
    try:
        command = json.load(sys.stdin).get("tool_input", {}).get("command", "")
    except (ValueError, AttributeError):
        command = ""
    if READS.match(command):
        return 0
    ensure(root)
    state = state_dir(root)
    (state / "heartbeat").touch()
    happened = unseen(state)
    if happened:
        for event in happened:
            print(f"host-guard: {said_of(event)}", file=sys.stderr)
        print("host-guard: this command was not run, so that you see that. Find out why before running it again.", file=sys.stderr)
        return 2
    try:
        reading = read_host(root)
    except (OSError, subprocess.SubprocessError, ValueError, AttributeError) as error:
        print(f"host-guard: refused: the host cannot be read, so nothing says this is safe: {error}", file=sys.stderr)
        return 2
    if reading.used_percent >= limits.stop_used_percent or disk_reasons(reading, limits):
        why = [f"RAM use is {reading.used_percent:.0f}%"] if reading.used_percent >= limits.stop_used_percent else disk_reasons(reading, limits)
        print(f"host-guard: refused: {'; '.join(why)}. Reading the host is still allowed.", file=sys.stderr)
        return 2
    if HEAVY.search(command):
        reasons = start_reasons(reading, limits)
        if reasons:
            print(f"host-guard: refused, this is a heavy command: {'; '.join(reasons)}.", file=sys.stderr)
            return 2
        running = [p for p in ours(processes(), root, os.getpid()) if p.name in BUILDERS]
        if running:
            print(f"host-guard: refused: one heavy job at a time, and {running[0].name} (pid {running[0].pid}) is running: {running[0].command[:120]}", file=sys.stderr)
            return 2
    return 0


def status(root: Path, limits: Limits) -> int:
    state = state_dir(root)
    reading = read_host(root)
    print(f"RAM used {reading.used_percent:.1f}% (start under {limits.start_used_percent:.0f}, stop at {limits.stop_used_percent:.0f}, all stopped at {limits.panic_used_percent:.0f}); macOS: pressure level {reading.pressure_level} (1 normal, 2 warning, 4 critical), {reading.free_percent_says}% free; swap {reading.swap_used_gib:.1f} GiB")
    print(f"free: repository volume {reading.repo_free_gib:.0f} GiB, boot volume {reading.boot_free_gib:.0f} GiB")
    table = processes()
    mine = ours(table, root, os.getpid())
    print(f"ours: {len(mine)} process(es), {sum(p.bytes for p in mine) / GIB:.2f} GiB; largest: " + (f"{mine[0].name} {mine[0].bytes / GIB:.2f} GiB" if mine else "none"))
    print("largest on the host: " + ", ".join(f"{h['name']} (pid {h['pid']}) {h['gib']} GiB" for h in largest_on_host(table, 3)))
    print("watcher: " + ("running" if watcher_pid(state) else "not running"))
    if (state / "peak.json").exists():
        print(f"peak since its start: {(state / 'peak.json').read_text()}")
    log = state / "events.log"
    if log.exists():
        for event in map(json.loads, log.read_text().splitlines()[-5:]):
            print(said_of(event) if event.get("kind") == "stopped" else f"{event['at']}: {event.get('kind')}: {event.get('reasons', event.get('error'))}; largest: {[(h['name'], h['gib']) for h in event.get('largest', [])[:3]]}")
    reasons = start_reasons(reading, limits)
    print("a heavy job may start" if not reasons else f"a heavy job may not start: {'; '.join(reasons)}")
    return UNCLEAR if reasons else 0


def watcher_pid(state: Path) -> int | None:
    try:
        pid = int((state / "watch.pid").read_text())
    except (OSError, ValueError):
        return None
    lock = open(state / "watch.lock", "a")
    try:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        return pid
    finally:
        lock.close()
    return None


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
        reasons = start_reasons(read_host(root), limits)
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
        pid = watcher_pid(state_dir(root))
        if pid:
            os.kill(pid, signal.SIGTERM)
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
