#!/usr/bin/env bash
# scripts/catalog_check.sh — the catalog check's harness: the checker built from the base, run on the judged tree
# (leaf `M2.7.6.2`).
#
# ⭐ WHY THIS EXISTS. Premise 3 of `docs/specs/catalog/decision_catalog-records.md`: no code or file a pull request
# controls runs in the check, lies on the path of what it builds or runs, or reaches its credentials. What runs
# before the checker decides most of that, so it is written once, here, and tested against each construction the
# catalog's reviews found (rounds 12 to 15, `docs/reviews/decision_catalog-records-reviews.md`): build scripts and
# cargo configuration in a judged tree, where the trees sit, a target directory shared with the record builds, and
# rustup's toolchain file. The checker itself is `M2.7.4`'s; this script's tests use a stub.
#
# WHAT IT DOES, in order. Each step refuses, exit 3, and runs nothing further when its condition fails:
#   1. the checkout is the base: `HEAD` is `--base`, nothing in it modified or untracked, so every file of the
#      checkout on a build's path is the base's;
#   2. both trees are written from their blobs under `target/catalog-check/run/` — `checker/` from the base,
#      `judged/` from the judged commit — with `GIT_NO_REPLACE_OBJECTS` set and never checked out, so no filter,
#      attribute or hook of git's acts. A gitlink is not written. A symbolic link, a path outside §4's grammar or with
#      a `.git` segment, two paths equal but for ASCII case, and a cargo or rustup file name spelled otherwise are
#      refused (§3 of `docs/specs/catalog/decision_catalog-records-hashes.md`);
#   3. the pin is the base blob's `rust-toolchain.toml`: one `[toolchain]` table, `channel` a release number, no key
#      but `channel`, `components`, `targets` and `profile`. No judged toolchain file is read, because
#      `RUSTUP_TOOLCHAIN` is always set, and it outranks every file and override;
#   4. the environment is §3's allowlist — `PATH`, `HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN` set to the pin,
#      `CARGO_HOME` an empty directory of the harness's own, `CARGO_TARGET_DIR` — and nothing else, so no variable
#      the caller set reaches a build; `rustc -vV` must then name the pin;
#   5. every cargo configuration on the checker build's path, from `checker/` up to `/`, is listed before cargo runs
#      and must hold only `alias` keys, which cannot change a `cargo build` — cargo ignores an alias that shadows a
#      built-in command, measured `2026-10-01` on the pin. Only so: an alias's body can carry `--config`, and an
#      alias named `b`, `clippy` or `fmt` does run, so this holds because cargo is invoked here by `build`, a
#      built-in name, and nothing else (review round 1, V3). The judged tree is never on that path;
#   6. the checker is built in `checker/`, `--offline --locked --release`, into `checker-target/`, and run from there,
#      never copied, with `CARGO_TARGET_DIR` a fresh `records-target/`, so no build it makes can reach its binary.
#      Each argument after `--` has `{judged}` replaced by the judged tree's directory, `{base}` by the base's,
#      `{judged-commit}` by the judged commit's full object name and `{base-commit}` by the base's (`M2.7.4.2`).
# The verdict is the checker's exit code.
#
# ⚠️ OPEN, for the design's review (`M2.7.6.4`): the judged tree is written under the base checkout, so the
# checkout's `.cargo/config.toml` lies on the path of every build the checker makes in it. §3 admits outside the
# written index only "the tracked copies of the tree being judged", so as written a checker applying §3 would refuse
# every pull request that changes that file. The proposal is that §3 admit, as this script does, a configuration
# outside the written index that holds only `alias` keys — and only with the constraint step 5 states: the checker
# then invokes cargo by built-in subcommand names alone. Nothing depends on it until the checker exists.
#
# ⚠️ HONEST LIMIT: it cannot see the hosting — which commit is the base, whether the workflow running it is the
# pinned one, the token's scope. Those are the workflow's (`M2.7.6.3`) and the director's settings (findings §11).
#
# usage: bash scripts/catalog_check.sh --base <commit> --judged <commit> --checker <package>:<binary> [-- <arg>…]
#        bash scripts/catalog_check.sh --self-test
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/catalog_check"

