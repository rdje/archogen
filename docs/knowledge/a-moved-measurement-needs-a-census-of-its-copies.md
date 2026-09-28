---
slug: a-moved-measurement-needs-a-census-of-its-copies
answers:
  - "I changed a number my tests measure — what else do I have to update?"
  - "Why does the book contradict itself between two chapters?"
  - "How do I stop a measured figure from going stale in prose?"
  - "A doc comment states a measurement. Is that a defect?"
  - "My census grep found nothing — is the population empty, or is my pattern wrong?"
  - "My figure gate is green on the defect it was written for. What excused it?"
type: knowledge
date: 2026-09-27
---

# A moved measurement needs a census of its copies, not a memory of them

## The question

A test measures something — a corpus size, a coverage count, a reach. You change the thing
measured, update the test, and the suite goes green. What else did you have to update?

## The answer

**Census the population of copies first, then decide per copy.** The set of surfaces restating a
measurement is not the set you remember writing, and "I updated the test" says nothing about them.

Then apply the only distinction that matters:

| A surface that … | Must … |
| --- | --- |
| states the figure as **current** | become a *consumer* of the measurement — compared against it by a gate, or reduced to naming the test that measures |
| states the figure as **history** (past tense, dated, in a changelog or a closed leaf) | be left exactly alone — a record of a commit's state is never re-derived |
| is a **source header or doc comment** | carry **no figure at all**. State the rule and name the measuring test |

The third row is the one that surprises people. A doc comment feels like documentation, so it feels
safe to put a number in. It is the *least* safe place: nothing reads it, no build touches it, and it
is the surface a future engineer trusts most.

## Why

Measured in this repository, not anticipated. `M1.2` recorded that the schema refused **10 of the 11**
rejected boundary cases and that the boundary classifier caught the eleventh. `M1.7` closed that gap
— `(clause task … (holds kind task))` makes the schema recurse — grew the corpus to 13, rewrote the
test to assert `out_of_reach.is_empty()`, and updated `docs/book/src/workload.md`.

It did not update the two other copies:

```console
$ grep -rn '10 of the 11' crates/*/src/*.rs docs/book/src/*.md
crates/eadl-model/src/kind.rs:35://! the schema refuses 10 of the 11 rejected cases …
docs/book/src/kinds.md:70:schema refuses **10 of the 11** rejected cases …
```

Both were **still asserting it 47 commits later** (`git rev-list --count 9030111..HEAD` → `47`). The
figure had been true when written — `git ls-tree --name-only b53eb85 docs/semantics/boundary/reject/`
→ `11` — which is exactly why nobody doubted it.

⭐ The expensive part was not the staleness but the **self-contradiction**: `kinds.md` said 10 of 11
while its sibling chapter `workload.md` said 13 of 13, both reachable from `SUMMARY.md`. The book is
the director's only window into the project, so the two chapters disagreed in the one place anyone
reads. `BOOK-ANCHORS` could not see it: that gate proves a chapter *points at* something real, never
that what it says there is true.

⛔ And note what the census found besides the two defects: **7 further occurrences that were
correct** — past-tense history in `workload.md`, a "this test previously asserted" comment, two
`CHANGELOG.md` entries, and three inside closed leaves and a Decisions log. Fixing those would have
*rewritten history to look current*, which is its own defect. A census you do not classify will tell
you to damage correct records.

## How to apply

