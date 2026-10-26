# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

## archogen — the trust gate's census narrows nothing a build script can configure

`ARCHOGEN-M3-0933` (leaf `M3.5.2.9.3`), `2026-10-24`.

- A build script can turn on the test-only setting for its own package, so code the census set aside as test-only
  is now counted for every user of a package that has a build script; and a test now holds the census's refusals
  beside a build that fails.

## archogen — the trust gate's census reads every module's own file beside its paths

`ARCHOGEN-M3-0932` (leaf `M3.5.2.9.2`), `2026-10-24`.

- A module whose file is chosen by a conditional `path` attribute is now read at its usual file too, which the
  compiler reads when the condition is off; a `path` attribute inside a module is refused, and every path is
  checked for absolute or host-dependent spellings as soon as it is read.

## archogen — the trust gate's census reads only its alphabet

`ARCHOGEN-M3-0931` (leaf `M3.5.2.9.1`), `2026-10-24`.

- The census now refuses a source file holding, outside its comments and literals, any character but printable
  ASCII and ordinary whitespace — a Unicode space, a control byte, a byte-order mark — or opening with a shebang,
  since the compiler and the census could read such a file differently. Documentation is read with its pieces
  joined, past every kind of space, so prose stays prose and no split or odd space hides a doc test.

## archogen — the checkers' programs in the trust record, round 12, and a fifth change of method

`ARCHOGEN-M3-0930` (leaf `M3.5.2`), `2026-10-24`.

- The twelfth review of the trust record found 10 defects and 7 remarks: the census refused what it could not read,
  but its own idea of what a token is, which file a path names and what a dependency's settings are still differed
  from the compiler's. It is to read only what it can check it reads as the compiler does — a fixed alphabet, every
  path with its default, no test-only narrowing a build script can undo — refuse the rest, and be held to the
  compiler by a generated corpus of every pairing of the language's literals and comments.

## archogen — the trust records say what the round-11 census does

`ARCHOGEN-M3-0929` (leaf `M3.5.2.8.5`), `2026-10-24`.

- The parent trust record, the role-programs record, the report and the book now describe the census as it reads
  today, say what an admitted inclusion leaves unseen, state how the census was measured, and give every unit a
  census finds unnamed to the leaf that classifies them; the eleventh review's answers are all carried out.

## archogen — the fifteenth daily artifact cleanup

`ARCHOGEN-PROGRAM-0928` (leaf `PROGRAM.109`), `2026-10-24`.

- Remove only the idle main and wasm debug incremental caches — 29847 files, 4245385971 logical bytes — after two
  handoff censuses and an inventory whose identities were rechecked first; every other entry under `target/` is
  identical or accounted, the cited evidence keeps its digests, and the tracked inputs restore.
- Overwrite `docs/ARTIFACT_CLEANUP.md`, add the run's section to the repository annex, and seal `PROGRAM.100`–`.102`
  and `.104`–`.108`.

## archogen — the trust gate's census oracle runs rustdoc and follows each test's dependencies

`ARCHOGEN-M3-0927` (leaf `M3.5.2.8.4`), `2026-10-24`.

- The test that holds the census to the compiler now also follows every crate a test links and every build script
  behind them, runs rustdoc on documentation tests written in each form the eleventh review found, and covers that
  review's other cases; five deliberate breaks of the census each make it fail.

## archogen — the trust gate refuses what its written tree cannot show

`ARCHOGEN-M3-0926` (leaf `M3.5.2.8.3`), `2026-10-23`.

- The gate writes the commit's files without symbolic links, so a link where cargo looks for packages and targets
  is now refused, as is a target whose source file the commit lacks. The census's refusals are reported with the
  gate's other early ones, before anything is built, and a build script's files count for its package.

## archogen — the trust gate's census reads documentation with no markdown

`ARCHOGEN-M3-0925` (leaf `M3.5.2.8.2`), `2026-10-23`.

- A library's documentation can hold doc tests in more spellings than the census parsed — other fences, indented
  blocks, a test split across attributes. The census no longer reads markdown at all: it refuses every word in a
  library's documentation that could make a doc test read a file, except one set off by a backtick in prose,
  which no compiled code can hold.

## archogen — the trust gate's census reads every word that can include a file

`ARCHOGEN-M3-0924` (leaf `M3.5.2.8.1`), `2026-10-23`.

