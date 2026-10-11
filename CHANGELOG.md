# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — a staged read failure cannot silently substitute disk evidence

`ARCHOGEN-PROGRAM-0612` (leaf `PROGRAM.69`), `2026-10-11`.

- REVIEW-COUNTS falls back to disk only after a successful index census and path-byte comparison prove absence. A present
  unreadable staged entry or failed proof refuses with a reason; the shared reader covers index, history and record.
- Validation: ten new controls, including inherited pathspec settings and complementary present/absent bracketed
  filenames: 83 pass / 8 fail on the old reader, then 91 / 0; a glob-comparison mutation fails both filename
  controls. Staged count/history gates, book checks, stated order and task acceptance pass; final doctrines run in
  the commit hook. Record, history, book and resume pointers agree on round 32's answer and round 33 next.

## archogen — the doctrine registry's explanatory header stays literal

`ARCHOGEN-PROGRAM-0611` (leaf `PROGRAM.82`), `2026-10-11`.

- Replaced backticks and table pipes in the REVIEW-COUNTS description with words. Bash formerly tried to execute the
  header while initializing the registry, emitting a syntax error and dropping the example; parse-only validation
  passed. Check commands and verdict rules are unchanged, as are the held doctrine driver and hook.
- Validation: the exact array's initialization before/after gives diagnostics then empty stderr; all 29 entries retain
  three fields and the repaired description is complete. Shell syntax and the held-file diff pass; final doctrines
  run in the commit hook. Book, task logs and resume/frontier pointers agree.

## archogen — seventh daily artifact cleanup, with cold-cache verification

`ARCHOGEN-PROGRAM-0610` (leaf `PROGRAM.81`), `2026-10-11`.

- Removed only the main checkout's hosted and wasm incremental caches: 2 022 861 425 logical bytes in 18 540 files,
  including 1 245 `.bin`. Cited review evidence, nested read-only Git workspaces, tools and dependency products stay.
  The latest-only record and book explain the cleanup boundary and verification.
- Validation: OS census clear before removal; both exact paths gone afterward; focused tier 3 passed / 0 failed and
  wasm build exit 0 with those caches absent; book gates pass. Dependencies were retained, so this is cold-cache
  verification rather than a full dependency rebuild; the final staged tree must have its focused stamp at commit.

## archogen — sealed source lines recognize LF, CRLF and CR before eligibility

`ARCHOGEN-PROGRAM-0609` (leaf `PROGRAM.76`), `2026-10-11`.

- The shared reader now splits source cases at LF, CRLF or CR before measuring each complete line. A CR-only
  case formerly became one case-sized pattern, missing a quote of a single eligible line on disk and in the index.
  The original line thresholds, case-sensitive word comparison and partial-quote limits remain unchanged.
- Validation: six new source-ending controls; the two CR-only regressions failed first (141/2 of 143 arms).
  All 143 repaired arms pass; the filesystem case-distinction arm is explicitly skipped on this host. The real
  sealed-tree guard, syntax, review counts, book checks and doctrines pass.

## archogen — an unlisted review names its design record by the first link

`ARCHOGEN-PROGRAM-0608` (leaf `PROGRAM.69`), `2026-10-11`.

- The open-review list rule now uses the same first-link helper for the Design record cell as for the history
  cell. Secondary explanatory links no longer falsely require counting or produce missing-record refusals. The
  primary record's round table still requires listing; counted pairs remain their own declared authority.
- Validation: two new regression arms failed first, then all 81 count-check arms passed; shell syntax, staged real
  gates, book checks and doctrines. The prior first-link claim is marked; book and resume pointers agree.

## archogen — handoff refuses failed or missing OS census reads

`ARCHOGEN-PROGRAM-0607` (leaf `PROGRAM.80`), `2026-10-11`.

- The project handoff wrapper observes the native census's actual process snapshot and open-handle reads, in each
  sample. Denied, failed or missing reads, and an empty process snapshot, exit 2 with an unknown outcome and no
  OK verdict. A process that exits during an ancestry lookup is allowed; persistent-job detection is preserved.
- The native scaffold file is unchanged. The bootstrap, toolbox and book use the guarded wrapper; the book states
  the sampling limits. Private receipts stay on the repository volume and are removed on exit.
