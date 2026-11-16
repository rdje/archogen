#!/usr/bin/env bash
# scripts/move_record.sh — one scripted, verified step that moves a record (leaf `PROGRAM.117.2`,
# docs/decisions/decision_ceilings-bound-reads.md): an accepted design from docs/decisions/ to docs/specs/<subject>/,
# or a subject's records into a sub-folder of their own.
#
# ⭐ WHY THIS EXISTS. A move by hand (`PROGRAM.43`, `.56`, `.102`) is a census of every place the record is cited,
# done by reading: its own relative links, the indexes, the code that names it, the book. One missed citation is a
# dead link; one rewritten in a sealed archive or a verbatim proof falsifies history. This tool makes the census
# mechanical and the line between what is rewritten and what is left explicit.
#
# WHAT IT DOES, for `bash scripts/move_record.sh [--dry-run] <record> <destination>`:
#   1. refuses unless <record> is a tracked `.md` under docs/decisions/ or docs/specs/, neither an INDEX.md nor a
#      TEMPLATE.md, and <destination> — a path ending `.md`, or a directory ending `/` that keeps the record's name —
#      is under docs/decisions/ or docs/specs/ and does not exist; and, for a move, unless the working tree has no
#      unstaged change, so what it stages is its own work — a dry run plans from the disk and changes nothing;
#   2. `git mv`s the record, and rewrites every relative markdown link inside it to resolve as before;
#   3. repoints every citation of the old path in a live tracked text file: the repository-relative path written out
#      (a path that climbs to the root with `../` ends in it, so the same rewrite keeps it right), and every relative
#      markdown link that resolved to the record, its anchor kept;
#   4. leaves, and lists, what is not live: a path written as `<revision>:<path>`, which names the file at that
#      revision; the archives (docs/task-history/, docs/decision-history/, docs/review-history/, docs/history/); the
#      rolling ledgers CHANGELOG.md and DEV_NOTES.md; the verbatim proofs of docs/book/src/annex-trust-proofs.md; and
#      the files held for the scaffold (.doctrine/scaffold_held.txt). A bare file name is left: it still names it;
#   5. stages the result, then verifies: no live file names the old path but as a revision's, and DECISION-INDEX
#      passes. It names what it cannot settle: a new sub-folder to link from knowledge-map/subsystems.md, and a
#      rewritten Rust source or trust root, whose admissions `cargo xtask trust-inventory` re-derives.
# The sealed evaluation folder, docs/evaluation/, is never listed or opened (`docs/evaluation/README.md`).
#
# CONTRACT: exit 0 moved (or, with --dry-run, planned); 1 refused or unverified, named on stderr; 2 a usage error.
# `--self-test` runs its arms in scratch repositories under `target/doctrine_scratch/move_record/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
HERE="$(cd "$(dirname "$SELF")" && pwd)"
SCRATCH_ROOT="$(git -C "$HERE" rev-parse --show-toplevel)/target/doctrine_scratch/move_record"

