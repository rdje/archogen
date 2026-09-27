#!/usr/bin/env bash
# LS-002 re-measurement — does a complete s-expression document return EVERY top-level form, and
# refuse input it cannot completely recognize?
#
# This is NOT repro.sh. repro.sh replays the historical extraction route (`lispish_file` with
# `Lispish.spec`) and compares it to a frozen observation. That grammar is documented as an
# extraction example, not a complete-input validator, and it keeps its historical behaviour by
# design — so it can never answer this question. The vendor's completion notice directs the document
# requirements to `sexpr_file` with `SExprDocumentV1.spec`, and that is what this instrument drives.
# repro.sh's route is still run here, as the regression guard the notice asks consumers to retain.
#
#   usage: remeasure.sh --sexpr-bin <sexpr_file> --sexpr-grammar <SExprDocumentV1.spec>
#                       [--lispish-bin <lispish_file> --lispish-grammar <Lispish.spec>]
#          remeasure.sh --self-test
#
# The probes are this directory's own frozen inputs (evidence/*.eadl), and the expectation for each
# is written next to it below — archogen's own expectation, not something read back from the tool.
#
# ── The properties, from the report's own asks ──────────────────────────────────────────────────
#   P1 COMPLETE   every top-level form of a multi-form document is returned, in order
#   P2 REFUSES    input that cannot be completely recognized is rejected, not silently truncated:
#                 trailing junk, an unbalanced close, an unterminated form
#   P3 NO PARTIAL a rejection returns no accepted partial document on stdout
# Verdict:
#   0 = the defect is GONE — every probe behaves as archogen's expectation says
#   1 = STILL PRESENT — a form was dropped, or unusable input was accepted
#   2 = could not run, or could not decide: a binary or grammar is missing, python3 is absent, or a
#       document archogen expects to be accepted was REJECTED — which is a different defect
#       (over-rejection), not this one, and must not be scored as either
#
# CONTRACT (exit code is the verdict):  0 gone · 1 still present · 2 could not run or decide
set -uo pipefail

HERE="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
SEXPR_BIN=""; SEXPR_GRAMMAR=""; LISPISH_BIN=""; LISPISH_GRAMMAR=""; SELF_TEST=0

# probe file → expectation. `forms=N` counts TOP-LEVEL forms; `reject` means the document must not
# be accepted at all. Each expectation is checkable by eye against the file beside it.
PROBES="01-multiple-top-level-forms.eadl:forms=2
06-unterminated-form.eadl:reject
07-trailing-garbage.eadl:reject
08-comment-no-newline.eadl:forms=1
09-unbalanced-close.eadl:reject
10-semantically-invalid.eadl:forms=1
12-two-forms-one-line.eadl:forms=2
system.eadl:forms=4"

# form_count <json> — top-level forms in a linkedspec-sexpr-v1 document. Refuses rather than guesses.
form_count() {
  command -v python3 >/dev/null 2>&1 || { echo "NO_PYTHON3"; return 2; }
  printf '%s' "$1" | python3 -c '
import json, sys
raw = sys.stdin.read()
try:
    doc = json.loads(raw)
except Exception:
    print("NOT_JSON"); sys.exit(0)
if not isinstance(doc, dict) or "forms" not in doc:
    print("NO_FORMS_KEY"); sys.exit(0)
forms = doc["forms"]
if not isinstance(forms, list):
    print("FORMS_NOT_LIST"); sys.exit(0)
print(len(forms))
'
}

