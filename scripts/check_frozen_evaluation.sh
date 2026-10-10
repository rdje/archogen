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
# ⭐ CUSTODY (leaf PROGRAM.76). A seal that guards only writes leaves the text readable, and
# the cases are written in the very vocabulary the engine work searches for: on `2026-10-10` a
# search for a public fixture id, made for other work, landed in a sealed case. So while the
# set is sealed its text is in NO file of the working tree: each case lives only as the blob
# the manifest's `# sealed-in:` commit holds, and the manifest keeps its digest — a
# commitment, revealed at unsealing by `--restore`. No search of the tree, under any term,
# can reach a case; reading one takes a deliberate `git show` of the sealing commit.
#
# ⚠️ A seal that is only a promise is not a seal. This check makes these mechanical:
#   1. INTEGRITY    — every listed case's blob in the sealing commit hashes to its manifest
#                     digest, and once unsealed the restored file does too, so the set cannot be
#                     quietly rewritten to match what the engine turned out to do.
#   2. CUSTODY      — while sealed, no case is in the working tree or the index.
#   3. COMPLETENESS — no file appears in the sealed directory unlisted; once unsealed, none is
#                     missing.
#   4. NON-CONTAMINATION — no tracked file outside the sealed directory names a sealed case: a
#                     case discussed in a task tree, a decision record or a design note is no
#                     longer unseen.
#   5. EXPOSURE     — each `# exposed:` line names a listed case, so the measurement can count
#                     a case read before its time apart from the unseen ones.
#   It never prints a case's text: a blob is hashed through a pipe, and every arm checks it.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: the text stays in the repository's history —
# `git show <sealed-in>:…`, `git log -p` or a pickaxe search over history reaches it, and
# rewriting a published history is not done. The check proves the tree holds no case, that the
# commitments still resolve, and that no case is named or exposed unrecorded where the
# repository can see; it cannot prove nobody read one. A human who reads one and says nothing
# defeats it — an exposure is recorded, never kept quiet (docs/evaluation/README.md).
#
# UNSEALING. At the leaf named in the manifest's `unseals-at:` line, set `seal: unsealed`,
# add `unsealed-on:` / `unsealed-by:`, then run `--restore`, which writes each case back from
# the sealing commit and verifies its digest. Integrity and completeness keep running;
# custody and contamination lift, because from that point the cases are supposed to be read.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic; read-only, but for
# `--restore`, which writes the cases back and only once the set is unsealed.
# `--self-test` runs the RED arms (leaves `PROGRAM.18.1`, `PROGRAM.76`): each seeds one breach in a
# scratch repository under `target/doctrine_scratch/`, runs this check there, and requires the
# refusal to name the case it is about.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# ── RED arms ─────────────────────────────────────────────────────────────────────────────────
# ⛔ Every case name below is SYNTHETIC. Writing a real sealed slug anywhere tracked is the contamination
# this check exists to catch, and a self-test that did it would fail the gate it belongs to.
self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/frozen_evaluation/selftest" sealed_in=""
  sha() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi; }
  g() { git -C "$work" -c core.hooksPath=/dev/null -c commit.gpgsign=false -c user.name=arm -c user.email=arm@example.invalid "$@"; }
  # A scratch repository whose first commit seals two synthetic cases, then the manifest in one
  # of three states: `sealed` (the cases out of the tree), `unsealed` (restored), `unsealed-out`
  # (the seal lifted, the cases not yet restored).
  fresh() {
    local d="$work/docs/evaluation/frozen"
    rm -rf "$work"; mkdir -p "$d" "$work/docs/notes"
    git -C "$work" init -q
    printf 'ARM-CASE-TEXT alpha\n' > "$d/zz-01-arm-alpha.md"
    printf 'ARM-CASE-TEXT beta\n' > "$d/zz-02-arm-beta.md"
    printf 'nothing about the set here\n' > "$work/docs/notes/plan.md"
    g add -A; g commit -q -m "seal"
    sealed_in="$(g rev-parse HEAD)"
    { printf '# seal: %s\n' "${1%-out}"
      printf '# sealed-in: %s\n' "$sealed_in"
      printf '%s  zz-01-arm-alpha.md\n' "$(sha "$d/zz-01-arm-alpha.md")"
      printf '%s  zz-02-arm-beta.md\n' "$(sha "$d/zz-02-arm-beta.md")"
    } > "$d/MANIFEST.txt"
    case "$1" in sealed|unsealed-out) g rm -q --cached -- "$d/zz-01-arm-alpha.md" "$d/zz-02-arm-beta.md"
                                      rm -f "$d/zz-01-arm-alpha.md" "$d/zz-02-arm-beta.md" ;; esac
    g add -A
  }
  manifest() { printf '%s\n' "$@" >> "$work/docs/evaluation/frozen/MANIFEST.txt"; }
  arm() { # $1 = name, $2 = expected rc, $3 = text a refusal must carry ("" for a pass), $4… = arguments
    local name="$1" want="$2" must="$3" out rc
    shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if printf '%s' "$out" | grep -q 'ARM-CASE-TEXT'; then
      echo "SELF-TEST: $name — the check printed a sealed case's text" >&2; return
    fi
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ "$want" -ne 0 ] && ! printf '%s' "$out" | grep -q 'FROZEN-EVALUATION'; then
      echo "SELF-TEST: $name — exit $rc but the check never spoke" >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`, so it answered for another reason:" >&2
      printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  local d="$work/docs/evaluation/frozen"

  fresh sealed; arm "an intact sealed set passes, its cases out of the tree" 0 ""
  fresh sealed; printf 'ARM-CASE-TEXT alpha\n' > "$d/zz-01-arm-alpha.md"
  arm "a sealed case back in the working tree is refused (custody)" 1 "sealed case 'zz-01-arm-alpha.md' is in the working tree"
  fresh sealed; printf 'ARM-CASE-TEXT alpha\n' > "$d/zz-01-arm-alpha.md"; g add -- "$d/zz-01-arm-alpha.md"; rm "$d/zz-01-arm-alpha.md"
  arm "a sealed case in the index alone is refused (custody)" 1 "'docs/evaluation/frozen/zz-01-arm-alpha.md' is tracked while the set is sealed"
  fresh sealed; sed -i.bak 's/^[0-9a-f]\{64\}  zz-02/0000000000000000000000000000000000000000000000000000000000000000  zz-02/' "$d/MANIFEST.txt"; rm -f "$d/MANIFEST.txt.bak"
  arm "a digest its sealing commit's blob does not match is refused (integrity)" 1 "'zz-02-arm-beta.md' does not match its blob in the sealing commit"
  fresh sealed; manifest "1111111111111111111111111111111111111111111111111111111111111111  zz-03-arm-gamma.md"
  arm "a listed case the sealing commit does not hold is refused (integrity)" 1 "'zz-03-arm-gamma.md' is not in the sealing commit"
  fresh sealed; sed -i.bak "s/^# sealed-in: .*/# sealed-in: $(printf 'a%.0s' $(seq 40))/" "$d/MANIFEST.txt"; rm -f "$d/MANIFEST.txt.bak"
  arm "a sealing commit absent from the clone is refused" 1 "is not in this clone"
  fresh sealed; sed -i.bak '/^# sealed-in:/d' "$d/MANIFEST.txt"; rm -f "$d/MANIFEST.txt.bak"
  arm "a manifest that names no sealing commit is refused" 1 "has no '# sealed-in: <commit>' line"
  fresh sealed; printf 'extra\n' > "$d/zz-03-arm-gamma.md"
  arm "a file added to the set unlisted is refused (completeness)" 1 "unlisted file 'zz-03-arm-gamma.md'"
  fresh sealed; printf 'extra\n' > "$d/.zz-04-arm-delta.md"
  arm "a hidden file in the set is refused (completeness)" 1 "unlisted file '.zz-04-arm-delta.md'"
  fresh sealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/plan.md"; g add -A
  arm "a sealed case named in a tracked file is refused (non-contamination)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/scratch.md"
  arm "a sealed case named only in an untracked file passes — the repository cannot see it" 0 ""
  fresh sealed; manifest "# exposed: zz-02-arm-beta.md — read early, recorded"
  arm "an exposure naming a listed case passes" 0 ""
  fresh sealed; manifest "# exposed: zz-09-arm-iota.md — read early, recorded"
  arm "an exposure naming no listed case is refused (exposure)" 1 "names 'zz-09-arm-iota.md', which the manifest does not list"
  fresh sealed; sed -i.bak '/^# seal:/d' "$d/MANIFEST.txt" && rm -f "$d/MANIFEST.txt.bak"
  arm "a manifest that declares no seal state is refused" 1 "has no '# seal: sealed|unsealed' line"
  fresh sealed; arm "a restore while the set is sealed is refused, nothing written" 1 "the set is sealed" --restore
  arms=$((arms + 1))
  if [ -e "$d/zz-01-arm-alpha.md" ] || [ -e "$d/zz-02-arm-beta.md" ]; then
    echo "SELF-TEST: a refused restore wrote a case" >&2
  else ok=$((ok + 1)); echo "  ✅ a refused restore writes no case"; fi
  fresh unsealed-out; arm "an unsealed set whose cases are not restored is refused (completeness)" 1 "listed case 'zz-01-arm-alpha.md' is missing"
  fresh unsealed-out; arm "a restore once unsealed writes every case back" 0 "restored 2 case(s)" --restore
  arm "the restored set passes" 0 ""
  fresh unsealed-out; printf 'ARM-CASE-TEXT changed\n' > "$d/zz-01-arm-alpha.md"
  arm "a restore leaves a differing file in place and says so" 1 "'zz-01-arm-alpha.md' is in the tree and differs from its sealed text" --restore
  arms=$((arms + 1))
  if grep -q 'ARM-CASE-TEXT changed' "$d/zz-01-arm-alpha.md"; then ok=$((ok + 1)); echo "  ✅ the differing file is kept as it was"
  else echo "SELF-TEST: a restore overwrote a file that differed from its sealed text" >&2; fi
  fresh unsealed-out; sed -i.bak 's/^[0-9a-f]\{64\}  zz-02/0000000000000000000000000000000000000000000000000000000000000000  zz-02/' "$d/MANIFEST.txt"; rm -f "$d/MANIFEST.txt.bak"
  arm "a restore whose blob does not match its digest is refused" 1 "'zz-02-arm-beta.md' does not match its blob in the sealing commit — not restored" --restore
  arms=$((arms + 1))
  if [ -e "$d/zz-02-arm-beta.md" ] || [ -n "$(find "$d" -name '.*.restore.*')" ]; then
    echo "SELF-TEST: a restore wrote a case whose digest it could not verify, or left its part file" >&2
  else ok=$((ok + 1)); echo "  ✅ nothing unverified is written, no part file left"; fi
  fresh unsealed; printf 'ARM-CASE-TEXT rewritten\n' > "$d/zz-02-arm-beta.md"
  arm "a restored case edited is refused (integrity)" 1 "sealed case 'zz-02-arm-beta.md' was modified"
  fresh unsealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/plan.md"; g add -A
  arm "an unsealed set may be discussed" 0 ""
  rm -rf "$work"

  # The real tree, unchanged.
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then
    ok=$((ok + 1)); echo "  ✅ the real sealed set passes"
  else
    echo "SELF-TEST: the real sealed set is refused — run the check to see why" >&2
  fi

  echo "frozen-evaluation self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

DIR="docs/evaluation/frozen"
MANIFEST="$DIR/MANIFEST.txt"
fail=0
note() { printf 'FROZEN-EVALUATION: %s\n' "$1" >&2; fail=1; }

# Portable SHA-256 of standard input: coreutils on Linux, BSD/perl shasum on macOS.
sha256_in() {
  if command -v sha256sum >/dev/null 2>&1; then sha256sum | cut -d' ' -f1
  elif command -v shasum   >/dev/null 2>&1; then shasum -a 256 | cut -d' ' -f1
  else note "no sha256 tool available (sha256sum or shasum)"; return 1
  fi
}

# An absent set is not a breach: a project adopting this spine may have no sealed set yet.
[ -d "$DIR" ] || exit 0
if [ ! -f "$MANIFEST" ]; then
  note "$DIR exists but $MANIFEST does not — a set with no manifest is not sealed"
  exit 1
fi

field() { grep -E "^# $1:" "$MANIFEST" | head -1 | sed "s/^# $1:[[:space:]]*//"; }
seal="$(field seal)"
case "$seal" in
  sealed|unsealed) ;;
  *) note "$MANIFEST has no '# seal: sealed|unsealed' line (found: '${seal:-<none>}')" ;;
esac
sealed_in="$(field sealed-in)"
if ! printf '%s' "$sealed_in" | grep -qE '^[0-9a-f]{40}([0-9a-f]{24})?$'; then
  note "$MANIFEST has no '# sealed-in: <commit>' line naming the commit that holds the cases' text (found: '${sealed_in:-<none>}')"
  sealed_in=""
elif ! git cat-file -e "${sealed_in}^{commit}" 2>/dev/null; then
  note "the sealing commit $sealed_in is not in this clone, so no digest can be verified — fetch the full history (a shallow clone does not hold it)"
  sealed_in=""
fi

# Scratch on this repository's own volume, never `$TMPDIR` (leaf `PROGRAM.29`, `SCRATCH-LOCALITY`).
mkdir -p "$ROOT/target/doctrine_scratch"
tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/frozen_evaluation.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
grep -vE '^[[:space:]]*(#|$)' "$MANIFEST" > "$tmp/entries.txt" || true
awk 'NF >= 2 { print $2 }' "$tmp/entries.txt" > "$tmp/listed.txt"

# ── --restore: the reveal, once unsealed ─────────────────────────────────────────────────────
if [ "${1:-}" = "--restore" ]; then
  if [ "$seal" != "unsealed" ]; then
    note "the set is sealed — its cases stay out of the tree until the leaf its manifest names sets '# seal: unsealed'"
    exit 1
  fi
  [ -n "$sealed_in" ] || exit 1
  n=0
  while read -r want name; do
    [ -n "${name:-}" ] || continue
    path="$DIR/$name"
    if [ -e "$path" ]; then
      [ "$(sha256_in < "$path")" = "$want" ] \
        || note "'$name' is in the tree and differs from its sealed text — left in place, not overwritten"
      continue
    fi
    if ! git cat-file -e "$sealed_in:$path" 2>/dev/null; then
      note "listed case '$name' is not in the sealing commit $sealed_in"; continue
    fi
    part="$DIR/.$name.restore.$$"
    git cat-file blob "$sealed_in:$path" > "$part"
    if [ "$(sha256_in < "$part")" != "$want" ]; then
      rm -f "$part"; note "'$name' does not match its blob in the sealing commit — not restored"; continue
    fi
    mv "$part" "$path" && n=$((n + 1))
  done < "$tmp/entries.txt"
  echo "frozen-evaluation: restored $n case(s) from ${sealed_in:0:12}, each matching its digest"
  exit $fail
fi

# ── 1. INTEGRITY + 2. CUSTODY + 3. COMPLETENESS ──────────────────────────────────────────────
while read -r want name; do
  [ -n "${name:-}" ] || continue
  path="$DIR/$name"
  if [ -n "$sealed_in" ]; then
    if ! git cat-file -e "$sealed_in:$path" 2>/dev/null; then
      note "listed case '$name' is not in the sealing commit $sealed_in"
    elif [ "$(git cat-file blob "$sealed_in:$path" | sha256_in)" != "$want" ]; then
      note "listed case '$name' does not match its blob in the sealing commit — the digest or the commit is not the one sealed"
    fi
  fi
  if [ "$seal" = "sealed" ]; then
    [ -e "$path" ] && note "sealed case '$name' is in the working tree while the set is sealed — its text stays in the sealing commit until unsealing"
  elif [ "$seal" = "unsealed" ]; then
    if [ ! -f "$path" ]; then
      note "listed case '$name' is missing from $DIR — once unsealed, \`bash scripts/check_frozen_evaluation.sh --restore\` writes it back"
    elif [ "$(sha256_in < "$path")" != "$want" ]; then
      note "sealed case '$name' was modified after sealing"
    fi
  fi
done < "$tmp/entries.txt"

if [ "$seal" = "sealed" ]; then
  while IFS= read -r tracked; do
    [ "$tracked" = "$MANIFEST" ] || note "'$tracked' is tracked while the set is sealed — a case's text stays in the sealing commit"
  done < <(git ls-files -- "$DIR")
fi

# Any file in the directory that the manifest does not list, hidden ones included.
for path in "$DIR"/* "$DIR"/.[!.]*; do
  [ -f "$path" ] || continue
  name="$(basename "$path")"
  [ "$name" = "MANIFEST.txt" ] && continue
  grep -qxF "$name" "$tmp/listed.txt" || note "unlisted file '$name' in $DIR — seal the set or remove it"
done

# ── 5. EXPOSURE ──────────────────────────────────────────────────────────────────────────────
while IFS= read -r exposed; do
  grep -qxF "$exposed" "$tmp/listed.txt" \
    || note "an '# exposed:' line names '$exposed', which the manifest does not list"
done < <(grep -E '^# exposed:' "$MANIFEST" | sed 's/^# exposed:[[:space:]]*//' | cut -d' ' -f1)

# ── 4. NON-CONTAMINATION (only while sealed) ─────────────────────────────────────────────────
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
