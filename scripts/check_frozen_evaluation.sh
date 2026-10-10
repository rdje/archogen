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
# `--restore` — and `.gitattributes` marks the sealed paths `-diff`, so in a working tree that
# carries the mark (`6d61f65` on) a diff, a log patch or a `git grep` of a commit that holds them
# prints no line of them.
#
# ⚠️ A seal that is only a promise is not a seal. This check makes these mechanical:
#   1. INTEGRITY    — the manifest's entries are the sealing commit's own, that commit an ancestor
#                     of HEAD; every case's blob there hashes to its digest, and once unsealed the
#                     restored file does too: the set cannot be rewritten, grown or shrunk.
#   2. CUSTODY      — while sealed, no case is at its path, in the index, or — its whole text —
#                     anywhere in the index or the working tree outside ignored files; no file,
#                     the manifest included, staged or on disk, quotes a long line of one; no
#                     unregistered repository nested outside ignored folders, staged or not, nothing
#                     git cannot read or list;
#                     its path is marked `-diff`, staged and on disk.
#   3. COMPLETENESS — nothing is in the sealed directory but the manifest and, once unsealed, the
#                     listed cases, at any depth, hidden or linked; once unsealed, none is missing.
#   4. NON-CONTAMINATION — no tracked file outside the sealed directory names a sealed case: a
#                     case discussed in a task tree, a decision record or a design note is no
#                     longer unseen.
#   5. EXPOSURE     — each `# exposed:` line names a listed case. Recording an exposure is the
#                     project's rule (docs/evaluation/README.md); the check cannot see a read.
#   It prints paths, as their files are named, and line numbers, entry names and a commit id of the
#   right form — tokens with no space — and git's warnings while listing untracked files, which name
#   paths; never a manifest line's text nor a case's line: a field's value is checked inside its
#   pipeline before any variable holds it, a bad line is named by its number, a blob is hashed
#   through a pipe, a quote is found by the name of the file that holds it, and the scratch holds
#   no manifest line of another form, beside git's warnings; `--restore` prints the count it restored;
#   every arm that runs the check on synthetic cases requires no
#   case's text in its output, one in nine places with tracing forced on.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: the text stays in the published history, which is not
# rewritten. Two kinds of act put a case in front of a reader: one that writes to disk a commit
# older than `6d61f65` whose history holds the set — a checkout, a restore or a reset to it, an
# archive of it, a clone or a worktree at it, a revert of `2f6f331` — or finds it there, in another
# clone not yet past it; and one that hands a sealed blob to a reader or a tool — `git show` or `git
# blame` of it, a diff or a grep forced to text (`-a`), an external diff driver or `git difftool`, a
# viewer that diffs blobs itself (an editor's or a web history view), git run on the repository
# without this working tree; `git grep <commit>` and `git log -S` tell which file or commit holds a
# term. Ignored files — build output under `target/`, a folder an untracked `.gitignore` ignores —
# are not scanned, nor anything outside the repository a link points to, nor a registered submodule,
# another repository (REPOSITORY-BOUNDARY refuses what is created at a vendored checkout's first
# level). A quote shorter than a long line, or reworded, is not found. It sees what is staged and on
# disk. The check cannot prove nobody read a case; a human who reads one and says nothing defeats
# it. The published default branch has been past `6d61f65` since `a543d10`, pushed `2026-10-10`; a
# copy made of it before then — a fork, a mirror, a search engine's cache, an archive — may still
# show a case with no act at all, so a web search or fetch of this repository stays fenced. A
# harness transcript, a saved terminal scrollback or an editor's history made before `6d61f65`, or
# in a working tree without the mark, holds whatever was shown then; what to do with them is the
# director's.
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
set +x  # never traced: a traced command could carry a case's line (review R2 D3)
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# ── RED arms ─────────────────────────────────────────────────────────────────────────────────
# ⛔ Every case name below is SYNTHETIC. Writing a real sealed slug anywhere tracked is the contamination
# this check exists to catch, and a self-test that did it would fail the gate it belongs to.
self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/frozen_evaluation/selftest" sealed_in=""
  # git quotes a non-ASCII path unless told not to: forced on here, so the path arms hold on any machine (R7 R3).
  export GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=core.quotePath GIT_CONFIG_VALUE_0=true
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
    out="$(cd "$work" && bash ${TRACE:+-x} "$SELF" "$@" 2>&1)"; rc=$?
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
  fresh sealed; : > "$work/.gitattributes"; g add -A; printf 'docs/evaluation/frozen/** -diff\n' > "$work/.gitattributes"
  arm "a -diff mark dropped from the index alone is refused (custody)" 1 "'zz-01-arm-alpha.md' is not marked -diff"
  fresh sealed; printf '# note: %s\n' "$LONG" >> "$d/MANIFEST.txt"
  arm "a case's line quoted in a comment of the manifest is refused (custody)" 1 "MANIFEST.txt' quotes a line of a sealed case"
  fresh sealed; printf '%s\n' "$LONG" >> "$work/docs/notes/plan.md"
  TRACE=1 arm "a run traced with bash -x prints no case's line" 1 "'docs/notes/plan.md' quotes a line of a sealed case"
  # Two guards against a trace, each held on its own (reviews R2 D3, R3 AG1, AG2, R4 D2): xtrace off before any of the
  # check's code, and no case's line, nor any manifest value of the wrong form, ever a command's argument — a copy with
  # tracing forced on prints none, with a case's line quoted in a tracked file and in each of the manifest's places.
  holds "the check turns tracing off before any of its code runs" test \
    "$(grep -n '^set +x' "$SELF" | head -1 | cut -d: -f1)" -lt "$(grep -n '^self_test() {' "$SELF" | cut -d: -f1)"
  arms=$((arms + 1)); leaked=""
  for place in quote seal sealed-in exposed custody bare comment first name; do
    fresh sealed
    case "$place" in
      quote) printf '%s\n' "$LONG" >> "$work/docs/notes/plan.md"; g add -A ;;
      seal|sealed-in) edit "s/^# $place: .*/# $place: $LONG/" ;;
      exposed|custody) manifest "# $place: $LONG" ;;
      bare) manifest "$LONG" ;;
      comment) manifest "# $LONG" ;;
      first) { printf '%s\n' "$LONG"; cat "$d/MANIFEST.txt"; } > "$d/MANIFEST.new"; mv "$d/MANIFEST.new" "$d/MANIFEST.txt" ;;
      name) manifest "0000000000000000000000000000000000000000000000000000000000000000  $LONG" ;;
    esac
    sed '/^set +x/d' "$SELF" > "$work/../traced.sh"
    normal="$(cd "$work" && bash -x "$work/../traced.sh" 2>&1)"
    out="$normal$(cd "$work" && bash -x "$work/../traced.sh" --restore 2>&1)"
    printf '%s' "$out" | grep -q 'ARM-CASE-TEXT' && leaked="$leaked $place"
    case "$place" in  # each place's own refusal, in the normal run alone (review R6 D2)
      quote) want="quotes a line of a sealed case" ;;
      seal) want="has no '# seal: sealed|unsealed' line" ;;
      sealed-in) want="has no '# sealed-in: <commit>' line" ;;
      exposed) want="'# exposed:' line" ;;
      custody|comment) want="MANIFEST.txt' quotes a line of a sealed case" ;;
      *) want="is not '<sha256>  <file>'" ;;
    esac
    printf '%s' "$normal" | grep -qF -- "$want" || leaked="$leaked $place(no-refusal)"
    rm -f "$work/../traced.sh"
  done
  if [ -z "$leaked" ]; then ok=$((ok + 1)); echo "  ✅ a run with tracing forced on prints no case's line (9 places)"
  else echo "SELF-TEST: a traced run printed a case's line, from:$leaked" >&2; fi
  # A case's line in any field of the manifest, or as its name, is never printed: a value read there is never echoed
  # (review R3 D1).
  arms=$((arms + 1)); leaked=""
  for place in seal sealed-in exposed custody bare comment first name; do
    fresh sealed
    case "$place" in
      seal|sealed-in) edit "s/^# $place: .*/# $place: $LONG/" ;;
      exposed|custody) manifest "# $place: $LONG" ;;
      bare) manifest "$LONG" ;;
      comment) manifest "# $LONG" ;;
      first) { printf '%s\n' "$LONG"; cat "$d/MANIFEST.txt"; } > "$d/MANIFEST.new"; mv "$d/MANIFEST.new" "$d/MANIFEST.txt" ;;
      name) manifest "0000000000000000000000000000000000000000000000000000000000000000  $LONG" ;;
    esac
    out="$(cd "$work" && bash "$SELF" 2>&1; cd "$work" && bash "$SELF" --restore 2>&1)"
    printf '%s' "$out" | grep -q 'ARM-CASE-TEXT' && leaked="$leaked $place"
  done
  if [ -z "$leaked" ]; then ok=$((ok + 1)); echo "  ✅ a case's line in any field of the manifest is never printed (8 places)"
  else echo "SELF-TEST: a case's line in the manifest was printed, from:$leaked" >&2; fi
  fresh sealed; printf '%s\n' "$LONG" >> "$work/docs/notes/plan.md"; g add -A; printf 'nothing about the set here\n' > "$work/docs/notes/plan.md"
  arm "a quote staged and gone from disk is refused (custody)" 1 "'docs/notes/plan.md' quotes a line of a sealed case"
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/plan.md"
  arm "a case's whole text in a modified tracked file is refused (custody)" 1 "a sealed case's text is in the working tree at 'docs/notes/plan.md'"
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/café copy.md"
  arm "a copy under a name git would quote is refused (custody)" 1 "a sealed case's text is in the working tree at 'docs/notes/café copy.md'"
  fresh sealed; printf 'locked\n' > "$work/docs/notes/locked.md"; chmod 000 "$work/docs/notes/locked.md"
  arm "an untracked file that cannot be read is refused, the leg failing closed" 1 "'docs/notes/locked.md' cannot be read"
  chmod 600 "$work/docs/notes/locked.md"
  # What git cannot list fails closed (review R3 D2): an untracked folder it cannot open, a tracked file under a folder
  # that cannot be searched.
  fresh sealed; mkdir "$work/docs/notes/shut"; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/shut/copy.md"
  chmod 000 "$work/docs/notes/shut"
  arm "an untracked folder git cannot open is refused, a copy there unseen" 1 "git could not list every file"
  chmod 755 "$work/docs/notes/shut"
  fresh sealed; mkdir "$work/docs/notes/held"; printf 'tracked\n' > "$work/docs/notes/held/t.md"; g add -A
  g -c core.hooksPath=/dev/null commit -q -m held
  g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/held/t.md"; chmod 000 "$work/docs/notes/held"
  arm "a tracked file under a folder that cannot be searched is refused" 1 "'docs/notes/held' cannot be searched"
  chmod 755 "$work/docs/notes/held"
  # A nested repository staged as a gitlink that .gitmodules does not register: refused; one it registers: another
  # repository, named in the honest limit (review R4 D3).
  fresh sealed; git -C "$work/docs/notes" init -q nested; printf 'x\n' > "$work/docs/notes/nested/x.md"
  git -C "$work/docs/notes/nested" add -A; git -C "$work/docs/notes/nested" -c user.name=arm -c user.email=arm@example.invalid -c commit.gpgsign=false commit -q -m n
  g add -A 2>/dev/null
  arm "a repository staged as an unregistered gitlink is refused (custody)" 1 "'docs/notes/nested' is a repository staged as a gitlink that .gitmodules does not register"
  printf '[submodule "nested"]\n\tpath = docs/notes/nested\n\turl = ./nested\n' > "$work/.gitmodules"
  arm "a gitlink registered by a .gitmodules on disk alone is refused — a commit records the staged one" 1 "'docs/notes/nested' is a repository staged as a gitlink"
  g add -A
  arm "a gitlink .gitmodules registers passes — another repository, named in the honest limit" 0 ""
  # Paths printed as their files are named, in every leg (review R6 D1).
  fresh sealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/café.md"; g add -A
  arm "a non-ASCII path naming a case is printed as named" 1 "    docs/notes/café.md"
  fresh sealed; printf 'x\n' > "$d/naïve.md"; g add -A
  arm "a non-ASCII file tracked in the set is printed as named" 1 "'docs/evaluation/frozen/naïve.md' is tracked"
  # Paths as named in the other legs too: the tracked copy, the quote search of the index and of the disk, the gitlink
  # leg (review R7 AG1).
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/café copy.md"; g add -A
  arm "a tracked copy under a non-ASCII name is named as it is" 1 "a sealed case's text is tracked at 'docs/notes/café copy.md'"
  fresh sealed; printf '%s\n' "$LONG" > "$work/docs/notes/naïve.md"; g add -A; rm "$work/docs/notes/naïve.md"
  arm "a staged quote under a non-ASCII name is named as it is" 1 "'docs/notes/naïve.md' quotes a line of a sealed case"
  fresh sealed; printf '%s\n' "$LONG" > "$work/docs/notes/naïve.md"
  arm "an untracked quote under a non-ASCII name is named as it is" 1 "'docs/notes/naïve.md' quotes a line of a sealed case"
  fresh sealed; git -C "$work/docs/notes" init -q "né"; printf 'x\n' > "$work/docs/notes/né/x.md"
  git -C "$work/docs/notes/né" add -A
  git -C "$work/docs/notes/né" -c user.name=arm -c user.email=arm@example.invalid -c commit.gpgsign=false commit -q -m n
  printf '[submodule "ne"]\n\tpath = docs/notes/né\n\turl = ./ne\n' > "$work/.gitmodules"; g add -A 2>/dev/null
  arm "a registered gitlink under a non-ASCII name passes" 0 ""
  # A registered path with a space is read whole (review R5 R2).
  fresh sealed; git -C "$work/docs/notes" init -q "my nested"; printf 'x\n' > "$work/docs/notes/my nested/x.md"
  git -C "$work/docs/notes/my nested" add -A
  git -C "$work/docs/notes/my nested" -c user.name=arm -c user.email=arm@example.invalid -c commit.gpgsign=false commit -q -m n
  printf '[submodule "my nested"]\n\tpath = docs/notes/my nested\n\turl = ./my-nested\n' > "$work/.gitmodules"; g add -A 2>/dev/null
  arm "a registered gitlink whose path has a space passes" 0 ""
  # Git's warnings are taken from listing untracked files alone: a malformed attribute line warned of while a racy file
  # is re-read is no unlisted file, and the verdict does not hang on the race (review R5 D2).
  fresh sealed; printf '*.md comparator@deadline\n' >> "$work/.gitattributes"; printf 'touched\n' >> "$work/docs/notes/plan.md"; g add -A
  arm "a malformed attribute line warned of while listing modified files is no refusal" 0 ""
  arm "and the verdict is the same on a second run" 0 ""
  # The scratch never holds a manifest line of another form: a copy that keeps its scratch finds no case's line in it
  # (review R5 D3).
  fresh sealed; manifest "$LONG"
  sed "s/trap 'rm -rf \"\$tmp\"' EXIT/trap : EXIT/" "$SELF" > "$work/../kept.sh"
  kept_out="$(cd "$work" && bash "$work/../kept.sh" 2>&1)"
  arms=$((arms + 1))
  if ! printf '%s' "$kept_out" | grep -q 'ARM-CASE-TEXT' &&
     [ -n "$(find "$work/target/doctrine_scratch" -name 'frozen_evaluation.*' -type d 2>/dev/null)" ] &&
     ! grep -rq 'ARM-CASE-TEXT' "$work/target/doctrine_scratch" 2>/dev/null; then
    ok=$((ok + 1)); echo "  ✅ the check's scratch holds no manifest line of another form"
  else echo "SELF-TEST: the check's scratch was not kept, or held a case's line" >&2; fi
  rm -f "$work/../kept.sh"
  fresh sealed; git -C "$work/docs/notes" init -q nested; printf 'x\n' > "$work/docs/notes/nested/x.md"
  arm "a repository nested outside ignored folders is refused (custody)" 1 "'docs/notes/nested/' is a repository nested outside ignored folders"
  fresh sealed; ln -s nowhere "$d/zz-01-arm-alpha.md"
  arm "a dangling link at a case's path is refused (custody)" 1 "sealed case 'zz-01-arm-alpha.md' is in the working tree"
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
  arm "a manifest line that is no entry is refused, by its number" 1 "MANIFEST.txt's line 5 is not '<sha256>  <file>'"
  fresh sealed; manifest "$LONG"
  arm "a case's line pasted bare into the manifest is refused, never echoed" 1 "MANIFEST.txt' quotes a line of a sealed case"
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
  arm "an exposure naming no listed case is refused (exposure)" 1 "'# exposed:' line 5 names no case the manifest lists"
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
  fresh unsealed; rm "$d/zz-01-arm-alpha.md"; ln -s zz-02-arm-beta.md "$d/zz-01-arm-alpha.md"
  arm "once unsealed, a case that is a link is refused (integrity)" 1 "listed case 'zz-01-arm-alpha.md' in docs/evaluation/frozen is a link"
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

