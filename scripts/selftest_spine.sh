#!/usr/bin/env bash
# scripts/selftest_spine.sh — RED arms for the spine's scaffold-owned gates, the two doctrine drivers and the
# handoff tool, run from OUTSIDE them (leaf `PROGRAM.18.2`).
#
# ⭐ WHY FROM OUTSIDE. These scripts are on `scripts/update_scaffold.sh`'s overwrite list and their upstream is
# another repository, so an arm written into one would be erased by the next scaffold sync and could not be
# sent upstream from here (every other repository is read-only). So nothing here edits them: each arm builds
# a fresh scratch repository under `target/doctrine_scratch/`, seeds one breach, and runs the REAL script
# with its working directory there. Two of them locate their root by their own path rather than by git —
# `check_waiver_routing.sh` and `check_no_background_jobs.sh` — so for those the arm runs a byte-for-byte copy
# placed inside the scratch repository: the same bytes, in the place they look. The drivers are armed with
# STUB gates, because their property is "run every registered gate and propagate a failure"; each real gate
# has arms of its own.
#
# ⛔ The oracle is the lesson of `PROGRAM.27`: a refusing arm must print the text naming what it refuses, not
# only exit non-zero — an exit code proves a refusal, never its reason.
#
# CONTRACT: exit 0 = every arm passed; 1 = at least one failed (each named on stderr). Touches nothing outside
# `target/doctrine_scratch/spine_selftest/`, which it removes on success. The one arm that starts a process
# kills and reaps it before moving on.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
WORK="$ROOT/target/doctrine_scratch/spine_selftest"
arms=0; ok=0

# A fresh scratch git repository named $1; prints its path.
repo() {
  local d="$WORK/$1"
  rm -rf "$d"; mkdir -p "$d"
  git -C "$d" init -q
  git -C "$d" config user.name arm; git -C "$d" config user.email arm@example.invalid
  printf '%s' "$d"
}

# $1 = arm name, $2 = expected exit, $3 = text the output must carry ("" for none), $4 = directory to run in,
# $5… = the command.
arm() {
  local name="$1" want="$2" must="$3" dir="$4" out rc; shift 4
  arms=$((arms + 1))
  out="$(cd "$dir" && "$@" 2>&1)"; rc=$?
  if [ "$rc" -ne "$want" ]; then
    echo "SELF-TEST: $name — expected exit $want, got $rc" >&2
    printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
  fi
  if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
    echo "SELF-TEST: $name — exit $rc, but not about \`$must\`, so it is not this arm's outcome:" >&2
    printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
  fi
  ok=$((ok + 1)); echo "  ✅ $name"
}

lines() { local i; for i in $(seq "$1"); do printf 'line %s\n' "$i"; done; }

# ── MEMORY-ARCH ─────────────────────────────────────────────────────────────────────────────────
memory_repo() {
  local d; d="$(repo memory)"
  mkdir -p "$d/docs/decisions" "$d/docs/tasks"
  lines 3 > "$d/MEMORY.md"
  printf -- '- decision_a.md\n' > "$d/docs/decisions/INDEX.md"
  printf 'a\n' > "$d/docs/decisions/decision_a.md"
  printf 'index\n' > "$d/docs/TASK_TREE.md"; printf 'tree\n' > "$d/docs/tasks/X.md"
  printf 'readme\n' > "$d/README.md"; printf 'claude\n' > "$d/CLAUDE.md"
  printf '%s' "$d"
}
G="$ROOT/scripts/check_memory_architecture.sh"
d="$(memory_repo)"; arm "MEMORY-ARCH: a complete memory layout passes" 0 "" "$d" bash "$G"
d="$(memory_repo)"; lines 51 > "$d/MEMORY.md"
arm "MEMORY-ARCH: a resume pointer over the line cap is refused" 1 "MEMORY.md has 51 lines" "$d" bash "$G"
d="$(memory_repo)"; for _ in 1 2 3; do printf '%*s\n' 3000 '' | tr ' ' x; done > "$d/MEMORY.md"
arm "MEMORY-ARCH: a resume pointer over the byte cap is refused" 1 "bytes (> cap 7168)" "$d" bash "$G"
d="$(memory_repo)"; printf 'b\n' > "$d/docs/decisions/decision_b.md"
arm "MEMORY-ARCH: a decision record missing from the index is refused" 1 "decision_b.md is not listed" "$d" bash "$G"
d="$(memory_repo)"; rm "$d/CLAUDE.md"
arm "MEMORY-ARCH: a missing agent bootstrap is refused" 1 "CLAUDE.md (agent bootstrap) is missing" "$d" bash "$G"

