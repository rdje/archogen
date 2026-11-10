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
each process's footprint and what it has written. On any other system it watches nothing and says so.

What is ours goes by what each process shows of itself: its working directory or its executable under the root, or having been started by a process
that shows so. A job is the topmost such process with all it started, and a job is what a line judges and what it stops: a compiler stopped under a script would only
be followed by the next. So a job that holds little and is no build, a `git`, an `rm`, an editor or a read, is left alone at a host's line, whatever it is called, and at a
disk's unless it is writing. Never another project's, never by a program's name across the host, never an agent or a terminal. It guards against accident, not against an
agent set on going round it.
"""

from __future__ import annotations

import ctypes
import fcntl
import itertools
import json
import os
import re
import shlex
import signal
import subprocess
import sys
import time
import unicodedata
from collections import deque
from dataclasses import asdict, dataclass, field, fields
from pathlib import Path

GIB = 1 << 30
UNCLEAR = 75  # EX_TEMPFAIL: not now, and nothing was run.


@dataclass
class Limits:
    """Each line the guard holds. An override may only tighten one (`tightened`), so no flag can loosen the guard."""

    # RAM the whole host uses, in percent. The director's danger zone starts at 88.
    start_used_percent: float = 70.0  # no heavy job starts above this
    stop_used_percent: float = 85.0  # our largest job stops
    panic_used_percent: float = 88.0  # every job of ours that holds memory stops, and every build
    # macOS's own judgement, `kern.memorystatus_vm_pressure_level`: 1 normal, 2 warning, 4 critical. With the compressor taking the load, RAM
    # use can sit still while swap climbs, so this says what a percentage cannot.
    start_pressure_level: float = 2  # no heavy job starts at this
    stop_pressure_level: float = 2  # our largest job stops
    panic_pressure_level: float = 4  # every job of ours that holds memory stops, and every build
    # Swap, which on this machine reached 62 GiB before anyone noticed, RSS showing under one.
    start_swap_gib: float = 16.0
    stop_swap_growth_gib: float = 3.0  # within the window below, while ours runs
    # What this repository's own processes may hold: a quarter of the machine, an eighth in one.
    all_processes_gib: float = 6.0
    one_process_gib: float = 3.0
    processes: float = 512
    # Disk: what must stay free where this repository writes and where the host swaps, and what may go in one window.
    stop_repo_free_gib: float = 100.0
    stop_boot_free_gib: float = 30.0
    stop_disk_growth_gib: float = 50.0  # lost on the volume within the window below while ours runs, whoever wrote it: nothing cheap can say who did
    # Under this a job holds nothing worth stopping it for at a host's line, whatever it is called: stopping it would not help the host.
    worth_stopping_gib: float = 0.25
    # A watcher that has not read the host for this long is not watching, however alive it looks.
    watcher_stale_seconds: float = 60.0


CODE = Path(__file__).resolve()
# How far back growth is counted. A long-lived small process of ours, a `tail -f`, must not make "while ours runs" mean hours of other projects' growth.
GROWTH_WINDOW_SECONDS = 900.0
# What a job must have written since the last sample to count as writing, at a disk floor.
WRITING_BYTES = 1 << 20
# What a volume must lose within this long to have fallen, at a floor with no writer seen. Other projects move this shared volume's figure by hundreds of MiB, the record says what was measured, so a
# small fall says nothing of ours, and a script of `rm` was stopped for one: a floor's way out. `check_host_guard.sh` holds these and the windows beside `Limits`.
FELL_BYTES = 1 << 30
FELL_WINDOW_SECONDS = 10.0
LOG_AT_MOST = 1 << 20  # What the log keeps, and the most any read of it can cost.
TOLD_OF_LATELY_SECONDS = 600.0  # A session with no place in the log is told of what was stopped this recently: it may be new, or `cargo clean` may have taken its place.

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


def root_among(parents: list[Path]) -> Path | None:
    """The checkout holding this file: the nearest with a `.git`, directory or file. So a worktree or a submodule of this repository inside another checkout is
    its own root, and never takes that other project for ours."""
    for at in parents:
        if (at / ".git").exists():
            return at
    return None


def repository_root() -> Path:
    root = root_among(list(Path(__file__).resolve().parents))
    if root is None:
        raise SystemExit("host-guard: this file is in no repository")
    return root


def canonical(root: Path) -> Path:
    """A root as the volume spells it. This volume ignores a name's case and its Unicode form, keeping those it was made with, and what the kernel says of a process's directory is that
    spelling: spelt otherwise, nothing would be ours."""
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
    folded = lambda name: unicodedata.normalize("NFD", name).lower()
    return Path(spelt) if folded(spelt) == folded(str(resolved)) else resolved


def state_path(root: Path) -> Path:
    # A scoped root is its own state; a checkout's is under its ignored `target/`.
    return root / "target" / "host-guard" / "state" if (root / ".git").exists() else root / "state"


class Cleared(Exception):
    """A watcher's directory is gone, and a watcher does not make it again."""


# Set in a watcher. Its state is written in a directory only what asks makes: a watcher making its own would stand in the way of whatever is removing it, as `git worktree remove` is, and a removal is told from `cargo clean` by nothing it could read.
WATCHING = False


def state_dir(root: Path, for_a_stop: bool = False) -> Path:
    """Where a root's state is kept, made where it is not: but by a watcher, which writes only where one stands, a stop apart, since what it stopped must be told."""
    if not root.is_dir():
        raise FileNotFoundError(f"{root} is gone")
    state = state_path(root)
    if WATCHING and not for_a_stop:
        if not state.is_dir():
            raise Cleared(str(state))
        return state
    state.mkdir(parents=True, exist_ok=True)
    return state


def say(message: str) -> None:
    """To stderr, if it can be written: a full disk under it must cost nothing else."""
    try:
        print(message, file=sys.stderr, flush=True)
    except (OSError, ValueError):
        pass


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


# ---- processes

PYTHON_FLAGS_WITH_VALUES = ("-W", "-X")
# Whole flags an interpreter reads a value after, in the next word, so that what follows one is no program: any other whole flag takes none, and the next word is what it runs.
# Some of them, each run on this host: `node --stack-size 900` read no such value, so it is not here, and `node --title` leaves no command line to read, the process showing as its title.
FLAGS_WITH_VALUES = {
    "python": ("--check-hash-based-pycs",),
    "node": (
        "-r", "--require", "--import", "--loader", "--experimental-loader", "-C", "--conditions", "--env-file", "--env-file-if-exists", "--inspect-port", "--disable-warning",
        "--unhandled-rejections", "--max-http-header-size", "--dns-result-order", "--redirect-warnings", "--diagnostic-dir", "--report-dir", "--trace-event-categories", "--secure-heap",
        "--input-type", "--report-signal", "--report-filename", "--openssl-config", "--tls-cipher-list", "--icu-data-dir", "--test-reporter", "--localstorage-file", "--use-largepages",
        "--trace-event-file-pattern", "--watch-kill-signal", "--network-family-autoselection-attempt-timeout",
    ),
    "ruby": ("--encoding", "--external-encoding", "--internal-encoding", "--backtrace-limit", "--parser", "--enable", "--disable", "--dump", "--crash-report"),
}
# Whole flags that give `node` a program's text, so that it runs no file. It reads no cluster.
NODE_TEXT_FLAGS = ("-e", "-p", "-pe", "--eval", "--print")
# How an interpreter reads a cluster of one dash, letter by letter, as each was run on this host. `plain` takes nothing, a digit after one being its own; `value` takes the rest
# of the cluster where there is any and the next word where there is none; `text` takes a program's text so, and the interpreter then runs no file; `module` takes a module's
# name so, which is what it runs. Any other letter ends the reading of its cluster, read as taking the rest of the cluster or nothing and never the next word: so `perl -M strict` and
# `ruby -W x` are read, which take none. A letter this does not hold that does take the next word is the record's limit, as `ruby -X dir` was until it was held.
CLUSTERS = {
    "python": {"plain": "bBdEhiIOPqRsStuvVx?", "value": "WX", "text": "c", "module": "m"},
    "perl": {"plain": "acdfnpsStTuUvwWXl0123456789", "value": "I", "text": "eE", "module": ""},
    "ruby": {"plain": "acdhlnpsSUvwy0123456789", "value": "CEIrX", "text": "e", "module": ""},
}
INTERPRETERS = {"python", "python3", "Python", "node", "ruby", "perl", "bash", "sh", "zsh"}
SHELLS = {"bash", "sh", "zsh", "fish", "dash", "ksh", "csh", "tcsh", "nu", "elvish"}


def interpreter_reads(kind: str, words: list[str]) -> tuple[str, int]:
    """The word an interpreter of this kind is given to run, among the words past its own name, and where the words after it begin: as it reads its own flags, the first word that is no flag
    and no value of one, a whole flag's by `FLAGS_WITH_VALUES` and a cluster's letter by letter by `CLUSTERS`, a module's name where Python is given one. No word where a flag gives it a
    program's text or where it reads its input. Each word is read once, so reading costs no more than the words' length."""
    takes_values, letters, at = FLAGS_WITH_VALUES.get(kind, ()), CLUSTERS.get(kind), 0
    while at < len(words):
        word = words[at]
        if word == "--":
            at += 1
            break
        if word == "-" or (kind == "node" and word in NODE_TEXT_FLAGS):
            return "", len(words)
        if not word.startswith("-"):
            break
        took = 2 if word in takes_values else 1
        if letters and not word.startswith("--"):
            for place, letter in enumerate(word[1:], start=1):
                rest = word[place + 1 :]
                if letter in letters["text"]:
                    return "", len(words)
                if letter in letters["module"]:
                    return (rest, at + 1) if rest else (" ".join(words[at + 1 : at + 2]), at + 2)
                if letter in letters["value"]:
                    took = 1 if rest else 2
                    break
                if letter not in letters["plain"]:
                    break
        at += took
    return (words[at], at + 1) if at < len(words) else ("", len(words))


@dataclass
class Process:
    pid: int
    ppid: int
    command: str
    cwd: str
    exe: str
    bytes: int
    written: int = 0  # What it has written in its life.
    waiting: bool = False  # A `make` that has started nothing but a `run` still waiting its turn.
    argv: tuple = ()  # Its arguments as the kernel holds them, the first its own name, each whole whatever spaces it holds; none where the kernel gives none.

    @property
    def name(self) -> str:
        # `command` may hold spaces in its first word, `/Library/Application Support/…`: the executable's own path settles where it ends.
        # A process that renamed itself, `exec -a`, does not begin with it, and goes by the name it chose.
        if self.exe and self.command.startswith(self.exe):
            return os.path.basename(self.exe)
        if self.argv:
            # A first argument that is empty or blank names nothing, and `ps`'s line then shows the next word, a word of the environment or blanks: the file it runs is its name.
            return os.path.basename(self.argv[0] if self.argv[0].strip() else self.exe)
        return os.path.basename(self.command.split(" ", 1)[0]) if self.command else ""

    @property
    def unread_name(self) -> str:
        """The name `ps` brackets, `(sysctl)`, for a process whose words it cannot read, one starting or ending; nothing otherwise."""
        return self.command[1:-1] if re.fullmatch(r"\([^()]*\)", self.command) else ""

    @property
    def arguments(self) -> str:
        return self.command[len(self.exe) :] if self.exe and self.command.startswith(self.exe) else self.command.partition(" ")[2]

    @property
    def words_read(self) -> list[str]:
        """Its arguments, the path it runs whole: `ps` gives one line, so a space in a path reads as two words, and the first could be any name, an agent's among them. The file standing there says where the path ends."""
        words = self.arguments.split()
        for at, word in enumerate(words[:6]):
            if word.startswith("-"):
                continue
            if os.path.isfile(os.path.join(self.cwd, word)):
                return words
            for end in range(at + 2, min(at + 9, len(words) + 1)):
                whole = " ".join(words[at:end])
                if "/" in whole and os.path.isfile(os.path.join(self.cwd, whole)):
                    return [*words[:at], whole, *words[end:]]
        return words

    @property
    def words(self) -> list[str]:
        """Its arguments, word by word: the kernel's own where it gives them, so that a flag's value holding a space is one word, a terminal's `--init-file` under `Visual Studio
        Code.app` among them; and `ps`'s one line as `words_read` reads it where the kernel gives none."""
        return list(self.argv[1:]) if self.argv else self.words_read

    @property
    def goes_by(self) -> str:
        """The word it goes by: the kernel's first where it gives its words, empty where that is empty and never the next word `ps` shows; and of `ps`'s one line where the kernel gives
        none, the path it runs whole where the line begins with it, its first word otherwise."""
        if self.argv:
            return self.argv[0]
        if self.exe and self.command.startswith(self.exe):
            return self.exe
        return self.command.split(None, 1)[0] if self.command.strip() else ""

    @property
    def told(self) -> list[str]:
        """Its words with the one it goes by first: the kernel's own where it gives them, and `ps`'s one line as `words_read` reads it where the kernel gives none."""
        if self.argv:
            return list(self.argv)
        return [self.goes_by, *self.words_read] if self.command.strip() else []

    @property
    def given(self) -> str:
        """The word an interpreter is given to run, as it reads its own flags: the first that is no flag and no value of one, a whole flag's by `FLAGS_WITH_VALUES` and a cluster's letter by
        letter by `CLUSTERS`, a module's name where Python is given one. Nothing where a flag gives it a program's text, where it reads its input, or where it is no interpreter. A shell's is
        the script `shell_told` reads."""
        kind = "python" if self.name.lower().startswith("python") else self.name
        if kind != "python" and kind not in INTERPRETERS:
            return ""
        words = self.words
        if kind in SHELLS:
            return shell_told(words)[1] or ""
        return interpreter_reads(kind, words)[0]

    @property
    def script(self) -> str:
        """What an interpreter runs, by its file's name; nothing otherwise."""
        return os.path.basename(self.given)

    @property
    def spawns(self) -> bool:
        """Whether it is a build, which spawns more for as long as it lives, however little it holds. What only asks, or cleans, is none, nor a `make` waiting its turn."""
        if self.name not in BUILDS or self.waiting:
            return False
        arguments = self.words
        return not make_only_asks(arguments) if self.name in MAKES else is_heavy([self.name, *arguments])

    @property
    def label(self) -> str:
        return f"{self.name} {self.script}" if self.script else self.name