- The census now reads each word in a source that can make the compiler read a file — a module, an inclusion, an
  assembly template, a documentation or debugger attribute — whole, or refuses it: a raw-identifier module is
  followed, a module a macro places is refused, a test-only attribute narrows a module alone, and a path is
  refused when it leaves the tree, passes a symbolic link anywhere, or is spelt in a way hosts read differently.

## archogen — the checkers' programs in the trust record, round 11, and a fourth change of method

`ARCHOGEN-M3-0923` (leaf `M3.5.2`), `2026-10-23`.

- The eleventh review of the trust record found 8 defects and 9 remarks, up a fourth round: the census's reader
  followed the spellings it knew and passed others in silence — a doc test fenced another way, a raw-identifier
  module, a test-only attribute on a field. It is to account for every word that can make the compiler read a
  file, refusing whatever it cannot read whole, and to read documentation with no markdown at all; the oracle is
  to run rustdoc too.

## archogen — the trust record says what the census now does

`ARCHOGEN-M3-0922` (leaf `M3.5.2.7.4`), `2026-10-23`.

- The trust records now list the census among what the gate reads, cite the census as it stands, and quote the
  Rust reference they rest on; the gate's report words its outside list as the record does.

## archogen — the trust gate's census is held to the compiler

`ARCHOGEN-M3-0921` (leaf `M3.5.2.7.3`), `2026-10-23`.

- A test now builds a corpus of every case the trust record's reviewers raised, in twelve build configurations, and
  checks that every file the compiler read is one the census counts. Its first run found a real gap — a module
  whose file depends on a `cfg_attr` — now closed.

## archogen — the trust gate's census builds nothing

`ARCHOGEN-M3-0920` (leaf `M3.5.2.7.2`), `2026-10-23`.

- What each test compiles is now read from its sources and cargo's metadata alone, whatever the profile, features,
  flags or host, so no configuration hides a file. Test-only code of a library counts for its own tests, not for
  what links it; a non-member's tests are not counted; and a comparison program is held to what its test reaches.
  On this tree it gives the same answers as before, and takes no build.

## archogen — the trust gate reads every source's inclusions

`ARCHOGEN-M3-0919` (leaf `M3.5.2.7.1`), `2026-10-23`.

- The trust gate now reads every file a build target reaches through `mod`, `#[path]` or an `include!`, whatever the
  build configuration, doc tests included, and refuses an inclusion whose file its text does not name — a computed
  path, an alias, a macro's — unless an admission reviews it. The tree's one such site, in `rt-analysis`'s tests,
  is admitted after the next push.

## archogen — the checkers' programs in the trust record, round 10, and a third change of method

`ARCHOGEN-M3-0918` (leaf `M3.5.2`), `2026-10-23`.

- The tenth review of the trust record found 7 defects and 8 remarks, up a third round: each round found another
  build configuration the census did not try. The census is to stop depending on any: what a test compiles will be
  read from cargo's metadata and from every source's inclusion sites, read as text with nothing built, and an
  oracle will build a corpus of every probe so far in many configurations to check it misses nothing.

## archogen — the trust gate pairs a comparison with roots alone

`ARCHOGEN-M3-0917` (leaf `M3.5.2.6.4`), `2026-10-23`.

- The trust gate now pairs a comparison program with roots alone, as its record says, never with another comparison,
  whose shared role packages are shared by design. A run the gate cannot judge past while judging generators names
  the refusals already found, and the report's wording is tightened.

## archogen — what a test compiles is a table the trust gate is held to

`ARCHOGEN-M3-0916` (leaf `M3.5.2.6.3`), `2026-10-23`.

- Everything that decides what a test compiles — which targets are built, the profile, features, platform, files a
  source includes, build scripts — is now one table in the trust record, each rule quoted from cargo's own
  documentation. A test checks that every row is held by a real fixture and that whatever the census does not see
  is said in the gate's report. The report also names the packages the census leaves out.

## archogen — the trust gate's census builds tests as `cargo test` does

`ARCHOGEN-M3-0915` (leaf `M3.5.2.6.2`), `2026-10-23`.

- The census of what each test compiles now builds in `cargo test`'s own profile and features, and builds by name
  the tests and benches `cargo test` skips unless asked, so code under `debug_assertions` or a required feature
  is seen. A test that does not build stops the run instead of being judged silently.

## archogen — a file belongs to the package that holds it

`ARCHOGEN-M3-0914` (leaf `M3.5.2.6.1`), `2026-10-23`.

- The trust gate no longer counts a file inside a package nested in a role's directory as that role's: a test or a
  program depending on the nested package joins and runs nothing of the role around it.

