# Generated sources' refused shapes and believed tool: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active`
- **Owner / source:** leaf `M3.6.6.4` (`docs/tasks/M3.md`). This is the review history of
  [`decision_trust-generated-refusals.md`](../specs/trust/decision_trust-generated-refusals.md), kept apart from it as
  `docs/reviews/INDEX.md` describes.

## The fact / decision

**Round 1**, `2026-10-10`, of `f3f1ad0`, by a read-only context that had not written the record. It reproduced the
measurement at `614d5c7`, ran the crate's tests (163 passed) and the whole mutation catalogue (*"OK — 333 mutation(s)"*),
checked the code moved only messages, and probed each route with fixtures of its own — its module untracked, so its
outcomes are not durable. Verdict: 8 defects and 8 remarks.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R1-D1 | defect | the chain route's program-target step "becomes a later step of a script that runs both": its build, made from the commit, cannot compile an uncommitted intermediate, and a committed one is a chain or escapes as a plain file | the later step reads the intermediate at run time, handed over by the script, its build compiling no generated file; one that must compile it has no route (§5) |
| R1-D2 | defect | the route named "the first step's committed inputs", leaving a later step's committed inputs out of every provenance | "every committed file any step reads" |
| R1-D3 | defect | a committed copy or pin whose header marks it generated is a chain the parent refuses, so "each need has a route" was false | the routes hold for unmarked files; a marked one has no route (§5), said in §1, §3, §4 and the chapter |
| R1-D4 | defect | the gitlink route served an input alone | a vendored generator's route: a committed script runs it, the vendored one that script's believed tool |
| R1-D5 | defect | "their messages name this record": only the chain's do | "the chain's refusals name this record"; the changelog's entry corrected |
| R1-D6 | defect | R1-6 was answered by transitive provenance, not left open | the history as it was: transitive provenance, then R3-5 and R3-7, then the narrowing |
| R1-D7 | defect | "the form's `command` names the tool": the command names the script | the tool is named in the script the form names as its generator |
| R1-D8 | defect | the leaf's `grep -n "R3-7"` output | `97`, `112` |
| R1-R1 | remark | "a chain hides nothing" holds for one the parent's rules see | qualified: a chain through an unmarked, undeclared file is the parent's §8 limit |
| R1-R2 | remark | a change of tool is a provenance change only through the pin | said |
| R1-R3 | remark | "no file under it has a sha256 the commit records" | the files are another repository's blobs, which the gate never reads |
| R1-R4 | remark | §5 judges generator files; inputs are matched | said |
| R1-R5 | remark | the doc comment at `trust_generated.rs` garbled | rewritten |
| R1-R6 | remark | two catalogue `why` lines still said "until `M3.6.6.4`" | they name the record |
| R1-R7 | remark | the leaf's "held with their new messages": the tests match the messages' heads | the leaf says the refusals are held |
| R1-R8 | remark | the decisions index's row lacked "under review"; the reviews index row split two rows; the parent's §6 note removed text from a closed record | each repaired; the note adds text alone |

## Why

The record closes on a round that finds no defect, as every design here does; each round, its findings and their
answers are appended above this section.