class Darwin:
    """What `ps` cannot say: a process's working directory and executable, its footprint, which counts what is compressed and swapped, and what it has written. RSS
    does not count those: on `2026-11-05` the host's swap held 62 GiB while the largest process showed 0.9."""

    def __init__(self) -> None:
        self.lib = ctypes.CDLL("libproc.dylib", use_errno=True)
        self.libc = ctypes.CDLL(None, use_errno=True)
        # One buffer for every reading of a process's arguments, the most the kernel holds of one.
        self.most = os.sysconf("SC_ARG_MAX")
        self.held = ctypes.create_string_buffer(self.most)

    def usage(self, pid: int) -> tuple[int, int] | None:
        """Its footprint and what it has written, from `rusage_info_v2`."""
        buffer = ctypes.create_string_buffer(160)
        if self.lib.proc_pid_rusage(pid, 2, buffer) != 0:
            return None
        return int.from_bytes(buffer.raw[72:80], sys.byteorder), int.from_bytes(buffer.raw[152:160], sys.byteorder)

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

    def arguments(self, pid: int) -> list[str]:
        """Its arguments as the kernel holds them, `KERN_PROCARGS2`, which outlast its executable's file: a count, that file's path, then each argument. None of a process that is gone, of a zombie,
        of another user's, or of a number that names no one process."""
        names, size = (ctypes.c_int * 3)(1, 49, pid), ctypes.c_size_t(self.most)
        if pid < 1 or self.libc.sysctl(names, 3, self.held, ctypes.byref(size), None, 0) != 0 or size.value < 5:
            return []
        raw = ctypes.string_at(self.held, size.value)
        ends = raw.find(b"\0", 4)
        if ends < 0:
            return []
        # The path with its end is padded to a multiple of eight bytes, as measured on this host over twenty lengths of path: so an argument that is empty, the first among them, is
        # told from the padding, and nothing after the last is taken for one.
        starts = ends + 1 + -(ends + 1 - 4) % 8
        words = raw[starts:].split(b"\0")
        return [word.decode("utf-8", "replace") for word in words[:int.from_bytes(raw[:4], sys.byteorder)]]


NATIVE = Darwin() if sys.platform == "darwin" else None


def processes() -> list[Process]:
    """This user's processes, nobody else's. A zombie holds nothing and cannot be stopped again, so it is not among them."""
    me = os.getuid()
    # A command line is bytes, and any process may hold ones that are no text. One such process must not blind the watcher.
    listed = subprocess.run(["ps", "-axww", "-o", "pid=,ppid=,uid=,stat=,rss=,command="], capture_output=True, timeout=30, check=True)
    table = []
    # `ps` writes one process to a line, a line feed inside an argument as `\012`: so a row ends at a line feed and nowhere else. Read as text, a row would also end at U+2028, at U+2029
    # and at whatever else a text's lines end at, the rest of that argument would read as a row of its own, and one such process anywhere on the host would blind every guard.
    for row in listed.stdout.split(b"\n"):
        parts = row.decode("utf-8", "replace").split(None, 5)
        try:
            pid, ppid, uid, kilobytes = int(parts[0]), int(parts[1]), int(parts[2]), int(parts[4])
        except (IndexError, ValueError):
            continue  # No row of a process: an empty line, or one cut short.
        if uid != me or parts[3].startswith("Z"):
            continue
        used = NATIVE.usage(pid)
        held, written = (kilobytes * 1024, 0) if used is None else used
        # A command line that is all blanks is a process's all the same, `exec -a ' '` or a title set to blanks: its row has five fields, and it is counted and stopped as any other.
        table.append(Process(pid, ppid, parts[5] if len(parts) > 5 else "", NATIVE.cwd(pid), NATIVE.exe(pid), held, written, argv=tuple(NATIVE.arguments(pid))))
    # A table that does not hold the one reading it is not the table: `ps` answering nothing, or cut short, must read as blind and never as calm.
    if not any(process.pid == os.getpid() for process in table):
        raise ValueError("`ps` does not list the process reading it")
    return table


# Who may be working here, and vouches for nothing it starts. A name is searched for, not matched whole: `cursor-agent`, and an agent run by an interpreter goes by its
# script. Wrongly sparing a process costs less than wrongly stopping an agent, and the record says so.
AGENT = re.compile(r"claude|codex|kimi|gemini|aider|cursor|copilot|opencode|goose", re.I)
INFRASTRUCTURE = {"login", "sshd", "mosh-server", "tmux", "screen", "zellij", "abduco", "dtach", "iterm2", "terminal", "code", "alacritty", "kitty", "ghostty", "wezterm-gui", "caffeinate"}
APPLICATION = ("/Applications/", "/System/", "/usr/libexec/", "/usr/sbin/")
# The guard's self-test is spared as the guard is, by its own name: a line stopping it would end the run that says what it could not run, and leave its rigs behind. As a `run`'s job it is a
# job, which `guard_told` alone tells.
SELF_TEST_NAME = "selftest_host_guard.py"
GUARD_NAME = "host_guard.py"


def guard_told(process: Process, name: str = GUARD_NAME) -> list[str] | None:
    """What a guard was told, the words past its file's name, or None of a process that is none: a Python, by the word it goes by or by being this interpreter, given a file named as the
    guard is. Its words are read once each, as the interpreter reads its own flags, and they are the kernel's own where it gives them: so no command line costs more than its length to read,
    a path holding a space is one word, and a process whose first word is empty is no Python by its second. A pattern over `ps`'s line read `-W -a` two ways, as a flag with its value and as
    two flags, so its time doubled with each such pair, and a command line of 252 bytes held every guard for days (round 33)."""
    goes_by = process.goes_by
    if not ("python" in goes_by or "Python" in goes_by or goes_by == sys.executable):
        return None
    words = process.told
    given, after = interpreter_reads("python", words[1:])
    return words[1 + after :] if os.path.basename(given) == name else None


# Long flags of two dashes that read a value in the next word, past where `bash` takes any: another shell's, `zsh --emulate sh`.
SHELL_FLAGS_WITH_VALUES = {"--init-file", "--rcfile", "--init-command", "--emulate"}
# The long flags `bash` takes with one dash as with two, each run on both here, `-pretty-print` on the later alone and `-protected` and `-wordexp` on the system's: so `-posix` is no cluster
# holding an `s`, nor `-norc` one holding a `c`. It takes them before its first cluster and nowhere after, where `-posix` is the cluster it looks like.
BASH_LONG_FLAGS = {
    "debug", "debugger", "dump-po-strings", "dump-strings", "help", "init-file", "login", "noediting", "noprofile", "norc", "posix", "pretty-print", "protected", "rcfile", "restricted", "verbose",
    "version", "wordexp",
}
# The shells whose `-n` reads what it is given and runs none of it wherever it stands among the plain flags, each run on this host. Another's was not run here or is another flag, `nu -n`
# being told no config file, or does not hold wherever it stands: `csh` and `tcsh` give `-c` the next word, a `-n` among them, and run what follows. What one of those holds is read as run.
PARSES_ONLY = {"bash", "sh", "zsh", "dash", "ksh"}
# What spawns more for as long as it lives, however little it holds.
BUILDS = {"cargo", "rustc", "rustdoc", "clippy-driver", "make", "gmake"}
# What shows a heavy job is under way. Not `make`, which may only be waiting its turn inside its own `run`.
BUILDING = {"cargo", "rustc", "rustdoc", "clippy-driver"}
MAKES = {"make", "gmake"}
# The tools a guard reads the host with, which are nothing a job has started.
READS_THE_HOST = {"ps", "sysctl", "vm_stat"}


def shell_told(arguments: list[str]) -> tuple[str | None, str | None]:
    """The command a shell given these is told to run, with `-c`, and the script it is given; neither if it runs its input. As `bash` reads them, in its two passes. Its long flags come first, with one
    dash or two. Then its clusters, of either sign: a `c` or an `s` holds wherever it stands in one, and each `o` or `O` takes a word after its cluster, `-eou pipefail` one and `-oo a b` two. So `-c`
    holds wherever it stands, and the first word that is no option is what it was told. `-wordexp` expands each word it is given as a shell's text, the one after a `-c` text among them, so
    every word past its options is read as told."""

    def long_flag(word: str) -> str | None:
        name = word[2:] if word.startswith("--") else word[1:]
        return name if word.startswith("-") and name in BASH_LONG_FLAGS else None

    told_a_command, on_its_input, expands, at = False, False, False, 0
    while at < len(arguments) and long_flag(arguments[at]):
        expands = expands or long_flag(arguments[at]) == "wordexp"
        at += 2 if long_flag(arguments[at]) in ("init-file", "rcfile") else 1
    while at < len(arguments):
        word = arguments[at]
        if word in ("--", "-"):
            at += 1
            break
        if not word.startswith(("-", "+")):
            break
        if word.startswith("--"):
            # No `bash` takes a long flag here, and another shell may.
            at += 2 if word in SHELL_FLAGS_WITH_VALUES else 1
            continue
        told_a_command = told_a_command or "c" in word
        on_its_input = on_its_input or "s" in word
        at += 1 + sum(letter in "oO" for letter in word[1:])
    if expands:
        return "\n".join(arguments[at:]), None
    if at >= len(arguments):
        return ("", None) if told_a_command else (None, None)
    if told_a_command:
        return arguments[at], None
    return (None, None) if on_its_input else (None, arguments[at])


def shell_only_parses(name: str, arguments: list[str]) -> bool:
    """Whether the shell `name` given these is told to read what it is given and run none of it, and only as exactly that: `-n` among the plain flags that change nothing it runs, of a shell whose `-n`
    was run here (`PARSES_ONLY`). Any other flag may be what undoes it, `+n`, `+o noexec`, so it is no such telling, and what it holds is read as run."""
    told = False
    for word in arguments:
        if word == "--" or not word.startswith(("-", "+")):
            return told and name in PARSES_ONLY
        if not re.fullmatch(r"-[ceuxn]+", word):
            return False
        told = told or "n" in word
    return told and name in PARSES_ONLY


def is_agent(process: Process) -> bool:
    return any(AGENT.search(word) for word in (process.name, os.path.basename(process.exe), process.script))


def is_terminal_shell(process: Process) -> bool:
    """A shell someone or something types into: a login shell by its dash, or any shell told neither a command (`-c`, `-lc`) nor a script. A shell running a command or a script is a job."""
    name = process.name
    if name.startswith("-"):
        return name[1:] in SHELLS
    if name not in SHELLS:
        return False
    return shell_told(process.words) == (None, None)


def protected(process: Process) -> bool:
    if process.name.lower() in INFRASTRUCTURE or is_agent(process) or is_terminal_shell(process):
        return True
    # By the word it goes by, the kernel's own where it gives one, and by the file it runs: never by the next word `ps` shows of a process whose first is empty.
    return guard_told(process) is not None or guard_told(process, SELF_TEST_NAME) is not None or process.goes_by.startswith(APPLICATION) or process.exe.startswith(APPLICATION)


def names_root(command: str, root: str) -> bool:
    """Whether a guard's command line says `--root` is exactly this root, and not one whose name merely begins the same, a space among what may follow: a flag follows a root, or nothing, as `main` writes it. What follows its `--` is its job's words, and a job may name any root."""
    own = re.split(r"\s--(?:\s|$)", command, maxsplit=1)[0]
    return re.search(r"--root\s+" + re.escape(root) + r"(?:\s+-|\s*$)", own) is not None


