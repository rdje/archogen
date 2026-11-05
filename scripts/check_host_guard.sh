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

# Its lines, as written: none looser than this. The first three are the director's, or inside them; the rest were measured or chosen, and moving
# one means moving it here too, in a commit that says why.
limits = {}
for node in ast.walk(ast.parse(guard.read_text(encoding="utf-8"))):
    if isinstance(node, ast.ClassDef) and node.name == "Limits":
        for item in node.body:
            if isinstance(item, ast.AnnAssign) and isinstance(item.target, ast.Name) and isinstance(item.value, ast.Constant):
                limits[item.target.id] = item.value.value
AT_MOST = {
    "start_used_percent": 70, "stop_used_percent": 85, "panic_used_percent": 88, "start_pressure_level": 2, "stop_pressure_level": 2, "panic_pressure_level": 4,
    "start_swap_gib": 16, "stop_swap_growth_gib": 3, "all_processes_gib": 6, "one_process_gib": 3, "processes": 512, "stop_disk_growth_gib": 50,
    "worth_stopping_gib": 0.25, "watcher_stale_seconds": 60,
}
AT_LEAST = {"stop_repo_free_gib": 100, "stop_boot_free_gib": 30}
for name, line in AT_MOST.items():
    if name not in limits:
        breaches.append(f"scripts/host_guard.py states no `{name}`")
    elif limits[name] > line:
        breaches.append(f"`{name}` is {limits[name]}, looser than {line}")
for name, line in AT_LEAST.items():
    if name not in limits:
        breaches.append(f"scripts/host_guard.py states no `{name}`")
    elif limits[name] < line:
        breaches.append(f"`{name}` is {limits[name]}, looser than {line}")
for name in limits.keys() - AT_MOST.keys() - AT_LEAST.keys():
    breaches.append(f"`{name}` is a limit this check does not hold: add it")
if {"start_used_percent", "stop_used_percent", "panic_used_percent"} <= limits.keys():
    if not limits["start_used_percent"] < limits["stop_used_percent"] < limits["panic_used_percent"]:
        breaches.append("the lines must rise: start, then stop, then everything of ours stopped")

# Everything `make` compiles or tests goes through it. These recipes are written out rather than read for what they might mean: a loose reading of a shell line
# was passed by `cargo fmt && cargo test`. Changing one means changing it here too, in a commit that says why.
makefile = (root / "Makefile").read_text(encoding="utf-8")
if len(re.findall(r"^GUARD\s*[:+?]?=", makefile, re.M)) != 1 or not re.search(r"^GUARD\s*:=\s*python3 -I -B scripts/host_guard\.py run --wait \d+ --\s*$", makefile, re.M):
    breaches.append("the Makefile's `GUARD` is not `python3 -I -B scripts/host_guard.py run --wait SECONDS --`, defined once")
RECIPES = {
    "focused": ["$(GUARD) cargo xtask verify --tier focused"],
    "integration": ["$(GUARD) cargo xtask verify --tier integration"],
    "tiers": ["$(GUARD) cargo xtask verify --list"],
    "gate": ["$(GUARD) scripts/check_doctrines.sh"],
    "check": ["cargo fmt --all -- --check", "$(GUARD) cargo clippy --all-targets --all-features -- -D warnings", "$(GUARD) cargo test --all"],
    "fmt": ["cargo fmt --all"],
    "clippy": ["$(GUARD) cargo clippy --all-targets --all-features -- -D warnings"],
    "test": ["$(GUARD) cargo test --all"],
    "book": ["$(GUARD) scripts/build_book.sh"],
}
found = {}
target = None
for line in makefile.splitlines():
    if line.startswith("\t"):
        found.setdefault(target, []).append(line.strip())
        continue
    if re.match(r"\s*(?:\.RECIPEPREFIX|-?include)\b", line):
        breaches.append(f"the Makefile says `{line.strip()}`, so what it runs cannot be read from it")
    named = re.match(r"([a-zA-Z_-]+)\s*:(?!=)(.*)", line)
    if named:
        target = named.group(1)
        if ";" in named.group(2):
            breaches.append(f"Makefile `{target}` writes its recipe on its target's line")
for name, recipe in RECIPES.items():
    if found.get(name) != recipe:
        breaches.append(f"Makefile `{name}` runs {found.get(name)}, and this check holds it to {recipe}")
HEAVY = re.compile(r"(?<![\w./-])(?:\S*/)?(?:cargo|rustc|mdbook|build_book\.sh|check_doctrines\.sh)(?![\w.-])")
for name, recipe in found.items():
    if name not in RECIPES:
        for command in recipe:
            bare = command.lstrip("@-")
            if HEAVY.search(bare) and not re.fullmatch(r'echo\s+"[^"]*"', bare):
                breaches.append(f"Makefile `{name}` runs `{command}`, which this check does not hold")

# Every shell command an agent runs here is checked first, a subagent's too, by exactly this, and nothing turns it off or lets it go by.
EXPECTED = 'python3 -I -B "$CLAUDE_PROJECT_DIR/scripts/host_guard.py" hook'
settings = root / ".claude" / "settings.json"
try:
    loaded = json.loads(settings.read_text(encoding="utf-8"))
except (OSError, ValueError) as error:
    loaded = {}
    breaches.append(f".claude/settings.json does not read: {error}")
if loaded.get("disableAllHooks"):
    breaches.append(".claude/settings.json turns every hook off")
held = [
    hook
    for entry in loaded.get("hooks", {}).get("PreToolUse", [])
    if entry.get("matcher") == "Bash"
    for hook in entry.get("hooks", [])
    if hook.get("command") == EXPECTED
]
if not held:
    breaches.append(f".claude/settings.json does not run `{EXPECTED}` before a shell command")
elif held[0].get("onFailure") != "block":
    breaches.append("the hook must block when it cannot run: a guard that fails open is no guard")
elif held[0].get("async"):
    breaches.append("the hook runs in the background, where it refuses nothing")
elif not held[0].get("timeout"):
    breaches.append("the hook states no timeout, so one that hangs would hang every command")
elif set(held[0]) - {"type", "command", "timeout", "onFailure", "statusMessage"}:
    breaches.append(f"the hook says more than this check knows, which may narrow what it sees: {sorted(set(held[0]) - {'type', 'command', 'timeout', 'onFailure', 'statusMessage'})}")

# What an agent of any harness reads first says so.
if "scripts/host_guard.py" not in (root / "CLAUDE.md").read_text(encoding="utf-8"):
    breaches.append("CLAUDE.md does not name scripts/host_guard.py, so an agent on another harness would not know of it")

for breach in breaches:
    print(f"HOST-GUARD: {breach}")
if breaches:
    sys.exit(1)
print("host-guard: OK (every line at or inside its mark; the recipes it holds and every agent's shell command through it; that it stops what it should is the self-test's)")
PY
