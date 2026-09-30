#!/usr/bin/env bash
# scripts/third_opinion.sh — a third independent reader of archogen's surface syntax (leaf `M1.22`).
#
# ⭐ WHY. `docs/semantics/grammar.md` had two opinions, archogen's reader and archogen's conformance suite, and they
# are one codebase's two views. LinkedSpec's `SExprDocumentV1.spec`, run by its published consumer `sexpr_file`, is
# a recognizer nobody here wrote. `M1.14` closed the question because it had no complete-input mode; `M1.20.5`
# measured that mode delivered. This runs both over every tracked description and records, per document, whether
# they agree.
#
# ⚠️ SCOPE: token boundaries and document completeness — what the grammar defines. Each side is reduced to one line
# per top-level form: lists as parentheses, atoms as raw source text (`cargo run --example reader_shape` for
# archogen; the recognizer's own `lexeme` for LinkedSpec). Kinds and values are **not** compared: the recognizer
# calls `-1` a number and `10ms` a symbol by its rules, and eADL's exactness rules are archogen's own.
#
# THE CLASSES, per document:
#   agree              — both reject it, or both accept it with the same forms
#   archogen-stricter  — archogen's reader refuses it, and the recognizer accepts it
#   recognizer-stricter — the recognizer refuses it, and archogen's reader accepts it
#   mismatch           — both accept it, with different forms: a divergence needing a decision
#
# THE RATCHET is `docs/semantics/third-opinion.txt`, one `<class> <path>` per document. A document recorded as
# `agree` that no longer agrees fails; so does any class that moves, a document missing from the record, and a
# recorded document that is gone — deletion never passes. `--bless` rewrites the record, an explicit act.
#
# The population is derived: every tracked `*.eadl` outside `vendor/`.
#
# CONTRACT: exit 0 = the record holds; 1 = it does not, each change named; 2 = unavailable (no recognizer here).
# `--self-test` runs the classification and the ratchet against stubs in `target/doctrine_scratch/`.
# Seams, for the self-test: `THIRD_OPINION_RECORD`, `THIRD_OPINION_LIST` (a file of paths, instead of git).
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

RECORD="${THIRD_OPINION_RECORD:-docs/semantics/third-opinion.txt}"
SEXPR_BIN=".app-data/target-2ac834913/debug/sexpr_file"
GRAMMAR="vendor/linkedspec/specs/SExprDocumentV1.spec"
SHAPE="target/debug/examples/reader_shape"
BLESS=0

# The recognizer's JSON for one file, reduced to the same shape; `error` when it refused the document.
recognizer_shape() {
  "$SEXPR_BIN" --grammar "$ROOT/$GRAMMAR" "$1" 2>/dev/null | python3 -c '
import json, sys
try:
    d = json.load(sys.stdin)
except Exception:
    print("error unreadable-output"); sys.exit(0)
if d.get("format") != "linkedspec-sexpr-v1":
    print("error " + str(d.get("type", "unknown"))); sys.exit(0)
def shape(f):
    return "(" + " ".join(shape(i) for i in f["items"]) + ")" if f.get("kind") == "list" else f["lexeme"]
for f in d["forms"]:
    print(shape(f))'
}

# $1 = archogen's shape, $2 = the recognizer's shape → the class
classify() {
  local a="$1" r="$2" a_err=0 r_err=0
  case "$a" in error*) a_err=1 ;; esac
  case "$r" in error*) r_err=1 ;; esac
  if [ "$a_err" = 1 ] && [ "$r_err" = 1 ]; then echo agree
  elif [ "$a_err" = 1 ]; then echo archogen-stricter
  elif [ "$r_err" = 1 ]; then echo recognizer-stricter
  elif [ "$a" = "$r" ]; then echo agree
  else echo mismatch; fi
}

population() {
  if [ -n "${THIRD_OPINION_LIST:-}" ]; then cat "$THIRD_OPINION_LIST"
  else git ls-files -- '*.eadl' ':!vendor'; fi
}

# The measured classes, one `<class> <path>` per line, sorted by path.
measure() {
  local path
  while IFS= read -r path; do
    [ -f "$path" ] || continue
    printf '%s %s\n' "$(classify "$("$SHAPE" "$path" 2>/dev/null)" "$(recognizer_shape "$path")")" "$path"
  done < <(population) | sort -k2
}

