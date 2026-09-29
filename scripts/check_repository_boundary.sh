#!/usr/bin/env bash
# scripts/check_repository_boundary.sh — REPOSITORY-BOUNDARY: no archogen agent writes into another
# repository, observed at the one place this repository can see one — its own vendored submodules.
#
# ⭐ WHY THIS EXISTS. `docs/decisions/decision_repository-boundary-read-only.md`: every git repository other
# than this one is read-only, in both directions. The rule lived only in a session prompt until leaf
# `PROGRAM.11`, which is how a durable record once stated an inbound crossing as an outbound one. The rule is
# now written in `CLAUDE.md`; this check is the part of it that is observable from inside this repository.
#
# WHAT IT CHECKS, for every submodule this repository's index records as a gitlink (mode 160000):
#   1. the checkout is AT THE PIN — its HEAD is the commit the index records (a staged pin move is respected,
#      because the index is what the commit will record);
#   2. NO LOCAL-ONLY COMMIT — nothing reachable from HEAD or a local branch that no remote-tracking ref and
#      no tag reaches. ⛔ Tags are excluded on measurement: `rev-list --all --not --remotes` counts every
#      fetched tag as "local" and reported 4 040 such commits in one nested checkout that nobody wrote;
#   3. NO MODIFIED TRACKED FILE — a write into its content;
#   4. NO UNTRACKED FILE — a file created inside it, which the decision names as a breach as well.
# A submodule that is not checked out is reported and passes: nothing is there to have been written to.
#
# ⚠️ HONEST LIMITS, stated rather than hidden:
#   - It cannot see a write into a checkout elsewhere on the filesystem, nor prevent an inbound one; the
#     inbound half is a rule in `CLAUDE.md` and a leaf, not a gate.
#   - It covers the pins THIS repository owns, not the checkouts nested inside them. Their state belongs to
#     the vendor's own documented build (RGX's published bootstrap moves and dirties them, measured), which
#     the director's rule permits — "normal documented builds and reuse of their outputs are permitted".
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only against every repository it inspects.
# `--self-test` builds scratch repositories under `target/doctrine_scratch/`, on this repository's volume,
# and never touches a real one.
set -uo pipefail

SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="target/doctrine_scratch/repository_boundary"

