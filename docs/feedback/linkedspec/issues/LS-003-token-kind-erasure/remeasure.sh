#!/usr/bin/env bash
# LS-003 re-measurement — do quoted string, bare symbol and number survive as distinguishable kinds,
# with their exact lexemes?
#
# This is NOT repro.sh. repro.sh replays the historical extraction route (`lispish_file` with
# `Lispish.spec`), whose parent rules are documented as discarding the distinction between symbols,
# quoted strings and numeric tokens — so it can never answer this question. The vendor's completion
# notice directs the document requirements to `sexpr_file` with `SExprDocumentV1.spec`, whose contract
# tags every atom with a kind and preserves its spelling exactly: no numeric conversion, no escape
# decoding, quotes included in a string lexeme. That is the route measured here; the historical one is
# run alongside as the regression guard the notice asks consumers to retain.
#
#   usage: remeasure.sh --sexpr-bin <sexpr_file> --sexpr-grammar <SExprDocumentV1.spec>
#                       [--lispish-bin <lispish_file> --lispish-grammar <Lispish.spec>]
#          remeasure.sh --self-test
#
# ── The properties, from the report's own "Expected" ─────────────────────────────────────────────
# "The result distinguishes the token kinds the surface syntax distinguishes." Mechanically, each
# probe's document is flattened to its atoms in order as `kind:lexeme`, and compared with an
# expectation written down here BEFORE any run:
#   P1 QUOTED   `(name "ARCHOGEN")` → `symbol:name`, `string:"ARCHOGEN"` — quotes inside the lexeme
#   P2 BARE     `(name ARCHOGEN)`   → `symbol:name`, `symbol:ARCHOGEN`
#   P3 DISTINGUISHED  P1 and P2 must not produce the same atom list — that equality IS the defect
#   P4 NUMBER   `(task beat (period 10 ms) …)` → `number:10` and `symbol:ms`, not two plain strings
# Verdict:
#   0 = the defect is GONE — every probe matches its expected atoms, and P1 differs from P2
#   1 = STILL PRESENT — a kind was erased, a lexeme was converted or unquoted, or P1 equals P2
#   2 = could not run, or could not decide: a binary or grammar is missing, the result is not a
#       readable document, or an atom carries no kind/lexeme — which means the route under test is
#       not the tagged document route, and scoring it either way would be a lie
#
# CONTRACT (exit code is the verdict):  0 gone · 1 still present · 2 could not run or decide
set -uo pipefail

HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
SEXPR_BIN=""; SEXPR_GRAMMAR=""; LISPISH_BIN=""; LISPISH_GRAMMAR=""; SELF_TEST=0

# atoms <json> — flatten a linkedspec-sexpr-v1 document to `kind:lexeme` lines, in document order.
# Refuses (exit 2) rather than guessing when the value is not that shape.
atoms() {
  printf '%s' "$1" | python3 -c '
import json, sys
raw = sys.stdin.read()
try:
    doc = json.loads(raw)
except Exception:
    print("ERR:not-json"); sys.exit(2)
if not isinstance(doc, dict) or "forms" not in doc:
    print("ERR:no-forms-key"); sys.exit(2)
forms = doc["forms"]
if not isinstance(forms, list):
    print("ERR:forms-not-list"); sys.exit(2)
def walk(node):
    if not isinstance(node, dict):
        print("ERR:node-not-object"); sys.exit(2)
    kind = node.get("kind")
    if kind == "list":
        items = node.get("items")
        if not isinstance(items, list):
            print("ERR:list-without-items"); sys.exit(2)
        for child in items:
            walk(child)
        return
    if kind is None or "lexeme" not in node:
        print("ERR:atom-without-kind-or-lexeme"); sys.exit(2)
    print(str(kind) + ":" + str(node["lexeme"]))
for form in forms:
    walk(form)
'
}

# expected_atoms <probe> — archogen's expectation, written before the run and checkable by eye
# against the input file beside it.
expected_atoms() {
  case "$1" in
    03-number-unit-tokens.eadl)
      printf '%s\n' symbol:task symbol:beat symbol:period number:10 symbol:ms \
                    symbol:deadline number:10 symbol:ms ;;
    04-quoted-string.eadl)
      printf '%s\n' symbol:name 'string:"ARCHOGEN"' ;;
    05-bare-symbol.eadl)
      printf '%s\n' symbol:name symbol:ARCHOGEN ;;
    *) return 2 ;;
  esac
}

PROBES="03-number-unit-tokens.eadl 04-quoted-string.eadl 05-bare-symbol.eadl"

