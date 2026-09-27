#!/usr/bin/env bash
# scripts/linkedspec_eval.sh — the one place this project derives LinkedSpec's runtime storage paths.
#
# ⛔ WHY THIS EXISTS. The vendor's Rust integration guide requires an application-owned data root,
#   cache root, scratch root, package store and target directory, all derived at runtime from the
#   application root, and says to keep those runtime-derived paths in a launcher rather than in a
#   manifest. Every LinkedSpec re-measurement leaf needs the same five values; deriving them by hand
#   in each one is how an off-volume path (a home cache, /tmp) sneaks in unnoticed.
#
# ⛔ WHAT THIS IS NOT. Not a build system for archogen. No crate in this workspace depends on
#   vendor/linkedspec, and this script never touches archogen's own Cargo workspace, its lockfile or
#   its crates. It exists to evaluate a VENDORED dependency through its published interface, for the
#   outbound feedback register in docs/feedback/linkedspec/. The guide's "copy the example source into
#   your application" route is deliberately NOT used: it would add serde_json and a path dependency on
#   the vendored runtime to this workspace, which the zero-dependency engine decision forbids. The
#   example is built in place instead, which is the route the guide's own "Reproduce the integration
#   checks" section uses.
#
#   usage: scripts/linkedspec_eval.sh env                  print the storage environment
#          scripts/linkedspec_eval.sh prepare              RGX's published bootstrap
#          scripts/linkedspec_eval.sh build [--network]    build the integration example's binaries
#          scripts/linkedspec_eval.sh bins                 print the built binaries that exist
#          scripts/linkedspec_eval.sh reference            the guide's own two-form input, checked
#          scripts/linkedspec_eval.sh run <bin> [args...]  run a built binary inside the environment
#
#   --network  allow the first package resolution to reach the registry. The default is
#              --offline --locked, which is what the guide says a prepared consumer should use.
#
# Exit: 0 success · 2 could not run (a prerequisite is missing) · otherwise the underlying command's
# status, which is the measurement and is never rewritten here.
set -uo pipefail

die() { echo "linkedspec_eval: $*" >&2; exit 2; }

ROOT="$(git rev-parse --show-toplevel 2>/dev/null)" || die "not inside a git repository"
CHECKOUT="$ROOT/vendor/linkedspec"
DATA="$ROOT/.app-data"
EXAMPLE_MANIFEST="examples/integration/rust/Cargo.toml"
DOCUMENT_SPEC="specs/SExprDocumentV1.spec"

[ -d "$CHECKOUT" ] || die "no vendored checkout at vendor/linkedspec"
PIN="$(git -C "$CHECKOUT" rev-parse --short HEAD 2>/dev/null || echo unknown)"

export LINKEDSPEC_PROJECT_DATA_ROOT="$DATA/linkedspec"
export LINKEDSPEC_CACHE_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/cache"
export LINKEDSPEC_SCRATCH_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/scratch"
export CARGO_HOME="$DATA/cargo-home"
# A target directory PER PIN. The guide warns that carrying one target directory across a revision
# change can leave a build using the previous revision's products; naming the directory after the pin
# makes that impossible to do by accident and keeps the older pin's build for comparison.
export CARGO_TARGET_DIR="$DATA/target-$PIN"

wrapper() { [ -f "$CHECKOUT/tools/$1" ] || die "the documented wrapper tools/$1 is missing from this checkout"; }

# The example declares its own workspace, so its binaries may land either in the pin-named target
# directory or in the checkout's own rust/target. Both are searched; nothing is assumed.
find_bin() { # $1 name
  local candidate
  for candidate in "$CARGO_TARGET_DIR/debug/$1" "$CHECKOUT/rust/target/debug/$1" \
                   "$CHECKOUT/examples/integration/rust/target/debug/$1"; do
    [ -x "$candidate" ] && { printf '%s\n' "$candidate"; return 0; }
  done
  return 1
}

cmd="${1:-}"; [ -n "$cmd" ] || { sed -n '2,32p' "$0"; exit 2; }
shift || true