def is_run_of(process: Process, root: str) -> bool:
    """Whether it is a guard's `run` of this root, by its own words: the kernel's where it gives them, a root being one word whatever it holds, and `ps`'s one line where it gives none. What
    follows its `--` is its job's words, and a job may name any root."""
    told = guard_told(process)
    if told is None or told[:1] != ["run"]:
        return False
    if not process.argv:
        return names_root(process.command, root)
    own = told[1 : told.index("--")] if "--" in told else told[1:]
    return any(flag == "--root" and named == root for flag, named in zip(own, own[1:]))


def line_of(table: list[Process], pid: int) -> set[int]:
    """`pid` and its parents, up the table."""
    by_pid = {process.pid: process for process in table}
    line: set[int] = set()
    while pid in by_pid and pid not in line:
        line.add(pid)
        pid = by_pid[pid].ppid
    return line


def children_of(table: list[Process]) -> dict[int, list[int]]:
    children: dict[int, list[int]] = {}
    for process in table:
        children.setdefault(process.ppid, []).append(process.pid)
    return children


def says_no_job(process: Process) -> bool:
    """Whether it is a tool the guard reads the host with, or a moment `ps` cannot read the words of: `(sysctl)`, `(env)` and `(bash)` as a recipe's shell starts, `(Python)` as the guard forks. What it becomes the next look tells, at most a look late, so a build by its name, `(cargo)`, alone stays what it says."""
    return process.name in READS_THE_HOST or (bool(process.unread_name) and process.unread_name not in BUILDS)


def only_waiting(pid: int, by_pid: dict[int, Process], children: dict[int, list[int]], root: str) -> bool:
    """Whether what `pid` has started is at most a `run` still waiting its turn: a shell, the guard, and what reads the host or its file. So a `make` still reading its Makefile holds no build."""
    todo = list(children.get(pid, []))
    while todo:
        process = by_pid.get(todo.pop())
        if process is None:
            continue
        if not (guard_told(process) is not None or process.name in SHELLS or says_no_job(process) or process.name == "tail"):
            return False
        # A `run` of this root that has started its job has a child that is none of these, which the walk then meets.
        todo.extend(children.get(process.pid, []))
    return True


def ours(table: list[Process], root: Path, me: int, spare_its_line: bool = True) -> list[Process]:
    """This repository's processes that may be stopped, largest first: under its root by working directory or executable, and what those started,
    wherever it runs. A protected process is never one of them and vouches for nothing: the walk does not pass through it, so a terminal started here
    gives no claim on a build it runs elsewhere. A guard's `run` of this root is the one exception, since it says itself that its job is ours. Whoever asks is spared with
    the line that started it, so that it does not count its own shell, unless it is the watcher, whose parents are jobs like any other: the `make` that happened to start it."""
    text = str(root)
    by_pid = {process.pid: process for process in table}
    children = children_of(table)
    under = lambda path: path == text or path.startswith(text + os.sep)
    spared = {process.pid for process in table if protected(process)}
    found = {p.pid for p in table if p.pid not in spared and (under(p.cwd) or under(p.exe))}
    todo = list(found) + [p.pid for p in table if is_run_of(p, text)]
    while todo:
        for child in children.get(todo.pop(), []):
            if child not in found and child not in spared:
                found.add(child)
                todo.append(child)
    line = line_of(table, me) if spare_its_line else {me}
    mine = [by_pid[pid] for pid in found if pid not in line]
    for process in mine:
        process.waiting = process.name in MAKES and only_waiting(process.pid, by_pid, children, text)
    return sorted(mine, key=lambda process: process.bytes, reverse=True)


@dataclass
class Job:
    """The topmost process of ours and all of ours it started: what a person would start again, and so what is judged and stopped. Stopping less leaves a script to start the next."""

    top: Process
    members: list[Process]

    @property
    def bytes(self) -> int:
        return sum(process.bytes for process in self.members)

    @property
    def building(self) -> bool:
        return any(process.spawns for process in self.members)

    @property
    def label(self) -> str:
        largest = max(self.members, key=lambda process: process.bytes)
        return self.top.label if largest is self.top else f"{self.top.label}, running {largest.label}"


def jobs_of(mine: list[Process]) -> list[Job]:
    """`mine` by the topmost of each's parents among them, largest first."""
    by_pid = {process.pid: process for process in mine}
    jobs: dict[int, Job] = {}
    for process in mine:
        top, seen = process, {process.pid}
        while top.ppid in by_pid and top.ppid not in seen:
            top = by_pid[top.ppid]
            seen.add(top.pid)
        jobs.setdefault(top.pid, Job(top, [])).members.append(process)
    return sorted(jobs.values(), key=lambda job: job.bytes, reverse=True)


def tree_of(pid: int, table: list[Process]) -> list[int]:
    """`pid` and what it started, children first: a parent's death does not reach them. A protected process under it is left, with all it started."""
    by_pid = {process.pid: process for process in table}
    children = children_of(table)
    order, todo = [], [pid]
    while todo:
        at = todo.pop()
        process = by_pid.get(at)
        if process is not None and at != pid and protected(process):
            continue
        order.append(at)
        todo.extend(children.get(at, []))
    order.reverse()
    return order


def stop(pids: list[int], seconds: float = 1.0) -> None:
    """Ask them all at once, then make sure of them all at once. Under memory pressure a process may not get to run its handler, and one target
    at a time would cost a second each while the host fills. A number below 2 is no one process, a group or every process of this user, and is never signalled."""
    pids = [pid for pid in pids if pid > 1]
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


def record(root: Path, **event) -> bool:
    """Write down what happened, and whether it was written. Never raises: a watcher must not die of its own bookkeeping, `target/` removed by `cargo clean` or a full disk among the causes."""
    event["at"] = time.strftime("%Y-%m-%dT%H:%M:%S")
    try:
        path = state_dir(root, for_a_stop=event.get("kind") == "stopped") / "events.log"
        if size_of(path) > LOG_AT_MOST:
            os.replace(path, path.with_suffix(".log.1"))
        with open(path, "a", encoding="utf-8") as log:
            log.write(json.dumps(event) + "\n")
        return True
    except Cleared:
        pass
    except Exception as error:
        say(f"host-guard: cannot record {event.get('kind')}: {error}")
    return False


def plain(value: object, most: int = 300) -> str | int | float | None:
    """A value of the log as the guard writes one: a text, cut short, or a number a machine's word holds, so none of thousands of digits, an infinity or what is no number. Anything else is nothing."""
    if isinstance(value, str):
        return value[:most]
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        return None
    return value if -(2**63) < value < 2**63 else None


def event_of(line: bytes) -> dict | None:
    """A line of the log as the guard writes one, and no more of it than that: each field of its own kind or left out, a list's rows likewise, and nothing nested deeper, which is looked at and never said.
    So what says a line cannot fail on one written by hand: three rounds found that one reader at a time, a wrong type, a surrogate, a nesting too deep to say."""
    try:
        event = json.loads(line)
    except (ValueError, RecursionError):
        return None
    if not isinstance(event, dict) or not isinstance(event.get("kind"), str):
        return None
    reasons = event.get("reasons")
    read: dict = {
        "kind": event["kind"][:40],
        "at": plain(event.get("at"), 40),
        "error": plain(event.get("error")),
        "reasons": [each[:300] for each in reasons[:16] if isinstance(each, str)] if isinstance(reasons, list) else plain(reasons),
    }
    for name, fields in (("stopped", ("pid", "name", "gib", "command")), ("largest", ("pid", "name", "gib"))):
        rows = event.get(name)
        if isinstance(rows, list):
            read[name] = [{field: plain(row.get(field)) for field in fields} for row in rows[:64] if isinstance(row, dict)]
    return read


def events_of(text: bytes) -> list[dict]:
    """The lines of a part of the log that read, in their order."""
    return [event for event in map(event_of, text.splitlines()) if event is not None]


def names_a_stop(event: dict) -> bool:
    """Whether a line says what the watcher stopped: of that kind, and naming at least one job."""
    return event["kind"] == "stopped" and bool(event.get("stopped"))


def size_of(path: Path) -> int:
    try:
        return path.stat().st_size
    except OSError:
        return 0


@dataclass
class Motion:
    """What moved since the last sample: what each process wrote and which processes are new; and whether a volume fell, over `FELL_WINDOW_SECONDS`. At a watcher's first sample nothing has:
    there is no last."""

    written: dict[int, int] = field(default_factory=dict)
    new: set[int] = field(default_factory=set)
    repo_fell: bool = False
    boot_fell: bool = False


def disk_line(reading: Reading, jobs: list[Job], limits: Limits, baseline: Reading | None, motion: Motion) -> tuple[list[Job], list[str]]:
    """Under a disk's floor, or losing too much in the window: what is writing, and every build. A script of writers each shorter than a sample shows no writer, only a volume
    falling, the very one that is short, and a job that has started something new since the last sample."""
    reasons = disk_reasons(reading, limits)
    lost = bool(baseline) and baseline.repo_free_gib - reading.repo_free_gib > limits.stop_disk_growth_gib
    if lost:
        reasons.append(f"the repository's volume lost {baseline.repo_free_gib - reading.repo_free_gib:.1f} GiB in the last minutes, whoever wrote it, over {limits.stop_disk_growth_gib:.1f}")
    if not reasons:
        return [], []
    targets = [job for job in jobs if job.building or sum(motion.written.get(p.pid, 0) for p in job.members) >= WRITING_BYTES]
    falling = motion.repo_fell and (lost or reading.repo_free_gib < limits.stop_repo_free_gib) or motion.boot_fell and reading.boot_free_gib < limits.stop_boot_free_gib
    if not targets and falling:
        targets = [job for job in jobs if job.top.pid not in motion.new and any(p.pid in motion.new for p in job.members)]
    return targets, reasons


def danger_line(reading: Reading, jobs: list[Job], limits: Limits) -> tuple[list[Job], list[str]]:
    """At the director's danger line, or critical pressure: every job holding memory, and every build."""
    reasons = []
    if reading.used_percent >= limits.panic_used_percent:
        reasons.append(f"RAM use is {reading.used_percent:.0f}%, at the {limits.panic_used_percent:.0f}% danger line")
    if reading.pressure_level >= limits.panic_pressure_level:
        reasons.append(f"macOS reports critical memory pressure, level {reading.pressure_level}")
    if not reasons:
        return [], []
    return [job for job in jobs if job.bytes >= limits.worth_stopping_gib * GIB or job.building], reasons


def ceiling_line(jobs: list[Job], limits: Limits) -> tuple[list[Job], list[str]]:
    """What this repository's own processes may hold, whatever the host's state. Many small jobs together are held here, one at a time."""
    targets: list[Job] = []
    reasons: list[str] = []
    count = sum(len(job.members) for job in jobs)
    if count > limits.processes:
        over = count - int(limits.processes)
        for job in sorted(jobs, key=lambda job: len(job.members), reverse=True):
            if over <= 0:
                break
            targets.append(job)
            over -= len(job.members)
        reasons.append(f"{count} processes of ours, over {limits.processes:.0f}")
    for job in jobs:
        largest = max(job.members, key=lambda process: process.bytes)
        if largest.bytes > limits.one_process_gib * GIB:
            targets.append(job)
            reasons.append(f"one process of ours holds {largest.bytes / GIB:.2f} GiB, over {limits.one_process_gib:.2f}")
            break
    total = sum(job.bytes for job in jobs)
    if jobs and total > limits.all_processes_gib * GIB:
        targets.append(jobs[0])
        reasons.append(f"our processes hold {total / GIB:.2f} GiB, over {limits.all_processes_gib:.2f}")
    return targets, reasons


def stop_line(reading: Reading, jobs: list[Job], limits: Limits, baseline: Reading | None) -> tuple[list[Job], list[str]]:
    """Short of the danger line: our largest job, if it holds enough to help."""
    reasons = []
    if limits.stop_used_percent <= reading.used_percent < limits.panic_used_percent:
        reasons.append(f"RAM use is {reading.used_percent:.0f}%, at the {limits.stop_used_percent:.0f}% stop line")
    if limits.stop_pressure_level <= reading.pressure_level < limits.panic_pressure_level:
        reasons.append(f"macOS reports memory pressure at level {reading.pressure_level}, where 1 is normal")
    if baseline and reading.swap_used_gib - baseline.swap_used_gib > limits.stop_swap_growth_gib:
        reasons.append(f"swap grew {reading.swap_used_gib - baseline.swap_used_gib:.1f} GiB in the last minutes, over {limits.stop_swap_growth_gib:.1f}")
    if not reasons:
        return [], []
    return ([jobs[0]] if jobs and jobs[0].bytes >= limits.worth_stopping_gib * GIB else []), reasons


