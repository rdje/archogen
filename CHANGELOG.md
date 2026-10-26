# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

## archogen — the configuration checker re-checks the description

`ARCHOGEN-M3-0954` (leaf `M3.5.4.3`), `2026-10-26`.

- The configuration checker's second check runs the language's own checks over the description again, with the
  language definition compiled into the checker, so no plan of a description the toolchain refuses is accepted.

## archogen — the sixteenth artifact cleanup

`ARCHOGEN-PROGRAM-0953` (leaf `PROGRAM.111`), `2026-10-26`.

- Idle compiler caches removed, about 1.6 GB, every one rebuilt by the next build.

## archogen — small checks per slice, full verification after the feature set

`ARCHOGEN-PROGRAM-0952` (leaf `PROGRAM.110`), `2026-10-26`.

- The director's verification doctrine is recorded: each slice now runs only the checks of what it changed and the
  commit's own integrity checks; the full test suites, proofs and CI run once, after the configuration checker's
  features are in, under a task of their own.

## archogen — the configuration checker refuses a stale or misnamed plan

`ARCHOGEN-M3-0951` (leaf `M3.5.4.2`), `2026-10-26`.

- The configuration checker's first check: a plan written for another version of the description is refused as
  stale, with the repair "resolve again", before anything else is judged; a plan that does not read, or names
  another profile or a system the description does not declare, is refused as a corrupted plan.

## archogen — the assignment record the configuration checker judges

`ARCHOGEN-M3-0950` (leaf `M3.5.4.1`), `2026-10-24`.

- The configuration checker now reads and writes `archogen-assignment/1`, the record of what the search chose for
  one system: the profile, the system, a digest of the whole elaborated program, where the image goes, and one
  binding per requester and fact, in one canonical text pinned by a golden file and the version register.

## archogen — the configuration checker has a crate of its own

`ARCHOGEN-M3-0948` (leaf `M3.5.3`), `2026-10-24`.

- `crates/archogen-plan-check` now exists, holding the relation's executable model, moved byte for byte from the
  relation crate; its tests stay where they were and use it as a development dependency. The trust gate roots the
  configuration checker at its library, and the mutation catalogue's entries follow the moved files.

## archogen — the trust record's review history archived

`ARCHOGEN-M3-0947` (leaf `M3`), `2026-10-24`.

- The closed review's history of the record for the checkers' programs moves, byte for byte, to the archive of
  closed reviews, a short pointer left at its old place; the finished task subtree is sealed.

## archogen — the checkers' programs in the trust record, reviewed

`ARCHOGEN-M3-0946` (leaf `M3.5.2`), `2026-10-24`.

- The fifteenth review of the record that makes the generator's and the configuration checker's comparison a form of
  the trust gate's own found no defect, so its review closes: the census of what each test compiles is read only
  where it can be checked against the compiler, and held there by tests the compiler itself runs. The next step
  moves the configuration checker into a crate of its own.

## archogen — the trust report says how far the census is checked

`ARCHOGEN-M3-0945` (leaf `M3.5.2.11.4`), `2026-10-24`.

- The trust report, the record and the book now say that the census matches the compiler only as far as its rules
  and generated tests reach, and that an exempt documentation example still reads the file it names; nine quotations
  of the compiler's documentation are now exact.

## archogen — an optional dependency of a package outside the workspace is followed

`ARCHOGEN-M3-0944` (leaf `M3.5.2.11.3`), `2026-10-24`.

- The trust gate's census now follows a dependency that a package outside the workspace enables only through its own
  feature, and refuses one it cannot read; the compiler-backed test builds with such a feature and uses the census's
  own graph instead of a copy.

## archogen — an attribute the documentation tool adds to every example is read

`ARCHOGEN-M3-0943` (leaf `M3.5.2.11.2`), `2026-10-24`.

- The trust gate's census now reads what a `doc(test(…))` attribute adds to every documentation example as part of
  those examples, so a file it would make them read is refused; the compiler-backed test checks it.

