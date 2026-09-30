#!/usr/bin/env bash
# scripts/check_version_register.sh — VERSION-REGISTER: everything versioned is in the register, at the value the
# code declares (leaf `PROGRAM.6.1`).
#
# ⭐ WHY THIS EXISTS. ROADMAP.md §15 separates the versions of the language, the profile, the engine, catalog
# entries, models and evidence formats, and requires that a description keep its meaning under its locked
# version. That is only checkable if every versioned surface is written down with its change rule and what pins
# it — `docs/book/src/versions.md` — and if a new identifier or a bump cannot land without that page moving.
#
# THE POPULATION is derived from the code, never listed here:
#   - every `pub const X: &str = "<name>/<n>…"` under `crates/*/src` — a format identifier;
#   - every `pub const X_VERSION: &str = "<x.y.z>"` — a version constant;
#   - every `pub const X: Version = Version { major: <n>, minor: <n> }` — an API version, as `<n>.<n>` (leaf
#     `API.3.4`: the engine API's is a major and a minor, and a shape the register could not see would let a
#     bump land unrecorded);
#   - every `id: "<name>-v<n>"` under `crates/*/src` — a profile id;
#   - the engine version, `version = "…"` in every workspace member's manifest, which must all agree.
# Each is claimed by exactly one register entry (`Declared at`: `const:<file>:<NAME>`, `profile:<file>:<id>`,
# `manifests`) whose `Version` carries the value; every entry's declaration must still exist.
#
# ⚠️ HONEST LIMIT: it checks that a version is recorded and matches; it cannot check that a change which should
# have bumped a version did. That is what each entry's `Pinned by` is for, and where it says "pending", nothing is.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

REGISTER="docs/book/src/versions.md"
FIELDS="Surface|Version|Declared at|Changes when|Pinned by|Keeps"

fail=0
note() { printf 'VERSION-REGISTER: %s\n' "$1" >&2; fail=$((fail + 1)); }

# The register as `id<TAB>field<TAB>value`, sections `## \`<id>\``.
entries() {
  awk '
    /^## `[a-z0-9-]+`[[:space:]]*$/ { id = $0; sub(/^## `/, "", id); sub(/`.*$/, "", id); print id "\t\t"; next }
    /^## / { id = ""; next }
    id != "" && /^\| [^|]+ \| .* \|[[:space:]]*$/ {
      line = $0; sub(/^\| /, "", line); sub(/ \|[[:space:]]*$/, "", line)
      k = index(line, " | "); if (k == 0) next
      f = substr(line, 1, k - 1); v = substr(line, k + 3)
      if (f == "Field" || f ~ /^-+$/) next
      print id "\t" f "\t" v
    }' "$REGISTER"
}

# Every workspace member directory the root manifest declares, globs expanded.
members() {
  awk '/^members[[:space:]]*=/{f=1} f{print} f&&/\]/{exit}' Cargo.toml | tr -d '[]"' |
    sed 's/^members[[:space:]]*=//' | tr ',' '\n' | while IFS= read -r entry; do
      entry="$(printf '%s' "$entry" | tr -d '[:space:]')"; [ -n "$entry" ] || continue
      for dir in $entry; do [ -f "$dir/Cargo.toml" ] && printf '%s\n' "${dir%/}"; done
    done
}

# Every version the code declares, as `locator<TAB>value`.
declared() {
  local f
  while IFS= read -r f; do
    [ -f "$f" ] || continue
    sed -nE 's/^[[:space:]]*pub const ([A-Z0-9_]+): &str = "([a-z][a-z0-9-]*\/[0-9]+)[^"]*";.*/\1\t\2/p' "$f" |
      while IFS=$'\t' read -r name value; do printf 'const:%s:%s\t%s\n' "$f" "$name" "$value"; done
    sed -nE 's/^[[:space:]]*pub const ([A-Z0-9_]*VERSION): &str = "([0-9]+\.[0-9]+\.[0-9]+)";.*/\1\t\2/p' "$f" |
      while IFS=$'\t' read -r name value; do printf 'const:%s:%s\t%s\n' "$f" "$name" "$value"; done
    sed -nE 's/^[[:space:]]*pub const ([A-Z0-9_]+): Version = Version \{ major: ([0-9]+), minor: ([0-9]+) \};.*/\1\t\2.\3/p' "$f" |
      while IFS=$'\t' read -r name value; do printf 'const:%s:%s\t%s\n' "$f" "$name" "$value"; done
    sed -nE 's/^[[:space:]]*id: "([a-z0-9-]+-v[0-9]+)",.*/\1/p' "$f" |
      while IFS= read -r id; do printf 'profile:%s:%s\t%s\n' "$f" "$id" "$id"; done
  done < <(git ls-files -- 'crates/*/src/*.rs' 'crates/*/src/**/*.rs')
  local versions
  versions="$(members | while IFS= read -r d; do
    awk '/^\[package\]/{p=1; next} /^\[/{p=0} p && /^version[[:space:]]*=/{print; exit}' "$d/Cargo.toml" |
      sed -E 's/.*"([^"]+)".*/\1/'; done | sort -u)"
  if [ "$(printf '%s\n' "$versions" | grep -c .)" -eq 1 ]; then
    printf 'manifests\t%s\n' "$versions"
  elif [ -n "$versions" ]; then
    printf 'manifests\t%s\n' "DISAGREE:$(printf '%s' "$versions" | paste -sd, -)"
  fi
}

