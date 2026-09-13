---
slug: presence-checks-cannot-see-an-extra-key
answers:
  - "My test suite is green but the output is wrong — what kind of assertion did I write?"
  - "How do I test a parser against a corpus so it actually catches drift?"
  - "Why assert an exact set instead of checking the fields I care about?"
type: knowledge
date: 2026-09-13
---

# A presence check cannot see an extra key

## The question

The suite is green. Every field the test asks for is there, with the right value. The output is
still wrong. What kind of assertion was written?

## The answer

A **presence** check — `assert!(headers.contains_key("rationale"))`, or fetching a field and
comparing it. It can only fail when something it names is missing or changed. It is blind to
everything it does not name: an extra field, a field that appeared in the wrong place, a value
that was truncated at a point the test does not reach.

Assert the **exact set** wherever a set is what the code produces.

```rust
// blind to a seventh header appearing
assert!(get("rationale").is_some());
assert!(get("verdict").is_some());

// fails the moment the shape changes
assert_eq!(keys, ["case", "verdict", "ambiguous", "tests", "rationale", "other-side"]);
```

## The measured instance

The eADL reader parses `; key: value` metadata out of comments. One boundary-corpus rationale
wraps onto a line beginning:

```text
;   implementation-independence: a different timer with the same width and rate …
```

That is exactly a bare kebab-case key followed by a colon. The reader opened a spurious seventh
header and truncated the rationale at that point — silently, because a truncated value is still
a value.

Two suites were green over it. The unit tests used inputs chosen to exercise a branch, and none
of them happened to wrap onto a key-shaped line. The corpus suite ran over all 21 real files
and asserted that `case`, `verdict`, `ambiguous`, `tests` and `rationale` were present, with
`rationale.len() > 40`. All true. All blind.

It was found by running a diagnostic tool over a real file and **reading the output**:

```console
$ cargo run -q -p eadl-front --example diagnose -- .../counter-width-and-rate.eadl
headers:
  ...
  rationale: AMBIGUOUS, resolved ACCEPT. The width and rate of an offered counter loo…
  implementation-independence: a different timer with the same width and rate is inter…
  other-side: engine: whether to extend the epoch, …
```

The extra line is obvious to a reader and invisible to a presence check.

## The second lesson

The fix needed **two independent discriminators** — the line must be unindented *and* its key
must be bare — and each had already been tried alone and failed on a real file. Indentation
alone fails because every comment begins with a space after the `;`. Key shape alone fails on
the wrap above.

When a format is disambiguated by a single heuristic, look for the input that defeats it before
shipping. There usually is one, and it is usually already sitting in the corpus.

## How to apply

1. Where the code produces a **set** — headers, fields, diagnostics, plan bindings — assert the
   set, not memberships.
2. Run the real tool over the real inputs and **read the output** at least once. A suite proves
   what it was told to look for; looking is how you find what nobody thought to assert.
3. When a heuristic disambiguates a format, write the case that defeats it as a test, in both
   directions — the new rule must not make the good case unreachable either.

Related: [[doctrine-seams-vs-forking-a-check]] — a check that has only ever been seen green has
not been shown to check anything.
