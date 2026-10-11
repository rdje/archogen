#!/usr/bin/env bash
# scripts/check_review_counts.sh — REVIEW-COUNTS: for each design review the reviews index lists as counted, the
# record's count of what each round found, and the index's count of rounds, are its review history's (leaf `PROGRAM.69`,
# review rounds 23 to 34).
#
# ⭐ WHY. A design record keeps one summary row per review round, with the count of what the round found, and the
# round's own table in its review history lists each finding. The count was copied by hand, and the task-tree sealing
# record's count disagreed with its history four times (reviews R19 D1, R21 D1, R22 D1, R23 D1); the index's count of
# rounds slipped once (R17). Derived and checked, they cannot drift.
#
# ⭐ WHICH REVIEWS: the ones docs/reviews/INDEX.md lists under its heading `## Counted by `REVIEW-COUNTS``, each a line
# "- `<history>` with `<record>`", both paths from docs/reviews/ — kept with the index every review's answer edits, so a
# review in any tree joins the list with no change to this script. An index with no such heading, a line under it
# opening "- " that is no such entry, and a review listed twice are refused. Nothing is derived and nothing skipped: a
# listed review the check cannot judge whole — a file missing, no round line, a round with no well-formed findings
# table, no summary table of either shape, not one index row — is refused, and so is one whose index row's status does
# not begin `open`: a review leaves the list in the commit that closes it (rounds 24 to 28 found every derived or
# partial scope leaving a review, or a round, unread). THE LIST'S RULE: every open review whose record keeps a ROUND
# TABLE — a line opening `| Round |`, of any shape — is listed from its first round. An index row whose status begins
# `open` is refused unless its history is listed when its record cell links a record holding a round table and its
# history holds a round line or that table a row; refused too: such a row whose history cell links no file, whose
# record link resolves to nothing, or whose history is missing. A record cell that links no file names no record.
#
# WHAT IT READS — the staged copy of each file (`git show :<path>`), the working tree's for one independently proved
# absent from the index by a successful `git ls-files -z` census and a byte-for-byte path comparison. A failed proof
# or an unreadable present staged entry refuses rather than substituting the disk copy (round 32):
#   - in docs/reviews/INDEX.md, the counted list, and its four-column table headed Review history, Design record,
#     Rounds, Status, through the first blank line; indentation and optional edge pipes accepted, escaped pipes kept
#     inside a cell, every row required to have four cells. In each file cell the first unescaped bracketed label
#     must be an inline file link: a balanced label (backticks permitted), a nonempty bare destination with no spaces,
#     parentheses or angle brackets, or an angle-delimited destination (spaces/parentheses permitted), and an optional
#     space-separated title in double quotes, single quotes or non-nesting parentheses. Paths are read literally,
#     without Markdown/entity/percent decoding. Unsupported first bracketed syntax, including reference/shortcut
#     links, refuses before a secondary link can substitute; a cell with no unescaped opening bracket names no file;
#   - in a listed review's history, its ROUNDS — each line opening `**Round <n>**` — and each round's FINDINGS TABLE:
#     the first table after its line and before the next round's, which must be headed exactly
#     `| # | Kind | Finding | Answer |`, its next line a separator row of dashes, and its finding rows to the first line
#     that is no table row; each finding row an id and a kind, and none a row of dashes — `—` as its id marking a
#     finding of the answer's own, which is not the review's;
#   - in a listed review's record, its summary table: the header `| Round | Findings | Defects | …` (THE FINDINGS SHAPE)
#     or `| Round | Date | Defects | …` (THE DATE SHAPE), a separator row of dashes, and every row under it to the first
#     line that is no table row.
#
# All three tables split cells on unescaped pipes; a Markdown-escaped pipe is part of its cell, including in a
# summary's defects cell, so it cannot hide a later count (round 30).
#
# THE RULE: every round of a listed review has one line, one well-formed findings table and one summary row; every row
# under a summary header names a round by a number, of the digits 0 to 9; a summary row for a round with no line is
# refused. For each round, in either shape:
#   - the defects cell begins with its number of findings whose kind begins `defect`;
#   - wherever they stand in the defects cell, "N arm gap(s)" is its findings whose kind begins `arm gap`, "N
#     pre-existing arm gap(s)" those whose kind is exactly `pre-existing arm gap`, "N remark(s)" those whose kind begins
#     `remark`; every occurrence is judged, as is each explicit "N defect(s)"; an unstated count is not judged;
#   - THE FINDINGS SHAPE: the findings cell is its number of findings.
# The index row's Rounds cell is the history's last round.
#
# MODES: (none) the gate — exit 0 clean, 1 each disagreement named · --self-test.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

