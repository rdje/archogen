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
# ⭐ CUSTODY (leaf PROGRAM.76). A seal that guards only writes leaves the text readable, and the
# cases are written in the very vocabulary the engine work searches: on `2026-10-10` a search
# made for other work landed in a sealed case. So while the set is sealed its text is in no
# file of the working tree: each case lives only as the blob the manifest's `# sealed-in:`
# commit holds, the manifest keeps its digest — a commitment, revealed at unsealing by
# `--restore` — and `.gitattributes` marks the sealed paths `-diff`, so a diff, a log patch or
# a `git grep` of a commit that holds them prints no line of them.
#
# ⚠️ A seal that is only a promise is not a seal. This check makes these mechanical:
#   1. INTEGRITY    — the manifest's entries are the sealing commit's own, that commit an ancestor
#                     of HEAD; every case's blob there hashes to its digest, and once unsealed the
#                     restored file does too: the set cannot be rewritten, grown or shrunk.
#   2. CUSTODY      — while sealed, no case is at its path, in the index, or — its whole text —
#                     anywhere in the index or the working tree outside ignored files; no tracked
#                     or untracked file quotes a long line of one; its path is marked `-diff`.
#   3. COMPLETENESS — nothing is in the sealed directory but the manifest and, once unsealed, the
#                     listed cases, at any depth, hidden or linked; once unsealed, none is missing.
#   4. NON-CONTAMINATION — no tracked file outside the sealed directory names a sealed case: a
#                     case discussed in a task tree, a decision record or a design note is no
#                     longer unseen.
#   5. EXPOSURE     — each `# exposed:` line names a listed case. Recording an exposure is the
#                     project's rule (docs/evaluation/README.md); the check cannot see a read.
#   It never prints a case's text: a blob is hashed through a pipe, a quote is found by name of
#   the file that holds it, and every arm checks that no output carries a case's text.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: the text stays in the published history, which is
# not rewritten. `git show <commit>:<path>`, a diff asked for with `--text`, and a checkout of any
# commit from the sealing one up to the one that took the cases out (`2f6f331`'s parent) put it
# in front of a reader; so does a working tree of another clone not yet past `2f6f331`. Ignored
# files — build output under `target/` — are not scanned. A quote shorter than a long line, or
# reworded, is not found. The check cannot prove nobody read a case; a human who reads one and
# says nothing defeats it.
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
  local LONG="ARM-CASE-TEXT alpha asks one comparator to keep two requesters each on its own deadline"
  sha() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | cut -d' ' -f1; else shasum -a 256 "$1" | cut -d' ' -f1; fi; }
  g() { git -C "$work" -c core.hooksPath=/dev/null -c commit.gpgsign=false -c user.name=arm -c user.email=arm@example.invalid "$@"; }
  local d="$work/docs/evaluation/frozen"
  entries() {
    printf '%s  zz-01-arm-alpha.md\n' "$(sha "$d/zz-01-arm-alpha.md")"
    printf '%s  zz-02-arm-beta.md\n' "$(sha "$d/zz-02-arm-beta.md")"
  }
  # A scratch repository whose first commit seals two synthetic cases with their manifest and the
  # attribute, then the manifest in one of three states: `sealed` (the cases out of the tree),
  # `unsealed` (restored), `unsealed-out` (the seal lifted, the cases not yet restored).
  fresh() {
    rm -rf "$work"; mkdir -p "$d" "$work/docs/notes"
    git -C "$work" init -q
    printf 'ARM-CASE-TEXT alpha\n%s\n' "$LONG" > "$d/zz-01-arm-alpha.md"
    printf 'ARM-CASE-TEXT beta\n' > "$d/zz-02-arm-beta.md"
    printf 'docs/evaluation/frozen/** -diff\n' > "$work/.gitattributes"
    printf 'nothing about the set here\n' > "$work/docs/notes/plan.md"
    { printf '# seal: sealed\n'; entries; } > "$d/MANIFEST.txt"
    g add -A; g commit -q -m "seal"
    sealed_in="$(g rev-parse HEAD)"
    { printf '# seal: %s\n' "${1%-out}"; printf '# sealed-in: %s\n' "$sealed_in"; entries; } > "$d/MANIFEST.txt"
    case "$1" in sealed|unsealed-out) g rm -q --cached -- "$d/zz-01-arm-alpha.md" "$d/zz-02-arm-beta.md"
                                      rm -f "$d/zz-01-arm-alpha.md" "$d/zz-02-arm-beta.md" ;; esac
    g add -A
  }
  manifest() { printf '%s\n' "$@" >> "$d/MANIFEST.txt"; }
  edit() { sed "$1" "$d/MANIFEST.txt" > "$d/MANIFEST.new" && mv "$d/MANIFEST.new" "$d/MANIFEST.txt"; }
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
  holds() { # $1 = name, then a test that must hold after the arm before it
    local name="$1"; shift
    arms=$((arms + 1))
    if "$@"; then ok=$((ok + 1)); echo "  ✅ $name"; else echo "SELF-TEST: $name — it does not hold" >&2; fi
  }
  absent() { [ ! -e "$d/zz-01-arm-alpha.md" ] && [ ! -e "$d/zz-02-arm-beta.md" ] && [ -z "$(find "$d" -name '.*.restore.*')" ]; }

  fresh sealed; arm "an intact sealed set passes, its cases out of the tree" 0 ""
  # Custody.
  fresh sealed; printf 'ARM-CASE-TEXT alpha\n' > "$d/zz-01-arm-alpha.md"
  arm "a sealed case back at its path is refused (custody)" 1 "sealed case 'zz-01-arm-alpha.md' is in the working tree"
  fresh sealed; printf 'ARM-CASE-TEXT alpha\n' > "$d/zz-01-arm-alpha.md"; g add -- "$d/zz-01-arm-alpha.md"; rm "$d/zz-01-arm-alpha.md"
  arm "a sealed case in the index alone is refused (custody)" 1 "'docs/evaluation/frozen/zz-01-arm-alpha.md' is tracked while the set is sealed"
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/copy.md"; g add -A
  arm "a case's whole text tracked elsewhere is refused (custody)" 1 "a sealed case's text is tracked at 'docs/notes/copy.md'"
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/loose.md"
  arm "a case's whole text untracked elsewhere is refused (custody)" 1 "a sealed case's text is in the working tree at 'docs/notes/loose.md'"
  fresh sealed; printf '%s\n' "$LONG" >> "$work/docs/notes/plan.md"; g add -A
  arm "a long line of a case quoted in a tracked file is refused (custody)" 1 "'docs/notes/plan.md' quotes a line of a sealed case"
  fresh sealed; printf 'see: %s\n' "$LONG" > "$work/docs/notes/draft.md"
  arm "a long line of a case quoted in an untracked file is refused (custody)" 1 "'docs/notes/draft.md' quotes a line of a sealed case"
  fresh sealed; rm "$work/.gitattributes"; g add -A
  arm "a sealed path not marked -diff is refused (custody)" 1 "'zz-01-arm-alpha.md' is not marked -diff"
  # Integrity.
  fresh sealed; edit 's/^[0-9a-f]\{64\}  zz-02/0000000000000000000000000000000000000000000000000000000000000000  zz-02/'
  arm "a digest its sealing commit's blob does not match is refused (integrity)" 1 "'zz-02-arm-beta.md' does not match its blob in the sealing commit"
  fresh sealed; manifest "1111111111111111111111111111111111111111111111111111111111111111  zz-03-arm-gamma.md"
  arm "a listed case the sealing commit does not hold is refused (integrity)" 1 "'zz-03-arm-gamma.md' is not in the sealing commit"
  fresh sealed; edit '/  zz-02-arm-beta.md$/d'
  arm "a case dropped from the manifest is refused (integrity)" 1 "the manifest's cases are not the sealing commit's"
  fresh sealed; edit "s/^# sealed-in: .*/# sealed-in: $(printf 'a%.0s' $(seq 40))/"
  arm "a sealing commit absent from the clone is refused" 1 "is not in this clone"
  fresh sealed
  printf 'ARM-CASE-TEXT forged\n' > "$d/zz-01-arm-alpha.md"; g add -- "$d/zz-01-arm-alpha.md"
  orphan="$(g commit-tree "$(g write-tree)" -m orphan)"; g rm -q --cached -- "$d/zz-01-arm-alpha.md"; rm -f "$d/zz-01-arm-alpha.md"
  edit "s/^# sealed-in: .*/# sealed-in: $orphan/"
  arm "a sealing commit outside HEAD's history is refused (integrity)" 1 "is not an ancestor of HEAD"
  fresh sealed; edit '/^# sealed-in:/d'
  arm "a manifest that names no sealing commit is refused" 1 "has no '# sealed-in: <commit>' line"
  fresh sealed; manifest "not a digest line"
  arm "a manifest line that is no entry is refused" 1 "a manifest line that is not '<sha256>  <file>'"
  # Completeness.
  fresh sealed; printf 'extra\n' > "$d/zz-03-arm-gamma.md"
  arm "a file added to the set unlisted is refused (completeness)" 1 "unlisted entry 'zz-03-arm-gamma.md'"
  fresh sealed; printf 'extra\n' > "$d/.zz-04-arm-delta.md"
  arm "a hidden file in the set is refused (completeness)" 1 "unlisted entry '.zz-04-arm-delta.md'"
  fresh sealed; printf 'extra\n' > "$d/..zz-05-arm-epsilon.md"
  arm "a file named with two leading dots is refused (completeness)" 1 "unlisted entry '..zz-05-arm-epsilon.md'"
  fresh sealed; mkdir "$d/sub"; printf 'extra\n' > "$d/sub/zz-06-arm-zeta.md"
  arm "a file in a folder of the set is refused (completeness)" 1 "unlisted entry 'sub/zz-06-arm-zeta.md'"
  fresh sealed; ln -s ../../notes "$d/linked"
  arm "a linked folder in the set is refused (completeness)" 1 "unlisted entry 'linked'"
  # Non-contamination and exposure.
  fresh sealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/plan.md"; g add -A
  arm "a sealed case named in a tracked file is refused (non-contamination)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/scratch.md"
  arm "a sealed case named only in an untracked file passes — the repository cannot see it" 0 ""
  fresh sealed; manifest "# exposed: zz-02-arm-beta.md — read early, recorded"
  arm "an exposure naming a listed case passes" 0 ""
  fresh sealed; manifest "# exposed: zz-09-arm-iota.md — read early, recorded"
  arm "an exposure naming no listed case is refused (exposure)" 1 "names 'zz-09-arm-iota.md', which the manifest does not list"
  fresh sealed; edit '/^# seal:/d'
  arm "a manifest that declares no seal state is refused" 1 "has no '# seal: sealed|unsealed' line"
  # The reveal.
  fresh sealed; arm "a restore while the set is sealed is refused" 1 "the set is sealed" --restore
  holds "a refused restore writes no case" absent
  fresh unsealed-out
  g update-index --add --cacheinfo "100644,$(g rev-parse "$sealed_in:docs/evaluation/frozen/zz-01-arm-alpha.md"),docs/evaluation/frozen/zz-01-arm-alpha.md"
  edit "s/^# sealed-in: .*/# sealed-in: $(printf 'a%.0s' $(seq 40))/"  # the index holds a case: never a source
  arm "a restore with no sealing commit to read is refused" 1 "is not in this clone" --restore
  holds "a restore with no sealing commit writes no case" absent
  fresh unsealed-out; arm "an unsealed set whose cases are not restored is refused (completeness)" 1 "listed case 'zz-01-arm-alpha.md' is missing"
  fresh unsealed-out; arm "a restore once unsealed writes every case back" 0 "restored 2 case(s)" --restore
  arm "the restored set passes" 0 ""
  fresh unsealed-out; printf 'ARM-CASE-TEXT changed\n' > "$d/zz-01-arm-alpha.md"
  arm "a restore leaves a differing file in place and says so" 1 "'zz-01-arm-alpha.md' is in the tree and differs from its sealed text" --restore
  holds "the differing file is kept as it was" grep -q 'ARM-CASE-TEXT changed' "$d/zz-01-arm-alpha.md"
  fresh unsealed-out; edit 's/^[0-9a-f]\{64\}  zz-02/0000000000000000000000000000000000000000000000000000000000000000  zz-02/'
  arm "a restore whose blob does not match its digest is refused" 1 "'zz-02-arm-beta.md' does not match its blob in the sealing commit — not restored" --restore
  holds "nothing unverified is written, no part file left" test ! -e "$d/zz-02-arm-beta.md" -a -z "$(find "$d" -name '.*.restore.*')"
  fresh unsealed; printf 'ARM-CASE-TEXT rewritten\n' > "$d/zz-02-arm-beta.md"
  arm "a restored case edited is refused (integrity)" 1 "sealed case 'zz-02-arm-beta.md' was modified"
  fresh unsealed; printf 'ARM-CASE-TEXT rewritten\n' > "$d/zz-02-arm-beta.md"
  edit "s/^[0-9a-f]\{64\}  zz-02/$(sha "$d/zz-02-arm-beta.md")  zz-02/"
  arm "once unsealed, a case and its digest rewritten together are refused (integrity)" 1 "'zz-02-arm-beta.md' does not match its blob in the sealing commit"
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
ENTRY='^[0-9a-f]{64}  [^/[:space:]]+$'
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

