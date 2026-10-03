# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the catalog's checker, rehearsed through CI's own harness

`ARCHOGEN-M2-0389` (leaf `M2.7.4.2`, closed).

- The CI job that had failed closed for want of a checker now passes on the empty catalog: rehearsed from a clean
  checkout exactly as the workflow runs it, the checker built from the base commit and run on the judged tree.

## archogen — the catalog's checker

`ARCHOGEN-M2-0388` (leaf `M2.7.4.2`, step 1).

- The catalog now has its checker: a tool that reads the repository's history from git exactly as the design
  prescribes, loads the catalog at the commit being judged, and replays the lock against the commits it builds on. It
  judges the commit about to be made, a commit already made, or a pull request's merge against its base. On this
  repository's empty catalog it passes; in test repositories it refuses a record changed without a version bump, a
  removed review and a hand-edited lock, each with its code.

## archogen — the gate's three toolchain claims, measured

`ARCHOGEN-M2-0387` (leaf `M2.7.4.1`).

- The catalog's gate rests on three things the pinned toolchain does: the compiler lists every source file it read,
  `rustc --print sysroot` points at the toolchain's own files, and Cargo and rustup find their configuration files
  only above the working directory and in `CARGO_HOME`. Each was written down and never measured. Each is now
  measured on the pin and re-measured, with a control, on every integration run.

## archogen — what a trace shows on each fault path: decided

`ARCHOGEN-M2-0385` (leaf `M2.15`, closed).

- The record of which events each fault path produces, and when a trace from the host and one from the target agree
  on them, is decided after seven independent reviews. The host run is fed what the target observed, and the
  comparison keeps to what is the fault paths' own; how the comparator replays and decides is left to the leaves
  that build it. One follow-up is filed: the runtime crates' documentation still quotes superseded contract text.

## archogen — the book explains the port's record

`ARCHOGEN-M2-0377` (leaf `M2.12.5`, closing `M2.12`).

- The catalog chapter now explains the port's record in plain words first: why the port is written in the
  processor's own language, where that code may be and what it may say, what the port must declare about faults, and
  what the format relies on in the toolchain. Two words join the glossary. The work on the port's record — its format,
  its statement, the checks and the book — is complete.

## archogen — the toolchain's premises, checked on every integration run

`ARCHOGEN-M2-0376` (leaf `M2.12.4.4`, closing `M2.12.4`).

- What the port's record format rests on in the pinned toolchain is now checked on every integration run, not only
  measured once: how the compiler aligns and names the port's functions, how the assembler reads labels and
  integers and expands its instructions, and what a panic does. Each experiment the record prints is read from it at
  its hash. A toolchain bump that changes any of these fails before a record can rest on it. With this, the
  catalog's support for the port's record is complete.

## archogen — the catalog checks what the port declares about faults

`ARCHOGEN-M2-0375` (leaf `M2.12.4.3`).

- The catalog now holds the port's record to its statement: every fact it owes about faults is there, the ones the
  fault contract requires are never `no`, the facts that only make sense under a condition appear only where it
  holds, and the port and every record selected with it agree on one convention for reporting what a failed check
  found.

## archogen — the catalog reads the port's assembly

`ARCHOGEN-M2-0374` (leaf `M2.12.4.2`).

- In a package declared as assembly, the catalog's scanner now admits the port's assembly, and only in the narrow
  form the design allows: written out in full, inside a function built only for the bare-metal target, each line one
  instruction from a short list with each operand of the right kind, every register an inline block touches
  declared, and every jump to code going through a name the compiler resolves.

## archogen — the catalog reads the port's record form

`ARCHOGEN-M2-0373` (leaf `M2.12.4.1`).

- The catalog crate now reads a record the way the port's record will be written: a fact about code may point at
  several places in it, an implementation may declare which of its packages hold assembly, and a known fact about
  the port's code is admitted only when it points into such a package, on a target the assembly dialect runs on.
  The assembly itself is still refused until the next step teaches the scanner to read it.

## archogen — what the port must declare: decided

`ARCHOGEN-M2-0371` (leaf `M2.12.3`, step 8, closed).

- The seventh independent review found no defect, so what the architecture port's catalog record must declare about
  faults is decided: the facts it states, the costs of its fatal path, and the convention by which a failed check
  reports what it found, matched across records. Its remaining wording points are answered in the same change.
  Building it into the catalog crate comes next, before the catalog's first lock.

## archogen — what the port must declare: the sixth review

`ARCHOGEN-M2-0370` (leaf `M2.12.3`, step 7).

- The sixth review found one defect: a rule the previous round corrected in one place was still stated the old way
  in another. Both now say the same thing. The new checks the port's record must pass are now scheduled to be built
  before the catalog's first lock is written, so no record admitted early is refused later. The runtime annex now
  says a stack's guard may be noticed by a fault or by a check. A seventh review is next.

