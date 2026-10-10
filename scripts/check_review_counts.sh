#!/usr/bin/env bash
# scripts/check_review_counts.sh — REVIEW-COUNTS: for each design review this script lists, the record's count of what
# each round found, and the reviews index's count of rounds, are its review history's (leaf `PROGRAM.69`, review rounds
# 23 to 26).
#
# ⭐ WHY. A design record keeps one summary row per review round, with the count of what the round found, and the
# round's own table in its review history lists each finding. The count was copied by hand, and the task-tree sealing
# record's count disagreed with its history four times (reviews R19 D1, R21 D1, R22 D1, R23 D1); the index's count of
# rounds slipped once (R17). Derived and checked, they cannot drift.
#
# ⭐ WHICH REVIEWS: the ones listed in PAIRS below, each a review history and its design record — the open reviews
# whose records keep such a summary table. Nothing is derived and nothing skipped: a listed pair the check cannot judge
# — a file missing, no round line, no summary table, no index row — is refused. A review is added to the list when its
# record's summary table starts, and leaves it in the commit that closes the review (rounds 24 to 26 found every derived
# scope leaving a review unread or unnamed).
#
# WHAT IT READS, for each pair — the staged copy of each file (`git show :<path>`), the working tree's for one not in
# the index:
#   - in the history, its ROUNDS — each line opening `**Round <n>**` — and each round's FINDINGS: the first table headed
#     `| # | Kind | Finding | Answer |` after its line and before the next round's, to its first line that is no table
#     row, one finding per row whose first cell is an id — a row whose id is `—`, a finding of the answer's own, is not
#     the review's. A round with a line and no such table is not judged, and is counted in the OK line;
#   - in the record, its summary table: the header `| Round | Findings | Defects | …` (THE FINDINGS SHAPE) or
#     `| Round | Date | Defects | …` (THE DATE SHAPE), and every row under it to the first line that is no table row;
#   - in docs/reviews/INDEX.md, the row whose first cell links the history.
#
# THE RULE: a round has one line and one summary row, and every row under the header names a round by a number; a
# summary row for a round with no line is refused, and a round with a line and no summary row. For each round with a
# findings table:
#   - both shapes: the defects cell begins with its number of findings of kind `defect`;
#   - THE FINDINGS SHAPE: the findings cell is its number of findings; "N arm gap(s)", wherever it stands in the
#     defects cell, its findings of kind `arm gap`; "N pre-existing arm gap(s)", those of kind `pre-existing arm gap`;
#   - THE DATE SHAPE: "N remark(s)" is its findings whose kind begins `remark`;
#   a count a cell does not state is not judged. And the index row's Rounds cell is the history's last round with a
#   findings table.
#
# MODES: (none) the gate — exit 0 clean, 1 each disagreement named · --self-test.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

# history|record — one line a review. The self-test sets REVIEW_COUNTS_PAIRS to its scratch world's.
PAIRS="${REVIEW_COUNTS_PAIRS:-docs/reviews/decision_task-tree-sealing-reviews.md|docs/decisions/decision_task-tree-sealing.md
docs/reviews/decision_trust-generated-refusals-reviews.md|docs/specs/trust/decision_trust-generated-refusals.md}"

