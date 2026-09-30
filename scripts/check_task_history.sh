#!/usr/bin/env bash
# scripts/check_task_history.sh — TASK-HISTORY: closed subtrees sealed out of the task trees (leaf `PROGRAM.32`,
# docs/decisions/decision_task-tree-sealing.md, LIVE_DOCUMENT_SIZE_CONTAINMENT.md's `archive_terminal`).
#
# ⭐ WHY. On 2026-09-30, 77% of `docs/tasks/M1.md` and 79% of `docs/tasks/PROGRAM.md` were the bodies of leaves marked
# `done`. The director's §8 ruling (option C) seals them out: when every leaf of one of a tree's top-level subtrees is
# `done`, those leaves move, byte for byte and in the tree's order, into `docs/task-history/<TREE>/<SUBTREE>.md`, and
# each leaves a two-line stub where it stood — its `- ID:` line and a `Status: `done`` line that links the sealed file
# and names the leaf's commit. `docs/task-history/INDEX.md` records each file's leaves, lines, bytes and sha256.
#
# MODES:
#   bash scripts/check_task_history.sh                  # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_task_history.sh --seal <TREE>    # seal every closed subtree of docs/tasks/<TREE>.md, prove, check
#   bash scripts/check_task_history.sh --self-test
#
# A LEAF is its `- ID: `…`` line and every line up to the next `- ID:` line or `## ` heading, TASK-ACCEPTANCE's own
# slicing. Its BODY is that span without its trailing blank lines, which stay in the tree after the stub.
#
# THE GATE'S LEGS:
#   1. every sealed file's leaf count, lines, bytes and sha256 are its index row's, so a sealed file cannot change;
#   2. sealed files and rows correspond one to one;
#   3. every row the index holds at HEAD is still there, unchanged: the index is append-only;
#   4. every leaf in a sealed file has exactly one stub in its tree, `done`, linking that file; every stub links a
#      sealed file that holds its leaf; a stub is exactly its two lines.
#
# THE SEAL writes nothing unless the tree it would leave, with every new stub replaced by its body from its new sealed
# file, is the tree as it stood, byte for byte.
#
# ⚠️ HONEST LIMIT: a row and its file edited together are caught only against HEAD (leg 3), as for HISTORY-LEDGERS.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/task_history"
mkdir -p "$SCRATCH"

