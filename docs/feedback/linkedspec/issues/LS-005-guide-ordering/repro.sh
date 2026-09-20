#!/usr/bin/env bash
# LS-005 reproducer — self-contained.
#
# A documentation-ordering defect, checked mechanically rather than asserted. Reads only the
# upstream guide in a LinkedSpec checkout.
#
#   usage: repro.sh /path/to/linkedspec
#
# CONTRACT (exit code is the verdict):
#   0 = the ordering problem is present
#   3 = the guide changed — possibly fixed
#   2 = could not run
set -uo pipefail
LINKEDSPEC="${1:?usage: $0 /path/to/linkedspec}"
GUIDE="$LINKEDSPEC/docs/linkedspec-book/src/public-api/integration-rust.md"
[ -f "$GUIDE" ] || { echo "LS-005: guide not found at $GUIDE" >&2; exit 2; }

line_of() { grep -n -- "$1" "$GUIDE" | head -1 | cut -d: -f1; }
add_pin=$(line_of '## Add and pin the source dependency')
pgen=$(line_of '### Initial PGEN preparation')
next=$(awk -v s="$add_pin" 'NR>s && /^## /{print NR; exit}' "$GUIDE")

[ -n "$add_pin" ] && [ -n "$pgen" ] && [ -n "$next" ] || { echo "LS-005: expected section headings not found — the guide has been restructured."; exit 3; }

# Does the "Add and pin" section itself tell the reader they cannot build yet?
section=$(sed -n "${add_pin},$((next-1))p" "$GUIDE")
forward_ref=$(printf '%s' "$section" | grep -ciE 'PGEN preparation|bootstrap|regex_parser_bootstrap' || true)

echo "== LS-005 observations =="
echo "  'Add and pin the source dependency' at line ..... $add_pin"
echo "  section ends at line ............................ $((next-1))"
echo "  'Initial PGEN preparation' at line .............. $pgen"
echo "  forward references to the bootstrap in between .. $forward_ref"
echo
sym1=0; sym2=0
[ "$pgen" -gt "$next" ] && sym1=1        # preparation is in a LATER top-level section
[ "$forward_ref" -eq 0 ] && sym2=1       # and nothing in "Add and pin" points at it

echo "  symptom 1 — PGEN preparation lives in a later section ......... $([ $sym1 -eq 1 ] && echo REPRODUCED || echo 'not seen')"
echo "  symptom 2 — 'Add and pin' never points the reader at it ....... $([ $sym2 -eq 1 ] && echo REPRODUCED || echo 'not seen')"
echo
if [ "$sym1$sym2" = "11" ]; then
  echo "LS-005: RESULT — a reader who follows the sections in order attempts a build that cannot succeed."; exit 0
fi
echo "LS-005: RESULT — the guide no longer matches the recorded observation; see README.md."; exit 3
