#!/usr/bin/env bash
# scripts/check_decision_history.sh — DECISION-HISTORY: settled sections sealed out of decision records (leaf
# `PROGRAM.41`, docs/decisions/decision_decisions-folder-ceiling.md, LIVE_DOCUMENT_SIZE_CONTAINMENT.md's
# `archive_terminal`).
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
# A SECTION is a `## ` heading outside a code fence and every line up to the next such heading. It is numbered when
# its heading reads `## <N>. `. Its BODY is that span without its trailing blank lines, which stay in the record after
# the stub. A STUB is exactly three lines: the heading, a blank line, and
#   Sealed, byte for byte, in [`<record>/<NN>.md`](<relative link>) — settled, and never edited again.
# A record is found by its file name anywhere under `docs/decisions/`, so a partition that moves it breaks nothing.
#
# THE GATE'S LEGS:
#   1. every sealed file's lines, bytes and sha256 are its index row's, and it opens with its section's heading;
#   2. sealed files and rows correspond one to one, and nothing else is under docs/decision-history/;
#   3. HISTORY-WIDE, every row any committed version of the index held is still there, unchanged, and every sealed
#      file is byte for byte what the commit that added it wrote — so CI, where HEAD is the commit under test, catches
#      a forgery as the pre-commit hook does;
#   4. every sealed file has exactly one stub, in its own record, under its own heading, linking it; every stub in any
#      record is a sealed file's;
#   5. PROVENANCE: every sealed file is, byte for byte, the section its record held just before the commit that sealed
#      it (HEAD, for a seal not yet committed) — so a seal made by hand, or a section edited on its way in, is refused.
#
# THE SEAL writes nothing unless the record it would leave, with every new stub replaced by its body from its new
# sealed file, is the record as it stood, byte for byte; and it rolls everything back if the gate then refuses.
#
# ⚠️ HONEST LIMIT: history rewritten under the gate (a force-push, a replaced object) is premises 2 and 3's, as for
# the catalog (docs/decisions/catalog/decision_catalog-records.md §0). Within history, legs 3 and 5 hold whatever HEAD
# is. Which sections are settled is the sealing leaf's judgement, recorded in its tree; the gate proves the move.
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
STUB_RE = re.compile(r"^Sealed, byte for byte, in \[`([^`]+)`\]\(([^)]+)\) — settled, and never edited again\.$")
ROW_RE = re.compile(r"^\| `([A-Za-z0-9_.+-]+)` \| `(\d{2,})` \| (\d+) \| (\d+) \| `([0-9a-f]{64})` \| `(\d{4}-\d{2}-\d{2})` \|$")
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

def at(commit, path):
    data = git("show", "%s:%s" % (commit, path))
    return None if data is None else decode(data, "%s:%s" % (commit, path))

def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def key(n):
    return "%02d" % int(n)

def sections(text):
    """(number or None, first, end) over text.split('\n'), for every `## ` heading outside a fence."""
    lines = text.split("\n")
    out, cur, fence = [], None, False
    for i, line in enumerate(lines):
        if line.lstrip().startswith("```"):
            fence = not fence
        if not fence and line.startswith("## "):
            if cur:
                out.append((cur[0], cur[1], i))
            m = NUM_RE.match(line)
            cur = (key(m.group(1)) if m else None, i)
    if cur:
        out.append((cur[0], cur[1], len(lines)))
    return lines, out

def body_end(lines, first, end):
    e = end
    while e > first + 1 and lines[e - 1].strip() == "":
        e -= 1
    return e