def judged(reading: Reading, mine: list[Process], limits: Limits, baseline: Reading | None = None, motion: Motion | None = None) -> tuple[list[Job], list[str], bool]:
    """What to stop now, why, and whether a line was reached with nothing of ours to stop for it. Each line judges alone and they add up, so that none can silence another: a disk's
    reason once switched the RAM line off."""
    jobs = jobs_of(mine)
    verdicts = (disk_line(reading, jobs, limits, baseline, motion or Motion()), danger_line(reading, jobs, limits), ceiling_line(jobs, limits), stop_line(reading, jobs, limits, baseline))
    targets = {job.top.pid: job for found, _ in verdicts for job in found}
    reasons = [reason for _, said in verdicts for reason in said]
    return list(targets.values()), reasons, bool(reasons) and not targets


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
    except Cleared:
        pass
    except Exception as error:
        say(f"host-guard: cannot note the peak: {error}")


def baseline_of(history: deque, now: float, reading: Reading, ours_runs: bool) -> Reading | None:
    """What growth is counted from: the oldest reading of the window while ours runs. With nothing of ours running there is no count."""
    if not ours_runs:
        history.clear()
        return None
    history.append((now, reading))
    while now - history[0][0] > GROWTH_WINDOW_SECONDS:
        history.popleft()
    return history[0][1]


@dataclass
class Memory:
    """What a watcher keeps from one sample to the next. `written` is None until its first."""

    history: deque = field(default_factory=deque)
    said_at: float = 0.0
    written: dict[int, int] | None = None
    free: deque = field(default_factory=deque)

    def moved(self, reading: Reading, mine: list[Process], now: float | None = None) -> Motion:
        now = time.monotonic() if now is None else now
        motion = Motion()
        if self.written is not None:
            motion.written = {p.pid: p.written - self.written.get(p.pid, 0) for p in mine}
            motion.new = {p.pid for p in mine} - self.written.keys()
        self.free.append((now, reading.repo_free_gib, reading.boot_free_gib))
        while now - self.free[0][0] > FELL_WINDOW_SECONDS:
            self.free.popleft()
        _, repo, boot = self.free[0]
        motion.repo_fell = repo - reading.repo_free_gib >= FELL_BYTES / GIB
        motion.boot_fell = boot - reading.boot_free_gib >= FELL_BYTES / GIB
        self.written = {p.pid: p.written for p in mine}
        return motion


def sample(root: Path, limits: Limits, memory: Memory) -> tuple[Reading, list[Process]]:
    reading = read_host(root)
    table = processes()
    mine = ours(table, root, os.getpid(), spare_its_line=False)
    baseline = baseline_of(memory.history, time.monotonic(), reading, bool(mine))
    targets, reasons, not_ours = judged(reading, mine, limits, baseline, memory.moved(reading, mine))
    if targets:
        # Written first, which cannot fail or wait long, so that a job's own `run` can say why it ended. Then the stop, and nothing below may keep it from happening.
        stopped = [{"pid": job.top.pid, "parent": job.top.ppid, "name": job.label, "gib": round(job.bytes / GIB, 2), "command": job.top.command[:160]} for job in targets]
        record(root, kind="stopped", reasons=reasons, stopped=stopped, reading=asdict(reading), largest=largest_on_host(table))
        pids: list[int] = []
        for job in targets:
            pids.extend(pid for pid in tree_of(job.top.pid, table) if pid not in pids)
        stop(pids)
        # The growth that caused this has been answered: what comes next is not stopped for it again.
        memory.history.clear()
    elif not_ours and time.monotonic() - memory.said_at > 60:
        memory.said_at = time.monotonic()
        record(root, kind="pressure-not-ours", reasons=reasons, reading=asdict(reading), largest=largest_on_host(table))
    note_peak(root, reading, sum(process.bytes for process in mine))
    return reading, mine


def hold_lock(root: Path) -> int | None:
    """One watcher per root, by a lock on the root itself: unlike a file under `target/`, `cargo clean` cannot take it away and leave two."""
    fd = os.open(str(root), os.O_RDONLY)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except OSError:
        os.close(fd)
        return None
    return fd


def file_is_whole() -> bool:
    """Whether this file as it stands compiles and ends as it ends: what a save can be seen to have broken with nothing run. An edit half done may pass, and its failing is what the record states."""
    try:
        source = CODE.read_bytes()
        compile(source, str(CODE), "exec")
    except (OSError, SyntaxError, ValueError):
        return False
    return source.endswith(b"\n    sys.exit(main(sys.argv[1:]))\n")


def write_notes(root: Path, notes: dict[str, str]) -> None:
    """Each in turn, by one look at the directory: in a watcher all of them or none, so a name written last is never found without the rest."""
    try:
        state = state_dir(root)
        for name, text in notes.items():
            (state / name).write_text(text)
    except Cleared:
        pass
    except Exception as error:
        say(f"host-guard: cannot write {' or '.join(notes)}: {error}")


def write_note(root: Path, name: str, text: str) -> None:
    write_notes(root, {name: text})


def names_it(root: Path, image: str) -> bool:
    """Whether the notes of `root` name this watcher and what it runs. Notes gone, half gone or naming another are not its own, and it writes them again."""
    state = state_path(root)
    try:
        return (state / "watch.pid").read_text() == str(os.getpid()) and (state / "watch.code").read_text() == image
    except (OSError, ValueError):
        return False


def code_stamp() -> tuple[int, int] | None:
    try:
        stats = CODE.stat()
    except OSError:
        return None
    return stats.st_mtime_ns, stats.st_size


STARTED_FROM = "ARCHOGEN_HOST_GUARD_STARTED_FROM"
# Set for the image a watcher takes itself up as, to what it must still say, a list that may be empty: nothing asked for that image, so it makes no directory, and what could not be said it says
# where one stands.
TAKEN_UP = "ARCHOGEN_HOST_GUARD_TAKEN_UP"


def started_from() -> tuple[int, int] | None:
    """The file as it stood before this image read it, where what started it says: a save while it comes up is one it has not taken up, and is then seen. Taking it for another costs one more taking up."""
    told = os.environ.pop(STARTED_FROM, "")
    try:
        when, size = told.split(":")
        return int(when), int(size)
    except ValueError:
        return code_stamp()


def stamp_for_the_next(stamp: tuple[int, int] | None) -> str:
    return f"{stamp[0]}:{stamp[1]}" if stamp else ""


def handed_on(told: str) -> list[dict]:
    """What the image before this one must say and could not, as it handed it on: the two kinds a watcher holds unsaid and nothing else, each with its one text, the last four of them."""
    try:
        events = json.loads(told)
    except ValueError:
        return []
    said_by = {"taken-up": "reasons", "unread": "error"}
    kept = []
    for event in events if isinstance(events, list) else []:
        kind = event.get("kind") if isinstance(event, dict) else None
        key = said_by.get(kind) if isinstance(kind, str) else None
        if key and isinstance(event.get(key), str):
            kept.append({"kind": event["kind"], key: event[key]})
    return kept[-4:]


def watch(root: Path, limits: Limits, once: bool, idle_seconds: float = 600.0, again: list[str] | None = None) -> int:
    lock = hold_lock(root)
    if lock is None:
        return 0  # One is watching.
    # What started it asked for it, so this once it makes its directory: but nothing asked for one that took itself up.
    taken_up = os.environ.pop(TAKEN_UP, "")
    global WATCHING
    WATCHING = bool(taken_up)
    write_note(root, "watch.pid", str(os.getpid()))
    WATCHING = True
    try:
        (state_path(root) / "peak.json").unlink(missing_ok=True)
    except OSError:
        pass
    held_inode = os.stat(root).st_ino
    os.chdir("/")
    memory = Memory()
    unread_said, last_seen, busy, read_at, running = 0.0, time.monotonic(), False, time.monotonic(), started_from()
    # What it must say and could not, no directory standing: said once one does, in its order, the last few of them.
    unsaid: list[dict] = handed_on(taken_up)
    # What it runs, for `status`: a watcher running an older file looks like any other.
    image = stamp_for_the_next(running)
    write_note(root, "watch.code", image)
    while True:
        # A root that is gone, a removed worktree, is watched by nobody: nothing of it can run, and its state must not bring it back. One remade at that name is another, which holds its own lock
        # and its own notes, so this is looked at before a round reads or writes anything there.
        try:
            if os.stat(root).st_ino != held_inode:
                return 0
        except OSError:
            return 0
        wait, notes = 2.0, {}
        try:
            reading, mine = sample(root, limits, memory)
            busy, read_at = bool(mine), time.monotonic()
            # It reads more often the closer the host is to a line, and as seldom as is safe when nothing of ours runs: this costs the shared host too.
            near = reading.used_percent >= limits.start_used_percent + 10 or disk_reasons(reading, limits)
            wait = 0.5 if near else 1.0 if busy or reading.used_percent >= limits.start_used_percent else 2.0
            notes["sampled"] = str(time.time())
        except Exception as error:  # Whatever it is, the watcher goes on trying, and `sampled` going stale says it is blind. What it last saw may still run, so it stays for that, but no longer than it would stay idle.
            busy = busy and time.monotonic() - read_at <= idle_seconds
            if time.monotonic() - unread_said > 60:
                unread_said = time.monotonic()
                record(root, kind="unread", error=str(error))
        # What names it outlasts what clears them, `cargo clean` among it, and what writes over them, whatever its round did. Its pid comes last, with what says it has read, so that whoever finds it named finds the rest.
        if not names_it(root, image):
            notes.update({"watch.code": image, "watch.pid": str(os.getpid())})
        write_notes(root, notes)
        while unsaid and record(root, **unsaid[0]):
            unsaid.pop(0)
        # A guard changed under a watcher holds from then, whatever this round did, a line that raises among it. The lock closes with the old image and the new one takes it, so nothing goes unwatched between.
        # One that can be seen to be broken is not taken up, and said once.
        stamp = code_stamp()
        if stamp not in (None, running):
            running = stamp
            if again is not None:
                refused = f"{CODE.name} as saved does not compile or is cut short: this watcher goes on as it was"
                if file_is_whole():
                    # The image it becomes says that it took the file up, with what this one held unsaid before it: nothing is said of a taking up that did not happen.
                    os.environ[STARTED_FROM] = stamp_for_the_next(stamp)
                    os.environ[TAKEN_UP] = json.dumps([*unsaid, {"kind": "taken-up", "reasons": f"this watcher took up {CODE.name} as saved"}][-4:])
                    try:
                        os.execv(sys.executable, [sys.executable, "-I", "-B", str(CODE), *again])
                    except OSError as error:
                        # What would run it is gone, an interpreter an upgrade removed: this watcher goes on as it was, and says that.
                        os.environ.pop(STARTED_FROM, None)
                        os.environ.pop(TAKEN_UP, None)
                        refused = f"{CODE.name} as saved cannot be taken up, {sys.executable} being no program to run ({error.strerror}): this watcher goes on as it was"
                if not record(root, kind="unread", error=refused):
                    unsaid.append({"kind": "unread", "error": refused + ", where no directory stood to say so"})
                    del unsaid[:-4]
        try:
            fresh = time.time() - (state_path(root) / "heartbeat").stat().st_mtime < idle_seconds
        except OSError:
            fresh = False
        if busy or fresh:
            last_seen = time.monotonic()
        elif time.monotonic() - last_seen > idle_seconds:
            return 0
        if once:
            return 0
        time.sleep(wait)


def runs_the_file(root: Path) -> bool:
    """Whether the watcher started from the file as it stands. One from before this note was written reads as not."""
    try:
        return (state_path(root) / "watch.code").read_text() == stamp_for_the_next(code_stamp())
    except (OSError, ValueError):
        return False


def watches(arguments: list[str], root: Path | None = None) -> bool:
    """Whether a process given these is a guard's watcher, of `root` where one is named: an interpreter given a file named as the guard is, first, and told to watch, the root it names by
    its whole path being that one. By its arguments, as the kernel holds them, and not by its file, which an upgrade can remove under it, nor by its being an interpreter, which any job is.
    A watcher writes its root whole on its own command line before it does anything else (`main`), so one that names none, or one relatively, is the watcher of no root this can tell: a
    stale note naming such a process names no one."""
    given, after = interpreter_reads("python", arguments[1:])
    if not (os.path.basename(given) == GUARD_NAME and arguments[1 + after : 2 + after] == ["watch"]):
        return False
    if root is None:
        return True
    told = arguments[2 + after :]
    named = told[told.index("--root") + 1] if "--root" in told[:-1] else ""
    return os.path.isabs(named) and canonical(Path(named)) == canonical(root)


def watcher_pid(root: Path) -> int | None:
    """The watcher of `root`, if one runs: the lock on the root is held, and the pid its file names is one process, never a group or every process, whose arguments, as the kernel holds them, are a
    guard's watching of this root. So a note that is stale names no one while an asker holds the lock for its instant, another job of an interpreter among those it could name, and a watcher
    whose interpreter an upgrade removed is still one. No `ps`, so that a slow one cannot make a live watcher look gone, and no signal, which a number that is no process's would carry elsewhere."""
    try:
        pid = int((state_path(root) / "watch.pid").read_text())
    except (OSError, ValueError):
        return None
    # A number below 2 names no one process: nothing is this one's group, a negative one a group or every process of this user, and 1 the host's first. None is a watcher, whatever a note says.
    if pid < 2:
        return None
    lock = hold_lock(root)
    if lock is not None:
        os.close(lock)
        return None
    return pid if watches(NATIVE.arguments(pid), root) else None


