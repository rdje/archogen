#!/usr/bin/env bash
# scripts/check_book_index.sh — the book's index, generated (leaf `PROGRAM.47.3`); BOOK-INDEX, the gate that keeps it so.
#
# ⭐ WHY THIS EXISTS. The director ruled on `2026-10-02` that the book "keep [an] index at the end"
# (`docs/decisions/decision_book-in-layers.md`). A hand-kept index is wrong the day a heading moves, so this one is
# written by this script from the chapters themselves, and checked on every commit to be what the script writes.
#
# WHAT THE INDEX HOLDS, alphabetically, under a letter each:
#   - every headword of the glossary (`docs/book/src/glossary.md`), with the chapters that use it — an acronym where
#     it appears as written, a term as a word in any case — and a link to its definition;
#   - every section heading (`##`, `###`) of every chapter, linked to the section.
# A section's link is the anchor mdBook writes, computed as mdBook 0.5.2 does — measured `2026-10-02` on a probe book:
# the heading's text without its markup, lower-cased, letters, digits, `-` and `_` kept, each space a `-`, every other
# character dropped, and a repeat on one page suffixed `-1`, `-2`. The self-test pins the measured cases.
#
# usage: bash scripts/check_book_index.sh            # BOOK-INDEX: the index is what this script writes (exit 1 if not)
#        bash scripts/check_book_index.sh --write    # write it
#        bash scripts/check_book_index.sh --self-test
# CONTRACT: the check is read-only and explains on stderr; `--self-test` runs its arms in scratch repositories under
# `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/book_index"

