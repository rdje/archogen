#!/usr/bin/env bash
# scripts/check_source_ledger.sh — SOURCE-LEDGER: what this project relies on from outside is written down,
# at the version it is pinned to, and every document that names such a source cites its entry (leaf
# `PROGRAM.5`).
#
# ⭐ WHY THIS EXISTS. ROADMAP.md §15: "Maintain a dependency/evidence ledger with exact upstream source
# versions, retrieval dates, hashes where captured, scope, known limitations, and revalidation triggers. Claims
# about external tools belong in this ledger rather than as timeless assertions inside architecture decisions."
# Measured when this was written: the book and the decision records carried **no** external URL at all — every
# external source was named in prose (QEMU, LinkedSpec, chipdoc, …), in 12 files, and nothing tied a name to a
# version. The ledger is `docs/book/src/ledger.md`, one section per source.
#
# THREE LEGS:
#   1. PINS — derived, never listed: every gitlink, every `*_VERSION_PINNED=` in a tracked `targets/*.env`,
#      `DOCTRINE_VERSION`, `rust-toolchain.toml`'s channel and every CI `uses:` ref. Each is claimed by exactly
#      one entry's `Pinned at` (as `gitlink:<path>`, `env:<file>:<VAR>`, `file:<path>`, `toml:<file>:<key>` or
#      `uses:<action>`), and that entry's `Version` carries the pinned value. A pin that moves without its entry
#      is refused; so is an entry claiming a pin the repository no longer has.
#   2. CITATIONS — every book chapter and decision record that names a ledgered source (by a name in the
#      entry's `Named as`) links to its entry, `ledger.md#<id>`; and every such link leads to an entry.
#      A name counts only as a whole word: `Cargo.toml` is a file, not a mention of Cargo.
#   3. ENTRIES — every field present and non-empty, `Retrieved` an absolute date, no version given as
#      "latest" (§19: "no unverified latest version is prescribed"), a `Pinned at` that names a pin or says
#      "not pinned", at least one name.
#
# ⚠️ HONEST LIMIT: a claim is found by its source's NAME. A claim about a source the ledger does not name at
# all is not seen — that residue is review. And a version is checked against the pin, not against the world:
# an installed tool that drifted from an unpinned entry is not seen either (`PROGRAM.30`).
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

LEDGER="docs/book/src/ledger.md"
FIELDS="Source|Version|Pinned at|Retrieved|Hash|Scope|Known limitations|Revalidation trigger|Named as"

