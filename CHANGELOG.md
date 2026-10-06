# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

## archogen — the task trees' frontiers show the frontier again

`ARCHOGEN-PROGRAM-0464` (leaf `PROGRAM.62`).

- Two task trees' "current frontier" sections had grown into 33 KB of stories about work finished weeks ago, which
  filled the folder's size limit and left no room for new work. Each story was checked to have its line in the tree's
  commit log, where finished work belongs, and the sections now say only what is open and why.

## archogen — the engine reads the capability vocabulary

`ARCHOGEN-M3-0463` (leaf `M3.1.2.2`).

- The engine now reads the vocabulary file into a table it can consult, and checks what the file's own format cannot
  express: that each fact's direction suits its kind of value, that a derived fact names the rule that computes it
  and the inputs that rule reads, that no derivation goes round in a circle, and that no fact is named like a word
  the language reserves. The table matches, fact for fact, the hand-written copy the design's reviewers checked. Each
  rule has a test that fails when the rule is removed.

## archogen — the capability vocabulary becomes part of the language

`ARCHOGEN-M3-0462` (leaf `M3.1.2.1`).

- The list of facts a requirement may constrain — for each, its kind of value and which way an offered value must
  lie against a required one — is now a file of the language itself, `docs/semantics/vocabulary/vocabulary.eadl`,
  with its 35 entries written in a new kind, `deffact`. It is frozen with `eadl/1`, so changing an entry needs a
  migration note. `archogen check` answers a file that writes such an entry as the language's own definition rather
  than judging it as a system, exactly as it already answers a kind module. Every existing verdict is unchanged.

## archogen — the integration tier is no longer described as incomplete

`ARCHOGEN-PROGRAM-0459` (leaf `PROGRAM.59`).

- Six days after the emulator check was verified and the integration tier began to pass, four documents still said
  the tier was incomplete everywhere: the commit workflow, the book's chapter on verification, the policy on
  incomplete tiers, and the program tree. They now say what holds today: the tier passes where its tools are
  installed, and reports itself incomplete where one is missing. The dated history is left as it was.

## archogen — macOS builds stop piling up object files

`ARCHOGEN-PROGRAM-0458` (leaf `PROGRAM.58`).

- On macOS, Cargo's default kept every object file each build compiled and never deleted any, so the build directory
  grew without limit: about 6 100 files per rebuild of the tests, 1.45 million in six days. The workspace now sets
  `split-debuginfo = "off"`, which is already what Linux uses, and a rebuild leaves none behind. The one cost: on
  macOS a crash's backtrace names functions but not file and line. The toolbox gives a one-line way to get them back
  for a debugging session. Every tier step passes, including the browser and bare-metal builds.

## archogen — the fifth artifact cleanup

`ARCHOGEN-PROGRAM-0457` (leaf `PROGRAM.57`).

- About 9.4 GB of regenerable build output and closed reviews' scratch removed; `target` went from 11 GB to 1.6 GB.
  The inventory found a defect rather than ordinary build caching: on macOS, every build leaves all of its compiled
  object files behind, and nothing ever deletes them — 1.45 million of them had piled up in six days. That defect is
  filed and owned as `PROGRAM.58`. Everything was rebuilt and re-checked from cold: 1 218 tests passed, 0 failed, and
  every doctrine is green.

## archogen — the substitutability design passes its review

`ARCHOGEN-M3-0456` (leaf `M3.1.1`).

- The thirty-first reader found no defect, so the design for deciding when an offer meets a requirement is accepted
  after thirty-one independent reviews, the last fifteen against an executable model of it. Its four remarks were
  answered in the design and the checker. Next is the production implementation, which will be tested against that
  model.

## archogen — the substitutability design's thirtieth review

`ARCHOGEN-M3-0455` (leaf `M3.1.1`, step 31).

- The thirtieth reader found one defect: the design refused "a clause" written inside an offer but never said which
  words are clauses, so a refinement written there slipped through unchecked. A clause is now any clause the
  language's kinds declare, and the checker reads that list from the kind files themselves. The checker also now
  tries values just past the edge of every domain.

## archogen — the substitutability design's twenty-ninth review

`ARCHOGEN-M3-0454` (leaf `M3.1.1`, step 30).

- The twenty-ninth reader found one defect: an offer or a requirement written inside a system's platform binding was
  read by nothing. It was the sixth time a review found such a place, so instead of one more patch the design now
  states, for every place a declaration may write something, exactly what may go there, and refuses anything else.
  The checker also now reads the design's vocabulary table straight from the record and fails if the model's
  table drifts from it.

## archogen — the substitutability design's twenty-eighth review

`ARCHOGEN-M3-0453` (leaf `M3.1.1`, step 29).

- The twenty-eighth reader found two places where the design's wording promised more than its rule gives, both
  about one tiny value written in two units that the arithmetic cannot compare. The model already gave the honest
  answer — that nothing can be decided — so the wording was corrected, and the checker now tries such values too.
  A requirement hidden inside an offer, at any depth, is now refused.

## archogen — the substitutability design's twenty-seventh review

`ARCHOGEN-M3-0452` (leaf `M3.1.1`, step 28).

