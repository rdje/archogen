#!/usr/bin/env bash
# scripts/check_figure_register.sh — FIGURE-REGISTER: a commit may not add a figure to a live document without saying
# what keeps it true (leaf `PROGRAM.20.3`).
#
# ⭐ WHY THIS EXISTS. The most frequent defect in this repository is a restated figure that went stale: a schema reach
# superseded 47 commits earlier (`M1.23`), a corpus size false from its first commit (`M1.24`), "Three descriptions"
# over a directory of four (`S0.8`), "Three rules" above four (`M1.12.2`), "five of the thirteen" written against the
# test's own `5 of 12` (`PROGRAM.20.2`). Each sweep found the shapes its pattern could see; this makes the commit that
# *adds* a figure answer for it.
#
# THE RULE, per staged live document (book chapters, the normative documents under `docs/semantics/`, `README.md`,
# the three snapshots): the number of **unclassified** figure-shaped phrases may not rise against `HEAD` — the
# `TABLE-ARITY-RATCHET` idiom, so the phrases already there are a reported backlog, not a reason to bypass. A figure is
# classified by a row of `docs/figures.md`:
#   - `gated`       — a named test reads this file and compares the figure with a measurement (the test must exist
#                     and mention the file);
#   - `record`      — a dated measurement, true of its date and never re-read as current (the row gives the date);
#   - `not-a-count` — figure-shaped and not a carried figure ("two descriptions that differ").
# Rows naming a phrase that no longer occurs are stale and refused.
#
# WHAT IS FIGURE-SHAPED: `N of M`; a number followed by a count noun (`3 descriptions`, `541 tests`); a spelled-out
# number followed by one (`three descriptions`). Not: anything in a code fence (`BOOK-TRANSCRIPTS` owns those) or an
# inline code span, a `§N` section reference, or a phrase on a line that carries its own measurement date — a dated
# figure is a record by construction.
#
# ⚠️ HONEST LIMIT: it proves a figure is watched or listed, never that a listed one is correct — `BOOK-ANCHORS`' limit
# one level down. And its shapes are a pattern: a figure spelled some other way is not seen.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

