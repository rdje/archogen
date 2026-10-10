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
# made for other work landed in a sealed case. So while the set is sealed no case's file is in
# the working tree: each case lives only as the blob the manifest's `# sealed-in:`
# commit holds, the manifest keeps its digest — a commitment, revealed at unsealing by
# `--restore` — and `.gitattributes` marks the sealed paths `-diff`, so in a working tree that
# carries the mark (`6d61f65` on) a diff, a log patch or a `git grep` of a commit that holds them
# prints no line of them.
#
# ⚠️ A seal that is only a promise is not a seal. This check makes these mechanical:
#   1. INTEGRITY    — the manifest's entries are the sealing commit's own, that commit an ancestor
#                     of HEAD; every case's blob there hashes to its digest, and once unsealed the
#                     restored file does too: the set cannot be rewritten, grown or shrunk.
#   2. CUSTODY      — while sealed, no case is at its path, in the index, or — a file whose content is
#                     its text, byte for byte — anywhere in the index or in what the check walks: the
#                     working tree but for what its `.gitignore` files ignore, files and folders,
#                     registered submodules (named by the staged `.gitmodules` and held as gitlinks),
#                     links and its own `.git`, every file read raw, whatever git says of it; no
#                     file, the manifest included, staged or in what it walks, quotes a long line of
#                     one, trimmed and stripped of a list or heading marker; no unregistered repository nested outside
#                     ignored folders, staged or not, no entry named `.git` in any case but the top
#                     level's, nothing the check cannot read or search, no walk that cannot finish;
#                     its path is marked `-diff` — as this working tree's diffs obey, staged and on
#                     disk, and by the staged and the on-disk `.gitattributes` itself.
#                     (reviews R9 D2–D4, R10 D1–D6)
#   3. COMPLETENESS — nothing is in the sealed directory but the manifest and, once unsealed, the
#                     listed cases, at any depth, hidden or linked; once unsealed, none is missing.
#   4. NON-CONTAMINATION — no tracked file outside the sealed directory, staged or on disk — a
#                     flagged file read from the disk — this script included, names a sealed case in
#                     its content or its path: a case discussed in a task tree, a decision record or
#                     a design note is no longer unseen (reviews R9 D5, D6, R10 D2, R3).
#   It runs git with submodule recursion off and no template, a caller's `git -c` dropped, reads
#   the files it walks itself, and takes as ignored only what the `.gitignore` files in the tree
#   ignore; it holds against accident, not against concealment on purpose — see the README
#   (reviews R10, R11).
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
# in a working tree without the mark, holds whatever was shown then, and so may a harness's memory
# written then; a model trained or an index built on the branch while it held the set — from
# `a3c0cbd`, `2026-10-06`, to `a543d10` — may show a case with no act, so `M6.5` records those
# used; a short quote committed outside the folder before `6d61f65` shows in that commit's diff.
# What to do with them is the director's.
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
# The settings that would change what git gives this check, forced for every git it runs: submodule recursion off,
# with which `git grep` stops (review R11 D1); no template for the empty repository it makes (R10 D5); git's default
# path quoting. A caller's `git -c` would win over them, so it is dropped. What is on disk the check reads itself, and
# what is ignored is what the repository's own `.gitignore` files ignore (below), so no other setting changes either.
unset GIT_CONFIG_PARAMETERS
export GIT_CONFIG_COUNT=3 GIT_CONFIG_KEY_0=submodule.recurse GIT_CONFIG_VALUE_0=false \
  GIT_CONFIG_KEY_1=init.templateDir GIT_CONFIG_VALUE_1= GIT_CONFIG_KEY_2=core.quotePath GIT_CONFIG_VALUE_2=true

