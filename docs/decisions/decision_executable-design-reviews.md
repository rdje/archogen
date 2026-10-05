# A design review converges on an executable design: a model, a falsification corpus, a mechanized hand-off ledger

- **Type:** `decision`
- **Date:** `2026-10-05`
- **Status:** `active` — the method for `M3.1.1` and `M3.6.1` from their next rounds, and for every design record
  reviewed under the closure rule after them
- **External sources:** [the pinned Rust toolchain](../book/src/ledger.md#rust-toolchain) — what rustc and cargo do is
  a measuring instrument's fixture, against that pin
- **Owner / source:** leaf `PROGRAM.52` (`docs/tasks/PROGRAM.md`). The director, `2026-10-05`, on the suggestion that
  a defect in a hand-off need not count against a design's closure: "almost every SW problem can be fixed. You need to
  think out-of-the-box but the outcome shall always be sota, signoff and production-grade." The closure rule stands —
  a design closes on the first independent round that finds no defect; what changes is what the round reviews.

## The fact / decision

**Why the two loops did not converge.** Each round handed a fresh reader some 50 KB of prose and asked it to find
what is wrong. Prose has no bound on how it can be misread, every answer added prose, and the reader re-derived by
hand what a machine could have settled.

| Record | Rounds | Defects per round | Where the late defects were |
| --- | --- | --- | --- |
| `decision_substitutability-relation.md` | 15 | 7, 7, 8, 3, 3, 3, 5, 1, 2, 8, 7, 8, 9, 9, 8 | a hand-off not carried, or carried in weaker words; a claim about a tracked verdict; a requirement spelling given no outcome; one rule stated twice; one arithmetic endpoint (R14 N9) |
| `decision_trust-inventory.md` | 6 | 11, 12, 16, 11, 10, 10 | an input channel the inventory did not see — assembler `.incbin`, a macro's include, a link-time symbol, a file's new reader — each found by a reader building a scratch workspace; a delegation's wording; one premise asserted without a count (R5) |

Three of those classes are mechanical, and a fourth is measurement: whether a hand-off is carried, whether a rule
stated twice says the same thing, whether every input has exactly one outcome, and what rustc and cargo actually do.
The method moves each of them out of the reader's judgement and into something that fails.

1. **An executable model.** A design whose subject is a decision procedure ships, with its record, a reference model
   of that procedure: a direct transcription, rule by rule, each rule citing its section, optimised for nothing,
   depending on `std` alone (`decision_zero-dependency-engine-core.md`). Beside it, a **bounded exhaustive checker**
   enumerates a finite universe of inputs — every domain, every written form, values on both sides of every bound —
   and asserts the acceptance's properties on all of them: every input has exactly one outcome; no value satisfies a
   bound against its direction; `exactly` admits only what every direction admits; a precondition set is judged by
   inclusion, so no stronger precondition passes; each derivation agrees with a brute-force simulation of what it
   derives (a counter's reads over every phase, for small moduli and rates). The production implementation is later
   tested against the model over the same universe.
2. **A measuring instrument.** A design whose subject is what a toolchain does ships the instrument that measures it,
   and every channel a reader has measured by hand is a fixture of the instrument's tests (`TOOLBOX.md`: a tool
   first, never a guessed root cause). A claim about rustc or cargo is a fixture's result, not a sentence.
3. **A falsification corpus.** Every probe of every round becomes a fixture with the outcome the record gives it, and
   a fixed defect stays fixed. A later finding is a defect when it carries a reproducer: a fixture the model or the
   instrument gets wrong, or two quoted sentences that contradict each other.
4. **A mechanized hand-off ledger.** A record keeps its hand-offs in one machine-read table — an identifier, the
   receiving leaf, and the obligation in one sentence. The receiving leaf's acceptance quotes that sentence verbatim
   beside the identifier. The doctrine `HANDOFF-LEDGER` refuses a commit in which a row's sentence is missing from its
   leaf or differs from it, or a leaf quotes an identifier no ledger holds. So no hand-off can be left uncarried, and
   none can be carried in weaker words than the record's: there is one copy of each sentence, checked against the
   other.

**What a round reviews.** The record, the model or instrument, and the corpus. The reader runs the checker and the
corpus, and spends its judgement on what no machine can settle: whether the rules are the right rules, whether the
model transcribes the record faithfully, and whether the universe misses a case. The closure rule is unchanged.

## Why

- A defect class closed by a check stays closed; a defect class closed by a sentence returns under another reader's
  phrasing. Round 14's horizon endpoint, `modulus / rate` where the reads allow only `(modulus − 1) / rate`, was in
  the record for thirteen rounds of reading, and a simulation of a 4-tick counter at 1 Hz finds it in milliseconds.
- Hand-offs were the largest late class in the first loop, and a hand-off is a string equality across two files.
- In the second loop, every round's new defect was a channel a reader measured in a scratch workspace that nobody
  kept. Kept as fixtures, those measurements accumulate instead of being redone, and the next channel is the only
  thing left to find.
- This is how state-of-the-art teams make a design sign-off sound: an executable specification checked exhaustively
  over a bounded universe, the implementation tested against it, and every reported failure a regression test.

## How to apply

- A design record under the closure rule names its model or instrument, its checker and its corpus in its leaf's
  acceptance, and holds its hand-offs in a ledger table marked `<!-- machine-read: handoffs -->`.
- An answer to a finding lands as a fixture first, failing, then the rule that makes it pass, then the record's words.
- Related: [[decision_substitutability-relation]], [[decision_trust-inventory]], [[decision_zero-dependency-engine-core]].
