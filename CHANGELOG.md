# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

## archogen — the changelog and the development notes stop growing without bound

`ARCHOGEN-PROGRAM-0207` (leaf `PROGRAM.31`).

- The director delegated the §8 ruling on how the project's histories are kept. This is the first half of it. The
  changelog had reached 327 KB in seventeen days, and the development notes 135 KB.
- Both are now rolling ledgers. When the changelog holds 40 entries, its oldest 20 move, byte for byte, into the
  next numbered file under `docs/history/changelog/`; the development notes do the same at 20 and 10. An index,
  `docs/history/INDEX.md`, lists every sealed file with its range, its size and a fingerprint.
- Nothing of archogen's is lost or rewritten. The live file followed by the sealed files, newest first, is exactly
  archogen's entries as they were, checked byte for byte when the first files were sealed. What was removed is
  the project template's own entries, which it had wrongly shipped into both files: a template defect since fixed
  there, and on the director's word archogen keeps no reference to it.
- A new check runs on every commit. It refuses a sealed file that changed, a sealed file missing from the index or
  an index line that changed, entries out of order, and a live file that has grown past twice its window. The
  last is repaired by one command, which seals and proves the result.
- The first proposal was to seal by month. It would have sealed nothing, since the whole history is one month, so
  the boundary is a count of entries instead.

## archogen — the catalog design states what it defends against, and its sixth review is answered

`ARCHOGEN-M2-0206` (leaf `M2.7.1`, a fourth checkpoint).

- The sixth review found five more defects, and most of them assumed someone who controls the build machine or
  GitHub's settings. Asked, the director ruled that the design states what it defends against. It now does, in a
  new first section:
  - every change that reaches the catalog through the repository must be caught, including someone skipping the
    local check or editing the review record by hand;
  - it assumes, and names, that the installed compiler is untampered, that git's local settings don't alter
    files, and that the main branch is protected. Where a cheap check exists, the check is made.
- The biggest change removes a whole class of problem. What a rejection covered, what a review answered, and
  which entry replaced which are no longer written down separately. They are read from the commit where each
  review was first recorded, so there is nothing to edit away.
- The build check now writes every file exactly as git stores it, confirms the compiler is the pinned release, and
  refuses more ways for code to reach what was never reviewed.
- A seventh review is next, judged against the stated threat model.

## archogen — a web page that checks a description

`ARCHOGEN-API-0205` (leaf `API.5.4`).

- `crates/archogen-wasm/page/` is a web page: type an eADL description, press Check, and it shows the verdict and
  the diagnostics exactly as `archogen check` would. The engine-API chapter of the book says how to open it: build
  the module, serve the repository, open the page.
- The book shows what the page answers for an example, and the integration check reproduces that answer on every
  run, so the chapter cannot drift from the page.
- The page's own code, including how it loads the module and responds to the button, is run on every check against
  a stand-in for the browser. Its first run caught a real mistake in how the check started the page, fixed before
  commit.
- Not done yet: the page has never been opened in a real browser, because this session had no browser tools. That
  is filed as its own task (`API.5.5`) rather than claimed.

## archogen — the catalog design's fifth review answered

`ARCHOGEN-M2-0204` (leaf `M2.7.1`, a third checkpoint).

- A fifth independent reviewer found two more ways the catalog design could let unreviewed content back a claim
  someone relies on, and a third through a branch made before a rejection was published. All are now answered:
  - a claim someone relies on must now show, from the build of the system image itself, that the compiler read
    only reviewed files;
  - the record of reviews can no longer be quietly edited by hand, because every new line is recomputed and
    compared;
  - a rejection published on the main line now binds claims made from any branch.
- A rejection now follows content by what it is as well as by where it is: the same file bytes, or the same
  statements, carry it even after a rename.
- The rules for what the build may read were tightened: no assembly code or compiler flags in this first version,
  and no build configuration the check has not seen.
- The design and its review history are now two records. The history had grown to the point of crowding out the
  design, and would have passed the size limit the README policy set for decision records.
- A sixth review is next.

## archogen — the browser module gives the command line's answers

`ARCHOGEN-API-0203` (leaf `API.5.3`).

- The module a web page will load is now built and run on every integration check, the way a page runs it: through
  the same small loader file, in a JavaScript runtime (Node).