fail=0
note() { printf 'REPOSITORY-BOUNDARY: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The pins to check, one `<path> <commit>` per line: the index's gitlinks, unless an arm supplies them.
pins() {
  if [ -n "${REPOSITORY_BOUNDARY_PINS:-}" ]; then
    printf '%s\n' "$REPOSITORY_BOUNDARY_PINS"
  else
    git ls-files -s | awk '$1 == "160000" { print $4, $2 }'
  fi
}

check_pin() { # $1 = path, $2 = the commit the index records
  local path="$1" pin="$2" head local_only modified untracked
  if [ ! -e "$path/.git" ]; then
    echo "  $path is not checked out — nothing there to have been written to" >&2
    return
  fi
  head="$(git -C "$path" rev-parse HEAD 2>/dev/null)" || { note "$path is a checkout git cannot read"; return; }
  if [ "$head" != "$pin" ]; then
    note "$path is checked out at $head, not at the pin $pin — move the pin by committing the gitlink, or check the pin out again"
  fi
  local_only="$(git -C "$path" rev-list HEAD --branches --not --remotes --tags -- 2>/dev/null)"
  if [ -n "$local_only" ]; then
    note "$path carries $(printf '%s\n' "$local_only" | wc -l | tr -d ' ') commit(s) that exist only here, e.g. $(printf '%s\n' "$local_only" | head -1) — a commit inside another repository is a write into it"
  fi
  modified="$(git -C "$path" status --porcelain --untracked-files=no --ignore-submodules=all 2>/dev/null)"
  if [ -n "$modified" ]; then
    note "$path has modified tracked files: $(printf '%s\n' "$modified" | head -3 | sed 's/^...//' | paste -sd, -) — its content is another project's"
  fi
  untracked="$(git -C "$path" status --porcelain --ignore-submodules=all 2>/dev/null | grep '^??' || true)"
  if [ -n "$untracked" ]; then
    note "$path has files created inside it: $(printf '%s\n' "$untracked" | head -3 | sed 's/^...//' | paste -sd, -) — a file inside another repository is a write into it"
  fi
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  rm -rf "$work"; mkdir -p "$work"
  # A published upstream (bare), and a vendored clone of it: the shape `vendor/<name>` has.
  git init -q --bare "$work/upstream.git"
  git clone -q "$work/upstream.git" "$work/seed" 2>/dev/null
  git -C "$work/seed" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m "published 1"
  git -C "$work/seed" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m "published 2"
  git -C "$work/seed" push -q origin HEAD:main 2>/dev/null
  local pin
  pin="$(git -C "$work/seed" rev-parse HEAD)"
  # A published tag on a commit that NO branch reaches — only the tag does. ⛔ It has to be off every branch:
  # a tag on a branch commit would pass the tag arm whether or not the check excludes tags.
  git -C "$work/seed" checkout -q --detach 2>/dev/null
  git -C "$work/seed" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m "tagged only"
  git -C "$work/seed" -c user.name=arm -c user.email=arm@example.invalid tag -a -m side published-tag HEAD
  git -C "$work/seed" push -q origin refs/tags/published-tag 2>/dev/null

  fresh() { # a clean vendored checkout at the pin, in $work/vendor
    rm -rf "$work/vendor"
    git clone -q "$work/upstream.git" "$work/vendor" 2>/dev/null
    git -C "$work/vendor" checkout -q "$pin" 2>/dev/null
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(REPOSITORY_BOUNDARY_PINS="$work/vendor $pin" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p;s/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — exit $rc, but not about \`$must\`, so it refused for another reason:" >&2
      printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }

  fresh; arm "a clean checkout at its pin passes" 0 ""
  fresh; git -C "$work/vendor" fetch -q --tags 2>/dev/null
  arm "a fetched tag is not a local commit" 0 ""
  fresh; git -C "$work/vendor" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m "written here"
  arm "a commit made inside the vendored checkout is refused" 1 "exist only here"
  fresh; git -C "$work/vendor" checkout -q -b local-work 2>/dev/null
  git -C "$work/vendor" -c user.name=arm -c user.email=arm@example.invalid commit -q --allow-empty -m "on a branch"
  git -C "$work/vendor" checkout -q "$pin" 2>/dev/null
  arm "a commit on a local branch is refused even with HEAD back at the pin" 1 "exist only here"
  fresh; git -C "$work/vendor" checkout -q HEAD~1 2>/dev/null
  arm "a checkout moved off its pin is refused" 1 "not at the pin"
  fresh; printf 'x\n' > "$work/vendor/stray.txt"
  arm "a file created inside the vendored checkout is refused" 1 "stray.txt"
  fresh; printf 'tracked\n' > "$work/vendor/seeded.txt"
  git -C "$work/vendor" -c user.name=arm -c user.email=arm@example.invalid add seeded.txt
  git -C "$work/vendor" -c user.name=arm -c user.email=arm@example.invalid commit -q -m "seeded"
  git -C "$work/vendor" push -q origin HEAD:refs/heads/seeded 2>/dev/null
  git -C "$work/vendor" fetch -q 2>/dev/null
  pin="$(git -C "$work/vendor" rev-parse HEAD)"
  printf 'edited\n' > "$work/vendor/seeded.txt"
  arm "a modified tracked file is refused" 1 "seeded.txt"
  rm -rf "$work/vendor"
  arm "a submodule that is not checked out passes, and says so" 0 "not checked out"

  # The real tree, unchanged: the arm the others cannot stand in for.
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then
    ok=$((ok + 1)); echo "  ✅ the real tree's vendored checkouts pass"
  else
    echo "SELF-TEST: the real tree's vendored checkouts are refused — run the check to see why" >&2
  fi

  echo "repository-boundary self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked=0
while read -r path pin; do
  [ -n "${path:-}" ] || continue
  checked=$((checked + 1))
  check_pin "$path" "$pin"
done < <(pins)

if [ "$fail" -ne 0 ]; then
  echo "REPOSITORY-BOUNDARY: $fail breach(es) — every other repository is read-only (docs/decisions/decision_repository-boundary-read-only.md)" >&2
  exit 1
fi
echo "repository-boundary: OK ($checked vendored checkout(s) at their pins, with nothing written into them)"
exit 0