def is_stub(lines, first, end):
    return body_end(lines, first, end) == first + 3 and lines[first + 1] == "" and STUB_RE.match(lines[first + 2]) is not None

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
    names = (git("ls-tree", "-r", "--name-only", commit, "--", DECISIONS) or b"").decode().split("\n")
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
    """The five legs; returns (sealed files, stubs)."""
    index = rows(read(INDEX), INDEX) if os.path.exists(INDEX) else []
    files = {}  # path -> (stem, n)
    for stem, n, nlines, nbytes, digest, _ in index:
        fpath = os.path.join(HIST, stem, n + ".md")
        if fpath in files:
            note("%s lists %s section %s twice" % (INDEX, stem, n)); continue
        if not os.path.exists(fpath):
            note("%s lists %s section %s, and %s does not exist" % (INDEX, stem, n, fpath)); continue
        files[fpath] = (stem, n)
        data = read(fpath)
        if str(data.count("\n")) != nlines: note("%s has %d lines, and its row says %s — a sealed file changed" % (fpath, data.count("\n"), nlines))
        if str(len(data.encode("utf-8"))) != nbytes: note("%s has %d bytes, and its row says %s — a sealed file changed" % (fpath, len(data.encode("utf-8")), nbytes))
        if sha(data) != digest: note("%s's sha256 is not its row's — a sealed file changed" % fpath)
        m = NUM_RE.match(data)
        if not m or key(m.group(1)) != n: note("%s does not open with the heading of section %s" % (fpath, n))
    # Leg 2: one to one, nothing else.
    if os.path.isdir(HIST):
        for name in sorted(os.listdir(HIST)):
            p = os.path.join(HIST, name)
            if os.path.isdir(p):
                for f in sorted(os.listdir(p)):
                    if os.path.join(p, f) not in files:
                        note("%s/%s is in no row of %s — a sealed file must be listed" % (p, f, INDEX))
            elif name != "INDEX.md":
                note("%s is neither the index nor a record's folder" % p)
    # Leg 3, history-wide.
    current = set(index)
    for commit in (git("log", "--format=%H", "--", INDEX) or b"").decode().split():
        old = at(commit, INDEX)
        if old is None:
            continue
        for r in rows(old, "%s:%s" % (commit[:12], INDEX)):
            if r not in current:
                note("%s's row for %s section %s, committed in %s, is gone or changed — the index is append-only" % (INDEX, r[0], r[1], commit[:12]))
    added, commit = {}, None
    for line in (git("log", "--diff-filter=A", "--format=@%H", "--name-only", "--", HIST) or b"").decode().split("\n"):
        if line.startswith("@"):
            commit = line[1:]
        elif line.strip():
            added[line.strip()] = commit  # newest first, so the one kept is the oldest
    for fpath in files:
        c = added.get(fpath)
        if c and at(c, fpath) != read(fpath):
            note("%s is not what %s wrote when it sealed it — a sealed file changed" % (fpath, c[:12]))
    # Leg 4: stubs against files, in every record.
    recs = records_now()
    for stem in sorted({s for s, _ in files.values()}):
        paths = [p for s, p in recs if s == stem]
        if len(paths) > 1:
            note("%s share the file name %s.md, so its sealed sections cannot tell their record" % (" and ".join(paths), stem))
    stubs = {}
    for stem, rpath in recs:
        rlines, rsecs = sections(read(rpath))
        for n, first, end in rsecs:
            span = rlines[first:end]
            marked = any("Sealed, byte for byte, in [" in l for l in span)
            if not marked:
                if (stem, n) in files.values():
                    note("%s: section %s is sealed in %s/%s/%s.md and also live here" % (rpath, n, HIST, stem, n))
                continue
            if not is_stub(rlines, first, end):
                note("%s: the stub of section %s is not exactly its heading, a blank line and the stub's line" % (rpath, n)); continue
            fpath = os.path.join(HIST, stem, (n or "") + ".md")
            m = STUB_RE.match(rlines[first + 2])
            if m.group(1) != "%s/%s.md" % (stem, n) or rlines[first + 2] != stub_line(rpath, stem, n):
                note("%s: the stub of section %s does not link %s by its own path" % (rpath, n, fpath))
            if fpath not in files:
                note("%s: section %s has a stub, and no sealed file is listed for it" % (rpath, n)); continue
            if (stem, n) in stubs:
                note("%s: section %s has two stubs" % (rpath, n))
            stubs[(stem, n)] = rpath
            if read(fpath).split("\n")[0] != rlines[first]:
                note("%s: the heading above the stub of section %s is not the sealed file's" % (rpath, n))
    for fpath, sk in files.items():
        if sk not in stubs:
            note("%s is sealed and has no stub in its record" % fpath)
    # Leg 5, provenance.
    for fpath, (stem, n) in files.items():
        c = added.get(fpath)
        base = (c + "^") if c else "HEAD"
        rpath = record_at(base, stem)
        before = at(base, rpath) if rpath else None
        if before is None:
            note("%s: %s held no record %s.md under %s to be sealed from" % (fpath, base, stem, DECISIONS)); continue
        blines, bsecs = sections(before)
        found = [(f, e) for (x, f, e) in bsecs if x == n]
        if len(found) != 1:
            note("%s: %s's %s did not hold exactly one section %s" % (fpath, base, rpath, n)); continue
        f, e = found[0]
        if is_stub(blines, f, e):
            note("%s: section %s was already a stub in %s's %s" % (fpath, n, base, rpath)); continue
        if "\n".join(blines[f:body_end(blines, f, e)]) + "\n" != read(fpath):
            note("%s is not the section %s's %s held — a section was edited on its way in" % (fpath, base, rpath))
    return len(files), len(stubs)