core() { # $1 = gate | seal, $2 = tree (seal only), $3 = the index at HEAD, or empty
  python3 - "$@" <<'PY'
import hashlib, os, re, sys, datetime

mode, tree, head_index = sys.argv[1], sys.argv[2], sys.argv[3]
TASKS, HIST = "docs/tasks", "docs/task-history"
INDEX = os.path.join(HIST, "INDEX.md")
ID_RE = re.compile(r"^- ID: `([^`]+)`")
STATUS_RE = re.compile(r"^  Status: `([^`]+)`")
STUB_RE = re.compile(r"^  Status: `done` — sealed in \[`([^`]+)`\]\(\.\./task-history/([^)]+)\); commit (.+)$")
WORK_UNIT = re.compile(r"ARCHOGEN-[A-Z0-9-]*[A-Z0-9]-[0-9]{4}")
fails = []

def note(msg):
    fails.append(msg)

def read(path):
    with open(path, "rb") as f:
        return f.read().decode("utf-8")

def leaves(text):
    """Each leaf as (id, first, end) over text.split('\n'): lines[first:end] is its span."""
    lines = text.split("\n")
    out, cur = [], None
    for i, line in enumerate(lines):
        m = ID_RE.match(line)
        if m or line.startswith("## "):
            if cur:
                out.append((cur[0], cur[1], i))
            cur = (m.group(1), i) if m else None
    if cur:
        out.append((cur[0], cur[1], len(lines)))
    return lines, out

def body_end(lines, first, end):
    """The end of a leaf's body: its span without trailing blank lines."""
    e = end
    while e > first + 1 and lines[e - 1].strip() == "":
        e -= 1
    return e

def status(lines, first):
    m = STATUS_RE.match(lines[first + 1]) if first + 1 < len(lines) else None
    return m.group(1) if m else None

def subtree(leaf_id, tree_name):
    parts = leaf_id.split(".")
    return ".".join(parts[:2]) if leaf_id != tree_name and len(parts) >= 2 else None

def commit_of(lines, first, bend):
    for i in range(first, bend):
        if lines[i].startswith("  Commit:"):
            j = i
            while True:
                m = WORK_UNIT.search(lines[j])
                if m:
                    return "`" + m.group(0) + "`"
                j += 1
                if j >= bend or not lines[j].startswith("    "):
                    return "in the tree's Commit Log"
    return "in the tree's Commit Log"

def sha(data):
    return hashlib.sha256(data.encode("utf-8")).hexdigest()

def rows(index_text):
    """{tree: [(subtree, leaves, lines, bytes, sha, sealed)]} from an index's tables."""
    out, cur = {}, None
    for line in index_text.split("\n"):
        m = re.match(r"^## `([^`]+)`$", line)
        if m:
            cur = m.group(1); out.setdefault(cur, []); continue
        if cur and re.match(r"^\| `[^`]+` \|", line):
            cells = [c.strip().strip("`") for c in line.strip().strip("|").split("|")]
            out[cur].append(tuple(cells[:6]))
    return out

HEADER = """# docs/task-history/INDEX.md — closed subtrees sealed out of the task trees

Each file below holds the leaves of one of a task tree's top-level subtrees, moved here byte for byte by
`bash scripts/check_task_history.sh --seal <TREE>` once every leaf under it was `done`, and never edited again
(`docs/decisions/decision_task-tree-sealing.md`). Each leaf left a two-line stub in its tree that links here. A row
records the file's leaf count, lines, bytes and sha256, and the day it was sealed. The rows are append-only, and
`TASK-HISTORY` checks every file against its row, and every leaf against its stub, on every commit.

To read a closed leaf, follow its stub. To prove a file, compare `sha256sum docs/task-history/<TREE>/<SUBTREE>.md`
with its row.
"""

def add_rows(tree_name, new_rows):
    text = read(INDEX) if os.path.exists(INDEX) else HEADER
    lines = text.rstrip("\n").split("\n")
    head = "## `%s`" % tree_name
    if head not in lines:
        lines += ["", head, "", "| Subtree | Leaves | Lines | Bytes | sha256 | Sealed |", "| --- | --- | --- | --- | --- | --- |"]
    start = lines.index(head)
    last = start
    for k in range(start + 1, len(lines)):
        if lines[k].startswith("## "):
            break
        if lines[k].startswith("| "):
            last = k
    lines[last + 1:last + 1] = new_rows
    with open(INDEX, "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")

if mode == "seal":
    path = os.path.join(TASKS, tree + ".md")
    if not os.path.exists(path):
        sys.exit("task-history: %s does not exist" % path)
    before = read(path)
    lines, ls = leaves(before)
    groups = {}
    for lid, first, end in ls:
        key = subtree(lid, tree)
        if key:
            groups.setdefault(key, []).append((lid, first, end))
    sealable = []
    for key, members in groups.items():
        stubs = [m for m in members if STUB_RE.match(lines[m[1] + 1] if m[1] + 1 < len(lines) else "")]
        if stubs:
            if len(stubs) != len(members) and all(status(lines, m[1]) == "done" for m in members):
                print("task-history: %s reopened after it was sealed; its new leaves stay live" % key, file=sys.stderr)
            continue
        if all(status(lines, m[1]) == "done" for m in members) and not os.path.exists(os.path.join(HIST, tree, key + ".md")):
            sealable.append(key)
    if not sealable:
        print("task-history: %s — nothing to seal" % path)
        sys.exit(0)
    # Build the new tree and the sealed files.
    seal_of = {m[0]: key for key in sealable for m in groups[key]}
    sealed = {key: [] for key in sealable}
    out, i = [], 0
    spans = sorted((m for key in sealable for m in groups[key]), key=lambda m: m[1])
    for lid, first, end in spans:
        out.extend(lines[i:first])
        key = seal_of[lid]
        bend = body_end(lines, first, end)
        sealed[key].append("\n".join(lines[first:end]))
        rel = "%s/%s.md" % (tree, key)
        out.append(lines[first])
        out.append("  Status: `done` — sealed in [`%s`](../task-history/%s); commit %s" % (rel, rel, commit_of(lines, first, bend)))
        out.extend(lines[bend:end])
        i = end
    out.extend(lines[i:])
    after = "\n".join(out)
    files = {}
    for key in sealable:
        content = "\n".join(sealed[key])
        if not content.endswith("\n"):
            content += "\n"
        files[key] = content
    # The proof: every new stub replaced by its body from its new file reconstructs the tree as it stood.
    alines, als = leaves(after)
    rebuilt, i = [], 0
    for lid, first, end in als:
        m = STUB_RE.match(alines[first + 1]) if first + 1 < len(alines) else None
        if not m or lid not in seal_of:
            continue
        flines, fls = leaves(files[seal_of[lid]])
        found = [(f, e) for (x, f, e) in fls if x == lid]
        if len(found) != 1:
            sys.exit("task-history: the seal of %s would lose %s; nothing was written" % (path, lid))
        f, e = found[0]
        rebuilt.extend(alines[i:first])
        rebuilt.extend(flines[f:body_end(flines, f, e)])
        i = first + 2
    rebuilt.extend(alines[i:])
    if "\n".join(rebuilt) != before:
        sys.exit("task-history: the seal of %s would not reconstruct it byte for byte; nothing was written" % path)
    os.makedirs(os.path.join(HIST, tree), exist_ok=True)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    new_rows = []
    for key in sorted(sealable, key=lambda k: min(m[1] for m in groups[k])):
        fpath = os.path.join(HIST, tree, key + ".md")
        with open(fpath, "w", encoding="utf-8") as f:
            f.write(files[key])
        data = files[key]
        new_rows.append("| `%s` | %d | %d | %d | `%s` | `%s` |" % (key, len(groups[key]), data.count("\n"), len(data.encode("utf-8")), sha(data), today))
    with open(path, "w", encoding="utf-8") as f:
        f.write(after)
    add_rows(tree, new_rows)
    print("task-history: %s — sealed %d subtree(s), %d leaves: %s; the reconstruction is byte for byte"
          % (path, len(sealable), len(seal_of), " ".join(sorted(sealable, key=lambda k: min(m[1] for m in groups[k])))))
    sys.exit(0)

# The gate.
index = rows(read(INDEX)) if os.path.exists(INDEX) else {}
checked = 0
held = {}  # leaf id -> sealed file path, from the sealed files
for tname, trows in index.items():
    listed = set()
    for key, nleaves, nlines, nbytes, digest, _ in trows:
        fpath = os.path.join(HIST, tname, key + ".md")
        listed.add(key + ".md")
        if not os.path.exists(fpath):
            note("%s lists %s, and %s does not exist" % (INDEX, key, fpath)); continue
        data = read(fpath)
        checked += 1
        if str(data.count("\n")) != nlines: note("%s has %d lines, and its row says %s — a sealed file changed" % (fpath, data.count("\n"), nlines))
        if str(len(data.encode("utf-8"))) != nbytes: note("%s has %d bytes, and its row says %s — a sealed file changed" % (fpath, len(data.encode("utf-8")), nbytes))
        if sha(data) != digest: note("%s's sha256 is not its row's — a sealed file changed" % fpath)
        flines, fls = leaves(data)
        if str(len(fls)) != nleaves: note("%s holds %d leaves, and its row says %s" % (fpath, len(fls), nleaves))
        for lid, f, e in fls:
            if status(flines, f) != "done": note("%s holds %s, whose status is not `done`" % (fpath, lid))
            if lid in held: note("%s is sealed twice, in %s and %s" % (lid, held[lid], fpath))
            held[lid] = fpath
    tdir = os.path.join(HIST, tname)
    for name in sorted(os.listdir(tdir)) if os.path.isdir(tdir) else []:
        if name not in listed: note("%s/%s is in no row of %s — a sealed file must be listed" % (tdir, name, INDEX))
if os.path.isdir(HIST):
    for name in sorted(os.listdir(HIST)):
        if os.path.isdir(os.path.join(HIST, name)) and name not in index:
            note("%s/%s has no table in %s" % (HIST, name, INDEX))
# Append-only against HEAD.
if head_index:
    current = rows(read(INDEX)) if os.path.exists(INDEX) else {}
    for tname, trows in rows(read(head_index)).items():
        for r in trows:
            if r not in current.get(tname, []):
                note("%s's %s row `%s` is not as HEAD has it — the index is append-only" % (INDEX, tname, r[0]))
# Stubs against bodies, over every tree.
stubs = {}
for name in sorted(os.listdir(TASKS)) if os.path.isdir(TASKS) else []:
    if not name.endswith(".md") or name == "TEMPLATE.md":
        continue
    tpath = os.path.join(TASKS, name)
    tlines, tls = leaves(read(tpath))
    for lid, first, end in tls:
        line = tlines[first + 1] if first + 1 < len(tlines) else ""
        m = STUB_RE.match(line)
        if " — sealed in [" in line and not m:
            note("%s: the stub of %s is not in the stub's form" % (tpath, lid)); continue
        if not m:
            if lid in held: note("%s: %s is sealed in %s and also live here" % (tpath, lid, held[lid]))
            continue
        if body_end(tlines, first, end) != first + 2:
            note("%s: the stub of %s holds more than its two lines" % (tpath, lid))
        target = os.path.join(HIST, m.group(2))
        if m.group(1) != m.group(2): note("%s: the stub of %s names %s and links %s" % (tpath, lid, m.group(1), m.group(2)))
        if lid in stubs: note("%s has two stubs, in %s and %s" % (lid, stubs[lid], tpath))
        stubs[lid] = tpath
        if held.get(lid) != target:
            note("%s: the stub of %s links %s, which does not hold it" % (tpath, lid, target))
for lid, fpath in held.items():
    if lid not in stubs:
        note("%s is sealed in %s and has no stub in its tree" % (lid, fpath))

for msg in fails:
    print("TASK-HISTORY: " + msg, file=sys.stderr)
if fails:
    print("TASK-HISTORY: %d breach(es) — docs/decisions/decision_task-tree-sealing.md" % len(fails), file=sys.stderr)
    sys.exit(1)
print("task-history: OK (%d sealed file(s), each against its row; the index append-only; %d stub(s), each against its body)" % (checked, len(stubs)))
PY
}