REGISTER="docs/figures.md"
LIVE_RE='^(docs/book/src/[^/]+\.md|docs/semantics/[^/]+\.md|README\.md|LIVE_STATUS\.md|MEMORY\.md|docs/TASK_TREE\.md)$'
fail=0
note() { printf 'FIGURE-REGISTER: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The figure-shaped phrases of the text on stdin, one per line, lower-cased.
figures() {
  perl -ne '
    BEGIN {
      $nouns = q{tests?|descriptions?|cases|files?|leaves|leaf|arms?|entries|entry|constructs?|crates?|chapters?|suites?|rows?|lines?|bytes|commits?|transcripts?|examples?|blocks?|diagnostics?|targets?|members?|properties|mutations?|surfaces?|trees?|steps?|tiers?|gates?|doctrines?|records?|issues?|bugs?|legs?|items?|productions?|rules?|kinds?|shapes?};
      $words = q{two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|thirteen|fourteen|fifteen|twenty|thirty|forty|fifty|hundred};
    }
    if (/^```/) { $fence = !$fence; next }
    next if $fence;
    next if /20\d\d-\d\d-\d\d/;
    s/`[^`]*`//g;
    while (/(?<![§\d])\b(\d[\d, ]*\s+of\s+(?:the\s+)?\d[\d,]*)\b/g) { print lc($1), "\n" }
    while (/(?<![§\d.])\b(\d[\d,]*\s+(?:$nouns))\b/gi) { print lc($1), "\n" }
    while (/\b((?:$words)\s+(?:$nouns))\b/gi) { print lc($1), "\n" }
  '
}

# The register: `file<TAB>phrase<TAB>class<TAB>why`, phrases lower-cased.
register() {
  [ -f "$REGISTER" ] || return 0
  awk '/^\| `/ { line = $0; sub(/^\|[[:space:]]*/, "", line); sub(/[[:space:]]*\|[[:space:]]*$/, "", line)
         n = split(line, c, /[[:space:]]*\|[[:space:]]*/)
         gsub(/`/, "", c[1]); gsub(/`/, "", c[3]); p = tolower(c[2]); gsub(/^"|"$/, "", p)
         print c[1] "\t" p "\t" c[3] "\t" c[4] }' "$REGISTER"
}

# Unclassified phrases of file $1 in text on stdin, given the register in $REG.
unclassified() {
  local file="$1" known
  known="$(printf '%s\n' "$REG" | awk -F'\t' -v f="$file" '$1 == f { print $2 }')"
  figures | while IFS= read -r phrase; do
    printf '%s\n' "$known" | grep -qxF -- "$phrase" || printf '%s\n' "$phrase"
  done
}

scan() {
  REG="$(register)"
  local file class why phrase gate
  # the register's own rows
  while IFS=$'\t' read -r file phrase class why; do
    [ -n "$file" ] || continue
    case "$class" in
      gated)
        gate="$(printf '%s' "$why" | grep -oE '`[^`]+`' | head -n 1 | tr -d '`')"
        if [ -z "$gate" ] || [ ! -f "$gate" ]; then note "$REGISTER: \`$phrase\` in $file is gated by \`${gate:-nothing}\`, which does not exist"
        elif ! grep -qF -- "$file" "$gate"; then note "$REGISTER: \`$phrase\` in $file is gated by $gate, which never reads $file"; fi ;;
      record)
        printf '%s' "$why" | grep -qE '20[0-9]{2}-[0-9]{2}-[0-9]{2}' || note "$REGISTER: \`$phrase\` in $file is a record with no measurement date" ;;
      not-a-count)
        [ -n "$(printf '%s' "$why" | tr -d '[:space:]')" ] || note "$REGISTER: \`$phrase\` in $file is not-a-count with no reason" ;;
      *) note "$REGISTER: \`$phrase\` in $file has class '$class' — write gated, record or not-a-count" ;;
    esac
    if [ -f "$file" ] && ! figures < "$file" | grep -qxF -- "$phrase"; then
      note "$REGISTER: \`$phrase\` no longer occurs in $file — the row is stale"
    fi
  done <<< "$REG"
  # the ratchet, per staged live file
  local staged before after new
  staged="$(git diff --cached --name-only --diff-filter=AM 2>/dev/null | grep -E "$LIVE_RE" || true)"
  while IFS= read -r file; do
    [ -n "$file" ] || continue
    before="$(git show "HEAD:$file" 2>/dev/null | unclassified "$file" | sort)"
    after="$(git show ":$file" 2>/dev/null | unclassified "$file" | sort)"
    backlog=$((backlog + $(printf '%s\n' "$after" | grep -c . || true)))
    if [ "$(printf '%s\n' "$after" | grep -c . || true)" -gt "$(printf '%s\n' "$before" | grep -c . || true)" ]; then
      new="$(comm -13 <(printf '%s\n' "$before") <(printf '%s\n' "$after") | grep . | sort -u | paste -sd';' -)"
      note "$file adds a figure no row of $REGISTER classifies: ${new:-a repeat of one already there} — gate it, date it, or mark it not-a-count"
    fi
  done <<< "$staged"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/figure_register/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/book/src" "$work/tests"
    git -C "$work" init -q
    printf '# Chapter\n\nThe engine has five tiers of checks.\n' > "$work/docs/book/src/ch.md"
    printf '# Figures\n\n| File | Phrase | Class | Why |\n| --- | --- | --- | --- |\n| `docs/book/src/ch.md` | five tiers | gated | `tests/tiers.rs` compares it |\n' > "$work/$REGISTER"
    printf '// reads docs/book/src/ch.md\n' > "$work/tests/tiers.rs"
    printf 'history: 40 commits\n' > "$work/CHANGELOG.md"
    git -C "$work" add -A; git -C "$work" -c user.name=arm -c user.email=arm@example.invalid commit -q -m base
  }
  stage() { printf '%b' "$2" >> "$work/$1"; git -C "$work" add -A; }
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
  fresh; arm "a register whose rows are sound, and nothing staged, passes" 0 ""
  fresh; stage docs/book/src/ch.md 'The directory holds three descriptions.\n'
  arm "a spelled-out figure added unclassified is refused" 1 "adds a figure no row of $REGISTER classifies: three descriptions"
  fresh; stage docs/book/src/ch.md 'It holds 3 descriptions.\n'
  arm "a digits figure added unclassified is refused" 1 "classifies: 3 descriptions"
  fresh; stage docs/book/src/ch.md 'Of these, 5 of 12 differ.\n'
  arm "an N of M figure added unclassified is refused" 1 "classifies: 5 of 12"
  fresh; stage docs/book/src/ch.md 'Measured 2026-09-30: 5 of 12 differ.\n'
  arm "a figure on a line carrying its measurement date is a record by construction" 0 ""
  fresh; stage docs/book/src/ch.md 'See `3 descriptions` and\n\n```text\n7 tests\n```\n'
  arm "a figure in a code span or a code fence is not prose" 0 ""
  fresh; stage docs/book/src/ch.md 'As §7 rule 5 says.\n'
  arm "a section reference is not a figure" 0 ""
  fresh; stage CHANGELOG.md '41 commits later\n'
  arm "a history is not a live document" 0 ""
  fresh; printf '| `docs/book/src/ch.md` | two rows | not-a-count | it names the two rows of a table |\n' >> "$work/$REGISTER"; stage docs/book/src/ch.md 'Compare the two rows.\n'
  arm "a figure classified not-a-count passes" 0 ""
  fresh; printf '// reads nothing relevant\n' > "$work/tests/tiers.rs"; git -C "$work" add -A
  arm "a gated row whose gate never reads the file is refused" 1 "is gated by tests/tiers.rs, which never reads docs/book/src/ch.md"
  fresh; printf '| `docs/book/src/ch.md` | nine gates | record | `2026-09-30` |\n' >> "$work/$REGISTER"; git -C "$work" add -A
  arm "a row whose phrase no longer occurs is stale" 1 "\`nine gates\` no longer occurs in docs/book/src/ch.md"
  fresh; printf '| `docs/book/src/ch.md` | five tiers | record | undated |\n' > "$work/x"; sed -i.bak 's/| five tiers | gated | `tests\/tiers.rs` compares it |/| five tiers | record | undated |/' "$work/$REGISTER"; rm -f "$work/$REGISTER.bak" "$work/x"; git -C "$work" add -A
  arm "a record row with no date is refused" 1 "is a record with no measurement date"
  fresh; sed -i.bak 's/five tiers of checks/tiers of checks/' "$work/docs/book/src/ch.md"; sed -i.bak '/five tiers/d' "$work/$REGISTER"; rm -f "$work"/*.bak "$work/docs/book/src/ch.md.bak" "$work/$REGISTER.bak"; git -C "$work" add -A
  arm "removing a figure and its row passes — the ratchet only refuses a rise" 0 ""
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real register is sound"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "figure-register self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

backlog=0
scan
if [ "$fail" -ne 0 ]; then
  echo "FIGURE-REGISTER: $fail breach(es) — a figure added to a live document says what keeps it true ($REGISTER)" >&2
  exit 1
fi
echo "figure-register: OK (no staged live document adds an unclassified figure; $backlog unclassified figure(s) in the staged live documents are backlog)"
exit 0
