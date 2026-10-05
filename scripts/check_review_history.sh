#!/usr/bin/env bash
# scripts/check_review_history.sh — REVIEW-HISTORY: closed review histories archived out of docs/reviews/ (leaf
# `PROGRAM.55`, docs/decisions/decision_reviews-folder-ceiling.md), `DECISION-HISTORY`'s pattern for whole files.
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
# A STUB is exactly three lines and a final newline: the history's own first line, a blank line, and
#   Archived, byte for byte, in [`<FILE>`](../review-history/<FILE>) — its review closed, and never edited again.
#
# THE SEAL refuses, writing nothing: a name that is not a file directly in docs/reviews/ or is its index; a history
# already a stub; one with a carriage return or no final newline; one whose row in docs/reviews/INDEX.md is not one,
# or whose status does not begin `closed`; and an archived file of that name. It writes the archived file, the stub
# and the row, and rolls everything back if the gate then refuses, for any reason.
#
# THE GATE'S LEGS:
#   0. the repository is not shallow, and every history read succeeds;
#   1. the index begins with its header, byte for byte; every archived file's lines, bytes and sha256 are its row's;
#      no row is listed twice;
#   2. archived files and rows correspond one to one, and nothing else tracked or untracked-and-not-ignored is under
#      docs/review-history/;
#   3. HISTORY-WIDE, with `--full-history`: every row any committed version of the index held is still there,
#      unchanged, and every archived file is byte for byte what the commit that added it wrote;
#   4. every archived file has its stub at docs/reviews/<FILE>, exactly; every stub line in docs/reviews/ is in such a
#      stub; and the history's row in docs/reviews/INDEX.md is one, and its status begins `closed`;
#   5. PROVENANCE: every archived file is, byte for byte, what docs/reviews/<FILE> held just before the commit that
#      archived it (HEAD, for one not yet committed), when it was no stub and its row was already `closed`; its row's
#      date lies between that commit's parent and the archiving commit, in UTC.
#
# ⚠️ HONEST LIMITS:
#   - history rewritten under the gate is premises 2 and 3's, as for the catalog (docs/specs/catalog/
#     decision_catalog-records.md §0); within history, legs 3 and 5 hold whatever HEAD is;
#   - whether a review is truly closed is its leaf's and its index row's; this reads the row, so archiving an open
#     review needs its row written `closed` first, which review of that commit sees;
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
STUB_RE = re.compile(r"^Archived, byte for byte, in \[`([^`]+)`\]\(([^)]+)\) — its review closed, and never edited again\.$")
ROW_RE = re.compile(r"^\| `([A-Za-z0-9_.+-]+\.md)` \| (\d+) \| (\d+) \| `([0-9a-f]{64})` \| `(\d{4}-\d{2}-\d{2})` \|$")
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

def read(path):
    with open(path, "rb") as f:
        return decode(f.read(), path)

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

def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()

def utc_day(commit):
    out = git("log", "-1", "--format=%ct", commit)
    return None if out is None else datetime.datetime.fromtimestamp(int(out.decode().strip()), datetime.timezone.utc).strftime("%Y-%m-%d")

def stub_text(name, first_line):
    return "%s\n\nArchived, byte for byte, in [`%s`](../review-history/%s) — its review closed, and never edited again.\n" % (first_line, name, name)

def statuses(rindex_text):
    """{file name: [status, …]} for every row of docs/reviews/INDEX.md whose first cell links a history."""
    out = {}
    for line in rindex_text.split("\n"):
        m = re.match(r"^\| \[`([^`]+)`\]\(([^)]+)\) \|", line)
        if not m:
            continue
        cells = line.strip().strip("|").split(" | ")
        out.setdefault(m.group(1), []).append(cells[-1].strip())
    return out

def rows(index_text, where):
    out = []
    for n, line in enumerate(index_text.split("\n"), 1):
        if line.startswith("| `"):
            r = ROW_RE.match(line)
            if not r:
                note("%s:%d is not a row of the form `| `FILE.md` | lines | bytes | `sha256` | `date` |`" % (where, n)); continue
            out.append(r.groups())
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

