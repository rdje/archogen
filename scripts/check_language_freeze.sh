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
note() { printf 'LANGUAGE-FREEZE: %s\n' "$1" >&2; fail=1; }

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
    [ -n "$candidate" ] && notes+=("$candidate")
  done < <(grep -rl '^- status:[[:space:]]*pending' "$NOTES" 2>/dev/null)

  for id in "${ids[@]}"; do
    covered=""
    for candidate in ${notes+"${notes[@]}"}; do
      if names_construct "$candidate" "$id"; then covered=yes; break; fi
    done
    [ -n "$covered" ] || printf '%s\n' "$id"
  done
}

# Whether a note names a construct, or claims all of them.
names_construct() {
  local file="$1" id="$2" line
  line="$(grep '^- constructs:' "$file" 2>/dev/null | head -1)"
  [ -n "$line" ] || return 1
  case "$line" in
    *all*) return 0 ;;
  esac
  case "$line" in
    *"$id"*) return 0 ;;
  esac
  return 1
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

  # One arm: run this check with overridden paths and require both the verdict and its own word.
  arm() { # $1 = name, $2 = expected rc, $3... = env assignments
    local name="$1" want="$2"; shift 2
    arms=$((arms + 1))
    local out rc
    out="$(env "$@" LANGUAGE_FREEZE_SELFTEST=1 bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2
      printf '%s\n' "$out" | head -4 | sed 's/^/    /' >&2
      return
    fi
    if [ "$want" -ne 0 ] && ! printf '%s' "$out" | grep -q 'LANGUAGE-FREEZE'; then
      echo "SELF-TEST: $name — exit $rc but the check never spoke, so the refusal is not this check's" >&2
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
moved, dropped, added = [], [], []
for row in rows:
    digest, cid = row.split("  ", 1)
    moved.append(("0" * 64 + "  " + cid) if cid.endswith("#ebnf") else row)
    if cid == "suite/examples/s0-heartbeat/system.eadl":
        continue
    dropped.append(row)
write("moved.txt", moved)
write("dropped.txt", dropped + ["f" * 64 + "  suite/docs/semantics/cases/new-case.eadl"])
PY
  [ -f "$work/moved.txt" ] || { echo "SELF-TEST: the fixtures were not written" >&2; return 1; }

  # ── leg A: a construct moved, a construct appeared, a construct vanished ──
  arm "a moved digest is refused" 1 \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt"
  arm "a construct the baseline no longer freezes is refused" 1 \
    "LANGUAGE_FREEZE_BASELINE=$work/dropped.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt"
  arm "a construct nobody declared is refused" 1 \
    "LANGUAGE_FREEZE_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_FRESH=$work/dropped.txt"
  arm "an agreeing baseline passes" 0 \
    "LANGUAGE_FREEZE_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_FRESH=$work/tracked.txt"

  # ── leg B: an amendment with no note, with a note, and with a spent note ──
  mkdir -p "$work/notes"
  arm "an amended baseline with no note is refused" 1 \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"
  cat > "$work/notes/pending.md" <<'MD'
# Arm fixture

- status: pending
- constructs: docs/semantics/grammar.md#ebnf
MD
  arm "an amended baseline a pending note names passes" 0 \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"
  sed -i.bak 's/^- status: pending/- status: applied/' "$work/notes/pending.md" && rm -f "$work/notes/pending.md.bak"
  arm "an applied note cannot cover a later movement" 1 \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"
  cat > "$work/notes/other.md" <<'MD'
# Arm fixture

- status: pending
- constructs: suite/examples/periodic-three/system.eadl
MD
  arm "a pending note for a different construct does not cover this one" 1 \
    "LANGUAGE_FREEZE_BASELINE=$work/moved.txt" "LANGUAGE_FREEZE_FRESH=$work/moved.txt" \
    "LANGUAGE_FREEZE_HEAD_BASELINE=$work/tracked.txt" "LANGUAGE_FREEZE_NOTES=$work/notes"

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

if [ "$fail" -ne 0 ]; then
  echo "LANGUAGE-FREEZE: $fail breach(es) — a frozen construct of eadl/1 moved without a migration note" >&2
  exit 1
fi
echo "language-freeze: OK ($(grep -vc '^#' "$BASELINE" | tr -d ' ') frozen construct(s) agree with the working tree)"
exit 0