scan() {
  if [ ! -f "$REGISTER" ]; then
    note "$REGISTER is missing — nothing records what is versioned"; return
  fi
  local table ids held claims
  table="$(entries)"
  ids="$(printf '%s\n' "$table" | awk -F'\t' '$2 == "" && $1 != "" { print $1 }')"
  if [ -z "$ids" ]; then
    note "$REGISTER has no entry — an empty register is a breach, not a pass"; return
  fi
  [ -z "$(printf '%s\n' "$ids" | sort | uniq -d)" ] || note "an entry appears twice: $(printf '%s\n' "$ids" | sort | uniq -d | paste -sd, -)"
  while IFS= read -r msg; do [ -n "$msg" ] && note "$msg"; done < <(printf '%s\n' "$table" | FIELDS="$FIELDS" awk -F'\t' '
    $2 == "" && $1 != "" { order[++n] = $1; next }
    $1 != "" { v[$1 SUBSEP $2] = $3 }
    END {
      m = split(ENVIRON["FIELDS"], want, "|")
      for (i = 1; i <= n; i++) {
        id = order[i]
        for (j = 1; j <= m; j++) { x = v[id SUBSEP want[j]]; gsub(/[[:space:]]/, "", x); if (x == "") print "entry `" id "` has no `" want[j] "`" }
        if (v[id SUBSEP "Declared at"] !~ /`(const:[^`]+|profile:[^`]+|manifests)`/) print "entry `" id "`: Declared at names no declaration"
      }
    }')
  claims="$(printf '%s\n' "$table" | awk -F'\t' '$2 == "Declared at" { s = $3
    while (match(s, /`(const:[^`]+|profile:[^`]+|manifests)`/)) { print $1 "\t" substr(s, RSTART + 1, RLENGTH - 2); s = substr(s, RSTART + RLENGTH) } }')"
  held="$(declared)"
  local loc val owners n version
  while IFS=$'\t' read -r loc val; do
    [ -n "$loc" ] || continue
    if [ "${val#DISAGREE:}" != "$val" ]; then
      note "the workspace members disagree on the engine version (${val#DISAGREE:}) — one engine, one version"; continue
    fi
    owners="$(printf '%s\n' "$claims" | awk -F'\t' -v l="$loc" '$2 == l { print $1 }')"
    n="$(printf '%s' "$owners" | grep -c . || true)"
    if [ "$n" -eq 0 ]; then
      note "the code declares $loc = '$val', and no entry of $REGISTER claims it — add one"
    elif [ "$n" -gt 1 ]; then
      note "$loc is claimed by more than one entry: $(printf '%s' "$owners" | paste -sd, -)"
    else
      version="$(printf '%s\n' "$table" | awk -F'\t' -v id="$owners" '$1 == id && $2 == "Version" { print $3; exit }')"
      printf '%s' "$version" | grep -qF -- "$val" ||
        note "entry \`$owners\` does not carry the declared value of $loc — the code says '$val'; move the entry with the code"
    fi
  done <<< "$held"
  while IFS=$'\t' read -r id loc; do
    [ -n "$loc" ] || continue
    printf '%s\n' "$held" | awk -F'\t' -v l="$loc" '$1 == l { found = 1 } END { exit !found }' ||
      note "entry \`$id\` names the declaration $loc, which the code no longer has — the entry is stale"
  done <<< "$claims"
  checked_entries="$(printf '%s\n' "$ids" | grep -c .)"
  checked_declared="$(printf '%s\n' "$held" | grep -c . || true)"
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/version_register/selftest"
  entry() { # $1 id, $2 version, $3 declared at
    printf '\n## `%s`\n\n| Field | Value |\n| --- | --- |\n| Surface | the %s |\n| Version | %s |\n| Declared at | %s |\n| Changes when | it changes |\n| Pinned by | a test |\n| Keeps | its meaning |\n' "$1" "$1" "$2" "$3"
  }
  fresh() {
    rm -rf "$work"; mkdir -p "$work/crates/a/src" "$work/crates/b/src" "$work/docs/book/src"
    git -C "$work" init -q
    printf '[workspace]\nmembers = ["crates/*"]\n' > "$work/Cargo.toml"
    printf '[package]\nname = "a"\nversion = "1.2.3"\n' > "$work/crates/a/Cargo.toml"
    printf '[package]\nname = "b"\nversion = "1.2.3"\n' > "$work/crates/b/Cargo.toml"
    printf 'pub const FORMAT: &str = "thing/1";\npub const IMPL_VERSION: &str = "0.4.0";\n' > "$work/crates/a/src/lib.rs"
    printf 'const P: Profile = Profile {\n    id: "core-v1",\n};\npub const API: Version = Version { major: 2, minor: 1 };\n' > "$work/crates/b/src/lib.rs"
    { printf '# Versions\n'
      entry format '`thing/1`' '`const:crates/a/src/lib.rs:FORMAT`'
      entry impl '`0.4.0`' '`const:crates/a/src/lib.rs:IMPL_VERSION`'
      entry profile '`core-v1`' '`profile:crates/b/src/lib.rs:core-v1`'
      entry api '`2.1`' '`const:crates/b/src/lib.rs:API`'
      entry engine '`1.2.3`' '`manifests`'
      printf '\n## Not versioned yet\n\n- models\n'
    } > "$work/$REGISTER"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a register carrying every declared version passes" 0 ""
  fresh; sed -i.bak 's/"thing\/1"/"thing\/2"/' "$work/crates/a/src/lib.rs"; rm -f "$work/crates/a/src/lib.rs.bak"
  arm "a format bumped without its entry is refused" 1 "the code says 'thing/2'"
  fresh; printf 'pub const WIRE: &str = "wire/1";\n' >> "$work/crates/a/src/lib.rs"
  arm "a new format identifier with no entry is refused" 1 "const:crates/a/src/lib.rs:WIRE = 'wire/1'"
  fresh; printf 'pub const PLAN_VERSION: &str = "2.0.0";\n' >> "$work/crates/b/src/lib.rs"
  arm "a new version constant with no entry is refused" 1 "const:crates/b/src/lib.rs:PLAN_VERSION = '2.0.0'"
  fresh; sed -i.bak 's/minor: 1 }/minor: 2 }/' "$work/crates/b/src/lib.rs"; rm -f "$work/crates/b/src/lib.rs.bak"
  arm "an API minor bumped without its entry is refused" 1 "the code says '2.2'"
  fresh; printf 'pub const WIRE_API: Version = Version { major: 1, minor: 0 };\n' >> "$work/crates/a/src/lib.rs"
  arm "a new API version with no entry is refused" 1 "const:crates/a/src/lib.rs:WIRE_API = '1.0'"
  fresh; sed -i.bak 's/core-v1/core-v2/' "$work/crates/b/src/lib.rs"; rm -f "$work/crates/b/src/lib.rs.bak"
  arm "a profile id changed without its entry is refused" 1 "profile:crates/b/src/lib.rs:core-v2"
  fresh; sed -i.bak 's/1.2.3/1.3.0/' "$work/crates/b/Cargo.toml"; rm -f "$work/crates/b/Cargo.toml.bak"
  arm "workspace members disagreeing on the engine version is refused" 1 "the workspace members disagree"
  fresh; for c in a b; do sed -i.bak 's/1.2.3/1.3.0/' "$work/crates/$c/Cargo.toml"; rm -f "$work/crates/$c/Cargo.toml.bak"; done
  arm "an engine release without its entry is refused" 1 "the code says '1.3.0'"
  fresh; printf 'pub const IMPL: &str = "x";\n' > "$work/crates/a/src/lib.rs"; printf 'pub const FORMAT: &str = "thing/1";\n' > "$work/crates/b/src/fmt.rs"
  git -C "$work" add -A
  arm "an entry whose declaration moved is stale" 1 "names the declaration const:crates/a/src/lib.rs:FORMAT, which the code no longer has"
  fresh; sed -i.bak '/^| Pinned by |/d' "$work/$REGISTER"; rm -f "$work/$REGISTER.bak"
  arm "an entry with a field missing is refused" 1 "has no \`Pinned by\`"
  fresh; printf 'fn f() {\n    // pub const X: &str = "fake/9";\n}\n' >> "$work/crates/a/src/lib.rs"
  arm "a commented-out constant is not a declaration" 0 ""
  fresh; printf '# Versions\n\nNothing yet.\n' > "$work/$REGISTER"
  arm "a register with no entry is a breach, not a pass" 1 "has no entry"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real register carries every declared version"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "version-register self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi

checked_entries=0; checked_declared=0
scan
if [ "$fail" -ne 0 ]; then
  echo "VERSION-REGISTER: $fail breach(es) — $REGISTER records what is versioned and what pins it (ROADMAP.md §15)" >&2
  exit 1
fi
echo "version-register: OK ($checked_entries entries; $checked_declared declared version(s), each carried by its entry)"
exit 0
