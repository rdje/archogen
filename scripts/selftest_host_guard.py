#!/usr/bin/env python3
"""selftest_host_guard.py: the guard stops what it should and nothing else. Each case runs in a scratch root of its own, so a watcher with
a limit tightened to nothing has only that root's processes to stop, and the memory it uses to prove so stays under a few hundred MiB."""

from __future__ import annotations

import json
import os
import shutil
import signal
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
GUARD = HERE / "host_guard.py"
ROOT = next(at for at in HERE.parents if (at / ".git").is_dir())
BASE = ROOT / "target" / "host-guard" / "selftest"
PY = [sys.executable, "-I", "-B"]
HOLD = """
import os, sys, time
n = int(sys.argv[1]) << 20
b = bytearray(n)
for i in range(0, n, 16384):
    b[i] = 1
open(sys.argv[2], "w").close()
time.sleep(90)
"""
PARENT = """
import os, subprocess, sys, time
n = int(sys.argv[1]) << 20
b = bytearray(n)
for i in range(0, n, 16384):
    b[i] = 1
child = subprocess.Popen(["sleep", "90"], cwd=sys.argv[3], start_new_session=True)
open(sys.argv[4], "w").write(str(child.pid))
open(sys.argv[2], "w").close()
time.sleep(90)
"""
GROW = """
import sys, time
chunks = []
for _ in range(60):
    chunk = bytearray(10 << 20)
    for i in range(0, len(chunk), 16384):
        chunk[i] = 1
    chunks.append(chunk)
    time.sleep(0.15)
open(sys.argv[1], "w").close()
time.sleep(90)
"""
started: list[subprocess.Popen] = []
failures: list[str] = []
not_run: list[str] = []
passed = 0


def check(what: str, holds: bool, detail: object = "") -> None:
    global passed
    if holds:
        passed += 1
    else:
        failures.append(what)
        print(f"  FAILED {what}: {detail}")


def host_clear() -> bool:
    """Whether the host itself would let a heavy job start. No limit can be loosened to test what happens when it would, so while another
    project holds the memory these cases cannot run: said, and never counted as passing."""
    return guard("check").returncode == 0


def fresh(name: str) -> Path:
    path = BASE / name
    shutil.rmtree(path, ignore_errors=True)
    path.mkdir(parents=True)
    return path


def guard(*args: str, stdin: str | None = None, root: Path | None = None, limits: dict | None = None) -> subprocess.CompletedProcess:
    verb, rest = args[0], list(args[1:])
    flags: list[str] = []
    if root:
        flags += ["--root", str(root)]
    if limits is not None:
        flags += ["--limits", json.dumps(limits)]
    return subprocess.run([*PY, str(GUARD), verb, *flags, *rest], input=stdin, capture_output=True, text=True, timeout=60)


