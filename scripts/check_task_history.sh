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
# slicing. Its BODY is that span without its trailing blank lines, which stay in the tree after the stub. Every line of
# a body after its first is indented or blank; the seal refuses a leaf that is not, since a fence, a heading or prose at
# column 0 would make the line slicing tear it (the review of `PROGRAM.32.4`, P2).
#
# THE GATE'S LEGS:
#   1. every sealed file's leaf count, lines, bytes and sha256 are its index row's;
#   2. sealed files and rows correspond one to one, and nothing else is under docs/task-history/;
#   3. HISTORY-WIDE, every row any committed version of the index held is still there, unchanged, and every sealed file
#      is byte for byte what the commit that added it wrote — so CI, where HEAD is the commit under test, catches a
#      forgery as the pre-commit hook does (P4). Both reads take `--full-history`, so a merge that keeps only a side
#      that never sealed cannot hide the seal, and a shallow repository, or a failed read, is a breach (`PROGRAM.42`);
#   4. every leaf in a sealed file has exactly one stub, in its own tree, `done`, linking that file, and every stub links
#      a sealed file that holds its leaf; a stub is exactly its two lines; a leaf sits in its own subtree's file;
#   5. PROVENANCE: every sealed leaf is, byte for byte, the leaf its tree held just before the commit that sealed it
#      (HEAD, for a seal not yet committed), `done` there, and its whole subtree was sealed with it — so a seal made by
#      hand, or a body edited on the way in, is refused (P3);
#   6. no live leaf sits in a subtree that is sealed: new work opens a new top-level subtree (P6).
#
# THE SEAL writes nothing unless the tree it would leave, with every new stub replaced by its body from its new sealed
# file, is the tree as it stood, byte for byte; and it rolls everything back if the gate then refuses the result.
#
# ⚠️ HONEST LIMIT: history rewritten under the gate (a force-push, a replaced object) is premise 2 and 3's, as for the
# catalog (decision_catalog-records.md §0). Within history, legs 3 and 5 hold whatever HEAD is.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/task_history"
mkdir -p "$SCRATCH"