# ── DOCPATH ─────────────────────────────────────────────────────────────────────────────────────
G="$ROOT/scripts/check_docpaths.sh"
d="$(repo docpath)"; printf 'see docs/book\n' > "$d/a.md"; git -C "$d" add a.md
arm "DOCPATH: a staged page with repository-relative paths passes" 0 "" "$d" bash "$G"
d="$(repo docpath)"; printf 'see /Users/someone/project/x\n' > "$d/a.md"; git -C "$d" add a.md
arm "DOCPATH: a staged page with a checkout-specific path is refused" 1 "a.md contains checkout-specific" "$d" bash "$G"
d="$(repo docpath)"; printf 'see /Users/someone/project/x\n' > "$d/a.txt"; git -C "$d" add a.txt
arm "DOCPATH: its scope is markdown — a staged .txt is not checked" 0 "" "$d" bash "$G"
d="$(repo docpath)"; printf 'see /Users/someone/project/x\n' > "$d/a.md"
arm "DOCPATH: its scope is the staged diff — an unstaged page is not checked" 0 "" "$d" bash "$G"

# ── TASK-TREE-OWNERSHIP ─────────────────────────────────────────────────────────────────────────
G="$ROOT/scripts/check_task_tree_ownership.sh"
owned_repo() { local d; d="$(repo ownership)"; mkdir -p "$d/crates" "$d/docs/tasks" "$d/scripts"; printf '%s' "$d"; }
d="$(owned_repo)"; printf 'fn a() {}\n' > "$d/crates/a.rs"; git -C "$d" add -A
arm "TASK-TREE-OWNERSHIP: staged code with no task leaf is refused" 1 "code files are staged but no docs/tasks/" "$d" bash "$G"
d="$(owned_repo)"; printf 'fn a() {}\n' > "$d/crates/a.rs"; printf 'leaf\n' > "$d/docs/tasks/X.md"; git -C "$d" add -A
arm "TASK-TREE-OWNERSHIP: staged code beside a task leaf passes" 0 "" "$d" bash "$G"
d="$(owned_repo)"; printf 'fn a() {}\n' > "$d/crates/a.rs"; git -C "$d" add -A
arm "TASK-TREE-OWNERSHIP: the documented exception passes" 0 "" "$d" env SPINE_ALLOW_UNOWNED=1 bash "$G"
# ⚠️ Pins the gate's OWN scope, which is narrower than `.doctrine/code_paths.txt`: a script is not "code" to
# it. `TASK-ACCEPTANCE` reads the seam and still refuses an unowned script, so the gap is covered — but it is
# the scaffold's to close, and this arm says what the gate does today.
d="$(owned_repo)"; printf 'echo\n' > "$d/scripts/a.sh"; git -C "$d" add -A
arm "TASK-TREE-OWNERSHIP: its code is Rust and Cargo — a staged script alone passes it" 0 "" "$d" bash "$G"

