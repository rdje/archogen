# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the sealed cases' guard names files as they are named, and says the published branch still holds them

`ARCHOGEN-PROGRAM-0562` (leaf `PROGRAM.76`), `2026-10-10`.

- Two places in the seal's check printed a non-ASCII path in git's quoted form, and two of its own tests checked less
  than they claimed. Both are fixed. The texts now also say what a review found: the repository's published branch
  is still at a revision that holds the sealed cases, so a fresh clone, its web view and its code search reach them
  until the branch is pushed past the change that took them out; web searches of the repository are fenced until
  then.
- Validation: 65 arms; two failing on the previous check and four deliberate breaks each caught, both by untracked
  runners, so not durable.

## archogen — every refusal the generated-sources record names holds wherever its entry stands

`ARCHOGEN-M3-0560` (leaf `M3.6.6.4`), `2026-10-10`.

- Two more rounds found refusals tested only where the refused file stood alone in its form, or at one position. One sweep now places each
  kind of refused entry the record names — declared, marked, under a vendored checkout, named twice, another role's
  program, a program built from a generated file, a lone Rust file — alone, first, in the middle and last, and the gate
  refuses it every time.
- Validation: sixteen route tests; the crate's 181 tests; the mutations each fails under, by an untracked runner, so
  not durable.

## archogen — the sealed cases' guard keeps no manifest line, and its verdict no race

`ARCHOGEN-PROGRAM-0558` (leaf `PROGRAM.76`), `2026-10-10`.

- The seal's check copied every manifest line into its scratch, and took a warning git gives while re-reading a
  freshly written file for a refusal, so its verdict could hang on timing. It now reads only well-formed entries,
  takes git's warnings from listing untracked files alone, and reads a submodule's registration from what is staged.
  The texts put the acts that still reach a case as two kinds: writing an older commit to disk, and handing a sealed
  file to a reader or a tool.
- Validation: 63 arms, five failing on the previous check and four deliberate breaks each caught, both by untracked
  runners, so not durable.

## archogen — the generated-sources routes' tests hold each judgment both ways

`ARCHOGEN-M3-0555` (leaf `M3.6.6.4`), `2026-10-10`.

- Three more review rounds found rules the gate kept but no route test held: a declared script taken for a chain,
  every step's generator and every reader judged, in either order, and a later step's own input. Each now has a test
  that fails when the rule is broken; the record claims a copy refused only by the role rules, which an admission
  cannot lift.
- Validation: fifteen route tests, three of the parent's named beside them; the crate's 180 tests; the mutations each
  rule's test fails under, by an untracked runner, so not durable.

## archogen — the sealed cases' guard holds no wrong-form value in the shell, and refuses a stray gitlink

`ARCHOGEN-PROGRAM-0554` (leaf `PROGRAM.76`), `2026-10-10`.

- The seal's check held the manifest's values in shell variables before checking them, so a forced trace could print a
  case's line written there, and a stray clone staged as a gitlink passed unseen. Each value is now checked inside its
  pipeline before anything holds it, and a gitlink no `.gitmodules` entry registers is refused. The texts say exactly
  what it prints — paths, line numbers, names and commit ids — and name the viewers, archives and reverts that still
  reach a case.
- Validation: 58 arms, a forced trace over nine places among them; two failing on the previous check and four
  deliberate breaks each caught, both by untracked runners, so not durable.

## archogen — an interrupted seal says no more than reached its outputs

`ARCHOGEN-PROGRAM-0552` (leaf `PROGRAM.69`), `2026-10-10`.

- With its output closed from the start an interrupted seal crashed, and an outcome written there counted as said;
  with its error output broken it exited 120. Sealing now counts an outcome said only where it could be written, and
  an interrupt exits 130 whatever became of its outputs. The texts add an abort to what can still leave a seal's
  writes.
- Validation: arms with each output closed or broken, and a git that reads its own mask; four deliberate breaks, each
  caught by an untracked runner, so not durable (116 arms).

## archogen — the sealed cases' guard prints no manifest value and fails closed on what git cannot list

`ARCHOGEN-PROGRAM-0550` (leaf `PROGRAM.76`), `2026-10-10`.

- The seal's check still echoed two manifest values it could not read, so a case's line written there would print; and
  a copy in a folder git could not open, or under a folder that could not be searched, passed. It now echoes no
  manifest value of the wrong form, names an exposure it cannot match by its line's number, and refuses whatever git
  cannot list. The texts name what it does not look into: a registered submodule, a folder a link points outside to, a
  folder an untracked ignore file hides.
