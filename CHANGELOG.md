# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

