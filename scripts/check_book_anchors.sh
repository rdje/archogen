#!/usr/bin/env bash
# BOOK-ANCHORS — prose that describes the code cites code that exists.
#
# ⭐ WHY THIS EXISTS. The mdBook is the project's public surface and, for its director, the only
# window into it — the code is not read, the book is. That makes a plausible, confident chapter
# describing something the engine no longer does the single most expensive kind of drift here: it
# is not merely wrong, it is wrong in the one artifact used to decide whether the work is right.
#
# TWO POPULATIONS, one pair of legs. The book is the most expensive instance, not the only one: the
# normative documents under `docs/semantics/` are prose about the code in the same sense, and they
# are the authority the code is held to — `crates/eadl-front/tests/reference.rs` reads its tables out
# of `docs/semantics/reference.md` and `conformance.rs` derives a recognizer out of
# `docs/semantics/grammar.md`. A rotted citation there sends an implementer to nothing, which is the
# same failure one layer down. Both populations are walked, and a failure names which one it was on.
#
# Two legs, and they fail for different reasons:
#
#   1. ANCHORED   — every document that describes behavior cites at least one repository path.
#                   A document with no anchor cannot be checked against anything, by this script
#                   or by a reader; it is an essay about a system rather than a description of
#                   one. Measured when this check was written: `presence.md` cited nothing.
#   2. RESOLVABLE — every repository path a document cites EXISTS. This is the leg that catches
#                   real drift: a renamed module, a deleted fixture, a crate that became
#                   something else. The book keeps reading fine, which is the problem.
#
# ⚠️ OVERLAP, STATED RATHER THAN LEFT TO BE DISCOVERED. Leg 2 on `docs/semantics/reference.md` is
# also `reference.rs`'s leg 6, which reports the same rot with a line number and a row name. That is
# deliberate and not duplication by accident: the Rust leg runs inside `cargo test` and can point at
# the table row, while this one runs in the doctrine enforcer and in CI without a build, so the
# doctrine's own claim covers the normative documents and not only the book. The two read the same
# notion of a citation — `reference.rs`'s `REPO_DIRS`/`ROOT_DOCS` mirror `PATH_RE` below — and if one
# widens, the other must widen with it.
#
# ⛔ MATCHED ON A REPOSITORY PATH, NOT ON ANY FILENAME. A chapter legitimately names
# `src/main.rs` of a GENERATED crate, and `os-rt.eadl` by its basename in prose. Requiring those
# to exist at the repository root would make writing about generated output cost a doctrine
# breach, which is how a gate teaches authors to route around it — the same false positive
# `scripts/check_s0_retirement.sh` hit on its first run, and the same fix: match what is
# unambiguously a claim about this repository.
#
# ⚠️ HONEST LIMIT: this proves a document points somewhere real, never that what it says about
# that place is true. Prose cannot be checked by grep. It removes the cheapest failure — a
# citation that has rotted — and leaves the expensive one to review. That residue is not left
# unowned: for the reference's diagnostic table it is `reference.rs`'s legs 4, 5 and 8, which
# compare the stated codes against the code that emits them and against the book that renders them.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic; read-only.
# `--self-test` runs the RED arms.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

SRC="docs/book/src"
# The normative documents: the authority the code is held to, and prose about the code in exactly the
# sense the book is. Top level only — `docs/semantics/boundary/README.md` describes a corpus rather
# than the language, and its own figures are gated by `corpus.rs`.
NORMATIVE_SRC="docs/semantics"
# Chapters that are deliberately not about behavior, so leg 1 does not apply.
NO_ANCHOR_NEEDED='^(SUMMARY|introduction)$'
# What counts as a claim about this repository: a path under a tracked top-level directory, or a
# named root document.
PATH_RE='(crates|scripts|examples|docs|xtask|targets|knowledge-map)/[A-Za-z0-9_./-]+|(ROADMAP|COMMIT|TOOLBOX|MEMORY|LIVE_STATUS|CHANGELOG|DEV_NOTES|README|DOCTRINE_ENFORCEMENT|MEMORY_ARCHITECTURE|README_POLICY)\.md|Makefile|Cargo\.toml'

fail=0
note() { printf 'BOOK-ANCHORS: %s\n' "$1" >&2; fail=1; }

[ -d "$SRC" ] || exit 0

