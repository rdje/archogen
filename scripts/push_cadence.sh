#!/usr/bin/env bash
# scripts/push_cadence.sh — how far this branch is ahead of `origin/main`, against the ruled push threshold
# (leaf `PROGRAM.23`, `docs/decisions/decision_push-cadence.md`).
#
# ⭐ WHY THIS EXISTS. The director ruled a push cadence on `2026-09-28`, and for as long as it lived only in prose
# nothing read it: the branch went unpushed from `2026-09-13`, and "is a push due?" had no mechanical answer. A
# rule that lives only in a prompt is enforced nowhere.
#
# ONE PRODUCER. The threshold is read from the decision record's own field, `PUSH_AT_COMMITS_AHEAD=<n>`, and the
# record's title (`N = <n>`) must agree with it. No other document carries the number, and the live count is
# carried by none: it moves with every commit, so documents point here instead.
#
# ⛔ A REPORT, NEVER A GATE — and that is the design, not a gap. A push needs the director's authorization (the
# decision sets *when* one is due, not a standing permission), so a check that blocked commits at the threshold
# would strand the work it exists to protect. Nothing on the commit or tier path calls this script; "due" is an
# exit code for whoever asks, and a loud line.
#
# CONTRACT: exit 0 = below the threshold; 3 = a push is due (at or above it); 2 = cannot tell — no `origin/main`,
# or a threshold missing, malformed or contradicted by its own record. Read-only.
# `--self-test` runs the RED arms in scratch repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

RECORD="docs/decisions/decision_push-cadence.md"
say() { printf 'push-cadence: %s\n' "$1"; }

report() {
  local fields threshold title ahead
  [ -f "$RECORD" ] || { say "$RECORD is missing — nothing rules when a push is due"; return 2; }
  fields="$(grep -oE 'PUSH_AT_COMMITS_AHEAD=[^`[:space:]]*' "$RECORD")"
  [ "$(printf '%s' "$fields" | grep -c .)" -eq 1 ] ||
    { say "$RECORD must carry exactly one PUSH_AT_COMMITS_AHEAD=<n> field; it carries $(printf '%s' "$fields" | grep -c .)"; return 2; }
  threshold="${fields#PUSH_AT_COMMITS_AHEAD=}"
  case "$threshold" in ''|*[!0-9]*) say "the threshold in $RECORD is '$threshold', not a number of commits"; return 2 ;; esac
  title="$(head -n 1 "$RECORD" | grep -oE 'N = [0-9]+' | grep -oE '[0-9]+$')"
  [ "$title" = "$threshold" ] ||
    { say "$RECORD's title says N = ${title:-nothing}, and its field says $threshold — one record, two answers"; return 2; }
  git rev-parse --verify -q refs/remotes/origin/main >/dev/null ||
    { say "no origin/main here — the distance cannot be measured (the threshold is $threshold)"; return 2; }
  ahead="$(git rev-list --count refs/remotes/origin/main..HEAD)"
  if [ "$ahead" -lt "$threshold" ]; then
    say "$ahead commit(s) ahead of origin/main; the ruled threshold is $threshold — $((threshold - ahead)) to go"
    return 0
  fi
  say "PUSH DUE — $ahead commit(s) ahead of origin/main, at or past the ruled threshold of $threshold"
  say "  the decision sets when a push is due, not a permission to push: ask the director, and read \`make integration\` first"
  return 3
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/push_cadence/selftest"
  fresh() { # $1 = commits ahead of origin/main, $2 = the record's title number, $3 = its field value
    rm -rf "$work"; mkdir -p "$work/docs/decisions"
    git -C "$work" init -q
    printf '# Push cadence: `N = %s` commits ahead\n\n- **Threshold:** `PUSH_AT_COMMITS_AHEAD=%s`\n' "$2" "$3" > "$work/$RECORD"
    git -C "$work" add -A
    git -C "$work" -c user.name=arm -c user.email=arm@example.invalid commit -q -m base
    git -C "$work" update-ref refs/remotes/origin/main HEAD
    local i=0
    while [ "$i" -lt "$1" ]; do
      git -C "$work" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m "c$i"; i=$((i + 1))
    done
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh 2 3 3; arm "one below the threshold is not due, and says how far" 0 "2 commit(s) ahead of origin/main; the ruled threshold is 3 — 1 to go"
  fresh 3 3 3; arm "exactly at the threshold, a push is due" 3 "PUSH DUE — 3 commit(s) ahead"
  fresh 4 3 3; arm "past the threshold, a push is due" 3 "PUSH DUE — 4 commit(s) ahead"
  fresh 0 3 3; arm "level with origin/main is nothing to push" 0 "0 commit(s) ahead"
  fresh 1 3 3; git -C "$work" update-ref -d refs/remotes/origin/main
  arm "no origin/main is 'cannot tell', not zero" 2 "no origin/main here"
  fresh 1 3 three; arm "a threshold that is not a number is refused" 2 "is 'three', not a number"
  fresh 1 5 3; arm "a title and a field that disagree are refused" 2 "title says N = 5, and its field says 3"
  fresh 1 3 3; printf '\n`PUSH_AT_COMMITS_AHEAD=9`\n' >> "$work/$RECORD"
  arm "a second copy of the threshold is refused — one producer" 2 "exactly one PUSH_AT_COMMITS_AHEAD=<n> field; it carries 2"
  fresh 1 3 3; rm "$work/$RECORD"; arm "no record is 'cannot tell'" 2 "is missing"
  rm -rf "$work"
  arms=$((arms + 1))
  local real rc
  real="$(bash "$SELF" 2>&1)"; rc=$?
  if [ "$rc" -eq 0 ] || [ "$rc" -eq 3 ]; then ok=$((ok + 1)); echo "  ✅ the real record gives an answer: $real" | head -n 1
  else echo "SELF-TEST: the real record gives no answer (exit $rc): $real" >&2; fi
  echo "push-cadence self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") report; exit $? ;;
  *) say "unknown argument '$1' — the script takes none, or --self-test"; exit 2 ;;
esac
