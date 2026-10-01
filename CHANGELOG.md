# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the MCP server's design, read at the source

`ARCHOGEN-API-0293` (leaf `API.6.1`).

- Before building the server that lets AI agents drive archogen, the protocol was read at its source: its current
  revision, from July 2026, changed how a client and server meet, and older clients cannot talk to a server that
  only speaks the new one. The design answers both, offers exactly the operations the command table declares, and
  never lets a refused description read as a success.

## archogen — an attack review closes two ways past the workflow gate

`ARCHOGEN-M2-0292` (leaf `M2.7.6.4`).

- An independent reviewer attacked the catalog check's protection for real and found two ways a workflow could
  hold a writable token unseen by the gate: a YAML key written with an escape code, and a setting placed on a
  neighbouring step. Both are now refused, each with a test that fails if the protection is removed. The check's
  harness and workflow held; they gained two smaller hardenings.

## archogen — the fault contract answers its seventh reading, checked before it lands

`ARCHOGEN-M2-0290` (leaf `M2.9`).

- The seventh reading found three remaining ambiguities, among them what resumes a task's half-finished completion
  after an interrupt, and which task a fatal fault's record names when a burst of held-back interrupts arrives
  as several traps. Because six readings in a row found most of their problems in the previous answers, the
  answers were checked by a separate reader before landing; it found four more, fixed in the same change.

## archogen — the catalog check has its workflow

`ARCHOGEN-M2-0289` (leaf `M2.7.6.3`).

- A pull request now has a CI job that will run the catalog's check: it takes the main line's side of the merge
  to build and run the checker, and only reads the pull request's side. Until the checker is written the job
  fails, never passes; making the hosting require it is the director's setting.

## archogen — the catalog check builds its checker where a pull request cannot reach

`ARCHOGEN-M2-0288` (leaf `M2.7.6.2`).

- The script that will run the catalog's check builds the checker from the main line's own files, never from the
  pull request being judged, in a clean environment with the pinned compiler, and runs it from a place no other
  build can write. Its tests plant hostile build scripts, compiler wrappers, toolchain files and environment
  variables and show that none of them ever runs; the checker itself is still to be written.

## archogen — the fault contract answers its sixth reading

`ARCHOGEN-M2-0287` (leaf `M2.9`).

- The sixth independent reading found three remaining ambiguities, all small and all in sentences written to
  answer the fifth: which task a fatal fault's record names when the scheduler returns to the task it interrupted,
  or a held-back release turns out late, and what happens to a finishing task that is taken out of the schedule
  while it finishes. Each answer matches what the runtime's code already records; neither runtime model changed.

## archogen — no workflow can write to the repository

`ARCHOGEN-M2-0285` (leaf `M2.7.6.1`).

- Every CI workflow now runs with a read-only token and leaves no credentials in its checkout, and a new gate
  refuses a workflow that grants a write permission, keeps the token, or uses a trigger that runs with the main
  repository's token on code a pull request controls. It is the first piece of protecting the catalog's check,
  whose verdict nothing a pull request controls may post; the hosting's own settings remain the director's.

## archogen — the fault contract answers its fifth reading

`ARCHOGEN-M2-0284` (leaf `M2.9`).

- The fifth independent reading found five remaining ambiguities, four of them only in which task a fatal fault's
  record names. The answers say what runs in the instant after a task's last instruction, when a board delivers a
  release held back by a critical section, and where interrupt entry ends and an interrupt's own handler begins;
  each was checked against the runtime's code first, and neither runtime model needed to change. The review loop
  now has its stopping rule: the first reading that finds no defect closes it.

## archogen — a commit that forgets its task-log row is refused

`ARCHOGEN-PROGRAM-0283` (leaf `PROGRAM.46`).

- Every commit that names its unit of work must now have a row in that work's task log, and a commit that forgot
  one is refused before it lands. Twice today a commit landed without its row and was caught only by hand; the
  eighteen older commits without a row are listed and may only be worked off, never added to.

## archogen — the tour says how you interact with a generated system

`ARCHOGEN-PROGRAM-0282` (leaf `PROGRAM.45`).

- The book's tour now answers the questions a newcomer asks next: you drive a generated system through declared
  inputs — a button, a sensor line, a byte on the serial port, each waking a task that must respond in time — watch
  its declared outputs, and look inside with a debugger. There is no login, shell or filesystem in this first
  profile, and the chapter says why and where they sit on the roadmap.
- What runs today is kept apart from what is ahead: event-driven input and a debugger on the emulator are still to
  come.

## archogen — the fault contract answers its fourth reading

`ARCHOGEN-M2-0281` (leaf `M2.9`).

- The fourth independent reading found nine remaining ambiguities, three of them in text written to answer the
  third. The answers now state what a board port must achieve and leave how to the port's own reviewed record, which
  is where the new ambiguities kept appearing; the three mistakes are corrected, each checked against the runtime's
  code before landing. Neither runtime model needed to change.

## archogen — the independent model accepts every priority the language does

`ARCHOGEN-M2-0279` (leaf `M2.9`).

