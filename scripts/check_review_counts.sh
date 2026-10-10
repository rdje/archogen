#!/usr/bin/env bash
# scripts/check_review_counts.sh — REVIEW-COUNTS: a design record's count of what each review round found, and the
# reviews index's row for it, are its review history's (leaf `PROGRAM.69`, review rounds 23 and 24).
#
# ⭐ WHY. A design record keeps one summary row per review round, with the count of what the round found, and the
# round's own table in its review history lists each finding. The count was copied by hand, and the task-tree sealing
# record's count disagreed with its history four times (reviews R19 D1, R21 D1, R22 D1, R23 D1); the index's count of
# rounds slipped once (R17). Derived and checked, they cannot drift.
#
# WHAT IT READS — the staged copy of each file (`git show :<path>`), the working tree's for one not in the index:
#   - docs/reviews/INDEX.md: every row whose first cell links a review history and whose second cell links a design
#     record, anywhere in the cell, both files;
#   - in the history, its ROUNDS — each line opening `**Round <n>**` — and each round's FINDINGS: the first table
#     headed `| # | Kind | Finding | Answer |` after its line and before the next round's, one finding per row whose
#     first cell is an id — a row whose id is `—`, a finding of the answer's own, is not the review's;
#   - in the record, its summary table, by its header, to the first line that is no table row:
#     `| Round | Findings | Defects | …` (THE FINDINGS SHAPE) or `| Round | Date | Defects | …` (THE DATE SHAPE).
# NOT READ, and named in the OK line when it has a summary table: a record whose history is archived — a stub under
# docs/reviews/ pointing to docs/review-history/, its review closed; its counts were written by the rules of their day. Nor judged: a round with
# a line but no findings table — rounds a history keeps as prose — and a record with neither header.
#
# THE RULE, for each round with a findings table:
#   - both shapes: the defects cell's leading number is its findings of kind `defect`;
#   - THE FINDINGS SHAPE: the findings cell is its number of findings; "N arm gap(s)", wherever it stands in the
#     defects cell, its findings of kind `arm gap`; "N pre-existing arm gap(s)", those of kind `pre-existing arm gap`;
#   - THE DATE SHAPE: "N remark(s)" is its findings whose kind begins `remark`;
#   a count a cell does not state is not judged. And for every record read: a summary row for a round its history has
#   no line for is refused, and a round its history has a line for with no summary row; the index row's Rounds cell is
#   the history's last round; its status's "round N found D defect(s)", with "A arm gap(s)" and "P pre-existing arm
#   gap(s)" where it says them, is round N's table. A round's own paragraph, prose, is not read.
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
def rounds(text):
    """Each round with a line: its findings' kinds, or None where it has no findings table."""
    out = {}
    parts = re.split(r"(?m)^\*\*Round (\d+)\*\*", text)
    for i in range(1, len(parts), 2):
        lines = parts[i + 1].split("\n")
        kinds = None
        if HEAD in lines:
            k, kinds = lines.index(HEAD) + 2, []
            while k < len(lines) and lines[k].startswith("| "):
                cells = [c.strip() for c in lines[k].strip().strip("|").split("|")]
                if len(cells) > 1 and cells[0] != "—":
                    kinds.append(cells[1].lower())
                k += 1
        out[int(parts[i])] = kinds
    return out
def summary(text):
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
            out[int(m.group(1))] = (shape, m.group(2).strip(), m.group(3).strip())
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
bad, judged, records, archived = [], 0, 0, []
for l in index.split("\n"):
    cells = l.split(" | ")
    if len(cells) < 4:
        continue
    mh = re.match(r"^\| \[`[^`]+`\]\(([^)]+)\)$", cells[0])
    mr = re.search(r"\[`[^`]+`\]\(([^)]+)\)", cells[1])
    if not mh or not mr:
        continue
    hist = os.path.normpath(os.path.join("docs/reviews", mh.group(1)))
    rec = os.path.normpath(os.path.join("docs/reviews", mr.group(1)))
    h, r = read(hist), read(rec)
    if h is None or r is None:
        continue
    heads, rows = rounds(h), summary(r)
    if not heads and "Archived, byte for byte" in h:
        if rows and rec not in archived:
            archived.append(rec)  # a record whose counts this check would otherwise read
        continue
    if not rows or not heads:
        continue
    records += 1
    def say(n, what, said, real, where=rec):
        bad.append("%s: round %d says %s %s; %s's round %d holds %d" % (where, n, said, what, hist, n, real))
    for n in sorted(set(heads) | set(rows)):
        if n not in heads:
            bad.append("%s: round %d's summary row names a round %s has no line for" % (rec, n, hist)); continue
        if n not in rows:
            bad.append("%s: round %d is in its history and has no summary row" % (rec, n)); continue
        if heads[n] is None:
            continue
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
    # The index row: its Rounds cell, and what its status says the last round found.
    last = max(heads)
    if cells[2].strip() != str(last):
        bad.append("docs/reviews/INDEX.md: %s's row says %s round(s); %s's last round is %d" % (hist, cells[2].strip(), hist, last))
    st = re.search(r"round (\d+) found (\d+) defects?", cells[3])
    if st and heads.get(int(st.group(1))) is not None:
        n = int(st.group(1)); count = tally(heads[n]); rest = cells[3][st.start():]
        rest = re.split(r";|answered", rest)[0]
        if int(st.group(2)) != count["defect"]:
            say(n, "defect(s)", st.group(2), count["defect"], "docs/reviews/INDEX.md")
        g = stated(rest, "arm gaps?")
        if g is not None and g != count["arm gap"]:
            say(n, "arm gap(s)", g, count["arm gap"], "docs/reviews/INDEX.md")
        pg = stated(rest, "pre-existing arm gaps?")
        if pg is not None and pg != count["pre-existing arm gap"]:
            say(n, "pre-existing arm gap(s)", pg, count["pre-existing arm gap"], "docs/reviews/INDEX.md")
