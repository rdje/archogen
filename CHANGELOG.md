# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

## archogen — the review-count check holds exactly what it says, and names what it skips (withdrawn: round 26)

`ARCHOGEN-PROGRAM-0586` (leaf `PROGRAM.69`), `2026-10-10`.

- A review found the new count check still claiming more than it held: some reviews it never read, an index status in
  free prose it read loosely, a duplicated row it let pass. It no longer reads the prose status; it refuses a
  duplicated row or round; and its output names every review it judges nothing of, with the reason (narrowed by
  review round 26: the reviews its script lists, each refused if it cannot be judged). Every document that describes
  it now points to the script's own header for its scope (not so, review round 26 found: four kept their own claim).
- Validation: 20 arms; six deliberate breaks, each caught by its own arm, by an untracked runner, so not durable; the
  sealing tool's 121 arms; every self-test; the focused tier on the staged tree.

## archogen — the leaf for the parent's tests restates nothing of its evidence

`ARCHOGEN-M3-0585` (leaf `M3.6.6.4`), `2026-10-10`.

- One sentence of the leaf still said more of the review history than it holds: that each finding there comes with
  the mutation that showed it. It now says only that the history records each as the round that found it states it.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — the sealed cases' guard reads the disk itself, and says what it holds against

`ARCHOGEN-PROGRAM-0584` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found the guard still trusting git's view of what changed on disk, which local settings can blind — a
  file-system monitor, the laxest stat check, a clean filter — and a template that marked its scratch repository. The
  guard now forces the settings it relies on, reads the working tree's bytes itself in one walk, and refuses if that
  walk cannot finish (corrected by review round 11). Its documentation
  now says what it holds against: accident, not someone setting out to hide a case.
- Validation: 92 arms; eight deliberate breaks covering its nine holes, two of them sharing one, each caught by its own
  arms, by an untracked runner, so not durable; the check on the tree in about a second; every self-test; the focused tier on the staged tree.

## archogen — the leaf for the parent's tests points to its evidence instead of restating it

`ARCHOGEN-M3-0583` (leaf `M3.6.6.4`), `2026-10-10`.

- Each round found a detail wrong in the leaf's summary of what earlier criteria missed. The leaf now restates none
  of it: it points to the review history, where each finding and its evidence already stand, and designs its
  criterion to meet them.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — the review-count check reads the reviews index too, and says what it does not read (not so: round 25)

`ARCHOGEN-PROGRAM-0582` (leaf `PROGRAM.69`), `2026-10-10`.

- The new count check now also holds the reviews index's row for each open review (narrowed by review round 26: the
  reviews its script lists, each refused if it cannot be judged) — how many rounds it says, and what it says the last
  round found — to the review history's tables. It finds rounds by their own lines, so a summary row for a round the
  history never had is refused. Records whose reviews closed and whose histories are archived are not read, and the
  check now names them rather than passing them silently (not so, review round 25 found: eight passed unnamed).
- Validation: 18 arms; eight deliberate breaks, each caught by its own arm, by an untracked runner, so not durable;
  the sealing tool's 121 arms; every self-test; the focused tier on the staged tree.

## archogen — the leaf for the parent's tests designs its own criterion

`ARCHOGEN-M3-0581` (leaf `M3.6.6.4`), `2026-10-10`.

- Round 18 made the project's mutation sweep the measure of where the parent's rules are tested. Round 19 found the
  sweep cannot see the defects the earlier rounds found — it never narrows a loop to one element — and cannot yet run
  the trust tests at all. The generated-sources record now prescribes no criterion: the leaf for the parent's tests
  states the problem and what any criterion must meet, and designs and reviews its own.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — the leaf for the parent's tests is judged by a mutation sweep, not a list

`ARCHOGEN-M3-0580` (leaf `M3.6.6.4`), `2026-10-10`.

- Each description of where the parent's rules must be tested — a list of axes, then a census of the code's loops —
  was found leaving one out. The leaf now names no axis: it is done when the project's mutation sweep over the code
  that judges generated sources finds every mutant killed by a test, or listed as equivalent with its reason. The
  record's tested section names none either.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — five ways past the sealed cases' guard closed

`ARCHOGEN-PROGRAM-0579` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found five ways past the check that guards the sealed cases: a case's text in a folder or a file named
  `.git`, which git never lists, or in a tracked file git was told not to look at; the diff-hiding mark set where no
  commit carries it; and a case's name in a staged file whose copy on disk was clean, or in the check's own script,
  which the name search skipped (corrected by review round 10). Each is now refused, with a test that fails if the hole is put back. The evaluation
  README now lists every copy that can still show a case (not so: review rounds 10 and 11 found it short), and GitHub's own record of pushes bounds the four days the
  published branch held them.