- Validation: six regression arms failed against the old core; all nine arms pass after repair. A real restricted
  run refuses, a real OS-access run passes; zero receipt directories remain; syntax and book gates pass. No Rust
  source changed. The next review answer, PROGRAM.69 round 31, is owned in the task tree.

## archogen — escaped summary-cell pipes cannot hide later counts

`ARCHOGEN-PROGRAM-0606` (leaf `PROGRAM.69`), `2026-10-11`.

- One cell reader now serves index, summary and findings tables. An escaped pipe stays inside a summary's defects
  cell, so every later stated count remains visible. Eight false-count arms failed first; all 78 arms now pass.
- The previous commit's confirmatory runner is complete: 47 of 49 suites passed restricted; the two process suites
  passed with OS access through the same runner (2/2 handoff and 39/39 spine arms). Those two runs cover all 49
  suites before this parser repair; the parser's focused 78-arm run is current. No Rust source changed.
- The denied OS reads exposed a separate pre-existing defect: the native handoff census and its wrapper say OK
  without a process snapshot. Owned as high-priority `PROGRAM.80`, the wrapper repair next, the held native file
  untouched. Earlier repeated-count claims are marked; the book and resume pointers agree.

## archogen — review counts judge repeated statements, and history reads must prove absence (narrowed by round 30)

`ARCHOGEN-PROGRAM-0605` (leaf `PROGRAM.69`), `2026-10-11`.

- The reviews index's four-column table accepts indentation and optional edge pipes, keeps escaped pipes inside
  their cell, and refuses malformed rows, missing or duplicate headers and missing separators. Every stated count
  in either summary shape is judged, so a correct first count cannot hide a false second one.
- A historical task-history index blob that cannot be read is refused unless a separate successful Git tree read
  proves that path absent at the commit. A real historical deletion remains valid. The leaf's stale Status is current.
- Validation: new regressions first failed (13 count-check arms, two history-check arms), then all 69 and 124 arms
  passed; syntax, real gates, census, book gates and doctrines green. The broader self-test runner was pending at
  commit; its result is recorded by `ARCHOGEN-PROGRAM-0606`. The earlier claims are marked in the book and records.
- Correction by round 30: an escaped pipe in a summary cell still hid later stated counts; the shared cell reader
  in `ARCHOGEN-PROGRAM-0606` repairs that case.

## archogen — the sealed-set guard recognizes comment and string wraps, with an explicit quote boundary

`ARCHOGEN-PROGRAM-0604` (leaf `PROGRAM.76`), `2026-10-11`.

- Whole cases and complete long source lines are matched as case-sensitive words after prose markers and word-edge
  punctuation are removed. Doc comments, Lisp comments, no-space quote markers, continued strings and adjacent
  string literals no longer hide those patterns, on disk or in the index.
- A partial quote across source lines can remain unfound even when longer than either line; encodings that change
  word boundaries and rewordings are also stated limits. The draft's arbitrary word windows are not adopted: an
  overlap alone cannot establish which document quoted which. No case text was displayed or exposure inferred.
- The error paths have separate arms for the disk and index searches; recursion arms put a synthetic case's name
  inside an active registered submodule. The top-level `.git` case distinction has a conditional arm, explicitly
  skipped on a case-insensitive filesystem.
- Validation: new arms on the previous reader, 127 passed / 10 failed; revised reader, 137 passed / 0 failed; real-tree
  check; shell syntax; book anchors and glossary; review counts; doctrine gate. No Rust source changed.

## archogen — the sealed cases' guard: a review's findings and the answer's direction, recorded at a hand-off

`ARCHOGEN-PROGRAM-0603` (leaf `PROGRAM.76`), `2026-10-11`.

- A review found the guard missing a quote wrapped in some comment and string styles, and a quote running across two
  of a case's lines. The answer under way compares words, not lines; its first run found three tracked documents
  sharing a run of eight words or more with a case, which the next session settles before the guard changes. The
  guard itself is unchanged.
- Validation: the guard on the tree as committed; the no-background check; the focused tier on the staged tree.

## archogen — the review-count check reads index rows and says which reviews must be listed (narrowed by round 29)

`ARCHOGEN-PROGRAM-0602` (leaf `PROGRAM.69`), `2026-10-11`.

