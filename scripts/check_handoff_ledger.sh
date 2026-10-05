#!/usr/bin/env bash
# scripts/check_handoff_ledger.sh — HANDOFF-LEDGER: every hand-off a design record makes is carried, word for word, by
# the leaf it names (leaf `PROGRAM.52`, docs/decisions/decision_executable-design-reviews.md).
#
# ⭐ WHY THIS EXISTS. Across `M3.1.1`'s fifteen review rounds and `M3.6.1`'s six, the largest late class of defect was a
# hand-off: a record said another leaf owns a question or a constraint, and that leaf's acceptance did not say it, or
# said it in weaker words. A hand-off is a string equality across two files, so it is checked as one.
#
# THE LEDGER. A record holds its hand-offs in one table marked `<!-- machine-read: handoffs -->`, three columns:
#   | Id | Leaf | Obligation |      e.g.  | `SR-H4` | `M3.4` | the closure rooted at the system … |
# The identifier is `<PREFIX>-H<number>`, unique across every ledger; the leaf is a task-tree leaf; the obligation is
# one sentence, with no `|` in it.
#
# THE QUOTE. The receiving leaf — its block in `docs/tasks/*.md` from its `- ID:` line to the next, or, once sealed,
# in `docs/task-history/` — holds `[<Id>] <obligation>`, the same characters, whitespace and line wraps apart.
#
# REFUSED: a row whose sentence its leaf does not quote; a row naming a leaf found nowhere; an identifier malformed or
# held by two rows; a quote `[<PREFIX>-H<n>]` in a leaf whose identifier no ledger holds, or whose ledger names
# another leaf, or whose text after the tag is not the row's sentence. So a hand-off can be neither dropped nor
# carried in weaker words: there is one sentence, and it is checked in both places.
#
# THE POPULATION: ledgers are found in every tracked `.md` outside docs/tasks/, docs/task-history/ and docs/reviews/;
# quotes in every leaf block of docs/tasks/ and docs/task-history/. No ledger and no quote is a clean tree, not a
# breach: a record with nothing to hand over keeps no ledger.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs the RED arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

check() {
  python3 - <<'PY'
import re, subprocess, sys
MARK = "<!-- machine-read: handoffs -->"
ID = re.compile(r"^[A-Z]{2,6}-H[0-9]+$")
TAG = re.compile(r"\[([A-Z]{2,6}-H[0-9]+)\]")
fails = []
tracked = subprocess.run(["git", "ls-files", "--", "*.md"], capture_output=True, text=True).stdout.split()
skip = ("docs/tasks/", "docs/task-history/", "docs/reviews/")
norm = lambda s: re.sub(r"\s+", " ", s).strip()
def cells(line):
    parts = [c.strip() for c in re.split(r"(?<!\\)\|", line.strip())]
    return parts[1:-1] if len(parts) >= 2 else []
rows = {}
for path in tracked:
    if path.startswith(skip):
        continue
    try:
        lines = open(path, encoding="utf-8").read().split("\n")
    except (OSError, UnicodeDecodeError):
        continue
    for at, line in enumerate(lines):
        if line.strip() != MARK:
            continue
        seen_header = seen_sep = False
        for k in range(at + 1, len(lines)):
            t = lines[k].strip()
            if not t and not seen_header:
                continue
            if not t.startswith("|"):
                break
            if not seen_header:
                seen_header = True; continue
            if not seen_sep:
                seen_sep = True; continue
            c = cells(t)
            if len(c) != 3:
                fails.append("%s:%d — a ledger row has %d cells, not Id | Leaf | Obligation" % (path, k + 1, len(c))); continue
            hid, leaf, sentence = c[0].strip("`"), c[1].strip("`"), norm(c[2])
            if not ID.match(hid):
                fails.append("%s:%d — `%s` is not an identifier of the form PREFIX-H<number>" % (path, k + 1, hid)); continue
            if hid in rows:
                fails.append("%s:%d — `%s` is already held by %s" % (path, k + 1, hid, rows[hid][2])); continue
            if not sentence:
                fails.append("%s:%d — `%s` has no obligation" % (path, k + 1, hid)); continue
            rows[hid] = (leaf, sentence, "%s:%d" % (path, k + 1))
# Leaf blocks, open trees first, then sealed history.
blocks = {}
for path in [p for p in tracked if p.startswith("docs/tasks/")] + [p for p in tracked if p.startswith("docs/task-history/")]:
    try:
        text = open(path, encoding="utf-8").read().split("\n")
    except (OSError, UnicodeDecodeError):
        continue
    cur = None; buf = []
    def close():
        if cur is not None and cur not in blocks:
            blocks[cur] = (path, norm("\n".join(buf)))
    for n, line in enumerate(text, 1):
        m = re.match(r"^- ID: `([^`]+)`", line)
        if m or line.startswith("## "):
            close(); cur = m.group(1) if m else None; buf = []
            continue
        if cur is not None:
            buf.append(line)
        else:
            for t in TAG.finditer(line):
                fails.append("%s:%d — `[%s]` stands outside every leaf's block; a hand-off is quoted by the leaf that takes it"
                             % (path, n, t.group(1)))
    close()
for hid, (leaf, sentence, where) in sorted(rows.items()):
    if leaf not in blocks:
        fails.append("%s — `%s` names leaf `%s`, found in no task tree or sealed history" % (where, hid, leaf)); continue
    if ("[%s] %s" % (hid, sentence)) not in blocks[leaf][1]:
        fails.append("%s — `%s`'s obligation is not quoted word for word in `%s` (%s); quote it as `[%s] <the sentence>`"
                     % (where, hid, leaf, blocks[leaf][0], hid))
quotes = 0
for leaf, (path, body) in sorted(blocks.items()):
    for m in TAG.finditer(body):
        quotes += 1
        hid = m.group(1)
        if hid not in rows:
            fails.append("%s, leaf `%s` — quotes `[%s]`, which no ledger holds" % (path, leaf, hid)); continue
        want_leaf, sentence, where = rows[hid]
        if want_leaf != leaf:
            fails.append("%s, leaf `%s` — quotes `[%s]`, which %s hands to `%s`" % (path, leaf, hid, where, want_leaf)); continue
        if not body[m.start():].startswith("[%s] %s" % (hid, sentence)):
            fails.append("%s, leaf `%s` — the text after `[%s]` is not %s's sentence" % (path, leaf, hid, where))
for f in fails:
    print("HANDOFF-LEDGER: " + f, file=sys.stderr)
if fails:
    print("HANDOFF-LEDGER: %d breach(es) — every hand-off a record makes is quoted word for word by the leaf it names" % len(fails), file=sys.stderr)
    sys.exit(1)
ledgers = len({w.rsplit(":", 1)[0] for (_, _, w) in rows.values()})
print("handoff-ledger: OK (%d hand-off(s) in %d ledger(s), each quoted word for word by its leaf; %d quote(s), each held)"
      % (len(rows), ledgers, quotes))
PY
}