# A field's value is checked inside its pipeline, before any variable holds it, so a value of the wrong form never
# reaches a command, a note or a trace (reviews R3 D1, R4 D2).
field() { grep -E "^# $1:" "$MANIFEST" | head -1 | sed "s/^# $1:[[:space:]]*//"; }
seal="$(field seal | grep -xE 'sealed|unsealed')"
[ -n "$seal" ] || note "$MANIFEST has no '# seal: sealed|unsealed' line"

# Scratch on this repository's own volume, never `$TMPDIR` (leaf `PROGRAM.29`, `SCRATCH-LOCALITY`). It holds well-
# formed entries, names, blob ids and paths only, never a manifest line of another form nor a case's text (R5 D3).
mkdir -p "$ROOT/target/doctrine_scratch"
tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/frozen_evaluation.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
# A line that is no entry is named by its number alone: its text could be anything, a case's line among it (R2 D2).
while IFS= read -r n; do
  note "$MANIFEST's line $n is not '<sha256>  <file>' — every line but a comment or a blank is an entry"
done < <(grep -nvE "$ENTRY|^[[:space:]]*(#|\$)" "$MANIFEST" | cut -d: -f1)
grep -E "$ENTRY" "$MANIFEST" > "$tmp/entries.txt" || true
awk '{ print $2 }' "$tmp/entries.txt" > "$tmp/listed.txt"