def watcher_asked_for(root: Path, seconds: float = 6.0) -> int | None:
    """The watcher of `root`, for what asks after it by its pid, `status` and `stop`. The asking makes the directory `cargo clean` took, and a watcher that holds the root's lock is given three of its slowest rounds to name itself there."""
    try:
        state_dir(root)
    except OSError:
        return None
    deadline = time.monotonic() + seconds
    while True:
        pid = watcher_pid(root)
        if pid or time.monotonic() >= deadline:
            return pid
        try:
            lock = hold_lock(root)
        except OSError:
            return None
        if lock is not None:
            os.close(lock)
            return None
        time.sleep(0.1)


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
    state_dir(root)
    # It outlives what started it, so a narrow `PATH` of that one's must not leave it blind to everyone. What is first on the path stays first.
    env = dict(os.environ, PATH=os.pathsep.join([os.environ.get("PATH", ""), "/usr/bin", "/bin", "/usr/sbin", "/sbin"]))
    env[STARTED_FROM] = stamp_for_the_next(code_stamp())
    # What a watcher hands the image it becomes is no asker's to hand a new one.
    env.pop(TAKEN_UP, None)
    subprocess.Popen(
        [sys.executable, "-I", "-B", str(Path(__file__).resolve()), "watch", "--root", str(root)],
        stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True, cwd="/", env=env,
    )


def unseen(root: Path, session: str = "") -> list[dict]:
    """What has been stopped since this session last read it, marking it read only once read. A bad line is skipped and never costs the rest. Each session has its own
    place, so that one is not refused for another's stop and the other told nothing. A named session with no place, which may be new or may have lost it to `cargo clean`,
    is told only of what was stopped lately."""
    state = state_path(root)
    name = re.sub(r"\W", "", session)[:64]
    log, seen = state / "events.log", state / (f"seen.{name}" if name else "seen")
    end = size_of(log)
    lately_only = False
    try:
        start = max(0, int(seen.read_text()))
    except (OSError, ValueError):
        start, lately_only = 0, bool(name)
    if end < start:  # A log replaced or rotated: all of it is new.
        start = 0
    if end == start:
        write_note(root, seen.name, str(end))
        return []
    start = max(start, end - LOG_AT_MOST)
    with open(log, "rb") as lines:
        lines.seek(start)
        text = lines.read()
    since = time.time() - TOLD_OF_LATELY_SECONDS
    events = [event for event in events_of(text) if names_a_stop(event) and (not lately_only or when(event) >= since)]
    write_note(root, seen.name, str(start + len(text)))
    return events


def when(event: dict) -> float:
    """When it happened, or never: a line whose time is no time is not recent, and a hand-written one would otherwise refuse or tell every new session for good."""
    try:
        return time.mktime(time.strptime(str(event.get("at")), "%Y-%m-%dT%H:%M:%S"))
    except (ValueError, OverflowError):
        return 0.0


def said_of(event: dict) -> str:
    """What a stop says, of a line `event_of` read: so of texts and numbers alone."""
    stopped, reasons = event.get("stopped", []), event.get("reasons")
    names = ", ".join(f"{s['name']} (pid {s['pid']}, {s['gib']} GiB)" for s in stopped[:4])
    more = len(stopped) - 4
    holders = ", ".join(f"{h['name']} {h['gib']} GiB" for h in event.get("largest", [])[:3])
    return (
        f"{event['at']}: stopped {names}{f' and {more} more' if more > 0 else ''}: {'; '.join(reasons) if isinstance(reasons, list) else ''}"
        + (f". Holding the most on the host: {holders}" if holders else "")
    )


# ---- what a shell command would run

ASSIGNMENT = re.compile(r"[A-Za-z_]\w*(?:\[[^\]]*\])?\+?=")
REDIRECTION = re.compile(r"\d*(?:<<<|<<-|<<|<>|<&|>>|>&|>\||<|>)|&>>|&>")
ARRAY_ASSIGNMENT = re.compile(r"[A-Za-z_]\w*\+?=")
A_FUNCTION = re.compile(r"\(\s*\)")
A_DESCRIPTOR_VARIABLE = re.compile(r"\{[A-Za-z_]\w*\}")
A_SUBSTITUTION_ALONE = re.compile(r"-?(?:\$\(…\)|`…`|\s)+")
A_PLAIN_TARGET = re.compile(r"-?(?:\d+|\$[!$])|%\S*")
BEFORE_A_COMMAND = {"if", "then", "elif", "else", "while", "until", "do", "!", "{", "coproc"}
NOT_A_COMMAND = {"for", "select", "case", "esac", "fi", "done", "}", "[[", "]]"}


def a_loop_header(words: list[str]) -> bool:
    """Whether `words`, the keywords before them gone, are a `for` or `select` whose body may begin at the next word. After `in` they are its list, which only a `;` or a line ends, and may hold `do`."""
    at = 0
    while at < len(words) and (words[at] in BEFORE_A_COMMAND or words[at] in ("time", "-p")):
        at += 1
    return words[at : at + 1] in (["for"], ["select"]) and words[at + 2 : at + 3] != ["in"]

# What runs another command: the flags of its that take a value, and how many plain words stand before the command.
WRAPPERS = {
    "env": ({"-u", "-P", "-S", "-C", "--unset", "--chdir", "--split-string"}, 0),
    "arch": ({"-e", "-d", "-arch"}, 0),
    "xcrun": ({"--sdk", "-sdk", "--toolchain", "-toolchain"}, 0),
    "taskpolicy": ({"-c", "-d", "-g", "-t", "-l", "-p", "-m", "-j", "-P", "-S"}, 0),
    "sandbox-exec": ({"-f", "-n", "-p", "-D"}, 0),
    "script": ({"-t"}, 1),
    "nohup": (set(), 0),
    "nice": ({"-n", "--adjustment"}, 0),
    "time": ({"-o", "--output"}, 0),
    "timeout": ({"-k", "-s", "--kill-after", "--signal"}, 1),
    "gtimeout": ({"-k", "-s", "--kill-after", "--signal"}, 1),
    "caffeinate": ({"-t", "-w"}, 0),
    "sudo": ({"-u", "-g", "-h", "-p", "-C", "-D", "-R", "-T", "-U"}, 0),
    "command": (set(), 0),
    "builtin": (set(), 0),
    "exec": ({"-a"}, 0),
    "xargs": ({"-I", "-J", "-L", "-n", "-P", "-R", "-S", "-s", "-E", "-d", "-a"}, 0),
    "stdbuf": ({"-i", "-o", "-e"}, 0),
}


@dataclass
class Command:
    """One simple command a shell would run: its words as the command would get them, quotes gone and a substitution as written, and which pipeline it stands in. `fed` says that
    its arguments come from its input, as `xargs` gives them. `own_from` says where among all that is read the substitutions in its own words begin, which it stands just after."""

    words: list[str]
    pipeline: int
    fed: bool = False
    own_from: int = 0

    @property
    def name(self) -> str:
        return os.path.basename(self.words[0])


def runs_its_input(words: list[str]) -> bool:
    """Whether these run a shell told neither a command nor a script, so that what it is fed is what it runs."""
    words = unwrapped(words)[0]
    if not words:
        return False
    name, arguments = os.path.basename(words[0]), words[1:]
    if name in (".", "source"):
        return arguments[:1] == ["/dev/stdin"]
    if name not in SHELLS or shell_only_parses(name, arguments):
        return False
    command, script = shell_told(arguments)
    return command is None and script in (None, "/dev/stdin")


def plain_word(words: list[str], takes_a_value: set[str] = frozenset()) -> str | None:
    """The first word that is no flag or a flag's value."""
    skip = False
    for word in words:
        if skip:
            skip = False
        elif word in takes_a_value:
            skip = True
        elif not word.startswith(("-", "+")):
            return word
    return None


def unwrapped(words: list[str]) -> tuple[list[str], bool]:
    """What `words` run once the assignments, keywords and wrappers before it are gone, and whether a wrapper feeds it arguments. `command -v` runs nothing."""
    fed = False
    while words:
        name = os.path.basename(words[0])
        if words[0] == "coproc":
            words = words[2:] if words[2:3] == ["{"] else words[1:]
        elif words[0] in BEFORE_A_COMMAND or ASSIGNMENT.match(words[0]):
            words = words[1:]
        elif words[0] == "function":
            words = words[2:]
        elif name == "rustup" and words[1:2] == ["run"]:
            words = words[3:]
        elif name in WRAPPERS:
            takes_a_value, plain = WRAPPERS[name]
            rest, at = words[1:], 0
            while at < len(rest):
                word = rest[at]
                split = re.fullmatch(r"-[a-zA-Z0]*S(.*)", word, re.S) if name == "env" else None
                if split and split.group(1):
                    rest = rest[:at] + split.group(1).split() + rest[at + 1 :]  # What it is given is what it runs.
                elif name == "env" and (split or word == "--split-string") and at + 1 < len(rest):
                    rest = rest[:at] + rest[at + 1].split() + rest[at + 2 :]
                elif word in takes_a_value:
                    at += 2
                elif name == "env" and word == "-":
                    at += 1
                elif word.startswith("-") and word != "-":
                    if name == "command" and re.fullmatch(r"-\w*[vV]\w*", word):
                        return [], False
                    at += 1
                elif name == "env" and ASSIGNMENT.match(word):
                    at += 1
                elif plain:
                    plain -= 1
                    at += 1
                else:
                    break
            fed = fed or name == "xargs"
            words = rest[at:]
        else:
            break
    return words, fed


