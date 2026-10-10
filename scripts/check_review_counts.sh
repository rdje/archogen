#!/usr/bin/env bash
# scripts/check_review_counts.sh — REVIEW-COUNTS: for each design review this script lists, the record's count of what
# each round found, and the reviews index's count of rounds, are its review history's (leaf `PROGRAM.69`, review rounds
# 23 to 27).
#
# ⭐ WHY. A design record keeps one summary row per review round, with the count of what the round found, and the
# round's own table in its review history lists each finding. The count was copied by hand, and the task-tree sealing
# record's count disagreed with its history four times (reviews R19 D1, R21 D1, R22 D1, R23 D1); the index's count of
# rounds slipped once (R17). Derived and checked, they cannot drift.
#
# ⭐ WHICH REVIEWS: the ones listed in PAIRS below, each a review history and its design record. Nothing is derived and
# nothing skipped: a listed review the check cannot judge whole — a file missing, no round line, a round with no
# well-formed findings table, no summary table, not one index row — is refused (rounds 24 to 27 found every derived or
# partial scope leaving a review, or a round, unread). The list is every open review whose record keeps a summary
# table, from its first round: an index row whose status begins `open`, whose linked record holds a summary header, and
# whose history holds a round line or its record a summary row, is refused unless its history is listed. A review
# leaves the list in the commit that closes it.
#
# WHAT IT READS, for each listed review — the staged copy of each file (`git show :<path>`), the working tree's for one
# not in the index:
#   - in the history, its ROUNDS — each line opening `**Round <n>**` — and each round's FINDINGS TABLE: the first table
#     after its line and before the next round's, headed exactly `| # | Kind | Finding | Answer |`, its next line a
#     separator row of dashes, and its finding rows to the first line that is no table row; each finding row's first
#     cell an id — `—` marking a finding of the answer's own, which is not the review's;
#   - in the record, its summary table: the header `| Round | Findings | Defects | …` (THE FINDINGS SHAPE) or
#     `| Round | Date | Defects | …` (THE DATE SHAPE), a separator row of dashes, and every row under it to the first
#     line that is no table row;
#   - in docs/reviews/INDEX.md, every row: the listed reviews' own, and each open one's linked record.
#
# THE RULE: every round of a listed review has one line, one well-formed findings table and one summary row; every row
# under a summary header names a round by a number of 0 to 9 digits; a summary row for a round with no line is refused.
# For each round:
#   - both shapes: the defects cell begins with its number of findings of kind `defect`;
#   - THE FINDINGS SHAPE: the findings cell is its number of findings; "N arm gap(s)", wherever it stands in the defects
#     cell, its findings of kind `arm gap`; "N pre-existing arm gap(s)", those of kind exactly `pre-existing arm gap`;
#   - THE DATE SHAPE: "N remark(s)" is its findings whose kind begins `remark`;
#   a count a cell does not state is not judged. The index row's Rounds cell is the history's last round.
#
# MODES: (none) the gate — exit 0 clean, 1 each disagreement named · --self-test.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# history|record — one line a review. The self-test sets REVIEW_COUNTS_PAIRS to its scratch world's.
PAIRS="${REVIEW_COUNTS_PAIRS:-docs/reviews/decision_task-tree-sealing-reviews.md|docs/decisions/decision_task-tree-sealing.md}"

