# Each fact names the function it describes

- version: eadl/1
- date: 2026-10-22
- leaf: M3.3.6.1 (`docs/tasks/M3.md`)
- status: applied
- constructs: suite/docs/semantics/kinds/deffact.eadl, suite/docs/semantics/vocabulary/vocabulary.eadl
- invalidates: none, measured — every tracked description's `archogen check` verdict, frozen in `crates/archogen-cli/tests/verdicts.txt`, is unchanged; no description writes a `deffact`, and nothing reads `function` until the joint check's rule 13 (`M3.3.6.2`), which `archogen check` does not call until `M3.4`

## What changed

The kind `deffact` gained an optional clause, `function`, holding one symbol: the name of the function the fact
describes — the operations of one device that are only valid together — or `each`, for a qualifier that describes every
function its requirer relies on. The typed table makes it required of a guarantee and refuses it on a statement, refuses
`each` on a consumed fact, a group's head or sub-fact and a derived fact, and refuses a group's sub-fact or a
derivation's input that is not its own function's.

Every guarantee of `/1` names its function, each with its reason in a new column of
`docs/decisions/decision_substitutability-relation.md` §1.1: `counter` — the counter's width, modulus, rate, unit, wrap,
horizon, coherent reads and low-power counting, its two alarms `absolute-deadline` (with `supported-horizon` and
`delivery-bound`) and `relative-delay`, and `channels`, the alarms' pool; `interrupt`, `arrival`, `output`, `queue`,
`transfer`, `processor`, `scheduler`, `clock`, `release`, `debug`, `mutex` and `bus`; `each` for `available-in-state`
and `reachable-at-privilege`. `or-through-mediation`, a statement, names none. Two doc lines narrowed to the counter:
`observation-coherent`'s ("a read of the counter's value") and `tick-unit`'s ("the unit a counter reports time in").

## Why

`SR-H14` asks that the requirements one service makes of one device's facts be met by one provider, and `ROADMAP.md`
§12 M3 asks for "provider bundles where operations are only valid together". Nothing in `/1` said which facts are one
device's, and a `requires` clause is not the unit: the suite writes one counter's facts across a clause and an
own-level `needs` (`examples/bounded-queue/system.eadl`). The design and its review are `M3.3.6`'s in
`docs/tasks/M3.md`.

## Which descriptions it invalidates

None. A description never writes `deffact`; the vocabulary and its kind are the language's own definitions, which
`archogen check` answers with exit 20, as before. The joint rule that reads `function` lands with `M3.3.6.2`, and
`archogen check` calls the joint rules only from `M3.4`.

## Which version it lands in

`eadl/1`: the vocabulary is versioned with the language, and an entry changes only by this process (record §1.1).
