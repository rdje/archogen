#!/usr/bin/env bash
# TASK-ACCEPTANCE — a staged CODE change must be owned by a task-tree leaf that carries a ticked
# acceptance checklist, and each hard-gated box must be backed by EVIDENCE INSIDE ITS OWN BULLET.
#
# ⭐ THE DISCIPLINE, stated with no project's nouns: a change lands with (a) the CAUSE located,
# (b) the EFFECT measured, and (c) a statement that nothing regressed — each backed by output
# from a tool that was actually run, not by prose. "I fixed it" is a claim; a pasted verdict is
# an artifact someone else can re-run.
#
# ⭐⭐ WHY BOX-SCOPING IS THE SOUNDNESS PROPERTY, not a stylistic nicety. Three leakage holes were
# MEASURED on the projects this was distilled from, and each made the check weaker than it reads:
#   (1) cross-FILE leakage — the greps ran over ALL staged task files, so a co-staged, unrelated
#       tree file could supply the signature for a leaf that carried none. That is exactly how
#       one leaf passed: on tokens belonging to a different tree.
#   (2) incidental-PROSE leakage — a whole-file grep matched a token mentioned anywhere in the
#       leaf rather than inside the ticked box it was supposed to back.
#   (3) cross-LEAF leakage — one awk ran over the whole tree FILE and stopped at the first box it
#       found, so a file holding N leaves verified exactly one of them, whichever came first.
#       Measured: a commit staging five source files printed OK having read a checklist written
#       thirteen days earlier for a different leaf, and a mutation of the committing leaf's own
#       box left the verdict byte-identical. Closed by leaf-scoping, below.
#   ⇒ the signature must sit in the SAME BULLET, of the SAME LEAF, that owns the change. Anything
#     looser is a check that reports green on evidence it never actually tied to a claim.
#
# ⭐⭐ LEAF-SCOPING, AND WHY IT FAILS CLOSED. A tree file holds many leaves, so "the leaf that owns
# this change" has to be *identified*, not found by scanning. There is no signal for it inside the
# staged paths: a commit legitimately touches its own leaf, its parent, the frontier table and any
# leaf it routes a finding to, and measuring seven real code commits showed the set of leaf sections
# a commit touches names the right leaf once in seven. So the owner is taken from the commit message,
# which is the one place the author states it — see `.doctrine/commit_message_file` below.
# ⛔ When the owner cannot be identified the check REFUSES rather than falling back to the first
# checklist in the file. That fallback is hole (3): it produces a green verdict about a leaf nobody
# claimed, which is worse than no verdict, because the author reads it as protection. A gate that
# cannot tell what it is looking at must say so.
#
# ⚠️ HONEST LIMITS, stated rather than hidden:
#   • This verifies a box was TICKED and that tool-shaped output sits inside it. It cannot verify
#     the output is true. A ticked box is leg 1 (presence); the un-fakeable leg is re-running the
#     cited command in CI. Do not describe this check as proving correctness — it proves the author
#     cited something re-runnable.
#   • ONE commit names ONE owning leaf, which is what the subject convention already assumes. A
#     commit that genuinely closes work on two leaves is two commits; this check verifies the one
#     that is named and says nothing about the other.
#   • The message file is read as it stands. A stale one left over from a previous commit names that
#     commit's leaf, which is caught only if that leaf is not in any staged tree file — so clear it
#     after committing, as the commit workflow requires.
#
# ── PROJECT SEAMS (this is what keeps the check neutral) ─────────────────────────────────────
#   .doctrine/code_paths.txt        one glob per line — what counts as a CODE change here.
#                                   Absent -> the built-in default below (Rust workspace shape).
#   .doctrine/evidence_tokens.txt   one regular expression per line — YOUR tools' output signatures,
#                                   ADDED to the universal defaults. Absent -> defaults only.
#   .doctrine/commit_message_file   one line: the path, relative to the repository root, of the file
#                                   holding the pending commit message. Absent -> the neutral default
#                                   below. Its SUBJECT is expected to name the owning leaf as
#                                   `(leaf <ID>)`; the ID shape itself is not assumed.
#   TASK_ACCEPTANCE_LEAF=<ID>       names the owning leaf directly, overriding the message file —
#                                   for a commit made without one (an IDE dialog, `git commit -m`).
# ⛔ Never hardcode one project's tool names or file names in this file. That is the difference
#   between a portable standard and a fork of somebody else's workflow.
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md §4): exit code is the verdict; explains on stderr;
# deterministic; read-only; staged-scope-aware; path-agnostic.
#
# `--self-test` runs the RED arms, each in a throwaway repository so the real index is never touched.
set -uo pipefail
# Captured BEFORE any `cd`: `--self-test` resolves `$0` against the directory the caller stood in, and
# every RED arm then runs the check from inside a throwaway repository.
INVOCATION_PWD="$PWD"
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# ── what counts as a code change ─────────────────────────────────────────────────────────────
default_code_re='(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$'
if [ -f .doctrine/code_paths.txt ]; then
  code_re="$(grep -vE '^\s*(#|$)' .doctrine/code_paths.txt | paste -sd'|' -)"
  [ -n "$code_re" ] || code_re="$default_code_re"
