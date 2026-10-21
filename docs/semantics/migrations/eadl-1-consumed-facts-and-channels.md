# A fact can be consumed, and a provider states its channels

- version: eadl/1
- date: 2026-10-21
- leaf: M3.3.1 (`docs/tasks/M3.md`)
- status: applied
- constructs: suite/docs/semantics/kinds/deffact.eadl, suite/docs/semantics/vocabulary/vocabulary.eadl
- invalidates: none, measured — every tracked description's `archogen check` verdict, frozen in `crates/archogen-cli/tests/verdicts.txt`, is unchanged; no tracked description writes `channels`, and nothing reads `use` until the joint check (`M3.3.2`), which `archogen check` does not call until `M3.4`

## What changed

The kind `deffact` gained an optional clause, `use`, holding `shared` — the default — or `consumed`, the latter only on
a boolean fact or a group's head, whose bare offer is `true`. A consumed fact is a function a requester programs, an
alarm, and a requester bound to it takes one of the provider's channels; a shared fact is read without being taken.
`/1` marks `relative-delay` and `absolute-deadline` consumed and every other entry shared, and gains one entry,
`channels`: a count, a guarantee, `at-least` — how many requesters a provider serves at once through its consumed
facts, one pool for all of them. The rows, with each mark's reason, are in
`docs/decisions/decision_substitutability-relation.md` §1.1, and the joint check that reads them is `M3.3`'s design in
`docs/tasks/M3.md`.

## Why

`ROADMAP.md` §5.4 asks the resolver to consider "exclusive versus shared resources and finite capacity", and F08
requires that two exclusive requests on one comparator be rejected unless a selected realization supplies sharing. A
count of what a provider serves is meaningless unless it is known which functions take a share of it: reading a
counter takes nothing, programming its comparator takes a channel. So the mark belongs to the vocabulary, where every
fact's meaning is fixed, and the count to the provider, which states what its hardware offers.

## Which descriptions it invalidates

None. The marks are read only by the joint check, which no command calls yet. A later mark — `transfer-engine`'s
channels, once the profile admits DMA — can refuse a system accepted before, and is a migration of its own (§15).

## Which version it lands in

`eadl/1`: the vocabulary grew by one entry and one clause, and no description that meant one thing now means another.