# ── RED arms ─────────────────────────────────────────────────────────────────────────────────
# ⛔ Every case name below is SYNTHETIC. Writing a real sealed slug anywhere tracked is the contamination
# this check exists to catch, and a self-test that did it would fail the gate it belongs to.
self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/frozen_evaluation/selftest" sealed_in=""
  # git quotes a non-ASCII path unless told not to: forced on here, so the path arms hold on any machine (R7 R3) — over
  # the configuration files, and over a caller's `git -c`, which reaches this shell as GIT_CONFIG_PARAMETERS and would
  # win; the caller's own GIT_CONFIG_COUNT entries are dropped with it, the self-test's git being its own (R8 R5).
  unset GIT_CONFIG_PARAMETERS
  export GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=core.quotePath GIT_CONFIG_VALUE_0=true
  local LONG="ARM-CASE-TEXT alpha asks one comparator to keep two requesters each on its own deadline"
  local BULLET="ARM-CASE-TEXT bullet keeps both requesters apart, each one held to its own deadline"
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
    printf 'ARM-CASE-TEXT alpha\n%s\n- %s\n' "$LONG" "$BULLET" > "$d/zz-01-arm-alpha.md"
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

  holds "the self-test's quoting holds over a caller's git -c core.quotePath=false (R9 R9)" \
    test "$(GIT_CONFIG_PARAMETERS="'core.quotepath'='false'" bash -c "$(sed -n '/^  unset GIT_CONFIG_PARAMETERS$/,/GIT_CONFIG_VALUE_0=true$/p' "$SELF")