- It imports nothing, as read from the compiled module itself. A WebAssembly module can only reach the outside
  world through its imports, so this one cannot touch files, the network or a clock, whatever page loads it.
- Every description the repository tracks goes through it. Each answer is byte-for-byte what the same code gives
  when built normally, and each verdict is what `archogen check` gives for the same file. The descriptions cover
  six different verdicts.
- The check was shown able to fail. A loader deliberately broken to ask the wrong question was refused on every
  description. Hand-made modules, one that imports something and one that exports the wrong things, are each
  refused.
- Node now has an entry in the book's ledger of outside tools. The entry for Miri was re-checked, because its
  stated trigger, the first raw-pointer code in the project, has now happened (in a test), and that test passes
  under Miri.

## archogen — the web-page binding built and tested

`ARCHOGEN-API-0202` (leaf `API.5.2`).

- The module a web page will load now exists as the crate `archogen-wasm`. A page hands it a description, and gets
  back JSON carrying the same verdict and diagnostics the command line would print.
- Every answer is checked field by field against the engine's own answer, by a separate JSON reader written for
  the purpose. The answer's shape is frozen under its format name, so it cannot change without the name changing.
- The part where a page writes into the module's memory was run under Miri, a checker for memory misuse, and
  passed.
- Two deliberate mistakes, a string written a second valid way and a module accepted twice, are each caught by
  their own test and kept in the mutation catalogue.
- Next: building the module for the browser, and checking that every description in the repository gets the same
  answer from it as from the command line.

## archogen — how archogen will run in a web page, decided

`ARCHOGEN-API-0201` (leaf `API.5.1`).

- archogen's description checker will be loadable by a web page, as a WebAssembly module. The design is now fixed
  in `docs/decisions/decision_wasm-binding.md`. A page hands the module a description, and gets back the same
  verdict and the same diagnostics the command line prints, as JSON.
- The module will be built from archogen's own code alone, with no generated glue and no outside library.
- It will import nothing. A WebAssembly module can only reach the outside world through what it imports, so this
  means it cannot touch files, the network or anything else, whatever page loads it. The check for that will be the
  browser engine's own reading of the module, not a reading of the source.
- It will be checked against the command line: every description in the repository, run through the module, must
  give the same answer.
- Two facts the design rests on were measured on the pinned compiler rather than assumed: what such a module
  imports and exports by default, and that the compiler treats exporting a function by a fixed name as unsafe code,
  which the design therefore names as its one exception.

## archogen — the catalog's fingerprint, computed inside the engine and checked three ways

`ARCHOGEN-M2-0200` (leaf `M2.7.2`).

- Every entry in the engine's knowledge catalog will be identified by a SHA-256 fingerprint, and a review will hold
  only for the fingerprint it names. The engine now computes SHA-256 itself (`archogen_evidence::sha256`), because
  it depends on no outside code and runs no other program.
- It is checked three ways, none of which is its own code: it reproduces the standard's published examples; its
  internal constants are recomputed from their mathematical definition; and for every message length that puts
  the padding somewhere different, it gives the same answer as the system's own tool.
- The third check matters most. A deliberately planted off-by-one in the padding passes every published example,
  because none of them is the one length that exposes it. It fails the length-by-length comparison. Both results
  are kept in the mutation catalogue, so the gap in the published examples stays documented and the bug stays
  caught.

## archogen — the catalog design's fourth review answered: a rejection now follows the content it rejected

`ARCHOGEN-M2-0199` (leaf `M2.7.1`, a second checkpoint).

- A fourth independent reviewer went through the catalog design and found four ways for a rejected or unreviewed
  entry to back a claim someone relies on. It also showed that an earlier fix, for renaming an entry to escape a
  rejection, could still be got round. Every finding is now answered in the design, and so are two more found
  while answering.
- A rejection now follows the content it rejected. If a rejected cost or piece of code moves to another entry, or
  the entry is renamed, the rejection moves with it until a reviewer answers it. Put back content that was
  rejected, and it does not become approved again.
- Changing what an entry promises now sends both of its models back for review. So does adding a new target, for
  entries that claim to hold on every target.
- A claim that someone will rely on must now read committed history, check that no record of a review has been
  removed from it, and rest on a system image built from exactly the reviewed code.