def gate():
    """The legs; returns the number of archived files."""
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
    files, dates = {}, {}
    for name, nlines, nbytes, digest, day in index:
        fpath = os.path.join(HIST, name)
        if fpath in files:
            note("%s lists %s twice" % (INDEX, name)); continue
        if not os.path.exists(fpath):
            note("%s lists %s, and %s does not exist" % (INDEX, name, fpath)); continue
        files[fpath], dates[fpath] = name, day
        data = read(fpath)
        if str(data.count("\n")) != nlines: note("%s has %d lines, and its row says %s — an archived file changed" % (fpath, data.count("\n"), nlines))
        if str(len(data.encode("utf-8"))) != nbytes: note("%s has %d bytes, and its row says %s — an archived file changed" % (fpath, len(data.encode("utf-8")), nbytes))
        if sha(data) != digest: note("%s's sha256 is not its row's — an archived file changed" % fpath)
    # Leg 2.
    for p in sorted(x for x in history("ls-files", "-co", "--exclude-standard", "--", HIST).split("\n") if x):
        if p != INDEX and p not in files:
            note("%s is neither the index nor an archived file listed in it" % p)
    # Leg 3.
    current = set(index)
    for commit in history("log", "--full-history", "--format=%H", "--", INDEX).split():
        old = at(commit, INDEX)
        if old is None:
            continue
        for r in rows(old, "%s:%s" % (commit[:12], INDEX)):
            if r not in current:
                note("%s's row for %s, committed in %s, is gone or changed — the index is append-only" % (INDEX, r[0], commit[:12]))
    added, commit = {}, None
    for line in history("log", "--full-history", "--diff-filter=A", "--format=@%H", "--name-only", "--", HIST).split("\n"):
        if line.startswith("@"):
            commit = line[1:]
        elif line.strip():
            added[line.strip()] = commit  # newest first, so the one kept is the oldest
    for fpath in files:
        c = added.get(fpath)
        if c and at(c, fpath) != read(fpath):
            note("%s is not what %s wrote when it archived it — an archived file changed" % (fpath, c[:12]))
    # Leg 4.
    status = statuses(read(RINDEX)) if os.path.exists(RINDEX) else {}
    for fpath, name in files.items():
        spath = os.path.join(REVIEWS, name)
        if not os.path.exists(spath):
            note("%s is archived and has no stub at %s" % (fpath, spath))
        elif read(spath) != stub_text(name, read(fpath).split("\n")[0]):
            note("%s is not exactly the stub of %s: its first line, a blank line and the line linking it" % (spath, fpath))
        st = status.get(name, [])
        if len(st) != 1:
            note("%s has %d rows in %s, and an archived history has one" % (name, len(st), RINDEX))
        elif not st[0].startswith("closed"):
            note("%s's row in %s says %r, and only a closed review's history is archived" % (name, RINDEX, st[0][:40]))
    for d, _, names in os.walk(REVIEWS):
        for n in sorted(names):
            p = os.path.join(d, n)
            if not n.endswith(".md"):
                continue
            text = read(p)
            if any(STUB_RE.match(l) for l in text.split("\n")):
                if d != REVIEWS or os.path.join(HIST, n) not in files:
                    note("%s holds a stub line, and no archived file of its name is listed" % p)
    # Leg 5.
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    for fpath, name in files.items():
        c = added.get(fpath)
        base = (c + "^") if c else "HEAD"
        before = at(base, os.path.join(REVIEWS, name))
        if before is None:
            note("%s: %s held no %s/%s to archive" % (fpath, base, REVIEWS, name)); continue
        if any(STUB_RE.match(l) for l in before.split("\n")):
            note("%s: %s/%s was already a stub in %s" % (fpath, REVIEWS, name, base)); continue
        if before != read(fpath):
            note("%s is not what %s/%s held in %s — a history was edited on its way in" % (fpath, REVIEWS, name, base))
        was = statuses(at(base, RINDEX) or "").get(name, [])
        if len(was) != 1 or not was[0].startswith("closed"):
            note("%s: its review was not closed in %s's %s, so it was archived open" % (fpath, base, RINDEX))
        low, high = utc_day(base), (utc_day(c) if c else today)
        if low is None or high is None or not (low <= dates[fpath] <= high):
            note("%s's row is dated %s, outside the days its archiving could have been made (%s to %s, UTC)" % (fpath, dates[fpath], low, high))
    return len(files)

