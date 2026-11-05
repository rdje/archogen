#!/usr/bin/env bash
# scripts/check_host_guard.sh — HOST-GUARD: nothing here endangers the host it shares (leaf `PROGRAM.127`,
# docs/decisions/decision_host-guard.md).
#
# ⭐ WHY. The machine holds 24 GiB and many other projects. On 2026-11-05 swap reached 62 GiB before anyone here noticed, and the
# director's lines are that RAM use above 88% is the danger zone and above 90% the host may crash. The guard only protects what goes
# through it, so this holds the things that make it unavoidable, and that its lines stay inside the director's.
#
# Usage: bash scripts/check_host_guard.sh
# Exit:  0 = held · 1 = a breach, named
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)" || exit 2
python3 -I -B - "$ROOT" <<'PY'
import ast, json, re, sys
from pathlib import Path

root = Path(sys.argv[1])
breaches = []
guard = root / "scripts" / "host_guard.py"
if not guard.is_file():
    print("HOST-GUARD: scripts/host_guard.py is gone")
    sys.exit(1)
if not (root / "scripts" / "selftest_host_guard.py").is_file():
    breaches.append("scripts/selftest_host_guard.py is gone, so nothing shows the guard stops what it should and nothing else")

# Its lines, as written, inside the director's.
limits = {}
for node in ast.walk(ast.parse(guard.read_text(encoding="utf-8"))):
    if isinstance(node, ast.ClassDef) and node.name == "Limits":
        for item in node.body:
            if isinstance(item, ast.AnnAssign) and isinstance(item.target, ast.Name) and isinstance(item.value, ast.Constant):
                limits[item.target.id] = item.value.value
for name in ("start_used_percent", "stop_used_percent", "panic_used_percent", "stop_pressure_level", "one_process_gib", "all_processes_gib"):
    if name not in limits:
        breaches.append(f"scripts/host_guard.py states no `{name}`")
if {"start_used_percent", "stop_used_percent", "panic_used_percent"} <= limits.keys():
    if limits["panic_used_percent"] > 88:
        breaches.append(f"`panic_used_percent` is {limits['panic_used_percent']}: the director's danger zone starts at 88")
    if not limits["start_used_percent"] < limits["stop_used_percent"] < limits["panic_used_percent"]:
        breaches.append("the lines must rise: start, then stop, then everything of ours stopped")

# Everything `make` compiles or tests goes through it. `cargo fmt` neither.
recipe = None
for line in (root / "Makefile").read_text(encoding="utf-8").splitlines():
    if not line.startswith("\t"):
        found = re.match(r"([a-zA-Z_-]+):", line)
        recipe = found.group(1) if found else recipe
        continue
    command = line.strip()
    if re.match(r"cargo\b", command) and not re.match(r"cargo fmt\b", command):
        breaches.append(f"Makefile `{recipe}` runs `{command}` outside the guard: write it `$(GUARD) {command}`")
if "host_guard.py run" not in (root / "Makefile").read_text(encoding="utf-8"):
    breaches.append("the Makefile's `GUARD` does not run scripts/host_guard.py")

# Every shell command an agent runs here is checked first, a subagent's too.
settings = root / ".claude" / "settings.json"
try:
    hooks = json.loads(settings.read_text(encoding="utf-8")).get("hooks", {}).get("PreToolUse", [])
except (OSError, ValueError) as error:
    hooks = []
    breaches.append(f".claude/settings.json does not read: {error}")
held = [
    hook
    for entry in hooks
    if entry.get("matcher") == "Bash"
    for hook in entry.get("hooks", [])
    if "host_guard.py" in hook.get("command", "") and hook.get("command", "").rstrip().endswith("hook")
]
if not held:
    breaches.append(".claude/settings.json runs no `host_guard.py hook` before a shell command")
elif held[0].get("onFailure") != "block":
    breaches.append("the hook must block when it cannot run: a guard that fails open is no guard")

# What an agent of any harness reads first says so.
if "scripts/host_guard.py" not in (root / "CLAUDE.md").read_text(encoding="utf-8"):
    breaches.append("CLAUDE.md does not name scripts/host_guard.py, so an agent on another harness would not know of it")

for breach in breaches:
    print(f"HOST-GUARD: {breach}")
if breaches:
    sys.exit(1)
print("host-guard: OK (the lines inside the director's; every `make` build, every agent's shell command and CLAUDE.md through it)")
PY