- A review found the count check's rule that open reviews be listed passing a review whose index row was written a
  little differently, and the texts stating the rule wider than it was. The check now reads every row of the index by
  its cells and links, and refuses one it cannot read; every open review whose record keeps a table of its rounds, of
  any shape, must be listed from its first round, and a closed one must leave the list. A round's findings table must
  be the first table after its line, with no row of dashes or row with no kind, and every count a summary row states
  is judged, whichever shape the record keeps.
- Validation: 51 arms; thirteen deliberate breaks of the new rules and round 27's eleven, each caught by its own arm,
  by untracked runners, so not durable; the sealing tool's 121 arms; every self-test; the focused tier on the staged
  tree.

- Correction by review round 29: indented and malformed index rows were skipped, and only the first stated count
  of a kind was judged. `ARCHOGEN-PROGRAM-0605` repairs both.

## archogen — the design for the generated-sources rules' tests holds every recorded defect exactly

`ARCHOGEN-M3-0601` (leaf `M3.6.9`), `2026-10-11`.

- The design's first review found its new kinds of mutation reaching 7 of the 23 one-way defects on record, and its
  sweep's copy placed outside the repository by an exception only the director grants. Now each defect on record is
  itself a catalogue entry the rules' own tests must catch, and the sweep, with more kinds of mutation, carries the
  check past the record — how far, measured defect by defect when it is built. The copy stays inside the repository:
  the tests admit, in the tests alone, the configuration of the repository the copy was made from. The tests, the
  code swept and the commands are named in full.
- Validation: the parent's tests timed, 88 in 12.5 s; the record's, the reviews' and the book's gates; the focused
  tier on the staged tree.

## archogen — the accepted designs' folder has room again

`ARCHOGEN-PROGRAM-0600` (leaf `PROGRAM.79`), `2026-10-11`.

- The folder of accepted designs had under 4 KB left, and the answer to the open adequacy design's first review needs
  7 KB. Three trust designs have moved in since its last raise five days ago. Its ceiling rises from 384 to 512 KiB,
  measured and recorded in the decision that governs it; compacting it was weighed and refused, since nothing there is
  history.
- Validation: the routes check and its self-test; the folder measured at this commit's parent and at the last raise;
  the focused tier on the staged tree.

## archogen — the reviews the count check judges are listed in the reviews index

`ARCHOGEN-PROGRAM-0599` (leaf `PROGRAM.69`), `2026-10-11`.

- The count check's list of reviews was written in its script, so a design review in any other part of the project
  could join it only by a change to that script, owned by another leaf. The list now sits in the reviews index, which
  every review's answer already edits, under its own heading; a missing heading, a line there that is no entry, and a
  review listed twice stop the commit.
- Validation: 38 arms; four deliberate breaks of the list's rules, each caught by its own arm, and round 27's twelve,
  by untracked runners, so not durable; every self-test; the focused tier on the staged tree.

## archogen — the review-count check judges every round of the reviews it lists, and every open one is listed (not so: round 28)

`ARCHOGEN-PROGRAM-0598` (leaf `PROGRAM.69`), `2026-10-11`.