def seal(name):
    if os.sep in name or not NAME_RE.match(name) or name == "INDEX.md" or not os.path.isfile(os.path.join(REVIEWS, name)):
        sys.exit("review-history: %s is not a review history directly under %s; nothing was written" % (name, REVIEWS))
    spath, fpath = os.path.join(REVIEWS, name), os.path.join(HIST, name)
    before = read(spath)
    if "\r" in before or not before.endswith("\n"):
        sys.exit("review-history: %s holds a carriage return or lacks a final newline; nothing was written" % spath)
    if any(STUB_RE.match(l) for l in before.split("\n")):
        sys.exit("review-history: %s is already a stub; nothing was written" % spath)
    st = statuses(read(RINDEX)).get(name, []) if os.path.exists(RINDEX) else []
    if len(st) != 1 or not st[0].startswith("closed"):
        sys.exit("review-history: %s's row in %s is not one row whose status begins `closed`; nothing was written" % (name, RINDEX))
    if os.path.exists(fpath):
        sys.exit("review-history: %s exists; nothing was written" % fpath)
    archived = before if FAULT != "drop-byte" else before[:-2] + "\n"
    if archived != before:
        sys.exit("review-history: the archived copy of %s would not be byte for byte; nothing was written" % spath)
    index_before = read(INDEX) if os.path.exists(INDEX) else None
    made_dir = not os.path.isdir(HIST)
    os.makedirs(HIST, exist_ok=True)
    today = datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%d")
    wrote = False
    try:
        with open(fpath, "w", encoding="utf-8") as fh:
            fh.write(archived)
        wrote = True
        with open(spath, "w", encoding="utf-8") as fh:
            fh.write(stub_text(name, before.split("\n")[0]))
        text = index_before if index_before is not None else HEADER
        text = text.rstrip("\n") + "\n" + "| `%s` | %d | %d | `%s` | `%s` |\n" % (
            name, archived.count("\n"), len(archived.encode("utf-8")), sha(archived), today)
        with open(INDEX, "w", encoding="utf-8") as fh:
            fh.write(text)
        gate()
    except Unreadable as e:
        note(str(e))
    if fails:
        if wrote and os.path.exists(fpath):
            os.remove(fpath)
        with open(spath, "w", encoding="utf-8") as fh:
            fh.write(before)
        if index_before is None:
            if os.path.exists(INDEX):
                os.remove(INDEX)
        else:
            with open(INDEX, "w", encoding="utf-8") as fh:
                fh.write(index_before)
        if made_dir and os.path.isdir(HIST) and not os.listdir(HIST):
            os.rmdir(HIST)
        sys.exit("review-history: the gate refused the archiving of %s, so it was rolled back:\n  " % spath + "\n  ".join(fails))
    print("review-history: %s — archived, %d bytes, byte for byte; a stub left at its path" % (spath, len(archived.encode("utf-8"))))

try:
    if mode == "seal":
        seal(args[0])
        fails.clear()
    n = gate()
except Unreadable as e:
    note(str(e))
    n = 0
for msg in fails:
    print("REVIEW-HISTORY: " + msg, file=sys.stderr)
if fails:
    print("REVIEW-HISTORY: %d breach(es) — PROGRAM.55, docs/decisions/decision_reviews-folder-ceiling.md" % len(fails), file=sys.stderr)
    sys.exit(1)