else
  code_re="$default_code_re"
fi

# ── where the pending commit message lives, and how it names the owning leaf ─────────────────
default_msgfile='git_message_brief.txt'
if [ -f .doctrine/commit_message_file ]; then
  msgfile="$(grep -vE '^\s*(#|$)' .doctrine/commit_message_file | head -1)"
  [ -n "$msgfile" ] || msgfile="$default_msgfile"
else
  msgfile="$default_msgfile"
fi

# ── evidence signatures ──────────────────────────────────────────────────────────────────────
# Universal defaults. Every entry is either standard Rust/Cargo tooling (this template is a Rust
# scaffold, so these apply to ANY consumer) or plain build-flow forensics available in ANY
# project. ⛔ No entry may name a specific project's tool.
#
# ⚠️ PRICED AGAINST A REAL CORPUS, and the first cut was TOO NARROW — measured, not guessed.
# The first version of this list rejected a leaf whose boxes cited `awk version 20200816`,
# `probes: 3 pass / 6 fail` and `exit=0`: all genuinely tool-emitted, none matched. That is the
# failure mode where *a signature family that does not fit the real corpus becomes a gate authors
# learn to waive*. Generic result shapes (`exit=N`, `rc=N`, `N pass / N fail`, version banners)
# were added because they are what tools actually print — not to make the gate easier.
DEFAULT_SIG='error\[E[0-9]{4}\]|could not compile|clippy::[a-z_]{3,}|panicked at|assertion (failed|`)|test result: (ok|FAILED)|running [0-9]+ tests?|cargo (test|build|bench|flamegraph)|flamegraph|self-time|call-graph|/usr/bin/sample|\bspindump\b|\bperf (record|stat)\b|\bvalgrind\b|git (ls-files|log -S|log --all -S|rev-list|fsck|reflog|diff-tree|merge-base|cat-file|show )|\bshellcheck\b|bash -n |sh -n |make -n |make --dry-run|\bE2BIG\b|\bENOSPC\b|\bEACCES\b|\bARG_MAX\b|exit(ed)?[ =:](code )?[0-9]+|\brc=[0-9]+|PIPESTATUS|[0-9]+ (pass|passed|ok)[ ,/]+[0-9]+ (fail|failed)|version [0-9]{4,}|[0-9]+\.[0-9]+\.[0-9]+'
SIG="$DEFAULT_SIG"
if [ -f .doctrine/evidence_tokens.txt ]; then
  extra="$(grep -vE '^\s*(#|$)' .doctrine/evidence_tokens.txt | paste -sd'|' -)"
  [ -n "$extra" ] && SIG="$SIG|$extra"
fi

