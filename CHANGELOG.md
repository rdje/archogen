# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the checkers' programs in the trust record, round 9, and a second change of method

`ARCHOGEN-M3-0913` (leaf `M3.5.2`), `2026-10-23`.

- The ninth independent review of the trust record found 4 defects and 7 remarks, up for a second round: the census
  missed tests cargo builds only on request, code only a debug build compiles, and nested packages. Rather than
  answer probe by probe, what decides what a test compiles becomes a table drawn from cargo's own documentation,
  each row checked against what cargo actually compiles. The classification of `rt-core`'s tests is now tied to
  one push.

## archogen — a program compiling another role's file runs two roles

`ARCHOGEN-M3-0912` (leaf `M3.5.2.5.3`), `2026-10-23`.

- A root, or a comparison, that compiles a file of another role's package by a path or an `include!` is now
  refused as one that depends on that package is, not merely reported as sharing the file. No real program
  changes.

## archogen — the trust gate reports a step's refusals together

`ARCHOGEN-M3-0911` (leaf `M3.5.2.5.2`), `2026-10-23`.

- A commit the trust gate refuses for several reasons before any build — a missing launcher, a stale form, a
  package outside the tree, a refused cargo configuration — now hears every one in one report, and a run it cannot
  judge names the refusals already found. A test compiling a third role's package is told to split or classify
  it, not offered a pair form that would be refused.

## archogen — the trust gate's census reads what the compiler compiled

`ARCHOGEN-M3-0910` (leaf `M3.5.2.5.1`), `2026-10-23`.

- Whether a test joins two roles is now decided from the compiler's own record of what each build read, not from
  how a file's text names another, so no spelling hides a comparison; a package with a build script is judged by its
  dependencies and never run. The gate takes about half a minute more.

## archogen — the checkers' programs in the trust record, round 8, and a change of method

`ARCHOGEN-M3-0909` (leaf `M3.5.2`), `2026-10-23`.

- The eighth review found 3 defects and 10 remarks: each round had found another way to spell a file inclusion the
  census missed. From the next leaf the census reads what the compiler compiled, so no spelling can hide one.

## archogen — the specifications folder one step larger

`ARCHOGEN-PROGRAM-0908` (leaf `PROGRAM.108`), `2026-10-23`.

- `docs/specs/` may hold 640 KiB, measured and recorded, so the open trust review's answers have room.

## archogen — a test joining two pairs is told it must be split or classified

`ARCHOGEN-M3-0907` (leaf `M3.5.2.4.2`), `2026-10-23`.

- The last of the seventh review's findings answered: no form can name a test joining two pairs, and the gate says
  so; a test's entry point never serves as a generator file, now tested.

## archogen — the trust gate sees a test that copies a role's file in

`ARCHOGEN-M3-0906` (leaf `M3.5.2.4.1`), `2026-10-23`.

- A test that compiles another role's file through a path attribute or `include!` now counts as joining that
  role, with no dependency needed; the report says what the census reads and what it cannot.

## archogen — the checkers' programs in the trust record, round 7

`ARCHOGEN-M3-0905` (leaf `M3.5.2`), `2026-10-23`.

- The seventh review found 2 defects and 6 remarks, down from 5: a test could still compile another role's files
  by a path attribute, unseen, and two records disagreed on generator files. Both answered; the code follows.

## archogen — the trust gate names the right form for each pair

`ARCHOGEN-M3-0904` (leaf `M3.5.2.3.2`), `2026-10-23`.

- A test joining the reference model and the implementation is told to take a harness form, one joining the
  generator and a checker a derivation; a test both named and classified is refused. Round 6 is answered.

## archogen — the trust gate counts every test any build could compile

`ARCHOGEN-M3-0903` (leaf `M3.5.2.3.1`), `2026-10-23`.

- A test that joins two roles is found whether it reaches them through a build script's dependency, a feature or
  another platform's dependency, and whatever flags say it is skipped by default; the report says what stays outside.

## archogen — the checkers' programs in the trust record, round 6

`ARCHOGEN-M3-0902` (leaf `M3.5.2`), `2026-10-23`.

- The sixth review found 5 defects and 8 remarks; the census of tests that join two roles counted only what the
  default build makes. From the next leaf it counts what any build can compile, and says what stays outside.

## archogen — the engine's API reference, built with the book

`ARCHOGEN-PROGRAM-0901` (leaf `PROGRAM.107`), `2026-10-23`.

- The book now carries a reference of the crates a consumer compiles against, generated by rustdoc under `api/` and
  linked from the engine API chapter. Every public item is documented, a doc link to nothing fails the book, and the
  book stays the contract.

## archogen — task-tree sealing, written for any project to adopt

`ARCHOGEN-PROGRAM-0900` (leaf `PROGRAM.106`), `2026-10-23`.

- `docs/TASK_TREE_SEALING.md` explains how this project keeps its task trees short without losing a byte of their
  history, in terms any project and any agent or harness can adopt. Next, the engine's API reference by rustdoc.

## archogen — every rule the checkers' trust record states, tested

`ARCHOGEN-M3-0899` (leaf `M3.5.2.2.3`), `2026-10-23`.

- The last of the fifth review's findings answered in code: the invocation judged through the gate, a derivation
  read as the comparison harness for what it generates and whom it may use, and the book's list of what the gate
  does not see held to the report's by a test.

## archogen — the trust gate finds a comparison by what it compiles

`ARCHOGEN-M3-0898` (leaf `M3.5.2.2.2`), `2026-10-23`.

- Every test, bench, unit test or doc test whose package compiles the code of two roles a claim keeps apart is now a
  program the gate names or a review classifies, whatever its sources call the crates; a derivation may pair the
  generator with either checker. Today it finds three of `rt-core`'s tests, classified after the next push.

## archogen — the trust gate reads a clause whole, and a front's fault at its form

`ARCHOGEN-M3-0897` (leaf `M3.5.2.2.1`), `2026-10-23`.

- A clause in `trust/roots.eadl` holding a number or a list is refused rather than read without it, and a fault in
  the front's own form stops the reading there, as the records say.

## archogen — the checkers' programs in the trust record, round 5 read against the code

`ARCHOGEN-M3-0896` (leaf `M3.5.2`), `2026-10-23`.

- The fifth review read the record against the gate's code and found 7 defects and 10 remarks: a census that reads
  crate names from a test's text misses a comparison spelt another way. From the next leaf the build decides: a test
  whose package compiles two roles' packages is a program the gate names or a review classifies.

## archogen — the trust gate's readers for the checkers' programs, whole

`ARCHOGEN-M3-0895` (leaf `M3.5.2.1.6`), `2026-10-23`.

- Every report now says it does not see a derivation's run-time reads, an edit to either transcription it holds
  together, or which programs the front really runs. With two last fixtures the gate's readers are whole; no form
  uses them until they are on `main`.

## archogen — a comparison found by machine

`ARCHOGEN-M3-0894` (leaf `M3.5.2.1.5`), `2026-10-23`.

- The trust gate reads every integration test's sources and reports one that names both a generator's crate and a
  checker's while no form names it, so a comparison two roles share cannot go unseen; today's tree has none.