core() { # $1 = gate | seal, $2 = tree (seal only)
  GIT_NO_REPLACE_OBJECTS=1 python3 - "$@" <<'PY'
import hashlib, os, re, subprocess, sys, datetime

mode, tree = sys.argv[1], sys.argv[2]
TASKS, HIST = "docs/tasks", "docs/task-history"
INDEX = os.path.join(HIST, "INDEX.md")
ID_RE = re.compile(r"^- ID: `([^`]+)`")
STATUS_RE = re.compile(r"^  Status: `([^`]+)`")
STUB_RE = re.compile(r"^  Status: `done` — sealed in \[`([^`]+)`\]\(\.\./task-history/([^)]+)\); commit (.+)$")
WORK_UNIT = re.compile(r"ARCHOGEN-[A-Z0-9-]*[A-Z0-9]-[0-9]{4}(?![0-9])")
ROW_RE = re.compile(r"^\| `([^`]+)` \| (\d+) \| (\d+) \| (\d+) \| `([0-9a-f]{64})` \| `(\d{4}-\d{2}-\d{2})` \|$")
fails = []

def note(msg):
    fails.append(msg)

class Unreadable(Exception):
    pass

def decode(data, what):
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError as e:
        raise Unreadable("%s is not UTF-8 (byte %d)" % (what, e.start))

def read(path):
    with open(path, "rb") as f:
        return decode(f.read(), path)

_git = {}
def git(*args):
    """stdout of a read-only git command, or None when it fails."""
    if args not in _git:
        r = subprocess.run(("git",) + args, capture_output=True)
        _git[args] = r.stdout if r.returncode == 0 else None
    return _git[args]

def at(commit, path):
    data = git("show", "%s:%s" % (commit, path))
    return None if data is None else decode(data, "%s:%s" % (commit, path))

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

def body(lines, first, end):
    return "\n".join(lines[first:body_end(lines, first, end)])

def status(lines, first):
    m = STATUS_RE.match(lines[first + 1]) if first + 1 < len(lines) else None
    return m.group(1) if m else None

def is_stub(lines, first):
    return first + 1 < len(lines) and STUB_RE.match(lines[first + 1]) is not None

def subtree(leaf_id, tree_name):
    parts = leaf_id.split(".")
    return ".".join(parts[:2]) if leaf_id != tree_name and len(parts) >= 2 else None

def commit_log_names(lines, leaf_id):
    on = False
    for line in lines:
        if line.startswith("## "):
            on = line.strip() == "## Commit Log"
        elif on and line.startswith("| `%s` |" % leaf_id):
            return True
    return False

def commit_of(lines, first, bend, leaf_id):
    """The first work-unit id of the leaf's Commit field; else where the commit is recorded, or that it is not."""
    for i in range(first, bend):
        if lines[i].startswith("  Commit:"):
            j = i
            while True:
                m = WORK_UNIT.search(lines[j])
                if m:
                    return "`" + m.group(0) + "`"
                j += 1
                if j >= bend or not lines[j].startswith("    "):
                    break
            break
    if commit_log_names(lines, leaf_id):
        return "in the tree's Commit Log"
    return "not recorded"

def sha(data):
    return hashlib.sha256(data.encode("utf-8")).hexdigest()

def rows(index_text, where):
    """{tree: [(subtree, leaves, lines, bytes, sha, sealed)]}; a malformed row is a named breach."""
    out, cur = {}, None
    for n, line in enumerate(index_text.split("\n"), 1):
        m = re.match(r"^## `([^`]+)`$", line)
        if m:
            cur = m.group(1); out.setdefault(cur, []); continue
        if cur and line.startswith("| `"):
            r = ROW_RE.match(line)
            if not r:
                note("%s:%d is not a row of the form `| `SUBTREE` | leaves | lines | bytes | `sha256` | `date` |`" % (where, n)); continue
            out[cur].append(r.groups())
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

def gate():
    """The six legs; returns the number of sealed files checked and of stubs."""
    shallow = git("rev-parse", "--is-shallow-repository")
    if shallow is None or shallow.decode().strip() != "false":
        note("the repository is shallow, or git cannot say, so the commits legs 3 and 5 read may be missing — fetch the full history")
    index = rows(read(INDEX), INDEX) if os.path.exists(INDEX) else {}
    checked, held = 0, {}  # held: leaf id -> sealed file path
    files = {}             # sealed file path -> (tree, subtree)
    for tname, trows in index.items():
        listed = set()
        for key, nleaves, nlines, nbytes, digest, _ in trows:
            fpath = os.path.join(HIST, tname, key + ".md")
            listed.add(key + ".md")
            if not os.path.exists(fpath):
                note("%s lists %s, and %s does not exist" % (INDEX, key, fpath)); continue
            data = read(fpath)
            checked += 1
            files[fpath] = (tname, key)
            if str(data.count("\n")) != nlines: note("%s has %d lines, and its row says %s — a sealed file changed" % (fpath, data.count("\n"), nlines))
            if str(len(data.encode("utf-8"))) != nbytes: note("%s has %d bytes, and its row says %s — a sealed file changed" % (fpath, len(data.encode("utf-8")), nbytes))
            if sha(data) != digest: note("%s's sha256 is not its row's — a sealed file changed" % fpath)
            flines, fls = leaves(data)
            if str(len(fls)) != nleaves: note("%s holds %d leaves, and its row says %s" % (fpath, len(fls), nleaves))
            for lid, f, e in fls:
                if status(flines, f) != "done": note("%s holds %s, whose status is not `done`" % (fpath, lid))
                if subtree(lid, tname) != key: note("%s holds %s, which is not in subtree %s" % (fpath, lid, key))
                if lid in held: note("%s is sealed twice, in %s and %s" % (lid, held[lid], fpath))
                held[lid] = fpath
        tdir = os.path.join(HIST, tname)
        for name in sorted(os.listdir(tdir)) if os.path.isdir(tdir) else []:
            if name not in listed: note("%s/%s is in no row of %s — a sealed file must be listed" % (tdir, name, INDEX))
    if os.path.isdir(HIST):
        for name in sorted(os.listdir(HIST)):
            p = os.path.join(HIST, name)
            if os.path.isdir(p):
                if name not in index: note("%s has no table in %s" % (p, INDEX))
            elif name != "INDEX.md":
                note("%s is neither the index nor a tree's folder" % p)
    # Leg 3, history-wide: every row any committed index held, and every sealed file as the commit that added it wrote it.
    current = {t: set(r) for t, r in index.items()}
    index_log = git("log", "--full-history", "--format=%H", "--", INDEX)
    if index_log is None:
        note("`git log` of %s failed, so leg 3 cannot hold — a failed read is never an empty history" % INDEX)
    for commit in (index_log or b"").decode().split():
        old = at(commit, INDEX)
        if old is None:
            continue
        for tname, trows in rows(old, "%s:%s" % (commit[:12], INDEX)).items():
            for r in trows:
                if r not in current.get(tname, set()):
                    note("%s's %s row `%s`, committed in %s, is gone or changed — the index is append-only" % (INDEX, tname, r[0], commit[:12]))
    added = {}
    log = git("log", "--full-history", "--diff-filter=A", "--format=@%H", "--name-only", "--", HIST)
    if log is None:
        note("`git log` of %s failed, so legs 3 and 5 cannot hold — a failed read is never an empty history" % HIST)
    commit = None
    for line in (log or b"").decode().split("\n"):
        if line.startswith("@"):
            commit = line[1:]
        elif line.strip():
            added[line.strip()] = commit  # the log runs newest first, so the last one kept is the oldest
    for fpath in files:
        c = added.get(fpath)
        if c and at(c, fpath) != read(fpath):
            note("%s is not what %s wrote when it sealed it — a sealed file changed" % (fpath, c[:12]))
    # Legs 4 and 6: stubs against bodies and trees; no live leaf in a sealed subtree.
    stubs = {}
    sealed_keys = {(t, k) for (t, k) in files.values()}
    for name in sorted(os.listdir(TASKS)) if os.path.isdir(TASKS) else []:
        if not name.endswith(".md") or name == "TEMPLATE.md":
            continue
        tpath = os.path.join(TASKS, name)
        tname = name[:-3]
        tlines, tls = leaves(read(tpath))
        for lid, first, end in tls:
            line = tlines[first + 1] if first + 1 < len(tlines) else ""
            m = STUB_RE.match(line)
            if " — sealed in [" in line and not m:
                note("%s: the stub of %s is not in the stub's form" % (tpath, lid)); continue
            if not m:
                if lid in held: note("%s: %s is sealed in %s and also live here" % (tpath, lid, held[lid]))
                if (tname, subtree(lid, tname)) in sealed_keys:
                    note("%s: %s is live in subtree %s, which is sealed — new work opens a new top-level subtree" % (tpath, lid, subtree(lid, tname)))
                continue
            if body_end(tlines, first, end) != first + 2:
                note("%s: the stub of %s holds more than its two lines" % (tpath, lid))
            target = os.path.join(HIST, m.group(2))
            if m.group(1) != m.group(2): note("%s: the stub of %s names %s and links %s" % (tpath, lid, m.group(1), m.group(2)))
            if not m.group(2).startswith(tname + "/"): note("%s: the stub of %s links another tree's file, %s" % (tpath, lid, m.group(2)))
            if lid in stubs: note("%s has two stubs, in %s and %s" % (lid, stubs[lid], tpath))
            stubs[lid] = tpath
            if held.get(lid) != target:
                note("%s: the stub of %s links %s, which does not hold it" % (tpath, lid, target))
    for lid, fpath in held.items():
        if lid not in stubs:
            note("%s is sealed in %s and has no stub in its tree" % (lid, fpath))
    # Leg 5, provenance: each sealed leaf against its tree just before the commit that sealed it.
    for fpath, (tname, key) in files.items():
        c = added.get(fpath)
        base = (c + "^") if c else "HEAD"
        before = at(base, "%s/%s.md" % (TASKS, tname))
        if before is None:
            note("%s: %s had no %s/%s.md to be sealed from" % (fpath, base, TASKS, tname)); continue
        blines, bls = leaves(before)
        was = {lid: (f, e) for lid, f, e in bls}
        flines, fls = leaves(read(fpath))
        sealed_ids = {lid for lid, _, _ in fls}
        for lid, f, e in fls:
            if lid not in was:
                note("%s holds %s, which %s's tree did not hold — a seal must move a leaf that existed" % (fpath, lid, base)); continue
            bf, be = was[lid]
            if body(flines, f, e) != body(blines, bf, be):
                note("%s holds %s with a body that is not the one %s's tree held — a leaf was edited on its way in" % (fpath, lid, base))
            if status(blines, bf) != "done":
                note("%s holds %s, which was not `done` in %s's tree — a seal takes only closed leaves" % (fpath, lid, base))
        for lid, bf, be in bls:
            if subtree(lid, tname) == key and not is_stub(blines, bf) and lid not in sealed_ids:
                note("%s seals subtree %s without %s, which %s's tree held — a subtree is sealed whole" % (fpath, key, lid, base))
    return checked, len(stubs)

def seal(tree_name):
    path = os.path.join(TASKS, tree_name + ".md")
    if not os.path.exists(path):
        sys.exit("task-history: %s does not exist" % path)
    before = read(path)
    lines, ls = leaves(before)
    groups = {}
    for lid, first, end in ls:
        key = subtree(lid, tree_name)
        if key:
            groups.setdefault(key, []).append((lid, first, end))
    sealable = []
    for key, members in groups.items():
        if any(is_stub(lines, m[1]) for m in members):
            continue
        if all(status(lines, m[1]) == "done" for m in members) and not os.path.exists(os.path.join(HIST, tree_name, key + ".md")):
            sealable.append(key)
    if not sealable:
        print("task-history: %s — nothing to seal" % path)
        return
    # Fail closed: a sealed body is its ID line and indented or blank lines only.
    torn = []
    for key in sealable:
        for lid, first, end in groups[key]:
            for k in range(first + 1, body_end(lines, first, end)):
                if lines[k] and not lines[k].startswith(" "):
                    torn.append("%s, line %d: %r" % (lid, k + 1, lines[k][:60]))
    if torn:
        sys.exit("task-history: %s holds leaves with a line at column 0 after their ID, which the line slicing would tear;"
                 " nothing was written:\n  " % path + "\n  ".join(torn))
    seal_of = {m[0]: key for key in sealable for m in groups[key]}
    sealed = {key: [] for key in sealable}
    out, i = [], 0
    for lid, first, end in sorted((m for key in sealable for m in groups[key]), key=lambda m: m[1]):
        out.extend(lines[i:first])
        bend = body_end(lines, first, end)
        sealed[seal_of[lid]].append("\n".join(lines[first:end]))
        rel = "%s/%s.md" % (tree_name, seal_of[lid])
        commit = commit_of(lines, first, bend, lid)
        if commit == "not recorded" or "`pending`" in "\n".join(l for l in lines[first:bend] if l.startswith("  Commit:")):
            print("task-history: warning — %s is `done` and its Commit field names no commit; its stub says %s" % (lid, commit), file=sys.stderr)
        out.append(lines[first])
        out.append("  Status: `done` — sealed in [`%s`](../task-history/%s); commit %s" % (rel, rel, commit))
        out.extend(lines[bend:end])
        i = end
    out.extend(lines[i:])
    after = "\n".join(out)
    files = {key: ("\n".join(sealed[key]) + ("" if "\n".join(sealed[key]).endswith("\n") else "\n")) for key in sealable}
    # The proof: every new stub replaced by its body from its new file reconstructs the tree as it stood.
    alines, als = leaves(after)
    rebuilt, i = [], 0
    for lid, first, end in als:
        if not is_stub(alines, first) or lid not in seal_of:
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
    # Write, then prove the result with the gate; roll back if it refuses.
    index_before = read(INDEX) if os.path.exists(INDEX) else None
    os.makedirs(os.path.join(HIST, tree_name), exist_ok=True)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    order = sorted(sealable, key=lambda k: min(m[1] for m in groups[k]))
    written = []
    for key in order:
        fpath = os.path.join(HIST, tree_name, key + ".md")
        with open(fpath, "w", encoding="utf-8") as f:
            f.write(files[key])
        written.append(fpath)
    with open(path, "w", encoding="utf-8") as f:
        f.write(after)
    add_rows(tree_name, ["| `%s` | %d | %d | %d | `%s` | `%s` |" % (key, len(groups[key]), files[key].count("\n"),
                         len(files[key].encode("utf-8")), sha(files[key]), today) for key in order])
    gate()
    if fails:
        for fpath in written:
            os.remove(fpath)
        with open(path, "w", encoding="utf-8") as f:
            f.write(before)
        if index_before is None:
            os.remove(INDEX)
        else:
            with open(INDEX, "w", encoding="utf-8") as f:
                f.write(index_before)
        sys.exit("task-history: the gate refused the seal of %s, so it was rolled back:\n  " % path + "\n  ".join(fails))
    print("task-history: %s — sealed %d subtree(s), %d leaves: %s; the reconstruction is byte for byte"
          % (path, len(sealable), len(seal_of), " ".join(order)))

try:
    if mode == "seal":
        seal(tree)
        fails.clear()
    checked, nstubs = gate()
except Unreadable as e:
    note(str(e))
    checked, nstubs = 0, 0
for msg in fails:
    print("TASK-HISTORY: " + msg, file=sys.stderr)
if fails:
    print("TASK-HISTORY: %d breach(es) — docs/decisions/decision_task-tree-sealing.md" % len(fails), file=sys.stderr)
    sys.exit(1)
print("task-history: OK (%d sealed file(s), each against its row and its sealing commit; the index append-only across history; %d stub(s), each against its body; every sealed leaf proven against its tree before its seal)" % (checked, nstubs))
PY
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

- ID: `T.3`
  Status: `done`
  Goal: a closed subtree holding a line at column 0
```text
a fence the slicing would tear
```
  Commit: `ARCHOGEN-T-0009`

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
  sub() { # $1 = file, $2 = python regex, $3 = replacement (one substitution, must match)
    python3 - "$work/$1" "$2" "$3" <<'PY'
import re, sys
p, pat, rep = sys.argv[1:4]
s = open(p, encoding="utf-8").read()
n = re.subn(pat, rep, s, count=1, flags=re.M)
assert n[1] == 1, "no match for %r in %s" % (pat, p)
open(p, "w", encoding="utf-8").write(n[0])
PY
  }
  restore() { git -C "$work" checkout -q -- . && git -C "$work" clean -qfd; }
  fresh
  arm "a tree with nothing sealed passes" 0 "0 sealed file(s)"
  arm "a closed subtree with a column-0 line is refused and nothing is written" 1 "nothing was written" --seal T
  [ ! -d "$work/docs/task-history" ] && git -C "$work" diff --quiet || { arms=$((arms + 1)); echo "SELF-TEST: the refused seal wrote something" >&2; }
  sub docs/tasks/T.md '^```text\na fence the slicing would tear\n```\n' '  a line indented as a field\n'
  commit
  arm "the seal proves its reconstruction" 0 "the reconstruction is byte for byte" --seal T
  arm "the sealed tree passes the gate" 0 "2 sealed file(s)"
  grep -q '^  Goal: a closed leaf of an open subtree' "$work/docs/tasks/T.md" || { arms=$((arms + 1)); echo "SELF-TEST: the open subtree's closed leaf was sealed" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.1.md`\](../task-history/T/T.1.md); commit `ARCHOGEN-T-0002`$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: T.1.1's stub does not name its commit" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.1.md`\](../task-history/T/T.1.md); commit not recorded$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: T.1.2's stub does not say its commit is not recorded" >&2; }
  arm "sealing again seals nothing" 0 "nothing to seal" --seal T
  commit
  printf 'x' >> "$work/docs/task-history/T/T.1.md"
  arm "a byte added to a sealed file is refused" 1 "a sealed file changed"
  restore
  sub docs/task-history/INDEX.md '^\| `T\.1` \|.*\n' ''
  arm "a sealed file with no row is refused" 1 "is in no row of"
  restore
  sub docs/task-history/INDEX.md '^(\| `T\.1` \| 3 \|)' '\1'
  sub docs/task-history/INDEX.md '^(\| `T\.1` \| )3( \|)' '\g<1>9\2'
  arm "a leaf count unlike the row's is refused" 1 "and its row says 9"
  restore
  sub docs/task-history/INDEX.md '^(\| `T\.1` \|.*\n)' '\1| `T.8` | 1 | 1 | 1 | `0000000000000000000000000000000000000000000000000000000000000000` | `2026-09-30` |\n'
  arm "a row listing a file that does not exist is refused" 1 "does not exist"
  restore
  sub docs/task-history/INDEX.md '^(\| `T\.1` \|.*\| `)[0-9-]+(` \|)$' '\g<1>1999-01-01\2'
  commit
  arm "a committed row that changed is refused across history" 1 "the index is append-only"
  git -C "$work" reset -q --hard HEAD~1
  python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
f = w + "/docs/task-history/T/T.1.md"; s = open(f).read().replace("its child, with", "its child, now with"); open(f, "w").write(s)
i = w + "/docs/task-history/INDEX.md"; t = open(i).read()
t = re.sub(r"^\| `T\.1` \| (\d+) \| (\d+) \| (\d+) \| `[0-9a-f]+` \|", lambda m: "| `T.1` | %s | %d | %d | `%s` |" % (m.group(1), s.count("\n"), len(s.encode()), hashlib.sha256(s.encode()).hexdigest()), t, flags=re.M)
open(i, "w").write(t)
PY
  commit
  arm "a sealed file and its row forged together and committed are refused" 1 "is not what"
  git -C "$work" reset -q --hard HEAD~1
  # A merge that keeps only a side that never sealed (PROGRAM.42): git's default simplification hid the seal.
  local main; main="$(git -C "$work" symbolic-ref --short HEAD)"
  git -C "$work" checkout -q -b other HEAD~1
  printf 'unrelated\n' > "$work/notes.md"; commit
  git -C "$work" -c user.name=t -c user.email=t@t merge -q -s ours -m "drop the seal" "$main"
  arm "a merge that keeps only a side that never sealed cannot hide the seal" 1 "the index is append-only"
  git -C "$work" checkout -q "$main"; git -C "$work" branch -q -D other
  git clone -q --depth 1 "file://$work" "$work-shallow" 2>/dev/null
  arms=$((arms + 1))
  if out="$(cd "$work-shallow" && bash "$SELF" 2>&1)"; then echo "SELF-TEST: a shallow clone passed" >&2
  elif printf '%s' "$out" | grep -qF "the repository is shallow"; then ok=$((ok + 1)); echo "  ✅ a shallow clone is refused"
  else echo "SELF-TEST: a shallow clone was refused for another reason" >&2; fi
  rm -rf "$work-shallow"
  sub docs/tasks/T.md '^- ID: `T\.1\.2`\n  Status: .*\n' ''
  arm "a sealed leaf with no stub is refused" 1 "has no stub in its tree"
  restore
  awk '{ print } after && /^  Status: / { print "  Goal: a line added to a stub"; after = 0 } /^- ID: `T.1.1`$/ { after = 1 }' \
    "$work/docs/tasks/T.md" > "$work/t.md" && mv "$work/t.md" "$work/docs/tasks/T.md"
  arm "a stub that holds more than its two lines is refused" 1 "holds more than its two lines"
  restore
  sub docs/tasks/T.md '\(\.\./task-history/T/T\.1\.md\); commit `ARCHOGEN-T-0001`' '(../task-history/T/T.9.md); commit `ARCHOGEN-T-0001`'
  arm "a stub whose link text and target differ is refused" 1 "names T/T.1.md and links T/T.9.md"
  restore
  sub docs/tasks/T.md '^  Status: `done` — sealed in \[`T/T\.1\.md`\]\(\.\./task-history/T/T\.1\.md\); commit `ARCHOGEN-T-0001`$' '  Status: `active` — sealed in [`T/T.1.md`](../task-history/T/T.1.md)'
  arm "a stub whose status is not done is refused" 1 "is not in the stub's form"
  restore
  printf '\n- ID: `T.1.1`\n  Status: `done` — sealed in [`T/T.1.md`](../task-history/T/T.1.md); commit `ARCHOGEN-T-0002`\n' >> "$work/docs/tasks/T.md"
  arm "two stubs for one leaf are refused" 1 "has two stubs"
  restore
  printf '\n- ID: `T.1.9`\n  Status: `pending`\n  Goal: new work under a sealed subtree\n' >> "$work/docs/tasks/T.md"
  arm "a live leaf in a sealed subtree is refused" 1 "new work opens a new top-level subtree"
  restore
  sub docs/task-history/T/T.1.md '^  Status: `done`$' '  Status: `active`'
  arm "a sealed leaf whose status is not done is refused" 1 "whose status is not \`done\`"
  restore
  printf 'stray\n' > "$work/docs/task-history/notes.txt"
  arm "a stray file in the history folder is refused" 1 "neither the index nor a tree's folder"
  restore
  mkdir -p "$work/docs/task-history/U" && printf 'x\n' > "$work/docs/task-history/U/U.1.md"
  arm "a tree folder with no table is refused" 1 "has no table in"
  restore
  # A seal made by hand of a done leaf from an open subtree, with its file, row and stub: provenance refuses it.
  python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
t = open(w + "/docs/tasks/T.md").read()
m = re.search(r"^- ID: `T\.2\.1`\n(?:  .*\n)*", t, re.M)
leaf = m.group(0)
t = t.replace(leaf, "- ID: `T.2.1`\n  Status: `done` — sealed in [`T/T.2.md`](../task-history/T/T.2.md); commit not recorded\n")
open(w + "/docs/tasks/T.md", "w").write(t)
open(w + "/docs/task-history/T/T.2.md", "w").write(leaf)
i = open(w + "/docs/task-history/INDEX.md").read().rstrip("\n") + "\n| `T.2` | 1 | %d | %d | `%s` | `2026-09-30` |\n" % (leaf.count("\n"), len(leaf.encode()), hashlib.sha256(leaf.encode()).hexdigest())
open(w + "/docs/task-history/INDEX.md", "w").write(i)
PY
  arm "a seal made by hand from an open subtree is refused" 1 "a subtree is sealed whole"
  restore
  python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
f = w + "/docs/task-history/T/T.1.md"
s = open(f).read().replace("a closed subtree", "a closed subtree, reworded"); open(f, "w").write(s)
i = w + "/docs/task-history/INDEX.md"
t = re.sub(r"^\| `T\.1` \| (\d+) \| (\d+) \| (\d+) \| `[0-9a-f]+` \|", lambda m: "| `T.1` | %s | %d | %d | `%s` |" % (m.group(1), s.count("\n"), len(s.encode()), hashlib.sha256(s.encode()).hexdigest()), open(i).read(), flags=re.M)
open(i, "w").write(t)
PY
  arm "a sealed body edited with its row, before any commit, is refused as not the tree's" 1 "a leaf was edited on its way in"
  restore
  mkdir -p "$work/docs/tasks" && printf -- '# V\n\n## Task Tree\n\n- ID: `V.1`\n  Status: `done` — sealed in [`T/T.1.md`](../task-history/T/T.1.md); commit `ARCHOGEN-T-0001`\n' > "$work/docs/tasks/V.md"
  arm "a stub in another tree is refused" 1 "links another tree's file"
  restore
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
    core seal "$2"; exit $? ;;
  "") core gate ""; exit $? ;;
  *) echo "usage: bash scripts/check_task_history.sh [--seal <TREE> | --self-test]" >&2; exit 2 ;;
esac
