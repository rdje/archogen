# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — one correction in the generated-sources review history set right

`ARCHOGEN-M3-0587` (leaf `M3.6.6.4`), `2026-10-10`.

- The review history's answer to a remark of round 21 misattributed which rounds had found what; round 22's entry
  corrects it, round 21's standing as written. One sentence of the record now carries the qualifier the rest of it
  uses: the parent's tests hold its rules at the instances they build.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — the review-count check holds exactly what it says, and names what it skips

`ARCHOGEN-PROGRAM-0586` (leaf `PROGRAM.69`), `2026-10-10`.

- A review found the new count check still claiming more than it held: some reviews it never read, an index status in
  free prose it read loosely, a duplicated row it let pass. It no longer reads the prose status; it refuses a
  duplicated row or round; and its output names every review it judges nothing of, with the reason. Every document
  that describes it now points to the script's own header for its scope.
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
  file-system monitor, the laxest stat check, a clean filter, a template. The guard now forces the settings it relies
  on, reads the working tree's bytes itself in one walk, and refuses if that walk cannot finish. Its documentation
  now says what it holds against: accident, not someone setting out to hide a case.
- Validation: 92 arms; eight deliberate breaks, one per hole, each caught by its own arms, by an untracked runner, so
  not durable; the check on the tree in about a second; every self-test; the focused tier on the staged tree.

## archogen — the leaf for the parent's tests points to its evidence instead of restating it

`ARCHOGEN-M3-0583` (leaf `M3.6.6.4`), `2026-10-10`.

- Each round found a detail wrong in the leaf's summary of what earlier criteria missed. The leaf now restates none
  of it: it points to the review history, where each finding and its evidence already stand, and designs its
  criterion to meet them.
- Validation: sixteen route tests; the crate's 183 tests; the book's gates; the focused tier on the staged tree.

## archogen — the review-count check reads the reviews index too, and says what it does not read

`ARCHOGEN-PROGRAM-0582` (leaf `PROGRAM.69`), `2026-10-10`.

- The new count check now also holds the reviews index's row for each open review — how many rounds it says, and
  what it says the last round found — to the review history's tables. It finds rounds by their own lines, so a
  summary row for a round the history never had is refused. Records whose reviews closed and whose histories are
  archived are not read, and the check now names them rather than passing them silently.
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
  README now lists every copy that can still show a case, and GitHub's own record of pushes bounds the four days the
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
  open under `docs/reviews/` and refuses a record whose count, defects, arm gaps or remarks disagree; on its first run
  it named exactly the wrong row.
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