# ── README-STABILITY ────────────────────────────────────────────────────────────────────────────
G="$ROOT/scripts/check_readme_stability.sh"
readme_repo() { local d; d="$(repo readme)"; printf 'A landing page. See README_POLICY.md.\n' > "$d/README.md"; printf 'policy\n' > "$d/README_POLICY.md"; printf '%s' "$d"; }
d="$(readme_repo)"; arm "README-STABILITY: a small linked landing page passes" 0 "README-STABILITY: OK" "$d" bash "$G"
d="$(readme_repo)"; { lines 300; printf 'See README_POLICY.md.\n'; } > "$d/README.md"
arm "README-STABILITY: a page over the line cap is refused" 1 "README.md is 301 lines" "$d" bash "$G"
d="$(readme_repo)"; { printf 'See README_POLICY.md.\n'; printf '%*s\n' 16500 '' | tr ' ' x; } > "$d/README.md"
arm "README-STABILITY: a page over the byte cap is refused" 1 "bytes (> cap 16384)" "$d" bash "$G"
d="$(readme_repo)"; printf 'See README_POLICY.md.\nReleased on 2026-01-02.\n' > "$d/README.md"
arm "README-STABILITY: release history on the landing page is refused" 1 "date-stamped line" "$d" bash "$G"
d="$(readme_repo)"; printf 'A landing page.\n' > "$d/README.md"
arm "README-STABILITY: a page that stops linking its policy is refused" 1 "no longer links README_POLICY.md" "$d" bash "$G"
d="$(readme_repo)"; rm "$d/README_POLICY.md"
arm "README-STABILITY: a missing policy is refused outright" 2 "README-STABILITY: REFUSED" "$d" bash "$G"

# ── WAIVER-ROUTING (locates its root by its own path: a byte-for-byte copy runs inside the repository) ──
waiver_repo() {
  local d; d="$(repo waiver)"; mkdir -p "$d/scripts" "$d/docs/tasks"
  cp "$ROOT/scripts/check_waiver_routing.sh" "$d/scripts/"
  printf '# T\n\n- ID: `T.1`\n' > "$d/docs/tasks/T.md"; git -C "$d" add -A; git -C "$d" commit -q -m base
  printf '%s' "$d"
}
d="$(waiver_repo)"; printf 'the signatures do not apply to this leaf\n' >> "$d/docs/tasks/T.md"; git -C "$d" add -A
arm "WAIVER-ROUTING: a waiver with no owning leaf is refused" 1 "docs/tasks/T.md states a gate does not apply" "$d" bash scripts/check_waiver_routing.sh
d="$(waiver_repo)"; printf 'the signatures do not apply (gate gap owned by TREE.5)\n' >> "$d/docs/tasks/T.md"; git -C "$d" add -A
arm "WAIVER-ROUTING: a waiver that names its owner passes" 0 "waiver-routing: OK" "$d" bash scripts/check_waiver_routing.sh
d="$(waiver_repo)"
arm "WAIVER-ROUTING: nothing staged in a task tree passes" 0 "" "$d" bash scripts/check_waiver_routing.sh
cmp -s "$ROOT/scripts/check_waiver_routing.sh" "$d/scripts/check_waiver_routing.sh" \
  || { arms=$((arms + 1)); echo "SELF-TEST: the WAIVER-ROUTING copy is not byte-identical to the script" >&2; }

# ── KNOWLEDGE-MAP (the check calls the generator at the repository's own path: both are copied) ──
map_repo() {
  local d; d="$(repo knowledge)"; mkdir -p "$d/knowledge-map/scripts" "$d/docs/tasks" "$d/docs/decisions"
  cp "$ROOT/knowledge-map/scripts/gen_knowledge_map.sh" "$ROOT/knowledge-map/scripts/check_knowledge_map.sh" "$d/knowledge-map/scripts/"
  printf 'tree\n' > "$d/docs/tasks/X.md"; printf 'decision\n' > "$d/docs/decisions/decision_a.md"
  (cd "$d" && bash knowledge-map/scripts/gen_knowledge_map.sh > KNOWLEDGE_MAP.md)
  printf '%s' "$d"
}
d="$(map_repo)"; arm "KNOWLEDGE-MAP: a regenerated map passes" 0 "" "$d" bash knowledge-map/scripts/check_knowledge_map.sh
d="$(map_repo)"; printf 'new tree\n' > "$d/docs/tasks/Y.md"
arm "KNOWLEDGE-MAP: a source added without regenerating is refused" 1 "KNOWLEDGE_MAP.md is out of sync" "$d" bash knowledge-map/scripts/check_knowledge_map.sh
d="$(map_repo)"; rm "$d/KNOWLEDGE_MAP.md"
arm "KNOWLEDGE-MAP: a missing map is refused" 1 "is missing" "$d" bash knowledge-map/scripts/check_knowledge_map.sh

