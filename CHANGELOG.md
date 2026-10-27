# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the search writes its plan

`ARCHOGEN-M3-1004` (leaf `M3.4.3`), `2026-10-27`.

- What the search chooses is written as the assignment record the configuration checker reads, by the generator's own
  code. A plan written for `s0-heartbeat` is read back and accepted by the checker.

## archogen — the search chooses a system's providers

`ARCHOGEN-M3-1003` (leaf `M3.4.2`), `2026-10-27`.

- The resolver can now choose, for one system, a provider for each thing each of its users requires, among the
  system's own hardware, in a fixed order, until the joint rules accept the whole choice — or say why none exists, in
  the roadmap's diagnostic terms, or stop at a budget without claiming impossibility. `archogen resolve`, which will
  run it, comes next.

## archogen — the checker's browser module, loaded and checked as a page runs it

`ARCHOGEN-M3-1001` (leaf `M3.5.7.2`), `2026-10-27`.

- A page loads the configuration checker's module through `crates/archogen-plan-check-wasm/js/plan-check.mjs`, which
  fingerprints the module with the browser's own SHA-256. A new `integration` step, `plan-check-wasm`, runs the
  compiled module that way over every tracked description, and holds each answer to the host build's byte for byte
  and to `archogen-plan-check`'s verdict over the same files.

## archogen — the configuration checker compiles to a browser module of its own

`ARCHOGEN-M3-1000` (leaf `M3.5.7.1`), `2026-10-27`.

- `crates/archogen-plan-check-wasm` is the configuration checker's own WebAssembly module, sharing nothing with the
  generator's: a page hands it the description, its modules and the plan as texts, and it answers the same result
  record the executable and the MCP tool write, naming itself. It imports nothing. Its loader and the checks of the
  compiled module are the next leaf's.

## archogen — the configuration checker serves its own MCP tool

`ARCHOGEN-M3-0998` (leaf `M3.5.1.6.4`), `2026-10-27`.

- `archogen-plan-check --mcp` serves the plan check to an agent as the MCP tool `plan-check`: the description's and
  the plan's texts in, never paths, and the result record answered inline — the same record, byte for byte, that the
  executable writes for files holding the same texts. It runs over the protocol package the generator's server uses.

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