self_test() {
  local arms=0 ok=0 work="$ROOT/target/doctrine_scratch/handoff_ledger/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/decisions" "$work/docs/tasks" "$work/docs/task-history/T"
    git -C "$work" init -q
    printf '# R\n\n<!-- machine-read: handoffs -->\n| Id | Leaf | Obligation |\n| --- | --- | --- |\n| `RX-H1` | `T.2` | every input has exactly one `outcome` |\n' > "$work/docs/decisions/decision_r.md"
    printf '# T\n\n- ID: `T.1`\n  Status: `pending`\n\n- ID: `T.2`\n  Status: `pending`\n  Acceptance: [RX-H1] every input has exactly\n  one `outcome`; and more.\n\n## Current Frontier\n' > "$work/docs/tasks/T.md"
    git -C "$work" add -A
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry ("" for a pass)
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    git -C "$work" add -A
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh; arm "a hand-off quoted word for word, across a line wrap, passes" 0 ""
  fresh; sed -i.bak 's/Acceptance: \[RX-H1\] every input has exactly/Acceptance: every input has exactly/' "$work/docs/tasks/T.md"; rm -f "$work/docs/tasks/T.md.bak"
  arm "a hand-off its leaf does not quote is refused" 1 "is not quoted word for word in \`T.2\`"
  fresh; sed -i.bak 's/every input has exactly$/every input has at most/' "$work/docs/tasks/T.md"; rm -f "$work/docs/tasks/T.md.bak"
  arm "a hand-off carried in weaker words is refused" 1 "the text after \`[RX-H1]\` is not"
  fresh; awk 'NR==4{print; print "  Goal: [RX-H9] something no ledger says"; next} {print}' "$work/docs/tasks/T.md" > "$work/t" && mv "$work/t" "$work/docs/tasks/T.md"
  arm "a quote whose identifier no ledger holds is refused" 1 "which no ledger holds"
  fresh; printf -- '- `2026-10-05`: see [RX-H1] every input has exactly one `outcome`\n' >> "$work/docs/tasks/T.md"
  arm "a quote outside every leaf's block is refused" 1 "stands outside every leaf's block"
  fresh; awk 'NR==4{print; print "  Goal: [RX-H1] every input has exactly one `outcome`"; next} {print}' "$work/docs/tasks/T.md" > "$work/t" && mv "$work/t" "$work/docs/tasks/T.md"
  arm "a quote in a leaf the ledger does not name is refused" 1 "which docs/decisions/decision_r.md:6 hands to \`T.2\`"
  fresh; printf '\n<!-- machine-read: handoffs -->\n| Id | Leaf | Obligation |\n| --- | --- | --- |\n| `RX-H1` | `T.2` | again |\n' > "$work/docs/decisions/decision_s.md"
  arm "one identifier in two ledgers is refused" 1 "is already held by"
  fresh; printf '| `RX-H2` | `T.9` | a leaf that is nowhere |\n' >> "$work/docs/decisions/decision_r.md"
  arm "a hand-off to a leaf found nowhere is refused" 1 "found in no task tree or sealed history"
  fresh; printf '| `rx-h3` | `T.2` | lower case |\n' >> "$work/docs/decisions/decision_r.md"
  arm "a malformed identifier is refused" 1 "is not an identifier of the form"
  fresh; printf '| `RX-H4` | `T.3` | sealed and still carried |\n' >> "$work/docs/decisions/decision_r.md"
  printf '# T.3\n\n- ID: `T.3`\n  Status: `done`\n  Acceptance: [RX-H4] sealed and still\n  carried.\n' > "$work/docs/task-history/T/T.3.md"
  arm "a hand-off quoted by a sealed leaf passes" 0 ""
  fresh; rm -f "$work/docs/decisions/decision_r.md"; sed -i.bak 's/\[RX-H1\] //' "$work/docs/tasks/T.md"; rm -f "$work/docs/tasks/T.md.bak"
  arm "no ledger and no quote is a clean tree" 0 ""
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real tree's ledgers are each quoted by their leaves"
  else echo "SELF-TEST: the real tree is refused — run the check to see why" >&2; fi
  echo "handoff-ledger self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}
if [ "${1:-}" = "--self-test" ]; then
  self_test
  exit $?
fi
check
