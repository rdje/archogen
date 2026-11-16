#!/usr/bin/env bash
# scripts/check_decision_index.sh — DECISION-INDEX: every record under docs/decisions/ or docs/specs/, in the folder or
# in a partition of it, is linked by its own path from docs/decisions/INDEX.md or from a sub-folder's INDEX.md that an
# index already reached links, and every link those indexes hold resolves (leaves `PROGRAM.39`, `PROGRAM.117.3`;
# docs/decisions/decision_ceilings-bound-reads.md).
#
# ⭐ WHY. `docs/decisions/` may be partitioned by subject into sub-folders, a subject's records together, when a
# listing nears its fan-out (docs/decisions/decision_ceilings-bound-reads.md). The template's own
# index check, `MEMORY-ARCH`, reads `docs/decisions/*.md` alone and is held unchanged (findings §10), so a record in a
# sub-folder would drop out of it silently. This check is the project's counterpart that sees the partitions: a
# record is found by its path from the index, and a link the index holds that leads nowhere is refused.
#
# ⭐ SUB-INDEXES (`PROGRAM.117.3`). A single index grows with every record and would one day bind the folder it lists,
# so an index never has to hold every row: a partition may keep its own `INDEX.md`, read once the root, or an index
# the root reaches, links it. An index under the two folders that nothing reaches is refused, since what it lists
# would be listed nowhere; an index outside them, the reviews' among them, is never read as one. The specifications
# under docs/specs/ are records too, held to the same rule.
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
ROOTS = ("docs/decisions", "docs/specs")
INDEX = "docs/decisions/INDEX.md"
LINK = re.compile(r"\]\(([^)#\s]+\.md)(?:#[^)]*)?\)")
fails = []
if not os.path.exists(INDEX):
    print("DECISION-INDEX: %s is missing" % INDEX, file=sys.stderr); sys.exit(1)
everything = set(subprocess.run(["git", "ls-files"], capture_output=True, text=True).stdout.split())
under = lambda p: any(p.startswith(r + "/") for r in ROOTS)
# The root index, and every index under the two folders that an index already reached links: each link resolved
# against its own index's folder, and a record found by that path.
linked, reached, queue = set(), [], [INDEX]
while queue:
    index = queue.pop(0)
    if index in reached:
        continue
    reached.append(index)
    for rel in sorted(set(LINK.findall(open(index, encoding="utf-8").read()))):
        target = os.path.normpath(os.path.join(os.path.dirname(index), rel))
        if target not in everything:
            fails.append("%s links `%s`, and %s is not a tracked file" % (index, rel, target))
            continue
        linked.add(target)
        if os.path.basename(target) == "INDEX.md" and under(target):
            queue.append(target)
tracked = subprocess.run(["git", "ls-files", "--", *ROOTS], capture_output=True, text=True).stdout.split()
records = [p for p in tracked if p.endswith(".md") and os.path.basename(p) not in ("INDEX.md", "TEMPLATE.md")]
for p in sorted(records):
    if p not in linked:
        fails.append("%s is not linked by its path from %s or from an index it reaches" % (p, INDEX))
for p in sorted(p for p in tracked if os.path.basename(p) == "INDEX.md" and p not in reached):
    fails.append("%s is an index no index reaches from %s, so what it lists is listed nowhere" % (p, INDEX))
for f in fails:
    print("DECISION-INDEX: " + f, file=sys.stderr)
if fails:
    print("DECISION-INDEX: %d breach(es) — every record under %s, its partitions included, is linked from its index"
          % (len(fails), " and ".join(ROOTS)), file=sys.stderr)
    sys.exit(1)
decisions = [p for p in records if p.startswith(ROOTS[0] + "/")]
part = lambda p: os.path.dirname(os.path.relpath(p, ROOTS[0]))
subs = sorted({part(p) for p in decisions} - {""})
print("decision-index: OK (%d record(s), %d in partitions %s, and %d specification(s), each linked by its path from "
      "%d index(es); every link resolves)"
      % (len(decisions), sum(1 for p in decisions if part(p)), ", ".join(subs) or "none",
         len(records) - len(decisions), len(reached)))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/decisions/part"; git -C "$work" init -q
    printf '# a\n' > "$work/docs/decisions/decision_a.md"
    printf '# b\n' > "$work/docs/decisions/part/decision_b.md"
    printf '# t\n' > "$work/docs/decisions/TEMPLATE.md"
    mkdir -p "$work/docs/specs/trust"; printf '# s\n' > "$work/docs/specs/trust/contract.md"
    printf '# Index\n\n| Record |\n| --- |\n| [`decision_a.md`](decision_a.md) |\n| [`decision_b.md`](part/decision_b.md) |\n| [`contract.md`](../specs/trust/contract.md) |\n' > "$work/docs/decisions/INDEX.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
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
  fresh; arm "a specification linked from the index by its path passes" 0 "and 1 specification(s)"
  fresh; printf '# u\n' > "$work/docs/specs/trust/unlisted.md"; git -C "$work" add -A
  arm "a specification no index links is refused (PROGRAM.117.3)" 1 "docs/specs/trust/unlisted.md is not linked"
  fresh; printf '# c\n' > "$work/docs/decisions/part/decision_c.md"
  printf '# Part\n\n| [`decision_c.md`](decision_c.md) |\n' > "$work/docs/decisions/part/INDEX.md"
  printf '| [`part`](part/INDEX.md) |\n' >> "$work/docs/decisions/INDEX.md"; git -C "$work" add -A
  arm "a record its partition's own index links, that index linked from the root, passes" 0 "from 2 index(es)"
  fresh; printf '# c\n' > "$work/docs/decisions/part/decision_c.md"
  printf '# Part\n\n| [`decision_c.md`](decision_c.md) |\n' > "$work/docs/decisions/part/INDEX.md"; git -C "$work" add -A
  arm "an index no index reaches is refused, and what it lists with it" 1 "part/INDEX.md is an index no index reaches"
  fresh; printf '# Part\n\n| [`gone.md`](gone.md) |\n' > "$work/docs/decisions/part/INDEX.md"
  printf '| [`part`](part/INDEX.md) |\n' >> "$work/docs/decisions/INDEX.md"; git -C "$work" add -A
  arm "a dead link in a sub-index is refused" 1 "docs/decisions/part/INDEX.md links \`gone.md\`"
  fresh; printf '# Part\n\n| [`up`](../INDEX.md) |\n' > "$work/docs/decisions/part/INDEX.md"
  printf '| [`part`](part/INDEX.md) |\n' >> "$work/docs/decisions/INDEX.md"; git -C "$work" add -A
  arm "two indexes linking each other end the walk" 0 "from 2 index(es)"
  fresh; mkdir -p "$work/docs/reviews"; printf '# c\n' > "$work/docs/decisions/part/decision_c.md"
  printf '# Reviews\n\n| [`c`](../decisions/part/decision_c.md) |\n' > "$work/docs/reviews/INDEX.md"
  printf '| [`reviews`](../reviews/INDEX.md) |\n' >> "$work/docs/decisions/INDEX.md"; git -C "$work" add -A
  arm "an index outside the two folders lists nothing for them" 1 "docs/decisions/part/decision_c.md is not linked"
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
