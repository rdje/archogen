#!/usr/bin/env bash
# scripts/check_stated_order.sh — STATED-ORDER: every place that restates an order — which leaf is next, which follow,
# which entry is newest — says what its source says (leaf `PROGRAM.20.1`).
#
# ⭐ WHY THIS EXISTS. The defect class `PROGRAM.20` records has a shape that no figure-shaped sweep can see, because
# nothing in it is a number: an index row naming a leaf whose own status is already `done`; a successor clause naming
# the frontier as its own successor; a changelog whose header says "newest first" and whose top entry was seven
# commits old. Each was a copy that read plausibly, and each went stale at the commit that moved its source without
# looking at it. `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` names the remedy: a copy of mechanically owned state is kept only
# as a *verified copy*, with a verifier that runs.
#
# THE LEGS:
#   1. FRONTIER TABLES — each tree's `## Current Frontier` row states the same status as the leaf it names.
#   2. RESTATED HEADS — `LIVE_STATUS.md` (`frontier \`X\``), `docs/TASK_TREE.md` (the row's leading id) and `MEMORY.md`
#      (`frontier \`X\`` and the next action) name the tree's order-1 frontier leaf — the next action may name one of
#      its children — and never a `done` leaf.
#   3. SUCCESSOR LISTS — a `Then \`.a\`, \`.b\`, …` clause equals the frontier's following order, and never names the
#      head it follows.
#   4. THE CHANGELOG — `CHANGELOG.md`'s entry ids, first per entry, are strictly decreasing: newest first, as its header
#      says.
#
# ⚠️ HONEST LIMIT: it checks the restated *order*, not the prose around it — a row may name the right leaf and describe
# it wrongly. That residue is review.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