# ── one leaf's boxes, from a slice of a tree file ────────────────────────────────────────────
# $1 = file holding ONE leaf's section, $2 = leaf id (for messages).
# The three hard-gated boxes. FIX / REPRODUCE / LOCKSTEP are good practice but not blocked, so
# an honest author is never forced to invent evidence for a box that does not apply.
# Returns 0 if all three are ticked and evidence-backed; explains on stderr and returns 1 if not.
check_leaf_boxes() {
  slice="$1"; owner="$2"
  rc=0
  for spec in 'ROOT CAUSE:root.?cause' 'ADDRESSED:addressed' 'NO REGRESSION:no.?regress'; do
    label="${spec%%:*}"; kw="${spec#*:}"
    # A box's BULLET = the "- [x] ..." line plus its indented continuation lines, so wrapped
    # markdown still counts while a token elsewhere in the leaf does not.
    # ⛔ POSIX awk only — no `IGNORECASE`, which is a gawk extension that BSD awk (the default
    # on several platforms) silently IGNORES, so the box would never match and every leaf would
    # be reported as having no checklist at all. Case-folding is done with tolower(), which is
    # POSIX. A template must run on whatever awk the consumer has.
    awk -v kw="$kw" '
      BEGIN{ inbox=0 }
      {
        line = $0
        isbox = (line ~ /^[[:space:]]*-[[:space:]]*\[[xX ]\]/)
        if (isbox) {
          if (inbox) exit
          if (match(tolower(line), kw)) { inbox=1; print; next }
          next
        }
        if (inbox) {
          # A bullet continues through indented and blank lines; anything flush-left ends it.
          if (line ~ /^[[:space:]]+/ || line ~ /^[[:space:]]*$/) { print; next }
          exit
        }
      }
    ' "$slice" > "$slice.box"

    if [ ! -s "$slice.box" ]; then
      echo "TASK-ACCEPTANCE: leaf $owner has no '$label' box in its acceptance checklist." >&2
      rc=1; continue
    fi
    if ! head -1 "$slice.box" | grep -qE '\[[xX]\]'; then
      echo "TASK-ACCEPTANCE: leaf $owner — the '$label' box is present but NOT ticked." >&2
      rc=1; continue
    fi
    if ! grep -qE "$SIG" "$slice.box"; then
      {
        echo "TASK-ACCEPTANCE: leaf $owner — the '$label' box is ticked but carries no tool-output"
        echo "  evidence INSIDE ITS OWN BULLET. A tick is a claim; the box asks for output from a"
        echo "  command you ran. Add the invocation and its real output to that bullet, or declare"
        echo "  your project's own signatures in .doctrine/evidence_tokens.txt (one extended regular"
        echo "  expression per line)."
      } >&2
      rc=1
    fi
    rm -f "$slice.box"
  done
  return "$rc"
}