head_index() { # the index at HEAD, into a scratch file, or nothing
  if git cat-file -e "HEAD:docs/task-history/INDEX.md" 2>/dev/null; then
    git show "HEAD:docs/task-history/INDEX.md" > "$SCRATCH/head-index.md" && printf '%s' "$SCRATCH/head-index.md"
  fi
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/tasks"; git -C "$work" init -q
    cat > "$work/docs/tasks/T.md" <<'MD'
# T: a tree

## Task Tree

- ID: `T`
  Status: `active`
  Goal: the tree

- ID: `T.1`
  Status: `done`
  Goal: a closed subtree
  Commit: `ARCHOGEN-T-0001 (leaf T.1)`

- ID: `T.1.1`
  Status: `done`
  Goal: its child, with a line that is long enough to matter and a `code span`
  Commit: checkpoints `ARCHOGEN-T-0002` and `-0003`

- ID: `T.2`
  Status: `active`
  Goal: an open subtree

- ID: `T.2.1`
  Status: `done`
  Goal: a closed leaf of an open subtree, which stays live
  Commit: `pending`

- ID: `T.1.2`
  Status: `done`
  Goal: a leaf of T.1 that sits apart from its siblings
  Commit: see children

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `T.2` | `active` | open |
MD
    git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm base
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the arguments
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  commit() { git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm step; }
  fresh
  arm "a tree with nothing sealed passes" 0 "0 sealed file(s)"
  arm "the seal proves its reconstruction" 0 "the reconstruction is byte for byte" --seal T
  arm "the sealed tree passes the gate" 0 "1 sealed file(s)"
  arm "an open subtree stays live, its closed leaf included" 0 ""
  grep -q '^  Goal: a closed leaf of an open subtree' "$work/docs/tasks/T.md" || { arms=$((arms + 1)); echo "SELF-TEST: the open subtree's closed leaf was sealed" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.1.md`\](../task-history/T/T.1.md); commit `ARCHOGEN-T-0002`$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: T.1.1's stub does not name its commit" >&2; }
  arm "sealing again seals nothing" 0 "nothing to seal" --seal T
  commit
  printf 'x' >> "$work/docs/task-history/T/T.1.md"
  arm "a byte added to a sealed file is refused" 1 "a sealed file changed"
  git -C "$work" checkout -q -- docs/task-history/T/T.1.md
  sed -i.bak '/^| `T.1` |/d' "$work/docs/task-history/INDEX.md"; rm -f "$work/docs/task-history/INDEX.md.bak"
  arm "a sealed file with no row is refused" 1 "is in no row of"
  git -C "$work" checkout -q -- docs/task-history/INDEX.md
  sed -i.bak '/^| `T.1` |/ s/| `[0-9-]*` |$/| `1999-01-01` |/' "$work/docs/task-history/INDEX.md"; rm -f "$work/docs/task-history/INDEX.md.bak"
  arm "a committed row that changed is refused as not append-only" 1 "the index is append-only"
  git -C "$work" checkout -q -- docs/task-history/INDEX.md
  sed -i.bak '/^- ID: `T.1.2`$/,+1d' "$work/docs/tasks/T.md"; rm -f "$work/docs/tasks/T.md.bak"
  arm "a sealed leaf with no stub is refused" 1 "has no stub in its tree"
  git -C "$work" checkout -q -- docs/tasks/T.md
  awk '{ print } after && /^  Status: / { print "  Goal: a line added to a stub"; after = 0 } /^- ID: `T.1.1`$/ { after = 1 }' \
    "$work/docs/tasks/T.md" > "$work/t.md" && mv "$work/t.md" "$work/docs/tasks/T.md"
  arm "a stub that holds more than its two lines is refused" 1 "holds more than its two lines"
  git -C "$work" checkout -q -- docs/tasks/T.md
  sed -i.bak 's|^  Status: `done` — sealed in \[`T/T.1.md`\](../task-history/T/T.1.md); commit `ARCHOGEN-T-0001`$|  Status: `done` — sealed in [`T/T.9.md`](../task-history/T/T.9.md); commit `ARCHOGEN-T-0001`|' "$work/docs/tasks/T.md"; rm -f "$work/docs/tasks/T.md.bak"
  arm "a stub linking a file that does not hold its leaf is refused" 1 "which does not hold it"
  git -C "$work" checkout -q -- docs/tasks/T.md
  sed -i.bak 's|^  Status: `done` — sealed in \[`T/T.1.md`\](../task-history/T/T.1.md); commit `ARCHOGEN-T-0001`$|  Status: `active` — sealed in [`T/T.1.md`](../task-history/T/T.1.md)|' "$work/docs/tasks/T.md"; rm -f "$work/docs/tasks/T.md.bak"
  arm "a stub whose status is not done is refused" 1 "is not in the stub's form"
  git -C "$work" checkout -q -- docs/tasks/T.md
  arm "and the sealed tree passes again" 0 ""
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real task history passes"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "task-history self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --seal)
    [ -n "${2:-}" ] || { echo "usage: bash scripts/check_task_history.sh --seal <TREE>" >&2; exit 2; }
    core seal "$2" "" || exit 1
    core gate "" "$(head_index)"; exit $? ;;
  "") core gate "" "$(head_index)"; exit $? ;;
  *) echo "usage: bash scripts/check_task_history.sh [--seal <TREE> | --self-test]" >&2; exit 2 ;;
esac
