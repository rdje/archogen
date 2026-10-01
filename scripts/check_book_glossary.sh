#!/usr/bin/env bash
# scripts/check_book_glossary.sh — BOOK-GLOSSARY: the book's glossary is live (leaf `PROGRAM.47.2`).
#
# ⭐ WHY THIS EXISTS. The director ruled on `2026-10-02` that the book keep "a live glossary for acronyms"
# (`docs/decisions/decision_book-in-layers.md`): a student meeting an abbreviation must find it explained. A glossary
# kept by care falls behind the chapters on the first busy day, so this gate holds it to the chapters both ways.
#
# THE RULE, over `docs/book/src/*.md` but `SUMMARY.md`, the glossary itself and the generated index:
#   1. every acronym a chapter uses is a headword of the glossary's *Acronyms and abbreviations* section. An acronym
#      is a word of two or more capitals and digits, hyphenated parts allowed (`API`, `RISC-V`, `SHA-256`), or a
#      lower-case letter before two or more capitals (`eADL`). Not counted: fenced code, inline code, link targets,
#      HTML comments; the project's identifiers, a capital and digits (`M2`, `F26`, `S0`, `R7`); Roman numerals;
#   2. every headword of that section is used by some chapter, so no entry outlives its last use;
#   3. every headword of *Words with a meaning of their own here* appears, as a word, in some chapter;
#   4. each section's entries are in alphabetical order, ignoring case.
# A headword is a bolded word at the head of an entry, before its ` — `; one entry may carry several
# (`**GB**, **KB**, **MB** — …`).
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs its arms in scratch
# repositories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/book_glossary"

