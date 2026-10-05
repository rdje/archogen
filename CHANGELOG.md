# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — closed review histories archived

`ARCHOGEN-PROGRAM-0438` (leaf `PROGRAM.55`).

- A review history stops growing when its review closes, but it stayed in the folder whose size is capped, crowding
  out the reviews still running. A closed history can now move to `docs/review-history/`, unchanged to the byte,
  leaving a short note at its old path so every link to it still works. A new check proves each moved file against
  the original. The catalog's 75 KB history moved first.

## archogen — the trust gate's ninth review: four defects left

`ARCHOGEN-M3-0437` (leaf `M3.6.1`, step 10).

- The ninth reader of the trust design found four defects, down from eleven, each shown by a probe. One was a
  build-cache directory left in the recorded build settings, which made two checkouts of one commit look different.
  Another was a program that could dodge the "one program, one role" rule by calling itself the comparison harness.
  Both are fixed, and a program's declared run-time data files are now read, hashed and compared like any other
  shared file.

## archogen — the substitutability design's eighteenth review: two defects left

`ARCHOGEN-M3-0436` (leaf `M3.1.1`, step 19).

- The eighteenth reader found two defects, down from six in the round before, each shown by a test. When three
  values were written for one fact, the answer could depend on the order they were written in. A value written
  inside `absent` was quietly read as "this fact does not exist at all". Both are fixed, and the model's checker now
  tries every order of three offers and every value written inside `absent`.

## archogen — the trust gate's first review against its instrument

`ARCHOGEN-M3-0435` (leaf `M3.6.1`, step 9).

- The eighth reader of the trust design ran its measuring instrument rather than reading about it. All eleven
  defects it found were in what the instrument measured, each shown by a probe the instrument answered wrongly. One
  was a library whose source file was named `.txt`: it was compiled and never inspected. Another was a single
  approval quietly covering every identical line in its file. A third left an approved assembler block's later lines
  unchecked. Each probe is now a test that failed first. The instrument now rejects non-Rust crate roots, ties every
  approval to one site and to its whole statement, and records each program's built artifact.

## archogen — the substitutability design's first review against its model

`ARCHOGEN-M3-0434` (leaf `M3.1.1`, step 18).

- The seventeenth reader worked against the executable model and backed each of its six defects with a test the
  model and the design answered differently. The tests landed first, twelve of them failing on the model as it stood, then the rules that make them pass: the
  "must keep running" part of a power-state requirement now lives in the vocabulary rather than in code, and a group of
  requirements no longer gives a different answer when its parts are written in another order.

## archogen — the trust configuration's place in the conformance suite

`ARCHOGEN-M3-0433` (leaf `M3.6.2.1`).

- `trust/roots.eadl`, added by the previous commit, broke a test: every description in the repository must be either
  in the language's conformance suite or excluded from it with a reason, and this one was neither. It is now
  excluded, since it configures the trust gate rather than describing a system. The previous commit's record said
  the full test tier passed, but that run came before the file existed. The record is corrected, and a leaf is filed
  for a check that ties a cited run to the tree it is committed with.

## archogen — the trust gate's measuring instrument

`ARCHOGEN-M3-0432` (leaf `M3.6.2`).

- `cargo xtask trust-inventory` builds each program whose independence a claim relies on from the commit's own
  files, records everything its compiler read, and lists what any two of them share. It refuses, before anything
  runs, a build script or any construct the catalog's rules refuse unless a reviewed admission names the exact line.
  On this repository it finds no two programs sharing code and fourteen admitted lines; sixteen test repositories
  show each way past reviewers found for code to slip in unseen now being caught.

## archogen — the substitutability design made executable

`ARCHOGEN-M3-0431` (leaf `M3.1.1.1`).

- The design for deciding when an offer satisfies a requirement now has an executable model, written rule by rule
  from the design, and a checker that runs it over tens of thousands of inputs: no value is read the wrong way, no
  stronger precondition passes as a capability, and a counter's wrap horizon matches a simulation of its reads.
  Every problem past reviewers found with a concrete input is now a permanent test.

## archogen — both design records' hand-offs in ledgers

`ARCHOGEN-PROGRAM-0430` (leaf `PROGRAM.52.2`).

- The two designs under review now state each piece of work they hand to another task once, in a table; the
  receiving tasks quote those sentences word for word, and a commit check holds the two together. Forty-six
  hand-offs, none left in prose.

## archogen — the trust-dependency gate made default-deny

`ARCHOGEN-M3-0428` (leaf `M3.6.1`, step 8).

- For five reviews in a row, a reader found one more way for code or data to slip into a program unseen, past a
  hand-built list of forbidden constructs. The list was the problem: the gate now applies the catalog's complete
  rules and admits, by review, only the exact lines today's programs need — thirteen, measured. Anything else is
  refused before anyone has to find it.

## archogen — the substitutability relation's sixteenth review: three defects

`ARCHOGEN-M3-0427` (leaf `M3.1.1`, step 17).

- The sixteenth reader found three defects, the fewest in seven rounds. A block named after a fact could capture a
  requirement on that fact once imported as a module, so such a name is refused. A provider available only in an
  idle power state no longer satisfies an idle-state requirement, because every use also runs in the running state.