git config --get core.quotePath")" = true
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
  fresh sealed; : > "$work/.gitattributes"
  arm "a -diff mark dropped on disk alone is refused (custody, R8 AG2)" 1 "'zz-01-arm-alpha.md' is not marked -diff"
  # The mark from a source no commit carries counts for nothing (review R9 D4).
  fresh sealed; : > "$work/.gitattributes"; g add -A; printf 'docs/evaluation/frozen/** -diff\n' > "$work/.git/info/attributes"
  arm "a -diff mark from .git/info/attributes alone is refused (custody, R9 D4)" 1 "'zz-01-arm-alpha.md' is not marked -diff by the staged .gitattributes itself"
  fresh sealed; : > "$work/.gitattributes"; g add -A; printf 'docs/evaluation/frozen/** -diff\n' > "$work/../attributes"
  g config core.attributesFile "$work/../attributes"
  arm "a -diff mark from core.attributesFile alone is refused (custody, R9 D4)" 1 "'zz-01-arm-alpha.md' is not marked -diff by the on-disk .gitattributes itself"
  rm -f "$work/../attributes"
  # A file git is told not to look at, and an entry named .git, hide nothing (review R9 D3, D2).
  fresh sealed; g update-index --assume-unchanged docs/notes/plan.md; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/plan.md"
  arm "a case's text in a tracked file marked assume-unchanged is refused (custody, R9 D3)" 1 "a sealed case's text is in the working tree at 'docs/notes/plan.md'"
  fresh sealed; g update-index --skip-worktree docs/notes/plan.md; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/plan.md"
  arm "a case's text in a tracked file marked skip-worktree is refused (custody, R9 D3)" 1 "a sealed case's text is in the working tree at 'docs/notes/plan.md'"
  fresh sealed; mkdir "$work/docs/notes/.git"; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/.git/copy.md"
  arm "a case's copy in a folder named .git is refused (custody, R9 D2)" 1 "'docs/notes/.git' is an entry named .git"
  fresh sealed; mkdir -p "$work/docs/loose/.GIT"; printf 'see: %s\n' "$LONG" > "$work/docs/loose/.GIT/q.md"
  arm "a quote in an untracked folder named .GIT is refused (custody, R9 D2)" 1 "'docs/loose/.GIT' is an entry named .git"
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/.git"
  arm "a case's text in a file named .git is refused (custody, R9 D2)" 1 "'docs/notes/.git' is an entry named .git"
  # Round 10: a path .gitmodules names is no submodule unless the index holds it as one; a file is read raw, whatever
  # git's monitor, stat settings, filters or flags say of it; no template's attributes; a walk that cannot finish
  # refuses; a mark from the per-user attributes file is no commit's; a .git inside an ignored folder is not walked.
  fresh sealed; printf '[submodule "x"]\n\tpath = docs/notes\n\turl = ./x\n' > "$work/.gitmodules"; g add .gitmodules
  mkdir "$work/docs/notes/.git"; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/.git/copy.md"
  arm "a path .gitmodules names but the index holds as no gitlink is walked (custody, R10 D1)" 1 "'docs/notes/.git' is an entry named .git"
  # A monitor hook speaking git's protocol — a token, no path changed — makes git trust every entry it has seen.
  fresh sealed; printf '#!/bin/sh\nprintf "tok\\000"\n' > "$work/../fsmon"; chmod +x "$work/../fsmon"; g config core.fsmonitor "$work/../fsmon"
  g update-index --fsmonitor; g status --porcelain > /dev/null
  g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/plan.md"
  arm "a case's text a file-system monitor reports unchanged is refused (custody, R10 D3)" 1 "a sealed case's text is in the working tree at 'docs/notes/plan.md'"
  # The laxest stat settings, a file of the same size and its time restored: git sees no change (the reviewer's probe).
  fresh sealed; printf 'xxxxxxxxxxxxxxxxxx\n' > "$work/docs/notes/plan.md"; g add -A; touch -r "$work/docs/notes/plan.md" "$work/../stamp"
  g config core.checkStat minimal; g config core.trustctime false; sleep 1; g update-index --refresh > /dev/null
  g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/plan.md"; touch -r "$work/../stamp" "$work/docs/notes/plan.md"; rm -f "$work/../stamp"
  arm "a case's text under git's laxest stat settings is refused (custody, R10 D3)" 1 "a sealed case's text is in the working tree at 'docs/notes/plan.md'"
  fresh sealed; g config filter.rev.clean "rev"; printf 'docs/notes/** filter=rev\n' > "$work/.git/info/attributes"
  g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/copy.md"
  arm "a case's text behind a clean filter is refused, its bytes read raw (custody, R10 D4)" 1 "a sealed case's text is in the working tree at 'docs/notes/copy.md'"
  fresh sealed; mkdir -p "$work/../template/info"; printf 'docs/evaluation/frozen/** -diff\n' > "$work/../template/info/attributes"
  : > "$work/.gitattributes"; g add -A; printf 'docs/evaluation/frozen/** -diff\n' > "$work/.git/info/attributes"
  GIT_TEMPLATE_DIR="$work/../template" arm "a mark from a template's attributes counts for nothing (custody, R10 D5)" 1 "'zz-01-arm-alpha.md' is not marked -diff by the staged .gitattributes itself"
  fresh sealed; mkdir -p "$work/../nopython"; printf '#!/bin/sh\nexit 3\n' > "$work/../nopython/python3"; chmod +x "$work/../nopython/python3"
  PATH="$work/../nopython:$PATH" arm "a walk that cannot finish refuses (custody, R10 D6)" 1 "the working tree could not be walked"
  fresh sealed; mkdir -p "$work/../xdg/git"; printf 'docs/evaluation/frozen/** -diff\n' > "$work/../xdg/git/attributes"; : > "$work/.gitattributes"; g add -A
  XDG_CONFIG_HOME="$work/../xdg" arm "a mark from the per-user attributes file counts for nothing (custody, R10 AG)" 1 "'zz-01-arm-alpha.md' is not marked -diff by the staged .gitattributes itself"
  fresh sealed; printf 'ignored/\n' > "$work/.gitignore"; g add .gitignore; mkdir -p "$work/ignored/.git"; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/ignored/.git/copy.md"
  arm "a .git inside an ignored folder is not walked — ignored folders are the limit's (R10 R1)" 0 ""
  rm -rf "$work/../fsmon" "$work/../template" "$work/../nopython" "$work/../xdg"
  # Round 11: no setting turns a search off or widens what is ignored; the repository's own .gitignore files decide
  # what is ignored, files and folders; a bullet's line is found without its marker; the walk imports nothing.
  fresh sealed; printf '[submodule]\n\trecurse = true\n' > "$work/../gconfig"; printf 'see: %s\n' "$LONG" > "$work/docs/notes/draft.md"
  GIT_CONFIG_GLOBAL="$work/../gconfig" arm "a quote on disk with submodule recursion on is refused (custody, R11 D1)" 1 "'docs/notes/draft.md' quotes a line of a sealed case"
  fresh sealed; printf '%s\n' "$LONG" >> "$work/docs/notes/plan.md"
  GIT_CONFIG_GLOBAL="$work/../gconfig" arm "an unstaged quote in a tracked file with submodule recursion on is refused (custody, R11 D1)" 1 "'docs/notes/plan.md' quotes a line of a sealed case"
  fresh sealed; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/copy.md"; printf '%s\n' "$LONG" >> "$work/docs/notes/plan.md"; g add -A
  mkdir -p "$work/../gitshim"; printf '#!/bin/sh\ncase "$1" in grep) exit 2 ;; esac\nexec %s "$@"\n' "$(command -v git)" > "$work/../gitshim/git"; chmod +x "$work/../gitshim/git"
  PATH="$work/../gitshim:$PATH" arm "a git grep that cannot run refuses (custody, R11 D1)" 1 "git grep could not search the index"
  fresh sealed; mkdir -p "$work/../xdg-ignore"; printf '*.bak\n' > "$work/../xdg-ignore/ignore"; printf '[core]\n\texcludesFile = %s\n' "$work/../xdg-ignore/ignore" > "$work/../xdg-ignore/config"
  g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/plan.md.bak"
  GIT_CONFIG_GLOBAL="$work/../xdg-ignore/config" arm "a per-user ignore file hides nothing (custody, R11 D2)" 1 "a sealed case's text is in the working tree at 'docs/notes/plan.md.bak'"
  fresh sealed; printf 'scratch/\n' > "$work/../excludes"; g config core.excludesFile "$work/../excludes"; mkdir "$work/docs/notes/scratch"; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/scratch/c.md"
  arm "an exclude file in the repository's configuration hides nothing (custody, R11 D2)" 1 "a sealed case's text is in the working tree at 'docs/notes/scratch/c.md'"
  fresh sealed; printf 'docs/notes/\n' >> "$work/.git/info/exclude"; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/copy.md"
  arm "the local exclude file hides nothing (custody, R11 D2)" 1 "a sealed case's text is in the working tree at 'docs/notes/copy.md'"
  fresh sealed; printf '*.log\n' > "$work/.gitignore"; g add .gitignore; g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/session.log"
  arm "a file the repository's .gitignore ignores is not read — ignored files are the limit's (R11 D3)" 0 ""
  fresh sealed; printf 'notes: %s\n' "$BULLET" > "$work/docs/notes/draft.md"
  arm "a bullet's line quoted without its marker is refused on disk (custody, R11 R2)" 1 "'docs/notes/draft.md' quotes a line of a sealed case"
  fresh sealed; printf 'notes: %s\n' "$BULLET" >> "$work/docs/notes/plan.md"; g add -A; g show HEAD:docs/notes/plan.md > "$work/docs/notes/plan.md"
  arm "a bullet's line quoted without its marker is refused staged (custody, R11 R2)" 1 "'docs/notes/plan.md' quotes a line of a sealed case"
  fresh sealed; printf 'raise SystemExit(7)\n' > "$work/hashlib.py"; printf 'raise SystemExit(7)\n' > "$work/stat.py"
  arm "the walk imports nothing from the tree (custody, R11 R3)" 0 ""
  rm -rf "$work/../gconfig" "$work/../gitshim" "$work/../xdg-ignore" "$work/../excludes"
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
  fresh sealed; printf 'x\n' > "$work/docs/notes/café.md"; g add -A
  g show "$sealed_in:docs/evaluation/frozen/zz-02-arm-beta.md" > "$work/docs/notes/café.md"
  arm "a case's text in a modified tracked file git would quote is refused (custody, R8 AG1)" 1 "a sealed case's text is in the working tree at 'docs/notes/café.md'"
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
  # The submodule active, as one cloned with its superproject is, so a recursion has somewhere to go.
  g config submodule.nested.url ./docs/notes/nested
  printf '%s\n' "$LONG" > "$work/docs/notes/nested/q.md"; git -C "$work/docs/notes/nested" add -A
  git -C "$work/docs/notes/nested" -c user.name=arm -c user.email=arm@example.invalid -c commit.gpgsign=false commit -q -m q; g add -A
  GIT_CONFIG_PARAMETERS="'submodule.recurse'='true'" arm "a caller's git -c cannot widen the index's search into a registered submodule (R11 D1)" 0 ""
  printf '[submodule]\n\trecurse = true\n' > "$work/../gconfig-recurse"
  GIT_CONFIG_GLOBAL="$work/../gconfig-recurse" arm "a configured submodule recursion cannot widen the index's search into a registered submodule (R11 D1)" 0 ""
  rm -f "$work/../gconfig-recurse"
  # Paths printed as their files are named, in every leg (review R6 D1).
  fresh sealed; printf 'we discussed zz-01-arm-alpha today\n' > "$work/docs/notes/café.md"; g add -A
  arm "a non-ASCII path naming a case is printed as named" 1 "    docs/notes/café.md"
  fresh sealed; printf 'see zz-01-arm-alpha\n' >> "$work/docs/notes/plan.md"; g add -A; g show HEAD:docs/notes/plan.md > "$work/docs/notes/plan.md"
  arm "a case named in a staged file, gone from disk, is refused (non-contamination, R9 D5)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; mkdir -p "$work/scripts"; printf '# zz-01-arm-alpha\n' > "$work/scripts/check_frozen_evaluation.sh"; g add -A
  arm "a case named in the check's own script is refused (non-contamination, R9 D6)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; printf 'see zz-01-arm-alpha\n' >> "$work/docs/notes/plan.md"
  arm "a case named unstaged in a tracked file is refused (non-contamination, R11 AG)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; g update-index --assume-unchanged docs/notes/plan.md; printf 'see zz-01-arm-alpha\n' >> "$work/docs/notes/plan.md"
  arm "a case named on disk in a file marked assume-unchanged is refused (non-contamination, R10 D2)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; g update-index --skip-worktree docs/notes/plan.md; printf 'see zz-01-arm-alpha\n' >> "$work/docs/notes/plan.md"
  arm "a case named on disk in a file marked skip-worktree is refused (non-contamination, R10 D2)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
  fresh sealed; printf 'notes\n' > "$work/docs/notes/zz-01-arm-alpha-notes.md"; g add -A
  arm "a tracked path naming a case is refused (non-contamination, R10 R3)" 1 "sealed case 'zz-01-arm-alpha' is named outside"
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
# formed entries, names, blob ids, sizes and paths, never a manifest line of another form nor a case's text (R5 D3) —
# and an empty repository holding copies of the staged and the on-disk `.gitattributes`, which hold a case's line only
# if the staged or the on-disk `.gitattributes` quotes one, which the check refuses (reviews R10 D7, R11 D7).
mkdir -p "$ROOT/target/doctrine_scratch"
tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/frozen_evaluation.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
scratch="${tmp#"$ROOT"/}"  # as git names paths, relative to the top level
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
  # The mark as the tree records it: the staged and the on-disk `.gitattributes`, each read alone in an empty repository,
  # so a mark from `.git/info/attributes` or `core.attributesFile`, which no commit carries, does not count (review R9
  # D4); the checks above are what this working tree's own diffs obey.
  # That repository is made in the check's own scratch, which the listings below skip: made fresh under a name of its
  # own on every run and removed on exit, nothing but the check's own files is ever in it.
  iso=(env -u GIT_DIR -u GIT_WORK_TREE -u GIT_INDEX_FILE GIT_ATTR_NOSYSTEM=1 GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null)
  attrs="$tmp/attributes"
  "${iso[@]}" GIT_TEMPLATE_DIR= git init -q --template= "$attrs" 2>/dev/null  # no template's attributes (R10 D5)
  for where in staged on-disk; do
    if [ "$where" = staged ]; then git show :.gitattributes > "$attrs/.gitattributes" 2>/dev/null || : > "$attrs/.gitattributes"
    else cat .gitattributes > "$attrs/.gitattributes" 2>/dev/null || : > "$attrs/.gitattributes"; fi
    while read -r want name; do
      [ "$("${iso[@]}" git -C "$attrs" -c core.attributesFile=/dev/null check-attr diff -- "$DIR/$name" 2>/dev/null | sed 's/.*: diff: //')" = "unset" ] ||
        note "'$name' is not marked -diff by the $where .gitattributes itself — a mark from .git/info/attributes or core.attributesFile is no commit's"
    done < "$tmp/entries.txt"
  done
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
    # What is ignored is what the repository's own `.gitignore` files ignore, files and folders, listed once: no
    # per-user or local exclude file, which no commit carries, widens it (review R11 D2, D3).
    git ls-files -z --others --ignored --directory --exclude-per-directory=.gitignore > "$tmp/ignored" 2>/dev/null ||
      note "git could not list what the repository's .gitignore files ignore — a case's copy cannot be ruled out"
    # What git cannot list fails closed: a folder it cannot open is named by its own warning (review R3 D2); a
    # repository nested outside ignored folders is refused, since its files are none git lists (R2 D4).
    git ls-files -z --others --exclude-per-directory=.gitignore 2> "$tmp/loose.err" > "$tmp/loose"
    while IFS= read -r warning; do
      note "git could not list every file: ${warning#warning: } — a case's copy there cannot be ruled out"
    done < "$tmp/loose.err"
    while IFS= read -r -d '' at; do
      case "$at" in
        "$scratch"/*) ;;  # the check's own scratch
        */) note "'$at' is a repository nested outside ignored folders — its files cannot be checked for a case; ignore it or move it" ;;
      esac
    done < "$tmp/loose"
    # A long line of a case: each line of its text, trimmed and stripped of a list, quote, heading or table marker, of
    # 60 characters or more and 8 words or more (review R11 R2), read from the blobs through a pipe, never written down,
    # never in a command's arguments (review R2 D2, D3).
    quotes() { while IFS= read -r id; do git cat-file blob "$id"; done < "$tmp/blobs.txt" |
                 awk '{ gsub(/^[[:space:]]+|[[:space:]]+$/, ""); sub(/^([-*+>#|]+|[0-9]+[.)])[[:space:]]+/, "") }
                      length($0) >= 60 && NF >= 8'; }
    # The working tree read by the check itself, never through git's view of what changed: one walk from the top,
    # pruning only what the repository's `.gitignore` files ignore, a registered submodule — a path the staged
    # `.gitmodules` names that the index holds as a gitlink — and the check's own scratch, every other file read raw. It
    # refuses a file whose bytes are a case's, hashed as git hashes a blob, and one that holds a long line of a case,
    # whatever git says of it — modified or not, flagged, filtered or watched (reviews R9 D3, R10 D3, D4, R11 D1); an
    # entry named `.git` in any case but the top level's (R9 D2); a file it cannot read or a folder it cannot search. A
    # walk that cannot finish refuses (R10 D6). Links are not followed; python runs isolated, importing nothing from the
    # tree (R11 R3).
    tab="$(printf '\t')"
    git ls-files -s -z | while IFS= read -r -d '' rec; do
      case "$rec" in "160000 "*) printf '%s\n' "${rec#*"$tab"}" ;; esac
    done > "$tmp/gitlinks"
    { grep -xF -f "$tmp/gitlinks" "$tmp/registered" 2>/dev/null; printf '%s\n' "$scratch"; } > "$tmp/prune"  # (R10 D1)
    while IFS= read -r id; do git cat-file -s "$id"; done < "$tmp/blobs.txt" > "$tmp/sizes"
    walk="$(cat <<'PY'
