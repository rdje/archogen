---
slug: pin-the-vendor-head-and-measure-the-delta
answers:
  - "A vendor's notice names a specific publication — do I pin that commit or their latest head?"
  - "Is it safe to move a vendored dependency pin forward, and how do I know rather than guess?"
  - "The evidence I was given cites an older revision — does it still apply at the one I pin?"
type: knowledge
date: 2026-09-27
---

# Pin the vendor's latest published head, and measure the delta to the revision the evidence cites

## The question

A vendor fixes bugs you reported and sends a notice naming a specific commit: "published
`fd3e328d5`, adopt it." Their branch head is two commits past that. Which do you pin?

## The answer

**Pin the latest published head, name it explicitly, and measure what differs between it and the
revision the notice cited.** Attributability — the reason to prefer a named revision at all — is
satisfied by *naming*, not by *picking the older one*.

Then let a path-level diff decide whether the cited evidence still applies:

```console
$ git diff --name-only <cited>..<head> | grep -Ei '\.spec$|specs/|\.rs$|Cargo'
(no output — every changed path is documentation)
```

No code or specification path in the delta means the evidence gathered at the cited revision
describes the *same* code at head, so it carries over; and if the cited revision is an ancestor, its
links still resolve.

## Why

Measured on this project, `2026-09-27`. The first choice was the cited publication `fd3e328d5`, on
the reasoning that "measuring an unnamed revision would make the `verified` states unattributable".
The reasoning was **half right, which is why it was wrong**: it correctly required a named revision
and then wrongly concluded the name had to be the older one.

⛔ **And the conservative choice had a real cost.** The two skipped commits included `8b5b5ffd8`,
which carried the vendor's own correction for a repository-boundary write that had *actually
happened* — a new read-only rule, an incident record, and a recovery patch. Pinning the older
revision deliberately excluded the remedy for the one misbehaviour that had fired. "Pin the version
the evidence names" sounds careful and can strand you behind a fix you need.

The delta turned out to be 25 paths: nine root documents and sixteen under `docs/`, with **no**
`.rs`, `Cargo.*` or `specs/` path among them — so the Rust backend and the specifications under
test were identical at both revisions, and moving to head cost nothing in attributability.

⭐ **Check the nested pins before re-running a long sync.** The nested submodule's revision was
identical at both commits (`git ls-tree <rev> rgx`), so this move took seconds, where the first
adoption took minutes because it cloned 2.1 GB. The expensive part is often a one-time cost that a
diff can tell you to skip.

## How to apply

- Default to the vendor's **latest published head** unless you have evidence a report is unfixed or
  a new misbehaviour appeared — and record that preference so it is not re-litigated each time.
- **Name the revision you pin** in the register, distinct from the revision the original
  observations were *taken* at. Two different facts, two different rows.
- **Measure the delta before moving**: a path-level `git diff --name-only` filtered for the
  language's code and spec paths. It converts "is this safe?" from an opinion into a listing, and it
  tells you whether documentation you depend on — an integration guide, a migration note — changed
  too. ⚠️ Here one did: the pinned Rust integration guide was among the changed files, so the
  follow-up measurement had to follow the guide at head, not the one the notice linked.
- **Inspect release metadata, not implementation.** Commit subjects and a path-level diffstat are
  the vendor's published release record. Reading their source to answer "is it safe to move the pin"
  crosses the submodule boundary (§20) and is not needed.
- **A pin move is adoption, never acceptance.** It makes verification *possible*; it is not
  verification. Keep it in its own leaf so a green pin move cannot be misread as a passing
  re-measurement.
- Related: [[a-rule-only-in-the-prompt-is-enforced-nowhere]] — the same incident, and the rule that
  was missing from the repository when it happened.
