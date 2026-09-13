---
slug: closing-a-leaf-whose-work-landed-elsewhere
answers:
  - "Another milestone already built what my task-tree leaf describes — do I just delete the leaf?"
  - "How do I close a leaf whose implementation arrived from somewhere else?"
  - "Is a leaf done when its work exists, or when its acceptance is verified?"
type: knowledge
date: 2026-09-13
---

# A leaf is closed by its acceptance, not by its implementation existing

## The question

A task-tree leaf says "build X". By the time you reach it, another tree has built X — better, and
for its own reasons. Do you delete the leaf, mark it done, or build a second X?

## The answer

None of the three. **Re-verify the leaf's acceptance criteria against the thing that actually
exists, and close it on that evidence.** The leaf is a contract about a *property*, and the
property is still unverified even when something that plausibly has it is sitting in the tree.

Concretely, for `S0.2` ("a minimal reader … the three fixtures parse; a malformed fixture
reports a span-localized error") after `M1` had built the whole frontend: the first clause was
already implied by other tests, and the second one — *span-localized* — was asserted **nowhere**
for this corpus. Closing the leaf therefore still cost a test file, and that test file found the
clause that nobody had checked.

## Why

The three wrong answers each lose something specific:

- **Deleting the leaf** loses the acceptance. The criteria were written when the risk was fresh;
  "a malformed fixture reports a span-localized error" is a real property that a working reader
  can lack, and deleting the leaf deletes the only place anyone wrote it down.
- **Ticking it because the work exists** is the failure the whole acceptance checklist exists to
  prevent: a claim in place of a measurement. It is also the cheapest way to end up with a tree
  full of green leaves and no evidence.
- **Building a second implementation** is worse than either, and it is what the roadmap's
  retirement clause explicitly permits you to avoid: "the prototype implementation can be
  discarded or replaced as semantics settle. Keep its functional fixtures."

Note what survives in all cases — the **fixtures and the acceptance**, not the code. That is the
part a leaf is really carrying.

## How to apply

1. Read the leaf's acceptance clause by clause. For each, ask *where is this asserted today?*
2. Whatever is unasserted is the leaf's remaining work, however small. Write that test.
3. Record in the leaf **why** it closed this way — that its implementation arrived from another
   tree — so the next reader does not go looking for the code it describes.
4. Prove the new assertion can fail ([[an-oracle-is-independent-by-construction]]): mutate the
   subject, not the test, and watch it go red.
