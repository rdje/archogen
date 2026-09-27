#!/usr/bin/env bash
# LS-001 re-measurement — does the documented vendoring layout resolve inside a Cargo workspace?
#
# This is NOT repro.sh. repro.sh freezes the original observation and, in its second half, PATCHES A
# DEPENDENCY MANIFEST to show the proposed fix working. That half must not be re-run: the guide at the
# adopted revision forbids exactly that workaround ("do not modify dependency manifests or add vendored
# crates to the application's workspace members to work around this error"), and a consumer has no
# business editing a vendored checkout. This instrument measures the documented route only, and it
# writes nothing anywhere — every probe is `cargo metadata --no-deps --offline`.
#
#   usage: remeasure.sh /path/to/linkedspec [--app-root <dir>]
#          remeasure.sh --self-test
#
# --app-root defaults to the repository root this script lives in, i.e. the consuming application's
# workspace root — the layout the report was about. Printed paths are relative, so a frozen run stays
# portable to another checkout.
#
# What is measured, for the two manifests the documented route touches (the integration example a
# consumer builds, and the nested PGEN package RGX's public bootstrap builds):
#   P1 does it resolve from the application workspace root?
#   P2 does it carry its own `[workspace]` boundary — the vendored half of the remedy?
#   P3 does the application root carry the documented `exclude` entry — the consumer half?
# P2 and P3 are recorded so the verdict says WHICH half carries it, rather than implying both.
#
# CONTRACT (exit code is the verdict):
#   0 = the defect is GONE — the documented layout resolves inside a Cargo workspace
#   1 = the defect is STILL PRESENT — a manifest the documented route touches still collides
#   2 = could not run, or not applicable: no cargo, a manifest missing, an application root with no
#       [workspace] at all (where the collision cannot occur), or a failure that is NOT the collision
#       — which is reported as such rather than counted as this defect
set -uo pipefail

APP_ROOT=""
CHECKOUT=""
SELF_TEST=0

while [ $# -gt 0 ]; do
  case "$1" in
    --app-root)  APP_ROOT="${2:?--app-root needs a directory}"; shift 2 ;;
    --self-test) SELF_TEST=1; shift ;;
    -h|--help)   sed -n '2,28p' "$0"; exit 0 ;;
    -*)          echo "LS-001 remeasure: unknown argument: $1" >&2; exit 2 ;;
    *)           [ -z "$CHECKOUT" ] || { echo "LS-001 remeasure: unexpected argument: $1" >&2; exit 2; }
                 CHECKOUT="$1"; shift ;;
  esac
done

COLLISION="believes it's in a workspace when it's not"

# display <base-dir> <path> — a portable spelling: relative to base, or <checkout>/relative.
display() {
  case "$2" in
    "$1"/*) printf '%s' "${2#"$1"/}" ;;
    *)      printf '<checkout>/%s' "${2##*/}" ;;
  esac
}

# probe <app_root> <manifest> — prints one indented observation; returns
#   0 resolved · 1 collided · 2 failed for another reason
probe() {
  local app="$1" m="$2" out rc
  out="$( cd "$app" && cargo metadata --format-version 1 --no-deps --offline --manifest-path "$m" 2>&1 >/dev/null )"
  rc=$?
  if [ "$rc" -eq 0 ]; then
    printf '      resolves (rc=0)\n'; return 0
  fi
  if printf '%s' "$out" | grep -qF "$COLLISION"; then
    printf '      COLLIDES (rc=%s)\n' "$rc"; return 1
  fi
  printf '      FAILED FOR ANOTHER REASON (rc=%s) — %s\n' "$rc" "$(printf '%s' "$out" | head -1)"; return 2
}

# app_excludes <app_root> <relative-path> — the consumer half of the remedy.
app_excludes() {
  local manifest="$1/Cargo.toml" rel="$2"
  [ -f "$manifest" ] || { echo "no-root-manifest"; return; }
  grep -qE '^[[:space:]]*\[workspace\]' "$manifest" || { echo "no-workspace"; return; }
  # The exclude array may wrap across lines, so flatten before matching.
  if tr '\n' ' ' < "$manifest" | grep -qE "exclude[[:space:]]*=[[:space:]]*\[[^]]*\"$rel\""; then
    echo yes
  else
    echo no
  fi
}

