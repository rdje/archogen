#!/usr/bin/env bash
# scripts/check_readme_routes.sh — README-ROUTES: every destination the README sends a reader or an author to is
# registered, classified and under a pressure control (leaf `PROGRAM.35.2`, `README_POLICY.md`).
#
# ⭐ WHY THIS EXISTS. A cap on the landing page does not remove the pressure to write things down; it moves it. The
# README's guards tell an author where the detail goes instead, and if that place is unbounded the cap has only
# displaced the growth — `README_POLICY.md` records one adoption whose neighbouring status file reached 1.5 MB that
# way. So the policy asks for every route out of the README to be followed until it ends somewhere controlled.
#
# THE POPULATION is derived, never listed:
#   - every repository link in `README.md` (a `navigation` route);
#   - every path the README's guards print when the page is over its caps — `README-STABILITY` run with its line cap
#     at 0, and `LIVE-SNAPSHOTS` run on a scratch copy with the README's ceilings at 0 (an `overflow` route). These
#     are the guards' **actual** output, not a transcription of it;
#   - every path the policy's neutral body names in a code span (`navigation`);
#   - every destination a registered row says its own overflow goes on to (`overflow`, followed transitively).
# THE REGISTRY is data: the table under `### Routed destinations` in `README_POLICY.md`'s fenced adoption note. A
# path is governed by its own row, or by the row of the deepest registered directory above it.
# THE LEGS:
#   1. every derived path is governed; every row is reached by something (a row nothing routes to is stale);
#   2. each row's Route cell is exactly the set of route classes derived for it;
#   3. each dimension a row must control carries a control: an inclusive ceiling that the destination is within
#      (a file's lines, bytes and longest line; a directory's tracked files, largest file's lines and bytes, longest
#      line and total bytes), a registered doctrine that owns that dimension, or `debt: <leaf>` naming an open leaf;
#   4. every onward destination has a row, and no chain of onward routes is a cycle;
#   5. an empty registry, a guard that prints no hint, or a malformed cell is a breach, not a pass.
#
# ⚠️ HONEST LIMITS: a deferral to a doctrine is checked to name a registered doctrine, not that the doctrine bounds
# this file; the onward routes of a registered row are declared, and compared with nothing but the registry; every
# path a guard's hint prints counts as an overflow route, the policy it cites for reference included; and a third
# guard on `README.md` would have to be added below.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only apart from scratch under
# `target/doctrine_scratch/readme_routes/`. `--self-test` runs the RED arms in scratch repositories there.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