check() {
  python3 - <<'PY'
import glob, os, re, sys

BOOK = "docs/book/src"
GLOSSARY = os.path.join(BOOK, "glossary.md")
SKIP = {"SUMMARY.md", "glossary.md", "book-index.md"}
TOKEN = re.compile(r"(?<![\w-])(?:[A-Z][A-Z0-9]+(?:-[A-Z0-9]+)*|[a-z][A-Z]{2,})(?![\w-])")
IDENT = re.compile(r"[A-Z][0-9]+(?:\.[0-9]+)*")
ROMAN = re.compile(r"[IVX]+")

def prose(text):
    """The text a reader reads as words: code, link targets and comments blanked, line structure kept."""
    blank = lambda m: re.sub(r"[^\n]", " ", m.group(0))
    text = re.sub(r"^(```|~~~).*?^\1[^\n]*$", blank, text, flags=re.S | re.M)
    text = re.sub(r"<!--.*?-->", blank, text, flags=re.S)
    text = re.sub(r"`[^`\n]*`", blank, text)
    text = re.sub(r"\]\([^)\n]*\)", lambda m: "]" + " " * (len(m.group(0)) - 1), text)
    return text

if not os.path.exists(GLOSSARY):
    print("BOOK-GLOSSARY: %s is missing" % GLOSSARY, file=sys.stderr)
    sys.exit(1)
sections, current = {}, None
for n, line in enumerate(open(GLOSSARY, encoding="utf-8"), 1):
    if line.startswith("## "):
        current = line[3:].strip()
        sections[current] = []
    elif current and line.startswith("- **"):
        head = line.split(" — ", 1)[0]
        words = re.findall(r"\*\*([^*]+)\*\*", head)
        sections[current].append((n, words))
ACRONYMS, TERMS = "Acronyms and abbreviations", "Words with a meaning of their own here"
fails = []
for name in (ACRONYMS, TERMS):
    if name not in sections:
        fails.append("%s has no section `## %s`" % (GLOSSARY, name))
defined = {w for _, ws in sections.get(ACRONYMS, []) for w in ws}
for name, entries in sections.items():
    keys = [ws[0].lower() for _, ws in entries if ws]
    for (n, ws), prev in zip(entries[1:], keys):
        if ws and ws[0].lower() < prev:
            fails.append("%s:%d: `%s` is out of alphabetical order in `%s`" % (GLOSSARY, n, ws[0], name))

used, text_all = {}, []
for path in sorted(glob.glob(os.path.join(BOOK, "*.md"))):
    if os.path.basename(path) in SKIP:
        continue
    text = prose(open(path, encoding="utf-8").read())
    text_all.append(text)
    for n, line in enumerate(text.split("\n"), 1):
        for m in TOKEN.finditer(line):
            word = m.group(0)
            if IDENT.fullmatch(word) or ROMAN.fullmatch(word):
                continue
            used.setdefault(word, "%s:%d" % (path, n))
for word, where in sorted(used.items()):
    if word not in defined:
        fails.append("%s: `%s` is used and the glossary does not define it — add it to `## %s`, or write it as code if "
                     "it is a name" % (where, word, ACRONYMS))
for n, ws in sections.get(ACRONYMS, []):
    for w in ws:
        if w not in used:
            fails.append("%s:%d: `%s` is defined and no chapter uses it any more — remove the entry" % (GLOSSARY, n, w))
joined = "\n".join(text_all)
for n, ws in sections.get(TERMS, []):
    for w in ws:
        if not re.search(r"(?<![\w-])" + re.escape(w) + r"(?![\w-])", joined, re.I):
            fails.append("%s:%d: `%s` appears in no chapter — remove the entry" % (GLOSSARY, n, w))
for f in fails:
    print("BOOK-GLOSSARY: " + f, file=sys.stderr)
if fails:
    print("BOOK-GLOSSARY: %d breach(es) — the glossary defines every acronym the book uses, and nothing it no longer "
          "uses" % len(fails), file=sys.stderr)
    sys.exit(1)
print("book-glossary: OK (%d acronym(s) used and defined; %d term(s), each in a chapter)"
      % (len(used), len(sections.get(TERMS, []))))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest"
  fresh() { # $1 = a chapter's text, $2 = the glossary's acronym entries, $3 = its term entries
    rm -rf "$work"; mkdir -p "$work/docs/book/src"; git -C "$work" init -q
    printf '%s\n' "$1" > "$work/docs/book/src/a.md"
    printf '# Summary\n\n- [A](a.md) uses API in its title\n' > "$work/docs/book/src/SUMMARY.md"
    printf '# Words\n\n## Acronyms and abbreviations\n\n%s\n## Words with a meaning of their own here\n\n%s\n' "$2" "$3" \
      > "$work/docs/book/src/glossary.md"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  local acr='- **API** — Application Programming Interface.
- **RISC-V** — an instruction set.
' term='- **task** — a piece of work.
'
  fresh 'A task calls an API on RISC-V.' "$acr" "$term"
  arm "a chapter whose acronyms are all defined passes" 0 "2 acronym(s) used and defined"
  fresh 'A task calls an API on RISC-V over a UART.' "$acr" "$term"
  arm "an acronym the glossary does not define is refused" 1 "\`UART\` is used and the glossary does not define it"
  fresh 'A task calls an API on RISC-V; `UART` in code, a [link](x/UART.md), <!-- UART -->, and
```text
UART
```' "$acr" "$term"
  arm "code, link targets and comments are not words" 0 "2 acronym(s)"
  fresh 'A task calls an API on RISC-V, as M2 and F26 say, in volume II.' "$acr" "$term"
  arm "project identifiers and Roman numerals are not acronyms" 0 "2 acronym(s)"
  fresh 'A task calls an API.' "$acr" "$term"
  arm "an entry no chapter uses is refused" 1 "\`RISC-V\` is defined and no chapter uses it"
  fresh 'An API on RISC-V.' "$acr" "$term"
  arm "a term no chapter uses is refused" 1 "\`task\` appears in no chapter"
  fresh 'A task calls an API on RISC-V.' '- **RISC-V** — an instruction set.
- **API** — Application Programming Interface.
' "$term"
  arm "an entry out of alphabetical order is refused" 1 "\`API\` is out of alphabetical order"
  fresh 'A task calls an API on RISC-V with eADL.' "$acr" "$term"
  arm "a lower-case-led acronym is counted" 1 "\`eADL\` is used and the glossary does not define it"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the real book passes"
  else echo "SELF-TEST: the real book is refused — run the check to see why" >&2; fi
  echo "book-glossary self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") check; exit $? ;;
  *) echo "usage: bash scripts/check_book_glossary.sh [--self-test]" >&2; exit 2 ;;
esac
