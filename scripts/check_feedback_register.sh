#!/usr/bin/env bash
# scripts/check_feedback_register.sh — FEEDBACK-REGISTER: every outbound bug register agrees with the issues that
# are its source, and its totals are a recount of its rows (leaf `PROGRAM.15`).
#
# ⭐ WHY THIS EXISTS. A vendor reads `docs/feedback/<vendor>/INDEX.md` first, and each row restates what the
# issue's own `README.md` says. `FEEDBACK-SELF-CONTAINED` checks that every issue is *named* in the register;
# nothing compared what the row *says* with its issue, or recounted the totals. Consistency held only while every
# state change hand-edited both files — five transitions across six commits on `2026-09-27` — and measured when
# this was written, it had already failed one level up: `docs/feedback/README.md` still said LinkedSpec had
# **5 open bugs and 2 blockers**, three days after all five were verified.
#
# THE LEGS, per vendor:
#   1. ROWS — each register row's State and Severity equal its issue's `**State**` and the leading token of its
#      `**Severity**`; the issue directory the row links exists.
#   2. VERIFIED — a `verified` row's issue carries a dated archogen re-measurement in its History
#      (`- \`YYYY-MM-DD\` — archogen: re-measured …`), and the row's `Last verified` is the latest such date.
#   3. TOTALS BY STATE — each `| \`state\` | count | IDs |` row equals a recount of the register.
#   4. TOTALS BY SEVERITY, OPEN ONLY — "open" is defined in the register as unresolved: `open`, `acknowledged`
#      or `fixed-upstream`; each severity's count equals a recount of the unresolved rows.
#   5. THE CROSS-VENDOR INDEX — `docs/feedback/README.md`'s row for the vendor: Bugs, Open and Blockers equal
#      the register's row count, unresolved count and unresolved Blockers.
# SCOPE: the vendors with a staged file (a staged `docs/feedback/README.md` puts every vendor in scope), so one
# vendor's register cannot fail a commit about another; with nothing staged — CI, a manual run — every vendor.
#
# ⚠️ HONEST LIMIT: it proves the records agree with each other, never that the measurement behind them was right.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

BASE="docs/feedback"
UNRESOLVED="open acknowledged fixed-upstream"
fail=0
note() { printf 'FEEDBACK-REGISTER: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The rows of the first table under `## <heading>` in $1, header included, as tab-separated trimmed cells.
table() {
  HEADING="$2" awk '
    $0 == "## " ENVIRON["HEADING"] { on = 1; next }
    on && /^## / { exit }
    on && /^\|/ {
      if ($0 ~ /^\|[ -|]+\|[[:space:]]*$/ && $0 !~ /[A-Za-z0-9]/) next
      line = $0; sub(/^\|[[:space:]]*/, "", line); sub(/[[:space:]]*\|[[:space:]]*$/, "", line)
      n = split(line, cells, /[[:space:]]*\|[[:space:]]*/)
      out = cells[1]; for (i = 2; i <= n; i++) out = out "\t" cells[i]
      print out; seen = 1; next
    }
    on && seen && !/^\|/ { exit }' "$1"
}

# The value of `| **<field>** | <value> |` in an issue README.
field() { sed -nE "s/^\\| \\*\\*$2\\*\\* \\| (.*) \\|[[:space:]]*\$/\\1/p" "$1" | head -n 1; }

unresolved() { case " $UNRESOLVED " in *" $1 "*) return 0 ;; esac; return 1; }