# Scratch on this repository's own volume, never `$TMPDIR` (leaf `PROGRAM.29`, `SCRATCH-LOCALITY`). It holds
# names and digests only, never a case's text.
mkdir -p "$ROOT/target/doctrine_scratch"
tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/frozen_evaluation.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
grep -vE '^[[:space:]]*(#|$)' "$MANIFEST" > "$tmp/lines.txt" || true
while IFS= read -r line; do
  printf '%s\n' "$line" | grep -qE "$ENTRY" || note "a manifest line that is not '<sha256>  <file>': '${line:0:72}'"
done < "$tmp/lines.txt"
grep -E "$ENTRY" "$tmp/lines.txt" > "$tmp/entries.txt" || true
awk '{ print $2 }' "$tmp/entries.txt" > "$tmp/listed.txt"

sealed_in="$(field sealed-in)"
if ! printf '%s' "$sealed_in" | grep -qE '^[0-9a-f]{40}([0-9a-f]{24})?$'; then
  note "$MANIFEST has no '# sealed-in: <commit>' line naming the commit that holds the cases' text (found: '${sealed_in:-<none>}')"
  sealed_in=""
elif ! git cat-file -e "${sealed_in}^{commit}" 2>/dev/null; then
  note "the sealing commit $sealed_in is not in this clone, so no digest can be verified — fetch the full history (a shallow clone does not hold it)"
  sealed_in=""
