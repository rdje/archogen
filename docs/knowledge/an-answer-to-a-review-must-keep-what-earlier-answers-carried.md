---
slug: an-answer-to-a-review-must-keep-what-earlier-answers-carried
answers:
  - "My answers to a review round drew new defects in the next — how do I answer so they stop?"
  - "I rewrote a reviewed passage to fix a finding — what could the rewrite have lost?"
  - "My argument says a cost is paid by some charge — how do I know that charge is free?"
  - "A review finds a class of cases my rule does not charge — do I charge them or refuse them?"
type: knowledge
date: 2026-10-02
---

# An answer to a review must keep what earlier answers carried

## The question

A design goes through independent review rounds until one finds no defect. The answers to one round
draw new defects in the next, and the loop does not converge. What makes an answer safe?

## The answer

Three habits, each learned from a defect an answer introduced (leaf `M2.11`, its review rounds 2 and 3,
in `docs/reviews/decision_runtime-analysis-variant-reviews.md`; and leaf `M2.9`'s fifteen rounds):

1. **Edit the words a finding names; do not rewrite the passage whole.** `M2.11`'s third step rewrote
   the variant's condition 5 to narrow it, and dropped "of a declared source", a qualifier the first
   answer had carried. The third review rebuilt the first review's missed deadline from the gap. Before
   landing, diff the answer against every earlier answer of the same leaf, and the review history's
   answer columns, for a qualifier, clause or case that has gone.
2. **A justification that names a charge must show the charge is free.** Two `M2.11` defects were
   arguments that reached the right total by charging the wrong thing: a cut trap's entry to an
   arbitrary pending request, whose cost need not cover it, and a stretch counted as ended when it
   could still be running. The bound held both times; the reasoning did not. For each charge an
   argument spends, say why nothing else spends it.
3. **Prefer refusing a class to charging it.** Charging traps that run several services needed a cost
   split, a start for later services and a timer case, none owned by any record, and each drew a
   finding. Refusing them, one service per trap, deleted that text and the findings with it. The same
   held in `M2.9`: deleting text was the safest answer, and an answer by delegation to the record that
   owns a fact beat new mechanics.

## Why

Each round reads the text as it stands, not the history of how it got there. A qualifier lost in a
rewrite is invisible to the reviewer who did not see it, and an argument that cites a charge reads as
sound until someone asks what else the charge pays for. The loop converges when answers shrink the
text a reviewer must trust, not when they grow it.