check() {
  python3 -I - <<'PY'
import collections, os, re, subprocess, sys
HEAD = "| # | Kind | Finding | Answer |"
SEPARATOR = re.compile(r"^\|( *:?-+:? *\|)+$")
NUMBER = re.compile(r"[0-9]+")
def read(path):
    r = subprocess.run(["git", "show", ":" + path], capture_output=True, text=True)
    if r.returncode == 0:
        return r.stdout
    indexed = subprocess.run(["git", "ls-files", "--full-name", "-z"], capture_output=True)
    if indexed.returncode != 0:
        sys.exit("REVIEW-COUNTS: %s: index absence cannot be established after a failed staged read" % path)
    if os.fsencode(path) in indexed.stdout.split(b"\0"):
        sys.exit("REVIEW-COUNTS: %s: staged copy cannot be read; the path is present in the index" % path)
    return open(path, encoding="utf-8").read() if os.path.isfile(path) else None
bad = []
def cells_of(line):
    """Markdown table cells: edge pipes optional, a pipe escaped by an odd run of backslashes kept."""
    line = line.strip()
    cells, chars, escaped = [], [], False
    for ch in line:
        if ch == "|" and not escaped:
            cells.append("".join(chars).strip()); chars = []
        else:
            chars.append(ch)
        escaped = ch == "\\" and not escaped
    cells.append("".join(chars).strip())
    if line.startswith("|"):
        cells.pop(0)
    if line.endswith("|") and cells[-1] == "":
        cells.pop()
    return cells
def rounds(text, hist):
    """Each round with a line: its findings' kinds, or None where it has no well-formed findings table."""
    out = {}
    parts = re.split(r"(?m)^\*\*Round (\d+)\*\*", text)
    for i in range(1, len(parts), 2):
        n, lines, kinds = int(parts[i]), parts[i + 1].split("\n"), None
        if n in out:
            bad.append("%s: round %d has two lines" % (hist, n))
        first = next((j for j, l in enumerate(lines) if l.startswith("|")), None)
        if first is not None and lines[first].rstrip() == HEAD:
            k = first + 1
            if k >= len(lines) or not SEPARATOR.match(lines[k].strip()):
                bad.append("%s: round %d's findings table has no separator row under its header" % (hist, n))
            else:
                kinds, k = [], k + 1
                while k < len(lines) and lines[k].startswith("|"):
                    cells = cells_of(lines[k])
                    if SEPARATOR.match(lines[k].strip()):
                        bad.append("%s: round %d's findings table holds a row of dashes" % (hist, n))
                    elif not cells[0]:
                        bad.append("%s: round %d's findings table holds a row with no id" % (hist, n))
                    elif len(cells) < 2 or not cells[1]:
                        bad.append("%s: round %d's findings table holds a row with no kind" % (hist, n))
                    elif cells[0] != "—":
                        kinds.append(cells[1].lower())
                    k += 1
        out[n] = kinds
    return out
def summary(text, rec, quiet=False):
    """The summary table's rows by round, and whether its header was seen."""
    out, shape, seen, after_head = {}, None, False, False
    note = (lambda m: None) if quiet else bad.append
    for l in text.split("\n"):
        if l.startswith("| Round | Findings | Defects |"):
            shape, seen, after_head = "findings", True, True; continue
        if l.startswith("| Round | Date | Defects |"):
            shape, seen, after_head = "date", True, True; continue
        if shape and not l.startswith("|"):
            shape = None
        if not shape:
            continue
        if after_head:
            after_head = False
            if SEPARATOR.match(l.strip()):
                continue
            note("%s: its summary table has no separator row under its header" % rec)
        cells = cells_of(l)
        if len(cells) < 3 or not NUMBER.fullmatch(cells[0]):
            note("%s: a row under its summary header names no round by a number: %r" % (rec, cells[0][:20]))
            continue
        n = int(cells[0])
        if n in out:
            note("%s: round %d has two summary rows" % (rec, n))
        out[n] = (shape, cells[1], cells[2])
    return out, seen
def stated(cell, word):
    return [int(m.group(1)) for m in re.finditer(r"(?<![\w-])([0-9]+) " + word + r"\b", cell)]
def tally(kinds):
    return collections.Counter(
        "pre-existing arm gap" if k == "pre-existing arm gap" else
        "defect" if k.startswith("defect") else
        "arm gap" if k.startswith("arm gap") else
        "remark" if k.startswith("remark") else k for k in kinds)
index = read("docs/reviews/INDEX.md") or ""
LINK = re.compile(
    r"\]\([ \t]*(?:<(?P<angle>[^<>\r\n]+)>|(?P<bare>[^()\s<>]+))"
    r"(?:[ \t]+(?:\"(?:\\.|[^\"\\])*\"|'(?:\\.|[^'\\])*'|\((?:\\.|[^()\\])*\)))?[ \t]*\)")
rows_of_index, table, after_head, seen_index = [], False, False, False
for line in index.split("\n"):
    cells = cells_of(line)
    if cells == ["Review history", "Design record", "Rounds", "Status"]:
        if seen_index:
            bad.append("docs/reviews/INDEX.md: two index table headers")
        table, after_head, seen_index = True, True, True
        continue
    if not table:
        continue
    if after_head:
        after_head = False
        if len(cells) == 4 and all(re.fullmatch(r":?-+:?", c) for c in cells):
            continue
        bad.append("docs/reviews/INDEX.md: its index table has no separator row under its header")
    if not line.strip():
        table = False
        continue
    if len(cells) != 4:
        bad.append("docs/reviews/INDEX.md: index table row has %d cells, not 4" % len(cells))
        continue
    rows_of_index.append(cells)
if after_head:
    bad.append("docs/reviews/INDEX.md: its index table has no separator row under its header")
if not seen_index:
    bad.append("docs/reviews/INDEX.md: no four-column index table header")
link_cache = {}
def linked(cell):
    if cell in link_cache:
        return link_cache[cell]
    # Locate the first unescaped label, not the first destination spelling our grammar happens to recognize.
    i = 0
    while i < len(cell):
        if cell[i] == "\\":
            i += 2
        elif cell[i] == "[":
            break
        else:
            i += 1
    if i >= len(cell):
        link_cache[cell] = None
        return None
    depth, i = 1, i + 1
    while i < len(cell) and depth:
        if cell[i] == "\\":
            i += 2
            continue
        if cell[i] == "[":
            depth += 1
        elif cell[i] == "]":
            depth -= 1
            if not depth:
                break
        i += 1
    m = LINK.match(cell, i) if not depth else None
    if m is None:
        bad.append("docs/reviews/INDEX.md: unsupported first file-link syntax: %r" % cell[:60])
        link_cache[cell] = None
        return None
    path = m.group("angle") or m.group("bare")
    link_cache[cell] = os.path.normpath(os.path.join("docs/reviews", path))
    return link_cache[cell]
# Grammar applies to both file cells, including listed and closed rows; the counted pairs remain authoritative.
for c in rows_of_index:
    linked(c[0]); linked(c[1])
LIST = "## Counted by `REVIEW-COUNTS`"
pairs, inside = [], False
for l in index.split("\n"):
    if l.startswith("## "):
        inside = l.rstrip() == LIST
        continue
    if inside and l.startswith("- "):
        m = re.fullmatch(r"- `([^`/]+-reviews\.md)` with `([^`]+\.md)`", l.rstrip())
        if not m:
            bad.append("docs/reviews/INDEX.md: a line of the counted list that is no entry: %r" % l[:60]); continue
        pairs.append(("docs/reviews/" + m.group(1), os.path.normpath(os.path.join("docs/reviews", m.group(2)))))
if not any(l.rstrip() == LIST for l in index.split("\n")):
    bad.append("docs/reviews/INDEX.md: no heading %s — the list of the reviews this check judges" % LIST)
listed, judged = set(), 0
for hist, rec in pairs:
    if hist in listed:
        bad.append("docs/reviews/INDEX.md: %s is listed twice" % hist); continue
    listed.add(hist)
    h, r = read(hist), read(rec)
    if h is None or r is None:
        bad.append("%s: listed with %s, and %s is missing" % (hist, rec, hist if h is None else rec)); continue
    heads = rounds(h, hist)
    rows, seen = summary(r, rec)
    if not heads:
        bad.append("%s: listed, and it holds no round line" % hist); continue
    if not seen:
        bad.append("%s: listed, and it holds no summary table of either shape" % rec); continue
    row = [c for c in rows_of_index if linked(c[0]) == hist]
    if len(row) != 1:
        bad.append("docs/reviews/INDEX.md: %s has %d row(s), not one" % (hist, len(row)))
    elif len(row[0]) < 4 or not row[0][3].startswith("open"):
        bad.append("docs/reviews/INDEX.md: %s is listed, and its row's status is not open — a review leaves the list in"
                   " the commit that closes it" % hist)
    elif row[0][2] != str(max(heads)):
        bad.append("docs/reviews/INDEX.md: %s's row says %s round(s); its last round is %d" % (hist, row[0][2], max(heads)))
    def say(n, what, said, real):
        bad.append("%s: round %d says %s %s; %s's round %d holds %d" % (rec, n, said, what, hist, n, real))
    for n in sorted(set(heads) | set(rows)):
        if n not in heads:
            bad.append("%s: round %d's summary row names a round %s has no line for" % (rec, n, hist)); continue
        if heads[n] is None:
            bad.append("%s: round %d has no well-formed findings table — every round of a listed review keeps one" % (hist, n)); continue
        if n not in rows:
            bad.append("%s: round %d is in its history and has no summary row" % (rec, n)); continue
        judged += 1
        shape, a, b = rows[n]
        count = tally(heads[n])
        lead = re.match(r"([0-9]+)", b)
        if not lead:
            bad.append("%s: round %d's defects cell begins with no number" % (rec, n))
        elif int(lead.group(1)) != count["defect"]:
            say(n, "defect(s)", lead.group(1), count["defect"])
        if shape == "findings":
            if not NUMBER.fullmatch(a):
                bad.append("%s: round %d's findings cell is not a number" % (rec, n))
            elif int(a) != len(heads[n]):
                say(n, "finding(s)", a, len(heads[n]))
        for word, kind in (("defects?", "defect"), ("arm gaps?", "arm gap"), ("pre-existing arm gaps?", "pre-existing arm gap"),
                           ("remarks?", "remark")):
            for c in stated(b, word):
                if c != count[kind]:
                    say(n, kind + "(s)", c, count[kind])
# The list is every open review whose record keeps a round table — a header opening `| Round |` — from its first round.
for c in rows_of_index:
    if len(c) < 4 or not c[3].startswith("open"):
        continue
    hist = linked(c[0])
    if hist is None:
        bad.append("docs/reviews/INDEX.md: an open row whose history cell links no file: %r" % c[0][:60]); continue
    if hist in listed:
        continue
    rec = linked(c[1])
    if rec is None:
        continue
    r = read(rec)
    if r is None:
        bad.append("docs/reviews/INDEX.md: %s's row links a record that is missing: %s" % (hist, rec)); continue
    rl = r.split("\n")
    heads_at = [i for i, l in enumerate(rl) if l.startswith("| Round |")]
    if not heads_at:
        continue
    h = read(hist)
    if h is None:
        bad.append("%s: an open review whose record keeps a round table, and its history is missing" % hist)
    elif any(i + 2 < len(rl) and rl[i + 2].startswith("|") for i in heads_at) or \
            re.search(r"(?m)^\*\*Round [0-9]+\*\*", h):
        bad.append("%s: an open review whose record keeps a round table (`| Round |`), past its first round, and its"
                   " history is not listed" % hist)
for x in bad:
    print("REVIEW-COUNTS: " + x, file=sys.stderr)
if bad:
    sys.exit(1)
print("review-counts: OK (%d review(s) listed; %d round(s) judged, every one)" % (len(listed), judged))
PY
}

