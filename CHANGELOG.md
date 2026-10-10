# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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
  signals sealing leaves unheld — SIGKILL, SIGSTOP and seven fault signals — rather than claiming every signal.
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

- Sealing now holds every signal but SIGKILL, SIGSTOP and seven fault signals — not only Ctrl-C, a terminate and a
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

## archogen — a mutation that hangs is reported, not waited on

`ARCHOGEN-PROGRAM-0530` (leaf `PROGRAM.67`), `2026-10-10`.

- The mutation harness, which puts each catalogued defect back and checks a test still catches it, now runs each
  entry's tests under a time limit. A defect that makes a test loop for ever is reported as caught by a hang, with
  every process the tests started stopped, instead of stalling the whole run as one once did.
- Validation: two catalogued breaks of the limit and of the group kill, each caught; a hanging entry reported in 30
  seconds with its file restored; the crate's tests and the focused tier (`passed — 3 passed, 0 failed`).

## archogen — the third reader's record cannot fall behind

`ARCHOGEN-PROGRAM-0529` (leaf `PROGRAM.60`), `2026-10-10`.

- The record of what an independent reader thinks of every description could fall behind, because the check needs a
  vendor build and so ran only by hand. A test now refuses any description the record does not name, on every run,
  with no build needed; only the reader's verdict on a new description still waits for that build.
- Validation: the test failing on the record as it stood on `2026-09-30`, six descriptions named, and on one naming a
  file that is gone; the focused tier (`passed — 3 passed, 0 failed`).

## archogen — what "unchanged" means for the trust gate, exactly

`ARCHOGEN-M3-0528` (leaf `M3.6.7`), `2026-10-10`.

- The trust inventory's design said a change outside the checked programs always reads "unchanged". It now says
  exactly when: on the baseline's own machine, and only for a change to nothing the gate reads besides the programs —
  the toolchain pin, cargo's configuration on the build's path, each member's manifest, links under `catalog/` and the
  comparison harness's own sources are read and judged too. The trust chapter says the same.

## archogen — generated sources: what stays refused, and why

`ARCHOGEN-M3-0527` (leaf `M3.6.6.4`), `2026-10-10`.

- A new design record decides what the generated-sources design had refused until later. A chain of generators and a
  generator or input inside a vendored checkout stay refused for good, and a tool a script runs stays believed. Each
  comes with its reason and, where one exists, with a way to meet the need within one generation step: one declaration
  for the whole chain, a committed copy of a vendored file or generator, the tool's pin committed as an input; the
  record names the cases with none.
- No behaviour changes; the gate's refusal of a chain now names the record, and the trust chapter says it all. Its
  independent review is open.
- Validation: the gate's chain refusals re-checked by their catalogued mutations, the crate's tests, the book's
  checks and build, and the focused tier (`passed — 3 passed, 0 failed`).

## archogen — the book has room to grow again

`ARCHOGEN-PROGRAM-0526` (leaf `PROGRAM.74`), `2026-10-10`.

- The book's total size limit rises from 448 KiB to 576 KiB. It stood two bytes below the old limit after growing
  63 KB in a week, about 1.7 KB for each change that moved it, as its record foresaw when keeping the book in step with
  every rule; the limits on each chapter stay, and the next raise is to weigh splitting the book instead.
- Validation: the route gate and its self-test pass with the new limit and the record that fixes it.

## archogen — a seal interrupted at any instant is undone

`ARCHOGEN-PROGRAM-0525` (leaf `PROGRAM.69`), `2026-10-10`.

- Sealing finished work notes each write before making it, so an interrupt arriving just after one is undone like any
  other; anything already standing where a sealed file goes — a file, a folder or a link — is refused before a write;
  and a task tree or index that is a link is refused rather than replaced.
- The book no longer says every write is undone whatever stops the tool: a process killed outright leaves its writes,
  which the next check names.
- Validation: arms raising an interrupt just after each write and just before one, for a file at a sealed file's
  path, a linked tree, a failed restore and two failed removals — of them the three just after a write, the file
  at the path and the linked tree failing on the old tool; eleven
  deliberate breaks of the rules, each caught by an untracked runner, so not durable (85 arms).

## archogen — room for new design reviews

`ARCHOGEN-PROGRAM-0524` (leaf `PROGRAM.70.2`, closing `PROGRAM.70`), `2026-10-10`.

- Every closed design review's history now lives in the review archive, each read back byte for byte behind a short
  signpost at its old path, so the reviews folder fell from 391 513 to 78 575 bytes. One closed history stays, edited
  by an older commit while it read closed, which the archive refuses to take.
