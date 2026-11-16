#!/usr/bin/env bash
# scripts/check_readme_routes.sh — README-ROUTES: every destination the README sends a reader or an author to is
# registered, classified and governed by an explicit policy (leaf `PROGRAM.35.2`, `README_POLICY.md`).
#
# ⭐ WHY THIS EXISTS. A cap on the landing page does not remove the pressure to write things down; it moves it. The
# README's guards tell an author where the detail goes instead, and if that place is unbounded the cap has only
# displaced the growth — `README_POLICY.md` records one adoption whose neighbouring status file reached 1.5 MB that
# way. Every route therefore has an explicit policy. The book, the only user-facing surface, is deliberately
# unbounded under the director's 2026-10-18 ruling (`docs/decisions/decision_book-unbounded.md`).
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
#      (a file's lines, bytes and longest line; a directory's tracked files, an archive stub aside (below), largest
#      file's lines and bytes, longest line and total bytes), a registered doctrine that owns that dimension, or `debt: <leaf>` naming an open leaf.
#      Book routes instead require `unbounded` in every applicable dimension; that value is forbidden elsewhere.
#      A PARTITION is a directory row under another: its files meet their own row's per-file ceilings and not the
#      parent's, while the parent's tracked files and total count non-book material beneath it, sub-folders included.
#      Two cells bound what is read rather than how much is kept (`docs/decisions/decision_ceilings-bound-reads.md`,
#      `PROGRAM.117`): `fan-out N` in a directory row's Files — no directory the row governs lists more than N
#      entries, its tracked files (archive stubs aside) and its sub-folders, so a sub-folder takes the growth — and
#      `guard N` in its Total bytes, a runaway guard. A guard refuses a total over it; a total at three quarters of it
#      unless the cell reads `guard N, ruling asked by <leaf>`, an open leaf, or `guard N, ruled by <record>`, the
#      tracked ruling that kept it; and a commit growing the folder past its step, a sixty-fourth of N, measured in the
#      index against HEAD, unless README_ROUTES_GROWTH_LEAF names a leaf open at HEAD, or new in the commit.
#      Every number is whole — no leading zero, at most 18 digits — so no comparison fails open.
#      Book files contribute only to book-route metrics, so a bounded ancestor cannot impose an indirect book limit;
#   4. every onward destination has a row, and no chain of onward routes is a cycle;
#   5. an empty registry, a guard that prints no hint, or a malformed cell is a breach, not a pass;
#   6. no ceiling is above the maximum a decision fixes for it, in the table under `### Ceilings a decision fixes`, and
#      of its kind — a plain number, a `fan-out` or a `guard`, never a debt or a doctrine — and every decision that
#      table names exists; a maximum fixed for a directory binds, in that column, every row registered beneath it
#      whose nearest ancestor the table fixes there is that directory, the row itself included — a file row in the
#      per-file columns alone, which are the only ones it has.
# `--report` prints each directory row's files, widest directory and total, its growth since the newest commit at
# least a fortnight old, and when at that pace it is refused: telemetry, never a verdict.
#
# ⚠️ HONEST LIMITS: a guard's step compares the index with HEAD, so the pre-commit hook judges each commit and CI, which
# checks a pushed tip against itself, judges none: a commit made with `--no-verify` is not seen, an amended commit is
# measured against the commit it replaces, and a tree split across commits passes. The other dimensions measure the
# working tree, as they always have. A ruling clause is checked to name a tracked decision record, not that the record
# rules on this guard. A deferral to a doctrine is checked to name a registered doctrine, not that the doctrine bounds
# this file; the onward routes of a registered row are declared, and compared with nothing but the
# registry; every path a guard's hint prints counts as an overflow route, the policy it cites for reference included;
# and a third guard on `README.md` would have to be added below.
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

# A leaf's status as a revision records it, or nothing. $1 = the leaf, $2 = `HEAD` or `index`, the trees a commit made
# now would be measured against and would hold.
leaf_status_in() {
  local f
  if [ "$2" = HEAD ]; then git ls-tree --name-only HEAD -- docs/tasks/; else git ls-files -- docs/tasks/; fi |
    while IFS= read -r f; do
      case "$f" in docs/tasks/*/*) ;; *.md) if [ "$2" = HEAD ]; then git cat-file blob "HEAD:$f"; else git cat-file blob ":$f"; fi ;; esac
    done | LEAF="$1" awk '
      !found && $0 == "- ID: `" ENVIRON["LEAF"] "`" { want = 1; next }
      want { if (match($0, /Status: `[a-z_]+`/)) { print substr($0, RSTART + 9, RLENGTH - 10); found = 1 }; want = 0 }'
}

# The bytes the index holds under $1, book files aside: what a commit made now would hold there.
index_bytes() {
  git ls-files -s -- "$1" | awk -F'\t' '$2 !~ /^"?docs\/book\// { split($1, m, " "); print m[2] }' |
    git cat-file --batch-check='%(objectsize)' | awk 'NF == 1 && $1 ~ /^[0-9]+$/ { s += $1; next } { bad = 1 } END { print bad ? "unreadable" : s + 0 }'
}

longest() { LC_ALL=C awk '{ if (length($0) > m) m = length($0) } END { print m + 0 }' "$@"; }

# An ARCHIVE STUB is REVIEW-HISTORY's form, exactly: three lines and a final newline, the second blank, the third
# "Archived, byte for byte, in [`<NAME>`](../review-history/<NAME>) — its review closed, and never edited again." for
# the file's own <NAME>, with that archive tracked. It is a signpost to a closed history, which REVIEW-HISTORY proves on
# every commit, not a history a reader opens, so a directory's file count leaves it out — its bytes, lines and width
# are counted as any file's (PROGRAM.70.2, docs/decisions/decision_reviews-folder-ceiling.md).
is_archive_stub() { # $1 = a tracked file
  local f="$1" name="${1##*/}" dir
  case "$f" in */*) dir="${f%/*}" ;; *) dir="." ;; esac
  [ "$(wc -l < "$f" | tr -d ' ')" -eq 3 ] && [ -z "$(tail -c 1 "$f")" ] && [ -z "$(sed -n 2p "$f")" ] || return 1
  [ "$(sed -n 3p "$f")" = "Archived, byte for byte, in [\`$name\`](../review-history/$name) — its review closed, and never edited again." ] || return 1
  git ls-files --error-unmatch -- "$dir/../review-history/$name" >/dev/null 2>&1
}

# A whole number as the registry writes one: no sign, no leading zero, at most 18 digits — so a test of it is a
# 64-bit comparison that cannot fail open, nor read it as octal.
WHOLE="write a whole number, no leading zero, at most 18 digits"
whole() { [[ "$1" =~ ^(0|[1-9][0-9]{0,17})$ ]]; }

# The kind of a cell and its number, tab-separated: `plain`, `fan-out` or `guard` (a guard's ruling clause aside).
kind_of() {
  case "$1" in
    'fan-out '*) printf 'fan-out\t%s\n' "${1#fan-out }" ;;
    'guard '*) local n="${1#guard }"; n="${n%%, ruling asked by *}"; printf 'guard\t%s\n' "${n%%, ruled by *}" ;;
    *) printf 'plain\t%s\n' "$1" ;;
  esac
}

# The registered row governing $1: its own, or the deepest registered directory above it.
governing_row() {
  local path="$1" dest best=""
  while IFS= read -r dest; do
    if [ "$dest" = "$path" ]; then best="$dest"; break; fi
    case "$dest" in */) case "$path" in "$dest"*) [ "${#dest}" -gt "${#best}" ] && best="$dest" ;; esac ;; esac
  done <<< "$DESTS"
  printf '%s' "$best"
}