def seal(record, numbers):
    if not os.path.isfile(record) or not record.startswith(DECISIONS + "/") or not record.endswith(".md"):
        sys.exit("decision-history: %s is not a record under %s" % (record, DECISIONS))
    stem = os.path.basename(record)[:-3]
    before = read(record)
    lines, secs = sections(before)
    want = [key(n) for n in numbers]
    by = {}
    for n, f, e in secs:
        if n is not None:
            by.setdefault(n, []).append((f, e))
    for n in want:
        if len(by.get(n, [])) != 1:
            sys.exit("decision-history: %s has no single section %s; nothing was written" % (record, n))
        f, e = by[n][0]
        if is_stub(lines, f, e):
            sys.exit("decision-history: %s's section %s is already sealed; nothing was written" % (record, n))
        if os.path.exists(os.path.join(HIST, stem, n + ".md")):
            sys.exit("decision-history: %s/%s/%s.md exists; nothing was written" % (HIST, stem, n))
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
    after = "\n".join(out)
    # The proof: every new stub replaced by its body reconstructs the record as it stood.
    alines, asecs = sections(after)
    rebuilt, i = [], 0
    for n, f, e in asecs:
        if n in files and is_stub(alines, f, e):
            rebuilt.extend(alines[i:f])
            rebuilt.extend(files[n][:-1].split("\n"))
            i = f + 3
    rebuilt.extend(alines[i:])
    if "\n".join(rebuilt) != before:
        sys.exit("decision-history: the seal of %s would not reconstruct it byte for byte; nothing was written" % record)
    index_before = read(INDEX) if os.path.exists(INDEX) else None
    os.makedirs(os.path.join(HIST, stem), exist_ok=True)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    written = []
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
    if fails:
        for p in written:
            os.remove(p)
        if not os.listdir(os.path.join(HIST, stem)):
            os.rmdir(os.path.join(HIST, stem))
        with open(record, "w", encoding="utf-8") as fh:
            fh.write(before)
        if index_before is None:
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
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/decisions/part"; git -C "$work" init -q
    cat > "$work/docs/decisions/part/decision_r.md" <<'MD'
# A register

Two items.

## 1. An open item

Still open.

## 2. A settled item — **RESOLVED**

It was settled, with a `code span` and
- a bullet.

```text
## a heading inside a fence, which is not a section
```

## 3. Another open item

