#!/usr/bin/env bash
# scripts/check_commit_log_rows.sh — COMMIT-LOG-ROWS: every work-unit commit has its row in a task tree's Commit Log
# (leaf `PROGRAM.46`).
#
# ⭐ WHY THIS EXISTS. `COMMIT.md` has each commit's subject name its work unit, `ARCHOGEN-<TREE>-NNNN (leaf <ID>)`,
# and each tree keeps a Commit Log of them; nothing checked that the row was ever written. `ARCHOGEN-M2-0263` and
# `ARCHOGEN-M2-0280` both landed without one on `2026-10-01`, each found by hand a commit later — and the census taken
# when this was written found 18 of the 260 commits whose subject has that form with no row at all. Twenty older
# commits name a work unit without `(leaf …)`, predating the form, and are outside the rule.
#
# THE RULE. Every work-unit id in a subject of `HEAD`'s history, and the pending commit's — the first line of the
# commit message file `.doctrine/commit_message_file` names, as `TASK-ACCEPTANCE` reads it — appears in a Commit Log
# row: a table line (`| …`) under `docs/tasks/` or `docs/task-history/` carrying `` `ARCHOGEN-<TREE>-NNNN (leaf ``.
# The ids that had no row when this landed are [`BACKLOG`] below, measured, which may shrink and may not grow: an
# entry that gains a row, or names no commit, is refused until it is removed.
#
# ⚠️ HONEST LIMIT: it proves a row exists, not that the row says anything true, nor that it sits in the right tree.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs its arms in scratch repositories
# under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/commit_log_rows"

# The work-unit commits with no row when this gate landed (`2026-10-01`), measured by the check itself. Override with
# `COMMIT_LOG_BACKLOG` (space-separated), which the self-test does.
BACKLOG_DEFAULT="ARCHOGEN-BOOTSTRAP-0001 ARCHOGEN-LINKEDSPEC-0049 ARCHOGEN-M1-0085 ARCHOGEN-M1-0115 ARCHOGEN-M1-0117
ARCHOGEN-M1-0118 ARCHOGEN-M2-0191 ARCHOGEN-M2-0213 ARCHOGEN-M2-0244 ARCHOGEN-PROGRAM-0047 ARCHOGEN-PROGRAM-0084
ARCHOGEN-PROGRAM-0106 ARCHOGEN-PROGRAM-0125 ARCHOGEN-PROGRAM-0212 ARCHOGEN-PROGRAM-0214 ARCHOGEN-PROGRAM-0221
ARCHOGEN-PROGRAM-0232 ARCHOGEN-S0-0154"

