#!/usr/bin/env bash
# scripts/check_history_ledgers.sh — HISTORY-LEDGERS: the changelog and the development notes as rolling ledgers
# (leaf `PROGRAM.31`, docs/decisions/decision_history-ledgers.md, LIVE_DOCUMENT_SIZE_CONTAINMENT.md's
# `rolling_ledger`).
#
# ⭐ WHY. Both files gain an entry with nearly every commit, and nothing bounded them: `CHANGELOG.md` had reached
# 327 233 bytes in 17 days. Rotating by month, as first proposed, would have sealed nothing, because the whole
# history was one month. So the boundary is a count of entries: whenever a live ledger holds twice its window,
# its oldest window-full is sealed, byte for byte, into the next numbered segment under `docs/history/<ledger>/`,
# and `docs/history/INDEX.md` records the segment's range, counts and digest.
#
# MODES:
#   bash scripts/check_history_ledgers.sh            # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_history_ledgers.sh --seal     # perform every rollover due, prove the reconstruction, then check
#   bash scripts/check_history_ledgers.sh --self-test
#
# THE GATE'S LEGS, per ledger:
#   1. every segment's lines, bytes, entry count and sha256 are its index row's, so a sealed segment cannot change;
#   2. the segments and the rows correspond one to one, numbered from 0001 without a gap;
#   3. ACROSS HISTORY, every row any committed version of the index held is still there, unchanged, and every
#      segment is byte for byte what the commit that added it wrote — so CI, where HEAD is the commit under test,
#      catches a segment and its row forged together as the pre-commit hook does (leaf `PROGRAM.40`);
#   4. the live file holds fewer than twice its window of entries, or a rollover is required;
#   5. once a segment exists, the live header names docs/history/INDEX.md;
#   6. the order is continuous: work-unit numbers strictly fall, and dates never rise, from the live file's first
#      entry through the newest segment to the oldest.
#
# ⚠️ HONEST LIMIT: a first commit that seals and forges at once is the review's to see, as the catalog's lock is
# (decision_catalog-records.md §0); history rewritten under the gate is premise 2 and 3's there too.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

INDEX="docs/history/INDEX.md"
SCRATCH="$ROOT/target/doctrine_scratch/history_ledgers"
# name | live file | segment directory | entry heading (ERE) | window
LEDGERS=(
  "changelog|CHANGELOG.md|docs/history/changelog|^## |20"
  "dev-notes|DEV_NOTES.md|docs/history/dev-notes|^## _[(]|10"
)
fail=0
note() { printf 'HISTORY-LEDGERS: %s\n' "$1" >&2; fail=$((fail + 1)); }

sha() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi
}

# The start line of every entry of file $1 — a line matching ERE $2, outside code fences. An entry runs to the next
# entry's start, and the last one to the end of the file.
entries() {
  awk -v pattern="$2" '/^```/ { fenced = !fenced } !fenced && $0 ~ pattern { print NR }' "$1"
}

count_entries() { entries "$1" "$2" | wc -l | tr -d ' '; }

# The label of an entry: its first work-unit id, else the date in its heading, else its heading.
label() { # $1 = file, $2 = first line, $3 = last line
  sed -n "$2,$3p" "$1" | awk '
    match($0, /ARCHOGEN-[A-Z0-9]+-[0-9]+/) { print substr($0, RSTART, RLENGTH); found = 1; exit }
    NR == 1 { heading = $0 }
    NR == 1 && match($0, /[0-9]{4}-[0-9]{2}-[0-9]{2}/) { date = substr($0, RSTART, RLENGTH) }
    END { if (!found) print (date != "" ? date : substr(heading, 4, 60)) }'
}