- A review found the count check still passing a review it could not judge: a round whose findings table had another
  header was counted as having none, and its counts went unread. The check is now strict: every round of a listed
  review must keep a findings table in the one form, or the commit stops; a malformed table — no separator, a row with
  no id, a row of dashes — stops it too (not so, review round 28 found: a row of dashes in a findings table was
  counted); and an open review whose record keeps a count per round must be on the list from its first round (not so,
  review round 28 found: one whose index row's link had no backticks was passed). The five texts the review found
  still claiming what earlier reviews refuted, and three more like them found while answering, now say so.
- Validation: 35 arms; twelve deliberate breaks, each caught by its own arm, by an untracked runner, so not durable;
  the sealing tool's 121 arms; every self-test; the focused tier on the staged tree.

## archogen — a design for holding the generated-sources rules to their tests, by mutation

`ARCHOGEN-M3-0597` (leaf `M3.6.9`), `2026-10-10`.

- The generated-sources rules are held by the tests of their instrument only at the instances those tests build, and
  every attempt to name where else they must be held left a place out. The design decided here names none: every
  mutant the project's mutation sweep makes of the judgment code must be caught by those tests or shown harmless. It
  adds the three kinds of mutation the defects found so far were (not so, review round 1 found: they made 7 of the
  23), and runs the sweep's copy beside the repository, where the trust gate's own rules let its tests run (withdrawn
  by review round 1: the copy stays under `target/`). Its review is open.
- Validation: the record's and the book's gates; the focused tier on the staged tree.

## archogen — the sealed cases' guard reads every file as prose, staged or on disk

`ARCHOGEN-PROGRAM-0596` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found a quoted line of a case missed when the quote was re-wrapped across two lines — as every document
  here is wrapped — and a case missed inside a larger file, or with its line endings changed. The guard now reads
  every file it walks and every staged one itself, as prose: line breaks, indents and list or comment markers set
  aside. It finds a case's whole text, and a long line of one, however wrapped. On macOS it no longer takes a file
  as ignored by a case-blind match that Linux would not make.
- Validation: the self-test's arms; five deliberate breaks, each caught by its own arms, by an untracked runner, so
  not durable; the check on the tree in about two seconds; every self-test; the focused tier on the staged tree.

## archogen — the generated-sources rules' one-way defects are in the mutation catalogue

`ARCHOGEN-M3-0594` (leaf `M3.6.8`), `2026-10-10`.

- A review of the generated-sources record found, round after round, rules held only one way — the first generator
  judged and not the last, one reader and not another — each shown by a throwaway script. The 23 such mutations are
  now entries of the project's mutation catalogue, each run by the harness and each caught by the route tests, so the
  record of them outlives the scripts.
- Validation: the 23 entries through the harness, each killed; the catalogue's own tests; the focused tier on the
  staged tree.

## archogen — the generated-sources record's review closes, and generated sources are done

`ARCHOGEN-M3-0591` (leaf `M3.6.6.4`), `2026-10-10`.

- The record deciding which generated-source shapes stay refused, and the routes around them, passed its twenty-third
  review round with no defect. With it, the work on committed generated sources — the design, the instrument, the
  chapter and these decisions — is done. The leaf for the parent's own tests stays open, with its evidence in the
  review history.
- Validation: sixteen route tests; the crate's 183 tests; the review-count check; the book's gates; the focused tier
  on the staged tree.

## archogen — the review-count check judges a list of reviews, and refuses any it cannot judge (not so: round 27)

`ARCHOGEN-PROGRAM-0590` (leaf `PROGRAM.69`), `2026-10-10`.

- Three reviews in a row found the count check deriving which reviews to read and leaving one unread or unnamed. It
  no longer derives anything: its script lists the reviews it judges — the open ones whose records keep a count per
  round — and any listed review it cannot judge, a file missing or a table empty, stops the commit (not so, review
  round 27 found: a round whose table had another header was passed unjudged). A review leaves the list when it
  closes. The texts that said more are marked as narrowed (not so, review round 27 found: five were not).
- Validation: 25 arms; eight deliberate breaks, each caught by its own arm, by an untracked runner, so not durable;
  the sealing tool's 121 arms; every self-test; the focused tier on the staged tree.

## archogen — the sealed cases' guard finds quotes in what it reads, and decides alone what is ignored

`ARCHOGEN-PROGRAM-0588` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found two more settings that changed what the guard saw: one switched off its search of the disk without a
  word, and personal ignore files hid paths from it. The guard's own walk of the files now finds a case's long line
  as well as a whole copy, so no setting can switch that off; what is ignored is decided by the project's own ignore
  files alone; and any search that fails stops the commit. Its documents now say exactly what it finds — a whole copy,
  a long line, a name — in exactly which files (narrowed by review round 12: a name only in a tracked file).
- Validation: the self-test's arms; nine deliberate breaks, one per hole, each caught by its own arms, by an untracked
  runner, so not durable; the check on the tree in under two seconds; every self-test; the focused tier on the staged
  tree.

## archogen — one correction in the generated-sources review history set right

`ARCHOGEN-M3-0587` (leaf `M3.6.6.4`), `2026-10-10`.

- The review history's answer to a remark of round 21 misattributed which rounds had found what; round 22's entry
  corrects it, round 21's standing as written. One sentence of the record now carries the qualifier the rest of it
  uses: the parent's tests hold its rules at the instances they build.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

