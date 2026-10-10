#!/usr/bin/env bash
# scripts/check_review_counts.sh — REVIEW-COUNTS: a design record's count of what each review round found is its review
# history's (leaf `PROGRAM.69`, review round 23).
#
# ⭐ WHY. A design record keeps one summary row per review round, with the count of what the round found, and the
# round's own table in its review history lists each finding. The count was copied by hand, and the task-tree sealing
# record's count disagreed with its history four times (reviews R19 D1, R21 D1, R22 D1, R23 D1). Derived and checked,
# it cannot drift.
#
# WHAT IT READS — the staged copy of each file (`git show :<path>`), the working tree's for one not in the index:
#   - docs/reviews/INDEX.md: every row linking a review history and a design record, both as files;
#   - in the history, each round's FINDINGS: the first table headed `| # | Kind | Finding | Answer |` after the line
#     opening `**Round <n>**`, one finding per row whose first cell is an id — a row whose id is `—`, a finding of the
#     answer's own, is not the review's;
#   - in the record, its summary table, by its header: `| Round | Findings | Defects | …` — the round's total, then its
#     defects, its arm gaps and its pre-existing arm gaps — or `| Round | Date | Defects | …` — its defects, then its
#     remarks.
# A round the history gives no findings table, and a record with neither header, are not judged.
#
# THE RULE, for each judged round: the total is the round's findings; the defects cell's leading number, its findings
# of kind `defect`; "N arm gap(s)" before any "pre-existing", those of kind `arm gap`; "N pre-existing arm gap(s)",
# those of kind `pre-existing arm gap`; "N remark(s)", those whose kind begins `remark`. A row of the record for a
# round the history does not hold, or the reverse, is refused too.
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
def findings(text):
    out = {}
    parts = re.split(r"(?m)^\*\*Round (\d+)\*\*", text)
    for i in range(1, len(parts), 2):
        lines = parts[i + 1].split("\n")
        if HEAD not in lines:
            continue
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
            shape = "total"; continue
        if l.startswith("| Round | Date | Defects |"):
            shape = "split"; continue
        if shape and not l.startswith("|"):
            shape = None
        m = re.match(r"^\| (\d+) \| ([^|]*) \| ([^|]*) \|", l)
        if shape and m:
            out[int(m.group(1))] = (shape, m.group(2).strip(), m.group(3).strip())
    return out
def stated(cell, word):
    m = re.search(r"(\d+) " + word + r"\b", cell)
    return int(m.group(1)) if m else None
index = read("docs/reviews/INDEX.md") or ""
bad, judged = [], 0
for l in index.split("\n"):
    m = re.match(r"^\| \[`[^`]+`\]\(([^)]+)\) \| \[`[^`]+`\]\(([^)]+)\)", l)
    if not m:
        continue
    hist = os.path.normpath(os.path.join("docs/reviews", m.group(1)))
    rec = os.path.normpath(os.path.join("docs/reviews", m.group(2)))
    h, r = read(hist), read(rec)
    if h is None or r is None:
        continue
    found, rows = findings(h), summary(r)
    if not rows or not found:
        continue
    for n in sorted(set(found) | set(rows)):
        if n not in rows:
            bad.append("%s: round %d is in its history and has no summary row" % (rec, n)); continue
        if n not in found:
            if n >= min(found):
                bad.append("%s: round %d's summary row has no findings table in %s" % (rec, n, hist))
            continue
        judged += 1
        shape, a, b = rows[n]
        kinds = found[n]
        count = collections.Counter(
            "pre-existing arm gap" if k.startswith("pre-existing arm gap") else
            "defect" if k.startswith("defect") else
            "arm gap" if k.startswith("arm gap") else
            "remark" if k.startswith("remark") else k for k in kinds)
        def say(what, said, real):
            bad.append("%s: round %d says %s %s; %s's round %d holds %d" % (rec, n, said, what, hist, n, real))
        lead = re.match(r"(\d+)", b)
        if not lead:
            bad.append("%s: round %d's defects cell names no number" % (rec, n))
        elif int(lead.group(1)) != count["defect"]:
            say("defect(s)", lead.group(1), count["defect"])
        if shape == "total":
            if not a.isdigit():
                bad.append("%s: round %d's findings cell is not a number" % (rec, n))
            elif int(a) != len(kinds):
                say("finding(s)", a, len(kinds))
            g = stated(re.split("pre-existing", b)[0], "arm gaps?")
            if g is not None and g != count["arm gap"]:
                say("arm gap(s)", g, count["arm gap"])
            pg = stated(b, "pre-existing arm gaps?")
            if pg is not None and pg != count["pre-existing arm gap"]:
                say("pre-existing arm gap(s)", pg, count["pre-existing arm gap"])
        else:
            rm = stated(b, "remarks?")
            if rm is not None and rm != count["remark"]:
                say("remark(s)", rm, count["remark"])
