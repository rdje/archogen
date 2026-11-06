#!/usr/bin/env python3
"""selftest_host_guard.py: the guard stops what it should and nothing else. Each case runs in a scratch root of its own, so a watcher with a limit
tightened to nothing has only that root's processes to stop, and the memory it uses to prove so stays under a few hundred MiB. On a host already
over the start line it waits for room before each case that would add to the load, and says what it still could not run. Exit 0 all held · 1 one did not · 20 some could not run, which is not a pass.

With case names as arguments, only those run."""

from __future__ import annotations

import ast
import json
import os
import re
import select
import shutil
import signal
import stat
import subprocess
import sys
import threading
import time
from collections import deque
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
# Run as this file names itself, it must leave the tree as it found it: an untracked `scripts/__pycache__` is a code file no commit may stand beside.
sys.dont_write_bytecode = True
import host_guard as g  # noqa: E402

GUARD = HERE / "host_guard.py"
ROOT = next(at for at in HERE.parents if (at / ".git").exists())
BASE = ROOT / "target" / "host-guard" / "selftest"
PY = [sys.executable, "-I", "-B"]
HOLD = """
import sys, time
n = int(sys.argv[1]) << 20
b = bytearray(n)
for i in range(0, n, 16384):
    b[i] = 1
open(sys.argv[2], "w").close()
time.sleep(60)
"""
PARENT = """
import subprocess, sys, time
n = int(sys.argv[1]) << 20
b = bytearray(n)
for i in range(0, n, 16384):
    b[i] = 1
child = subprocess.Popen(["bash", "-c", sys.argv[5]], cwd=sys.argv[3], start_new_session=True)
open(sys.argv[4], "w").write(str(child.pid))
open(sys.argv[2], "w").close()
time.sleep(60)
"""
GROW = """
import sys, time
chunks = []
for _ in range(40):
    chunk = bytearray(10 << 20)
    for i in range(0, len(chunk), 16384):
        chunk[i] = 1
    chunks.append(chunk)
    time.sleep(0.15)
open(sys.argv[1], "w").close()
time.sleep(60)
"""
WRITE = """
import sys, time
with open(sys.argv[1], "wb") as out:
    for _ in range(60):
        out.write(bytes(10 << 20))
        out.flush()
        time.sleep(0.1)
open(sys.argv[2], "w").close()
time.sleep(60)
"""
WROTE_THEN_IDLE = """
import sys, time
with open(sys.argv[1], "wb") as out:
    out.write(bytes(10 << 20))
    out.flush()
open(sys.argv[2], "w").close()
time.sleep(60)
"""
IGNORE = """
import signal, sys, time
signal.signal(signal.SIGTERM, signal.SIG_IGN)
open(sys.argv[1], "w").close()
time.sleep(60)
"""
# How long in all it waits for room before saying what it could not run. A `make integration` is told; a host full for this long is said.
ROOM_SECONDS = 900.0
UNCLEAR_LATELY_SECONDS = 30.0
last_unclear = float("-inf")
TAKEN_AWAY = "the host filled meanwhile"
STOPPED_UNDER = "the real watcher stopped"
started: list[subprocess.Popen] = []
failures: list[str] = []
not_run: list[str] = []
passed = 0
# Whether the case at hand holds nothing and reads nothing of the host, so that a failure says the same whatever the host's state.
pure = False


def check(what: str, holds: bool, detail: object = "") -> None:
    global passed
    if holds:
        passed += 1
    elif not pure and (not host_clear() or time.monotonic() - last_unclear < UNCLEAR_LATELY_SECONDS):
        # This host is shared, and another project's load can arrive mid-test, and leave before this looks: then this says nothing either way, and it is no pass. So it goes again with room, and
        # a real failure stays one: it fails on a clear host too.
        not_run.append(f"{what} ({TAKEN_AWAY})")
    else:
        failures.append(what)
        print(f"  FAILED {what}: {str(detail)[:400]}")


def guard(*args: str, stdin: str | None = None, root: Path | None = None, limits: dict | None = None, env: dict | None = None) -> subprocess.CompletedProcess:
    verb, rest = args[0], list(args[1:])
    flags: list[str] = []
    if root:
        flags += ["--root", str(root)]
    if limits is not None:
        flags += ["--limits", json.dumps(limits)]
    return subprocess.run([*PY, str(GUARD), verb, *flags, *rest], input=stdin, capture_output=True, text=True, errors="replace", timeout=60, env=env)


def until(condition, seconds: float = 20) -> bool:
    """Whether `condition` comes true in time. A fixed wait for a job to begin reads a loaded host's slowness as a failure."""
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        if condition():
            return True
        time.sleep(0.05)
    return bool(condition())


def run_refused(job: subprocess.Popen, what: str) -> bool:
    """Whether the host filled between this test's look and `run`'s own, so that `run` started nothing: that says nothing of what the case asks, and is said as not run."""
    until(lambda: jobs_of(job.pid) or job.poll() is not None)
    if job.poll() == g.UNCLEAR:
        not_run.append(f"{what} ({TAKEN_AWAY})")
        return True
    return False


def note_when_unclear() -> None:
    """Beside the cases, every few seconds, since each look costs the shared host: when the host last would not have let a heavy job start. A `run` refused for it may be looked at after the load has gone."""
    global last_unclear
    while True:
        if not host_clear():
            last_unclear = time.monotonic()
        time.sleep(3.0)


def host_clear() -> bool:
    """Whether the host itself would let a heavy job start. No limit can be loosened to test what happens when it would, so while another project
    holds the memory the cases that need it cannot run: said, and never counted as passing. The host's room alone, and not `check`, which also counts a build of ours: a stub of this
    test's, under this root, would read as one, and three cases never run."""
    try:
        return not g.start_reasons(g.read_host(ROOT), g.tightened({}))
    except Exception:
        return False


def wait_for_room(room_until: float) -> bool:
    """Whether the host has room, within the bound: asked as `run` asks, every few seconds, since each look costs the shared host, and said first, since a long wait is not a hang. A test whose runner has
    gone, stopped by the guard, stops with it: nothing would read its answer."""
    runner, said = os.getppid(), False
    # Clear for as long as a failure is excused: on a host clear only this instant, a real failure would read as not run again and again.
    while not host_clear() or time.monotonic() - last_unclear < UNCLEAR_LATELY_SECONDS:
        if time.monotonic() >= room_until:
            return False
        if os.getppid() != runner:
            sys.exit(20)
        if not said:
            said = True
            print(f"host-guard self-test: waiting for room, up to {room_until - time.monotonic():.0f} s more", flush=True)
        time.sleep(min(5.0, max(0.0, room_until - time.monotonic())))
    return True


def fresh(name: str) -> Path:
    path = BASE / name
    shutil.rmtree(path, ignore_errors=True)
    path.mkdir(parents=True)
    return path


def spawn(args: list[str], cwd: Path, env: dict | None = None, stdin: int = subprocess.DEVNULL) -> subprocess.Popen:
    process = subprocess.Popen(args, cwd=cwd, stdin=stdin, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env, start_new_session=True)
    started.append(process)
    return process


def clean_up() -> None:
    """Everything this test started, with what that started, by the sessions it gave them: never by a name."""
    for process in started:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except OSError:
            pass
        try:
            process.wait(timeout=5)
        except subprocess.TimeoutExpired:
            pass
    started.clear()


def ready(file: Path) -> None:
    deadline = time.monotonic() + 20
    while not file.exists() and time.monotonic() < deadline:
        time.sleep(0.02)
    assert file.exists(), "it never got ready"


def holding(mib: int, cwd: Path) -> subprocess.Popen:
    file = cwd / f"ready-{len(started)}"
    process = spawn([*PY, "-c", HOLD, str(mib), str(file)], cwd)
    ready(file)
    return process


def named(name: str, cwd: Path, script: str = "sleep 60; sleep 60; true") -> subprocess.Popen:
    process = spawn(["bash", "-c", f'exec -a {name} bash -c "{script}"'], cwd)
    time.sleep(0.15)
    return process


def up(process: subprocess.Popen) -> bool:
    return process.poll() is None