- The independent runtime model now takes any positive 64-bit priority, as the language and the runtime already
  do, and the two are compared at the largest one. Rewriting it to the latest contract changed none of its
  behaviour, and it independently spotted one ambiguity the fourth review also reported.

## archogen — the book opens with a tour of what archogen is for

`ARCHOGEN-PROGRAM-0278` (leaf `PROGRAM.44`).

- A new chapter after the introduction follows one small description — two periodic tasks and a serial console —
  to the program generated from it and the schedule it prints, then says plainly what is real today, what the
  generated system becomes, what it can be used for, and the road to running on a microcontroller board.
- Today and tomorrow are kept apart: the running example is the experimental path, and the board-level program on
  the emulator is a hand-written measurement, not yet a generated system. The chapter's copies of the description
  and of its output are checked against the files on every test run.

## archogen — the fault contract answers its third reading

`ARCHOGEN-M2-0277` (leaf `M2.9`).

- The third independent reading of the runtime's fault contract found twelve places where two careful implementers
  could still build different systems, most of them where the contract meets the board port: when a task's job
  counts as finished, which traps are unexpected, how an interrupt controller's empty claim is treated, who may open
  a critical section and how a port notices when the wrong code does. Each is now decided in the text; neither
  runtime model needed to change.
- What the contract narrowed, such as when a missed deadline is reported, is marked for the director's review.

## archogen — the runtime's fault contract leaves the roadmap

`ARCHOGEN-M2-0276` (leaf `M2.19`).

- On the director's ruling, the detailed rules for how the runtime classifies, blames, contains and escalates faults
  moved out of the roadmap into the profile's own specification, unchanged. The roadmap keeps the requirement and a
  one-table summary, and stops changing every time the contract's reviewers find something; it shrank by about 130
  lines.

## archogen — any valid priority can reach the runtime

`ARCHOGEN-M2-0275` (leaf `M2.18`).

- The runtime's entry point for priorities took only numbers up to 65 535, while the language accepts any positive
  64-bit integer. It now takes the language's own integer, so every priority that passes the check can be built.

## archogen — a system with no tasks: valid to describe, refused to build

`ARCHOGEN-M2-0274` (leaf `M2.17`).

- A description whose system declares no task stays valid to check — the book's opening example composes a platform
  and services that way — and building one is refused, as it already was. The contract now says which step refuses
  it, instead of implying the check should.

## archogen — only a running task opens a critical section

`ARCHOGEN-M2-0273` (leaf `M2.9`).

- The runtime now halts if a critical section is opened or closed while no task is running, and records that no
  task is to blame. The independent model was re-derived to the same text, and the two now agree everywhere they
  are compared, the one case where they used to differ included.

## archogen — the runtime contract answers its second review

`ARCHOGEN-M2-0272` (leaf `M2.9`).

- The contract now says exactly when a late task is stopped — never while it holds a critical section — that the
  profile detects overruns by releases alone, with no budget or deadline monitor, which traps count as unexpected,
  and what a fatal fault's record keeps. It also lists, in full, which of today's changes altered either model.
- One rule follows in the runtime next: opening or closing a critical section when no task is running halts.

## archogen — the runtime contract's third reading, triaged

`ARCHOGEN-M2-0271` (leaf `M2.9`).

- A second reviewer who read only the contract found the first round's answers mostly sound, and 28 more points,
  11 of them real ambiguities. The largest was this morning's own addition: an execution-budget monitor the
  contract allowed but nothing in the system provides. It comes out of this profile, for the director's review.
- Two more places where the language check accepts what the runtime refuses were measured and filed: a system
  with no tasks, and priorities too large for the runtime to hold.

## archogen — a task's overrun policy must be one the runtime performs

`ARCHOGEN-M2-0270` (leaf `M2.14`).

- `archogen check` now refuses an `on-overrun` policy the runtime cannot perform. Two are admitted: `fault`, which
  is also what a task gets when it declares none, and `skip-late-job`. Before, any word was accepted and only the
  runtime would have had no idea what to do with it.
- Written down as a correction to `eadl/1`, with two new worked cases; no existing description's verdict moved.

## archogen — the rulings' outside precedents checked at their sources

`ARCHOGEN-M2-0269` (leaf `M2.16`).

- The runtime rulings cited how FreeRTOS, OSEK and AUTOSAR behave, from memory. Each was read in the published
  source and recorded with its exact wording. FreeRTOS was right; OSEK was misstated — it discards an activation
  beyond a task's limit and reports it — and AUTOSAR's behaviour is error recovery rather than a normal path. The
  records are corrected; no ruling changes, because none depended on those precedents alone.
- The two documents the target still lacks, the QEMU `virt` machine's and the RISC-V calling convention, are public,
  and will be read and recorded by the work that first relies on them.

## archogen — the runtime keeps the first fatal fault and stops there

`ARCHOGEN-M2-0268` (leaf `M2.9`).

- When the runtime hits a fatal fault it now keeps one record of it: what happened, which task it is blamed on,
  which task it interrupted, and whether a critical section is what made it fatal. After that, nothing changes:
  later events are answered with the same record.