# The widest directory a row governs, as "<entries>\t<directory>": the most entries one directory lists — the files
# on stdin directly in it, and its sub-folders holding any. A registered partition is one entry of its parent and
# governs its own listing. $1 = the destination; stdin = its tracked files, book files and archive stubs left out.
widest() {
  DEST="$1" DESTS="$DESTS" awk '
    BEGIN {
      n = split(ENVIRON["DESTS"], d, "\n")
      for (i = 1; i <= n; i++) if (d[i] ~ /\/$/) rows[d[i]] = 1
      dest = ENVIRON["DEST"]
    }
    function owner(dir,   best, r) {
      if (dir in owners) return owners[dir]
      best = ""; for (r in rows) if (index(dir, r) == 1 && length(r) > length(best)) best = r
      return owners[dir] = best
    }
    $0 != "" {
      k = split(substr($0, length(dest) + 1), p, "/"); dir = dest
      for (j = 1; j <= k; j++) {
        entry = p[j] (j < k ? "/" : "")
        if (!((dir, entry) in seen)) { seen[dir, entry] = 1; if (owner(dir) == dest) count[dir]++ }
        dir = dir p[j] "/"
      }
    }
    END {
      m = 0; w = dest
      for (x in count) if (count[x] > m || (count[x] == m && x < w)) { m = count[x]; w = x }
      printf "%d\t%s\n", m, w
    }'
}

