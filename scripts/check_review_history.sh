#!/usr/bin/env bash
# scripts/check_review_history.sh — REVIEW-HISTORY: closed review histories archived out of docs/reviews/ (leaf
# `PROGRAM.55`, hardened by `PROGRAM.55.1` after its independent review; docs/decisions/decision_reviews-folder-
# ceiling.md), `DECISION-HISTORY`'s pattern for whole files.
#
# ⭐ WHY. A review history only grows while its review is open, and is frozen when it closes; `docs/reviews/` has a
# total ceiling and no overflow of its own, so closed histories kept there crowd out the open ones, and a frozen file
# cannot be compacted. The ceiling's record named this archive as the step after its raise. A closed history moves
# here byte for byte; a three-line stub stays at its old path, so every citation of it still resolves.
#
# MODES:
#   bash scripts/check_review_history.sh                  # the gate: exit 0 clean · 1 a breach, named
#   bash scripts/check_review_history.sh --seal <FILE>    # archive docs/reviews/<FILE>, prove, check
#   bash scripts/check_review_history.sh --self-test
#
# <FILE> is a name matching `[A-Za-z0-9_.+-]+\.md`, directly under docs/reviews/, never its INDEX.md. A STUB is a file
# of exactly three lines and a final newline: the history's own first line, a blank line, and
#   Archived, byte for byte, in [`<FILE>`](../review-history/<FILE>) — its review closed, and never edited again.
# A STUB-SHAPED file is any file of docs/reviews/ of three lines or fewer that names the archive or opens a line with
# "Archived, byte for byte"; every one must be an exact stub with its archived file. A longer history may quote the
# stub's line, in a fence or not, and is no stub. A REVIEW'S ROW is the one row of docs/reviews/INDEX.md's table whose
# first cell is [`<FILE>`](<FILE>): exactly the header's four cells, its status matching `closed: `, and <FILE>
# linked nowhere else in that index, comments and fences included.
#
# THE GATE reads what the commit will hold: the staged copy of every file it judges (`git show :<path>`), so a file
# left unstaged, partly staged or ignored is refused at the hook, not in CI. `--seal` reads the working tree it writes.
#
# THE SEAL refuses, writing nothing: a <FILE> that is not a history directly in docs/reviews/; a stub-shaped history;
# one with a carriage return or no final newline; one whose review's row is not as above; and an archived file of
# that name. It writes the archived file, the stub and the row byte for byte, reads the archived file back, and rolls
# everything back if the gate then refuses, for any reason. An archiving is a commit of its own, never a merge.
#
# THE GATE'S LEGS:
#   0. the repository is not shallow, and every history read succeeds;
#   1. the index begins with its header, byte for byte, and holds nothing after its rows but blank lines; every
#      archived file is a file, its lines, bytes and sha256 its row's, with no carriage return and a final newline; no
#      row twice;
#   2. archived files and rows correspond one to one, and nothing else tracked or untracked-and-not-ignored is under
#      docs/review-history/;
#   3. HISTORY-WIDE, with `--full-history`: every row any committed version of the index held is still there,
#      unchanged — which, with leg 1, holds every archived file to the bytes it was archived with;
#   4. every archived file has its exact stub at docs/reviews/<FILE> and its review's row, closed; every stub-shaped
#      file of docs/reviews/ is such a stub, at the folder's top, with its archived file;
#   5. PROVENANCE: every archived file was added by a commit of its own, never a merge, and is, byte for byte, what
#      docs/reviews/<FILE> held just before it (HEAD, for one not yet committed), when it was no stub, its row already
#      closed, and no commit had changed it while its row read closed; every later commit touching docs/reviews/<FILE>,
#      a side branch merged in included, holds the exact stub; its row's date lies between that commit's parent and
#      the archiving commit, in UTC.
#
# ⚠️ HONEST LIMITS:
#   - history rewritten under the gate is premises 2 and 3's, as for the catalog (docs/specs/catalog/
#     decision_catalog-records.md §0); within history, legs 3 and 5 hold whatever HEAD is;
#   - whether a review is truly closed is its leaf's and its row's; this reads the row, and a row turned `closed` by a
#     commit of its own before the archiving is what review of that commit sees;
#   - the history-wide legs walk `git log` order; a history edited on a side branch while its row read closed there, and
#     merged before its archiving, is judged by that branch's own row;
#   - a breach committed past the hook stays red, since the index and the files are append-only: repair it by
#     rewriting the commits not yet pushed, never `main`'s, or in a leaf that owns the exception.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/review_history"
mkdir -p "$SCRATCH"