1. **Census before editing — with as many patterns as the figure has spellings.** A census is a
   population *bounded by its pattern*, so the pattern is part of the claim and travels with it.
   Measured twice over here: the `N of M` pattern found `M1.23`'s two stale copies and **could not
   see** a corpus *size* (`all 21 corpus files`), which is not an `N of M` figure; a
   digits-plus-size-noun pattern then found that one and **could not see** `the five ambiguous
   cases`, because it is spelled out. Three patterns, four live-false figures, and one commit
   (`9030111`) behind all of them. Sweep every shape the figure can take — ratio, size, spelled-out
   number — and state which shape each hit came from. A census that reports "nothing found" without
   naming its pattern has not measured a population.
   ⛔ **The worst case is not an under-count, it is zero — because a gate comparing two empty sets
   passes.** Measured building the diagnostic-code census (`M1.12.3`): the grep shape someone reaches
   for, `Diagnostic::error("…"` with the code on the constructor's own line, finds **0 of 7** codes in
   `crates/eadl-front/src/reader.rs`, **0 of 24** in `module.rs` and **0 of 22** in
   `crates/eadl-model/src/kind.rs`, because every call in that codebase puts the code on the *next*
   line. A census leg built on it would not have reported a short list; it would have reported that the
   code and the document agree perfectly, having compared nothing with nothing. Two defences, and the
   second is the one that generalises: **assert the population is non-empty** before comparing it, and
   **run both directions** — a producer→document leg alone passes on an empty producer set, but adding
   the document→producer leg means every row the document states is now unaccounted for, so the same
   broken pattern fails loudly instead of quietly.
   ⭐ **And the population is not only numbers.** A *state fact* has copies too, and they rot the same
   way. Measured here: **eight files** published "QEMU is not installed" after it was — including the
   register that routes items to the director, whose index hook said "four items" where the record
   carried six and omitted the two that actually needed rulings, and a book chapter whose tier
   transcript was wrong in *shape* (`incomplete`, exit `20`) and not merely in fact (the tier now
   reports `failed`, exit `1`). Nothing compared the prose with the tool's own verdict, because a
   tool's availability is not a figure anyone thinks to gate. A register that cannot mark an item
   **resolved** without rewriting it will be rewritten instead, so record the resolution above the
   original text and keep the original verbatim: a register that silently edits its own history cannot
   be audited.
   ⛔ **Census the concept, not the phrasings — and this card's own author got it wrong twice in two
   commits.** The first count of that population was "five files", produced by grepping for wordings
   already seen (`not installed|no QEMU|unavailable|install it, then re-run|…`). It could not see
   `currently absent on this machine`, and it could not see `UNAVAILABLE — not on PATH` inside a
   fenced console block. The bounded population was available the whole time:
   `git grep -il <the subject>` → 24 files, each then classified as *live*, *conditional*, or
   *record*. **Enumerate the files that could state the fact, not the wordings it might use.** A
   phrasing census under-reports silently, and an under-reported population reads exactly like a
   finished one.
2. **Make the surviving live copy a consumer.** One measurement function, shared by the assertion
   and the gate; the gate reads the prose with `include_str!` so a moved or deleted file stops the
   crate compiling instead of silently gating nothing.
3. ⛔ **Gate an explicit history list, never a tense.** "A figure that is neither the measurement
   nor marked past tense is a new stale claim" is the right *rule* and a trap as an
   *implementation*: scanning the line for `was`/`before`/`previously` is satisfiable by unrelated
   prose. Measured on the first run of the gate built from this card — it reported green on the
   exact defect it existed to catch, because the line reads
   `…worked classification case before adoption", and the five ambiguous cases…` and the `before`
   belongs to a quotation of the roadmap, seven words ahead of the stale number. A historical figure
   must be something an author **lists in a constant**, where the addition is visible in review, not
   something neighbouring words can excuse by accident. History stays legal; only the mechanism
   changes.
4. **Red-arm it both ways, permanently, and pin the count.** Revert the figure (must fire) *and*
   strip the marker off a legitimate historical figure (must also fire). Two refinements this card
   learned the hard way: make the arms **repeatable** by factoring the comparison into a function
   that returns its violations, then feeding it the prose that was *actually* wrong — a one-off
   mutation of the working tree proves the gate fired once, for one author, on one day, and is
   exactly the gap `PROGRAM.18` files against the shell controls. And assert **how many** violations
   came back, not merely that one mentions the right thing: an arm that only checks a substring also
   passes on a gate that started reporting everything. Measured — appending one noise violation
   failed all 7 arms and the gate, and left the 6 unrelated tests green.
5. Retyping the fresh number and changing nothing else is **not** the fix —
   `docs/CLAIM_VERIFICATION.md` §5B names that move explicitly.

Related: [[prose-beside-data-goes-unenforced]] is the same rot one level down, inside a single
declarative table. [[verify-the-mutation-applied]] — building this gate reproduced that failure
exactly: a `perl` mutation whose backreference was eaten by shell quoting left the crate
uncompilable, and the arm's output filter printed nothing, which read as a pass.
