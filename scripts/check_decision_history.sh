#!/usr/bin/env bash
# scripts/check_decision_history.sh — DECISION-HISTORY: settled sections sealed out of decision records (leaf
# `PROGRAM.41`, hardened by `PROGRAM.41.1`; docs/decisions/decision_decisions-folder-ceiling.md,
# docs/reviews/decision-history-reviews.md, LIVE_DOCUMENT_SIZE_CONTAINMENT.md's `archive_terminal`).
#
# ⭐ WHY. `docs/decisions/` has a ceiling that only the director can raise, and it was raised once (`PROGRAM.38`).
# Some records hold numbered sections that are settled and cited by number across the task trees: the findings
# register's resolved and ruled items. Sealing such a section moves it, byte for byte, into
# `docs/decision-history/<record>/<NN>.md`, and leaves its heading and a one-line stub where it stood, so every
# citation by number still finds it. `docs/decision-history/INDEX.md` records each file's lines, bytes and sha256.
# It is `TASK-HISTORY`'s pattern (`docs/decisions/decision_task-tree-sealing.md`), for sections instead of leaves.
#
# MODES:
#   bash scripts/check_decision_history.sh                          # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_decision_history.sh --seal <RECORD> <N>...   # seal sections N of a record, prove, check
#   bash scripts/check_decision_history.sh --self-test
#
# A SECTION is a `## ` heading at column 0, outside a code fence, and every line up to the next such heading. Fences
# follow CommonMark: an opener of three or more backticks or tildes, indented at most three spaces, closed only by a
# run of the same character at least as long. A section is numbered when its heading reads `## <N>. `. Its BODY is
# that span without its trailing blank lines, which stay in the record after the stub. A STUB is exactly three lines:
# the heading, a blank line, and, outside any fence,
#   Sealed, byte for byte, in [`<record>/<NN>.md`](<link from the record's folder>) — settled, and never edited again.
#
# THE SEAL refuses, writing nothing: a record with a carriage return or no final newline; a section number that is
# not a number, or given twice; a section with another `## ` line in it, fenced or not, or one that ends inside a fence.
# It writes nothing unless the record it would leave, with every new stub replaced by its body from its new sealed
# file, is the record as it stood, byte for byte; and it rolls everything back if the gate then refuses, for any reason.
#
# THE GATE'S LEGS:
#   0. the repository is not shallow, and every history read succeeds — a shallow clone would hide the commits legs 3
#      and 5 read, and CI's checkout sets `fetch-depth: 0` for that reason;
#   1. the index begins with its header, byte for byte; every sealed file's lines, bytes and sha256 are its row's, it
#      opens with its section's heading and holds no other `## ` line; no row is listed twice;
#   2. sealed files and rows correspond one to one, and nothing else tracked or untracked-and-not-ignored is under
#      docs/decision-history/;
#   3. HISTORY-WIDE, with `--full-history` so a merge cannot hide a seal: every row any committed version of the index
#      held is still there, unchanged, and every sealed file is byte for byte what the commit that added it wrote;
#   4. every sealed file has exactly one stub, in its own record, under its own heading, linking it; every stub line
#      in any record, outside a fence, is in such a place — none in a record's preamble, none under an unnumbered
#      heading;
#   5. PROVENANCE: every sealed file is, byte for byte, the section its record held just before the commit that
#      sealed it (HEAD, for a seal not yet committed), and its row's date lies between that commit's parent and the
#      sealing commit, in UTC.
#
# ⚠️ HONEST LIMITS:
#   - history rewritten under the gate (a force-push, a replaced object) is premises 2 and 3's, as for the catalog
#     (docs/specs/catalog/decision_catalog-records.md §0). Within history, legs 3 and 5 hold whatever HEAD is;
#   - a record is found by its file name. Moving it into another folder of docs/decisions/ is allowed, with every
#     stub's link rewritten for its new folder; renaming it is refused for good, since its sealed files keep the old
#     name. A stand-in record of the same name, moved in for one commit, could supply a section its real path never
#     held; that equals an edit made live and then sealed, and only review of the commits before a seal sees it;
#   - which sections are settled is the sealing leaf's judgement, recorded in its tree. Nothing here checks it, so
#     sealing is a route around the folder's ceiling that only review guards;
#   - a breach committed past the hook stays red for good, since the index and the files are append-only: repair it
#     by rewriting the commits that are not yet pushed, never `main`'s, or in a leaf that owns the exception.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/decision_history"
mkdir -p "$SCRATCH"