TARGET="README.md"
POLICY="README_POLICY.md"
DOCTRINE="LIVE_DOCUMENT_SIZE_CONTAINMENT.md"
HEADING="Routed destinations"
SCRATCH="$ROOT/target/doctrine_scratch/readme_routes"
fail=0
note() { printf 'README-ROUTES: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The rows of the table under `### $HEADING` in the policy, header excluded, as tab-separated trimmed cells.
registry_rows() {
  HEADING="$HEADING" awk '
    $0 == "### " ENVIRON["HEADING"] { on = 1; next }
    on && /^#/ { exit }
    on && /^\|/ {
      if ($0 !~ /[A-Za-z0-9]/) next
      line = $0; sub(/^\|[[:space:]]*/, "", line); sub(/[[:space:]]*\|[[:space:]]*$/, "", line)
      n = split(line, cells, /[[:space:]]*\|[[:space:]]*/)
      if (!header_seen) { header_seen = 1; next }
      out = cells[1]; for (i = 2; i <= n; i++) out = out "\t" cells[i]
      print out
    }' "$POLICY"
}

# Path-shaped tokens of stdin: a file ending `.md`, or a directory written with its trailing `/`. The governed page
# itself is not a destination.
paths_in() {
  grep -oE '([A-Za-z0-9_.-]+/)*[A-Za-z0-9_.-]+\.md|[A-Za-z0-9_-]+(/[A-Za-z0-9_.-]+)*/' | grep -vxF "$TARGET" | sort -u
}

readme_links() {
  grep -oE '\]\([^)]+\)' "$TARGET" | sed -E 's/^\]\(//; s/\)$//; s/#.*$//' |
    grep -vE '^(https?:|mailto:|$)' | grep -vxF "$TARGET" | sort -u
}

# The paths in code spans of the neutral body, below the fenced adoption note.
policy_body_paths() {
  awk '/^<!-- README-POLICY-LOCAL-ADOPTION:END -->$/ { on = 1; next } on' "$POLICY" |
    grep -oE '`[^`]+`' | tr -d '`' | paths_in
}

# The hint README-STABILITY prints with its line cap at 0. A guard that then passes has printed nothing to derive.
hint_readme_stability() {
  local out rc
  out="$(README_LINE_CAP=0 bash "$ROOT/scripts/check_readme_stability.sh" 2>&1 >/dev/null)"; rc=$?
  if [ "$rc" -ne 1 ]; then
    note "README-STABILITY exited $rc with its line cap at 0 — it printed no hint, so its overflow routes cannot be derived"
    return
  fi
  printf '%s\n' "$out" | paths_in
}

# The hint LIVE-SNAPSHOTS prints for a README over every ceiling, on a scratch copy: its doctrine is data, and the
# real one is not edited to provoke it. Run only when LIVE-SNAPSHOTS bounds the README at all.
hint_live_snapshots() {
  grep -q '^| `README.md` |' "$DOCTRINE" 2>/dev/null || return 0
  local work="$SCRATCH/live_snapshots_hint" out rc
  rm -rf "$work"; mkdir -p "$work"; git -C "$work" init -q
  cp "$TARGET" "$work/$TARGET"
  printf '### Bounds of the snapshots\n\n| Surface | Lines | Bytes | Longest line | Survivor measured |\n| --- | --- | --- | --- | --- |\n| `README.md` | 0 | 0 | 0 | none |\n' > "$work/$DOCTRINE"
  out="$(cd "$work" && bash "$ROOT/scripts/check_live_snapshots.sh" 2>&1 >/dev/null)"; rc=$?
  rm -rf "$work"
  if [ "$rc" -ne 1 ]; then
    note "LIVE-SNAPSHOTS exited $rc on a README over every ceiling — it printed no hint, so its overflow routes cannot be derived"
    return
  fi
  printf '%s\n' "$out" | paths_in
}

registered_doctrines() {
  cat scripts/check_doctrines.sh scripts/check_doctrines.project.sh 2>/dev/null |
    grep -oE '^[[:space:]]*"[A-Z][A-Z0-9-]+\|' | tr -d ' \t"|' | sort -u
}

# The status of a leaf some tree declares, or nothing.
leaf_status() {
  LEAF="$1" awk '
    $0 == "- ID: `" ENVIRON["LEAF"] "`" { want = 1; next }
    want { if (match($0, /Status: `[a-z_]+`/)) { s = substr($0, RSTART + 9, RLENGTH - 10); print s } ; exit }
  ' docs/tasks/*.md 2>/dev/null | head -n 1
}

longest() { LC_ALL=C awk '{ if (length($0) > m) m = length($0) } END { print m + 0 }' "$@"; }

# The registered row governing $1: its own, or the deepest registered directory above it.
governing_row() {
  local path="$1" dest best=""
  while IFS= read -r dest; do
    if [ "$dest" = "$path" ]; then best="$dest"; break; fi
    case "$dest" in */) case "$path" in "$dest"*) [ "${#dest}" -gt "${#best}" ] && best="$dest" ;; esac ;; esac
  done <<< "$DESTS"
  printf '%s' "$best"
}

# $1 = destination, $2 = dimension, $3 = cell, $4 = measured value ("" when the dimension does not apply)
control() {
  local dest="$1" dim="$2" cell="$3" actual="$4" leaf status
  case "$cell" in
    '—'|'-'|'')
      [ -z "$actual" ] || note "\`$dest\` has no pressure control on its $dim — write a ceiling, a doctrine, or \`debt: <leaf>\`"
      ;;
    'debt: '*)
      leaf="$(printf '%s' "${cell#debt: }" | tr -d '`')"
      status="$(leaf_status "$leaf")"
      if [ -z "$status" ]; then note "\`$dest\`'s $dim is debt owned by \`$leaf\`, which no tree declares"
      elif [ "$status" = "done" ]; then note "\`$dest\`'s $dim is debt owned by \`$leaf\`, and \`$leaf\` is done — a closed leaf owns no debt"
      else DEBT="$DEBT $leaf"; fi
      ;;
    [A-Z]*)
      printf '%s\n' "$DOCTRINES" | grep -qxF -- "$cell" ||
        note "\`$dest\`'s $dim is deferred to '$cell', which is not a registered doctrine"
      ;;
    *[!0-9]*)
      note "\`$dest\`'s $dim ceiling is '$cell' — write a number, a registered doctrine, or \`debt: <leaf>\`"
      ;;
    *)
      if [ -z "$actual" ]; then note "\`$dest\` has a $dim ceiling, and a $dim does not apply to it"
      elif [ "$actual" -gt "$cell" ]; then
        note "$dest: $actual $dim, over its ceiling of $cell — compact it under an owned leaf; raising a ceiling needs a decision record"
      fi
      ;;
  esac
}