- The design's worked example was recomputed by a separate implementation of the hashing rules. It first
  reproduced every value from before the change, then gave the new ones, and three standard tools agree on them.
  A fifth review is next.

## archogen — every place the README sends things is registered and bounded, or named as owned debt

`ARCHOGEN-PROGRAM-0198` (leaf `PROGRAM.35.2`; `PROGRAM.35` closes).

- A size limit on the landing page only moves the need to write things down somewhere else. The README policy
  therefore asks that every place the README points to, and every place its checks tell an author to put what does
  not fit, is itself kept in bounds.
- A new check, `README-ROUTES`, works that list out on every commit instead of trusting a hand-kept one. It reads
  the README's links, runs the README's two size checks on an oversized page, and reads the places they actually
  name. It then follows each place on to wherever that place's own check sends overflow. There are 18 such places.
  Each is registered in the policy with an owner, a lifecycle and a size limit measured from what it holds today.
- The changelog, the development notes and the task trees have no limit yet. The check records them as debt owned
  by open tasks, which wait on the director's decision about how these histories should be kept (the findings
  record, §8). They are named, not counted as controlled.
- The check refuses a link or a named place with no entry, a stale entry, a limit exceeded, debt pinned on a
  finished task, and a chain of places that loops back on itself. Sixteen self-test cases and six deliberate breaks
  of the real registry each produced the expected refusal.

## archogen — the README policy adopted at its source's revision, and the landing page's limits measured

`ARCHOGEN-PROGRAM-0197` (leaf `PROGRAM.35.1`).

- The policy that keeps `README.md` a short landing page is now the revision the director's standing instruction
  names: fsmgen's `README_POLICY.md`, copied byte for byte and checked against its source, under a note that says
  where it came from and who owns it here. The copy it replaces was an older version from the project template.
- The landing page's size limits are now measured from the page itself: 110 lines and 6,144 bytes, about a third
  more than the page holds today. Before, they were the template's defaults, 300 lines and 16,384 bytes, which
  would have let the page nearly quadruple before anything noticed. Pushing the page one line or one byte past
  its limit now fails the commit, as measured.
- One sentence left the page, because the status board already says it in more detail: that the roadmap is
  split into task trees and the discipline rules are enforced. The link to the status board stays.
- Still to do (`PROGRAM.35.2`): the policy also asks that every place the README sends detail is itself kept
  under control, so a size limit here cannot just push the growth into a neighbouring file. The changelog is
  one such place.
- This commit also adds the log rows the previous commit, the fourth artifact cleanup, left out of its task tree.

## archogen — the fourth artifact cleanup: 2.2 GB of build residue released

`ARCHOGEN-PROGRAM-0196` (leaf `PROGRAM.19`, fourth run under the standing owner).

- The build directory had grown from under 1 GB to 6.3 GB in a day. Most of it was the source and build tree left
  behind when the CI provisioner compiled the pinned emulator (1.7 GB), and the CI rehearsal's throwaway checkout
  (517 MB). Neither is read once the emulator is installed. Both, the doctrine gate's scratch and the finished
  leaves' probe files were deleted: 6.3 GB → 4.1 GB.
- Each deletion was checked first. Every scratch name was looked up in the tracked tree, and the leaf that made it
  was read. The one probe directory whose leaf is still open (`M1.29`) was kept. So were the tools, the caches and
  the build outputs the tiers use. The debug cache's growth was measured, and it is cargo's normal retention.
- Verified from cold: the emulator and book tool are still in place, the focused tier passes, and the whole suite
  reports 742 passed and 0 failed over 62 suites.

## archogen — how the engine's knowledge is recorded, designed and under review

`ARCHOGEN-M2-0195` (leaf `M2.7.1`, a checkpoint).

- The engine will keep what it knows as a **catalog**: how its scheduler behaves, what a context switch costs, how
  a timer fires. There is now a written design for a catalog entry, covering:
  - what it records;
  - how it is fingerprinted;
  - who has checked it;
  - when it may back a claim someone will rely on.
- "Checked" cannot go stale unnoticed. An entry's reviewed status is worked out from reviews tied to the exact
  content they saw, never typed in. Change one line, of the entry or of the code under it, and the review no
  longer applies. A rejection stays until someone answers it.
- The design was checked by independent reviewers, round after round. Each round found real holes, and the
  design was reworked each time. The next round is still to run.