elif ! git merge-base --is-ancestor "$sealed_in" HEAD 2>/dev/null; then
  note "the sealing commit $sealed_in is not an ancestor of HEAD — a set is sealed in this history or not at all"
  sealed_in=""
elif ! git cat-file -e "$sealed_in:$MANIFEST" 2>/dev/null; then
  note "the sealing commit $sealed_in holds no $MANIFEST to pin the set's entries"
  sealed_in=""
else
  # The set is the one sealed: the same entries, no case added, dropped or re-digested.
  git cat-file blob "$sealed_in:$MANIFEST" | grep -E "$ENTRY" | sort > "$tmp/sealed-entries.txt"
  sort "$tmp/entries.txt" > "$tmp/now-entries.txt"
  if ! cmp -s "$tmp/sealed-entries.txt" "$tmp/now-entries.txt"; then
    note "the manifest's cases are not the sealing commit's — differing: $(comm -3 "$tmp/sealed-entries.txt" "$tmp/now-entries.txt" | awk '{ print $2 }' | sort -u | tr '\n' ' ')"
  fi
fi

# ── --restore: the reveal, once unsealed ─────────────────────────────────────────────────────
if [ "${1:-}" = "--restore" ]; then
  if [ "$seal" != "unsealed" ]; then
    note "the set is sealed — its cases stay out of the tree until the leaf its manifest names sets '# seal: unsealed'"
    exit 1
  fi
  [ -n "$sealed_in" ] || exit 1
  n=0
  while read -r want name; do
    path="$DIR/$name"
    if [ -e "$path" ] || [ -L "$path" ]; then
      [ -f "$path" ] && [ ! -L "$path" ] && [ "$(sha256_in < "$path")" = "$want" ] \
        || note "'$name' is in the tree and differs from its sealed text — left in place, not overwritten"
      continue
    fi
    if ! git cat-file -e "$sealed_in:$path" 2>/dev/null; then
      note "listed case '$name' is not in the sealing commit $sealed_in, or this clone cannot fetch it"; continue
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

