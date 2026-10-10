#!/usr/bin/env bash
# scripts/check_review_counts.sh — REVIEW-COUNTS: a design record's count of what each review round found, and the
# reviews index's count of rounds, are its review history's (leaf `PROGRAM.69`, review rounds 23 to 25).
#
# ⭐ WHY. A design record keeps one summary row per review round, with the count of what the round found, and the
# round's own table in its review history lists each finding. The count was copied by hand, and the task-tree sealing
# record's count disagreed with its history four times (reviews R19 D1, R21 D1, R22 D1, R23 D1); the index's count of
# rounds slipped once (R17). Derived and checked, they cannot drift.
#
# WHAT IT READS — the staged copy of each file (`git show :<path>`), the working tree's for one not in the index:
#   - docs/reviews/INDEX.md: every row whose first cell links a review history, and, where its second cell links a
#     design record anywhere in it, that record;
#   - in the history, its ROUNDS — each line opening `**Round <n>**` — and each round's FINDINGS: the first table
#     headed `| # | Kind | Finding | Answer |` after its line and before the next round's, one finding per row whose
#     first cell is an id — a row whose id is `—`, a finding of the answer's own, is not the review's;
#   - in the record, its summary table, by its header, to the first line that is no table row:
#     `| Round | Findings | Defects | …` (THE FINDINGS SHAPE) or `| Round | Date | Defects | …` (THE DATE SHAPE).
# The OK line counts what it judged and names, with its reason, every index row it judged nothing of: a history
# archived (a stub pointing to docs/review-history/, its review closed) or with no round lines; a row linking no record,
# or a record with neither header, whose count of rounds is still checked. A round with a line and no findings table —
# launched and not completed, or kept as prose — is counted, not judged. The index row's status, prose, is not read.
#
# THE RULE: a round has one line and one summary row; a summary row for a round with no line is refused, and a round
# with a line and no summary row. For each round with a findings table:
#   - both shapes: the defects cell's leading number is its findings of kind `defect`;
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

check() {
  python3 - <<'PY'
import collections, os, re, subprocess, sys
HEAD = "| # | Kind | Finding | Answer |"
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
    out, shape = {}, None
    for l in text.split("\n"):
        if l.startswith("| Round | Findings | Defects |"):
            shape = "findings"; continue
        if l.startswith("| Round | Date | Defects |"):
            shape = "date"; continue
        if shape and not l.startswith("|"):
            shape = None
        m = re.match(r"^\| (\d+) \| ([^|]*) \| ([^|]*) \|", l)
        if shape and m:
            n = int(m.group(1))
            if n in out:
                bad.append("%s: round %d has two summary rows" % (rec, n))
            out[n] = (shape, m.group(2).strip(), m.group(3).strip())
    return out
def stated(cell, word):
    m = re.search(r"(?<![\w-])(\d+) " + word + r"\b", cell)
    return int(m.group(1)) if m else None
def tally(kinds):
    return collections.Counter(
        "pre-existing arm gap" if k.startswith("pre-existing arm gap") else
        "defect" if k.startswith("defect") else
        "arm gap" if k.startswith("arm gap") else
        "remark" if k.startswith("remark") else k for k in kinds)
index = read("docs/reviews/INDEX.md") or ""
judged, unjudged, rows_checked, skipped = 0, 0, 0, []
for l in index.split("\n"):
    cells = l.split(" | ")
    if len(cells) < 4:
        continue
    mh = re.match(r"^\| \[`[^`]+`\]\(([^)]+)\)$", cells[0])
    if not mh:
        continue
    hist = os.path.normpath(os.path.join("docs/reviews", mh.group(1)))
    h = read(hist)
    if h is None:
        continue
    heads = rounds(h, hist)
    if not heads:
        skipped.append("%s (%s)" % (hist, "archived" if "Archived, byte for byte" in h else "no round lines"))
        continue
    # The index row's count of rounds is the history's last round with a findings table: a round launched and not
    # completed, or kept as prose, has none.
    tabled = [n for n, k in heads.items() if k is not None]
    if tabled:
        rows_checked += 1
        if cells[2].strip() != str(max(tabled)):
            bad.append("docs/reviews/INDEX.md: %s's row says %s round(s); its last round with a findings table is %d"
                       % (hist, cells[2].strip(), max(tabled)))
    mr = re.search(r"\[`[^`]+`\]\(([^)]+)\)", cells[1])
    r = read(os.path.normpath(os.path.join("docs/reviews", mr.group(1)))) if mr else None
    rows = summary(r, os.path.normpath(os.path.join("docs/reviews", mr.group(1)))) if r else {}
    if not rows:
        skipped.append("%s (%s)" % (hist, "no record linked" if not mr else "its record has no summary table"))
        continue
    rec = os.path.normpath(os.path.join("docs/reviews", mr.group(1)))
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
        lead = re.match(r"(\d+)", b)
        if not lead:
            bad.append("%s: round %d's defects cell names no number" % (rec, n))
        elif int(lead.group(1)) != count["defect"]:
            say(n, "defect(s)", lead.group(1), count["defect"])
        if shape == "findings":
            if not a.isdigit():
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
print("review-counts: OK (%d round(s) judged, %d with no findings table not judged; %d index row(s)' count of rounds "
      "checked; not judged: %s)" % (judged, unjudged, rows_checked, "; ".join(skipped) or "none"))
PY
}

