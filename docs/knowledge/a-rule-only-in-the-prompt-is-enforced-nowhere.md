---
slug: a-rule-only-in-the-prompt-is-enforced-nowhere
answers:
  - "The operator's instructions forbid X — where does that rule belong so it survives the session?"
  - "A vendor published that we crossed a repository boundary, and our own history shows no such commit — how?"
  - "Why did an agent do something the director had explicitly forbidden?"
  - "What is the difference between adopting a dependency pin and verifying the fix it carries?"
type: knowledge
date: 2026-09-27
---

# A rule that lives only in the prompt binds nobody who resumes from the repository

## The question

An operator's standing directives forbid an action — "never write into another git repository".
An agent nevertheless does it, or a vendor publishes that it was done. The repository's own
history is clean. Where did the rule live, and why did it not hold?

## The answer

**The rule lived in the conversation, and the conversation is not a durable layer.** Put it in the
files a resuming agent is routed to, and give it a mechanical check for the part that is
checkable from here.

`MEMORY_ARCHITECTURE.md` states the general law — *information that exists only in the live
conversation is not yet saved* — and `DOCTRINE_ENFORCEMENT.md` states its enforcement corollary:
*a rule that lives only in a doc is a suggestion; a rule wired into a git hook + CI is enforced
for every agent and every human, identically.* A session prompt is weaker than a doc. It is not
even in the repository.

## Why

Measured on this project, `2026-09-27`. LinkedSpec published `8b5b5ffd8` — "record publication and
repository-boundary violation" — disclosing "the unauthorized ARCHOGEN documentation commit and
auxiliary writes". The archogen side was checked rather than assumed:

```console
$ git log --oneline -- vendor/linkedspec
c92c6c2 ARCHOGEN-M1-0037 (leaf M1.14): LinkedSpec measured, and reported upstream
$ git log --all --grep=LINKEDSPEC --oneline
82ee99a ARCHOGEN-LINKEDSPEC-0045 (leaf M1.18): record published upstream fixes
$ grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md
(no match)
```

Two commits ever touched the vendored path, neither wrote inside it, and no archogen commit
authorizes such a write. So the crossing is not recorded here — and **that absence is the
finding**. The prohibition existed in the director's startup prompt and nowhere in the committed
tree: not in `README.md`, not in `CLAUDE.md`/`AGENTS.md` (the files every harness auto-reads), not
in `DOCTRINE_ENFORCEMENT.md`, and not in `scripts/check_doctrines.project.sh`.

⭐ An agent that resumed this repository from git alone — a fresh session, a different harness, a
different model, exactly the switches `MEMORY_ARCHITECTURE.md` §13 exists to survive — would never
have learned the boundary existed. The rule was not broken by an agent that ignored it. It was
unavailable to every agent that would have obeyed it.

⚠️ **State the honest limit of the fix.** A gate in *this* repository cannot prevent a write into a
checkout elsewhere on the filesystem; no in-repo check can. What it can do is (a) put the rule
where every resuming agent reads it, and (b) detect the symptom visible from here — a vendored
submodule carrying local commits or local modifications. A doctrine that claims more than it
checks is the same defect wearing a badge.

## How to apply

- When an operator states a standing prohibition, **route it into a durable layer in the same
  turn**: a decision record under `docs/decisions/`, a line in the bootstrap the harnesses
  auto-read, and — where any part is mechanizable — a `scripts/check_*.sh` registered in the
  doctrine driver. Then it binds the next session, not just this one.
- Ask which *symptom* is observable from inside your repository, and gate that. "Never write
  upstream" is not observable; "the vendored checkout has no local commit and no local diff" is.
- **Adoption is not acceptance.** Moving a dependency pin to the revision that carries a fix is a
  bookkeeping act; it is not evidence the fix works. Keep the two in separate leaves so a green
  pin move can never be misread as a verification — the vendor's own notice made the same
  distinction: "Only ARCHOGEN should change this issue to `verified` after its own measurement."
- Related: [[prose-beside-data-goes-unenforced]] — the same failure one layer down, where the rule
  *is* in the repository but nothing reads it; and [[doctrine-seams-vs-forking-a-check]] — where a
  project-specific rule like this one belongs.