# score <probe> <rc> <stdout> — prints one observation block; returns 0 / 1 / 2 as the contract says
score() {
  local name="$1" rc="$2" out="$3" got want
  if [ "$rc" -ne 0 ]; then
    printf '%-30s | rc=%-3s | REJECTED — the document route refused a valid one-form input\n' "$name" "$rc"
    return 2
  fi
  got="$(atoms "$out")" || true
  case "$got" in
    ERR:*) printf '%-30s | rc=%-3s | unreadable result (%s) — not the tagged document route\n' \
             "$name" "$rc" "$got"
           return 2 ;;
  esac
  want="$(expected_atoms "$name")" || { printf '%-30s | no expectation in this script\n' "$name"; return 2; }
  if [ "$got" = "$want" ]; then
    printf '%-30s | rc=%-3s | kinds and lexemes exactly as expected\n' "$name" "$rc"
    printf '%s\n' "$got" | sed 's/^/      /'
    return 0
  fi
  printf '%-30s | rc=%-3s | MISMATCH (expected < / actual >)\n' "$name" "$rc"
  diff <(printf '%s\n' "$want") <(printf '%s\n' "$got") | sed 's/^/      /'
  return 1
}

# ── self-test: the scorer against synthetic results, no binaries needed ────────────────────────
self_test() {
  local arms=0 ok=0 rc
  local tagged='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"name"},{"kind":"symbol","lexeme":"ARCHOGEN"}],"kind":"list"}]}'
  local quoted='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"name"},{"kind":"string","lexeme":"\"ARCHOGEN\""}],"kind":"list"}]}'
  # The historical erasure: everything a bare symbol, quotes stripped.
  local erased='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"name"},{"kind":"symbol","lexeme":"ARCHOGEN"}],"kind":"list"}]}'
  # A numeric conversion: 10 came back as 10.0.
  local converted='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"task"},{"kind":"symbol","lexeme":"beat"},{"items":[{"kind":"symbol","lexeme":"period"},{"kind":"number","lexeme":"10.0"},{"kind":"symbol","lexeme":"ms"}],"kind":"list"},{"items":[{"kind":"symbol","lexeme":"deadline"},{"kind":"number","lexeme":"10.0"},{"kind":"symbol","lexeme":"ms"}],"kind":"list"}],"kind":"list"}]}'
  local numbers='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"task"},{"kind":"symbol","lexeme":"beat"},{"items":[{"kind":"symbol","lexeme":"period"},{"kind":"number","lexeme":"10"},{"kind":"symbol","lexeme":"ms"}],"kind":"list"},{"items":[{"kind":"symbol","lexeme":"deadline"},{"kind":"number","lexeme":"10"},{"kind":"symbol","lexeme":"ms"}],"kind":"list"}],"kind":"list"}]}'
  local untagged='["name","ARCHOGEN"]'
  arm() { # $1 expected verdict, $2 label, $3 probe, $4 rc, $5 stdout
    arms=$((arms + 1))
    score "$3" "$4" "$5" >/dev/null 2>&1; rc=$?
    if [ "$rc" -eq "$1" ]; then ok=$((ok + 1)); echo "  arm passes: $2 (exit $rc)"
    else echo "  ARM FAILED: $2 — expected exit $1, got $rc" >&2; fi
  }
  echo "LS-003 remeasure: self-test"
  arm 0 "a bare symbol keeps its kind and spelling"        05-bare-symbol.eadl      0 "$tagged"
  arm 0 "a quoted string keeps its kind and BOTH quotes"   04-quoted-string.eadl    0 "$quoted"
  arm 1 "the historical erasure of a quoted string"        04-quoted-string.eadl    0 "$erased"
  arm 0 "a number and a unit are different kinds"          03-number-unit-tokens.eadl 0 "$numbers"
  arm 1 "a numeric conversion is caught"                   03-number-unit-tokens.eadl 0 "$converted"
  arm 2 "an untagged result is refused, not read as symbols" 05-bare-symbol.eadl    0 "$untagged"
  arm 2 "unreadable JSON is refused"                       04-quoted-string.eadl    0 "not json at all"
  arm 2 "a rejection of a valid one-form input is not scored" 05-bare-symbol.eadl   1 ""
  # P3 — the equality that IS the defect — is a cross-probe property, so it gets its own check.
  local a b
  arms=$((arms + 1))
  a="$(atoms "$quoted")"; b="$(atoms "$erased")"
  if [ "$a" = "$b" ]; then
    echo "  ARM FAILED: probe 04 and probe 05 produced the same atoms, and that was not caught" >&2
  else
    ok=$((ok + 1))
    echo "  arm passes: a quoted string and a bare symbol do not collapse to the same atoms"
  fi
  echo "self-test: $ok/$arms arms passed"
  [ "$ok" -eq "$arms" ] || return 1
  return 0
}

while [ $# -gt 0 ]; do
  case "$1" in
    --sexpr-bin)       SEXPR_BIN="${2:?}"; shift 2 ;;
    --sexpr-grammar)   SEXPR_GRAMMAR="${2:?}"; shift 2 ;;
    --lispish-bin)     LISPISH_BIN="${2:?}"; shift 2 ;;
    --lispish-grammar) LISPISH_GRAMMAR="${2:?}"; shift 2 ;;
    --self-test)       SELF_TEST=1; shift ;;
    -h|--help)         sed -n '2,34p' "$0"; exit 0 ;;
    *) echo "LS-003 remeasure: unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [ "$SELF_TEST" -eq 1 ]; then self_test; exit $?; fi