# evaluate <name> <rc> <stdout> <expectation> — prints one observation line; returns
#   0 as expected · 1 defect present · 2 could not decide
evaluate() {
  local name="$1" rc="$2" out="$3" want="$4" n
  case "$want" in
    reject)
      if [ "$rc" -ne 0 ]; then
        if [ -n "$out" ]; then
          printf '%-34s | rc=%-3s | REJECTED but printed a value — P3 fails\n' "$name" "$rc"
          return 1
        fi
        printf '%-34s | rc=%-3s | rejected, no partial value — as expected\n' "$name" "$rc"
        return 0
      fi
      n="$(form_count "$out")"
      case "$n" in NO_PYTHON3|NOT_JSON|NO_FORMS_KEY|FORMS_NOT_LIST)
        printf '%-34s | rc=%-3s | ACCEPTED unusable input; unreadable result (%s)\n' "$name" "$rc" "$n"; return 2 ;;
      esac
      printf '%-34s | rc=%-3s | ACCEPTED with %s form(s) — should have been rejected\n' "$name" "$rc" "$n"
      return 1 ;;
    forms=*)
      local want_n="${want#forms=}"
      if [ "$rc" -ne 0 ]; then
        printf '%-34s | rc=%-3s | REJECTED a document expected to be accepted — not this defect\n' "$name" "$rc"
        return 2
      fi
      n="$(form_count "$out")"
      case "$n" in NO_PYTHON3|NOT_JSON|NO_FORMS_KEY|FORMS_NOT_LIST)
        printf '%-34s | rc=%-3s | unreadable result (%s)\n' "$name" "$rc" "$n"; return 2 ;;
      esac
      if [ "$n" = "$want_n" ]; then
        printf '%-34s | rc=%-3s | %s of %s top-level forms — as expected\n' "$name" "$rc" "$n" "$want_n"
        return 0
      fi
      if [ "$n" -lt "$want_n" ]; then
        printf '%-34s | rc=%-3s | %s of %s top-level forms — TRUNCATED on a success exit\n' "$name" "$rc" "$n" "$want_n"
        return 1
      fi
      printf '%-34s | rc=%-3s | %s forms, expected %s — MORE than the input has\n' "$name" "$rc" "$n" "$want_n"
      return 1 ;;
    *) printf '%-34s | bad expectation in this script: %s\n' "$name" "$want"; return 2 ;;
  esac
}

# ── self-test: the evaluator against synthetic results, no binaries needed ──────────────────────
self_test() {
  local arms=0 ok=0 rc
  local two='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"a"}],"kind":"list"},{"items":[{"kind":"symbol","lexeme":"b"}],"kind":"list"}]}'
  local one='{"format":"linkedspec-sexpr-v1","forms":[{"items":[{"kind":"symbol","lexeme":"a"}],"kind":"list"}]}'
  local empty='{"format":"linkedspec-sexpr-v1","forms":[]}'
  arm() { # $1 expected verdict, $2 label, $3 name, $4 rc, $5 stdout, $6 expectation
    arms=$((arms + 1))
    evaluate "$3" "$4" "$5" "$6" >/dev/null 2>&1; rc=$?
    if [ "$rc" -eq "$1" ]; then ok=$((ok + 1)); echo "  arm passes: $2 (exit $rc)"
    else echo "  ARM FAILED: $2 — expected exit $1, got $rc" >&2; fi
  }
  echo "LS-002 remeasure: self-test"
  arm 0 "both forms of a two-form document are counted"          a 0 "$two"   forms=2
  arm 1 "the historical truncation — one form of two — is caught" b 0 "$one"   forms=2
  arm 1 "an empty result for a four-form document is caught"      c 0 "$empty" forms=4
  arm 0 "a rejection with no value is what P2 and P3 ask for"     d 1 ""       reject
  arm 1 "accepting trailing junk is caught"                       e 0 "$one"   reject
  arm 1 "rejecting while still printing a value is caught"        f 1 "$one"   reject
  arm 2 "rejecting a document we expect to accept is not scored"  g 1 ""       forms=1
  arm 2 "an unreadable result is refused, not read as zero forms" h 0 "not json at all" forms=1
  arm 1 "more forms than the input has is caught"                 i 0 "$two"   forms=1
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
    -h|--help)         sed -n '2,30p' "$0"; exit 0 ;;
    *) echo "LS-002 remeasure: unknown argument: $1" >&2; exit 2 ;;
  esac