harness() {
  python3 - "$@" <<'PY'
import os, re, shutil, subprocess, sys, tomllib

USAGE = "usage: bash scripts/catalog_check.sh --base <commit> --judged <commit> --checker <package>:<binary> [-- <arg>…]"

def refuse(message):
    print("catalog-check: refused — " + message, file=sys.stderr)
    sys.exit(3)

opts, extra, argv, i = {}, [], sys.argv[1:], 0
while i < len(argv):
    if argv[i] == "--":
        extra = argv[i + 1:]
        break
    if argv[i] in ("--base", "--judged", "--checker") and i + 1 < len(argv):
        opts[argv[i][2:]] = argv[i + 1]
        i += 2
        continue
    print(USAGE, file=sys.stderr)
    sys.exit(2)
package, _, binary = opts.get("checker", "").partition(":")
if set(opts) != {"base", "judged", "checker"} or not re.fullmatch(r"[A-Za-z0-9_-]+", package) \
        or not re.fullmatch(r"[A-Za-z0-9_-]+", binary):
    print(USAGE, file=sys.stderr)
    sys.exit(2)

ROOT = os.getcwd()
GITENV = dict(os.environ, GIT_NO_REPLACE_OBJECTS="1")

def git(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True, env=GITENV)

def commit(name):
    r = git("rev-parse", "--verify", "--quiet", name + "^{commit}")
    if r.returncode:
        refuse("`%s` names no commit" % name)
    return r.stdout.strip()

base, judged = commit(opts["base"]), commit(opts["judged"])

# 1. The checkout is the base, unmodified.
head = commit("HEAD")
if head != base:
    refuse("the checkout is not the base: `HEAD` is %s, the base %s" % (head[:12], base[:12]))
status = git("status", "--porcelain", "--untracked-files=all")
if status.returncode or status.stdout.strip():
    refuse("the checkout is modified or has untracked files:\n" + status.stdout[:400])

# 2. Both trees, written from their blobs.
WORK = os.path.join(ROOT, "target", "catalog-check", "run")
for d in (os.path.join(ROOT, "target"), os.path.dirname(WORK), WORK):
    if os.path.islink(d):
        refuse("`%s` is a symbolic link" % os.path.relpath(d, ROOT))
shutil.rmtree(WORK, ignore_errors=True)
if os.path.lexists(WORK):
    refuse("`%s` could not be emptied" % os.path.relpath(WORK, ROOT))
os.makedirs(WORK)
SEGMENT = re.compile(r"[A-Za-z0-9._+-]+")
NAMES = (".cargo/config", ".cargo/config.toml", "rust-toolchain", "rust-toolchain.toml", "Cargo.toml", "Cargo.lock",
         "build.rs")

def write(rev, dest, label):
    r = subprocess.run(["git", "ls-tree", "-r", "-z", "--full-tree", rev], capture_output=True, env=GITENV)
    if r.returncode:
        refuse("%s: its tree could not be listed" % label)
    files, folded, seen, dirs = [], {}, set(), set()
    for record in r.stdout.split(b"\0"):
        if not record:
            continue
        meta, raw = record.split(b"\t", 1)
        mode, _, sha = meta.decode().split(" ")
        try:
            path = raw.decode("ascii")
        except UnicodeDecodeError:
            refuse("%s: a path outside §4's grammar, %r" % (label, raw[:80]))
        if mode == "160000":
            continue
        if mode == "120000":
            refuse("%s: `%s` is a symbolic link, which is never written" % (label, path))
        if mode not in ("100644", "100755"):
            refuse("%s: `%s` has mode %s" % (label, path, mode))
        segments = path.split("/")
        if any(s in (".", "..") or s.lower() == ".git" or not SEGMENT.fullmatch(s) for s in segments):
            refuse("%s: `%s` is outside §4's grammar" % (label, path))
        for k in range(1, len(segments) + 1):
            prefix = "/".join(segments[:k])
            if folded.setdefault(prefix.lower(), prefix) != prefix:
                refuse("%s: `%s` and `%s` differ only in ASCII case" % (label, folded[prefix.lower()], prefix))
        for name in NAMES:
            n = name.count("/") + 1
            tail = "/".join(segments[-n:])
            if len(segments) >= n and tail.lower() == name.lower() and tail != name:
                refuse("%s: `%s` spells `%s` otherwise" % (label, path, name))
        # A tree git's own checks refuse, written by hand (review round 1, V5): refused, never half-written.
        if path in seen or path in dirs or any("/".join(segments[:k]) in seen for k in range(1, len(segments))):
            refuse("%s: `%s` is listed twice, or is both a file and a directory" % (label, path))
        seen.add(path)
        dirs.update("/".join(segments[:k]) for k in range(1, len(segments)))
        files.append((mode, sha, path))
    cat = subprocess.run(["git", "cat-file", "--batch"], input="".join(sha + "\n" for _, sha, _ in files).encode(),
                         capture_output=True, env=GITENV)
    out, pos = cat.stdout, 0
    for mode, sha, path in files:
        end = out.find(b"\n", pos)
        header = out[pos:end].decode(errors="replace").split(" ")
        if cat.returncode or len(header) != 3 or header[0] != sha or header[1] != "blob":
            refuse("%s: the blob of `%s` could not be read" % (label, path))
        size = int(header[2])
        target = os.path.join(dest, path)
        try:
            os.makedirs(os.path.dirname(target), exist_ok=True)
            with open(target, "wb") as fh:
                fh.write(out[end + 1:end + 1 + size])
            os.chmod(target, 0o755 if mode == "100755" else 0o644)
        except OSError as e:
            refuse("%s: `%s` could not be written: %s" % (label, path, e))
        pos = end + 1 + size + 1
    return {path for _, _, path in files}

CHECKER, JUDGED = os.path.join(WORK, "checker"), os.path.join(WORK, "judged")
os.makedirs(CHECKER)
os.makedirs(JUDGED)
base_paths = write(base, CHECKER, "the base")
write(judged, JUDGED, "the judged tree")

# 3. The pin, from the base's blob.
if "rust-toolchain.toml" not in base_paths:
    refuse("the base has no `rust-toolchain.toml`")
try:
    with open(os.path.join(CHECKER, "rust-toolchain.toml"), "rb") as fh:
        toolchain = tomllib.load(fh)
except tomllib.TOMLDecodeError as e:
    refuse("the base's `rust-toolchain.toml` does not parse: %s" % e)
table = toolchain.get("toolchain")
if set(toolchain) != {"toolchain"} or not isinstance(table, dict) or "channel" not in table \
        or set(table) - {"channel", "components", "targets", "profile"}:
    refuse("the base's `rust-toolchain.toml` holds more than a `[toolchain]` table of `channel`, `components`, "
           "`targets` and `profile`")
pin = table["channel"]
if not isinstance(pin, str) or not re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", pin):
    refuse("the pin `%s` is not a release number" % pin)

# 4. The allowlisted environment.
home, path = os.environ.get("HOME", ""), os.environ.get("PATH", "")
if not home or not path:
    refuse("`HOME` or `PATH` is unset")
CARGO_HOME = os.path.join(WORK, "cargo-home")
os.makedirs(CARGO_HOME)
env = {"PATH": path, "HOME": home, "RUSTUP_HOME": os.environ.get("RUSTUP_HOME") or os.path.join(home, ".rustup"),
       "RUSTUP_TOOLCHAIN": pin, "CARGO_HOME": CARGO_HOME, "CARGO_TARGET_DIR": os.path.join(WORK, "checker-target")}
r = subprocess.run(["rustc", "-vV"], capture_output=True, text=True, env=env, cwd=CHECKER)
release = re.search(r"^release: (\S+)$", r.stdout, re.M)
if r.returncode or not release or release.group(1) != pin:
    refuse("`rustc -vV` does not name the pin %s: %s" % (pin, (r.stderr or r.stdout).strip()[:200]))

# 5. Every cargo configuration on the checker build's path holds only aliases.
def alias_only(config):
    try:
        with open(config, "rb") as fh:
            doc = tomllib.load(fh)
    except (OSError, tomllib.TOMLDecodeError):
        return False
    aliases = doc.get("alias", {})
    return set(doc) <= {"alias"} and isinstance(aliases, dict) and all(
        isinstance(v, str) or (isinstance(v, list) and all(isinstance(x, str) for x in v)) for v in aliases.values())

d = CHECKER
while True:
    for name in ("config", "config.toml"):
        config = os.path.join(d, ".cargo", name)
        if os.path.lexists(config) and (os.path.islink(config) or not alias_only(config)):
            refuse("`%s` is on the checker build's path and holds more than `alias` keys" % config)
    if os.path.dirname(d) == d:
        break
    d = os.path.dirname(d)
if os.listdir(CARGO_HOME):
    refuse("`CARGO_HOME` is not empty")

# 6. The checker, built from the base and run from its own target directory.
r = subprocess.run(["cargo", "build", "--offline", "--locked", "--release", "-p", package, "--bin", binary],
                   env=env, cwd=CHECKER)
program = os.path.join(WORK, "checker-target", "release", binary)
if r.returncode or not os.path.isfile(program):
    refuse("the checker `%s:%s` did not build from the base" % (package, binary))
print("catalog-check: the checker built from the base %s, judging %s" % (base[:12], judged[:12]), file=sys.stderr)
sys.stderr.flush()
substituted = [a.replace("{judged}", JUDGED).replace("{base}", CHECKER)
               .replace("{judged-commit}", judged).replace("{base-commit}", base) for a in extra]
run = subprocess.run([program, *substituted],
                     env=dict(env, CARGO_TARGET_DIR=os.path.join(WORK, "records-target")), cwd=CHECKER)
print("catalog-check: the checker's verdict: exit %d" % run.returncode, file=sys.stderr)
sys.exit(run.returncode)
PY
}

