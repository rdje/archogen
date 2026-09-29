#!/usr/bin/env bash
# FEEDBACK-SELF-CONTAINED — every reported bug's directory stands alone.
#
# ⭐ WHY THIS EXISTS. A bug report leaves this repository and is read inside someone else's
# project. If issue LS-003's directory references `../../SETUP.md`, then handing the vendor that
# one directory hands them a broken document — and the breakage is invisible here, where the
# neighbour file happens to exist. "Each bug is self-contained" is exactly the kind of promise
# that is true the day it is written and quietly false a month later.
#
# WHAT IT PROVES, per docs/feedback/<vendor>/issues/<ID>-<slug>/:
#   1. COMPLETENESS   — README.md, SETUP.md and an executable repro.sh are present, and
#                       evidence/ holds at least one file.
#   2. CLOSURE        — no file in the directory references a path outside it (`../`), so
#                       extracting the directory alone loses nothing.
#   3. PORTABILITY    — no file carries a checkout-specific absolute path, so a frozen
#                       observation reproduces on the recipient's machine, not only on ours.
#   4. REGISTRATION   — the vendor's INDEX.md names every issue directory that exists, so a bug
#                       cannot be added without appearing in the register.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: this proves a directory is CLOSED and COMPLETE.
# It cannot prove the reproducer reproduces anything — only running it does that, and the
# exit-code contract inside each issue is what carries that weight.
#
# CONTRACT: exit code is the verdict; explains on stderr; deterministic; read-only.
#           `--self-test` proves the green arm passes AND every red arm fires.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

note() { printf 'FEEDBACK-SELF-CONTAINED: %s\n' "$1" >&2; }

# list_files <dir> — files to inspect. Inside the repo: tracked plus not-yet-staged, never
# ignored scratch. The self-test fixture, under `target/`: everything — git ignores all of `target/`,
# so asking git would list nothing and every arm would pass vacuously (leaf `PROGRAM.29`).
list_files() {
  case "$1" in
    "$ROOT"/target/*) find "$1" -type f ;;
    "$ROOT"/*|docs/*) git ls-files --cached --others --exclude-standard "$1" ;;
    *) find "$1" -type f ;;
  esac
}

# scan <base> — the whole doctrine. Returns 0 if every issue directory stands alone.
scan() {
  local base="$1" fail=0 vendor index issue id required hits f
  [ -d "$base" ] || return 0
  shopt -s nullglob
  for vendor in "$base"/*/; do
    [ -d "${vendor}issues" ] || continue
    index="${vendor}INDEX.md"
    if [ ! -f "$index" ]; then
      note "${vendor} has issues/ but no INDEX.md — a set of bugs with no register"; fail=1
    fi
    for issue in "${vendor}issues"/*/; do
      id="$(basename "$issue")"
      # 1. COMPLETENESS
      for required in README.md SETUP.md repro.sh; do
        [ -f "$issue$required" ] || { note "$id is missing $required"; fail=1; }
      done
      [ -x "${issue}repro.sh" ] || { note "$id: repro.sh is present but not executable"; fail=1; }
      if [ -d "${issue}evidence" ]; then
        [ -n "$(find "${issue}evidence" -type f -print -quit)" ] || { note "$id: evidence/ is empty"; fail=1; }
      else
        note "$id has no evidence/ directory"; fail=1
      fi
      # 2. CLOSURE and 3. PORTABILITY
      while IFS= read -r f; do
        [ -n "$f" ] && [ -f "$f" ] || continue
        if hits="$(grep -n '\.\./' "$f" 2>/dev/null)"; then
          note "$id/${f##*/} escapes its directory: $(printf '%s' "$hits" | head -1)"; fail=1
        fi
        if hits="$(grep -nE '(/Users/|/home/|/Volumes/)' "$f" 2>/dev/null)"; then
          note "$id/${f##*/} carries a checkout-specific path: $(printf '%s' "$hits" | head -1)"; fail=1
        fi
      done < <(list_files "$issue")
      # 4. REGISTRATION
      if [ -f "$index" ] && ! grep -q "$id" "$index"; then
        note "$id exists but is not named in ${vendor}INDEX.md"; fail=1
      fi
    done
  done
  return $fail
}

# ── self-test ────────────────────────────────────────────────────────────────────────────────
if [ "${1:-}" = "--self-test" ]; then
  # Scratch on this repository's own volume, never `$TMPDIR` (leaf `PROGRAM.29`, `SCRATCH-LOCALITY`).
  mkdir -p "$ROOT/target/doctrine_scratch"
  tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/feedback_self_contained.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
  v="$tmp/vendor"; mkdir -p "$v/issues"
  mk() {
    mkdir -p "$1/evidence"
    printf '# issue\n' > "$1/README.md"; printf '# setup\n' > "$1/SETUP.md"
    printf '#!/usr/bin/env bash\necho ok\n' > "$1/repro.sh"; chmod +x "$1/repro.sh"
    printf 'observed\n' > "$1/evidence/EXPECTED.txt"
  }
  mk "$v/issues/XX-001-good"; printf 'XX-001-good\n' > "$v/INDEX.md"

  if ! scan "$tmp" 2>/dev/null; then
    echo "SELF-TEST FAILED: a closed, complete fixture was rejected" >&2; exit 1
  fi
  echo "self-test: GREEN arm passes (a closed fixture is accepted)"

  arms=0; caught=0
  probe() { # $1 = label, $2 = mutation applied to a second issue
    arms=$((arms + 1))
    mk "$v/issues/XX-002-bad"; printf 'XX-002-bad\n' >> "$v/INDEX.md"
    eval "$2"
    if scan "$tmp" 2>/dev/null; then printf '  RED ARM DID NOT FIRE: %s\n' "$1" >&2
    else caught=$((caught + 1)); printf '  red arm fires: %s\n' "$1"; fi
    rm -rf "$v/issues/XX-002-bad"; printf 'XX-001-good\n' > "$v/INDEX.md"
  }
  probe "a missing SETUP.md"            'rm "$v/issues/XX-002-bad/SETUP.md"'
  probe "a reference escaping the dir"  'echo "see ../../SETUP.md" >> "$v/issues/XX-002-bad/README.md"'
  probe "a checkout-specific path"      'echo "/Users/someone/checkout" >> "$v/issues/XX-002-bad/README.md"'
  probe "a repro.sh without +x"         'chmod -x "$v/issues/XX-002-bad/repro.sh"'
  probe "an empty evidence/"            'rm -f "$v/issues/XX-002-bad/evidence/"*'
  probe "an issue absent from INDEX.md" 'printf "XX-001-good\n" > "$v/INDEX.md"'

  printf 'self-test: %s/%s red arms fired\n' "$caught" "$arms"
  [ "$caught" -eq "$arms" ] || exit 1
  exit 0
fi

scan "${1:-docs/feedback}"