# $1 = destination, $2 = dimension, $3 = cell, $4 = measured value ("" when the dimension does not apply), $5 = the
# directory a fan-out measured
control() {
  local dest="$1" dim="$2" cell="$3" actual="$4" leaf status
  case "$dest" in docs/book/*)
    if [ -n "$actual" ] && [ "$cell" != unbounded ]; then
      note "\`$dest\`'s $dim must be unbounded — the director removed all book growth limits"
      return
    fi
    ;;
  esac
  case "$cell" in
    unbounded)
      case "$dest" in
        docs/book/*) [ -n "$actual" ] || note "\`$dest\` declares unbounded $dim, which does not apply to it" ;;
        *) note "\`$dest\`'s $dim is unbounded — that ruling applies only to book routes" ;;
      esac
      ;;
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
    # A fan-out or a guard in its own cell of a directory row is judged by `fanned` or `guarded`; one reaching here
    # stands where it bounds nothing.
    'fan-out '*) note "\`$dest\`'s $dim is '$cell' — a fan-out bounds only the Files of a directory row" ;;
    'guard '*) note "\`$dest\`'s $dim is '$cell' — a runaway guard bounds only the Total bytes of a directory row" ;;
    [A-Z]*)
      grep -qxF -- "$cell" <<< "$DOCTRINES" ||
        note "\`$dest\`'s $dim is deferred to '$cell', which is not a registered doctrine"
      ;;
    *[!0-9]*)
      note "\`$dest\`'s $dim ceiling is '$cell' — write a number, a registered doctrine, or \`debt: <leaf>\`"
      ;;
    *)
      if ! whole "$cell"; then note "\`$dest\`'s $dim ceiling is '$cell' — $WHOLE"
      elif [ -z "$actual" ]; then note "\`$dest\` has a $dim ceiling, and a $dim does not apply to it"
      elif ! whole "$actual" || ! [ "$actual" -le "$cell" ]; then
        note "$dest: $actual $dim, over its ceiling of $cell — compact it under an owned leaf; raising a ceiling needs a decision record"
      fi
      ;;
  esac
}

# A `fan-out N` cell. $1 = destination, $2 = the cell, $3 = the entries of its widest listing, $4 = that directory.
fanned() {
  local dest="$1" limit="${2#fan-out }"
  whole "$limit" || { note "\`$dest\`'s Files is '$2' — $WHOLE"; return; }
  whole "$3" || { note "\`$dest\`'s widest listing measured '$3' — what is not a number is not compared"; return; }
  [ "$3" -le "$limit" ] ||
    note "$4: $3 entries, over the fan-out of $limit \`$dest\` allows — give a subject a sub-folder of its own, or move accepted designs on (\`docs/decisions/decision_ceilings-bound-reads.md\`)"
}

# A `guard N` cell, `guard N, ruling asked by <leaf>` or `guard N, ruled by <record>`. $1 = destination, $2 = the
# cell, $3 = its total on disk, $4 = its total at HEAD and $5 = in the index ("" for both with no commit to compare).
# Three refusals: over the guard; at three quarters of it unless an open leaf asks the director for a ruling, or the
# ruling that kept it is a tracked record; and a commit growing the folder past its step, a sixty-fourth of the guard,
# over HEAD in the index — what the commit holds — unless README_ROUTES_GROWTH_LEAF names a leaf open at HEAD, or new.
guarded() {
  local dest="$1" cell="$2" tb="$3" was="$4" staged="$5" limit clause="" asked="" ruled="" status step leaf
  limit="${cell#guard }"
  case "$limit" in
    *', ruling asked by '*) clause=asked; asked="$(printf '%s' "${limit#*, ruling asked by }" | tr -d '`')"; limit="${limit%%, ruling asked by *}" ;;
    *', ruled by '*) clause=ruled; ruled="$(printf '%s' "${limit#*, ruled by }" | tr -d '`')"; limit="${limit%%, ruled by *}" ;;
  esac
  if ! whole "$limit" || [ "$limit" = 0 ]; then note "\`$dest\`'s Total bytes is '$cell' — a runaway guard is \`guard N\`, \`guard N, ruling asked by <leaf>\` or \`guard N, ruled by <record>\`, N $WHOLE"; return; fi
  case "$clause" in
    asked)
      status=""; [ -z "$asked" ] || status="$(leaf_status "$asked")"
      if [ -z "$asked" ]; then note "\`$dest\`'s guard says a ruling is asked, and names no leaf"; asked=""
      elif [ -z "$status" ]; then note "\`$dest\`'s guard names \`$asked\` as asking for a ruling, and no tree declares it"
      elif [ "$status" = "done" ]; then note "\`$dest\`'s guard names \`$asked\` as asking for a ruling, and \`$asked\` is done — write the guard the ruling fixed, or a leaf still asking"
      else ASKED="$ASKED $asked"; fi
      ;;
    ruled)
      # A decision record, tracked under its literal path — not a pathspec, which `.` or `*` would satisfy.
      case "$ruled" in
        */INDEX.md|*/TEMPLATE.md) false ;;
        docs/decisions/*.md|docs/specs/*.md)
          [ -f "$ruled" ] && [ "$(git --literal-pathspecs ls-files -- "$ruled")" = "$ruled" ] &&
            grep -q '^- \*\*Type:\*\* `decision`' "$ruled" ;;
        *) false ;;
      esac || { note "\`$dest\`'s guard says it was kept by a ruling, and \`$ruled\` is no tracked record under \`docs/decisions/\` or \`docs/specs/\` whose type is \`decision\`"; ruled=""; }
      ;;
  esac
  whole "$tb" || { note "\`$dest\`'s total measured '$tb' — what is not a number is not compared"; return; }
  if ! [ "$tb" -le "$limit" ]; then
    note "$dest: $tb bytes in total, over its runaway guard of $limit — a guard is not a budget: find what wrote this much, a generated or copied file, before any ruling raises it"
    return
  fi
  if [ -z "$asked$ruled" ] && [ $((tb * 4)) -ge $((limit * 3)) ]; then
    note "$dest: $tb bytes in total, three quarters of its runaway guard of $limit — file a leaf that asks the director for a ruling, and name it in the cell: \`guard $limit, ruling asked by <leaf>\`"
  fi
  [ -n "$was" ] || return
  whole "$staged" && whole "$was" || { note "\`$dest\`'s index or HEAD measured '$staged' or '$was' — what is not a number is not compared"; return; }
  step=$((limit / 64))
  [ $((staged - was)) -le "$step" ] && return
  leaf="${README_ROUTES_GROWTH_LEAF:-}"
  if [ -z "$leaf" ]; then
    note "$dest: $((staged - was)) bytes more in the index than at HEAD, past its step of $step in one commit, a sixty-fourth of its guard — a generated or copied file? Split an honest change into commits, or name the leaf that owns it: README_ROUTES_GROWTH_LEAF=<ID>"
    return
  fi
  status="$(leaf_status_in "$leaf" HEAD)"
  if [ -z "$status" ] && [ -n "$(leaf_status_in "$leaf" index)" ]; then status="new"; fi
  case "$status" in
    '') note "$dest: README_ROUTES_GROWTH_LEAF names \`$leaf\`, which no tree declares" ;;
    done) note "$dest: README_ROUTES_GROWTH_LEAF names \`$leaf\`, done at HEAD — a closed leaf owns no growth; unset it" ;;
    *) GROWN="$GROWN $dest" ;;
  esac
}

# A cell a decision fixes, against its maximum: of the maximum's kind and no larger. $1 = the row, $2 = the column,
# $3 = its cell, $4 = the maximum, $5 = the decision, $6 = the row above it the maximum is fixed for ("" for its own).
within_fixed() {
  local mform mnum cform cnum by="\`$5\` fixes"
  [ -z "$6" ] || by="$by for \`$6\`, binding the rows beneath it"
  IFS=$'\t' read -r mform mnum <<< "$(kind_of "$4")"
  IFS=$'\t' read -r cform cnum <<< "$(kind_of "$3")"
  if ! whole "$cnum"; then
    # A malformed number, plain or not, is the row's own check's to name; anything else changed the cell's kind.
    case "$3" in 'fan-out '*|'guard '*|[0-9]*) ;; *)
      note "\`$1\`'s $2 is '$3', of another kind than the '$4' that $by — a cell a decision fixes keeps its maximum's kind" ;;
    esac
  elif [ "$cform" != "$mform" ]; then
    note "\`$1\`'s $2 is '$3', of another kind than the '$4' that $by — a cell a decision fixes keeps its maximum's kind"
  else
    [ "$cnum" -le "$mnum" ] || note "\`$1\`'s $2 ceiling is $3, above the $4 that $by — raising it needs a new ruling"
  fi
}

scan() {
  [ -f "$TARGET" ] || { note "$TARGET is missing — it is the page whose routes this governs"; return; }
  [ -f "$POLICY" ] || { note "$POLICY is missing — nothing registers where the README routes"; return; }
  local rows nav ovf dest route owner life files lines bytes width total onward p row
  rows="$(registry_rows)"
  [ -n "$rows" ] || { note "$POLICY has no rows under \`### $HEADING\` — an empty registry is a breach, not a pass"; return; }
  DESTS="$(printf '%s\n' "$rows" | cut -f1 | tr -d '`')"
  DOCTRINES="$(registered_doctrines)"
  DEBT=""; ASKED=""; GROWN=""
  HAS_HEAD=""; git rev-parse -q --verify HEAD >/dev/null && HAS_HEAD=1

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
        grep -qxF -- "$to" <<< "$reached" || { reached="$reached$to"$'\n'; next="$next$to"$'\n'; }
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
      # Paths read whole, NUL-separated: `git ls-files` without `-z` quotes a name holding a tab, a quote or a byte
      # past ASCII, and a word-split list breaks one holding a space (`PROGRAM.118`).
      local listed=() p
      while IFS= read -r -d '' p; do listed+=("$p"); done < <(git ls-files -z -- "$dest")
      if [ "${#listed[@]}" -eq 0 ]; then note "\`$dest\` is registered, and nothing under it is tracked"; continue; fi
      # Per-file figures over the files this row governs; counts/totals include ordinary nested partitions.
      # Book files belong only to unbounded book-route metrics. Ancestors retain their existing controls over
      # other material, including the archive-stub exception to the file count alone.
      local n=0 ml=0 mb=0 tb=0 l b f own=() counted="" wide where was staged
      for f in "${listed[@]}"; do
        if [[ "$f" == docs/book/* && "$dest" != docs/book/* ]]; then
          continue
        fi
        case "$f" in *$'\n'*)
          note "a tracked path under \`$dest\` holds a newline, which no listing reads whole — rename it: $(printf '%q' "$f")"
          continue
          ;;
        esac
        is_archive_stub "$f" || { n=$((n + 1)); counted="$counted$f"$'\n'; }
        b=$(wc -c < "$f" | tr -d ' '); tb=$((tb + b))
        [ "$(governing_row "$f")" = "$dest" ] || continue
        own+=("$f")
        l=$(wc -l < "$f" | tr -d ' ')
        [ "$l" -gt "$ml" ] && ml=$l; [ "$b" -gt "$mb" ] && mb=$b
      done
      case "$dest:$files" in
        docs/book/*) control "$dest" "tracked files, archive stubs aside" "$files" "$n" ;;
        *':fan-out '*)
          IFS=$'\t' read -r wide where <<< "$(widest "$dest" <<< "$counted")"
          fanned "$dest" "$files" "$wide" "$where"
          ;;
        *) control "$dest" "tracked files, archive stubs aside" "$files" "$n" ;;
      esac
      control "$dest" "lines in its largest file" "$lines" "$ml"
      control "$dest" "bytes in its largest file" "$bytes" "$mb"
      control "$dest" "bytes on its longest line" "$width" "$( [ "${#own[@]}" -gt 0 ] && longest "${own[@]}" || echo 0 )"
      case "$dest:$total" in
        docs/book/*) control "$dest" "bytes in total" "$total" "$tb" ;;
        *':guard '*)
          was=""; staged=""
          if [ -n "$HAS_HEAD" ]; then
            was="$(git ls-tree -r -l HEAD -- "$dest" | awk -F'\t' '$2 !~ /^"?docs\/book\// { split($1, m, " "); s += m[4] } END { print s + 0 }')"
            staged="$(index_bytes "$dest")"
          fi
          guarded "$dest" "$total" "$tb" "$was" "$staged"
          ;;
        *) control "$dest" "bytes in total" "$total" "$tb" ;;
      esac
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

  # 6. Ceilings a decision fixes: no row's cell above its maximum, nor of another kind, and every decision named
  # exists. A maximum fixed for a directory binds, in that column, every row registered beneath it whose nearest
  # ancestor the table fixes there is that directory: otherwise a partition's or a file's row would lift the bound
  # with no ruling. A file row has no Files or Total bytes for it to bind, and is bound in the per-file columns.
  local caps cdest ccol cmax cdec col cell mnum sub up named
  caps="$(CAPS="Ceilings a decision fixes" awk '
    $0 == "### " ENVIRON["CAPS"] { on = 1; next }
    on && /^#/ { exit }
    on && /^\|/ {
      if ($0 !~ /[A-Za-z0-9]/) next
      line = $0; sub(/^\|[[:space:]]*/, "", line); sub(/[[:space:]]*\|[[:space:]]*$/, "", line)
      n = split(line, cells, /[[:space:]]*\|[[:space:]]*/)
      if (!header_seen) { header_seen = 1; next }
      out = cells[1]; for (i = 2; i <= n; i++) out = out "\t" cells[i]
      print out
    }' "$POLICY")"
  while IFS=$'\t' read -r cdest ccol cmax cdec; do
    cdest="${cdest//\`/}"; cdec="${cdec//\`/}"
    [ -n "$cdest" ] || continue
    case "$cdest" in docs/book/*)
      note "\`$cdest\` has a decision ceiling — the director removed all book growth limits"
      ;;
    esac
    [ -f "$cdec" ] || note "the ceiling of \`$cdest\`'s $ccol is fixed by \`$cdec\`, which does not exist"
    case "$ccol" in
      Files) col=5 ;; Lines) col=6 ;; Bytes) col=7 ;; "Longest line") col=8 ;; "Total bytes") col=9 ;;
      *) note "\`### Ceilings a decision fixes\` names a column '$ccol' the registry does not have"; continue ;;
    esac
    IFS=$'\t' read -r _ mnum <<< "$(kind_of "$cmax")"
    if ! whole "$mnum"; then
      note "\`### Ceilings a decision fixes\` gives \`$cdest\`'s $ccol the maximum '$cmax' — a number, \`fan-out N\` or \`guard N\`, N $WHOLE"
      continue
    fi
    cell="$(printf '%s\n' "$rows" | awk -F'\t' -v d="\`$cdest\`" -v c="$col" '$1 == d { print $c }')"
    if [ -z "$cell" ]; then note "\`### Ceilings a decision fixes\` names \`$cdest\`, which no row registers"; continue; fi
    within_fixed "$cdest" "$ccol" "$cell" "$cmax" "$cdec" ""
    case "$cdest" in */)
      while IFS= read -r sub; do
        [ "$sub" != "$cdest" ] && [[ "$sub" == "$cdest"* ]] || continue
        case "$sub" in docs/book/*) continue ;; esac
        case "$ccol:$sub" in Files:*[!/]|'Total bytes':*[!/]) continue ;; esac
        # The nearest ancestor the table fixes in this column, the row itself included, is the one that binds it.
        up="$sub"; named=""
        while [ "$up" != "$cdest" ]; do
          [ -z "$(awk -F'\t' -v d="$up" -v c="$ccol" '{ gsub(/`/, "", $1) } $1 == d && $2 == c' <<< "$caps")" ] || { named=1; break; }
          up="${up%/}"; up="${up%/*}/"
        done
        [ -z "$named" ] || continue
        cell="$(printf '%s\n' "$rows" | awk -F'\t' -v d="\`$sub\`" -v c="$col" '$1 == d { print $c }')"
        within_fixed "$sub" "$ccol" "$cell" "$cmax" "$cdec" "$cdest"
      done <<< "$DESTS"
      ;;
    esac
  done <<< "$caps"
}

# Telemetry, not a verdict: each directory row's files, widest directory and total, its growth since the newest commit
# at least a fortnight old, and when at that pace its total is refused — at its bound, or for a guard with no ruling clause at
# three quarters of it. The sealed evaluation set's history ends at
# `6d61f65` (`docs/evaluation/README.md`): where that commit exists, no older one is read, even for sizes.
report() {
  local rows base floor=6d61f65 days dest cell kind f b n tb counted wide where was limit refuse share rate reach
  rows="$(registry_rows)"
  [ -n "$rows" ] || { echo "README-ROUTES: $POLICY has no rows under \`### $HEADING\` to report on" >&2; return 1; }
  DESTS="$(printf '%s\n' "$rows" | cut -f1 | tr -d '`')"
  base="$(git rev-list -1 --before='14 days ago' HEAD)"
  [ -n "$base" ] || base="$(git rev-list --max-parents=0 HEAD | tail -n 1)"
  if git cat-file -e "$floor^{commit}" 2>/dev/null && ! git merge-base --is-ancestor "$floor" "$base"; then base="$floor"; fi
  days=$(( ($(git log -1 --format=%ct HEAD) - $(git log -1 --format=%ct "$base")) / 86400 ))
  printf 'README-ROUTES report: each directory row; growth since %s, %s day(s) before HEAD\n' "$(git rev-parse --short "$base")" "$days"
  printf '%-24s %5s %8s %10s  %-16s %6s %9s  %-16s %s\n' destination files entries bytes bound share bytes/day "refused in" "widest directory"
  while IFS=$'\t' read -r dest _ _ _ _ _ _ _ cell _; do
    dest="${dest//\`/}"
    case "$dest" in docs/book/*|'') continue ;; */) ;; *) continue ;; esac
    n=0; tb=0; counted=""
    while IFS= read -r -d '' f; do
      [[ "$f" == docs/book/* ]] && continue
      b=$(wc -c < "$f" | tr -d ' '); tb=$((tb + b))
      is_archive_stub "$f" || { n=$((n + 1)); counted="$counted$f"$'\n'; }
    done < <(git ls-files -z -- "$dest")
    IFS=$'\t' read -r wide where <<< "$(widest "$dest" <<< "$counted")"
    was="$(git ls-tree -r -l "$base" -- "$dest" | awk -F'\t' '$2 !~ /^"?docs\/book\// { split($1, m, " "); s += m[4] } END { print s + 0 }')"
    IFS=$'\t' read -r kind limit <<< "$(kind_of "$cell")"; whole "$limit" || limit=""
    # A guard with no ruling clause refuses at three quarters of it; that is the date that matters.
    refuse="$limit"
    case "$kind:$cell" in guard:*', ruling asked by '*|guard:*', ruled by '*) ;; guard:*) [ -z "$limit" ] || refuse=$((limit * 3 / 4)) ;; esac
    share="-"; rate="-"; reach="-"
    [ -n "$limit" ] && [ "$limit" -gt 0 ] && share="$(awk -v a="$tb" -v l="$limit" 'BEGIN { printf "%.1f%%", 100 * a / l }')"
    [ "$days" -gt 0 ] && rate=$(( (tb - was) / days ))
    if [ -n "$refuse" ] && [ "$tb" -ge "$refuse" ]; then reach="now"
    elif [ -n "$refuse" ] && [ "$rate" != "-" ] && [ "$rate" -gt 0 ]; then reach="$(( (refuse - tb) / rate )) day(s)"; fi
    printf '%-24s %5s %8s %10s  %-16s %6s %9s  %-16s %s\n' "$dest" "$n" "$wide" "$tb" "${cell:--}" "$share" "$rate" "$reach" "$where"
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
    if ! sed -i.bak "$2" "$work/$1" || ! rm -f "$work/$1.bak" || ! git -C "$work" add -A; then
      echo "SELF-TEST: fixture edit failed for $1" >&2
      exit 2
    fi
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass), $4 = text it must not
    local name="$1" want="$2" must="$3" never="${4:-}" out rc
    arms=$((arms + 1))
    # shellcheck disable=SC2086 # ARM_ENV is a list of NAME=value words for env
    out="$(cd "$work" && env ${ARM_ENV:-} bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$never" ] && grep -qF -- "$never" <<< "$out"; then
      echo "SELF-TEST: $name — says \`$never\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
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
  # A partition: docs/tasks/sub/ has its own row, reached through docs/tasks/'s onward cell, with larger per-file
  # ceilings. Its file is over the parent's per-file bytes and within its own, and the parent's total still counts it.
  partition() {
    edit README_POLICY.md 's/^\(| `docs\/tasks\/` .*| 4000 |\) — |$/\1 `docs\/tasks\/sub\/` |/'
    edit README_POLICY.md '/^| `docs\/tasks\/` |/a\
| `docs/tasks/sub/` | overflow | W | partitioned_canonical | 4 | 100 | 3000 | 200 | 6000 | — |'
    mkdir -p "$work/docs/tasks/sub"
  }
  fresh; partition; for i in $(seq 50); do printf '%049d\n' 0; done > "$work/docs/tasks/sub/S.md"; git -C "$work" add -A
  arm "a partition's file meets its own row's per-file ceilings, not its parent's" 0 ""
  fresh; partition; for f in S R; do for i in $(seq 45); do printf '%049d\n' 0; done > "$work/docs/tasks/sub/$f.md"; done; git -C "$work" add -A
  arm "under a plain total, a partition adds no capacity: its files count toward the parent's" 1 "docs/tasks/: 4562 bytes in total, over its ceiling of 4000"
  # Archive stubs (PROGRAM.70.2): docs/tasks/ at its four files, then a fifth that is REVIEW-HISTORY's exact stub,
  # its archive tracked, which the count leaves out; a look-alike — no archive behind it, or a fourth line — counts.
  stubbed() { # $1 = the stub's extra fourth line, or "" for none; $2 = 1 to track its archive
    for f in U V W; do printf 'u\n' > "$work/docs/tasks/$f.md"; done
    printf '# X\n\nArchived, byte for byte, in [`X.md`](../review-history/X.md) — its review closed, and never edited again.\n%s' "$1" > "$work/docs/tasks/X.md"
    if [ "$2" = 1 ]; then mkdir -p "$work/docs/review-history"; printf '# X, the history\n' > "$work/docs/review-history/X.md"; fi
    git -C "$work" add -A
  }
  fresh; stubbed "" 1
  arm "an archive stub is no file of its directory's count" 0 ""
  fresh; stubbed "" 0
  arm "a stub-shaped file with no archive behind it is counted" 1 "docs/tasks/: 5 tracked files"
  fresh; stubbed $'a fourth line\n' 1
  arm "a stub-shaped file of four lines is counted" 1 "docs/tasks/: 5 tracked files"
  fresh; stubbed 'text after its last newline' 1
  arm "a stub-shaped file with text after its last newline is counted" 1 "docs/tasks/: 5 tracked files"
  fresh; stubbed "" 1; sed -i.bak '2s/^$/not blank/' "$work/docs/tasks/X.md"; rm -f "$work/docs/tasks/X.md.bak"; git -C "$work" add -A
  arm "a stub-shaped file whose second line is not blank is counted" 1 "docs/tasks/: 5 tracked files"
  fresh; stubbed "" 1; git -C "$work" mv docs/tasks/X.md docs/tasks/Y.md
  arm "a stub that names another file than itself is counted" 1 "docs/tasks/: 5 tracked files"
  fresh; stubbed "" 1; printf '# Z, another history\n' > "$work/docs/review-history/Z.md"
  sed -i.bak '3s|(../review-history/X.md)|(../review-history/Z.md)|' "$work/docs/tasks/X.md"; rm -f "$work/docs/tasks/X.md.bak"; git -C "$work" add -A
  arm "a stub that links another archive than its own is counted" 1 "docs/tasks/: 5 tracked files"
  fresh; stubbed "" 1; for f in U V W; do for i in $(seq 26); do printf '%049d\n' 0; done > "$work/docs/tasks/$f.md"; done   # 3 × 1 300 bytes
  git -C "$work" add -A
  arm "an archive stub's bytes count toward its directory's total" 1 "bytes in total, over its ceiling of 4000"
  fresh; printf 'a decision\n' > "$work/D.md"; printf '\n### Ceilings a decision fixes\n\n| Destination | Column | Maximum | Decision |\n| --- | --- | --- | --- |\n| `docs/tasks/` | Total bytes | 3000 | `D.md` |\n' >> "$work/README_POLICY.md"; git -C "$work" add -A
  arm "a ceiling above the maximum a decision fixes is refused" 1 "above the 3000 that \`D.md\` fixes"
  fresh; printf '\n### Ceilings a decision fixes\n\n| Destination | Column | Maximum | Decision |\n| --- | --- | --- | --- |\n| `docs/tasks/` | Total bytes | 9000 | `NO-SUCH.md` |\n' >> "$work/README_POLICY.md"; git -C "$work" add -A
  arm "a cap whose decision does not exist is refused" 1 "which does not exist"
  # The book may exceed every former dimension together, while route governance still applies.
  book() {
    mkdir -p "$work/docs/book/src"
    printf '\n[book](docs/book/)\n' >> "$work/README.md"
    edit README_POLICY.md '/^| `A.md` |/a\
| `docs/book/` | navigation | W | maintained_reference | unbounded | unbounded | unbounded | unbounded | unbounded | — |'
    for i in $(seq 49); do printf '# Chapter %s\n' "$i" > "$work/docs/book/src/chapter-$i.md"; done
    for i in $(seq 751); do printf '%01025d\n' 0; done > "$work/docs/book/src/chapter-1.md"
    git -C "$work" add -A
  }
  fresh; book
  arm "the book can exceed all former file, line, byte, width and total limits" 0 ""
  local dim
  for dim in 5 6 7 8 9; do
    fresh; book
    awk -F'|' -v c="$dim" 'BEGIN { OFS="|" } /^\| `docs\/book\/` / { $(c+1)=" 9999999 " } { print }' \
      "$work/README_POLICY.md" > "$work/policy.next"
    mv "$work/policy.next" "$work/README_POLICY.md"; git -C "$work" add -A
    arm "a book growth ceiling in column $dim is refused even above current usage" 1 "must be unbounded"
  done
  fresh; book; edit README_POLICY.md 's/maintained_reference | unbounded |/maintained_reference | — |/'
  arm "an unspecified book dimension is refused" 1 "must be unbounded"
  fresh; book; edit README_POLICY.md 's/^| `docs\/book\/` | navigation |/| `docs\/book\/` | overflow |/'
  arm "an unbounded book still needs the correct derived route" 1 "what routes to it derives 'navigation'"
  fresh; edit README_POLICY.md 's/| — | 10 | 1000 | 100 |/| — | unbounded | 1000 | 100 |/'
  arm "the book ruling cannot remove another destination's limit" 1 "that ruling applies only to book routes"
  book_partition() {
    edit README_POLICY.md 's/^\(| `docs\/book\/` .*\) — |$/\1 `docs\/book\/src\/` |/'
    edit README_POLICY.md '/^| `docs\/book\/` |/a\
| `docs/book/src/` | overflow | W | maintained_reference | unbounded | unbounded | unbounded | unbounded | unbounded | — |'
  }
  fresh; book; book_partition
  arm "nested book routes also remain unbounded" 0 ""
  edit README_POLICY.md 's/^\(| `docs\/book\/src\/` .*\) unbounded | — |$/\1 9999999 | — |/'
  arm "a nested book route cannot reintroduce a growth ceiling" 1 "must be unbounded"
  fresh; book
  printf '\n### Ceilings a decision fixes\n\n| Destination | Column | Maximum | Decision |\n| --- | --- | --- | --- |\n| `docs/book/` | Total bytes | 9999999 | `A.md` |\n' >> "$work/README_POLICY.md"; git -C "$work" add -A
  arm "an obsolete book decision ceiling is refused" 1 "has a decision ceiling"
  # An ancestor's budget must never become an indirect book growth ceiling.
  book_ancestor() {
    printf '\n[all docs](docs/)\n' >> "$work/README.md"
    edit README_POLICY.md '/^| `docs\/book\/` |/a\
| `docs/` | navigation | W | maintained_reference | 1 | 100 | 3000 | 200 | 1000 | — |'
  }
  fresh; book; book_ancestor
  arm "a bounded ancestor excludes book growth from its file and byte budgets" 0 ""
  printf 'another document\n' > "$work/docs/extra.md"; git -C "$work" add -A
  arm "the ancestor still counts every non-book file" 1 "docs/: 2 tracked files"
  fresh; book; book_ancestor
  edit README_POLICY.md 's@^\(| `docs/` .*\) 1 | 100 |@\1 3 | 100 |@'
  for i in $(seq 40); do printf '%049d\n' 0; done > "$work/docs/extra.md"; git -C "$work" add -A
  arm "the ancestor still limits non-book bytes" 1 "bytes in total, over its ceiling of 1000"
  # Paths read whole (PROGRAM.118): a name holding a space, or a byte past ASCII, is measured as any other; a name
  # holding a newline, which no listing reads whole, is refused.
  fresh; printf '%0250d\n' 0 > "$work/docs/tasks/a b.md"; git -C "$work" add -A
  arm "a file whose name holds a space is measured, its longest line among the row's" 1 "docs/tasks/: 250 bytes on its longest line, over its ceiling of 200"
  fresh; for i in $(seq 42); do printf '%049d\n' 0; done > "$work/docs/tasks/é.md"; git -C "$work" add -A
  arm "a file whose name holds a byte past ASCII is measured" 1 "docs/tasks/: 2100 bytes in its largest file, over its ceiling of 2000"
  fresh; printf 'n\n' > "$work/docs/tasks/x"$'\n'"y.md"; git -C "$work" add -A
  arm "a file whose name holds a newline is refused, not misread" 1 "holds a newline, which no listing reads whole"
  # Ceilings that bound reads (PROGRAM.117): `docs/tasks/` at `fan-out $1`, its total a `guard 4000`.
  bounded() {
    edit README_POLICY.md 's/^\(| `docs\/tasks\/` | navigation + overflow | W | partitioned_canonical |\) 4 |\(.*\)| 4000 |/\1 fan-out '"$1"' |\2| guard 4000 |/'
  }
  caps() { # $1 = the one row of a `### Ceilings a decision fixes` inside the adoption note, its decision D.md
    printf 'a decision\n' > "$work/D.md"
    edit README_POLICY.md '/^<!-- README-POLICY-LOCAL-ADOPTION:END -->$/i\
\
### Ceilings a decision fixes\
\
| Destination | Column | Maximum | Decision |\
| --- | --- | --- | --- |\
'"$1"
  }
  fresh; bounded 3; printf 'u\n' > "$work/docs/tasks/U.md"; printf 'v\n' > "$work/docs/tasks/V.md"; git -C "$work" add -A
  arm "a directory listing as many entries as its fan-out passes" 0 "" "three quarters"
  printf 'w\n' > "$work/docs/tasks/W.md"; git -C "$work" add -A
  arm "a directory listing one entry more than its fan-out is refused, by its path" 1 "docs/tasks/: 4 entries, over the fan-out of 3"
  fresh; bounded 3; mkdir -p "$work/docs/tasks/sub"; for f in U V W; do printf 'u\n' > "$work/docs/tasks/sub/$f.md"; done
  printf 'x\n' > "$work/docs/tasks/X.md"; git -C "$work" add -A
  arm "growth a sub-folder takes passes, though the files outnumber the old count" 0 ""
  printf 'y\n' > "$work/docs/tasks/sub/Y.md"; git -C "$work" add -A
  arm "a sub-folder no row registers has its own listing bounded by the same fan-out" 1 "docs/tasks/sub/: 4 entries, over the fan-out of 3"
  fresh; bounded 3; for d in a b c; do mkdir -p "$work/docs/tasks/$d"; printf 'u\n' > "$work/docs/tasks/$d/U.md"; done; git -C "$work" add -A
  arm "a sub-folder is an entry of its parent's listing" 1 "docs/tasks/: 4 entries, over the fan-out of 3"
  fresh; partition; bounded 3; for f in U V W X; do printf 'u\n' > "$work/docs/tasks/sub/$f.md"; done; git -C "$work" add -A
  arm "a registered partition's listing is its own row's to bound, not its parent's fan-out" 0 ""
  printf 'y\n' > "$work/docs/tasks/sub/Y.md"; git -C "$work" add -A
  arm "and its own row bounds it" 1 "docs/tasks/sub/: 5 tracked files, archive stubs aside, over its ceiling of 4"
  fresh; bounded 4; stubbed "" 1
  arm "an archive stub is no entry of a fan-out" 0 ""
  fresh; bounded 4; stubbed "" 0
  arm "a stub-shaped file with no archive behind it is an entry" 1 "docs/tasks/: 5 entries, over the fan-out of 4"
  fresh; partition; bounded 2; for f in U V W; do printf 'u\n' > "$work/docs/tasks/sub/$f.md"; done; git -C "$work" add -A
  arm "a registered partition is one entry of its parent's listing, its files none" 0 ""
  printf 'x\n' > "$work/docs/tasks/X.md"; git -C "$work" add -A
  arm "so a third entry beside it is over a fan-out of two" 1 "docs/tasks/: 3 entries, over the fan-out of 2"
  # The guard: refused over it, and at three quarters of it unless an open leaf asks the director for a ruling.
  fill_to() { # docs/tasks/ made exactly $1 bytes, by pad files of 50-byte lines
    local have need i=0 f
    have="$(cd "$work" && git ls-files -z -- docs/tasks | xargs -0 cat | wc -c | tr -d ' ')"
    need=$(( $1 - have ))
    while [ "$need" -gt 0 ]; do
      f="$work/docs/tasks/P$i.md"; : > "$f"
      while [ "$need" -ge 50 ] && [ "$(wc -c < "$f" | tr -d ' ')" -lt 1950 ]; do printf '%049d\n' 0 >> "$f"; need=$((need - 50)); done
      if [ "$need" -gt 0 ] && [ "$need" -lt 50 ]; then head -c "$need" /dev/zero | tr '\0' 'p' >> "$f"; need=0; fi
      i=$((i + 1))
    done
    git -C "$work" add -A
  }
  asked() { edit README_POLICY.md 's/| guard 4000 |/| guard 4000, ruling asked by `'"$1"'` |/'; }
  fresh; bounded 4; fill_to 2999
  arm "a total one byte under three quarters of its runaway guard passes" 0 "" "three quarters"
  fresh; bounded 4; fill_to 3000
  arm "a total at three quarters of its runaway guard is refused until a leaf asks for a ruling" 1 "docs/tasks/: 3000 bytes in total, three quarters of its runaway guard of 4000"
  asked T.1
  arm "an open leaf asking for a ruling answers three quarters, and the verdict names it" 0 "a ruling on a guard asked by: T.1"
  fresh; bounded 4; fill_to 3000; asked T.2
  arm "a done leaf asks for nothing" 1 "\`T.2\` is done"
  fresh; bounded 4; fill_to 3000; asked T.9
  arm "a leaf no tree declares asks for nothing" 1 "names \`T.9\` as asking for a ruling, and no tree declares it"
  fresh; bounded 4; fill_to 4001; asked T.1
  arm "a total over its runaway guard is refused as a mistake, a ruling asked or not" 1 "4001 bytes in total, over its runaway guard of 4000 — a guard is not a budget"
  fresh; bounded 4; edit README_POLICY.md 's/| guard 4000 |/| guard 4000 soon |/'
  arm "a guard with anything else after its number is refused" 1 "a runaway guard is \`guard N\`, \`guard N, ruling asked by <leaf>\`"
  fresh; bounded 4; asked ""
  arm "a ruling asked by no leaf is refused" 1 "says a ruling is asked, and names no leaf"
  ruled() { edit README_POLICY.md 's|\| guard 4000 \||\| guard 4000, ruled by `'"$1"'` \||'; }
  fresh; bounded 4; fill_to 3000; mkdir -p "$work/docs/decisions"; printf '# A ruling\n\n- **Type:** `decision`\n' > "$work/docs/decisions/R.md"
  git -C "$work" add -A; ruled docs/decisions/R.md
  arm "a tracked decision record that kept the guard answers three quarters" 0 ""
  local spec
  for spec in docs/decisions/NO-SUCH.md . '*' docs/tasks/T.md docs/decisions/../../README.md docs/decisions/INDEX.md \
    docs/decisions/TEMPLATE.md docs/decisions/U.md docs/decisions/reference_N.md; do
    fresh; bounded 4; fill_to 3000; mkdir -p "$work/docs/decisions"
    for f in INDEX TEMPLATE; do printf '# %s\n\n- **Type:** `decision`\n' "$f" > "$work/docs/decisions/$f.md"; done
    printf '# A note\n\n- **Type:** `reference`\n' > "$work/docs/decisions/reference_N.md"; git -C "$work" add -A
    ruled "$spec"; printf 'on the disk alone\n' > "$work/docs/decisions/U.md"
    arm "a ruling clause naming '$spec' is no record, and refused" 1 "is no tracked record under"
  done
  # Whole numbers only: a leading zero, read as octal by a shell test, or past 64 bits, where a test fails open.
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| fan-out x | 50 | 2000 | 200 | 4000 |/'
  arm "a fan-out that is no whole number is refused" 1 "'fan-out x' — write a whole number"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| fan-out 04 | 50 | 2000 | 200 | 4000 |/'
  arm "a fan-out with a leading zero is refused" 1 "'fan-out 04' — write a whole number, no leading zero"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| 4 | 50 | 2000 | 200 | guard 99999999999999999999 |/'
  arm "a guard past 64 bits is refused, not compared" 1 "'guard 99999999999999999999' — a runaway guard is"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| 4 | 050 | 2000 | 200 | 4000 |/'
  arm "a plain ceiling with a leading zero is refused" 1 "ceiling is '050' — write a whole number"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| 4 | 99999999999999999999 | 2000 | 200 | 4000 |/'
  arm "a plain ceiling past 64 bits is refused, not compared" 1 "ceiling is '99999999999999999999' — write a whole number"
  # The step: one commit grows a guarded folder by at most a sixty-fourth of its guard, 62 bytes of 4000 here.
  commit_fixture() { # $1 = the commit's age in days
    local at; at="$(( $(date +%s) - $1 * 86400 )) +0000"
    GIT_AUTHOR_DATE="$at" GIT_COMMITTER_DATE="$at" git -C "$work" -c core.hooksPath=/dev/null -c commit.gpgsign=false \
      -c user.name=t -c user.email=t@t commit -qm "fixture, $1 day(s) old"
  }
  fresh; bounded 4; commit_fixture 0; fill_to "$(( $(cd "$work" && git ls-files -z -- docs/tasks | xargs -0 cat | wc -c) + 62 ))"
  arm "a commit growing a guarded folder by its step passes" 0 ""
  fresh; bounded 4; commit_fixture 0; fill_to "$(( $(cd "$work" && git ls-files -z -- docs/tasks | xargs -0 cat | wc -c) + 63 ))"
  arm "a commit growing it a byte past its step is refused" 1 "63 bytes more in the index than at HEAD, past its step of 62 in one commit"
  ARM_ENV="README_ROUTES_GROWTH_LEAF=T.1"
  arm "a declared leaf named for that growth owns it, and the verdict names it" 0 "growth past a step owned by \`T.1\`: docs/tasks/"
  ARM_ENV="README_ROUTES_GROWTH_LEAF=T.9"
  arm "a leaf no tree declares owns no growth" 1 "README_ROUTES_GROWTH_LEAF names \`T.9\`, which no tree declares"
  ARM_ENV="README_ROUTES_GROWTH_LEAF=T.2"
  arm "a leaf done at HEAD owns no growth, so a variable left set is refused" 1 "names \`T.2\`, done at HEAD"
  printf -- '\n- ID: `T.7`\n  Status: `pending`\n' >> "$work/docs/tasks/T.md"
  ARM_ENV="README_ROUTES_GROWTH_LEAF=T.7"
  arm "a leaf on the disk the commit does not hold owns no growth" 1 "names \`T.7\`, which no tree declares"
  printf -- '\n- ID: `T.3`\n  Status: `done`\n' >> "$work/docs/tasks/T.md"; git -C "$work" add -A
  ARM_ENV="README_ROUTES_GROWTH_LEAF=T.3"
  arm "a leaf the commit itself files and closes owns its growth" 0 "growth past a step owned by"
  ARM_ENV=""
  fresh; bounded 4; commit_fixture 0; fill_to "$(( $(cd "$work" && git ls-files -z -- docs/tasks | xargs -0 cat | wc -c) + 1000 ))"
  rm -f "$work"/docs/tasks/P*.md
  arm "the step measures the index, so growth staged and gone from the disk is refused" 1 "1000 bytes more in the index than at HEAD"
  fresh; bounded 4; commit_fixture 0
  git -C "$work" update-index --add --cacheinfo 100644,1234567890123456789012345678901234567890,docs/tasks/F.md
  arm "an index entry whose object cannot be sized is refused, not summed" 1 "measured 'unreadable'"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| 4 | fan-out 50 | 2000 | 200 | 4000 |/'
  arm "a fan-out in another column than Files is refused" 1 "a fan-out bounds only the Files of a directory row"
  fresh; edit README_POLICY.md 's/^| `A.md` | navigation | W | maintained_reference | — |/| `A.md` | navigation | W | maintained_reference | fan-out 3 |/'
  arm "a fan-out on a file row is refused" 1 "a fan-out bounds only the Files of a directory row"
  fresh; edit README_POLICY.md 's/^\(| `A.md` | navigation | W | maintained_reference | — | 10 | 1000 | 100 |\) — |/\1 guard 9000 |/'
  arm "a runaway guard on a file row is refused" 1 "a runaway guard bounds only the Total bytes of a directory row"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| 4 | 50 | guard 2000 | 200 | 4000 |/'
  arm "a runaway guard in another column than Total bytes is refused" 1 "a runaway guard bounds only the Total bytes of a directory row"
  fresh; bounded 4; caps '| `docs/tasks/` | Files | fan-out 3 | `D.md` |'
  arm "a fan-out above the one a decision fixes is refused" 1 "ceiling is fan-out 4, above the fan-out 3 that \`D.md\` fixes"
  fresh; bounded 4; caps '| `docs/tasks/` | Total bytes | guard 4000 | `D.md` |'
  arm "a runaway guard at the one a decision fixes passes" 0 ""
  fresh; caps '| `docs/tasks/` | Files | fan-out 9 | `D.md` |'
  arm "a plain count where a decision fixes a fan-out is refused" 1 "a cell a decision fixes keeps its maximum's kind"
  fresh; bounded 4; caps '| `docs/tasks/` | Total bytes | 9000 | `D.md` |'
  arm "a guard where a decision fixes a plain total is refused" 1 "a cell a decision fixes keeps its maximum's kind"
  fresh; edit README_POLICY.md 's/| 4000 | — |$/| debt: `T.1` | — |/'; caps '| `docs/tasks/` | Total bytes | guard 4000 | `D.md` |'
  arm "a cell a decision fixes cannot turn into a debt" 1 "a cell a decision fixes keeps its maximum's kind"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| LIVE-SNAPSHOTS | 50 | 2000 | 200 | 4000 |/'; caps '| `docs/tasks/` | Files | fan-out 4 | `D.md` |'
  arm "nor into a doctrine" 1 "a cell a decision fixes keeps its maximum's kind"
  fresh; bounded 4; caps '| `docs/tasks/` | Total bytes | guard 4000 | `D.md` |'; asked T.1
  arm "a guard's ruling clause leaves its kind and number to the decision's maximum" 0 ""
  fresh; bounded 4; caps '| `docs/tasks/` | Files | fan-out 064 | `D.md` |'
  arm "a decision's maximum with a leading zero is refused" 1 "the maximum 'fan-out 064'"
  fresh; edit README_POLICY.md 's/| 4 | 50 | 2000 | 200 | 4000 |/| 4 | 050 | 2000 | 200 | 4000 |/'; caps '| `docs/tasks/` | Lines | 60 | `D.md` |'
  arm "a malformed number under a decision is named once, as malformed" 1 "ceiling is '050' — write a whole number" "keeps its maximum's kind"
  # A maximum fixed for a directory bounds the partitions registered beneath it.
  fresh; partition; bounded 4; edit README_POLICY.md 's/^\(| `docs\/tasks\/sub\/` | overflow | W | partitioned_canonical |\) 4 |/\1 fan-out 50 |/'
  for i in $(seq 12); do printf 'u\n' > "$work/docs/tasks/sub/F$i.md"; done; git -C "$work" add -A
  arm "a partition's own row may set its fan-out where no decision fixes one" 0 ""
  caps '| `docs/tasks/` | Files | fan-out 4 | `D.md` |'
  arm "a partition's row cannot lift a fan-out a decision fixes for its parent" 1 "fixes for \`docs/tasks/\`, binding the rows beneath it"
  edit README_POLICY.md '/^| `docs\/tasks\/` | Files | fan-out 4 | `D.md` |$/a\
| `docs/tasks/sub/` | Files | fan-out 50 | `D.md` |'
  arm "unless the table fixes the partition's own" 0 ""
  # Beneath that partition, the nearest fixed ancestor binds: `docs/tasks/sub/`'s 50, not `docs/tasks/`' 4.
  edit README_POLICY.md 's/^\(| `docs\/tasks\/sub\/` .*\) — |$/\1 `docs\/tasks\/sub\/deep\/` |/'
  edit README_POLICY.md '/^| `docs\/tasks\/sub\/` | overflow |/a\
| `docs/tasks/sub/deep/` | overflow | W | partitioned_canonical | fan-out 50 | 100 | 3000 | 200 | 6000 | — |'
  mkdir -p "$work/docs/tasks/sub/deep"; printf 'd\n' > "$work/docs/tasks/sub/deep/D.md"; git -C "$work" add -A
  arm "a partition beneath a partition is bound by its nearest fixed ancestor" 0 ""
  edit README_POLICY.md 's/^\(| `docs\/tasks\/sub\/deep\/` | overflow | W | partitioned_canonical |\) fan-out 50 |/\1 fan-out 60 |/'
  arm "and refused above it" 1 "above the fan-out 50 that \`D.md\` fixes for \`docs/tasks/sub/\`"
  # A file row beneath a fixed directory keeps its own cells: a directory's Files and Total bytes do not apply to it.
  fresh; bounded 4; printf '\n[the tree](docs/tasks/T.md)\n' >> "$work/README.md"
  edit README_POLICY.md '/^| `docs\/tasks\/` |/a\
| `docs/tasks/T.md` | navigation | W | maintained_reference | — | 50 | 2000 | 200 | — | — |'
  caps '| `docs/tasks/` | Files | fan-out 4 | `D.md` |'
  arm "a file row beneath a directory with a fixed fan-out keeps its own cells" 0 ""
  # In the per-file columns a file row is bound by its directory's fixed maximum, as a partition is.
  fresh; printf '\n[the tree](docs/tasks/T.md)\n' >> "$work/README.md"
  edit README_POLICY.md '/^| `docs\/tasks\/` |/a\
| `docs/tasks/T.md` | navigation | W | maintained_reference | — | 60 | 2000 | 200 | — | — |'
  caps '| `docs/tasks/` | Lines | 50 | `D.md` |'
  arm "a file row cannot lift a per-file maximum its directory's decision fixes" 1 "\`docs/tasks/T.md\`'s Lines ceiling is 60, above the 50 that \`D.md\` fixes for \`docs/tasks/\`"
  edit README_POLICY.md 's/^\(| `docs\/tasks\/T.md` | navigation | W | maintained_reference | — |\) 60 |/\1 50 |/'
  arm "a file row at that maximum passes" 0 ""
  edit README_POLICY.md 's/^\(| `docs\/tasks\/T.md` | navigation | W | maintained_reference | — |\) 50 |/\1 60 |/'
  edit README_POLICY.md '/^| `docs\/tasks\/` | Lines | 50 | `D.md` |$/a\
| docs/tasks/T.md | Lines | 60 | `D.md` |'
  arm "a file row the table fixes on its own, backticks or not, is bound by its own maximum" 0 ""
  # A row beneath a fixed column takes that column's kind: a doctrine there would leave the decision's bound.
  fresh; partition; bounded 4; edit README_POLICY.md 's/^\(| `docs\/tasks\/sub\/` | overflow | W | partitioned_canonical |\) 4 |/\1 LIVE-SNAPSHOTS |/'
  caps '| `docs/tasks/` | Files | fan-out 4 | `D.md` |'
  arm "a partition beneath a fixed column cannot defer it to a doctrine" 1 "\`docs/tasks/sub/\`'s Files is 'LIVE-SNAPSHOTS'"
  fresh; printf '\n[the tree](docs/tasks/T.md)\n' >> "$work/README.md"
  edit README_POLICY.md '/^| `docs\/tasks\/` |/a\
| `docs/tasks/T.md` | navigation | W | maintained_reference | — | 50 | 2000 | LIVE-SNAPSHOTS | — | — |'
  caps '| `docs/tasks/` | Longest line | 200 | `D.md` |'
  arm "nor can a file row beneath a fixed per-file column" 1 "\`docs/tasks/T.md\`'s Longest line is 'LIVE-SNAPSHOTS', of another kind than the '200' that \`D.md\` fixes for \`docs/tasks/\`"
  fresh; bounded 4; caps '| `docs/tasks/` | Files | fan-out many | `D.md` |'
  arm "a decision's maximum that is no number is refused" 1 "the maximum 'fan-out many'"
  # The report: a commit twenty days old, then one adding 40 bytes, two a day over that window.
  fresh; bounded 4; commit_fixture 20; printf '%039d\n' 0 > "$work/docs/tasks/U.md"; git -C "$work" add -A; commit_fixture 0
  local report_out report_rc
  arms=$((arms + 1))
  report_out="$(cd "$work" && bash "$SELF" --report 2>&1)"; report_rc=$?
  if [ "$report_rc" -eq 0 ] && grep -qE '20 day\(s\) before HEAD' <<< "$report_out" &&
    grep -qE '^docs/tasks/ +2 +2 +[0-9]+ +guard 4000 +[0-9.]+% +2 +1449 day\(s\) +docs/tasks/$' <<< "$report_out"; then
    ok=$((ok + 1)); echo "  ✅ the report gives a directory row its files, widest listing, total, share, growth and when three quarters refuse it"
  else
    echo "SELF-TEST: the report — exit $report_rc:" >&2; printf '%s\n' "$report_out" | sed -n '1,4p' | sed 's/^/    /' >&2
  fi
  fresh
  local edit_out edit_rc
  arms=$((arms + 1))
  edit_out="$(edit README_POLICY.md 's/[' 2>&1)"; edit_rc=$?
  if [ "$edit_rc" -eq 2 ] && grep -qF 'fixture edit failed for README_POLICY.md' <<< "$edit_out"; then
    ok=$((ok + 1)); echo "  ✅ failed fixture preparation refuses before any verdict"
  else
    echo "SELF-TEST: failed fixture preparation must exit 2, got $edit_rc" >&2
  fi
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
if [ "${1:-}" = "--report" ]; then
  report
  exit $?
fi

checked=0
scan
if [ "$fail" -ne 0 ]; then
  echo "README-ROUTES: $fail breach(es) — $POLICY registers where the README sends what it does not hold" >&2
  exit 1
fi
debts="$(printf '%s\n' $DEBT | sed '/^$/d' | sort -u | tr '\n' ' ' | sed 's/ $//')"
asked="$(printf '%s\n' $ASKED | sed '/^$/d' | sort -u | tr '\n' ' ' | sed 's/ $//')"
grown="$(printf '%s\n' $GROWN | sed '/^$/d' | sort -u | tr '\n' ' ' | sed 's/ $//')"
echo "readme-routes: OK ($checked destination(s) governed${debts:+; recorded as debt, owned by: $debts}${asked:+; a ruling on a guard asked by: $asked}${grown:+; growth past a step owned by \`${README_ROUTES_GROWTH_LEAF:-}\`: $grown})"
exit 0
