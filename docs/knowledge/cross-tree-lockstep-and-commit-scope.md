---
slug: cross-tree-lockstep-and-commit-scope
answers:
  - "TASK-ACCEPTANCE refuses a task tree I only added a note to — what do I do?"
  - "How do I propagate a blocker into another tree without breaking the commit?"
  - "Why does a docs-only edit to a second task tree block a code commit?"
type: knowledge
date: 2026-09-13
---

# A commit carries one owning leaf; a second tree's note is its own commit

## The question

A code change owned by tree A also needs a note in tree B — a blocker, a dependency, a
changed expectation. You stage both and `TASK-ACCEPTANCE` refuses:

```text
TASK-ACCEPTANCE: docs/tasks/M5.md has no 'ROOT CAUSE' box in its acceptance checklist.
TASK-ACCEPTANCE: docs/tasks/M5.md has no 'ADDRESSED' box in its acceptance checklist.
TASK-ACCEPTANCE: docs/tasks/M5.md has no 'NO REGRESSION' box in its acceptance checklist.
=== 1 doctrine breach(es) — commit blocked ===
```

Tree A's checklist is complete and evidenced. Tree B landed no code and has nothing to tick.

## The answer

**Split the commit.** Code and its owning tree go together; the note in the other tree goes as
its own docs-only commit. `TASK-ACCEPTANCE` governs only commits that stage code
(`.doctrine/code_paths.txt`), so the second commit passes without anyone inventing a checklist.

Do **not** write an unticked or hand-waved checklist into tree B. A ticked box is a claim that
a tool was run; a box ticked to satisfy a gate is the exact thing the gate exists to prevent,
and it costs more than the extra commit saves.

## Why the check is like this

It is not a bug so much as the conservative closure of a real hole. The check's own header
records that it was hardened after two **measured** leakage modes, one of which was precisely
cross-file: a co-staged unrelated tree supplied the evidence signature for a leaf that carried
none. Requiring every staged tree to carry its own complete checklist closes that. The cost is
this false positive.

The strictness may even be intended — requiring every touched tree to justify itself. That is
why the tracked follow-up (`PROGRAM.8`) forbids any fix that reopens the leakage, and why its
first output may be an upstream report rather than a change.

## What not to do

- **Do not edit the check.** It is portable spine, and a local edit forks a shared standard —
  see [[doctrine-seams-vs-forking-a-check]]. There is no seam for this case yet; that absence
  is what `PROGRAM.8` owns.
- **Do not skip the note.** The reason it was being written is that a blocker in one tree
  changes what another tree can do. Dropping it to fit the gate loses the information the
  commit existed to preserve.
- **Do not `--no-verify`.** CI runs the same enforcer, so it buys nothing but a later failure
  with less context.

## How to apply

1. Stage code + its owning tree + live docs. Commit with the work-unit id and the leaf.
2. Stage the other tree's note. Commit it on its own; `COMMIT.md` allows the work-unit-id
   convention alone for a pure documentation edit.
3. Both commits carry the same work-unit id, so `git log --grep` still reconstructs the unit.
