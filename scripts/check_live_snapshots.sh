#!/usr/bin/env bash
# scripts/check_live_snapshots.sh — LIVE-SNAPSHOTS: every live document that shows current state stays bounded on
# lines, bytes and longest line (leaf `PROGRAM.17.2`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`).
#
# ⭐ WHY THIS EXISTS. A snapshot of *now* grows only if something appends history to it, and nothing stopped that:
# `LIVE_STATUS.md` went from 3 315 to 42 110 bytes in 21 lines, one table row 30 256 bytes long, by a closure note
# appended per closed leaf — while its line count, the only size a reader would glance at, barely moved. That is
# why the longest line is its own axis here: the doctrine names a dense row as a pathology that line and byte
# totals do not see.
#
# THE REGISTRY is data, in `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`'s adoption note: the table under
# `### Bounds of the snapshots`, one row per surface, inclusive ceilings — or the name of the doctrine that already
# owns a dimension, which this check then leaves alone.
# THE LEGS:
#   1. each surface exists, and is within every ceiling the table gives it (equality passes);
#   2. every surface the inventory classifies `bounded_snapshot` has a bounds row — a snapshot cannot be declared
#      and left unbounded;
#   3. a malformed value, or an empty table, is a breach, not a pass.
#
# ⚠️ HONEST LIMIT: it bounds size, not truth. A snapshot within its ceilings can still say something stale; each
# row's claims are derived from the trees by the author, and `PROGRAM.20` owns comparing a copy with its tree.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

DOCTRINE="LIVE_DOCUMENT_SIZE_CONTAINMENT.md"
fail=0
note() { printf 'LIVE-SNAPSHOTS: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The rows of the table under `### <heading>` in the doctrine, header excluded, as tab-separated trimmed cells.
rows_under() {
  HEADING="$1" awk '
    $0 == "### " ENVIRON["HEADING"] { on = 1; next }
    on && /^#/ { exit }
    on && /^\|/ {
      if ($0 !~ /[A-Za-z0-9]/) next
      line = $0; sub(/^\|[[:space:]]*/, "", line); sub(/[[:space:]]*\|[[:space:]]*$/, "", line)
      n = split(line, cells, /[[:space:]]*\|[[:space:]]*/)
      if (!header_seen) { header_seen = 1; next }
      out = cells[1]; for (i = 2; i <= n; i++) out = out "\t" cells[i]
      print out
    }' "$DOCTRINE"
}

longest() { LC_ALL=C awk '{ if (length($0) > m) m = length($0) } END { print m + 0 }' "$1"; }

# $1 = surface, $2 = axis name, $3 = ceiling cell, $4 = measured value
axis() {
  local surface="$1" name="$2" ceiling="$3" actual="$4"
  case "$ceiling" in
    [A-Z]*) return ;;
    ''|*[!0-9]*) note "$DOCTRINE: \`$surface\`'s $name ceiling is '$ceiling' — write a number, or the doctrine that owns it"; return ;;
  esac
  [ "$actual" -le "$ceiling" ] ||
    note "$surface: $actual $name, over its ceiling of $ceiling — a snapshot holds current state; move history to its tree's Commit Log and CHANGELOG.md"
}

scan() {
  [ -f "$DOCTRINE" ] || { note "$DOCTRINE is missing — nothing declares what a live document may hold"; return; }
  local bounds surface lines bytes width path
  bounds="$(rows_under "Bounds of the snapshots")"
  [ -n "$bounds" ] || { note "$DOCTRINE has no bounds under \`### Bounds of the snapshots\` — an empty registry is a breach, not a pass"; return; }
  while IFS=$'\t' read -r surface lines bytes width _; do
    [ -n "$surface" ] || continue
    path="${surface//\`/}"
    if [ ! -f "$path" ]; then note "$DOCTRINE bounds \`$path\`, which does not exist"; continue; fi
    axis "$path" "lines" "$lines" "$(wc -l < "$path" | tr -d ' ')"
    axis "$path" "bytes" "$bytes" "$(wc -c < "$path" | tr -d ' ')"
    axis "$path" "bytes on its longest line" "$width" "$(longest "$path")"
    checked=$((checked + 1))
  done <<< "$bounds"
  # 2. every bounded_snapshot of the inventory has a bounds row
  local bounded
  bounded="$(printf '%s\n' "$bounds" | cut -f1 | tr -d '`')"
  while IFS=$'\t' read -r surface lifecycle _; do
    [ -n "$surface" ] || continue
    case "$lifecycle" in *bounded_snapshot*) ;; *) continue ;; esac
    path="$(printf '%s' "$surface" | grep -oE '`[^`]+`' | head -n 1 | tr -d '`')"
    [ -n "$path" ] || continue
    printf '%s\n' "$bounded" | grep -qxF -- "$path" ||
      note "the inventory classifies \`$path\` as a bounded snapshot, and \`### Bounds of the snapshots\` gives it no bounds"
  done < <(rows_under "Inventory, measured \`2026-09-30\`")
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/live_snapshots/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work"
    git -C "$work" init -q
    cat > "$work/$DOCTRINE" <<'MD'