core() { # $1 = gate | seal, then the record and the section numbers (seal only)
  GIT_NO_REPLACE_OBJECTS=1 python3 - "$@" <<'PY'
import datetime, hashlib, os, re, subprocess, sys

mode, args = sys.argv[1], sys.argv[2:]
DECISIONS, HIST = "docs/decisions", "docs/decision-history"
INDEX = os.path.join(HIST, "INDEX.md")
NUM_RE = re.compile(r"^## (\d+)\. ")
ANY_H2 = re.compile(r"^ {0,3}## ")
FENCE_RE = re.compile(r"^( {0,3})(`{3,}|~{3,})(.*)$")
CLOSE_RE = re.compile(r"^ {0,3}(`{3,}|~{3,})\s*$")
STUB_RE = re.compile(r"^Sealed, byte for byte, in \[`([^`]+)`\]\(([^)]+)\) — settled, and never edited again\.$")
ROW_RE = re.compile(r"^\| `([A-Za-z0-9_.+-]+)` \| `(\d{2,})` \| (\d+) \| (\d+) \| `([0-9a-f]{64})` \| `(\d{4}-\d{2}-\d{2})` \|$")
FAULT = os.environ.get("DECISION_HISTORY_SELFTEST_FAULT", "")  # the self-test's one seam: it can only make a seal fail
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
def git(*a):
    """stdout of a read-only git command, or None when it fails."""
    if a not in _git:
        r = subprocess.run(("git",) + a, capture_output=True)
        _git[a] = r.stdout if r.returncode == 0 else None
    return _git[a]

def history(*a):
    """A history read that must succeed: a failure is a breach, never an empty history."""
    out = git(*a)
    if out is None:
        note("`git %s` failed, so the history legs cannot hold — the gate never reads a failure as no history" % " ".join(a))
        return ""
    return out.decode()

def at(commit, path):
    data = git("show", "%s:%s" % (commit, path))
    return None if data is None else decode(data, "%s:%s" % (commit, path))

def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def key(n):
    return "%02d" % int(n)

def utc_day(commit):
    out = git("log", "-1", "--format=%ct", commit)
    return None if out is None else datetime.datetime.fromtimestamp(int(out.decode().strip()), datetime.timezone.utc).strftime("%Y-%m-%d")

def scan(text):
    """(lines, fenced, sections, open_at_end): fenced[i] is True for a line inside a fence, its opener and its closer
    included; a section is (number or None, first, end) for each column-0 `## ` heading outside a fence."""
    lines = text.split("\n")
    fenced, secs, cur, fence = [], [], None, None
    for i, line in enumerate(lines):
        if fence is None:
            m = FENCE_RE.match(line)
            if m and not (m.group(2)[0] == "`" and "`" in m.group(3)):
                fence = (m.group(2)[0], len(m.group(2)))
                fenced.append(True)
                continue
            fenced.append(False)
            if line.startswith("## "):
                if cur:
                    secs.append((cur[0], cur[1], i))
                m = NUM_RE.match(line)
                cur = (key(m.group(1)) if m else None, i)
        else:
            fenced.append(True)
            m = CLOSE_RE.match(line)
            if m and m.group(1)[0] == fence[0] and len(m.group(1)) >= fence[1]:
                fence = None
    if cur:
        secs.append((cur[0], cur[1], len(lines)))
    return lines, fenced, secs, fence is not None

def body_end(lines, first, end):
    e = end
    while e > first + 1 and lines[e - 1].strip() == "":
        e -= 1
    return e

def is_stub(lines, fenced, first, end):
    return (body_end(lines, first, end) == first + 3 and lines[first + 1] == ""
            and not fenced[first + 2] and STUB_RE.match(lines[first + 2]) is not None)

def stub_line(record_path, stem, n):
    target = os.path.join(HIST, stem, n + ".md")
    rel = os.path.relpath(target, os.path.dirname(record_path))
    return "Sealed, byte for byte, in [`%s/%s.md`](%s) — settled, and never edited again." % (stem, n, rel)

def records_now():
    """[(stem, path)] for every Markdown file under docs/decisions on disk, in path order."""
    out = []
    for d, _, names in os.walk(DECISIONS):
        for name in names:
            if name.endswith(".md"):
                out.append((name[:-3], os.path.join(d, name)))
    return sorted(out, key=lambda x: x[1])

def record_at(commit, stem):
    names = history("ls-tree", "-r", "--name-only", commit, "--", DECISIONS).split("\n")
    hits = [p for p in names if os.path.basename(p) == stem + ".md"]
    return hits[0] if len(hits) == 1 else None

def rows(index_text, where):
    out = []
    for n, line in enumerate(index_text.split("\n"), 1):
        if line.startswith("| `"):
            r = ROW_RE.match(line)
            if not r:
                note("%s:%d is not a row of the form `| `RECORD` | `NN` | lines | bytes | `sha256` | `date` |`" % (where, n)); continue
            out.append(r.groups())
    return out

HEADER = """# docs/decision-history/INDEX.md — settled sections sealed out of decision records

Each file below holds one numbered section of a decision record, moved here byte for byte by
`bash scripts/check_decision_history.sh --seal <RECORD> <N>...` once it was settled, and never edited again
(`PROGRAM.41`, `docs/decisions/decision_decisions-folder-ceiling.md`). Its heading stayed in the record, above a
one-line stub that links here, so a citation of the section by number still finds it. A row records the file's lines,
bytes and sha256, and the day it was sealed. The rows are append-only, and `DECISION-HISTORY` checks every file
against its row, its stub and the record it came from, on every commit.

To prove a file, compare `sha256sum docs/decision-history/<RECORD>/<NN>.md` with its row.

| Record | Section | Lines | Bytes | sha256 | Sealed |
| --- | --- | --- | --- | --- | --- |
"""

def gate():
    """The legs; returns (sealed files, stubs)."""
    # Leg 0.
    shallow = git("rev-parse", "--is-shallow-repository")
    if shallow is None or shallow.decode().strip() != "false":
        note("the repository is shallow, or git cannot say, so the commits legs 3 and 5 read may be missing — fetch the full history")
    # Leg 1.
    index = []
    if os.path.exists(INDEX):
        itext = read(INDEX)
        if not itext.startswith(HEADER):
            note("%s does not begin with its header, byte for byte — the header states what the rows promise" % INDEX)
        index = rows(itext, INDEX)
    files, dates = {}, {}  # path -> (stem, n); path -> row date
    for stem, n, nlines, nbytes, digest, day in index:
        fpath = os.path.join(HIST, stem, n + ".md")
        if fpath in files:
            note("%s lists %s section %s twice" % (INDEX, stem, n)); continue
        if not os.path.exists(fpath):
            note("%s lists %s section %s, and %s does not exist" % (INDEX, stem, n, fpath)); continue
        files[fpath], dates[fpath] = (stem, n), day
        data = read(fpath)
        if str(data.count("\n")) != nlines: note("%s has %d lines, and its row says %s — a sealed file changed" % (fpath, data.count("\n"), nlines))
        if str(len(data.encode("utf-8"))) != nbytes: note("%s has %d bytes, and its row says %s — a sealed file changed" % (fpath, len(data.encode("utf-8")), nbytes))
        if sha(data) != digest: note("%s's sha256 is not its row's — a sealed file changed" % fpath)
        m = NUM_RE.match(data)
        if not m or key(m.group(1)) != n: note("%s does not open with the heading of section %s" % (fpath, n))
        if any(ANY_H2.match(l) for l in data.split("\n")[1:]):
            note("%s holds a `## ` line after its heading, so it is more than one section" % fpath)
    # Leg 2: one to one, nothing else that git sees.
    listed = history("ls-files", "-co", "--exclude-standard", "--", HIST).split("\n")
    for p in sorted(x for x in listed if x):
        if p == INDEX or p in files:
            continue
        if len(p.split("/")) == 4:
            note("%s is in no row of %s — a sealed file must be listed" % (p, INDEX))
        else:
            note("%s is neither the index nor a sealed file in a record's folder" % p)
    # Leg 3, history-wide, with --full-history so a merge that drops the seal's side cannot hide it.
    current = set(index)
    for commit in history("log", "--full-history", "--format=%H", "--", INDEX).split():
        old = at(commit, INDEX)
        if old is None:
            continue
        for r in rows(old, "%s:%s" % (commit[:12], INDEX)):
            if r not in current:
                note("%s's row for %s section %s, committed in %s, is gone or changed — the index is append-only" % (INDEX, r[0], r[1], commit[:12]))
    added, commit = {}, None
    for line in history("log", "--full-history", "--diff-filter=A", "--format=@%H", "--name-only", "--", HIST).split("\n"):
        if line.startswith("@"):
            commit = line[1:]
        elif line.strip():
            added[line.strip()] = commit  # newest first, so the one kept is the oldest
    for fpath in files:
        c = added.get(fpath)
        if c and at(c, fpath) != read(fpath):
            note("%s is not what %s wrote when it sealed it — a sealed file changed" % (fpath, c[:12]))
    # Leg 4: stubs against files, in every record, preamble included.
    recs = records_now()
    for stem in sorted({s for s, _ in files.values()}):
        paths = [p for s, p in recs if s == stem]
        if len(paths) > 1:
            note("%s share the file name %s.md, so its sealed sections cannot tell their record" % (" and ".join(paths), stem))
    stubs = {}
    for stem, rpath in recs:
        rlines, rfenced, rsecs, _ = scan(read(rpath))
        first_sec = rsecs[0][1] if rsecs else len(rlines)
        for i in range(first_sec):
            if not rfenced[i] and STUB_RE.match(rlines[i]):
                note("%s:%d holds a stub outside any section" % (rpath, i + 1))
        for n, first, end in rsecs:
            marked = any(not rfenced[i] and STUB_RE.match(rlines[i]) for i in range(first, end))
            label = "section %s" % n if n else "the section headed %r" % rlines[first][:40]
            if not marked:
                if n and (stem, n) in files.values():
                    note("%s: %s is sealed in %s/%s/%s.md and also live here" % (rpath, label, HIST, stem, n))
                continue
            if n is None:
                note("%s: a stub under a heading with no number, %r — only a numbered section is sealed" % (rpath, rlines[first][:40])); continue
            if not is_stub(rlines, rfenced, first, end):
                note("%s: the stub of %s is not exactly its heading, a blank line and the stub's line" % (rpath, label)); continue
            fpath = os.path.join(HIST, stem, n + ".md")
            m = STUB_RE.match(rlines[first + 2])
            if m.group(1) != "%s/%s.md" % (stem, n) or rlines[first + 2] != stub_line(rpath, stem, n):
                note("%s: the stub of %s does not link %s by its own path from the record's folder" % (rpath, label, fpath))
            if fpath not in files:
                note("%s: %s has a stub, and no sealed file is listed for it" % (rpath, label)); continue
            if (stem, n) in stubs:
                note("%s: %s has two stubs" % (rpath, label))
            stubs[(stem, n)] = rpath
            if read(fpath).split("\n")[0] != rlines[first]:
                note("%s: the heading above the stub of %s is not the sealed file's" % (rpath, label))
    for fpath, sk in files.items():
        if sk not in stubs:
            note("%s is sealed and has no stub in its record" % fpath)
    # Leg 5, provenance and the row's date.
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    for fpath, (stem, n) in files.items():
        c = added.get(fpath)
        base = (c + "^") if c else "HEAD"
        rpath = record_at(base, stem)
        before = at(base, rpath) if rpath else None
        if before is None:
            note("%s: %s held no single record %s.md under %s to be sealed from" % (fpath, base, stem, DECISIONS)); continue
        blines, bfenced, bsecs, _ = scan(before)
        found = [(f, e) for (x, f, e) in bsecs if x == n]
        if len(found) != 1:
            note("%s: %s's %s did not hold exactly one section %s" % (fpath, base, rpath, n)); continue
        f, e = found[0]
        if is_stub(blines, bfenced, f, e):
            note("%s: section %s was already a stub in %s's %s" % (fpath, n, base, rpath)); continue
        if "\n".join(blines[f:body_end(blines, f, e)]) + "\n" != read(fpath):
            note("%s is not the section %s's %s held — a section was edited on its way in" % (fpath, base, rpath))
        low, high = utc_day(base), (utc_day(c) if c else today)
        if low is None or high is None or not (low <= dates[fpath] <= high):
            note("%s's row is dated %s, outside the days its seal could have been made (%s to %s, UTC)" % (fpath, dates[fpath], low, high))
    return len(files), len(stubs)

def seal(record, numbers):
    record = os.path.normpath(record)
    if not os.path.isfile(record) or not record.startswith(DECISIONS + os.sep) or not record.endswith(".md"):
        sys.exit("decision-history: %s is not a record under %s; nothing was written" % (record, DECISIONS))
    for x in numbers:
        if not re.fullmatch(r"\d+", x):
            sys.exit("decision-history: %r is not a section number; nothing was written" % x)
    want = [key(n) for n in numbers]
    if len(set(want)) != len(want):
        sys.exit("decision-history: a section number is given twice; nothing was written")
    stem = os.path.basename(record)[:-3]
    before = read(record)
    if "\r" in before or not before.endswith("\n"):
        sys.exit("decision-history: %s holds a carriage return or lacks a final newline, and a stub would change its line"
                 " endings; normalise it in a change of its own first; nothing was written" % record)
    lines, fenced, secs, _ = scan(before)
    by = {}
    for n, f, e in secs:
        if n is not None:
            by.setdefault(n, []).append((f, e))
    for n in want:
        if len(by.get(n, [])) != 1:
            sys.exit("decision-history: %s has no single section %s; nothing was written" % (record, n))
        f, e = by[n][0]
        if is_stub(lines, fenced, f, e):
            sys.exit("decision-history: %s's section %s is already sealed; nothing was written" % (record, n))
        if os.path.exists(os.path.join(HIST, stem, n + ".md")):
            sys.exit("decision-history: %s/%s/%s.md exists; nothing was written" % (HIST, stem, n))
        be = body_end(lines, f, e)
        inner = [k + 1 for k in range(f + 1, be) if ANY_H2.match(lines[k])]
        if inner:
            sys.exit("decision-history: %s's section %s holds another `## ` line (line %s), fenced or not; a sealed file must"
                     " be one section; nothing was written" % (record, n, ", ".join(map(str, inner))))
        if fenced[be - 1] and not CLOSE_RE.match(lines[be - 1]):
            sys.exit("decision-history: %s's section %s ends inside a code fence; nothing was written" % (record, n))
    out, i, files = [], 0, {}
    for n in sorted(want, key=lambda n: by[n][0][0]):
        f, e = by[n][0]
        be = body_end(lines, f, e)
        out.extend(lines[i:f])
        files[n] = "\n".join(lines[f:be]) + "\n"
        out.extend([lines[f], "", stub_line(record, stem, n)])
        out.extend(lines[be:e])
        i = e
    out.extend(lines[i:])
    if FAULT == "drop-line":
        out.pop(len(out) // 2)
    after = "\n".join(out)
    # The proof: every new stub replaced by its body reconstructs the record as it stood.
    alines, afenced, asecs, _ = scan(after)
    rebuilt, i = [], 0
    for n, f, e in asecs:
        if n in files and is_stub(alines, afenced, f, e):
            rebuilt.extend(alines[i:f])
            rebuilt.extend(files[n][:-1].split("\n"))
            i = f + 3
    rebuilt.extend(alines[i:])
    if "\n".join(rebuilt) != before:
        sys.exit("decision-history: the seal of %s would not reconstruct it byte for byte; nothing was written" % record)
    index_before = read(INDEX) if os.path.exists(INDEX) else None
    made_dir = not os.path.isdir(os.path.join(HIST, stem))
    os.makedirs(os.path.join(HIST, stem), exist_ok=True)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    written = []
    try:
        for n in sorted(files):
            p = os.path.join(HIST, stem, n + ".md")
            with open(p, "w", encoding="utf-8") as fh:
                fh.write(files[n])
            written.append(p)
        with open(record, "w", encoding="utf-8") as fh:
            fh.write(after)
        text = index_before if index_before is not None else HEADER
        text = text.rstrip("\n") + "\n" + "".join("| `%s` | `%s` | %d | %d | `%s` | `%s` |\n" % (
            stem, n, files[n].count("\n"), len(files[n].encode("utf-8")), sha(files[n]), today) for n in sorted(files))
        with open(INDEX, "w", encoding="utf-8") as fh:
            fh.write(text)
        gate()
    except Unreadable as e:
        note(str(e))
    if fails:
        for p in written:
            os.remove(p)
        if made_dir and os.path.isdir(os.path.join(HIST, stem)) and not os.listdir(os.path.join(HIST, stem)):
            os.rmdir(os.path.join(HIST, stem))
        with open(record, "w", encoding="utf-8") as fh:
            fh.write(before)
        if index_before is None:
            if os.path.exists(INDEX):
                os.remove(INDEX)
        else:
            with open(INDEX, "w", encoding="utf-8") as fh:
                fh.write(index_before)
        sys.exit("decision-history: the gate refused the seal of %s, so it was rolled back:\n  " % record + "\n  ".join(fails))
    print("decision-history: %s — sealed section(s) %s, %d bytes; the reconstruction is byte for byte"
          % (record, " ".join(sorted(files)), sum(len(v.encode("utf-8")) for v in files.values())))

try:
    if mode == "seal":
        seal(args[0], args[1:])
        fails.clear()
    nfiles, nstubs = gate()
except Unreadable as e:
    note(str(e))
    nfiles, nstubs = 0, 0
for msg in fails:
    print("DECISION-HISTORY: " + msg, file=sys.stderr)
if fails:
    print("DECISION-HISTORY: %d breach(es) — PROGRAM.41, docs/decisions/decision_decisions-folder-ceiling.md" % len(fails), file=sys.stderr)
    sys.exit(1)
print("decision-history: OK (%d sealed section(s), each against its row and its sealing commit; the index append-only across history; %d stub(s), each linking its file under its own heading; every sealed section proven against its record before its seal)" % (nfiles, nstubs))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest" main
  record() { # the fixture record, at docs/decisions/part/decision_r.md
    cat > "$work/docs/decisions/part/decision_r.md" <<'MD'
# A register

Two items.

## 1. An open item

Still open.

## 2. A settled item — **RESOLVED**

It was settled, with a `code span` and
- a bullet.

```text
### a smaller heading inside a fence
```

## 3. Another open item

Open.
MD
  }
  fresh() {
    rm -rf "$work" "$work-shallow"; mkdir -p "$work/docs/decisions/part"; git -C "$work" init -q
    record
    printf '# Index\n' > "$work/docs/decisions/INDEX.md"
    printf '.DS_Store\n' > "$work/.gitignore"
    git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm base
    main="$(git -C "$work" symbolic-ref --short HEAD)"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the arguments
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  unchanged() { # $1 = what; the arm before it must have left the tree as committed
    arms=$((arms + 1))
    if [ -z "$(git -C "$work" status --porcelain)" ]; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — the tree changed:" >&2; git -C "$work" status --porcelain | sed 's/^/    /' >&2; fi
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
  handseal() { # $1 = record path, $2 = section, $3 = the body to seal ("" for the section itself), $4 = the row's date
    python3 - "$work" "$1" "$2" "$3" "$4" <<'PY'
import hashlib, os, re, sys
w, rec, n, body, day = sys.argv[1:6]
r = os.path.join(w, rec)
t = open(r).read()
m = re.search(r"^## %d\. .*$" % int(n), t, re.M)
head, start = m.group(0), m.start()
nxt = re.search(r"^## ", t[m.end():], re.M)
end = m.end() + nxt.start() if nxt else len(t)
section = t[start:end].rstrip("\n")
stem, nn = os.path.basename(rec)[:-3], "%02d" % int(n)
rel = os.path.relpath("docs/decision-history/%s/%s.md" % (stem, nn), os.path.dirname(rec))
stub = "%s\n\nSealed, byte for byte, in [`%s/%s.md`](%s) — settled, and never edited again." % (head, stem, nn, rel)
open(r, "w").write(t[:start] + stub + t[start + len(section):])
body = body if body else section + "\n"
os.makedirs(os.path.join(w, "docs/decision-history", stem), exist_ok=True)
open(os.path.join(w, "docs/decision-history", stem, nn + ".md"), "w").write(body)
i = os.path.join(w, "docs/decision-history/INDEX.md")
open(i, "a").write("| `%s` | `%s` | %d | %d | `%s` | `%s` |\n" % (stem, nn, body.count("\n"), len(body.encode()), hashlib.sha256(body.encode()).hexdigest(), day))
PY
  }
  reseal_row() { # re-derive section 02's row from its file, as a forger would
    python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
s = open(w + "/docs/decision-history/decision_r/02.md").read()
i = w + "/docs/decision-history/INDEX.md"
t = re.sub(r"^\| `decision_r` \| `02` \| \d+ \| \d+ \| `[0-9a-f]+` \|", "| `decision_r` | `02` | %d | %d | `%s` |" % (s.count("\n"), len(s.encode()), hashlib.sha256(s.encode()).hexdigest()), open(i).read(), flags=re.M)
open(i, "w").write(t)
PY
  }
  local R=docs/decisions/part/decision_r.md TODAY out
  TODAY="$(date -u +%Y-%m-%d)"

  fresh
  arm "a record with nothing sealed passes" 0 "0 sealed section(s)"
  arm "sealing a section the record does not have writes nothing" 1 "has no single section 07; nothing was written" --seal "$R" 7
  unchanged "and the refused seal wrote nothing"
  arm "a section number that is not a number is refused" 1 "is not a section number" --seal "$R" Why
  arm "a section named twice is refused" 1 "given twice" --seal "$R" 2 2
  printf '# Outside\n\n## 2. Settled\n\nDone.\n' > "$work/docs/x.md"
  arm "a record outside the decisions folder is refused, even by a path through it" 1 "is not a record under" --seal docs/decisions/../x.md 2
  rm -f "$work/docs/x.md"
  DECISION_HISTORY_SELFTEST_FAULT=drop-line arm "a seal that would not reconstruct its record writes nothing" 1 "would not reconstruct it byte for byte" --seal "$R" 2
  unchanged "and the unproven seal wrote nothing"
  cp "$work/$R" "$work/docs/decisions/decision_r.md"
  arm "a seal the gate refuses is rolled back" 1 "so it was rolled back" --seal "$R" 2
  rm -f "$work/docs/decisions/decision_r.md"
  unchanged "and the rolled-back seal left nothing behind"
  mkdir -p "$work/docs/decisions/bad" && printf '# Bad\n\n\xff\xfe\n' > "$work/docs/decisions/bad/decision_b.md"
  arm "a seal whose gate meets an unreadable record is rolled back" 1 "so it was rolled back" --seal "$R" 2
  rm -rf "$work/docs/decisions/bad"
  unchanged "and the seal rolled back on an unreadable record left nothing behind"
  arm "the seal proves its reconstruction" 0 "the reconstruction is byte for byte" --seal "$R" 2
  arm "the sealed record passes the gate" 0 "1 sealed section(s)"
  grep -qx 'Sealed, byte for byte, in \[`decision_r/02.md`\](../../decision-history/decision_r/02.md) — settled, and never edited again.' "$work/$R" ||
    { arms=$((arms + 1)); echo "SELF-TEST: the stub does not link its file from the record's own folder" >&2; }
  arm "sealing it again is refused" 1 "is already sealed; nothing was written" --seal "$R" 2
  commit
  printf 'x' >> "$work/docs/decision-history/decision_r/02.md"
  arm "a byte added to a sealed file is refused against its row" 1 "and its row says"
  restore
  sub docs/decision-history/INDEX.md '^\| `decision_r` \| `02` \|.*\n' ''
  arm "a sealed file with no row is refused" 1 "is in no row of"
  restore
  sub docs/decision-history/INDEX.md '^(\| `decision_r` \| `02` \|.*\n)' '\1\1'
  arm "a row listed twice is refused" 1 "section 02 twice"
  restore
  printf '| `decision_r` | `05` | 1 | 1 | `%064d` | `%s` |\n' 0 "$TODAY" >> "$work/docs/decision-history/INDEX.md"
  arm "a row whose file does not exist is refused" 1 "does not exist"
  restore
  sub docs/decision-history/INDEX.md '^Each file below' 'Every file below may be edited; each file below'
  arm "an index whose header was rewritten is refused" 1 "does not begin with its header"
  restore
  sub docs/decision-history/decision_r/02.md '^## 2\. A settled item — \*\*RESOLVED\*\*$' '## 5. A settled item — **RESOLVED**'
  reseal_row
  arm "a sealed file that does not open with its own heading is refused" 1 "does not open with the heading of section 02"
  restore
  printf '## 9. appended\n' >> "$work/docs/decision-history/decision_r/02.md"
  reseal_row
  arm "a sealed file holding a second heading is refused" 1 "holds a \`## \` line after its heading"
  restore
  sub docs/decision-history/INDEX.md '^(\| `decision_r` \| `02` \|.*\| `)[0-9-]+(` \|)$' '\g<1>1999-01-01\2'
  commit
  arm "a committed row that changed is refused across history" 1 "the index is append-only"
  git -C "$work" reset -q --hard HEAD~1
  sub docs/decision-history/decision_r/02.md 'It was settled' 'It was never settled'
  reseal_row
  commit
  arm "a sealed file and its row forged together and committed are refused" 1 "is not what"
  git -C "$work" reset -q --hard HEAD~1
  git -C "$work" checkout -q -b other HEAD~1
  printf 'unrelated\n' > "$work/notes.md"; commit
  git -C "$work" -c user.name=t -c user.email=t@t merge -q -s ours -m "drop the seal" "$main"
  arm "a merge that keeps only the other side cannot hide a seal" 1 "the index is append-only"
  git -C "$work" checkout -q "$main"; git -C "$work" branch -q -D other
  git clone -q --depth 1 "file://$work" "$work-shallow" 2>/dev/null
  arms=$((arms + 1))
  if out="$(cd "$work-shallow" && bash "$SELF" 2>&1)"; then echo "SELF-TEST: a shallow clone passed" >&2
  elif printf '%s' "$out" | grep -qF "the repository is shallow"; then ok=$((ok + 1)); echo "  ✅ a shallow clone is refused"
  else echo "SELF-TEST: a shallow clone was refused for another reason:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; fi
  rm -rf "$work-shallow"
  mkdir -p "$work/docs/decisions/other" && printf '# Another\n\n## 1. Else\n\nText.\n' > "$work/docs/decisions/other/decision_r.md"
  arm "a second record under a sealed record's file name is refused" 1 "share the file name decision_r.md"
  restore
  printf 'noise\n' > "$work/docs/decision-history/.DS_Store"
  arm "an ignored file in the history folder is not seen" 0 "1 sealed section(s)"
  printf 'stray\n' > "$work/docs/decision-history/notes.txt"
  arm "a stray file in the history folder is refused" 1 "neither the index nor a sealed file"
  restore
  sub "$R" '^Sealed, byte for byte, in .*\n' 'Sealed, byte for byte, in [`decision_r/02.md`](../../decision-history/decision_r/02.md) — settled, and never edited again.\nA line added to the stub.\n'
  arm "a stub holding more than its three lines is refused" 1 "is not exactly its heading"
  restore
  sub "$R" '^\n^Sealed, byte for byte, in .*\n' '\nIt was settled, and is live again.\n'
  arm "a sealed section made live again is refused" 1 "also live here"
  restore
  sub "$R" '^## 2\. A settled item — \*\*RESOLVED\*\*$' '## 2. A settled item, retitled'
  arm "a heading changed above its stub is refused" 1 "is not the sealed file's"
  restore
  sub "$R" '\(\.\./\.\./decision-history/decision_r/02\.md\)' '(../decision-history/decision_r/02.md)'
  arm "a stub whose link does not resolve from its record is refused" 1 "does not link"
  restore
  printf '\n## 2. A settled item — **RESOLVED**\n\nSealed, byte for byte, in [`decision_r/02.md`](../../decision-history/decision_r/02.md) — settled, and never edited again.\n' >> "$work/$R"
  arm "two stubs for one sealed section are refused" 1 "has two stubs"
  restore
  printf '\n## 4. Forged\n\nSealed, byte for byte, in [`decision_r/04.md`](../../decision-history/decision_r/04.md) — settled, and never edited again.\n' >> "$work/$R"
  arm "a stub with no sealed file is refused" 1 "no sealed file is listed for it"
  restore
  sub "$R" '^Two items\.$' 'Sealed, byte for byte, in [`decision_r/09.md`](../../decision-history/decision_r/09.md) — settled, and never edited again.'
  arm "a stub in a record's preamble is refused" 1 "holds a stub outside any section"
  restore
  printf '\n## Why\n\nSealed, byte for byte, in [`decision_r/None.md`](../../decision-history/decision_r/None.md) — settled, and never edited again.\n' >> "$work/$R"
  arm "a stub under a heading with no number is refused" 1 "a stub under a heading with no number"
  restore
  sub "$R" '^Still open\.$' 'Still open. The stub reads:\n\n```text\nSealed, byte for byte, in [`decision_r/01.md`](../../decision-history/decision_r/01.md) — settled, and never edited again.\n```'
  arm "a live section quoting the stub's words in a fence passes" 0 "1 sealed section(s)"
  restore
  git -C "$work" rm -q "$R"
  arm "a sealed section whose record was deleted is refused" 1 "has no stub in its record"
  git -C "$work" reset -q --hard HEAD
  git -C "$work" mv "$R" docs/decisions/decision_r.md
  sub docs/decisions/decision_r.md '\(\.\./\.\./decision-history/' '(../decision-history/'
  arm "a record moved to another folder, its stub's link following it, still passes" 0 "1 sealed section(s)"
  git -C "$work" reset -q --hard HEAD
  handseal "$R" 3 '## 3. Another open item

Closed, it says now.
' "$TODAY"
  arm "a section edited on its way in, by a seal made by hand, is refused" 1 "a section was edited on its way in"
  restore
  handseal "$R" 3 "" "2001-01-01"
  arm "a faithful seal made by hand, dated before its record existed, is refused" 1 "outside the days its seal could have been made"
  restore
  mkdir -p "$work/docs/decisions/new" && printf '# New\n\n## 1. Settled\n\nDone.\n' > "$work/docs/decisions/new/decision_n.md"
  arm "a seal of a record not yet committed is refused and rolled back" 1 "held no single record" --seal docs/decisions/new/decision_n.md 1
  rm -rf "$work/docs/decisions/new"
  unchanged "and the refused seal left nothing behind"
  printf '\n## 3. A second section 3\n\nAlso open.\n' >> "$work/$R"; commit
  python3 - "$work/$R" <<'PY'
import sys
p = sys.argv[1]; t = open(p).read()
t = t.replace("## 3. Another open item\n\nOpen.", "## 3. Another open item\n\nSealed, byte for byte, in [`decision_r/03.md`](../../decision-history/decision_r/03.md) — settled, and never edited again.")
open(p, "w").write(t)
PY
  printf '## 3. Another open item\n\nOpen.\n' > "$work/docs/decision-history/decision_r/03.md"
  printf '| `decision_r` | `03` | 3 | %d | `%s` | `%s` |\n' "$(wc -c < "$work/docs/decision-history/decision_r/03.md" | tr -d ' ')" \
    "$(shasum -a 256 "$work/docs/decision-history/decision_r/03.md" | cut -d' ' -f1)" "$TODAY" >> "$work/docs/decision-history/INDEX.md"
  arm "a seal made by hand where the record held its number twice is refused" 1 "did not hold exactly one section 03"
  restore; git -C "$work" reset -q --hard HEAD~1
  cp "$work/$R" "$work/docs/decisions/decision_r.md"; commit
  git -C "$work" rm -q docs/decisions/decision_r.md
  handseal "$R" 3 "" "$TODAY"
  arm "a seal made by hand where the record's name was held twice is refused" 1 "held no single record"
  restore; git -C "$work" reset -q --hard HEAD~1
  handseal "$R" 3 "" "$TODAY"
  mv "$work/docs/decision-history/decision_r/03.md" "$work/03.keep"
  cp "$work/docs/decision-history/INDEX.md" "$work/INDEX.keep"
  git -C "$work" checkout -q -- docs/decision-history/INDEX.md
  git -C "$work" add "$R"; git -C "$work" -c user.name=t -c user.email=t@t commit -qm "a stub alone, past the gate"
  mv "$work/03.keep" "$work/docs/decision-history/decision_r/03.md"; mv "$work/INDEX.keep" "$work/docs/decision-history/INDEX.md"
  arm "a sealed file added under a stub committed earlier is refused" 1 "was already a stub"
  restore; git -C "$work" reset -q --hard HEAD~1
  # Fences, on a record of their own.
  mkdir -p "$work/docs/decisions/f"
  printf '# Fenced\n\n## 1. Quoting a fence\n\n````markdown\n```\n````\n\n## 2. Live\n\nStill live.\n\n## 3. A tilde fence\n\n~~~text\n## 7. a log line\n~~~\n\n## 4. Open fence\n\n```text\nnever closed\n' > "$work/docs/decisions/f/decision_f.md"
  commit
  arm "a longer fence holding a shorter one seals as one section" 0 "sealed section(s) 01" --seal docs/decisions/f/decision_f.md 1
  { grep -qx '## 2. Live' "$work/docs/decisions/f/decision_f.md" && ! grep -q '^## 2' "$work/docs/decision-history/decision_f/01.md"; } ||
    { arms=$((arms + 1)); echo "SELF-TEST: the fence sealed a live section with its neighbour" >&2; }
  restore
  arm "a section holding a fenced \`## \` line is refused" 1 "holds another \`## \` line" --seal docs/decisions/f/decision_f.md 3
  arm "a section that ends inside a fence is refused" 1 "ends inside a code fence" --seal docs/decisions/f/decision_f.md 4
  printf 'Windows\r\n' >> "$work/docs/decisions/f/decision_f.md"
  arm "a record with a carriage return is refused" 1 "carriage return" --seal docs/decisions/f/decision_f.md 1
  restore
  printf 'no final newline' >> "$work/docs/decisions/f/decision_f.md"
  arm "a record with no final newline is refused" 1 "lacks a final newline" --seal docs/decisions/f/decision_f.md 1
  restore
  arm "and the sealed record passes again" 0 ""
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real decision history passes"
  else echo "SELF-TEST: the real decision history is refused — run the check to see why" >&2; fi
  echo "decision-history self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --seal)
    [ -n "${2:-}" ] && [ -n "${3:-}" ] || { echo "usage: bash scripts/check_decision_history.sh --seal <RECORD> <N>..." >&2; exit 2; }
    shift; core seal "$@"; exit $? ;;
  "") core gate; exit $? ;;
  *) echo "usage: bash scripts/check_decision_history.sh [--seal <RECORD> <N>... | --self-test]" >&2; exit 2 ;;
esac
