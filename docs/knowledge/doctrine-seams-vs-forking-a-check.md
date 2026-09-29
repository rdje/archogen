---
slug: doctrine-seams-vs-forking-a-check
answers:
  - "A doctrine check misfires on this repository's layout — do I edit the check?"
  - "Why is `docs/book/src/introduction.md` treated as a code change?"
  - "What is `.doctrine/code_paths.txt` for and when do I add a line to it?"
type: knowledge
date: 2026-09-13
---

# A portable check that misfires is a seam question, not a fork question

## The question

A spine doctrine refuses a commit for a reason that is wrong *for this repository's shape*.
Do I edit the check?

## The answer

No. Declare the project's shape in the seam the check already reads. Editing a portable
check to hardcode one project's paths turns a shared standard into a private fork of it, and
every later `scripts/update_scaffold.sh` reports the check as differing and sets the template's copy aside, so
each sync becomes a hand merge of that one edit.

The seams are in `.doctrine/` and documented by `.doctrine/README.md`:

| Seam | Consumed by | Says |
| --- | --- | --- |
| `.doctrine/code_paths.txt` | `TASK-ACCEPTANCE` | what counts as a code change here |
| `.doctrine/evidence_tokens.txt` | `TASK-ACCEPTANCE` | this project's tool output signatures |

## The measured instance

`TASK-ACCEPTANCE`'s neutral default is

```
(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$
```

Its `(^|/)src/` arm matches a path segment named `src` **at any depth**. This repository's
mdBook sources live at `docs/book/src/`, so editing one book page was classified as a code
change, and the check then demanded a ticked ROOT CAUSE / ADDRESSED / NO REGRESSION
checklist on all ten task trees staged in the same commit — thirty refusal lines for a
markdown edit.

```
$ git diff --cached --name-only --diff-filter=ACM \
  | grep -E '(^|/)(crates|src|scripts)/|\.(rs|sh)$|(^|/)Makefile$'
docs/book/src/introduction.md
```

The default is right to ship: a bare Rust workspace *does* keep code in `src/`. The project
is the only thing that knows it also keeps prose there.

## How to apply

1. Reproduce the misfire with the check's own predicate, on the real staged set. Do not
   reason about the regex — run it.
2. Write the seam line, and put the measurement **in the seam file** so the next reader
   inherits the evidence rather than the conclusion.
3. Confirm red → green with the full enforcer, and record it in the owning leaf.
4. If no seam exists for the case, that is a `WAIVER-ROUTING` situation: write the waiver
   and name the leaf that owns building the seam. An author hitting a gate's boundary is the
   highest-signal defect report the gate can receive.

Related: [[decision_eadl-engine-boundary]] applies the same instinct to a different
boundary — state where a thing belongs, rather than bending the thing that enforces it.