- Validation: 56 arms, among them a case's line seeded into eight places of the manifest and a run with tracing forced
  on; four failing on the previous check and seven deliberate breaks each caught, both by untracked runners, so not
  durable.

## archogen — every route the generated-sources record offers is a test

`ARCHOGEN-M3-0549` (leaf `M3.6.6.4`), `2026-10-10`.

- Review rounds found routes the record offered fail when a reviewer built them. Each route, and each case the record
  says has none, is now a test that builds it and checks the gate's verdict, and the record claims no more than they
  hold; one route, a program that writes its own source, was narrowed to what they show.
- Validation: twelve route tests, three of the parent's named beside them; the crate's 177 tests.

## archogen — an interrupted seal exits 130 even with its report lost

`ARCHOGEN-PROGRAM-0548` (leaf `PROGRAM.69`), `2026-10-10`.

- With its output a pipe whose reader had left, an interrupted seal exited 120 rather than 130; it now exits 130.
  SIGABRT is held as well, since an abort ends the process whatever is held, leaving SIGKILL, SIGSTOP, six fault
  signals and, on Linux, the C library's own unheld. The texts say that only an interrupt reports which outcome it
  found, and only where its output can be written.
- Validation: arms for the mask's breadth both ways and for the start recorded before the first write; five deliberate
  breaks, each caught by an untracked runner, so not durable (111 arms).

## archogen — the sealed cases' guard echoes no bad entry, traces nothing, and seeks more copies

`ARCHOGEN-PROGRAM-0547` (leaf `PROGRAM.76`), `2026-10-10`.

- The seal's check could print a sealed case's line in two ways — echoing a manifest line it could not read, and under
  a shell trace — and missed a copy under a name git quotes, in an unreadable file, in a nested repository, staged and
  gone from disk, or in the manifest itself. It now names a bad line by its number, never traces, reads paths whole,
  refuses a file it cannot read, and looks at the staged tree as well as the disk. The texts give the revision the
  diff guard starts at, and the acts that still reach a case. Local clones whose index still held the set were
  removed.
- Validation: 53 arms, nine failing on the previous check and ten deliberate breaks each caught, both by untracked
  runners, so not durable; no run prints a case's text.

## archogen — a seal's stop says the truth even when its report is lost

`ARCHOGEN-PROGRAM-0546` (leaf `PROGRAM.69`), `2026-10-10`.

- An interrupt that came while sealing's own report could not be written said nothing had been written, though the
  seal was made and kept; with the report's pipe broken it now says the outcome could not be said. The texts name the
  signals sealing leaves unheld — SIGKILL, SIGSTOP and seven fault signals, the C library's own named from round 17 —
  rather than
  claiming every signal.
- Validation: arms for the mask in force at the first write, a lost report, a plain file at the index's temporary path
  and the error path's own breach line; six deliberate breaks, each caught by an untracked runner, so not durable (109
  arms).

## archogen — the sealed cases hidden from diffs too, and their copies sought

`ARCHOGEN-PROGRAM-0544` (leaf `PROGRAM.76`), `2026-10-10`.

- The change that took the sealed cases out of the tree showed their text in its own diff, and the check looked for a
  case only at its own path. Diffs, log patches and history searches of the sealed paths now show no line of them in a
  working tree that carries the change; the check looks for a case's whole text anywhere in the working tree and for a
  long line of it quoted in any file, refuses anything else in the sealed folder at any depth, and pins the set to the
  one sealed. The texts now claim no more than that: a deliberate read of history still reaches a case.
- Validation: 40 arms, thirteen failing on the previous check and eleven deliberate breaks each caught, both by
  untracked runners, so not durable; no run prints a case's text.

## archogen — a seal stopped says what happened

`ARCHOGEN-PROGRAM-0543` (leaf `PROGRAM.69`), `2026-10-10`.

- Sealing now holds every signal but SIGKILL, SIGSTOP, seven fault signals and, on Linux, the C library's own — not
  only Ctrl-C, a terminate and a
  hang-up — until it is done, and an interrupt then reports what happened: before anything was written, or after the
  seal said its outcome. An error that stops a seal is reported before the stop takes effect. A stray file at a
  temporary path is refused before anything is written.
- Validation: arms for a stop at the first write, for four more signals, for an error under a held stop and for a stop
  before any write; nine failing on the previous tool; seven deliberate breaks, each caught by an untracked runner, so
  not durable (106 arms).

## archogen — the sealed evaluation cases out of the working tree

