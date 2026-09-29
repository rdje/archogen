#!/usr/bin/env bash
# scripts/check_scratch_locality.sh — SCRATCH-LOCALITY: nothing this repository owns puts its scratch off this
# repository's volume (leaf `PROGRAM.29`).
#
# ⭐ WHY THIS EXISTS. The director's data-locality rule: never default to a system temporary directory, a home
# cache, or any other off-volume location — use `target/` or the scratchpad. A `mktemp` with no template does
# exactly that: it writes under the system temporary directory, which on macOS is `/var/folders/…/T/`, on the
# system volume. Measured when this was written, by a logging `mktemp` on `PATH`: 26 calls in one run of the
# doctrine driver and the self-test runner, from five gates, every one of them off the volume. None was left
# behind — each gate removes its scratch on exit — so the breach is where the files are written, not that they
# linger.
#
# THE RULE, for every tracked shell script (`*.sh`, `.githooks/*`, `Makefile`, `*.mk`) and Rust source that
# this repository owns:
#   1. every `mktemp` carries a template under the repository's own `target/`, written
#      mktemp [-d] "$ROOT/target/…XXXXXX"
#      and nothing else is accepted — `-t`, `--tmpdir` and `-p` each resolve against the system directory;
#   2. no line names the system temporary directory, by path or by its environment variable;
#   3. no Rust source calls `temp_dir()` — a test writes under `CARGO_TARGET_TMPDIR`, which is inside `target/`.
# A full-line comment is not code and is skipped. A trailing comment is part of the line, so a reason that has
# to mention one of these shapes goes on a line of its own.
#
# OWNERSHIP is derived, never listed here: a file named in `scripts/update_scaffold.sh`'s NEUTRAL array is the
# scaffold's. Its breaches are COUNTED AND REPORTED, NOT REFUSED — another repository owns that file, and a
# local edit would be erased by the next sync. The change it needs is written down in
# `docs/decisions/decision_scratch-on-the-repository-volume.md` for whoever owns the scaffold.
#
# ⚠️ HONEST LIMITS: it reads source. A scratch path assembled at run time from pieces is not seen, nor is a
# tool that spills to the system directory on its own (`sort` on a large input). The run-time census recorded
# in the leaf is how the first was measured absent.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# ⛔ Every refused shape is assembled from pieces, so this script's own source never carries one. Excluding
# itself by name instead would let a real breach in this file go unseen.
MK="mk""temp"
SYS="t""mp"
CALL_RE="(^|[^A-Za-z0-9_.-])${MK}([[:space:]]|\\)|;|\$)"
GOOD_RE="${MK}([[:space:]]+-d)?[[:space:]]+\"\\\$(ROOT|\\{ROOT\\})/target/[^\"]*XXX[^\"]*\""
SYSDIR_RE="(^|[^A-Za-z0-9_.-])/(private/|var/)?${SYS}(/|[^A-Za-z0-9_.-]|\$)|\\\$\\{?T""MPDIR"
RUST_RE="(^|[^A-Za-z0-9_])temp_dir[[:space:]]*\\("

# The scaffold's files, one per line: the NEUTRAL array of the updater, if there is one.
scaffold_owned() {
  [ -f scripts/update_scaffold.sh ] || return 0
  awk '/^NEUTRAL=\(/{f=1; next} f&&/^\)/{exit} f{gsub(/[[:space:]]/,""); if ($0 != "" && $0 !~ /^#/) print}' \
    scripts/update_scaffold.sh
}

fail=0; owed=0; scanned=0
note() { printf 'SCRATCH-LOCALITY: %s\n' "$1" >&2; fail=$((fail + 1)); }
owned_by_scaffold="$(scaffold_owned)"

# $1 = file, $2 = line number, $3 = what is wrong
breach() {
  if printf '%s\n' "$owned_by_scaffold" | grep -qxF -- "$1"; then
    owed=$((owed + 1))
    printf '  scaffold-owned, reported not refused: %s:%s\n' "$1" "$2"
  else
    note "$1:$2 — $3"
  fi
}

# Lines of $1 that are code (not a full-line comment) matching $2, as `number<TAB>text`.
# ⛔ The pattern goes through the environment, not `awk -v`: `-v` processes backslash escapes, and would turn
# the pattern's `\)` into `)` and unbalance it.
code_lines() { # $1 = file, $2 = ERE, $3 = comment leader
  RE="$2" LEAD="$3" awk '{ line = $0; sub(/^[[:space:]]+/, "", line)
    if (index(line, ENVIRON["LEAD"]) == 1) next
    if ($0 ~ ENVIRON["RE"]) printf "%d\t%s\n", NR, $0 }' "$1"
}

