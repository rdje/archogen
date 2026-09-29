#!/usr/bin/env bash
# LS-005 re-measurement — does the guide still let a direct-entry reader build too early?
#
# This is NOT repro.sh. repro.sh freezes the original observation and answers "does the defect
# still reproduce?"; this instrument answers the question a consumer actually owes upstream after
# a published fix: "at THIS revision, is the reported defect gone?" It therefore checks the
# property LS-005 asked for, not the heading spellings that happened to exist when the report was
# written — the section it points at was renamed upstream between the report and the fix.
#
#   usage: remeasure.sh /path/to/linkedspec    measure the guide in a checkout
#          remeasure.sh --guide <file>         measure one guide file (what --self-test uses)
#          remeasure.sh --self-test            prove the verdict discriminates (4 arms)
#
# The three properties, all of which must hold for the defect to be gone:
#   P1 ORDER   the "add and pin" section states that preparation comes BEFORE metadata/build
#   P2 POINTER the same section names the preparation the reader has to complete
#   P3 TARGET  the guide really has that preparation section, so the pointer resolves
#
# ⭐ A trailing "checkout does not generate ..." sentence satisfies NONE of them. That sentence was
# in the guide when this issue was opened, and "as a trailing sentence inside a paragraph about
# what the commands retrieve, rather than as a blocking step" is exactly what the report called
# insufficient. An instrument that accepted it would report the unfixed guide as fixed.
#
# CONTRACT (exit code is the verdict):
#   0 = the defect is GONE at this revision
#   1 = the defect is STILL PRESENT
#   2 = could not run (no guide, or a section shape this instrument cannot measure)
set -uo pipefail
HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
# Scratch stays on the volume this directory lives on — under the enclosing work tree's `target/`, else
# beside this script — and never in the system temporary directory.
ROOT="$(git -C "$HERE" rev-parse --show-toplevel 2>/dev/null || printf '%s' "$HERE")"

GUIDE=""
SELF_TEST=0

while [ $# -gt 0 ]; do
  case "$1" in
    --guide)     GUIDE="${2:?--guide needs a file}"; shift 2 ;;
    --self-test) SELF_TEST=1; shift ;;
    -h|--help)   sed -n '2,26p' "$0"; exit 0 ;;
    -*)          echo "LS-005 remeasure: unknown argument: $1" >&2; exit 2 ;;
    *)           [ -z "$GUIDE" ] || { echo "LS-005 remeasure: unexpected argument: $1" >&2; exit 2; }
                 GUIDE="$1/docs/linkedspec-book/src/public-api/integration-rust.md"; shift ;;
  esac
done

# ── measure one guide ───────────────────────────────────────────────────────────────────────
# Prints the observations, then returns the verdict as its exit code.
measure() {  # $1 = guide file, $2 = label to print
  local guide="$1" label="$2" work sec flat start end prep_line p1 p2 p3 trailing later
  if [ ! -f "$guide" ]; then
    echo "LS-005 remeasure: guide not found: $guide" >&2; return 2
  fi
  mkdir -p "$ROOT/target/feedback_scratch"
  work="$(mktemp -d "$ROOT/target/feedback_scratch/LS-005.XXXXXX")"
  sec="$work/section.md"; flat="$work/section.flat"

  # The section runs from its own heading to the next top-level heading.
  awk '
    /^## / {
      if (insec) exit
      if (tolower($0) ~ /add and pin/) insec = 1
    }
    insec { print }
  ' "$guide" > "$sec"
  if [ ! -s "$sec" ]; then
    echo "LS-005 remeasure: no 'add and pin' section in $guide — the guide has been restructured" >&2
    rm -rf "$work"; return 2
  fi
  # Sentences wrap across lines in this guide, so phrase searches run on a flattened copy.
  tr '\n' ' ' < "$sec" > "$flat"

  start="$(grep -niE '^## .*add and pin' "$guide" | head -1 | cut -d: -f1)"
  [ -n "$start" ] || { echo "LS-005 remeasure: cannot locate the section heading in $guide" >&2; rm -rf "$work"; return 2; }
  end="$(( start + $(wc -l < "$sec" | tr -d ' ') - 1 ))"
  prep_line="$(grep -niE '^#+ .*preparation' "$guide" | head -1 | cut -d: -f1)"

  p1="$(grep -oiE 'before [a-z ,]*((cargo )?metadata|build[a-z]*)' "$flat" | wc -l | tr -d ' ')"
  p2="$(grep -oiE '(initial )?(rgx|pgen) preparation|preparation section|bootstrap' "$flat" | wc -l | tr -d ' ')"
  p3="$(grep -ciE '^#+ .*preparation' "$guide" || true)"
  trailing="$(grep -ciE 'checkout[^.]*does not' "$flat" || true)"
  later="no"; [ -n "$prep_line" ] && [ "$prep_line" -gt "$end" ] && later="yes"

  echo "== LS-005 re-measurement: $label =="
  echo "  'add and pin' section .......................... lines ${start}-${end}"
  echo "  first preparation heading ..................... line ${prep_line:-none} (after the section: $later)"
  echo "  P1 ordering statement before metadata/build ... $([ "$p1" -gt 0 ] && echo "present ($p1)" || echo ABSENT)"
  echo "  P2 the section names the preparation .......... $([ "$p2" -gt 0 ] && echo "present ($p2)" || echo ABSENT)"
  echo "  P3 a preparation section exists to point at ... $([ "${p3:-0}" -gt 0 ] && echo "present ($p3)" || echo ABSENT)"
  echo "  trailing 'checkout does not' mention .......... $([ "${trailing:-0}" -gt 0 ] && echo yes || echo no)"
  echo "  (a trailing mention alone is NOT a fix — see the header of this script)"
  echo

  rm -rf "$work"
  if [ "$p1" -gt 0 ] && [ "$p2" -gt 0 ] && [ "${p3:-0}" -gt 0 ]; then
    echo "LS-005 remeasure: RESULT — the defect is GONE. A reader who lands on 'add and pin' is"
    echo "  sent to workspace setup, local storage and preparation before any metadata or build"
    echo "  command, and the section named exists in the same guide."
    return 0
  fi
  echo "LS-005 remeasure: RESULT — the defect is STILL PRESENT. A reader who follows the sections"
  echo "  in order can still reach a build command before the guide has sent them to preparation."
  return 1
}