# The rows of ledger $1's table in index file $2 (default the index), as tab-separated cells.
rows() {
  local file="${2:-$INDEX}"
  [ -f "$file" ] || return 0
  LEDGER="$1" awk '
    $0 == "## `" ENVIRON["LEDGER"] "`" { on = 1; next }
    on && /^## / { exit }
    on && /^\| `[0-9]{4}` \|/ {
      line = $0; sub(/^\|[[:space:]]*/, "", line); sub(/[[:space:]]*\|[[:space:]]*$/, "", line)
      n = split(line, cells, /[[:space:]]*\|[[:space:]]*/)
      out = cells[1]; for (i = 2; i <= n; i++) out = out "\t" cells[i]
      gsub(/`/, "", out); print out
    }' "$file"
}

# Add a row to ledger $1's table, creating the index and the table when absent.
add_row() { # $1 = ledger, $2 = the row
  mkdir -p "$(dirname "$INDEX")"
  if [ ! -f "$INDEX" ]; then
    cat > "$INDEX" <<'MD'
# docs/history/INDEX.md — the sealed segments of the rolling ledgers

Each segment below is a run of a live ledger's oldest entries, moved here byte for byte by
`bash scripts/check_history_ledgers.sh --seal` and never edited again (`docs/decisions/decision_history-ledgers.md`).
A row records the segment's entry count, its newest and oldest entries, its lines, bytes and sha256, and the day it
was sealed. The rows are append-only, and `HISTORY-LEDGERS` checks every segment against its row on every commit.

To read a ledger whole, read its live file, then its segments from the highest number down: the concatenation is
the ledger as it would stand unsealed. To prove a segment, compare `sha256sum docs/history/<ledger>/<NNNN>.md` with
its row.
MD
  fi
  if ! grep -qxF "## \`$1\`" "$INDEX"; then
    printf '\n## `%s`\n\n| Segment | Entries | Newest | Oldest | Lines | Bytes | sha256 | Sealed |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n' "$1" >> "$INDEX"
  fi
  # Append after the last row of this ledger's table, and before the next ledger's heading.
  LEDGER="$1" ROW="$2" awk '
    { lines[NR] = $0 }
    $0 == "## `" ENVIRON["LEDGER"] "`" { section = NR }
    section && NR > section && /^## / { closed = 1 }
    section && NR > section && !closed && /^\| / { last = NR }
    END { for (i = 1; i <= NR; i++) { print lines[i]; if (i == last) print ENVIRON["ROW"] } }' "$INDEX" > "$INDEX.new" &&
    mv "$INDEX.new" "$INDEX"
}

# Seal every rollover due in one ledger, then prove that the live entries followed by the new segments, newest
# first, are byte for byte the region they came from. Nothing is replaced unless they are.
seal_one() { # name file dir pattern window
  local name="$1" file="$2" dir="$3" pattern="$4" window="$5"
  [ -f "$file" ] || return 0
  local starts=() total next today
  local line; while IFS= read -r line; do starts+=("$line"); done < <(entries "$file" "$pattern")
  local n=${#starts[@]}
  [ "$n" -ge $(( 2 * window )) ] || return 0
  mkdir -p "$dir" "$SCRATCH"
  total=$(wc -l < "$file" | tr -d ' ')
  today="$(date -u +%Y-%m-%d)"
  sed -n "${starts[0]},\$p" "$file" > "$SCRATCH/region.before"
  next=$(ls "$dir" 2>/dev/null | grep -E '^[0-9]{4}\.md$' | sed 's/\.md$//' | sort -n | tail -1)
  next=$(( 10#${next:-0} + 1 ))
  local sealed=() from to seg last_of_first
  while [ "$n" -ge $(( 2 * window )) ]; do
    from=${starts[$((n - window))]}
    to=$(( n == ${#starts[@]} ? total : starts[n] - 1 ))
    last_of_first=$(( starts[n - window + 1] - 1 ))
    seg=$(printf '%04d' "$next")
    sed -n "${from},${to}p" "$file" > "$dir/$seg.md"
    add_row "$name" "| \`$seg\` | $window | $(label "$file" "$from" "$last_of_first") | $(label "$file" "${starts[$((n - 1))]}" "$to") | $(wc -l < "$dir/$seg.md" | tr -d ' ') | $(wc -c < "$dir/$seg.md" | tr -d ' ') | \`$(sha "$dir/$seg.md")\` | \`$today\` |"
    sealed=("$seg" ${sealed[@]+"${sealed[@]}"})
    next=$((next + 1)); n=$((n - window))
  done
  head -n $(( starts[n] - 1 )) "$file" > "$SCRATCH/live"
  { sed -n "${starts[0]},\$p" "$SCRATCH/live"; for seg in "${sealed[@]}"; do cat "$dir/$seg.md"; done; } > "$SCRATCH/region.after"
  if ! cmp -s "$SCRATCH/region.before" "$SCRATCH/region.after"; then
    echo "history-ledgers: the seal of $file would not reconstruct it byte for byte; nothing was replaced" >&2
    return 1
  fi
  cp "$SCRATCH/live" "$file"
  echo "history-ledgers: $file — sealed ${sealed[*]} and kept $n entries; the reconstruction is byte for byte"
}

check_one() { # name file dir pattern window
  local name="$1" file="$2" dir="$3" pattern="$4" window="$5" seg entries newest oldest lines bytes digest path
  [ -f "$file" ] || { note "$file is missing"; return; }
  local listed="" expect=1
  while IFS=$'\t' read -r seg entries newest oldest lines bytes digest _; do
    [ -n "$seg" ] || continue
    listed="$listed $seg"
    path="$dir/$seg.md"
    [ "$((10#$seg))" -eq "$expect" ] || note "$name segment $seg is out of sequence: $(printf '%04d' "$expect") was next"
    expect=$((10#$seg + 1))
    if [ ! -f "$path" ]; then note "$INDEX lists $name segment $seg, and $path does not exist"; continue; fi
    [ "$(wc -l < "$path" | tr -d ' ')" = "$lines" ] || note "$path has $(wc -l < "$path" | tr -d ' ') lines, and its row says $lines — a sealed segment changed"
    [ "$(wc -c < "$path" | tr -d ' ')" = "$bytes" ] || note "$path has $(wc -c < "$path" | tr -d ' ') bytes, and its row says $bytes — a sealed segment changed"
    [ "$(sha "$path")" = "$digest" ] || note "$path's sha256 is not its row's — a sealed segment changed"
    [ "$(count_entries "$path" "$pattern")" = "$entries" ] || note "$path holds $(count_entries "$path" "$pattern") entries, and its row says $entries"
  done < <(rows "$name")
  local f
  for f in $(ls "$dir" 2>/dev/null); do
    case " $listed " in *" ${f%.md} "*) ;; *) note "$dir/$f is in no row of $INDEX — a segment must be listed" ;; esac
  done
  # Append-only across history: every row any committed index held, not only HEAD's, since HEAD is the commit under
  # test in CI and a comparison with it alone sees nothing there.
  mkdir -p "$SCRATCH"
  local current headrow commit; current="$(rows "$name")"
  for commit in $(GIT_NO_REPLACE_OBJECTS=1 git log --format=%H -- "$INDEX" 2>/dev/null); do
    GIT_NO_REPLACE_OBJECTS=1 git show "$commit:$INDEX" > "$SCRATCH/old-index.md" 2>/dev/null || continue
    while IFS= read -r headrow; do
      [ -n "$headrow" ] || continue
      printf '%s\n' "$current" | grep -qxF -- "$headrow" ||
        note "$INDEX's $name row \`${headrow%%$'\t'*}\`, committed in ${commit:0:12}, is gone or changed — the index is append-only"
    done < <(rows "$name" "$SCRATCH/old-index.md")
  done
  # Every segment is what the commit that added it wrote.
  local added
  for seg in $listed; do
    [ -f "$dir/$seg.md" ] || continue
    added="$(GIT_NO_REPLACE_OBJECTS=1 git log --diff-filter=A --format=%H -- "$dir/$seg.md" 2>/dev/null | tail -n 1)"
    [ -n "$added" ] || continue
    GIT_NO_REPLACE_OBJECTS=1 git show "$added:$dir/$seg.md" 2>/dev/null | cmp -s - "$dir/$seg.md" ||
      note "$dir/$seg.md is not what ${added:0:12} wrote when it sealed it — a sealed segment changed"
  done
  local live; live=$(count_entries "$file" "$pattern")
  [ "$live" -lt $(( 2 * window )) ] ||
    note "$file holds $live entries, and twice its window is $(( 2 * window )): a rollover is required — bash scripts/check_history_ledgers.sh --seal"
  if [ -n "$listed" ] && ! grep -qF "$INDEX" "$file"; then note "$file's header does not name $INDEX, where its older entries are"; fi
  # Continuity: the live entries, then the segments from the highest number down.
  local order=("$file")
  for seg in $(printf '%s\n' $listed | sort -rn); do [ -f "$dir/$seg.md" ] && order+=("$dir/$seg.md"); done
  if [ "$name" = changelog ]; then
    for f in "${order[@]}"; do awk -v pattern="$pattern" '
      /^```/ { fenced = !fenced }
      !fenced && $0 ~ pattern { want = 1; next }
      want && /^[[:space:]]*$/ { next }
      want && match($0, /^`ARCHOGEN-[A-Z0-9]+-[0-9]+`/) { id = substr($0, 2, RLENGTH - 2); sub(/.*-/, "", id); print id + 0 }
      want { want = 0 }' "$f"; done |
      awk 'prev != "" && $1 >= prev { bad = 1 } { prev = $1 } END { exit bad }' ||
      note "$name's entries are not strictly newest first across $file and its segments"
  else
    for f in "${order[@]}"; do grep -oE '^## _\([0-9]{4}-[0-9]{2}-[0-9]{2}\)_' "$f" | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}'; done |
      awk 'prev != "" && $1 > prev { bad = 1 } { prev = $1 } END { exit bad }' ||
      note "$name's dates rise somewhere across $file and its segments — the order is newest first"
  fi
  checked=$((checked + 1))
}

run_all() { # $1 = check | seal
  local entry name file dir pattern window
  for entry in "${LEDGERS[@]}"; do
    IFS='|' read -r name file dir pattern window <<< "$entry"
    if [ "$1" = seal ]; then seal_one "$name" "$file" "$dir" "$pattern" "$window" || return 1
    else check_one "$name" "$file" "$dir" "$pattern" "$window"; fi
  done
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work"; git -C "$work" init -q
    {
      printf '# CHANGELOG.md\n\nNewest first. Older entries: docs/history/INDEX.md.\n\n'
      for i in $(seq 45 -1 1); do printf '## archogen — entry %s\n\n`ARCHOGEN-T-%04d` (leaf T.%s).\n\n- a line\n\n' "$i" "$i" "$i"; done
    } > "$work/CHANGELOG.md"
    {
      printf '# DEV_NOTES.md\n\nOlder notes: docs/history/INDEX.md.\n\n'
      for i in $(seq 25 -1 1); do printf '## _(2026-09-%02d)_ — note %s\n\ntext\n\n' "$i" "$i"; done
    } > "$work/DEV_NOTES.md"
    git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm base
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the arguments
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  commit() { git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm step; }
  fresh
  arm "a ledger at twice its window requires a rollover" 1 "a rollover is required"
  arm "the seal proves its reconstruction and leaves a clean ledger" 0 "the reconstruction is byte for byte" --seal
  arm "the sealed ledgers pass the gate" 0 ""
  arm "sealing again seals nothing and still passes" 0 "" --seal
  commit
  printf 'x' >> "$work/docs/history/changelog/0001.md"
  arm "a byte added to a sealed segment is refused" 1 "a sealed segment changed"
  git -C "$work" checkout -q -- docs/history/changelog/0001.md
  sed -i.bak '/^| `0001` |/d' "$work/docs/history/INDEX.md"; rm -f "$work/docs/history/INDEX.md.bak"
  arm "a segment with no row, and a row gone from the index, are refused" 1 "is in no row of"
  git -C "$work" checkout -q -- docs/history/INDEX.md
  sed -i.bak '/^| `0001` |/ s/| `[0-9-]*` |$/| `1999-01-01` |/' "$work/docs/history/INDEX.md"; rm -f "$work/docs/history/INDEX.md.bak"
  arm "a committed row that changed is refused as not append-only" 1 "the index is append-only"
  git -C "$work" checkout -q -- docs/history/INDEX.md
  # A segment and its row forged together, and committed: the check a CI run makes, where HEAD is the forgery.
  printf 'forged\n' >> "$work/docs/history/changelog/0001.md"
  python3 - "$work" <<'PYF'
import hashlib, re, sys
w = sys.argv[1]
s = open(w + "/docs/history/changelog/0001.md", "rb").read()
i = w + "/docs/history/INDEX.md"
t = open(i).read()
t = re.sub(r"^(\| `0001` \|(?: [^|]* \|){3}) \d+ \| \d+ \| `[0-9a-f]+` \|", lambda m: "%s %d | %d | `%s` |" % (m.group(1), s.count(b"\n"), len(s), hashlib.sha256(s).hexdigest()), t, count=1, flags=re.M)
open(i, "w").write(t)
PYF
  commit
  arm "a segment and its row forged together and committed are refused" 1 "is not what"
  git -C "$work" reset -q --hard HEAD~1
  sed -i.bak 's/`ARCHOGEN-T-0025`/`ARCHOGEN-T-0002`/' "$work/CHANGELOG.md"; rm -f "$work/CHANGELOG.md.bak"
  arm "entries out of order across the live file and its segments are refused" 1 "not strictly newest first"
  git -C "$work" checkout -q -- CHANGELOG.md
  sed -i.bak 's/^## _(2026-09-15)_/## _(2026-10-15)_/' "$work/DEV_NOTES.md"; rm -f "$work/DEV_NOTES.md.bak"
  arm "a note whose date rises is refused" 1 "dates rise somewhere"
  git -C "$work" checkout -q -- DEV_NOTES.md
  sed -i.bak 's|docs/history/INDEX.md|older notes elsewhere|' "$work/DEV_NOTES.md"; rm -f "$work/DEV_NOTES.md.bak"
  arm "a live header that does not name the index is refused" 1 "does not name docs/history/INDEX.md"
  git -C "$work" checkout -q -- DEV_NOTES.md
  { head -n 4 "$work/CHANGELOG.md"; for i in $(seq 65 -1 46); do printf '## archogen — entry %s\n\n`ARCHOGEN-T-%04d` (leaf T.%s).\n\n- a line\n\n' "$i" "$i" "$i"; done; tail -n +5 "$work/CHANGELOG.md"; } > "$work/c"
  mv "$work/c" "$work/CHANGELOG.md"
  arm "twenty more entries require the next rollover" 1 "a rollover is required"
  arm "which seals the next segment in sequence" 0 "sealed 0002" --seal
  arm "and the ledgers pass again" 0 ""
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real ledgers pass"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "history-ledgers self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --seal) run_all seal || exit 1; checked=0; run_all check ;;
  "") checked=0; run_all check ;;
  *) echo "usage: bash scripts/check_history_ledgers.sh [--seal | --self-test]" >&2; exit 2 ;;
esac
if [ "$fail" -ne 0 ]; then
  echo "HISTORY-LEDGERS: $fail breach(es) — docs/decisions/decision_history-ledgers.md" >&2
  exit 1
fi
echo "history-ledgers: OK ($checked ledger(s): segments unchanged, the index complete and append-only, the order continuous, the windows bounded)"
exit 0