- The twenty-seventh reader found one defect: writing the same required value twice, in two units, could turn a
  description that checks into one the arithmetic gives up on. A side's requirements are now read as an offer is:
  one value in any spelling, any comparison that disproves them refused at once, and only what nothing decides left
  as beyond the arithmetic. The checker now proves that writing a value again never undoes a verdict.

## archogen — the substitutability design's twenty-sixth review

`ARCHOGEN-M3-0451` (leaf `M3.1.1`, step 27).

- The twenty-sixth reader found two defects. A dependency written as a list — a service's name followed by a
  requirement — silently dropped the requirement, so a horizon of an hour could go unchecked; a dependency or an
  absence is now a plain name, and anything else is refused. And a requirement block nested inside another was
  merged into it, though the design calls it a block of its own; it now gets its own verdict.

## archogen — the substitutability design's twenty-fifth review

`ARCHOGEN-M3-0449` (leaf `M3.1.1`, step 26).

- The twenty-fifth reader found one defect: a value written two ways, such as `1 ms` and `1000000 ns`, could be
  judged differently depending on which was written first, because only the first was compared. Every way of
  writing it is now compared, and the checker writes one amount in three units to prove the order no longer
  matters.

## archogen — the substitutability design's twenty-fourth review

`ARCHOGEN-M3-0448` (leaf `M3.1.1`, step 25).

- The twenty-fourth reader found two defects. A requirement written inside a system's platform binding, or inside
  another requirement, was never read, though a dependency beside it was; the model now reads requirements wherever
  presence reads dependencies, and the checker places both at every position. And two sections of the design
  disagreed on how amounts in one unit are compared; they now agree. A service that states two values for one fact
  is now refused, as a block is.

## archogen — the substitutability design's twenty-third review

`ARCHOGEN-M3-0447` (leaf `M3.1.1`, step 24).

- The twenty-third reader found three defects. Dependencies written in a system's platform binding, or nested
  inside another requirement, were seen by the presence check but never by the matching rules; the model now reads
  every place presence reads, and its checker uses presence itself as the referee. The design contradicted itself on
  whether a service's malformed values are refused; they are. And a dependency or an offer naming a number or a
  string had no answer; it is now refused, as the same mistake inside a requirement was one round earlier.

## archogen — the substitutability design's twenty-second review

`ARCHOGEN-M3-0446` (leaf `M3.1.1`, step 23).

- The twenty-second reader found three defects. For the fourth round running, one was a way of writing a requirement
  that the design gave no answer for. This time the whole class is closed: the design lists every form a requirement
  may take, and the model's checker tries a grammar of them. A service's offers were also being judged by rules meant
  for blocks, and one hand-off sentence let a later step weaken a refusal the design makes unconditional. Both are
  fixed.

## archogen — the trust design moves to the specifications folder

`ARCHOGEN-PROGRAM-0445` (leaf `PROGRAM.56`).

- The trust gate's design has passed review, so by the director's earlier ruling it moved from the decisions folder
  to `docs/specs/trust/`, freeing 46 KB for decisions still under review. The specifications folder's size limit was
  raised to make room for it, with a recorded decision and the measurement that asked for it. The decisions folder's
  own limit, which only the director may reopen, is unchanged.

## archogen — the substitutability design's twenty-first review

`ARCHOGEN-M3-0444` (leaf `M3.1.1`, step 22).

- The twenty-first reader found three defects, all in where a contradiction is judged. A requirement could say
  "allowed" in one clause and "forbidden" in another of the same declaration, and nothing noticed. A clause's result
  could depend on the order of its parts. A bare unknown name was silently dropped. Each declaration's requirements
  are now read together, every part of a clause is read, and a bare name is refused.

## archogen — the trust gate's design passes review

`ARCHOGEN-M3-0443` (leaf `M3.6.1`).

- The trust design's eleventh independent reader found no defect, which closes its review under the project's rule.
  Its defects per round ran 11, 12, 16, 11, 10, 10, 8, 11, 4, 1 and 0. Since round 8, readers tested the measuring
  instrument itself rather than the prose, and every defect came with a probe. Next is the gate itself and its F30
  test cases.

## archogen — the review-history archive hardened after its review

`ARCHOGEN-PROGRAM-0442` (leaf `PROGRAM.55.1`).

- An independent review of the new archive check found the first move correct, but the check too easy to fool. An
  open review could be passed off as closed by its wording, a late round merged in afterwards could be lost, and a
  file left out of the commit only failed later in CI. The check now reads the review's real table row, follows
  every later commit at the old path, and judges exactly what the commit will hold. Each of its refusals was proven
  by deliberately breaking it.

## archogen — the substitutability design's twentieth review

`ARCHOGEN-M3-0441` (leaf `M3.1.1`, step 21).

- The twentieth reader ran the new clause reader over every requirement the project's examples write. Half of them
  could not be read, because they also name the components they use. A contradiction that only arithmetic beyond its
  limits could settle also passed silently. Both are fixed: those names are read as the design says, and such a
  clause is now reported as beyond the supported profile.

## archogen — the trust gate's tenth review: one defect left

`ARCHOGEN-M3-0440` (leaf `M3.6.1`, step 11).

- The tenth reader of the trust design found one defect, down from four. Approving a renamed copy of a forbidden
  construct, such as `use std::include as inc;`, quietly approved every later use of the new name. Renames and
  wrappers of forbidden constructs can no longer be approved, so each use must stand where the check sees it.

