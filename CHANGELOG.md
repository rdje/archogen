# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