## archogen — a module inside a function and a macro named by a newer keyword read as the compiler reads them

`ARCHOGEN-M3-0942` (leaf `M3.5.2.11.1`), `2026-10-24`.

- The trust gate's census now finds the files of modules declared inside a function or a constant where the
  compiler does, as measured on the pinned compiler, and treats a macro named `gen`, `try`, `async`, `await` or
  `dyn` as the macro it is in older editions; the generated tests cover both.

## archogen — the checkers' programs in the trust record, round 14

`ARCHOGEN-M3-0941` (leaf `M3.5.2`), `2026-10-24`.

- The fourteenth review found 4 defects and 5 remarks, fewer again: four more ways of naming a file the census
  misses — a module declared inside a function, a macro whose name is a keyword only in newer editions, an attribute
  the documentation tool adds to every example, and an optional dependency of a package outside the workspace. Each
  is to be read as the compiler reads it or refused, and the generated tests extended to it.

## archogen — the trust records say what the round-13 census checks

`ARCHOGEN-M3-0940` (leaf `M3.5.2.10.4`), `2026-10-24`.

- The trust records, the report and the book now say the census reads the ways of naming a file it knows, and is
  held to the compiler as far as its tests reach, no further, instead of claiming every way; and they say where a
  test-only module narrows what is counted, and how the next measurement is to be taken.

## archogen — the census's oracle compiles every head, macro spelling and inline path

`ARCHOGEN-M3-0939` (leaf `M3.5.2.10.3`), `2026-10-24`.

- The test that holds the trust gate's census to the compiler now also compiles every way a file can begin, every
  way a macro can spell a file's name, and every way nested modules can place a file, and checks the census counts
  or refuses each file the compiler read; each new check fails against the census as it stood before this round's
  fixes.

## archogen — a file's first line and an inline module's path read as the compiler reads them

`ARCHOGEN-M3-0938` (leaf `M3.5.2.10.2`), `2026-10-24`.

- The trust gate's census now decides whether a file opens with a shebang line by the compiler's own rule, which
  the repository already held to the pinned compiler, and finds the files of modules nested in a module whose path
  is set where the compiler finds them, as measured on the pinned compiler.

## archogen — a macro that may rearrange what it is given is read whole or refused

`ARCHOGEN-M3-0937` (leaf `M3.5.2.10.1`), `2026-10-24`.

- The trust gate's census now refuses a file's name, a module or a debugger visualizer handed to a macro that can
  reorder what it is given — any macro but std's own expression macros such as `assert_eq!` and `format!` — and
  the same inside a macro's definition, with a macro or an attribute built from its parameters; reusing a name of
  std's for another macro, importing macros wholesale, exporting one, and a doc test defining its own are refused
  too.

## archogen — the checkers' programs in the trust record, round 13

`ARCHOGEN-M3-0936` (leaf `M3.5.2`), `2026-10-24`.

- The thirteenth review found 5 defects and 6 remarks, half the last: the census's rule of refusing what it cannot
  read as the compiler does held, but had not reached three ways of naming a file — through a macro's parameter, at
  a file's very first line, and from a module whose path is set inside another file. They are to be closed the same
  way, and the generated test extended to each.

## archogen — the trust records say what the round-12 census does

`ARCHOGEN-M3-0935` (leaf `M3.5.2.9.5`), `2026-10-24`.

- The trust records, the report and the book now say that a test reading another role's files at run time is
  unseen, when the census's refusals stand beside the gate's others, where their quotations come from, and what a
  non-member contributes; the shared lexer's own gap, in the gate's older token rules, is filed with its owner.

## archogen — the census's oracle generates every pairing of the language's literals and comments

`ARCHOGEN-M3-0934` (leaf `M3.5.2.9.4`), `2026-10-24`.

- A new test builds, with the pinned compiler, every pairing of the language's string, character and comment forms
  around a file inclusion, and checks the census reads each where the compiler does; four deliberate lexer faults
  each make it fail. The probe corpus gains the twelfth review's cases.