for x in bad:
    print("REVIEW-COUNTS: " + x, file=sys.stderr)
if bad:
    sys.exit(1)
print("review-counts: OK (%d round(s) judged, each record's counts its history's)" % judged)
PY
}

self_test() {
  local work="$ROOT/target/doctrine_scratch/review_counts" arms=0 ok=0
  rm -rf "${work:?}"; mkdir -p "$work/docs/reviews" "$work/docs/decisions"
  git -C "$work" init -q
  world() { # $1 = the sealing-shaped row of round 2, $2 = the split-shaped row of round 1
    printf '| Review history | Design record | Rounds | Status |\n| --- | --- | --- | --- |\n| [`a-reviews.md`](a-reviews.md) | [`a.md`](../decisions/a.md) | 2 | open |\n| [`b-reviews.md`](b-reviews.md) | [`b.md`](../decisions/b.md) | 1 | open |\n' > "$work/docs/reviews/INDEX.md"
    printf '# A\n\n**Round 1**, prose only.\n\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n| R2-AG1 | arm gap | x | y |\n| R2-R1 | remark | x | y |\n| R2-P1 | pre-existing arm gap | x | y |\n| — | found while answering | x | y |\n' > "$work/docs/reviews/a-reviews.md"
    printf '# A\n\n| Round | Findings | Defects | Verdict |\n| --- | --- | --- | --- |\n| 1 | 3 | 1 (prose) | v |\n%s\n' "$1" > "$work/docs/decisions/a.md"
    printf '# B\n\n**Round 1**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R1-D1 | defect | x | y |\n| R1-D2 | defect | x | y |\n| R1-R1 | remark | x | y |\n' > "$work/docs/reviews/b-reviews.md"
    printf '# B\n\n| Round | Date | Defects | Outcome |\n| --- | --- | --- | --- |\n%s\n' "$2" > "$work/docs/decisions/b.md"
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
  arm "counts that are the histories' pass, a prose round and an answer's own row not judged" 0 "2 round(s) judged"
  world '| 2 | 5 | 1, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a total off by one is refused" 1 "round 2 says 5 finding(s)"
  world '| 2 | 4 | 2, and 1 arm gap (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "a defect count off is refused" 1 "round 2 says 2 defect(s)"
  world '| 2 | 4 | 1, and 2 arm gaps (x), and 1 pre-existing arm gap (y) | v |' "$GOOD_B"
  arm "an arm-gap count off is refused" 1 "round 2 says 2 arm gap(s)"
  world '| 2 | 4 | 1, and 1 arm gap (x), and 0 pre-existing arm gaps (y) | v |' "$GOOD_B"
  arm "a pre-existing arm-gap count off is refused" 1 "round 2 says 0 pre-existing arm gap(s)"
  world "$GOOD_A" '| 1 | `2026-10-10` | 2, and 2 remarks | o |'
  arm "a remark count off is refused" 1 "round 1 says 2 remark(s)"
  world "$GOOD_A" '| 1 | `2026-10-10` | 3, and 1 remark | o |'
  arm "a split record's defect count off is refused" 1 "round 1 says 3 defect(s)"
  world "$GOOD_A" "$GOOD_B"; printf '\n**Round 2**, a table.\n\n| # | Kind | Finding | Answer |\n| --- | --- | --- | --- |\n| R2-D1 | defect | x | y |\n' >> "$work/docs/reviews/b-reviews.md"; git -C "$work" add -A
  arm "a round in the history with no summary row is refused" 1 "round 2 is in its history and has no summary row"
  world "$GOOD_A" "$GOOD_B"; sed -i.bak 's/^| 2 | 4 |/| 2 | 5 |/' "$work/docs/decisions/a.md"; rm -f "$work/docs/decisions/a.md.bak"
  arm "the staged copy is judged, not the disk's" 0 "2 round(s) judged"
  rm -rf "${work:?}"
  echo "review-counts self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test ;;
  "") check ;;
  *) echo "usage: $0 [--self-test]" >&2; exit 2 ;;
esac