fail=0
note() { printf 'STATED-ORDER: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The frontier of tree $1, in order: `id<TAB>status` lines.
frontier() {
  awk '
    /^## Current Frontier$/ { on = 1; next }
    on && /^## / { exit }
    on && /^\| [0-9]+ \| `/ {
      split($0, c, /[[:space:]]*\|[[:space:]]*/)
      id = c[3]; gsub(/`/, "", id); st = c[4]; gsub(/`/, "", st)
      print c[2] + 0 "\t" id "\t" st
    }' "docs/tasks/$1.md" | sort -n -k1,1 | cut -f2-
}

# The status a leaf's own body states.
leaf_status() { # $1 tree, $2 id
  awk -v want="- ID: \`$2\`" '$0 == want { f = 1; next } f && /^  Status: `/ { s = $0; sub(/^  Status: `/, "", s); sub(/`.*/, "", s); print s; exit } f && /^- ID: `/ { exit }' "docs/tasks/$1.md"
}

trees() { for f in docs/tasks/*.md; do n="$(basename "$f" .md)"; [ "$n" = TEMPLATE ] || printf '%s\n' "$n"; done; }

# $1 = where, $2 = tree, $3 = restated head, $4 = "child-ok" to accept a descendant of the head
check_head() {
  local where="$1" tree="$2" head="$3" mode="${4:-}" first st
  [ -f "docs/tasks/$tree.md" ] || { note "$where names tree \`$tree\`, which has no docs/tasks/$tree.md"; return; }
  first="$(frontier "$tree" | head -n 1 | cut -f1)"
  if [ "$head" != "$first" ] && ! { [ "$mode" = "child-ok" ] && [ "${head#"$first".}" != "$head" ]; }; then
    note "$where names \`$head\` as \`$tree\`'s next leaf, and its tree's frontier begins with \`${first:-nothing}\`"
  fi
  st="$(leaf_status "$tree" "$head")"
  [ -n "$st" ] || { note "$where names \`$head\`, which no leaf of docs/tasks/$tree.md declares"; return; }
  [ "$st" != "done" ] || note "$where names \`$head\` as next, and that leaf's status is \`done\`"
}

scan() {
  local tree id st own
  # 1. frontier tables against leaf bodies
  while IFS= read -r tree; do
    while IFS=$'\t' read -r id st; do
      [ -n "$id" ] || continue
      own="$(leaf_status "$tree" "$id")"
      if [ -z "$own" ]; then note "docs/tasks/$tree.md: its frontier lists \`$id\`, which it does not declare"
      elif [ "$own" != "$st" ]; then note "docs/tasks/$tree.md: its frontier says \`$id\` is \`$st\`, and the leaf says \`$own\`"; fi
    done < <(frontier "$tree")
  done < <(trees)
  # 2 and 3. restated heads and successor lists
  local line head rest then want
  if [ -f LIVE_STATUS.md ]; then
    while IFS= read -r line; do
      tree="$(printf '%s' "$line" | sed -nE 's/^\| `([A-Z0-9]+)` — .*/\1/p')"
      head="$(printf '%s' "$line" | grep -oE 'frontier `[A-Z0-9]+\.[0-9.]+`' | head -n 1 | grep -oE '`[^`]+`' | tr -d '`')"
      [ -n "$tree" ] && [ -n "$head" ] && check_head "LIVE_STATUS.md's \`$tree\` row" "$tree" "$head"
    done < LIVE_STATUS.md
  fi
  if [ -f docs/TASK_TREE.md ]; then
    while IFS= read -r line; do
      tree="$(printf '%s' "$line" | sed -nE 's/^\| \[`([A-Z0-9]+)`\]\(tasks\/.*/\1/p')"
      [ -n "$tree" ] || continue
      rest="$(printf '%s' "$line" | awk -F' \\| ' '{ print $3 }')"
      head="$(printf '%s' "$rest" | sed -nE 's/^`([A-Z0-9]+\.[0-9.]+)`.*/\1/p')"
      [ -n "$head" ] || continue
      check_head "docs/TASK_TREE.md's \`$tree\` row" "$tree" "$head"
      then="$(printf '%s' "$rest" | sed -nE 's/.*Then ((`\.[0-9.]+`(, )?)+).*/\1/p' | grep -oE '`\.[0-9.]+`' | tr -d '`' | sed "s/^/$tree/" | paste -sd' ' -)"
      if [ -n "$then" ]; then
        want="$(frontier "$tree" | cut -f1 | tail -n +2 | head -n "$(printf '%s\n' $then | grep -c .)" | paste -sd' ' -)"
        [ "$then" = "$want" ] || note "docs/TASK_TREE.md's \`$tree\` row says the next leaves are \`$then\`, and its frontier's are \`$want\`"
        printf '%s\n' $then | grep -qxF -- "$head" && note "docs/TASK_TREE.md's \`$tree\` row names \`$head\` as its own successor"
      fi
    done < docs/TASK_TREE.md
  fi
  if [ -f MEMORY.md ]; then
    line="$(grep -m1 -E '^- \*\*Active tree:\*\* ' MEMORY.md)"
    tree="$(printf '%s' "$line" | sed -nE 's/^- \*\*Active tree:\*\* `([A-Z0-9]+)`.*/\1/p')"
    head="$(printf '%s' "$line" | grep -oE 'frontier `[A-Z0-9]+\.[0-9.]+`' | head -n 1 | grep -oE '`[^`]+`' | tr -d '`')"
    [ -n "$tree" ] && [ -n "$head" ] && check_head "MEMORY.md's active tree" "$tree" "$head"
    head="$(grep -m1 -E '^- \*\*Next action:\*\* \*\*`' MEMORY.md | sed -nE 's/^- \*\*Next action:\*\* \*\*`([A-Z0-9]+\.[0-9.]+)`.*/\1/p')"
    [ -n "$tree" ] && [ -n "$head" ] && check_head "MEMORY.md's next action" "${head%%.*}" "$head" child-ok
  fi
  # 4. the changelog's order
  if [ -f CHANGELOG.md ]; then
    local out
    out="$(awk '/^## / { entry = NR; seen = 0 } entry && !seen && match($0, /ARCHOGEN-[A-Z0-9]+-[0-9]+/) {
             id = substr($0, RSTART, RLENGTH); n = id; sub(/.*-/, "", n); n += 0; seen = 1
             if (have && n >= prev) print "CHANGELOG.md:" entry ": " id " sits below " prev_id ", which is older — newest first, as its header says"
             prev = n; prev_id = id; have = 1 }' CHANGELOG.md)"
    [ -z "$out" ] || while IFS= read -r l; do note "$l"; done <<< "$out"
  fi
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/stated_order/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/tasks"
    git -C "$work" init -q
    cat > "$work/docs/tasks/T.md" <<'MD'
# T

- ID: `T.1`
  Status: `done`
- ID: `T.2`
  Status: `pending`
- ID: `T.2.1`
  Status: `in-progress`
- ID: `T.3`
  Status: `pending`
- ID: `T.4`
  Status: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `T.2` | `pending` | first |
| 2 | `T.3` | `pending` | second |
| 3 | `T.4` | `pending` | third |

## Commit Log
MD
    printf '# S\n\n| Area | Status | Notes |\n| --- | --- | --- |\n| `T` — the tree | In Progress | frontier `T.2`: the next thing |\n' > "$work/LIVE_STATUS.md"
    printf '# I\n\n| Tree | Status | Frontier | Owner |\n| --- | --- | --- | --- |\n| [`T`](tasks/T.md) | `active` | `T.2` — the next thing. Then `.3`, `.4`. | local |\n' > "$work/docs/TASK_TREE.md"
    printf '# M\n\n- **Active tree:** `T` → frontier `T.2`.\n- **Next action:** **`T.2.1`** — its child.\n' > "$work/MEMORY.md"
    printf '# Changelog\n\nNewest first.\n\n## three\n\n`ARCHOGEN-T-0003` (leaf `T.2`).\n\n## two\n\n`ARCHOGEN-T-0002`.\n\n## one\n\n`ARCHOGEN-T-0001`.\n' > "$work/CHANGELOG.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
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
  edit() { sed -i.bak "$2" "$work/$1"; rm -f "$work/$1.bak"; }
  fresh; arm "copies that agree with their sources pass" 0 ""
  fresh; edit docs/tasks/T.md 's/^| 1 | `T.2` | `pending` |/| 1 | `T.2` | `in-progress` |/'
  arm "a frontier row whose status its leaf contradicts is refused" 1 "its frontier says \`T.2\` is \`in-progress\`, and the leaf says \`pending\`"
  fresh; edit LIVE_STATUS.md 's/frontier `T.2`/frontier `T.1`/'
  arm "a head left behind by a closure is refused, and so is naming a done leaf" 1 "that leaf's status is \`done\`"
  fresh; edit docs/TASK_TREE.md 's/| `T.2` — the next thing. Then `.3`, `.4`./| `T.3` — the next thing./'
  arm "an index row naming a leaf that is not the frontier head is refused" 1 "names \`T.3\` as \`T\`'s next leaf, and its tree's frontier begins with \`T.2\`"
  fresh; edit docs/TASK_TREE.md 's/Then `.3`, `.4`./Then `.2`, `.3`./'
  arm "a successor list naming its own head is refused" 1 "names \`T.2\` as its own successor"
  fresh; edit docs/TASK_TREE.md 's/Then `.3`, `.4`./Then `.4`, `.3`./'
  arm "a successor list out of the frontier's order is refused" 1 "says the next leaves are \`T.4 T.3\`, and its frontier's are \`T.3 T.4\`"
  fresh; edit MEMORY.md 's/\*\*`T.2.1`\*\*/**`T.3`**/'
  arm "a next action that is neither the head nor its child is refused" 1 "MEMORY.md's next action names \`T.3\`"
  fresh; edit CHANGELOG.md 's/`ARCHOGEN-T-0002`/`ARCHOGEN-T-0004`/'
  arm "a changelog entry below an older one is refused" 1 "ARCHOGEN-T-0004 sits below ARCHOGEN-T-0003, which is older"
  fresh; edit LIVE_STATUS.md 's/frontier `T.2`/frontier `T.9`/'
  arm "a head naming no declared leaf is refused" 1 "names \`T.9\`, which no leaf of docs/tasks/T.md declares"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real snapshots, frontiers and changelog agree"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "stated-order self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

scan
if [ "$fail" -ne 0 ]; then
  echo "STATED-ORDER: $fail breach(es) — a restated order is a copy, and a copy says what its source says" >&2
  exit 1
fi
echo "stated-order: OK (every frontier table, restated head, successor list and the changelog's order agree with their sources)"
exit 0
