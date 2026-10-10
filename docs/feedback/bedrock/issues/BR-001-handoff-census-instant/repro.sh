#!/usr/bin/env bash
# BR-001 reproducer — self-contained.
#
# bedrock's handoff census, `scripts/check_no_background_jobs.sh`, judges one instant. A process whose command line
# names the checkout and that lives a few seconds — as a terminal's git-status helper does, respawned every few
# seconds — is named, and the census refuses the handoff; run again once it has exited, the census passes. This runs
# the census of the bedrock checkout it is given, read-only, while such a process lives, and again after it exits.
#
#   usage: repro.sh /path/to/bedrock
#
# CONTRACT (exit code is the verdict):
#   0 = reproduces: the census refused while a process of a few seconds' life ran, and named it
#   3 = changed: the census did not name that process while it lived — possibly a second sample, as proposed
#   2 = could not run
set -uo pipefail
BEDROCK="${1:?usage: $0 /path/to/bedrock}"
CENSUS="$BEDROCK/scripts/check_no_background_jobs.sh"
[ -f "$CENSUS" ] || { echo "BR-001: no census at $CENSUS" >&2; exit 2; }
BEDROCK="$(cd "$BEDROCK" && pwd)"

LIFE=4
# A shell that waits on its sleep, so its own command line — which names the checkout — lives as long as the sleep.
bash -c 'sleep "$1"; :' "$BEDROCK/br001-short-lived-helper" "$LIFE" >/dev/null 2>&1 &
helper=$!
sleep 1
during="$(bash "$CENSUS" 2>&1)"; during_rc=$?
wait "$helper" 2>/dev/null
after="$(bash "$CENSUS" 2>&1)"; after_rc=$?

named=0
printf '%s\n' "$during" | grep -q "br001-short-lived-helper" && named=1
named_after=0
printf '%s\n' "$after" | grep -q "br001-short-lived-helper" && named_after=1

echo "== BR-001 observations =="
echo "  a helper naming the checkout, alive ${LIFE} s, pid $helper"
echo "  census while it lived ......... exit $during_rc, names it: $([ $named -eq 1 ] && echo yes || echo no)"
echo "  census once it had exited ..... exit $after_rc, names it: $([ $named_after -eq 1 ] && echo yes || echo no)"
echo
if [ "$named" -eq 1 ] && [ "$during_rc" -eq 1 ]; then
  echo "  REPRODUCED — the census refused a handoff for a process that exited within ${LIFE} s"
  exit 0
fi
echo "  changed — the census did not refuse for a process of ${LIFE} s' life"
exit 3