scan() {
  [ -f "$TARGET" ] || { note "$TARGET is missing — it is the page whose routes this governs"; return; }
  [ -f "$POLICY" ] || { note "$POLICY is missing — nothing registers where the README routes"; return; }
  local rows nav ovf dest route owner life files lines bytes width total onward p row
  rows="$(registry_rows)"
  [ -n "$rows" ] || { note "$POLICY has no rows under \`### $HEADING\` — an empty registry is a breach, not a pass"; return; }
  DESTS="$(printf '%s\n' "$rows" | cut -f1 | tr -d '`')"
  DOCTRINES="$(registered_doctrines)"
  DEBT=""

  nav="$( { readme_links; policy_body_paths; } | sort -u)"
  ovf="$( { hint_readme_stability; hint_live_snapshots; } | sort -u)"

  # Onward routes, followed from every reached row until nothing new is reached; a reached row's onward
  # destinations are overflow routes.
  local reached frontier next edges=""
  while IFS=$'\t' read -r dest _ _ _ _ _ _ _ _ onward; do
    dest="${dest//\`/}"
    for p in $(printf '%s' "$onward" | grep -oE '`[^`]+`' | tr -d '`'); do edges="$edges$dest"$'\t'"$p"$'\n'; done
  done <<< "$rows"
  reached=""
  for p in $nav $ovf; do row="$(governing_row "$p")"; [ -n "$row" ] && reached="$reached$row"$'\n'; done
  frontier="$reached"
  while [ -n "$frontier" ]; do
    next=""
    for row in $frontier; do
      for p in $(printf '%s' "$edges" | awk -F'\t' -v r="$row" '$1 == r { print $2 }'); do
        ovf="$ovf"$'\n'"$p"
        local to; to="$(governing_row "$p")"
        [ -n "$to" ] || { note "\`$row\` sends its overflow on to \`$p\`, which no row governs"; continue; }
        printf '%s' "$reached" | grep -qxF -- "$to" || { reached="$reached$to"$'\n'; next="$next$to"$'\n'; }
      done
    done
    frontier="$next"
  done
  ovf="$(printf '%s\n' "$ovf" | sed '/^$/d' | sort -u)"

  # 1. every derived path is governed
  for p in $nav; do [ -n "$(governing_row "$p")" ] || note "$TARGET or the policy routes readers to \`$p\`, which no row of \`### $HEADING\` governs"; done
  for p in $(printf '%s\n' "$ovf" | grep -vxF -f <(printf '%s\n' "$edges" | cut -f2 | sed '/^$/d') 2>/dev/null); do
    [ -n "$(governing_row "$p")" ] || note "a guard's hint for $TARGET names \`$p\`, which no row of \`### $HEADING\` governs"
  done

  # 4. no cycle among onward routes: a depth-first walk that meets a destination already on its path
  local cycle
  cycle="$(printf '%s' "$edges" | awk -F'\t' '
    NF == 2 { adj[$1] = adj[$1] " " $2; nodes[$1] = 1 }
    function walk(n, path,   k, m, parts, i) {
      if (n in onpath) { print path; found = 1; return }
      if (n in done || found) return
      onpath[n] = 1
      m = split(adj[n], parts, " ")
      for (i = 1; i <= m; i++) if (parts[i] != "") walk(parts[i], path " → " parts[i])
      delete onpath[n]; done[n] = 1
    }
    END { for (n in nodes) if (!found) walk(n, n) }')"
  [ -z "$cycle" ] || note "onward routes form a cycle: $cycle — a chain that returns to where it started moves the pressure and ends nowhere"

  # 2 and 3, per row
  while IFS=$'\t' read -r dest route owner life files lines bytes width total onward; do
    dest="${dest//\`/}"
    [ -n "$dest" ] || continue
    [ -n "$owner" ] && [ "$owner" != "—" ] || note "\`$dest\` names no owner"
    [ -n "$life" ] && [ "$life" != "—" ] || note "\`$dest\` names no lifecycle class"
    local want="" navs=0 ovfs=0
    for p in $nav; do [ "$(governing_row "$p")" = "$dest" ] && navs=1; done
    for p in $ovf; do [ "$(governing_row "$p")" = "$dest" ] && ovfs=1; done
    [ "$navs" -eq 1 ] && want="navigation"
    [ "$ovfs" -eq 1 ] && want="${want:+$want + }overflow"
    if [ -z "$want" ]; then note "\`$dest\` is routed to by nothing — a stale row; remove it"; continue; fi
    [ "$route" = "$want" ] || note "\`$dest\`'s route is '$route', and what routes to it derives '$want'"

    if [ "${dest%/}" != "$dest" ]; then
      local listed; listed="$(git ls-files -- "$dest")"
      if [ -z "$listed" ]; then note "\`$dest\` is registered, and nothing under it is tracked"; continue; fi
      local n=0 ml=0 mb=0 tb=0 l b f
      while IFS= read -r f; do
        n=$((n + 1)); l=$(wc -l < "$f" | tr -d ' '); b=$(wc -c < "$f" | tr -d ' ')
        [ "$l" -gt "$ml" ] && ml=$l; [ "$b" -gt "$mb" ] && mb=$b; tb=$((tb + b))
      done <<< "$listed"
      control "$dest" "tracked files" "$files" "$n"
      control "$dest" "lines in its largest file" "$lines" "$ml"
      control "$dest" "bytes in its largest file" "$bytes" "$mb"
      # shellcheck disable=SC2086
      control "$dest" "bytes on its longest line" "$width" "$(longest $listed)"
      control "$dest" "bytes in total" "$total" "$tb"
    else
      if ! git ls-files --error-unmatch -- "$dest" >/dev/null 2>&1; then note "\`$dest\` is registered, and it is not a tracked file"; continue; fi
      control "$dest" "tracked files" "$files" ""
      control "$dest" "lines" "$lines" "$(wc -l < "$dest" | tr -d ' ')"
      control "$dest" "bytes" "$bytes" "$(wc -c < "$dest" | tr -d ' ')"
      control "$dest" "bytes on its longest line" "$width" "$(longest "$dest")"
      control "$dest" "bytes in total" "$total" ""
    fi
    checked=$((checked + 1))
  done <<< "$rows"
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/scripts" "$work/docs/tasks"
    git -C "$work" init -q
    printf '# W\n\n[a](A.md) · [the trees](docs/tasks/) · [history](CHANGELOG.md) · [policy](README_POLICY.md)\n' > "$work/README.md"
    printf 'a reference\n' > "$work/A.md"
    printf '# history\n' > "$work/CHANGELOG.md"
    printf -- '- ID: `T.1`\n  Status: `pending`\n\n- ID: `T.2`\n  Status: `done`\n' > "$work/docs/tasks/T.md"
    cat > "$work/README_POLICY.md" <<'MD'
<!-- README-POLICY-LOCAL-ADOPTION:BEGIN -->
### Routed destinations

| Destination | Route | Owner | Lifecycle | Files | Lines | Bytes | Longest line | Total bytes | Overflows to |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `A.md` | navigation | W | maintained_reference | — | 10 | 1000 | 100 | — | — |
| `CHANGELOG.md` | navigation + overflow | `T.1` | rolling_ledger | — | debt: `T.1` | debt: `T.1` | 200 | — | — |
| `README_POLICY.md` | navigation + overflow | W | maintained_reference | — | 100 | 8000 | LIVE-SNAPSHOTS | — | — |
| `docs/tasks/` | navigation + overflow | W | partitioned_canonical | 4 | 50 | 2000 | 200 | 4000 | — |
<!-- README-POLICY-LOCAL-ADOPTION:END -->

---

# Policy

Keep `<repository-root>/README_POLICY.md` beside `README.md`.
MD
    printf '### Bounds of the snapshots\n\n| Surface | Lines |\n| --- | --- |\n| `README.md` | 9 |\n' > "$work/$DOCTRINE"
    # Stub guards: this check's arms are about this check, and the real guards' hints are exercised by the real-tree
    # arm below.
    cat > "$work/scripts/check_readme_stability.sh" <<'SH'
#!/usr/bin/env bash
[ "$(wc -l < README.md)" -le "${README_LINE_CAP:-300}" ] && exit 0
printf 'README-STABILITY: README.md is over its cap.\n    release history .... CHANGELOG.md\n    Full policy: README_POLICY.md\n' >&2
exit 1
SH
    printf '#!/usr/bin/env bash\necho "LIVE-SNAPSHOTS: README.md: over its ceiling; move history to docs/tasks/ and CHANGELOG.md" >&2\nexit 1\n' > "$work/scripts/check_live_snapshots.sh"
    printf 'DOCTRINES=(\n  "LIVE-SNAPSHOTS|bounded|scripts/check_live_snapshots.sh"\n)\n' > "$work/scripts/check_doctrines.sh"
    printf 'PROJECT_DOCTRINES=(\n)\n' > "$work/scripts/check_doctrines.project.sh"
    git -C "$work" add -A
  }
  edit() { # $1 = file in the world, $2 = sed expression
    sed -i.bak "$2" "$work/$1"; rm -f "$work/$1.bak"; git -C "$work" add -A
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
  fresh; arm "a registry that governs every route passes, a deferral to a registered doctrine included" 0 ""
  fresh; printf 'b\n' > "$work/B.md"; edit README.md 's/· \[policy\]/· [b](B.md) · [policy]/'
  arm "a README link no row governs is refused" 1 "routes readers to \`B.md\`, which no row"
  fresh; edit scripts/check_readme_stability.sh 's|release history .... CHANGELOG.md|release history .... CHANGELOG.md, docs/extra/|'
  arm "a path the guard's own hint prints, and no row governs, is refused" 1 "hint for README.md names \`docs/extra/\`"
  fresh; edit README_POLICY.md 's/^| `A.md` | navigation |/| `A.md` | navigation + overflow |/'
  arm "a route cell that disagrees with what derives it is refused" 1 "\`A.md\`'s route is 'navigation + overflow', and what routes to it derives 'navigation'"
  fresh; printf 'c\n' > "$work/C.md"; edit README_POLICY.md '/^| `A.md` |/a\
| `C.md` | navigation | W | maintained_reference | — | 10 | 1000 | 100 | — | — |'
  arm "a row nothing routes to is refused as stale" 1 "\`C.md\` is routed to by nothing"
  fresh; printf '1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n' > "$work/A.md"; git -C "$work" add -A
  arm "a file one line over its ceiling is refused" 1 "A.md: 11 lines, over its ceiling of 10"
  fresh; printf '1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n' > "$work/A.md"; git -C "$work" add -A
  arm "a file exactly at its ceiling passes — the ceiling is inclusive" 0 ""
  fresh; for f in U V W; do for i in $(seq 30); do printf '%049d\n' 0; done > "$work/docs/tasks/$f.md"; done   # 3 × 1 500 bytes
  git -C "$work" add -A
  arm "a directory over its total is refused, though each file is within its own ceilings" 1 "bytes in total, over its ceiling of 4000"
  fresh; edit README_POLICY.md 's/debt: `T.1` | debt: `T.1`/debt: `T.2` | debt: `T.1`/'
  arm "debt owned by a closed leaf is refused" 1 "\`T.2\` is done"
  fresh; edit README_POLICY.md 's/^| `A.md` | navigation | W | maintained_reference | — | 10 |/| `A.md` | navigation | W | maintained_reference | — | NO-SUCH-GATE |/'
  arm "a deferral to a doctrine nobody registered is refused" 1 "'NO-SUCH-GATE', which is not a registered doctrine"
  fresh; edit README_POLICY.md 's/^\(| `docs\/tasks\/` .*| 4000 |\) — |$/\1 `CHANGELOG.md` |/'
  edit README_POLICY.md 's/^\(| `CHANGELOG.md` .*| 200 | — |\) — |$/\1 `docs\/tasks\/` |/'
  arm "onward routes that return to where they started are refused" 1 "onward routes form a cycle"
  fresh; edit README_POLICY.md 's/^\(| `docs\/tasks\/` .*| 4000 |\) — |$/\1 `Z.md` |/'
  arm "an onward route to a destination no row governs is refused" 1 "sends its overflow on to \`Z.md\`, which no row governs"
  fresh; edit README_POLICY.md 's/| 200 | 4000 | — |$/| 200 | — | — |/'
  arm "a directory with no control on its total is refused" 1 "\`docs/tasks/\` has no pressure control on its bytes in total"
  fresh; edit README_POLICY.md '/^| `/d'
  arm "an empty registry is a breach, not a pass" 1 "an empty registry is a breach"
  fresh; printf '#!/usr/bin/env bash\nexit 0\n' > "$work/scripts/check_readme_stability.sh"; git -C "$work" add -A
  arm "a guard that prints no hint is refused, not read as no routes" 1 "README-STABILITY exited 0 with its line cap at 0"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real README's routes are all governed"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "readme-routes self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked=0
scan
if [ "$fail" -ne 0 ]; then
  echo "README-ROUTES: $fail breach(es) — $POLICY registers where the README sends what it does not hold" >&2
  exit 1
fi
debts="$(printf '%s\n' $DEBT | sed '/^$/d' | sort -u | tr '\n' ' ' | sed 's/ $//')"
echo "readme-routes: OK ($checked destination(s) governed${debts:+; recorded as debt, owned by: $debts})"
exit 0