- The folder's file limit now counts the histories a reader opens, not those signposts, which can never leave since
  sealed records cite them: 4 files against 16, room for new designs' reviews.
- Validation: arms for a signpost left out of the count and six look-alikes counted, the first failing on the old
  rule; seven deliberate breaks of the rule, each caught; every self-test; the focused tier (`passed — 3 passed, 0 failed`).

## archogen — a seal stopped half-way leaves nothing torn

`ARCHOGEN-PROGRAM-0521` (leaf `PROGRAM.69`), `2026-10-10`.

- When sealing finished work out of a task tree is stopped — by a full disk, an interrupt or any error — every file it
  rewrites is now either written whole or left as it was, and the rollback undoes exactly what was written, each step
  on its own, naming anything it could not undo instead of leaving a half-written tree.
- A sealed file's place already taken, even by a dangling link, is refused before anything is written.
- Validation: arms for a full disk, a rollback the full disk stops, an interrupt, unreadable git output, a first seal's
  folder and a table's layout, the full-disk and taken-path arms failing first; nine deliberate breaks of the new
  rules, each caught by an untracked runner, so not durable (77 arms).

## archogen — generated sources in the trust chapter

`ARCHOGEN-M3-0518` (leaf `M3.6.6.3`), `2026-10-10`.

- The book's trust chapter now gives the whole account of committed generated sources: what a declaration says and
  that it is believed, how an undeclared one is recognised and what the recogniser cannot see, what stays refused until
  chains and vendored inputs are decided, what two checks share through a generator, what a generator may be, and every
  limit the gate's report states. The glossary gains the words it needs.
- The chapter's list of markers and its table of comment syntax are checked against the recogniser's code by the same
  tests that check the design record's, so the book cannot drift from what the tool does.
- Independent review rounds read the chapter against the record and the code; the defects they found were fixed, one
  of them at its root in the two design records.
- Validation: the tests failing before the chapter held its tables, then passing, and failing again for each break
  made in the chapter's copy; the real tree's inventory and gate; the crate's tests, the book's checks and build, and
  the focused tier (`passed — 3 passed, 0 failed`).

## archogen — the generated-sources gate is whole

`ARCHOGEN-M3-0516` (leaf `M3.6.6.2.5`, closing `M3.6.6.2`), `2026-10-10`.

- The trust gate's report now names every declared generated source among what is not yet reviewed, and ends with
  what the inventory cannot see — the linker, code gated to another target, data a program reads unhanded, and the
  limits of generated-source declarations, which are believed rather than re-run. The trust record gained a dated
  clarification saying how each of its passages reads now that generated sources are judged.
- With this, the instrument and gate for committed generated sources are complete: declared, recognised, judged by
  their generators, shared through their provenance, and stale when no longer read.
- Validation: two gate fixtures and three catalogued mutations, each killed; the whole mutation catalogue and the
  header recogniser's sweep re-run.

## archogen — an undeclared generated source is refused

`ARCHOGEN-M3-0515` (leaf `M3.6.6.2.4`), `2026-10-10`.

- A file a program reads whose first comment marks it generated — `@generated`, *do not edit*, *generated by* — is now
  refused unless a form in `trust/roots.eadl` declares its generator and inputs, and a generator whose own build reads
  a declared or marked file is refused as a chain of generators. A declaration whose file no program reads any more is
  stale on the baseline's host and must be removed. The trust inventory's five steps are all built.
- Validation: four fixtures — one per step's refusal among them — and eleven catalogued mutations, each killed; the real
  tree's inventory unchanged but for an empty list; the focused tier and the whole suite (1 329 tests, 0 failed) pass.

## archogen — a generator may not be another role's code

`ARCHOGEN-M3-0513` (leaf `M3.6.6.2.3`), `2026-10-10`.

- The trust inventory now judges the generator of every declared source a program reads. A table the generator's own
  executable wrote, read by a checker, is refused: the checker would trust the logic it checks. A generator that is a
  program is built and held to the same rules as the roots, its admitted sites counted with theirs; a script is judged
  by the package holding it; a Rust file that is no program's entry point is refused.
- The instrument's build, manifest and source rules became shared pieces, used by the roots and the generators alike;
  the real tree's inventory is byte for byte what it was.
- Validation: seven fixtures and eleven catalogued mutations, each killed, and the 32 earlier ones on the instrument as
  expected; the focused tier and the whole suite (1 325 tests, 0 failed) pass.

