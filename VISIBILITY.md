# VISIBILITY — this repository's declared posture

> **Declared posture: PUBLIC.**
>
> This repository is public, and **every project created or spawned from this template is
> public and is not confidential**, unless that project deliberately changes the line above
> and records why. Maintainer instruction, 2026-09-21.

This file exists so the posture is **stated once, in a known place, before content is
written** — rather than inferred from a remote setting that a reader cannot see from inside
a clone, and that can change without anything in the repository noticing.

## Why an explicit statement, and not just the remote setting

A repository's visibility is a property of the hosting platform. Nothing inside a clone
reveals it, so every contributor — human or agent — infers it, and different contributors
infer differently. The cost of a wrong inference is asymmetric and permanent:

- **Confidential material placed in a public repository cannot be un-published.** Deleting
  a file, amending a commit or force-pushing does not retract what was fetched, forked,
  mirrored, cached or indexed while it was reachable.
- The reverse error is cheap. Treating a private repository as public costs a little
  inconvenience; treating a public one as private costs a disclosure that cannot be undone.

So the posture is written down, and the rule below follows from it rather than from anyone's
memory of a conversation.

## The rule

**Nothing confidential goes into this repository.** Not in source, documentation, commit
messages, task trees, decision records, test fixtures, issues, pull requests, CI logs, or
history. A public repository cannot provide a confidential embargo through any of its
mechanisms, including a branch that is never merged or a commit that is later removed.

Treat everything committed here as published at the moment it is pushed.

### Where confidential material goes instead

- Report sensitive findings **privately to the accountable owner**, through a channel agreed
  in advance. A public tracker is not that channel.
- Do confidential work in a **separate private workspace**, and bring across only what is
  suitable for disclosure.
- If a project genuinely requires a private repository, that is a deliberate decision:
  change the **Declared posture** line above, state the reason and the authority, and record
  it in `docs/decisions/`. Do not leave the two disagreeing.

## ⛔ What the posture does NOT establish

Public source visibility is an authorization to publish **source**, and nothing else. Each of
these keeps its own gate and is unaffected by this file:

- name, crate, package, domain or handle clearance;
- licence selection and the grant the manifests declare;
- release qualification, signing and distribution;
- deployment security and operational exposure.

⚠️ A project that is public has **not** thereby cleared any of the above. Conversely, none of
them requires a private source repository.

## For a project created from this template

`scripts/bootstrap.sh` keeps this file. Read it once at bootstrap and decide deliberately:

1. If the project is public — the default — nothing to do; the statement above already holds.
2. If it must be private, change the **Declared posture** line, give the reason and the
   authority, and add the decision record. The file is the single place a reader looks.

⭐ Either way the answer is in the repository, in one named file, and a new contributor does
not have to ask.