# $1 = measured, $2 = recorded → every way they differ
ratchet() {
  python3 - "$1" "$2" <<'PY'
import sys
def load(p):
    out = {}
    for line in open(p):
        line = line.strip()
        if line and not line.startswith("#"):
            cls, path = line.split(" ", 1)
            out[path] = cls
    return out
now, rec = load(sys.argv[1]), load(sys.argv[2])
bad = []
for path, cls in rec.items():
    if path not in now:
        bad.append(f"{path}: recorded as {cls}, and gone from the corpus — deletion never passes; --bless deliberately")
    elif now[path] != cls:
        bad.append(f"{path}: recorded as {cls}, now {now[path]}")
for path, cls in now.items():
    if path not in rec:
        bad.append(f"{path}: {cls}, and not in the record — classify it deliberately with --bless")
for b in bad:
    print("THIRD-OPINION: " + b, file=sys.stderr)
sys.exit(1 if bad else 0)
PY
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/third_opinion/selftest"
  rm -rf "$work"; mkdir -p "$work"
  carm() { arms=$((arms + 1)); local got; got="$(classify "$2" "$3")"
    if [ "$got" = "$4" ]; then ok=$((ok + 1)); echo "  ✅ $1"; else echo "SELF-TEST: $1 — got $got, expected $4" >&2; fi; }
  carm "the same forms agree" "(a b)" "(a b)" agree
  carm "both refusing agree" "error read-unclosed-list" "error document_parse_error" agree
  carm "archogen refusing alone is archogen-stricter" "error read-malformed-number" "(a 10ms)" archogen-stricter
  carm "the recognizer refusing alone is recognizer-stricter" "(a b)"$'\n'"junk" "error document_parse_error" recognizer-stricter
  carm "different forms accepted by both is a mismatch" "(a b)" "(a) (b)" mismatch
  rarm() { arms=$((arms + 1)); local rc
    printf '%s\n' "$2" > "$work/now.txt"; printf '%s\n' "$3" > "$work/rec.txt"
    ratchet "$work/now.txt" "$work/rec.txt" 2>/dev/null; rc=$?
    if [ "$rc" = "$4" ]; then ok=$((ok + 1)); echo "  ✅ $1"; else echo "SELF-TEST: $1 — exit $rc, expected $4" >&2; fi; }
  rarm "an unchanged record holds" "agree a.eadl" "agree a.eadl" 0
  rarm "an agreeing document that stops agreeing fails" "mismatch a.eadl" "agree a.eadl" 1
  rarm "a recorded document that is gone fails — deletion never passes" "agree b.eadl" "agree a.eadl"$'\n'"agree b.eadl" 1
  rarm "a new document not yet classified fails" "agree a.eadl"$'\n'"agree c.eadl" "agree a.eadl" 1
  rm -rf "$work"
  echo "third-opinion self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --bless) BLESS=1 ;;
  "") ;;
  *) echo "third-opinion: unknown argument '$1' — none, --bless or --self-test" >&2; exit 2 ;;
esac

[ -x "$SEXPR_BIN" ] || { echo "third-opinion: unavailable — no recognizer at $SEXPR_BIN (build LinkedSpec's sexpr_file at the adopted pin)" >&2; exit 2; }
[ -f "$GRAMMAR" ] || { echo "third-opinion: unavailable — no $GRAMMAR" >&2; exit 2; }
cargo build -q -p eadl-front --example reader_shape || { echo "third-opinion: the reader_shape example does not build" >&2; exit 2; }
measured="$ROOT/target/doctrine_scratch/third_opinion/measured.txt"
mkdir -p "$(dirname "$measured")"
measure > "$measured"
echo "third-opinion: $(wc -l < "$measured" | tr -d ' ') document(s) — $(cut -d' ' -f1 "$measured" | sort | uniq -c | awk '{printf "%s %s, ", $1, $2}' | sed 's/, $//')"
if [ "$BLESS" = 1 ]; then
  { echo "# docs/semantics/third-opinion.txt — per document, whether archogen's reader and LinkedSpec's SExprDocumentV1"
    echo "# recognizer agree on its forms (leaf M1.22). Written by scripts/third_opinion.sh --bless; checked by it."
    cat "$measured"; } > "$RECORD"
  echo "third-opinion: the record rewritten — $RECORD"
  exit 0
fi
[ -f "$RECORD" ] || { echo "third-opinion: no record at $RECORD — --bless writes the first" >&2; exit 1; }
ratchet "$measured" "$RECORD" || exit 1
echo "third-opinion: OK — every document still classified as recorded"
