#!/usr/bin/env bash
# FROZEN-EVALUATION — the sealed evaluation set stays sealed, and stays unseen.
#
# ⭐ WHY THIS EXISTS. ROADMAP.md §12 M0 says "freeze a separate small set of previously unused
# evaluation cases before measuring reuse", and §16 says "evaluate at least three previously
# unused configurations after a catalog freeze". The word carrying the weight is UNUSED. A
# reuse measurement taken on cases the engine was tuned against measures nothing, and the
# tuning need not be deliberate — it is enough that a case's requirements were in front of
# someone while they chose what the engine should support.
#
# ⚠️ A seal that is only a promise is not a seal. This check makes three things mechanical:
#   1. INTEGRITY  — every sealed file still hashes to what it hashed to at seal time, so the
#                   set cannot be quietly rewritten to match what the engine turned out to do.
#   2. COMPLETENESS — no file appears in or disappears from the sealed directory unlisted.
#   3. NON-CONTAMINATION — no tracked file outside the sealed directory names a sealed case.
#                   That is the leg that actually protects the measurement: a case discussed in
#                   a task tree, a decision record or a design note is no longer unseen.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: this cannot prove nobody READ the cases. It
# proves they were not edited, not silently added to or removed from, and were not written
# about anywhere the repository can see. A human who reads them and says nothing defeats it.
# The check raises the cost of accidental contamination, which is the common case; it is not a
# defence against a determined author.
#
# UNSEALING. At the leaf named in the manifest's `unseals-at:` line, set `seal: unsealed` and
# add `unsealed-on:` / `unsealed-by:`. Integrity and completeness keep running; contamination
# is lifted, because from that point the cases are supposed to be discussed.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic; read-only.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

DIR="docs/evaluation/frozen"
MANIFEST="$DIR/MANIFEST.txt"
fail=0
note() { printf 'FROZEN-EVALUATION: %s\n' "$1" >&2; fail=1; }

# Portable SHA-256: coreutils on Linux, BSD/perl shasum on macOS.
sha256_of() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1
  elif command -v shasum   >/dev/null 2>&1; then shasum -a 256 "$1" | cut -d' ' -f1
  else note "no sha256 tool available (sha256sum or shasum)"; return 1
  fi
}

# An absent set is not a breach: a project adopting this spine may have no sealed set yet.
[ -d "$DIR" ] || exit 0
if [ ! -f "$MANIFEST" ]; then
  note "$DIR exists but $MANIFEST does not — a set with no manifest is not sealed"
  exit 1
fi

seal="$(grep -E '^# seal:' "$MANIFEST" | head -1 | sed 's/^# seal:[[:space:]]*//')"
case "$seal" in
  sealed|unsealed) ;;
  *) note "$MANIFEST has no '# seal: sealed|unsealed' line (found: '${seal:-<none>}')" ;;
esac

tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

# ── 1. INTEGRITY + 2. COMPLETENESS ───────────────────────────────────────────────────────────
grep -vE '^[[:space:]]*(#|$)' "$MANIFEST" > "$tmp/entries.txt" || true
: > "$tmp/listed.txt"
while read -r want name; do
  [ -n "${name:-}" ] || continue
  echo "$name" >> "$tmp/listed.txt"
  path="$DIR/$name"
  if [ ! -f "$path" ]; then
    note "listed case '$name' is missing from $DIR"
    continue
  fi
  got="$(sha256_of "$path")" || continue
  if [ "$got" != "$want" ]; then
    note "sealed case '$name' was modified after sealing"
    printf '  expected %s\n  actual   %s\n' "$want" "$got" >&2
  fi
done < "$tmp/entries.txt"

# Any file in the directory that the manifest does not list.
for path in "$DIR"/*; do
  [ -f "$path" ] || continue
  name="$(basename "$path")"
  [ "$name" = "MANIFEST.txt" ] && continue
  grep -qxF "$name" "$tmp/listed.txt" || note "unlisted file '$name' in $DIR — seal the set or remove it"
done

# ── 3. NON-CONTAMINATION (only while sealed) ─────────────────────────────────────────────────
if [ "$seal" = "sealed" ] && [ -s "$tmp/listed.txt" ]; then
  # Case slugs are the manifest's file names without their extension. Deriving them here — not
  # listing them — keeps this script itself from becoming the leak it exists to prevent.
  sed 's/\.[^.]*$//' "$tmp/listed.txt" | grep -vE '^\s*$' > "$tmp/slugs.txt"

  # Search tracked file CONTENT outside the sealed directory.
  #
  # `git grep` rather than `grep $(git ls-files)`: the command-substitution form word-splits
  # on whitespace (a tracked path containing a space becomes two wrong paths) and puts the
  # whole file list on one command line, which is an E2BIG waiting for the repository to grow.
  # git grep takes pathspecs, searches tracked files itself, and has neither failure mode.
  #
  # Paths are not searched, only content: the manifest legitimately holds every name. This
  # script is excluded explicitly rather than by construction, so that a future edit quoting
  # a slug as an example cannot silently switch the check off.
  while IFS= read -r slug; do
    [ -n "$slug" ] || continue
    hits="$(git grep -l --fixed-strings -- "$slug" \
              -- . ":(exclude)$DIR" ":(exclude)scripts/check_frozen_evaluation.sh" \
            2>/dev/null || true)"
    if [ -n "$hits" ]; then
      note "sealed case '$slug' is named outside $DIR — the set is no longer unseen"
      printf '%s\n' "$hits" | sed 's/^/    /' >&2
    fi
  done < "$tmp/slugs.txt"
fi

exit $fail