# ── self-test: prove the verdict discriminates ──────────────────────────────────────────────
if [ "$SELF_TEST" -eq 1 ]; then
  mkdir -p "$ROOT/target/feedback_scratch"
  t="$(mktemp -d "$ROOT/target/feedback_scratch/LS-005-self-test.XXXXXX")"; trap 'rm -rf "$t"' EXIT
  arms=0; ok=0
  arm() {  # $1 = expected exit, $2 = label, $3 = guide file
    arms=$((arms + 1))
    measure "$3" "$2" >/dev/null 2>&1; rc=$?
    if [ "$rc" -eq "$1" ]; then ok=$((ok + 1)); echo "  arm passes: $2 (exit $rc)"
    else echo "  ARM FAILED: $2 — expected exit $1, got $rc" >&2; fi
  }

  # RED arm 1 — the section VERBATIM as published at the revision this issue was measured on
  # (LinkedSpec ad290bdb4), which is the state the report describes: five commands, a trailing
  # sentence, and no forward pointer.
  cat > "$t/defective.md" <<'GUIDE'
# Integrating LinkedSpec into a Rust application

## Add and pin the source dependency

From your application's repository root:

```sh
git submodule add https://github.com/rdje/linkedspec.git vendor/linkedspec
git -C vendor/linkedspec submodule update --init rgx
git -C vendor/linkedspec/rgx submodule update --init subs/pgen
git -C vendor/linkedspec rev-parse HEAD
git add .gitmodules vendor/linkedspec
```

Review the revision and commit the submodule pointer with your application. For
an existing application clone, first run `git submodule update --init
vendor/linkedspec`, then the two nested commands above. These commands retrieve
the Rust dependency closure. `git submodule update --init --recursive` also works,
but retrieves additional optional dependency/test repositories which this native
Rust example does not need. Checkout does not generate PGEN's parser inputs.

Add this dependency to the application's existing `Cargo.toml`:

```toml
[dependencies]
linkedspec-runtime = { path = "vendor/linkedspec/rust/linkedspec-runtime" }
```

## Keep preparation and build products local

### Initial PGEN preparation

Run the published bootstrap command.
GUIDE

  # GREEN arm — the same section carrying the remedy the report asked for: an ordering statement
  # and a pointer, with the preparation section present.
  cat > "$t/fixed.md" <<'GUIDE'
# Integrating LinkedSpec into a Rust application

## Add and pin the source dependency

From your application's repository root:

```sh
git submodule add https://github.com/rdje/linkedspec.git vendor/linkedspec
git -C vendor/linkedspec submodule update --init rgx
git add .gitmodules vendor/linkedspec
```

**Before running Cargo metadata or building:** complete workspace setup, when applicable,
application-local storage, and RGX preparation. The checkout commands above obtain source;
the preparation section is a required part of this setup sequence. Checkout alone does not
complete the documented preparation.

## Keep preparation and build products local

### Initial RGX preparation

Run the published bootstrap command.
GUIDE

  # RED arm 2 — no recognisable section at all: the instrument must refuse to guess.
  printf '# Some other chapter\n\n## Unrelated heading\n\ntext\n' > "$t/restructured.md"

  # RED arm 3 — the pointer exists but resolves to nothing: P3 must be load-bearing.
  cat > "$t/dangling.md" <<'GUIDE'
# Integrating LinkedSpec into a Rust application

## Add and pin the source dependency

**Before running Cargo metadata or building:** complete RGX preparation.

## Keep build products local

Storage lives under an application-owned directory.
GUIDE

  echo "LS-005 remeasure: self-test"
  arm 1 "the section as published at the measured revision is reported STILL PRESENT" "$t/defective.md"
  arm 0 "the remedied section is reported GONE"                                     "$t/fixed.md"
  arm 2 "a restructured guide is refused rather than guessed"                        "$t/restructured.md"
  arm 1 "a pointer to a section that does not exist is not a fix"                    "$t/dangling.md"
  echo "self-test: $ok/$arms arms passed"
  [ "$ok" -eq "$arms" ] || exit 1
  exit 0
fi

[ -n "$GUIDE" ] || { echo "LS-005 remeasure: needs a LinkedSpec checkout or --guide <file>" >&2; exit 2; }
measure "$GUIDE" "${GUIDE##*/}"
exit $?
