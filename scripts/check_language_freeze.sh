#!/usr/bin/env bash
# scripts/check_language_freeze.sh — LANGUAGE-FREEZE: a frozen construct of `eadl/1` cannot change
# without a migration note.
#
# ⭐ WHY THIS EXISTS. `ROADMAP.md` §15 requires that "any changed behavior must be explicit", and §12
# M1's exit gate asks for a frozen compatibility baseline. `M1.13.4.5` wrote the baseline —
# `docs/semantics/BASELINE.txt`, one digest per frozen construct, derived rather than typed. A baseline
# nothing compares is a record, not a freeze, and this is the comparison.
#
# TWO LEGS, because one has a hole the other closes:
#   A. INTEGRITY    — the tracked baseline agrees with a fresh run over the working tree. This is the leg
#                     that catches a construct edited and the baseline left alone.
#   B. EXPLICITNESS — the tracked baseline agrees with `HEAD`'s, or a **pending** migration note names
#                     every construct the amendment moves. This is the leg that catches the other
#                     direction: a baseline regenerated because a construct moved, with nothing written
#                     down. Without it the first legitimate language change is impossible and the gate
#                     gets waived, which is how gates die.
#   C. SPENT NOTES  — a note that `HEAD` already carries as `pending` has landed its movement, so it covers
#                     nothing and must be flipped to `applied` now. Without it a pending note is a
#                     permission that stays open: `M1.28.2`'s stayed pending for five commits, and any
#                     movement of its five constructs in that time would have been covered silently.
#
# ⛔ WHAT A PENDING NOTE IS, since leg B was inert until `PROGRAM.27`: a file in the notes directory with a
# line that is **exactly** `- status: pending` (surrounding blanks aside). A status holds one value, so the
# rule is anchored at both ends — and that is a rule rather than a filename, because the directory's own
# README documents the form with `- status: pending | applied` and used to be read as a pending note that,
# through `constructs: … — or: all`, covered every construct there is. `constructs:` is a comma-separated
# list compared **exactly**, and `all` counts only as the whole value.
#
# ⛔ ANY MOVEMENT NEEDS A NOTE, INCLUDING A CORRECTION. A note may say "correction: the specification was
# wrong and no description changes meaning" — that is still explicit, which is the whole requirement, and
# it costs one paragraph. A gate that tried to tell a bug fix from a language change would need judgement
# it cannot have, so it does not try.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: a digest proves a construct *moved*. It cannot prove the
# note covering the movement is correct, or complete, or that every description it invalidates was found.
# That residue is review, exactly as BOOK-ANCHORS states its own. Nor does the baseline see a corpus
# file's comment header, because canonical form carries no comment — the headers that carry data are
# pinned by `corpus.rs` and `reference.rs` instead.
#
# THE NOTE FORM, the workflow, and both limits again are in `docs/semantics/migrations/README.md`.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic; read-only against the tracked
# tree. Scratch lives under `target/doctrine_scratch/`, on the repository's own volume.
# `--self-test` runs the RED arms; three of them override the paths below through the environment seams,
# and one mutates a real description and restores it byte-identically.
set -uo pipefail
# Captured BEFORE any `cd`, and made absolute: an arm invokes this script by path, and a relative `$0`
# stops resolving the moment the working directory is not the caller's. That is the false green
# `docs/knowledge/verify-the-mutation-applied.md` records — arms that scored passes on `exit 127`
# because the check was never found.
INVOCATION_PWD="$PWD"
SELF="$0"
case "$SELF" in
  /*) ;;
  *) SELF="$INVOCATION_PWD/$SELF" ;;
esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

BASELINE="${LANGUAGE_FREEZE_BASELINE:-docs/semantics/BASELINE.txt}"
NOTES="${LANGUAGE_FREEZE_NOTES:-docs/semantics/migrations}"
SCRATCH="target/doctrine_scratch/language_freeze"

fail=0
note() { printf 'LANGUAGE-FREEZE: %s\n' "$1" >&2; fail=$((fail + 1)); }

# ── the classifier: one comparator, in one language, shared with the arms ─────────────────────────
# The digests are computed in shell (`scripts/language_baseline.sh`) because the workspace carries zero
# dependencies and so has no hasher; the *classification* is Rust, because a moved digest, an added
# construct and a removed one are three different claims and a note has to answer to the right one. A
# second classifier here would be a second thing that can disagree about what moved.
classify() { # $1 = fresh file, $2 = tracked file; stdout = one line per difference
  cargo run -q -p eadl-front --example language_freeze -- --diff "$1" "$2" 2>/dev/null
}

remedy() { # $1 = the classification report, already computed
  cat >&2 <<TEXT
  what moved:
$(printf '%s\n' "$1" | sed 's/^/    /')
  to see it raw:
    scripts/language_baseline.sh --print | diff $BASELINE -
  to cover it: write a note in $NOTES/ — the form and the workflow are in
    $NOTES/README.md — naming every construct above, with \`status: pending\`, and then
    regenerate with: scripts/language_baseline.sh --emit
TEXT
}

# ── leg A: integrity — does the tracked baseline agree with the working tree? ──────────────────────
leg_integrity() {
  local fresh="$1"
  if [ ! -f "$BASELINE" ]; then
    note "$BASELINE does not exist. The baseline is generated, not written:
  scripts/language_baseline.sh --emit"
    return
  fi
  # ⛔ The status is captured explicitly. `if cmd; then … fi` with no else branch returns **0** in bash,
  # so reading `$?` after it reports the `if` and not the classifier — a leg that would have taken the
  # "not a readable baseline" branch for every real movement.
  local report rc
  report="$(classify "$fresh" "$BASELINE")"; rc=$?
  case "$rc" in
    0) return ;;
    1) note "a frozen construct moved and the baseline does not record it"
       printf '%s\n' "$report" | sed 's/^/    /' >&2
       remedy "$report" ;;
    *) note "$BASELINE or the fresh baseline is not a readable baseline file, so nothing can be
  compared — that is a defect in the file, not a movement in the language" ;;
  esac
}

# ── leg B: explicitness — was the baseline amended without a note? ─────────────────────────────────
leg_explicitness() {
  local head_file="$1"
  if [ ! -s "$head_file" ]; then
    # No prior baseline to differ from: the commit that first writes it. That is not a waiver, it is the
    # one moment when there is nothing to be explicit about, and it cannot recur.
    return
  fi
  local report rc
  report="$(classify "$BASELINE" "$head_file")"; rc=$?
  if [ "$rc" -ne 1 ]; then
    # 0 = the baseline was not amended, so there is nothing to be explicit about. Anything else is a
    # file nobody can read, which leg A already reports; repeating it here would double-count one defect.
    return
  fi
  local uncovered
  uncovered="$(uncovered_by_notes "$report")"
  if [ -z "$uncovered" ]; then
    return
  fi
  note "the baseline was amended and no pending migration note covers it"
  printf '%s\n' "$uncovered" | sed 's/^/    /' >&2
  cat >&2 <<TEXT
  an amendment is an explicit act, never a side effect: a note with \`status: pending\` in
  $NOTES/ has to name every construct above (or say \`constructs: all\` and explain them), and
  only then is regenerating the baseline the right next step. The form is in
  $NOTES/README.md.
TEXT
}

# Which constructs in a classification report no pending note names.
uncovered_by_notes() {
  local report="$1" id covered=""
  local -a ids=()
  while IFS= read -r line; do
    [ -n "$line" ] || continue
    # The classifier prints `<kind>\t<id> <rest>` per difference and then one summary line with no tab.
    # ⛔ The summary line is skipped by *shape*, not by wording: reading it as a difference produced an
    # id of `1`, which no note could name, so a covered movement looked uncovered. Measured by the arm
    # that failed on it rather than reasoned about.
    case "$line" in
      *"$(printf '\t')"*) ;;
      *) continue ;;
    esac
    id="$(printf '%s' "$line" | cut -f2- | cut -d' ' -f1)"
    [ -n "$id" ] && ids+=("$id")
  done <<< "$report"
  [ ${#ids[@]} -eq 0 ] && return 0

  local -a notes=()
  while IFS= read -r candidate; do
    [ -n "$candidate" ] || continue
    # A spent note covers nothing; leg C reports it.
    spent "$candidate" && continue
    notes+=("$candidate")
  done < <(pending_notes)

  for id in "${ids[@]}"; do
    covered=""
    for candidate in ${notes+"${notes[@]}"}; do
      if names_construct "$candidate" "$id"; then covered=yes; break; fi
    done
    [ -n "$covered" ] || printf '%s\n' "$id"
  done
}

# Every pending note, by the anchored rule above.
pending_notes() {
  grep -rlE "$PENDING_LINE" "$NOTES" 2>/dev/null
}
PENDING_LINE='^- status:[[:space:]]*pending[[:space:]]*$'

# Whether a pending note was already pending at `HEAD` — committed pending, so its movement has landed.
# An arm supplies `HEAD`'s notes as a directory; the real run reads them from git.
spent() {
  local file="$1" relative="${1#"$NOTES"/}"
  if [ -n "${LANGUAGE_FREEZE_HEAD_NOTES:-}" ]; then
    grep -qE "$PENDING_LINE" "$LANGUAGE_FREEZE_HEAD_NOTES/$relative" 2>/dev/null
  else
    git show "HEAD:$file" 2>/dev/null | grep -qE "$PENDING_LINE"
  fi
}

# Whether a note names a construct, or claims all of them. ⛔ Exact comparison of list items: the
# substring test this replaced let `all` anywhere in the line claim every construct, and let a note naming
# `…#ebnf-v2` cover a movement of `…#ebnf`.
names_construct() {
  local file="$1" id="$2" value item
  local -a items=()
  value="$(grep -m1 '^- constructs:' "$file" 2>/dev/null)" || return 1
  value="$(printf '%s' "${value#- constructs:}" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
  [ "$value" = "all" ] && return 0
  IFS=',' read -r -a items <<< "$value"
  for item in ${items+"${items[@]}"}; do
    item="$(printf '%s' "$item" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
    [ "$item" = "$id" ] && return 0
  done
  return 1
}

# ── leg C: spent notes — is a note that landed its movement still claiming to be pending? ─────────────
leg_spent() {
  local candidate
  while IFS= read -r candidate; do
    [ -n "$candidate" ] || continue
    spent "$candidate" || continue
    note "$candidate is still \`status: pending\` and HEAD already carries it that way"
    cat >&2 <<TEXT
  a note is pending only in the commit that lands its movement; after that it is a record, and a pending
  one is a permission that stays open — it would cover a later movement of the same constructs with
  nothing written down about it. Flip it now: \`- status: applied\`. The lifecycle is in
  $NOTES/README.md.
TEXT
  done < <(pending_notes)
}

# ── RED arms ─────────────────────────────────────────────────────────────────────────────────────
# Each arm states its mutation, its oracle, and restores what it touched. ⛔ An arm whose oracle is
# "not zero" cannot tell a refusal from a command that never ran, which is the false green
# `docs/knowledge/verify-the-mutation-applied.md` exists about — so every arm here requires this check's
# own word in the output as well as the exit code.
self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  rm -rf "$work"; mkdir -p "$work"
  cp "$BASELINE" "$work/tracked.txt" || { echo "SELF-TEST: cannot copy the baseline" >&2; return 1; }

  # One arm: run this check with overridden paths and require the verdict, the check's own word, and —
  # for a refusal — the construct or note the refusal is ABOUT.
  # ⛔ `PROGRAM.27`: the third requirement is new, and two arms failed it the day it was added. Their
  # fixture was unsorted, the classifier refused the whole file as unreadable, and "a construct nobody
  # declared is refused" passed on a refusal that never classified a construct. An exit code and a word
  # prove the check refused; only the subject proves it refused for the reason the arm is named after.
  arm() { # $1 = name, $2 = expected rc, $3 = text a refusal must carry ("" for a pass), $4... = env
    local name="$1" want="$2" must="$3"; shift 3
    arms=$((arms + 1))
    local out rc
    out="$(env "$@" LANGUAGE_FREEZE_SELFTEST=1 bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2
      printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2
      return
    fi
    if [ "$want" -ne 0 ] && ! printf '%s' "$out" | grep -q 'LANGUAGE-FREEZE'; then
      echo "SELF-TEST: $name — exit $rc but the check never spoke, so the refusal is not this check's" >&2
      return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — refused, but not about \`$must\`, so it refused for another reason:" >&2
      printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2
      return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }

  # Fixtures: a baseline with one digest moved, one row dropped, one row added.
  python3 - "$work" <<'PY'
import pathlib, sys
work = pathlib.Path(sys.argv[1])
rows = [l for l in (work / "tracked.txt").read_text().split("\n") if l and not l.startswith("#")]
def write(name, lines):
    (work / name).write_text("# arm fixture\n" + "\n".join(lines) + "\n")
# ⛔ Every fixture is written SORTED by id, as the generator writes the real file: an unsorted fixture is
# refused whole as unreadable, and an arm expecting a refusal then passes without classifying anything.
def by_id(rows):
    return sorted(rows, key=lambda row: row.split("  ", 1)[1])
moved, dropped = [], []
for row in rows:
    digest, cid = row.split("  ", 1)
    moved.append(("0" * 64 + "  " + cid) if cid.endswith("#ebnf") else row)
    if cid == "suite/examples/s0-heartbeat/system.eadl":
        continue
    dropped.append(row)
write("moved.txt", by_id(moved))
write("dropped.txt", by_id(dropped + ["f" * 64 + "  suite/docs/semantics/cases/new-case.eadl"]))
write("added.txt", by_id(rows + ["e" * 64 + "  suite/__language-freeze-self-test__.eadl"]))
PY
  [ -f "$work/moved.txt" ] || { echo "SELF-TEST: the fixtures were not written" >&2; return 1; }

  # ── leg A: a construct moved, a construct appeared, a construct vanished ──
  arm "a moved digest is refused" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt"
  arm "a construct the baseline no longer freezes is refused" 1 "suite/examples/s0-heartbeat/system.eadl" \
    "LANGUAGE_FREEZE_BASELINE=$work/dropped.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt"
  arm "a construct nobody declared is refused" 1 "suite/docs/semantics/cases/new-case.eadl" \
    "LANGUAGE_FREEZE_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_FRESH=$work/dropped.txt"
  arm "an agreeing baseline passes" 0 "" \
    "LANGUAGE_FREEZE_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt"

  # ── leg B: an amendment with no note, with a note, and with a spent note ──
  mkdir -p "$work/notes"
  arm "an amended baseline with no note is refused" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"
  cat > "$work/notes/pending.md" <<'MD'
# Arm fixture

- status: pending
- constructs: docs/semantics/grammar.md#ebnf
MD
  arm "an amended baseline a pending note names passes" 0 "" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"
  sed -i.bak 's/^- status: pending/- status: applied/' "$work/notes/pending.md" && rm -f "$work/notes/pending.md.bak"
  arm "an applied note cannot cover a later movement" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"
  cat > "$work/notes/other.md" <<'MD'
# Arm fixture

- status: pending
- constructs: suite/examples/periodic-three/system.eadl
MD
  arm "a pending note for a different construct does not cover this one" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"

  # ── PROGRAM.27: what a pending note is, and when it stops being one ──
  # ⛔ The nine arms above all point at a scratch notes directory holding only fixtures, which is why none
  # of them saw the directory's own README count as a pending note covering everything. These arms feed
  # the check the files that actually broke it — the deployed README, byte for byte — and one runs against
  # the deployed notes directory itself.
  mkdir -p "$work/readme-notes" "$work/exact-notes" "$work/all-notes" "$work/spent-notes" "$work/spent-head"
  cp docs/semantics/migrations/README.md "$work/readme-notes/README.md"
  arm "the notes directory's own README covers nothing" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/readme-notes" \
    "LANGUAGE_FREEZE_HEAD_NOTES=$work/spent-head"
  mkdir -p "$work/status-notes"
  cat > "$work/status-notes/template.md" <<'MD'
# Arm fixture — a status line that is not exactly `pending`, over a construct it does name exactly

- status: pending | applied
- constructs: docs/semantics/grammar.md#ebnf
MD
  arm "a status that is not exactly 'pending' makes no pending note" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/status-notes" \
    "LANGUAGE_FREEZE_HEAD_NOTES=$work/spent-head"
  cat > "$work/exact-notes/near-miss.md" <<'MD'
# Arm fixture — two near misses: a longer id, and `all` inside a word

- status: pending
- constructs: docs/semantics/grammar.md#ebnf-v2, suite/docs/semantics/cases/install.eadl
MD
  arm "a construct is named exactly, and 'all' inside a word claims nothing" 1 "docs/semantics/grammar.md#ebnf" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/exact-notes" \
    "LANGUAGE_FREEZE_HEAD_NOTES=$work/spent-head"
  cat > "$work/all-notes/all.md" <<'MD'
# Arm fixture — `all` as the whole value, surrounded by blanks

- status: pending
- constructs:   all
MD
  arm "'constructs: all' as the whole value covers every construct" 0 "" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/all-notes" \
    "LANGUAGE_FREEZE_HEAD_NOTES=$work/spent-head"
  cat > "$work/spent-notes/pending.md" <<'MD'
# Arm fixture — committed as pending, so its movement has already landed

- status: pending
- constructs: docs/semantics/grammar.md#ebnf
MD
  cp "$work/spent-notes/pending.md" "$work/spent-head/pending.md"
  arm "a note HEAD already carries as pending is spent and covers nothing" 1 "spent-notes/pending.md is still" \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/spent-notes" \
    "LANGUAGE_FREEZE_HEAD_NOTES=$work/spent-head"
  arm "a spent note is refused even when nothing moved" 1 "spent-notes/pending.md is still" \
    "LANGUAGE_FREEZE_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/spent-notes" \
    "LANGUAGE_FREEZE_HEAD_NOTES=$work/spent-head"
  rm "$work/spent-head/pending.md"

  # The deployed notes directory, not a copy of it: an amendment that adds a construct no real note could
  # name must stay uncovered. ⚠️ It would go green only if a real note in flight says `constructs: all`,
  # which is then exactly what it should report.
  arm "the deployed notes directory leaves an unnamed amendment uncovered" 1 "suite/__language-freeze-self-test__.eadl" \
    "LANGUAGE_FREEZE_BASELINE=$work/added.txt" "LANGUAGE_FREEZE_FRESH=$work/added.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt"

  # ── the arm against the real tree: a description edited, and restored ──
  arms=$((arms + 1))
  local victim="docs/semantics/cases/positive-minimal-system.eadl"
  local backup="$work/victim.bak"
  cp "$victim" "$backup"
  # Restore on ANY exit from this arm, including a failure: an arm that leaves the tree dirty has traded
  # a measurement for a defect.
  ( trap 'cp "$backup" "$victim"' EXIT
    printf '\n(defblock extra (offers x))\n' >> "$victim"
    out="$(bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne 1 ] || ! printf '%s' "$out" | grep -q 'positive-minimal-system'; then
      echo "SELF-TEST: editing a frozen description did not fire the gate on it (rc=$rc)" >&2
      printf '%s\n' "$out" | head -4 | sed 's/^/    /' >&2
      exit 1
    fi
    exit 0 )
  if [ $? -eq 0 ] && diff -q "$backup" "$victim" >/dev/null; then
    ok=$((ok + 1)); echo "  ✅ editing a frozen description fires the gate, and the file is restored"
  else
    echo "SELF-TEST: the real-tree arm failed or left $victim modified" >&2
    cp "$backup" "$victim"
  fi

  echo "language-freeze self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

# ── the run ───────────────────────────────────────────────────────────────────────────────────────
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

mkdir -p "$SCRATCH"

# The fresh baseline: computed, unless an arm supplied one.
FRESH="${LANGUAGE_FREEZE_FRESH:-$SCRATCH/fresh.txt}"
if [ -z "${LANGUAGE_FREEZE_FRESH:-}" ]; then
  if ! scripts/language_baseline.sh --print > "$FRESH"; then
    note "the baseline instrument failed, so there is nothing to compare against"
    note "a construct that cannot be read is a defect, not a construct to skip"
    exit 1
  fi
fi

# `HEAD`'s baseline, for leg B. An absent one is the commit that first wrote it.
HEAD_FILE="${LANGUAGE_FREEZE_HEAD_BASELINE:-$SCRATCH/head.txt}"
if [ -z "${LANGUAGE_FREEZE_HEAD_BASELINE:-}" ]; then
  git show "HEAD:$BASELINE" > "$HEAD_FILE" 2>/dev/null || : > "$HEAD_FILE"
fi

leg_integrity "$FRESH"
leg_explicitness "$HEAD_FILE"
leg_spent

if [ "$fail" -ne 0 ]; then
  echo "LANGUAGE-FREEZE: $fail breach(es) of the eadl/1 freeze — each is explained above" >&2
  exit 1
fi
echo "language-freeze: OK ($(grep -vc '^#' "$BASELINE" | tr -d ' ') frozen construct(s) agree with the working tree)"
exit 0
