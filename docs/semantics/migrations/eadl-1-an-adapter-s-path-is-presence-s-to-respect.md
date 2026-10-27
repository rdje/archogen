# Presence yields to an engine adapter's path

- version: eadl/1
- date: 2026-10-27
- leaf: M3.2.4 (`docs/tasks/M3.md`), answering `SR-H9`
- status: applied
- constructs: suite/examples/alternative-timer/system.eadl
- invalidates: two frozen verdicts move, measured over every tracked description — `examples/alternative-timer/system.eadl` from `13 infeasible-configuration` to `11 missing-fact`, and `crates/eadl-resolve/tests/search/adapter-deadline-over-delay.eadl` from `11 missing-fact` to `0`; no description's text changes

## What changed

Model §2 rules 2 and 3 — a required fact declared absent and offered by none is `infeasible-configuration`, one nothing
describes `missing-fact` — now read the engine's adapter catalog by name
(`docs/specs/adapters/decision_engine-adapters.md` §7). For a required fact a kind **adds** — offers, not passed
through from itself, and not named beneath — they ask whether one block or platform of the description offers
everything the kind's `beneath` names:

- **A path**: one does. The fact is the search's to bind or refuse, and presence refuses nothing for it — `SR-H9`, an
  adapter's offer satisfying past a declared absence.
- **A partial path**: one offers part of it and leaves the rest undescribed there. The fact is `missing-fact`, its hint
  naming the kind, the declaration and each fact to describe.
- **No path**: rule 2 or 3 reads as before, its hint naming each kind that adds the fact and what it needs.

A fact no kind adds, or one some declaration offers, is judged as before.

## Why

`archogen check` runs presence before any search, so a fact an adapter may realize must not be refused before the
search can bind one, and a refusal that describing one fact would lift says so. Presence reads names alone, so it
knows a path may exist, never whether it holds by value: a description whose path holds by name and fails by value is
accepted by `archogen check` and refused by `archogen resolve` until `M3.4.7` makes the check adopt the search's
verdict, an obligation written into that leaf.

## Which descriptions it invalidates

- `examples/alternative-timer/system.eadl`, `uc3`: `timer.delay` declares `absolute-deadline` absent and offers
  `relative-delay`, `tick-rate` and `observation-coherent`, leaving `counter-modulus` undescribed — a partial path for
  `deadline-over-delay`. From `13 infeasible-configuration` to `11 missing-fact`, whose repair, describing the
  modulus, is the change `SR-H10` brings to the director before it is made (`M3.4.7`).
- `crates/eadl-resolve/tests/search/adapter-deadline-over-delay.eadl`: its timer offers all four facts the kind needs,
  a path; from `11 missing-fact` to `0`, the search binding the adapter.

## Which version it lands in

`eadl/1`: presence learns where the engine may realize a fact; no description's text or meaning changes, only which of
its requirements the language check leaves to the search.
