#!/usr/bin/env bash
# BOOK-ANCHORS — the book's claims about the code resolve to code that exists.
#
# ⭐ WHY THIS EXISTS. The mdBook is the project's public surface and, for its director, the only
# window into it — the code is not read, the book is. That makes a plausible, confident chapter
# describing something the engine no longer does the single most expensive kind of drift here: it
# is not merely wrong, it is wrong in the one artifact used to decide whether the work is right.
#
# Two legs, and they fail for different reasons:
#
#   1. ANCHORED   — every chapter that describes behavior cites at least one repository path.
#                   A chapter with no anchor cannot be checked against anything, by this script
#                   or by a reader; it is an essay about a system rather than a description of
#                   one. Measured when this check was written: `presence.md` cited nothing.
#   2. RESOLVABLE — every repository path a chapter cites EXISTS. This is the leg that catches
#                   real drift: a renamed module, a deleted fixture, a crate that became
#                   something else. The book keeps reading fine, which is the problem.
#
# ⛔ MATCHED ON A REPOSITORY PATH, NOT ON ANY FILENAME. A chapter legitimately names
# `src/main.rs` of a GENERATED crate, and `os-rt.eadl` by its basename in prose. Requiring those
# to exist at the repository root would make writing about generated output cost a doctrine
# breach, which is how a gate teaches authors to route around it — the same false positive
# `scripts/check_s0_retirement.sh` hit on its first run, and the same fix: match what is
# unambiguously a claim about this repository.
#
# ⚠️ HONEST LIMIT: this proves a chapter points somewhere real, never that what it says about
# that place is true. Prose cannot be checked by grep. It removes the cheapest failure — a
# citation that has rotted — and leaves the expensive one to review.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic; read-only.
# `--self-test` runs the RED arms.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

SRC="docs/book/src"
# Chapters that are deliberately not about behavior, so leg 1 does not apply.
NO_ANCHOR_NEEDED='^(SUMMARY|introduction)$'
# What counts as a claim about this repository: a path under a tracked top-level directory, or a
# named root document.
PATH_RE='(crates|scripts|examples|docs|xtask|targets|knowledge-map)/[A-Za-z0-9_./-]+|(ROADMAP|COMMIT|TOOLBOX|MEMORY|LIVE_STATUS|CHANGELOG|DEV_NOTES|README|DOCTRINE_ENFORCEMENT|MEMORY_ARCHITECTURE|README_POLICY)\.md|Makefile|Cargo\.toml'

fail=0
note() { printf 'BOOK-ANCHORS: %s\n' "$1" >&2; fail=1; }

[ -d "$SRC" ] || exit 0

for chapter in "$SRC"/*.md; do
  stem="$(basename "$chapter" .md)"
  # Paths are cited inside code spans; that is the convention every chapter already follows and
  # it keeps prose like "the docs/ directory" out of the match.
  cited="$(grep -ohE "\`($PATH_RE)\`" "$chapter" 2>/dev/null | tr -d '`' | sort -u || true)"

  if ! printf '%s' "$stem" | grep -qE "$NO_ANCHOR_NEEDED"; then
    if [ -z "$cited" ]; then
      note "$chapter describes behavior but cites no repository path. A chapter with no anchor
  cannot be checked against anything — name the module, script or fixture that implements it."
    fi
  fi

  while IFS= read -r path; do
    [ -n "$path" ] || continue
    # A trailing slash names a directory; a trailing `*` a glob. Both resolve to their parent.
    probe="${path%/}"
    probe="${probe%/\*}"
    if [ ! -e "$probe" ]; then
      note "$chapter cites \`$path\`, which does not exist. The book is the project's public
  surface: a citation that has rotted reads exactly like one that has not."
    fi
  done <<< "$cited"
done

if [ "${1:-}" = "--self-test" ]; then
  tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
  arms=0; ok=0
  victim="$SRC/__self_test__.md"

  # Arm 1: a behavior chapter with no anchor at all.
  arms=$((arms + 1))
  printf '# Self test\n\nThis chapter describes behavior and cites nothing.\n' > "$victim"
  "$0" >/dev/null 2>&1 && echo "SELF-TEST: arm 1 did not fail on an unanchored chapter" >&2 || ok=$((ok + 1))
  rm -f "$victim"

  # Arm 2: a chapter citing a path that does not exist.
  arms=$((arms + 1))
  printf '# Self test\n\nImplemented by `crates/no-such-crate/src/gone.rs`.\n' > "$victim"
  "$0" >/dev/null 2>&1 && echo "SELF-TEST: arm 2 did not fail on a rotted citation" >&2 || ok=$((ok + 1))
  rm -f "$victim"

  # Arm 3: a chapter citing a real path passes — the check must not simply always fail.
  arms=$((arms + 1))
  printf '# Self test\n\nImplemented by `crates/eadl-model/src/presence.rs`.\n' > "$victim"
  if "$0" >/dev/null 2>&1; then ok=$((ok + 1)); else
    echo "SELF-TEST: arm 3 failed on a well-formed chapter — the check is not discriminating" >&2
  fi
  rm -f "$victim"

  echo "book-anchors self-test: $ok pass / $((arms - ok)) fail"
  [ "$ok" -eq "$arms" ] || exit 1
  exit 0
fi

[ "$fail" -eq 0 ] || exit 1
count=$(ls "$SRC"/*.md 2>/dev/null | grep -vc '/SUMMARY\.md$' | tr -d ' ')
echo "book-anchors: OK ($count chapter(s); every cited repository path resolves)"
exit 0