## archogen — every hand-off checked in both places

`ARCHOGEN-PROGRAM-0426` (leaf `PROGRAM.52.1`).

- A design that hands work to another task now writes each hand-off once, as one sentence in a table, and the commit
  hook refuses the commit unless the receiving task quotes that sentence word for word. A hand-off can no longer be
  dropped, or carried in weaker words, without a commit failing.

## archogen — design reviews made executable

`ARCHOGEN-PROGRAM-0425` (leaf `PROGRAM.52`, step 1).

- Two design reviews had stopped converging: each round's reader found eight to sixteen new defects in tens of
  kilobytes of prose. The method changes, not the bar: each design now ships an executable model or a measuring
  instrument with an exhaustive checker, every past probe becomes a permanent test, and every hand-off to other work
  is one sentence a commit check holds in both places. A design still closes only on a round that finds nothing.

## archogen — the trust-dependency gate's sixth review, answered

`ARCHOGEN-M3-0424` (leaf `M3.6.1`, step 7).

- The sixth reader showed a reference model could call its implementation's code through a link-time symbol without
  sharing any package or file, which is now refused, and that the previous answer had assumed the programs use two
  file-inclusion forms they do not use. Those forms are now refused, as the catalog refuses them, and only Rust
  sources are scanned. What a change reports is now measured against the forms the commit started from, so
  regenerating the baseline in the same commit hides nothing.

## archogen — the substitutability relation's fifteenth review, answered

`ARCHOGEN-M3-0423` (leaf `M3.1.1`, step 16).

- The fifteenth reader again found the relation itself sound. Writing a fact after `uses`, instead of `needs`,
  would have slipped a requirement past it, so that form is now refused. Questions about catalog records, which the
  design had begun to answer for other work, are now handed to that work's own reviewed design instead.

## archogen — a missing quantity is pointed at where it is missing

`ARCHOGEN-M1-0422` (leaf `M1.41`).

- When a description wrote a bound with no quantity, such as `(tick-rate (exactly))`, the error pointed at the first
  line of one of the language's own kind modules instead of at the description. It now points at the empty bound in
  the description's own file.

## archogen — formatting checked at every commit

`ARCHOGEN-PROGRAM-0421` (leaf `PROGRAM.51`).

- The commit hook now refuses a staged Rust file that is not in the formatter's canonical form, judging the bytes
  being committed rather than the working copy. The rule was already written down for authors; nothing held anyone
  to it, which is how unformatted code reached the main line earlier the same day.

## archogen — the composition code formatted

`ARCHOGEN-M2-0420` (leaf `M2.22`).

- The scheduling checker's composition code, added earlier the same day, had been committed without the repository's
  formatter, so its focused verification tier failed on format alone. It is formatted now, with no change to what it
  does.

## archogen — the trust-dependency gate's fifth review, answered

`ARCHOGEN-M3-0419` (leaf `M3.6.1`, step 6).

- The fifth reader found the design's hand-offs carried and its measurements sound, and ten places where a rule
  was loose: an assembler file reached through `include!` went unscanned, a new direct use of an already shared
  package went unreported, and a result from the wrong build of a checker would have been packaged. The gate's report
  now has two parts, what changed since the baseline and what still awaits acceptance, so an unrelated change stays
  silent without hiding anything.

## archogen — the substitutability relation's fourteenth review: a horizon one tick shorter

`ARCHOGEN-M3-0418` (leaf `M3.1.1`, step 15).

- A counter's wrap horizon is now its modulus less one, over its rate: two reads exactly a full wrap apart can be
  equal, so a counter that wraps every 60 seconds does not meet "unambiguous across at least 60 seconds". The design
  no longer claims what any tracked example's verdict will be once the relation is wired into the checker; the
  leaves that wire it must keep each verdict or migrate it by name, and the alternative-timer example's promise to
  build "unchanged" is now an open question for its own leaf, to be brought to the director if its description must
  change.

## archogen — a refinement keeps what the platform it refines offers

`ARCHOGEN-M1-0417` (leaf `M1.40`).

- A concrete platform that claims to refine an abstract one must now keep every value the abstract one offers, and
  every bound it states: an abstract tick unit of nanoseconds is no longer "kept" by microseconds, two bounds on
  one fact are both checked, and a guarantee is not kept by offering it as `false`. Values are compared as written,
  or by amount for quantities, which is the cautious side: a value spelt differently is refused, never wrongly
  accepted. No tracked description's verdict changed.

## archogen — the trust-dependency gate's fourth review, answered

`ARCHOGEN-M3-0416` (leaf `M3.6.1`, step 5).

- The fourth reader rebuilt every program the gate watches and confirmed what it measures. Most of what it found
  were parts the design handed to other work without those parts being written into that work's acceptance; they
  now are. It also found four ways something shared could go unseen: the reference model's comparison harness was
  built by no rule, a file reached through a shared macro was missed, a build-profile change reaches every program
  at once, and a renamed assembler macro slipped past a scan. Each is now built, compared or refused.