# Doctrine

### Inventory, measured `2026-09-30`

| Surface | Lifecycle | Measured |
| --- | --- | --- |
| `NOW.md` — the board | `bounded_snapshot` | 3 / 30 / 10 |
| `OLD.md` — the history | `rolling_ledger` | 9 / 90 / 10 |

### Bounds of the snapshots

| Surface | Lines | Bytes | Longest line | Survivor measured |
| --- | --- | --- | --- | --- |
| `NOW.md` | 4 | 40 | 12 | 3 / 30 / 10 |

## Next
MD
    printf 'row one\nrow two\nrow tre\n' > "$work/NOW.md"
    printf 'history\n' > "$work/OLD.md"
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
  fresh; arm "snapshots within their ceilings pass; a history is not bounded here" 0 ""
  fresh; printf 'row one\nrow two\nrow tre\nrow 4\n' > "$work/NOW.md"
  arm "a snapshot exactly at its line ceiling passes — the ceiling is inclusive" 0 ""
  fresh; printf 'a\nb\nc\nd\ne\n' > "$work/NOW.md"
  arm "one line over the ceiling is refused" 1 "NOW.md: 5 lines, over its ceiling of 4"
  fresh; printf '012345678\n012345678\n012345678\n0123456789\n' > "$work/NOW.md"   # 10 + 10 + 10 + 11 = 41 bytes, 4 lines
  arm "one byte over the ceiling is refused" 1 "NOW.md: 41 bytes, over its ceiling of 40"
  fresh; printf 'row one\n0123456789abc\n' > "$work/NOW.md"
  arm "a row wider than its ceiling is refused, though lines and bytes are fine" 1 "NOW.md: 13 bytes on its longest line, over its ceiling of 12"
  fresh; rm "$work/NOW.md"
  arm "a bounded surface that is gone is refused" 1 "bounds \`NOW.md\`, which does not exist"
  fresh; sed -i.bak 's/| `OLD.md` — the history | `rolling_ledger` |/| `OLD.md` — the history | `bounded_snapshot` |/' "$work/$DOCTRINE"; rm -f "$work/$DOCTRINE.bak"
  arm "a snapshot the inventory declares and the bounds omit is refused" 1 "classifies \`OLD.md\` as a bounded snapshot, and"
  fresh; sed -i.bak 's/| `NOW.md` | 4 | 40 | 12 |/| `NOW.md` | four | 40 | 12 |/' "$work/$DOCTRINE"; rm -f "$work/$DOCTRINE.bak"
  arm "a ceiling that is not a number is refused" 1 "lines ceiling is 'four'"
  fresh; sed -i.bak 's/| `NOW.md` | 4 | 40 | 12 |/| `NOW.md` | OTHER-DOCTRINE | 40 | 12 |/' "$work/$DOCTRINE"; rm -f "$work/$DOCTRINE.bak"; printf 'a\nb\nc\nd\ne\nf\n' > "$work/NOW.md"
  arm "a dimension another doctrine owns is left to that doctrine" 0 ""
  fresh; sed -i.bak '/^| `NOW.md` | 4 |/d' "$work/$DOCTRINE"; rm -f "$work/$DOCTRINE.bak"
  arm "an empty bounds table is a breach, not a pass" 1 "an empty registry is a breach"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real snapshots are within their ceilings"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "live-snapshots self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked=0
scan
if [ "$fail" -ne 0 ]; then
  echo "LIVE-SNAPSHOTS: $fail breach(es) — $DOCTRINE bounds what shows current state" >&2
  exit 1
fi
echo "live-snapshots: OK ($checked snapshot(s) within their ceilings on lines, bytes and longest line)"
exit 0
