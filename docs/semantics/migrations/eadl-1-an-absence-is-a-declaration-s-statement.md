# An absence is one declaration's statement

- version: eadl/1
- date: 2026-10-27
- leaf: M3.4.8 (`docs/tasks/M3.md`), deciding `SR-H16`
- status: pending
- constructs: suite/docs/semantics/cases/invalid-contradictory-presence.eadl
- invalidates: no frozen verdict moves, measured over every tracked description — `crates/archogen-cli/tests/verdicts.txt`
  unchanged; F06's case, whose two-block spelling the rule now accepts, rewritten to the contradiction it still refuses,
  so its verdict, `10 invalid-description`, stands

## What changed

Model §2 rule 1 — "a fact is not both offered and absent" — reads per declaration, a block's, a platform's or a
service's, where it read the whole description:

- **Refused as before:** one declaration offering a fact and declaring it absent, `invalid-description`; and — one
  scope for a declaration's absence, itself and what it is made of — a block or platform declaring a fact absent while
  a block or platform it composes, at any composition depth through every `uses` it writes, offers it.
- **Accepted now:** two declarations, the absent one not composing the offerer, one declaring a fact absent and one
  offering it — two devices' two statements, each judged on its own; a platform offering a fact while a block it
  composes declares it absent among them.
- **Every declaration compared**, where `crates/eadl-model/src/presence.rs` compared only the first offer of each fact
  with its first absence.

Model §3 rule 3 — "an absence is kept" — reads through composition and through a chain: a concrete platform keeps its
abstract's absences with every block and platform it composes, and keeps the absences of every platform up the chain
its abstract refines. The decision and its reason are `docs/decisions/decision_absence-per-provider.md`.

## Why

The relation already judges each provider on its own (`docs/decisions/decision_substitutability-relation.md` §5): two
blocks writing `(f false)` and `(f true)` were accepted while the same two writing `(absent f)` and `(f true)` were
refused. And §5.3 asks platform refinement to preserve "relevant negative constraints", which rule 3 read only off the
concrete platform's own offers and only across a direct pair. `SR-H16` gave the question to `M3.4`, to decide in a
reviewed record.

## Which descriptions it invalidates

The rule refuses less and more:

- **Accepted now, refused before:** two declarations, the absent one not composing the offerer, one declaring a fact
  absent and one offering it — `10 invalid-description` before, judged now on what else they write. F06's case was
  such, two blocks `a` and `b`; it is rewritten to one block offering and declaring absent the fact, so it still
  refuses and still shows F06.
- **Refused now, accepted before:** a concrete platform offering what a platform two links up its chain declares
  absent — accepted before, since presence exempted the chain and the refinement pass read only a direct pair —
  `infeasible-configuration` now, a violated exclusion obligation.
- **Moved code:** an abstract platform's absence beside a block that a refinement of it composes, offering the fact —
  `10 invalid-description` before, rule 1's description-wide contradiction; `13 infeasible-configuration` now, the
  exclusion obligation it breaks.

No tracked description writes either of the last two: the one tracked refinement carrying an absence,
`docs/semantics/cases/infeasible-refinement-adds-exclusion.eadl`, offers the excluded fact directly, unchanged.

## Which version it lands in

`eadl/1`: the language's own rule read per declaration, as the relation reads an offer; no description's meaning
changes, only which of its statements contradict each other.
