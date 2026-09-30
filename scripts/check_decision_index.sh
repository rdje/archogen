#!/usr/bin/env bash
# scripts/check_decision_index.sh — DECISION-INDEX: every decision record, in the folder or in a partition of it, is
# linked from docs/decisions/INDEX.md by its own path, and every link the index holds resolves (leaf `PROGRAM.39`,
# docs/decisions/decision_decisions-folder-ceiling.md).
#
# ⭐ WHY. `docs/decisions/` is partitioned by subject into sub-folders once it nears its ceiling. The template's own
# index check, `MEMORY-ARCH`, reads `docs/decisions/*.md` alone and is held unchanged (findings §10), so a record in a
# sub-folder would drop out of it silently. This check is the project's counterpart that sees the partitions: a
# record is found by its path from the index, and a link the index holds that leads nowhere is refused.
#
# MODES:
#   bash scripts/check_decision_index.sh              # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_decision_index.sh --self-test
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/decision_index"

check() {
  python3 - <<'PY'
import os, re, subprocess, sys
DIR = "docs/decisions"
INDEX = os.path.join(DIR, "INDEX.md")
fails = []
if not os.path.exists(INDEX):
    print("DECISION-INDEX: %s is missing" % INDEX, file=sys.stderr); sys.exit(1)
text = open(INDEX, encoding="utf-8").read()
links = set(re.findall(r"\]\(([^)#\s]+\.md)(?:#[^)]*)?\)", text))
tracked = subprocess.run(["git", "ls-files", "--", DIR], capture_output=True, text=True).stdout.split()
records = [p for p in tracked if p.endswith(".md") and os.path.basename(p) not in ("INDEX.md", "TEMPLATE.md")]
for p in sorted(records):
    rel = os.path.relpath(p, DIR)
    if rel not in links:
        fails.append("%s is not linked from %s by its path, `%s`" % (p, INDEX, rel))
everything = set(subprocess.run(["git", "ls-files"], capture_output=True, text=True).stdout.split())
for rel in sorted(links):
    target = os.path.normpath(os.path.join(DIR, rel))
    if target not in everything:
        fails.append("%s links `%s`, and %s is not a tracked file" % (INDEX, rel, target))
for f in fails:
    print("DECISION-INDEX: " + f, file=sys.stderr)
if fails:
    print("DECISION-INDEX: %d breach(es) — every record under %s, its partitions included, is linked from its index" % (len(fails), DIR), file=sys.stderr)
    sys.exit(1)
subs = sorted({os.path.dirname(os.path.relpath(p, DIR)) for p in records} - {""})
print("decision-index: OK (%d record(s), %d in partitions %s, each linked from the index by its path; every link resolves)"
      % (len(records), sum(1 for p in records if os.path.dirname(os.path.relpath(p, DIR))), ", ".join(subs) or "none"))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/decisions/part"; git -C "$work" init -q
    printf '# a\n' > "$work/docs/decisions/decision_a.md"
    printf '# b\n' > "$work/docs/decisions/part/decision_b.md"
    printf '# t\n' > "$work/docs/decisions/TEMPLATE.md"
    printf '# Index\n\n| Record |\n| --- |\n| [`decision_a.md`](decision_a.md) |\n| [`decision_b.md`](part/decision_b.md) |\n' > "$work/docs/decisions/INDEX.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "records in the folder and in a partition, each linked by its path, pass" 0 "1 in partitions part"
  fresh; printf '# c\n' > "$work/docs/decisions/part/decision_c.md"; git -C "$work" add -A
  arm "a record in a partition that the index does not link is refused" 1 "docs/decisions/part/decision_c.md is not linked"
  fresh; sed -i.bak 's|(part/decision_b.md)|(decision_b.md)|' "$work/docs/decisions/INDEX.md"; rm -f "$work/docs/decisions/INDEX.md.bak"; git -C "$work" add -A
  arm "a partition's record linked by its bare name is refused, and the dead link too" 1 "is not a tracked file"
  fresh; printf '| [`gone.md`](gone.md) |\n' >> "$work/docs/decisions/INDEX.md"; git -C "$work" add -A
  arm "a link the index holds that leads nowhere is refused" 1 "links \`gone.md\`"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real index passes"
  else echo "SELF-TEST: the real index is refused — run the check to see why" >&2; fi
  echo "decision-index self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") check; exit $? ;;
  *) echo "usage: bash scripts/check_decision_index.sh [--self-test]" >&2; exit 2 ;;
esac