## archogen — what the port must declare: the fifth review

`ARCHOGEN-M2-0369` (leaf `M2.12.3`, step 6).

- The fifth review found every earlier fix still in place and two defects: one wrong input refused under two codes,
  a slip the previous round's fix introduced, and a history line that left out the second amendment. Each is
  corrected in a sentence. The port must now also say how each guarded stack's overflow is noticed. A sixth review is
  next.

## archogen — what the port must declare: the fourth review

`ARCHOGEN-M2-0368` (leaf `M2.12.3`, step 5).

- The fourth review reproduced every experiment exactly and found three defects, two introduced by the previous
  round's own fixes: a definition that turned an "or" into an "and", and a quotation that narrowed a statement too far.
  Both are corrected with single sentences, and two refusals are filed where the catalog's rule list shows them. A
  fifth review is next.

## archogen — what the port must declare: the third review

`ARCHOGEN-M2-0367` (leaf `M2.12.3`, step 4).

- The third review confirmed the design's core and found four defects, among them experiment sources the design said
  were recorded when only their fingerprints were. The sources are now printed in full, and each fingerprint was
  re-derived from the printed text. A fourth review is next.

## archogen — what the port must declare: the second review

`ARCHOGEN-M2-0366` (leaf `M2.12.3`, step 3).

- The second review found the cross-record check sound, and six defects in the surrounding text, the sharpest a claim
  that processor code here cannot trap on purpose, which a single load from an empty address disproves. The claim is
  gone, a statement for such traps is added, and the rules on what the port must state are tightened. Every compiler
  fact the design relies on is now re-measured from recorded sources. A third review is next.

## archogen — what the port must declare: the first review

`ARCHOGEN-M2-0364` (leaf `M2.12.3`, step 2).

- The first review found the cross-record check broken in its own mechanism: two records naming the same convention
  collided with the catalog's one-supplier rule, and a name alone bound no meaning. A convention is now a catalog
  record of its own, which the port and every record that reports faults must depend on, so its meaning is hashed and
  a change to it is noticed. Several gaps in coverage are closed, and the fault contract's wording on the panic handler
  is made consistent throughout. A second review is next.

## archogen — what the processor port must declare

`ARCHOGEN-M2-0363` (leaf `M2.12.3`, step 1).

- The fault-handling contract leaves about twenty questions to the processor port: how it catches misuse, what its
  traps do, how a failed check reaches the fatal path, how long that path takes. The catalog design now says how the
  port's record answers each, as a yes-or-no statement explained and pointed at the code, with one of them checked
  automatically across records. Two facts about the compiler were measured first. A first independent review is
  next.

## archogen — the design for holding processor assembly is settled

`ARCHOGEN-M2-0360` (leaf `M2.12.2`).

- The seventh independent review of how the catalog may hold a processor port's assembly found nothing wrong, so the
  design is settled. Over seven reviews, each compiling small experiments with the project's own compiler, the
  problems found went 8, 2, 2, 1, 3, 1 and then 0, every answer narrowing what is admitted. The checker keeps refusing
  all assembly until the design is built and tested; next is how the port states the facts the fault contract leaves
  to it.

## archogen — the assembly design's sixth review: a quotation restored

`ARCHOGEN-M2-0359` (leaf `M2.12.2`, step 7).

- The sixth review found no way past the design, but one quotation an earlier tidy-up had removed while still claiming
  it was there, and one more thing for the human reviewer to check when hand-written code calls a function. Both are
  fixed. Defects found per review: 8, 2, 2, 1, 3, 1. A seventh review is next.

## archogen — the assembly design's fifth review: words that had drifted

`ARCHOGEN-M2-0358` (leaf `M2.12.2`, step 6).

- The fifth review found nothing the compiler does wrong with admitted code, but three places where the design's own
  words had drifted out of true in earlier answers, and one way a generic function could reach code the port does not
  hold. The words are restored and generic functions are refused that reach. A sixth review is next.

## archogen — the assembly design's fourth review: two names, one register

`ARCHOGEN-M2-0357` (leaf `M2.12.2`, step 5).

- The fourth review found one more way the compiler and the design could disagree: two inline-assembly values the
  design treats as separate can be given the same register by the compiler, so a value written early is what a later
  line reads. That form of output is now refused. Defects found per review: 8, 2, 2, 1. A fifth review is next.

## archogen — the assembly design's third review: what the compiler assumes

`ARCHOGEN-M2-0356` (leaf `M2.12.2`, step 4).

- The third review found that a short piece of inline assembly could change a register without saying so, and the
  compiler, trusting the declaration, would then jump to whatever number was left there. Inline assembly is now
  limited to the system-register work a port does inline, with every register it touches declared; full assembly
  stays in separate hand-written functions whose register discipline the human reviewer checks. A fourth review is
  next.

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