check_vendor() { # $1 = vendor directory name
  local vendor="$1" index="$BASE/$1/INDEX.md" rows id link severity state last dir readme
  [ -f "$index" ] || { note "$vendor has no INDEX.md"; return; }
  rows="$(table "$index" "Register" | tail -n +2 |
    awk -F'\t' '{ id = $1; link = $1; sub(/^\[/, "", id); sub(/\].*/, "", id); sub(/^[^(]*\(/, "", link); sub(/\).*/, "", link)
                  s = $5; gsub(/`/, "", s); l = $7; gsub(/`/, "", l); print id "\t" link "\t" $4 "\t" s "\t" l }')"
  [ -n "$rows" ] || { note "$index has no register rows under \`## Register\`"; return; }
  # 1–2. each row against its issue
  while IFS=$'\t' read -r id link severity state last; do
    [ -n "$id" ] || continue
    dir="$BASE/$vendor/${link%/}"; readme="$dir/README.md"
    if [ ! -f "$readme" ]; then note "$index: row $id links $link, which has no README.md"; continue; fi
    local own_state own_severity
    own_state="$(field "$readme" State | tr -d '`')"
    own_severity="$(field "$readme" Severity | awk '{print $1}')"
    [ "$own_state" = "$state" ] ||
      note "$index: row $id says State \`$state\`, and its own README says \`${own_state:-nothing}\` — edit both in one commit"
    [ "$own_severity" = "$severity" ] ||
      note "$index: row $id says Severity $severity, and its own README says ${own_severity:-nothing}"
    if [ "$state" = "verified" ]; then
      local latest
      latest="$(grep -oE '^- `[0-9]{4}-[0-9]{2}-[0-9]{2}` — archogen: re-measured' "$readme" | grep -oE '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sort | tail -n 1)"
      if [ -z "$latest" ]; then
        note "$index: row $id is \`verified\`, and its History has no dated archogen re-measurement — only a re-measurement may verify"
      elif [ "$latest" != "$last" ]; then
        note "$index: row $id says Last verified $last, and its latest archogen re-measurement is $latest"
      fi
    fi
  done <<< "$rows"
  # 3. totals by state
  local st count ids want_count want_ids
  while IFS=$'\t' read -r st count ids; do
    [ -n "$st" ] || continue
    st="${st//\`/}"
    want_count="$(printf '%s\n' "$rows" | awk -F'\t' -v s="$st" '$4 == s' | grep -c . || true)"
    want_ids="$(printf '%s\n' "$rows" | awk -F'\t' -v s="$st" '$4 == s { print $1 }' | sort | paste -sd, - | sed 's/,/, /g')"
    [ -n "$want_ids" ] || want_ids="—"
    [ "$count" = "$want_count" ] || note "$index: Totals by state says $st = $count, and the register has $want_count"
    [ "$ids" = "$want_ids" ] || note "$index: Totals by state lists $st as \`$ids\`, and the register's are \`$want_ids\`"
  done < <(table "$index" "Totals by state" | tail -n +2)
  local listed
  listed="$(table "$index" "Totals by state" | tail -n +2 | cut -f1 | tr -d '`')"
  printf '%s\n' "$rows" | cut -f4 | sort -u | while IFS= read -r s; do
    [ -n "$s" ] || continue
    printf '%s\n' "$listed" | grep -qxF -- "$s" || echo "FEEDBACK-REGISTER: $index: state \`$s\` is used by the register and missing from Totals by state" >&2
  done
  [ -z "$(printf '%s\n' "$rows" | cut -f4 | sort -u | while IFS= read -r s; do [ -n "$s" ] && ! printf '%s\n' "$listed" | grep -qxF -- "$s" && echo x; done)" ] || fail=$((fail + 1))
  # 4. totals by severity, unresolved only
  local sev
  while IFS=$'\t' read -r sev count; do
    [ -n "$sev" ] || continue
    want_count="$(printf '%s\n' "$rows" | while IFS=$'\t' read -r _ _ s st _; do unresolved "$st" && [ "$s" = "$sev" ] && echo x; done | grep -c . || true)"
    [ "$count" = "$want_count" ] ||
      note "$index: Totals by severity says $sev = $count unresolved, and the register has $want_count (unresolved: $UNRESOLVED)"
  done < <(table "$index" "Totals by severity, open only" | tail -n +2)
  # 5. the cross-vendor index
  local bugs open blockers xrow
  bugs="$(printf '%s\n' "$rows" | grep -c . || true)"
  open="$(printf '%s\n' "$rows" | while IFS=$'\t' read -r _ _ _ st _; do unresolved "$st" && echo x; done | grep -c . || true)"
  blockers="$(printf '%s\n' "$rows" | while IFS=$'\t' read -r _ _ s st _; do unresolved "$st" && [ "$s" = "Blocker" ] && echo x; done | grep -c . || true)"
  xrow="$(table "$BASE/README.md" "Vendors" | tail -n +2 | awk -F'\t' -v v="($vendor/INDEX.md)" 'index($2, v) { print $4 "\t" $5 "\t" $6 }')"
  if [ -z "$xrow" ]; then
    note "$BASE/README.md has no Vendors row pointing at $vendor/INDEX.md"
  else
    IFS=$'\t' read -r xb xo xk <<< "$xrow"
    [ "$xb" = "$bugs" ] || note "$BASE/README.md says $vendor has $xb bugs, and its register has $bugs rows"
    [ "$xo" = "$open" ] || note "$BASE/README.md says $vendor has $xo open, and its register has $open unresolved ($UNRESOLVED)"
    [ "$xk" = "$blockers" ] || note "$BASE/README.md says $vendor has $xk blockers, and its register has $blockers unresolved Blockers"
  fi
  checked=$((checked + 1))
}

