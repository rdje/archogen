# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the trust report shows every change its design lists

`ARCHOGEN-M3-0493` (leaf `M3.6.3.7`).

- The trust check's report of what a change does now also says when the list of checked programs is edited — a
  program added, removed or redefined — and when a program is unaccounted for, as its design always said. Before, a
  change that edited that list showed nothing, which a design review found.

## archogen — the book explains the trust gate

`ARCHOGEN-M3-0491` (leaf `M3.6.4`).

- The book has a chapter on why archogen's checking programs must not share a mistake with the generator they check:
  plain words and an everyday comparison first, then how the record of what each program is built from is made and
  compared, the reports and refusals, and where the check runs — and what it cannot prove.

## archogen — a commit can no longer leave a new source file behind

`ARCHOGEN-PROGRAM-0490` (leaf `PROGRAM.66`).

- One earlier commit recorded code that referred to a new file without including the file, so that commit could not
  be built, although every check had passed on the working copy. A new check now refuses to commit while a source
  file sits beside the commit untracked.

## archogen — the local stand-in for CI passes as a whole again

`ARCHOGEN-PROGRAM-0488` (leaf `PROGRAM.65`).

- The script that imitates a CI run now works in a folder next to the repository rather than inside it, as the
  director ruled, so nothing of the repository's own setup leaks into the imitation. It passes on the current commit
  and, on the commit GitHub first failed on, finds exactly GitHub's failures. The repository also has an ignored
  `.archogen-data/` folder for its own data.

## archogen — the trust gate's five required cases are tests

`ARCHOGEN-M3-0487` (leaf `M3.6.3.6`).

- The five situations the roadmap says the trust check must handle — new sharing, a shared part changed behind an
  unchanged name, shared data, an out-of-date record in a package, and an unrelated change that must raise no warning —
  are now automated tests run against small throwaway projects. Writing them found a gap: a changed shared part merged
  without its reviewed entry updated would have been reported by every later change; such a change is now refused
  until its entry is re-proposed. Every refusal the check can make is shown to be caught by a deliberate breakage.

## archogen — a package's trust record can be checked where it is used

`ARCHOGEN-M3-0485` (leaf `M3.6.3.5`).

- A new command checks an assurance package: that the record of what each independent program was built from
  belongs to the same commit and compiler as the package, that the programs shipped are the ones recorded, that each
  result was produced by the program its role names, and that every file handed to a program at run time was
  declared. The package's exact layout is set provisionally, for the later work that will write real packages.

## archogen — the trust gate runs in CI, built from the commit before

`ARCHOGEN-M3-0484` (leaf `M3.6.3.4`).

- The trust check now has its own CI job, run on every pull request and every push to `main`. The check is built from
  the commit being compared against, never from the change itself, so a change cannot weaken the check that judges
  it; the job also prints the reviewed-list entries proposed on GitHub's machine, which is what the list must be
  measured on. The release checks run it too, and never count it as passed until entries can be approved.

## archogen — the trust gate judges a commit

`ARCHOGEN-M3-0483` (leaf `M3.6.3.3`).

- A new command compares what the independent programs share now with what the previous commit's reviewed list
  holds, and writes a two-part report: what changed, and everything still awaiting review. On the machine the list was
  measured on, it refuses a commit that shares something new without proposing an entry for it, or that keeps an entry
  for something gone; elsewhere it says plainly that it could not compare. It is not yet run by CI.

## archogen — the local stand-in for CI no longer borrows the developer's identity

`ARCHOGEN-PROGRAM-0482` (leaf `PROGRAM.64`).

- The script that imitates a CI run on this machine now gives the run a home directory of its own, so tools that
  discard most of their environment no longer pick up the developer's git identity; on the commit CI first failed on,
  it now finds the same six failures GitHub did. A second gap surfaced: the imitation runs inside this repository, under
  a configuration the trust checks rightly refuse, so it cannot pass as a whole until the director decides where it may
  run instead.

## archogen — the checks pass on GitHub's machines

`ARCHOGEN-PROGRAM-0481` (leaf `PROGRAM.10.5`).

- The project's checks ran on GitHub's machines for the first time since September and failed on two faults in
  tests, both now fixed; the second run passed everything, the full integration checks included. The emulator, the
  book and every documentation gate behave there as they do here, so the CI work begun in September is closed.

## archogen — a test's assembly builds on Linux too

`ARCHOGEN-PROGRAM-0479` (leaf `PROGRAM.10.5.2`).

- The seventh failure of the first run on GitHub's machines: a test that hides a file's bytes inside assembly code, to
  check that the trust instrument refuses it, wrote that assembly in a way only Apple's object format accepts. It now
  returns to the code section the way both formats accept, so the test builds and runs on Linux as it does here.

## archogen — the catalog gate reads a commit's date as its design says

`ARCHOGEN-PROGRAM-0478` (leaf `PROGRAM.10.5.1`).

- The first run on GitHub's machines failed six of the catalog gate's tests: they had relied on the developer's own
  git identity, which those machines do not have. Behind that was a real departure from the design: the gate read the
  date of the commit being made with most of the committer's environment removed, so a date or time zone the committer
  set never reached it. It now reads that date in the committer's own environment, as the design says, and the tests
  bring their own identity.

## archogen — the trust gate can propose its baseline

`ARCHOGEN-M3-0477` (leaf `M3.6.3.2`).

