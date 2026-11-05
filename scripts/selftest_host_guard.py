#!/usr/bin/env python3
"""selftest_host_guard.py: the guard stops what it should and nothing else. Each case runs in a scratch root of its own, so a watcher with a limit
tightened to nothing has only that root's processes to stop, and the memory it uses to prove so stays under a few hundred MiB. On a host already
over the start line it does not run: it would add to the load. Exit 0 all held · 1 one did not · 20 some could not run, which is not a pass."""

from __future__ import annotations

import json
import os
import shutil
import signal
import stat
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import host_guard as g  # noqa: E402

GUARD = HERE / "host_guard.py"
ROOT = next(at for at in HERE.parents if (at / ".git").is_dir())
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
IGNORE = """
import signal, sys, time
signal.signal(signal.SIGTERM, signal.SIG_IGN)
open(sys.argv[1], "w").close()
time.sleep(60)
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
        print(f"  FAILED {what}: {str(detail)[:400]}")


def guard(*args: str, stdin: str | None = None, root: Path | None = None, limits: dict | None = None, env: dict | None = None) -> subprocess.CompletedProcess:
    verb, rest = args[0], list(args[1:])
    flags: list[str] = []
    if root:
        flags += ["--root", str(root)]
    if limits is not None:
        flags += ["--limits", json.dumps(limits)]
    return subprocess.run([*PY, str(GUARD), verb, *flags, *rest], input=stdin, capture_output=True, text=True, errors="replace", timeout=60, env=env)


def host_clear() -> bool:
    """Whether the host itself would let a heavy job start. No limit can be loosened to test what happens when it would, so while another project
    holds the memory the cases that need it cannot run: said, and never counted as passing."""
    return guard("check").returncode == 0


def fresh(name: str) -> Path:
    path = BASE / name
    shutil.rmtree(path, ignore_errors=True)
    path.mkdir(parents=True)
    return path


def spawn(args: list[str], cwd: Path, env: dict | None = None) -> subprocess.Popen:
    process = subprocess.Popen(args, cwd=cwd, stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, env=env, start_new_session=True)
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


def named(name: str, cwd: Path, script: str = "sleep 60; true") -> subprocess.Popen:
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


def hook_says(command: str, root: Path, limits: dict | None = None, env: dict | None = None) -> subprocess.CompletedProcess:
    return guard("hook", stdin=json.dumps({"tool_input": {"command": command}}), root=root, limits=limits, env=env)


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


def case_a_figure_not_read_is_never_safe() -> None:
    scope = fresh("unread")
    env = shim(scope, "vm_stat", "echo 'Mach Virtual Memory Statistics: (page size of 16384 bytes)'; echo 'Pages renamed:   5.'")
    done = guard("check", env=env)
    check("`vm_stat` with its labels renamed is not read as zero", done.returncode == 75 and "cannot be read" in done.stdout, done.stdout)
    done = hook_says("touch x", scope, env=env)
    check("and refuses a command", done.returncode == 2 and "cannot judge" in done.stderr, done.stderr)
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
    mine = holding(15, scope)
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


def case_an_agent_is_never_stopped() -> None:
    scope, outside = fresh("agents"), fresh("agents-outside")
    agent = named("claude", scope, f"(cd {outside} && exec bash -c 'sleep 60; true') & (exec bash -c 'sleep 60; true') & (exec -a node sleep 60) & wait; sleep 60")
    its_children = children(agent.pid)
    cursor = named("cursor-agent", scope)
    (scope / "aider").write_text("import time\ntime.sleep(60)\n")
    aider = spawn([*PY, str(scope / "aider")], scope)
    time.sleep(0.2)
    terminal = named("-bash", scope, "sleep 60; sleep 60; true")
    job_node = named("node", scope)
    dashed = named("-something", scope)
    plain = holding(15, scope)
    watch_once(scope, PANIC)
    check("at the danger line everything of ours stops", settle(plain))
    check("a job that happens to be node is ours", settle(job_node))
    check("a name that merely begins with a dash is ours", settle(dashed))
    time.sleep(0.2)
    check("an agent working in the root is never stopped", up(agent))
    check("nor one named by a longer name", up(cursor))
    check("nor one an interpreter runs", up(aider))
    check("nor a login shell", up(terminal))
    alive = [pid for pid in its_children if pid_up(pid)]
    # Its three: a shell elsewhere, left; its own helper, left; a shell here, stopped.
    check("what the agent started here is stopped, and what it started elsewhere and for itself is not", len(its_children) == 3 and len(alive) == 2, (its_children, alive))


def case_what_is_under_a_stopped_process() -> None:
    scope = fresh("spared")
    parent = spawn(["bash", "-c", "(exec -a claude bash -c 'sleep 60; true') & (exec -a git bash -c 'sleep 60; sleep 60; true') & wait; sleep 60"], scope)
    time.sleep(0.2)
    under = {}
    for pid in children(parent.pid):
        shown = subprocess.run(["ps", "-o", "command=", "-p", str(pid)], capture_output=True, text=True).stdout
        under[shown.split(" ", 1)[0]] = pid
    check("there is an agent and a git under it", {"claude", "git"} <= set(under), under)
    watch_once(scope, PANIC)
    check("the shell stops", settle(parent))
    time.sleep(0.2)
    check("an agent under a stopped shell does not", pid_up(under.get("claude", 0)))
    check("nor a small git, which would free nothing and could leave a lock", pid_up(under.get("git", 0)))
    scope = fresh("spared-limit")
    file, pid_file = scope / "ready", scope / "child-pid"
    holder = spawn([*PY, "-c", PARENT, "80", str(file), str(scope), str(pid_file), "exec -a claude sleep 60"], scope)
    ready(file)
    child = int(pid_file.read_text())
    watch_once(scope, {"one_process_gib": 0.05})
    check("through a job's limit too: the holder stops", settle(holder))
    time.sleep(0.2)
    check("and the agent under it does not", pid_up(child))


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
    watch_once(scope, PANIC)
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
    watch_once(scope, {"panic_pressure_level": 0})
    check("at its critical level everything of ours stops", settle(two) and settle(three))


def case_not_ours() -> None:
    scope = fresh("not-ours")
    small = holding(15, scope)
    watch_once(scope, {"stop_used_percent": 0})
    check("pressure with little of ours stops nothing", up(small))
    check("and is recorded as someone else's", any(event["kind"] == "pressure-not-ours" for event in events(scope)), events(scope))
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
    check("`status` says it reads the host", "watcher: running, and reading the host" in guard("status", root=scope).stdout, guard("status", root=scope).stdout)
    # One lock, on the root itself: nothing under `target/` can take it away and leave two.
    shutil.rmtree(scope / "state")
    for _ in range(3):
        guard("ensure", root=scope)
    time.sleep(1.5)
    found = subprocess.run(["pgrep", "-f", f"host_guard.py watch --root {scope}"], capture_output=True, text=True).stdout.split()
    extra = subprocess.run(["pgrep", "-f", f"host_guard.py watch --root {scope} --limits"], capture_output=True, text=True).stdout.split()
    check("however often it is asked for, one stays", len(found) == len(extra) == 1, (found, extra))
    watcher.terminate()
    watcher.wait(timeout=5)


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


def case_a_blind_watcher_is_said() -> None:
    scope = fresh("blind")
    env = shim(scope, "ps", "exit 1")
    watcher = looping_watcher(scope, {}, env=env)
    time.sleep(2.0)
    # This watcher cannot read the table; everything judging it below can.
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


def case_reads_and_what_rides_on_them() -> None:
    for read in (
        "ps -ax", "ps -ax | sort -k3 | head", "vm_stat | head", "top -l 1 | head", "du -sh target", "lsof -p 1", "rm -rf target/x", "sysctl -n hw.memsize 2>/dev/null",
        "git status", "pkill -f x", "mkdir -p target/tmp; export TMPDIR=\"$PWD/target/tmp\"; python3 -I -B scripts/host_guard.py status",
    ):
        check(f"a read: {read}", g.is_read(read))
    for not_one in (
        "ps -ax\ncargo build", "ps $(cargo build)", "ps `cargo build`", "git status\nmake focused", "kill 1\npython3 -c 'x'", "ps-heavy --allocate 40G", "ps -ax; cargo test",
        "git status && make focused", "kill 1 | cargo build", "python3 /other/not_host_guard.py", "python3 scripts/host_guard.py run -- true", "ps -ax > out", "ps -ax &", "top", "",
    ):
        check(f"not a read: {not_one!r}", not g.is_read(not_one))
    for heavy in (
        "cargo test -p x", "make focused", "cd a && cargo build", "FOO=1 cargo build", "timeout 9 cargo test", "for t in a; do cargo test; done", "bash -c 'cargo build'",
        "if cargo build; then :; fi", "env cargo build", "nohup cargo build", "{ cargo build; }", "`cargo build`", "gmake x", "cargo-nextest run", "bash scripts/build_book.sh",
        "target/m/install/archogen build s --out d", "archogen-plan-check --help",
    ):
        check(f"heavy: {heavy}", bool(g.HEAVY.search(g.code_of(heavy))))
    for light in (
        "ls", "cargo fmt -p x", "git commit -F m", "grep -n 'make ' Makefile", "git commit -m 'make cargo build'", "echo cargo build", "echo archogen-plan-check",
        "grep -rn archogen-plan-check docs/", "git commit -m 'x (make sure it holds)'", "git commit -F - <<'M'\nmake sure it holds\nM\n",
    ):
        check(f"not heavy: {light!r}", not g.HEAVY.search(g.code_of(light)))


def case_hook() -> None:
    scope = fresh("hook")
    try:
        done = hook_says("touch x", scope)
        check("a light command passes on a clear host", done.returncode == 0, done.stderr)
        done = hook_says("cargo test -p x", scope, {"start_used_percent": 0})
        check("a heavy command is refused over the start line", done.returncode == 2 and "heavy" in done.stderr, done.stderr)
        check("a light one is not", hook_says("touch x", scope, {"start_used_percent": 0}).returncode == 0)
        for limit in ({"stop_used_percent": 0}, {"stop_pressure_level": 0}, {"stop_repo_free_gib": 1e9}):
            done = hook_says("touch x", scope, limit)
            check(f"at a stop line every command is refused: {limit}", done.returncode == 2 and "refused" in done.stderr, done.stderr)
        for reads in ("ps -ax", "ps -ax | head", "rm -rf x", f"python3 -I -B {GUARD} status"):
            check(f"but the host can still be read and helped: {reads}", hook_says(reads, scope, {"stop_used_percent": 0}).returncode == 0)
        builder = named("cargo", scope)
        if host_clear():
            done = hook_says("cargo build", scope)
            check("one heavy job at a time", done.returncode == 2 and "one heavy job" in done.stderr, done.stderr)
        else:
            not_run.append("one heavy job at a time")
        check("a light command beside it passes", hook_says("touch x", scope).returncode == 0)
        builder.kill()
        builder.wait()
        with open(scope / "state" / "events.log", "a") as log:
            log.write(json.dumps({"kind": "stopped", "at": "now", "reasons": ["why"], "stopped": [{"pid": 1, "name": "x", "gib": 1.0, "command": "x"}]}) + "\n")
        done = hook_says("touch x", scope)
        check("what was stopped is told before anything but a read runs", done.returncode == 2 and "stopped x" in done.stderr, done.stderr)
        check("once", hook_says("touch x", scope).returncode == 0)
        time.sleep(0.5)
        check("the hook started a watcher", "watcher: running" in guard("status", root=scope).stdout)
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
        check("bad lines do not cost the good one", done.returncode == 2 and "stopped told" in done.stderr, done.stderr)
        check("and are read past", hook_says("touch x", scope).returncode == 0)
        check("`status` reads past them too", guard("status", root=scope).returncode == 0)
        (state / "seen").write_text("-5")
        done = hook_says("touch x", scope)
        check("a nonsense position starts again rather than crashing", done.returncode == 2 and "stopped told" in done.stderr, done.stderr)
        guard("stop", root=scope)
    finally:
        guard("stop", root=scope)
    scope = fresh("state-a-file")
    (scope / "state").write_text("a file where the state goes")
    done = hook_says("touch x", scope)
    check("state it cannot use refuses, saying so", done.returncode == 2 and "cannot judge" in done.stderr, done.stderr)
    check("and a read still runs, so it can be repaired", hook_says("rm -rf state", scope).returncode == 0)


def case_run() -> None:
    scope = fresh("run")
    try:
        marker = scope / "ran"
        done = guard("run", "--", "touch", str(marker), root=scope, limits={"start_used_percent": 0})
        check("over the start line a job is not run", done.returncode == 75 and not marker.exists() and "not started" in done.stderr, done)
        began = time.monotonic()
        done = subprocess.run([*PY, str(GUARD), "run", "--root", str(scope), "--limits", json.dumps({"start_used_percent": 0}), "--wait", "1", "--", "true"], capture_output=True, text=True, timeout=30)
        check("a job waits for room as long as it was told, and no longer", done.returncode == 75 and 1.0 <= time.monotonic() - began < 20, done)
        if not host_clear():
            not_run.extend(["a job's exit status is kept", "a job runs at a lower priority and a third of the cores", "`run` holds one job at a time"])
            return
        done = guard("run", "--", "bash", "-c", "exit 7", root=scope)
        check("a job's exit status is kept", done.returncode == 7, done)
        done = guard("run", "--", "bash", "-c", 'echo "$CARGO_BUILD_JOBS $RUST_TEST_THREADS $(ps -o nice= -p $$)"', root=scope)
        words = done.stdout.split()
        check("a job runs at a lower priority and a third of the cores", len(words) == 3 and words[2] == "10" and int(words[0]) == max(2, (os.cpu_count() or 4) // 3), done.stdout)
        builder = named("cargo", scope)
        done = guard("run", "--", "touch", str(marker), root=scope)
        check("`run` waits its turn behind a build of ours", done.returncode == 75 and "one heavy job" in done.stderr and not marker.exists(), done)
        builder.kill()
        builder.wait()
        first = subprocess.Popen([*PY, str(GUARD), "run", "--root", str(scope), "--", "sleep", "3"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
        started.append(first)
        time.sleep(0.7)
        done = guard("run", "--", "touch", str(marker), root=scope)
        check("and behind another `run`", done.returncode == 75 and "holds its turn" in done.stderr and not marker.exists(), done)
        first.wait(timeout=10)
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


def main() -> int:
    # This test holds a few hundred MiB. On a host already full, that is the wrong thing to do.
    if not host_clear():
        print("host-guard self-test: not run: the host is over the start line, and this test would add to it")
        print(guard("status").stdout)
        return 20
    BASE.mkdir(parents=True, exist_ok=True)
    for case in (
        case_only_tightened, case_check_refuses, case_a_figure_not_read_is_never_safe, case_one_process, case_all_processes, case_what_a_stopped_process_started,
        case_another_project_is_never_ours, case_an_agent_is_never_stopped, case_what_is_under_a_stopped_process, case_names_and_zombies, case_a_command_line_that_is_no_text,
        case_pressure, case_not_ours, case_growing, case_a_small_writer_is_counted, case_the_watcher_outlives_its_own_trouble, case_a_stale_pid_names_nobody,
        case_the_watcher_stays_while_a_job_runs, case_a_blind_watcher_is_said, case_reads_and_what_rides_on_them, case_hook, case_a_damaged_log, case_run,
    ):
        before = len(failures)
        try:
            case()
        except Exception as error:
            check(f"{case.__name__} ran to its end", False, repr(error))
        finally:
            clean_up()
        print(f"  {case.__name__}: {'ok' if len(failures) == before else 'FAILED'}")
    print(f"host-guard self-test: {passed} passed, {len(failures)} failed, {len(not_run)} not run")
    for what in not_run:
        print(f"  not run, the host being over the start line: {what}")
    if failures:
        return 1
    # `incomplete`, as a verification tier says it: not a pass.
    return 20 if not_run else 0


if __name__ == "__main__":
    sys.exit(main())
