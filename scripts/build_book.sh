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
# ⭐ THE API REFERENCE (leaf `PROGRAM.107`). After the book, rustdoc documents the crates `docs/book/api.crates` lists,
# warnings denied — an undocumented public item or a doc link that resolves nowhere fails the book — into the book's
# `api/`, and every link a chapter makes into `api/` must then resolve in what was built. The book is the contract;
# the reference is a view of the code, generated with it and never committed.
#
# CONTRACT: exit 0 = built with the pinned release; 1 = another release, or the build failed; 20 = no mdbook at all
# (the runner reports that as unavailable before calling this). `--self-test` runs the version check against stub
# mdbooks, and the reference against a stub cargo, in `target/doctrine_scratch/`; `BOOK_OUT` names the book's output
# there, `docs/book/book` otherwise.
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
  # The pinned parser silently changes code/prose boundaries for these source
  # forms. Refuse them with a location before invoking the renderer; never rewrite
  # literal evidence (M3.6.9.6.3.2.2.15.2.1.4.3.4.0).
  python3 -I -B scripts/check_book_sources.py || return 1
  mdbook build docs/book || return 1
  api_reference "${BOOK_OUT:-docs/book/book}"
}

# The reference of the crates `docs/book/api.crates` lists, into `$1/api`, and every chapter's link into `api/` held to
# it. `target/book-api` is the build's own directory, so no other build's documentation is copied with it.
api_reference() {
  local out="$1/api" target="$ROOT/target/book-api" args=() crate link missing=0
  while IFS= read -r crate; do args+=(-p "$crate"); done < <(grep -vE '^[[:space:]]*(#|$)' docs/book/api.crates)
  [ "${#args[@]}" -gt 0 ] || { note "docs/book/api.crates lists no crate"; return 1; }
  # A crate taken off the list must not leave its old pages to be copied and published.
  rm -rf "$target/doc"
  if ! RUSTDOCFLAGS="-D warnings" cargo doc --quiet --offline --no-deps --target-dir "$target" "${args[@]}"; then
    note "rustdoc refused the API reference: a public item without documentation, or a doc link to nothing"
    return 1
  fi
  rm -rf "$out" && mkdir -p "$out" && cp -R "$target/doc/." "$out/" || {
    note "the reference could not be copied into $out"; return 1; }
  while IFS= read -r link; do
    [ -e "$1/$link" ] || { note "a chapter links \`$link\`, which the API reference does not hold"; missing=1; }
  done < <(grep -ohE '\]\(api/[^)#[:space:]]+' docs/book/src/*.md | sed 's/^](//' | sort -u)
  return "$missing"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/build_book/selftest" pinned
  pinned="$(sed -n 's/^MDBOOK_VERSION_PINNED=//p' "$PINS")"
  stub() { # $1 = what --version prints, $2 = exit code of a build, $3 = what the stub cargo's reference holds
    rm -rf "$work"; mkdir -p "$work"
    printf '#!/usr/bin/env bash\n[ "$1" = --version ] && { echo "%s"; exit 0; }\nexit %s\n' "$1" "$2" > "$work/mdbook"
    chmod +x "$work/mdbook"
    # A cargo that writes, under `--target-dir`, an index for each crate in $3 — or fails, for `fail`.
    printf '#!/usr/bin/env bash\n[ "%s" = fail ] && exit 101\nwhile [ "$1" != --target-dir ]; do shift; done\nrm -rf "$2/doc"; mkdir -p "$2/doc"\nfor c in %s; do mkdir -p "$2/doc/$c" && : > "$2/doc/$c/index.html"; done\n' \
      "${3:-}" "${3:-}" > "$work/cargo"
    chmod +x "$work/cargo"
  }
  linked="$(grep -ohE '\]\(api/[^/)]+/' docs/book/src/*.md | sed 's/^](api\///; s/\/$//' | sort -u | tr '\n' ' ')"
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for none)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(PATH="$work:$PATH" BOOK_OUT="$work/out" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  stub "mdbook v$pinned" 0 "$linked"; arm "the pinned release builds, the reference holding every page a chapter links" 0 ""
  stub "mdbook v$pinned" 1; arm "a failing build is a failure" 1 ""
  stub "mdbook v$pinned" 0 fail; arm "rustdoc refusing the reference fails the book" 1 "rustdoc refused the API reference"
  stub "mdbook v$pinned" 0 ""; arm "a chapter's link into a reference that lacks its page fails the book" 1 "which the API reference does not hold"
  stub "mdbook v0.4.40" 0;  arm "another release is refused, before it builds anything" 1 "reports 'mdbook v0.4.40', and the book is pinned to 'mdbook v$pinned'"
  stub "mdbook v$pinned-rc.1" 0; arm "a release that only starts like the pin is another release" 1 "is pinned to 'mdbook v$pinned'"
  arms=$((arms + 1))
  if python3 -I -B scripts/check_book_sources.py --self-test; then
    ok=$((ok + 1)); echo "  ✅ source controls and actual wrapper refusal/recovery pass"
  else
    echo "SELF-TEST: source controls or actual wrapper refusal/recovery failed" >&2
  fi
  rm -rf "$work"
  echo "build-book self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") build; exit $? ;;
  *) note "unknown argument '$1' — the script takes none, or --self-test"; exit 2 ;;
esac