# One document, both legs. `role` is what a failure calls it, so a breach names the surface it was
# found on instead of reporting "a file" and leaving the reader to go and look.
check_document() {
  document="$1"; role="$2"; anchor="$3"
  # Paths are cited inside code spans; that is the convention every chapter already follows and
  # it keeps prose like "the docs/ directory" out of the match.
  cited="$(grep -ohE "\`($PATH_RE)\`" "$document" 2>/dev/null | tr -d '`' | sort -u || true)"

  if [ "$anchor" = yes ] && [ -z "$cited" ]; then
    note "$document is a $role that describes behavior but cites no repository path. A $role with
  no anchor cannot be checked against anything — name the module, script or fixture that implements it."
  fi

  while IFS= read -r path; do
    [ -n "$path" ] || continue
    # A trailing slash names a directory; a trailing `*` a glob. Both resolve to their parent.
    probe="${path%/}"
    probe="${probe%/\*}"
    if [ ! -e "$probe" ]; then
      note "$document cites \`$path\`, which does not exist. This is a $role — the text someone
  follows — and a citation that has rotted reads exactly like one that has not."
    fi
  done <<< "$cited"
}

chapters=0
for chapter in "$SRC"/*.md; do
  [ -e "$chapter" ] || continue
  stem="$(basename "$chapter" .md)"
  # `SUMMARY.md` is a table of contents, not a chapter. This is the same population the count line
  # below has always reported, so the figure does not move when the loop is restructured.
  [ "$stem" = SUMMARY ] || chapters=$((chapters + 1))
  if printf '%s' "$stem" | grep -qE "$NO_ANCHOR_NEEDED"; then
    check_document "$chapter" chapter no
  else
    check_document "$chapter" chapter yes
  fi
done

# ⛔ An empty normative population is a BREACH, not a "not applicable". `reference.rs`
# `include_str!`s `docs/semantics/reference.md`, so the frontend cannot compile without one; a
# directory that vanished would otherwise turn this half of the doctrine into a silent pass, which is
# the vacuous green every other check here exists to refuse.
normative=0
for document in "$NORMATIVE_SRC"/*.md; do
  [ -e "$document" ] || continue
  check_document "$document" "normative document" yes
  normative=$((normative + 1))
done
if [ "$normative" -eq 0 ]; then
  note "no normative document under $NORMATIVE_SRC/. This half of the doctrine would be checking
  nothing, and the language reference the frontend is held to is gone."
fi

if [ "${1:-}" = "--self-test" ]; then
  arms=0; ok=0
  victim="$SRC/__self_test__.md"
  norm_victim="$NORMATIVE_SRC/__self_test__.md"
  # Both victims live inside tracked directories, so an interrupted self-test must not leave one
  # behind: a stray `docs/semantics/__self_test__.md` would be a normative document nobody wrote.
  trap 'rm -f "$victim" "$norm_victim"' EXIT

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

  # Arm 4: a NORMATIVE document with a rotted citation. This is the arm the extended scope exists
  # for — the grammar and the reference are the authority the code is held to, and before this arm
  # nothing walked them, so a rotted citation in one was invisible to the doctrine.
  arms=$((arms + 1))
  printf '# Self test\n\nEnforced by `crates/eadl-front/src/gone.rs`.\n' > "$norm_victim"
  "$0" >/dev/null 2>&1 && echo "SELF-TEST: arm 4 did not fail on a normative document's rotted citation" >&2 || ok=$((ok + 1))
  rm -f "$norm_victim"

  # Arm 5: a normative document with no anchor — leg 1 applies to the new population too.
  arms=$((arms + 1))
  printf '# Self test\n\nThis normative document states rules and cites nothing.\n' > "$norm_victim"
  "$0" >/dev/null 2>&1 && echo "SELF-TEST: arm 5 did not fail on an unanchored normative document" >&2 || ok=$((ok + 1))
  rm -f "$norm_victim"

  # Arm 6: a well-formed normative document passes, so arms 4 and 5 are not passing on a scope that
  # simply always fails.
  arms=$((arms + 1))
  printf '# Self test\n\nEnforced by `crates/eadl-front/src/reader.rs`.\n' > "$norm_victim"
  if "$0" >/dev/null 2>&1; then ok=$((ok + 1)); else
    echo "SELF-TEST: arm 6 failed on a well-formed normative document — the new scope is not discriminating" >&2
  fi
  rm -f "$norm_victim"

  # ⚠️ The empty-normative-population guard has no arm, and the reason is stated rather than left
  # as an untested branch: firing it would mean removing or renaming `docs/semantics/`, which
  # `reference.rs` `include_str!`s — the self-test would have to break the build to prove the guard.
  # It is three lines and reads as a `note` like every other breach.

  echo "book-anchors self-test: $ok pass / $((arms - ok)) fail"
  [ "$ok" -eq "$arms" ] || exit 1
  exit 0
fi

[ "$fail" -eq 0 ] || exit 1
echo "book-anchors: OK ($chapters chapter(s), $normative normative document(s); every cited repository path resolves)"
exit 0
