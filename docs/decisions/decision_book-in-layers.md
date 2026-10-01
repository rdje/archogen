# The book is written in layers: plain words first, the precise rules after

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active`
- **Owner / source:** the director, `2026-10-02`: "Describe what you are doing in a incremental way. A student or
  newbie shall be able to read the book without being scared away. An expert SW engineer or embedded SW engineer
  shall not be bored the reading the book"; and, the same day, "be sure to include and keep a live glossary for
  acronyms … have annexes for really technicals and gory details you can't put in the normal chapters … keep in
  index at the end of the book". Carried out by leaf `PROGRAM.47`. (A contents page was asked for and withdrawn
  the same day: the sidebar [mdBook](../book/src/ledger.md#mdbook) builds from `SUMMARY.md` is the book's table of contents.)

## The fact / decision

Every chapter of the mdBook builds up in the same order:

1. **The idea, in plain words.** What the chapter is about and why it matters, before any rule, table, code path
   or section number. Every term is defined where it first appears, or linked to the glossary; an everyday
   comparison where it helps. A reader who stops here knows what the chapter is for.
2. **In one minute, for engineers.** A short box with the dense summary an embedded engineer would want, so an
   expert can skip the first layer without missing anything.
3. **How it works.** The concepts one at a time, each with a small example.
4. **The precise rules.** Signposted as such, for implementers and reviewers: exact behaviour, edge cases, and
   the normative documents and code they rest on. Nothing here is simplified; this layer is where rigour lives.
5. **Today and ahead.** What exists and is checked, and what does not yet — the tour's rule, scrupulous honesty
   about today versus tomorrow.

Three parts frame the chapters:

- **A glossary**, *Words this book uses*: every acronym the book uses, spelled out and explained, and the recurring
  terms in plain words, each linked to the chapter that treats it precisely. It is **live**: a gate, `BOOK-GLOSSARY`,
  refuses a commit whose book text uses an acronym the glossary does not define, or a glossary entry no chapter
  uses, so it cannot fall behind the chapters.
- **Annexes**, after the chapters, for the full mechanics a chapter states only in outline: the fault contract case
  by case, the timing composition, the catalog check's protection, the protocols archogen speaks. A chapter keeps
  the idea and the rules a reader needs, and points to its annex for the rest.
- **An index**, at the end: every glossary term and every chapter and annex heading, alphabetical, each linking to
  where it is treated. It is generated, never hand-kept, by `scripts/book_index.sh`, and a gate, `BOOK-INDEX`,
  refuses a commit whose index differs from what the generator writes — the Knowledge Map's pattern.

## The book's ceilings

The ruling grows the book on purpose: a glossary, an index, annexes and a plain opening in every chapter. So, with
this record, `docs/book/`'s ceilings in `README_POLICY.md` rise: its files from 32 to **48**, for the glossary, the
index and the annexes; its total bytes from 294 912 (288 KiB) to **393 216 (384 KiB)**. Measured when raised,
`2026-10-02`: the glossary took the book to 295 147 bytes, past the old total; an index of a few tens of kilobytes
and an opening of one or two kilobytes in each of the chapters account for most of the rest, and annexes mostly
move text rather than add it. The per-file ceilings stay — 750 lines, 48 KiB, 1 024 bytes a line — so a chapter
that outgrows them sheds its details into an annex rather than raising them.

## Why

A book read only by experts can open with the rule; this one is the director's only view of the project and the
first thing a student reads. Layering keeps both: the plain layer does not dilute the precise one, it precedes it.

## How to apply

- A chapter is rewritten in layers when it is next touched, and the rest one per leaf under `PROGRAM.47`, the
  chapters a newcomer meets first leading.
- A change to a normative text updates its chapter in the same commit; the update goes into the layer it belongs to
  — a new rule into the precise layer or its annex, and the plain layer only if the idea itself changed.
- The order of the work: this record, the glossary with its gate, the index with its generator and gate, then the
  chapters one leaf each, the runtime chapter first, moving what is gory into annexes as each is rewritten.
