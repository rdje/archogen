#!/usr/bin/env bash
# scripts/check_task_history.sh — TASK-HISTORY: closed subtrees sealed out of the task trees (leaf `PROGRAM.32`,
# docs/decisions/decision_task-tree-sealing.md, LIVE_DOCUMENT_SIZE_CONTAINMENT.md's `archive_terminal`).
#
# ⭐ WHY. On 2026-09-30, 77% of `docs/tasks/M1.md` and 79% of `docs/tasks/PROGRAM.md` were the bodies of leaves marked
# `done`. The director's §8 ruling (option C) seals them out: when every leaf of a subtree is `done`, those leaves move,
# byte for byte and in the tree's order, into `docs/task-history/<TREE>/<SUBTREE>.md`, and each leaves a two-line stub
# where it stood — its `- ID:` line and a `Status: `done`` line that links the sealed file and names the leaf's commit.
# `docs/task-history/INDEX.md` records each file's leaves, lines, bytes and sha256.
#
# THE UNIT is the OUTERMOST CLOSED SUBTREE (`PROGRAM.69`): one of the tree's top-level subtrees when every leaf under it
# is `done`, and, below a top-level subtree that is still open, the shortest leaf, ancestor or self, whose every leaf is
# `done`. Below the top level a subtree is a leaf of the tree: two leaves with no common leaf between them and an open
# subtree seal apart. Measured by `--census fea69ad`, the top-level unit alone left 274 483 bytes of finished leaves live under
# open top-level subtrees — `M3.6`, open while `M3.6.5` waits on the director, held `M3.6.1`, `.2`, `.4` and `.6.1` whole — with
# `docs/tasks/` at its ceiling.
#
# MODES:
#   bash scripts/check_task_history.sh                  # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_task_history.sh --seal <TREE>    # seal every closed subtree of docs/tasks/<TREE>.md, prove, check
#   bash scripts/check_task_history.sh --self-test
#   bash scripts/check_task_history.sh --census <COMMIT>  # bytes of `done` leaves under open top-level subtrees, and
#                                                         # in the closed subtrees below them a seal would take (PROGRAM.69)
#
# A LEAF is its `- ID: `…`` line and every line up to the next `- ID:` line or `## ` heading, TASK-ACCEPTANCE's own
# slicing. Its BODY is that span without its trailing blank lines, which stay in the tree after the stub. Every line of
# a body after its first is indented or blank; the seal refuses a leaf that is not, since a fence, a heading or prose at
# column 0 would make the line slicing tear it (the review of `PROGRAM.32.4`, P2).
#
# THE GATE'S LEGS:
#   1. every sealed file's leaf count, lines, bytes and sha256 are its index row's, and it holds a leaf;
#   2. sealed files and rows correspond one to one, and nothing else is under docs/task-history/;
#   3. HISTORY-WIDE, every row any committed version of the index held is still there, unchanged, and every sealed file
#      is byte for byte what the commit that added it wrote — so CI, where HEAD is the commit under test, catches a
#      forgery as the pre-commit hook does (P4). Both reads take `--full-history`, so a merge that keeps only a side
#      that never sealed cannot hide the seal, and a shallow repository, or a failed read, is a breach (`PROGRAM.42`);
#   4. every leaf in a sealed file has exactly one stub, in its own tree, `done`, linking that file, and every stub links
#      a sealed file that holds its leaf; a stub is exactly its two lines; a leaf sits in its own subtree's file;
#   5. PROVENANCE: every sealed leaf is, byte for byte, the leaf its tree held just before the commit that sealed it
#      (HEAD, for a seal not yet committed), `done` there, its whole subtree was sealed with it, and that subtree was the
#      outermost closed one holding each of its leaves — so a seal made by hand or edited is refused when it differs from
#      what the tool would make in its files, its stubs' links or its units: a body edited on the way in, a part of a
#      closed subtree sealed apart from it, or, below the top level, parts sealed together that no one leaf holds (P3).
#      ⚠️ Not checked: a stub's commit text, which only the seal derives (whether a Commit Log row is a leaf's is prose:
#      `PROGRAM.18` → `PROGRAM.18.1`, a child's row closing its parent), a row's date, and the column-0 rule, which
#      stays the seal's; nor where a stub stands among its tree's lines, nor text after the name on its ID line (R9-5);
#      nor a seal of only some of a tree's closed units, each the tool's, the rest sealing later as units of their own,
#      nor prose in the index outside its tables (R10 P1);
#   6. no live leaf sits in a subtree that is sealed, at any depth and in any tree file of docs/tasks/ (one in a sub-
#      folder is PROGRAM.72's): new work opens a new subtree
#      outside it (P6); and a tree file holds only leaves under its own name, each named once, so no leaf is judged in a
#      tree not its own; the index keeps one table per tree, its header first and its rows under it, one row a file.
#
# THE SEAL writes nothing unless the tree it would leave, with every new stub replaced by its body from its new sealed
# file, is the tree as it stood, byte for byte, no entry already takes a sealed file's path, and no link lies on a
# path it writes, nor any entry at a rewritten file's temporary path (R14 D3); and it rolls everything back if the
# gate then refuses the result, or an error stops it. A stop — any signal but SIGKILL, SIGSTOP, the six fault signals
# and, on Linux, the C library's own — is held from before the first write until the seal is done (R13 to R18); an
# abort ends it whatever the mask. Each file it rewrites is written whole or not at all (R9-1), each write noted
# before it is made (R10 D1); it says "rolled back" only of a rollback that undid everything (R12 D3).
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
import hashlib, os, re, shutil, signal, subprocess, sys, datetime

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
    """The top-level subtree holding a leaf, or None for the tree's root."""
    parts = leaf_id.split(".")
    return ".".join(parts[:2]) if leaf_id != tree_name and len(parts) >= 2 else None

def under(leaf_id, key):
    """Whether a leaf lies in the subtree `key`: is it, or is one of its descendants."""
    return leaf_id == key or leaf_id.startswith(key + ".")

def units(lines, ls, tree_name):
    """{leaf: its unit} for every live leaf a seal would take now: the outermost closed subtree holding it — its
    top-level subtree when every leaf under that is `done`, else the shortest leaf of the tree, ancestor or self, whose
    every leaf is `done` (PROGRAM.69). A stub is sealed already and counts as `done`; a leaf no closed subtree holds
    stays live."""
    present = {lid for lid, _, _ in ls}
    seen, open_ = set(), set()
    for lid, first, _ in ls:
        if subtree(lid, tree_name) is None:
            continue
        parts = lid.split(".")
        for k in range(2, len(parts) + 1):
            key = ".".join(parts[:k])
            seen.add(key)
            if status(lines, first) != "done":
                open_.add(key)
    out = {}
    for lid, first, _ in ls:
        if subtree(lid, tree_name) is None or is_stub(lines, first):
            continue
        parts = lid.split(".")
        for k in range(2, len(parts) + 1):
            key = ".".join(parts[:k])
            if k > 2 and key not in present:
                continue  # below the top level, a subtree is a leaf of the tree
            if key in seen and key not in open_:
                out[lid] = key
                break
    return out

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

Each file below holds the leaves of one closed subtree of a task tree — a top-level one, or the outermost closed one
below a top-level subtree still open — moved here byte for byte by
`bash scripts/check_task_history.sh --seal <TREE>` once every leaf under it was `done`, and never edited again
(`docs/decisions/decision_task-tree-sealing.md`). Each leaf left a two-line stub in its tree that links here. A row
records the file's leaf count, lines, bytes and sha256, and the day it was sealed. The rows are append-only, and
`TASK-HISTORY` checks every file against its row, and every leaf against its stub, on every commit.

To read a closed leaf, follow its stub. To prove a file, compare `sha256sum docs/task-history/<TREE>/<SUBTREE>.md`
with its row.
"""

TABLE_HEAD = "| Subtree | Leaves | Lines | Bytes | sha256 | Sealed |"
TABLE_SEP = "| --- | --- | --- | --- | --- | --- |"

def layout(index_text):
    """The index as `add_rows` writes it: one table per tree, each opening with its header, its rows contiguous under
    it — so a row placed in another tree's table, or outside any, is a breach (PROGRAM.69, the mutation sweep)."""
    lines = index_text.split("\n")
    seen, in_tables = set(), set()
    for n, line in enumerate(lines):
        m = re.match(r"^## `([^`]+)`$", line)
        if not m:
            continue
        if m.group(1) in seen:
            note("%s:%d: a second table for %s — a tree has one" % (INDEX, n + 1, m.group(1)))
        seen.add(m.group(1))
        if lines[n + 1:n + 4] != ["", TABLE_HEAD, TABLE_SEP]:
            note("%s:%d: the table of %s does not open with its header" % (INDEX, n + 1, m.group(1)))
        k = n + 4
        while k < len(lines) and lines[k].startswith("| `"):
            in_tables.add(k)
            k += 1
        if k == n + 4:
            note("%s:%d: the table of %s holds no row — a seal writes a table with its first row" % (INDEX, n + 1, m.group(1)))
    for n, line in enumerate(lines):
        if line.startswith("| `") and n not in in_tables:
            note("%s:%d: a row outside its tree's table, under its header" % (INDEX, n + 1))

# Every signal held through a seal (reviews R14 D2, R15 D2, R16 D5) but SIGKILL and SIGSTOP, which no mask holds, and the
# six signals a fault raises — SIGSEGV, SIGBUS, SIGFPE, SIGILL, SIGTRAP, SIGSYS — left out whoever sends them: blocked, a
# fault the interpreter raises on itself may hang it (SIGSEGV does on macOS, measured). SIGABRT is held: `abort()` ends
# the process whatever the mask. On Linux the C library keeps its own signals out of any mask (review R18 D1).
UNHELD = ("SIGKILL", "SIGSTOP", "SIGSEGV", "SIGBUS", "SIGFPE", "SIGILL", "SIGTRAP", "SIGSYS")
HELD = signal.valid_signals() - {getattr(signal, s) for s in UNHELD if hasattr(signal, s)}
STARTED = []  # one entry once a seal holds its stops and is about to write (review R15 D1)
SAID = []  # one entry once a seal under way has said its outcome — kept, refused, or stopped by an error

def say(text, stream):
    """Write a seal's outcome to `stream` and record it said — only where the stream exists: a descriptor closed at the
    start leaves it None and takes nothing, and a broken one raises before the record (reviews R15 D1, R17 D2)."""
    if stream is not None:
        print(text, file=stream, flush=True)
        SAID.append(text)

def quiet_exit(code):
    """Exit with `code` whatever became of the outputs: one closed or broken is pointed at nothing first, so the exit's
    own flush cannot make the code 120 (reviews R16 D6, R17 D1, D3)."""
    for stream, fd in ((sys.stdout, 1), (sys.stderr, 2)):
        try:
            if stream is not None:
                stream.flush()
        except (OSError, ValueError):
            try:
                os.dup2(os.open(os.devnull, os.O_WRONLY), fd)
            except OSError:
                pass
    sys.exit(code)

def temporary(target):
    return os.path.join(os.path.dirname(target), ".%s.seal-%d" % (os.path.basename(target), os.getpid()))