Open.
MD
    printf '# Index\n' > "$work/docs/decisions/INDEX.md"
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
  fresh
  arm "a record with nothing sealed passes" 0 "0 sealed section(s)"
  arm "sealing a section the record does not have writes nothing" 1 "has no single section 07; nothing was written" --seal docs/decisions/part/decision_r.md 7
  [ ! -d "$work/docs/decision-history" ] && git -C "$work" diff --quiet || { arms=$((arms + 1)); echo "SELF-TEST: the refused seal wrote something" >&2; }
  arm "the seal proves its reconstruction" 0 "the reconstruction is byte for byte" --seal docs/decisions/part/decision_r.md 2
  arm "the sealed record passes the gate" 0 "1 sealed section(s)"
  grep -qx 'Sealed, byte for byte, in \[`decision_r/02.md`\](../../decision-history/decision_r/02.md) — settled, and never edited again.' "$work/docs/decisions/part/decision_r.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: the stub does not link its file from the record's own folder" >&2; }
  grep -q '^## a heading inside a fence' "$work/docs/decision-history/decision_r/02.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: a fenced heading split the section" >&2; }
  arm "sealing it again is refused" 1 "is already sealed; nothing was written" --seal docs/decisions/part/decision_r.md 2
  commit
  printf 'x' >> "$work/docs/decision-history/decision_r/02.md"
  arm "a byte added to a sealed file is refused" 1 "a sealed file changed"
  restore
  sub docs/decision-history/INDEX.md '^\| `decision_r` \| `02` \|.*\n' ''
  arm "a sealed file with no row is refused" 1 "is in no row of"
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
  sub docs/decisions/part/decision_r.md '^Sealed, byte for byte, in .*\n' 'Sealed, byte for byte, in [`decision_r/02.md`](../../decision-history/decision_r/02.md) — settled, and never edited again.\nA line added to the stub.\n'
  arm "a stub holding more than its three lines is refused" 1 "is not exactly its heading"
  restore
  sub docs/decisions/part/decision_r.md '^\n^Sealed, byte for byte, in .*\n' '\nIt was settled, and is live again.\n'
  arm "a sealed section made live again is refused" 1 "also live here"
  restore
  sub docs/decisions/part/decision_r.md '^## 2\. A settled item — \*\*RESOLVED\*\*$' '## 2. A settled item, retitled'
  arm "a heading changed above its stub is refused" 1 "is not the sealed file's"
  restore
  sub docs/decisions/part/decision_r.md '\(\.\./\.\./decision-history/decision_r/02\.md\)' '(../decision-history/decision_r/02.md)'
  arm "a stub whose link does not resolve from its record is refused" 1 "does not link"
  restore
  printf '\n## 4. Forged\n\nSealed, byte for byte, in [`decision_r/04.md`](../../decision-history/decision_r/04.md) — settled, and never edited again.\n' >> "$work/docs/decisions/part/decision_r.md"
  arm "a stub with no sealed file is refused" 1 "no sealed file is listed for it"
  restore
  cp "$work/docs/decisions/part/decision_r.md" "$work/docs/decisions/decision_r.md"
  arm "a second record under a sealed record's file name is refused" 1 "share the file name decision_r.md"
  restore
  printf 'stray\n' > "$work/docs/decision-history/notes.txt"
  arm "a stray file in the history folder is refused" 1 "neither the index nor a record's folder"
  restore
  # A seal made by hand of section 3 with its body reworded on the way in: provenance refuses it.
  python3 - "$work" <<'PY'
import hashlib, sys
w = sys.argv[1]
r = w + "/docs/decisions/part/decision_r.md"
t = open(r).read()
sec = "## 3. Another open item\n\nOpen."
assert sec in t
body = "## 3. Another open item\n\nClosed, it says now.\n"
t = t.replace(sec, "## 3. Another open item\n\nSealed, byte for byte, in [`decision_r/03.md`](../../decision-history/decision_r/03.md) — settled, and never edited again.")
open(r, "w").write(t)
open(w + "/docs/decision-history/decision_r/03.md", "w").write(body)
i = w + "/docs/decision-history/INDEX.md"
open(i, "a").write("| `decision_r` | `03` | %d | %d | `%s` | `2026-09-30` |\n" % (body.count("\n"), len(body.encode()), hashlib.sha256(body.encode()).hexdigest()))
PY
  arm "a section edited on its way in, by a seal made by hand, is refused" 1 "a section was edited on its way in"
  restore
  git -C "$work" mv docs/decisions/part/decision_r.md docs/decisions/decision_r.md
  sub docs/decisions/decision_r.md '\(\.\./\.\./decision-history/' '(../decision-history/'
  arm "a record moved to another folder, its stub's link following it, still passes" 0 "1 sealed section(s)"
  restore; git -C "$work" reset -q --hard HEAD
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