check() {
  PAIRS="$PAIRS" python3 -I - <<'PY'
import collections, os, re, subprocess, sys
HEAD = "| # | Kind | Finding | Answer |"
SEPARATOR = re.compile(r"^\|( *:?-+:? *\|)+$")
NUMBER = re.compile(r"[0-9]+")
def read(path):
    r = subprocess.run(["git", "show", ":" + path], capture_output=True, text=True)
    if r.returncode == 0:
        return r.stdout
    return open(path, encoding="utf-8").read() if os.path.isfile(path) else None
bad = []
def rounds(text, hist):
    """Each round with a line: its findings' kinds, or None where it has no well-formed findings table."""
    out = {}
    parts = re.split(r"(?m)^\*\*Round (\d+)\*\*", text)
    for i in range(1, len(parts), 2):
        n, lines, kinds = int(parts[i]), parts[i + 1].split("\n"), None
        if n in out:
            bad.append("%s: round %d has two lines" % (hist, n))
        if HEAD in lines:
            k = lines.index(HEAD) + 1
            if k >= len(lines) or not SEPARATOR.match(lines[k].strip()):
                bad.append("%s: round %d's findings table has no separator row under its header" % (hist, n))
            else:
                kinds, k = [], k + 1
                while k < len(lines) and lines[k].startswith("|"):
                    cells = [c.strip() for c in lines[k].strip().strip("|").split("|")]
                    if len(cells) < 2 or not cells[0]:
                        bad.append("%s: round %d's findings table holds a row with no id" % (hist, n))
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
        cells = [c.strip() for c in l.strip().strip("|").split("|")]
        if len(cells) < 3 or not NUMBER.fullmatch(cells[0]):
            note("%s: a row under its summary header names no round by a number: %r" % (rec, cells[0][:20]))
            continue
        n = int(cells[0])
        if n in out:
            note("%s: round %d has two summary rows" % (rec, n))
        out[n] = (shape, cells[1], cells[2])
    return out, seen
def stated(cell, word):
    m = re.search(r"(?<![\w-])([0-9]+) " + word + r"\b", cell)
    return int(m.group(1)) if m else None
def tally(kinds):
    return collections.Counter(
        "pre-existing arm gap" if k.startswith("pre-existing arm gap") else
        "defect" if k.startswith("defect") else
        "arm gap" if k.startswith("arm gap") else
        "remark" if k.startswith("remark") else k for k in kinds)
index = read("docs/reviews/INDEX.md") or ""
listed, judged = set(), 0
for pair in os.environ["PAIRS"].split("\n"):
    if not pair.strip():
        continue
    hist, rec = pair.strip().split("|")
    listed.add(hist)
    h, r = read(hist), read(rec)
    if h is None or r is None:
        bad.append("%s: listed with %s, and %s is missing" % (hist, rec, hist if h is None else rec)); continue
    heads = rounds(h, hist)
    rows, seen = summary(r, rec)
    if not heads:
        bad.append("%s: listed, and it holds no round line" % hist); continue
    if not seen:
        bad.append("%s: listed, and it holds no summary table" % rec); continue
    name = os.path.relpath(hist, "docs/reviews")
    row = [l for l in index.split("\n") if l.startswith("| [`%s`](%s) |" % (name, name))]
    if len(row) != 1:
        bad.append("docs/reviews/INDEX.md: %s has %d row(s), not one" % (hist, len(row)))
    elif row[0].split(" | ")[2].strip() != str(max(heads)):
        bad.append("docs/reviews/INDEX.md: %s's row says %s round(s); its last round is %d"
                   % (hist, row[0].split(" | ")[2].strip(), max(heads)))
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
            g = stated(b, "arm gaps?")
            if g is not None and g != count["arm gap"]:
                say(n, "arm gap(s)", g, count["arm gap"])
            pg = stated(b, "pre-existing arm gaps?")
            if pg is not None and pg != count["pre-existing arm gap"]:
                say(n, "pre-existing arm gap(s)", pg, count["pre-existing arm gap"])
        else:
            rm = stated(b, "remarks?")
            if rm is not None and rm != count["remark"]:
                say(n, "remark(s)", rm, count["remark"])
# The list is every open review whose record keeps a summary table, from its first round.
for l in index.split("\n"):
    cells = l.split(" | ")
    if len(cells) < 4 or not cells[3].strip().startswith("open"):
        continue
    mh = re.match(r"^\| \[`[^`]+`\]\(([^)]+)\)$", cells[0])
    mr = re.search(r"\[`[^`]+`\]\(([^)]+)\)", cells[1])
    if not mh or not mr:
        continue
    hist = os.path.normpath(os.path.join("docs/reviews", mh.group(1)))
    rec = os.path.normpath(os.path.join("docs/reviews", mr.group(1)))
    h, r = read(hist) or "", read(rec)
    if hist in listed or r is None:
        continue
    rows, seen = summary(r, rec, quiet=True)
    if seen and (rows or re.search(r"(?m)^\*\*Round [0-9]+\*\*", h)):
        bad.append("%s: an open review whose record keeps a summary table, past its first round, and its history is not"
                   " listed" % hist)
for x in bad:
    print("REVIEW-COUNTS: " + x, file=sys.stderr)
if bad:
    sys.exit(1)
print("review-counts: OK (%d review(s) listed; %d round(s) judged, every one)" % (len(listed), judged))
PY
}