# ── the drivers, with stub gates at every path each one registers ───────────────────────────────
stub() { mkdir -p "$(dirname "$1")"; printf '#!/usr/bin/env bash\necho "stub %s"\nexit %s\n' "$1" "$2" > "$1"; chmod +x "$1"; }
driver_repo() { # $1 = driver path relative to the root; every path it names gets a passing stub
  local d paths p; d="$(repo "driver-$(basename "$1" .sh)")"
  mkdir -p "$d/$(dirname "$1")"; cp "$ROOT/$1" "$d/$1"; chmod +x "$d/$1"
  paths="$(grep -oE '(scripts|knowledge-map/scripts)/check_[a-z0-9_.]+\.sh' "$ROOT/$1" | sort -u | grep -vxF "$1")"
  for p in $paths; do stub "$d/$p" 0; done
  printf '%s' "$d"
}
D=scripts/check_doctrines.sh
d="$(driver_repo "$D")"; arm "the universal driver: every gate passing is green" 0 "all doctrines green" "$d" bash "$D"
d="$(driver_repo "$D")"; stub "$d/scripts/check_docpaths.sh" 1
arm "the universal driver: one failing gate blocks, and is named" 1 "❌ DOCPATH" "$d" bash "$D"
d="$(driver_repo "$D")"; rm "$d/scripts/check_table_arity.sh"
arm "the universal driver: a registered gate that is missing blocks" 1 "?? TABLE-ARITY-RATCHET" "$d" bash "$D"
D=scripts/check_doctrines.project.sh
d="$(driver_repo "$D")"; arm "the project driver: every gate passing exits 0" 0 "" "$d" bash "$D"
d="$(driver_repo "$D")"; stub "$d/scripts/check_language_freeze.sh" 1
arm "the project driver: one failing gate blocks, and is named" 1 "PROJECT: LANGUAGE-FREEZE breach" "$d" bash "$D"
d="$(driver_repo "$D")"; rm "$d/scripts/check_repository_boundary.sh"
arm "the project driver: a registered gate that is missing blocks" 1 "REPOSITORY-BOUNDARY — scripts/check_repository_boundary.sh is missing" "$d" bash "$D"

# ── the handoff tool (locates its root by its own path: a copy runs inside the scratch repository) ──
d="$(repo handoff)"; mkdir -p "$d/scripts"; cp "$ROOT/scripts/check_no_background_jobs.sh" "$d/scripts/"
printf 'held\n' > "$d/hold.txt"
arm "the handoff tool: nothing holding the repository is handoff-ready" 0 "handoff: OK" "$d" bash scripts/check_no_background_jobs.sh
sleep 30 < "$d/hold.txt" &
holder=$!
arm "the handoff tool: a process holding a file in the repository blocks" 1 "STILL RUNNING" "$d" bash scripts/check_no_background_jobs.sh
kill "$holder" 2>/dev/null; wait "$holder" 2>/dev/null
arm "the handoff tool: once the holder is gone it is handoff-ready again" 0 "handoff: OK" "$d" bash scripts/check_no_background_jobs.sh

echo "spine self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
if [ "$ok" -eq "$arms" ]; then
  rm -rf "$WORK"
  exit 0
fi
exit 1