move() { # $1 = --dry-run or "", $2 = record, $3 = destination
  ROOT="$(git rev-parse --show-toplevel)" || return 1
  cd "$ROOT" || return 1
  DRY="$1" RECORD="$2" DEST="$3" HELD_LIST=".doctrine/scaffold_held.txt" python3 -I - <<'PY'
import os, posixpath, re, subprocess, sys

def git(*args, check=True):
    r = subprocess.run(["git", *args], capture_output=True)
    if check and r.returncode != 0:
        sys.exit("MOVE-RECORD: git %s failed: %s" % (" ".join(args), r.stderr.decode(errors="replace").strip()))
    return r.stdout

def refuse(why):
    print("MOVE-RECORD: " + why, file=sys.stderr)
    sys.exit(1)

dry = os.environ["DRY"] == "--dry-run"
old = posixpath.normpath(os.environ["RECORD"])
dest = os.environ["DEST"]
new = posixpath.normpath(dest + posixpath.basename(old) if dest.endswith("/") else dest)
HOMES = ("docs/decisions/", "docs/specs/")

# 1. Refusals, before anything is touched.
tracked = set(git("ls-files", "-z", "--", ".", ":!docs/evaluation").decode().split("\0")) - {""}
if old not in tracked:
    refuse("`%s` is not a tracked file" % old)
if not old.endswith(".md") or not old.startswith(HOMES) or posixpath.basename(old) in ("INDEX.md", "TEMPLATE.md"):
    refuse("`%s` is no record: a move takes a `.md` under docs/decisions/ or docs/specs/, never an index or the template" % old)
if not new.endswith(".md") or not new.startswith(HOMES):
    refuse("`%s` is no record's place: the destination is a `.md` under docs/decisions/ or docs/specs/, or a folder there ending `/`" % new)
if os.path.lexists(new) or new in tracked:
    refuse("`%s` already exists" % new)
if not dry and git("diff", "--name-only", "--", ".", ":!docs/evaluation").strip():
    refuse("the working tree has unstaged changes — stage or set them aside, so what this stages is the move's alone")

held = set()
if os.path.exists(os.environ["HELD_LIST"]):
    held = {l.strip() for l in open(os.environ["HELD_LIST"], encoding="utf-8") if l.strip() and not l.startswith("#")}
LEFT = (("docs/task-history/", "an archive"), ("docs/decision-history/", "an archive"),
        ("docs/review-history/", "an archive"), ("docs/history/", "an archive"),
        ("CHANGELOG.md", "a rolling ledger"), ("DEV_NOTES.md", "a rolling ledger"),
        ("docs/book/src/annex-trust-proofs.md", "verbatim proofs"))

def left_why(path):
    if path in held:
        return "held for the scaffold"
    for prefix, why in LEFT:
        if path == prefix or (prefix.endswith("/") and path.startswith(prefix)):
            return why
    return None

def text(path):
    data = open(path, "rb").read()
    if b"\0" in data[:8192]:
        return None
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        return None

LINK = re.compile(r"(\]\()([^)\s]+)(\))")
def external(target):
    return target.startswith(("#", "/", "http:", "https:", "mailto:"))

# A path written out: not inside a longer name; a `<revision>:` before it names the file at that revision.
CITE = re.compile(r"(?<![A-Za-z0-9_.-])" + re.escape(old) + r"(?![A-Za-z0-9_-])")
REVISION = re.compile(r"(?:\b[0-9a-f]{7,40}|\bHEAD[~^0-9]*|\b[A-Za-z0-9_./-]+@\{[^}]*\}):$")

plan, kept, rewrites = [], [], {}
plan.append("move  %s -> %s" % (old, new))

# 2. The record's own links, resolved from its new folder.
own = text(old)
if own is None:
    refuse("`%s` is not UTF-8 text" % old)
def relink(m):
    target = m.group(2)
    if external(target):
        return m.group(0)
    path, _, anchor = target.partition("#")
    resolved = posixpath.normpath(posixpath.join(posixpath.dirname(old), path))
    moved = posixpath.relpath(resolved, posixpath.dirname(new)) + ("#" + anchor if anchor else "")
    if moved != target:
        plan.append("link  %s: %s -> %s" % (new, target, moved))
    return m.group(1) + moved + m.group(3)
moved_body = LINK.sub(relink, own)
if moved_body != own:
    rewrites[old] = moved_body

# 3 and 4. Every other tracked text file: citations by path, and relative links that resolved to the record.
for path in sorted(tracked - {old}):
    if not os.path.isfile(path) or os.path.islink(path):
        continue
    body = text(path)
    if body is None or (old not in body and posixpath.basename(old) not in body):
        continue
    why = left_why(path)
    def cite(m):
        line = body.count("\n", 0, m.start()) + 1
        if REVISION.search(body[max(0, m.start() - 64):m.start()]):
            kept.append("left  %s:%d — a revision's path" % (path, line))
            return m.group(0)
        if why:
            kept.append("left  %s:%d — %s" % (path, line, why))
            return m.group(0)
        plan.append("cite  %s:%d" % (path, line))
        return new
    out = CITE.sub(cite, body)
    if path.endswith(".md"):
        def link(m):
            target = m.group(2)
            if external(target):
                return m.group(0)
            ref, _, anchor = target.partition("#")
            if posixpath.normpath(posixpath.join(posixpath.dirname(path), ref)) != old:
                return m.group(0)
            line = out.count("\n", 0, m.start()) + 1
            if why:
                kept.append("left  %s:%d — %s, a link" % (path, line, why))
                return m.group(0)
            moved = posixpath.relpath(new, posixpath.dirname(path)) + ("#" + anchor if anchor else "")
            plan.append("link  %s:%d: %s -> %s" % (path, line, target, moved))
            return m.group(1) + moved + m.group(3)
        out = LINK.sub(link, out)
    if out != body:
        rewrites[path] = out

for line in plan + kept:
    print(line)
if dry:
    print("move-record: planned, nothing changed (--dry-run)")
    sys.exit(0)

# The move, then the rewrites, then the stage.
os.makedirs(posixpath.dirname(new), exist_ok=True)
fresh_folder = not any(p.startswith(posixpath.dirname(new) + "/") for p in tracked)
git("mv", "--", old, new)
for path, body in rewrites.items():
    target = new if path == old else path
    with open(target, "w", encoding="utf-8", newline="") as f:
        f.write(body)
git("add", "--", new, *[p for p in rewrites if p != old])

# 5. Verified: no live file names the old path but as a revision's.
left = []
for path in sorted(set(git("ls-files", "-z", "--", ".", ":!docs/evaluation").decode().split("\0")) - {""}):
    if left_why(path) or not os.path.isfile(path):
        continue
    body = text(path)
    if body is None:
        continue
    for m in CITE.finditer(body):
        if not REVISION.search(body[max(0, m.start() - 64):m.start()]):
            left.append("%s:%d" % (path, body.count("\n", 0, m.start()) + 1))
if left:
    refuse("the old path is still named in a live file: " + ", ".join(left))
if fresh_folder:
    print("note  %s/ is a new folder: link it from knowledge-map/subsystems.md — the generated map reads docs/decisions/*.md flat" % posixpath.dirname(new))
if any(p.endswith(".rs") or p.startswith("trust/") for p in rewrites):
    print("note  a Rust source or a trust root was rewritten: re-derive the admissions with `cargo xtask trust-inventory`")
print("move-record: %s -> %s, %d file(s) rewritten, %d citation(s) left; staged" % (old, new, len(rewrites), len(kept)))
PY
  local rc=$?
  [ "$rc" -eq 0 ] || return "$rc"
  [ "$1" = "--dry-run" ] && return 0
  bash "$HERE/check_decision_index.sh"
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH_ROOT/selftest" staged
  fresh() {
    chmod -R u+rwx "$work" 2>/dev/null; rm -rf "$work"
    mkdir -p "$work/docs/decisions" "$work/docs/specs" "$work/docs/tasks" "$work/docs/task-history" \
      "$work/docs/evaluation/frozen" "$work/crates/x/src" "$work/.doctrine"
    git -C "$work" init -q
    printf '# Index\n\n| Record |\n| --- |\n| [`decision_a.md`](decision_a.md) |\n| [`decision_b.md`](decision_b.md) |\n' > "$work/docs/decisions/INDEX.md"
    printf '# A\n\nSee [B](decision_b.md#why) and [the tree](../tasks/T.md).\n' > "$work/docs/decisions/decision_a.md"
    printf '# B\n\nAfter [A](decision_a.md), `docs/decisions/decision_a.md`.\n' > "$work/docs/decisions/decision_b.md"
    printf -- '- ID: `T.1`\n  Note: `docs/decisions/decision_a.md`; WHERE, `git show abc1234:docs/decisions/decision_a.md`.\n' > "$work/docs/tasks/T.md"
    printf -- '- sealed: `docs/decisions/decision_a.md`\n' > "$work/docs/task-history/T.0.md"
    printf 'const A: &str = include_str!("../../../docs/decisions/decision_a.md");\n' > "$work/crates/x/src/lib.rs"
    printf 'held: docs/decisions/decision_a.md\n' > "$work/HELD.md"
    printf 'HELD.md\n' > "$work/.doctrine/scaffold_held.txt"
    printf 'a sealed case naming docs/decisions/decision_a.md\n' > "$work/docs/evaluation/frozen/case.md"
    git -C "$work" add -A
    git -C "$work" -c core.hooksPath=/dev/null -c commit.gpgsign=false -c user.name=t -c user.email=t@t commit -qm fixture
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the move's arguments
    local name="$1" want="$2" must="$3" out rc
    shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return 1
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return 1
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  holds() { # $1 = name, $2 = file in the world, $3 = text it must hold
    arms=$((arms + 1))
    if grep -qF -- "$3" "$work/$2" 2>/dev/null; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — $2 does not hold \`$3\`" >&2; fi
  }
  # The sealed evaluation folder is unreadable throughout: a tool that opened it would fail.
  seal() { chmod 000 "$work/docs/evaluation/frozen/case.md" "$work/docs/evaluation/frozen"; }
  unseal() { chmod 755 "$work/docs/evaluation/frozen"; chmod 644 "$work/docs/evaluation/frozen/case.md"; } # its modes as committed

  fresh; seal
  arm "a dry run plans the move and changes nothing" 0 "move-record: planned, nothing changed" --dry-run docs/decisions/decision_a.md docs/specs/subj/
  arms=$((arms + 1)); unseal
  if [ -z "$(git -C "$work" status --porcelain)" ] && [ -f "$work/docs/decisions/decision_a.md" ]; then
    ok=$((ok + 1)); echo "  ✅ and the tree after it is as before"
  else echo "SELF-TEST: the dry run changed the tree:" >&2; git -C "$work" status --porcelain | sed -n '1,4p' | sed 's/^/    /' >&2; fi

  fresh; seal
  arm "a record moves to a subject's folder, verified, the index checked" 0 "move-record: docs/decisions/decision_a.md -> docs/specs/subj/decision_a.md" docs/decisions/decision_a.md docs/specs/subj/
  holds "its own links resolve from its new folder, anchors kept" docs/specs/subj/decision_a.md "[B](../../decisions/decision_b.md#why)"
  holds "its link up the tree too" docs/specs/subj/decision_a.md "[the tree](../../tasks/T.md)"
  holds "the index's link follows it" docs/decisions/INDEX.md "(../specs/subj/decision_a.md)"
  holds "another record's relative link follows it" docs/decisions/decision_b.md "[A](../specs/subj/decision_a.md)"
  holds "and its written-out path" docs/decisions/decision_b.md '`docs/specs/subj/decision_a.md`'
  holds "a live tree's citation follows it" docs/tasks/T.md 'Note: `docs/specs/subj/decision_a.md`'
  holds "a revision's path is left: it names the file at that revision" docs/tasks/T.md 'git show abc1234:docs/decisions/decision_a.md'
  holds "a path climbing to the root follows it" crates/x/src/lib.rs '"../../../docs/specs/subj/decision_a.md"'
  holds "an archive is left" docs/task-history/T.0.md '`docs/decisions/decision_a.md`'
  holds "a file held for the scaffold is left" HELD.md 'docs/decisions/decision_a.md'
  arms=$((arms + 1)); unseal
  staged="$(git -C "$work" diff --cached --no-renames --name-status)"
  if [ -z "$(git -C "$work" diff --name-only)" ] && grep -qE '^D[[:space:]]+docs/decisions/decision_a.md$' <<< "$staged" &&
    grep -qE '^A[[:space:]]+docs/specs/subj/decision_a.md$' <<< "$staged"; then
    ok=$((ok + 1)); echo "  ✅ the move is staged whole, nothing left unstaged"
  else echo "SELF-TEST: the move is not staged whole as a rename:" >&2; git -C "$work" status --porcelain | sed -n '1,6p' | sed 's/^/    /' >&2; fi

  fresh; seal
  arm "what is left is listed, by why" 0 "left  docs/task-history/T.0.md:1 — an archive" --dry-run docs/decisions/decision_a.md docs/specs/subj/
  arm "a revision's path is listed as left" 0 "left  docs/tasks/T.md:2 — a revision's path" --dry-run docs/decisions/decision_a.md docs/specs/subj/
  arm "a held file is listed as left" 0 "left  HELD.md:1 — held for the scaffold" --dry-run docs/decisions/decision_a.md docs/specs/subj/
  arm "a new folder is named for the knowledge map" 0 "note  docs/specs/subj/ is a new folder" docs/decisions/decision_a.md docs/specs/subj/
  fresh; seal
  arm "a rewritten Rust source is named for the trust inventory" 0 "cargo xtask trust-inventory" docs/decisions/decision_a.md docs/specs/subj/

  fresh; seal
  arm "a record not tracked is refused" 1 "is not a tracked file" docs/decisions/decision_z.md docs/specs/
  arm "an index is no record" 1 "is no record" docs/decisions/INDEX.md docs/specs/
  arm "a destination outside the record folders is refused" 1 "is no record's place" docs/decisions/decision_a.md docs/other/
  arm "a destination that exists is refused" 1 "already exists" docs/decisions/decision_a.md docs/decisions/decision_b.md
  printf 'more\n' >> "$work/docs/decisions/decision_b.md"
  arm "unstaged changes are refused, so the stage is the move's alone" 1 "unstaged changes" docs/decisions/decision_a.md docs/specs/subj/

  chmod -R u+rwx "$work" 2>/dev/null; rm -rf "$work"
  echo "move-record self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --dry-run) [ "$#" -eq 3 ] || { echo "usage: bash scripts/move_record.sh [--dry-run] <record> <destination>" >&2; exit 2; }; move --dry-run "$2" "$3"; exit $? ;;
  -*|'') echo "usage: bash scripts/move_record.sh [--dry-run] <record> <destination>" >&2; exit 2 ;;
  *) [ "$#" -eq 2 ] || { echo "usage: bash scripts/move_record.sh [--dry-run] <record> <destination>" >&2; exit 2; }; move "" "$1" "$2"; exit $? ;;
esac