core() { # $1 = gate | seal, then the file name (seal only)
  GIT_NO_REPLACE_OBJECTS=1 python3 - "$@" <<'PY'
import datetime, hashlib, os, re, subprocess, sys

mode, args = sys.argv[1], sys.argv[2:]
REVIEWS, HIST = "docs/reviews", "docs/review-history"
INDEX, RINDEX = os.path.join(HIST, "INDEX.md"), os.path.join(REVIEWS, "INDEX.md")
NAME_RE = re.compile(r"^[A-Za-z0-9_.+-]+\.md$")
STUB_LINE = re.compile(r"^Archived, byte for byte, in \[`[^`]+`\]\(\.\./review-history/[^)]+\) — its review closed, and never edited again\.$")
ROW_RE = re.compile(r"^\| `([A-Za-z0-9_.+-]+\.md)` \| (\d+) \| (\d+) \| `([0-9a-f]{64})` \| `(\d{4}-\d{2}-\d{2})` \|$")
TABLE_HEAD = "| Review history | Design record | Rounds | Status |"
TABLE_RULE = "| --- | --- | --- | --- |"
FAULT = os.environ.get("REVIEW_HISTORY_SELFTEST_FAULT", "")  # the self-test's one seam: it can only make a seal fail
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

_git = {}
def git(*a):
    if a not in _git:
        r = subprocess.run(("git",) + a, capture_output=True)
        _git[a] = r.stdout if r.returncode == 0 else None
    return _git[a]

def history(*a):
    out = git(*a)
    if out is None:
        note("`git %s` failed, so the history legs cannot hold — the gate never reads a failure as no history" % " ".join(a))
        return ""
    return out.decode()

def at(commit, path):
    data = git("show", "%s:%s" % (commit, path))
    return None if data is None else decode(data, "%s:%s" % (commit, path))

def content(path):
    """What the commit will hold at `path`: the staged copy at the gate, the working tree at the seal; None if none."""
    if mode == "seal":
        if os.path.isdir(path):
            note("%s is a directory, where a file must stand" % path); return None
        if not os.path.isfile(path):
            return None
        with open(path, "rb") as f:
            return decode(f.read(), path)
    data = git("show", ":" + path)
    if data is None:
        if os.path.isdir(path):
            note("%s is a directory, where a file must stand" % path)
        elif os.path.exists(path):
            note("%s is on disk and not staged, so the commit would not hold it — stage it" % path)
        return None
    staged = decode(data, ":" + path)
    if os.path.isfile(path):
        with open(path, "rb") as f:
            if f.read() != data:
                note("%s differs from its staged copy — stage it whole, or the commit holds another file" % path)
    return staged

def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def utc_day(commit):
    out = git("log", "-1", "--format=%ct", commit)
    return None if out is None else datetime.datetime.fromtimestamp(int(out.decode().strip()), datetime.timezone.utc).strftime("%Y-%m-%d")

def stub_text(name, first_line):
    return "%s\n\nArchived, byte for byte, in [`%s`](../review-history/%s) — its review closed, and never edited again.\n" % (first_line, name, name)

def stub_shaped(text):
    return text.count("\n") <= 3 and ("](../review-history/" in text or any(l.startswith("Archived, byte for byte") for l in text.split("\n")))

def review_rows(text):
    """{name: [(cells, status)]} for the rows of docs/reviews/INDEX.md's table, and {name: links anywhere}."""
    lines = text.split("\n")
    rows, links = {}, {}
    for m in re.finditer(r"\]\((?:\./)?([A-Za-z0-9_.+-]+\.md)\)", text):
        links[m.group(1)] = links.get(m.group(1), 0) + 1
    if lines.count(TABLE_HEAD) != 1:
        note("%s holds its table's header `%s` %d times, and has one" % (RINDEX, TABLE_HEAD, lines.count(TABLE_HEAD)))
        return rows, links
    i = lines.index(TABLE_HEAD) + 1
    if i >= len(lines) or lines[i] != TABLE_RULE:
        note("%s's table header is not followed by `%s`" % (RINDEX, TABLE_RULE)); return rows, links
    for line in lines[i + 1:]:
        if not line.startswith("|"):
            break
        cells = [c.strip() for c in line.strip().strip("|").split(" | ")]
        m = re.match(r"^\[`([^`]+)`\]\(([^)]+)\)$", cells[0])
        name = m.group(1) if m and m.group(1) == m.group(2) else None
        if name:
            rows.setdefault(name, []).append((len(cells), cells[-1]))
    return rows, links

def closed(rows, links, name):
    """None when <name>'s review is closed by its one row; else why not."""
    r = rows.get(name, [])
    if len(r) != 1:
        return "has %d rows in %s's table, and a review has one" % (len(r), RINDEX)
    if r[0][0] != 4:
        return "'s row in %s has %d cells, and the table has 4 — a cell beyond them is dropped from the page" % (RINDEX, r[0][0])
    if not re.match(r"^closed: ", r[0][1]):
        return "'s row in %s says %r, and only a review whose status begins `closed: ` is archived" % (RINDEX, r[0][1][:40])
    if links.get(name, 0) != 1:
        return "is linked %d times in %s, and only its own row may link it" % (links.get(name, 0), RINDEX)
    return None

def rows(index_text, where):
    out, past = [], False
    for n, line in enumerate(index_text.split("\n"), 1):
        if line.startswith("| `"):
            if past:
                note("%s:%d is a row after other text — the rows close the file" % (where, n)); continue
            r = ROW_RE.match(line)
            if not r:
                note("%s:%d is not a row of the form `| `FILE.md` | lines | bytes | `sha256` | `date` |`" % (where, n)); continue
            out.append(r.groups())
        elif out and line.strip():
            past = True
            note("%s:%d is text after the rows — nothing follows them but blank lines" % (where, n))
    return out

HEADER = """# docs/review-history/INDEX.md — closed review histories archived out of docs/reviews/

Each file below is a review history whose review closed, moved here byte for byte by
`bash scripts/check_review_history.sh --seal <FILE>` and never edited again (`PROGRAM.55`,
`docs/decisions/decision_reviews-folder-ceiling.md`). A three-line stub stays at its old path in `docs/reviews/`,
linking here, so a citation of it still finds it. A row records the file's lines, bytes and sha256, and the day it
was archived. The rows are append-only, and `REVIEW-HISTORY` checks every file against its row, its stub and the
history it came from, on every commit.

To prove a file, compare `sha256sum docs/review-history/<FILE>` with its row.

| File | Lines | Bytes | sha256 | Archived |
| --- | --- | --- | --- | --- |
"""

def reviews_files():
    """[path] of every Markdown file the commit holds under docs/reviews/."""
    if mode == "seal":
        out = []
        for d, _, names in os.walk(REVIEWS):
            out.extend(os.path.join(d, n) for n in names if n.endswith(".md"))
        return sorted(out)
    return sorted(p for p in history("ls-files", "-c", "--", REVIEWS).split("\n") if p.endswith(".md"))

def gate():
    """The legs; returns the number of archived files."""
    shallow = git("rev-parse", "--is-shallow-repository")
    if shallow is None or shallow.decode().strip() != "false":
        note("the repository is shallow, or git cannot say, so the commits legs 3 and 5 read may be missing — fetch the full history")
    # Leg 1.
    index, itext = [], content(INDEX)
    if itext is not None:
        if not itext.startswith(HEADER):
            note("%s does not begin with its header, byte for byte — the header states what the rows promise" % INDEX)
        index = rows(itext, INDEX)
    files, dates, bodies = {}, {}, {}
    for name, nlines, nbytes, digest, day in index:
        fpath = os.path.join(HIST, name)
        if fpath in files:
            note("%s lists %s twice" % (INDEX, name)); continue
        data = content(fpath)
        if data is None:
            note("%s lists %s, and %s is not a file the commit holds" % (INDEX, name, fpath)); continue
        files[fpath], dates[fpath], bodies[fpath] = name, day, data
        if str(data.count("\n")) != nlines: note("%s has %d lines, and its row says %s — an archived file changed" % (fpath, data.count("\n"), nlines))
        if str(len(data.encode("utf-8"))) != nbytes: note("%s has %d bytes, and its row says %s — an archived file changed" % (fpath, len(data.encode("utf-8")), nbytes))
        if sha(data) != digest: note("%s's sha256 is not its row's — an archived file changed" % fpath)
        if "\r" in data or not data.endswith("\n"):
            note("%s holds a carriage return or lacks a final newline, which the seal refuses" % fpath)
    # Leg 2.
    for p in sorted(x for x in history("ls-files", "-co", "--exclude-standard", "--", HIST).split("\n") if x):
        if p != INDEX and p not in files:
            note("%s is neither the index nor an archived file listed in it" % p)
    # Leg 3: the rows, append-only across history.
    current = set(index)
    for commit in history("log", "--full-history", "--format=%H", "--", INDEX).split():
        old = at(commit, INDEX)
        if old is None:
            continue
        for r in rows(old, "%s:%s" % (commit[:12], INDEX)):
            if r not in current:
                note("%s's row for %s, committed in %s, is gone or changed — the index is append-only" % (INDEX, r[0], commit[:12]))
    # Leg 4.
    rtext = content(RINDEX)
    rrows, rlinks = review_rows(rtext) if rtext is not None else ({}, {})
    for fpath, name in files.items():
        spath = os.path.join(REVIEWS, name)
        stub = content(spath)
        if stub is None:
            note("%s is archived and has no stub at %s" % (fpath, spath))
        elif stub != stub_text(name, bodies[fpath].split("\n")[0]):
            note("%s is not exactly the stub of %s: its first line, a blank line and the line linking it" % (spath, fpath))
        why = closed(rrows, rlinks, name)
        if why:
            note("%s %s" % (name, why))
    for p in reviews_files():
        text = content(p)
        if text is None or not stub_shaped(text):
            continue
        if os.path.dirname(p) != REVIEWS:
            note("%s is stub-shaped in a sub-folder; a stub stands at %s/<FILE>" % (p, REVIEWS))
        elif os.path.join(HIST, os.path.basename(p)) not in files:
            note("%s is stub-shaped, and no archived file of its name is listed" % p)
    # Leg 5.
    added, commit = {}, None
    for line in history("log", "--full-history", "--diff-filter=A", "--format=@%H", "--name-only", "--", HIST).split("\n"):
        if line.startswith("@"):
            commit = line[1:]
        elif line.strip():
            added[line.strip()] = commit  # newest first, so the one kept is the oldest
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    for fpath, name in files.items():
        spath = os.path.join(REVIEWS, name)
        c = added.get(fpath)
        if c is None and git("cat-file", "-e", "HEAD:" + fpath) is not None:
            note("%s entered the history through a merge commit; an archiving is a commit of its own" % fpath); continue
        base = (c + "^") if c else "HEAD"
        before = at(base, spath)
        if before is None:
            note("%s: %s held no %s to archive" % (fpath, base, spath)); continue
        if stub_shaped(before):
            note("%s: %s was already stub-shaped in %s" % (fpath, spath, base)); continue
        if before != bodies[fpath]:
            note("%s is not what %s held in %s — a history was edited on its way in" % (fpath, spath, base))
        was = at(base, RINDEX)
        if was is None or closed(*review_rows(was), name):
            note("%s: its review was not closed in %s's %s, so it was archived open" % (fpath, base, RINDEX))
        for x in history("log", "--full-history", "--format=%H", base, "--", spath).split():
            parent_index = at(x + "^", RINDEX)
            if parent_index is not None and closed(*review_rows(parent_index), name) is None:
                note("%s: %s changed %s while its review's row already read closed — a closed history is frozen" % (fpath, x[:12], spath))
        if c:
            for x in history("log", "--full-history", "--format=%H", "HEAD", "--not", c, "--", spath).split():
                if at(x, spath) != stub_text(name, bodies[fpath].split("\n")[0]):
                    note("%s: %s holds %s as other than its stub, after its archiving — a round merged in would be lost" % (fpath, x[:12], spath))
        low, high = utc_day(base), (utc_day(c) if c else today)
        if low is None or high is None or not (low <= dates[fpath] <= high):
            note("%s's row is dated %s, outside the days its archiving could have been made (%s to %s, UTC)" % (fpath, dates[fpath], low, high))
    return len(files)

def seal(name):
    if os.sep in name or not NAME_RE.match(name) or name == "INDEX.md" or not os.path.isfile(os.path.join(REVIEWS, name)):
        sys.exit("review-history: %s is not a review history directly under %s; nothing was written" % (name, REVIEWS))
    spath, fpath = os.path.join(REVIEWS, name), os.path.join(HIST, name)
    with open(spath, "rb") as f:
        before = decode(f.read(), spath)
    if "\r" in before or not before.endswith("\n"):
        sys.exit("review-history: %s holds a carriage return or lacks a final newline; nothing was written" % spath)
    if stub_shaped(before):
        sys.exit("review-history: %s is stub-shaped, already a stub or near one; nothing was written" % spath)
    with open(RINDEX, "rb") as f:
        why = closed(*review_rows(decode(f.read(), RINDEX)), name)
    if why:
        sys.exit("review-history: %s %s; nothing was written" % (name, why))
    if os.path.exists(fpath):
        sys.exit("review-history: %s exists; nothing was written" % fpath)
    archived = before if FAULT != "drop-byte" else before[:-2] + "\n"
    index_before = None
    if os.path.exists(INDEX):
        with open(INDEX, "rb") as f:
            index_before = decode(f.read(), INDEX)
    made_dir = not os.path.isdir(HIST)
    os.makedirs(HIST, exist_ok=True)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    wrote = False
    try:
        with open(fpath, "w", encoding="utf-8", newline="") as fh:
            fh.write(archived)
        wrote = True
        with open(fpath, "rb") as fh:
            if fh.read() != before.encode("utf-8"):
                note("the archived copy of %s does not read back byte for byte" % spath)
        if not fails:
            with open(spath, "w", encoding="utf-8", newline="") as fh:
                fh.write(stub_text(name, before.split("\n")[0]))
            text = index_before if index_before is not None else HEADER
            text = text.rstrip("\n") + "\n" + "| `%s` | %d | %d | `%s` | `%s` |\n" % (
                name, archived.count("\n"), len(archived.encode("utf-8")), sha(archived), today)
            with open(INDEX, "w", encoding="utf-8", newline="") as fh:
                fh.write(text)
            gate()
    except Unreadable as e:
        note(str(e))
    if fails:
        if wrote and os.path.exists(fpath):
            os.remove(fpath)
        with open(spath, "w", encoding="utf-8", newline="") as fh:
            fh.write(before)
        if index_before is None:
            if os.path.exists(INDEX):
                os.remove(INDEX)
        else:
            with open(INDEX, "w", encoding="utf-8", newline="") as fh:
                fh.write(index_before)
        if made_dir and os.path.isdir(HIST) and not os.listdir(HIST):
            os.rmdir(HIST)
        sys.exit("review-history: the archiving of %s was refused, so it was rolled back:\n  " % spath + "\n  ".join(fails))
    print("review-history: %s — archived, %d bytes, read back byte for byte; a stub left at its path; stage all three" % (spath, len(archived.encode("utf-8"))))

n = 0
try:
    if mode == "seal":
        seal(args[0])  # exits on a refusal, its own gate run over the files it wrote
        sys.exit(0)
    n = gate()
except Unreadable as e:
    note(str(e))
fails = list(dict.fromkeys(fails))  # one message per breach, however many legs read the file
for msg in fails:
    print("REVIEW-HISTORY: " + msg, file=sys.stderr)
if fails:
    print("REVIEW-HISTORY: %d breach(es) — PROGRAM.55, docs/decisions/decision_reviews-folder-ceiling.md" % len(fails), file=sys.stderr)
    sys.exit(1)
print("review-history: OK (%d archived history(ies), each against its row, its stub and its closed review, read as staged; the rows append-only across history; every archived file its frozen history before archiving)" % n)
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest" main
  fresh() {
    rm -rf "$work" "$work-shallow"; mkdir -p "$work/docs/reviews"; git -C "$work" init -q
    printf '# Closed design: its reviews\n\nRound 1 found nothing.\n' > "$work/docs/reviews/c-reviews.md"
    printf '# Open design: its reviews\n\nRound 1 found two defects.\n' > "$work/docs/reviews/o-reviews.md"
    printf '# Review histories\n\n| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n| [`c-reviews.md`](c-reviews.md) | `c` | 1 | closed: round 1 found no defect |\n| [`o-reviews.md`](o-reviews.md) | `o` | 1 | open: round 2 next |\n' > "$work/docs/reviews/INDEX.md"
    printf '.DS_Store\n' > "$work/.gitignore"
    git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm base
    main="$(git -C "$work" symbolic-ref --short HEAD)"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the arguments; the gate reads the stage
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    [ "${1:-}" = "--seal" ] || git -C "$work" add -A
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,4p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  raw() { # as `arm`, without staging first
    local name="$1" want="$2" must="$3" out rc; shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ] || { [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; }; then
      echo "SELF-TEST: $name — expected exit $want and \`$must\`, got $rc:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  check() { # $1 = name, then a shell condition that must hold
    local name="$1"; shift
    arms=$((arms + 1))
    if "$@"; then ok=$((ok + 1)); echo "  ✅ $name"; else echo "SELF-TEST: $name — it does not hold" >&2; fi
  }
  unchanged() {
    arms=$((arms + 1))
    if [ -z "$(git -C "$work" status --porcelain --untracked-files=all)" ]; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — the tree changed:" >&2; git -C "$work" status --porcelain --untracked-files=all | sed 's/^/    /' >&2; fi
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
  restore() { git -C "$work" reset -q && git -C "$work" checkout -q -- . && git -C "$work" clean -qfd; }
  handseal() { # $1 = file name, $2 = the body to archive ("" for the history itself), $3 = the row's date
    python3 - "$work" "$1" "$2" "$3" <<'PY'
import hashlib, os, sys
w, name, body, day = sys.argv[1:5]
src = os.path.join(w, "docs/reviews", name)
t = open(src).read() if os.path.exists(src) else body
body = body if body else t
os.makedirs(os.path.join(w, "docs/review-history"), exist_ok=True)
open(os.path.join(w, "docs/review-history", name), "w").write(body)
open(src, "w").write("%s\n\nArchived, byte for byte, in [`%s`](../review-history/%s) — its review closed, and never edited again.\n" % (t.split("\n")[0], name, name))
open(os.path.join(w, "docs/review-history/INDEX.md"), "a").write("| `%s` | %d | %d | `%s` | `%s` |\n" % (name, body.count("\n"), len(body.encode()), hashlib.sha256(body.encode()).hexdigest(), day))
PY
  }
  setrow() { # $1 = a python expression of (lines, bytes, sha) from s, the archived file's text
    python3 - "$work" "$1" <<'PY'
import hashlib, re, sys
w, expr = sys.argv[1:3]
s = open(w + "/docs/review-history/c-reviews.md").read()
h = hashlib.sha256(s.encode()).hexdigest()
lines, nbytes, digest = eval(expr)
i = w + "/docs/review-history/INDEX.md"
t = re.sub(r"^\| `c-reviews\.md` \| \d+ \| \d+ \| `[0-9a-f]+` \|", "| `c-reviews.md` | %d | %d | `%s` |" % (lines, nbytes, digest), open(i).read(), flags=re.M)
open(i, "w").write(t)
PY
  }
  local TODAY out H=docs/review-history
  TODAY="$(date -u +%Y-%m-%d)"

  fresh
  arm "a folder with nothing archived passes" 0 "0 archived history"
  arm "archiving a file that does not exist writes nothing" 1 "is not a review history directly under" --seal x-reviews.md
  arm "archiving the reviews index is refused" 1 "is not a review history directly under" --seal INDEX.md
  arm "archiving by a path is refused" 1 "is not a review history directly under" --seal ../reviews/c-reviews.md
  arm "archiving an open review's history is refused" 1 "whose status begins \`closed: \`" --seal o-reviews.md
  sub docs/reviews/INDEX.md 'open: round 2 next' 'closedness not reached: open, round 2 next'
  arm "a status that only begins with the word closed is refused" 1 "whose status begins \`closed: \`" --seal o-reviews.md
  restore
  sub docs/reviews/INDEX.md 'open: round 2 next \|' 'open: round 2 next | closed: a fifth cell |'
  arm "a fifth cell the page drops cannot close a review" 1 "has 5 cells" --seal o-reviews.md
  restore
  sub docs/reviews/INDEX.md '^\| \[`o-reviews\.md`\]\(o-reviews\.md\)' '| [o-reviews.md](o-reviews.md)'
  printf '<!--\n| [`o-reviews.md`](o-reviews.md) | `o` | 1 | closed: hidden |\n-->\n' >> "$work/docs/reviews/INDEX.md"
  arm "a row the table does not hold, beside a hidden one in a comment, cannot close a review" 1 "has 0 rows" --seal o-reviews.md
  restore
  unchanged "and the refused archivings wrote nothing"
  printf 'no final newline' > "$work/docs/reviews/n-reviews.md"
  printf '| [`n-reviews.md`](n-reviews.md) | `n` | 1 | closed: done |\n' >> "$work/docs/reviews/INDEX.md"
  arm "a history with no final newline is refused by the seal itself" 1 "lacks a final newline; nothing was written" --seal n-reviews.md
  restore
  REVIEW_HISTORY_SELFTEST_FAULT=drop-byte arm "an archived copy that does not read back byte for byte is rolled back" 1 "does not read back byte for byte" --seal c-reviews.md
  unchanged "and the unproven archiving left nothing behind"
  mkdir -p "$work/$H" && printf 'stray\n' > "$work/$H/notes.txt"
  arm "an archiving the gate refuses is rolled back" 1 "so it was rolled back" --seal c-reviews.md
  rm -f "$work/$H/notes.txt"
  check "and the rolled-back archiving left nothing under the archive" test -z "$(ls -A "$work/$H" 2>/dev/null)"
  rmdir "$work/$H" 2>/dev/null
  unchanged "and nothing elsewhere"
  printf '# Bad\n\n\xff\xfe\n' > "$work/docs/reviews/bad.md"
  arm "an archiving whose gate meets an unreadable history is rolled back" 1 "so it was rolled back" --seal c-reviews.md
  rm -f "$work/docs/reviews/bad.md"
  unchanged "and the archiving rolled back on an unreadable file left nothing behind"
  mkdir -p "$work/$H" && printf 'already\n' > "$work/$H/c-reviews.md"
  arm "an archived file of that name already there is refused" 1 "exists; nothing was written" --seal c-reviews.md
  rm -rf "$work/$H"
  arm "an archiving proves its copy" 0 "read back byte for byte" --seal c-reviews.md
  check "and its stub links the archived file" grep -qx 'Archived, byte for byte, in \[`c-reviews.md`\](../review-history/c-reviews.md) — its review closed, and never edited again.' "$work/docs/reviews/c-reviews.md"
  raw "an archiving left unstaged is refused at the gate" 1 "is on disk and not staged"
  arm "the staged archiving passes the gate" 0 "1 archived history"
  arm "archiving it again is refused" 1 "is stub-shaped" --seal c-reviews.md
  commit
  printf 'x' >> "$work/$H/c-reviews.md"
  arm "a byte added to an archived file is refused against its row" 1 "and its row says"
  restore
  printf 'x' >> "$work/$H/c-reviews.md"; git -C "$work" add -A; git -C "$work" show "HEAD:$H/c-reviews.md" > "$work/$H/c-reviews.md"
  raw "an archived file whose staged copy differs is refused" 1 "differs from its staged copy"
  restore
  setrow '(s.count("\n") + 1, len(s.encode()), h)'
  arm "a row whose lines alone are wrong is refused" 1 "lines, and its row says"
  restore
  sub "$H/c-reviews.md" 'found nothing' 'found NOTHING'
  arm "a same-length change is refused by its sha256 alone" 1 "sha256 is not its row's"
  restore
  sub $H/INDEX.md '^\| `c-reviews\.md` \|.*\n' ''
  arm "an archived file with no row is refused" 1 "is neither the index nor an archived file"
  restore
  sub $H/INDEX.md '^(\| `c-reviews\.md` \|.*\n)' '\1\1'
  arm "a row listed twice is refused" 1 "lists c-reviews.md twice"
  restore
  printf '| `z-reviews.md` | 1 | 1 | `%064d` | `%s` |\n' 0 "$TODAY" >> "$work/$H/INDEX.md"
  arm "a row whose file does not exist is refused" 1 "is not a file the commit holds"
  restore
  printf '| `c-reviews.md` | 3 |\n' >> "$work/$H/INDEX.md"
  arm "a malformed row is refused" 1 "is not a row of the form"
  restore
  printf '\nThe row above is withdrawn.\n' >> "$work/$H/INDEX.md"
  arm "prose after the rows is refused" 1 "is text after the rows"
  restore
  sub $H/INDEX.md '^Each file below' 'Any file below may change; each file below'
  arm "an index whose header was rewritten is refused" 1 "does not begin with its header"
  restore
  sub $H/INDEX.md '^(\| `c-reviews\.md` \|.*\| `)[0-9-]+(` \|)$' '\g<1>1999-01-01\2'
  commit
  arm "a committed row that changed is refused across history" 1 "the index is append-only"
  git -C "$work" reset -q --hard HEAD~1
  sub "$H/c-reviews.md" 'found nothing' 'found a defect'
  setrow '(s.count("\n"), len(s.encode()), h)'
  commit
  arm "an archived file and its row forged together and committed are refused" 1 "the index is append-only"
  git -C "$work" reset -q --hard HEAD~1
  git -C "$work" checkout -q -b other HEAD~1
  printf 'unrelated\n' > "$work/notes.md"; commit
  git -C "$work" -c user.name=t -c user.email=t@t merge -q -s ours -m "drop the archive" "$main"
  arm "a merge that keeps only the other side cannot hide an archiving" 1 "the index is append-only"
  git -C "$work" checkout -q "$main"; git -C "$work" branch -q -D other
  git -C "$work" checkout -q -b late HEAD~1
  printf '\nRound 2, a late erratum.\n' >> "$work/docs/reviews/c-reviews.md"; commit
  git -C "$work" checkout -q "$main"
  git -C "$work" -c user.name=t -c user.email=t@t merge -q -s ours -m "keep the archive" late
  arm "a round merged in after the archiving, then dropped, is refused" 1 "a round merged in would be lost"
  git -C "$work" reset -q --hard HEAD~1; git -C "$work" branch -q -D late
  git clone -q --depth 1 "file://$work" "$work-shallow" 2>/dev/null
  arms=$((arms + 1))
  if out="$(cd "$work-shallow" && bash "$SELF" 2>&1)"; then echo "SELF-TEST: a shallow clone passed" >&2
  elif printf '%s' "$out" | grep -qF "the repository is shallow"; then ok=$((ok + 1)); echo "  ✅ a shallow clone is refused"
  else echo "SELF-TEST: a shallow clone was refused for another reason:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; fi
  rm -rf "$work-shallow"
  printf 'noise\n' > "$work/$H/.DS_Store"
  arm "an ignored file in the archive is not seen" 0 "1 archived history"
  printf 'stray\n' > "$work/$H/notes.txt"
  arm "a stray file in the archive is refused" 1 "is neither the index nor an archived file"
  restore
  printf 'One more line.\n' >> "$work/docs/reviews/c-reviews.md"
  arm "a stub holding more than its three lines is refused" 1 "is not exactly the stub"
  restore
  cp "$work/$H/c-reviews.md" "$work/docs/reviews/c-reviews.md"
  arm "an archived history made live again is refused" 1 "is not exactly the stub"
  restore
  git -C "$work" rm -q docs/reviews/c-reviews.md
  arm "an archived history whose stub was deleted is refused" 1 "has no stub"
  restore
  sub docs/reviews/c-reviews.md '\(\.\./review-history/c-reviews\.md\)' '(review-history/c-reviews.md)'
  arm "a stub whose link does not resolve is refused" 1 "is not exactly the stub"
  restore
  mkdir -p "$work/docs/reviews/old" && git -C "$work" mv docs/reviews/c-reviews.md docs/reviews/old/c-reviews.md
  arm "a stub moved into a sub-folder is refused" 1 "is stub-shaped in a sub-folder"
  restore
  printf '# Another\n\nArchived, byte for byte, in [`q-reviews.md`](../review-history/q-reviews.md) — its review closed, and never edited again.\n' > "$work/docs/reviews/q-reviews.md"
  arm "a stub with no archived file is refused" 1 "no archived file of its name is listed"
  restore
  printf '# Closed design: its reviews\n\nArchived, byte for byte, in [`o-reviews.md`](../review-history/o-reviews.md) — its review closed, and never edited again. \n' > "$work/docs/reviews/o-reviews.md"
  arm "a near-stub left where a history stood is refused" 1 "is stub-shaped, and no archived file"
  restore
  printf '# Quoting\n\nThe stub reads:\n\n```text\nArchived, byte for byte, in [`x.md`](../review-history/x.md) — its review closed, and never edited again.\n```\n' > "$work/docs/reviews/k-reviews.md"
  arm "a longer history quoting the stub's line passes" 0 "1 archived history"
  restore
  rm "$work/docs/reviews/c-reviews.md" && mkdir "$work/docs/reviews/c-reviews.md" && printf 'x\n' > "$work/docs/reviews/c-reviews.md/x"
  arm "a directory where the stub stands is named" 1 "has no stub"
  restore
  sub docs/reviews/INDEX.md 'closed: round 1 found no defect' 'open: reopened'
  arm "an archived history whose review was reopened is refused" 1 "only a review whose status begins"
  restore
  printf '| [`c-reviews.md`](c-reviews.md) | `c` | 2 | closed: again |\n' >> "$work/docs/reviews/INDEX.md"
  arm "an archived history with two rows is refused" 1 "has 2 rows in"
  restore
  printf '\nSee [the closed one](c-reviews.md).\n' >> "$work/docs/reviews/INDEX.md"
  arm "a second link to an archived history in the index is refused" 1 "is linked 2 times"
  restore
  handseal o-reviews.md "" "$TODAY"
  sub docs/reviews/INDEX.md 'open: round 2 next' 'closed: marked so in the same commit'
  arm "a history archived while its review was open is refused" 1 "so it was archived open"
  restore
  sub docs/reviews/INDEX.md 'open: round 2 next' 'closed: round 2 found no defect'
  commit
  handseal o-reviews.md '# Open design: its reviews

Round 1 found nothing at all.
' "$TODAY"
  arm "a history edited on its way in, by an archiving made by hand, is refused" 1 "a history was edited on its way in"
  restore
  handseal o-reviews.md "" "2001-01-01"
  arm "a faithful archiving made by hand, dated before its history existed, is refused" 1 "outside the days its archiving could have been made"
  restore
  handseal o-reviews.md "" "$TODAY"
  arm "a faithful archiving made by hand passes" 0 "2 archived history"
  restore
  printf '\nRound 2, an erratum after the close.\n' >> "$work/docs/reviews/o-reviews.md"; commit
  handseal o-reviews.md "" "$TODAY"
  arm "a history changed after its review closed is refused" 1 "a closed history is frozen"
  restore; git -C "$work" reset -q --hard HEAD~1
  printf '# Never a history\n\nIt was never committed.\n' > "$work/docs/reviews/f-reviews.md"
  printf '| [`f-reviews.md`](f-reviews.md) | `f` | 1 | closed: made up |\n' >> "$work/docs/reviews/INDEX.md"
  handseal f-reviews.md "" "$TODAY"
  arm "an archive of a history no commit held is refused" 1 "held no docs/reviews/f-reviews.md to archive"
  restore
  printf '# Stub first\n\nArchived, byte for byte, in [`s-reviews.md`](../review-history/s-reviews.md) — its review closed, and never edited again.\n' > "$work/docs/reviews/s-reviews.md"
  printf '| [`s-reviews.md`](s-reviews.md) | `s` | 1 | closed: done |\n' >> "$work/docs/reviews/INDEX.md"; commit
  cp "$work/docs/reviews/s-reviews.md" "$work/s.keep"
  handseal s-reviews.md "$(cat "$work/s.keep")
" "$TODAY"
  rm -f "$work/s.keep"
  arm "an archive whose history was already a stub is refused" 1 "was already stub-shaped"
  restore; git -C "$work" reset -q --hard HEAD~1
  git -C "$work" reset -q --hard HEAD~1
  printf 'Windows\r\n' >> "$work/docs/reviews/o-reviews.md"
  sub docs/reviews/INDEX.md 'open: round 2 next' 'closed: round 2 found no defect'
  arm "a history with a carriage return is refused by the seal itself" 1 "lacks a final newline; nothing was written" --seal o-reviews.md
  restore
  rm -rf "$work/$H/c-reviews.md"; printf '%s\r\n' '# Closed design: its reviews' > "$work/$H/c-reviews.md"
  arm "an archived file with a carriage return is refused" 1 "holds a carriage return"
  restore
  arm "and the archive passes again" 0 "1 archived history"
  sub docs/reviews/INDEX.md 'open: round 2 next' 'closed: round 2 found no defect'; commit
  git -C "$work" checkout -q -b side HEAD
  printf 'side\n' > "$work/side.md"; commit
  git -C "$work" checkout -q "$main"
  printf 'main moves\n' > "$work/m.md"; commit
  git -C "$work" -c user.name=t -c user.email=t@t merge -q --no-ff --no-commit side >/dev/null 2>&1
  handseal o-reviews.md "" "$TODAY"
  git -C "$work" add -A; git -C "$work" -c user.name=t -c user.email=t@t commit -qm "an archiving inside a merge"
  arm "an archiving made inside a merge commit is refused" 1 "entered the history through a merge commit"
  rm -rf "$work" && mkdir -p "$work/docs/review-history" && git -C "$work" init -q
  printf '# x\n' > "$work/docs/review-history/INDEX.md"; git -C "$work" add -A
  arm "a failed history read is a breach, never no history" 1 "failed, so the history legs cannot hold"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real review history passes"
  else echo "SELF-TEST: the real review history is refused — run the check to see why" >&2; fi
  echo "review-history self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --seal)
    [ -n "${2:-}" ] || { echo "usage: bash scripts/check_review_history.sh --seal <FILE>" >&2; exit 2; }
    core seal "$2"; exit $? ;;
  "") core gate; exit $? ;;
  *) echo "usage: bash scripts/check_review_history.sh [--seal <FILE> | --self-test]" >&2; exit 2 ;;
esac
