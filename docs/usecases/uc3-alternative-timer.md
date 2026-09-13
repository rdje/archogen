# `uc3-alternative-timer` — indirect realization

**Status:** rejection fixture before M3; supported from M3. **First gate:** M3.

A system that requires an **absolute-deadline** time service on a platform whose hardware
offers only a **relative delay** timer. Nothing matches directly. Either the engine composes
the required service from what is offered, or it says so.

## What it exercises

§5.4 states the rule this case exists for:

> Engine search may realize an OS feature indirectly from lower-level hardware functionality.
> A software deadline service can use a supported relative timer through an engine-owned
> adapter, provided the resulting behavior and bounds satisfy the eADL requirement. Direct
> hardware offer matching is only one realization path. If no known realization exists, explain
> the missing engine capability; do not declare the requested function logically impossible
> unless that conclusion has actually been established.

Three distinct behaviors are therefore under test:

| Situation | Required answer |
| --- | --- |
| Before the adapter exists (`M3.2`) | a **missing engine capability** report naming what is absent |
| After the adapter exists | a built system, with the adapter's own costs and obligations in the plan |
| If the adapter cannot meet the bound | `infeasible-configuration` or `not-established` with the violated bound named — never "impossible" |

## The adapter's obligations are the point

An adapter is a real provider, not a convenience. §6.2 requires it to account for the time
between reading "now" and completing the programming, including bounded interrupt delay and
already-expired deadlines. Those become:

- a **cost** in the resolved plan, charged to a declared ledger category;
- a **proof obligation** — that the composed behavior satisfies the eADL contract;
- two **fixtures** — F14 (a deadline already expired during programming produces the
  contractual immediate/late behavior with no silently lost timer) and F13 (rollover and
  ambiguous horizon).

## What must not happen

The description must not change. If the only way to build this system is to weaken
`absolute-deadline` into `relative-delay` in the eADL source, the engine has failed and the
boundary has been crossed — §17's first stop/rework trigger: "a supposedly functional eADL
example needs driver/model/code-generation instructions → repair the boundary; move
implementation to engine knowledge."

## Why the status changes

This is the only one of the four whose expected answer changes as a capability lands, and that
is deliberate. It is how the project distinguishes *adding an engine capability* from
*weakening a requirement until it passes*. Both transitions turn a red fixture green; only one
of them is progress.
