#!/usr/bin/env bash
# S0-RETIREMENT — a prototype with a stated expiry cannot quietly become permanent.
#
# ⭐ WHY THIS EXISTS. ROADMAP.md §12 S0 permits a temporary implementation and attaches one
# condition to it: "By M4, the supported path replaces any temporary hard-coded assumptions with
# checked engine plans; NO HIDDEN SPECIAL-CASE GENERATOR IS GRANDFATHERED INTO THE RELEASE."
# That sentence describes a slow failure, not a dramatic one. A prototype that works becomes the
# thing everybody builds on, and the assumptions that were obvious while it was written become
# invisible a milestone later. Nobody decides to grandfather it; it just stops being noticed.
#
# Three things are made mechanical:
#   1. MARKED   — every `S0-ASSUMPTION: <id>` in the source appears in the retirement record.
#                 A list in a document drifts from the code; a marker cannot be moved without
#                 touching the line it annotates.
#   2. OWNED    — every assumption in the record names a task-tree leaf that actually exists.
#                 "Removed later" is not an owner; a leaf id is.
#   3. UNUSED   — the prototype crate is imported ONLY by its declared consumers, so the
#                 supported path cannot come to depend on it by accident. This is the leg that
#                 catches what "grandfathered" actually looks like in practice.
#
# ⚠️ HONEST LIMIT, stated rather than hidden: nothing here forces an author to WRITE a marker
# when they hard-code something new. It makes an unmarked assumption a thing someone chose not to
# record, rather than a thing nobody noticed — and it makes every recorded one impossible to lose.
# That is the same bargain the acceptance checklist makes.
#
# RETIRING S0: delete the markers, empty the record's table, delete crates/archogen-s0, and set
# the record to `retired`. This check passes at every step of that, and passes trivially once the
# crate is gone.
#
# CONTRACT (DOCTRINE_ENFORCEMENT.md): exit code is the verdict; explains on stderr;
# deterministic; read-only; path-agnostic. `--self-test` runs the RED arms.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

RECORD="docs/decisions/decision_s0-retirement.md"
CRATE="crates/archogen-s0"
# The only files permitted to import the prototype. A new entry here is a deliberate decision to
# widen the blast radius of something scheduled for deletion.
ALLOWED_CONSUMERS='^crates/archogen-cli/src/build_cmd\.rs$|^crates/archogen-cli/tests/s0_[a-z]+\.rs$'

fail=0
note() { printf 'S0-RETIREMENT: %s\n' "$1" >&2; fail=1; }

# Once the prototype is gone, so is the doctrine.
if [ ! -d "$CRATE" ]; then
  exit 0
fi

if [ ! -r "$RECORD" ]; then
  note "$CRATE exists but $RECORD does not — a prototype with no stated expiry is a permanent one"
  exit 1
fi

# ── 1. MARKED ────────────────────────────────────────────────────────────────────────────────
# Every id marked in the source must appear in the record's table.
markers="$(git grep -ho 'S0-ASSUMPTION: [a-z][a-z0-9-]*' -- crates/ 2>/dev/null \
  | sed 's/^S0-ASSUMPTION: //' | sort -u)"
if [ -z "$markers" ]; then
  note "no S0-ASSUMPTION markers found in crates/ — either the prototype has no hard-coded
  assumptions (delete the record's table and this check's subject) or the markers were removed"
fi
while IFS= read -r id; do
  [ -n "$id" ] || continue
  if ! grep -q "\`$id\`" "$RECORD"; then
    note "\`$id\` is marked in the source but is not listed in $RECORD"
  fi
done <<< "$markers"

# ── 2. OWNED ─────────────────────────────────────────────────────────────────────────────────
# Every row of the record's table names a leaf that exists in some task tree. Rows look like:
#   | `some-id` | prose | `M4.3` |
rows="$(grep -E '^\| `[a-z][a-z0-9-]*` \|' "$RECORD" || true)"
if [ -z "$rows" ]; then
  note "$RECORD lists no assumptions, but the source still carries markers"