# The vendors in scope: those with a staged file, every vendor when the cross-vendor index is staged or nothing is.
vendors() {
  local all staged
  all="$(for i in "$BASE"/*/INDEX.md; do [ -f "$i" ] && basename "$(dirname "$i")"; done)"
  staged="$(git diff --cached --name-only 2>/dev/null)"
  if [ -z "$staged" ] || printf '%s\n' "$staged" | grep -qxF "$BASE/README.md"; then
    printf '%s\n' "$all"; return
  fi
  printf '%s\n' "$staged" | sed -nE "s#^$BASE/([^/]+)/.*#\\1#p" | sort -u | while IFS= read -r v; do
    printf '%s\n' "$all" | grep -qxF -- "$v" && printf '%s\n' "$v"
  done
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/feedback_register/selftest"
  issue() { # $1 dir, $2 id, $3 state, $4 severity, $5 history lines
    mkdir -p "$work/$BASE/acme/issues/$1"
    printf '# %s\n\n| Field | Value |\n| --- | --- |\n| **ID** | `%s` |\n| **State** | `%s` |\n| **Severity** | %s |\n\n## History\n\n- `2026-01-01` — opened by archogen.\n%s' \
      "$2" "$2" "$3" "$4" "$5" > "$work/$BASE/acme/issues/$1/README.md"
  }
  fresh() {
    rm -rf "$work"; mkdir -p "$work/$BASE/acme"
    git -C "$work" init -q
    issue AC-001-one AC-001 verified "Blocker for us" $'- `2026-02-02` — archogen: re-measured at `abc`.\n'
    issue AC-002-two AC-002 open Major ""
    issue AC-003-three AC-003 withdrawn "— (not a defect)" ""
    cat > "$work/$BASE/acme/INDEX.md" <<'MD'
# Acme

## Register

| ID | Title | Kind | Severity | State | Opened | Last verified |
| --- | --- | --- | --- | --- | --- | --- |
| [AC-001](issues/AC-001-one/) | one | Build | Blocker | `verified` | `2026-01-01` | `2026-02-02` |
| [AC-002](issues/AC-002-two/) | two | Docs | Major | `open` | `2026-01-01` | `2026-01-01` |
| [AC-003](issues/AC-003-three/) | three | Docs | — | `withdrawn` | `2026-01-01` | `2026-01-01` |

## Totals by state

| State | Count | IDs |
| --- | --- | --- |
| `open` | 1 | AC-002 |
| `verified` | 1 | AC-001 |
| `withdrawn` | 1 | AC-003 |

## Totals by severity, open only

| Severity | Count |
| --- | --- |
| Blocker | 0 |
| Major | 1 |
MD
    cat > "$work/$BASE/README.md" <<'MD'
# Outbound

## Vendors

| Vendor | Tracker | Component | Bugs | Open | Blockers |
| --- | --- | --- | --- | --- | --- |
| Acme | [`acme/`](acme/INDEX.md) | widgets | 3 | 1 | 0 |
MD
    git -C "$work" add -A; git -C "$work" -c user.name=arm -c user.email=arm@example.invalid commit -q -m base
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
  fresh; arm "a register that agrees with its issues and recounts passes" 0 ""
  fresh; edit "$BASE/acme/issues/AC-002-two/README.md" 's/| `open` |/| `acknowledged` |/'
  arm "a row whose State its issue contradicts is refused" 1 "row AC-002 says State \`open\`, and its own README says \`acknowledged\`"
  fresh; edit "$BASE/acme/issues/AC-002-two/README.md" 's/| Major |/| Minor |/'
  arm "a row whose Severity its issue contradicts is refused" 1 "row AC-002 says Severity Major, and its own README says Minor"
  fresh; edit "$BASE/acme/issues/AC-001-one/README.md" '/re-measured/d'
  arm "a verified row with no re-measurement behind it is refused" 1 "row AC-001 is \`verified\`, and its History has no dated archogen re-measurement"
  fresh; edit "$BASE/acme/INDEX.md" 's/| `verified` | `2026-01-01` | `2026-02-02` |/| `verified` | `2026-01-01` | `2026-03-03` |/'
  arm "a Last verified date that is not the latest re-measurement is refused" 1 "says Last verified 2026-03-03, and its latest archogen re-measurement is 2026-02-02"
  fresh; edit "$BASE/acme/INDEX.md" 's/| `open` | 1 | AC-002 |/| `open` | 2 | AC-002 |/'
  arm "a state total that is not a recount is refused" 1 "Totals by state says open = 2, and the register has 1"
  fresh; edit "$BASE/acme/INDEX.md" 's/| `withdrawn` | 1 | AC-003 |/| `withdrawn` | 1 | AC-001 |/'
  arm "a state's ID list that is not the register's is refused" 1 "lists withdrawn as \`AC-001\`, and the register's are \`AC-003\`"
  fresh; edit "$BASE/acme/INDEX.md" '/| `withdrawn` | 1 | AC-003 |/d'
  arm "a state the register uses and the totals omit is refused" 1 "state \`withdrawn\` is used by the register and missing from Totals by state"
  fresh; edit "$BASE/acme/INDEX.md" 's/^| Major | 1 |/| Major | 0 |/'
  arm "an open-only severity total that is not a recount is refused" 1 "Totals by severity says Major = 0 unresolved, and the register has 1"
  fresh; edit "$BASE/README.md" 's/| widgets | 3 | 1 | 0 |/| widgets | 3 | 3 | 0 |/'
  arm "a cross-vendor Open count that is not the register's is refused" 1 "says acme has 3 open, and its register has 1 unresolved"
  fresh; edit "$BASE/acme/issues/AC-002-two/README.md" 's/| Major |/| Blocker |/'; edit "$BASE/acme/INDEX.md" 's/| Docs | Major | `open`/| Docs | Blocker | `open`/; s/^| Major | 1 |/| Major | 0 |/; s/^| Blocker | 0 |/| Blocker | 1 |/'
  arm "a cross-vendor Blockers count that is not the register's is refused" 1 "says acme has 0 blockers, and its register has 1 unresolved Blockers"
  fresh; rm -rf "$work/$BASE/acme/issues/AC-003-three"
  arm "a row whose issue is gone is refused" 1 "row AC-003 links issues/AC-003-three/, which has no README.md"
  fresh; mkdir -p "$work/$BASE/other"; printf '# Other\n\n## Register\n\n| ID | Title | Kind | Severity | State | Opened | Last verified |\n| --- | --- | --- | --- | --- | --- | --- |\n| [OT-001](issues/OT-001-x/) | x | Docs | Minor | `open` | `2026-01-01` | `2026-01-01` |\n' > "$work/$BASE/other/INDEX.md"
  git -C "$work" add "$BASE/other"; git -C "$work" -c user.name=arm -c user.email=arm@example.invalid commit -q -m other
  edit "$BASE/acme/INDEX.md" 's/| three | Docs/| three! | Docs/'; git -C "$work" add "$BASE/acme/INDEX.md"
  arm "a staged change to one vendor is not failed by another vendor's register" 0 ""
  git -C "$work" reset -q
  arm "with nothing staged, every vendor is in scope" 1 "other/INDEX.md: row OT-001 links issues/OT-001-x/, which has no README.md"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real registers agree with their issues"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "feedback-register self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked=0
while IFS= read -r vendor; do
  [ -n "$vendor" ] || continue
  check_vendor "$vendor"
done < <(vendors)
if [ "$fail" -ne 0 ]; then
  echo "FEEDBACK-REGISTER: $fail breach(es) — a register and its issues are edited in one commit, and its totals are a recount" >&2
  exit 1
fi
echo "feedback-register: OK ($checked vendor register(s) agree with their issues, their totals and the cross-vendor index)"
exit 0