[ -n "$SEXPR_BIN" ] && [ -x "$SEXPR_BIN" ] || { echo "LS-003 remeasure: --sexpr-bin <sexpr_file> must be an executable" >&2; exit 2; }
[ -n "$SEXPR_GRAMMAR" ] && [ -f "$SEXPR_GRAMMAR" ] || { echo "LS-003 remeasure: --sexpr-grammar <SExprDocumentV1.spec> must be a file" >&2; exit 2; }
command -v python3 >/dev/null 2>&1 || { echo "LS-003 remeasure: python3 is required to read the tagged result; without it this instrument refuses rather than guesses" >&2; exit 2; }

# Absolute paths: the guard below runs repro.sh from THIS directory, where a relative --bin would
# silently stop resolving.
abspath() {
  case "$1" in
    /*) printf '%s\n' "$1" ;;
    *)  if [ -e "$1" ]; then ( cd "$(dirname "$1")" && printf '%s/%s\n' "$(pwd -P)" "$(basename "$1")" )
        else printf '%s\n' "$PWD/$1"; fi ;;
  esac
}
SEXPR_BIN="$(abspath "$SEXPR_BIN")"; SEXPR_GRAMMAR="$(abspath "$SEXPR_GRAMMAR")"
[ -n "$LISPISH_BIN" ] && LISPISH_BIN="$(abspath "$LISPISH_BIN")"
[ -n "$LISPISH_GRAMMAR" ] && LISPISH_GRAMMAR="$(abspath "$LISPISH_GRAMMAR")"

echo "== LS-003 re-measurement: the document route =="
echo "  consumer ...... ${SEXPR_BIN##*/}"
echo "  grammar ....... ${SEXPR_GRAMMAR##*/}"
echo
ok=0; bad=0; und=0
a04=""; a05=""
for probe in $PROBES; do
  path="$HERE/evidence/$probe"
  if [ ! -f "$path" ]; then
    printf '%-30s | MISSING INPUT in this directory\n' "$probe"; und=$((und + 1)); continue
  fi
  out="$("$SEXPR_BIN" --grammar "$SEXPR_GRAMMAR" "$path" 2>/dev/null)"; rc=$?
  score "$probe" "$rc" "$out"; ev=$?
  case "$ev" in 0) ok=$((ok + 1)) ;; 1) bad=$((bad + 1)) ;; *) und=$((und + 1)) ;; esac
  case "$probe" in
    04-quoted-string.eadl) a04="$(atoms "$out" 2>/dev/null || true)" ;;
    05-bare-symbol.eadl)   a05="$(atoms "$out" 2>/dev/null || true)" ;;
  esac
done
echo
# P3 — the equality that IS this defect, checked across the two probes rather than inside one.
if [ -n "$a04" ] && [ -n "$a05" ]; then
  if [ "$a04" = "$a05" ]; then
    echo "  probe 04 vs probe 05 ......... IDENTICAL — a quoted string and a bare symbol collapse"
    bad=$((bad + 1))
  else
    echo "  probe 04 vs probe 05 ......... DIFFERENT — the distinction survives"
    ok=$((ok + 1))
  fi
else
  echo "  probe 04 vs probe 05 ......... could not compare (one of them was not readable)"
  und=$((und + 1))
fi
echo "  checks ........ $((ok + bad + und)) · as expected $ok · defect $bad · undecided $und"

if [ -n "$LISPISH_BIN" ] && [ -n "$LISPISH_GRAMMAR" ]; then
  echo
  echo "== the historical extraction route (regression guard) =="
  if [ -x "$LISPISH_BIN" ] && [ -f "$LISPISH_GRAMMAR" ]; then
    guard="$(cd "$HERE" && bash repro.sh --bin "$LISPISH_BIN" --grammar "$LISPISH_GRAMMAR" 2>&1)"; grc=$?
    printf '%s\n' "$guard" | grep -v '^$' | sed 's/^/  /'
    echo "  repro.sh exit ................. $grc  (0 = the frozen observation still reproduces)"
  else
    echo "  could not run: binary or grammar missing"
  fi
fi

echo
verdict=0
[ "$bad" -gt 0 ] && verdict=1
[ "$verdict" -eq 0 ] && [ "$und" -gt 0 ] && verdict=2
case "$verdict" in
  0) echo "LS-003 remeasure: RESULT — the defect is GONE. Quoted string, bare symbol and number are"
     echo "  distinguishable kinds with exact lexemes, and the two inputs that used to produce one"
     echo "  result no longer do." ;;
  1) echo "LS-003 remeasure: RESULT — the defect is STILL PRESENT: a kind was erased, a lexeme was"
     echo "  converted or unquoted, or the two probes still collapse. See the lines above." ;;
  *) echo "LS-003 remeasure: RESULT — could not decide: $und check(s) produced no scorable result."
     echo "  That is reported rather than scored as a pass or a failure." ;;
esac
exit "$verdict"