import hashlib, os, stat, sys
prune = {os.fsencode(l.rstrip("\n")) for l in open(sys.argv[1], encoding="utf-8", errors="surrogateescape") if l.strip()}
ids = {l.strip() for l in open(sys.argv[2]) if l.strip()}
sizes = {int(l) for l in open(sys.argv[3]) if l.strip()}
algo = sys.argv[4]
ignored = {x for x in open(sys.argv[5], "rb").read().split(b"\0") if x}
quotes = [l.rstrip(b"\n") for l in sys.stdin.buffer if l.strip()]
out = sys.stdout.buffer
def emit(kind, path):
    out.write(kind + b"\0" + path + b"\0")
level = [b""]
while level:
    dirs = []
    for d in level:
        try:
            names = os.listdir(d or b".")
        except OSError:
            emit(b"unsearchable", d)
            continue
        for n in names:
            p = d + b"/" + n if d else n
            if n.lower() == b".git":
                if d:
                    emit(b"gitname", p)
                continue
            st = os.lstat(p)
            if stat.S_ISDIR(st.st_mode):
                if p not in prune and p + b"/" not in ignored:
                    dirs.append(p)
            elif stat.S_ISREG(st.st_mode) and p not in ignored:
                try:
                    with open(p, "rb") as f:
                        data = f.read()
                except OSError:
                    emit(b"unreadable", p)
                    continue
                if len(data) in sizes and hashlib.new(algo, b"blob %d\0" % len(data) + data).hexdigest() in ids:
                    emit(b"copy", p)
                elif any(q in data for q in quotes):
                    emit(b"quote", p)
    level = dirs