done

if [ "$SELF_TEST" -eq 1 ]; then self_test; exit $?; fi

[ -n "$SEXPR_BIN" ] && [ -x "$SEXPR_BIN" ] || { echo "LS-002 remeasure: --sexpr-bin <sexpr_file> must be an executable" >&2; exit 2; }
[ -n "$SEXPR_GRAMMAR" ] && [ -f "$SEXPR_GRAMMAR" ] || { echo "LS-002 remeasure: --sexpr-grammar <SExprDocumentV1.spec> must be a file" >&2; exit 2; }
command -v python3 >/dev/null 2>&1 || { echo "LS-002 remeasure: python3 is required to count top-level forms; without it this instrument refuses rather than guesses" >&2; exit 2; }

# Absolute paths, because the regression guard below runs repro.sh from THIS directory and a relative
# --bin would silently stop resolving there.
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

echo "== LS-002 re-measurement: the document route =="
echo "  consumer ...... ${SEXPR_BIN##*/}"
echo "  grammar ....... ${SEXPR_GRAMMAR##*/}"
echo "  probes ........ $(printf '%s\n' "$PROBES" | grep -c .) frozen inputs in this directory's evidence/"
echo
total=0; okc=0; bad=0; und=0
while IFS=: read -r probe want; do
  [ -n "$probe" ] || continue
  total=$((total + 1))
  path="$HERE/evidence/$probe"
  if [ ! -f "$path" ]; then
    printf '%-34s | MISSING INPUT in this directory\n' "$probe"; und=$((und + 1)); continue
  fi
  out="$("$SEXPR_BIN" --grammar "$SEXPR_GRAMMAR" "$path" 2>/dev/null)"; rc=$?
  evaluate "$probe" "$rc" "$out" "$want"; ev=$?
  case "$ev" in
    0) okc=$((okc + 1)) ;;
    1) bad=$((bad + 1)) ;;
    *) und=$((und + 1)) ;;
  esac
done <<EOF
$PROBES
EOF
echo
echo "  probes ........ $total · as expected $okc · defect $bad · undecided $und"
verdict=0
[ "$bad" -gt 0 ] && verdict=1
[ "$verdict" -eq 0 ] && [ "$und" -gt 0 ] && verdict=2

# The regression guard the notice asks consumers to retain: the historical extraction route must
# still produce its frozen observation. Reported separately — it is a different claim.
if [ -n "$LISPISH_BIN" ] && [ -n "$LISPISH_GRAMMAR" ]; then
  echo
  echo "== the historical extraction route (regression guard) =="
  [ -x "$LISPISH_BIN" ] && [ -f "$LISPISH_GRAMMAR" ] || { echo "  could not run: binary or grammar missing"; }
  if [ -x "$LISPISH_BIN" ] && [ -f "$LISPISH_GRAMMAR" ]; then
    guard="$(cd "$HERE" && bash repro.sh --bin "$LISPISH_BIN" --grammar "$LISPISH_GRAMMAR" 2>&1)"; grc=$?
    printf '%s\n' "$guard" | grep -v '^$' | sed 's/^/  /'
    echo "  repro.sh exit ................. $grc  (0 = the frozen observation still reproduces)"
  fi
fi

echo
case "$verdict" in
  0) echo "LS-002 remeasure: RESULT — the defect is GONE. Every top-level form of every accepted"
     echo "  document was returned, and every document that cannot be completely recognized was"
     echo "  rejected without a partial value." ;;
  1) echo "LS-002 remeasure: RESULT — the defect is STILL PRESENT: a form was dropped on a success"
     echo "  exit, or unusable input was accepted. See the lines above." ;;
  *) echo "LS-002 remeasure: RESULT — could not decide: $und probe(s) did not produce a scorable"
     echo "  result. That is reported rather than scored as a pass or a failure." ;;
esac
exit "$verdict"
