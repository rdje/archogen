# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the MCP server holds up against a hostile client

`ARCHOGEN-API-0337` (leaf `API.6.5`).

- An outside review attacked the new server and found it could be stalled for minutes by one large request, refused
  some valid descriptions because of how a client escaped them, and answered some errors in a form the protocol
  forbids. All fixed and tested. One finding — a pathological description producing an enormous answer, in the
  command line too — is the engine's, and is next.

## archogen — the fault contract's fourteenth reading

`ARCHOGEN-M2-0336` (leaf `M2.9`).

- The fourteenth reading found two gaps: when exactly a job finishes if its last step is a call into the runtime, and
  when a runtime call first counts as having changed anything. Both are settled. Five checks before landing kept
  finding the same kind of slip in the list of test cases the contract asks for; the open question of which boards
  can run which test is now left to the test suite that will know the boards.

## archogen — the fault contract's thirteenth reading

`ARCHOGEN-M2-0334` (leaf `M2.9`).

- The thirteenth reading found one gap, in last round's answer about calls into the runtime that name no real
  operation: whether an interruption before the call is recognised may simply drop it. It may, and nothing is
  reported. The check before landing found nothing to fix for the first time this round.

## archogen — an AI agent can now drive archogen

`ARCHOGEN-API-0333` (leaf `API.6.4`).

- `archogen mcp` starts a server that speaks the Model Context Protocol, the standard way AI assistants call tools.
  An agent can list what archogen offers and ask it to check a description, and gets the same verdict, byte for
  byte, as the web build and the same pass or refusal as the command line. Tools not built yet say which piece of
  work will build them. Both the current protocol and the one before it are spoken.

## archogen — the MCP server can read and write JSON

`ARCHOGEN-API-0332` (leaf `API.6.3`).

- The coming MCP server will read requests from whatever program drives it, so it gets its own small JSON reader
  rather than a borrowed one: it accepts exactly the JSON standard, refuses anything oversized, nested too deep,
  malformed, or naming the same field twice, and says where it stopped. Its writer has one way to write each value.
- A formatting slip in an earlier commit had made the quick verification tier fail; it is fixed.

## archogen — the fault contract's twelfth reading

`ARCHOGEN-M2-0330` (leaf `M2.9`).

- The twelfth reading found one gap, older than the last round: a call into the runtime that names no real
  operation had no owner for the moment before it is recognised. It is now the caller's, and treated like any other
  runtime call that never returns. Two of the three checks before landing each caught the previous fix adding a rule
  that clashed with another; what landed adds none.

## archogen — the workflow gate refuses three more ways to be misread

`ARCHOGEN-M2-0329` (leaf `M2.7.6.4`).

- The gate that keeps every CI workflow on a read-only token now refuses a key written twice in one place, two
  keys that differ only in case — GitHub upper-cases an action's input names, so both would land in one setting —
  and a quoted key. The second outside review of the check's protection could not be completed by an agent, and
  is recorded as such.

## archogen — the book's index lists topics, not layers

`ARCHOGEN-PROGRAM-0326` (leaf `PROGRAM.47.6`).

- Every chapter now opens with the same four headings — the idea in plain words, how it works, the precise rules,
  today and ahead — and the index had listed each of them once per chapter. It now leaves them out, so what remains
  is topics: 270 entries instead of 342.

## archogen — the fault contract's eleventh reading

`ARCHOGEN-M2-0319` (leaf `M2.9`).

- The eleventh reading found one gap: when a fault caught by the processor itself enters the fatal handler, and
  what is kept if a second fault strikes on the way there. Each fault now has a defined moment at which it is raised.
  The answers were checked six times before landing, and what held was the simplest: leave to each board port what
  only its design can say.

## archogen — the fault contract's tenth reading

`ARCHOGEN-M2-0309` (leaf `M2.9`).

- The tenth reading found two small gaps in how a failed check is classified, one of them in a fix applied without
  being read again; both are closed in a sentence each, and every new sentence is now read before it lands.

## archogen — the runtime chapter, written for a newcomer and an expert

`ARCHOGEN-PROGRAM-0301` (leaf `PROGRAM.47.4`).

- The chapter on the runtime now starts in plain words — a cook with a row of orders, each due by a time — then
  gives engineers a one-minute summary and the precise rules. The case-by-case details and their history moved
  to the book's first annex, and a stale claim about the bare-metal build was corrected.

## archogen — the fault contract's ninth reading finds one defect

`ARCHOGEN-M2-0299` (leaf `M2.9`).

- The ninth independent reading found a single remaining ambiguity, down from a dozen in the first rounds, and
  judged the contract's hand-offs to each board port sound. The answer simplified the most-patched passage into a
  plain list, and the check before landing caught one more: an application's own checks are now judged by how
  they end, since no port could tell what they meant.

## archogen — the book has an index

`ARCHOGEN-PROGRAM-0298` (leaf `PROGRAM.47.3`).

- The book now ends with an index: every abbreviation and term with the chapters that use it, and every section of
  every chapter, linked. It is written by a script from the chapters themselves and checked on every change, so it
  is never out of date; each of its links was checked against the built book.

## archogen — the book has a glossary that cannot fall behind

`ARCHOGEN-PROGRAM-0297` (leaf `PROGRAM.47.2`).

- The book now ends with *Words this book uses*: every abbreviation it uses, from API to xRET, spelled out and
  explained, and the project's everyday-looking words that mean something precise. A check refuses any change that
  uses an abbreviation the glossary does not explain, or leaves an entry nothing uses.

## archogen — the book will be written in layers

`ARCHOGEN-PROGRAM-0296` (leaf `PROGRAM.47.1`).

- On the director's ruling, every chapter of the book will open in plain words a student can follow, offer a
  one-minute summary for engineers, then give the precise rules; the technical depths move to annexes, and a live
  glossary of acronyms and terms and a generated index frame the chapters. The ruling is recorded; the chapters
  follow one at a time.

## archogen — the fault contract answers its eighth reading, and the book catches up

`ARCHOGEN-M2-0295` (leaf `M2.9`).

- The eighth reading found two remaining ambiguities. One is answered by removing a case: a call into the runtime
  through a trap never handles an interrupt itself. The other, how a program's own failed check (a panic) is
  classified, took five attempts: each version that tried to dictate the few instructions before the panic handler
  was found ambiguous for some board port design. The contract now fixes what holds for every port and leaves that
  window to each port's own reviewed record.
- The book's runtime chapter had fallen behind three of the contract's rulings; it now matches, and the fixture
  list for faults lives in the contract, with the roadmap pointing to it.

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

