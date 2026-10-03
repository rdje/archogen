# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the trust-dependency gate is designed

`ARCHOGEN-M3-0407` (leaf `M3.6.1`, step 1).

- The roadmap asks that the program which generates a system and the programs that check it never come to share
  code or data unnoticed. The design for that check is written: each such program is built from scratch, every file
  its compiler read is recorded with a hash, and whatever two of them share must be accepted in a reviewed list by
  someone other than the person who changed it. Measured today, they share nothing. The first time they will is
  already known — when the scheduling checker starts reading the catalog — and that change will then wait for a
  reviewer, as the roadmap intends. The design goes to independent review before any code.

## archogen — the substitutability relation's ninth review, answered

`ARCHOGEN-M3-0406` (leaf `M3.1.1`, step 10).

- A ninth reader found two defects, both in how the relation treats the privilege a function demands. The previous
  round's advice for hardware where a higher privilege cannot reach a lower one's memory used a form the record
  elsewhere reads as "demand unknown", so it could not work; and a block declaring the privilege fact absent was
  read as demanding nothing at all. Both special cases are gone: the fact now means the lowest level from which a
  function is reachable, every higher level reaching it too — hardware that does not work that way states no value —
  and an absent privilege fact is absent like any other fact. Defects per round: 7, 7, 8, 3, 3, 3, 5, 1, 2.

## archogen — the substitutability relation's eighth review, answered

`ARCHOGEN-M3-0405` (leaf `M3.1.1`, step 9).

- An eighth reader found one defect: the record had worked an example in which one block offers a fact and another
  declares it absent, a description the language already refuses as contradictory before any matching runs. The
  record now says so, and says what can stand in for an absent fact within one description — a derivation from a
  provider's own facts — and what will once the catalog's records join. Ten smaller points answered in a sentence
  each. No probe made the relation read a demand as a capability or a value the wrong way. Defects per round:
  7, 7, 8, 3, 3, 3, 5, 1.

## archogen — the substitutability relation's seventh review, answered

`ARCHOGEN-M3-0404` (leaf `M3.1.1`, step 8).

- A seventh reader checked the shrunk design against the repository's one physical counter and found it could not
  describe itself: the target's timer is 64 bits wide, its modulus `2^64` is past what a 64-bit literal can write,
  and the previous round's rule forbade writing the horizon beside the rate instead. A count may now be written as a
  power of two, `(pow2 64)`, and the target's horizon — some 58 000 years — fits the arithmetic. The same reader
  found that a block declaring its reads tear, `(observation-coherent false)`, still met a service that merely
  depended on coherence; a boolean a service depends on now means `true`. Two restatements aligned, one word
  corrected against the kind definitions. Defects per round: 7, 7, 8, 3, 3, 3, 5.

## archogen — the substitutability relation's sixth review: the design shrinks

`ARCHOGEN-M3-0403` (leaf `M3.1.1`, step 7).

- For the third round running a reader found a way for an author-written horizon to slip past the guard meant for
  it — this time by leaving a sibling fact's digits out. Three rounds on one seam is the signal to stop patching:
  the guard, and the rule that guessed a counter's modulus from its width, are gone. A counter states its modulus,
  as the profile has always asked; a derived fact is either computed from its grounds or offered alone, never both;
  and there is nothing left for a sibling's spelling to switch off. Two restatements aligned. Defects per round:
  7, 7, 8, 3, 3, 3.

## archogen — the substitutability relation's fifth review, answered

`ARCHOGEN-M3-0402` (leaf `M3.1.1`, step 6).

- A fifth reader, asked to attack each timer case with a crafted offer, got through once: the previous round's
  guard against an author-written horizon hung on the derivation that computes it, and one word about the counter's
  wrap behaviour switched the derivation — and the guard — off. The guard is now a property of the facts themselves,
  whatever is said or left unsaid about wrapping, and a modulus or a horizon written for anything but a modular
  counter is refused outright. Two more seams closed: a platform's own requirements are now judged like a service's,
  and a fact offered without a value has one verdict everywhere. Defects per round: 7, 7, 8, 3, 3.

## archogen — the substitutability relation's fourth review, answered

`ARCHOGEN-M3-0401` (leaf `M3.1.1`, step 5).

- A fourth reader walked four of the corpus's descriptions through the relation by hand and found every verdict
  right, but also found that an author could write a counter's horizon outright and have it believed over what its
  width and rate allow — the one shape in which §5.2's faster-wrapping counter still slipped through. Now no offered
  value may claim more than the facts it is computed from allow, for every derived fact alike. Two restatements
  were aligned. Defects per round: 7, 7, 8, 3.

## archogen — the review histories' folder gets a larger ceiling

`ARCHOGEN-PROGRAM-0400` (leaf `PROGRAM.49`).

- The folder that keeps every design's review history reached the size it was allowed: it was sized for six
  histories and now holds eleven, and a history runs as long as its review takes, sixteen rounds for the catalog's
  record. A history is never edited once its review closes, so nothing there can be shortened. The ceiling rises to
  384 KiB, with the measurement and the rate written into a decision of its own, which also names the next step if
  the rate holds: an archive for closed histories, not another raise.

