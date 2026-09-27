---
slug: a-moved-measurement-needs-a-census-of-its-copies
answers:
  - "I changed a number my tests measure — what else do I have to update?"
  - "Why does the book contradict itself between two chapters?"
  - "How do I stop a measured figure from going stale in prose?"
  - "A doc comment states a measurement. Is that a defect?"
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

1. **Census before editing.** `grep -rnE '[0-9]+ of (the )?[0-9]+' <live surfaces>` — and classify
   every hit as *live* or *record* before touching one. The count of hits is a population, not a
   defect count.
2. **Make the surviving live copy a consumer.** One measurement function, shared by the assertion
   and the gate; the gate reads the prose with `include_str!` so a moved or deleted file stops the
   crate compiling instead of silently gating nothing.
3. **Gate the tense, not just the number.** A figure that is neither the measurement nor marked past
   tense is a new stale claim. Accepting past tense is what keeps history legal.
4. **Red-arm it both ways**: revert the figure (must fire) *and* strip the past tense off a
   legitimate historical figure (must also fire). A gate that only catches the first will be
   satisfied by prose that quietly stops marking its history.
5. Retyping the fresh number and changing nothing else is **not** the fix —
   `docs/CLAIM_VERIFICATION.md` §5B names that move explicitly.

Related: [[prose-beside-data-goes-unenforced]] is the same rot one level down, inside a single
declarative table. [[verify-the-mutation-applied]] — building this gate reproduced that failure
exactly: a `perl` mutation whose backreference was eaten by shell quoting left the crate
uncompilable, and the arm's output filter printed nothing, which read as a pass.