def put(target, text):
    """Write `text` to `target` whole or not at all: a temporary file beside it, then a rename, so a write a full disk
    stops leaves the target as it was (review R9-1). The target is never a link, nor is anything at its temporary path:
    the seal refuses either before it writes (review R14 D3)."""
    tmp = temporary(target)
    created = False
    try:
        with open(tmp, "x", encoding="utf-8") as f:  # created new, never written through an entry there (review R13 D3)
            created = True
            f.write(text)
        if os.path.exists(target):
            shutil.copymode(target, tmp)
        os.replace(tmp, target)
        created = False
    finally:
        if created and os.path.lexists(tmp):  # only what this call created
            os.remove(tmp)

def current(target):
    """A file's text now, read as `read` reads it — its bytes, no line end translated — or None when there is none."""
    try:
        return read(target)
    except FileNotFoundError:
        return None

def add_rows(tree_name, new_rows):
    text = read(INDEX) if os.path.exists(INDEX) else HEADER
    lines = text.rstrip("\n").split("\n")
    head = "## `%s`" % tree_name
    if head not in lines:
        lines += ["", head, "", TABLE_HEAD, TABLE_SEP]
    start = lines.index(head)
    last = start
    for k in range(start + 1, len(lines)):
        if lines[k].startswith("## "):
            break
        if lines[k].startswith("| "):
            last = k
    lines[last + 1:last + 1] = new_rows
    put(INDEX, "\n".join(lines) + "\n")

def gate():
    """The six legs; returns the number of sealed files checked and of stubs."""
    shallow = git("rev-parse", "--is-shallow-repository")
    if shallow is None or shallow.decode().strip() != "false":
        note("the repository is shallow, or git cannot say, so the commits legs 3 and 5 read may be missing — fetch the full history")
    index = rows(read(INDEX), INDEX) if os.path.exists(INDEX) else {}
    if os.path.exists(INDEX):
        layout(read(INDEX))
    checked, held = 0, {}  # held: leaf id -> sealed file path
    files = {}             # sealed file path -> (tree, subtree)
    for tname, trows in index.items():
        listed = set()
        for key, nleaves, nlines, nbytes, digest, _ in trows:
            fpath = os.path.join(HIST, tname, key + ".md")
            if key + ".md" in listed:
                note("%s lists %s twice in %s's table — one row a sealed file" % (INDEX, key, tname)); continue
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
            if not fls: note("%s holds no leaf — a seal moves at least one" % fpath)
            if str(len(fls)) != nleaves: note("%s holds %d leaves, and its row says %s" % (fpath, len(fls), nleaves))
            for lid, f, e in fls:
                if status(flines, f) != "done": note("%s holds %s, whose status is not `done`" % (fpath, lid))
                if not under(lid, key): note("%s holds %s, which is not in subtree %s" % (fpath, lid, key))
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
        named = [lid for lid, _, _ in tls]
        for lid in sorted({x for x in named if named.count(x) > 1}):
            note("%s holds %s %d times — a leaf is named once" % (tpath, lid, named.count(lid)))
        for lid, first, end in tls:
            if not under(lid, tname):
                note("%s holds %s, which is not under its tree's name %s — a leaf is filed in its own tree" % (tpath, lid, tname))
            line = tlines[first + 1] if first + 1 < len(tlines) else ""
            m = STUB_RE.match(line)
            if " — sealed in [" in line and not m:
                note("%s: the stub of %s is not in the stub's form" % (tpath, lid)); continue
            if not m:
                if lid in held: note("%s: %s is sealed in %s and also live here" % (tpath, lid, held[lid]))
                for t2, key in sorted(sealed_keys):
                    if under(lid, key):
                        note("%s: %s is live in subtree %s, which is sealed in %s — new work opens a new subtree outside it" % (tpath, lid, key, t2))
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
        # The file is exactly the bytes the seal writes from those leaves' spans, in the base tree's order: no leaf
        # reordered, no line added between them (review R3-5).
        if all(lid in was for lid in sealed_ids):
            spans = ["\n".join(blines[was[lid][0]:was[lid][1]]) for lid, _, _ in bls if lid in sealed_ids]
            made = "\n".join(spans)
            made += "" if made.endswith("\n") else "\n"
            if made != read(fpath):
                note("%s is not the bytes its leaves' spans in %s's tree make, in that tree's order — a seal writes them as they stood" % (fpath, base))
        for lid, f, e in fls:
            if lid not in was:
                note("%s holds %s, which %s's tree did not hold — a seal must move a leaf that existed" % (fpath, lid, base)); continue
            bf, be = was[lid]
            if body(flines, f, e) != body(blines, bf, be):
                note("%s holds %s with a body that is not the one %s's tree held — a leaf was edited on its way in" % (fpath, lid, base))
            if status(blines, bf) != "done":
                note("%s holds %s, which was not `done` in %s's tree — a seal takes only closed leaves" % (fpath, lid, base))
        for lid, bf, be in bls:
            if under(lid, key) and not is_stub(blines, bf) and lid not in sealed_ids:
                note("%s seals subtree %s without %s, which %s's tree held — a subtree is sealed whole" % (fpath, key, lid, base))
        # The outermost closed subtree, whole: in the tree before the seal, each sealed leaf's unit is this file's key —
        # neither a part of a closed subtree sealed apart, nor, below the top level, parts sealed together that no one leaf
        # holds (review R1-2).
        outermost = units(blines, bls, tname)
        wrong = sorted("%s in %s" % (lid, outermost.get(lid) or "no closed subtree") for lid in sealed_ids
                       if lid in was and outermost.get(lid) != key)
        if wrong:
            note("%s seals subtree %s, and in %s's tree its leaves' outermost closed subtree was not %s (%s) — a seal takes the outermost closed subtree, whole"
                 % (fpath, key, base, key, ", ".join(wrong[:4]) + (" …" if len(wrong) > 4 else "")))
    return checked, len(stubs)

def census(commit):
    """Per tree file at `commit`: the bytes of live `done` leaves under top-level subtrees still open, and of those in
    the closed subtrees below them that a seal would take — each leaf's span sliced as `leaves` slices it (PROGRAM.69)."""
    names = git("ls-tree", "--name-only", commit, TASKS + "/")
    if names is None:
        sys.exit("task-history: `git ls-tree %s %s/` failed" % (commit, TASKS))
    total, in_closed, count = 0, 0, 0
    for path in sorted(names.decode().split("\n")):
        name = os.path.basename(path)
        if not name.endswith(".md") or name == "TEMPLATE.md":
            continue
        tname = name[:-3]
        lines, ls = leaves(at(commit, path) or "")
        unit = units(lines, ls, tname)
        open_tops = {subtree(lid, tname) for lid, f, _ in ls if subtree(lid, tname) and status(lines, f) != "done"}
        live = [(lid, f, e) for lid, f, e in ls
                if subtree(lid, tname) in open_tops and status(lines, f) == "done" and not is_stub(lines, f)]
        if not live:
            continue
        size = lambda f, e: len(("\n".join(lines[f:e]) + "\n").encode("utf-8"))
        tree_total = sum(size(f, e) for _, f, e in live)
        tree_closed = sum(size(f, e) for lid, f, e in live if lid in unit)
        keys = {unit[lid] for lid, _, _ in live if lid in unit}
        print("census: %s — %d byte(s) of `done` leaves under open top-level subtrees, %d of them in %d closed subtree(s) below the top level"
              % (tname, tree_total, tree_closed, len(keys)))
        total, in_closed, count = total + tree_total, in_closed + tree_closed, count + len(keys)
    print("census: %s — %d byte(s) of `done` leaves under open top-level subtrees, %d of them in %d closed subtree(s) below the top level"
          % (commit, total, in_closed, count))

