# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the MCP protocol package reviewed, and four faults fixed

`ARCHOGEN-M3-0997` (leaf `M3.5.1.6.3`), `2026-10-26`.

- The shared MCP protocol package's entry in the trust record now states what it risks and what checks it. Reviewing
  it found four faults, all fixed: a tool's metadata written twice, a bad one let through in the older protocol era, a
  request id that could crash a debug build, and an unreadable tool answer passed on. The package now has tests of its
  own, comparing every answer with an independent model of the protocol's rules.

## archogen — the browser module and the MCP server write strings one way

`ARCHOGEN-M3-0996` (leaf `M3.5.1.6.2`), `2026-10-26`.

- The browser module's JSON encoder now writes every string through the same writer the MCP server uses, instead of
  its own copy; its answers are unchanged, byte for byte.

## archogen — the MCP protocol is a package every server can share

`ARCHOGEN-M3-0995` (leaf `M3.5.1.6.1`), `2026-10-26`.

- The MCP server's protocol handling moved out of the command line into `crates/archogen-transport`, a package of no
  role's, so each checker can serve its own tools over the same protocol without sharing the generator's. `archogen
  mcp` answers exactly as before.

## archogen — the trust gate knows the front runs the configuration checker

`ARCHOGEN-M3-0993` (leaf `M3.5.1.4.4`), `2026-10-26`.

- `trust/roots.eadl` now says the `archogen` front runs the configuration checker and reads the install manifest, so
  the trust gate records that invocation and the launcher it goes through, and `cargo xtask install` places exactly
  the programs the front runs.

## archogen — the front believes a checker's result only when it holds together

`ARCHOGEN-M3-0992` (leaf `M3.5.1.4.3`), `2026-10-26`.

- When `archogen` runs the configuration checker over a plan of its own, as `archogen resolve` will, it now believes
  the result only when the record, the program's exit code and its own fingerprints of the description and the plan
  all agree; anything else is reported as a toolchain failure naming what disagreed.

## archogen — `archogen plan-check` runs the configuration checker

`ARCHOGEN-M3-0991` (leaf `M3.5.1.4.1`), `2026-10-26`.

- `archogen plan-check DESCRIPTION PLAN` now runs the configuration checker, a program of its own installed beside
  `archogen`, as git runs its sibling commands. It runs a checker only after confirming the installed program is the
  one the install placed, and nothing but the command line can start it: the library, the engine API and the MCP
  server still run no program.

## archogen — the scaffold's files are the ones the director's ruling holds

`ARCHOGEN-PROGRAM-0989` (leaf `PROGRAM.116`), `2026-10-26`.

- Two checks treated every file the scaffold updater lists as off-limits, though the director had ruled that a file
  this project has changed is its own. They now read the ruling's list, and the acceptance check's own scratch and
  reads — the kind that failed on CI earlier the same day — are fixed rather than reported upstream.

## archogen — what the configuration checker shares with the generator, classified

`ARCHOGEN-M3-0988` (leaf `M3.5.6.5`), `2026-10-26`.

- Each item the configuration checker shares with another program — the language reader, the model, the record and
  JSON packages, the kind definitions, the digest package, the toolchain — now carries in the trust gate's baseline
  what kind of item it is, the property it can affect, the error both could make and what checks it independently,
  ready for review.

## archogen — the shared digest package classified for timing

`ARCHOGEN-M3-0987` (leaf `M3.5.1.8`), `2026-10-26`.

- The trust gate's form for the digest package the generator and the scheduling checker both build now states what
  it is, what it can affect and what independently checks it, ready for review.

## archogen — the configuration checker follows the settled joint rules

`ARCHOGEN-M3-0986` (leaf `M3.5.5.7.5`), `2026-10-26`.

- The configuration checker no longer lets two devices share an address because a service links them, and it refuses
  a description whose reserved memory range states no size where that size would decide what the image must avoid.

## archogen — the joint rules say what the two readings decided

`ARCHOGEN-M3-0985` (leaf `M3.5.5.7.3`), `2026-10-26`.

- The joint rules' text now settles five points the configuration checker's independent author had to guess at, and
  two the generator and the checker read differently: only blocks and platforms compose one another, and a reserved
  memory range with no size leaves the description short of a fact. The checker follows in the next leaf.

## archogen — the generator and the checker read every system alike

`ARCHOGEN-M3-0984` (leaf `M3.5.5.7.2`), `2026-10-26`.

- A new test holds the generator's reading of a system — its hardware, what it reaches, whom it serves — equal to the
  configuration checker's, which was written apart, over every tracked system and thousands of generated ones; the
  trust gate lists the test as a reading the two roles share.

## archogen — the configuration checker's reading of a system can be compared

`ARCHOGEN-M3-0983` (leaf `M3.5.5.7.1`), `2026-10-26`.

- The configuration checker now offers, as a public function, how its joint check reads a system: its hardware, what
  it reaches and whom it serves. A context that never saw the generator's code made the change, so a test can compare
  the two readings without either having shaped the other.

## archogen — the trust gate sees the relation's two comparisons

`ARCHOGEN-M3-0981` (leaf `M3.5.3.1`), `2026-10-26`.

- The two tests that hold the generator's relation equal to the configuration checker's model, `production` and
  `vocabulary`, are now named in `trust/roots.eadl`, so the trust gate builds them, scans what they compile and lists
  each as a reading the two roles share.

## archogen — a check's own reading cannot fail it

`ARCHOGEN-PROGRAM-0980` (leaf `PROGRAM.114`), `2026-10-26`.

- Every check in this repository's scripts that piped a captured output into `grep -q` now reads it without a pipe,
  and a new gate, `PIPE-READS`, refuses the pattern, which can turn a passing check into a failure on Linux.

## archogen — the self-test failure on CI found and fixed

`ARCHOGEN-PROGRAM-0978` (leaf `PROGRAM.113.3`), `2026-10-26`.

- The task-history gate's own tests failed on CI's Linux runner, not because the gate was wrong but because each test
  piped a large output into `grep -q`: GNU grep stops reading at its first match, the writer then meets a closed pipe,
  and `pipefail` turns the match into a failure. They now read the output without a pipe; the same pattern elsewhere is
  filed.

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

