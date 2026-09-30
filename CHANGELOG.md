# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the catalog checks its lock across history

`ARCHOGEN-M2-0249` (leaf `M2.7.3.3.2`).

- Given a project's history, the catalog now holds its lock to being append-only, recomputes every commit's new
  lines exactly as blessing would have written them, and checks each review at the commit that first recorded it.
  So a lock edited by hand is caught whether a line was dropped, changed or added, and a review that was wrong when
  it was recorded is caught later, even if the local check was bypassed at the time.
- The one repair §9 allows, a waiver, is accepted only for a recorded review that really fails where it was
  recorded, named at the commit that recorded it.

## archogen — the catalog reads its lock and checks it against the records

`ARCHOGEN-M2-0248` (leaf `M2.7.3.3.1`).

- The catalog's lock pins every reviewed version and keeps every review for good. The crate now reads it, writes
  it back unchanged, and refuses a lock that disagrees with the records beside it: a component changed without its
  version moving, a version going backwards, a version or a review never locked, and a review taken back out of
  its record or misquoted in the lock.
- The worked example's lock is read, checked and rewritten byte for byte, and what blessing would write for its
  record is exactly that lock. Checking the lock across a project's history is the next step.

## archogen — the composition's ninth review is answered

`ARCHOGEN-M2-0247` (leaf `M2.10.1`, a tenth checkpoint).

- The ninth review found the model sound again under both interrupt orders. Its two real findings were about what
  the platform facts left uncovered: application code that kept running outside any task once interrupts were on,
  and start-up code of one component touching a timer or a device whose rules another component's review vouches
  for. Each is now owned: after start-up only tasks and services run, a new fact of the port says when start-up
  ends, and the timer's counter and each device are set up only by the component answerable for them. The
  interrupt controller's set-up is left to the check planned for real images.
- The tenth review runs next.

## archogen — the catalog computes its hashes, and the worked example agrees

`ARCHOGEN-M2-0246` (leaf `M2.7.3.2`).