- Validation: 81 arms; six deliberate breaks, one per hole, each caught by its own arm, by an untracked runner, so not
  durable; the check on the tree in about a second; every self-test; the focused tier on the staged tree.

## archogen — the leaf for the parent's tests derives its axes instead of listing them

`ARCHOGEN-M3-0578` (leaf `M3.6.6.4`), `2026-10-10`.

- Three rounds in a row found the list of axes in the leaf filed for the parent's own tests leaving one out. Its goal
  now derives them: a census of every loop in the code that a refusal or a judgment sits in gives each rule its
  axes. The note on the leaf's log names its one exception — its first changelog entry, sealed in a segment after two
  early rounds edited it.
- Validation: the sealed entry against its commit; sixteen route tests; the crate's 183 tests; the book's gates; the
  focused tier on the staged tree.

## archogen — a review round's count is checked against its findings, and a seal's rollback names what it leaves

`ARCHOGEN-PROGRAM-0576` (leaf `PROGRAM.69`), `2026-10-10`.

- A design record states how many findings each review round made, and the task-tree sealing record had that count
  wrong for the fourth time. A new check, `REVIEW-COUNTS`, now counts each round's findings in a review history still
  open under `docs/reviews/` (narrowed by review round 26: the reviews its script lists, each refused if it cannot be
  judged) and refuses a record whose count, defects, arm gaps or remarks disagree; on its first run it named exactly
  the wrong row.
- When a seal stops because another writer put a file where the seal was writing, the folder it made for that file
  stays, and the tool now names it and no longer says the seal was rolled back. A test now holds that every leaf with
  a line that would tear is named, in every part of the tree, not only the first.
- Validation: 121 arms of the sealing tool and 9 of the new check; two deliberate breaks each caught by its own arm,
  by an untracked runner, so not durable; the focused tier on the staged tree.

## archogen — the generated-sources record's ledgers say what each commit claimed, and the record what holds

`ARCHOGEN-M3-0575` (leaf `M3.6.6.4`), `2026-10-10`.

- Three review rounds in turn found notes added to old log rows stopping short of what later rounds had found. Every
  log row and changelog entry of the leaf is back to the text its own commit wrote, and one note above the log says
  so: each states a commit's claim when made, the review history narrows it, and the record's tested section is what
  holds now. The leaf filed for the parent's tests now gives each rule only the axes it has, and adds how a program
  reads a file — compiled, handed to it, or through the comparison harness.
- Validation: every restored row and entry compared with its commit's own text; sixteen route tests; the crate's 183
  tests; the book's gates; the focused tier on the staged tree.

## archogen — the trust gate has its baseline, measured on the CI runner

`ARCHOGEN-M3-0574` (leaf `M3.6.3.2.1`), `2026-10-10`.

- The gate compares what two programs share against a baseline measured on one machine, the CI runner, since its
  digests hold only there. The runner's first judging run printed its proposal — a form for each of the nine things
  two programs share, none of them accepted — and it is now committed. On this machine the gate passes and says it did
  not compare, as it should off the runner; that the runner reads "unchanged" for the next commit is the next push's
  run to show.
- Validation: the gate on the staged tree, against its parent and against itself; every test, with the baseline
  recorded in the verdict table and the third reader's record; the focused tier on the staged tree.

## archogen — CI is green on the runner again

`ARCHOGEN-PROGRAM-0573` (leaf `PROGRAM.77`), `2026-10-10`.

- The push carrying the fix ran every workflow green: the test suite on the runner in about two minutes, the
  integration tier in four, where both jobs had been lost after 45. That run is the check this machine could not
  make, and it confirms the cause found in the `kill` program's source.
- Validation: the runs of `8cdac11` — `rust`, `doctrines`, `trust-gate` — each a success.

## archogen — the sealed cases' limits name every copy that can still show one

`ARCHOGEN-PROGRAM-0572` (leaf `PROGRAM.76`), `2026-10-10`.

- A review found the evaluation README still naming two ways to read a sealed case where its honest limit names
  more, and two of the check's rules with no test. The README now points at every copy the limit names, and the limit
  names three more: a harness's memory written before the cases left the tree, a model trained or an index built on
  the published branch in the four days it held them, and a short quote committed elsewhere before the check
  existed — while no other path in history ever held a case's whole text: only the commit that sealed each added it,
  and only the one that took it out removed it (corrected by review round 9). Two new tests hold a case's text in a modified file whose name git would quote, and the diff-hiding mark
  dropped on disk alone.
