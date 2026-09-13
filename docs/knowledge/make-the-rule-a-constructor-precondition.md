---
slug: make-the-rule-a-constructor-precondition
answers:
  - "How do I stop a rule being broken, rather than noticing afterwards that it was?"
  - "A specification says 'must not do X' — where does that belong in the code?"
  - "Why is a review instruction weaker than a type?"
type: knowledge
date: 2026-09-13
---

# A rule you cannot construct a violation of beats a rule you check for

## The question

A specification states a prohibition — *do not count the same interval twice*, *do not report a
bound for a task set this model does not cover*, *do not state a conclusion without its
qualifier*. Where does that go? A comment, a lint, a test, a review checklist?

## The answer

**Into the constructor.** Make the illegal state unconstructible, so the rule is enforced at the
moment someone tries to break it rather than discovered later by whoever happens to look.

This project has now arrived at the same shape three times independently, which is the reason to
write it down:

| Rule | Where it could have gone | Where it went |
| --- | --- | --- |
| §7.1: "one global verified flag is prohibited" | a review of report wording | `Conclusion` has no `Verified` variant; every positive one carries its qualifier, so the unqualified sentence is not a value |
| §7.4: eight applicability conditions | a caveat in the analysis's docs | `TaskSet::admit` refuses the set; the analysis cannot return a number for a system the model does not describe |
| §7.4.1: "charge only mutually disjoint intervals" | a reviewer comparing a table | `Ledger::seal` refuses an overlap or a gap; a double-charged trace does not close |

Each replaced *"someone will notice"* with *"it will not compile, or it will not construct"*.

## Why

The failures these prevent share a shape: **the wrong answer looks exactly like the right one.**
A ledger that charges an interrupt twice still totals a plausible number. An analysis that ran on
a task set with equal priorities still returns a response time. A report that says "verified"
reads better than one that says "holds in this model under eight assumptions". Nothing about the
output invites suspicion, so a check that depends on suspicion does not fire.

A constructor precondition does not depend on anyone looking. It also puts the error message at
the point of the mistake, in the vocabulary of the specification — `Ledger::seal` says *"time 3 is
charged twice: to `L useful execution` and again to `interrupt service`. §7.4.1: every physical
execution interval has ONE primary ledger category"* — which teaches the rule to whoever hit it.

## How to apply

- **Look for the illegal state, not the illegal action.** "Do not double-charge" is an action;
  "two intervals claiming the same instant" is a state, and states can be refused.
- **Refuse at the boundary, once.** `TaskSet::admit` is the only way to build one, so every
  function downstream can assume the conditions without re-checking or re-documenting them.
- **Keep the escape hatch explicit when the rule is conditional.** §7.4.1 *permits* deliberate
  over-counting in a safe envelope, so `Accounting` names which kind a total is and disjointness
  is required only for the exact one. A rule enforced where it does not apply gets disabled
  wherever it does.
- **Say the specification's words in the error.** A refusal that cites §7.4.1 is a refusal
  somebody can act on; "invalid ledger" is one they will work around.
- ⚠️ It is not a substitute for a test. Prove the constructor actually refuses — by mutating it
  and watching something go red ([[verify-the-mutation-applied]]); disabling the overlap check
  here turns two tests red, and that is the evidence the guard is load-bearing.