- The catalog crate now computes every hash the design defines: each part of a record hashed over its own text
  and files, and again over everything it rests on (the code under it, what that code's packages reach, its
  targets' files, the ledger sections it cites, the records it depends on). The design's worked example states 23
  of these values; all 23 come out right, and the test reads them from the example's own file, so the two cannot
  drift apart.
- The worked example has no packages and no ledger, so a second test workspace does: each input is changed in
  turn, and exactly the right hashes must move. To prove the tests, each input was left out of the code in turn:
  all 52 such changes made a test fail, after four gaps the first round of this found were closed.
- The design's rules for packages that go beyond hashing, among them a scanner of Rust source for forbidden
  words, are a leaf of their own, `M2.7.3.7`.

## archogen — the catalog's code begins: a record read and checked

`ARCHOGEN-M2-0245` (leaf `M2.7.3.1`).

- A new crate, `archogen-catalog`, reads one catalog record and refuses anything that breaks the accepted design's
  rules for a single file: its bytes, its one form, every field in order, and the grammar of every name, version,
  locator and review. Each refusal carries the one code the design gives it, with the file, the field and the line.
- One valid record, and one change of it for each rule, are its tests. To show each rule is really tested, each
  check was removed in turn: 45 of 46 removals made a test fail. The one that did not is a final check that
  nothing can reach, since every field it looks at has been checked already; it stays as a guard.
- Next: the design's hashes, with its worked example's 23 values as the test.

## archogen — the composition's eighth review is answered

`ARCHOGEN-M2-0243` (leaf `M2.10.1`, a ninth checkpoint).

- The eighth review simulated the model under the emulator's own interrupt order and found it sound, in over six
  thousand task sets. Its three real findings were in how three platform facts were worded, each leaving a way for
  a service to run that nothing paid for: a timer interrupt that went on to serve devices, start-up code of the
  application that nothing constrained, and a device request that no event had made. Each is reworded to close it.
- The catalog's last small finding on this record lands here too. The ninth review runs next.

## archogen — the catalog design is accepted, after sixteen reviews

`ARCHOGEN-M2-0242` (leaf `M2.7.1`, closed).

- **The sixteenth review found nothing that blocks.** The catalog's own rules held once more, every one of the
  worked example's 23 hashes checked out by two independent routes, and every fact the design states about the
  repository was true. Its ten small findings are answered in this change.
- **So the design is accepted.** It says what a catalog record holds, how its content is hashed, how its evidence
  status is worked out rather than written, when a claim may rest on it, and what it defends against. Sixteen
  reviews, each by a fresh reader trying to break it, took it there.
- **The book has a chapter on it**, "Where the engine's knowledge comes from: the catalog", written from the design
  as accepted.
- **Next:** the catalog's code (`M2.7.3`), the protection of its check (`M2.7.6`), and the first records
  (`M2.7.4`). The composition's eighth review is still running.

## archogen — the catalog's fifteenth review and the composition's seventh are answered

`ARCHOGEN-M2-0241` (leaf `M2.7.1`, a fourteenth checkpoint; with `M2.10.1`'s eighth).

- **The composition's seventh review** found that the emulator does not order interrupts as the RISC-V
  specification says. Without an optional extension, QEMU takes a pending timer interrupt before a pending
  external one. That was checked in QEMU's own source and is now in the ledger. The design had assumed the
  specification's order, and would have refused the emulator altogether. It now admits the emulator's order, given
  a fact about the port that makes the model's formulas hold.
  - One real gap is closed as well: nothing stopped code from writing a device's registers in a way that raises a
    request no event caused. Now only the device's own service and start-up code may.
- **The catalog's fifteenth review** held the catalog's own rules sound again, and once more found holes only in how
  the check that guards it is protected: four rounds running, each in the same area. So the design now states what
  that protection must achieve, and the protection itself becomes its own piece of work, `M2.7.6`, with a test for
  every attack the reviews found. The reviews of the catalog no longer carry it.
- **Both reviews run again.**

## archogen — the catalog's fourteenth review is answered

`ARCHOGEN-M2-0240` (leaf `M2.7.1`, a thirteenth checkpoint).

- **The fourteenth review** found the design one short revision from acceptable. The last round's protection had a
  gap on cargo's side. Cargo reads build configuration before any of the design's refusals applied, and a
  configuration can name a program for cargo to run. So a change could still run its own code in its own check.
  - Now every cargo configuration is checked before cargo runs at all, and the checking tool is built where the
    change's own files cannot reach it.
  - The director's list for the hosting settings grew by three: a second person to approve changes to the checking
    tool, since one person approving their own change protects nothing; fresh machines for the check; and a pinned
    check if a merge queue is used.
  - The work these answers commit the project to is named in the leaves that will build it.
- **The record was split again** to stay under its size limit. Its hash grammar now sits in a file of its own, moved
  unchanged and checked by comparison.
- **The composition's seventh review is running.** The catalog's fifteenth follows.

## archogen — the two older sealing checks had the same blind spot, now closed

`ARCHOGEN-PROGRAM-0239` (leaf `PROGRAM.42`).

- The review of the newest sealing check found that a particular kind of git merge could hide a seal from it. The
  two older checks read history the same way, so the same merge was tried on each, in a throwaway repository.
- Both were fooled. The task trees' check passed after a sealed task was edited again. The changelog's check passed
  after twenty old entries were dropped outright.
- Both now read the whole history, and both refuse a shallow copy of the repository or a history read that fails.
  Each has the merge and a shallow copy as test cases, proven by removing the fix and watching the case fail.
- The folders holding sealed history are now exempt from line-ending conversion, so a checkout on another system
  cannot make them look changed.

## archogen — the decisions folder's sealing check, hardened after its review

`ARCHOGEN-PROGRAM-0238` (leaf `PROGRAM.41.1`; filed by `ARCHOGEN-PROGRAM-0236`).

- An independent review confirmed the first seal of four settled findings: every file is exactly what the register
  held, and nothing was lost. It also found the check not yet safe for more seals. A particular kind of git merge
  could hide a seal from it, after which the settled text could be edited unseen. And its reading of code blocks
  was simple enough that a seal could take a live section with it.
- Both are fixed, along with sixteen smaller findings. The check reads the whole history, whatever a merge did. It
  reads code blocks as Markdown does. It refuses a shallow copy of the repository, and undoes a seal completely
  whenever it refuses one.
- Each of its 54 test cases was proven by breaking the rule it guards and watching that case fail. One of 42 such
  breaks goes unnoticed, and the review file says why: no construction reaches it that another rule does not
  catch first.
- The two older sealing checks read history the same way; `PROGRAM.42` tries the same merge on them next.
- The catalog's fourteenth review and the composition's seventh have not run: their launch stopped at once on the
  account's weekly usage limit.

## archogen — the catalog's thirteenth review and the composition's sixth are answered

`ARCHOGEN-M2-0237` (leaf `M2.7.1`, a twelfth checkpoint; with `M2.10.1`'s seventh). Before it, `ARCHOGEN-M2-0235`
gave the work these reviews commit the project to its owners: `M2.7.6`, the catalog check's protection, and `M4.10`,
everything the catalog design leaves to `M4`, which no `M4` leaf held.

- **The catalog's thirteenth review** found the last round's fix one step short. The checking tool is built with
  three packages the protected list did not name, so a change could still run its own code inside its own check.
  Now the check builds its tool from the version being merged into, and code owners review everything that tool is
  built from. So no change runs its own code when it is checked.
  - The repair route for a bad ledger line now works in the cases it exists for, and never weakens a rejection.
  - The director's findings list now names the two hosting settings this adds.
- **The composition's sixth review** found the model sound in simulation, and two gaps in its facts, both real on
  the emulator. Reading the interrupt controller's claim register is itself a claim, and nothing stopped a task
  doing it. And QEMU's controller queues a new request whenever a device raises its line, even mid-service. Both
  were confirmed in QEMU's source and are now covered, with the source recorded.
- **Both reviews run again.**

## archogen — settled findings leave the decisions folder, their headings kept

`ARCHOGEN-PROGRAM-0234` (leaf `PROGRAM.41`).

- The decisions folder had reached 376 904 of the 380 000 bytes at which its next compaction was due. That
  compaction was named in advance: the four settled items of the director's findings register, one resolved, one
  for information and two ruled.
- Each moved, byte for byte, to `docs/decision-history/`. Its heading stays in the register above one line that
  links it, so every place that cites an item by its number still finds it. The folder is now 368 643 bytes.
- A new check, `DECISION-HISTORY`, holds the sealed items unchanged for good. It checks each against the register
  as it stood just before the seal, so an item edited on its way out is refused, and it catches a forgery committed
  later, in CI as before a commit. It is the task trees' sealing, applied to sections.

## archogen — the catalog's twelfth review and the composition's fifth are answered

`ARCHOGEN-M2-0233` (leaf `M2.7.1`, an eleventh checkpoint; with `M2.10.1`'s sixth).

- **The catalog's twelfth review**, the first judged by its closure rule, found one weakness that matters now: a
  change could alter the code that checks it, because the design protected only the check's definitions. Now
  everything the check builds or runs is protected, and a change to the checker lands on its own, reviewed, before
  any catalog change it judges.
  - The rules version at the head of the catalog's lock is fully specified, so a hand edit to it is refused.
  - No production claim can be made until the director has confirmed how `main` is protected.
  - A ledger line that fails its check, through a bug in the checker for instance, can be repaired by a waiver the
    director rules on. A waiver can only weaken what the catalog says, never strengthen it.
  - The review named two later horizons, a second rules version and the port's assembly (`M2.12`), and the
    closure rule now names them too.
- **The composition's fifth review** found its sums and its timer margin sound across 5 900 simulated task sets.
  Its one under-count matters only on a real board: a level-triggered device acknowledged before it is cleared is
  served twice per event, which the interrupt controller's specification allows. A new platform fact must rule it
  out, and the specification's sentence was checked at its source. The runtime analysis shares the blind spot, and
  `M2.11` now covers it too.
  - Also stated: code acts in the role of whoever calls it, and the processor's own interrupt settings are held
    like the timer's.
- **Both reviews run again.** The composition now closes by the same rule as the catalog.

## archogen — the decisions folder's next compaction is scheduled

`ARCHOGEN-PROGRAM-0232` (leaf `PROGRAM.41`, opened).

- After its split, the decisions folder holds 366 543 of the 393 216 bytes it may hold, and each catalog review adds
  a few kilobytes. The ceiling record names the next compaction: sealing the settled items of the director's
  findings register, about 9.7 KB, with each keeping its section number.
- It is now a task of its own, due before the folder passes 380 000 bytes, so the limit is not met by surprise.

## archogen — the catalog's eleventh review and the composition's fourth are answered

`ARCHOGEN-M2-0231` (leaf `M2.7.1`, a tenth checkpoint; with `M2.10.1`'s fifth).

- **The catalog's eleventh review** found its core rules sound again. It found two weaknesses that matter only once
  a physical board or built images exist: the timer's rounding cost was tied to no code, and nothing covered code
  releasing tasks outside its role. Both are fixed.
  - The rules each review is checked under are now versioned, so a stricter rule later cannot make honest history
    unreadable.
  - The protection of `main` the design assumes now names what keeps its checks from being emptied by the change
    they judge.
- **The composition's fourth review** found two real under-counts in how late the timer can start.
  - One is a timer service that starts just before a release is visible, then rewrites the timer late: 102
    against 83.
  - The other is an interrupt controller that tells the processor about an interrupt after it has been taken, so
    the processor looks and finds nothing. The controller's specification allows it, and repeated it can keep the
    timer from ever running.
  - Both are fixed: the first by a larger margin, the second by a platform fact that must hold. The specification's
    sentences were checked at their source and recorded.
  - The runtime analysis itself has the same blind spot, now a task of its own, `M2.11`.
- **The catalog design has had eleven reviews.** The last two found only weaknesses for boards and built images,
  which come later. So it will close on the first review that finds nothing affecting the current, simulator-only
  stage; anything found after that for later stages is fixed and owned there.

## archogen — the decisions folder is split by subject, without adding room

`ARCHOGEN-PROGRAM-0230` (leaf `PROGRAM.39`).

- The decisions folder was a step away from its warning point, with more review answers waiting. The catalog
  design's four records, the largest and fastest-growing part, moved together into a sub-folder,
  `docs/decisions/catalog/`.
- The split adds no room. The sub-folder has its own per-file limits, but everything in it still counts toward
  the folder's single total. The one-time ceiling the director allowed is now written in a table that the size
  check reads, so raising it again needs a new ruling.
- Two of the project template's checks read the folder only at its top level, and must stay unchanged. So a new
  check makes sure every record in a sub-folder is still listed in the folder's index. The knowledge map links the
  sub-folder by hand.

## archogen — the check on the changelog's sealed history now holds on the server too

`ARCHOGEN-PROGRAM-0229` (leaf `PROGRAM.40`).

- The check that keeps the changelog's and development notes' sealed history unchanged compared the index only
  with the last commit. On the server, the last commit is the one being checked, so a sealed file and its index
  line forged together would have compared equal to themselves.
- It now checks every version of the index ever committed, and every sealed file against the commit that first
  added it, as the task-history check does since the previous change. A new test commits such a forgery and is
  refused.

## archogen — the task-history tool is hardened after its independent review

`ARCHOGEN-PROGRAM-0228` (leaf `PROGRAM.32.4`; `PROGRAM.32` closed).

- An independent review rebuilt the two task trees from what was moved out, with its own code. It found the move
  correct and lossless, and every fingerprint right. It also showed that the checking tool could be fooled:
  - a hand-made move of unfinished work passed;
  - so did an edited moved item;
  - so did a moved file forged together with its index line and committed, which the automated check on the
    server would have missed.
- The tool now checks that every moved item is exactly what its tree held just before the move. It checks every
  moved file against the commit that first added it, so the server's check catches a forgery too. It refuses to
  move an item it cannot slice cleanly, and undoes a move its own check rejects.
- Some figures published with the first move were measured at the wrong moment, and are corrected. The check on
  the changelog and development notes has the same blind spot on the server, and is next.

## archogen — the catalog's tenth review and the composition's third are answered

`ARCHOGEN-M2-0227` (leaf `M2.7.1`, a ninth checkpoint; with `M2.10.1`'s fourth).

- **The catalog's tenth review** found one defect. Facts about the kernel's timer, sections and traps were not
  tied to the record whose timing costs describe that same code. So two honestly reviewed records could be
  combined into one wrong answer. Each fact is now grouped with the cost whose code it states. A claim also now
  needs the built image to contain all the code its facts are about.
- **The composition's third review** found one real under-count. When the timer service rewrites the timer late,
  the timer interrupt can drop and come back only after the service, which leaves room for a task's whole masked
  section first. The reviewer's example gave 60 against a calculated 42, checked again by hand. The calculation now
  counts the service's own remaining time in that case. The sentence of the RISC-V specification this rests on was
  checked at its source and recorded.
- **One item goes to the director** (findings §11). The catalog's safety assumes that `main` only changes through
  merged pull requests that pass their checks, but this project commits directly to `main`. Nothing depends on it
  yet, because no catalog record exists; it must be settled before the first one is written.
- The decisions folder is now close to the point where it must be split into sub-folders, so that comes next.

## archogen — finished work moves out of the two largest task trees

`ARCHOGEN-PROGRAM-0226` (leaf `PROGRAM.32.3`).

- 112 finished items moved, unchanged, out of `M1.md` and `PROGRAM.md` into 69 files under `docs/task-history/`.
  Each left two lines behind with a link. `M1.md` went from 7 633 lines to 2 278, and `PROGRAM.md` from 381 KB to
  111 KB. What is still open, the list of what is next, and the logs did not change.
- The move was checked twice. The tool proved it before writing anything. Then separate code rebuilt both files
  from the placeholders and the moved files, and got the originals byte for byte. Every moved file's fingerprint
  was checked again with a second tool.
- The task-tree folder now has real size limits instead of recorded debt. The new folder has its own limits, and is
  checked on every commit.

## archogen — the tool that moves finished work out of the task trees, and the check that keeps it there

`ARCHOGEN-PROGRAM-0225` (leaf `PROGRAM.32.2`).

- A new tool moves finished parts of a task tree into `docs/task-history/`, unchanged, and leaves a two-line
  placeholder for each finished item. Before it writes anything, it proves that putting every item back would give
  the original file byte for byte.
- A new check runs on every commit. It refuses a moved file that changed, a file missing from its index, an index
  line that changed, and a placeholder that is missing, altered, or points to the wrong file.
- Tried on copies of the two largest trees, it moved 112 finished items and proved both files. `M1.md` would shrink
  from 718 KB to 259 KB, and `PROGRAM.md` from 381 KB to 107 KB. The real move is the next step.
- The check that every code change names its task now says plainly when the named task is already finished and
  moved out, rather than reporting that it lacks a checklist.

## archogen — how finished work leaves the task trees is decided

`ARCHOGEN-PROGRAM-0224` (leaf `PROGRAM.32.1`).

- Most of the two largest task trees is finished work: over three quarters of each file is leaves marked done. The
  director's ruling asked for finished work to be moved out, whole, with a line left in its place.
- A new decision record fixes how.
  - When every item under a heading of the tree is finished, those items move, unchanged, into one file per heading
    in a new folder, `docs/task-history/`.
  - Each item leaves two lines behind, its name and a link.
  - An index records each file's fingerprint, and a check on every commit proves nothing sealed ever changes.
- A read-only audit checked every script that reads the task trees against such a move first. The design keeps all
  of them working, including the project template's scripts that must stay unchanged. The tool is next.

## archogen — the catalog design's ninth review is answered

`ARCHOGEN-M2-0223` (leaf `M2.7.1`, an eighth checkpoint).

- The ninth independent review found no defect in the catalog's rules. Everything the eighth review had reopened
  now holds. It again confirmed every digest of the worked example and every fact the design states about the
  repository.
- Four problems came close, all in the names the new composition design had just added to the catalog. They are
  fixed:
  - one table now says where each fact lives in a record;
  - names that must come from the same record are grouped, and the grouping is checked for every profile and
    target, not just record by record;
  - "the runtime record" is defined;
  - a hardware fact that spoke about a description's interrupt sources is replaced by one per source, which a
    catalog reviewer can actually check.
- Sixteen smaller findings are answered too. Among them: an answer to a rejection now covers only what its
  reviewer saw; the build is re-checked on any change to the catalog; and history is read with git's environment
  fixed.

## archogen — the decisions folder's size limit is raised once, as a reviewed exception

`ARCHOGEN-PROGRAM-0222` (leaf `PROGRAM.38`).

- The decisions folder had reached its limit, and the catalog design's next answers need a little more room. The
  usual fix, splitting the folder into sub-folders, would hide those records from two of the project template's
  checks that archogen may not change yet. Asked, the director chose a one-time raise, reviewed first.
- An independent review found four defects in the first draft:
  - it argued that the content was in the right place, not the policy's own test, which is that the folder's role
    grew;
  - it wrongly said no append-only content was left;
  - its promise to split later contradicted its reason for not splitting now;
  - a later split, as planned, would have added room a second time.
- The rewritten record answers each:
  - the folder's role did grow. Its designs under review went from one record of 72 KB to four records of 136 KB;
  - the raise is stated plainly as the director's exception;
  - the split is a leaf of its own, `PROGRAM.39`, which starts well before the new limit and adds no room;
  - only a new ruling by the director can raise the limit again.
- The limit is now 40 files and 384 KiB. With this change the folder holds 29 files, and has about 61 KB of room.

## archogen — the director rules how far the hold on the scripts folder reaches

`ARCHOGEN-PROGRAM-0221` (leaf `PROGRAM.32`, unblocked).

- The director ruled that the hold covers only the project template's scripts that archogen has never changed. A
  script archogen has changed is archogen's own, and a template update will not touch it.
- Measured against the history, that is 17 files: 15 scripts and the two git hooks, all unchanged since the
  template was imported. Everything else is archogen's, including the three template files it has already
  changed and the 36 it created.
- Sealing closed task-tree leaves is unblocked, because what it needs to change is archogen's. It has to work with
  the 17 held files as they are.

## archogen — the web page is confirmed in a real browser, and the browser binding is complete

`ARCHOGEN-API-0220` (leaf `API.5.5`; `API.5` closed).

- The director reported the rest of the browser run. When the page first loads, it shows `invalid-description
  (exit 10)` for its built-in example, as the book says. The browser was Chrome 154.0.8037.58 on arm64. With the
  earlier report of the answer after Check, both steps of the book's walkthrough are now confirmed in a real
  browser.
- That completes the browser binding. It needs no files and runs no programs. On every tracked description it
  gives the same answer as the command line. And a web page runs it as the book describes.

## archogen — the second review of how the four shared inputs are built is answered

`ARCHOGEN-M2-0219` (leaf `M2.10.1`, a third checkpoint).

- The second independent review found one more way the composed waiting time could fall short. An interrupt can
  become pending while a task is running with interrupts enabled. Before the processor takes it, the task can
  switch interrupts off, and the delivery delay is then paid again afterwards.
- The reviewer's simulation reproduced the gap, and gave an example in which a missed deadline would have been
  reported as met. The waiting time now counts the delivery delay twice, and on that same example the analysis no
  longer claims the deadline holds.
- Ten smaller findings are answered too. Among them: the application must promise not to touch the timer or the
  interrupt controller itself; a masked section may end in more than one way; and every way the calculation can
  fail now has a stated verdict.

## archogen — the runtime analysis design's review history joins the others, and the decisions folder fits again

`ARCHOGEN-PROGRAM-0218` (leaf `PROGRAM.37`).

- This pays the debt the previous commit recorded. The runtime analysis design still carried its four reviews
  inside it, 13 482 bytes, the largest review history left among the decisions. It moved, unchanged, to the review
  folder, and the design keeps a four-line summary and a link.
- The decisions folder is at 318 625 bytes, against a limit of 327 680 that was not raised.

## archogen — the catalog design's section on the runtime analysis's inputs gets a file of its own

`ARCHOGEN-M2-0217` (leaf `M2.7.1`, a seventh checkpoint).

- The catalog design had reached 1 167 of the 1 200 lines its folder allows a file. Its section on what the runtime
  analysis takes from the catalog moved, unchanged, into a companion record. It is still part of the design, and it
  is reviewed with it.
- That section then took the names the new composition design needs: the costs of each runtime call and of a
  task's completion, the seven platform facts the composition relies on, and where delivery ends and a context
  switch ends. The catalog's next review checks them.
- With the new records, the decisions folder went over its total size limit again, 331 050 bytes against 327 680.
  The limit was not raised. A new leaf moves the runtime analysis design's review history into the review folder,
  which pays it with room to spare.

## archogen — the first review of how the four shared inputs are built is answered

`ARCHOGEN-M2-0216` (leaf `M2.10.1`, a second checkpoint).

- An independent review found the arithmetic sound. A simulation of 1 920 random task sets found no case where the
  composed values fell short. It also found three situations the design had missed, each with a worked
  counterexample:
  - a task that finishes while interrupts are still masked;
  - a timer written during start-up;
  - inconsistent verdicts when a value cannot be bounded.
- All three are fixed. The design now also states the seven platform facts it relies on, such as that a waiting
  interrupt is taken as soon as a task unmasks. Each is a reviewed catalog fact, checked when the composition
  runs, so the analysis itself stays as it was reviewed.
- Two sentences of the RISC-V privileged specification were checked against the ratified text and given an entry in
  the book's list of outside sources. One of them settled a question the analysis's design had left open.

## archogen — the catalog design's eighth review is answered

`ARCHOGEN-M2-0215` (leaf `M2.7.1`, a sixth checkpoint).

- The eighth independent review confirmed every digest of the worked example and every fact the design states
  about the repository. It found one defect: Cargo can build a procedural macro from a manifest spelling the design
  did not refuse, and such a macro can put forms the rules forbid into compiled code without their appearing in
  any source.
- The design now refuses every such spelling. Before building, it also asks Cargo itself which packages the build
  will use, and refuses any procedural macro, build script or downloaded package, or any disagreement with the
  design's own reading of the manifests.
- Three more problems were close to defects, and are fixed:
  - dependency lists written in unusual but valid ways were missed;
  - a rejection could be lost when a branch was squashed or rebased. Catalog changes now reach the main branch by
    merge commits, and a rejection binds for good once it is there;
  - two fields of a review's ledger line were never checked.
- Fifteen smaller gaps and wording problems are answered too. The design is now close to its file's size limit,
  so its section on the runtime analysis's inputs moves to a record of its own at the next checkpoint.

## archogen — the director is asked how far the hold on the scripts folder reaches

`ARCHOGEN-PROGRAM-0214` (leaf `PROGRAM.32`, a question recorded).

- The hold on `scripts/` is being followed to the letter, and it now stops two things: sealing closed task-tree
  leaves, and any new check. The folder holds both the project template's files, which the hold protects for a
  later update, and archogen's own checks, which the template has no copy of.
- The findings register gains a tenth item asking whether the hold covers both. The recommendation is the template's
  files only. The register's index line, which still said eight items, now says ten.

## archogen — how the runtime analysis's four shared inputs are built from their parts

`ARCHOGEN-M2-0213` (leaf `M2.10.1`, a checkpoint: written, review pending).

- The runtime timing analysis takes four inputs that no single party knows whole. A task's execution time is part
  the application's code and part the kernel's. How long a task can be masked mixes both. How late a release's
  service can start depends on the kernel, the hardware and the order interrupts are taken in. Until now the
  caller had to supply each whole.
- A new decision record says how each is built from parts, and who owns each part:
  - execution time adds the task's own code, each call into the runtime and the completion path;
  - the longest masked stretch is the longest of the task's masked runs, each adding up everything inside it;
  - a release's lateness is a fixed point: rounding, whatever masked work is in progress, delivery, and every
    interrupt service that can be taken first.
- The rule throughout is the definition's own shape: parts that run one after another are added, never replaced
  by the larger of them, which is the under-charge an earlier review warned about.
- The record now goes to an independent review, and is implemented once the catalog code exists.

## archogen — sealing closed task-tree leaves waits on the director's hold on the scripts folder

`ARCHOGEN-PROGRAM-0212` (leaf `PROGRAM.32`, status only).

- Sealing closed leaves out of the task trees needs a new check and changes to the checks that read leaves, and all
  of those live in `scripts/`, which the director has asked to be left alone for now. The leaf is marked blocked on
  that hold, so the status board says why it is not moving. Nothing else changed.

## archogen — review histories get a folder of their own, and the decisions folder is back under its limit

`ARCHOGEN-PROGRAM-0211` (leaf `PROGRAM.36`).

- The previous commit took the decisions folder just over its size limit and recorded that as debt. This pays it.
  Most of the growth there was a review history: the catalog design's seven rounds of independent review, which
  only ever get longer.
- Review histories now live in `docs/reviews/`, one file per reviewed design, with a short index and size limits of
  their own. The catalog design's history moved there unchanged, checked by its fingerprint. The decisions folder is
  at 292 983 bytes, against a limit of 327 680 that was not raised.
- The book's page on where the landing page sends things still said the changelog and the development notes were
  unbounded, which stopped being true when they became rolling ledgers. It now says what is bounded and by what.

## archogen — the catalog design's seventh review is answered, the first under its threat model

`ARCHOGEN-M2-0210` (leaf `M2.7.1`, a fifth checkpoint).

- The seventh independent review was the first judged against the stated threat model. It found three defects,
  and none needed a premise broken:
  - a module could be loaded from a file whose name does not end in `.rs`, and so escape the source scan;
  - a macro written in the package could re-create a refused attribute or foreign block;
  - a review line copied by hand onto an unpublished branch could decide what a published rejection covered.
- The fixes:
  - the scan now covers every file the compiler actually read;
  - a package the catalog relies on may not define macros;
  - every commit a review is read from is first checked against the review's recorded line, in the published
    history too;
  - the packages an image compiles are held to the same rules as the ones a claim read, and are recorded.
- While answering, a weakness was found in an earlier answer: a rejected cost was meant to follow its target
  through a rename, but it could never match, because a target's files hold its own name. It now follows the
  target's kind and instruction set.
- The worked example of the hash grammar moved, unchanged, into a record of its own, because the design had
  outgrown its file's size limit. All 23 of its digests were recomputed.
- With the example in its own file, the decisions folder as a whole went just over its size limit, 329 428 bytes
  against 327 680. The limit was not raised. The overrun is recorded as owned debt, and a new leaf, `PROGRAM.36`,
  moves the review histories, which only ever grow, to a home of their own before the next review round.

## archogen — the web page answers in a real browser as the book says

`ARCHOGEN-API-0209` (leaf `API.5.5`, partial).

- The director followed the book's steps for opening the page in a browser. After Check with the book's
  description, the page showed `ok (exit 0)` and `accepted against profile rt-static-up-v1 (eadl/1), 1
  declaration(s)`: the book's transcript, line for line. So the module loaded and answered in a real browser, not
  only in the stand-in the automated check uses.
- Two observations the leaf asks for are not reported yet: the answer the page shows when it first loads, and the
  browser's name and version. The leaf stays open for them rather than claiming them.

## archogen — removing the project template's references is filed, and postponed

`ARCHOGEN-TEMPLATE-REFS-0208` (tree `TEMPLATE-REFS`).

- The director ruled that archogen should carry no reference to the project template it was created from, then
  postponed the work: the template's own files are being reworked, and will soon be updatable here wherever they
  have not been changed locally.
- A new task tree holds the work until then: a fresh count of every reference, the live documents, the template's
  functional files taken through its own update route, a ruling on the history, and a check that refuses a new
  reference. When it was filed there were 134 references in 30 files.
- Nothing under `scripts/` is touched before the director says so, the setup script and the template updater
  included.