- Validation: 71 arms; each new arm the one failing when its leg is read the old way, by an untracked runner, so
  not durable.

## archogen — the generated-sources record's log says what each round's answer held

`ARCHOGEN-M3-0571` (leaf `M3.6.6.4`), `2026-10-10`.

- The leaf's Commit Log kept the claims each round's commit made when it was written — every refusal at every
  position, every reader — though later rounds found each narrower. Every row a later round narrowed now says so
  beside it. The leaf filed for the parent's own tests now names each clause, generators and inputs, and two more
  of the parent's rules: a file named in both clauses, and a lone Rust file.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — the trust gate's first run in CI had no gate to judge with

`ARCHOGEN-M3-0570` (leaf `M3.6.3.2.1`), `2026-10-10`.

- The gate in CI is built from the commit before the push, so a change cannot weaken the gate that judges it. The
  first push carrying the workflow had a base older than the gate itself, and the run stopped at "unrecognized
  arguments". Every later push has a base that holds the gate, so this happens once; the next push's run judges,
  and its proposal is the runner's baseline the leaf waits on. Run here on the next push's range, the gate passes.
- Validation: the run's log; the gate on the next push's range; the book's gates.

## archogen — the test suite no longer ends the CI runner it runs on

`ARCHOGEN-PROGRAM-0569` (leaf `PROGRAM.77`), `2026-10-10`.

- The push `a543d10` lost both jobs of its `rust` run: after about 45 minutes the hosted runner "lost communication",
  and no log was kept. The mutation runner's test killed a hung command's process group with the `kill` program,
  `kill -KILL -<group>`. Ubuntu's `kill`, procps-ng's, reads that negative number as an option and, by a special case
  in its source, signals pid `-1` — every process the user may signal, the runner's own agent among them. macOS's
  reads it as a pid, so the test passed here. The group is now signalled through the system call itself, `0` and
  `-1` refused before it, and a test refuses any Rust source that runs the `kill` program.
- Validation: the module's 12 tests; four catalogued mutations, each killed; the crate's 183 tests; the focused tier
  on the staged tree. Not reproduced on Linux, which no host here is: the push that restores CI is the run that
  checks it.

## archogen — the generated-sources record says which of its rules are the parent's

`ARCHOGEN-M3-0568` (leaf `M3.6.6.4`), `2026-10-10`.

- Review rounds 9 to 14 each found one more axis — where an entry stands, the order of readers, a third reader, the
  order of forms, the comparison harness as a reader — on which a rule the record restated from its parent was tested
  one way only. The record now says those rules are the parent's, decided by its record and held by its tests, and
  states as its own only the routes its tests build; how broadly the parent's tests hold its rules across every axis is
  a leaf of its own, `M3.6.9`. Two earlier texts that claimed more than their tests held are narrowed.
- Validation: sixteen route tests; the crate's 181 tests; the book's gates; the focused tier on the staged tree.

## archogen — the generated-sources routes' tests read every reader in every order

`ARCHOGEN-M3-0566` (leaf `M3.6.6.4`), `2026-10-10`.

- Rounds 12 and 13 found more rules held at some positions only: a middle reader, a middle input, a root's own
  program, a marked file a built generator compiles, and the third reader never asserted. The reader tests now run all
  six orders of three readers, asserting every one the rule refuses; the sweep holds fifteen kinds of refused entry;
  the record's §6 says exactly the positions and counts each test holds.
- Validation: sixteen route tests; the crate's 181 tests; each mutation failing a test, by an untracked runner, so not
  durable.

## archogen — the published branch no longer shows the sealed cases

`ARCHOGEN-PROGRAM-0565` (leaf `PROGRAM.76`), `2026-10-10`.

- The repository's published branch was pushed past the change that took the sealed evaluation cases out of the tree,
  on the director's word, as the one exception to the push cadence: a fresh clone, the branch's web view and its code
  search no longer show a case. Copies made of the branch before then — forks, mirrors, search caches, archives — may
  still, and web searches of the repository stay fenced. The texts now bound the risky revisions exactly, and the seal's
  check names a non-ASCII path as it is in every leg, held by arms that force git to quote such names.
- Validation: 69 arms; four deliberate breaks each caught, by an untracked runner, so not durable.

