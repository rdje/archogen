# Generated sources' tests held to a mutation criterion: the independent review, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-10`
- **Status:** `active`
- **Owner / source:** leaf `M3.6.9` (`docs/tasks/M3.md`). This is the review history of
  [`decision_trust-generated-adequacy.md`](../specs/trust/decision_trust-generated-adequacy.md), kept apart from it as
  `docs/reviews/INDEX.md` describes.

## The fact / decision

**Round 1**, `2026-10-11`, by a context that did not write the record, in a clone of `583cf07` and a second clone for
the trust tests, fenced from the sealed evaluation set. It ran the 23 `route-…` catalogue mutants against the parent's
tests, made the proposed operators' mutants by hand, and reproduced the placement claim — runners untracked, so not
durable. Verdict: 9 defects and 8 remarks. The criterion held the evidence to operators that, by its count, made 7 of
the 23 recorded defects; the answer makes the evidence a part of the criterion — each recorded defect a catalogue
entry the parent's tests must kill — and states the operators' reach as a table measured entry by entry. The
placement took an exception to the scratch rule that only the director has ruled; the answer takes none: the tests'
seam admits the configuration of the repository the sweep's copy was made from.

| # | Kind | Finding | Answer |
| --- | --- | --- | --- |
| R1-D1 | defect | the operators make 7 of the 23 entries' mutants, 16 none; on two lines an operator mutant is killed while the defect survives; three recorded defects are outside the catalogue | the evidence a part of the criterion: a `parent-…` twin of each entry, and of the three, killed by the parent's tests; `guard` and `contains` added, `chain` widened; the reach measured, entry by entry |
| R1-D2 | defect | `Y.into_iter().take(1)` does not compile on a borrowed loop, so its mutants would pass as unviable | the header rewritten through `(E).into_iter()`; unviable counts reported per operator; a loop of the entries' lines whose mutants do not compile a spelling to fix |
| R1-D3 | defect | the scope leaves out code judging the parent's rules: what a program reads, the reading order, the harness's provenance, §6 in the inventory and the gate, the standing list; no operator drops a `.chain(…)` | each named for `--fn`; `chain` drops a `.chain(…)` |
| R1-D4 | defect | the parent's tests, the filters, the timeout and the root left until after the review | named now: the three files' tests, the route tests skipped, 88 tests; the commands whole |
| R1-D5 | defect | the placement takes a second exception to scratch under `target/`, which only the director has ruled | no exception: the copy stays under `target/` |
| R1-D6 | defect | "the tests refuse a configuration above the repository they judge" is false: they admit the outer repository's as a tracked copy | what the gate admits stated; the tests' seam widened to a second root |
| R1-D7 | defect | "the sweep's line operators never make" repeats R20-D2's refuted claim; not every defect narrows a closure's condition | the defects' shapes stated as they are, with round 1's count |
| R1-D8 | defect | the changelog's entry and the decisions index's row claim operators for the defects found | the row narrowed; the entry marked |
| R1-D9 | defect | the book's sentence names `M3.6.6.4`'s rules and every mutant of the code | the parent's rules; each recorded defect, and every mutant the sweep makes |
| R1-R1 | remark | the sweep counts a timeout as a pass | `--timeouts-fail` |
| R1-R2 | remark | one equivalents file for two swept files fails the stale check | one per swept file |
| R1-R3 | remark | the sweep rejects `--filter --skip`; `--filter=--skip` works | the commands use `--filter=` |
| R1-R4 | remark | `--root ..` from a clone under `target/`, a name collision, two sweeps on one root | no `--root` |
| R1-R5 | remark | "`SCRATCH-LOCALITY` admits it" is vacuous: the gate never sees the path | withdrawn with the placement |
| R1-R6 | remark | the CI rehearsal's checkout and the test seam unweighed | the seam decided; the checkout refused |
| R1-R7 | remark | judgment kinds no operator mutates: `if let`, `let … else`, `match` arms, `.values().any`, `max_by_key`, conditions over several lines | named as outside the operators; a defect of one held by its entry |
| R1-R8 | remark | the leaf omits R23-R2 and cites the history's stub under `docs/reviews/` | both corrected |

Round 2, by a context that did not write the record, is next.

## Why

The record closes on a round that finds no defect, as every design here does; each round, its findings and their
answers are appended above this section.
