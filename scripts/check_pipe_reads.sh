#!/usr/bin/env bash
# scripts/check_pipe_reads.sh — PIPE-READS: no script that runs under `pipefail` reads a captured output through
# `printf … | grep -q` or `echo … | grep -q` (leaf `PROGRAM.114`).
#
# ⭐ WHY. `grep -q` stops reading at its first match. GNU grep then closes the pipe while `printf` may still be writing,
# and `printf` fails on the closed pipe; under `pipefail` the pipeline fails, so a check whose `grep` matched reads as a
# check that did not. It happens only when the output is larger than what a pipe holds before the reader is gone — 64 KB
# on Linux, more on macOS, where BSD grep also reads to the end — so it passes on a developer's machine and fails on the
# CI runner, intermittently. `PROGRAM.113.3` measured it: the task-history gate's signal arms failed on GitHub's runner
# for exactly this, their outputs grown past the pipe with the gate's own source, while the seal they tested was sound.
# The fix is a here-string, `grep -q … <<< "$out"`: no pipeline, so `pipefail` has nothing to fail.
#
# POPULATION: every tracked shell script — `scripts/*.sh`, `.githooks/*`, `knowledge-map/scripts/*.sh` — that sets
# `pipefail`; a comment line is not code. OWNERSHIP is derived as `SCRATCH-LOCALITY` derives it: a file named in
# `scripts/update_scaffold.sh`'s NEUTRAL array is the scaffold's, its sites COUNTED AND REPORTED, NOT REFUSED — another
# repository owns that file, and its change is recorded in `PROGRAM.114` for whoever owns the scaffold.
#
# ⚠️ HONEST LIMIT: the form is matched by its text — `printf '%s'` or `'%s\n'`, or `echo`, of one variable, piped into
# a `grep` whose flags hold `q`. A read spelled otherwise (a `cat` of a file, a function wrapping the pipe) is not
# seen; it is the idiom this repository writes, and the census that filed the leaf found every site in this form.
#
# MODES:
#   bash scripts/check_pipe_reads.sh              # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_pipe_reads.sh --self-test
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/pipe_reads"

check() {
  python3 - <<'PY'
import os, re, subprocess, sys
# The idiom, assembled so this gate's own text does not hold it.
IDIOM = re.compile(r"""(?:printf '%s(?:\\n)?'|echo) "\$[A-Za-z_][A-Za-z0-9_]*" \| grep -[A-Za-z]*q""")
PIPEFAIL = re.compile(r"^[ \t]*set[ \t].*\bpipefail\b", re.M)
listed = subprocess.run(["git", "ls-files", "--", "scripts/*.sh", ".githooks/*", "knowledge-map/scripts/*.sh"],
                        capture_output=True, text=True)
if listed.returncode != 0:
    print("PIPE-READS: `git ls-files` failed, so nothing was scanned — a failed read is never a clean tree", file=sys.stderr)
    sys.exit(1)
scaffold = set()
if os.path.exists("scripts/update_scaffold.sh"):
    text = open("scripts/update_scaffold.sh", encoding="utf-8").read()
    block = re.search(r"^NEUTRAL=\(\n(.*?)^\)", text, re.M | re.S)
    if block:
        scaffold = {l.strip() for l in block.group(1).split("\n") if l.strip() and not l.strip().startswith("#")}
fails, owed, scanned = [], [], 0
for path in listed.stdout.split():
    try:
        text = open(path, encoding="utf-8").read()
    except (OSError, UnicodeDecodeError) as e:
        fails.append("%s cannot be read: %s" % (path, e))
        continue
    if not PIPEFAIL.search(text):
        continue
    scanned += 1
    for n, line in enumerate(text.split("\n"), 1):
        if line.lstrip().startswith("#") or not IDIOM.search(line):
            continue
        (owed if path in scaffold else fails).append("%s:%d" % (path, n))
for site in owed:
    print("  scaffold-owned, reported not refused: %s" % site)
if fails:
    for site in fails:
        print("PIPE-READS: %s reads a captured output through a pipe into `grep -q` under `pipefail` — GNU grep's -q "
              "closes the pipe at its first match, the writer fails, and pipefail fails the check though it matched; "
              "read it with a here-string, `grep -q … <<< \"$var\"`" % site, file=sys.stderr)
    print("PIPE-READS: %d breach(es)" % len(fails), file=sys.stderr)
    sys.exit(1)
print("pipe-reads: OK (%d pipefail script(s) scanned; %d site(s) in scaffold-owned files reported, their change "
      "recorded for upstream)" % (scanned, len(owed)))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  # The idiom's pieces, so no line of this file is a site.
  local pipe_read="printf '%s' \"\$out\" | grep"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/scripts"; git -C "$work" init -q
    printf 'NEUTRAL=(\n  scripts/neutral.sh\n)\n' > "$work/scripts/update_scaffold.sh"
    printf '#!/usr/bin/env bash\nset -uo pipefail\nout=x\ngrep -qF x <<< "$out"\n' > "$work/scripts/clean.sh"
    git -C "$work" add -A
  }
  put() { # $1 = file under scripts/, $2 = its body after the shebang
    printf '#!/usr/bin/env bash\n%s\n' "$2" > "$work/scripts/$1"; git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a here-string read under pipefail passes" 0 "1 pipefail script(s) scanned"
  fresh; put bad.sh "set -uo pipefail
out=x
if $pipe_read -qF x; then :; fi"
  arm "a printf pipe into grep -q under pipefail is refused, by file and line" 1 "scripts/bad.sh:4"
  fresh; put echo.sh "set -o pipefail
out=x
echo \"\$out\" | grep -qE x && :"
  arm "an echo pipe into grep -q under pipefail is refused" 1 "scripts/echo.sh:4"
  fresh; put loose.sh "out=x
$pipe_read -qF x"
  arm "the same read in a script without pipefail passes — no pipefail, no failure to manufacture" 0 "pipe-reads: OK"
  fresh; put noted.sh "set -uo pipefail
# $pipe_read -q is the idiom this gate refuses"
  arm "the idiom in a comment passes" 0 "pipe-reads: OK"
  fresh; put neutral.sh "set -uo pipefail
out=x
$pipe_read -qF x"
  arm "a scaffold-owned site is reported, not refused" 0 "scaffold-owned, reported not refused: scripts/neutral.sh:4"
  # The rule's reason: a here-string reads an output far past any pipe, under pipefail, as the match it is.
  arms=$((arms + 1))
  local big; big="needle"$'\n'"$(head -c 1000000 /dev/zero | tr '\0' 'x')"
  if ( set -o pipefail; grep -q needle <<< "$big" ); then ok=$((ok + 1)); echo "  ✅ a 1 MB output read through a here-string under pipefail is the match it is"
  else echo "SELF-TEST: a 1 MB output read through a here-string under pipefail failed" >&2; fi
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real tree passes"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "pipe-reads self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") check; exit $? ;;
  *) echo "usage: bash scripts/check_pipe_reads.sh [--self-test]" >&2; exit 2 ;;
esac