## archogen — the substitutability relation's third review, answered

`ARCHOGEN-M3-0399` (leaf `M3.1.1`, step 4).

- A third reader found eight defects. Two mattered most: the relation would have computed a counter's horizon and
  accepted it even where the platform had declared that horizon absent, and a block that demands machine privilege
  would have been accepted by a service that merely depends on it, without anyone checking the privilege. Both are
  closed — an absence is looked for before anything is derived, and every dependency is checked for the conditions
  its provider imposes. Three defects came from rules earlier answers had added; where two special cases had grown,
  one general rule now stands. The record goes to a fourth reader.

## archogen — the substitutability relation's second review, answered

`ARCHOGEN-M3-0398` (leaf `M3.1.1`, step 3).

- A second reader, who had not seen the first round, found seven more defects. The sharpest: the record would have
  refused an honest description of a counter that reloads at a million ticks, and the only description it accepted
  then computed a seven-minute horizon for a counter that wraps ten times a second. Now an offered value always wins
  over what a rule would derive, and a counter says whether it wraps. Two of the first round's answers had not
  reached every place their rule is stated; they do now. Facts are judged per provider, so two timers with
  different widths are two offers and not a contradiction. The whole first vocabulary — every fact the corpus
  writes, with its kind of value, its role and which way "better" runs — is now in the record.

## archogen — the substitutability relation's first review, answered

`ARCHOGEN-M3-0397` (leaf `M3.1.1`, step 2).

- An independent reader found seven defects in the record's first text: a derivation that, read literally, left the
  record's own worked horizon undefined; a constraint on a fact nothing depends on that the relation would have
  passed unexamined; a mediation rule nothing declared; a counter's width read as more-is-better where a service
  reads one word; an arithmetic limit off by one; a rule that would have refused the target's own description; a
  unit the language does not know. Each is answered by editing the words it names. The record goes to a second
  reader.

## archogen — the substitutability relation is decided, and `M3` begins

`ARCHOGEN-M3-0396` (leaf `M3.1.1`, step 1).

- When does what a platform offers satisfy what a service requires? The engine will decide it by one relation,
  fact by fact, over a small set of value kinds, and every fact a description writes will be an entry of a versioned
  vocabulary that fixes what kind of value it takes, whether it is a promise or a condition, and which way "better"
  runs. A condition is read backwards, so an implementation that demands more privilege is never taken for a more
  capable one; a horizon is computed from a counter's width and rate, so a faster counter of the same width is
  rightly worse. The design is written and goes to independent review before any code.

## archogen — the runtime crates' documentation says what the fault contract now says

`ARCHOGEN-M2-0395` (leaf `M2.20`).

- `rt-core`'s crate documentation no longer calls a second release arriving while one is latched an overrun on
  arrival: it is kept as a mark and judged, with the first, when the masked section closes, as the contract's rule 1
  says. `rt-reference`'s eleven quotations of the contract's earlier wording read as the contract does today. No
  behaviour moved.

## archogen — the engine composes the analysis's four composite inputs

`ARCHOGEN-M2-0394` (leaf `M2.10.2`; `M2.10` closed).

- The runtime analysis no longer takes a task's cost, its longest masked stretch, or the delays before the timer's
  and a source's service as figures the caller hands over whole. The engine puts each together from its parts — the
  catalog's costs for kernel code, the application's own figures, the plan's interrupt order — as the composition
  record says and under the facts it states, and the conclusion names every part with its owner and evidence
  category. Every way the composition can stop has the verdict the record gives it, and a set that stopped anywhere
  reports no task as holding. Twenty-one hand-derived tests; sixteen deliberate breakages, one rule each, all caught.

## archogen — the catalog's gate runs from the hooks

`ARCHOGEN-M2-0393` (leaf `M2.7.4.4`).

- Every commit now passes the catalog's gate: the pre-commit hook judges the pending commit's catalog against its
  parents and blocks a refusal, the merge hook reports, and a hook after the commit re-judges it against its true
  parents, which catches an amended commit. Blessing the lock is a request made on the commit. With no catalog yet,
  the gate costs a commit nothing. Two facts the design left to measurement — what the merge hook can see, and what an
  amend's hooks see — are measured and written into the design.

## archogen — the checker builds every commit a push replays

`ARCHOGEN-M2-0391` (leaf `M2.7.4.3`, closed).

- When a pull request carries several commits, the checker now builds each one that changes what a record's
  packages are built from, not only the last, so a commit made with the local gate bypassed is built too, as the
  design requires. A commit that changes only documents is left alone.

## archogen — the checker builds what a record names

`ARCHOGEN-M2-0390` (leaf `M2.7.4.3`, step 1).

- The checker now does what no scan can: it builds every package a catalog record names, in debug and release, for
  the host and for each target the record covers, and reads back from the compiler which files it actually read and
  which environment variables it depended on. A file outside the record's declared sources, a dependency on the
  environment, a package that fails to build, or a cargo configuration on the build's path that is not the
  repository's own tracked copy, each is refused with its code.

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