def pid_up(pid: int) -> bool:
    state = subprocess.run(["ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return bool(state) and not state.startswith("Z")


def children(pid: int) -> list[int]:
    return [int(word) for word in subprocess.run(["pgrep", "-P", str(pid)], capture_output=True, text=True).stdout.split()]


def jobs_of(pid: int) -> list[int]:
    """What a `run` started, leaving out the watcher it may have started too."""
    found = []
    for child in children(pid):
        shown = subprocess.run(["ps", "-ww", "-o", "command=", "-p", str(child)], capture_output=True, text=True).stdout
        # What it reads the host with is no job of its, though it is its child for a moment; and one gone by the time `ps` looked was such a moment.
        if not shown.strip() or os.path.basename(shown.split(" ", 1)[0].strip()) in g.READS_THE_HOST:
            continue
        if "host_guard.py" not in shown:
            found.append(child)
    return found


def events(root: Path) -> list[dict]:
    log = root / "state" / "events.log"
    found = []
    if log.exists():
        for line in log.read_bytes().splitlines():
            try:
                found.append(json.loads(line))
            except ValueError:
                pass
    return found


def stopped_events(root: Path) -> list[dict]:
    return [event for event in events(root) if event.get("kind") == "stopped"]


def settle(process: subprocess.Popen, seconds: float = 4) -> bool:
    """Whether `process` has ended, within a moment."""
    try:
        process.wait(timeout=seconds)
        return True
    except subprocess.TimeoutExpired:
        return False


def looping_watcher(scope: Path, limits: dict, env: dict | None = None) -> subprocess.Popen:
    process = subprocess.Popen([*PY, str(GUARD), "watch", "--root", str(scope), "--limits", json.dumps(limits)], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env, start_new_session=True)
    started.append(process)
    return process


def watch_once(scope: Path, limits: dict) -> subprocess.CompletedProcess:
    return guard("watch", "--once", root=scope, limits=limits)


def shim(scope: Path, name: str, script: str) -> dict:
    """An environment in which `name` is `script`."""
    bin_dir = scope / "bin"
    bin_dir.mkdir(exist_ok=True)
    (bin_dir / name).write_text("#!/bin/sh\n" + script + "\n")
    (bin_dir / name).chmod(0o755)
    env = dict(os.environ)
    env["PATH"] = f"{bin_dir}:{env['PATH']}"
    return env


def hook_says(command: str, root: Path, limits: dict | None = None, env: dict | None = None, session: str | None = None, agent: str | None = None) -> subprocess.CompletedProcess:
    given = {"tool_input": {"command": command}}
    if session is not None:
        given["session_id"] = session
    if agent is not None:
        given["agent_id"] = agent
    return guard("hook", stdin=json.dumps(given), root=root, limits=limits, env=env)


def told(done: subprocess.CompletedProcess) -> str:
    """What the hook has told beside a command that runs, in the shape the harness reads: nothing, or an `additionalContext` of a `PreToolUse`, with no decision of its own."""
    if not done.stdout.strip():
        return ""
    output = json.loads(done.stdout)["hookSpecificOutput"]
    assert output["hookEventName"] == "PreToolUse" and "permissionDecision" not in output, output
    return output["additionalContext"]


def stop_event(at: str, name: str) -> str:
    return json.dumps({"kind": "stopped", "at": at, "reasons": ["why"], "stopped": [{"pid": 1, "name": name, "gib": 1.0, "command": "x"}]}) + "\n"


def proc(pid: int, command: str, mib: int = 1, ppid: int = 0) -> g.Process:
    return g.Process(pid, ppid, command, "", "", mib << 20)


def tops(targets: list[g.Job]) -> set[int]:
    return {job.top.pid for job in targets}


PANIC = {"panic_used_percent": 0}


def case_only_tightened() -> None:
    for loose in ({"stop_used_percent": 99}, {"panic_used_percent": 95}, {"stop_repo_free_gib": 1}, {"all_processes_gib": 1000}, {"watcher_stale_seconds": 9999}):
        done = guard("check", limits=loose)
        check(f"a loosened limit is refused: {loose}", done.returncode == 2 and "may only be tightened" in done.stderr, done.stderr)
    done = guard("check", limits={"no_such_limit": 1})
    check("an unknown limit is refused", done.returncode == 2 and "no limit is called" in done.stderr, done.stderr)
    done = guard("check", limits={"stop_used_percent": "low"})
    check("a limit that is no number is refused, not a traceback", done.returncode == 2 and "takes a number" in done.stderr, done.stderr)


def case_check_refuses() -> None:
    done = guard("check", limits={"start_used_percent": 0})
    check("RAM over the start line is not clear", done.returncode == 75 and "RAM use" in done.stdout, done.stdout)
    done = guard("check", limits={"stop_repo_free_gib": 1e9})
    check("a volume under its floor is not clear", done.returncode == 75 and "free" in done.stdout, done.stdout)
    done = guard("check", limits={"stop_boot_free_gib": 1e9})
    check("the boot volume under its floor is not clear", done.returncode == 75 and "boot volume" in done.stdout, done.stdout)
    done = guard("check", limits={"start_pressure_level": 0})
    check("macOS's pressure level stops a heavy job starting", done.returncode == 75 and "memory pressure" in done.stdout, done.stdout)
    # `check` and `status` say what `run` and the hook will do: one heavy job at a time.
    scope = fresh("check-beside")
    builder = named("cargo", scope)
    done = guard("check", root=scope)
    check("beside a heavy job of ours, it is not clear", done.returncode == 75 and "one heavy job at a time" in done.stdout, done.stdout)
    last = guard("status", root=scope).stdout.splitlines()[-1]
    check("and `status` does not say one may start", last.startswith("a heavy job may not start") and "one heavy job at a time" in last, last)
    os.killpg(builder.pid, signal.SIGKILL)
    builder.wait()


def case_a_figure_not_read_is_never_safe() -> None:
    scope = fresh("unread")
    env = shim(scope, "vm_stat", "echo 'Mach Virtual Memory Statistics: (page size of 16384 bytes)'; echo 'Pages renamed:   5.'")
    done = guard("check", env=env)
    check("`vm_stat` with its labels renamed is not read as zero", done.returncode == 75 and "cannot be read" in done.stdout, done.stdout)
    done = hook_says("cargo build", scope, env=env)
    check("and a heavy command is refused", done.returncode == 2 and "cannot be read" in done.stderr, done.stderr)
    done = hook_says("touch x", scope, env=env)
    check("a light one still runs, so it can be looked at", done.returncode == 0, done.stderr)
    guard("stop", root=scope)


def case_one_process() -> None:
    scope, outside = fresh("one"), fresh("one-outside")
    big, small = holding(80, scope), holding(15, scope)
    control = holding(80, outside)
    done = watch_once(scope, {"one_process_gib": 0.05})
    check("the watcher ran once", done.returncode == 0, done.stderr)
    check("the process over its limit is stopped", settle(big))
    check("the one under it is not", up(small))
    check("a process of the same size outside the root is not", up(control))
    stopped = stopped_events(scope)
    check("what was stopped is recorded, with why", len(stopped) == 1 and stopped[0]["stopped"][0]["pid"] == big.pid and "one process" in stopped[0]["reasons"][0], stopped)
    check("and who held the most on the host, ours or not", bool(stopped) and len(stopped[0]["largest"]) == 5 and all("gib" in held for held in stopped[0]["largest"]), stopped)


def case_all_processes() -> None:
    scope = fresh("all")
    largest, other = holding(70, scope), holding(45, scope)
    watch_once(scope, {"all_processes_gib": 0.1})
    check("over the total, the largest is stopped", settle(largest))
    check("and no more than it takes", up(other))
    watch_once(scope, {"all_processes_gib": 0.1})
    check("back under the total, nothing more is stopped", up(other))


def case_what_a_stopped_process_started() -> None:
    scope, outside = fresh("tree"), fresh("tree-outside")
    file, pid_file = scope / "ready", scope / "child-pid"
    parent = spawn([*PY, "-c", PARENT, "80", str(file), str(outside), str(pid_file), "sleep 60; true"], scope)
    ready(file)
    child = int(pid_file.read_text())
    watch_once(scope, {"one_process_gib": 0.05})
    check("the process is stopped", settle(parent))
    time.sleep(0.2)
    check("and what it started, in another session and directory", not pid_up(child))


def case_another_project_is_never_ours() -> None:
    scope, elsewhere = fresh("project"), fresh("project-elsewhere")
    (scope / "README").write_text("x")
    agent_elsewhere = named("claude", elsewhere, f"bash -c 'sleep 60; true' & wait; true --add-dir {scope}")
    its_build = children(agent_elsewhere.pid)
    tmux_here = spawn(["bash", "-c", f"exec -a tmux bash -c \"(cd {elsewhere} && exec bash -c 'sleep 60; true') & wait; sleep 60\""], scope)
    time.sleep(0.15)
    build_under_tmux = children(tmux_here.pid)
    mentions = spawn(["bash", "-c", f"cat {scope}/README >/dev/null; sleep 60; true"], elsewhere)
    a_copy = spawn(["bash", "-c", f"true '{scope} copy/run.sh'; sleep 60; true"], elsewhere)
    fixture = spawn(["bash", "-c", f"true /other/fixtures{scope}/file; sleep 60; true"], elsewhere)
    check("there is something to protect", bool(its_build) and bool(build_under_tmux), (its_build, build_under_tmux))
    mine = named("cargo", scope)
    watch_once(scope, PANIC)
    check("what is ours stops", settle(mine))
    time.sleep(0.2)
    check("a build of an agent elsewhere that merely names this root is not", all(pid_up(pid) for pid in its_build))
    check("nor that agent", up(agent_elsewhere))
    check("a build elsewhere under a terminal started here is not", all(pid_up(pid) for pid in build_under_tmux))
    check("nor that terminal", up(tmux_here))
    check("a shell elsewhere that reads a file here is not", up(mentions))
    check("a path that continues past the root's name with a space is not", up(a_copy))
    check("a path that only ends with the root's is not", up(fixture))


def case_a_terminal_vouches_for_nothing() -> None:
    scope, elsewhere = fresh("terminals"), fresh("terminals-elsewhere")
    shells = []
    rc_file = elsewhere / "rc"
    rc_file.write_text("")
    for args in (
        ["bash", "--noprofile", "--norc", "-i"], ["/bin/zsh", "-f", "-il"], ["/bin/zsh", "-f", "-l"], ["/bin/sh", "-i"], ["bash", "--init-file", str(rc_file)],
        ["bash", "--rcfile", str(rc_file), "-i"], ["/bin/zsh", "-o", "no_rcs", "-i"],
    ):
        shells.append(spawn(args, scope, stdin=subprocess.PIPE))
    zsh = shells[1]
    zsh.stdin.write(f"(cd {elsewhere} && exec bash -c 'exec -a cargo sleep 60') &\n".encode())
    zsh.stdin.flush()
    time.sleep(0.4)
    its_build = children(zsh.pid)
    check("a shell typed into has started a build elsewhere", bool(its_build), its_build)
    mine = named("cargo", scope)
    watch_once(scope, PANIC)
    check("what is ours stops", settle(mine))
    time.sleep(0.2)
    for shell in shells:
        check(f"a terminal's shell does not, with no dash on its name: {shell.args}", up(shell))
    check("nor the build elsewhere under it", all(pid_up(pid) for pid in its_build))
    for shell in shells:
        shell.stdin.close()
    check("macOS's `screen`, which is `SCREEN`, is a terminal", g.protected(g.Process(1, 1, "/usr/bin/SCREEN -dmS x", "", "/usr/bin/SCREEN", 0)))
    check("a shell told a command is a job", not g.is_terminal_shell(g.Process(1, 1, "bash -lc 'x'", "", "", 0)))
    check("a shell running a script is a job", not g.is_terminal_shell(g.Process(1, 1, "bash /x/script.sh", "", "", 0)))
    check("`-tcsh` is a login shell", g.protected(g.Process(1, 1, "-tcsh", "", "", 0)))


def case_a_root_is_exactly_itself() -> None:
    base = fresh("exact")
    a, a_other = base / "A", base / "A-other"
    a.mkdir()
    a_other.mkdir()
    if not host_clear():
        not_run.append(f"another root whose name begins the same is not ours ({TAKEN_AWAY})")
        return
    job = subprocess.Popen([*PY, str(GUARD), "run", "--root", str(a_other), "--", "bash", "-c", "exec -a cargo sleep 20"], cwd=base, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    started.append(job)
    if run_refused(job, "another root whose name begins the same is not ours"):
        return
    its_job = jobs_of(job.pid)
    watch_once(a, PANIC)
    time.sleep(0.2)
    check("another root's job, whose name begins the same, is not ours", bool(its_job) and all(pid_up(pid) for pid in its_job), (its_job, job.poll(), children(job.pid)))
    guard("stop", root=a_other)
    time.sleep(0.5)
    watcher = looping_watcher(a_other, {})
    time.sleep(0.6)
    (a / "state").mkdir(exist_ok=True)
    (a / "state" / "watch.pid").write_text(str(watcher.pid))
    guard("stop", root=a)
    time.sleep(0.3)
    check("`stop` does not end another root's watcher", up(watcher))
    watcher.terminate()
    watcher.wait(timeout=5)
    os.killpg(job.pid, signal.SIGKILL)
    job.wait(timeout=10)


def case_agents() -> None:
    scope, outside = fresh("agents"), fresh("agents-outside")
    (scope / "hold.py").write_text(HOLD)
    agent = named("claude", scope, f"(cd {outside} && exec bash -c 'exec -a cargo sleep 60') & (exec bash -c 'exec -a cargo sleep 60') & wait; sleep 60")
    its_children = children(agent.pid)
    cursor = named("cursor-agent", scope)
    (scope / "aider").write_text("import time\ntime.sleep(60)\n")
    aider = spawn([*PY, str(scope / "aider")], scope)
    time.sleep(0.2)
    terminal = named("-bash", scope)
    data_file = spawn([*PY, "-c", "import time; time.sleep(60)", "data/gemini.json"], scope)
    watch_once(scope, PANIC)
    time.sleep(0.2)
    check("an agent working in the root is never stopped", up(agent))
    check("nor one named by a longer name", up(cursor))
    check("nor one an interpreter runs", up(aider))
    check("nor a login shell", up(terminal))
    alive = [pid for pid in its_children if pid_up(pid)]
    check("a build it started here is stopped, and one it started elsewhere is not", len(its_children) == 2 and len(alive) == 1, (its_children, alive))
    check("a program behind an interpreter's flag is found", g.is_agent(g.Process(1, 1, "python3 -X utf8 /x/bin/kimi", "", "", 0)) and g.is_agent(g.Process(1, 1, "node --max-old-space-size 8192 /x/bin/codex", "", "", 0)))
    check("behind a flag's file too", g.is_agent(g.Process(1, 1, "node --require ./preload.js /x/bin/claude", "", "", 0)))
    for job in ("python3 rig.py --out target/review/codex", "python3 -m pytest target/codex", "python3 -c pass target/codex", "python3 /x/rig --out target/codex"):
        check(f"a directory named after one, among a program's own arguments, makes no agent: {job}", not g.is_agent(g.Process(1, 1, job, "", "", 0)))
    check("a data file named after one is no agent", not g.is_agent(g.Process(1, 1, "python3 hold.py data/gemini.json", "", "", 0)) and not g.is_agent(g.Process(1, 1, "python3 hold.py x/CLAUDE.md", "", "", 0)))
    check("a job that merely takes such a file is a job", not g.protected(g.Process(1, 1, "python3 hold.py data/gemini.json", "", "", 0)) and up(data_file))
    # A job an agent's shell execed looks like the agent's own, and is a job: it counts, and is stopped past its limit.
    scope = fresh("agents-exec")
    (scope / "hold.py").write_text(HOLD)
    file = scope / "held"
    agent = named("codex", scope, f"{sys.executable} -I -B {scope}/hold.py 60 {file} & wait; sleep 60")
    ready(file)
    held = children(agent.pid)
    mine = g.ours(g.processes(), scope, os.getpid())
    check("a job an agent's shell execed is seen as ours", any(p.pid in held for p in mine), [p.label for p in mine])
    watch_once(scope, {"one_process_gib": 0.03})
    time.sleep(0.2)
    check("and is stopped past its limit", bool(held) and not any(pid_up(pid) for pid in held), held)
    check("and the agent is not", up(agent))


def case_what_holds_little_is_left_alone() -> None:
    scope = fresh("little")
    (scope / "hold.py").write_text(HOLD)
    parent = spawn(["bash", "-c", "(exec -a git sleep 60) & (exec -a rm sleep 60) & (exec -a vim sleep 60) & (exec -a sed sleep 60) & (exec -a find sleep 60) & (exec -a python3 sleep 60) & wait"], scope)
    time.sleep(0.3)
    small = children(parent.pid)
    check("there are small tools under a shell", len(small) == 6, small)
    builder = named("cargo", scope)
    make = named("make", scope)
    watch_once(scope, PANIC)
    check("at the danger line a build stops, however little it holds", settle(builder) and settle(make))
    time.sleep(0.2)
    check("what holds little stays, whatever it is called", all(pid_up(pid) for pid in small), small)
    check("and the shell they run under", up(parent))
    big = holding(40, scope)
    watch_once(scope, {"panic_used_percent": 0, "worth_stopping_gib": 0.02})
    check("what holds memory stops", settle(big))
    # A job holds what its parts hold, and small jobs apart hold only their own.
    scope = fresh("little-many")
    (scope / "hold.py").write_text(HOLD)
    many = spawn(["bash", "-c", f"{sys.executable} -I -B hold.py 25 a & {sys.executable} -I -B hold.py 25 b & wait"], scope)
    ready(scope / "a")
    ready(scope / "b")
    apart = holding(25, scope)
    watch_once(scope, {"panic_used_percent": 0, "worth_stopping_gib": 0.04})
    check("a job of small processes that together hold memory stops", settle(many))
    check("and a small job apart does not", up(apart))
    # What holds little stays at the stop line too, however it is called.
    reading = g.Reading(86.0, 1, 50, 1.0, 1000.0, 200.0)
    limits = g.tightened({"worth_stopping_gib": 0.05})
    git, sed = proc(1, "git status", 40), proc(2, "sed x", 20)
    targets, _, not_ours = g.judged(reading, [git, sed], limits)
    check("at the stop line nothing of ours worth stopping is said, and nothing stopped", targets == [] and not_ours, targets)
    large = proc(3, "git gc", 100)
    targets, _, _ = g.judged(reading, [git, sed, large], limits)
    check("what holds much stops, whatever it is called", tops(targets) == {3}, tops(targets))
    # Small jobs apart are the ceilings' to hold.
    swarm = [proc(i, "sleep 9") for i in range(1, 7)]
    targets, reasons, _ = g.judged(g.Reading(90.0, 1, 50, 1.0, 1000.0, 200.0), swarm, g.tightened({"processes": 2}))
    check("at the danger line too, too many processes stop", len(targets) == 4 and any("processes" in reason for reason in reasons), reasons)
    swarm = [proc(i, "sleep 9", 100) for i in range(1, 5)]
    targets, reasons, _ = g.judged(g.Reading(50.0, 1, 50, 1.0, 1000.0, 200.0), swarm, g.tightened({"all_processes_gib": 0.2}))
    check("and over the total the largest job stops, one a sample", len(targets) == 1 and "our processes hold" in reasons[0], reasons)


def case_each_line_judges_alone() -> None:
    """Every state of the host against one set of jobs: what each line asks for stops, whatever the others say, and nothing else does. A disk's reason once switched the RAM line off."""
    limits = g.tightened({})
    mine = [proc(1, "vim notes", 150), proc(2, "python3 job.py", 2048), proc(3, "cargo build", 10), proc(4, "cat big"), proc(5, "bash -c x", 2), proc(6, "sleep 9", 200, ppid=5), proc(7, "sleep 9", 200, ppid=5)]
    motion = g.Motion(written={4: 10 << 20})
    bad, states = [], 0
    for ram in (50.0, 84.9, 85.0, 87.9, 88.0, 92.0):
        for pressure in (1, 2, 4):
            for free in (1000.0, 90.0):
                for lost in (0.0, 51.0):
                    for swap in (0.0, 4.0):
                        reading = g.Reading(ram, pressure, 50, 1.0 + swap, free, 200.0)
                        baseline = g.Reading(50.0, 1, 50, 1.0, free + lost, 200.0)
                        targets, reasons, not_ours = g.judged(reading, mine, limits, baseline, motion)
                        want: set[int] = set()
                        if ram >= 88.0 or pressure == 4:
                            want |= {2, 3, 5}
                        elif ram >= 85.0 or pressure == 2 or swap:
                            want |= {2}
                        if ram >= 88.0 and swap:
                            want |= {2}
                        if free < 100.0 or lost:
                            want |= {3, 4}
                        states += 1
                        if tops(targets) != want or (bool(reasons) != bool(want)) or not_ours:
                            bad.append((ram, pressure, free, lost, swap, sorted(tops(targets)), sorted(want)))
    check(f"over {states} states of the host, each line stops what it asks for and nothing else", not bad and states == 6 * 3 * 2 * 2 * 2, bad[:4])
    # With nothing of ours worth stopping, a line reached is said as someone else's.
    small = [proc(1, "vim notes", 10), proc(2, "git status", 5)]
    for reading in (g.Reading(86.0, 1, 50, 1.0, 1000.0, 200.0), g.Reading(92.0, 4, 50, 1.0, 1000.0, 200.0), g.Reading(50.0, 1, 50, 1.0, 90.0, 200.0)):
        targets, reasons, not_ours = g.judged(reading, small, limits)
        check(f"nothing of ours stops, and the line is said: {reading.used_percent}%, {reading.repo_free_gib} GiB", targets == [] and reasons and not_ours, reasons)
    # A job's top is the highest of ours, through any depth.
    chain = [proc(1, "bash -c x", ppid=99), proc(2, "make", ppid=1), proc(3, "cargo build", ppid=2), proc(4, "rustc x", ppid=3)]
    jobs = g.jobs_of(chain)
    check("a chain of ours is one job", len(jobs) == 1 and jobs[0].top.pid == 1 and len(jobs[0].members) == 4, jobs)
    check("and a loop in what the table says ends", len(g.jobs_of([proc(1, "a", ppid=2), proc(2, "b", ppid=1)])) >= 1)
    check("what only asks, or cleans, is no build", not proc(1, "cargo clean").spawns and not proc(2, "cargo --version").spawns and not proc(3, "rustc -vV").spawns)
    check("what compiles is", proc(1, "cargo build").spawns and proc(2, "rustc --crate-name x").spawns and proc(3, "make x").spawns)
    # As the kernel shows a real one: through rustup's proxy the first word may or may not be the executable's own path.
    exe = "/u/.rustup/toolchains/x/bin/cargo"
    for shown in ("cargo clean", f"{exe} clean", "/u/.cargo/bin/cargo clean", "cargo +nightly clean", "cargo clean -p x"):
        check(f"`cargo clean` as shown: {shown}", not g.Process(1, 0, shown, "", exe, 0).spawns)
    for shown in ("cargo build", f"{exe} test -p x", "/u/.cargo/bin/cargo clippy", "cargo +nightly miri test"):
        check(f"a build as shown: {shown}", g.Process(1, 0, shown, "", exe, 0).spawns)


def case_a_job_is_stopped_whole() -> None:
    scope = fresh("whole")
    if not host_clear():
        not_run.append(f"a job is stopped whole ({TAKEN_AWAY})")
        return
    steps = scope / "steps"
    script = f"while :; do echo x >> {steps}; (exec -a cargo sleep 30); done"
    job = subprocess.Popen([*PY, str(GUARD), "run", "--root", str(scope), "--", "bash", "-c", script], cwd=scope, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    started.append(job)
    if run_refused(job, "a job is stopped whole"):
        return
    until(lambda: jobs_of(job.pid) and g.watcher_pid(scope))
    guard("stop", root=scope)
    until(lambda: not g.watcher_pid(scope))
    watcher = looping_watcher(scope, PANIC)
    check("the danger line ends the script around a build, not only the build", settle(job, 8))
    check("so it starts no other", steps.exists() and len(steps.read_text().splitlines()) == 1, steps.read_text() if steps.exists() else "")
    check("and it took one stop", len(stopped_events(scope)) == 1, stopped_events(scope))
    check("a job ended by a signal says so", job.returncode not in (0, None), job.returncode)
    watcher.terminate()
    watcher.wait(timeout=5)
    # All it started goes with it, a small `git` among them, and an agent or terminal under it does not.
    scope = fresh("whole-tree")
    parent = spawn(["bash", "-c", "(exec -a git sleep 60) & (exec -a cargo sleep 60) & (exec -a claude sleep 60) & wait"], scope)
    time.sleep(0.3)
    under = children(parent.pid)
    check("there are three under a shell", len(under) == 3, under)
    table = {p.pid: p.name for p in g.processes()}
    watch_once(scope, PANIC)
    check("the job stops", settle(parent))
    time.sleep(0.2)
    alive = [table.get(pid) for pid in under if pid_up(pid)]
    check("with all it started, short of an agent", alive == ["claude"], alive)
    clean = [pid for pid in under if pid_up(pid)]
    for pid in clean:
        os.kill(pid, signal.SIGKILL)


def case_a_queued_make_is_no_build() -> None:
    scope = fresh("queued")
    limits = json.dumps({"start_used_percent": 0})
    (scope / "Makefile").write_text(f"waits:\n\t{sys.executable} -I -B {GUARD} run --root {scope} --limits '{limits}' --wait 40 -- touch {scope}/ran\nruns:\n\t{sys.executable} -I -B {GUARD} run --root {scope} --wait 40 -- sleep 30\n")
    try:
        make = spawn(["make", "waits"], scope)
        until(lambda: g.watcher_pid(scope))
        # Its `run` started a watcher of the default lines, which holds the lock: with that left, the watcher below would read nothing and any answer would pass.
        guard("stop", root=scope)
        until(lambda: not g.watcher_pid(scope))
        mine = g.ours(g.processes(), scope, os.getpid())
        check("a real `make` waiting its turn is seen, and seen to wait", [(p.name, p.waiting, p.spawns) for p in mine] == [("make", True, False)], [(p.name, p.waiting, p.spawns) for p in mine])
        watch_once(scope, PANIC)
        time.sleep(0.2)
        check("and is no build, so it waits on at the danger line", up(make) and not (scope / "ran").exists())
        os.killpg(make.pid, signal.SIGKILL)
        make.wait()
        time.sleep(0.2)
        if not host_clear():
            not_run.append(f"a `make` whose job has started is a build ({TAKEN_AWAY})")
            return
        make = spawn(["make", "runs"], scope)
        if not until(lambda: g.watcher_pid(scope) and any(p.name == "make" and p.spawns and not p.waiting for p in g.ours(g.processes(), scope, os.getpid())), 40):
            # Its `run` was given no room in that time, which says nothing of a `make` that has started a job.
            not_run.append(f"a `make` whose job has started is a build ({TAKEN_AWAY})")
            return
        guard("stop", root=scope)
        until(lambda: not g.watcher_pid(scope))
        mine = g.ours(g.processes(), scope, os.getpid())
        check("once its job has started, it is seen to be one", any(p.name == "make" and p.spawns and not p.waiting for p in mine), [(p.name, p.waiting, p.spawns) for p in mine])
        # A watcher its own `run` started has that `make` for a parent, and must hold it all the same.
        table = g.processes()
        child = next(p.pid for p in table if p.ppid == make.pid)
        check("a process does not count the line that started it", not any(p.pid == make.pid for p in g.ours(table, scope, child)))
        check("but the watcher does: its parents are jobs like any other", any(p.pid == make.pid for p in g.ours(table, scope, child, spare_its_line=False)))
        watch_once(scope, PANIC)
        check("and stops at the danger line", settle(make))
    finally:
        guard("stop", root=scope)


def case_at_a_disk_floor_what_writes_stops() -> None:
    scope = fresh("floor")
    idle, file = scope / "wrote", scope / "finished"
    past = spawn([*PY, "-c", WROTE_THEN_IDLE, str(scope / "past"), str(idle)], scope)
    ready(idle)
    limits = {"stop_repo_free_gib": 1e9}
    watcher = looping_watcher(scope, limits)
    time.sleep(1.0)
    writer = spawn([*PY, "-c", WRITE, str(scope / "data"), str(file)], scope)
    reader = named("rm", scope)
    build = named("cargo", scope)
    cleaning = spawn(["bash", "-c", 'exec -a "cargo clean" sleep 60'], scope)
    check("at a disk floor what is writing stops, however little it holds", settle(writer, 8))
    check("and every build", settle(build))
    time.sleep(1.0)
    check("and what is not writing goes on, so disk can be freed", up(reader))
    check("`cargo clean` among it, which is how disk is freed", up(cleaning))
    check("a process that wrote before the watcher began is not counted at its first sample", up(past))
    (scope / "data").unlink(missing_ok=True)
    # A script of writers each shorter than a sample, going on for many samples: it goes whole, whichever of them is seen, or by the volume falling under it.
    loop = spawn(["bash", "-c", "for i in $(seq 1 400); do dd if=/dev/zero of=f$i bs=1048576 count=8 2>/dev/null; sleep 0.02; done; touch ended"], scope)
    began = time.monotonic()
    settled = settle(loop, 40)
    took = time.monotonic() - began
    # A script slower than a fall is one the record says goes unstopped by this line, and a loaded disk can hold it there: then this says nothing, and says that.
    written = sum(file.stat().st_size for file in scope.glob("f[0-9]*"))
    ended = (scope / "ended").exists()
    if (ended or not settled) and written / took < g.FELL_BYTES / g.FELL_WINDOW_SECONDS:
        not_run.append(f"a script of short writers is stopped whole: the host held it to {written >> 20} MiB in {took:.0f} s, under the rate a fall needs")
        if not settled:
            os.killpg(loop.pid, signal.SIGKILL)
            loop.wait()
    else:
        check("a script of short writers is stopped whole", settled, events(scope)[-1:])
        check("before it ended", not ended, f"{written >> 20} MiB in {took:.0f} s")
    watcher.terminate()
    watcher.wait(timeout=5)
    for written in scope.glob("f[0-9]*"):
        written.unlink(missing_ok=True)
    # None of it seen writing, only the volume falling and a job starting something new since the last sample.
    floor = g.Reading(50.0, 1, 50, 1.0, 90.0, 200.0)
    limits = g.tightened({})
    shell, new_child, unrelated = proc(1, "bash -c loop", ppid=99), proc(2, "dd", ppid=1), proc(3, "tail -f log", ppid=99)
    targets, _, not_ours = g.judged(floor, [shell, new_child, unrelated], limits, None, g.Motion(new={2}, repo_fell=True))
    check("a job that keeps starting what the volume falls under is stopped, and one that does not is left", tops(targets) == {1} and not not_ours, tops(targets))
    targets, _, _ = g.judged(floor, [shell, new_child, unrelated], limits, None, g.Motion(new={2}))
    check("with the volume not falling, it is not", targets == [], tops(targets))
    targets, _, _ = g.judged(floor, [shell, new_child, unrelated], limits, None, g.Motion(new={2}, boot_fell=True))
    check("nor with the other volume falling, which is not the one short", targets == [], tops(targets))
    targets, _, _ = g.judged(g.Reading(50.0, 1, 50, 1.0, 1000.0, 20.0), [shell, new_child, unrelated], limits, None, g.Motion(new={2}, boot_fell=True))
    check("the boot volume short and falling is the same", tops(targets) == {1}, tops(targets))
    targets, _, _ = g.judged(floor, [proc(1, "bash -c rm", ppid=99)], limits, None, g.Motion(new={1}, repo_fell=True))
    check("a command that has just begun is not, so disk can still be freed", targets == [], tops(targets))
    # What a shared volume costs anyway is no fall: a script of `rm` stopped for it would be a floor's way out taken away.
    def free(repo: float, boot: float = 200.0) -> g.Reading:
        return g.Reading(50.0, 1, 50, 1.0, repo, boot)
    memory = g.Memory()
    check("a first sample has no fall", not memory.moved(free(90.0), [], 0.0).repo_fell)
    little = 49 / 1024
    check("what others cost a shared volume in a sample is no fall", not memory.moved(free(90.0 - little), [], 0.5).repo_fell)
    check("nor is many such samples short of the line", not memory.moved(free(90.0 - 0.9), [], 5.0).repo_fell)
    check("a gibibyte within the window is", memory.moved(free(90.0 - 1.0), [], 9.0).repo_fell)
    check("nor does it lean on the other volume", not memory.moved(free(90.0 - 1.0), [], 9.5).boot_fell)
    memory = g.Memory()
    memory.moved(free(90.0), [], 0.0)
    check("and only within it: a slow fall is no fall", not memory.moved(free(89.0), [], g.FELL_WINDOW_SECONDS + 1.0).repo_fell)
    memory = g.Memory()
    memory.moved(free(90.0), [], 0.0)
    check("what is freed counts against a fall", not memory.moved(free(95.0), [], 1.0).repo_fell)
    # The way out of a floor, for real: a script of `rm` and of `du`, whatever else the volume is doing, short of a fall.
    scope = fresh("floor-way-out")
    for i in range(24):
        (scope / f"d{i}").mkdir()
        (scope / f"d{i}" / "f").write_bytes(b"x" * 4096)
    watcher = looping_watcher(scope, {"stop_repo_free_gib": 1e9})
    time.sleep(1.0)
    before = g.read_host(ROOT)
    cleaning = spawn(["bash", "-c", "for i in $(seq 0 23); do rm -rf d$i; du -s . >/dev/null; sleep 0.25; done; touch ended"], scope)
    ended = settle(cleaning, 30)
    after = g.read_host(ROOT)
    fell = before.repo_free_gib - after.repo_free_gib >= g.FELL_BYTES / g.GIB or before.boot_free_gib - after.boot_free_gib >= g.FELL_BYTES / g.GIB
    if fell:
        not_run.append("a script of `rm` at a floor: another project wrote a gibibyte meanwhile")
    else:
        check("at a floor, a script of `rm` and `du` runs to its end, so disk can be freed", ended and (scope / "ended").exists(), stopped_events(scope)[-1:])
    watcher.terminate()
    watcher.wait(timeout=5)


def case_a_guard_changed_holds_from_then() -> None:
    repo, copy, watcher = a_copy_watched("changed")
    good = copy.read_text()
    log = repo / "target" / "host-guard" / "state" / "events.log"
    marker = repo / "taken-up"
    copy.write_text(takes_up(good, marker))
    check("a watcher takes up a file that changes", until(lambda: marker.exists()))
    check("itself, with nothing unwatched between", up(watcher))
    check("and goes on naming its root", f"--root {g.canonical(repo)}" in marker.read_text())
    # A save while it is coming up is another test's: this one saves once it reads.
    sampled = repo / "target" / "host-guard" / "state" / "sampled"
    was = sampled.read_text()
    until(lambda: sampled.read_text() != was)
    # What a save can be seen to have broken is not taken up, and said.
    for name, broken in (("emptied", ""), ("cut short", good[: len(good) // 2].rsplit("\n", 1)[0] + "\n"), ("no longer compiling", good + "\ndef broken(:\n")):
        before = log.stat().st_size if log.exists() else 0
        copy.write_text(broken)
        check(f"one {name} is said", until(lambda: log.exists() and log.stat().st_size > before and "does not compile or is cut short" in log.read_text(errors="replace")[before:], 15))
        check(f"and goes on as it was: {name}", up(watcher))
    marker.unlink()
    copy.write_text(takes_up(good, marker))
    check("and repaired, it is taken up with nothing more", until(lambda: marker.exists()) and up(watcher))
    # One that compiles and fails ends it, as it would any: said by what looks.
    copy.write_text(good.replace("def hold_lock(", "def hold_lock_(", 1))
    check("one that fails when it runs ends the watcher", settle(watcher, 15))
    done = subprocess.run([*PY, str(GUARD), "status", "--root", str(repo)], capture_output=True, text=True, timeout=30)
    check("which is said to be gone", "watcher: not running" in done.stdout, done.stdout)


def case_a_job_under_run_outlasts_a_changed_guard() -> None:
    repo = fresh("changed-under-run")
    (repo / ".git").mkdir()
    (repo / "scripts").mkdir()
    copy = repo / "scripts" / "host_guard.py"
    shutil.copy(GUARD, copy)
    good = copy.read_text()
    job = spawn([*PY, str(copy), "run", "--", "sleep", "8"], repo)
    if run_refused(job, "a job under `run` outlasts a changed guard"):
        return
    marker = repo / "taken-up"
    copy.write_text(takes_up(good, marker))
    check("the watcher takes it up beside the job", until(lambda: marker.exists(), 20))
    check("and its job goes on to its end", settle(job, 30) and job.returncode == 0, job.returncode)
    subprocess.run([*PY, str(copy), "stop", "--root", str(repo)], capture_output=True, timeout=30)


def takes_up(good: str, marker: Path) -> str:
    """This guard, leaving `marker` with what it was started with once a watcher runs it."""
    head = "from __future__ import annotations\n"
    return good.replace(head, head + f"import sys\nif sys.argv[1:2] == ['watch']:\n    open({str(marker)!r}, 'w').write(' '.join(sys.argv))\n", 1)


def a_copy_watched(name: str, *flags: str) -> tuple[Path, Path, subprocess.Popen]:
    repo = fresh(name)
    (repo / ".git").mkdir()
    (repo / "scripts").mkdir()
    copy = repo / "scripts" / "host_guard.py"
    shutil.copy(GUARD, copy)
    watcher = spawn([*PY, str(copy), "watch", "--root", str(repo), *flags], repo)
    until(lambda: (repo / "target" / "host-guard" / "state" / "sampled").exists())
    return repo, copy, watcher


def case_a_watcher_ended_by_its_own_error_says_so() -> None:
    repo = fresh("watcher-raises")
    (repo / ".git").mkdir()
    (repo / "scripts").mkdir()
    copy = repo / "scripts" / "host_guard.py"
    in_round, at_start = repo / "raise-in-its-round", repo / "raise-at-its-start"
    source = GUARD.read_text()
    source = source.replace("def sample(root: Path, limits: Limits, memory: Memory) -> tuple[Reading, list[Process]]:\n", f"def sample(root: Path, limits: Limits, memory: Memory) -> tuple[Reading, list[Process]]:\n    if os.path.exists({str(in_round)!r}):\n        raise ValueError('from the rig')\n", 1)
    source = source.replace("def hold_lock(root: Path) -> int | None:\n", f"def hold_lock(root: Path) -> int | None:\n    if os.path.exists({str(at_start)!r}):\n        raise ValueError('from the rig')\n", 1)
    copy.write_text(source)
    state = repo / "target" / "host-guard" / "state"
    log = state / "events.log"
    watcher = spawn([*PY, str(copy), "watch", "--root", str(repo)], repo)
    until(lambda: (state / "sampled").exists())
    in_round.write_text("")
    # No line of its round ends it: said, and it goes on trying.
    check("an error in a watcher's round is said", until(lambda: log.exists() and "from the rig" in log.read_text(errors="replace"), 15))
    time.sleep(2.5)
    check("and it goes on", up(watcher))
    in_round.unlink()
    before = (state / "sampled").read_text()
    check("and reads again once it can", until(lambda: (state / "sampled").read_text() != before, 15))
    subprocess.run([*PY, str(copy), "stop", "--root", str(repo)], capture_output=True, timeout=30)
    settle(watcher, 15)
    # Before its round begins there is nothing to go on with: it says so, and ends.
    log.unlink()
    at_start.write_text("")
    again = spawn([*PY, str(copy), "watch", "--root", str(repo)], repo)
    check("a watcher that cannot start says so", settle(again, 15) and log.exists() and "the watcher ended: ValueError('from the rig')" in log.read_text(errors="replace"))


def case_a_root_spelt_relatively_stays_that_root() -> None:
    repo = fresh("relative-root")
    (repo / ".git").mkdir()
    (repo / "scripts").mkdir()
    (repo / "sub").mkdir()
    copy = repo / "scripts" / "host_guard.py"
    shutil.copy(GUARD, copy)
    watcher = spawn([*PY, str(copy), "watch", "--root", "sub"], repo)
    sampled = repo / "sub" / "state" / "sampled"
    try:
        check("a watcher named a relative root reads for it", until(sampled.exists))
        before = sampled.read_text()
        check("and goes on reading for it, from wherever it went", until(lambda: sampled.read_text() != before) and up(watcher))
    finally:
        subprocess.run([*PY, str(copy), "stop", "--root", str(repo / "sub")], capture_output=True, timeout=30)


def case_make_looks_for_the_whole_guard() -> None:
    repo = fresh("make-asks")
    (repo / ".git").mkdir()
    (repo / "scripts").mkdir()
    shutil.copy(ROOT / "Makefile", repo / "Makefile")
    copy = repo / "scripts" / "host_guard.py"
    shutil.copy(GUARD, copy)
    good = copy.read_text()
    tiers = lambda *flags: subprocess.run(["make", *flags, "tiers"], cwd=repo, capture_output=True, text=True, errors="replace", timeout=60, stdin=subprocess.DEVNULL)
    done = tiers("-n")
    check("with the guard whole, `make` goes on to its recipe", done.returncode == 0 and "host_guard.py run" in done.stdout, done)
    for name, broken in (("emptied", ""), ("cut short", good[: len(good) // 2].rsplit("\n", 1)[0] + "\n")):
        copy.write_text(broken)
        done = tiers("-n")
        check(f"with one {name}, `make` runs nothing and says so", done.returncode != 0 and "is not whole" in done.stderr and "host_guard.py run" not in done.stdout, done)
    # Broken so as to raise, it exits with its error, so nothing reads as passed. One that runs and does nothing some other way is no accident `make` looks for.
    copy.write_text(good.replace("from __future__ import annotations\n", "from __future__ import annotations\ndef broken(:\n", 1))
    check("with one that fails, `make` fails", tiers().returncode != 0)


def case_a_root_remade_is_another() -> None:
    base = fresh("root-remade")
    scope = base / "scope"
    scope.mkdir()
    watcher = looping_watcher(scope, {})
    until(lambda: (scope / "state" / "sampled").exists())
    held = scope.with_name("held")
    scope.rename(held)
    scope.mkdir()
    check("a watcher of a root remade at its name ends", settle(watcher, 10))
    shutil.rmtree(held)


def case_a_root_that_is_gone_stays_gone() -> None:
    base = fresh("root-gone")
    scope = base / "scope"
    scope.mkdir()
    watcher = looping_watcher(scope, {})
    until(lambda: (scope / "state" / "sampled").exists())
    shutil.rmtree(scope)
    check("its watcher ends", settle(watcher, 10))
    time.sleep(0.5)
    check("and does not bring it back", not scope.exists())
    report = subprocess.run([*PY, str(GUARD), "run", "--root", str(scope), "--", "true"], capture_output=True, text=True, timeout=30)
    check("and a `run` there says so and runs nothing", report.returncode == 75 and "gone" in report.stderr, report.stderr)


def case_growth_is_of_the_last_minutes() -> None:
    def at(swap: float) -> g.Reading:
        return g.Reading(50.0, 1, 50, swap, 1000.0, 200.0)

    history: deque = deque()
    check("with nothing of ours running there is no count", g.baseline_of(history, 0.0, at(1.0), False) is None)
    check("it starts when ours does", g.baseline_of(history, 10.0, at(2.0), True).swap_used_gib == 2.0)
    g.baseline_of(history, 20.0, at(9.0), True)
    check("and holds through the window", g.baseline_of(history, 500.0, at(9.0), True).swap_used_gib == 2.0)
    check("and no further: a long-lived small process does not make it hours", g.baseline_of(history, 20.0 + g.GROWTH_WINDOW_SECONDS, at(9.0), True).swap_used_gib == 9.0)
    limits = g.tightened({})
    build = g.Process(2, 1, "cargo build", "", "", 1 << 30)
    targets, reasons, _ = g.judged(at(9.0), [build], limits, at(2.0))
    check("growth past the line stops ours, and says so", [job.top for job in targets] == [build] and "swap grew" in reasons[0], reasons)
    targets, _, _ = g.judged(at(9.0), [build], limits, None)
    check("with the count begun again, the same reading does not", targets == [])


def case_where_a_root_is() -> None:
    base = fresh("where")
    main, linked, other = base / "main", base / "linked", base / "other"
    (main / ".git").mkdir(parents=True)
    linked.mkdir()
    (linked / ".git").write_text("gitdir: elsewhere")
    (other / ".git").mkdir(parents=True)
    (other / "vendored").mkdir()
    (other / "vendored" / ".git").write_text("gitdir: elsewhere")
    check("a checkout holds what is under it", g.root_among([main / "target" / "wt", main]) == main)
    check("a linked worktree is its own root", g.root_among([linked / "scripts", linked, base]) == linked)
    check("a submodule or worktree inside another project never takes that project", g.root_among([other / "vendored" / "scripts", other / "vendored", other]) == other / "vendored")
    check("and keeps its state under `target/`", g.state_path(linked) == linked / "target" / "host-guard" / "state")
    check("nothing is no root", g.root_among([base]) is None)


def case_names_and_zombies() -> None:
    p = g.Process(1, 1, "/Library/Application Support/x/foo --a", "", "/Library/Application Support/x/foo", 0)
    check("a path with a space does not cut a process's name short", p.name == "foo", p.name)
    p = g.Process(1, 1, "/usr/bin/python3 -I -B /x/rig.py a", "", "/usr/bin/python3", 0)
    check("an interpreter goes by what it runs", p.label == "python3 rig.py", p.label)
    scope = fresh("zombie")
    parent = spawn([*PY, "-c", "import subprocess, time; subprocess.Popen(['true']); time.sleep(60)"], scope)
    time.sleep(0.4)
    check("a zombie is not in the table", not any(p.ppid == parent.pid for p in g.processes()), [p.command for p in g.processes() if p.ppid == parent.pid])
    files = [scope / f"ignores-{i}" for i in range(4)]
    for file in files:
        spawn([*PY, "-c", IGNORE, str(file)], scope)
        ready(file)
    began = time.monotonic()
    watch_once(scope, {"panic_used_percent": 0, "worth_stopping_gib": 0.001})
    check("processes that ignore being asked cost one wait, not one each", time.monotonic() - began < 3.0, time.monotonic() - began)


def case_a_command_line_that_is_no_text() -> None:
    scope = fresh("no-text")
    strange = spawn(["bash", "-c", "exec -a $'x\\xe9y' sleep 60"], fresh("no-text-elsewhere"))
    time.sleep(0.15)
    big = holding(80, scope)
    watch_once(scope, {"one_process_gib": 0.05})
    check("a command line that is no text does not blind the watcher", settle(big), events(scope))
    check("and is not an error", not any(event["kind"] == "unread" for event in events(scope)), events(scope))
    check("and that process is left alone", up(strange))


def case_pressure() -> None:
    scope = fresh("pressure")
    one = holding(20, scope)
    watch_once(scope, {"stop_pressure_level": 0, "worth_stopping_gib": 0.001})
    check("macOS's pressure level stops our largest", settle(one))
    scope = fresh("pressure-critical")
    two, three = holding(15, scope), holding(15, scope)
    watch_once(scope, {"panic_pressure_level": 0, "worth_stopping_gib": 0.001})
    check("at its critical level all that holds memory stops", settle(two) and settle(three))


def case_not_ours() -> None:
    scope = fresh("not-ours")
    small = holding(15, scope)
    watch_once(scope, {"stop_used_percent": 0})
    check("pressure with little of ours stops nothing", up(small))
    check("and is recorded as someone else's", any(event["kind"] == "pressure-not-ours" for event in events(scope)), events(scope))
    watch_once(scope, {"panic_used_percent": 0})
    check("at the danger line too", up(small))
    watch_once(scope, {"stop_used_percent": 0, "worth_stopping_gib": 0.001})
    check("pressure with something of ours worth stopping stops it", settle(small))


def case_growing() -> None:
    scope = fresh("growing")
    file = scope / "ready"
    grower = spawn([*PY, "-c", GROW, str(file)], scope)
    watcher = looping_watcher(scope, {"one_process_gib": 0.1})
    check("a process that keeps growing is stopped while it grows", settle(grower, 15))
    check("before it reaches what it was going for", not file.exists())
    deadline = time.monotonic() + 5
    stopped = []
    while not stopped and time.monotonic() < deadline:
        stopped = stopped_events(scope)
        time.sleep(0.05)
    check("within a sample or two of the line", bool(stopped) and stopped[0]["stopped"][0]["gib"] < 0.3, stopped)
    watcher.terminate()
    watcher.wait(timeout=5)


def case_a_small_writer_is_counted() -> None:
    scope = fresh("writer")
    file = scope / "done"
    writer = spawn([*PY, "-c", WRITE, str(scope / "data"), str(file)], scope)
    watcher = looping_watcher(scope, {"stop_disk_growth_gib": 0.03})
    check("a small process writing past the disk line is stopped", settle(writer, 15))
    check("before it finished", not file.exists())
    watcher.terminate()
    watcher.wait(timeout=5)
    (scope / "data").unlink(missing_ok=True)


def case_the_watcher_outlives_its_own_trouble() -> None:
    scope = fresh("survives")
    limits = {"one_process_gib": 0.05}
    watcher = looping_watcher(scope, limits)
    time.sleep(1.5)
    check("it starts", up(watcher) and (scope / "state" / "watch.pid").exists())
    shutil.rmtree(scope / "state")
    big = holding(80, scope)
    check("with its state removed, as `cargo clean` does, it still stops what it should", settle(big, 6))
    check("and lives", up(watcher))
    (scope / "state").mkdir(exist_ok=True)
    for bad in ("[]", "{}", "null", "not json"):
        (scope / "state" / "peak.json").write_text(bad)
        time.sleep(1.2)
        check(f"a peak file of `{bad}` does not kill it", up(watcher))
    log = scope / "state" / "events.log"
    log.touch()
    log.chmod(stat.S_IRUSR)
    big = holding(80, scope)
    check("with its log closed to it, it still stops what it should", settle(big, 6))
    check("and lives", up(watcher))
    log.chmod(stat.S_IRUSR | stat.S_IWUSR)
    said = guard("status", root=scope).stdout
    check("`status` says it reads the host", "watcher: running, and reading the host" in said, said)
    # One lock, on the root itself: nothing under `target/` can take it away and leave two.
    shutil.rmtree(scope / "state")
    for _ in range(3):
        guard("ensure", root=scope)
    time.sleep(1.5)
    found = subprocess.run(["pgrep", "-f", f"host_guard.py watch --root {scope}"], capture_output=True, text=True).stdout.split()
    check("however often it is asked for, one stays", found == [str(watcher.pid)], (found, watcher.pid))
    watcher.terminate()
    watcher.wait(timeout=5)
    # With nothing it can write, stderr on the same volume among it, it still stops what it should.
    scope = fresh("survives-full")
    big = holding(80, scope)
    full = spawn(["bash", "-c", f"ulimit -f 0; exec {sys.executable} -I -B {GUARD} watch --root {scope} --limits '{json.dumps(limits)}'"], scope)
    check("with no write possible, it still stops what it should", settle(big, 8))
    check("and lives", up(full))


def case_a_stale_pid_names_nobody() -> None:
    scope = fresh("stale")
    innocent = spawn(["sleep", "30"], fresh("stale-elsewhere"))
    (scope / "state").mkdir()
    (scope / "state" / "watch.pid").write_text(str(innocent.pid))
    done = guard("stop", root=scope)
    time.sleep(0.2)
    check("`stop` does not signal a process that is no watcher", done.returncode == 0 and up(innocent), done)
    check("`status` does not call it one", "watcher: not running" in guard("status", root=scope).stdout)


def case_the_watcher_stays_while_a_job_runs() -> None:
    scope = fresh("stays")
    job = named("something", scope)
    code = f"import sys; sys.path.insert(0, {str(HERE)!r}); import host_guard as g; from pathlib import Path; g.watch(Path({str(scope)!r}), g.tightened({{}}), False, idle_seconds=1.5)"
    watcher = spawn([*PY, "-c", code], ROOT)
    time.sleep(3.5)
    check("a small job keeps its watcher past the idle time", up(watcher))
    os.killpg(job.pid, signal.SIGKILL)
    job.wait()
    check("and it ends once nothing of ours runs", settle(watcher, 6))


def case_what_is_not_the_table_is_blindness() -> None:
    scope = fresh("no-table")
    for name, script in (("nothing", "exit 0"), ("cut short", '/bin/ps "$@" | head -3')):
        env = shim(scope, "ps", script)
        job = named("cargo", scope)
        subprocess.run([*PY, str(GUARD), "watch", "--once", "--root", str(scope), "--limits", json.dumps(PANIC)], env=env, capture_output=True, timeout=30)
        check(f"a `ps` that answers {name} is no sample", any(event["kind"] == "unread" for event in events(scope)), events(scope))
        (scope / "state" / "events.log").unlink(missing_ok=True)
        os.killpg(job.pid, signal.SIGKILL)
        job.wait()
    limits = g.tightened({})
    state = fresh("future") / "state"
    state.mkdir()
    (state / "sampled").write_text("")
    os.utime(state / "sampled", (time.time() + 3600, time.time() + 3600))
    check("a time in the future is no proof of having looked", not g.healthy(state.parent, limits))
    code = f"import sys; sys.path.insert(0, {str(HERE)!r}); import host_guard as g; from pathlib import Path; g.watch(Path({str(scope)!r}), g.tightened({{}}), False, idle_seconds=4.0)"
    env = shim(scope, "ps", '[ -f "' + str(scope) + '/flag" ] && exit 1; exec /bin/ps "$@"')
    job = named("something", scope)
    watcher = spawn([*PY, "-c", code], ROOT, env=env)
    time.sleep(1.2)
    (scope / "flag").write_text("")
    time.sleep(3.5)
    check("a watcher gone blind does not leave while it last saw a job", up(watcher))
    check("nor stays for it longer than it would stay idle", settle(watcher, 30))


def case_a_blind_watcher_is_said() -> None:
    scope = fresh("blind")
    env = shim(scope, "ps", "exit 1")
    watcher = looping_watcher(scope, {}, env=env)
    time.sleep(2.0)
    (scope / "state").mkdir(exist_ok=True)
    pid_file = scope / "state" / "watch.pid"
    check("it is alive", up(watcher) and pid_file.exists())
    said = guard("status", root=scope, limits={"watcher_stale_seconds": 1})
    check("`status` says it is blind", "BLIND" in said.stdout, said.stdout)
    done = hook_says("cargo build", scope, {"watcher_stale_seconds": 1})
    check("a heavy command is refused", done.returncode == 2 and "no watcher has read" in done.stderr, done.stderr)
    unread = [event for event in events(scope) if event["kind"] == "unread"]
    check("and it does not fill its log saying so", len(unread) == 1, unread)
    watcher.terminate()
    watcher.wait(timeout=5)


def heavy_text(command: str) -> bool:
    return any(g.is_heavy(each.words) for each in g.commands_of(command))


def case_what_counts_as_heavy() -> None:
    for heavy in (
        "cargo test -p x", "make -C other all", "cd a && cargo build", "FOO=1 cargo build", "timeout 9 cargo test", "for t in a; do cargo test; done", "bash -c 'cargo build'",
        "if cargo build; then :; fi", "env cargo build", "nohup cargo build", "{ cargo build; }", "`cargo build`", "gmake x", "cargo-nextest run", "bash scripts/build_book.sh",
        "target/m/install/archogen build s --out d", "archogen-plan-check --result r s p", "# let's build\ncargo build\n# it's done", 'out="$(cargo build 2>&1)"', "caffeinate -i cargo test",
        "/usr/bin/time -l cargo build", "rustup run stable cargo build", "zsh -c 'cargo build'", "bash -lc 'cargo build'", "sudo cargo build", "env -i cargo build", "cargo +nightly miri test",
        "cargo --locked build", "cargo --config x build", "make", "make -f other.mk focused", "qemu-system-riscv32 -M virt", "mdbook build", "rustc x.rs", "xargs cargo build", "eval 'cargo build'",
        "cargo test -- --help", "cargo run -- --version", "make --jobs 4 other", "bash -euo pipefail scripts/build_book.sh", "bash -c -- 'cargo build'", "bash -euo pipefail -c 'cargo build'", "bash <<'E'\ncargo build\nE",
        "bash -s -- x <<'E'\ncargo build\nE", "bash -s x <<< 'cargo build'", "bash /dev/stdin <<'E'\ncargo build\nE", "source /dev/stdin <<'E'\ncargo build\nE", 'x="$(cat <<EOF\nbody\nEOF)" && cargo build',
        "cargo xtask --help", "cargo xtask verify --help", "make -Cvendor all", "make -f x.mk all", "env -S'cargo build'", "env -vS 'cargo build'", 'x="$(cat <<EOF\nbody\nEOF )" && cargo build', "case q in q) ls;;#it's\nesac; cargo build", "x=(#it's\n b); cargo build",
        "a[0]=x cargo build", "echo ${x:-'}'} ; cargo build", "env -S 'cargo build'", "coproc name { cargo build; }", "{fd}>log cargo build", "[[ a == a ]]>log; cargo build", "if [[ '#c' =~ ^([[:space:]]*)#(.*)$ ]]; then cargo build; fi",
        "bash <<< 'cargo build'", "trap 'cargo build' EXIT", "echo $(( $(cargo build) + 1 ))", 'x="$(if true; then case y in y) cargo build;; esac; fi)"', "x=( $(cargo build) )", "[[ -n $(cargo metadata --format-version 1 >x; cargo build) ]]",
    ):
        check(f"heavy: {heavy}", heavy_text(heavy))
    for light in (
        "ls", "cargo fmt -p x", "git commit -F m", "grep -n 'make ' Makefile", "git commit -m 'make cargo build'", "echo cargo build", "echo archogen-plan-check",
        "grep -rn archogen-plan-check docs/", "git commit -m 'x (make sure it holds)'", "git commit -F - <<'M'\nmake sure it holds\nM\n",
        "R=/Volumes/SSD/Documents/github/archogen; mkdir -p x", "X=/y/archogen ls", "cargo --version", "rustc -V", "cargo tree", "cargo metadata", "make help", "make hooks", "make -n focused",
        "make focused", "mdbook --version", "python3 -c \"print('make')\"", "rg -c 'cargo' Cargo.toml", "scripts/foo.sh make", "file target/m/install/archogen", "ls # first; make sure of it",
        "cargo clean", "cargo clean -p x", "make focused>log 2>&1", "make focused;echo done", "make -j4 focused", "make -j 4 focused", "make focused integration", "command -v cargo",
        "python3 scripts/host_guard.py run --wait 1800 -- bash -c 'cd x && cargo test'", "python3 scripts/host_guard.py run --wait 1800 -- env RUST_BACKTRACE=1 cargo test",
        "python3 scripts/host_guard.py run --wait 1800 -- bash scripts/build_book.sh", "bash -c 'python3 scripts/host_guard.py run -- cargo test'", "case x in cargo) ls;; esac", "for x in cargo make; do ls; done",
        "rustc --explain E0308", "cargo build --help", "cargo update -p serde", "cargo add serde", "cargo remove serde", "cargo info serde", "cargo clippy --version", "cargo generate-lockfile", "cargo fetch", "mdbook help",
        "qemu-system-riscv32 --version", "archogen --help", "archogen help build", "archogen-plan-check --help", "make --jobs 4 focused", "names=(rustc cargo); echo ${names[0]}", "(( make > 1 ))", "((make>1)) && ls",
        "[[ $x == cargo ]]", "[[ $x =~ ^(cargo|make)$ ]] && ls", "case $x in cargo|make) ls;; esac", "cargo() { ls; }", "function make { ls; }",
        'make -j "$N" focused', "make --version", "make --help", "gmake -v", "make -h", "make -qp", "make -ns all", "make -nj4 all", "make -j4 help", "make -C sub -n", "make -f x.mk --help", "env ls", "kill -TERM 1",  "for x in a; do make() { ls; }; done", "if true; then cargo() { :; }; fi", "bash -n -c 'cargo build'", "sh -n <<'E'\ncargo build\nE", "zsh -n <<'E'\ncargo build\nE", "command -pv cargo",
    ):
        check(f"not heavy: {light!r}", not heavy_text(light))


# Each of these runs in the real shell, under a `PATH` of stubs and nothing else, so what is named here is never the real program. So none may name a dangerous program by its whole
# path, ask for a login shell, which may set its own `PATH`, or clear the environment.
THE_SHELL_RUNS_THESE = (
    "cargo build", "cd x && cargo build", "FOO=1 cargo build", "timeout 9 cargo test", "for t in a; do cargo test; done", "bash -c 'cargo build'", 'sh -c "cargo build"', "if cargo build; then :; fi",
    "env cargo build", "env FOO=1 cargo build", "env -u FOO cargo build", "nohup cargo build >/dev/null 2>&1", "nice -n 5 cargo build", "{ cargo build; }", "(cargo build)", "(cd x && cargo build)",
    "`cargo build`", "$(cargo build)", 'out="$(cargo build 2>&1)"', "out=`cargo build`", 'echo "$(cargo build)"', "echo ${X:-$(cargo build)}", 'echo "${X:-$(cargo build)}"', "echo $((1 + 2)); cargo test",
    "! cargo build", "time cargo build", "time -p cargo build", "command cargo build", "exec cargo build", "caffeinate -i cargo test", "/usr/bin/time -l cargo build", "rustup run stable cargo build",
    "eval 'cargo build'", "cargo build & wait", "cargo build; make focused", "cargo build && make focused", "cargo build | cat", "cat <(cargo build)", "ca\\rgo build", "'cargo' \"build\"", "cargo bu''ild", "$'cargo' build",
    "# let's build\ncargo build\n# it's done", "# don't\npkill -x rustc\n# don't", "cargo build # make sure", "cargo \\\n build", "cat <<EOF\n$(cargo build)\nEOF", "cat <<-EOF\n\t$(cargo build)\n\tEOF",
    "cat <<'EOF'\n$(cargo build)\nEOF", "cat <<\\EOF\n$(cargo build)\nEOF", "cat <<'A' <<'B'\nmake\nA\ncargo\nB\nls", 'cat <<<"$(cargo build)"', "make focused>log 2>&1", "make focused;echo done", "make -j4 focused",
    "echo x >log; ls", "2>err cargo build", "cargo build 2>&1 >/dev/null", "pkill -x rustc", "(pkill -x rustc)", "$(pkill -x rustc)", "bash -c 'pkill -x rustc'", "sh -c 'pgrep -x rustc | xargs kill'",
    "kill 999999", "kill -9 999999", "kill $(pgrep -x rustc)", 'kill "$(pgrep -x rustc)"', "kill `pgrep -x rustc`", 'for p in $(pgrep -x rustc); do kill "$p"; done', "pids=$(pgrep -x rustc); kill $pids",
    "pgrep -x rustc | while read p; do kill $p; done", "pgrep -x rustc | xargs kill", "ps -ax | awk '/rustc/{print $1}' | while read p; do kill $p; done", "for p in $(ps -ax | grep rustc | awk '{print $1}'); do kill $p; done",
    "cat job.pid | xargs kill", 'grep -n "kill\\|pgrep" f', 'rg "xargs kill" scripts/', 'git commit -m "kill the pgrep rig"', "git commit -m 'make cargo build'", "git commit -m 'x (make sure it holds)'",
    "git commit -F - <<'M'\nmake sure it holds\nM\n", "echo cargo build", "echo 'pkill -x rustc'", "echo '$(cargo build)'", "echo \\$\\(cargo build\\)", 'echo "use kill, not pgrep"', "python3 -c \"print('make')\"",
    "python3 scripts/host_guard.py run --wait 1800 -- bash -c 'cd x && cargo test'", "X=/y/archogen ls", "file target/m/install/archogen", "find . -name pkill -print", "ls # first; make sure of it",
    "case cargo in cargo) ls;; esac", "case cargo in\n  cargo) ls ;;\nesac", "for x in cargo make; do ls; done", "f() { ls; }; f",
    # What round 6's reader found parting them: a case inside a substitution, arithmetic, a shell's options, what runs later, and what is only text.
    "case y in (y) cargo build;; esac; ls", "case y in\n# it's a comment\ny) cargo build ;;\nesac\nls", "(( y = 1 << 3 ))\npkill -x rustc", "((x<<=1)); cargo build", "echo $[1<<2]; cargo build", "echo $((1<<2)); cargo build",
    "echo $(( $(cargo build) + 1 ))", 'echo "$(( $(cargo build) + 1 ))"', "for ((i=0; i<1; i++)); do cargo build; done", "((cargo build); ls)", "bash -euo pipefail -c 'pkill -x rustc'", "bash -eo pipefail -c 'cargo build'", "bash -c -- 'cargo build'",
    "bash -c -e 'cargo build'", "bash -o pipefail -c 'cargo build'", "trap 'pkill -x rustc' EXIT", "trap 'cargo build' EXIT; ls", "bash <<'EOF'\npkill -x rustc\nEOF", "bash <<EOF\ncargo build\nEOF", "bash -s <<'EOF'\nmake focused\nEOF",
    "bash <<< 'cargo build'", "sh <<'EOF'\ncargo build\nEOF", "tools=(pkill killall); echo ${tools[0]}", "names=(rustc cargo); echo ${names[0]}", "x=( $(cargo build) )", "x+=(a b); ls", "(( make > 1 ))", "((make>1)); ls",
    "[[ a < b && c > d || e == f ]]; ls", "[[ -z $(cargo build) ]]; ls", "if [[ -n $(ls) ]]; then cargo build; fi", "for t in a b; do case $t in a) ls;; b) cargo build;; esac; done",
    "pkill() { :; }; ls", "function make { :; }; ls", "function cargo() { :; }; ls",
    # What round 9's reader found parting them.
    "for ((i=0;i<1;i++)) do cargo build; done", "for ((i=0;i<1;i++)) { cargo build; }",
    "if true; then for ((i=0;i<1;i++)) do cargo build; done; fi", "while for ((i=0;i<1;i++)) do cargo build; done; false; do :; done", "{ for ((i=0;i<1;i++)) do cargo build; done; }", "time for ((i=0;i<1;i++)) do cargo build; done",
    # What round 8's reader found parting them.
    "env -vS 'cargo build'", 'x="$(cat <<EOF\nbody\nEOF )" && cargo build', "case q in q) ls;;#it's\nesac; cargo build", "x=(#it's\n b); cargo build", "[[ x =~ a#b ]]; cargo build",
    # What round 7's reader found parting them.
    "if [[ '#c' =~ ^([[:space:]]*)#(.*)$ ]]; then cargo build; fi", "[[ a == a ]]>log; cargo build", "bash -s -- x <<'E'\ncargo build\nE", "bash -s x <<< 'cargo build'", "bash /dev/stdin <<'E'\ncargo build\nE",
    'x="$(cat <<EOF\nbody\nEOF)" && cargo build', "a[0]=x cargo build", "echo ${x:-'}'} ; cargo build", "env -S 'cargo build'", "for x in a; do g() { :; }; done; ls", "if true; then h() { :; }; fi; ls", "bash -n -c 'cargo build'; ls", "command -pv cargo; ls",
)
# Those the shell may not reach: what it runs must still be what is read, and what is read may be more.
THE_SHELL_MAY_NOT_REACH = ("ls || cargo build", "false && cargo build", "if false; then cargo build; else make focused; fi", "while false; do cargo build; done", "case x in y) cargo build;; esac", "case x in cargo) make focused;; esac", "f() { cargo build; }", "xargs cargo build </dev/null",
    'arch="$(case "$(ls)" in arm64) touch a;; *) file b;; esac)"; cargo build', 'case "$t" in a) ls;; pkill|killall) make focused;; esac', "case y in\nz) make x ;;\nesac\nls", "case y in z) cargo build; esac", "case y in z) cat <<E ;; esac\nmake\nE\nmake focused",
    "[[ $x =~ ^(pkill|killall)$ ]] && cargo build")
# A bare `)` of a case inside a substitution: bash 3.2 takes it for the substitution's end, as its own bug, so it runs less than bash 4 and later do, which the reader reads. Reading more is the safe side.
ONLY_LATER_BASH_RUNS_THESE = (
    'x="$(case y in y) cargo build;; esac)"; make focused', 'x="$(case y in y) cargo build;; esac )"; make focused', 'x="$(if true; then case y in y) cargo build;; esac; fi)"', 'x=$(case y in y) cargo build; esac); ls',
    "coproc name { cargo build; }", "{fd}>log cargo build",
)
# Every way of standing, around everything that may stand there.
HOW_IT_STANDS = ("%s", "(%s)", "$(%s)", "`%s`", "bash -c '%s'", 'sh -c "%s"', "env %s", "nohup %s", "x=$(%s)", "{ %s; }", "if %s; then :; fi", "! %s", "time %s", 'echo "$(%s)"', "# it's\n%s\n# it's", "true; %s", "true && %s", "true | %s", "eval '%s'")
WHAT_STANDS = ("cargo build", "make -j4 focused>log", "pgrep -x rustc | xargs kill", "git commit -m '(make) x'", 'ls "a b"', "kill 999999")
STUBS = ("cargo", "make", "gmake", "rustc", "mdbook", "archogen", "pkill", "killall", "kill", "pgrep", "ps", "grep", "awk", "rg", "git", "python3", "sleep", "ls", "cat", "touch", "mkdir", "file", "find")


def every_bash() -> list[str]:
    """Each `bash` there is here: the system's, an old one, and the first on the path, which is what a harness may run a command with."""
    found = ["/bin/bash"]
    onward = shutil.which("bash")
    if onward and os.path.realpath(onward) != os.path.realpath(found[0]):
        found.append(onward)
    return found


def case_the_reader_sees_what_the_shell_runs() -> None:
    for bash in every_bash():
        the_reader_against(bash)


def the_reader_against(bash: str) -> None:
    """The oracle no code here wrote: the shell itself. Each form runs under stubs that write down how they were called, and the reader must name exactly those."""
    scope = fresh("shell")
    stubs, work, log = scope / "stubs", scope / "work", scope / "ran"
    stubs.mkdir()
    (work / "x").mkdir(parents=True)
    for name in STUBS:
        (stubs / name).write_text("#!/bin/sh\nsep=$(printf '\\037')\nline=\"${0##*/}\"\nfor word in \"$@\"; do line=\"$line$sep$word\"; done\nprintf '%s\\n' \"$line\" >> \"$RAN_LOG\"\nprintf '999999\\n'\n")
    for name, real in (("bash", bash), ("sh", "/bin/sh"), ("env", "/usr/bin/env"), ("nohup", "/usr/bin/nohup"), ("nice", "/usr/bin/nice"), ("xargs", "/usr/bin/xargs"), ("caffeinate", "/usr/bin/caffeinate")):
        (stubs / name).write_text(f'#!/bin/sh\nexec {real} "$@"\n')
    (stubs / "timeout").write_text('#!/bin/sh\nshift\nexec "$@"\n')
    (stubs / "rustup").write_text('#!/bin/sh\nshift 2\nexec "$@"\n')
    for stub in stubs.iterdir():
        stub.chmod(0o755)
    (scope / "env").write_text("enable -n kill\n")
    environment = {"PATH": str(stubs), "RAN_LOG": str(log), "BASH_ENV": str(scope / "env"), "HOME": str(scope)}

    def ran(command: str) -> set[tuple[str, ...]]:
        log.write_text("")
        # A shell does not wait for a process substitution, so under load its log was read before `cat <(cargo build)` had written: 6 of 150 on one bash, measured. Everything the shell
        # starts holds this descriptor, whatever it redirects, so that it closing says the last of them has ended.
        reader, writer = os.pipe()
        try:
            shell = subprocess.Popen([bash, "-c", command], cwd=work, env=environment, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, pass_fds=(writer,))
            os.close(writer)
            writer = -1
            select.select([reader], [], [], 20)
            shell.wait(timeout=20)
        finally:
            os.close(reader)
            if writer >= 0:
                os.close(writer)
        return {tuple(line.split("\x1f")) for line in log.read_text().splitlines()}

    def read(command: str) -> tuple[set[str], set[tuple[str, ...]], set[tuple[str, ...]]]:
        """The names read, how each is called where no expansion can change it, and how each is called as read, an awk program's `$1` among what no expansion touches."""
        found = [each for each in g.commands_of(command) if each.name in STUBS and not each.fed]
        every = {(each.name, *each.words[1:]) for each in found}
        whole = {call for call in every if not any(mark in word for word in call for mark in "$`…")}
        return {each.name for each in g.commands_of(command) if each.name in STUBS}, whole, every

    def what_it_shows(shown: set[tuple[str, ...]]) -> set[tuple[str, ...]]:
        # What a substitution fed it is not in the command's text.
        return {call for call in shown if not any("999999" in word or "/dev/fd/" in word for word in call)}

    version = int(subprocess.run([bash, "-c", "echo ${BASH_VERSINFO[0]}"], capture_output=True, text=True, timeout=20).stdout.strip())
    may_not_reach = THE_SHELL_MAY_NOT_REACH if version >= 4 else THE_SHELL_MAY_NOT_REACH + ONLY_LATER_BASH_RUNS_THESE
    forms = list(THE_SHELL_RUNS_THESE) + (list(ONLY_LATER_BASH_RUNS_THESE) if version >= 4 else [])
    for standing in HOW_IT_STANDS:
        for standing_there in WHAT_STANDS:
            if "'" in standing_there and "'" in standing:
                continue
            if '"' in standing_there and '"' in standing:
                continue
            # `kill` is a builtin of `sh`, which no stub can stand in for, so that one would be the real thing.
            if standing.startswith("sh -c") and standing_there.startswith("kill"):
                continue
            forms.append(standing % standing_there)
    disagree = []
    for form in forms:
        shown = ran(form)
        names, whole, every = read(form)
        if names != {call[0] for call in shown} or not whole <= shown or not what_it_shows(shown) <= every:
            disagree.append((form, sorted(names), sorted(shown), sorted(every)))
    check(f"over {len(forms)} forms the reader names exactly what {bash} ran, and how", not disagree, disagree[:8])
    disagree = []
    for form in may_not_reach:
        shown = ran(form)
        names = read(form)[0]
        if not {call[0] for call in shown} <= names:
            disagree.append((form, sorted(names), sorted(shown)))
    check(f"and what {bash} may not reach is read all the same", not disagree, disagree[:3])
    check("each of those is read to hold what it may run", all("cargo" in read(form)[0] or "make" in read(form)[0] for form in may_not_reach), may_not_reach)
    check("a quote left open is read to its end, with nothing raised", [each.words for each in g.commands_of("ls 'a b")] == [["ls", "a b"]])
    check("a substitution left open too", [each.name for each in g.commands_of("echo $(cargo build")] == ["cargo", "echo"])
    check("and much of what is not there costs little", len(g.commands_of("cat <<A\n" * 2000)) == 1)
    check("a case left open reads to its end", [each.name for each in g.commands_of("case x in a) cargo build")] == ["cargo"])
    check("a condition left open too", [each.name for each in g.commands_of("[[ $(cargo build)")] == ["cargo"])
    check("an array left open too", [each.name for each in g.commands_of("x=($(cargo build)")] == ["cargo"])
    check("and arithmetic, read as a subshell as the shell would", [each.name for each in g.commands_of("((cargo build")] == ["cargo"])
    for arguments, told_it in (
        ("-euo pipefail -c x", ("x", None)), ("-eo pipefail -c x", ("x", None)), ("-c -e x", ("x", None)), ("-c -- x", ("x", None)), ("-lc x", ("x", None)), ("--rcfile f -c x", ("x", None)), ("-e script", (None, "script")),
        ("-o pipefail script", (None, "script")), ("script -c x", (None, "script")), ("-- script", (None, "script")), ("-s", (None, None)), ("-eo pipefail", (None, None)), ("--norc", (None, None)), ("--rcfile f", (None, None)), ("", (None, None)),
    ):
        check(f"a shell given `{arguments}` is told {told_it}", g.shell_told(arguments.split()) == told_it, g.shell_told(arguments.split()))
    check("`-s` keeps a shell on its input, its words being only its arguments", g.shell_told(["-s", "--", "x"]) == (None, None) and g.shell_told(["-s", "x"]) == (None, None))
    check("`-n` runs nothing it reads", g.shell_only_parses(["-n"]) and g.shell_only_parses(["-en"]) and not g.shell_only_parses(["-e", "x"]) and not g.shell_only_parses(["--norc"]))
    for quiet, asks in (("make --version", False), ("make --help", False), ("gmake -v", False), ("make -n focused", False), ("make help", False), ("make focused", True), ("cargo build --help", False), ("cargo build", True)):
        check(f"a build only if it builds: {quiet}", g.Process(1, 0, quiet, "", "", 0).spawns == asks)
    check("a root whose name a space goes on is another", not g.names_root("python3 host_guard.py run --root /x/a copy -- ls", "/x/a") and g.names_root("python3 host_guard.py run --root /x/a -- ls", "/x/a") and g.names_root("python3 host_guard.py status --root /x/a", "/x/a"))
    check("this interpreter running this file is the guard, a space in its path or not", g.protected(g.Process(1, 0, f"{sys.executable} -I -B {g.CODE} watch --root /x", "", "", 0)))
    check("`kill`'s targets are read as it reads them", g.kill_targets(["-9", "-$(…)"]) == ["-$(…)"] and g.kill_targets(["--", "-$G"]) == ["-$G"] and g.kill_targets(["-s", "TERM", "12"]) == ["12"] and g.kill_targets(["-TERM", "12"]) == ["12"])
    check("a shell told nothing is a terminal, whatever its options", g.is_terminal_shell(proc(1, "bash -eo pipefail")) and g.is_terminal_shell(proc(1, "bash --rcfile f")))
    check("and one told a command, wherever `-c` stands, is none", not g.is_terminal_shell(proc(1, "bash -euo pipefail -c x")) and not g.is_terminal_shell(proc(1, "bash -c -- x")))


def case_the_hook_passes_nothing_it_cannot_read() -> None:
    scope = fresh("unreadable")
    for given in ("not json", "", "{}", "[]", "null", "5", '{"tool_input": {"cmd": "cargo build"}}', '{"tool_input": {"command": 5}}', '{"tool_input": {"command": ""}}', "[" * 100000, json.dumps({"tool_input": {"command": "$(" * 5000}})):
        done = guard("hook", stdin=given, root=scope)
        check(f"input that is no command is refused: {given[:30]!r}", done.returncode == 2 and "cannot be read" in done.stderr, done.stderr)


def case_killing_by_name() -> None:
    scope = fresh("by-name")
    try:
        for command in (
            "pkill -x rustc", "pkill -9 -f .", "killall cargo", "cd x && pkill rustc", f"cd {scope} && pkill -x rustc", f"R={scope}; killall cargo", "kill $(pgrep -x rustc)",
            'kill "$(pgrep -x rustc)"', 'kill -9 "$(pgrep -f rustc)"', "kill `pgrep -x rustc`", 'for p in $(pgrep -x rustc); do kill "$p"; done', "pids=$(pgrep -x rustc); kill $pids",
            "pgrep -x rustc | while read p; do kill $p; done", "pgrep -x rustc | xargs kill", "/usr/bin/pkill rustc", "ps -ax | grep rustc | awk '{print $1}' | xargs kill", "kill $(ps -ax | grep rustc)",
            "(pkill -x rustc)", "$(pkill -x rustc)", "bash -c 'pkill -x rustc'", "sh -c 'pgrep -x rustc | xargs kill'", "# don't\npkill -x rustc\n# don't", "env pkill rustc", "sudo killall cargo",
            "ps -ax | awk '/rustc/{print $1}' | while read p; do kill $p; done", "for p in $(ps -ax | grep rustc | awk '{print $1}'); do kill $p; done",
            'arch="$(case "$(uname -m)" in arm64) echo a;; *) echo b;; esac)"; pkill -x rustc', 'x="$(case y in y) echo a;; esac )"\npkill -x rustc', "(( y = 1 << 3 ))\npkill -x rustc", "((x<<=1)); killall cargo", "echo $[1<<2]; pkill rustc",
            "bash -euo pipefail -c 'pkill -x rustc'", "bash -eo pipefail -c 'pkill -x rustc'", "bash -c -- 'pkill -x rustc'", "trap 'pkill -x rustc' EXIT", "bash <<'EOF'\npkill -x rustc\nEOF", "bash <<< 'pkill -x rustc'", "case y in y) pkill -x rustc;; esac",
            "[[ $line =~ ^([[:space:]]*)#(.*)$ ]] && pkill -x rustc", "bash -s -- x <<'E'\npkill -x rustc\nE", "bash -s x <<< 'killall cargo'", 'x="$(cat <<EOF\nbody\nEOF)"\npkill -x rustc', f"pkill -f {scope}-other/x", f"ps -ax | grep {scope}-other/x | xargs kill",
            'kill "$(pgrep -x rustc)"', 'ps -ax | grep x; kill "$PID"', "for ((i=0;i<1;i++)) do pkill -x rustc; done", "if true; then for ((i=0;i<1;i++)) do pkill -x rustc; done; fi", "time for ((i=0;i<1;i++)) do pkill -x rustc; done", "pkill -v -P 123", "pkill -vP 123", "pkill -P 5,1 -x rustc", "kill -9 -1", "kill -- -1", "pkill -PIPE rustc", "pkill -FPE rustc", "ps -upeter | awk '{print $1}' | xargs kill", 'pkill -P "$(pgrep -x make)"', 'P=$(pgrep -x make); pkill -P "$P"', "pgrep -x make > p; pkill -F p", "pkill -F <(pgrep -x rustc)", "ps -ax -opid -ocommand | grep rustc | awk '{print $1}' | xargs kill", "pkill -P 1 -x rustc", "pgrep -P 1 -x rustc | xargs kill -9", f"pkill -v -f {scope}", f"pgrep -vf {scope} | xargs kill", "kill -- -$(pgrep -x rustc)", "kill -TERM -$(pgrep -f myjob)", "PGID=$(pgrep -f myjob); kill -- -$PGID", "kill -s TERM $(pgrep -x rustc)",
            f"ps -ax | grep rustc | grep -v {scope} | awk '{{print $1}}' | xargs kill", f"pgrep -x rustc | tee {scope}/pids | xargs kill", f"pkill -f '{scope} copy'",
        ):
            done = hook_says(command, scope)
            check(f"never by a name across the host: {command}", done.returncode == 2 and "never by a program's name" in done.stderr, done.stderr)
        for command in (
            f"pkill -f '{scope}/x'", f"kill $(pgrep -f {scope}/x)", "kill 123", "kill -9 123", 'kill "$(cat job.pid)"', "kill $(cat job.pid)", "kill $(jobs -p)", "git commit -m 'pkill -x rustc'",
            "rg pkill docs/", "git grep -n killall -- scripts", "find . -name pkill -print", "python3 -I -B -c pass pkill probe", "pgrep -x rustc", "ps -ax | grep rustc", "git commit -m 'kill the pgrep rig'",
            "pkill -P 123", "pkill -P123", "pkill -fP 123 sleep", 'pkill -P "$PID"', "pkill -g 123", "ps -o pid= -g 123 | xargs kill", "ps -ax | head -3; kill -- -$$", 'ps 123; kill "$x"', 'ps up 123; kill "$x"', 'pgrep -lP "$PID"; kill "$PID"', "for w in do pkill; do echo $w; done", "kill $(pgrep -P 123)", "cat target/x/job.pid | xargs kill", f"pgrep -f {scope}/x | xargs kill", f"ps -ax | grep {scope}/x | awk '{{print $1}}' | xargs kill",
            'grep -n "kill\\|pgrep" f', 'rg "xargs kill" scripts/', 'echo "use kill, not pgrep"', "kill 123; ps -ax | grep x", "ps -ax | grep x; kill 123", "kill $!; pgrep -x rustc", "ps -p 123; kill $(cat p)",
            "tools=(pkill killall); echo ${tools[0]}", 'case "$t" in a) echo a;; pkill|killall) echo b;; esac', "for t in a b; do case $t in a) echo a;; pkill) echo b;; esac; done", "[[ $cmd =~ ^(pkill|killall)$ ]] && echo careful",
            "(( pkill > 1 ))", "pkill() { :; }; echo", "cat <<'EOF'\npkill -x rustc\nEOF",
            f"ps -ax |\n grep {scope}/x |\n awk '{{print $1}}' |\n xargs kill", 'ps -fp "$PID" && kill "$PID"', "pkill -F job.pid", "pgrep -F job.pid | xargs kill", 'ps -ax | head; kill "$(cat job.pid)"', "if true; then pkill() { :; }; fi",
            'PID=$(pgrep -n -f "make focused"); while kill -0 "$PID" 2>/dev/null; do sleep 5; done', f"pkill -f '{scope}.*rustc'", "bash -n -c 'pkill -x rustc'", "sh -n <<'E'\npkill -x rustc\nE", "ps -ax | head; kill -s TERM 123", "ps -ax | head; kill -TERM 123", "ps -ax | head; kill -- -123", 'kill -- -"$(cat job.pgid)"',
        ):
            done = hook_says(command, scope)
            check(f"not by a name across the host: {command}", done.returncode == 0, done.stderr)
    finally:
        guard("stop", root=scope)


def case_hook() -> None:
    scope = fresh("hook")
    try:
        done = hook_says("touch x", scope)
        check("a light command passes", done.returncode == 0, done.stderr)
        done = hook_says("cargo test -p x", scope, {"start_used_percent": 0})
        check("a heavy command is refused over the start line", done.returncode == 2 and "heavy" in done.stderr, done.stderr)
        check("a light one is not", hook_says("touch x", scope, {"start_used_percent": 0}).returncode == 0)
        for limit in ({"stop_used_percent": 0, "start_used_percent": 0}, {"stop_pressure_level": 0, "start_pressure_level": 0}, {"stop_repo_free_gib": 1e9}):
            check(f"a light one is not, whatever the host's state, so it can be looked at and helped: {limit}", hook_says("touch x", scope, limit).returncode == 0)
            check(f"nor waiting: {limit}", hook_says("sleep 60", scope, limit).returncode == 0)
            check(f"a heavy one is: {limit}", hook_says("cargo build", scope, limit).returncode == 2)
        builder = named("cargo", scope)
        if host_clear():
            done = hook_says("cargo build", scope)
            check("one heavy job at a time", done.returncode == 2 and "one heavy job" in done.stderr, done.stderr)
        else:
            not_run.append(f"one heavy job at a time ({TAKEN_AWAY})")
        check("a light command beside it passes", hook_says("touch x", scope).returncode == 0)
        os.killpg(builder.pid, signal.SIGKILL)
        builder.wait()
        now = time.strftime("%Y-%m-%dT%H:%M:%S")
        with open(scope / "state" / "events.log", "a") as log:
            log.write(stop_event(time.strftime("%Y-%m-%dT%H:%M:%S", time.localtime(time.time() - 3600)), "long-ago"))
            log.write(stop_event("hook-proof", "no-time"))
        done = hook_says("touch x", scope, session="b")
        check("a session new to this log is not told what was stopped long ago, nor a line whose time is no time", done.returncode == 0 and told(done) == "", done)
        check("nor is another", told(hook_says("touch x", scope, session="a")) == "")
        with open(scope / "state" / "events.log", "a") as log:
            log.write(stop_event(now, "x"))
        check("a light command beside nothing new prints nothing for the harness to read", hook_says("touch x", scope, session="c").returncode == 0)
        done = hook_says("touch x", scope)
        check("what was stopped is told beside the command, which runs", done.returncode == 0 and "stopped x" in told(done), done)
        check("once", told(hook_says("touch x", scope)) == "")
        for session in ("a", "b"):
            done = hook_says("touch x", scope, session=session)
            check(f"each session is told for itself: {session}", done.returncode == 0 and "stopped x" in told(done), done)
            check(f"and once: {session}", told(hook_says("touch x", scope, session=session)) == "")
        # A subagent shares its session's id: a stop is told to it and to the agent that started it, each once, whichever comes first.
        hook_says("touch x", scope, session="a", agent="sub")  # Its place in the log begins here, so that the stop below is what it is told.
        with open(scope / "state" / "events.log", "a") as log:
            log.write(stop_event(now, "for-both"))
        check("a stop is told to a subagent", "stopped for-both" in told(hook_says("touch x", scope, session="a", agent="sub")))
        check("and to the agent that started it, its session's id being the same", "stopped for-both" in told(hook_says("touch x", scope, session="a")))
        check("each once", told(hook_says("touch x", scope, session="a", agent="sub")) == "" and told(hook_says("touch x", scope, session="a")) == "")
        # `cargo clean` takes a session's place: what was stopped lately is told all the same.
        (scope / "state" / "seen.a").unlink()
        done = hook_says("touch x", scope, session="a")
        check("a session that lost its place is told what was stopped lately", done.returncode == 0 and "stopped x" in told(done), done)
        # However often it stops, the way out of it runs: a `kill` of the job among it.
        with open(scope / "state" / "events.log", "a") as log:
            log.write(stop_event(now, "y"))
        done = hook_says("kill 999999", scope, session="a")
        check("a stop just made refuses nothing, the way out of it among it", done.returncode == 0 and "stopped y" in told(done), done)
        with open(scope / "state" / "events.log", "a") as log:
            log.write(stop_event(now, "z"))
        done = hook_says("cargo build", scope, {"start_used_percent": 0}, session="a")
        check("a command refused for another reason still tells it", done.returncode == 2 and "stopped z" in done.stderr, done)
        done = hook_says("cargo build", scope, {"start_used_percent": 0})
        check("a heavy command refused is told how to wait", done.returncode == 2 and "run --wait" in done.stderr, done.stderr)
        # Told the last few, and how many came before them, never fewer in silence.
        with open(scope / "state" / "events.log", "a") as log:
            for n in range(9):
                log.write(stop_event(now, f"many{n}"))
        said = told(hook_says("touch x", scope, session="a"))
        check("many stops are told as the last few and how many more", "and 4 stopped before these" in said and "stopped many8" in said and "stopped many3" not in said, said)
        check("`make` of a guarded recipe waits by itself, so the hook lets it", hook_says("make focused", scope, {"start_used_percent": 0}).returncode == 0)
        time.sleep(0.5)
        check("the hook started a watcher", "watcher: running" in guard("status", root=scope).stdout)
        check("`Monitor` is read as a shell command", guard("hook", stdin=json.dumps({"tool_name": "Monitor", "tool_input": {"command": "pkill -x rustc"}}), root=scope).returncode == 2)
        check("and with a socket and no shell it has nothing to read", guard("hook", stdin=json.dumps({"tool_name": "Monitor", "tool_input": {"ws": {"url": "wss://x"}}}), root=scope).returncode == 0)
        check("a shell tool with no command is still refused", guard("hook", stdin=json.dumps({"tool_name": "Bash", "tool_input": {}}), root=scope).returncode == 2)
        check("a heavy command the guard runs is not refused for being one", hook_says("python3 scripts/host_guard.py run --wait 1800 -- bash -c 'cd x && cargo test'", scope, {"start_used_percent": 0}).returncode == 0)
        check("nor `cargo clean`, at a disk floor, where it is needed", hook_says("cargo clean", scope, {"stop_repo_free_gib": 1e9}).returncode == 0)
    finally:
        guard("stop", root=scope)
    time.sleep(0.4)
    check("and `stop` ends it", "watcher: not running" in guard("status", root=scope).stdout)


def case_a_damaged_log() -> None:
    scope = fresh("damaged")
    try:
        state = scope / "state"
        state.mkdir()
        good = json.dumps({"kind": "stopped", "at": "now", "reasons": ["why"], "stopped": [{"pid": 1, "name": "told", "gib": 1.0, "command": "x"}]})
        (state / "events.log").write_bytes(b"not json\n\xff\xfe\n[1]\n" + good.encode() + b'\n{"kind": "stopped"}\n')
        done = hook_says("touch x", scope)
        check("bad lines do not cost the good one", done.returncode == 0 and "stopped told" in told(done), done)
        check("and are read past", told(hook_says("touch x", scope)) == "")
        check("`status` reads past them too", guard("status", root=scope).returncode == 0)
        (state / "seen").write_text("-5")
        done = hook_says("touch x", scope)
        check("a nonsense position starts again rather than crashing", done.returncode == 0 and "stopped told" in told(done), done)
        (state / "seen").write_text("5000000")
        done = hook_says("touch x", scope)
        check("a position past a replaced log starts again, and the stop is told", done.returncode == 0 and "stopped told" in told(done), done)
        with open(state / "events.log", "a") as log:
            log.write(json.dumps({"kind": "stopped", "at": "now", "reasons": 5, "largest": 5, "stopped": [{"pid": 1, "name": "odd", "gib": 1.0, "command": "x"}]}) + "\n")
        done = hook_says("touch x", scope)
        check("a line whose fields are not what they should be is told as far as it reads", done.returncode == 0 and "stopped odd" in told(done), done)
        with open(state / "events.log", "a") as log:
            for _ in range(6):
                log.write(json.dumps({"kind": "pressure-not-ours", "at": "later", "reasons": ["x"]}) + "\n")
        said = guard("status", root=scope).stdout
        check("`status` names a stop whatever lines came after", "stopped told" in said, said)
        (state / "peak.json").write_bytes(b"\xff\xfe")
        check("`status` reads past a binary peak file", guard("status", root=scope).returncode == 0)
    finally:
        guard("stop", root=scope)
    scope = fresh("state-a-file")
    (scope / "state").write_text("a file where the state goes")
    check("state it cannot use does not stop a light command, so it can be repaired", hook_says("rm -rf state", scope).returncode == 0)
    done = hook_says("cargo build", scope)
    check("and a heavy one is refused, with no watcher to hold it", done.returncode == 2, done.stderr)


def case_run() -> None:
    scope = fresh("run")
    try:
        marker = scope / "ran"
        done = guard("run", "--", "touch", str(marker), root=scope, limits={"start_used_percent": 0})
        check("over the start line a job is not run", done.returncode == 75 and not marker.exists() and "not started" in done.stderr, done)
        began = time.monotonic()
        done = subprocess.run([*PY, str(GUARD), "run", "--root", str(scope), "--limits", json.dumps({"start_used_percent": 0}), "--wait", "1", "--", "true"], capture_output=True, text=True, timeout=30)
        check("a job waits for room as long as it was told, and no longer", done.returncode == 75 and 1.0 <= time.monotonic() - began < 20, done)
        check("and says what it waits for", "waiting for room" in done.stderr, done.stderr)
        if not host_clear():
            not_run.append(f"what needs a clear host to `run` ({TAKEN_AWAY})")
            return
        done = guard("run", "--", "bash", "-c", "exit 7", root=scope)
        check("a job's exit status is kept", done.returncode == 7, done)
        done = guard("run", "--", "bash", "-c", 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS $(ps -o nice= -p $$)"', root=scope)
        words = done.stdout.split()
        check("a job runs at a lower priority and a third of the cores", len(words) == 3 and words[2] == "10" and int(words[0]) == max(2, (os.cpu_count() or 4) // 3), done.stdout)
        builder = named("cargo", scope)
        done = guard("run", "--", "touch", str(marker), root=scope)
        check("`run` waits its turn behind a build of ours", done.returncode == 75 and "one heavy job" in done.stderr and not marker.exists(), done)
        os.killpg(builder.pid, signal.SIGKILL)
        builder.wait()
        first = subprocess.Popen([*PY, str(GUARD), "run", "--root", str(scope), "--", "sleep", "6"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
        started.append(first)
        time.sleep(0.7)
        done = guard("run", "--", "touch", str(marker), root=scope)
        check("and behind another `run`", done.returncode == 75 and "one heavy job" in done.stderr and not marker.exists(), done)
        (scope / "state" / "job.lock").unlink(missing_ok=True)
        done = guard("run", "--", "touch", str(marker), root=scope)
        check("with its lock file removed too", done.returncode == 75 and "is running" in done.stderr and not marker.exists(), done)
        env = dict(os.environ, HOST_GUARD_JOB=str(scope))
        done = guard("run", "--", "touch", str(marker), root=scope, env=env)
        check("and no environment can claim to be its job", done.returncode == 75 and not marker.exists(), done)
        done = hook_says("cargo build", scope)
        check("a heavy command is refused beside it, whatever its job is called", done.returncode == 2 and "one heavy job" in done.stderr, done.stderr)
        first.wait(timeout=15)
        # Two `make`s, each waiting in its own `run`, must not take each other for a running build: one goes, then the other.
        makes = [spawn(["bash", "-c", f"exec -a make bash -c '{sys.executable} -I -B {GUARD} run --root {scope} --wait 25 -- bash -c \"sleep 1; touch {scope}/{name}\"; true'"], scope) for name in ("one", "two")]
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline and not ((scope / "one").exists() and (scope / "two").exists()):
            time.sleep(0.2)
        check("two `make`s take turns rather than waiting on each other", (scope / "one").exists() and (scope / "two").exists())
        # On a loaded host the second may still hold its turn, which the next step would read as its own refusal.
        until(lambda: all(make.poll() is not None for make in makes), 40)
        # What a job leaves behind holds no turn: it is no job of a `run`, and nothing could find it.
        left = scope / "left-pid"
        done = guard("run", "--", "bash", "-c", f"nohup sleep 20 >/dev/null 2>&1 & echo $! > {left}", root=scope)
        check("a job that left something running has ended", done.returncode == 0, done)
        done = guard("run", "--", "bash", "-c", "exit 6", root=scope)
        check("and what it left holds no turn", done.returncode == 6, done)
        os.kill(int(left.read_text()), signal.SIGKILL)
        done = guard("run", "--", "/nonexistent/program", root=scope)
        check("a program that cannot start is said, as a shell says it", done.returncode == 127 and "cannot be run" in done.stderr and "Traceback" not in done.stderr, done)
        done = guard("run", "--", "bash", "-c", "kill -9 $$", root=scope)
        check("a job ended by a signal is 128 and the signal", done.returncode == 137, done)
        # A job's own stop is said by its own `run`. The watcher its earlier `run`s started has the default lines, and holds the lock: it goes first.
        guard("stop", root=scope)
        time.sleep(0.5)
        watcher = looping_watcher(scope, {"one_process_gib": 0.03})
        time.sleep(0.5)
        (scope / "hold.py").write_text(HOLD)
        done = guard("run", "--", sys.executable, "-I", "-B", str(scope / "hold.py"), "60", str(scope / "held"), root=scope)
        check("a `run` says why its own job was stopped", "stopped" in done.stderr and "one process" in done.stderr, done)
        # And with the log replaced under it, by `cargo clean`.
        (scope / "held").unlink(missing_ok=True)
        (scope / "cleans.py").write_text(HOLD.replace('open(sys.argv[2], "w").close()', 'import shutil; shutil.rmtree(sys.argv[3], ignore_errors=True); open(sys.argv[2], "w").close()'))
        done = guard("run", "--", sys.executable, "-I", "-B", str(scope / "cleans.py"), "60", str(scope / "held"), str(scope / "state"), root=scope)
        check("a `run` says so with its log replaced under it", "stopped" in done.stderr, done)
        watcher.terminate()
        watcher.wait(timeout=5)
        # A `run` that goes blind ends its job, with no process table to read.
        flag = scope / "flag"
        env = shim(scope, "ps", f'[ -f "{flag}" ] && exit 1; exec /bin/ps "$@"')
        guard("stop", root=scope)
        time.sleep(0.5)
        blind = subprocess.Popen([*PY, str(GUARD), "run", "--root", str(scope), "--limits", json.dumps({"watcher_stale_seconds": 1}), "--", "sleep", "40"], env=env, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, start_new_session=True)
        started.append(blind)
        time.sleep(1.5)
        its_job = jobs_of(blind.pid)
        flag.write_text("")
        # The watcher this `run` started has the same `ps`, so it goes blind too.
        try:
            rc = blind.wait(timeout=25)
        except subprocess.TimeoutExpired:
            rc = None
        check("a `run` no watcher can see for ends its job", rc == 75 and bool(its_job) and not any(pid_up(pid) for pid in its_job), (rc, its_job))
        flag.unlink()
        guard("stop", root=scope)
        # A `run` started from anywhere, naming no root, says on its command line whose it is.
        repo, elsewhere = fresh("run-repo"), fresh("run-elsewhere")
        (repo / ".git").mkdir()
        (repo / "scripts").mkdir()
        shutil.copy(GUARD, repo / "scripts" / "host_guard.py")
        copy = repo / "scripts" / "host_guard.py"
        away = spawn([*PY, str(copy), "run", "--", "sleep", "6"], elsewhere)
        time.sleep(1.0)
        said = subprocess.run([*PY, str(copy), "status"], capture_output=True, text=True, timeout=30).stdout
        check("a `run` from another directory has its job counted as ours", "ours: 0 " not in said and "ours:" in said, said)
        away.wait(timeout=15)
        subprocess.run([*PY, str(copy), "stop"], capture_output=True, timeout=30)
        nested = [*PY, str(GUARD), "run", "--root", str(scope), "--", "bash", "-c", "exit 4"]
        done = guard("run", "--", *nested, root=scope)
        check("a job nested in its own `run` is not its own rival", done.returncode == 4, done)
        with open(scope / "state" / "events.log", "ab") as log:
            log.write(b"not json\n")
            log.write(json.dumps({"kind": "stopped", "at": "now", "reasons": ["why"], "stopped": [{"pid": 1, "name": "pending", "gib": 1.0, "command": "x"}]}).encode() + b"\n")
        done = guard("run", "--", "bash", "-c", "exit 3", root=scope)
        check("`run` tells what was stopped, and a bad line costs nothing", done.returncode == 3 and "stopped pending" in done.stderr, done)
    finally:
        guard("stop", root=scope)


# What holds nothing and reads nothing of the host: these say the same on a host of any state.
PURE = (case_each_line_judges_alone, case_growth_is_of_the_last_minutes, case_what_counts_as_heavy, case_the_reader_sees_what_the_shell_runs, case_where_a_root_is)
THE_REST = (
    case_a_job_under_run_outlasts_a_changed_guard, case_a_watcher_ended_by_its_own_error_says_so, case_a_root_spelt_relatively_stays_that_root, case_make_looks_for_the_whole_guard,
    case_only_tightened, case_check_refuses, case_a_figure_not_read_is_never_safe, case_one_process, case_all_processes, case_what_a_stopped_process_started,
    case_another_project_is_never_ours, case_a_terminal_vouches_for_nothing, case_a_root_is_exactly_itself, case_agents, case_what_holds_little_is_left_alone, case_a_job_is_stopped_whole,
    case_a_queued_make_is_no_build, case_at_a_disk_floor_what_writes_stops, case_names_and_zombies, case_a_command_line_that_is_no_text, case_pressure, case_not_ours, case_growing,
    case_a_small_writer_is_counted, case_the_watcher_outlives_its_own_trouble, case_a_guard_changed_holds_from_then, case_a_root_remade_is_another, case_a_root_that_is_gone_stays_gone, case_a_stale_pid_names_nobody, case_the_watcher_stays_while_a_job_runs,
    case_what_is_not_the_table_is_blindness, case_a_blind_watcher_is_said, case_the_hook_passes_nothing_it_cannot_read, case_killing_by_name, case_hook, case_a_damaged_log, case_run,
)


def real_stops_since(offset: int) -> list[str]:
    """What this repository's real watcher has stopped of this test's since `offset` in its log. This test's rigs sit under this root, so on a full host it stops them too, and a stub named after a
    build first. Another session's stop in that log says nothing of a rig here, and a failure beside it stays a failure."""
    mine = {process.pid for process in started}
    log = ROOT / "target" / "host-guard" / "state" / "events.log"
    try:
        with open(log, "rb") as lines:
            lines.seek(offset)
            text = lines.read().decode("utf-8", "replace")
    except OSError:
        return []
    stops = []
    for line in text.splitlines():
        try:
            event = json.loads(line)
        except ValueError:
            continue
        if isinstance(event, dict) and event.get("kind") == "stopped":
            # A job is named, and the rig's own `run` is its parent.
            stops.extend(str(stopped.get("name")) for stopped in event.get("stopped", []) if isinstance(stopped, dict) and (stopped.get("pid") in mine or stopped.get("parent") in mine))
    return stops


def real_log_size() -> int:
    try:
        return (ROOT / "target" / "host-guard" / "state" / "events.log").stat().st_size
    except OSError:
        return 0


def run_case(case) -> None:
    before = len(failures)
    offset = real_log_size()
    try:
        case()
    except Exception as error:
        check(f"{case.__name__} ran to its end", False, repr(error))
    finally:
        # What the real watcher stopped under a case says nothing of what the case asks: said as not run, and only when the log shows it stopped one of this test's own then.
        stopped = real_stops_since(offset) if len(failures) > before else []
        clean_up()
    if stopped:
        not_run.extend(f"{what} ({STOPPED_UNDER} {', '.join(stopped[:3])} under it)" for what in failures[before:])
        del failures[before:]
    print(f"  {case.__name__}: {'ok' if len(failures) == before else 'FAILED'}", flush=True)


def main(names: list[str]) -> int:
    global pure, passed
    unknown = set(names) - {case.__name__ for case in (*PURE, *THE_REST)}
    if unknown:
        print(f"host-guard self-test: no case is called {sorted(unknown)}")
        return 2
    BASE.mkdir(parents=True, exist_ok=True)
    chosen = lambda case: not names or case.__name__ in names
    pure = True
    for case in filter(chosen, PURE):
        run_case(case)
    pure = False
    threading.Thread(target=note_when_unclear, daemon=True).start()
    rest = list(filter(chosen, THE_REST))
    # The rest holds a few hundred MiB. On a host already full, that is the wrong thing to do: it waits for room, and only then says what it could not run. So does a case the host took away
    # meanwhile, which this shared host does often, within one bound for all of them.
    room_until = time.monotonic() + ROOM_SECONDS
    room = lambda: wait_for_room(room_until)
    if rest and not room():
        print("host-guard self-test: the rest not run: the host stayed over the start line, and they would add to it")
        print(guard("status").stdout)
        not_run.extend(f"{case.__name__} ({TAKEN_AWAY})" for case in rest)
        rest = []
    for case in rest:
        # Each holds what it holds: on a host that has filled since the last, it waits too.
        if not room():
            not_run.append(f"{case.__name__} ({TAKEN_AWAY})")
            continue
        before, counted = len(not_run), passed
        run_case(case)
        # Only what the host took away, and not what a case could not run for a reason of its own, and twice more at most: some of them write gigabytes.
        for _ in range(2):
            if not any(TAKEN_AWAY in what or STOPPED_UNDER in what for what in not_run[before:]) or not room():
                break
            print(f"  {case.__name__}: the host took it away: again, with room", flush=True)
            del not_run[before:]
            passed = counted
            run_case(case)
    pure = True
    check("this test leaves the tree as it found it, with no bytecode beside its script", not (HERE / "__pycache__").exists())
    print(f"host-guard self-test: {passed} passed, {len(failures)} failed, {len(not_run)} not run")
    if not_run:
        print("  each says why, or the host was over the start line when it came to it")
    for what in not_run:
        print(f"  not run: {what}")
    if failures:
        return 1
    # `incomplete`, as a verification tier says it: not a pass.
    return 20 if not_run else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
