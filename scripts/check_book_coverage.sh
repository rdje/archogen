#!/usr/bin/env bash
# scripts/check_book_coverage.sh — BOOK-COVERAGE: every workspace member is described by the book, the mirror
# direction of BOOK-ANCHORS (leaf `PROGRAM.24`).
#
# ⭐ WHY THIS EXISTS. `BOOK-ANCHORS` starts from the book and asks whether what it cites exists. Nothing started
# from the code and asked whether the book describes it — and the book is the director's only window into the
# project, so a capability no chapter describes is, to its one reader, a capability that does not exist.
# Measured when this was written: the `rt-analysis` crate — the whole scheduling checker — was named in no
# chapter, while `docs/book/src/analysis.md`, the chapter about exactly what it does, passed `BOOK-ANCHORS` on
# both legs.
#
# THE RULE (the leaf's "strong" shape): every workspace member is NAMED in a chapter that also CITES a
# repository path inside it — the member's directory or a path beneath it, written as `BOOK-ANCHORS` writes a
# citation. ⛔ A name alone does not count: a list of crate names in an appendix would satisfy a name-only rule
# and tell a reader nothing about where anything lives. And the name and the path must be in the SAME chapter,
# or a crate named in one chapter and cited in another is described by neither.
#
# THE POPULATION is derived from the root `Cargo.toml`'s `members` — globs expanded, each member's package name
# read from its own manifest — never listed here, so a crate added tomorrow is in scope with nobody editing this.
#
# ⚠️ HONEST LIMIT: it proves a member is named beside a path into it, never that the chapter describes it well.
# That residue is review, as it is for `BOOK-ANCHORS`.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

BOOK="docs/book/src"
# `BOOK-ANCHORS`' notion of a citation, for the directories a member can live in.
CITED_RE='`(crates|xtask)(/[A-Za-z0-9_./-]*)?`'

# Every member directory the root manifest declares, globs expanded, one per line.
members() {
  local list entry dir
  list="$(awk '/^members[[:space:]]*=/{f=1} f{print} f&&/\]/{exit}' Cargo.toml | tr -d '[]"' | sed 's/^members[[:space:]]*=//' | tr ',' '\n')"
  while IFS= read -r entry; do
    entry="$(printf '%s' "$entry" | tr -d '[:space:]')"
    [ -n "$entry" ] || continue
    for dir in $entry; do
      [ -f "$dir/Cargo.toml" ] && printf '%s\n' "${dir%/}"
    done
  done <<< "$list"
}

package_name() { grep -m1 -E '^name[[:space:]]*=' "$1/Cargo.toml" | sed 's/.*"\(.*\)".*/\1/'; }

fail=0
checked=0
while IFS= read -r dir; do
  [ -n "$dir" ] || continue
  checked=$((checked + 1))
  name="$(package_name "$dir")"
  naming=""; described=""
  for chapter in "$BOOK"/*.md; do
    [ "$(basename "$chapter")" = "SUMMARY.md" ] && continue
    grep -qF -- "$name" "$chapter" || continue
    naming="$naming $(basename "$chapter")"
    if grep -ohE "$CITED_RE" "$chapter" | tr -d '`' | sed 's#/$##' | grep -qE "^$dir(/|$)"; then
      described="$described $(basename "$chapter")"
    fi
  done
  if [ -z "$described" ]; then
    fail=$((fail + 1))
    if [ -z "$naming" ]; then
      printf 'BOOK-COVERAGE: `%s` (%s) is named in no chapter of %s\n' "$name" "$dir" "$BOOK" >&2
    else
      printf 'BOOK-COVERAGE: `%s` (%s) is named in%s without a citation of a path inside %s\n' "$name" "$dir" "$naming" "$dir" >&2
    fi
  fi
done < <(members)

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/book_coverage/selftest"
  fresh() { # a workspace of two members, both described
    rm -rf "$work"; mkdir -p "$work/crates/alpha/src" "$work/crates/beta/src" "$work/$BOOK"
    git -C "$work" init -q
    printf '[workspace]\nmembers = ["crates/*"]\n' > "$work/Cargo.toml"
    printf '[package]\nname = "alpha"\n' > "$work/crates/alpha/Cargo.toml"
    printf '[package]\nname = "beta"\n' > "$work/crates/beta/Cargo.toml"
    printf '# A\n\nThe `alpha` crate lives in `crates/alpha/src/lib.rs`.\n' > "$work/$BOOK/a.md"
    printf '# B\n\nThe `beta` crate is `crates/beta`.\n' > "$work/$BOOK/b.md"
    printf '# Summary\n\n- beta alpha\n' > "$work/$BOOK/SUMMARY.md"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text a refusal must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — refused, but not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "every member named beside a path into it passes (a directory citation counts)" 0 ""
  fresh; printf '# B\n\nNothing about it here.\n' > "$work/$BOOK/b.md"
  arm "a member named in no chapter is refused" 1 '`beta` (crates/beta) is named in no chapter'
  fresh; printf '# B\n\nCrates: alpha, beta.\n' > "$work/$BOOK/b.md"
  arm "a name without a path — the appendix — does not describe a member" 1 '`beta` (crates/beta) is named in b.md without a citation'
  # ⛔ Staged with a member whose package name differs from its directory: a path into `crates/beta/…` contains
  # the name `beta`, so for such a member the name condition is met by the path itself and this arm could not
  # fail — the same-chapter requirement bites only when the two differ.
  fresh; mkdir -p "$work/crates/d"; printf '[package]\nname = "delta-core"\n' > "$work/crates/d/Cargo.toml"
  printf '# B\n\nThe `beta` crate is `crates/beta`; see also `crates/d/src/lib.rs`.\n' > "$work/$BOOK/b.md"
  printf '# C\n\nThe delta-core crate.\n' > "$work/$BOOK/c.md"
  arm "a name in one chapter and a path in another describe it in neither" 1 '`delta-core` (crates/d) is named in c.md without a citation'
  fresh; mkdir -p "$work/crates/gamma"; printf '[package]\nname = "gamma"\n' > "$work/crates/gamma/Cargo.toml"
  arm "a member added under a glob is in scope without editing the check" 1 '`gamma` (crates/gamma) is named in no chapter'
  fresh; printf '# B\n\nThe `beta` crate. See `crates/betamax/x.rs`.\n' > "$work/$BOOK/b.md"
  arm "a path into a member with a longer name is not a path into this one" 1 '`beta` (crates/beta) is named in b.md without a citation'
  fresh; printf '# Summary\n\n- beta `crates/beta`\n' > "$work/$BOOK/SUMMARY.md"; printf '# B\n\nNothing.\n' > "$work/$BOOK/b.md"
  arm "the table of contents does not describe anything" 1 '`beta` (crates/beta) is named in no chapter'
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real book describes every workspace member"
  else echo "SELF-TEST: the real book is refused — run the check to see why" >&2; fi
  echo "book-coverage self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

if [ "$checked" -eq 0 ]; then
  echo "BOOK-COVERAGE: Cargo.toml declares no workspace member this check could read — an empty population is a breach, not a pass" >&2
  exit 1
fi
if [ "$fail" -ne 0 ]; then
  echo "BOOK-COVERAGE: $fail of $checked workspace member(s) described by no chapter — name each beside a path into it" >&2
  exit 1
fi
echo "book-coverage: OK ($checked workspace member(s), each named in a chapter beside a path into it)"
exit 0