# ── 1. INTEGRITY, and each case's blob ───────────────────────────────────────────────────────
: > "$tmp/blobs.txt"
while read -r want name; do
  path="$DIR/$name"
  if [ -n "$sealed_in" ]; then
    if ! git cat-file -e "$sealed_in:$path" 2>/dev/null; then
      note "listed case '$name' is not in the sealing commit $sealed_in, or this clone cannot fetch it"
    elif [ "$(git cat-file blob "$sealed_in:$path" | sha256_in)" != "$want" ]; then
      note "listed case '$name' does not match its blob in the sealing commit — the digest or the commit is not the one sealed"
    else
      git rev-parse "$sealed_in:$path" >> "$tmp/blobs.txt"
    fi
  fi
  if [ "$seal" = "unsealed" ]; then
    if [ ! -e "$path" ] && [ ! -L "$path" ]; then
      note "listed case '$name' is missing from $DIR — once unsealed, \`bash scripts/check_frozen_evaluation.sh --restore\` writes it back"
    elif [ -L "$path" ] || [ ! -f "$path" ]; then
      note "listed case '$name' in $DIR is a link or no plain file"
    elif [ "$(sha256_in < "$path")" != "$want" ]; then
      note "sealed case '$name' was modified after sealing"
    fi
  fi