- A fault in an interrupt handler or the idle loop is no task's, and the task it interrupted is named as such.
- Nesting critical sections past the limit, closing one that was never opened, or starting a task inside one now
  halts, instead of being refused and leaving the system one section out of balance.
- The independent model was re-derived again from the contract, and the two still agree on every randomised
  sequence, now including these faults.

## archogen — the runtime contract says what the runtime does

`ARCHOGEN-M2-0267` (leaf `M2.9`).

- The fault-handling contract was rewritten so that a reader of the text alone gets the behaviour both
  implementations share: what the two overrun policies do, how releases that arrive during a critical section are
  judged, whom each fault is blamed on, and that a fatal fault halts everything and keeps the first cause.
- Three of its rules are new to the runtime and are being carried into it next: nesting critical sections too deep,
  or closing one that was never opened, now halts rather than being refused; a fault in an interrupt handler is
  no task's; and a halted runtime changes nothing afterwards.
- One is for the director's review: the runtime does not watch deadlines; a missed one shows as an overrun at the
  task's next release.

## archogen — the runtime contract's second independent review, triaged

`ARCHOGEN-M2-0266` (leaf `M2.9`).

- A reviewer who read only the written contract, and neither implementation, found it not yet sufficient on its
  own: several rules both implementations follow were written in the decision records rather than in the
  contract, and a few were written nowhere. Each of its findings now has an answer and a step that carries it out.
- Two came out as work of their own: the language accepts any overrun policy, even a made-up name, and the
  events the runtime's fault paths produce in a trace are still to be defined.

## archogen — a task priority below 1 is refused when the description is checked

`ARCHOGEN-M2-0265` (leaf `M2.13`).

- `archogen check` now refuses a task with `(priority 0)` or a negative priority, saying that a priority is a
  rank from 1, the highest. Before, such a description was accepted and only failed later, when the runtime
  refused to build it. Priorities with gaps, such as 1, 5 and 9, stay valid.
- The language's meaning does not change: a rank below 1 never had one. The change is written down as a
  correction to `eadl/1`, with a new worked case, and no existing description's verdict moved.

## archogen — the runtime and its independent model agree on the amended contract

`ARCHOGEN-M2-0264` (leaf `M2.9`).

- The runtime now follows the rulings: a task released twice inside a critical section has its overrun policy
  applied when the section ends, and a job that finishes while holding the interrupt mask releases it. The
  independent model was re-derived from the written contract alone, and the two agree on every randomised event
  sequence, which now also covers overruns, critical sections ended by a completion, and fatal faults. Reverting
  any ruling in either one makes the comparison fail.
- Rewriting the comparison caught three runtime defects, now fixed: a trap was blamed on no task instead of the
  running one; an overrun reported by a budget monitor started a job nobody released, and a skipped job looked like
  an ordinary release; and a second fault could overwrite the first one's evidence.
- Task priorities may now have gaps, such as 1, 5 and 9, since only their order matters. The language check still
  accepts priority 0, which the runtime refuses; that is filed to be fixed next.
- The work landed on the main line; the side branch it had been parked on is removed.

## archogen — the runtime contract's two open questions are ruled

`ARCHOGEN-M2-0263` (leaf `M2.9`).

- With the director's delegation, the two behaviour questions the runtime contract's review left open are decided.
  A task's release that arrives twice inside a critical section is no longer fatal for its timing alone: the
  second is kept, and the task's own overrun policy applies when the section ends, as it would one instruction
  later. A job may finish while holding the interrupt mask it took; finishing releases it.
- The work that carries these into the runtime and its independent reference model resumes on the main line,
  where it lands once reviewed; the side branch it was parked on goes.

## archogen — the catalog crate is complete: premise 3 judged at claim time

`ARCHOGEN-M2-0261` (leaf `M2.7.3.6.2`).

- A production result now checks the protection of the main line it rests on: that it holds from the commit the
  director names, that every change since arrived as a merge the hosting made and signed, that the tooling was built
  from that protected line, and that the checker has not changed since. Until the director turns the protection on,
  every production result says so.
- With this, the catalog crate is complete: records, hashes, the lock, evidence status, invalidation, lookups, the
  production namespace, claims and the package rules, every rule held by a test that fails without it.

## archogen — catalogued code is held to the package rules

`ARCHOGEN-M2-0260` (leaf `M2.7.3.7`).

- Code a catalog component points at can no longer carry assembly, file inclusion, linker controls, macros of its
  own, extra build configurations or compiler flags: every package a component reaches is checked, its manifests
  read by their meaning and its Rust source scanned token by token, so a word in a comment or a string is fine and
  the same word as code is not.
- The runtime core passes, as the design measured.

## archogen — accepted designs move to their own home

`ARCHOGEN-PROGRAM-0259` (leaf `PROGRAM.43`).

- The decisions folder had about 4 KB left under a ceiling only the director can raise. On the director's ruling,
  the two accepted catalog designs moved to `docs/specs/catalog/`, a home for specifications that code is built
  against, and the folder went from 388 884 to 205 397 bytes without raising anything.
- Every link follows the records, the decisions index still lists them, and the ruling is recorded as a decision.

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