class ShellReader:
    """Enough of the shell's own grammar to say what it would run, instead of a pattern over its text: three rounds of review found that a pattern reads a word in a message as a
    command, misses one in `$(…)`, and pairs a comment's apostrophe with a quote. `selftest_host_guard.py` holds this to the shell itself, running each form against stubs."""

    def __init__(self, text: str, found: list[Command], pipelines: itertools.count) -> None:
        self.text, self.at, self.found, self.pipelines = text, 0, found, pipelines
        self.here_docs: list[tuple[str, bool, bool, list[str]]] = []  # Each body starts at the next newline, at whatever level the reading then stands.
        self.substitutions = 0  # How many `$(…)` the reading stands in.

    def commands(self, closing: str | None = None) -> bool:
        """Up to the end of the text, or to the `)` that closes this level, or to the end of a case's branch: an `esac` then says so by returning true."""
        text = self.text
        words: list[str] = []
        depth, pipeline = 0, next(self.pipelines)
        continued, started = False, len(self.found)  # After a `|`, a newline does not end the pipeline. What it found from `started` on stands in this command's words.

        def finish() -> None:
            nonlocal words, started
            if words:
                self.emit(words, pipeline, started)
            words = []
            started = len(self.found)

        while self.at < len(text):
            c, two = text[self.at], text[self.at : self.at + 2]
            redirection = REDIRECTION.match(text, self.at)
            at_start = all(word in BEFORE_A_COMMAND or word == "time" for word in words)
            if c in " \t":
                self.at += 1
            elif two == "\\\n":
                self.at += 2
            elif c == "\n":
                finish()
                if not continued:
                    pipeline = next(self.pipelines)
                self.at += 1
                self.read_here_docs()
            elif c == "#":
                end = text.find("\n", self.at)
                self.at = len(text) if end < 0 else end
            elif c == ")":
                self.at += 1
                finish()
                if depth == 0 and closing == ")":
                    return False
                depth = max(0, depth - 1)
                pipeline = next(self.pipelines)
            elif c == "(" and words and ARRAY_ASSIGNMENT.fullmatch(words[-1]) and text[self.at - 1] == "=":
                self.array()
            elif c == "(" and words and A_FUNCTION.match(text, self.at) and all(word in BEFORE_A_COMMAND or word == "time" for word in words[:-1]):
                self.at = A_FUNCTION.match(text, self.at).end()  # Its name is no command, and its body is read as any other.
                words = words[:-1]
            elif two == "((" and (at_start or words[-1:] == ["for"]) and (end := self.arithmetic_end(self.at + 2)) is not None:
                self.read_expansions(text[self.at + 2 : end - 2])
                self.at = end
            elif c == "(":
                finish()
                depth += 1
                self.at += 1
                pipeline = next(self.pipelines)
            elif two in ("<(", ">("):
                self.at += 2
                self.substitutions += 1
                self.commands(closing=")")
                self.substitutions -= 1
                words.append(f"{two}…)")
            elif redirection:
                self.at = redirection.end()
                while text[self.at : self.at + 1] in (" ", "\t"):
                    self.at += 1
                target, quoted = self.word()
                operator = redirection.group().lstrip("0123456789")
                if operator in ("<<", "<<-"):
                    self.here_docs.append((target, operator == "<<-", quoted, words))
                elif operator == "<<<" and runs_its_input(words):
                    ShellReader(target, self.found, self.pipelines).commands()
            elif closing == "case" and depth == 0 and two in (";;", ";&"):
                finish()
                self.at += 2
                if text[self.at : self.at + 1] == "&":
                    self.at += 1
                return False
            elif two in ("&&", "||"):
                finish()
                self.at += 2
                pipeline = next(self.pipelines)
            elif c in ";&":
                finish()
                self.at += 1
                pipeline = next(self.pipelines)
            elif two == "|&":
                finish()
                self.at += 2
                continued = True
            elif c == "|":
                finish()
                self.at += 1
                continued = True
            else:
                continued = False
                word = self.word()[0]
                # A `for` or `select` needs no `;` before its body: what follows its header is commands.
                if word in ("do", "{") and a_loop_header(words):
                    words = []
                elif A_DESCRIPTOR_VARIABLE.fullmatch(word) and REDIRECTION.match(text, self.at):
                    pass  # `{fd}>log`: where a redirection puts a descriptor, and no word of the command.
                elif at_start and word == "case":
                    self.case()
                elif at_start and word == "[[":
                    self.condition()
                elif at_start and word == "function":
                    self.blanks()
                    self.word()
                    self.at = self.past_empty_parentheses()
                elif at_start and word == "esac" and closing == "case":
                    finish()
                    return True
                else:
                    words.append(word)
        finish()
        return closing == "case"

    def blanks(self, newlines: bool = False, a_word_begins_after: str = " \t\n;&|()") -> None:
        """Past blanks, and with `newlines` past them and comments too. A `#` begins a comment only where a word could begin, which in a `[[ ]]` is after a blank alone: `)#(` in a regular expression is none."""
        text = self.text
        while self.at < len(text):
            c = text[self.at]
            if c in " \t" or text[self.at : self.at + 2] == "\\\n":
                self.at += 1 if c != "\\" else 2
            elif newlines and c == "\n":
                self.at += 1
                self.read_here_docs()
            elif newlines and c == "#" and (self.at == 0 or text[self.at - 1] in a_word_begins_after):
                end = text.find("\n", self.at)
                self.at = len(text) if end < 0 else end
            else:
                return

    def past_empty_parentheses(self) -> int:
        found = A_FUNCTION.match(self.text, self.at)
        return found.end() if found else self.at

    def case(self) -> None:
        """A `case`, from its subject on: each pattern is text whatever it says, and each branch commands, to the `esac` that closes it."""
        text = self.text
        self.blanks(newlines=True)
        self.word()
        self.blanks(newlines=True)
        self.word()
        while self.at < len(text):
            self.blanks(newlines=True)
            first = True
            if text[self.at : self.at + 1] == "(":
                self.at += 1
                first = False  # `(esac)` is a pattern, and only a bare `esac` ends it.
            while self.at < len(text):
                self.blanks()
                c = text[self.at : self.at + 1]
                if c == ")":
                    self.at += 1
                    break
                before = self.at
                word = self.word()[0]
                if first and word == "esac":
                    return
                first = False
                if self.at == before:
                    self.at += 1  # An alternative's `|`, or anything the shell would refuse.
            if self.commands(closing="case"):
                return

    def condition(self) -> None:
        """A `[[ … ]]`: text but for what a substitution in it runs, whatever a pattern or a regular expression in it says, `(`, `|`, `<` and `&&` among it."""
        text = self.text
        while self.at < len(text):
            self.blanks(newlines=True, a_word_begins_after=" \t\n")
            if text[self.at : self.at + 2] == "]]" and text[self.at + 2 : self.at + 3] in ("", " ", "\t", "\n", ";", "&", "|", ")", "#", "<", ">"):
                self.at += 2
                return
            before = self.at
            self.word()
            if self.at == before:
                self.at += 1

    def array(self) -> None:
        """`name=(…)`'s elements: text but for what a substitution in them runs."""
        text = self.text
        self.at += 1
        while self.at < len(text):
            self.blanks(newlines=True)
            if text[self.at : self.at + 1] == ")":
                self.at += 1
                return
            before = self.at
            self.word()
            if self.at == before:
                self.at += 1

    def arithmetic_end(self, at: int) -> int | None:
        """Past the `))` of what stands at `at`, just inside a `((`: or nothing, when the inner parenthesis does not close right before the outer, and they are subshells, as the shell decides it."""
        text, depth = self.text, 1
        while at < len(text):
            c = text[at]
            if c == "\\":
                at += 1
            elif c in "'\"":
                end = text.find(c, at + 1)
                at = len(text) if end < 0 else end
            elif c == "(":
                depth += 1
            elif c == ")":
                depth -= 1
                if depth == 0:
                    return at + 2 if text[at + 1 : at + 2] == ")" else None
            at += 1
        return None

    def read_expansions(self, inside: str) -> None:
        """Arithmetic runs nothing but what a substitution in it would."""
        ShellReader(inside, self.found, self.pipelines).expansions([], None)

    def word(self) -> tuple[str, bool]:
        """One word, as the command would get it, and whether any of it was quoted."""
        text = self.text
        parts: list[str] = []
        quoted = False
        while self.at < len(text):
            c = text[self.at]
            if c in " \t\n;&|()<>":
                break
            if c == "\\":
                following = text[self.at + 1 : self.at + 2]
                parts.append("" if following == "\n" else following)
                self.at += 2
                quoted = True
            elif c == "'":
                end = text.find("'", self.at + 1)
                end = len(text) if end < 0 else end
                parts.append(text[self.at + 1 : end])
                self.at = end + 1
                quoted = True
            elif c == '"':
                self.at += 1
                self.expansions(parts, '"')
                quoted = True
            elif c == "`":
                self.backticks(parts)
            elif c == "$":
                self.dollar(parts)
            else:
                parts.append(c)
                self.at += 1
        return "".join(parts), quoted

    def expansions(self, parts: list[str], closing: str | None) -> None:
        """What stands between double quotes, or in a here-document: text, in which only a substitution still runs."""
        text = self.text
        while self.at < len(text):
            c = text[self.at]
            if c == closing:
                self.at += 1
                return
            if c == "\\":
                following = text[self.at + 1 : self.at + 2]
                parts.append(following if following in ('"', "$", "`", "\\") else "" if following == "\n" else c + following)
                self.at += 2
            elif c == "`":
                self.backticks(parts)
            elif c == "$":
                self.dollar(parts, in_quotes=True)
            else:
                parts.append(c)
                self.at += 1

    def dollar(self, parts: list[str], in_quotes: bool = False) -> None:
        text = self.text
        two, three = text[self.at : self.at + 2], text[self.at : self.at + 3]
        end = self.arithmetic_end(self.at + 3) if three == "$((" else None
        if end is not None:
            self.read_expansions(text[self.at + 3 : end - 2])
            parts.append(text[self.at : end])
            self.at = end
        elif two == "$(":
            self.at += 2
            self.substitutions += 1
            self.commands(closing=")")
            self.substitutions -= 1
            parts.append("$(…)")
        elif two == "$[":
            depth, at = 0, self.at + 1
            while at < len(text):
                depth += {"[": 1, "]": -1}.get(text[at], 0)
                at += 1
                if depth == 0:
                    break
            self.read_expansions(text[self.at + 2 : at - 1])
            parts.append(text[self.at : at])
            self.at = at
        elif two == "${":
            self.at += 2
            parts.append("${")
            while self.at < len(text) and text[self.at] != "}":
                c = text[self.at]
                if c == "$":
                    self.dollar(parts, in_quotes=True)
                elif c == "`":
                    self.backticks(parts)
                elif c == "\\":
                    parts.append(text[self.at : self.at + 2])
                    self.at += 2
                elif c == '"':
                    self.at += 1
                    self.expansions(parts, '"')
                elif c == "'" and not in_quotes:
                    end = text.find("'", self.at + 1)
                    end = len(text) if end < 0 else end
                    parts.append(text[self.at + 1 : end])
                    self.at = end + 1
                else:
                    parts.append(c)
                    self.at += 1
            parts.append("}")
            self.at += 1
        elif two == "$'" and not in_quotes:
            at, said = self.at + 2, []
            while at < len(text) and text[at] != "'":
                if text[at] == "\\":
                    said.append({"n": "\n", "t": "\t"}.get(text[at + 1 : at + 2], text[at + 1 : at + 2]))
                    at += 2
                else:
                    said.append(text[at])
                    at += 1
            parts.append("".join(said))
            self.at = at + 1
        elif two == '$"' and not in_quotes:
            self.at += 2
            self.expansions(parts, '"')
        else:
            parts.append("$")
            self.at += 1

    def backticks(self, parts: list[str]) -> None:
        text = self.text
        at, inner = self.at + 1, []
        while at < len(text) and text[at] != "`":
            if text[at] == "\\" and text[at + 1 : at + 2] in ("`", "\\", "$"):
                inner.append(text[at + 1])
                at += 2
            else:
                inner.append(text[at])
                at += 1
        self.at = at + 1
        ShellReader("".join(inner), self.found, self.pipelines).commands()
        parts.append("`…`")

    def read_here_docs(self) -> None:
        """Each body waiting: text, in which a substitution still runs unless its delimiter was quoted; and what a shell it feeds runs, read as what is run."""
        text = self.text
        here_docs, self.here_docs = self.here_docs, []
        for delimiter, strip_tabs, quoted, fed in here_docs:
            lines: list[str] = []
            while self.at < len(text):
                end = text.find("\n", self.at)
                end = len(text) if end < 0 else end
                line = text[self.at : end]
                plain = line.lstrip("\t") if strip_tabs else line
                # Inside a substitution a delimiter may close on its `)`, which is then the substitution's.
                if self.substitutions and plain.startswith(delimiter) and plain[len(delimiter) :].lstrip(" \t").startswith(")"):
                    self.at += len(line) - len(plain) + len(delimiter)
                    break
                self.at = end + 1
                if plain == delimiter:
                    break
                lines.append(line)
            if runs_its_input(fed):
                ShellReader("\n".join(lines), self.found, self.pipelines).commands()
            elif not quoted:
                self.read_expansions("\n".join(lines))

    def emit(self, words: list[str], pipeline: int, started: int) -> None:
        if words[0] in NOT_A_COMMAND:
            return
        words, fed = unwrapped(words)
        if not words or words[0] in NOT_A_COMMAND:
            return
        name = os.path.basename(words[0])
        if name in SHELLS and shell_only_parses(name, words[1:]):
            return
        script = shell_told(words[1:])[0] if name in SHELLS else None
        if script is not None:
            ShellReader(script, self.found, self.pipelines).commands()
        elif name == "eval":
            ShellReader(" ".join(words[1:]), self.found, self.pipelines).commands()
        elif name == "trap":
            ShellReader(plain_word(words[1:]) or "", self.found, self.pipelines).commands()
        else:
            self.found.append(Command(words, pipeline, fed, started))


def commands_of(text: str) -> list[Command]:
    """What `text` would run, in the order it would reach them: a substitution before the command it stands in, and what a wrapper or `bash -c` runs in place of them. An
    unterminated quote reads to the end, since the shell will refuse it anyway. Too deep to read raises, and the hook refuses what it cannot read."""
    found: list[Command] = []
    ShellReader(text, found, itertools.count()).commands()
    return found


# ---- what runs through it

CARGO_FLAGS_WITH_VALUES = {"--config", "-Z", "-C", "--color"}
# What only asks, edits a manifest or a lock file, fetches, or frees room. Nothing here compiles or holds much.
CARGO_ONLY_ASKS = {"fmt", "tree", "metadata", "locate-project", "pkgid", "search", "help", "clean", "version", "update", "generate-lockfile", "add", "remove", "info", "init", "new", "fetch"}
# Asked of a tool, these say something and run nothing, up to a `--`, past which words are what it runs.
ASKING_FLAGS = {"-V", "-vV", "--version", "--help", "-h"}
# `make` of a recipe that goes through the guard and waits for room by itself.
MAKE_GUARDED = {"focused", "integration", "tiers", "gate", "check", "clippy", "test", "book", "help", "hooks", "fmt"}
MAKE_FLAGS_WITH_VALUES = {"-I", "-l", "-o", "-W", "--include-dir", "--load-average", "--old-file", "--what-if"}
# Asked of `make`, these run no recipe. The letters are read as a cluster reads them, up to one that takes the rest as its value.
MAKE_ASKS = {"--dry-run", "--just-print", "--recon", "--question", "--version", "--help"}
MAKE_ASKS_IN_A_CLUSTER = re.compile(r"-[abdeikLmprRsStwB]*[nqvh]")
# What a cargo alias runs: `--help` after one goes to a program it first compiles.
CARGO_ALIASES = {"xtask"}