case "$cmd" in
  env)
    echo "pin (vendor/linkedspec HEAD) ..... $PIN"
    echo "LINKEDSPEC_PROJECT_DATA_ROOT ..... $LINKEDSPEC_PROJECT_DATA_ROOT"
    echo "LINKEDSPEC_CACHE_ROOT ............ $LINKEDSPEC_CACHE_ROOT"
    echo "LINKEDSPEC_SCRATCH_ROOT .......... $LINKEDSPEC_SCRATCH_ROOT"
    echo "CARGO_HOME ....................... $CARGO_HOME"
    echo "CARGO_TARGET_DIR ................. $CARGO_TARGET_DIR"
    for d in "$LINKEDSPEC_PROJECT_DATA_ROOT" "$CARGO_HOME" "$CARGO_TARGET_DIR"; do
      printf '%-33s%s\n' "  exists:" "$([ -d "$d" ] && echo yes || echo no) $d"
    done
    ;;

  prepare)
    wrapper project_data_run.sh
    command -v make >/dev/null || die "make not found"
    mkdir -p "$LINKEDSPEC_PROJECT_DATA_ROOT"
    # The guide's exact form: the wrapper retains the local package store and temporary storage, and
    # `env -u CARGO_TARGET_DIR` lets the dependency's documented build use its own default target.
    CARGO_NET_OFFLINE="${LINKEDSPEC_EVAL_OFFLINE:-false}" \
      bash "$CHECKOUT/tools/project_data_run.sh" env -u CARGO_TARGET_DIR \
      make -C "$CHECKOUT/rgx" bootstrap
    exit $?
    ;;

  build)
    wrapper run_cargo_local.sh
    command -v cargo >/dev/null || die "cargo not found"
    offline=(--offline --locked)
    if [ "${1:-}" = --network ]; then offline=(); shift; fi
    mkdir -p "$CARGO_TARGET_DIR"
    bash "$CHECKOUT/tools/run_cargo_local.sh" build --bins "${offline[@]}" \
      --manifest-path "$EXAMPLE_MANIFEST"
    rc=$?
    echo
    if [ "$rc" -eq 0 ]; then
      echo "linkedspec_eval: binaries built at pin $PIN:"
      for b in sexpr_file lispish_file; do
        p="$(find_bin "$b")" && echo "  $b -> ${p#"$ROOT"/}" || echo "  $b -> NOT FOUND"
      done
    else
      echo "linkedspec_eval: build failed (rc=$rc)." >&2
      echo "  If the cause is a package missing from the retained store, re-run with --network:" >&2
      echo "  the guide allows network for the first resolution and --offline --locked afterwards." >&2
    fi
    exit "$rc"
    ;;

  bins)
    rc=0
    for b in sexpr_file lispish_file; do
      p="$(find_bin "$b")" && echo "$b -> ${p#"$ROOT"/}" || { echo "$b -> NOT FOUND (run: $0 build)" >&2; rc=2; }
    done
    exit "$rc"
    ;;

  reference)
    # The guide's own reference input and its documented result: two forms, tagged, quotes kept in
    # the string lexeme. A conformance check on the published contract, not on archogen's own inputs.
    bin="$(find_bin sexpr_file)" || die "sexpr_file is not built — run: $0 build"
    wrapper project_data_run.sh
    work="$DATA/reference-check"; mkdir -p "$work"
    printf '(v 1 "1")\n(done)\n' > "$work/document.sexp"
    out="$(bash "$CHECKOUT/tools/project_data_run.sh" "$bin" \
             --grammar "$CHECKOUT/$DOCUMENT_SPEC" "$work/document.sexp" 2>&1)"; rc=$?
    echo "exit status ....... $rc"
    echo "stdout ............ $out"
    if [ "$rc" -ne 0 ]; then echo "REFERENCE CHECK: could not run"; exit 2; fi
    for want in '"format":"linkedspec-sexpr-v1"' '"kind":"symbol","lexeme":"v"' \
                '"kind":"number","lexeme":"1"' '"kind":"string","lexeme":"\"1\""' \
                '"kind":"symbol","lexeme":"done"'; do
      if printf '%s' "$out" | grep -qF "$want"; then echo "  present: $want"
      else echo "  MISSING: $want"; echo "REFERENCE CHECK: the published result shape did not appear"; exit 1; fi
    done
    forms="$(printf '%s' "$out" | grep -o '"kind":"list"' | wc -l | tr -d ' ')"
    echo "  top-level forms . $forms (the guide documents two)"
    [ "$forms" -eq 2 ] || { echo "REFERENCE CHECK: expected two top-level forms"; exit 1; }
    echo "REFERENCE CHECK: the published two-form tagged document, exactly as documented"
    ;;

  run)
    bin="${1:-}"; [ -n "$bin" ] || die "usage: $0 run <sexpr_file|lispish_file|path> [args...]"
    shift
    wrapper project_data_run.sh
    case "$bin" in */*) path="$bin" ;; *) path="$(find_bin "$bin")" || die "$bin is not built — run: $0 build" ;; esac
    bash "$CHECKOUT/tools/project_data_run.sh" "$path" "$@"
    exit $?
    ;;

  -h|--help|help) sed -n '2,32p' "$0" ;;

  *) die "unknown command: $cmd (try: env, prepare, build, bins, reference, run)" ;;
esac