done < "$tmp/entries.txt"

# ── 2. CUSTODY (only while sealed) ───────────────────────────────────────────────────────────
if [ "$seal" = "sealed" ]; then
  while read -r want name; do
    path="$DIR/$name"
    { [ -e "$path" ] || [ -L "$path" ]; } && note "sealed case '$name' is in the working tree while the set is sealed — its text stays in the sealing commit until unsealing"
    [ "$(git check-attr diff -- "$path" | sed 's/.*: diff: //')" = "unset" ] \
      || note "'$name' is not marked -diff in .gitattributes — a diff or a log patch of a commit that holds the set would print its text"
  done < "$tmp/entries.txt"
  while IFS= read -r tracked; do
    [ "$tracked" = "$MANIFEST" ] || note "'$tracked' is tracked while the set is sealed — a case's text stays in the sealing commit"
  done < <(git ls-files -- "$DIR")
  if [ -s "$tmp/blobs.txt" ]; then
    # A case's whole text at any other path: in the index, by its blob id; in the working tree, untracked or
    # modified, by the blob id of what is there. Ignored files are not scanned.
    git ls-files -s | awk 'NR == FNR { b[$1]; next } ($2 in b) { sub(/^[^\t]*\t/, ""); print }' "$tmp/blobs.txt" - \
      | while IFS= read -r at; do printf "FROZEN-EVALUATION: a sealed case's text is tracked at '%s'\n" "$at" >&2; done
    git ls-files -s | awk 'NR == FNR { b[$1]; next } ($2 in b) { found = 1 } END { exit !found }' "$tmp/blobs.txt" - && fail=1
    { git ls-files --others --exclude-standard -- . ":(exclude)$DIR"; git ls-files -m -- . ":(exclude)$DIR"; } | sort -u |
      while IFS= read -r at; do [ -f "$at" ] && printf '%s\n' "$at"; done > "$tmp/loose.txt"
    if [ -s "$tmp/loose.txt" ]; then
      git hash-object --stdin-paths < "$tmp/loose.txt" > "$tmp/loose-ids.txt" 2>/dev/null || : > "$tmp/loose-ids.txt"
      while IFS= read -r at && IFS= read -r id <&3; do
        grep -qxF "$id" "$tmp/blobs.txt" && note "a sealed case's text is in the working tree at '$at'"
      done < "$tmp/loose.txt" 3< "$tmp/loose-ids.txt"
    fi
    # A long line of a case quoted anywhere outside the set, tracked or untracked: found by the name of the file
    # that holds it, the patterns read from the blobs through a pipe, never written down.
    quotes() { while IFS= read -r id; do git cat-file blob "$id"; done < "$tmp/blobs.txt" |
                 awk '{ gsub(/^[[:space:]]+|[[:space:]]+$/, "") } length($0) >= 60 && NF >= 8'; }
    if [ -n "$(quotes | head -1)" ]; then
      while IFS= read -r at; do
        note "'$at' quotes a line of a sealed case — the set is no longer unseen"
      done < <(git grep -l --untracked -F -f <(quotes) -- . ":(exclude)$DIR" ":(exclude)scripts/check_frozen_evaluation.sh" 2>/dev/null)
    fi
  fi
fi

# ── 3. COMPLETENESS: nothing in the set but the manifest and, once unsealed, its cases ───────
while IFS= read -r entry; do
  rel="${entry#"$DIR"/}"
  [ "$rel" = "MANIFEST.txt" ] && continue
  if [ "$seal" = "unsealed" ] && grep -qxF "$rel" "$tmp/listed.txt"; then continue; fi
  if [ "$seal" = "sealed" ] && grep -qxF "$rel" "$tmp/listed.txt"; then continue; fi  # custody's to name
  note "unlisted entry '$rel' in $DIR — seal the set or remove it"
done < <(find "$DIR" -mindepth 1 2>/dev/null | sort)

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
