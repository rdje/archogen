# A plan names the engine's adapters: `archogen-assignment/2`

- version: eadl/1
- date: 2026-10-27
- leaf: M3.2.5 (`docs/tasks/M3.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: no description — every tracked description's verdict unchanged; a plan record written as `archogen-assignment/1` is refused by the configuration checker as another format, `plan-record`, and is resolved again

## What changed

The assignment record a plan carries is `archogen-assignment/2` (`docs/specs/adapters/decision_engine-adapters.md`
§5): `/1` with an `(adapters adapters/1)` clause after `(program …)`, naming the engine's adapter catalog the plan
was resolved with, and an engine adapter's instance — `(adapter KIND OVER)`, or `(adapter KIND OVER REQUESTER)` for an
exclusive kind — wherever a requester or a provider stands. `docs/semantics/reference.md`'s diagnostics table states
`plan-record`'s refusals for it: `/1` among the other formats, a catalog other than the checker's, a kind its table
does not hold, an instance not of its kind's shape. The table's example records are written `/2`.

## Why

The format register makes a provider or requester kind a new version, and an adapter's instance is both. Every plan
is written `/2` from this commit, one format at a time; `/1`'s golden stays frozen beside `/2`'s and is refused.

## Which descriptions it invalidates

None: a description's meaning and verdict do not move. A plan written before this commit names no catalog and is
refused as another format; resolving the description again writes its `/2` plan.

## Which version it lands in

`eadl/1`: the language is unchanged; the record of what the search chose names one more kind of provider.
