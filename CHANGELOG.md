# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. This file is a rolling
ledger: whenever it holds 40 entries, its oldest 20 are sealed, byte for byte, into the next segment under
`docs/history/changelog/`, listed with its digest in `docs/history/INDEX.md`
(`docs/decisions/decision_history-ledgers.md`).

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

## archogen — the substitutability design's nineteenth review

`ARCHOGEN-M3-0439` (leaf `M3.1.1`, step 20).

- The nineteenth reader found the executable model faithful to the design, and two defects in the design itself.
  Writing one value twice could change a result, because two copies of a tiny value were compared through an
  arithmetic that overflows. A requirement could ask for mediation both "allowed" and "forbidden" at once, and the
  first would win. Values in one unit are now compared as written, and contradictory requirements are refused.
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