def make_targets(arguments: list[str]) -> list[str]:
    targets, at = [], 0
    while at < len(arguments):
        word = arguments[at]
        if word in MAKE_FLAGS_WITH_VALUES or word in ("-C", "-f", "--directory", "--file", "--makefile"):
            at += 1
        elif word in ("-j", "--jobs") and arguments[at + 1 : at + 2] and (arguments[at + 1].isdigit() or arguments[at + 1].startswith("$")):
            at += 1
        elif not word.startswith("-") and not ASSIGNMENT.match(word):
            targets.append(word)
        at += 1
    return targets


def make_only_asks(arguments: list[str]) -> bool:
    """Whether `make` given these runs no recipe, or only the one that says what there is."""
    return make_targets(arguments) == ["help"] or any(word in MAKE_ASKS or MAKE_ASKS_IN_A_CLUSTER.match(word) for word in arguments)


def make_is_guarded(arguments: list[str]) -> bool:
    """Whether `make` with these runs only what goes through the guard here: another directory or makefile may run anything, and with no target it may too."""
    if make_only_asks(arguments):
        return True
    if any(word in ("-C", "-f", "--directory", "--file", "--makefile") or word.startswith(("-C", "-f", "--directory=", "--file=", "--makefile=")) for word in arguments):
        return False
    targets = make_targets(arguments)
    return bool(targets) and all(target in MAKE_GUARDED for target in targets)


def only_asks(arguments: list[str]) -> bool:
    """Whether a tool given these only says something: `help` first, or an asking flag before any `--`."""
    if arguments[:1] == ["help"]:
        return True
    for word in arguments:
        if word == "--":
            return False
        if word in ASKING_FLAGS or word.startswith(("--print", "--explain")):
            return True
    return False


def is_heavy(words: list[str]) -> bool:
    """Whether a command may use real memory or write much. This is a courtesy to the host, not its guard: what reaches past it, a program in a variable, a `python3` that allocates,
    the watcher still holds."""
    if not words:
        return False
    name, arguments = os.path.basename(words[0]), words[1:]
    if name == "cargo":
        subcommand = plain_word(arguments, CARGO_FLAGS_WITH_VALUES)
        return subcommand is not None and subcommand not in CARGO_ONLY_ASKS and (subcommand in CARGO_ALIASES or not only_asks(arguments))
    if name in MAKES:
        return not make_is_guarded(arguments)
    if name in ("rustc", "rustdoc", "clippy-driver", "mdbook"):
        return bool(arguments) and not only_asks(arguments)
    if name in SHELLS:
        script = shell_told(arguments)[1]
        return script is not None and os.path.basename(script) == "build_book.sh"
    return (name.startswith(("cargo-", "qemu-system")) or name in ("archogen", "archogen-plan-check", "build_book.sh")) and not only_asks(arguments)


def kill_targets(arguments: list[str]) -> list[str]:
    """What `kill` given these signals, as it reads them: a signal is `-s NAME`, `-n NUMBER`, or `-NAME` or `-NUMBER` before any target, and `--` ends them. So `-$(…)` and `-$GROUP`, a process group, are targets."""
    targets: list[str] = []
    at, signalled = 0, False
    while at < len(arguments):
        word = arguments[at]
        if word == "--":
            return targets + arguments[at + 1 :]
        if word in ("-s", "-n", "--signal"):
            at, signalled = at + 2, True
            continue
        if targets or signalled or not re.fullmatch(r"-[A-Za-z0-9+]+", word):
            targets.append(word)
        signalled = True
        at += 1
    return targets


def what_a_run_runs(words: list[str]) -> list[str]:
    """What `words`, a guard's `run`, are told to run, as `main` reads them, its `--` being optional; nothing of anything else, a message that names one among it."""
    at = 0
    if words and (os.path.basename(words[0]) in INTERPRETERS or os.path.basename(words[0]).startswith("python")):
        at = 1
        while at < len(words) and words[at].startswith("-"):
            at += 2 if words[at] in PYTHON_FLAGS_WITH_VALUES else 1
    if words[at : at + 1] == [] or os.path.basename(words[at]) != GUARD_NAME or words[at + 1 : at + 2] != ["run"]:
        return []
    rest = words[at + 2 :]
    while rest and rest[0] in ("--root", "--limits", "--wait", "--once"):
        rest = rest[1:] if rest[0] == "--once" else rest[2:]
    return rest[1:] if rest[:1] == ["--"] else rest


def is_number(word: str, value: int) -> bool:
    """Whether `word` is `value` as the widest of the tools reads a number, `pgrep`: `01`, `0x1`, `+1` and ` 1` are each 1. `kill` refuses some of them, which stops nothing."""
    text = word.strip()
    for base in (0, 10):
        try:
            return int(text, base) == value
        except ValueError:
            pass
    return False


def kills_by_name(commands: list[Command], root: Path) -> bool:
    """Whether these stop processes by a name rather than a pid, with nothing saying it means this repository's: `pkill` or `killall` naming no path here, or `kill` of what a search
    of the process table found. A search names this root if anything in its pipeline does. `-P` goes by a parent's pid, and `kill` of numbers by theirs, whatever else ran before."""
    text = str(root)
    # What the guard's own `run` is told to run is that command, and a refusal sends an agent there. Read here as itself, a search among it, and alone for what it wraps.
    read_through: list[Command] = []
    for command in commands:
        told = what_a_run_runs(command.words)
        if told and kills_by_name(commands_of(shlex.join(told)), root):
            return True
        read_through.append(Command(told, command.pipeline, command.fed, command.own_from) if told else command)
    commands = read_through

    def names_this_root(word: str) -> bool:
        # Exactly it: `archogen-other` and `archogen copy` begin the same, and neither a name's own characters nor a space end a path.
        return re.search(re.escape(text) + r"(?![\w -]|\.\w)", word) is not None

    def pipeline_names_this_root(search: Command) -> bool:
        # What it excludes or where it saves what it found says nothing of which processes it means (G8-4).
        for other in commands:
            if other.pipeline != search.pipeline or other.name == "tee":
                continue
            if inverted(other):
                continue
            if any(names_this_root(word) for word in other.words):
                return True
        return False

    def inverted(command: Command) -> bool:
        # What it excludes says nothing of which processes it means, and `pkill -v` of a path stops all that is not it.
        return command.name in ("grep", "egrep", "fgrep", "rg", "pgrep", "pkill") and any(re.fullmatch(r"-[a-zA-Z]*v[a-zA-Z]*|--invert-match|--inverse", word) for word in command.words[1:])

    def goes_by_pid(command: Command) -> str | None:
        """What it goes by, a pid file or a parent's pid, in any cluster, `-lP 123`, if it does. 1 is no such pid: every orphan on the host has it for a parent. Inverted, it is all that is not those."""
        if inverted(command):
            return None
        words = command.words[1:]
        values = []
        for at, word in enumerate(words):
            # A signal's name reads like a cluster, and is none.
            flag = None if word in ("-PIPE", "-PROF", "-FPE") else re.fullmatch(r"-[acfilLnoqSvx]*([PFg])(.*)", word, re.S)
            if flag:
                value = flag.group(2) or (words[at + 1] if at + 1 < len(words) else "")
                # However a number is spelt: `pkill -P 01` is `-P 1`, and what `$((…))` or `$[…]` works out could be. And wherever it stands: a repeated `-P` is any of them.
                if flag.group(1) == "P" and any(is_number(each, 1) or each.startswith(("$((", "$[")) for each in value.strip("'\"").split(",")):
                    return None
                values.append(value)
        # What the caller reads it by is the least plain of them.
        return next((value for value in values if not A_PLAIN_TARGET.fullmatch(value)), values[0] if values else None)

    def searches_by_name(command: Command) -> bool:
        if command.name == "pgrep":
            return goes_by_pid(command) is None
        # Others that list the table, by a name or for a pipeline to pick from.
        if command.name in ("top", "htop"):
            return True
        if command.name == "lsof":
            return any(re.fullmatch(r"-[a-zA-Z]*c.*", word) for word in command.words[1:])
        if command.name != "ps":
            return False
        words = command.words[1:]
        # What selects more than a pid does, and beside a `-p` or a `-G` prints all it selects: `ps -ef -p 1`, `ps -Ap 1`, `ps -G 20 1`. Beside a plain pid some print that alone, `ps ax 1`, `ps -U 501 1`, and are refused with the rest: a rewrite is all that costs.
        plain = "AaCcdEefhjLlMmrSTvwXx"
        if any(re.fullmatch(rf"-[{plain}]*[Aaed]\S*|-[{plain}]*[UuGt].*", word) for word in words):
            return True
        # As BSD writes them, with no dash: `ps ax`, `ps U 501`, `ps G 20`, `ps dp 1`. A `u` is a format, as `ps up 123` writes it.
        if words and re.fullmatch(r"[A-Za-z]*[AaedUGt][A-Za-z]*", words[0]):
            return True
        a_pid = r"(?:\d+|\$[\w$!@*]+|\$\{[^}]*\}|\$\(.*\)|`.*`)"
        for at, word in enumerate(words):
            # `-p` or a group's `-g` in any cluster of `ps`'s plain flags, as `ps -fp 123` and `ps -p$$` write it. Past a flag taking a value it is that value: `-opid`.
            if re.fullmatch(rf"-[{plain}]*[pg]\S*|--pid.*", word):
                return False
            # Or a pid written plainly, however a shell spells one, or several: `ps 123`, `ps up 123`, `ps -f "$PID"`, `ps $$`, `ps "$(cat job.pid)"`, `` ps `cat job.pid` ``, `ps "$a,$b"`, unless it is what the flag before it takes: `-O 1`.
            before = words[at - 1] if at else ""
            if re.fullmatch(rf"{a_pid}(?:,{a_pid})*", word.strip("'\""), re.S) and not re.fullmatch(rf"-[{plain}]*[oOkNW]", before):
                return False
        return True

    for at, command in enumerate(commands):
        if command.name in ("pkill", "killall"):
            given = goes_by_pid(command) if command.name == "pkill" else None
            if given is None:
                if inverted(command) or not any(names_this_root(word) for word in command.words[1:]):
                    return True
            elif not A_PLAIN_TARGET.fullmatch(given):
                # What it goes by is what `kill`'s target is: what a substitution alone feeds it is what that ran, and a variable or a file may hold what anything before it found.
                alone = A_SUBSTITUTION_ALONE.fullmatch(given)
                for search in commands[command.own_from if alone else 0 : at]:
                    if searches_by_name(search) and not pipeline_names_this_root(search):
                        return True
        elif command.name == "kill":
            # Signal 0 stops nothing: it asks whether a process stands. Only as exactly that, `kill -0 PID` or `kill -s 0 PID`: a later flag, however it is written, may be what it sends, `-s KILL`, `-sKILL`, `-n9`, so what has one reads as any `kill` does.
            asks = command.words[2:] if command.words[1:2] == ["-0"] else command.words[3:] if command.words[1:3] == ["-s", "0"] else None
            if asks is not None and not any(word.startswith("-") for word in asks):
                continue
            targets = kill_targets(command.words[1:])
            # -1 is every process this user may signal, on all of the host, however a number is spelt.
            if any(is_number(word, -1) or "$((" in word or "$[" in word for word in targets):
                return True
            if command.fed or not all(A_PLAIN_TARGET.fullmatch(word) for word in targets):
                # What a substitution alone feeds it is what that substitution ran. A variable may hold what anything before it found.
                alone = not command.fed and all(A_SUBSTITUTION_ALONE.fullmatch(word) or A_PLAIN_TARGET.fullmatch(word) for word in targets)
                for search in commands[command.own_from if alone else 0 : at]:
                    if searches_by_name(search) and not pipeline_names_this_root(search):
                        return True
    return False


def heavy_running(root: Path) -> list[Process]:
    """What of ours is a heavy job now: a build, or another `run` whose job has started. A `run` still waiting its turn has started nothing, nor has a `make` waiting inside it."""
    table = processes()
    me = os.getpid()
    text = str(root)
    line = line_of(table, me)
    children = children_of(table)
    by_pid = {process.pid: process for process in table}
    running = [p for p in ours(table, root, me) if p.name in BUILDING and p.spawns]
    for process in table:
        if process.pid not in line and is_run_of(process, text):
            # The watcher it may have started is its child too, and no job; nor are the tools it reads the host with.
            job = [by_pid[pid] for pid in children.get(process.pid, []) if pid in by_pid and guard_told(by_pid[pid]) is None]
            if any(not says_no_job(child) for child in job):
                running.append(process)
    return running


def one_at_a_time(root: Path) -> list[str]:
    """Why a heavy job may not start beside what runs now."""
    return [f"one heavy job at a time, and {process.label} (pid {process.pid}) is running" for process in heavy_running(root)[:1]]