## archogen — the substitutability relation's thirteenth review: its providers narrowed

`ARCHOGEN-M3-0415` (leaf `M3.1.1`, step 14).

- The thirteenth reader again found every timer case right for one provider, and nine defects at the design's
  borders. The relation now judges only a description's own blocks and platforms: a catalog record becomes a
  provider under a new leaf, and an adapter only through the search that holds the mediation gate. A provider that
  says nothing about a fact no longer changes a description's verdict, so the tracked infeasible cases keep theirs,
  and the alternative-timer example can build once its adapter exists. The refinement check is handed the finding
  that it keeps no value an abstract platform offers.

## archogen — the trust-dependency gate's third review: the design narrows

`ARCHOGEN-M3-0414` (leaf `M3.6.1`, step 4).

- The third reader found more defects than the second, mostly in parts the gate's design had taken on that belong
  elsewhere: what an unreviewed sharing does to a claim or a release, and how a review is recorded and protected.
  The design now decides only what is built, what is read and what is shared, and reports it; who accepts a sharing
  and what an unaccepted one costs are written into the acceptance of the leaves that own them, one of them new and
  waiting on the director. The reader also showed that a third assembler form can pull in a file unrecorded, now
  refused.

## archogen — the substitutability relation's twelfth review, answered

`ARCHOGEN-M3-0413` (leaf `M3.1.1`, step 13).

- A twelfth reader again found every timer case right at one provider, and eight defects around it. The most
  consequential was an earlier answer's instruction that would have quietly weakened the existing refinement check;
  that check is now left as it is, and the new relation reads offers its own way. Three handovers to other work had
  been named in the design but not written into the acceptance of the work that takes them; they are now. Two gaps
  in the refinement check found along the way are filed as `M1.40`. Defects per round: 7, 7, 8, 3, 3, 3, 5, 1, 2, 8,
  7, 8.

## archogen — a closed review's rounds move to where review histories live

`ARCHOGEN-PROGRAM-0412` (leaf `PROGRAM.50`).

- The folder of decision records reached its ceiling again. Rather than ask for a larger one, thirteen sections that
  were really the round-by-round history of the fault contract's review moved, unchanged and with their digest
  recorded, into a review history of their own — where every other design's review history already lives. The
  decision itself, and its first two rounds, stay where they were.

## archogen — the trust-dependency gate's second review, answered

`ARCHOGEN-M3-0411` (leaf `M3.6.1`, step 3).

- The second reader found the gate, as written, would have blocked for ever: a reviewed change could never pass the
  pull request that proposed it. Now each finding is refused (the author can fix it alone), pending (only a review
  settles it, and only the release's assurance step waits for it), or accepted — so the review is the merge. Before
  the director's protected commit exists, the gate claims nothing about independence at all. The same reader showed,
  by compiling a three-line crate, that the assembler can pull a file into a program without the compiler recording
  it; that is now refused. The reader also found that `archogen analyze` is still assigned to a leaf that closed long
  ago, now filed as `M2.21`.

## archogen — the substitutability relation's eleventh review: narrowed to one provider

`ARCHOGEN-M3-0410` (leaf `M3.1.1`, step 12).

- An eleventh reader found the relation sound for one provider — every timer case read correctly, no stronger
  precondition passing — and seven defects in what it said about whole descriptions with several providers: which
  providers count, how an absence at one meets an offer at another, and which error a description gets. Those are
  questions for the search that wires the relation into a description (`M3.4`), and they now live in its acceptance,
  each with the case the reviews found. The relation itself now gives one of five answers at one provider. Defects
  per round: 7, 7, 8, 3, 3, 3, 5, 1, 2, 8, 7.

## archogen — the trust-dependency gate's first review, answered

`ARCHOGEN-M3-0409` (leaf `M3.6.1`, step 2).

- The first reader of the trust-gate design found eleven ways sharing could slip past it: it watched the generator's
  library instead of the program that actually generates, never compared the reference model with the code it
  checks, let an author edit the list of watched programs, read the working copy rather than the commit, and could
  not see what build scripts, macros, linked libraries or outside build settings bring in. Most answers adopt rules
  the catalog's build checker already enforces — build from the commit, clear the environment, refuse what cannot be
  seen — and the watched list and the accepted list both now need the director's second reviewer, the request in
  findings §11 extended to cover them.

## archogen — the substitutability relation's tenth review: the design shrinks again

`ARCHOGEN-M3-0408` (leaf `M3.1.1`, step 11).

- A tenth reader ran its attacks against the built checker and found eight defects, nearly all in parts of the
  design that had grown into questions other work owns: which privilege a caller actually runs at, which parts of a
  description are examined, and how a bound written in an offer is read. Rather than patch them, the design lets go:
  the reversed "condition" rule for privilege, which drew defects in six of ten rounds, is gone — a function now
  states the set of privilege levels it can be reached from, and a caller's level must be in it — and the question
  of which level a caller is actually bound to moves to the joint-constraint work (`M3.3`), written into its
  acceptance. Defects per round: 7, 7, 8, 3, 3, 3, 5, 1, 2, 8.

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

