# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the assembly design's second review: two readings that differed

`ARCHOGEN-M2-0355` (leaf `M2.12.2`, step 3).

- The second review confirmed the reworked design with the compiler's own output, and found two places where the
  design's wording could be read differently from how the compiler reads the code: unnamed placeholders, and labels
  written with a leading zero, which the assembler treats as octal. Both are now refused outright. A third review is
  next.

## archogen — the assembly design's first review: names that become links

`ARCHOGEN-M2-0354` (leaf `M2.12.2`, step 2).

- An independent reviewer compiled small experiments and showed that the draft let some instructions name things,
  such as a processor register, that the toolchain quietly turns into links to whatever code carries that name — the
  very kind of hidden link the design exists to rule out. The design now checks every operand by its position in its
  instruction, keeps a short list of instructions the port actually needs, and requires hand-written functions to
  end cleanly. A second review is next.

## archogen — the programmatic interface is complete

`ARCHOGEN-API-0353` (leaf `API.7`).

- The book's chapter on the programmatic interface now says, in one table, what each way in can do and what it gets
  back: the command line, the Rust library, the web page and the AI-agent server. All four check descriptions and
  always return the verdict a person would see; none can generate a system. With it, the programmatic-interface
  work is closed. One question remains with the director: whether a request archogen did not judge must still carry
  a formal verdict.

## archogen — a design for holding processor-specific code in the catalog

`ARCHOGEN-M2-0352` (leaf `M2.12.2`, step 1).

- The catalog's design now says how a record may hold the small amount of assembly a processor port needs: only in
  packages the record names, written so that every link to other code is visible to the checks, with a closed list
  of instructions and no assembler directives. It is a draft; independent reviews come next, and the checker keeps
  refusing all assembly until the design is settled and built.

## archogen — what the processor-specific code needs, measured

`ARCHOGEN-M2-0351` (leaf `M2.12.1`).

- Before designing how the catalog will hold the small amount of assembly code a processor needs, its needs were
  measured with the project's own compiler: what that code must do, what the compiler does with it, and what the
  language reference promises. The results, with the experiment that produced them, are in a new part of the
  catalog's design record; the design itself is next.

## archogen — the timing analysis's interrupt rule is settled

`ARCHOGEN-M2-0350` (leaf `M2.11`).

- The fifth independent review of the new interrupt rule found nothing wrong, so the work closes. Over five reviews
  the problems found went 5, 1, 3, 3 and then 0. The analysis now refuses a platform unless every interrupt it takes
  runs exactly one handler, paid for by a timer release or by an event, and it charges the pause before each handler.
  Its model is `fixed-priority-with-overheads/2`. Next is the format that lets the processor-specific assembly code be
  catalogued and reviewed.

## archogen — the timing analysis charges the pause between interrupts

`ARCHOGEN-M2-0349` (leaf `M2.11`, step 5).

- The fourth review found a cost the analysis had never charged: when one interrupt handler finishes with another
  interrupt waiting, the processor pauses for its delivery delay before the next, and a task set could miss a
  deadline the analysis said it met (237 against 224). Every handler is now charged that pause. No earlier result
  moves, since every checked example had a delay of zero. A fifth review is next.

## archogen — the timing analysis's rule: a third review catches a lost word

`ARCHOGEN-M2-0348` (leaf `M2.11`, step 4).

- The third review found that the last rewrite had dropped "of a declared source", which let a forgotten interrupt
  source reopen the missed deadline the first review found. It is restored, and two arguments that reached the right
  total by charging the wrong thing are corrected. A fourth review is next; the rule closes when one finds nothing.

## archogen — the timing analysis's rule, settled by a second review

`ARCHOGEN-M2-0347` (leaf `M2.11`, step 3).

- A second independent review found the corrected rule sound and no task set that breaks the bound, but one wrong
  argument and several unclear words. The rule is now simpler: every interrupt the system takes runs exactly one
  handler, paid for by a timer release or by an event. The words it relies on are defined. A third review is next.

## archogen — the timing analysis's new rule, corrected by its review

`ARCHOGEN-M2-0344` (leaf `M2.11`, step 2).

- An independent reviewer found the new rule still let through a case: an interrupt taken before the event it ends up
  serving, which a task set could turn into a missed deadline the analysis said would hold (33 against 27). The rule
  now looks at the moment the interrupt is taken, a stale timer interrupt is excluded too, and the analysis's model
  is renamed `fixed-priority-with-overheads/2` so no earlier result is mistaken for a current one. A second review
  checks these answers next.

## archogen — the CI protection's review needs a human reviewer

`ARCHOGEN-M2-0343` (leaf `M2.7.6.4`).

- The third review round of how the catalog's CI check is protected was stopped by the AI harness's own safety
  screening before it reported, as the second was. It is recorded as not completed, not as a pass. The next round
  is a reviewer the director names, and the project's other work moves ahead of it in the meantime.

## archogen — the timing analysis refuses interrupts nobody pays for

`ARCHOGEN-M2-0342` (leaf `M2.11`, step 1).

- The timing analysis charges each interrupt source once per event it receives. A review had found two ways an
  interrupt controller, as its specification and the emulator allow, can serve one event twice, or take an interrupt
  with no event at all; neither was charged. The analysis
  now refuses a platform that does not declare this never happens, and says so by name. Nothing it already
  concluded changes. An independent review of the change is next.

## archogen — the MCP server is done

`ARCHOGEN-API-0341` (leaf `API.6`).

- Any agent that speaks the Model Context Protocol can now start `archogen mcp` and ask it to check a description.
  Every promise made for the server was checked against a test or a record, and the full pre-push test run passed
  all eleven of its stages. What remains of the programmatic interface is its own book chapter (`API.7`).

## archogen — the fault contract is settled

`ARCHOGEN-M2-0340` (leaf `M2.9`).

- The fifteenth independent reading of the runtime's fault contract found nothing that two careful implementers could
  read differently, and the director approved the claims it narrows. Over fifteen rounds the readings found 13, 11, 12,
  9, 5, 3, 3, 2, 1, 2, 1, 1, 1, 2 and then 0 problems. Which of its test cases each future board must run is left to
  the test suite that will know the boards.

## archogen — an answer is never much larger than its question

`ARCHOGEN-API-0338` (leaf `API.6.6`).

- An error message quotes the line it points to; on a line a megabyte long, quoted once per error, a small request
  produced a gigabyte-and-a-half answer. Now a long line is quoted as a short window around the problem, so the
  command line and the server stay small and fast whatever they are sent.

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