scan() {
  while IFS= read -r f; do
    [ -f "$f" ] || continue
    scanned=$((scanned + 1))
    while IFS=$'\t' read -r n text; do
      [ -n "$n" ] || continue
      calls="$(printf '%s\n' "$text" | grep -oE "$CALL_RE" | wc -l | tr -d ' ')"
      good="$(printf '%s\n' "$text" | grep -oE "$GOOD_RE" | wc -l | tr -d ' ')"
      if [ "$calls" -gt "$good" ]; then
        breach "$f" "$n" "a ${MK} with no template under target/ writes to the system temporary directory; write ${MK} -d \"\$ROOT/target/doctrine_scratch/<name>.XXXXXX\""
      fi
    done < <(code_lines "$f" "$CALL_RE" "#")
    while IFS=$'\t' read -r n text; do
      [ -n "$n" ] || continue
      breach "$f" "$n" "names the system temporary directory — scratch belongs under target/"
    done < <(code_lines "$f" "$SYSDIR_RE" "#")
  done < <(git ls-files -- '*.sh' '.githooks/*' 'Makefile' '*.mk')

  while IFS= read -r f; do
    [ -f "$f" ] || continue
    scanned=$((scanned + 1))
    while IFS=$'\t' read -r n text; do
      [ -n "$n" ] || continue
      breach "$f" "$n" "temp_dir() is the system temporary directory — a test writes under CARGO_TARGET_TMPDIR, inside target/"
    done < <(code_lines "$f" "$RUST_RE" "//")
  done < <(git ls-files -- '*.rs')
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/scratch_locality/selftest"
  # The shapes the fixtures write, assembled the same way as the patterns above.
  local good="tmp=\"\$(${MK} -d \"\$ROOT/target/doctrine_scratch/x.XXXXXX\")\""
  fresh() { # a repository with one project script using the accepted form, and a scaffold updater
    rm -rf "$work"; mkdir -p "$work/scripts" "$work/src"
    git -C "$work" init -q
    printf '#!/usr/bin/env bash\n# a bare %s in a comment is not a call\n%s\n' "$MK" "$good" > "$work/scripts/check_x.sh"
    printf 'NEUTRAL=(\n  scripts/check_owned.sh\n)\n' > "$work/scripts/update_scaffold.sh"
    printf 'fn main() {}\n' > "$work/src/main.rs"
    git -C "$work" add -A
  }
  put() { mkdir -p "$(dirname "$work/$1")"; printf '#!/usr/bin/env bash\n%s\n' "$2" > "$work/$1"; git -C "$work" add -A; } # $1 = path, $2 = line
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a template under the repository's target/ passes, and a comment is not a call" 0 ""
  fresh; put scripts/check_y.sh "tmp=\"\$(${MK} -d)\""
  arm "a bare ${MK} -d is refused, at its line" 1 "scripts/check_y.sh:2"
  fresh; put scripts/check_y.sh "f=\"\$(${MK})\"; rm -f \"\$f\""
  arm "a bare ${MK} for a file is refused" 1 "scripts/check_y.sh:2"
  fresh; put scripts/check_y.sh "d=\"\$(${MK} -d -t arm.XXXXXX)\""
  arm "a template given with -t still resolves against the system directory, and is refused" 1 "scripts/check_y.sh:2"
  fresh; put scripts/check_y.sh "d=\"\$(${MK} -d \"target/arm.XXXXXX\")\""
  arm "a relative template is not derived from the repository root, and is refused" 1 "scripts/check_y.sh:2"
  fresh; put scripts/check_y.sh "d=\"\$(${MK} -d \"\$HOME/.cache/arm.XXXXXX\")\""
  arm "an absolute template off the repository — a home cache — is refused" 1 "scripts/check_y.sh:2"
  fresh; put scripts/check_y.sh "log=/${SYS}/arm.log"
  arm "a literal system temporary path is refused" 1 "scripts/check_y.sh:2 — names the system temporary directory"
  fresh; put scripts/check_y.sh "cp a \"\$T""MPDIR/a\""
  arm "the system temporary directory by its variable is refused" 1 "scripts/check_y.sh:2 — names the system temporary directory"
  fresh; put scripts/check_y.sh "ok=\"\$(${MK} -d \"\$ROOT/target/a.XXXXXX\")\"; bad=\"\$(${MK} -d)\""
  arm "one good call does not cover a bad one on the same line" 1 "scripts/check_y.sh:2"
  fresh; put .githooks/pre-commit "tmp=\"\$(${MK} -d)\""
  arm "a hook is in scope, not only scripts/" 1 ".githooks/pre-commit:2"
  fresh; printf 'fn f() { let _ = std::env::temp_dir(); }\n' > "$work/src/lib.rs"; git -C "$work" add -A
  arm "a Rust temp_dir() is refused" 1 "src/lib.rs:1"
  fresh; put scripts/check_owned.sh "tmp=\"\$(${MK} -d)\""
  arm "a scaffold-owned file is reported, not refused" 0 "scaffold-owned, reported not refused: scripts/check_owned.sh:2"
  fresh; printf '#!/usr/bin/env bash\ntmp="$(%s -d)"\n' "$MK" > "$work/scripts/check_new.sh"
  arm "an untracked script is outside the repository's view" 0 ""
  git -C "$work" add -A
  arm "the same script, once staged, is in scope" 1 "scripts/check_new.sh:2"
  rm -rf "$work"; mkdir -p "$work"; git -C "$work" init -q; printf 'x\n' > "$work/README.md"; git -C "$work" add -A
  arm "a repository with no script and no Rust source to read is a breach, not a pass" 1 "an empty population is a breach"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real tree keeps its scratch on its own volume"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "scratch-locality self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

scan
if [ "$scanned" -eq 0 ]; then
  echo "SCRATCH-LOCALITY: no tracked script or Rust source found — an empty population is a breach, not a pass" >&2
  exit 1
fi
if [ "$fail" -ne 0 ]; then
  echo "SCRATCH-LOCALITY: $fail breach(es) — scratch belongs on this repository's volume, under target/" >&2
  exit 1
fi
echo "scratch-locality: OK ($scanned file(s) scanned; $owed site(s) in scaffold-owned files reported, their change recorded for upstream)"
exit 0