- The trust gate's reference list — one entry per piece of code or data two independent programs share — can now be
  written by the tool as a proposal for review, with the review's own judgements left blank and no way to mark an
  entry approved from the file itself. Its fingerprints depend on the machine they were taken on, and the design fixes
  that machine as the CI runner, so the list to commit must be proposed there: that waits on the next push.

## archogen — every program the workspace builds is accounted for

`ARCHOGEN-M3-0476` (leaf `M3.6.3.1`).

- The trust instrument now lists every program the workspace can build — executables, examples, the browser module
  — and each that is not one of the programs a claim relies on is declared as such, with a reason and the parts of
  the engine it compiles. A new program, or one that starts compiling another role's code, is reported for review.
  Today's eight are declared. This is the first step of the trust gate.

## archogen — finished work that carried promises can be archived

`ARCHOGEN-PROGRAM-0472` (leaf `PROGRAM.63`).

- Archiving the finished matching work out of the task tree was refused twice, by two checks that had never met
  archived work of this kind. The check that each design's promises are carried word for word read the two-line
  placeholder left behind instead of the archived text; and the archive allowed shorter lines than the tree it copies
  byte for byte. Both are fixed, each proven by a test that failed before the fix.

## archogen — the book explains matching

`ARCHOGEN-M3-0470` (leaf `M3.1.3`).

- A new chapter, *Matching an offer to a requirement*, explains in plain words and then in full how the engine decides
  whether what one part offers satisfies what another requires: why more is not always better, why a faster counter
  can be worse, and why a function reachable only from a privileged mode does not serve a less privileged caller. With
  it, the work on matching closes: design, implementation held to the design's model, and the book. The trust gate is
  next.

## archogen — the matching rules become normative, and their implementation is complete

`ARCHOGEN-M3-0469` (leaf `M3.1.2.6`).

- The model layer's normative document now governs the matching engine: its rules are stated there, and a check holds
  every diagnostic code the engine can produce to that document. Each refusal reports one of the design's two
  verdicts, names exactly what is wrong and says what to do about it. With this the production implementation of
  the matching rules is complete, held throughout to the design's executable model; every existing verdict is
  unchanged. The book's chapter on matching comes next.

## archogen — the engine decides whether an offer satisfies a requirement

`ARCHOGEN-M3-0468` (leaf `M3.1.2.5`).

- The engine now answers the design's central question: does what this block offers satisfy what that service
  requires? It works out each fact's standing — offered, absent, unknown, or computed, such as a counter's
  unambiguous time span from its wrap count and rate — and judges each requirement against it, with a stronger
  precondition never counted as a better capability. It lists every provider's answer for the resolver to choose from
  later. More than 150 000 pairings agree with the design's model, and the time-span rule agrees with a simulation
  of the counter's actual readings. `archogen check` does not use it yet.

## archogen — the engine reads what a declaration requires

`ARCHOGEN-M3-0467` (leaf `M3.1.2.4`).

- The engine now reads every requirement a declaration writes, wherever the language lets it stand, and reads the
  declaration as one whole: two clauses that contradict each other are refused just as two constraints in one clause
  would be. A declaration named like a fact is refused even inside an imported module, where its full name would hide
  it. More than eleven thousand generated requirements, clauses and declarations read exactly as the design's model
  reads them. The first comparison found an input that crashed both this reader and the previous slice's offer reader;
  both are fixed, and the input is now generated.

## archogen — the engine reads what a provider offers

`ARCHOGEN-M3-0466` (leaf `M3.1.2.3`).

- The engine now reads what a block or a platform offers: each fact's value in exact arithmetic, and the refusals the
  design makes — two values for one fact, an offer beside its own absence, a derived fact beside what it is derived
  from, a modulus too large for its counter. Checked against the design's executable model on more than seventy
  thousand generated declarations, which read identically. Writing the defects that check must catch found two kinds
  of input the model's own checker had never generated; both are now generated, for the model too.

## archogen — the deliberate-defect catalog runs again, and cannot rot unseen

`ARCHOGEN-PROGRAM-0465` (leaf `PROGRAM.61`).

- The catalog of deliberate defects, each of which some test must catch, had stopped running as a whole a week ago:
  one entry named a line of code that had since changed. That entry now names the line as it reads, all 152 entries
  behave as expected again, and the ordinary test run now refuses any entry whose code has moved, in the same change
  that moves it.

## archogen — the task trees' frontiers show the frontier again

`ARCHOGEN-PROGRAM-0464` (leaf `PROGRAM.62`).

- Two task trees' "current frontier" sections had grown into 33 KB of stories about work finished weeks ago, which
  filled the folder's size limit and left no room for new work. Each story was checked to have its line in the tree's
  commit log, where finished work belongs, and the sections now say only what is open and why.

## archogen — the engine reads the capability vocabulary

`ARCHOGEN-M3-0463` (leaf `M3.1.2.2`).

- The engine now reads the vocabulary file into a table it can consult, and checks what the file's own format cannot
  express: that each fact's direction suits its kind of value, that a derived fact names the rule that computes it
  and the inputs that rule reads, that no derivation goes round in a circle, and that no fact is named like a word
  the language reserves. The table matches, fact for fact, the hand-written copy the design's reviewers checked. Each
  rule has a test that fails when the rule is removed.