for x in bad:
    print("REVIEW-COUNTS: " + x, file=sys.stderr)
if bad:
    sys.exit(1)
print("review-counts: OK (%d round(s) of %d record(s) judged, each count and index row its history's; not read, "
      "their histories archived: %s)" % (judged, records, ", ".join(archived) or "none"))
PY
}

self_test() {
  local work="$ROOT/target/doctrine_scratch/review_counts" arms=0 ok=0
  rm -rf "${work:?}"; mkdir -p "$work/docs/reviews" "$work/docs/decisions"
  git -C "$work" init -q
  INDEX_A='| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open: round 2 found 1 defect and 1 arm gap |'
  INDEX_B='| [`b-reviews.md`](b-reviews.md) | §1 of [`b.md`](../decisions/b.md) | 1 | open |'
  world() { # $1 = the findings-shaped row of round 2, $2 = the date-shaped row of round 1, $3 = a's index row
    printf '| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n%s\n%s\n| [`c-reviews.md`](c-reviews.md) | [`c.md`](../decisions/c.md) | 3 | closed |\n' "${3:-$INDEX_A}" "$INDEX_B" > "$work/docs/reviews/INDEX.md"
    printf '# A\n\n**Round 1**, prose only.\n\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n| R2-AG1 | arm gap | x | y |\n| R2-R1 | remark | x | y |\n| R2-P1 | pre-existing arm gap | x | y |\n| — | found while answering | x | y |\n' > "$work/docs/reviews/a-reviews.md"
    printf '# A\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 3 | 1 (prose) | v |\n%s\n\n| 9 | 7 | 7 | after the table |\n' "$1" > "$work/docs/decisions/a.md"
    printf '# B\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n| R1-D2 | defect | x | y |\n| R1-R1 | remark, not this change'"'"'s | x | y |\n' > "$work/docs/reviews/b-reviews.md"
    printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n%s\n' "$2" > "$work/docs/decisions/b.md"
    printf '# C\n\nArchived, byte for byte, in [`c-reviews.md`](../review-history/c-reviews.md) — its review closed, and never edited again.\n' > "$work/docs/reviews/c-reviews.md"
    printf '# C\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 99 | 99 | v |\n' > "$work/docs/decisions/c.md"
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
  arm "counts that are the histories' pass: a prose round, an answer's own row and a row past the table not judged" 0 "2 round(s) of 2 record(s) judged"
  arm "an archived history's record is not read, and named" 0 "their histories archived: docs/decisions/c.md"
  arm "a record linked mid-cell is read, and a remark of any kind beginning so counted (R24 AG1)" 0 "2 round(s) of 2 record(s)"
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
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 3 | open |'
  arm "an index row's count of rounds off is refused (R24 D2)" 1 "a-reviews.md's row says 3 round(s)"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open: round 2 found 4 defects and 1 arm gap; answered |'
  arm "an index row's count of defects off is refused (R24 D2)" 1 "docs/reviews/INDEX.md: round 2 says 4 defect(s)"
  world "$GOOD_A" "$GOOD_B" '| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open: round 2 found 1 defect and 3 arm gaps; answered |'
  arm "an index row's count of arm gaps off is refused (R24 D2)" 1 "docs/reviews/INDEX.md: round 2 says 3 arm gap(s)"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 2 | 4 |/| 2 | 5 |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"
  arm "the staged copy is judged, not the disk's" 0 "2 round(s) of 2 record(s) judged"
  rm -rf "${work:?}"
  echo "review-counts self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  "") check ;;
  *) echo "usage: $0 [--self-test]" >&2; exit 2 ;;
esac