print("review-history: OK (%d archived history(ies), each against its row, its stub and its closed review; the index append-only across history; every archived file proven against its history before archiving)" % n)
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
  unchanged() {
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
  handseal() { # $1 = file name, $2 = the body to archive ("" for the history itself), $3 = the row's date
    python3 - "$work" "$1" "$2" "$3" <<'PY'
import hashlib, os, sys
w, name, body, day = sys.argv[1:5]
src = os.path.join(w, "docs/reviews", name)
t = open(src).read()
body = body if body else t
os.makedirs(os.path.join(w, "docs/review-history"), exist_ok=True)
open(os.path.join(w, "docs/review-history", name), "w").write(body)
open(src, "w").write("%s\n\nArchived, byte for byte, in [`%s`](../review-history/%s) — its review closed, and never edited again.\n" % (t.split("\n")[0], name, name))
i = os.path.join(w, "docs/review-history/INDEX.md")
open(i, "a").write("| `%s` | %d | %d | `%s` | `%s` |\n" % (name, body.count("\n"), len(body.encode()), hashlib.sha256(body.encode()).hexdigest(), day))
PY
  }
  reseal_row() { # re-derive c-reviews.md's row from its file, as a forger would
    python3 - "$work" <<'PY'
import hashlib, re, sys
w = sys.argv[1]
s = open(w + "/docs/review-history/c-reviews.md").read()
i = w + "/docs/review-history/INDEX.md"
t = re.sub(r"^\| `c-reviews\.md` \| \d+ \| \d+ \| `[0-9a-f]+` \|", "| `c-reviews.md` | %d | %d | `%s` |" % (s.count("\n"), len(s.encode()), hashlib.sha256(s.encode()).hexdigest()), open(i).read(), flags=re.M)
open(i, "w").write(t)
PY
  }
  local TODAY out
  TODAY="$(date -u +%Y-%m-%d)"

  fresh
  arm "a folder with nothing archived passes" 0 "0 archived history"
  arm "archiving a file that does not exist writes nothing" 1 "is not a review history directly under" --seal x-reviews.md
  arm "archiving the reviews index is refused" 1 "is not a review history directly under" --seal INDEX.md
  arm "archiving by a path is refused" 1 "is not a review history directly under" --seal ../reviews/c-reviews.md
  arm "archiving an open review's history is refused" 1 "whose status begins \`closed\`" --seal o-reviews.md
  unchanged "and the refused archiving wrote nothing"
  REVIEW_HISTORY_SELFTEST_FAULT=drop-byte arm "an archived copy that would not be byte for byte writes nothing" 1 "would not be byte for byte" --seal c-reviews.md
  unchanged "and the unproven archiving wrote nothing"
  mkdir -p "$work/docs/review-history" && printf 'stray\n' > "$work/docs/review-history/notes.txt"
  arm "an archiving the gate refuses is rolled back" 1 "so it was rolled back" --seal c-reviews.md
  rm -rf "$work/docs/review-history"
  unchanged "and the rolled-back archiving left nothing behind"
  arm "an archiving proves its copy" 0 "byte for byte; a stub left at its path" --seal c-reviews.md
  arm "the archived history passes the gate" 0 "1 archived history"
  grep -qx 'Archived, byte for byte, in \[`c-reviews.md`\](../review-history/c-reviews.md) — its review closed, and never edited again.' "$work/docs/reviews/c-reviews.md" ||
    { arms=$((arms + 1)); echo "SELF-TEST: the stub does not link its archived file" >&2; }
  arm "archiving it again is refused" 1 "is already a stub" --seal c-reviews.md
  commit
  printf 'x' >> "$work/docs/review-history/c-reviews.md"
  arm "a byte added to an archived file is refused against its row" 1 "and its row says"
  restore
  sub docs/review-history/INDEX.md '^\| `c-reviews\.md` \|.*\n' ''
  arm "an archived file with no row is refused" 1 "is neither the index nor an archived file"
  restore
  sub docs/review-history/INDEX.md '^(\| `c-reviews\.md` \|.*\n)' '\1\1'
  arm "a row listed twice is refused" 1 "lists c-reviews.md twice"
  restore
  printf '| `z-reviews.md` | 1 | 1 | `%064d` | `%s` |\n' 0 "$TODAY" >> "$work/docs/review-history/INDEX.md"
  arm "a row whose file does not exist is refused" 1 "does not exist"
  restore
  sub docs/review-history/INDEX.md '^Each file below' 'Any file below may change; each file below'
  arm "an index whose header was rewritten is refused" 1 "does not begin with its header"
  restore
  sub docs/review-history/INDEX.md '^(\| `c-reviews\.md` \|.*\| `)[0-9-]+(` \|)$' '\g<1>1999-01-01\2'
  commit
  arm "a committed row that changed is refused across history" 1 "the index is append-only"
  git -C "$work" reset -q --hard HEAD~1
  sub docs/review-history/c-reviews.md 'found nothing' 'found a defect'
  reseal_row
  commit
  arm "an archived file and its row forged together and committed are refused" 1 "is not what"
  git -C "$work" reset -q --hard HEAD~1
  git -C "$work" checkout -q -b other HEAD~1
  printf 'unrelated\n' > "$work/notes.md"; commit
  git -C "$work" -c user.name=t -c user.email=t@t merge -q -s ours -m "drop the archive" "$main"
  arm "a merge that keeps only the other side cannot hide an archiving" 1 "the index is append-only"
  git -C "$work" checkout -q "$main"; git -C "$work" branch -q -D other
  git clone -q --depth 1 "file://$work" "$work-shallow" 2>/dev/null
  arms=$((arms + 1))
  if out="$(cd "$work-shallow" && bash "$SELF" 2>&1)"; then echo "SELF-TEST: a shallow clone passed" >&2
  elif printf '%s' "$out" | grep -qF "the repository is shallow"; then ok=$((ok + 1)); echo "  ✅ a shallow clone is refused"
  else echo "SELF-TEST: a shallow clone was refused for another reason:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; fi
  rm -rf "$work-shallow"
  printf 'noise\n' > "$work/docs/review-history/.DS_Store"
  arm "an ignored file in the archive is not seen" 0 "1 archived history"
  printf 'stray\n' > "$work/docs/review-history/notes.txt"
  arm "a stray file in the archive is refused" 1 "is neither the index nor an archived file"
  restore
  printf 'One more line.\n' >> "$work/docs/reviews/c-reviews.md"
  arm "a stub holding more than its three lines is refused" 1 "is not exactly the stub"
  restore
  cp "$work/docs/review-history/c-reviews.md" "$work/docs/reviews/c-reviews.md"
  arm "an archived history made live again is refused" 1 "is not exactly the stub"
  restore
  git -C "$work" rm -q docs/reviews/c-reviews.md
  arm "an archived history whose stub was deleted is refused" 1 "has no stub"
  git -C "$work" reset -q --hard HEAD
  sub docs/reviews/c-reviews.md '\(\.\./review-history/c-reviews\.md\)' '(review-history/c-reviews.md)'
  arm "a stub whose link does not resolve is refused" 1 "is not exactly the stub"
  restore
  printf '# Another\n\nArchived, byte for byte, in [`q-reviews.md`](../review-history/q-reviews.md) — its review closed, and never edited again.\n' > "$work/docs/reviews/q-reviews.md"
  arm "a stub with no archived file is refused" 1 "no archived file of its name is listed"
  restore
  sub docs/reviews/INDEX.md 'closed: round 1 found no defect' 'open: reopened'
  arm "an archived history whose review was reopened is refused" 1 "only a closed review's history is archived"
  restore
  printf '| [`c-reviews.md`](c-reviews.md) | `c` | 2 | closed: again |\n' >> "$work/docs/reviews/INDEX.md"
  arm "an archived history with two rows in the reviews index is refused" 1 "has 2 rows in"
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
  git -C "$work" reset -q --hard HEAD~1
  printf 'Windows\r\n' >> "$work/docs/reviews/o-reviews.md"
  sub docs/reviews/INDEX.md 'open: round 2 next' 'closed: round 2 found no defect'
  arm "a history with a carriage return is refused" 1 "carriage return" --seal o-reviews.md
  restore
  arm "and the archive passes again" 0 "1 archived history"
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