- One consequence goes back to the timing analysis. Four of its inputs mix engine, application and build
  knowledge, and no single owner can supply them. They stay with the caller for now, and a new step is filed to
  work out how to put them together without under-counting.

## archogen — the first program runs on the emulated board

`ARCHOGEN-M2-0194` (leaf `M2.8.4`).

- A tiny program now runs on the emulated RISC-V board. It starts up, sets a timer, takes the timer interrupt and
  returns from it, checks that none of the values it was holding were disturbed, prints each step, and switches
  the machine off, which tells the emulator it passed.
- It proves its own check can fail. A deliberately broken copy, which spoils one value on the way back from the
  interrupt, must be caught, and is caught every time it runs.
- It is now a step of the full verification tier, which passes 10 of 10. This completes the emulator part of the
  step. Running on a real board waits for a board.

## archogen — the emulated board is verified, and the full verification tier passes

`ARCHOGEN-M2-0193` (leaf `M2.8.3.4`).

- The emulated RISC-V board is now marked verified. The pinned emulator presents exactly the hardware
  description on record, and that description agrees with the board's eADL description. Every check repeats all
  of this on a fresh reading.
- The last open gap in the full verification tier is closed. It now passes outright: nine steps passed, none
  set aside.
- Flipping the flag exposed a hole, now closed. "Verified" could be claimed without the comparison having run.
  The check now refuses that, and its own tests cover it.
- The pages that still said the emulator was missing or unverified are corrected, from real runs.

## archogen — the board's compiler target is written down where the board is

`ARCHOGEN-M2-0192` (leaf `M2.8.3.3`).

- The Rust target the runtime is built for, and the instruction-set extensions it assumes, were written only
  inside the verification tool. They now sit in the target's own settings file, the one place a reader looks, and
  the build step reads them from there.
- Tests make sure nothing else declares them again, and that the compiler toolchain file installs that target.

## archogen — the emulated board is described in eADL, and matches what the emulator reports

`ARCHOGEN-M2-0190` (leaf `M2.8.3.2`).

- The emulated RISC-V board now has its eADL description, `targets/riscv-virt-up.eadl`, with one core, a timer,
  a console, and memory to run from. archogen's own checker accepts it.
- A new check compares each of those facts with the hardware description the emulator reports, matching each by
  what the device is. It runs every time the emulator is checked, on a fresh reading, and today everything matches.
- The target is not yet marked verified. That flip is the next step but one, with this run as its evidence.

## archogen — how the first real target is described and checked, decided

`ARCHOGEN-M2-0189` (leaf `M2.8.3.1`).

- The emulated RISC-V board archogen targets will be described in eADL, in the words the language already has.
  It states one fact for each thing the supported profile requires: one core, a timer, somewhere to print, and
  memory to run from.
- A check will compare each fact with the hardware description the emulator itself reports, matching each by what
  the device is, not just its address. The target is marked verified only when that check passes on a fresh
  reading.
- It claims nothing it cannot check. The instruction set, the interrupt timings and the unused devices stay out.

## archogen — the real-time analysis refuses what it cannot know (F17)

`ARCHOGEN-M2-0188` (leaf `M2.6.4`).

- The roadmap's case F17 is now checked. Take a system the analysis accepts, then break one thing at a time: an
  interrupt nobody described, a cost nobody knows, interrupts that nest, a task that waits on itself or locks the
  scheduler. The analysis refuses every one with a stated reason, and never quietly gives a weaker answer.
- The simpler analysis that ignores overheads cannot be mistaken for the real one: its result names a different
  model and always says "no overhead".
- This completes the runtime timing analysis step. What it still needs before it can be cited for a real system
  is the catalog of real costs and platform facts, the next step.

## archogen — the real-time analysis agrees with results worked out independently

`ARCHOGEN-M2-0187` (leaf `M2.6.3`).

- Someone who never saw the code worked out, from the design alone, what the analysis should answer for 18 test
  cases. The code agrees with every one, down to each intermediate step of the calculation.
- On the roadmap's repeated-preemption example, the analysis bounds the high-priority task at 9 (the real timeline
  gives 5) and the low-priority task at 49 (really 23). It is safe and pessimistic, as intended, and it does not
  claim the low-priority deadline is met.
- The independent derivation also found four small gaps in the design text, now closed, and settled one of its
  open questions.