check() {
  COMMIT_LOG_BACKLOG="${COMMIT_LOG_BACKLOG-$BACKLOG_DEFAULT}" python3 - <<'PY'
import os, re, subprocess, sys
ID = re.compile(r"^(ARCHOGEN-[A-Z0-9]+-[0-9]{4}) \(leaf ")
backlog = set(os.environ.get("COMMIT_LOG_BACKLOG", "").split())

subjects = subprocess.run(["git", "log", "--format=%s"], capture_output=True, text=True).stdout.splitlines()
ids = {m.group(1) for s in subjects if (m := ID.match(s))}

# The pending commit, named by the message file `TASK-ACCEPTANCE` reads.
message = "git_message_brief.txt"
if os.path.exists(".doctrine/commit_message_file"):
    for line in open(".doctrine/commit_message_file", encoding="utf-8"):
        line = line.strip()
        if line and not line.startswith("#"):
            message = line
            break
pending = None
if os.path.exists(message):
    first = open(message, encoding="utf-8").readline().strip()
    if (m := ID.match(first)):
        pending = m.group(1)
        ids.add(pending)

files = subprocess.run(["git", "ls-files", "-c", "-o", "--exclude-standard", "--", "docs/tasks", "docs/task-history"],
                       capture_output=True, text=True).stdout.split()
rows = set()
ROW = re.compile(r"`(ARCHOGEN-[A-Z0-9]+-[0-9]{4}) \(leaf ")
for path in files:
    if not path.endswith(".md") or not os.path.exists(path):
        continue
    for line in open(path, encoding="utf-8"):
        if line.startswith("|"):
            rows.update(ROW.findall(line))

fails = []
for i in sorted(ids - rows - backlog):
    fails.append("%s has no row in any tree's Commit Log%s" % (i, " (the pending commit)" if i == pending else ""))
for i in sorted(backlog & rows):
    fails.append("%s is in the backlog and now has a row: remove it from BACKLOG" % i)
for i in sorted(backlog - ids):
    fails.append("%s is in the backlog and names no commit in this history: remove it from BACKLOG" % i)
for f in fails:
    print("COMMIT-LOG-ROWS: " + f, file=sys.stderr)
if fails:
    print("COMMIT-LOG-ROWS: %d breach(es) — every work-unit commit has a row in its tree's Commit Log" % len(fails),
          file=sys.stderr)
    sys.exit(1)
print("commit-log-rows: OK (%d work-unit commit(s), each with a Commit Log row; %d in the backlog, which may only shrink)"
      % (len(ids) - len(backlog & ids), len(backlog & ids)))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  commit() { # $1 = subject
    git -C "$work" add -A && git -C "$work" -c user.name=t -c user.email=t@t commit -q --allow-empty -m "$1"
  }
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/tasks"; git -C "$work" init -q
    printf '# T\n\n## Commit Log\n\n| Leaf | Commit | Notes |\n| --- | --- | --- |\n| `T.1` | `ARCHOGEN-T-0001 (leaf T.1)` | one |\n' \
      > "$work/docs/tasks/T.md"
    commit "ARCHOGEN-T-0001 (leaf T.1): one"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, $4 = backlog
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && COMMIT_LOG_BACKLOG="${4:-}" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a commit with its row passes" 0 "1 work-unit commit(s)"
  fresh; commit "ARCHOGEN-T-0002 (leaf T.1): two"
  arm "a commit with no row is refused" 1 "ARCHOGEN-T-0002 has no row"
  fresh; printf 'ARCHOGEN-T-0003 (leaf T.1): three\n' > "$work/git_message_brief.txt"
  arm "the pending commit with no row is refused" 1 "ARCHOGEN-T-0003 has no row in any tree's Commit Log (the pending commit)"
  printf '| `T.1` | `ARCHOGEN-T-0003 (leaf T.1)` | three |\n' >> "$work/docs/tasks/T.md"
  arm "the pending commit whose row is written passes" 0 "2 work-unit commit(s)"
  fresh; commit "ARCHOGEN-T-0004 (leaf T.1): four"
  arm "a commit in the backlog passes" 0 "1 in the backlog" "ARCHOGEN-T-0004"
  arm "a backlog entry that has a row is refused" 1 "ARCHOGEN-T-0001 is in the backlog and now has a row" "ARCHOGEN-T-0001 ARCHOGEN-T-0004"
  arm "a backlog entry naming no commit is refused" 1 "ARCHOGEN-T-0099 is in the backlog and names no commit" "ARCHOGEN-T-0004 ARCHOGEN-T-0099"
  fresh; printf 'mention ARCHOGEN-T-0005 (leaf T.1) in prose only\n' >> "$work/docs/tasks/T.md"; commit "ARCHOGEN-T-0005 (leaf T.1): five"
  arm "a mention outside a table row is not a row" 1 "ARCHOGEN-T-0005 has no row"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real history passes"
  else echo "SELF-TEST: the real history is refused — run the check to see why" >&2; fi
  echo "commit-log-rows self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") check; exit $? ;;
  *) echo "usage: bash scripts/check_commit_log_rows.sh [--self-test]" >&2; exit 2 ;;
esac