engine() { # $1 = check | write | anchors
  python3 - "$1" <<'PY'
import glob, os, re, sys, textwrap

MODE = sys.argv[1]
BOOK = "docs/book/src"
INDEX = os.path.join(BOOK, "book-index.md")
SKIP = {"SUMMARY.md", "book-index.md"}

def plain(md):
    """A heading's text as a reader sees it: links to their text, code and emphasis marks removed."""
    md = re.sub(r"\[([^\]]*)\]\([^)]*\)", r"\1", md)
    return md.replace("`", "").replace("**", "").replace("__", "").strip()

def anchor(text, used):
    """The id mdBook 0.5.2 gives a heading with this plain text, given the ids already used on its page."""
    out = "".join(c.lower() if (c.isalnum() or c in "-_") else ("-" if c.isspace() else "") for c in text)
    base, n = out, 0
    while out in used:
        n += 1
        out = "%s-%d" % (base, n)
    used.add(out)
    return out

if MODE == "anchors":
    used = set()
    for line in os.environ.get("ANCHOR_INPUT", "").splitlines():
        print(anchor(plain(line), used))
    sys.exit(0)

def prose(text):
    blank = lambda m: re.sub(r"[^\n]", " ", m.group(0))
    text = re.sub(r"^(```|~~~).*?^\1[^\n]*$", blank, text, flags=re.S | re.M)
    text = re.sub(r"<!--.*?-->", blank, text, flags=re.S)
    text = re.sub(r"`[^`\n]*`", blank, text)
    return re.sub(r"\]\([^)\n]*\)", "]", text)

titles = {}
for line in open(os.path.join(BOOK, "SUMMARY.md"), encoding="utf-8"):
    m = re.search(r"\[([^\]]+)\]\(([^)]+\.md)\)", line)
    if m:
        titles[m.group(2)] = plain(m.group(1))
chapters = [f for f in titles if f not in SKIP and os.path.exists(os.path.join(BOOK, f))]

entries = []  # (sort key, display, [links])
for f in chapters:
    text = open(os.path.join(BOOK, f), encoding="utf-8").read()
    used, fence = set(), None
    for line in text.split("\n"):
        if fence:
            if line.startswith(fence):
                fence = None
            continue
        if line.startswith("```") or line.startswith("~~~"):
            fence = line[:3]
            continue
        m = re.match(r"^(#{1,6})\s+(.*?)\s*#*\s*$", line)
        if not m:
            continue
        shown = plain(m.group(2))
        a = anchor(shown, used)
        if len(m.group(1)) in (2, 3):
            entries.append((shown, shown, ["[%s](%s#%s)" % (titles[f], f, a)]))

texts = {f: prose(open(os.path.join(BOOK, f), encoding="utf-8").read()) for f in chapters if f != "glossary.md"}
gloss = open(os.path.join(BOOK, "glossary.md"), encoding="utf-8").read()
section = None
for line in gloss.split("\n"):
    if line.startswith("## "):
        section = line[3:].strip()
    elif section and line.startswith("- **"):
        for word in re.findall(r"\*\*([^*]+)\*\*", line.split(" — ", 1)[0]):
            acronym = section.startswith("Acronyms")
            pat = re.compile(r"(?<![\w-])" + re.escape(word) + r"(?![\w-])", 0 if acronym else re.I)
            where = [f for f in chapters if f in texts and pat.search(texts[f])]
            links = ["[definition](glossary.md)"] + ["[%s](%s)" % (titles[f], f) for f in where]
            entries.append((word, "**%s**" % word, links))

def key(e):
    k = re.sub(r"^[^0-9A-Za-zÀ-ÿ]+", "", e[0]).lower()
    return (k, e[0])
groups = {}
for e in sorted(entries, key=key):
    k = key(e)[0]
    letter = k[:1].upper() if k[:1].isalpha() else "Numbers and symbols"
    groups.setdefault(letter, []).append(e)

out = ["# Index", "",
       "Looking for a word or a topic? Every abbreviation and term the [glossary](glossary.md) defines is listed here",
       "with the chapters that use it, and every section of every chapter with a link to it.",
       "",
       "This page is written by `scripts/check_book_index.sh` from the chapters themselves, and the `BOOK-INDEX` check refuses",
       "a change that leaves it stale, so it is never edited by hand.", ""]
order = sorted((g for g in groups if g != "Numbers and symbols")) + (["Numbers and symbols"] if "Numbers and symbols" in groups else [])
for g in order:
    out += ["## %s" % g, ""]
    for _, display, links in groups[g]:
        item = "%s — %s" % (display, ", ".join(links))
        out += textwrap.wrap(item, width=120, initial_indent="- ", subsequent_indent="  ",
                             break_long_words=False, break_on_hyphens=False) or ["- " + item]
    out.append("")
generated = "\n".join(out).rstrip("\n") + "\n"

if MODE == "write":
    open(INDEX, "w", encoding="utf-8").write(generated)
    print("book-index: wrote %s (%d entries)" % (INDEX, len(entries)))
    sys.exit(0)
current = open(INDEX, encoding="utf-8").read() if os.path.exists(INDEX) else None
if current != generated:
    have = (current or "").split("\n"); want = generated.split("\n")
    first = next((i for i in range(max(len(have), len(want)))
                  if (have[i] if i < len(have) else None) != (want[i] if i < len(want) else None)), 0)
    print("BOOK-INDEX: %s is %s — regenerate it: bash scripts/check_book_index.sh --write" %
          (INDEX, "missing" if current is None else "stale from line %d" % (first + 1)), file=sys.stderr)
    sys.exit(1)
print("book-index: OK (%d entries, as the chapters and the glossary give them)" % len(entries))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() {
    rm -rf "$work"; mkdir -p "$work/docs/book/src"; git -C "$work" init -q
    printf '# Summary\n\n- [The runtime](a.md)\n\n[Words this book uses](glossary.md)\n\n[Index](book-index.md)\n' \
      > "$work/docs/book/src/SUMMARY.md"
    printf '# A\n\nA task uses an API.\n\n## Why the split\n\n### Third level, it'"'"'s here\n' > "$work/docs/book/src/a.md"
    printf '# Words\n\n## Acronyms and abbreviations\n\n- **API** — x.\n\n## Words with a meaning of their own here\n\n- **task** — y.\n' \
      > "$work/docs/book/src/glossary.md"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, $4 = mode
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" ${4:-} 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh
  arm "a missing index is refused" 1 "is missing"
  (cd "$work" && bash "$SELF" --write >/dev/null)
  arm "the written index passes" 0 "book-index: OK"
  arms=$((arms + 1))
  if grep -qF '[The runtime](a.md#why-the-split)' "$work/docs/book/src/book-index.md" \
     && grep -qF '[The runtime](a.md#third-level-its-here)' "$work/docs/book/src/book-index.md" \
     && grep -qF '**task** — [definition](glossary.md), [The runtime](a.md)' "$work/docs/book/src/book-index.md"; then
    ok=$((ok + 1)); echo "  ✅ sections link to their anchors, and a term to its definition and its chapters"
  else echo "SELF-TEST: the index's links are not the expected ones" >&2; fi
  printf '\n## A new section\n' >> "$work/docs/book/src/a.md"
  arm "a heading added without regenerating is refused" 1 "stale from line"
  (cd "$work" && bash "$SELF" --write >/dev/null)
  printf -- '- a hand-written line\n' >> "$work/docs/book/src/book-index.md"
  arm "a hand edit is refused" 1 "stale from line"
  # The anchors mdBook 0.5.2 wrote for these headings on a probe book, `2026-10-02`.
  arms=$((arms + 1))
  local got want
  got="$(cd "$work" && ANCHOR_INPUT="$(printf '%s\n' 'Masking is modelled, not assumed away' '`qemu`' \
    '⚠️ Today and ahead' 'What a report may claim: `x` and §5.5' 'Same' 'Same' 'Ünïcode café' 'A/B — C (d)' \
    'The `archogen` command line' '**Bold** and [a link](x.md)' "Third level, it's here" '1. Numbered start')" \
    bash "$SELF" --anchors)"
  want="$(printf '%s\n' masking-is-modelled-not-assumed-away qemu -today-and-ahead what-a-report-may-claim-x-and-55 same \
    same-1 ünïcode-café ab--c-d the-archogen-command-line bold-and-a-link third-level-its-here 1-numbered-start)"
  if [ "$got" = "$want" ]; then ok=$((ok + 1)); echo "  ✅ anchors are mdBook 0.5.2's, as measured"
  else echo "SELF-TEST: anchors differ from mdBook's:" >&2; diff <(printf '%s\n' "$want") <(printf '%s\n' "$got") | sed 's/^/    /' >&2; fi
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real book's index is current"
  else echo "SELF-TEST: the real book's index is stale — run the check to see why" >&2; fi
  echo "book-index self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  --write) engine write; exit $? ;;
  --anchors) engine anchors; exit $? ;;
  "") engine check; exit $? ;;
  *) echo "usage: bash scripts/check_book_index.sh [--write | --self-test]" >&2; exit 2 ;;
esac
