---
slug: enumerate-the-population-from-the-specification
answers:
  - "My two implementations agree on every input I have — which input is neither of them ever given?"
  - "A mutation-tested gate is green. What can it still not see?"
  - "Where should a test's input population come from?"
  - "Why did a grammar and its reader disagree for a whole milestone with nothing failing?"
  - "I added a construct to the language and every test passed — which reader have I not told?"
type: knowledge
date: 2026-09-28
---

# Agreement over your inputs is not agreement over the specification

## The question

Two independent implementations of one specification agree on every input the suite supplies, and the
suite is mutation-tested: putting a plausible-wrong rule into either implementation turns it red. What
can still be wrong?

## The answer

**Any input nobody supplies.** Mutation testing asks *"what would a wrong implementation do to my
inputs?"* It cannot ask *"which input does my suite not contain?"* — and only the second question finds
a hole in the population.

Measured here (`docs/tasks/M1.md`, leaves `M1.11`, `M1.12`): eADL's normative grammar and its reader
were compared over every description the repository ships *and* over one probe per grammar production,
and the recognizer was mutation-tested in both directions. Three literal forms still disagreed:

| Literal | Grammar | Reader |
| --- | --- | --- |
| `0X10` | refused — the prefix was spelled `"0x"` only | read as `16` |
| `0x_10` | refused — a `hex_digit` must come first | read as `16` |
| `"\0"` | accepted — `escape` admitted `"0"` | refused as a bad escape |

None of the three appears in any description the repository ships. That is not an assumption but three
censuses: `git grep -nE '0X[0-9a-fA-F]'`, `git grep -nE '0[xX]_'` and `git grep -nF '\0' -- '*.eadl'`,
each returning nothing. So no input either implementation was ever given could expose them, and a suite
that agreed on everything it had was agreeing on a sample.

The reason is structural rather than careless: the probes were **one per production**, and all three
divergences live in a *combination* — an uppercase prefix on a hexadecimal literal, a separator in the
position where a digit is required, a backslash followed by a character the escape set does not define.
A population built by walking a specification's parts one at a time cannot contain its combinations.

## Why

- **A corpus is a sample of what authors happened to write**, not of what the language admits. It is
  strong evidence about the paths a real description takes and says nothing about the forms nobody has
  needed yet — which is exactly where two implementations diverge, because divergence is only punished
  where inputs exist.
- **A mutation is a hypothesis about the implementation, not about the input space.** Mutation testing
  sharpens a gate against the population it already has; it cannot widen the population.
- **A specification enumerates a space; a suite enumerates points.** Agreement at points licenses a
  claim about the space only when the points were chosen *from* the enumeration.

## How to apply

- **Take the population from the specification, not from the corpus.** Here that meant a table of
  `literal → value → canonical text` in the normative reference, enumerating every literal *form* the
  grammar's productions admit — signs, separators, leading zeros, both radices, boundary magnitudes,
  malformed shapes, every escape and a refused one. The table is a systematic enumeration, so a
  combination has a row instead of needing someone to think of it.
- **Make the enumeration a document, and execute it from both sides.** One table, read out of the
  document by two test files: `reference.rs` compares it with the reader, `conformance.rs` compares it
  with the recognizer derived from the grammar. Neither implementation can be adjusted to match the
  other without editing a normative document, and the document is what review reads.
- **Require a verdict for every cell, refusals included — and keep two kinds of refusal apart.**
  `1.2.3` is *not well-formed*: no conforming implementation may read it. `9223372036854775808` *is*
  well-formed and its value does not fit. A syntax that could express "too large" would have to know
  the width of the value domain, which is the implementation detail a grammar exists to leave out. So
  the recognizer must reject the first and accept the second, and stating that as two notations
  (`error C` / `refused C`) is what let one table drive both legs.
- **Derive the expected values independently.** A table printed from the implementation is a
  transcription and will agree with it forever ([[an-oracle-is-independent-by-construction]]). Deriving
  each value arithmetically is why this table's first run found three defects instead of confirming
  one.
- **Keep the corpus — as a regression set, not as the population.** It is still the only input that
  came from a real author rather than from the author of the test, and a coverage leg whose population
  *is* the corpus ("every literal the repository ships has a row") catches a deleted row that no
  enumeration check would notice.
- ⛔ **Census before claiming the gap.** "No input contains X" quantifies over the whole repository and
  is false the moment one file uses it. Run the search, record the command with the result
  ([[a-moved-measurement-needs-a-census-of-its-copies]]).

Related: [[a-gate-is-only-as-sharp-as-its-fixtures]] reaches the same blind spot from the other side,
by mutating the subject; the two remedies are complements, not alternatives — mutation proves the gate
would notice a wrong rule, enumeration proves the gate was given the input. [[verify-the-mutation-applied]]
applies to every mutation an arm feeds itself.

## The population has a second axis: the kinds of file a rule reaches

Everything above is about *inputs to one gate*. The same statement bites a second way, on the consumers
of a rule rather than the inputs to a check, and it was measured here by `M1.13.4.1`
(`docs/tasks/M1.md`).

A language version identifier, `(eadl-version eadl/1)`, was added as a top-level **form**. The rule it
needs is "this form is not a declaration", and every consumer that iterates top-level forms owes it.
Two were found and fixed at the time, by running the real path and reading the refusal: the module
loader (which counts a module file's forms) and the schema pass (which validates each form against the
kind registry). Both were found the right way — through the entry point the consumer uses — and the
leaf closed green.

⛔ **There was a third.** The same file also holds the loader that reads the shipped *kind modules*,
and it handed every top-level form to `read_kind`, so a kind file stating its own version was refused
as `schema-not-a-kind` and took the whole registry with it — `tool-failure` for every description the
toolchain was asked about. No description-level test could reach it, because nothing reads a kind
module except that loader, and **no fixture put the new construct in one**.

What found it was not reading the code for consumers. It was writing the identifier into all 62 shipped
descriptions — every file kind the toolchain reads — and running everything: 45 failures, 36 of them
this one consumer.

- **Generalise it: when a language gains a construct, put it in one file of every kind the toolchain
  reads before deciding the change is complete.** Descriptions, kind modules, module files, frozen
  evidence. A rule's consumers are a population, and only the ones your fixtures reach will tell you
  they are missing.
- **Then make the rule one accessor, not one predicate per caller.** The fix was `declarations()` beside
  the predicate, so a pass that treats a form as a declaration goes through the same filter — a rule
  each consumer re-implements is a rule the next consumer lacks.
- ⭐ **Census the consumers by shape, not by name.** `grep` for the iteration (`for form in
  &document.forms`, `.forms.iter()`) rather than for the construct: a consumer that has never seen the
  construct cannot be found by searching for it.
- The same run found the shape's test-side copy: three test helpers each hand-rolled a kind-module
  loader, so a suite could have stayed green on a loader that could not read the shipped files. They now
  call the production loader.