self_test() {
  local work="$ROOT/target/doctrine_scratch/review_counts" arms=0 ok=0
  rm -rf "${work:?}"; mkdir -p "$work/docs/reviews" "$work/docs/decisions"
  git -C "$work" init -q
  INDEX_A='| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open |'
  world() { # $1 = the findings-shaped row of round 2, $2 = the date-shaped row of round 1, $3 = a's index row
    printf '| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n%s\n%s\n%s\n' "${3:-$INDEX_A}" \
      '| [`b-reviews.md`](b-reviews.md) | §1 of [`b.md`](../decisions/b.md) | 1 | open |' \
      '| [`c-reviews.md`](c-reviews.md) | [`c.md`](../decisions/c.md) | 3 | closed |' > "$work/docs/reviews/INDEX.md"
    printf '\n## Counted by `REVIEW-COUNTS`\n\nThe reviews it judges.\n\n- `a-reviews.md` with `../decisions/a.md`\n- `b-reviews.md` with `../decisions/b.md`\n\n## After\n\n- `c-reviews.md` with `../decisions/c.md`\n' >> "$work/docs/reviews/INDEX.md"
    # a: round 1 one finding; round 2 one defect, two arm gaps, one pre-existing arm gap, one pre-existing finding, a
    # remark, an answer's own row — then rows past a blank line and a second table of the same header, neither read.
    printf '# A\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n| R2-AG1 | arm gap | x | y |\n| R2-AG2 | arm gap | x | y |\n| R2-R1 | remark | x | y |\n| R2-P1 | pre-existing arm gap | x | y |\n| R2-P2 | pre-existing | x | y |\n| — | found while answering | x | y |\n\n| R2-D9 | defect | past the table | y |\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D8 | defect | a second table | y |\n' > "$work/docs/reviews/a-reviews.md"
    printf '# A\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 1 | 1 (x) | v |\n%s\n\n| 9 | 7 | 7 | after the table |\n' "$1" > "$work/docs/decisions/a.md"
    printf '# B\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n| R1-D2 | defect | x | y |\n| R1-R1 | remark, not this change'"'"'s | x | y |\n| R1-P1 | pre-existing arm gaps, another leaf'"'"'s | x | y |\n' > "$work/docs/reviews/b-reviews.md"
    printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n%s\n' "$2" > "$work/docs/decisions/b.md"
    # c: closed, its record keeping a table with wrong counts — no open review, so not the list's.
    printf '# C\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n' > "$work/docs/reviews/c-reviews.md"
    printf '# C\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 99 | 99 | v |\n' > "$work/docs/decisions/c.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local out rc; arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -eq "$2" ] && printf '%s' "$out" | grep -qF -- "$3"; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — rc $rc:" >&2; printf '%s\n' "$out" | head -3 | sed 's/^/    /' >&2; fi
  }
  GOOD_A='| 2 | 6 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |'
  GOOD_B='| 1 | `2026-10-10` | 2, and 1 remark, and 0 pre-existing arm gaps | o |'
  world "$GOOD_A" "$GOOD_B"
  arm "counts that are the histories' pass: the first table only, to its end; an answer's own row and a row past the summary table not judged; a remark of any kind beginning so; a closed review's table not the list's" 0 "2 review(s) listed; 3 round(s) judged, every one"
  world '| 2 | 7 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a total off by one is refused" 1 "round 2 says 7 finding(s)"
  world '| 2 | 6 | 2, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a defect count off is refused" 1 "round 2 says 2 defect(s)"
  world '| 2 | 6 | 1, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "an arm-gap count off, equal to the pre-existing one, is refused (R26 AG1)" 1 "round 2 says 1 arm gap(s)"
  world '| 2 | 6 | 1, and 1 pre-existing arm gap (y), and 3 arm gaps (x) | v |' "$GOOD_B"
  arm "an arm-gap count after the pre-existing clause is read (R24 AG2)" 1 "round 2 says 3 arm gap(s)"
  world '| 2 | 6 | 1, and 2 arm gaps (x), and 2 pre-existing arm gaps (y) | v |' "$GOOD_B"
  arm "a pre-existing arm-gap count counts that kind exactly, a pre-existing finding apart (R26 AG1, R27 AG1)" 1 "round 2 says 2 pre-existing arm gap(s)"
  world '| 2 | five | 1, and 2 arm gaps (x) | v |' "$GOOD_B"
  arm "a findings cell that is no number is refused" 1 "round 2's findings cell is not a number"
  world '| 2 | ⁶ | 1, and 2 arm gaps (x) | v |' "$GOOD_B"
  arm "a findings cell of a digit that is not 0 to 9 is refused, never a traceback (R26 R4)" 1 "round 2's findings cell is not a number"
  world '| 2 | 6 | about 1, and 2 arm gaps (x) | v |' "$GOOD_B"
  arm "a defects cell that does not begin with its number is refused (R26 AG4)" 1 "round 2's defects cell begins with no number"
  world "$GOOD_A" '| 1 | `2026-10-10` | 2, and 2 remarks | o |'
  arm "a remark count off is refused" 1 "round 1 says 2 remark(s)"
  world "$GOOD_A" '| 1 | `2026-10-10` | 3, and 1 remark | o |'
  arm "a date-shaped record's defect count off is refused" 1 "round 1 says 3 defect(s)"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n' >> "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a round in the history with no summary row is refused" 1 "round 2 is in its history and has no summary row"
  world "$GOOD_A" "$GOOD_B"; printf '| 0 | `2026-10-10` | 0 | o |\n' >> "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "a summary row for a round the history has no line for is refused (R24 D3)" 1 "round 0's summary row names a round"
  world "$GOOD_A" "$GOOD_B"; printf '| 1 | `2026-10-10` | 2, and 1 remark | again |\n' >> "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "two summary rows for one round are refused (R25 D3)" 1 "round 1 has two summary rows"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 1**, again.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D9 | defect | x | y |\n' >> "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "two lines for one round are refused (R25 D3)" 1 "round 1 has two lines"
  world "$GOOD_A" '| **1** | `2026-10-10` | 2, and 1 remark | o |'
  arm "a row under the header whose round is no number is refused, never skipped (R26 D2)" 1 "names no round by a number"
  world "$GOOD_A" '| --- | 9 | 9, and 9 remarks | o |'
  arm "a row of dashes carrying counts under the header is refused (R27 R1)" 1 "names no round by a number"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak '/^| --- | --- | --- | --- |$/d' "$work/docs/decisions/b.md"; rm -f "$work/docs/decisions/b.md.bak"; git -C "$work" add -A
  arm "a summary table with no separator row under its header is refused (R27 R1)" 1 "its summary table has no separator row"
  world "$GOOD_A" ''
  arm "a summary table with its header and no row refuses each round (R26 D2)" 1 "round 1 is in its history and has no summary row"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| # | Kind | Finding | Answer |$/| Id | Kind | Finding | Answer |/' "$work/docs/reviews/b-reviews.md"; rm -f "$work/docs/reviews/b-reviews.md.bak"; git -C "$work" add -A
  arm "a listed review's round whose table has another header is refused, never left unjudged (R27 D1)" 1 "round 1 has no well-formed findings table"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 2**, launched, not completed.\n' >> "$work/docs/reviews/b-reviews.md"; printf '| 2 | `2026-10-10` | 0 | o |\n' >> "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "a listed review's round with no findings table is refused, in the second review listed (R27 AG2)" 1 "b-reviews.md: round 2 has no well-formed findings table"
  world "$GOOD_A" "$GOOD_B"; awk '{ print } /^\| # \| Kind \| Finding \| Answer \|$/ { getline; next }' "$work/docs/reviews/b-reviews.md" > "$work/b.tmp"; mv "$work/b.tmp" "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a findings table with no separator row is refused, its first row never dropped (R27 R1)" 1 "round 1's findings table has no separator row"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| R1-D2 | defect |/|  | defect |/' "$work/docs/reviews/b-reviews.md"; rm -f "$work/docs/reviews/b-reviews.md.bak"; git -C "$work" add -A
  arm "a finding row with no id is refused, never counted (R27 R1)" 1 "holds a row with no id"
  world "$GOOD_A" "$GOOD_B"; rm "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a listed review whose history is missing is refused (R26 D1)" 1 "b-reviews.md is missing"
  world "$GOOD_A" "$GOOD_B"; rm "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "a listed review whose record is missing is refused, never a traceback (R27 R2)" 1 "docs/decisions/b.md is missing"
  world "$GOOD_A" "$GOOD_B"; printf '# B\n\nNo round yet.\n' > "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a listed review whose history holds no round line is refused (R26 AG2)" 1 "listed, and it holds no round line"
  world "$GOOD_A" "$GOOD_B"; printf '# B\n\nNo table.\n' > "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "a listed review whose record holds no summary table is refused (R26 AG2)" 1 "listed, and it holds no summary table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak '/^| \[`b-reviews.md`\]/d' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a listed review with no index row is refused (R26 D1)" 1 "b-reviews.md has 0 row(s), not one"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak '/^| \[`b-reviews.md`\]/p' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a listed review with two index rows is refused (R27 R2)" 1 "b-reviews.md has 2 row(s), not one"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak '/^## Counted by/d' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an index with no counted list is refused, never read as listing nothing" 1 "no heading ## Counted by"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^- `b-reviews.md` with /- `b-reviews.md` and /' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a line of the counted list that is no entry is refused, never skipped" 1 "a line of the counted list that is no entry"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^- `b-reviews.md` with `..\/decisions\/b.md`$/&\n&/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a review listed twice is refused" 1 "b-reviews.md is listed twice"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 3 | open |'
  arm "an index row's count of rounds off is refused (R24 D2)" 1 "a-reviews.md's row says 3 round(s)"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 3 | closed |/| 3 | open |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open review whose record keeps a summary table, unlisted, is refused (R27 AG3)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 3 | closed |/| 0 | open |/' "$work/docs/reviews/INDEX.md"; printf '# C\n\nNo round yet.\n' > "$work/docs/reviews/c-reviews.md"; printf '# C\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n' > "$work/docs/decisions/c.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open review before its first round, its summary table empty, need not be listed" 0 "2 review(s) listed; 3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 3 | closed |/| 0 | open |/' "$work/docs/reviews/INDEX.md"; printf '# C\n\nNo round yet.\n' > "$work/docs/reviews/c-reviews.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open review with no round line and a summary row, unlisted, is refused" 1 "c-reviews.md: an open review whose record keeps a round table"
  open_c() { sed -i.bak 's/| 3 | closed |/| 3 | open |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; }
  world "$GOOD_A" "$GOOD_B"; open_c; sed -i.bak 's/^| \[`c-reviews.md`\](c-reviews.md) |/| [c-reviews.md](c-reviews.md)  |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open row whose history link has no backticks, or more spaces, is read, never dropped (R28 D1)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; open_c; sed -i.bak 's/^| \[`c-reviews.md`\](c-reviews.md) |/| c-reviews.md |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open row whose history cell links no file is refused (R28 D1)" 1 "an open row whose history cell links no file"
  world "$GOOD_A" "$GOOD_B"; open_c; printf '# C\n\n| Round | Findings | What it turned on | Outcome |\n| --- | --- | --- | --- |\n| 1 | 9 | x | y |\n' > "$work/docs/decisions/c.md"; git -C "$work" add -A
  arm "an open review whose record keeps a round table of a third shape, unlisted, is refused (R28 D2)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; open_c; printf '# C\n\nNo table.\n' > "$work/docs/decisions/c.md"; git -C "$work" add -A
  arm "an open review past its first round whose record keeps no round table need not be listed (R28 AGa)" 0 "2 review(s) listed; 3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"; open_c; rm "$work/docs/reviews/c-reviews.md"; git -C "$work" add -A
  arm "an open review whose record keeps a round table and whose history is missing is refused (R28 AGb)" 1 "c-reviews.md: an open review whose record keeps a round table, and its history is missing"
  world "$GOOD_A" "$GOOD_B"; open_c; rm "$work/docs/decisions/c.md"; git -C "$work" add -A
  arm "an open row linking a record that is missing is refused" 1 "links a record that is missing"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | closed: done |'
  arm "a listed review whose row is closed is refused: it leaves the list as it closes (R28 AGc)" 1 "a-reviews.md is listed, and its row's status is not open"
  world "$GOOD_A" '| 1 | `2026-10-10` | 2, and 1 remark, and 1 pre-existing arm gap | o |'
  arm "a pre-existing arm-gap count counts that kind exactly, never a kind that only begins so (R28 D3)" 1 "round 1 says 1 pre-existing arm gap(s)"
  world '| 2 | 6 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y), and 2 remarks | v |' "$GOOD_B"
  arm "a remark count in a findings-shaped row is judged (R28 Rb)" 1 "round 2 says 2 remark(s)"
  world "$GOOD_A" '| 1 | `2026-10-10` | 2, and 1 remark, and 5 arm gaps | o |'
  arm "an arm-gap count in a date-shaped row is judged (R28 Rb)" 1 "round 1 says 5 arm gap(s)"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| --- | --- | --- | --- |$/&\n&/' "$work/docs/reviews/b-reviews.md"; rm -f "$work/docs/reviews/b-reviews.md.bak"; git -C "$work" add -A
  arm "a row of dashes in a findings table is refused, never counted (R28 D5)" 1 "round 1's findings table holds a row of dashes"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| R1-D2 | defect |/| R1-D2 |  |/' "$work/docs/reviews/b-reviews.md"; rm -f "$work/docs/reviews/b-reviews.md.bak"; git -C "$work" add -A
  arm "a finding row with no kind is refused, never counted (R28 Rc)" 1 "holds a row with no kind"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^\*\*Round 1\*\*, a table\.$/&\n\n| Id | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D7 | defect | x | y |/' "$work/docs/reviews/b-reviews.md"; rm -f "$work/docs/reviews/b-reviews.md.bak"; git -C "$work" add -A
  arm "a round whose first table is not the findings table is refused, never judged on a later one (R28 Ra)" 1 "b-reviews.md: round 1 has no well-formed findings table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 2 | 6 |/| 2 | 7 |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"
  arm "the staged copy is judged, not the disk's" 0 "3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"; git -C "$work" rm -q --cached docs/decisions/b.md; printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n| 1 | `2026-10-10` | 5, and 1 remark | o |\n' > "$work/docs/decisions/b.md"
  arm "a file not in the index is read from the working tree (R25 AG2)" 1 "round 1 says 5 defect(s)"
  # Round 29: no row disappears because Markdown permits indentation or omits its edge pipes, and a malformed
  # row refuses before the open-review rule. Every stated count is judged, even the second of the same kind.
  world "$GOOD_A" "$GOOD_B"; open_c
  sed -i.bak 's/^| \[`c-reviews.md`\]/ | [`c-reviews.md`]/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an indented open index row is read (R29 D1)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; open_c
  sed -i.bak 's/^| \(.*c-reviews.md.*\) |$/\1/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open index row without edge pipes is read (R29 D1)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; open_c
  sed -i.bak 's/| 3 | open |/| open |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a short index row is refused, never skipped (R29 D1)" 1 "index table row has 3 cells, not 4"
  world "$GOOD_A" "$GOOD_B"; open_c
  sed -i.bak 's/| 3 | open |/| 3 | open | extra |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an index row with an extra cell is refused (R29 D1)" 1 "index table row has 5 cells, not 4"
  world "$GOOD_A" "$GOOD_B" "${INDEX_A/| open |/| open: a label with \\| an escaped pipe |}"
  arm "an escaped pipe stays inside its index cell (R29 D1)" 0 "3 round(s) judged"
  world "$GOOD_A" "$GOOD_B" " ${INDEX_A#| }"
  arm "a listed index row with indentation and no leading pipe is read (R29 D1)" 0 "3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"
  sed -i.bak '/^| Review history |/d' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a missing index table header is refused (R29 D1)" 1 "no four-column index table header"
  world "$GOOD_A" "$GOOD_B"
  sed -i.bak '/^| Review history |/p' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "two index table headers are refused (R29 D1)" 1 "two index table headers"
  world "$GOOD_A" "$GOOD_B"
  sed -i.bak '/^| --- |/d' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an index table without its separator is refused (R29 D1)" 1 "index table has no separator row"
  local shape kind value count
  for shape in findings date; do
    for kind in 'arm gaps' 'pre-existing arm gaps' remarks defects; do
      if [ "$shape" = findings ]; then
        case "$kind" in 'arm gaps') count=2 ;; *) count=1 ;; esac
        value="| 2 | 6 | 1, and $count $kind, and 9 $kind | v |"
        world "$value" "$GOOD_B"
        arm "the second stated $kind count is judged in a findings row (R29 D2)" 1 "round 2 says 9"
      else
        case "$kind" in remarks) count=1 ;; defects) count=2 ;; *) count=0 ;; esac
        value="| 1 | 2026-10-11 | 2, and $count $kind, and 9 $kind | o |"
        world "$GOOD_A" "$value"
        arm "the second stated $kind count is judged in a date row (R29 D2)" 1 "round 1 says 9"
      fi
    done
  done
  world '| 2 | 6 | 1, and 2 arm gaps, and 2 arm gaps, and 1 pre-existing arm gap, and 1 pre-existing arm gap, and 1 remark, and 1 remark, and 1 defect, and 1 defect | v |' "$GOOD_B"
  arm "repeated correct counts are all judged and pass (R29 D2)" 0 "3 round(s) judged"
  # Round 30: a Markdown-escaped pipe is part of a summary cell, not its end. A false count after it still refuses.
  for shape in findings date; do
    for kind in 'arm gaps' 'pre-existing arm gaps' remarks defects; do
      if [ "$shape" = findings ]; then
        case "$kind" in 'arm gaps') count=2 ;; *) count=1 ;; esac
        world "| 2 | 6 | 1, and $count $kind (x\|y), and 9 $kind | v |" "$GOOD_B"
        arm "a false $kind count after an escaped pipe in a findings row refuses (R30 D1)" 1 "round 2 says 9"
      else
        case "$kind" in remarks) count=1 ;; defects) count=2 ;; *) count=0 ;; esac
        world "$GOOD_A" "| 1 | 2026-10-11 | 2, and $count $kind (x\|y), and 9 $kind | o |"
        arm "a false $kind count after an escaped pipe in a date row refuses (R30 D1)" 1 "round 1 says 9"
      fi
    done
  done
  world '| 2 | 6 | 1, and 2 arm gaps (x\|y), and 1 pre-existing arm gap, and 1 remark | v |' \
    '| 1 | 2026-10-11 | 2, and 1 remark (x\|y), and 0 pre-existing arm gaps | o |'
  arm "correct counts beyond escaped pipes in both shapes pass (R30 D1)" 0 "3 round(s) judged"
  # Round 31: an unlisted row names its record by the first link; explanatory secondary links name no record.
  world "$GOOD_A" "$GOOD_B"; open_c
  printf '# C, no round table\n' > "$work/docs/decisions/c.md"
  cp "$work/docs/decisions/b.md" "$work/docs/decisions/explanation.md"
  sed -i.bak 's@](../decisions/c.md)@](../decisions/c.md); see [explanation](../decisions/explanation.md)@' "$work/docs/reviews/INDEX.md"
  rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a secondary explanatory table does not make the primary record counted (R31 D1)" 0 "3 round(s) judged"
  rm "$work/docs/decisions/explanation.md"; git -C "$work" add -A
  arm "a secondary missing link is not an unreadable primary record (R31 D1)" 0 "3 round(s) judged"
  # The first record's round table still requires listing, even when the secondary record has none.
  cp "$work/docs/decisions/b.md" "$work/docs/decisions/c.md"
  printf '# Explanation, no round table\n' > "$work/docs/decisions/explanation.md"; git -C "$work" add -A
  arm "the primary record's table is judged before secondary prose (R31 D1)" 1 "c-reviews.md: an open review whose record keeps a round table"
  # Round 33: the first file link keeps authority across destination/title spellings. Unsupported first syntax
  # refuses, never selecting a later explanatory link. Only synthetic fixture text is used.
  c_cell() {
    python3 - "$work/docs/reviews/INDEX.md" "$1" <<'PY_CELL'
import pathlib, sys
p = pathlib.Path(sys.argv[1])
s = p.read_text()
s = s.replace('| [`c-reviews.md`](c-reviews.md) | [`c.md`](../decisions/c.md) |',
              '| [`c-reviews.md`](c-reviews.md) | ' + sys.argv[2] + ' |')
p.write_text(s)
PY_CELL
  }
  local destination title primary finding
  for destination in '../decisions/c.md' '<../decisions/c.md>' '<../decisions/c space.md>' '<../decisions/c(foo).md>'; do
    finding='R33 D1'
    if [ "$destination" = '<../decisions/c(foo).md>' ]; then finding='R34 AG2'; fi
    for title in '' ' "primary title"' " 'primary title'" ' (primary title)'; do
      for primary in table plain; do
        world "$GOOD_A" "$GOOD_B"; open_c
        if [ "$primary" = table ]; then
          printf '# Explanation, no round table\n' > "$work/docs/decisions/explanation.md"
        else
          cp "$work/docs/decisions/c.md" "$work/docs/decisions/explanation.md"
          printf '# C, no round table\n' > "$work/docs/decisions/c.md"
        fi
        cp "$work/docs/decisions/c.md" "$work/docs/decisions/c space.md"
        cp "$work/docs/decisions/c.md" "$work/docs/decisions/c(foo).md"
        c_cell "[c]($destination$title); see [explanation](../decisions/explanation.md)"
        git -C "$work" add -A
        if [ "$primary" = table ]; then
          arm "the first $destination$title table requires counting before plain secondary prose ($finding)" 1 "c-reviews.md: an open review whose record keeps a round table"
        else
          arm "the first $destination$title plain record ignores a secondary table ($finding)" 0 "3 round(s) judged"
        fi
      done
    done
  done
  world "$GOOD_A" "$GOOD_B" "${INDEX_A/](a-reviews.md)/](<a-reviews.md> \"primary title\")}"
  arm "a listed history's angle destination and title identify its row (R33 D1)" 0 "3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"
  mv "$work/docs/reviews/a-reviews.md" "$work/docs/reviews/a space-reviews.md"
  python3 - "$work/docs/reviews/INDEX.md" <<'PY_HISTORY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text()
