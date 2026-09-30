#!/usr/bin/env bash
# scripts/build_book.sh — build the mdBook with the mdBook release it is pinned to, and refuse any other (leaf
# `PROGRAM.30`).
#
# ⭐ WHY THE VERSION IS CHECKED. The book is the director's window, and its links depend on how mdBook derives a
# heading's anchor: every `ledger.md#…` citation the `SOURCE-LEDGER` gate enforces is such an anchor. A release that
# derived them differently would build the same sources into a book whose links go nowhere, with no error. So the
# `book` step builds with the pinned release only, the one pin `MDBOOK_VERSION_PINNED` in `.github/ci-tools.env`
# (which `SOURCE-LEDGER` holds to the `mdbook` ledger entry, and which CI installs).
#
# CONTRACT: exit 0 = built with the pinned release; 1 = another release, or the build failed; 20 = no mdbook at all
# (the runner reports that as unavailable before calling this). `--self-test` runs the version check against stub
# mdbooks in `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

PINS=".github/ci-tools.env"
note() { printf 'build-book: %s\n' "$1" >&2; }

build() {
  local pinned have
  pinned="$(sed -n 's/^MDBOOK_VERSION_PINNED=//p' "$PINS" 2>/dev/null)"
  [ -n "$pinned" ] || { note "$PINS states no MDBOOK_VERSION_PINNED — nothing says which mdBook builds this book"; return 1; }
  command -v mdbook >/dev/null 2>&1 || { note "mdbook is not on PATH (the pin is $pinned)"; return 20; }
  have="$(mdbook --version 2>/dev/null)"
  if [ "$have" != "mdbook v$pinned" ]; then
    note "this mdbook reports '$have', and the book is pinned to 'mdbook v$pinned' ($PINS)"
    note "  another release may derive heading anchors differently, and every ledger.md#… link depends on them"
    return 1
  fi
  mdbook build docs/book
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/build_book/selftest" pinned
  pinned="$(sed -n 's/^MDBOOK_VERSION_PINNED=//p' "$PINS")"
  stub() { # $1 = what --version prints, $2 = exit code of a build
    rm -rf "$work"; mkdir -p "$work"
    printf '#!/usr/bin/env bash\n[ "$1" = --version ] && { echo "%s"; exit 0; }\nexit %s\n' "$1" "$2" > "$work/mdbook"
    chmod +x "$work/mdbook"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for none)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(PATH="$work:$PATH" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  stub "mdbook v$pinned" 0; arm "the pinned release builds" 0 ""
  stub "mdbook v$pinned" 1; arm "a failing build is a failure" 1 ""
  stub "mdbook v0.4.40" 0;  arm "another release is refused, before it builds anything" 1 "reports 'mdbook v0.4.40', and the book is pinned to 'mdbook v$pinned'"
  stub "mdbook v$pinned-rc.1" 0; arm "a release that only starts like the pin is another release" 1 "is pinned to 'mdbook v$pinned'"
  rm -rf "$work"
  echo "build-book self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") build; exit $? ;;
  *) note "unknown argument '$1' — the script takes none, or --self-test"; exit 2 ;;
esac