self_test() {
  local work="$ROOT/target/doctrine_scratch/review_counts" arms=0 ok=0
  local LISTED="docs/reviews/a-reviews.md|docs/decisions/a.md
docs/reviews/b-reviews.md|docs/decisions/b.md"
  rm -rf "${work:?}"; mkdir -p "$work/docs/reviews" "$work/docs/decisions"
  git -C "$work" init -q
  INDEX_A='| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open |'
  world() { # $1 = the findings-shaped row of round 2, $2 = the date-shaped row of round 1, $3 = a's index row
    printf '| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n%s\n%s\n%s\n' "${3:-$INDEX_A}" \
      '| [`b-reviews.md`](b-reviews.md) | §1 of [`b.md`](../decisions/b.md) | 1 | open |' \
      '| [`c-reviews.md`](c-reviews.md) | [`c.md`](../decisions/c.md) | 3 | closed |' > "$work/docs/reviews/INDEX.md"
    # a: round 1 one finding; round 2 one defect, two arm gaps, one pre-existing arm gap, one pre-existing finding, a
    # remark, an answer's own row — then rows past a blank line and a second table of the same header, neither read.
    printf '# A\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n| R2-AG1 | arm gap | x | y |\n| R2-AG2 | arm gap | x | y |\n| R2-R1 | remark | x | y |\n| R2-P1 | pre-existing arm gap | x | y |\n| R2-P2 | pre-existing | x | y |\n| — | found while answering | x | y |\n\n| R2-D9 | defect | past the table | y |\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D8 | defect | a second table | y |\n' > "$work/docs/reviews/a-reviews.md"
    printf '# A\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 1 | 1 (x) | v |\n%s\n\n| 9 | 7 | 7 | after the table |\n' "$1" > "$work/docs/decisions/a.md"
    printf '# B\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n| R1-D2 | defect | x | y |\n| R1-R1 | remark, not this change'"'"'s | x | y |\n' > "$work/docs/reviews/b-reviews.md"
    printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n%s\n' "$2" > "$work/docs/decisions/b.md"
    # c: closed, its record keeping a table with wrong counts — no open review, so not the list's.
    printf '# C\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n' > "$work/docs/reviews/c-reviews.md"
    printf '# C\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 99 | 99 | v |\n' > "$work/docs/decisions/c.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local out rc; arms=$((arms + 1))
    out="$(cd "$work" && REVIEW_COUNTS_PAIRS="$LISTED" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -eq "$2" ] && printf '%s' "$out" | grep -qF -- "$3"; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — rc $rc:" >&2; printf '%s\n' "$out" | head -3 | sed 's/^/    /' >&2; fi
  }
  GOOD_A='| 2 | 6 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |'
  GOOD_B='| 1 | `2026-10-10` | 2, and 1 remark | o |'
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
  world "$GOOD_A" "$GOOD_B"; sed -i.bak '/b-reviews.md/d' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a listed review with no index row is refused (R26 D1)" 1 "b-reviews.md has 0 row(s), not one"
  world "$GOOD_A" "$GOOD_B"; grep 'b-reviews.md' "$work/docs/reviews/INDEX.md" > "$work/b.row"; cat "$work/b.row" >> "$work/docs/reviews/INDEX.md"; rm "$work/b.row"; git -C "$work" add -A
  arm "a listed review with two index rows is refused (R27 R2)" 1 "b-reviews.md has 2 row(s), not one"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 3 | open |'
  arm "an index row's count of rounds off is refused (R24 D2)" 1 "a-reviews.md's row says 3 round(s)"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 3 | closed |/| 3 | open |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open review whose record keeps a summary table, unlisted, is refused (R27 AG3)" 1 "c-reviews.md: an open review whose record keeps a summary table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 3 | closed |/| 0 | open |/' "$work/docs/reviews/INDEX.md"; printf '# C\n\nNo round yet.\n' > "$work/docs/reviews/c-reviews.md"; printf '# C\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n' > "$work/docs/decisions/c.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open review before its first round, its summary table empty, need not be listed" 0 "2 review(s) listed; 3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 3 | closed |/| 0 | open |/' "$work/docs/reviews/INDEX.md"; printf '# C\n\nNo round yet.\n' > "$work/docs/reviews/c-reviews.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "an open review with no round line and a summary row, unlisted, is refused" 1 "c-reviews.md: an open review whose record keeps a summary table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 2 | 6 |/| 2 | 7 |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"
  arm "the staged copy is judged, not the disk's" 0 "3 round(s) judged"
  world "$GOOD_A" "$GOOD_B"; git -C "$work" rm -q --cached docs/decisions/b.md; printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n| 1 | `2026-10-10` | 5, and 1 remark | o |\n' > "$work/docs/decisions/b.md"
  arm "a file not in the index is read from the working tree (R25 AG2)" 1 "round 1 says 5 defect(s)"
  rm -rf "${work:?}"
  echo "review-counts self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  "") check ;;
  *) echo "usage: $0 [--self-test]" >&2; exit 2 ;;
esac