self_test() {
  local work="$ROOT/target/doctrine_scratch/review_counts" arms=0 ok=0
  rm -rf "${work:?}"; mkdir -p "$work/docs/reviews" "$work/docs/decisions"
  git -C "$work" init -q
  INDEX_A='| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open |'
  world() { # $1 = the findings-shaped row of round 2, $2 = the date-shaped row of round 1, $3 = a's index row
    printf '| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n%s\n%s\n%s\n%s\n' "${3:-$INDEX_A}" \
      '| [`b-reviews.md`](b-reviews.md) | §1 of [`b.md`](../decisions/b.md) | 1 | open |' \
      '| [`c-reviews.md`](c-reviews.md) | [`c.md`](../decisions/c.md) | 3 | closed |' \
      '| [`d-reviews.md`](d-reviews.md) | the headers of some scripts | 1 | open: round 2 not completed |' > "$work/docs/reviews/INDEX.md"
    printf '# A\n\n**Round 1**, prose only.\n\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n| R2-AG1 | arm gap | x | y |\n| R2-R1 | remark | x | y |\n| R2-P1 | pre-existing arm gap | x | y |\n| — | found while answering | x | y |\n' > "$work/docs/reviews/a-reviews.md"
    printf '# A\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 3 | 1 (prose) | v |\n%s\n\n| 9 | 7 | 7 | after the table |\n' "$1" > "$work/docs/decisions/a.md"
    printf '# B\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n| R1-D2 | defect | x | y |\n| R1-R1 | remark, not this change'"'"'s | x | y |\n' > "$work/docs/reviews/b-reviews.md"
    printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n%s\n' "$2" > "$work/docs/decisions/b.md"
    printf '# C\n\nArchived, byte for byte, in [`c-reviews.md`](../review-history/c-reviews.md) — its review closed, and never edited again.\n' > "$work/docs/reviews/c-reviews.md"
    printf '# C\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 99 | 99 | v |\n' > "$work/docs/decisions/c.md"
    printf '# D\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n\n**Round 2**, launched, not completed.\n' > "$work/docs/reviews/d-reviews.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local out rc; arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -eq "$2" ] && printf '%s' "$out" | grep -qF -- "$3"; then ok=$((ok + 1)); echo "  ✅ $1"
    else echo "SELF-TEST: $1 — rc $rc:" >&2; printf '%s\n' "$out" | head -3 | sed 's/^/    /' >&2; fi
  }
  GOOD_A='| 2 | 4 | 1, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |'
  GOOD_B='| 1 | `2026-10-10` | 2, and 1 remark | o |'
  world "$GOOD_A" "$GOOD_B"
  arm "counts that are the histories' pass: a prose round counted, an answer's own row and a row past the table not judged; a remark of any kind beginning so counted, a record linked mid-cell" 0 "2 round(s) judged, 1 with no findings table"
  arm "every row judged nothing of is named with its reason" 0 "c-reviews.md (archived); docs/reviews/d-reviews.md (no record linked)"
  arm "a row with no record has its count of rounds checked: the last round with a findings table" 0 "3 index row(s)' count of rounds"
  world '| 2 | 5 | 1, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a total off by one is refused" 1 "round 2 says 5 finding(s)"
  world '| 2 | 4 | 2, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a defect count off is refused" 1 "round 2 says 2 defect(s)"
  world '| 2 | 4 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "an arm-gap count off is refused" 1 "round 2 says 2 arm gap(s)"
  world '| 2 | 4 | 1, and 1 pre-existing arm gap (y), and 2 arm gaps (x) | v |' "$GOOD_B"
  arm "an arm-gap count after the pre-existing clause is read (R24 AG2)" 1 "round 2 says 2 arm gap(s)"
  world '| 2 | 4 | 1, and 1 arm gap (x), and 0 pre-existing arm gaps (y) | v |' "$GOOD_B"
  arm "a pre-existing arm-gap count off is refused" 1 "round 2 says 0 pre-existing arm gap(s)"
  world '| 2 | four | 1, and 1 arm gap (x) | v |' "$GOOD_B"
  arm "a findings cell that is no number is refused" 1 "round 2's findings cell is not a number"
  world '| 2 | 4 | one (x) | v |' "$GOOD_B"
  arm "a defects cell naming no number is refused" 1 "round 2's defects cell names no number"
  world "$GOOD_A" '| 1 | `2026-10-10` | 2, and 2 remarks | o |'
  arm "a remark count off is refused" 1 "round 1 says 2 remark(s)"
  world "$GOOD_A" '| 1 | `2026-10-10` | 3, and 1 remark | o |'
  arm "a date-shaped record's defect count off is refused" 1 "round 1 says 3 defect(s)"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n' >> "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a round in the history with no summary row is refused" 1 "round 2 is in its history and has no summary row"
  world "$GOOD_A" "$GOOD_B"; printf '| 0 | `2026-10-10` | 0 | o |\n' >> "$work/docs/decisions/b.md"; git -C "$work" add -A
  arm "a summary row for a round the history has no line for is refused (R24 D3)" 1 "round 0's summary row names a round"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 1 | 3 | 1 (prose) | v |$/&\n| 2 | 99 | 99 | v |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"; git -C "$work" add -A
  arm "two summary rows for one round are refused (R25 D3)" 1 "round 2 has two summary rows"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 1**, again.\n' >> "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "two lines for one round are refused (R25 D3)" 1 "round 1 has two lines"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 3 | open |'
  arm "an index row's count of rounds off is refused (R24 D2)" 1 "a-reviews.md's row says 3 round(s)"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/| 1 | open: round 2 not completed |/| 2 | open |/' "$work/docs/reviews/INDEX.md"; rm -f "$work/docs/reviews/INDEX.md.bak"; git -C "$work" add -A
  arm "a round with a line and no findings table does not count toward the index's rounds" 1 "d-reviews.md's row says 2 round(s)"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 2 | 4 |/| 2 | 5 |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"
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