check() {
  PAIRS="$PAIRS" python3 -I - <<'PY'
import collections, os, re, subprocess, sys
HEAD = "| # | Kind | Finding | Answer |"
NUMBER = re.compile(r"[0-9]+")
def read(path):
    r = subprocess.run(["git", "show", ":" + path], capture_output=True, text=True)
    if r.returncode == 0:
        return r.stdout
    return open(path, encoding="utf-8").read() if os.path.isfile(path) else None
bad = []
def rounds(text, hist):
    """Each round with a line: its findings' kinds, or None where it has no findings table."""
    out = {}
    parts = re.split(r"(?m)^\*\*Round (\d+)\*\*", text)
    for i in range(1, len(parts), 2):
        n, lines, kinds = int(parts[i]), parts[i + 1].split("\n"), None
        if n in out:
            bad.append("%s: round %d has two lines" % (hist, n))
        if HEAD in lines:
            k, kinds = lines.index(HEAD) + 2, []
            while k < len(lines) and lines[k].startswith("| "):
                cells = [c.strip() for c in lines[k].strip().strip("|").split("|")]
                if len(cells) > 1 and cells[0] != "—":
                    kinds.append(cells[1].lower())
                k += 1
        out[n] = kinds
    return out
def summary(text, rec):
    """The summary table's rows by round, and whether its header was seen."""
    out, shape, seen = {}, None, False
    for l in text.split("\n"):
        if l.startswith("| Round | Findings | Defects |"):
            shape, seen = "findings", True; continue
        if l.startswith("| Round | Date | Defects |"):
            shape, seen = "date", True; continue
        if shape and not l.startswith("|"):
            shape = None
        if not shape or re.match(r"^\| *-+ *\|", l):
            continue
        cells = [c.strip() for c in l.strip().strip("|").split("|")]
        if len(cells) < 3 or not NUMBER.fullmatch(cells[0]):
            bad.append("%s: a row under its summary header names no round by a number: %r" % (rec, cells[0][:20]))
            continue
        n = int(cells[0])
        if n in out:
            bad.append("%s: round %d has two summary rows" % (rec, n))
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
judged, unjudged, pairs = 0, 0, 0
for pair in os.environ["PAIRS"].split("\n"):
    if not pair.strip():
        continue
    hist, rec = pair.strip().split("|")
    pairs += 1
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
    tabled = [n for n, k in heads.items() if k is not None]
    if len(row) != 1:
        bad.append("docs/reviews/INDEX.md: %s has %d row(s), not one" % (hist, len(row)))
    elif tabled and row[0].split(" | ")[2].strip() != str(max(tabled)):
        bad.append("docs/reviews/INDEX.md: %s's row says %s round(s); its last round with a findings table is %d"
                   % (hist, row[0].split(" | ")[2].strip(), max(tabled)))
    def say(n, what, said, real):
        bad.append("%s: round %d says %s %s; %s's round %d holds %d" % (rec, n, said, what, hist, n, real))
    for n in sorted(set(heads) | set(rows)):
        if n not in heads:
            bad.append("%s: round %d's summary row names a round %s has no line for" % (rec, n, hist)); continue
        if n not in rows:
            bad.append("%s: round %d is in its history and has no summary row" % (rec, n)); continue
        if heads[n] is None:
            unjudged += 1; continue
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
for x in bad:
    print("REVIEW-COUNTS: " + x, file=sys.stderr)
if bad:
    sys.exit(1)
print("review-counts: OK (%d review(s) listed; %d round(s) judged, %d with no findings table not judged)" % (pairs, judged, unjudged))
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
    printf '| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n%s\n%s\n' "${3:-$INDEX_A}" \
      '| [`b-reviews.md`](b-reviews.md) | §1 of [`b.md`](../decisions/b.md) | 1 | open |' > "$work/docs/reviews/INDEX.md"
    # a: round 1 prose, round 2 a findings table — one defect, two arm gaps, one pre-existing arm gap, a remark, an
    # answer's own row — then a second table of the same header and rows past a blank line, neither of them read.
    printf '# A\n\n**Round 1**, prose only.\n\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n| R2-AG1 | arm gap | x | y |\n| R2-AG2 | arm gap | x | y |\n| R2-R1 | remark | x | y |\n| R2-P1 | pre-existing arm gap | x | y |\n| — | found while answering | x | y |\n\n| R2-D9 | defect | past the table | y |\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D8 | defect | a second table | y |\n' > "$work/docs/reviews/a-reviews.md"
    printf '# A\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 3 | 1 (prose) | v |\n%s\n\n| 9 | 7 | 7 | after the table |\n' "$1" > "$work/docs/decisions/a.md"
    printf '# B\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n| R1-D2 | defect | x | y |\n| R1-R1 | remark, not this change'"'"'s | x | y |\n' > "$work/docs/reviews/b-reviews.md"
    printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n%s\n' "$2" > "$work/docs/decisions/b.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local out rc; arms=$((arms + 1))
    out="$(cd "$work" && REVIEW_COUNTS_PAIRS="$LISTED" bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -eq "$2" ] && printf '%s' "$out" | grep -qF -- "$3"; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — rc $rc:" >&2; printf '%s\n' "$out" | head -3 | sed 's/^/    /' >&2; fi
  }
  GOOD_A='| 2 | 5 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |'
  GOOD_B='| 1 | `2026-10-10` | 2, and 1 remark | o |'
  world "$GOOD_A" "$GOOD_B"
  arm "counts that are the histories' pass: the first table only, to its end; a prose round counted; an answer's own row and a row past the summary table not judged; a remark of any kind beginning so" 0 "2 review(s) listed; 2 round(s) judged, 1 with no findings table"
  world '| 2 | 6 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a total off by one is refused" 1 "round 2 says 6 finding(s)"
  world '| 2 | 5 | 2, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a defect count off is refused" 1 "round 2 says 2 defect(s)"
  world '| 2 | 5 | 1, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "an arm-gap count off, equal to the pre-existing one, is refused (R26 AG1)" 1 "round 2 says 1 arm gap(s)"
  world '| 2 | 5 | 1, and 1 pre-existing arm gap (y), and 3 arm gaps (x) | v |' "$GOOD_B"
  arm "an arm-gap count after the pre-existing clause is read (R24 AG2)" 1 "round 2 says 3 arm gap(s)"
  world '| 2 | 5 | 1, and 2 arm gaps (x), and 2 pre-existing arm gaps (y) | v |' "$GOOD_B"
  arm "a pre-existing arm-gap count off, equal to the arm gaps, is refused (R26 AG1)" 1 "round 2 says 2 pre-existing arm gap(s)"
  world '| 2 | five | 1, and 2 arm gaps (x) | v |' "$GOOD_B"
  arm "a findings cell that is no number is refused" 1 "round 2's findings cell is not a number"
  world '| 2 | ⁵ | 1, and 2 arm gaps (x) | v |' "$GOOD_B"
  arm "a findings cell of a digit that is not 0 to 9 is refused, never a traceback (R26 R4)" 1 "round 2's findings cell is not a number"
  world '| 2 | 5 | about 1, and 2 arm gaps (x) | v |' "$GOOD_B"
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
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 1**, again.\n' >> "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "two lines for one round are refused (R25 D3)" 1 "round 1 has two lines"
  world "$GOOD_A" '| **1** | `2026-10-10` | 2, and 1 remark | o |'
  arm "a row under the header whose round is no number is refused, never skipped (R26 D2)" 1 "names no round by a number"
  world "$GOOD_A" ''
  arm "a summary table with its header and no row refuses each round (R26 D2)" 1 "round 1 is in its history and has no summary row"
  world "$GOOD_A" "$GOOD_B"; rm "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a listed review whose history is missing is refused (R26 D1)" 1 "b-reviews.md is missing"
  world "$GOOD_A" "$GOOD_B"; printf '# B\n\nNo round yet.\n' > "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a listed review whose history holds no round line is refused (R26 AG2)" 1 "listed, and it holds no round line"
  world "$GOOD_A" "$GOOD_B"; printf '# B\n\nNo table.\n' > "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "a listed review whose record holds no summary table is refused (R26 AG2)" 1 "listed, and it holds no summary table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak '/b-reviews.md/d' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a listed review with no index row is refused (R26 D1)" 1 "b-reviews.md has 0 row(s), not one"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 3 | open |'
  arm "an index row's count of rounds off is refused (R24 D2)" 1 "a-reviews.md's row says 3 round(s)"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 3**, launched, not completed.\n' >> "$work/docs/reviews/a-reviews.md"; printf '| 3 | 0 | 0 (not completed) | v |\n' > "$work/a3"; sed -i.bak "/^| 2 | 5 |/r $work/a3" "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak" "$work/a3"; git -C "$work" add -A
  arm "a round with a line and no findings table does not count toward the index's rounds, and is counted as not judged" 0 "2 with no findings table"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 2 | 5 |/| 2 | 6 |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"
  arm "the staged copy is judged, not the disk's" 0 "2 round(s) judged"
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