fi
while IFS= read -r row; do
  [ -n "$row" ] || continue
  id="$(printf '%s' "$row" | sed -E 's/^\| `([^`]+)`.*/\1/')"
  leaf="$(printf '%s' "$row" | sed -E 's/.*\| `([A-Za-z0-9]+\.[0-9.]+)` \|[[:space:]]*$/\1/')"
  case "$leaf" in
    *'|'*|"$row")
      note "\`$id\` does not name an owning leaf — 'removed later' is not an owner, a leaf id is"
      continue
      ;;
  esac
  # The leaf must be declared by some tree, not merely mentioned.
  if ! grep -rqE "^- ID: \`$leaf\`" docs/tasks/; then
    note "\`$id\` names leaf \`$leaf\`, which no tree under docs/tasks/ declares"
  fi
  # …and the marker must exist, so the record cannot list an assumption the code dropped.
  if ! git grep -q "S0-ASSUMPTION: $id" -- crates/ 2>/dev/null; then
    note "$RECORD lists \`$id\`, but no source line is marked with it — remove the row, or
  restore the marker at the line that makes the assumption"
  fi
done <<< "$rows"

# ── 3. UNUSED ────────────────────────────────────────────────────────────────────────────────
# Nothing outside the declared consumers may import the prototype.
# ⛔ Matched on USE, not on mention. The first run of this check reported
# `crates/archogen-cli/src/lib.rs` because its module documentation names `archogen_s0` in prose
# — a false positive, and the kind that teaches authors to route around a gate. A dependency is
# `use archogen_s0`, `archogen_s0::…` or `extern crate`; naming the crate in a sentence is not
# one, and a doctrine that cannot tell them apart makes documentation expensive.
consumers="$(git grep -lE 'use archogen_s0|archogen_s0::|extern crate archogen_s0' -- crates/ \
  2>/dev/null | grep -v "^$CRATE/" | sort -u)"
while IFS= read -r file; do
  [ -n "$file" ] || continue
  # The CLI manifest names it as a dependency; that is how the one real consumer reaches it.
  [ "$file" = "crates/archogen-cli/Cargo.toml" ] && continue
  if ! printf '%s' "$file" | grep -qE "$ALLOWED_CONSUMERS"; then
    note "$file imports the S0 prototype. Only its declared consumers may — the supported path
  must not come to depend on a crate scheduled for deletion. If this is deliberate, widen
  ALLOWED_CONSUMERS in this check and say why in $RECORD"
  fi
done <<< "$consumers"

# ── RED arms ─────────────────────────────────────────────────────────────────────────────────
# A check that has only ever been seen green has not been shown to check anything (TOOLBOX.md).
if [ "${1:-}" = "--self-test" ]; then
  tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
  arms=0; arms_ok=0

  # Arm 1: a marker with no row in the record.
  arms=$((arms + 1))
  cp "$RECORD" "$tmp/record.bak"
  sed -E 's/^\| `horizon-is-one-hyperperiod`.*$//' "$RECORD" > "$tmp/record.new"
  cp "$tmp/record.new" "$RECORD"
  if "$0" 2>/dev/null; then
    echo "SELF-TEST: arm 1 did not fail on an unlisted marker" >&2
  else
    arms_ok=$((arms_ok + 1))
  fi
  cp "$tmp/record.bak" "$RECORD"

  # Arm 2: a row naming a leaf no tree declares.
  arms=$((arms + 1))
  sed -E 's/^(\| `no-lock-data` \|[^|]*\| )`M4\.1`/\1`M9.9`/' "$RECORD" > "$tmp/record.new"
  cp "$tmp/record.new" "$RECORD"
  if "$0" 2>/dev/null; then
    echo "SELF-TEST: arm 2 did not fail on a leaf no tree declares" >&2
  else
    arms_ok=$((arms_ok + 1))
  fi
  cp "$tmp/record.bak" "$RECORD"

  # Arm 3: a row whose marker no longer exists in the source.
  arms=$((arms + 1))
  sed -E 's/^\| `no-lock-data`/| `no-such-assumption`/' "$RECORD" > "$tmp/record.new"
  cp "$tmp/record.new" "$RECORD"
  if "$0" 2>/dev/null; then
    echo "SELF-TEST: arm 3 did not fail on a row with no marker" >&2
  else
    arms_ok=$((arms_ok + 1))
  fi
  cp "$tmp/record.bak" "$RECORD"

  echo "s0-retirement self-test: $arms_ok pass / $((arms - arms_ok)) fail"
  [ "$arms_ok" -eq "$arms" ] || exit 1
  exit 0
fi

[ "$fail" -eq 0 ] || exit 1
echo "s0-retirement: OK ($(printf '%s\n' "$markers" | grep -c . ) assumption(s), each marked, listed and owned)"
exit 0
