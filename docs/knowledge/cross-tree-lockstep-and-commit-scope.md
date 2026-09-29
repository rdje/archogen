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

⚠️ **Re-measured `2026-09-29`, and the gate no longer refuses this.** `PROGRAM.21` made
`TASK-ACCEPTANCE` **leaf-scoped**: it takes the owner from `TASK_ACCEPTANCE_LEAF` or the `(leaf <ID>)`
token in the pending message's subject, finds that leaf among the staged tree files, and checks **only its
boxes** — a second staged tree with an unticked or absent checklist no longer blocks anything. Verified
rather than read off the changelog: `ARCHOGEN-M1-0105` staged `crates/` plus `docs/tasks/M1.md` and
`docs/tasks/PROGRAM.md`, the second carrying only `pending` boxes, and the check printed
`task-acceptance: OK (leaf M1.28.2 in docs/tasks/M1.md — …)`.

**Split the commit anyway, for traceability rather than for the gate.** Code and its owning tree go
together; the note in the other tree goes as its own docs-only commit, so `git log --grep <work-unit-id>`
reconstructs one unit and a reader of either tree sees a commit whose subject names it.
`TASK-ACCEPTANCE` governs only commits that stage code (`.doctrine/code_paths.txt`), so the second commit
passes without anyone inventing a checklist.

Do **not** write an unticked or hand-waved checklist into tree B. A ticked box is a claim that
a tool was run; a box ticked to satisfy a gate is the exact thing the gate exists to prevent,
and it costs more than the extra commit saves.

## Why the check was like this, and what replaced the strictness

It was not a bug so much as the conservative closure of a real hole. The check's own header records that
it was hardened after two **measured** leakage modes, one of which was precisely cross-file: a co-staged
unrelated tree supplied the evidence signature for a leaf that carried none. Requiring every staged tree to
carry its own complete checklist closed that, at the cost of this false positive.

⭐ **The replacement closes the hole without the false positive, and it is worth knowing which property
did the work.** Leaf-scoping removes the *ambiguity* rather than demanding more checklists: the owner is
named by the author, and when it cannot be determined the check **refuses** instead of reading the first
checklist in the file — which was the actual leakage. A co-staged unrelated tree can no longer supply
evidence for a leaf that carries none, because the check never looks at it. So the split above is now
housekeeping, not a workaround, and a session that finds this card's original error message should suspect
it is reading a pre-`PROGRAM.21` checkout.

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
