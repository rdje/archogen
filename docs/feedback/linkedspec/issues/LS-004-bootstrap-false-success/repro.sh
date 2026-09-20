#!/usr/bin/env bash
# LS-004 reproducer — self-contained.
#
# Shows that the PGEN bootstrap continues past a failed `cargo` invocation and then prints a
# seeding message for a file it did not produce. Needs only a LinkedSpec checkout with its
# rgx / rgx-subs-pgen submodules initialised; nothing from this repository is read.
#
#   usage: repro.sh /path/to/linkedspec [workdir]
#
# It provokes the cargo failure using LS-001 (the workspace collision), because that is the
# cheapest reliable way to make a prerequisite fail. The DEFECT UNDER TEST IS NOT LS-001: it is
# that the bootstrap keeps going afterwards and reports a success it did not achieve. Any other
# cause of a failing cargo would show the same thing.
#
# CONTRACT (exit code is the verdict):
#   0 = all three symptoms reproduce
#   3 = behaviour changed — possibly fixed; read the output
#   2 = could not run
set -uo pipefail
LINKEDSPEC="${1:?usage: $0 /path/to/linkedspec [workdir]}"
WORK="${2:-$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)/.repro-work}"
PGEN_REL="rgx/subs/pgen"
[ -d "$LINKEDSPEC/$PGEN_REL/rust" ] || { echo "LS-004: $LINKEDSPEC/$PGEN_REL/rust missing — run the nested submodule init first (../../SETUP.md step 1)" >&2; exit 2; }
command -v make >/dev/null || { echo "LS-004: make not found" >&2; exit 2; }

rm -rf "$WORK"; mkdir -p "$WORK/vendor"
printf '[workspace]\nresolver = "2"\nmembers = []\n' > "$WORK/Cargo.toml"
rsync -a --exclude "target/" "$LINKEDSPEC/" "$WORK/vendor/linkedspec/"

PGEN="$WORK/vendor/linkedspec/$PGEN_REL"
rm -rf "$PGEN/generated"
log="$WORK/bootstrap.log"
( cd "$WORK" && CARGO_TARGET_DIR="$PGEN/rust/target" \
    make -C "vendor/linkedspec/$PGEN_REL/rust" SHELL=/bin/bash regex_parser_bootstrap ) >"$log" 2>&1
make_rc=$?

cargo_failures=$(grep -c "believes it's in a workspace when it's not" "$log" || true)
seeded=$(grep -c "seeded\." "$log" || true)
generated_files=$(find "$PGEN/generated" -type f 2>/dev/null | wc -l | tr -d ' ')

echo "== LS-004 observations =="
echo "  make exit status .................. $make_rc"
echo "  failed cargo invocations in log ... $cargo_failures"
echo "  'seeded.' messages printed ........ $seeded"
echo "  files actually in generated/ ...... $generated_files"
echo
echo "-- relevant log tail --"
grep -E "believes it's in a workspace|No such file or directory|seeded\.|Error [0-9]+" "$log" | tail -8 | sed 's/^/  /'
echo

sym1=0; sym2=0; sym3=0
[ "$cargo_failures" -ge 2 ] && sym1=1   # kept going after the first failure
[ "$seeded" -ge 1 ] && sym2=1           # claimed a seed
[ "$generated_files" -eq 0 ] && sym3=1  # but produced nothing

echo "  symptom 1 — continued past a failed prerequisite ... $([ $sym1 -eq 1 ] && echo REPRODUCED || echo 'not seen')"
echo "  symptom 2 — printed a seeding success message ...... $([ $sym2 -eq 1 ] && echo REPRODUCED || echo 'not seen')"
echo "  symptom 3 — generated/ is empty afterwards ......... $([ $sym3 -eq 1 ] && echo REPRODUCED || echo 'not seen')"
echo
if [ "$sym1$sym2$sym3" = "111" ]; then
  echo "LS-004: RESULT — all three symptoms reproduce."; exit 0
fi
echo "LS-004: RESULT — behaviour differs from the recorded observation; see README.md."; exit 3
