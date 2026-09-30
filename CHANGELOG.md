# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — claims cite what they read, and say why they fall short

`ARCHOGEN-M2-0258` (leaf `M2.7.3.6.1`).

- A result that relies on the catalog now reads it only through lookups, so what it cites is exactly what it read,
  and what it rests on follows from that by construction; nobody writes a list of sources that could leave one out.
- A result claimed as production-grade is refused with every reason that applies: unreviewed components, inputs
  from outside the catalog, timing not measured on real hardware, code with no built image, a history that lacks
  what the main line recorded, and, until the director turns it on, the main line's protection.
- A result resting on a timing model is untouched when a behavioral model changes, and the other way round.

## archogen — the catalog holds its production namespace

`ARCHOGEN-M2-0257` (leaf `M2.7.3.5.3`).

- A component filed as production-ready must have every part reviewed as such, nothing left unknown, everything it
  builds on production-ready too, and the parts its kind needs actually present. One that falls short stops the
  catalog loading rather than being quietly demoted, so the next edit to a reviewed component cannot slip through.

## archogen — the catalog answers the analysis's lookups, one source per name

`ARCHOGEN-M2-0256` (leaf `M2.7.3.5.2`).

- The analysis asks the catalog for a fact or a cost by name, for a profile and a target. Exactly one component may
  answer; two that disagree, or even agree, are refused as a conflict to investigate rather than averaged.
- A fact about the code of a measured cost is read only from the component that supplies that cost, so one review
  always sees the fact and the code together.

## archogen — the catalog reader knows the facts the analysis reads

`ARCHOGEN-M2-0255` (leaf `M2.7.3.5.1`).

- Each fact the timing analysis reads has one place in a catalog record, and one kind of evidence: a fact about
  code must point into code. A record that files one elsewhere, or backs a code fact with a document, is refused.
- Until the processor-specific assembly can be catalogued, every fact about it must be stated as unknown, so no
  review can vouch for code no record holds.

## archogen — the composition of the analysis's inputs is accepted, after eleven reviews

`ARCHOGEN-M2-0254` (leaf `M2.10.1`, closed).

- The design for putting four of the timing analysis's inputs together from their parts, each part with one owner,
  passed its eleventh independent review with nothing live left to fix. The review simulated it under both
  interrupt orders, proved the simulation can find the earlier faults, and checked writes by devices that move data
  on their own.
- Its three last findings, all about later milestones or wording, are answered. Building it waits on the catalog
  crate, which is well under way.

## archogen — the catalog says which results a change invalidates

`ARCHOGEN-M2-0253` (leaf `M2.7.3.4.3`).

- Given what a result relied on, each component's content and review status, and each lookup it made, the catalog
  now names what no longer holds: a component gone or unreadable, content changed, a review since given, or a
  lookup that would now find something else. It errs toward too many, never too few.
- It keeps working when the catalog itself no longer loads, judging each component on its own, which is exactly
  when knowing what broke matters most.

## archogen — the catalog checks each review where it was recorded

`ARCHOGEN-M2-0252` (leaf `M2.7.3.4.2`).

- A review can no longer be dated after the commit that recorded it, read in that commit's own time zone, so a
  reviewer east of UTC is not refused before UTC's midnight. A review can answer only a rejection that actually
  reaches what it reviews, and a component's own rejection only once it has been recorded earlier.
- Deleting a rejected component no longer sheds its rejection: its successor inherits it, or the deletion is
  refused. A retired name cannot be reused, and a successor cannot drop where it came from.

## archogen — the catalog derives evidence status

`ARCHOGEN-M2-0251` (leaf `M2.7.3.4.1`).

- The catalog now works out, from its history, whether each part of each component is fit for production. A
  rejection sticks to what it rejected: to the component, to anything that replaced it, and to any other component
  that copies the same facts, files, costs or promises. It lifts only where a reviewer answered it having seen
  everything of it the component now holds.
- A component is production-ready only where a review approves exactly its current content, and an approval never
  outlives a rejection of that same content. A repair that §9 allows can lower a status but never raise one.

## archogen — the composition's tenth review is answered

`ARCHOGEN-M2-0250` (leaf `M2.10.1`, an eleventh checkpoint).

- The tenth review found no defect and the model sound again, and its simulation now proves it can find faults by
  catching each earlier one when it is put back. One wording, who may point the processor's trap handler elsewhere,
  still allowed a late timer under a loose reading, so the leaf does not close on this round.
- Answered: only the component that owns the trap path sets the trap handler; the promise that no release falls
  due before start-up ends is now the caller's to make until the image plan orders start-up; and a fact that cannot
  be read now leaves a result undecided rather than refusing the platform. The eleventh review runs next.

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