def seal(tree_name):
    path = os.path.join(TASKS, tree_name + ".md")
    if not os.path.exists(path):
        sys.exit("task-history: %s does not exist" % path)
    before = read(path)
    lines, ls = leaves(before)
    named = [lid for lid, _, _ in ls]
    twice = sorted({x for x in named if named.count(x) > 1})
    if twice:
        sys.exit("task-history: %s holds %s more than once — a leaf is named once; nothing was written" % (path, ", ".join(twice)))
    foreign = [lid for lid, _, _ in ls if not under(lid, tree_name)]
    if foreign:
        sys.exit("task-history: %s holds %s, not under its tree's name %s — a leaf is filed in its own tree; nothing was written"
                 % (path, ", ".join(foreign), tree_name))
    unit = units(lines, ls, tree_name)
    groups = {}
    for lid, first, end in ls:
        if lid in unit:
            groups.setdefault(unit[lid], []).append((lid, first, end))
    # A unit the index rows is sealed already — a live leaf under it is the gate's to refuse (leg 6), so it is left
    # alone. Any other unit's file path must be free: an entry there — a file, a folder, a link to anything or to
    # nothing — is no seal of it, and is refused before a write (reviews R9-2, R10 D2).
    rowed = {m.group(1) for m in map(ROW_RE.match, (read(INDEX).split("\n") if os.path.exists(INDEX) else [])) if m}
    sealable = [key for key in groups if key not in rowed]
    if not sealable:
        print("task-history: %s — nothing to seal" % path)
        return
    taken = [os.path.join(HIST, tree_name, key + ".md") for key in sealable
             if os.path.lexists(os.path.join(HIST, tree_name, key + ".md"))]
    if taken:
        sys.exit("task-history: %s already taken — an entry at a sealed file's path; nothing was written" % ", ".join(taken))
    # A seal writes the tree, its index and its history in place: no link anywhere on their paths below the root — the
    # files, their history folder, or any folder above them (reviews R10 AG3, R11 P2, R12 D2).
    real_root = os.path.realpath(".")
    links = [x for x in (path, INDEX, os.path.join(HIST, tree_name)) if os.path.realpath(x) != os.path.join(real_root, x)]
    if links:
        sys.exit("task-history: %s is reached through a link; nothing was written — a seal writes the tree, its index "
                 "and its history in place" % ", ".join(links))
    # Nor is anything at the temporary path of a file it rewrites — a file, a link, anything (review R14 D3).
    temps = [x for x in (temporary(path), temporary(INDEX)) if os.path.lexists(x)]
    if temps:
        sys.exit("task-history: %s already taken — an entry at a temporary file's path; nothing was written" % ", ".join(temps))
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
    tree_dir = os.path.join(HIST, tree_name)
    made_dirs = [d for d in (HIST, tree_dir) if not os.path.isdir(d)]  # what a rollback removes again (review R5-2)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    order = sorted(sealable, key=lambda k: min(m[1] for m in groups[k]))
    written = []   # the sealed files this run creates, each noted before it is created
    replaced = []  # (file, its text before, None if it had none), each noted before it is rewritten

    def rollback():
        # Each write was noted before it was made, so an interrupt just after one leaves it noted (review R10 D1); a
        # noted write is undone only where it happened — a rewritten file that no longer holds its text before, a sealed
        # file that exists. Each step is on its own: a restore that fails stops no other restore, and a removal that
        # fails no other removal (R9-1). The index and the tree go back first; while either still names the sealed
        # files, because its restore failed, they stay. What could not be undone is named.
        undone = []
        for target, text in reversed(replaced):
            try:
                if current(target) != text:
                    if text is None:
                        os.remove(target)
                    else:
                        put(target, text)
            except Exception as e:
                undone.append("%s, not restored: %s" % (target, e))
        if not undone:
            for fpath in written:
                try:
                    if os.path.lexists(fpath):
                        os.remove(fpath)
                except Exception as e:
                    undone.append("%s, not removed: %s" % (fpath, e))
            for d in reversed(made_dirs):
                try:
                    if os.path.isdir(d) and not os.listdir(d):
                        os.rmdir(d)
                except Exception as e:
                    undone.append("%s, not removed: %s" % (d, e))
        for u in undone:
            print("task-history: the rollback could not undo %s — the next gate run names what is left" % u, file=sys.stderr)
        return not undone

    # Every write and the gate's proof in one guard: any exception that stops the run — a file the gate cannot read,
    # which the run's own handler then names as a breach, or any other — rolls the seal back first and is said before it
    # goes on (reviews R5-2, R6-2, R7-4, R9-2, R14 D1). The tree and the index are written whole or not at all, and the
    # rollback undoes what was written alone, each step on its own, naming what it could not undo (R9-1). Every signal
    # but SIGKILL, SIGSTOP, the six fault signals and, on Linux, the C library's own (`HELD`) is held for the whole of
    # it, from before the first write: a stop that comes takes effect once the seal is done — proven and kept, refused
    # and rolled back, or stopped by an error and rolled back — its outcome
    # said where the output can be written, so the seal is whole either way, with no moment between a write and its
    # record for a stop to fall in (reviews R13, R14 D2, R16 D2). A signal ignored on entry stays ignored. The git the
    # proof runs inherits the mask: a stop does not end a git that hangs. ⚠️ What the mask leaves out — SIGKILL,
    # SIGSTOP's pause aside, the six fault signals whoever sends them, an abort the process raises on itself, and on Linux
    # the C library's own signals, which may — and the machine stopping leave the writes, which the next gate run proves
    # as any seal not yet committed (review R8-2); and a failure that defeats the rollback's own writes too, a full disk,
    # leaves what it names.
    previous = signal.pthread_sigmask(signal.SIG_BLOCK, HELD)
    try:
        STARTED.append(path)
        try:
            os.makedirs(tree_dir, exist_ok=True)
            for key in order:
                fpath = os.path.join(HIST, tree_name, key + ".md")
                written.append(fpath)
                try:
                    f = open(fpath, "x", encoding="utf-8")  # created, never written through an entry already there
                except FileExistsError:
                    written.pop()  # not this run's to remove
                    raise
                with f:
                    f.write(files[key])
            replaced.append((path, before))
            put(path, after)
            replaced.append((INDEX, index_before))
            add_rows(tree_name, ["| `%s` | %d | %d | %d | `%s` | `%s` |" % (key, len(groups[key]), files[key].count("\n"),
                                 len(files[key].encode("utf-8")), sha(files[key]), today) for key in order])
            gate()
        except BaseException as e:
            whole = rollback()
            # Said, and flushed, before the mask lets a held stop end the process, which would leave it unsaid (R14 D1).
            say("task-history: the seal of %s stopped on %s: %s, %s" % (path, type(e).__name__, e, "so it was rolled back"
                if whole else "and its rollback left what it named above"), sys.stderr)
            raise
        if fails:
            whole = rollback()
            # Said, and flushed, before the mask lets a held stop end the process (review R13).
            say("task-history: the gate refused the seal of %s, %s:\n  "
                % (path, "so it was rolled back" if whole else "and its rollback left what it named above")
                + "\n  ".join(fails), sys.stderr)
            sys.exit(1)
        say("task-history: %s — sealed %d subtree(s), %d leaves: %s; the reconstruction is byte for byte"
            % (path, len(sealable), len(seal_of), " ".join(order)), sys.stdout)
    finally:
        signal.pthread_sigmask(signal.SIG_SETMASK, previous)

if mode == "census":
    try:
        census(tree)
    except (Unreadable, OSError) as e:
        sys.exit("task-history: %s" % e)
    sys.exit(0)
try:
    if mode == "seal":
        try:
            seal(tree)
        except KeyboardInterrupt:
            # A stop is held from before the first write, so it comes before any write or once the seal is done — its
            # outcome said, or, when the saying failed (a closed or broken output), unsaid; said itself where the error
            # output can be written, and the exit 130 either way (reviews R14 D1, R15 D1, R17 D1, D3).
            message = "task-history: interrupted %s" % (
                "before the seal wrote anything" if not STARTED else
                "once the seal was done; its outcome is said above" if SAID else
                "once the seal was done; its outcome could not be said — the next gate run proves what is there")
            if sys.stderr is not None:
                try:
                    print(message, file=sys.stderr, flush=True)
                except OSError:
                    pass
            quiet_exit(128 + signal.SIGINT)
        fails.clear()
    checked, nstubs = gate()
except (Unreadable, OSError) as e:
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
  Goal: a closed leaf of an open subtree, sealed in a file of its own
  Commit: `pending`

- ID: `T.2.2`
  Status: `done`
  Goal: a closed leaf with an open child, which stays live
  Commit: `ARCHOGEN-T-0004`

- ID: `T.2.2.1`
  Status: `blocked`
  Goal: the open child, blocked, which is open as any status but `done` is

- ID: `T.2.2.5.1`
  Status: `done`
  Goal: a closed leaf four levels down with no leaf `T.2.2.5` above it, sealed apart
  Commit: `ARCHOGEN-T-0016`

- ID: `T.2.2.5.2`
  Status: `done`
  Goal: its cousin, sealed apart too
  Commit: `ARCHOGEN-T-0017`

- ID: `T.2.3`
  Status: `done`
  Goal: a closed subtree below an open one, sealed whole in one file
  Commit: `ARCHOGEN-T-0007`

- ID: `T.2.3.1`
  Status: `done`
  Goal: its closed child, sealed with it — the café's census counts bytes, not characters
  Commit: `ARCHOGEN-T-0008`

- ID: `T.2.4.1`
  Status: `done`
  Goal: a closed leaf with no leaf `T.2.4` above it, sealed apart
  Commit: `ARCHOGEN-T-0010`

- ID: `T.2.4.2`
  Status: `done`
  Goal: its cousin, sealed apart too
  Commit: `ARCHOGEN-T-0011`

- ID: `T.2.5`
  Status: `done`
  Goal: a closed leaf whose Commit field names its work unit on its second line
  Commit: closed with the slice after it, in
    `ARCHOGEN-T-0022`

- ID: `T.2.10`
  Status: `active`
  Goal: an open leaf whose name begins as `T.2.1`'s does, which is not under it

- ID: `T.4.1`
  Status: `done`
  Goal: a closed leaf of a top-level subtree with no leaf `T.4`, sealed with its cousin
  Commit: `ARCHOGEN-T-0018`

- ID: `T.4.2`
  Status: `done`
  Goal: its cousin
  Commit: `ARCHOGEN-T-0019`

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
    cat > "$work/docs/tasks/TEMPLATE.md" <<'MD'
# TEMPLATE: no tree, which the gate and the census pass over

## Task Tree

- ID: `X.1`
  Status: `active`
  Goal: an open subtree

- ID: `X.1.1`
  Status: `done`
  Goal: a closed leaf below it
