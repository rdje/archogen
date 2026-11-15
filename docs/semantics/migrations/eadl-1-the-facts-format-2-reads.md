# The vocabulary gains the seven facts format `/2` reads

- version: eadl/1
- date: 2026-11-15
- leaf: M4.1.7.3.1 (`docs/tasks/M4.md`)
- status: applied
- constructs: suite/docs/semantics/vocabulary/vocabulary.eadl
- invalidates: none, measured — no tracked description writes any of the seven, `git grep -l -F "(<fact>" -- '*.eadl'` naming none outside the vocabulary, so no verdict reads them; model §8 rule 14 reads five of them as no reader of a time base, which no tracked description binds

## What changed

`/1` gains seven entries, appended after `wrap-behavior`, each with its row in
`docs/decisions/decision_substitutability-relation.md` §1.1:

- five describing a counter's compare, facts of the function `counter`, where `relative-delay` is: `alarm-wiring`, an
  enumeration `core controller`, `exact`; `alarm-kind`, an enumeration `level-compare delay edge`, `exact`;
  `compare-step`, a count of the counter's ticks, `at-most`; `compare-window`, a count of its ticks, `at-least`; and
  `compare-magnitude`, a boolean, `exact`;
- two of the platform, facts of the function `processor`: `non-maskable-interrupts` and `firmware-traps`, counts,
  `at-most`.

Each is a guarantee and shared. `catalog-target` gains no entry: it is a name, which no domain of the record's §2
holds, no requirement constrains it, and the catalog's selection alone reads it, as `region` and `ordering` have none.

Model §8 rule 14 reads none of the five that describe a compare as a reader of a time base: each says how the compare
is programmed or delivered, and none reads the counter's time. The generator's joint rules
(`crates/eadl-resolve/src/joint.rs`, `DESCRIBES_A_COMPARE`) and the checker's own
(`crates/archogen-plan-check/src/joint.rs`) leave them out of the rule, each held by a test.

## Why

Format `/2`, the runtime plan for a real machine, reads them (`docs/decisions/decision_runtime-plan-format-2.md` §2,
§3): the release alarm's record hands the five to the vocabulary (`decision_release-alarm.md`'s `RA-H8`), since the
checker's `ρ`, its alarm's kind and what one write of a compare reaches are the target's statements; and the profile's
admission asks every target how many interrupt sources no mask stops and how many traps firmware beneath the image
takes, refusing one above zero and one silent (`/2` §2, R4-D15). A fact the vocabulary does not declare may be offered
but not constrained (the vocabulary's header), and each of these is asked as a constraint, so each needs its entry.

## Which descriptions it invalidates

None. No tracked description writes any of the seven, so every `archogen check` verdict frozen in
`crates/archogen-cli/tests/verdicts.txt` stands. A target's statements of them, with what checks each, are
`M4.1.7.3.3`'s, and what `check` refuses of them `M4.1.7.3.2`'s.

## Which version it lands in

`eadl/1`: the vocabulary grew by seven entries, and no description that meant one thing now means another.