# ── slice one leaf's section out of a tree file ──────────────────────────────────────────────
# A leaf runs from its flush-left `- ID: `<id>`` line to the next such line or the next `## `
# heading. Exact string comparison, not a regular expression, so an id containing `.` cannot
# match a different leaf's.
slice_leaf() {
  awk -v want="- ID: \`$1\`" '
    BEGIN { inside = 0 }
    {
      if ($0 == want) { inside = 1; print; next }
      if (inside) {
        if ($0 ~ /^- ID: `/ || $0 ~ /^## /) exit
        print
      }
    }
  ' "$2"
}

# ── RED arms ─────────────────────────────────────────────────────────────────────────────────
if [ "${1:-}" = "--self-test" ]; then
  SELF="$(mktemp -d)"; trap 'rm -rf "$SELF"' EXIT
  arms=0; ok=0

  # One throwaway repository per arm, so an arm cannot inherit another's staging and so the real
  # index is never touched. `leaf1` is always complete; the arms vary what is named and what is not.
  new_repo() {
    repo="$SELF/$1"; mkdir -p "$repo/docs/tasks"
    ( cd "$repo" && git init -q . && git config user.email t@t && git config user.name t )
    cat > "$repo/docs/tasks/TREE.md" <<'MD'
# Tree

## Task Tree

- ID: `T.1`
  Status: `done`

  ### Acceptance Checklist

  - [x] **ROOT CAUSE (WHY + WHERE)** — cause located.
    ```text
    $ cargo test --all
    test result: ok. 12 passed; 0 failed
    ```
  - [x] **ADDRESSED (verified)** — effect measured.
    ```text
    exit=0
    ```
  - [x] **NO REGRESSION** — nothing broke.
    ```text
    test result: ok. 12 passed; 0 failed
    ```

- ID: `T.2`
  Status: `pending`
  Goal: a leaf that carries no acceptance checklist at all.
  Verification: `pending`
  Commit: `pending`

- ID: `T.3`
  Status: `pending`

  ### Acceptance Checklist

  - [ ] **ROOT CAUSE (WHY + WHERE)** — present but unticked.
  - [ ] **ADDRESSED (verified)** — present but unticked.
  - [ ] **NO REGRESSION** — present but unticked.

## Current Frontier

| Order | Leaf |
| --- | --- |
MD
    printf 'fn main() {}\n' > "$repo/change.rs"
    ( cd "$repo" && git add -A >/dev/null 2>&1 )
  }

  # arm <name> <leaf named in the brief, or "-"> <expected: pass|fail>
  #
  # ⛔ THE ORACLE IS EXACT, NOT "NON-ZERO" — and that is measured, not defensive. The first run of
  # these arms scored **four passes on `exit 127`**: the check could not be *found*, because `$0` was a
  # relative path and every arm `cd`s into a throwaway repository. A refusal and a failure to exec are
  # different claims, and an arm that accepts any non-zero exit cannot tell them apart, so it reports
  # green on a check that never ran — the false-green shape `verify-the-mutation-applied` exists for.
  # A `fail` arm therefore requires exit **1** AND the check's own prefix in its output; a `pass` arm
  # requires exit **0** AND the OK line.
  arm() {
    name="$1"; named="$2"; want="$3"
    arms=$((arms + 1))
    if [ "$named" = "-" ]; then
      rm -f "$repo/git_message_brief.txt"
    else
      printf 'WORK-0001 (leaf %s): a subject\n' "$named" > "$repo/git_message_brief.txt"
    fi
    out="$( cd "$repo" && "$OLDSELF" 2>&1 )"; rc=$?
    verdict=bad
    if [ "$want" = fail ]; then
      if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'TASK-ACCEPTANCE'; then verdict=good; fi
    else
      if [ "$rc" -eq 0 ] && printf '%s' "$out" | grep -q 'task-acceptance: OK'; then verdict=good; fi
    fi
    if [ "$verdict" = good ]; then
      ok=$((ok + 1))
      printf 'SELF-TEST: ✅ arm %s — %s, exit %s\n' "$arms ($name)" "$want" "$rc"
    else
      printf 'SELF-TEST: ❌ arm %s — wanted %s, got exit %s\n%s\n' "$arms ($name)" "$want" "$rc" \
        "$(printf '%s' "$out" | sed 's/^/    /')" >&2
    fi
  }

  # Absolute, and resolved from the INVOCATION directory rather than `$ROOT`: `$0` is relative to
  # wherever the caller stood, and every arm below runs the check from inside a throwaway repository.
  case "$0" in
    /*) OLDSELF="$0" ;;
    *)  OLDSELF="$INVOCATION_PWD/$0" ;;
  esac

  # Arm 1 — THE MEASURED DEFECT. A file whose FIRST leaf is complete and whose second carries no
  # checklist at all, with the change owned by the second. The unscoped check reads `T.1`'s boxes
  # and prints OK; leaf-scoping must refuse and name `T.2`.
  new_repo a1; arm "second leaf owns it, first is complete" "T.2" fail

  # Arm 2 — the same fixture, the complete leaf named. Proves arm 1 fails for the right reason and
  # not because the check now refuses everything.
  new_repo a2; arm "first leaf owns it and is complete" "T.1" pass

  # Arm 3 — a named leaf whose boxes exist but are unticked.
  new_repo a3; arm "named leaf carries unticked boxes" "T.3" fail

  # Arm 4 — no owner identifiable at all: no message file, no environment override. Must refuse
  # rather than fall back to the first checklist in the file, which is hole (3).
  new_repo a4; arm "no owner can be identified" "-" fail

  # Arm 5 — a stale message file naming a leaf no staged tree file contains.
  new_repo a5; arm "brief names a leaf that is not staged" "T.9" fail

  # Arm 6 — the environment override names the incomplete leaf, with a message file naming the
  # complete one. Proves the override wins, so a commit made without a message file is checkable.
  new_repo a6
  arms=$((arms + 1))
  printf 'WORK-0001 (leaf T.1): a subject\n' > "$repo/git_message_brief.txt"
  out="$( cd "$repo" && TASK_ACCEPTANCE_LEAF=T.2 "$OLDSELF" 2>&1 )"; rc=$?
  if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'T\.2'; then
    ok=$((ok + 1)); printf 'SELF-TEST: ✅ arm %s\n' "6 (env override beats the message file)"
  else
    printf 'SELF-TEST: ❌ arm %s — wanted exit 1 naming T.2, got exit %s\n%s\n' \
      "6 (env override beats the message file)" "$rc" \
      "$(printf '%s' "$out" | sed 's/^/    /')" >&2
  fi

  # Arm 7 — ⛔ TEMPLATE.md is the blank form authors COPY, so its boxes are deliberately unticked.
  # Staging it beside a real leaf must not make the check demand a ticked box in the template.
  new_repo a7
  mkdir -p "$repo/docs/tasks"
  printf -- '- ID: `T.9`\n\n  - [ ] **ROOT CAUSE (WHY + WHERE)** — blank on purpose.\n' \
    > "$repo/docs/tasks/TEMPLATE.md"
  ( cd "$repo" && git add -A >/dev/null 2>&1 )
  arm "TEMPLATE.md staged beside a complete leaf" "T.1" pass

  # Arm 8 — a code change with NO task-tree leaf staged at all.
  arms=$((arms + 1))
  new_repo a8
  ( cd "$repo" && git rm -q --cached docs/tasks/TREE.md >/dev/null 2>&1 )
  printf 'WORK-0001 (leaf T.1): a subject\n' > "$repo/git_message_brief.txt"
  out="$( cd "$repo" && "$OLDSELF" 2>&1 )"; rc=$?
  if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'NO owning task-tree leaf'; then
    ok=$((ok + 1)); printf 'SELF-TEST: ✅ arm %s\n' "8 (code with no leaf staged)"
  else
    printf 'SELF-TEST: ❌ arm %s — wanted exit 1 and the no-leaf refusal, got exit %s\n%s\n' \
      "8 (code with no leaf staged)" "$rc" "$(printf '%s' "$out" | sed 's/^/    /')" >&2
  fi

  # Arm 9 — the success message must name what it examined, so a green verdict is checkable.
  new_repo a9
  printf 'WORK-0001 (leaf T.1): a subject\n' > "$repo/git_message_brief.txt"
  out="$( cd "$repo" && "$OLDSELF" 2>&1 )"; rc=$?
  arms=$((arms + 1))
  if [ "$rc" -eq 0 ] && printf '%s' "$out" | grep -q 'leaf T\.1'; then
    ok=$((ok + 1)); printf 'SELF-TEST: ✅ arm %s\n' "9 (the OK message names the leaf it read)"
  else
    printf 'SELF-TEST: ❌ arm %s — exit %s\n%s\n' "9 (the OK message names the leaf it read)" "$rc" \
      "$(printf '%s' "$out" | sed 's/^/    /')" >&2
  fi

  # Arm 10 — a sealed leaf named as the owner (TASK-HISTORY, leaf `PROGRAM.32`). Its tree keeps only a two-line
  # stub; the refusal must say the leaf is closed and sealed, not that it lacks a checklist.
  new_repo a10
  printf -- '\n- ID: `T.7`\n  Status: `done` — sealed in [`TREE/T.7.md`](../task-history/TREE/T.7.md); commit `WORK-0007`\n' \
    >> "$repo/docs/tasks/TREE.md"
  ( cd "$repo" && git add -A >/dev/null 2>&1 )
  printf 'WORK-0001 (leaf T.7): a subject\n' > "$repo/git_message_brief.txt"
  out="$( cd "$repo" && "$OLDSELF" 2>&1 )"; rc=$?
  arms=$((arms + 1))
  if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'is closed and sealed'; then
    ok=$((ok + 1)); printf 'SELF-TEST: ✅ arm %s\n' "10 (a sealed owner is named as closed)"
  else
    printf 'SELF-TEST: ❌ arm %s — wanted exit 1 and the sealed refusal, got exit %s\n%s\n' \
      "10 (a sealed owner is named as closed)" "$rc" "$(printf '%s' "$out" | sed 's/^/    /')" >&2
  fi

  echo "task-acceptance self-test: $ok pass / $((arms - ok)) fail"
  [ "$ok" -eq "$arms" ] || exit 1
  exit 0
fi

staged="$(git diff --cached --name-only --diff-filter=ACM 2>/dev/null || true)"
[ -n "$staged" ] || exit 0

# grep a FILE, never `printf "$var" | grep -q`: under pipefail, grep -q exits at the first match
# and the producer takes SIGPIPE, so the pipeline reports FAILURE ON SUCCESS once the input is
# large. That failure mode is silent and, at a `|| continue`, fails OPEN.
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
printf '%s\n' "$staged" > "$tmp/staged.txt"

grep -E "$code_re" "$tmp/staged.txt" > "$tmp/code.txt" 2>/dev/null || true
[ -s "$tmp/code.txt" ] || exit 0          # pure-docs change: this doctrine does not govern it

# ⛔ TEMPLATE.md is the blank form authors COPY — its boxes are deliberately unticked, so
# treating it as a leaf makes the doctrine block every commit that edits the template. Found by
# this check refusing its own commit: a FALSE POSITIVE, unlike the two refusals before it, which
# were correct. The same exclusion exists in the layer-C check in this repo (INDEX/TEMPLATE).
grep -E '^docs/tasks/.*\.md$' "$tmp/staged.txt" | grep -vE '(^|/)TEMPLATE\.md$' \
  > "$tmp/leaves.txt" 2>/dev/null || true
if [ ! -s "$tmp/leaves.txt" ]; then
  {
    echo "TASK-ACCEPTANCE: a CODE change is staged but NO owning task-tree leaf (docs/tasks/*.md) is."
    echo "  staged code:"; sed 's/^/    /' "$tmp/code.txt"
    echo "  Stage the docs/tasks/<TREE>.md that owns this change, carrying the acceptance checklist."
  } >&2
  exit 1
fi

# ── identify the leaf that owns this change ──────────────────────────────────────────────────
# Not inferred from the staged paths: see LEAF-SCOPING in the header. A commit touches its own
# leaf, its parent, the frontier and anything it routes to, and measuring seven real code commits
# showed the touched sections name the right leaf once in seven.
owner=""; source_of_owner=""
if [ -n "${TASK_ACCEPTANCE_LEAF:-}" ]; then
  owner="$TASK_ACCEPTANCE_LEAF"; source_of_owner='the TASK_ACCEPTANCE_LEAF environment variable'
elif [ -s "$msgfile" ]; then
  owner="$(head -1 "$msgfile" \
    | sed -n 's/.*([[:space:]]*leaf[[:space:]][[:space:]]*\([^)]*\)).*/\1/p')"
  source_of_owner="the subject of $msgfile"
fi

if [ -z "$owner" ]; then
  {
    echo "TASK-ACCEPTANCE: a CODE change is staged but the check CANNOT TELL WHICH LEAF owns it."
    echo "  It does not guess. Falling back to the first checklist in the file is how this check"
    echo "  came to print OK having read a different leaf's boxes, and a green verdict about a leaf"
    echo "  nobody claimed is worse than no verdict: the author reads it as protection."
    echo "  Name the owning leaf either way:"
    echo "    • write the commit message to '$msgfile' with '(leaf <ID>)' in its SUBJECT, or"
    echo "    • TASK_ACCEPTANCE_LEAF=<ID> <your commit command>"
    echo "  tried: TASK_ACCEPTANCE_LEAF (unset), '$msgfile' ($([ -e "$msgfile" ] && echo 'exists but its subject names no leaf' || echo 'absent'))"
    echo "  staged code:"; sed 's/^/    /' "$tmp/code.txt"
    echo "  staged task trees:"; sed 's/^/    /' "$tmp/leaves.txt"
  } >&2
  exit 1
fi

# ── find the named leaf among the staged tree files, and check ITS boxes ─────────────────────
found=""
while IFS= read -r leaf; do
  [ -r "$leaf" ] || continue
  slice_leaf "$owner" "$leaf" > "$tmp/slice.md"
  if [ -s "$tmp/slice.md" ]; then found="$leaf"; break; fi
done < "$tmp/leaves.txt"

if [ -z "$found" ]; then
  {
    echo "TASK-ACCEPTANCE: the owning leaf is named \`$owner\` (from $source_of_owner) but NO"
    echo "  STAGED docs/tasks/*.md contains it. Either that leaf's tree file is not staged, or the"
    echo "  name is stale — a '$msgfile' left over from a previous commit names that commit's leaf,"
    echo "  which is why the workflow clears it after committing."
    echo "  staged task trees:"; sed 's/^/    /' "$tmp/leaves.txt"
  } >&2
  exit 1
fi

# ⛔ A sealed leaf is closed: its body lives in docs/task-history/, and the tree keeps only its two-line stub
# (TASK-HISTORY, leaf `PROGRAM.32`, docs/decisions/decision_task-tree-sealing.md). Say so, rather than report a stub as
# a leaf that lacks its checklist, which would send the author looking for boxes that were never meant to be there.
if sed -n '2p' "$tmp/slice.md" | grep -q '^  Status: `done` — sealed in \['; then
  {
    echo "TASK-ACCEPTANCE: the owning leaf \`$owner\` (from $source_of_owner) is closed and sealed: its tree keeps"
    echo "  only a stub, and its body is in $(sed -n '2p' "$tmp/slice.md" | sed 's/.*(\.\.\/\(task-history\/[^)]*\)).*/docs\/\1/')."
    echo "  A change needs an open leaf. Name one, or open a new leaf for this change, in its tree first."
  } >&2
  exit 1
fi

check_leaf_boxes "$tmp/slice.md" "$owner" || exit 1

# ⭐ The message states what was actually examined, because a success message that claims more than
# the check did is the defect this scoping exists to end.
echo "task-acceptance: OK (leaf $owner in $found — ROOT CAUSE, ADDRESSED and NO REGRESSION boxes ticked, each carrying tool output inside its own bullet)"
exit 0