## archogen — the trust inventory sees what generated sources share

`ARCHOGEN-M3-0512` (leaf `M3.6.6.2.2`), `2026-10-10`.

- A program that reads a declared generated source now records the generator files and inputs it reaches through it,
  and two programs whose reach meets share a new kind of item the trust gate reports: one data file under two
  generators, a generator both use, a data file one reads that the other's source was made from, or a byte copy of an
  input under another name. Two generated tables differing in every byte, written by one script from one data file,
  were invisible to the gate before; now they are one shared item.
- A declared source whose generator or input is itself generated, or is not a plain file of the commit, is refused.
- Validation: six fixtures and eleven catalogued mutations, each killed; the real tree's inventory unchanged but for
  six empty provenance fields; the focused tier and the whole suite (1 318 tests, 0 failed) pass.

## archogen — finished work leaves the task trees even while its parent stays open

`ARCHOGEN-PROGRAM-0510` and `ARCHOGEN-PROGRAM-0511` (leaf `PROGRAM.69`, its review still open), `2026-10-10`.

- A finished part of a task tree is now sealed out to `docs/task-history/` even when the larger part holding it stays
  open — as `M3.6` does while one of its leaves waits on the director. 46 finished leaves of `M1`, `M2` and `M3` moved,
  byte for byte, taking 255 871 bytes out of `docs/tasks/`, which had reached its 819 200-byte ceiling.
- The gate that guards the sealed history grew stricter: each sealed file is rebuilt byte for byte from the tree before
  its seal and must be the largest finished part; no live leaf may sit inside a sealed part in any tree; the index
  keeps one table per tree and one row per sealed file; a tree names each leaf once. A census mode measures the
  finished leaves under parts still open.
- Validation: independent review rounds 2 to 6, each finding answered, the review still open; a mutation sweep of the
  tool's core on round 5's tool (its runner untracked), 182 of 196 mutants killed and the 14 others reasoned; 66 self-test arms; every
  gate's self-test and every doctrine green.

## archogen — the trust inventory reads generated-source declarations

`ARCHOGEN-M3-0509` (leaf `M3.6.6.2.1`), `2026-10-10`.

- `trust/roots.eadl` takes a `defgenerated` form, declaring a committed generated source with its generator files,
  inputs, command and reason. The inventory reads these forms first, before it writes or builds anything, and refuses
  a malformed one — or two forms declaring one file — as `trust-undeclared-input`; before, any such form left the gate
  unable to judge. Another form of the file departing still leaves the gate unable to judge, now with the declarations'
  refusals printed beside it.
- Two defects of the run fixed on the way: a refusal before any build left an earlier run's inventory in place, to be
  read as this commit's; and a program whose build failed hid the refusals of the programs after it. Now a refused run
  leaves no inventory, and a failed build's step runs to its end, its refusals printed beside the failure.
- Validation: eight new tests, twenty new catalogued mutations each killed; the real tree's inventory byte-identical
  to the previous instrument's; the focused tier and the whole suite (1 312 tests, 0 failed) pass.

## archogen — the sixth artifact cleanup

`ARCHOGEN-PROGRAM-0508` (leaf `PROGRAM.68`), `2026-10-10`.

- About 4 GB of regenerable build output and closed work's scratch removed (`target` 5.6 GB → 1.6 GB): the
  compiler's incremental caches, the finished design review's loose logs and sweep scratch, uncited probe output and
  test scratch. Everything a record cites as evidence was kept. One record said a directory had been removed when it
  had not; removing it made the record true. The three adopted policies' sources have not changed.
- Validation: twelve of twelve sampled paths gone; from cold, the focused tier passed, the whole suite passed
  (1 304 tests, 0 failed, 97 suites) and every doctrine is green.

## archogen — the interrupted generated-sources review recovered and closed

`ARCHOGEN-M3-0507` (leaf `M3.6.6.1`), `2026-10-08`.

- Recovered the clean round-14 checkpoint and completed the interrupted round-15 review in a fresh context that
  did not write the design. It found no defect, closing the design review. The saved next action is `M3.6.6.2`,
  the generated-sources instrument; the session stops before starting it, as the director requested.
- Validation: the recogniser's 51 catalogued mutations killed; 136 xtask tests passed; both listed sweep
  equivalents and two delimiter mutations re-run; the real-tree inventory's generated-marker list empty;
  the hand-off ledger and doctrine enforcer green. The complete sweep and complete workspace mutation
  catalogue retain round 14's evidence on unchanged code; this round's selected checks are named in its history.

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