def run(root: Path, limits: Limits, command: list[str], wait_seconds: float = 0.0) -> int:
    if not command:
        say("host-guard: `run` takes a command, after `--`")
        return 2
    try:
        state = state_dir(root)
        for event in unseen(root, "run"):
            say(f"host-guard: {said_of(event)}")
        ensure(root)
        job_lock = open(state / "job.lock", "a")
    except OSError as error:
        say(f"host-guard: not started: its state cannot be used, so nothing would watch this job: {error}")
        return UNCLEAR
    # A job nested in its own `run` is no rival. That is read from the process table, so no environment can claim it.
    try:
        table = processes()
        by_pid = {process.pid: process for process in table}
        nested = any(is_run_of(by_pid[pid], str(root)) for pid in line_of(table, os.getpid()) - {os.getpid()})
    except Exception:
        nested = False  # `why_not` will say the host cannot be read.
    begun = time.monotonic()

    def why_not() -> list[str]:
        try:
            reasons = start_reasons(read_host(root), limits)
            if nested:
                return reasons
            reasons.extend(one_at_a_time(root))
        except Exception as error:
            return [f"the host cannot be read, so nothing says this is safe: {error}"]
        try:
            fcntl.flock(job_lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except OSError:
            reasons.append("one heavy job at a time, and another `run` holds its turn")
        return reasons

    # The host is shared: when it is full, the polite thing is to wait for room, not to add to it.
    reasons = why_not()
    said_at = 0.0
    while reasons and time.monotonic() - begun < wait_seconds:
        # Waiting is no silence: whoever started this should know why nothing is happening.
        if time.monotonic() - said_at > 60:
            said_at = time.monotonic()
            say(f"host-guard: waiting for room, up to {wait_seconds - (time.monotonic() - begun):.0f} s more: {'; '.join(reasons)}.")
        time.sleep(min(5.0, max(0.1, wait_seconds - (time.monotonic() - begun))))
        try:
            (state_dir(root) / "heartbeat").touch()
            ensure(root)
        except OSError:
            pass
        reasons = why_not()
    if reasons:
        say(f"host-guard: not started: {'; '.join(reasons)}.")
        say("  hint: wait for the host to have room, and look at `host_guard.py status`")
        return UNCLEAR
    # A build's cost is its parallelism, and this machine is shared: a third of its cores, and a lower priority.
    env = dict(os.environ)
    jobs = str(max(2, (os.cpu_count() or 4) // 3))
    env.setdefault("CARGO_BUILD_JOBS", jobs)
    env.setdefault("RUST_TEST_THREADS", jobs)
    logged = size_of(state / "events.log")
    # In a session of its own, so that all of it can be stopped with no process table to read. At least this low a priority, however many guards it is nested in.
    try:
        job = subprocess.Popen(command, env=env, start_new_session=True, preexec_fn=lambda: os.nice(max(0, 10 - os.nice(0))))
    except OSError as error:
        say(f"host-guard: `{command[0]}` cannot be run: {error}")
        return 127 if isinstance(error, FileNotFoundError) else 126
    try:
        ensure(root)
    except OSError:
        pass

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
            # One that left for a changed guard is started from the file as it stands. One that cannot start from it leaves this to end the job, below.
            try:
                ensure(root)
            except OSError:
                pass
            if time.monotonic() - blind_since > limits.watcher_stale_seconds:
                say("host-guard: no watcher can read the host, so this job is unwatched: stopping it")
                to_job(signal.SIGTERM)
                try:
                    job.wait(timeout=2)
                except subprocess.TimeoutExpired:
                    pass
                to_job(signal.SIGKILL)
                job.wait()
                return UNCLEAR
    # The log may have been replaced under it, by `cargo clean`: then all of it is new.
    try:
        path = state_path(root) / "events.log"
        with open(path, "rb") as lines:
            lines.seek(0 if size_of(path) < logged else logged)
            text = lines.read(LOG_AT_MOST)
    except OSError:
        text = b""
    for event in filter(names_a_stop, events_of(text)):
        say(f"host-guard: {said_of(event)}")
    # As a shell says it: a job ended by a signal is 128 and the signal.
    return 128 - status if status < 0 else status


def heavy_refused(root: Path, limits: Limits) -> str | None:
    """Why a heavy command may not start now, if it may not."""
    try:
        reasons = start_reasons(read_host(root), limits)
    except Exception as error:
        return f"host-guard: refused, this is a heavy command, and the host cannot be read, so nothing says it is safe: {error}"
    if reasons:
        return f"host-guard: refused, this is a heavy command: {'; '.join(reasons)}. To wait for room, run it in the background as `python3 scripts/host_guard.py run --wait 1800 -- COMMAND`; `make` does so by itself."
    # A watcher just started has had no time to read the host, so it gets a moment before it is called blind.
    deadline = time.monotonic() + 3.0
    while not healthy(root, limits) and time.monotonic() < deadline:
        time.sleep(0.1)
    if not healthy(root, limits):
        return "host-guard: refused, this is a heavy command, and no watcher has read the host lately: look at `host_guard.py status`."
    try:
        running = heavy_running(root)
    except Exception as error:
        return f"host-guard: refused, this is a heavy command, and the process table cannot be read: {error}"
    if running:
        return f"host-guard: refused: one heavy job at a time, and {running[0].label} (pid {running[0].pid}) is running: {running[0].command[:120]}. Wait for it, or queue behind it as `python3 scripts/host_guard.py run --wait 1800 -- COMMAND`."
    return None


def hook(root: Path, limits: Limits) -> int:
    """Exit 2 refuses the command, and what it prints to stderr goes to the agent. It refuses three things: what it cannot read, stopping by a name, and a heavy command with no room. What the
    watcher has stopped is told beside the command's result and refuses nothing, since the way out may be that very command. Anything else runs, whatever the host's state: a light
    command cannot hurt, the watcher holds what turns out otherwise, and a host in trouble must stay readable and repairable."""
    try:
        given = json.load(sys.stdin)
        # A tool that opens a socket and runs no shell has nothing to read, as `Monitor` can.
        if given.get("tool_name") not in (None, "Bash") and "command" not in given["tool_input"]:
            return 0
        command = given["tool_input"]["command"]
        # A subagent shares its session's id, and has an `agent_id` of its own: each is told for itself, or a stop goes to whichever runs a command first.
        session = str(given.get("session_id", "")) + str(given.get("agent_id", ""))
        if not isinstance(command, str) or not command.strip():
            raise ValueError("no command")
        commands = commands_of(command)
        kills = kills_by_name(commands, root)
    except Exception as error:
        say(f"host-guard: refused: this cannot be read as a command to judge: {error!r}")
        return 2
    if kills:
        say("host-guard: refused: never by a program's name across the host, which can stop another project's work. Stop what is ours by its pid, or name this repository's path in the pattern.")
        return 2
    happened: list[dict] = []
    try:
        ensure(root)
        (state_dir(root) / "heartbeat").touch()
        happened = unseen(root, session)
    except Exception as error:
        say(f"host-guard: cannot keep its own state: {error}")
    told = [f"host-guard: {said_of(event)}" for event in happened[-5:]]
    if len(happened) > 5:
        told.insert(0, f"host-guard: and {len(happened) - 5} stopped before these, in `{state_path(root) / 'events.log'}`")
    refused = heavy_refused(root, limits) if any(is_heavy(each.words) for each in commands) else None
    if refused:
        for line in told:
            say(line)
        say(refused)
        return 2
    if told:
        # Beside the tool's result, as the harness's reference has it for `additionalContext`. With no decision here, the command goes on through the normal permissions.
        print(json.dumps({"hookSpecificOutput": {"hookEventName": "PreToolUse", "additionalContext": "\n".join(told)}}))
    return 0


def status(root: Path, limits: Limits) -> int:
    # What asks makes the directory, and a watcher names itself in it again within its round.
    state, watcher = state_path(root), watcher_asked_for(root)
    try:
        reading = read_host(root)
    except Exception as error:
        print(f"the host cannot be read: {error}")
        return 0
    print(f"RAM used {reading.used_percent:.1f}% (start under {limits.start_used_percent:.0f}, stop at {limits.stop_used_percent:.0f}, all holding memory stopped at {limits.panic_used_percent:.0f}); macOS: pressure level {reading.pressure_level} (1 normal, 2 warning, 4 critical), {reading.free_percent_says}% free; swap {reading.swap_used_gib:.1f} GiB")
    print(f"free: repository volume {reading.repo_free_gib:.0f} GiB, boot volume {reading.boot_free_gib:.0f} GiB")
    try:
        table = processes()
        mine = ours(table, root, os.getpid())
        print(f"ours: {len(mine)} process(es), {sum(p.bytes for p in mine) / GIB:.2f} GiB; largest: " + (f"{mine[0].label} {mine[0].bytes / GIB:.2f} GiB" if mine else "none"))
        print("largest of this user's on the host: " + ", ".join(f"{h['name']} (pid {h['pid']}) {h['gib']} GiB" for h in largest_on_host(table, 3)))
    except Exception as error:
        print(f"the process table cannot be read: {error}")
    print(("watcher: running, and reading the host" if healthy(root, limits) else "watcher: running, and BLIND: it has not read the host lately") if watcher else "watcher: not running")
    if watcher and not runs_the_file(root):
        print(f"watcher: it runs an older {CODE.name} than the one saved, and has not taken it up. If this stays, `stop` it, and the next command starts one from the file")
    try:
        print(f"peak since its start: {(state / 'peak.json').read_text(errors='replace')}")
    except OSError:
        pass
    try:
        with open(state / "events.log", "rb") as log:
            log.seek(max(0, size_of(state / "events.log") - LOG_AT_MOST))
            lines = log.read()
    except OSError:
        lines = b""
    events = events_of(lines)
    # What was stopped is what it is asked: the last few of those, however many other lines came after. A text of the log may hold what no terminal takes, which is said as it can be.
    for event in list(filter(names_a_stop, events))[-3:]:
        print(said_of(event).encode("utf-8", "replace").decode("utf-8"))
    for event in [e for e in events if e["kind"] != "stopped"][-2:]:
        said = f"{event['at']}: {event['kind']}: {event['reasons'] if event['reasons'] is not None else event['error']}"
        print(said.encode("utf-8", "replace").decode("utf-8"))
    reasons = start_reasons(reading, limits)
    try:
        reasons.extend(one_at_a_time(root))
    except Exception as error:
        reasons.append(f"the process table cannot be read: {error}")
    print("a heavy job may start" if not reasons else f"a heavy job may not start: {'; '.join(reasons)}")
    return 0


def main(argv: list[str]) -> int:
    args = list(argv)
    verb = args.pop(0) if args else "status"
    if sys.platform != "darwin":
        # This host is macOS. Elsewhere nothing is read, so nothing is claimed: a job runs unwatched, and says it.
        if verb == "run":
            command = args[args.index("--") + 1:] if "--" in args else args
            say("host-guard: this reads macOS only: running unwatched")
            return subprocess.run(command).returncode
        say("host-guard: this reads macOS only")
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
    # A `run` is known by what follows its root, the root's own spelling and a flag, so it writes both, as named or not.
    if verb == "run":
        rest, at = list(argv[1:]), 0
        while at < len(rest) and rest[at] in ("--root", "--wait", "--limits", "--once"):
            if rest[at] == "--root":
                del rest[at : at + 2]
            else:
                at += 1 if rest[at] == "--once" else 2
        if rest[at : at + 1] != ["--"]:
            rest.insert(at, "--")
        written = ["run", "--root", str(root), *rest]
        if written != list(argv):
            os.execv(sys.executable, [sys.executable, "-I", "-B", str(Path(__file__).resolve()), *written])
    try:
        limits = tightened(overrides)
    except ValueError as error:
        say(f"host-guard: {error}")
        return 2
    if verb == "check":
        try:
            reasons = start_reasons(read_host(root), limits) + one_at_a_time(root)
        except Exception as error:
            print(f"not clear: the host cannot be read, so nothing says this is safe: {error}")
            return UNCLEAR
        print("clear" if not reasons else f"not clear: {'; '.join(reasons)}")
        return UNCLEAR if reasons else 0
    if verb == "run":
        try:
            return run(root, limits, args[1:] if args[:1] == ["--"] else args, wait_seconds)
        except KeyboardInterrupt:
            say("host-guard: interrupted")
            return 130
    if verb == "watch":
        # It goes to `/`, so it names its root as resolved, on its own command line and before anything else, as a `run` does: what asks whose watcher a process is reads a whole
        # path there, never one that lay from wherever it was started, and what it takes itself up with is the same words.
        rest = list(argv[1:])
        if "--root" in rest:
            del rest[rest.index("--root") : rest.index("--root") + 2]
        written = ["watch", "--root", str(root), *rest]
        if written != list(argv):
            os.execv(sys.executable, [sys.executable, "-I", "-B", str(Path(__file__).resolve()), *written])
        try:
            return watch(root, limits, once, again=written)
        except Exception as error:
            record(root, kind="unread", error=f"the watcher ended: {error!r}")
            raise
    if verb == "ensure":
        ensure(root)
        return 0
    if verb == "hook":
        return hook(root, limits)
    if verb == "status":
        return status(root, limits)
    if verb == "stop":
        pid = watcher_asked_for(root)
        if pid and pid > 1:
            os.kill(pid, signal.SIGTERM)
        return 0
    say(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