sealed_in="$(field sealed-in | grep -xE '[0-9a-f]{40}([0-9a-f]{24})?')"
if [ -z "$sealed_in" ]; then
  note "$MANIFEST has no '# sealed-in: <commit>' line naming the commit that holds the cases' text"
  sealed_in=""
elif ! git cat-file -e "${sealed_in}^{commit}" 2>/dev/null; then
  note "the sealing commit $sealed_in is not in this clone, so no digest can be verified — fetch the full history (a shallow clone does not hold it)"
  sealed_in=""
elif ! git merge-base --is-ancestor "$sealed_in" HEAD 2>/dev/null; then
  note "the sealing commit $sealed_in is not an ancestor of HEAD — a set is sealed in this history or not at all"
  sealed_in=""
elif ! git cat-file -e "$sealed_in:$MANIFEST" 2>/dev/null; then
  note "the sealing commit $sealed_in holds no $MANIFEST to pin the set's entries, or this clone cannot fetch it"
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
    # In the working tree and in the index, the one a commit records (review R2).
    [ "$(git check-attr diff -- "$path" 2>/dev/null | sed 's/.*: diff: //')" = "unset" ] &&
      [ "$(git check-attr --cached diff -- "$path" 2>/dev/null | sed 's/.*: diff: //')" = "unset" ] \
      || note "'$name' is not marked -diff in .gitattributes, staged and on disk — a diff or a log patch of a commit that holds the set would print its text"
  done < "$tmp/entries.txt"
  while IFS= read -r -d '' tracked; do
    [ "$tracked" = "$MANIFEST" ] || note "'$tracked' is tracked while the set is sealed — a case's text stays in the sealing commit"
  done < <(git ls-files -z -- "$DIR")
  if [ -s "$tmp/blobs.txt" ]; then
    # A case's whole text at any other path: in the index, by its blob id; in the working tree, untracked or
    # modified, by the blob id of what is there, each file hashed on its own and one that cannot be read named, so the
    # leg fails closed. Paths are read NUL-separated, never quoted. Ignored files are not scanned; a repository nested
    # outside ignored folders is refused, since its files are none the index or `--others` lists (review R2 D4).
    tab="$(printf '\t')"
    # A repository staged as a gitlink that `.gitmodules` does not register: its files are none the check can list, so
    # it is refused (review R4 D3). A registered submodule is another repository, named in the honest limit.
    # Registration is read from the staged `.gitmodules`, the one a commit records, each path whole (review R5 R1, R2).
    git config -z --blob :.gitmodules --get-regexp '^submodule\..*\.path$' 2>/dev/null |
      while IFS= read -r -d '' kv; do printf '%s\n' "${kv#*$'\n'}"; done > "$tmp/registered"
    while IFS= read -r -d '' rec; do
      case "$rec" in
        "160000 "*) at="${rec#*"$tab"}"
                    grep -qxF -- "$at" "$tmp/registered" ||
                      note "'$at' is a repository staged as a gitlink that .gitmodules does not register — its files cannot be checked for a case" ;;
      esac
    done < <(git ls-files -s -z)
    # One pass finds whether any entry holds a case's blob; only then is each path named, read NUL-separated.
    if git ls-files -s | awk 'NR == FNR { b[$1]; next } ($2 in b) { f = 1 } END { exit !f }' "$tmp/blobs.txt" -; then
      while IFS= read -r -d '' rec; do
        id="${rec#* }"; id="${id%% *}"
        grep -qxF "$id" "$tmp/blobs.txt" && note "a sealed case's text is tracked at '${rec#*"$tab"}'"
      done < <(git ls-files -s -z)
    fi
    # Fail closed on what git cannot list: a folder it cannot open is named by git's own warning, a tracked file under a
    # folder that cannot be searched by the nearest such folder (review R3 D2).
    { git ls-files -z --others --exclude-standard 2> "$tmp/loose.err"; git ls-files -z -m 2>/dev/null; } > "$tmp/loose"
    while IFS= read -r warning; do
      note "git could not list every file: ${warning#warning: } — a case's copy there cannot be ruled out"
    done < "$tmp/loose.err"
    while IFS= read -r -d '' at; do
      case "$at" in
        */) note "'$at' is a repository nested outside ignored folders — its files cannot be checked for a case; ignore it or move it" ;;
        *) if [ ! -e "$at" ] && [ ! -L "$at" ]; then
             d="$(dirname -- "$at")"
             while [ "$d" != "." ] && [ "$d" != "/" ]; do
               if [ -e "$d" ] && [ ! -x "$d" ]; then
                 note "'$d' cannot be searched, so a case's copy there cannot be ruled out"; break
               fi
               d="$(dirname -- "$d")"
             done
             continue
           fi
           [ -f "$at" ] || continue
           if ! id="$(git hash-object -- "$at" 2>/dev/null)"; then
             note "'$at' cannot be read, so a case's copy there cannot be ruled out"
           elif grep -qxF "$id" "$tmp/blobs.txt"; then
             note "a sealed case's text is in the working tree at '$at'"
           fi ;;
      esac
    done < "$tmp/loose"
    # A long line of a case quoted anywhere, the manifest included, staged or on disk, tracked or untracked: found by
    # the name of the file that holds it, the patterns read from the blobs through a pipe, never written down, never in
    # a command's arguments (review R2 D2, D3).
    quotes() { while IFS= read -r id; do git cat-file blob "$id"; done < "$tmp/blobs.txt" |
                 awk '{ gsub(/^[[:space:]]+|[[:space:]]+$/, "") } length($0) >= 60 && NF >= 8'; }
    if quotes | grep -q .; then
      while IFS= read -r -d '' at; do
        note "'$at' quotes a line of a sealed case — the set is no longer unseen"
      done < <({ git grep -z -l --cached -F -f <(quotes) 2>/dev/null; git grep -z -l --untracked -F -f <(quotes) 2>/dev/null; } |
               sort -zu)
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
# An exposure names a listed case; one that does not is named by its line's number, never by its text (R3 D1).
# The name is matched inside the pipeline; only an unmatched line's number reaches the shell (review R4 D2).
while IFS= read -r n; do
  note "$MANIFEST's '# exposed:' line $n names no case the manifest lists"
done < <(grep -nE '^# exposed:' "$MANIFEST" | sed -E 's/^([0-9]+):# exposed:[[:space:]]*([^[:space:]]*).*/\1 \2/' |
         awk 'NR == FNR { l[$0]; next } !($2 in l) { print $1 }' "$tmp/listed.txt" -)

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
    hits="$(git grep -z -l --fixed-strings -- "$slug" \
              -- . ":(exclude)$DIR" ":(exclude)scripts/check_frozen_evaluation.sh" \
            2>/dev/null | tr '\0' '\n' || true)"  # paths as their files are named (review R6 D1)
    if [ -n "$hits" ]; then
      note "sealed case '$slug' is named outside $DIR — the set is no longer unseen"
      printf '%s\n' "$hits" | sed 's/^/    /' >&2
    fi
  done < "$tmp/slugs.txt"
fi

exit $fail
