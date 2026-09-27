---
slug: a-rule-only-in-the-prompt-is-enforced-nowhere
answers:
  - "The operator's instructions forbid X — where does that rule belong so it survives the session?"
  - "Another project's agent wrote into my repository — what do I check before I trust or reject it?"
  - "Why did an agent get a standing prohibition backwards, when the operator had stated it clearly?"
  - "What is the difference between adopting a dependency pin and verifying the fix it carries?"
type: knowledge
date: 2026-09-27
---

# A rule that lives only in the prompt binds nobody who resumes from the repository

## The question

An operator's standing directives forbid an action — "never write into another git repository".
Something happens that the directive covers, and the agent gets it *backwards*: it cannot tell
which side crossed, because the rule it is reasoning from is not in the repository it is reading.
Where does such a rule belong?

## The answer

**In the files a resuming agent is routed to, plus a mechanical check for whatever part of it is
observable from inside the repository.** A session prompt is not a durable layer — it is weaker
than a doc, because it is not even in the tree.

`MEMORY_ARCHITECTURE.md` states the law: *information that exists only in the live conversation is
not yet saved.* `DOCTRINE_ENFORCEMENT.md` states the corollary: *a rule that lives only in a doc is
a suggestion; a rule wired into a git hook + CI is enforced for every agent and every human,
identically.*

## Why

Measured on this project, `2026-09-27`. LinkedSpec published `8b5b5ffd8` — "record publication and
repository-boundary violation" — disclosing "the unauthorized ARCHOGEN documentation commit and
auxiliary writes". Reading that from inside archogen, with the operator's prohibition present only
in the session prompt, the direction was ambiguous: the sentence names ARCHOGEN, and nothing in the
committed tree said which way the boundary ran. The first durable record written from it said
archogen had written into LinkedSpec. **It was the other way round** — LinkedSpec's agent had
modified a few `.md` files *here*, to deliver its fix notice. The operator had to correct it.

The census that settled it, all inside this repository:

```console
$ git log -12 --format='%h | A:%an <%ae> | C:%cn <%ce>'   # every commit: one local identity
$ git reflog --date=short                                  # linear — no reset, rebase or amend
$ git show --name-only 82ee99a                             # docs/feedback/linkedspec/** + live docs
$ grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md
(no match)
```

⭐ **The measurable cost of an unwritten rule was a wrong durable record, committed.** That is the
sharpest version of the argument: not "an agent might disobey", but *an agent that was trying to
obey could not determine what the rule said*, and wrote its error into layer C where the next
session would trust it. The `grep` returning nothing is the whole finding — the prohibition was
absent from `README.md`, `CLAUDE.md`/`AGENTS.md` (what every harness auto-reads),
`DOCTRINE_ENFORCEMENT.md` and `scripts/check_doctrines.project.sh`.

⭐ **And the inbound write itself was handled correctly, by a mechanism worth naming.** Leaf
`M1.18` recorded the authorization, preserved the 36 original files it did not change by SHA-256,
and refused to let the vendor's notice set `verified` — "Only ARCHOGEN may set verified after its
own adoption and rerun". An external agent's claim about its own fix entered the tree as an
*attributed claim*, not as a result. When something writes into your repository from outside, that
is the shape to hold it in.

⚠️ **State the honest limit of the fix.** A gate in *this* repository cannot prevent a write into a
checkout elsewhere on the filesystem, and cannot prevent an inbound write either — it can only make
the inbound one land through a leaf. What it can do is (a) put the rule where every resuming agent
reads it, and (b) detect the outbound symptom visible from here: a vendored submodule carrying local
commits or local modifications. A doctrine claiming more than it checks is the same defect wearing a
badge.

## How to apply

- When an operator states a standing prohibition, **route it into a durable layer in the same
  turn**: a decision record, a line in the bootstrap the harnesses auto-read, and — where any part
  is mechanizable — a `scripts/check_*.sh` registered in the doctrine driver.
- **Write the direction down.** A boundary rule with two sides needs both stated; "never cross the
  boundary" does not tell a reader which way it is facing, and an incident report from the other
  side will name *you* either way.
- Ask which *symptom* is observable from inside your repository and gate that. "Never write
  upstream" is not observable; "the vendored checkout has no local commit and no local diff" is.
- **Treat an externally delivered change as a claim.** Leaf, authorization, file list, preserved
  originals, and no verdict it is not entitled to set.
- **Adoption is not acceptance.** Moving a dependency pin to the revision carrying a fix is
  bookkeeping, not evidence. Keep the two in separate leaves so a green pin move can never be
  misread as verification.
- Related: [[prose-beside-data-goes-unenforced]] — the same failure one layer down, where the rule
  *is* in the repository but nothing reads it; and [[doctrine-seams-vs-forking-a-check]] — where a
  project-specific rule like this one belongs.