fail=0
note() { printf 'SOURCE-LEDGER: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The ledger as `id<TAB>field<TAB>value` lines: a section is `## \`<id>\``, a field is a two-cell table row.
entries() {
  awk '
    /^## `[a-z0-9-]+`[[:space:]]*$/ { id = $0; sub(/^## `/, "", id); sub(/`.*$/, "", id); print id "\t\t"; next }
    /^## / { id = ""; next }
    id != "" && /^\| [^|]+ \| .* \|[[:space:]]*$/ {
      line = $0; sub(/^\| /, "", line); sub(/ \|[[:space:]]*$/, "", line)
      k = index(line, " | "); if (k == 0) next
      f = substr(line, 1, k - 1); v = substr(line, k + 3)
      if (f == "Field" || f ~ /^-+$/) next
      print id "\t" f "\t" v
    }' "$LEDGER"
}

# Every pin the repository holds, as `locator<TAB>value`.
pins() {
  git ls-files -s | awk '$1 == "160000" { print "gitlink:" $4 "\t" $2 }'
  local f
  while IFS= read -r f; do
    [ -f "$f" ] || continue
    sed -nE 's/^([A-Z0-9_]+_VERSION_PINNED)=["'"'"']?([^"'"'"']*)["'"'"']?[[:space:]]*$/\1\t\2/p' "$f" |
      while IFS=$'\t' read -r var val; do printf 'env:%s:%s\t%s\n' "$f" "$var" "$val"; done
  done < <(git ls-files -- 'targets/*.env')
  if git ls-files --error-unmatch DOCTRINE_VERSION >/dev/null 2>&1; then
    printf 'file:DOCTRINE_VERSION\t%s\n' "$(head -n 1 DOCTRINE_VERSION)"
  fi
  if git ls-files --error-unmatch rust-toolchain.toml >/dev/null 2>&1; then
    sed -nE 's/^channel[[:space:]]*=[[:space:]]*"([^"]+)".*/toml:rust-toolchain.toml:channel\t\1/p' rust-toolchain.toml
  fi
  while IFS= read -r f; do
    [ -f "$f" ] || continue
    sed -nE 's/^[[:space:]-]*uses:[[:space:]]*([^@[:space:]]+)@([^[:space:]#]+).*/uses:\1\t\2/p' "$f"
  done < <(git ls-files -- '.github/workflows/*.yml' '.github/workflows/*.yaml')
}

# The documents whose claims must resolve: book chapters and decision records, never the indexes.
documents() {
  git ls-files -- 'docs/book/src/*.md' 'docs/decisions/*.md' |
    grep -vxE "docs/book/src/SUMMARY\.md|$LEDGER|docs/decisions/INDEX\.md|docs/decisions/TEMPLATE\.md"
}

scan() {
  if [ ! -f "$LEDGER" ]; then
    note "$LEDGER is missing — the external sources this project relies on are written down nowhere"; return
  fi
  local table ids id f v field loc val owners n docs doc line name
  table="$(entries)"
  ids="$(printf '%s\n' "$table" | awk -F'\t' '$2 == "" && $1 != "" { print $1 }')"
  if [ -z "$ids" ]; then
    note "$LEDGER has no entry — an empty ledger is a breach, not a pass"; return
  fi
  printf '%s\n' "$ids" | sort | uniq -d | while IFS= read -r id; do [ -n "$id" ] && echo "SOURCE-LEDGER: entry \`$id\` appears twice" >&2; done
  [ -n "$(printf '%s\n' "$ids" | sort | uniq -d)" ] && fail=$((fail + 1))
  field_of() { printf '%s\n' "$table" | awk -F'\t' -v id="$1" -v f="$2" '$1 == id && $2 == f { print $3; exit }'; }

  # 3. ENTRIES — one pass over the parsed table.
  while IFS= read -r msg; do [ -n "$msg" ] && note "$msg"; done < <(printf '%s\n' "$table" | FIELDS="$FIELDS" awk -F'\t' '
    $2 == "" && $1 != "" { order[++n] = $1; next }
    $1 != "" { v[$1 SUBSEP $2] = $3 }
    END {
      m = split(ENVIRON["FIELDS"], want, "|")
      for (i = 1; i <= n; i++) {
        id = order[i]
        for (j = 1; j <= m; j++) { x = v[id SUBSEP want[j]]; gsub(/[[:space:]]/, "", x); if (x == "") print "entry `" id "` has no `" want[j] "`" }
        r = v[id SUBSEP "Retrieved"]
        if (r !~ /^`?[0-9][0-9][0-9][0-9]-[0-9][0-9]-[0-9][0-9]`?/) print "entry `" id "`: Retrieved is \x27" r "\x27 — write an absolute date, YYYY-MM-DD"
        if (tolower(v[id SUBSEP "Version"]) ~ /(^|[^a-z])latest([^a-z]|$)/) print "entry `" id "`: a version given as \"latest\" is no version (ROADMAP.md §19)"
        pa = v[id SUBSEP "Pinned at"]
        if (pa !~ /`(gitlink|env|file|toml|uses):[^`]+`/ && pa !~ /^not pinned/) print "entry `" id "`: Pinned at names no pin and does not say \"not pinned\""
        if (v[id SUBSEP "Named as"] !~ /`[^`]+`/) print "entry `" id "` gives no name to find its claims by"
      }
    }')

  # 1. PINS — every pin claimed by exactly one entry carrying its value; every claimed pin still held.
  local claims held
  claims="$(printf '%s\n' "$table" | awk -F'\t' '$2 == "Pinned at" { s = $3
    while (match(s, /`(gitlink|env|file|toml|uses):[^`]+`/)) { print $1 "\t" substr(s, RSTART + 1, RLENGTH - 2); s = substr(s, RSTART + RLENGTH) } }')"
  held="$(pins)"
  while IFS=$'\t' read -r loc val; do
    [ -n "$loc" ] || continue
    owners="$(printf '%s\n' "$claims" | awk -F'\t' -v l="$loc" '$2 == l { print $1 }')"
    n="$(printf '%s' "$owners" | grep -c . || true)"
    if [ "$n" -eq 0 ]; then
      note "the repository pins $loc = '$val', and no entry of $LEDGER claims it — add one"
    elif [ "$n" -gt 1 ]; then
      note "$loc is claimed by more than one entry: $(printf '%s' "$owners" | paste -sd, -)"
    elif ! printf '%s' "$(field_of "$owners" Version)" | grep -qF -- "$val"; then
      note "entry \`$owners\` does not carry the pinned value of $loc — the repository pins '$val'; move the entry with the pin"
    fi
  done <<< "$held"
  while IFS=$'\t' read -r id loc; do
    [ -n "$loc" ] || continue
    printf '%s\n' "$held" | awk -F'\t' -v l="$loc" '$1 == l { found = 1 } END { exit !found }' ||
      note "entry \`$id\` claims the pin $loc, which the repository no longer has — the entry is stale"
  done <<< "$claims"

  # 2. CITATIONS — a document naming a source cites its entry; every citation resolves. One pass per document:
  # the pairs `id<TAB>name` go in through the environment, and the pass reports each entry the document names
  # without citing (at its first mention) and each citation that leads nowhere.
  local pairs
  pairs="$(printf '%s\n' "$table" | awk -F'\t' '$2 == "Named as" { s = $3
    while (match(s, /`[^`]+`/)) { print $1 "\t" substr(s, RSTART + 1, RLENGTH - 2); s = substr(s, RSTART + RLENGTH) } }')"
  docs="$(documents)"
  while IFS= read -r doc; do
    [ -n "$doc" ] || continue
    while IFS=$'\t' read -r kind a b c; do
      case "$kind" in
        mention) note "$doc:$a names $b without citing its entry — link ledger.md#$c" ;;
        unknown) note "$doc cites ledger.md#$a, which is no entry of $LEDGER" ;;
      esac
    done < <(PAIRS="$pairs" awk '
      BEGIN { n = split(ENVIRON["PAIRS"], rows, "\n")
              for (r = 1; r <= n; r++) { if (rows[r] == "") continue; split(rows[r], f, "\t"); pid[r] = f[1]; pname[r] = f[2]; known[f[1]] = 1 } }
      { line[NR] = $0; s = $0
        while (match(s, /ledger\.md#[a-z0-9-]+/)) { cited[substr(s, RSTART + 10, RLENGTH - 10)] = 1; s = substr(s, RSTART + RLENGTH) } }
      # The first line naming `name` as a whole word — not inside a longer word, and not a file name or glob
      # ("Cargo.toml", "Cargo.*"): a `.` ends a sentence only when a space or the line end follows it.
      function first(name,    k, s, off, i, before, after, after2) {
        for (k = 1; k <= NR; k++) {
          s = line[k]; off = 0
          while ((i = index(s, name)) > 0) {
            before = (off + i > 1) ? substr(line[k], off + i - 1, 1) : ""
            after = substr(s, i + length(name), 1); after2 = substr(s, i + length(name) + 1, 1)
            if (before !~ /[A-Za-z0-9_\/-]/ && after !~ /[A-Za-z0-9_\/-]/ && !(after == "." && after2 != "" && after2 !~ /[[:space:]]/)) return k
            off += i + length(name) - 1; s = substr(s, i + length(name))
          }
        }
        return 0
      }
      END {
        for (c in cited) if (!(c in known)) print "unknown\t" c
        for (r = 1; r <= n; r++) {
          if (!(r in pid) || (pid[r] in cited) || (pid[r] in reported)) continue
          if ((k = first(pname[r])) > 0) { print "mention\t" k "\t" pname[r] "\t" pid[r]; reported[pid[r]] = 1 }
        }
      }' "$doc")
  done <<< "$docs"

  checked_entries="$(printf '%s\n' "$ids" | grep -c .)"
  checked_pins="$(printf '%s\n' "$held" | grep -c . || true)"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/source_ledger/selftest"
  entry() { # $1 id, $2 version, $3 pinned at, $4 named as, $5 retrieved
    printf '\n## `%s`\n\n| Field | Value |\n| --- | --- |\n| Source | the %s source |\n| Version | %s |\n| Pinned at | %s |\n| Retrieved | %s |\n| Hash | not captured |\n| Scope | the arms |\n| Known limitations | none measured |\n| Revalidation trigger | the pin moving |\n| Named as | %s |\n' \
      "$1" "$1" "$2" "$3" "${5:-\`2026-09-29\`}" "$4"
  }
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/book/src" "$work/docs/decisions" "$work/targets"
    git -C "$work" init -q
    { printf '# Ledger\n'
      entry alpha '`1.2.3`' '`env:targets/t.env:ALPHA_VERSION_PINNED`' '`Alpha`'
      entry beta 'none — not adopted' 'not pinned — a reference' '`Beta`'
    } > "$work/$LEDGER"
    printf 'TARGET_ID=t\nALPHA_VERSION_PINNED=1.2.3\n' > "$work/targets/t.env"
    printf '# A\n\nAlpha runs the target ([Alpha](ledger.md#alpha)).\n' > "$work/docs/book/src/a.md"
    printf '# D\n\nBeta is read, never built against ([Beta](../book/src/ledger.md#beta)).\n' > "$work/docs/decisions/d.md"
    printf '# Summary\n\n- [Alpha and Beta](a.md)\n' > "$work/docs/book/src/SUMMARY.md"
    printf '# Index\n\nAlpha, Beta.\n' > "$work/docs/decisions/INDEX.md"
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
  fresh; arm "a ledger whose pins and citations agree passes; the indexes need not cite" 0 ""
  fresh; sed -i.bak 's/=1.2.3/=1.2.4/' "$work/targets/t.env"; rm -f "$work/targets/t.env.bak"
  arm "a pin moved without its entry is refused" 1 "the repository pins '1.2.4'"
  fresh; printf 'GAMMA_VERSION_PINNED=9\n' >> "$work/targets/t.env"
  arm "a new pin with no entry is refused" 1 "env:targets/t.env:GAMMA_VERSION_PINNED = '9', and no entry"
  fresh; printf 'TARGET_ID=t\n' > "$work/targets/t.env"
  arm "an entry claiming a pin the repository no longer has is stale" 1 "entry \`alpha\` claims the pin env:targets/t.env:ALPHA_VERSION_PINNED"
  fresh; git -C "$work" update-index --add --cacheinfo "160000,$(git -C "$work" hash-object -t blob /dev/null),vendor/v"
  arm "a vendored checkout with no entry is refused" 1 "gitlink:vendor/v"
  fresh; printf 'jobs:\n  a:\n    steps:\n      - uses: acme/setup@v2\n' > "$work/ci.yml"; mkdir -p "$work/.github/workflows"; mv "$work/ci.yml" "$work/.github/workflows/"; git -C "$work" add -A
  arm "a CI action ref with no entry is refused" 1 "uses:acme/setup = 'v2'"
  fresh; printf '[toolchain]\nchannel = "1.95.0"\n' > "$work/rust-toolchain.toml"; git -C "$work" add -A
  arm "a toolchain channel with no entry is refused" 1 "toml:rust-toolchain.toml:channel = '1.95.0'"
  fresh; printf 'scaffold 1.0\n' > "$work/DOCTRINE_VERSION"; git -C "$work" add -A
  arm "a scaffold version with no entry is refused" 1 "file:DOCTRINE_VERSION = 'scaffold 1.0'"
  fresh; entry alpha '`1.2.3`' 'not pinned — a second copy' '`Alpha`' >> "$work/$LEDGER"
  arm "an entry that appears twice is refused" 1 "entry \`alpha\` appears twice"
  fresh; entry gamma '`1.2.3`' '`env:targets/t.env:ALPHA_VERSION_PINNED`' '`Gamma`' >> "$work/$LEDGER"
  arm "a pin claimed by two entries is refused" 1 "claimed by more than one entry: alpha,gamma"
  fresh; printf '# B\n\nAlpha is fast.\n' > "$work/docs/book/src/b.md"; git -C "$work" add -A
  arm "a chapter naming a source without citing it is refused" 1 "docs/book/src/b.md:3 names Alpha"
  fresh; printf '# E\n\nWe rely on Beta.\n' > "$work/docs/decisions/e.md"; git -C "$work" add -A
  arm "a decision naming a source without citing it is refused" 1 "docs/decisions/e.md:3 names Beta"
  fresh; printf '# B\n\nSee [it](ledger.md#gamma).\n' > "$work/docs/book/src/b.md"; git -C "$work" add -A
  arm "a citation to no entry is refused" 1 "cites ledger.md#gamma, which is no entry"
  fresh; printf '# B\n\nThe Alphabet, and Alpha.toml, and Alpha.*, and AlphaBeta.\n' > "$work/docs/book/src/b.md"; git -C "$work" add -A
  arm "a name inside a longer word, or a file name, is not a mention" 0 ""
  fresh; { printf '# Ledger\n'; entry alpha '`1.2.3`' '`env:targets/t.env:ALPHA_VERSION_PINNED`' '`Alpha`' 'last week'
           entry beta 'none' 'not pinned — a reference' '`Beta`'; } > "$work/$LEDGER"
  arm "a relative retrieval date is refused" 1 "Retrieved is 'last week'"
  fresh; { printf '# Ledger\n'; entry alpha 'the latest `1.2.3`' '`env:targets/t.env:ALPHA_VERSION_PINNED`' '`Alpha`'
           entry beta 'none' 'not pinned — a reference' '`Beta`'; } > "$work/$LEDGER"
  arm "a version given as latest is refused" 1 "entry \`alpha\`: a version given as \"latest\""
  fresh; sed -i.bak '/^| Hash |/d' "$work/$LEDGER"; rm -f "$work/$LEDGER.bak"
  arm "an entry with a field missing is refused" 1 "has no \`Hash\`"
  fresh; { printf '# Ledger\n'; entry alpha '`1.2.3`' 'somewhere' '`Alpha`'; entry beta 'none' 'not pinned' '`Beta`'; } > "$work/$LEDGER"
  arm "a Pinned at that neither names a pin nor says not pinned is refused" 1 "Pinned at names no pin"
  fresh; printf '# Ledger\n\nNothing yet.\n' > "$work/$LEDGER"
  arm "a ledger with no entry is a breach, not a pass" 1 "has no entry"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real ledger carries every pin, and every naming document cites it"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "source-ledger self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked_entries=0; checked_pins=0
scan
if [ "$fail" -ne 0 ]; then
  echo "SOURCE-LEDGER: $fail breach(es) — $LEDGER is where an external source's version and limits are written down (ROADMAP.md §15)" >&2
  exit 1
fi
echo "source-ledger: OK ($checked_entries entries; $checked_pins pin(s), each carried by its entry; every naming document cites it)"
exit 0
