---
slug: prose-beside-data-goes-unenforced
answers:
  - "Half my config is enforced and half is prose — how do I stop the prose rotting?"
  - "Why did the checker accept something the specification table forbids?"
  - "How do I track a rule I have written down but not implemented?"
type: knowledge
date: 2026-09-13
---

# Prose sitting beside enforced data will go unenforced — count it in a test

## The question

A specification object carries two lists: one the code consults, one that reads as documentation.
How do you keep the second from quietly becoming false?

## The answer

**Count the unenforced half in a test, and make the number part of the contract.**

```rust
assert_eq!(PROFILE.decisions.len(), 13, "the table changed size — re-count what is enforced");
let unenforced = /* rows not in the enforced set */;
assert_eq!(unenforced.len(), 12, "still prose, classified by leaf M1.10: {unenforced:?}");
```

The test does not enforce the twelve rows. It makes their number a fact that cannot drift: adding
a row fails it, enforcing one fails it, and either failure lands on the person who changed
something rather than on whoever hits the gap months later.

## Why

`Profile` here has `exclusions` (consulted by the admission pass) and `decisions` (thirteen rows
of concern/decision). Both are `&'static [..]`, both look equally authoritative in the source,
and both are published on the same documentation page. Only the first was ever read:

```console
$ git grep -n '\.decisions' -- crates/
crates/eadl-model/src/profile.rs:313:        for row in RT_STATIC_UP_V1.decisions {
```

One hit — and it is the *drift test* comparing the table to the published page. So the table was
kept scrupulously in sync with the documentation while meaning nothing to the checker, and a
description with two tasks at the same priority was accepted by a profile whose own row says
"static **unique** task priorities".

⭐ That is the specific trap: the row had a **guard that made it look guarded**. A drift test
between prose and prose is easy to mistake for enforcement, and it is the reason nobody noticed.

Note also what "twelve unenforced" is *not*: twelve defects. "Rust `no_std` core" is a property
of the engine, not of any description. The census's value is that it forces someone to say which
kind each row is, instead of leaving all thirteen ambiguous.

## How to apply

- When you enforce part of a declarative table, add the census in the same change. It costs four
  lines and it is the only thing standing between you and the next silent row.
- Name the **leaf that will classify the rest** in the assertion message, so the failure tells
  the reader where the work is tracked rather than only that a number moved.
- Prefer a verdict that says what admitting the request would cost. Refusing duplicate priorities
  as "unsupported by this profile, because equal priorities need a tie-break policy with its own
  analysis" is a statement about work; "invalid" would be a statement about the author, and it
  would also be wrong.
- Related: [[an-oracle-is-independent-by-construction]] — the same instinct applied to a test
  rather than to a table.