MD
    git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm base
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the arguments; `limit=N arm …` runs it
    #          under a file-size limit of N KiB, as a full disk would stop a write (review R9-1)
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && { [ -z "${limit:-}" ] || ulimit -f "$limit"; } && bash "$SELF" "$@" 2>&1)"; rc=$?; LAST="$out"
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if printf '%s' "$out" | grep -q '^Traceback'; then
      echo "SELF-TEST: $name — a traceback, where a refusal is named:" >&2; printf '%s\n' "$out" | tail -2 | sed 's/^/    /' >&2; return
    fi
    if [ "$want" -ne 0 ] && printf '%s' "$out" | grep -qE '^task-history: [^ ]+ — sealed '; then
      echo "SELF-TEST: $name — a refused run that says it sealed:" >&2; printf '%s\n' "$out" | grep -E '^task-history: [^ ]+ — sealed ' | sed 's/^/    /' >&2; return
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
  printf -- '# V\n\n## Task Tree\n\n- ID: `V.1`\n  Status: `done`\n  Goal: a closed subtree, committed\n  Commit: `ARCHOGEN-V-0001`\n\n- ID: `V.1.1`\n  Status: `done`\n  Goal: its closed child\n  Commit: `ARCHOGEN-V-0002`\n' > "$work/docs/tasks/V.md"
  commit
  printf '\n- ID: `V.1.2`\n  Status: `pending`\n  Goal: an open child, not yet committed, which the seal reads and leg 5 does not\n' >> "$work/docs/tasks/V.md"
  arm "a first seal the gate refuses is rolled back" 1 "so it was rolled back" --seal V
  [ -z "$(git -C "$work" status --porcelain --untracked-files=all -- docs/task-history)" ] && [ ! -d "$work/docs/task-history" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a first seal rolled back left something under docs/task-history/, or the folder" >&2; }
  arm "and the gate passes after it" 0 "0 sealed file(s)"
  git -C "$work" checkout -q -- . && git -C "$work" reset -q --hard HEAD~1
  arm "the census counts bytes of closed subtrees below an open one" 0 "census: HEAD — 1181 byte(s) of \`done\` leaves under open top-level subtrees, 1062 of them in 7 closed subtree(s) below the top level" --census HEAD
  arm "the census of a commit git does not know is refused" 1 "failed" --census no-such-commit
  arm "the seal of a tree that does not exist is refused" 1 "does not exist" --seal NOPE
  arm "the seal proves its reconstruction" 0 "the reconstruction is byte for byte" --seal T
  arm "the sealed tree passes the gate" 0 "10 sealed file(s)"
  grep -q '^  Status: `done` — sealed in \[`T/T.2.5.md`\](../task-history/T/T.2.5.md); commit `ARCHOGEN-T-0022`$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: a Commit field naming its work unit on its second line did not give the stub its commit" >&2; }
  # Before the seal's first commit, a row's every field is its file's — leg 3 has no commit to compare yet.
  cp "$work/docs/task-history/INDEX.md" "$work/docs/index.keep"
  sub docs/task-history/INDEX.md '^(\| `T\.1` \| \d+ \| )\d+( \|)' '\g<1>999\2'
  arm "before its first commit, a row's line count unlike its file's is refused" 1 "lines, and its row says 999"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  sub docs/task-history/INDEX.md '^(\| `T\.1` \| \d+ \| \d+ \| )\d+( \|)' '\g<1>99999\2'
  arm "before its first commit, a row's byte count unlike its file's is refused" 1 "bytes, and its row says 99999"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  sub docs/task-history/INDEX.md '^(\| `T\.1` \| \d+ \| \d+ \| \d+ \| `)[0-9a-f]{64}' '\g<1>0000000000000000000000000000000000000000000000000000000000000000'
  arm "before its first commit, a row's sha256 unlike its file's is refused" 1 "sha256 is not its row's"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  sub docs/task-history/INDEX.md '^(\| `T\.1` \|.*\n)' '\1\n'
  arm "a row parted from its table by a blank line is refused" 1 "a row outside its tree's table, under its header"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  printf '\n## `T`\n\n| Subtree | Leaves | Lines | Bytes | sha256 | Sealed |\n| --- | --- | --- | --- | --- | --- |\n' >> "$work/docs/task-history/INDEX.md"
  arm "a second table for one tree is refused" 1 "a second table for T"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  sub docs/task-history/INDEX.md '^(\| Subtree \| Leaves \| Lines \| Bytes \| sha256 \| )Sealed( \|)' '\1Date\2'
  arm "a table that does not open with its header is refused" 1 "does not open with its header"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  sub docs/task-history/INDEX.md '^\| --- \| --- \| --- \| --- \| --- \| --- \|$' '| -- | --- | --- | --- | --- | --- |'
  arm "a table whose separator is not the header's is refused" 1 "does not open with its header"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  sub docs/task-history/INDEX.md '^(## `T`)\n\n' '\1\nprose between a heading and its table\n'
  arm "a table not a blank line below its heading is refused" 1 "does not open with its header"
  cp "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  printf '\n## `V`\n\n| Subtree | Leaves | Lines | Bytes | sha256 | Sealed |\n| --- | --- | --- | --- | --- | --- |\n' >> "$work/docs/task-history/INDEX.md"
  mkdir -p "$work/docs/task-history/V"
  arm "a table that holds no row is refused" 1 "the table of V holds no row"
  rmdir "$work/docs/task-history/V"
  mv "$work/docs/index.keep" "$work/docs/task-history/INDEX.md"
  grep -q '^  Status: `done` — sealed in \[`T/T.2.2.5.2.md`\](../task-history/T/T.2.2.5.2.md); commit `ARCHOGEN-T-0017`$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: two leaves four levels down with no leaf between them and an open subtree were not sealed apart" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.4.md`\](../task-history/T/T.4.md); commit `ARCHOGEN-T-0019`$' "$work/docs/tasks/T.md" &&
    grep -q '^| `T.4` | 2 |' "$work/docs/task-history/INDEX.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: a top-level subtree with no leaf of its own was not sealed whole, as before" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.2.3.md`\](../task-history/T/T.2.3.md); commit `ARCHOGEN-T-0008`$' "$work/docs/tasks/T.md" &&
    grep -q '^| `T.2.3` | 2 |' "$work/docs/task-history/INDEX.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: a closed subtree below an open one was not sealed whole, in one file" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.2.4.2.md`\](../task-history/T/T.2.4.2.md); commit `ARCHOGEN-T-0011`$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: two closed leaves with no leaf between them and an open subtree were not sealed apart" >&2; }
  grep -q '^  Goal: a closed leaf with an open child, which stays live' "$work/docs/tasks/T.md" || { arms=$((arms + 1)); echo "SELF-TEST: a closed leaf with an open child was sealed" >&2; }
  grep -q '^  Goal: an open subtree' "$work/docs/tasks/T.md" || { arms=$((arms + 1)); echo "SELF-TEST: the open subtree's own leaf was sealed" >&2; }
  grep -q '^  Status: `done` — sealed in \[`T/T.2.1.md`\](../task-history/T/T.2.1.md); commit not recorded$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: the open subtree's closed leaf was not sealed in a file of its own" >&2; }
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
  sub docs/task-history/INDEX.md '^(\| `T\.1` \|.*\n)' '\1\1'
  arm "a row twice for one sealed file is refused" 1 "lists T.1 twice in T's table"
  restore
  printf '\n' > "$work/docs/task-history/T/T.9.md"
  sub docs/task-history/INDEX.md '^(\| `T\.1` \|.*\n)' '\1| `T.9` | 0 | 1 | 1 | `01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b` | `2026-10-10` |\n'
  arm "a sealed file that holds no leaf is refused" 1 "T.9.md holds no leaf — a seal moves at least one"
  restore
  # A column-0 line in a closed subtree below an open one, in its second leaf: the seal refuses, nothing written.
  printf '\n- ID: `T.2.9`\n  Status: `done`\n  Goal: a closed subtree below an open one\n\n- ID: `T.2.9.1`\n  Status: `done`\n  Goal: its child, holding a line at column 0\n```text\ntorn\n```\n' >> "$work/docs/tasks/T.md"
  commit
  arm "a column-0 line in a later leaf of a unit below an open one is refused, nothing written" 1 "T.2.9.1, line" --seal T
  [ -z "$(git -C "$work" status --porcelain --untracked-files=all)" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a seal refused for a torn leaf wrote something" >&2; }
  git -C "$work" reset -q --hard HEAD~1
  printf '\n- ID: `T.2.10`\n  Status: `active`\n  Goal: a second leaf of one name\n' >> "$work/docs/tasks/T.md"
  arm "a leaf named twice in a tree is refused" 1 "holds T.2.10 2 times"
  arm "and the seal of that tree writes nothing" 1 "holds T.2.10 more than once — a leaf is named once; nothing was written" --seal T
  restore
  sub docs/task-history/INDEX.md '^(\| `T\.1` \| \d+ \| \d+ \| \d+ \| `)[0-9a-f]{64}' '\g<1>not-a-digest'
  arm "a row not in the row's form is refused" 1 "is not a row of the form"
  restore
  sub docs/tasks/T.md '\[`T/T\.1\.md`\]\(\.\./task-history/T/T\.1\.md\); commit `ARCHOGEN-T-0002`' '[`T/T.3.md`](../task-history/T/T.3.md); commit `ARCHOGEN-T-0002`'
  arm "a stub linking another sealed file is refused" 1 "links docs/task-history/T/T.3.md, which does not hold it"
  restore
  printf '\n- ID: `T.8`\n  Status: `done` — sealed in [`T/T.8.md`](../task-history/T/T.8.md); commit not recorded\n' >> "$work/docs/tasks/T.md"
  python3 - "$work" <<'PY'
import hashlib, sys
w = sys.argv[1]
leaf = "- ID: `T.8`\n  Status: `done`\n  Goal: a leaf the tree never held\n"
open(w + "/docs/task-history/T/T.8.md", "w").write(leaf)
i = w + "/docs/task-history/INDEX.md"
t = open(i).read().split("\n")
k = max(n for n, l in enumerate(t) if l.startswith("| `T."))
t.insert(k + 1, "| `T.8` | 1 | %d | %d | `%s` | `2026-10-10` |" % (leaf.count("\n"), len(leaf.encode()), hashlib.sha256(leaf.encode()).hexdigest()))
open(i, "w").write("\n".join(t))
PY
  arm "a sealed leaf its tree never held is refused" 1 "which HEAD's tree did not hold"
  restore
  printf -- '# Y\n\n## Task Tree\n\n- ID: `Y.1`\n  Status: `done` — sealed in [`Y/Y.1.md`](../task-history/Y/Y.1.md); commit not recorded\n' > "$work/docs/tasks/Y.md"
  python3 - "$work" <<'PY'
import hashlib, os, sys
w = sys.argv[1]
leaf = "- ID: `Y.1`\n  Status: `done`\n  Goal: a leaf of a tree no commit held\n"
os.makedirs(w + "/docs/task-history/Y")
open(w + "/docs/task-history/Y/Y.1.md", "w").write(leaf)
i = w + "/docs/task-history/INDEX.md"
t = open(i).read().rstrip("\n") + "\n\n## `Y`\n\n| Subtree | Leaves | Lines | Bytes | sha256 | Sealed |\n| --- | --- | --- | --- | --- | --- |\n"
t += "| `Y.1` | 1 | %d | %d | `%s` | `2026-10-10` |\n" % (leaf.count("\n"), len(leaf.encode()), hashlib.sha256(leaf.encode()).hexdigest())
open(i, "w").write(t)
PY
  arm "a sealed file of a tree its base did not hold is refused" 1 "had no docs/tasks/Y.md to be sealed from"
  restore
  # A refused seal into a history that exists puts the index back as it was.
  printf '\n- ID: `T.7`\n  Status: `done`\n  Goal: a closed subtree, committed unsealed\n  Commit: `ARCHOGEN-T-0020`\n\n- ID: `T.7.1`\n  Status: `done`\n  Goal: its closed child\n  Commit: `ARCHOGEN-T-0021`\n' >> "$work/docs/tasks/T.md"
  commit
  printf '\n- ID: `T.7.2`\n  Status: `pending`\n  Goal: an open child, not yet committed\n' >> "$work/docs/tasks/T.md"
  arm "a seal the gate refuses puts the index back" 1 "so it was rolled back" --seal T
  [ -z "$(git -C "$work" status --porcelain --untracked-files=all -- docs/task-history)" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a refused seal into an existing history left the history changed" >&2; }
  git -C "$work" checkout -q -- . && git -C "$work" reset -q --hard HEAD~1
  # A leaf's commit named only in the Commit Log the sealing commit adds: the stub says so, and the gate agrees.
  printf '\n- ID: `T.6`\n  Status: `done`\n  Goal: a closed subtree whose commit the tree'\''s Commit Log names\n  Commit: see the log\n' >> "$work/docs/tasks/T.md"
  commit
  printf '\n## Commit Log\n\n| Leaf | Commit | Notes |\n| --- | --- | --- |\n| `T.6` | `ARCHOGEN-T-0023 (leaf T.6)` | closed |\n' >> "$work/docs/tasks/T.md"
  arm "a commit the sealing commit's Commit Log names is the stub's" 0 "sealed 1 subtree(s), 1 leaves: T.6;" --seal T
  grep -q '^  Status: `done` — sealed in \[`T/T.6.md`\](../task-history/T/T.6.md); commit in the tree'\''s Commit Log$' "$work/docs/tasks/T.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: a commit the Commit Log names did not reach the stub" >&2; }
  restore
  git -C "$work" reset -q --hard HEAD~1
  # A failed history read is a breach, never an empty history: git made to fail on `--full-history`.
  mkdir -p "$work/fakebin"
  printf '#!/bin/sh\nfor a in "$@"; do [ "$a" = "--full-history" ] && exit 128; done\nexec %s "$@"\n' "$(command -v git)" > "$work/fakebin/git"
  chmod +x "$work/fakebin/git"
  for which in "docs/task-history/INDEX.md failed, so leg 3" "docs/task-history failed, so legs 3 and 5"; do
    arms=$((arms + 1))
    if out="$(cd "$work" && PATH="$work/fakebin:$PATH" bash "$SELF" 2>&1)"; then echo "SELF-TEST: a failed history read passed" >&2
    elif printf '%s' "$out" | grep -qF -- "\`git log\` of $which"; then ok=$((ok + 1)); echo "  ✅ a failed read of \`git log\` of ${which%%,*} is a breach"
    else echo "SELF-TEST: a failed history read was refused for another reason" >&2; fi
  done
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
  arm "a live leaf in a sealed subtree is refused" 1 "new work opens a new subtree outside it"
  restore
  printf '\n- ID: `T.2.1.1`\n  Status: `pending`\n  Goal: new work under a subtree sealed below an open one\n' >> "$work/docs/tasks/T.md"
  arm "a live leaf in a subtree sealed below an open one is refused" 1 "is live in subtree T.2.1, which is sealed"
  restore
  printf '\n- ID: `T.4`\n  Status: `pending`\n  Goal: a leaf named as a sealed subtree that had no leaf of its own\n' >> "$work/docs/tasks/T.md"
  arm "a live leaf named as a sealed subtree is refused" 1 "is live in subtree T.4, which is sealed in T"
  restore
  mkdir -p "$work/docs/tasks" && printf -- '# V\n\n## Task Tree\n\n- ID: `T.2.3.9`\n  Status: `pending`\n  Goal: new work under a sealed subtree, filed in another tree\n' > "$work/docs/tasks/V.md"
  arm "a live leaf under a sealed subtree in another tree file is refused" 1 "is live in subtree T.2.3, which is sealed in T"
  restore
  printf -- '# V\n\n## Task Tree\n\n- ID: `V.1`\n  Status: `done`\n  Goal: a closed subtree the seal would take\n  Commit: `ARCHOGEN-V-0003`\n\n- ID: `T.9`\n  Status: `pending`\n  Goal: a leaf filed under another tree\x27s name\n' > "$work/docs/tasks/V.md"
  arm "a leaf filed under another tree's name is refused" 1 "is not under its tree's name V"
  arms=$((arms + 1)); out="$(cd "$work" && bash "$SELF" --seal V 2>&1)"
  if printf '%s' "$out" | grep -qF "not under its tree's name V — a leaf is filed in its own tree; nothing was written" &&
     ! printf '%s' "$out" | grep -qF "rolled back"; then ok=$((ok + 1)); echo "  ✅ and the seal of that tree refuses before it writes anything"
  else echo "SELF-TEST: the seal of a tree holding a foreign leaf wrote before refusing:" >&2; printf '%s\n' "$out" | sed -n '1,2p' | sed 's/^/    /' >&2; fi
  restore
  printf '\n- ID: `T.1.9`\n  Status: `done`\n  Goal: a closed leaf added late under a sealed subtree\n' >> "$work/docs/tasks/T.md"
  arm "a late closed leaf under a sealed subtree is refused, its file left as sealed" 1 "new work opens a new subtree outside it" --seal T
  git -C "$work" diff --quiet -- docs/task-history || { arms=$((arms + 1)); echo "SELF-TEST: a seal over a late leaf rewrote a sealed file" >&2; }
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
  # A seal made by hand of a done leaf whose child is open, with its file, row and stub: provenance refuses it.
  python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
t = open(w + "/docs/tasks/T.md").read()
m = re.search(r"^- ID: `T\.2\.2`\n(?:  .*\n)*", t, re.M)
leaf = m.group(0)
t = t.replace(leaf, "- ID: `T.2.2`\n  Status: `done` — sealed in [`T/T.2.2.md`](../task-history/T/T.2.2.md); commit `ARCHOGEN-T-0004`\n")
t = re.sub(r"^- ID: `T\.2\.2\.1`\n(?:  .*\n)*\n?", "", t, flags=re.M)  # leg 6 then has no live leaf to see
open(w + "/docs/tasks/T.md", "w").write(t)
open(w + "/docs/task-history/T/T.2.2.md", "w").write(leaf)
i = open(w + "/docs/task-history/INDEX.md").read().rstrip("\n") + "\n| `T.2.2` | 1 | %d | %d | `%s` | `2026-09-30` |\n" % (leaf.count("\n"), len(leaf.encode()), hashlib.sha256(leaf.encode()).hexdigest())
open(w + "/docs/task-history/INDEX.md", "w").write(i)
PY
  arm "a seal made by hand of a subtree with an open leaf is refused" 1 "a subtree is sealed whole"
  restore
  # Below an open subtree, a part sealed by hand apart from its closed subtree; and parts no one leaf holds sealed together.
  printf '\n- ID: `T.2.6`\n  Status: `done`\n  Goal: a closed subtree below an open one\n  Commit: `ARCHOGEN-T-0012`\n\n- ID: `T.2.6.1`\n  Status: `done`\n  Goal: its closed child\n  Commit: `ARCHOGEN-T-0013`\n\n- ID: `T.2.7.1`\n  Status: `done`\n  Goal: a closed leaf with no leaf `T.2.7` above it\n  Commit: `ARCHOGEN-T-0014`\n\n- ID: `T.2.7.2`\n  Status: `done`\n  Goal: its cousin\n  Commit: `ARCHOGEN-T-0015`\n' >> "$work/docs/tasks/T.md"
  commit
  hand_seal() { # $1 = the file's key, then the leaves it takes
    python3 - "$work" "$@" <<'PY'
import hashlib, re, sys
w, key, ids = sys.argv[1], sys.argv[2], sys.argv[3:]
t = open(w + "/docs/tasks/T.md").read()
leaves = []
for lid in ids:
    m = re.search(r"^- ID: `%s`\n(?:  .*\n)*" % re.escape(lid), t, re.M)
    leaves.append(m.group(0))
    t = t.replace(m.group(0), "- ID: `%s`\n  Status: `done` — sealed in [`T/%s.md`](../task-history/T/%s.md); commit not recorded\n" % (lid, key, key))
open(w + "/docs/tasks/T.md", "w").write(t)
body = "\n".join(leaves)
open(w + "/docs/task-history/T/%s.md" % key, "w").write(body)
i = open(w + "/docs/task-history/INDEX.md").read().split("\n")
k = max(n for n, l in enumerate(i) if l.startswith("| `T"))  # after T's last row, in T's own table
i.insert(k + 1, "| `%s` | %d | %d | %d | `%s` | `2026-10-10` |" % (key, len(ids), body.count("\n"), len(body.encode()), hashlib.sha256(body.encode()).hexdigest()))
open(w + "/docs/task-history/INDEX.md", "w").write("\n".join(i))
PY
  }
  hand_seal T.2.6.1 T.2.6.1
  arm "below an open subtree, a part sealed apart from its closed subtree is refused" 1 "a seal takes the outermost closed subtree"
  restore
  hand_seal T.2.7 T.2.7.1 T.2.7.2
  arm "parts sealed together that no one leaf holds are refused" 1 "T.2.7.1 in T.2.7.1"
  restore
  git -C "$work" reset -q --hard HEAD~1
  # A part of a closed subtree sealed by hand apart from it: the seal takes the outermost closed subtree.
  printf '\n- ID: `T.5`\n  Status: `done`\n  Goal: a closed subtree\n  Commit: `ARCHOGEN-T-0005`\n\n- ID: `T.5.1`\n  Status: `done`\n  Goal: its closed child\n  Commit: `ARCHOGEN-T-0006`\n' >> "$work/docs/tasks/T.md"
  commit
  python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
t = open(w + "/docs/tasks/T.md").read()
m = re.search(r"^- ID: `T\.5\.1`\n(?:  .*\n)*", t, re.M)
leaf = m.group(0)
t = t.replace(leaf, "- ID: `T.5.1`\n  Status: `done` — sealed in [`T/T.5.1.md`](../task-history/T/T.5.1.md); commit `ARCHOGEN-T-0006`\n")
open(w + "/docs/tasks/T.md", "w").write(t)
open(w + "/docs/task-history/T/T.5.1.md", "w").write(leaf)
i = open(w + "/docs/task-history/INDEX.md").read().rstrip("\n") + "\n| `T.5.1` | 1 | %d | %d | `%s` | `2026-09-30` |\n" % (leaf.count("\n"), len(leaf.encode()), hashlib.sha256(leaf.encode()).hexdigest())
open(w + "/docs/task-history/INDEX.md", "w").write(i)
PY
  arm "a part sealed apart from its closed subtree is refused" 1 "a seal takes the outermost closed subtree"
  restore
  git -C "$work" reset -q --hard HEAD~1
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
  python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
f = w + "/docs/task-history/T/T.1.md"
parts = re.split(r"(?m)^(?=- ID: )", open(f).read())
s = "".join([parts[0]] + parts[1:][::-1])
s = s if s.endswith("\n") else s + "\n"
open(f, "w").write(s)
i = w + "/docs/task-history/INDEX.md"
t = re.sub(r"^\| `T\.1` \| (\d+) \| (\d+) \| (\d+) \| `[0-9a-f]+` \|", lambda m: "| `T.1` | %s | %d | %d | `%s` |" % (m.group(1), s.count("\n"), len(s.encode()), hashlib.sha256(s.encode()).hexdigest()), open(i).read(), flags=re.M)
open(i, "w").write(t)
PY
  arm "a sealed file's leaves reordered with its row, before any commit, is refused" 1 "a seal writes them as they stood"
  restore
  mkdir -p "$work/docs/tasks" && printf -- '# V\n\n## Task Tree\n\n- ID: `V.1`\n  Status: `done` — sealed in [`T/T.1.md`](../task-history/T/T.1.md); commit `ARCHOGEN-T-0001`\n' > "$work/docs/tasks/V.md"
  arm "a stub in another tree is refused" 1 "links another tree's file"
  restore
  arm "and the sealed tree passes again" 0 ""
  printf -- '# W\n\n## Task Tree\n\n- ID: `W.1`\n  Status: `done`\n  Goal: a second tree'\''s closed subtree\n  Commit: `ARCHOGEN-W-0001`\n' > "$work/docs/tasks/W.md"
  commit
  arm "a second tree is sealed into a table of its own" 0 "sealed 1 subtree(s), 1 leaves: W.1;" --seal W
  commit
  sub docs/tasks/T.md '^(- ID: `T\.2`\n  Status: )`active`' '\1`done`'
  sub docs/tasks/T.md '^(- ID: `T\.2\.2\.1`\n  Status: )`blocked`' '\1`done`'
  sub docs/tasks/T.md '^(- ID: `T\.2\.10`\n  Status: )`active`' '\1`done`'
  commit
  arms=$((arms + 1)); out="$(cd "$work" && bash "$SELF" --seal T 2>&1)"
  if printf '%s' "$out" | grep -qF "sealed 1 subtree(s), 4 leaves: T.2;" &&
     printf '%s' "$out" | grep -qF "warning — T.2 is \`done\` and its Commit field names no commit" &&
     ! printf '%s' "$out" | grep -qF "warning — T.2.2 is"; then
    ok=$((ok + 1)); echo "  ✅ a parent closing later is sealed beside its sealed parts, warning of a leaf that names no commit alone"
  else echo "SELF-TEST: a parent closing later was not sealed beside its parts, with its one warning:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; fi
  arm "and all its files pass, each tree's rows in its own table" 0 "12 sealed file(s)"
  sub docs/tasks/T.md '^(- ID: `T`\n  Status: )`active`' '\1`done`'
  commit
  hand_seal T T
  arm "the tree's root leaf sealed by hand is refused" 1 "T in no closed subtree"
  restore
  printf '# Z\n\n## Task Tree\n\n- ID: `Z.1`\n  Status: `pending`\n  Goal: not UTF-8 \377\n' > "$work/docs/tasks/Z.md"
  commit
  arm "a tree that is not UTF-8 is a breach the gate names" 1 "is not UTF-8"
  arm "and one the census names" 1 "is not UTF-8" --census HEAD
  printf -- '# Q\n\n## Task Tree\n\n- ID: `Q.1`\n  Status: `done`\n  Goal: a closed subtree sealed beside a tree the gate cannot read\n  Commit: `ARCHOGEN-Q-0001`\n' > "$work/docs/tasks/Q.md"
  commit
  arm "a seal the gate cannot judge after it wrote is rolled back, the breach named" 1 "TASK-HISTORY: docs/tasks/Z.md is not UTF-8" --seal Q
  [ -z "$(git -C "$work" status --porcelain --untracked-files=all -- docs/task-history docs/tasks)" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a seal refused on an unreadable file left its writes behind" >&2; }
  git -C "$work" rm -q docs/tasks/Z.md
  commit
  mkdir "$work/docs/tasks/R.md"
  arm "a seal the gate cannot open a file after it wrote is rolled back, the failure named" 1 "TASK-HISTORY: [Errno 21] Is a directory" --seal Q
  rmdir "$work/docs/tasks/R.md"
  [ -z "$(git -C "$work" status --porcelain --untracked-files=all -- docs/task-history docs/tasks)" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a seal stopped by a file it cannot open left its writes behind" >&2; }
  clean() { [ -z "$(git -C "$work" status --porcelain --untracked-files=all -- docs/task-history docs/tasks)" ]; }
  # A file-size limit the tree's rewrite exceeds and the sealed file does not, as a full disk: the tree is written
  # whole or not at all, and the rollback undoes only what was written (review R9-1).
  printf -- '# P\n\n## Task Tree\n\n- ID: `P`\n  Status: `active`\n  Goal: %s\n\n- ID: `P.1`\n  Status: `done`\n  Goal: a closed subtree in a tree too large to rewrite\n  Commit: `ARCHOGEN-P-0001`\n' "$(python3 -c 'print("x" * 20000)')" > "$work/docs/tasks/P.md"
  commit
  limit=16 arm "a seal a full disk stops is rolled back, the tree as it was" 1 "File too large" --seal P
  clean && [ ! -d "$work/docs/task-history/P" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a seal stopped by a file-size limit left the tree torn or its writes behind" >&2; }
  restore
  # A rollback the same limit stops: the seal's smaller tree fits, the gate then cannot read a tree, and restoring the
  # larger one fails. The rollback names what it could not undo and keeps the sealed file the tree names (review R9-1).
  printf '# Z\n\n## Task Tree\n\n- ID: `Z.1`\n  Status: `pending`\n  Goal: not UTF-8 \377\n' > "$work/docs/tasks/Z.md"
  printf -- '# P\n\n## Task Tree\n\n- ID: `P`\n  Status: `active`\n  Goal: %s\n\n- ID: `P.1`\n  Status: `done`\n  Goal: %s\n  Commit: `ARCHOGEN-P-0001`\n' "$(python3 -c 'print("x" * 10000)')" "$(python3 -c 'print("y" * 8000)')" > "$work/docs/tasks/P.md"
  commit
  limit=16 arm "a rollback a full disk stops names what it could not undo" 1 "could not undo docs/tasks/P.md, not restored" --seal P
  printf '%s' "$LAST" | grep -qF "and its rollback left what it named above" && ! printf '%s' "$LAST" | grep -qF "rolled back" ||
    { arms=$((arms + 1)); echo "SELF-TEST: an error's rollback that was not whole says it was rolled back (review R15 AG2)" >&2; }
  grep -q '^  Status: `done` — sealed in' "$work/docs/tasks/P.md" && [ -f "$work/docs/task-history/P/P.1.md" ] &&
    [ -z "$(git -C "$work" status --porcelain -- docs/task-history/INDEX.md)" ] ||
    { arms=$((arms + 1)); echo "SELF-TEST: a rollback that could not restore the tree removed the sealed file it names, or tore it" >&2; }
  restore
  git -C "$work" rm -q docs/tasks/Z.md docs/tasks/P.md
  commit
  # A sealed file's path already taken — here by a dangling link, which `os.path.exists` calls absent: refused before
  # anything is written (review R9-2).
  mkdir "$work/docs/task-history/Q"; ln -s "$work/no-such-dir/Q.1.md" "$work/docs/task-history/Q/Q.1.md"
  arm "a sealed file's path already taken, a dangling link, is refused before anything is written" 1 "nothing was written" --seal Q
  rm "$work/docs/task-history/Q/Q.1.md"; rmdir "$work/docs/task-history/Q"
  clean || { arms=$((arms + 1)); echo "SELF-TEST: a seal refused for a taken path wrote something" >&2; }
  restore
  # A stop during the seal is held by the signal mask until the seal is done — proven and kept, or refused and rolled
  # back — and then takes effect with its own exit code, the seal whole either way (review R13). Mid-proof, a fake git
  # sends the interrupt; git output that is not UTF-8 is an exception instead, rolled back and ending in its traceback
  # (review R9-2).
  proven() { [ -f "$work/docs/task-history/Q/Q.1.md" ] && (cd "$work" && bash "$SELF" >/dev/null 2>&1); }
  fake="$SCRATCH/fakebin"; rm -rf "$fake"; mkdir -p "$fake"
  for how in interrupt undecodable; do
    if [ "$how" = interrupt ]; then act='kill -INT $PPID'; else act='printf "\\377\\n"; exit 0'; fi
    printf '#!/bin/sh\nfor a in "$@"; do [ "$a" = "--is-shallow-repository" ] && { %s; }; done\nexec "%s" "$@"\n' "$act" "$(command -v git)" > "$fake/git"
    chmod +x "$fake/git"
    arms=$((arms + 1))
    out="$(cd "$work" && PATH="$fake:$PATH" bash "$SELF" --seal Q 2>&1)"; rc=$?
    if [ "$how" = interrupt ]; then
      if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qE '^task-history: [^ ]+ — sealed ' && printf '%s' "$out" | grep -qF "interrupted once the seal was done; its outcome is said above" && proven; then
        ok=$((ok + 1)); echo "  ✅ an interrupt mid-proof waits for the seal, which is proven and kept"
      else
        echo "SELF-TEST: an interrupt mid-proof — rc $rc, or the seal not finished and proven:" >&2
        printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
      fi
    elif [ "$rc" -ne 0 ] && printf '%s' "$out" | grep -qF UnicodeDecodeError && ! printf '%s' "$out" | grep -qE '^task-history: [^ ]+ — sealed ' && clean &&
         [ ! -d "$work/docs/task-history/Q" ]; then
      ok=$((ok + 1)); echo "  ✅ git output that is not UTF-8, mid-proof, rolls the seal back"
    else
      echo "SELF-TEST: git output that is not UTF-8 mid-proof — rc $rc, or its writes left behind:" >&2
      printf '%s\n' "$out" | tail -2 | sed 's/^/    /' >&2
    fi
    restore
  done
  rm -rf "$fake"
  # A stop just after or before a write, from a hook on the interpreter's path: held until the seal is done.
  hook="$SCRATCH/hook"; rm -rf "$hook"; mkdir -p "$hook"
  cat > "$hook/sitecustomize.py" <<'HOOK'
import builtins, os, signal
WHEN, FIRED, TREES = os.environ.get("SEAL_HOOK", ""), set(), []
def fire(sig=signal.SIGINT):
    if sig not in FIRED:
        FIRED.add(sig); os.kill(os.getpid(), sig)
if WHEN:
    real_replace, real_open, real_makedirs, real_mask = os.replace, builtins.open, os.makedirs, signal.pthread_sigmask
    if WHEN == "tmplink":
        os.getpid = lambda: 4242  # so the temporary file's name is known before the run
    def makedirs(name, *a, **k):
        if WHEN == "mkdirfail" and os.path.abspath(str(name)).endswith("/task-history/Q"):
            fire()
            raise OSError("the folder is held")  # the first write fails, under a held interrupt
        real_makedirs(name, *a, **k)
        if WHEN in ("mkdir", "mkdirplant") and os.path.abspath(str(name)).endswith("/task-history/Q"):
            fire()
        if WHEN == "maskcheck" and os.path.abspath(str(name)).endswith("/task-history/Q"):
            left = {getattr(signal, s) for s in ("SIGKILL", "SIGSTOP", "SIGSEGV", "SIGBUS", "SIGFPE", "SIGILL", "SIGTRAP",
                                                 "SIGSYS") if hasattr(signal, s)}
            libc = set(range(32, signal.SIGRTMIN)) if hasattr(signal, "SIGRTMIN") else set()  # the C library's own
            mask = set(real_mask(signal.SIG_BLOCK, []))
            missing, held = signal.valid_signals() - left - libc - mask, left & mask
            with real_open(os.environ["SEAL_MASKCHECK"], "w") as out:
                out.write("ok\n" if not missing and not held else "missing %s, held %s\n" % (sorted(missing), sorted(held)))
    def mask(how, sigs):
        if WHEN == "early":  # a stop before the mask is set: before the first write
            fire()
        return real_mask(how, sigs)
    def replace(src, dst, *a, **k):
        if WHEN in ("refusedhold", "refusedint") and os.path.abspath(str(dst)).endswith("/docs/tasks/K.md"):
            TREES.append(1)
            if WHEN == "refusedint" and len(TREES) == 1:
                fire()
            if len(TREES) == 2:  # the rollback's restore of the tree fails
                raise OSError("the tree is held")
        if WHEN == "errhold" and os.path.abspath(str(dst)).endswith("/INDEX.md"):
            raise OSError("the index is held")  # an error, after a stop held since the tree's rename
        real_replace(src, dst, *a, **k)
        dst = os.path.abspath(str(dst))
        if WHEN == "errhold" and dst.endswith("/docs/tasks/Q.md"):
            fire()
        if WHEN.startswith("sig:") and dst.endswith("/docs/tasks/Q.md"):
            fire(getattr(signal, "SIG" + WHEN[4:]))
        if WHEN in ("tree", "double") and dst.endswith("/docs/tasks/Q.md"):
            fire()
        if WHEN == "index" and dst.endswith("/INDEX.md"):
            fire()
        if WHEN == "double" and dst.endswith("/INDEX.md"):
            fire(signal.SIGTERM)
        if WHEN == "term" and dst.endswith("/docs/tasks/Q.md"):
            fire(signal.SIGTERM)
        if WHEN == "hup" and dst.endswith("/docs/tasks/Q.md"):
            fire(signal.SIGHUP)
        if WHEN == "restore" and dst.endswith("/docs/tasks/K.md"):
            TREES.append(1)
            if len(TREES) == 2:  # the second rename of the tree is the rollback's restore
                fire()
    def opener(file, mode="r", *a, **k):
        if WHEN == "precreate" and mode == "x" and "/task-history/" in os.path.abspath(str(file)):
            fire()
        if WHEN == "vanish" and mode == "x" and "/task-history/" in os.path.abspath(str(file)):
            os.rmdir(os.path.dirname(os.path.abspath(str(file))))
        if WHEN in ("plant", "mkdirplant") and mode == "x" and "/task-history/" in os.path.abspath(str(file)):
            with real_open(file, "w") as other:
                other.write("another writer's\n")
        f = real_open(file, mode, *a, **k)
        if WHEN == "create" and mode == "x" and "/task-history/" in os.path.abspath(str(file)):
            fire()
        return f
    os.replace, builtins.open, os.makedirs, signal.pthread_sigmask = replace, opener, makedirs, mask
HOOK
  for when in mkdir tree index create precreate term hup double sig:QUIT sig:USR1 sig:USR2 sig:ALRM; do
    case "$when" in
      mkdir) what="an interrupt just after the history folder's creation, the first write"; want=130 ;;
      sig:*) what="a SIG${when#sig:} just after the tree's rename"; want=$((128 + $(kill -l "${when#sig:}"))) ;;
      tree) what="an interrupt just after the tree's rename"; want=130 ;;
      index) what="an interrupt just after the index's rename"; want=130 ;;
      create) what="an interrupt just after a sealed file's creation"; want=130 ;;
      precreate) what="an interrupt just before a sealed file's creation"; want=130 ;;
      term) what="a request to terminate just after the tree's rename"; want=143 ;;
      hup) what="a hang-up just after the tree's rename"; want=129 ;;
      double) what="an interrupt, then a request to terminate, mid-seal"; want="130 143" ;;
    esac
    arms=$((arms + 1))
    out="$(cd "$work" && ulimit -c 0 && SEAL_HOOK="$when" PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
    if case " $want " in *" $rc "*) true ;; *) false ;; esac && printf '%s' "$out" | grep -qE '^task-history: [^ ]+ — sealed ' && proven; then
      ok=$((ok + 1)); echo "  ✅ $what waits for the seal, proven and kept, then exits $rc"
    else
      echo "SELF-TEST: $what — rc $rc, not $want, or the seal not finished and proven:" >&2
      printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
    fi
    restore
  done
  # A stop before the mask is set finds nothing written, and says so (review R14 D1 c).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=early PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "interrupted before the seal wrote anything" && clean &&
     [ ! -e "$work/docs/task-history/Q" ]; then
    ok=$((ok + 1)); echo "  ✅ a stop before the first write leaves nothing written, and says so"
  else
    echo "SELF-TEST: a stop before the first write — rc $rc, a write left, or another message:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  restore; rm -rf "$work/docs/task-history/Q"
  # An error that stops the seal while a stop is held: the rollback runs under the mask, the error is said, and only
  # then the stop takes effect (reviews R14 D1 b, AG1).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=errhold PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "stopped on OSError: the index is held, so it was rolled back" &&
     printf '%s' "$out" | grep -qF "interrupted once the seal was done; its outcome is said above" && clean &&
     [ ! -e "$work/docs/task-history/Q" ]; then
    ok=$((ok + 1)); echo "  ✅ an error under a held stop is rolled back and said before the stop takes effect"
  else
    echo "SELF-TEST: an error under a held stop — rc $rc, a write left, or the error unsaid:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  restore; rm -rf "$work/docs/task-history/Q"
  # An interrupt during the rollback of a seal the gate refused: held until the rollback is done (review R13).
  printf -- '# K\n\n## Task Tree\n\n- ID: `K.1`\n  Status: `done`\n  Goal: a closed subtree\n  Commit: `ARCHOGEN-K-0001`\n\n- ID: `K.1.1`\n  Status: `done`\n  Goal: its closed child\n  Commit: `ARCHOGEN-K-0002`\n' > "$work/docs/tasks/K.md"
  commit
  printf '\n- ID: `K.1.2`\n  Status: `pending`\n  Goal: an open child, not yet committed\n' >> "$work/docs/tasks/K.md"
  cp "$work/docs/tasks/K.md" "$SCRATCH/K.keep"
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=restore PYTHONPATH="$hook" bash "$SELF" --seal K 2>&1)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "so it was rolled back" && cmp -s "$SCRATCH/K.keep" "$work/docs/tasks/K.md" &&
     [ ! -e "$work/docs/task-history/K" ] && git -C "$work" diff --quiet -- docs/task-history; then
    ok=$((ok + 1)); echo "  ✅ an interrupt during a refused seal's rollback waits for it to end"
  else
    echo "SELF-TEST: an interrupt during a refused seal's rollback — rc $rc, or a write left behind:" >&2
    printf '%s\n' "$out" | tail -2 | sed 's/^/    /' >&2
  fi
  rm -rf "$work/docs/task-history/K"; restore
  # A rollback whose restore of the tree fails never says the seal was rolled back (review R12 D3).
  cp "$SCRATCH/K.keep" "$work/docs/tasks/K.md"
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=refusedhold PYTHONPATH="$hook" bash "$SELF" --seal K 2>&1)"; rc=$?
  if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -qF "and its rollback left what it named above" && ! printf '%s' "$out" | grep -qF "so it was rolled back"; then
    ok=$((ok + 1)); echo "  ✅ a refused seal whose rollback is not whole does not say it was rolled back"
  else
    echo "SELF-TEST: a refused seal whose rollback is not whole — rc $rc, or it says rolled back:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  rm -rf "$work/docs/task-history/K"; restore
  cp "$SCRATCH/K.keep" "$work/docs/tasks/K.md"
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=refusedint PYTHONPATH="$hook" bash "$SELF" --seal K 2>&1)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "and its rollback left what it named above" &&
     printf '%s' "$out" | grep -qF "interrupted once the seal was done; its outcome is said above" &&
     ! printf '%s' "$out" | grep -qF "rolled back"; then
    ok=$((ok + 1)); echo "  ✅ an interrupt under a rollback that is not whole never says rolled back"
  else
    echo "SELF-TEST: an interrupt under a rollback that is not whole — rc $rc, or it says rolled back:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  rm -rf "$work/docs/task-history/K"; restore
  git -C "$work" rm -q docs/tasks/K.md; commit
  # Another writer's file appearing at a sealed file's path just before the seal creates it: the seal stops, and the
  # rollback leaves that file as the other writer left it (review R11 AG2).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=plant PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -ne 0 ] && printf '%s' "$out" | grep -qF "File exists" &&
     [ "$(cat "$work/docs/task-history/Q/Q.1.md" 2>/dev/null)" = "another writer's" ] && git -C "$work" diff --quiet -- docs/tasks docs/task-history; then
    ok=$((ok + 1)); echo "  ✅ another writer's file at a sealed path is left as it was"
  else
    echo "SELF-TEST: another writer's file at a sealed path — rc $rc, or the rollback removed it:" >&2
    printf '%s\n' "$out" | tail -2 | sed 's/^/    /' >&2
  fi
  rm -rf "$work/docs/task-history/Q"; restore
  # A link already at the temporary file's path: refused before the first write, and left as it was (reviews R13 D3, R14 D3).
  ln -s "$SCRATCH/elsewhere.md" "$work/docs/tasks/.Q.md.seal-4242"
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=tmplink PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -qF "an entry at a temporary file's path; nothing was written" &&
     [ ! -e "$SCRATCH/elsewhere.md" ] && [ -L "$work/docs/tasks/.Q.md.seal-4242" ] && [ ! -L "$work/docs/tasks/Q.md" ] &&
     [ ! -e "$work/docs/task-history/Q" ] && git -C "$work" diff --quiet -- docs/tasks docs/task-history; then
    ok=$((ok + 1)); echo "  ✅ a link at the temporary path is refused before the first write, and left as it was"
  else
    echo "SELF-TEST: a link at the temporary path — rc $rc, or written through or removed:" >&2
    printf '%s\n' "$out" | tail -2 | sed 's/^/    /' >&2
  fi
  rm -f "$work/docs/tasks/.Q.md.seal-4242" "$SCRATCH/elsewhere.md"; rm -rf "$work/docs/task-history/Q"; restore
  # A plain file at the index's temporary path: refused before the first write too, and left as it was (review R15 AG3).
  mkdir -p "$work/docs/task-history"; printf 'another writer\n' > "$work/docs/task-history/.INDEX.md.seal-4242"
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=tmplink PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -qF "an entry at a temporary file's path; nothing was written" &&
     [ "$(cat "$work/docs/task-history/.INDEX.md.seal-4242")" = "another writer" ] && [ ! -e "$work/docs/task-history/Q" ] &&
     git -C "$work" diff --quiet -- docs/tasks docs/task-history; then
    ok=$((ok + 1)); echo "  ✅ a plain file at the index's temporary path is refused before the first write"
  else
    echo "SELF-TEST: a plain file at the index's temporary path — rc $rc, or written or removed:" >&2
    printf '%s\n' "$out" | tail -2 | sed 's/^/    /' >&2
  fi
  rm -f "$work/docs/task-history/.INDEX.md.seal-4242"; restore
  # The mask in force at the first write holds every signal but SIGKILL, SIGSTOP, the six fault signals and the C
  # library's own, and none of the eight others (reviews R15 AG1, R16 AG1).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=maskcheck SEAL_MASKCHECK="$SCRATCH/maskcheck" PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 0 ] && [ "$(cat "$SCRATCH/maskcheck" 2>/dev/null)" = "ok" ] && proven; then
    ok=$((ok + 1)); echo "  ✅ the mask at the first write holds every signal but the eight, and none of them"
  else
    echo "SELF-TEST: the mask at the first write — rc $rc, $(cat "$SCRATCH/maskcheck" 2>/dev/null)" >&2
  fi
  rm -f "$SCRATCH/maskcheck"; restore
  # A stop held while the seal's own report cannot be written, its output a pipe nobody reads: the seal kept, and the
  # stop says the outcome could not be said, never that nothing was written (review R15 D1).
  arms=$((arms + 1))
  (cd "$work" && SEAL_HOOK=tree PYTHONPATH="$hook" bash "$SELF" --seal Q 2>"$SCRATCH/err" | true; echo "${PIPESTATUS[0]}" > "$SCRATCH/rc")
  if [ "$(cat "$SCRATCH/rc")" = 130 ] && grep -qF "interrupted once the seal was done; its outcome could not be said" "$SCRATCH/err" &&
     ! grep -qF "before the seal wrote anything" "$SCRATCH/err" && proven; then
    ok=$((ok + 1)); echo "  ✅ a stop after a seal whose report could not be written says so"
  else
    echo "SELF-TEST: a stop after an unwritten report:" >&2; tail -3 "$SCRATCH/err" | sed 's/^/    /' >&2
  fi
  rm -f "$SCRATCH/err" "$SCRATCH/rc"; restore
  # The start recorded before the first write: an interrupt held at the folder's creation, then another writer's file at
  # the sealed path — the seal stops on it, is rolled back, says so, and the interrupt names an outcome said, never a
  # seal that wrote nothing (review R16 AG2).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=mkdirplant PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "stopped on FileExistsError" &&
     printf '%s' "$out" | grep -qF "interrupted once the seal was done; its outcome is said above" &&
     ! printf '%s' "$out" | grep -qF "before the seal wrote anything"; then
    ok=$((ok + 1)); echo "  ✅ a stop held from the folder's creation names the outcome said, the start recorded first"
  else
    echo "SELF-TEST: a stop held from the folder's creation — rc $rc, or the wrong message:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  rm -rf "$work/docs/task-history/Q"; restore
  # The first write itself failing under a held interrupt: the start was recorded before it (review R16 AG2).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=mkdirfail PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "stopped on OSError: the folder is held" &&
     printf '%s' "$out" | grep -qF "interrupted once the seal was done; its outcome is said above" && clean; then
    ok=$((ok + 1)); echo "  ✅ a first write that fails under a held stop finds the start recorded before it"
  else
    echo "SELF-TEST: a first write that fails under a held stop — rc $rc, or the wrong message:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  restore
  # The outputs closed or broken: the interrupt still exits 130, says what it can, and never claims an outcome said that
  # went nowhere (reviews R17 D1, D2, D3).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=tree PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1 >&-)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "once the seal was done; its outcome could not be said" &&
     ! printf '%s' "$out" | grep -q 'Traceback' && proven; then
    ok=$((ok + 1)); echo "  ✅ an interrupt with the output closed exits 130 and says its outcome went unsaid"
  else
    echo "SELF-TEST: an interrupt with the output closed — rc $rc:" >&2; printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  restore
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=tree PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&-)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qE '^task-history: [^ ]+ — sealed ' && proven; then
    ok=$((ok + 1)); echo "  ✅ an interrupt with the error output closed exits 130, the seal kept"
  else
    echo "SELF-TEST: an interrupt with the error output closed — rc $rc" >&2
  fi
  restore
  arms=$((arms + 1))
  (cd "$work" && SEAL_HOOK=tree PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1 | true; echo "${PIPESTATUS[0]}" > "$SCRATCH/rc")
  if [ "$(cat "$SCRATCH/rc")" = 130 ] && proven; then
    ok=$((ok + 1)); echo "  ✅ an interrupt with both outputs a pipe nobody reads exits 130, the seal kept"
  else
    echo "SELF-TEST: an interrupt with both outputs broken — rc $(cat "$SCRATCH/rc")" >&2
  fi
  rm -f "$SCRATCH/rc"; restore
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=early PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1 >&-)"; rc=$?
  if [ "$rc" -eq 130 ] && printf '%s' "$out" | grep -qF "before the seal wrote anything" && ! printf '%s' "$out" | grep -q 'Traceback' && clean; then
    ok=$((ok + 1)); echo "  ✅ an interrupt before any write, the output closed, exits 130"
  else
    echo "SELF-TEST: an interrupt before any write with the output closed — rc $rc" >&2
  fi
  restore; rm -rf "$work/docs/task-history/Q"
  # The git the proof runs inherits the mask (review R17 AG1): a git that reads its own mask finds the interrupt held.
  fake="$SCRATCH/fakegit"; rm -rf "$fake"; mkdir -p "$fake"
  printf '#!/usr/bin/env python3\nimport os, signal, sys\nwith open(os.environ["SEAL_GITMASK"], "a") as out:\n    out.write("held\\n" if signal.SIGINT in signal.pthread_sigmask(signal.SIG_BLOCK, []) else "open\\n")\nos.execv(%s, ["git"] + sys.argv[1:])\n' "\"$(command -v git)\"" > "$fake/git"
  chmod +x "$fake/git"; : > "$SCRATCH/gitmask"
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_GITMASK="$SCRATCH/gitmask" PATH="$fake:$PATH" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 0 ] && grep -qx held "$SCRATCH/gitmask" && proven; then
    ok=$((ok + 1)); echo "  ✅ the git the proof runs inherits the mask"
  else
    echo "SELF-TEST: the git the proof runs — rc $rc, $(sort "$SCRATCH/gitmask" | uniq -c | tr '\n' ' ')" >&2
  fi
  rm -rf "$fake" "$SCRATCH/gitmask"; restore
  # A hang-up ignored on entry, as under nohup, stays ignored: the seal runs to its end and the run passes (review R12).
  arms=$((arms + 1))
  out="$(cd "$work" && trap '' HUP && SEAL_HOOK=hup PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 0 ] && printf '%s' "$out" | grep -qE '^task-history: [^ ]+ — sealed '; then
    ok=$((ok + 1)); echo "  ✅ a hang-up ignored on entry stays ignored, and the seal runs to its end"
  else
    echo "SELF-TEST: a hang-up ignored on entry — rc $rc, the seal not made:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  rm -rf "$work/docs/task-history/Q"; restore
  # The history folder gone just before a sealed file's creation: the rollback finds no file to remove, and names
  # nothing it could not undo (review R12 AG-d).
  arms=$((arms + 1))
  out="$(cd "$work" && SEAL_HOOK=vanish PYTHONPATH="$hook" bash "$SELF" --seal Q 2>&1)"; rc=$?
  if [ "$rc" -eq 1 ] && ! printf '%s' "$out" | grep -qF "could not undo" && clean && [ ! -e "$work/docs/task-history/Q" ]; then
    ok=$((ok + 1)); echo "  ✅ a noted file never created is no file the rollback fails to remove"
  else
    echo "SELF-TEST: a noted file never created — rc $rc, or the rollback named it:" >&2
    printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2
  fi
  restore
  rm -rf "$hook"
  # A linked index, and a linked history folder, are refused before a write (review R11 AG1, P2).
  mv "$work/docs/task-history/INDEX.md" "$work/docs/task-history/INDEX.real"; ln -s INDEX.real "$work/docs/task-history/INDEX.md"
  arm "a linked index is refused before anything is written" 1 "through a link; nothing was written" --seal Q
  [ -L "$work/docs/task-history/INDEX.md" ] || { arms=$((arms + 1)); echo "SELF-TEST: a linked index was replaced" >&2; }
  rm "$work/docs/task-history/INDEX.md"; mv "$work/docs/task-history/INDEX.real" "$work/docs/task-history/INDEX.md"
  mkdir -p "$SCRATCH/outside"; ln -s "$SCRATCH/outside" "$work/docs/task-history/Q"
  arm "a linked history folder is refused before anything is written" 1 "through a link; nothing was written" --seal Q
  [ -z "$(ls -A "$SCRATCH/outside")" ] || { arms=$((arms + 1)); echo "SELF-TEST: a seal wrote through a linked history folder" >&2; }
  rm "$work/docs/task-history/Q"; rm -rf "$SCRATCH/outside"; restore
  # A link above them: the task trees' folder, and the history's own (review R12 D2, AG-b).
  mv "$work/docs/tasks" "$SCRATCH/tasks.real"; ln -s "$SCRATCH/tasks.real" "$work/docs/tasks"
  arm "a linked task-trees folder is refused before anything is written" 1 "through a link; nothing was written" --seal Q
  rm "$work/docs/tasks"; mv "$SCRATCH/tasks.real" "$work/docs/tasks"
  mv "$work/docs/task-history" "$SCRATCH/history.real"; ln -s "$SCRATCH/history.real" "$work/docs/task-history"
  arm "a linked history root is refused before anything is written" 1 "through a link; nothing was written" --seal Q
  rm "$work/docs/task-history"; mv "$SCRATCH/history.real" "$work/docs/task-history"
  git -C "$work" diff --quiet || { arms=$((arms + 1)); echo "SELF-TEST: a linked folder's arm left the fixture changed" >&2; }
  # Any entry at a sealed file's path, not only a link to nothing, is refused before a write (review R10 D2).
  mkdir "$work/docs/task-history/Q"; printf 'not a seal\n' > "$work/docs/task-history/Q/Q.1.md"
  arm "a file already at a sealed file's path is refused before anything is written" 1 "nothing was written" --seal Q
  rm -rf "$work/docs/task-history/Q"
  clean || { arms=$((arms + 1)); echo "SELF-TEST: a seal refused for a file at its path wrote something" >&2; }
  restore
  # A tree file that is a link: refused before a write, never replaced by a file (review R10 AG3).
  mv "$work/docs/tasks/Q.md" "$work/docs/Q.real"; ln -s ../Q.real "$work/docs/tasks/Q.md"; commit
  arm "a tree file that is a link is refused before anything is written" 1 "through a link; nothing was written" --seal Q
  [ -L "$work/docs/tasks/Q.md" ] && clean || { arms=$((arms + 1)); echo "SELF-TEST: a linked tree was written through or replaced" >&2; }
  git -C "$work" rm -q docs/tasks/Q.md; mv "$work/docs/Q.real" "$work/docs/tasks/Q.md"; commit
  # Each rollback step on its own (review R10 AG1, AG2): mid-proof, the index becomes a folder, so its restore fails,
  # and the tree's must still run; and two sealed files become folders, so neither removal can run, and both are named.
  fake="$SCRATCH/fakebin"; rm -rf "$fake"; mkdir -p "$fake"
  printf '#!/bin/sh\nfor a in "$@"; do [ "$a" = "--is-shallow-repository" ] && { mv docs/task-history/INDEX.md docs/task-history/INDEX.keep; mkdir docs/task-history/INDEX.md; }; done\nexec "%s" "$@"\n' "$(command -v git)" > "$fake/git"
  chmod +x "$fake/git"
  arm_path() { local name="$1" must="$2"; shift 2; arms=$((arms + 1))
    out="$(cd "$work" && PATH="$fake:$PATH" bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne 0 ] && printf '%s' "$out" | grep -qF -- "$must"; then return 0; fi
    echo "SELF-TEST: $name — rc $rc, not about \`$must\`:" >&2; printf '%s\n' "$out" | tail -3 | sed 's/^/    /' >&2; return 1
  }
  if arm_path "a restore that fails" "INDEX.md, not restored" --seal Q && git -C "$work" diff --quiet -- docs/tasks; then
    ok=$((ok + 1)); echo "  ✅ an index restore that fails is named, and the tree is restored all the same"
  else echo "SELF-TEST: a failed index restore stopped the tree's, or was not named" >&2; fi
  rm -rf "$work/docs/task-history/INDEX.md"; mv "$work/docs/task-history/INDEX.keep" "$work/docs/task-history/INDEX.md"
  rm -rf "$work/docs/task-history/Q"; restore
  printf -- '# S\n\n## Task Tree\n\n- ID: `S.1`\n  Status: `done`\n  Goal: one closed subtree\n  Commit: `ARCHOGEN-S-0001`\n\n- ID: `S.2`\n  Status: `done`\n  Goal: another\n  Commit: `ARCHOGEN-S-0002`\n' > "$work/docs/tasks/S.md"
  commit
  printf '#!/bin/sh\nfor a in "$@"; do [ "$a" = "--is-shallow-repository" ] && for f in S.1 S.2; do rm docs/task-history/S/$f.md; mkdir docs/task-history/S/$f.md; : > docs/task-history/S/$f.md/x; done; done\nexec "%s" "$@"\n' "$(command -v git)" > "$fake/git"
  if arm_path "removals that fail" "S/S.1.md, not removed" --seal S && printf '%s' "$out" | grep -qF "S/S.2.md, not removed"; then
    ok=$((ok + 1)); echo "  ✅ every removal the rollback cannot make is named, the first failure stopping none"
  else echo "SELF-TEST: a removal that failed stopped the next, or went unnamed" >&2; fi
  rm -rf "$work/docs/task-history/S" "$fake"; restore
  git -C "$work" rm -q docs/tasks/S.md; commit
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
  --census)
    [ -n "${2:-}" ] || { echo "usage: bash scripts/check_task_history.sh --census <COMMIT>" >&2; exit 2; }
    core census "$2"; exit $? ;;
  "") core gate ""; exit $? ;;
  *) echo "usage: bash scripts/check_task_history.sh [--seal <TREE> | --census <COMMIT> | --self-test]" >&2; exit 2 ;;
esac