self_test() {
  local arms=0 ok=0 repo="$SCRATCH/repo" marker="$SCRATCH/PLANTED" planted="$SCRATCH/planted.sh" base
  rm -rf "$SCRATCH"; mkdir -p "$repo/checker/src" "$repo/.cargo"
  # The planted program: it leaves a mark, then does what it was asked, so a build that ran it still succeeds.
  printf '#!/bin/sh\ntouch "%s"\nexec "$@"\n' "$marker" > "$planted"; chmod +x "$planted"
  printf '/target/\n' > "$repo/.gitignore"
  printf '[workspace]\nresolver = "2"\nmembers = ["checker"]\nexclude = ["target"]\n' > "$repo/Cargo.toml"
  printf '# This file is automatically @generated by Cargo.\n# It is not intended for manual editing.\nversion = 4\n\n[[package]]\nname = "checker"\nversion = "0.1.0"\n' \
    > "$repo/Cargo.lock"
  printf '[package]\nname = "checker"\nversion = "0.1.0"\nedition = "2021"\n' > "$repo/checker/Cargo.toml"
  cat > "$repo/checker/src/main.rs" <<'RS'
// A stub checker: it names where it runs from and where its builds go, and fails when the judged tree asks it to.
fn main() {
    let judged = std::env::args().nth(1).expect("the judged tree");
    let verdict = std::fs::read_to_string(std::path::Path::new(&judged).join("VERDICT")).unwrap_or_default();
    println!("stub checker: running {}", std::env::current_exe().unwrap().display());
    println!("stub checker: builds into {}", std::env::var("CARGO_TARGET_DIR").unwrap_or_default());
    // What a build in the judged tree would compile with: the checker's own builds run there (`M2.7.4`).
    let rustc = std::process::Command::new("rustc").arg("-vV").current_dir(&judged).output();
    let rustc = rustc.map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default();
    let release = rustc.lines().find(|l| l.starts_with("release: ")).unwrap_or("release: none");
    println!("stub checker: in the judged tree, {release}");
    println!("stub checker: further arguments: {}", std::env::args().skip(2).collect::<Vec<_>>().join(" "));
    if verdict.trim() == "fail" {
        std::process::exit(1);
    }
}
RS
  cp "$ROOT/rust-toolchain.toml" "$repo/rust-toolchain.toml"
  printf '[alias]\ncheck-catalog = "run --package checker --"\n' > "$repo/.cargo/config.toml"
  g() { git -C "$repo" -c user.name=t -c user.email=t@t -c core.ignorecase=false "$@"; }
  g init -q && g add -A && g commit -q -m base
  base="$(g rev-parse HEAD)"
  judged() { # each argument `path=content`, or `path@mode=content`; prints the judged commit, the base's child
    local index="$SCRATCH/index" spec path mode content sha
    rm -f "$index"; GIT_INDEX_FILE="$index" g read-tree "$base"
    for spec in "$@"; do
      path="${spec%%=*}"; content="${spec#*=}"; mode=100644
      case "$path" in *@*) mode="${path#*@}"; path="${path%@*}" ;; esac
      sha="$(printf '%s' "$content" | g hash-object -w --stdin)"
      GIT_INDEX_FILE="$index" g update-index --add --cacheinfo "$mode,$sha,$path" || return 1
    done
    printf '%s' "$(g commit-tree "$(GIT_INDEX_FILE="$index" g write-tree)" -p "$base" -m judged)"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, $4 = the judged commit, then env for the run
    local name="$1" want="$2" must="$3" commit="$4" out rc
    shift 4
    arms=$((arms + 1)); rm -f "$marker"
    out="$(cd "$repo" && env "$@" bash "$SELF" --base "$base" --judged "$commit" --checker checker:checker -- '{judged}' 2>&1)"
    rc=$?
    if [ -e "$marker" ]; then
      echo "SELF-TEST: $name — the planted program ran" >&2; return
    fi
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | tail -4 | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | tail -4 | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  arm_with_args() { # $1 = name, $2 = expected rc, $3 = text the output must carry, $4 = the judged commit, then the
    local name="$1" want="$2" must="$3" commit="$4" out rc # arguments after `--`, placeholders included
    shift 4
    arms=$((arms + 1)); rm -f "$marker"
    out="$(cd "$repo" && bash "$SELF" --base "$base" --judged "$commit" --checker checker:checker -- '{judged}' "$@" 2>&1)"
    rc=$?
    if [ -e "$marker" ]; then
      echo "SELF-TEST: $name — the planted program ran" >&2; return
    fi
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | tail -4 | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | tail -4 | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  local same pin; same="$(judged)"
  pin="$(sed -n 's/^channel = "\(.*\)"$/\1/p' "$ROOT/rust-toolchain.toml")"
  arm_with_args "the commit placeholders name the judged commit and the base" 0 "further arguments: $same $base" "$same" '{judged-commit}' '{base-commit}'
  arm "an unchanged tree: the checker runs from its own target directory" 0 "checker-target/release/checker" "$same"
  arm "the checker's builds go to a fresh target directory, not its own" 0 "builds into $repo/target/catalog-check/run/records-target" "$same"
  arm "a judged build script never runs" 0 "the checker's verdict: exit 0" \
    "$(judged "checker/build.rs=fn main() { std::fs::write(\"$marker\", \"\").unwrap(); }")"
  arm "a judged cargo configuration naming a wrapper is not on the checker's path" 0 "the checker's verdict: exit 0" \
    "$(judged ".cargo/config.toml=[build]
rustc-wrapper = \"$planted\"
")"
  arm "a judged toolchain file is never read, not even by the checker's builds" 0 "in the judged tree, release: $pin" \
    "$(judged "rust-toolchain.toml=[toolchain]
path = \"$SCRATCH/no-such-toolchain\"
")"
  arm "the judged tree's copy of this harness never runs" 0 "the checker's verdict: exit 0" \
    "$(judged "scripts/catalog_check.sh@100755=#!/bin/sh
touch \"$marker\"
")"
  arm "the caller's environment does not reach the build" 0 "the checker's verdict: exit 0" "$same" \
    RUSTC_WRAPPER="$planted" CARGO_BUILD_RUSTC_WRAPPER="$planted" RUSTFLAGS="-C linker=$planted" \
    RUSTUP_TOOLCHAIN=nightly CARGO_TARGET_DIR="$SCRATCH/elsewhere" CARGO_HOME="$SCRATCH/elsewhere"
  if [ -e "$SCRATCH/elsewhere" ]; then echo "SELF-TEST: the caller's CARGO_TARGET_DIR or CARGO_HOME was used" >&2; ok=$((ok - 1)); fi
  arm "the checker's verdict is the harness's" 1 "the checker's verdict: exit 1" "$(judged "VERDICT=fail")"
  mkdir -p "$repo/target/.cargo"; printf '[build]\nrustc-wrapper = "%s"\n' "$planted" > "$repo/target/.cargo/config.toml"
  arm "a configuration left in the checkout's target/ is refused before cargo runs" 3 "holds more than \`alias\` keys" "$same"
  rm -rf "$repo/target/.cargo"
  # A base other than the checkout: `arm` always names the checkout's commit as the base, so this run names the
  # judged commit instead.
  arms=$((arms + 1))
  out="$(cd "$repo" && bash "$SELF" --base "$same" --judged "$same" --checker checker:checker 2>&1)"; rc=$?
  if [ "$rc" -eq 3 ] && printf '%s' "$out" | grep -qF "the checkout is not the base"; then
    ok=$((ok + 1)); echo "  ✅ a base other than the checkout is refused"
  else echo "SELF-TEST: a base other than the checkout — expected exit 3, got $rc" >&2; fi
  printf 'x\n' > "$repo/untracked.txt"
  arm "a checkout with an untracked file is refused" 3 "modified or has untracked files" "$same"
  rm -f "$repo/untracked.txt"
  arm "a judged symbolic link is refused" 3 "is a symbolic link" "$(judged "escape@120000=/")"
  arm "judged paths equal but for ASCII case are refused" 3 "differ only in ASCII case" "$(judged "Docs/a=1" "docs/b=2")"
  arm "a cargo file name spelled otherwise is refused" 3 "spells \`build.rs\` otherwise" "$(judged "checker/Build.rs=fn main() {}")"
  # Trees git's own checks refuse, written by hand: a path that is both a file and a directory, and a path twice.
  literal() { # $1 = python expression building the tree's entries from `blob` and `sub`, the hashes as bytes
    python3 - "$repo" "$1" <<'LIT'
import subprocess, sys
repo, expr = sys.argv[1], sys.argv[2]
def obj(kind, data):
    out = subprocess.run(["git", "-C", repo, "hash-object", "-t", kind, "-w", "--literally", "--stdin"],
                         input=data, capture_output=True, check=True).stdout.strip()
    return bytes.fromhex(out.decode())
blob = obj("blob", b"x\n")
sub = obj("tree", b"100644 x\0" + blob)
base = subprocess.run(["git", "-C", repo, "cat-file", "tree", "HEAD"], capture_output=True, check=True).stdout
tree = obj("tree", base + eval(expr)).hex()
print(subprocess.run(["git", "-C", repo, "-c", "user.name=t", "-c", "user.email=t@t", "commit-tree", tree, "-m", "l"],
                     capture_output=True, text=True, check=True).stdout.strip())
LIT
  }
  arm "a hand-written tree with a path both a file and a directory is refused (round 1, V5)" 3 "both a file and a directory" \
    "$(literal 'b"100644 zz\0" + blob + b"40000 zz\0" + sub')"
  arm "a hand-written tree listing a path twice is refused (round 1, V5)" 3 "listed twice" \
    "$(literal 'b"100644 zz\0" + blob + b"100644 zz\0" + blob')"
  sed -i.bak 's/^channel = .*/channel = "stable"/' "$repo/rust-toolchain.toml" && rm -f "$repo/rust-toolchain.toml.bak"
  g commit -q -am "a named channel"; base="$(g rev-parse HEAD)"
  arm "a base pinned to a named channel is refused" 3 "is not a release number" "$base"
  rm -rf "$SCRATCH"
  echo "catalog-check self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  *) harness "$@"; exit $? ;;
esac
