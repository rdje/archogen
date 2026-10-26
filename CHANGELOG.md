# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the failing self-test arms trace what happened

`ARCHOGEN-PROGRAM-0977` (leaf `PROGRAM.113.2`), `2026-10-26`.

- The task-history gate's signal tests now record, in order, the signal sent, each change of the signal mask and each
  outcome printed, so CI's next run shows where the Linux runner departs from the designed sequence.

## archogen — the failing self-test arms now say why

`ARCHOGEN-PROGRAM-0976` (leaf `PROGRAM.113.1`), `2026-10-26`.

- Several signal arms of the task-history gate's own tests fail on CI's Linux runner; their failure line now carries
  the evidence, which CI's log otherwise drops, so the next run names the cause.

## archogen — the trust gate's forms, measured by the gate that judges them

`ARCHOGEN-M3-0975` (leaf `M3.5.6.5`), `2026-10-26`.

- The trust gate now running on `main` measures one aspect of eight shared items otherwise than the older gate that
  proposed their forms; its own proposal replaces them, awaiting review.

## archogen — an intermittent self-test arm filed

`ARCHOGEN-PROGRAM-0974` (leaf `PROGRAM.113`), `2026-10-26`.

- One of the task-history gate's own tests failed once on CI's Linux runner and passed when re-run; it is filed to be
  reproduced on Linux and fixed by what that shows.

## archogen — the trust gate's forms for what the configuration checker shares

`ARCHOGEN-M3-0973` (leaf `M3.5.6.5`), `2026-10-26`.

- The trust gate's first run after the push found fourteen items two programs now share — mostly what the
  configuration checker shares with the generator — with no form; the forms it proposed on its own host are committed,
  awaiting review, so CI judges them.

## archogen — the trust inventory writes again, every program accounted for

`ARCHOGEN-M3-0972` (leaf `M3.5.8`), `2026-10-26`.

- The trust inventory admits the one inclusion site it could not follow and classifies the runtime core's three test
  programs, so it writes its record with every program target accounted for.
- The generator's tests no longer compile the configuration checker: the checker's codes are now exercised by the
  checker's own tests, through its executable.

## archogen — the task history's index is append-only in order

`ARCHOGEN-PROGRAM-0971` (leaf `PROGRAM.71`), `2026-10-26`.

- The task-history gate now refuses a committed row of the index moved, or a row inserted above committed ones, as
  well as one removed or changed: the index is append-only, as its record says.

## archogen — the trust gate judges the configuration checker as the program it is

`ARCHOGEN-M3-0970` (leaf `M3.5.6.2`), `2026-10-26`.

- The trust gate's record of the configuration checker now names its executable, `archogen-plan-check`, rather than
  its library, so an install places it beside `archogen`.

## archogen — the configuration checker runs as its own program

`ARCHOGEN-M3-0969` (leaf `M3.5.6.1`), `2026-10-26`.

- `archogen-plan-check DESCRIPTION PLAN` checks a plan against its description, writes a result record naming the
  program that ran and what it judged, and exits with the result's status — the program `archogen` will run beside
  itself. A plain description, with no module, is read as one module of its own.

## archogen — the joint check, re-written independently

`ARCHOGEN-M3-0967` (leaf `M3.5.5.8`), `2026-10-26`.

- The configuration checker's joint check is now the one a fresh context wrote from the rules alone, never having
  read the generator's version. Run beside the earlier one over every test case, the two agree wherever a plan reaches
  the check.

## archogen — F12's joint corruptions, and the joint check's independence found missing

`ARCHOGEN-M3-0966` (leaf `M3.5.5.6`), `2026-10-26`.

- Fixture F12 now corrupts a valid plan once per joint rule, and the configuration checker refuses each.
- The joint check was meant to be written by a context that never read the generator's version of the same rules;
  the session that wrote it had read that version four days earlier. It is to be re-written by a fresh context.

## archogen — the configuration checker uses each device whole

`ARCHOGEN-M3-0965` (leaf `M3.5.5.5`), `2026-10-26`.

- The configuration checker refuses a plan that reads one counter's width and another's rate for the same service,
  and checks a need that qualifies everything a service relies on — such as being available while the processor
  idles — against every part that service is bound to.

## archogen — the configuration checker checks every caller can reach its parts

`ARCHOGEN-M3-0964` (leaf `M3.5.5.4`), `2026-10-26`.

- The configuration checker refuses a plan that binds a task or service to a part it cannot reach at the processor
  level the profile runs everything at, and reports a service that says it calls from another level as a
  description this profile cannot build.

## archogen — the configuration checker guards addresses and where the program goes

`ARCHOGEN-M3-0963` (leaf `M3.5.5.3`), `2026-10-26`.

- The configuration checker refuses a plan that uses two parts whose registers share an address, unless one is built
  from the other, and a plan that places the program nowhere, or anywhere but memory it can run from on the system's
  own hardware.

## archogen — the configuration checker counts a timer's channels

`ARCHOGEN-M3-0962` (leaf `M3.5.5.2`), `2026-10-26`.

- The configuration checker refuses a plan that binds more delays and deadlines to a timer than it has channels, or
  two to a timer whose channels are not described; a service several tasks share counts once.

## archogen — the configuration checker judges bindings together: a system's own hardware

`ARCHOGEN-M3-0961` (leaf `M3.5.5.1`), `2026-10-26`.

- The configuration checker's fifth check refuses a plan that binds a need to a part the system's platform never
  composes, under the new code `plan-joint`, stated in the language reference with an input that fires it.

## archogen — doctrine 22's record states what every commit still runs

`ARCHOGEN-PROGRAM-0959` (leaf `PROGRAM.112`), `2026-10-26`.

- The record of the director's doctrine 22 no longer lists as deferred what `make focused` runs at every commit —
  workspace clippy and the whole test suite — and says a pin a change moves is re-pinned by that change.

## archogen — the plan checks' subtree sealed, their migration note a record

`ARCHOGEN-M3-0958` (leaf `M3`), `2026-10-26`.

- The closed `M3.5.4` — the assignment record and checks (1)–(4) — sealed into the task history, and the note
  covering the plan codes' rows in the language reference marked applied.

## archogen — a plan is checked whole, and its verdicts are stated

`ARCHOGEN-M3-0957` (leaf `M3.5.4.5`), `2026-10-26`.

- The configuration checker runs its checks in order over a plan, and every corruption of fixture F12 is refused
  under a code of its own. The codes are stated in the language reference, each with an input that fires it, and a
  new book chapter, *Checking a plan*, explains them.

## archogen — the configuration checker judges each binding

`ARCHOGEN-M3-0956` (leaf `M3.5.4.4`), `2026-10-26`.

- The configuration checker now refuses a plan that leaves a requirement unbound, binds what nothing requires, a
  requester its system does not serve or a provider the description does not declare, or binds a requirement to a
  provider that does not satisfy it; each finding points at the requirement in the description and the binding in
  the plan.

## archogen — the configuration checker refuses an impossible provider or memory map

`ARCHOGEN-M3-0955` (leaf `M3.5.4.6`), `2026-10-26`.

- The configuration checker now refuses a description whose provider offers a function that takes a channel while
  declaring none, whose memory region is not well formed, or whose placement memory is overlapped by a device or
  another memory or reserved whole.