s = s.replace('](a-reviews.md)', '](<a space-reviews.md> "primary title")')
s = s.replace('- `a-reviews.md` with ', '- `a space-reviews.md` with ')
p.write_text(s)
PY_HISTORY
  git -C "$work" add -A
  arm "a listed history's spaced angle destination agrees with the counted list (R33 D1)" 0 "3 round(s) judged"
  for primary in '[c](../decisions/c(foo).md)' '[c](<../decisions/c.md)' \
    '[c](../decisions/c.md unquoted-title)' '[c]()' '[c][primary]' '[c]'; do
    world "$GOOD_A" "$GOOD_B"; open_c
    printf '# Explanation, no round table\n' > "$work/docs/decisions/explanation.md"
    c_cell "$primary; see [explanation](../decisions/explanation.md)"; git -C "$work" add -A
    arm "unsupported first syntax $primary refuses instead of selecting the explanation (R33 D1)" 1 "unsupported first file-link syntax"
  done
  world "$GOOD_A" "$GOOD_B"; open_c
  c_cell '[c [nested]](../decisions/c.md)'; git -C "$work" add -A
  arm "a balanced nested label retains its primary destination (R33 D1)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; open_c
  c_cell '\[literal note] before [c](../decisions/c.md)'; git -C "$work" add -A
  arm "an escaped opening bracket is prose before the first link (R33 D1)" 1 "c-reviews.md: an open review whose record keeps a round table"
  world "$GOOD_A" "$GOOD_B"; open_c
  c_cell 'a record described without a link'; git -C "$work" add -A
  arm "a bracket-free record cell still names no file (R33 D1)" 0 "3 round(s) judged"
  # Round 34: grammar validation is unconditional, including record cells the counting paths never consult.
  world "$GOOD_A" "$GOOD_B"
  python3 - "$work/docs/reviews/INDEX.md" <<'PY_LISTED'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text()