PY
)"
    quotes | env -u GIT_DIR -u GIT_WORK_TREE python3 -I -c "$walk" "$tmp/prune" "$tmp/blobs.txt" "$tmp/sizes" \
      "$(git rev-parse --show-object-format 2>/dev/null || echo sha1)" "$tmp/ignored" > "$tmp/walk" 2>/dev/null
    walked=${PIPESTATUS[1]}
    [ "$walked" -eq 0 ] || note "the working tree could not be walked (exit $walked) — a case's copy cannot be ruled out"
    while IFS= read -r -d '' kind && IFS= read -r -d '' at; do
      case "$kind" in
        copy) note "a sealed case's text is in the working tree at '$at'" ;;
        quote) note "'$at' quotes a line of a sealed case — the set is no longer unseen" ;;
        gitname) note "'$at' is an entry named .git — git lists and searches nothing in it, so a case's copy there cannot be ruled out; rename or remove it" ;;
        unreadable) note "'$at' cannot be read, so a case's copy there cannot be ruled out" ;;
        unsearchable) note "'$at' cannot be searched, so a case's copy there cannot be ruled out" ;;
      esac
    done < "$tmp/walk"
    # The index, which a commit records: a long line of a case in any staged file, the manifest included, found by the
    # name of the file that holds it; a search that cannot run refuses (review R11 D1).
    if quotes | grep -q .; then
      git grep -z -l --cached -F -f <(quotes) > "$tmp/staged-quotes" 2>/dev/null; searched=$?
      [ "$searched" -le 1 ] || note "git grep could not search the index for a case's line (exit $searched) — a quote cannot be ruled out"
      while IFS= read -r -d '' at; do
        note "'$at' quotes a line of a sealed case — the set is no longer unseen"
      done < "$tmp/staged-quotes"
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
  # Content and paths are searched, outside the sealed directory, whose manifest holds every name. The index is
  # searched as well as the disk, since a commit records what is staged (review R9 D5), and this
  # script with every other file: it names only its synthetic arm cases, so a case's name written
  # here is refused as anywhere (review R9 D6).
  # git grep reads the index's copy of a file marked assume-unchanged or skip-worktree, so those files' disk copies are
  # searched on their own (review R10 D2); and a tracked path that names a case is refused as its content would be
  # (R10 R3).
  git ls-files -z -v -- . ":(exclude)$DIR" 2>/dev/null | while IFS= read -r -d '' rec; do
    case "${rec%% *}" in [a-z]|S) [ -f "${rec#* }" ] && printf '%s\0' "${rec#* }" ;; esac
  done > "$tmp/flagged"
  while IFS= read -r slug; do
    [ -n "$slug" ] || continue
    git grep -z -l --fixed-strings -- "$slug" -- . ":(exclude)$DIR" > "$tmp/named-disk" 2>/dev/null; on_disk=$?
    git grep -z -l --cached --fixed-strings -- "$slug" -- . ":(exclude)$DIR" > "$tmp/named-index" 2>/dev/null; staged=$?
    [ "$on_disk" -le 1 ] && [ "$staged" -le 1 ] ||
      note "git grep could not search for a case's name (exit $on_disk, $staged) — a name cannot be ruled out (review R11 D1)"
    hits="$({ cat "$tmp/named-disk" "$tmp/named-index"
              [ -s "$tmp/flagged" ] && xargs -0 grep -l -Z -F -- "$slug" < "$tmp/flagged"
              git ls-files -z -- . ":(exclude)$DIR" | tr '\0' '\n' | grep -F -- "$slug" | tr '\n' '\0'; } \
            2>/dev/null | sort -zu | tr '\0' '\n' || true)"  # paths as their files are named (review R6 D1)
    if [ -n "$hits" ]; then
      note "sealed case '$slug' is named outside $DIR — the set is no longer unseen"
      printf '%s\n' "$hits" | sed 's/^/    /' >&2
    fi
  done < "$tmp/slugs.txt"
fi

exit $fail