`ARCHOGEN-PROGRAM-0542` (leaf `PROGRAM.76`), `2026-10-10`.

- The five sealed evaluation cases were guarded against edits and against being named elsewhere, but not against
  being read, and a search made for other work opened one. Their text is now out of the working tree: the commit
  that sealed them holds it, the manifest keeps each digest, and no search of the tree can reach a case. At the
  reuse measurement a restore writes each back and verifies it. The case read early is recorded, without being
  named outside the sealed folder, and will be reported apart from the four unseen ones.
- Validation: 26 arms, seventeen failing on the old check and nine deliberate breaks each caught, both by untracked
  runners, so not durable; no run prints a case's text.

## archogen — a seal finishes before a request to stop takes effect

`ARCHOGEN-PROGRAM-0539` (leaf `PROGRAM.69`), `2026-10-10`.

- Reviews kept finding moments in which a request to stop could slip between sealing's writes and its record of
  them. Instead of answering a stop after the fact, sealing now holds Ctrl-C, a terminate and a hang-up
  until it is done — proven and kept, or refused and undone — so the history is whole either way, with no such moment
  left. Its temporary file is created new, never written through something already there.
- Validation: arms for a stop at each write, mid-proof, twice, and during a refused seal's undoing, each with its own
  exit code, ten failing on the old tool; five deliberate breaks, each caught by an untracked runner, so not durable
  (98 arms). One arm had been passing for the wrong reason — the shell's own report of a killed job quotes the source
  — and now reads the tool's output line alone.

## archogen — a report to the template this project came from

`ARCHOGEN-PROGRAM-0537` (leaf `PROGRAM.75`), `2026-10-10`.

- The template's handoff check judges one instant, so a terminal helper living a few seconds can make it fail and
  then pass. archogen works around it with its own second sample, and now reports it to the template's owner as an
  outbound issue with a reproducer measured on the template's current revision, proposing the same second sample.
- Validation: the reproducer, reproducing; the feedback registers' checks.

## archogen — sealing claims only what it does

`ARCHOGEN-PROGRAM-0535` (leaf `PROGRAM.69`), `2026-10-10`.

- The previous entry still said too much. A request to stop that arrives once sealing has finished its check now
  finds the seal kept, and the texts say so; a stop recorded until then is answered by undoing everything. A link
  anywhere on a path the seal writes — not only the files themselves — is refused, and "rolled back" is printed only
  when everything was undone. A hang-up ignored on entry, as under `nohup`, stays ignored.
- Validation: arms for each, six failing on the old tool; seven deliberate breaks of the rules, each caught by an
  untracked runner, so not durable (98 arms).

## archogen — a cited test run backs the very tree committed

`ARCHOGEN-PROGRAM-0534` (leaf `PROGRAM.53`), `2026-10-10`.

- A leaf's evidence that the focused tests passed could come from a run made before the last file changed. The
  focused tier now records the exact tree it passed on, and a commit whose leaf cites that run is refused unless the
  tree it commits is that tree.
- Validation: the gate's four new arms, two failing on the old gate; the crate's tests; every self-test; the focused
  tier on the staged tree, stamped.

## archogen — the handoff check no longer depends on the instant

`ARCHOGEN-PROGRAM-0533` (leaf `PROGRAM.54`), `2026-10-10`.

- Before a session hands over, a check lists any background job still running in the repository. A terminal helper
  that lives a few seconds could make it fail at one moment and pass the next. A new wrapper samples twice, a few
  seconds apart, and counts only what both samples hold, so a real job is still caught and a passing helper is not.
  The same idea is filed to be proposed to the template the check comes from.
- Validation: the wrapper's two arms, the second sample's removal caught; every self-test; the focused tier
  (`passed — 3 passed, 0 failed`).

## archogen — a seal asked to stop is undone

`ARCHOGEN-PROGRAM-0531` (leaf `PROGRAM.69`), `2026-10-10`.

- Sealing finished work now records a request to stop — Ctrl-C, a terminate or a hang-up — and answers it between its
  writes by undoing them all, the undoing itself never cut short. The previous entry's "at any instant" was too wide:
  a stop arriving while it undid a refused seal could escape. Now a stop before the seal's check is done undoes it; a
  process killed outright leaves writes, which the next check names. A sealing folder that is a link is refused too.
- Validation: arms for a stop at each write, mid-proof and during the undoing, for a terminate, a linked index and
  folder, and another writer's file, the stop and folder arms failing on the old tool; six deliberate breaks, each
  caught by an untracked runner, so not durable (90 arms).

