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
  where it is treated — but the four headings that mark a chapter's layers, which name a part and no topic. It is generated, never hand-kept, by `scripts/check_book_index.sh`, and a gate, `BOOK-INDEX`,
  refuses a commit whose index differs from what the generator writes — the Knowledge Map's pattern.

## Book growth — no limits from `2026-10-18`

The director removed every book growth limit on `2026-10-18`; see
[The book grows as needed](decision_book-unbounded.md). The reading layers above remain binding.

Earlier rulings are superseded: `PROGRAM.47` raised the file count from 32 to 48 and total bytes
from 294912 to 393216 on `2026-10-02`; `PROGRAM.48` raised total bytes to 458752 on `2026-10-03`;
`PROGRAM.74` raised them to 589824 on `2026-10-10`. Their per-file limits were 750 lines, 49152 bytes
and 1024 bytes per line. None applies now. The full earlier rationale and measurements remain in
Git at `3d622a6251cc912d7f2a2a2510317a84b326703f:docs/decisions/decision_book-in-layers.md`
and in those task records. No future containment or approval is required for the book to grow.

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