old = '| [`a.md`](../decisions/a.md) |'
assert s.count(old) == 1
p.write_text(s.replace(old, '| [a] |'))
PY_LISTED
  git -C "$work" add -A
  arm "a listed record's unsupported shortcut refuses despite its authoritative counted pair (R34 AG1)" 1 "unsupported first file-link syntax"
  world "$GOOD_A" "$GOOD_B"; c_cell '[closed]'; git -C "$work" add -A
  arm "a closed record's unsupported shortcut still refuses (R34 AG1)" 1 "unsupported first file-link syntax"
  world "$GOOD_A" "$GOOD_B"
  python3 - "$work/docs/reviews/INDEX.md" <<'PY_CLOSED'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text().replace('| open |', '| closed |')
p.write_text('\n'.join(l for l in s.split('\n') if not l.startswith('- `')))
PY_CLOSED
  git -C "$work" add -A
  arm "valid all-closed rows permit an empty counted list (R34 AG1)" 0 "0 review(s) listed; 0 round(s) judged"
  python3 - "$work/docs/reviews/INDEX.md" <<'PY_CLOSED_HISTORY'
import pathlib, sys
p = pathlib.Path(sys.argv[1]); s = p.read_text()
old = '[`c-reviews.md`](c-reviews.md)'
assert s.count(old) == 1
p.write_text(s.replace(old, '[closed]'))
PY_CLOSED_HISTORY
  git -C "$work" add -A
  arm "a closed history's unsupported shortcut refuses with no counted row to visit it (R34 AG1)" 1 "unsupported first file-link syntax"
  # Round 32: a failed staged read is no proof that a path is absent from the index.
  local bin="$work/fault-bin" real_git
  real_git="$(type -P git)"; mkdir -p "$bin"
  cat > "$bin/git" <<'SH'