def spawn(args: list[str], cwd: Path) -> subprocess.Popen:
    process = subprocess.Popen(args, cwd=cwd, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    started.append(process)
    return process


def holding(mib: int, cwd: Path) -> subprocess.Popen:
    ready = cwd / f"ready-{len(started)}"
    process = spawn([*PY, "-c", HOLD, str(mib), str(ready)], cwd)
    deadline = time.monotonic() + 20
    while not ready.exists() and time.monotonic() < deadline:
        time.sleep(0.02)
    assert ready.exists(), "the allocation never finished"
    return process


def named(name: str, cwd: Path, script: str = "sleep 90") -> subprocess.Popen:
    process = spawn(["bash", "-c", f'exec -a {name} bash -c "{script}"'], cwd)
    time.sleep(0.15)
    return process


def up(process: subprocess.Popen) -> bool:
    return process.poll() is None


def pid_up(pid: int) -> bool:
    state = subprocess.run(["ps", "-o", "stat=", "-p", str(pid)], capture_output=True, text=True).stdout.strip()
    return bool(state) and not state.startswith("Z")


def events(root: Path) -> list[dict]:
    log = root / "state" / "events.log"
    return [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []


def settle(process: subprocess.Popen) -> bool:
    """Whether `process` has ended, within a moment."""
    try:
        process.wait(timeout=4)
        return True
    except subprocess.TimeoutExpired:
        return False


def case_only_tightened() -> None:
    for loose in ({"stop_used_percent": 99}, {"panic_used_percent": 95}, {"stop_repo_free_gib": 1}, {"all_processes_gib": 1000}):
        done = guard("check", limits=loose)
        check(f"a loosened limit is refused: {loose}", done.returncode == 2 and "may only be tightened" in done.stderr, done.stderr)
    done = guard("check", limits={"no_such_limit": 1})
    check("an unknown limit is refused", done.returncode == 2 and "no limit is called" in done.stderr, done.stderr)


def case_check_refuses() -> None:
    done = guard("check", limits={"start_used_percent": 0})
    check("RAM over the start line is not clear", done.returncode == 75 and "RAM use" in done.stdout, done.stdout)
    done = guard("check", limits={"stop_repo_free_gib": 1e9})
    check("a volume under its floor is not clear", done.returncode == 75 and "free" in done.stdout, done.stdout)
    done = guard("check", limits={"stop_boot_free_gib": 1e9})
    check("the boot volume under its floor is not clear", done.returncode == 75 and "boot volume" in done.stdout, done.stdout)


def case_one_process() -> None:
    scope, outside = fresh("one"), fresh("one-outside")
    big, small = holding(120, scope), holding(15, scope)
    control = holding(120, outside)
    done = guard("watch", "--once", root=scope, limits={"one_process_gib": 0.05})
    check("the watcher ran once", done.returncode == 0, done.stderr)
    check("the process over its limit is stopped", settle(big))
    check("the one under it is not", up(small))
    check("a process of the same size outside the root is not", up(control))
    stopped = [event for event in events(scope) if event["kind"] == "stopped"]
    check("what was stopped is recorded, with why", len(stopped) == 1 and stopped[0]["stopped"][0]["pid"] == big.pid and "one process" in stopped[0]["reasons"][0], stopped)
    check("and who held the most on the host, ours or not", bool(stopped) and len(stopped[0]["largest"]) == 5 and all("gib" in held for held in stopped[0]["largest"]), stopped)


def case_all_processes() -> None:
    scope = fresh("all")
    largest, other = holding(70, scope), holding(45, scope)
    guard("watch", "--once", root=scope, limits={"all_processes_gib": 0.1})
    check("over the total, the largest is stopped", settle(largest))
    check("and no more than it takes", up(other))
    guard("watch", "--once", root=scope, limits={"all_processes_gib": 0.1})
    check("back under the total, nothing more is stopped", up(other))


def case_tree() -> None:
    scope, outside = fresh("tree"), fresh("tree-outside")
    ready, pid_file = scope / "ready", scope / "child-pid"
    parent = spawn([*PY, "-c", PARENT, "120", str(ready), str(outside), str(pid_file)], scope)
    deadline = time.monotonic() + 20
    while not (ready.exists() and pid_file.read_text().strip() if pid_file.exists() else False) and time.monotonic() < deadline:
        time.sleep(0.02)
    child = int(pid_file.read_text())
    guard("watch", "--once", root=scope, limits={"one_process_gib": 0.05})
    check("the process is stopped", settle(parent))
    time.sleep(0.2)
    check("and what it started, in another session and directory", not pid_up(child))


def case_panic() -> None:
    scope, outside = fresh("panic"), fresh("panic-outside")
    sibling = fresh("panic-other")
    one, two = holding(20, scope), holding(20, scope)
    agent = named("claude", scope, f"(cd {outside} && exec sleep 91) & wait; sleep 90")
    its_children = subprocess.run(["pgrep", "-P", str(agent.pid)], capture_output=True, text=True).stdout.split()
    check("the agent has a child to stop", bool(its_children), its_children)
    control = holding(20, outside)
    prefix = holding(20, sibling)
    small_git = named("git", scope, "sleep 90; sleep 90; true")
    guard("watch", "--once", root=scope, limits={"panic_used_percent": 0})
    check("a small git is not stopped mid-write, which would free nothing", up(small_git))
    check("at the danger line everything of ours stops", settle(one) and settle(two))
    check("an agent working in the root is never stopped", up(agent))
    time.sleep(0.2)
    check("what the agent started is stopped, wherever it runs", not any(pid_up(int(pid)) for pid in its_children), its_children)
    check("a process outside the root is not stopped", up(control))
    check("nor one in a directory that merely starts with the root's name", up(prefix))


def case_a_command_line_that_is_no_text() -> None:
    # Measured `2026-11-05`: one such process anywhere on the host left the watcher unable to read the process table.
    scope = fresh("no-text")
    strange = spawn(["bash", "-c", "exec -a $'x\\xe9y' sleep 90"], fresh("no-text-elsewhere"))
    time.sleep(0.15)
    big = holding(120, scope)
    guard("watch", "--once", root=scope, limits={"one_process_gib": 0.05})
    check("a command line that is no text does not blind the watcher", settle(big), events(scope))
    check("and is not an error", not any(event["kind"] == "unread" for event in events(scope)), events(scope))
    check("and that process is left alone", up(strange))


def case_not_ours() -> None:
    scope = fresh("not-ours")
    small = holding(15, scope)
    guard("watch", "--once", root=scope, limits={"stop_used_percent": 0})
    check("pressure with little of ours stops nothing", up(small))
    check("and is recorded as someone else's", any(event["kind"] == "pressure-not-ours" for event in events(scope)), events(scope))
    guard("watch", "--once", root=scope, limits={"stop_used_percent": 0, "worth_stopping_gib": 0.001})
    check("pressure with something of ours worth stopping stops it", settle(small))


def case_growing() -> None:
    scope = fresh("growing")
    ready = scope / "ready"
    grower = spawn([*PY, "-c", GROW, str(ready)], scope)
    watcher = subprocess.Popen([*PY, str(GUARD), "watch", "--root", str(scope), "--limits", json.dumps({"one_process_gib": 0.1})], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    started.append(watcher)
    check("a process that keeps growing is stopped while it grows", settle_within(grower, 15))
    check("before it reaches what it was going for", not ready.exists())
    deadline = time.monotonic() + 5
    stopped = []
    while not stopped and time.monotonic() < deadline:
        stopped = [event for event in events(scope) if event["kind"] == "stopped"]
        time.sleep(0.05)
    check("within a sample or two of the line", bool(stopped) and stopped[0]["stopped"][0]["gib"] < 0.3, stopped)
    watcher.terminate()
    watcher.wait(timeout=5)


def settle_within(process: subprocess.Popen, seconds: float) -> bool:
    try:
        process.wait(timeout=seconds)
        return True
    except subprocess.TimeoutExpired:
        return False


def hook_says(command: str, root: Path, limits: dict | None = None) -> subprocess.CompletedProcess:
    return guard("hook", stdin=json.dumps({"tool_input": {"command": command}}), root=root, limits=limits)


def case_hook() -> None:
    scope = fresh("hook")
    try:
        check("a light command passes", hook_says("ls", scope).returncode == 0)
        for heavy in ("cargo test -p x", "make focused", "cd a && cargo build", "FOO=1 cargo build", "timeout 9 cargo test", "for t in a; do cargo test; done", "target/m/install/archogen build s --out d"):
            done = hook_says(heavy, scope, {"start_used_percent": 0})
            check(f"a heavy command is refused over the start line: {heavy}", done.returncode == 2 and "heavy" in done.stderr, done.stderr)
        for light in ("ls", "cargo fmt -p x", "git commit -F m", "grep -n 'make ' Makefile", "git commit -m 'make cargo build'", "echo cargo build"):
            check(f"over the start line a light command passes: {light}", hook_says(light, scope, {"start_used_percent": 0}).returncode == 0)
        done = hook_says("ls", scope, {"stop_used_percent": 0})
        check("over the stop line every command is refused", done.returncode == 2 and "RAM use" in done.stderr, done.stderr)
        for reads in ("ps -ax", "sysctl -n vm.swapusage", "git status", "pkill -f x", f"python3 -I -B {GUARD} status"):
            check(f"but the host can still be read and helped: {reads}", hook_says(reads, scope, {"stop_used_percent": 0}).returncode == 0)
        for hidden in ("ps -ax; cargo test", "git status && make focused", "kill 1 | cargo build"):
            check(f"and nothing rides in on them: {hidden}", hook_says(hidden, scope, {"stop_used_percent": 0}).returncode == 2)
        builder = named("cargo", scope, "sleep 90; true")
        if host_clear():
            done = hook_says("cargo build", scope)
            check("one heavy job at a time", done.returncode == 2 and "one heavy job" in done.stderr, done.stderr)
        else:
            not_run.append("one heavy job at a time")
        check("a light command beside it passes", hook_says("ls", scope).returncode == 0)
        builder.kill()
        builder.wait()
        (scope / "state" / "events.log").open("a").write(json.dumps({"kind": "stopped", "at": "now", "reasons": ["why"], "stopped": [{"pid": 1, "name": "x", "gib": 1.0, "command": "x"}]}) + "\n")
        done = hook_says("ls", scope)
        check("what was stopped is told before anything else runs", done.returncode == 2 and "stopped x" in done.stderr, done.stderr)
        check("once", hook_says("ls", scope).returncode == 0)
        status = guard("status", root=scope)
        check("the hook started a watcher", "watcher: running" in status.stdout, status.stdout)
    finally:
        guard("stop", root=scope)
    time.sleep(0.3)
    check("and `stop` ends it", "watcher: not running" in guard("status", root=scope).stdout)


def case_run() -> None:
    scope = fresh("run")
    try:
        if host_clear():
            done = guard("run", "--", "bash", "-c", "exit 7", root=scope)
            check("a job's exit status is kept", done.returncode == 7, done)
        else:
            not_run.append("a job's exit status is kept")
        began = time.monotonic()
        done = subprocess.run([*PY, str(GUARD), "run", "--root", str(scope), "--limits", json.dumps({"start_used_percent": 0}), "--wait", "1", "--", "true"], capture_output=True, text=True, timeout=30)
        check("a job waits for room as long as it was told, and no longer", done.returncode == 75 and 1.0 <= time.monotonic() - began < 20, done)
        marker = scope / "ran"
        done = guard("run", "--", "touch", str(marker), root=scope, limits={"start_used_percent": 0})
        check("over the start line a job is not run", done.returncode == 75 and not marker.exists() and "not started" in done.stderr, done)
        if host_clear():
            done = guard("run", "--", "bash", "-c", 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS $(ps -o nice= -p $$)"', root=scope)
            words = done.stdout.split()
            check("a job runs at a lower priority and a third of the cores", len(words) == 3 and words[2] == "10" and int(words[0]) == max(2, (os.cpu_count() or 4) // 3), done.stdout)
        else:
            not_run.append("a job runs at a lower priority and a third of the cores")
    finally:
        guard("stop", root=scope)


def case_pressure() -> None:
    done = guard("check", limits={"start_pressure_level": 0})
    check("macOS's pressure level stops a heavy job starting", done.returncode == 75 and "memory pressure" in done.stdout, done.stdout)
    scope = fresh("pressure")
    one = holding(20, scope)
    guard("watch", "--once", root=scope, limits={"stop_pressure_level": 0, "worth_stopping_gib": 0.001})
    check("macOS's pressure level stops our largest", settle(one))
    scope = fresh("pressure-critical")
    two, three = holding(15, scope), holding(15, scope)
    guard("watch", "--once", root=scope, limits={"panic_pressure_level": 0})
    check("at its critical level everything of ours stops", settle(two) and settle(three))


def main() -> int:
    # This test holds a few hundred MiB. On a host already full, that is the wrong thing to do.
    if not host_clear():
        print("host-guard self-test: not run: the host is over the start line, and this test would add to it")
        print(guard("status").stdout)
        return 20
    BASE.mkdir(parents=True, exist_ok=True)
    for case in (case_only_tightened, case_check_refuses, case_one_process, case_all_processes, case_tree, case_panic, case_a_command_line_that_is_no_text, case_pressure, case_not_ours, case_growing, case_hook, case_run):
        case()
        for process in started:
            if up(process):
                process.kill()
                process.wait()
        started.clear()
    print(f"host-guard self-test: {passed} passed, {len(failures)} failed, {len(not_run)} not run")
    for what in not_run:
        print(f"  not run, the host being over the start line: {what}")
    if failures:
        return 1
    # `incomplete`, as a verification tier says it: not a pass.
    return 20 if not_run else 0


if __name__ == "__main__":
    sys.exit(main())