# measure <label> <app_root> <checkout|none> <manifest>...
measure() {
  local label="$1" app="$2" checkout="$3"; shift 3
  local m rel excluded own collisions=0 others=0 probed=0 own_yes=0

  command -v cargo >/dev/null || { echo "LS-001 remeasure: cargo not found" >&2; return 2; }
  [ -d "$app" ] || { echo "LS-001 remeasure: app root not found: $app" >&2; return 2; }
  for m in "$@"; do
    [ -f "$m" ] || {
      echo "LS-001 remeasure: manifest not found: $(display "$app" "$m")" >&2
      echo "  (a nested package needs its submodule initialized first)" >&2
      return 2
    }
  done

  # The exclude entry is spelled relative to the application root.
  case "$checkout" in
    none)   rel="vendor" ;;
    "$app"/*) rel="${checkout#"$app"/}" ;;
    *)      rel="vendor/linkedspec" ;;
  esac
  excluded="$(app_excludes "$app" "$rel")"
  case "$excluded" in
    no-workspace)
      echo "== LS-001 re-measurement: $label =="
      echo "  the application root has no [workspace] — the collision cannot occur here; not applicable"
      return 2 ;;
    no-root-manifest)
      echo "LS-001 remeasure: no Cargo.toml at the application root: $app" >&2; return 2 ;;
  esac

  echo "== LS-001 re-measurement: $label =="
  echo "  application workspace root carries exclude = [\"$rel\"] ... $excluded"
  echo "  manifests the documented route touches:"
  for m in "$@"; do
    probed=$((probed + 1))
    own="no"; grep -qE '^[[:space:]]*\[workspace\]' "$m" && { own="yes"; own_yes=$((own_yes + 1)); }
    printf '  - %s\n' "$(display "$app" "$m")"
    printf '      own [workspace] boundary: %s\n' "$own"
    probe "$app" "$m"; rc=$?
    case "$rc" in
      1) collisions=$((collisions + 1)) ;;
      2) others=$((others + 1)) ;;
    esac
  done
  echo "  probed $probed · collisions $collisions · other failures $others · own boundaries $own_yes"
  echo

  if [ "$others" -ne 0 ]; then
    echo "LS-001 remeasure: RESULT — could not decide: a manifest failed for a reason that is not this defect."
    return 2
  fi
  if [ "$collisions" -eq 0 ]; then
    echo "LS-001 remeasure: RESULT — the defect is GONE. Every manifest the documented route touches"
    echo "  resolves inside the application's Cargo workspace."
    if [ "$excluded" = yes ] && [ "$own_yes" -gt 0 ]; then
      echo "  Carried by BOTH halves of the remedy: $own_yes of $probed vendored manifests declare their own"
      echo "  [workspace] boundary, and the application root excludes the vendored tree."
    elif [ "$excluded" = yes ]; then
      echo "  Carried by the application root's documented exclude entry alone: no probed manifest"
      echo "  declares its own boundary."
    elif [ "$own_yes" -eq "$probed" ]; then
      echo "  Carried by the vendored manifests' own [workspace] boundaries — no app-root exclusion needed."
    else
      echo "  Carried without either half of the documented remedy being visible here: $own_yes of $probed"
      echo "  manifests declare a boundary and the application root excludes nothing. Treat this verdict"
      echo "  as unexplained rather than as a pass."
      return 2
    fi
    return 0
  fi
  echo "LS-001 remeasure: RESULT — the defect is STILL PRESENT: $collisions manifest(s) collide with the"
  echo "  application workspace. Cargo's own message names the two remedies: an exclude entry in the"
  echo "  application root, or an empty [workspace] table in the vendored manifest."
  return 1
}

# ── self-test: synthetic layouts, no LinkedSpec checkout, no network, no writes outside temp ──
if [ "$SELF_TEST" -eq 1 ]; then
  command -v cargo >/dev/null || { echo "LS-001 remeasure: the self-test needs cargo" >&2; exit 2; }
  t="$(mktemp -d)"; trap 'rm -rf "$t"' EXIT
  arms=0; ok=0
  arm() { # $1 expected exit, $2 label, $3 app root, $4 manifest
    arms=$((arms + 1))
    measure "$2" "$3" none "$4" >/dev/null 2>&1; rc=$?
    if [ "$rc" -eq "$1" ]; then ok=$((ok + 1)); echo "  arm passes: $2 (exit $rc)"
    else echo "  ARM FAILED: $2 — expected exit $1, got $rc" >&2; fi
  }

  # A consuming application that IS a Cargo workspace — the ordinary shape, and the report's case.
  mkapp() { # $1 dir, $2 extra [workspace] lines
    mkdir -p "$1/app/src"
    printf '[package]\nname = "app"\nversion = "0.1.0"\nedition = "2021"\n' > "$1/app/Cargo.toml"
    echo 'fn main() {}' > "$1/app/src/main.rs"
    printf '[workspace]\nresolver = "2"\nmembers = ["app"]\n%s\n' "$2" > "$1/Cargo.toml"
  }
  mkpkg() { # $1 manifest path, $2 extra tables
    mkdir -p "$(dirname "$1")/src"; echo 'fn main() {}' > "$(dirname "$1")/src/main.rs"
    printf '[package]\nname = "vendored"\nversion = "0.1.0"\nedition = "2021"\n%s\n' "$2" > "$1"
  }

  # 1. the reported shape: no vendored boundary, no app-root exclusion -> collision
  mkapp "$t/a1" ""; mkpkg "$t/a1/vendor/pkg/Cargo.toml" ""
  arm 1 "no vendored boundary and no app-root exclusion COLLIDES (the reported defect)" \
        "$t/a1" "$t/a1/vendor/pkg/Cargo.toml"

  # 2. the consumer's documented remedy: exclude the vendored directory at the application root
  mkapp "$t/a2" 'exclude = ["vendor"]'; mkpkg "$t/a2/vendor/pkg/Cargo.toml" ""
  arm 0 "the documented app-root exclude entry resolves it" \
        "$t/a2" "$t/a2/vendor/pkg/Cargo.toml"

  # 3. the vendored remedy: the package declares its own workspace boundary
  mkapp "$t/a3" ""; mkpkg "$t/a3/vendor/pkg/Cargo.toml" '
[workspace]'
  arm 0 "a vendored manifest with its own [workspace] boundary resolves it" \
        "$t/a3" "$t/a3/vendor/pkg/Cargo.toml"

  # 4. a failure that is NOT the collision must not be counted as this defect
  mkapp "$t/a4" 'exclude = ["vendor"]'
  mkdir -p "$t/a4/vendor/broken/src"; echo 'fn main() {}' > "$t/a4/vendor/broken/src/main.rs"
  printf 'this is not a manifest\n' > "$t/a4/vendor/broken/Cargo.toml"
  arm 2 "a manifest that fails for another reason is refused, not counted as the defect" \
        "$t/a4" "$t/a4/vendor/broken/Cargo.toml"

  # 5. an application with no enclosing workspace: the collision cannot occur, so the instrument
  #    must decline rather than report a fix it did not observe.
  mkdir -p "$t/a5/vendor/pkg/src"; echo 'fn main() {}' > "$t/a5/vendor/pkg/src/main.rs"
  printf '[package]\nname = "app"\nversion = "0.1.0"\nedition = "2021"\n' > "$t/a5/Cargo.toml"
  mkpkg "$t/a5/vendor/pkg/Cargo.toml" ""
  arm 2 "an application root with no [workspace] is declined as not applicable" \
        "$t/a5" "$t/a5/vendor/pkg/Cargo.toml"

  echo "self-test: $ok/$arms arms passed"
  [ "$ok" -eq "$arms" ] || exit 1
  exit 0
fi

[ -n "$CHECKOUT" ] || { echo "LS-001 remeasure: needs a LinkedSpec checkout (or --self-test)" >&2; exit 2; }
[ -d "$CHECKOUT/examples/integration/rust" ] || { echo "LS-001 remeasure: not a LinkedSpec checkout: $CHECKOUT" >&2; exit 2; }
if [ -z "$APP_ROOT" ]; then
  here="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd -P)"
  APP_ROOT="$(git -C "$here" rev-parse --show-toplevel 2>/dev/null || true)"
  [ -n "$APP_ROOT" ] || { echo "LS-001 remeasure: cannot derive the app root; pass --app-root <dir>" >&2; exit 2; }
fi
CHECKOUT_ABS="$(cd -- "$CHECKOUT" && pwd -P)"
rev="$(git -C "$CHECKOUT_ABS" rev-parse --short HEAD 2>/dev/null || echo 'unknown revision')"
measure "${CHECKOUT##*/} at $rev" "$APP_ROOT" "$CHECKOUT_ABS" \
  "$CHECKOUT_ABS/examples/integration/rust/Cargo.toml" \
  "$CHECKOUT_ABS/rgx/subs/pgen/rust/Cargo.toml"
exit $?