#!/usr/bin/env bash
if [ "${1:-}" = show ] && [ "${2:-}" = ":${REVIEW_COUNTS_FAULT_PATH:-}" ]; then exit 128; fi
if [ "${1:-}" = ls-files ] && [ "${REVIEW_COUNTS_PROOF_FAIL:-}" = yes ]; then exit 128; fi
exec "$REVIEW_COUNTS_REAL_GIT" "$@"
SH
  chmod +x "$bin/git"
  local path
  for path in docs/reviews/INDEX.md docs/reviews/b-reviews.md docs/decisions/b.md; do
    world "$GOOD_A" "$GOOD_B"
    PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH="$path" \
      arm "a failed read of a present staged $path refuses despite a good disk copy (R32 D1)" 1 "staged copy cannot be read"
  done
  world "$GOOD_A" "$GOOD_B"
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH=docs/decisions/b.md GIT_LITERAL_PATHSPECS=1 \
    arm "inherited literal-pathspec settings cannot hide a present staged entry (R32 D1)" 1 "staged copy cannot be read"
  world "$GOOD_A" "$GOOD_B"
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH=docs/decisions/b.md REVIEW_COUNTS_PROOF_FAIL=yes \
    arm "a failed absence proof for a present staged path refuses (R32 D1)" 1 "index absence cannot be established"
  world "$GOOD_A" "$GOOD_B"; git -C "$work" rm -q --cached docs/decisions/b.md
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH=docs/decisions/b.md REVIEW_COUNTS_PROOF_FAIL=yes \
    arm "a failed absence proof for a genuinely absent path refuses (R32 D1)" 1 "index absence cannot be established"
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH=docs/decisions/b.md \
    arm "proven index absence permits the good disk copy (R32 D1)" 0 "3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"; open_c
  printf '# C, disk copy has no round table\n' > "$work/docs/decisions/c.md"
  git -C "$work" add docs/reviews/INDEX.md
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH=docs/decisions/c.md \
    arm "a failed primary-record read cannot hide an unlisted table (R32 D1)" 1 "staged copy cannot be read"
  world "$GOOD_A" "$GOOD_B"
  mv "$work/docs/decisions/b.md" "$work/docs/decisions/b[1].md"
  sed -i.bak 's@../decisions/b.md@../decisions/b[1].md@g' "$work/docs/reviews/INDEX.md"
  rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH='docs/decisions/b[1].md' \
    arm "a present staged filename with glob characters still refuses on read failure (R32 D1)" 1 "staged copy cannot be read"
  world "$GOOD_A" "$GOOD_B"
  mv "$work/docs/decisions/b.md" "$work/docs/decisions/b[1].md"
  cp "$work/docs/decisions/b[1].md" "$work/docs/decisions/b1.md"
  sed -i.bak 's@../decisions/b.md@../decisions/b[1].md@g' "$work/docs/reviews/INDEX.md"
  rm -f "$work/docs/reviews/INDEX.md.bak"
  # world stages existing fixture extras too: remove the previous arm's bracketed entry explicitly.
  git -C "$work" rm -q --cached -- docs/decisions/b.md ':(literal)docs/decisions/b[1].md'
  git -C "$work" add docs/reviews/INDEX.md docs/decisions/b1.md
  PATH="$bin:$PATH" REVIEW_COUNTS_REAL_GIT="$real_git" REVIEW_COUNTS_FAULT_PATH='docs/decisions/b[1].md' \
    arm "an absent literal path is not confused with a different glob-matching entry (R32 D1)" 0 "3 round(s) judged"
  rm -rf "${work:?}"
  echo "review-counts self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  "") check ;;
  *) echo "usage: $0 [--self-test]" >&2; exit 2 ;;
esac
