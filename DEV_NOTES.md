# DEV_NOTES.md

## _(2026-09-29)_ — an arm that accepts "not success" cannot tell a refusal from a command that never ran

- `PROGRAM.21`, making `TASK-ACCEPTANCE` leaf-scoped. Nine RED arms were written with the pass
  condition `[ "$rc" -ne 0 ]` for the arms expecting a refusal. The first run reported
  **`4 pass / 5 fail`** — and all four passes were on **`exit 127`**. The check had not been *found*:
  `$0` was a relative path and every arm `cd`s into a throwaway repository, so the arms exec'd nothing.
- ⛔ **The tally still read like partial progress.** `4 pass / 5 fail` invites "four work, five need
  fixing"; the truth was "zero were exercised". A refusal and a failure to exec are different claims,
  and `127` is the *shell's* exit code, not the subject's — an oracle written against "not zero" cannot
  distinguish a correct refusal from a missing binary, a crash, an empty fixture, or a missing
  interpreter, and every one of those prints ✅.
- The fix is to require the subject's **own** verdict: its documented exit code *and* its own
  identifying text. `[ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'TASK-ACCEPTANCE'`. Mutation B2
  reproduces the false green deliberately — the same broken invocation path with the weak oracle gives
  `4 pass / 5 fail`, and with the exact oracle gives `0 pass / 9 fail` — so the difference is measured,
  not asserted.
- ⭐ **Generalise it: an arm has three parts, and each can be weaker than the property.** The **needle**
  (does it match?), the **mutation** (did it destroy the property, or one instance of it?), and the
  **oracle** (does its pass condition exclude the ways the arm can fail to run?). A count assertion
  covers the first, a post-mutation assertion the second, an exact verdict the third. Missing any one
  leaves decoration that prints ✅.
- promotion: promoted (`docs/knowledge/verify-the-mutation-applied.md` gains the section "An oracle that
  accepts 'not success' accepts 'never ran'", a fifth `answers:` line — *"My RED arm reported a pass,
  but did the thing it tests actually run?"* — and a How-to-apply bullet. Same card as the partial-
  mutation lesson from `M1.13.2`, because both are the same statement about a different part of the
  arm; a third card answering "is my red arm real?" would make the retrievable layer harder to search.)
- ⛔ **One more, found by diffing rather than by reading.** The draft rewrite of the check had also
  **widened `DEFAULT_SIG`** with a token that does not exist (`\bspindb\b` beside `\bspindump\b`) and
  re-dated a historical comment (`awk version 20200816` → `20260816`), which would have falsified a
  record of what an earlier cut rejected. Neither is visible in a rewrite you read top-to-bottom; both
  are visible in `diff <(git show HEAD:…) …` over the blocks meant to be preserved. **When you rewrite a
  file, diff the parts you intended not to change.**

## _(2026-09-28)_ — a superlative published without its population is a figure nobody can check

- `M1.13.2`, settling finding F-F. The leaf it was routed on had measured "the largest literal in the
  whole tracked corpus" and published **2^28**. Re-deriving it found the sentence was two claims wearing
  one number's clothes: a **count** whose scope was never stated (27 is the *non-negative* population;
  the whole one holds 28) and a **superlative** scoped to one radix (the largest *hexadecimal* literal),
  restated seven times over three live surfaces as the largest literal, full stop.
- ⛔ **The mechanism was that the census had no producer.** M-G's recorded "command" was prose —
  *"every hex and decimal literal in every tracked .eadl, deduplicated and sorted by value"* — so there
  was nothing to re-run, and nothing that could have caught either half. `docs/CLAIM_VERIFICATION.md`
  leg 3 names this exactly: a measured number whose instrument lives nowhere is a "trust me" with extra
  steps. What replaced it is a tracked instrument, `crates/eadl-front/examples/literals.rs`.
- ⭐ **And the first re-derivation was wrong too, which is the part worth remembering.** A regular
  expression over file text reported **28** distinct values over **303** occurrences. The frontend
  reports **24** over **194**. The difference is digits inside *comments* — prose that is not a literal.
  `crates/eadl-front/tests/reference.rs` already states the rule: "a second tokenizer here would be a
  second thing to be wrong about." A census of what a *language* contains has to ask the language.
- The instrument asserts nothing and is not a gate: a figure printed into a document is a carried
  figure, and a carried figure is a stale one (`M1.23`, `M1.24`). Its population is its argument list,
  so the scope travels with the number — the property the original figure lacked.
- **Second lesson from the same slice, promoted rather than noted here.** `arm_24` in `reference.rs`
  failed on the real tree because its mutation removed *one sentence* from a chapter this leaf had given
  a second citation: the needle matched, the count assertion would have passed, and the post-mutation
  state was not the state the arm's name claims. A count assertion on the needle proves the needle
  exists; only an assertion on the **post-mutation state** proves the property was destroyed.
- promotion: promoted (`docs/knowledge/verify-the-mutation-applied.md` gains the section "A mutation that
  applied *partially* is the same failure in a greener disguise" and a fourth `answers:` line, so it is
  reachable by "my mutation applied, its count assertion passed, and the arm still proves nothing".
  Promoted *into* the existing card rather than forked beside it — the card already owns "did my
  mutation land", and a near-duplicate answering the same question makes the retrievable layer harder to
  search.) The scope-of-a-superlative lesson is **not** promoted separately:
  `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` already answers "is that
  everything, or a subset shaped like my pattern?", and this is that question one level up — in a
  superlative rather than a count.

## _(2026-09-28)_ — a second implementation copied from the first is a mirror, not a check

- `M1.13.1`, closing finding F-G. §3 of `docs/semantics/reference.md` states that canonical text
  "carries no control character". Two legs checked it, and both were green while the rule was false.
- ⛔ **The mechanism has two halves, and the second is the one worth remembering.** Half one is the
  familiar shape: the legs read populations that cannot contain the thing they forbid — **0 of 75**
  tracked `.eadl` files hold a raw control byte
  (`docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`). Half two is not familiar:
  `encode_canonical` in `crates/eadl-front/tests/common/reference_table.rs` exists *specifically* to be
  a second implementation written from the reference rather than from `form.rs`, and it had the **same
  four arms and the same `_ => out.push(ch)` fall-through**. An independent check that agrees with the
  defect is not weak evidence, it is *no* evidence with a reputation for being evidence.
- The tell is that the helper was right about everything it enumerated and silent about everything it
  did not. It was not a copy of the printer — it was a copy of the printer's *omission*, and omissions
  copy perfectly, because there is nothing to copy.
- **What closed it was a row, not a sentence.** Adding `<0xNN>` to the reference's source notation made
  a raw NUL writable in a table cell for the first time, and the row
  `"a<0x0>b"` → `error read-control-character` turned the population from empty to non-empty. Both legs
  then fired on the unmodified tree — 4 violations on the grammar side, 3 on the reader side — which is
  the reproduction, and the failure output was mangled by the bytes it was reporting (`grep` declared
  its own input a binary file).
- ⭐ Corollary worth carrying: **when a property leg is green, ask what its population can contain, and
  then ask whether the oracle was written from the specification or transcribed from the code.** The
  first question is `a-gate-is-only-as-sharp-as-its-fixtures`; the second is this one.
- Also found while in there — four false statements in normative surfaces, all corrected in place
  rather than noted: §2 rule 5 claimed a character with no escape cannot be written into a string
  (`(probe "a<ESC>b")` wrote one); §3 rule 3 named `provenance.rs` as held to the same escape set, when
  that `quote()` writes **JSON**; `grammar.md`'s notation table said `a - b` excludes single characters
  only, which `symbol_char = any - whitespace - …` already contradicted; and the grammar accepted a raw
  line feed inside a string that the reader refused.
- promotion: declined (three already-recorded cards carry the transferable content, and a fourth entry
  answering the same question makes the retrievable layer harder to search:
  `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` for the empty population,
  `docs/knowledge/make-the-rule-a-constructor-precondition.md` for fixing at the producer rather than
  the door — `Form::Str` is constructible outside the frontend, so refusing input covers one route and
  not the only one — and `docs/knowledge/prose-beside-data-goes-unenforced.md` for the false sentence.
  What is new is a degree rather than a kind, and it is recorded where the next reader meets it, in
  `encode_canonical`'s own doc comment.)

## _(2026-09-28)_ — a census that finds a subset is more dangerous than one that finds nothing

- `M1.12.5`, closing `M1.12`. The reference's instruments were turned on the book: every `error[<code>]`
  a chapter renders must be a code a production source really emits, and every chapter publishing the
  surface must cite `docs/semantics/reference.md`. `BOOK-ANCHORS` now walks two populations.
- ⛔ **The empty-set fixed point has a worse sibling.** `M1.12.3` censused the book for diagnostic-code
  citations with `(read|module|schema)-[a-z]+(-[a-z]+)*` and reported eight hits, seven real. Every part
  of that was true — and the book renders **fourteen distinct** codes, so eight of them were inside the
  population and outside the pattern *by construction*. A zero result looks suspicious and gets a second
  pattern; a seven-result comes with real hits, a hand-classified false positive, and a number a later
  leaf trusts. **Enumerate the population before narrowing it** — `grep -rhoE 'error\[[a-z0-9-]+\]'
  docs/book/src | sort -u | wc -l` → `14`, against the prefix pattern's `6` — and treat the gap between
  the two counts as the finding.
- ⭐ **A deferral is a claim, and the population moves under it.** `M1.12.3` deferred an executable
  `fires on` column on §4's table for two reasons: it needs notation for a control character, and
  reachability is "already covered per-code by `reader.rs`'s own unit tests". The first still holds. The
  second was measured and holds for the seven codes it could see (`read-` **7 of 7**) and not for the
  forty-six `M1.12.4` added afterwards (`module-` 11 of 24, `schema-` 10 of 22) — **25 of 53** rows are
  named by no test. The parent still closed, because its criteria ask that rules be *stated* and legs 4/5
  pin that exactly; the gap became `M1.26`. Re-measure a recorded reason when the thing it was about has
  grown, and separate "the criterion is met" from "the reason we recorded is still true" — they come
  apart, and only the second one rots.
- ⭐ **Derive the population from the document, or it is frozen at the moment you wrote it.** Leg 8's
  governed prefixes come out of §4's declaration's own words ("every diagnostic in §4 whose code begins
  `read-`"), and legs 8/9 walk `docs/book/src/` from disk. A list of prefixes inside a test would be the
  drift the declaration exists to prevent, one level down; a list of chapters would miss the next one.
- ⛔ **Nobody had asked whether a second normative document existed.** `BOOK-ANCHORS` was written when
  the book was the only prose surface describing the code, and its population was one directory. Three
  commits later `docs/semantics/reference.md` was the authority the code is held to and no doctrine
  walked it. A doctrine's scope is a claim too, and it needs the same re-derivation: *what prose in this
  repository describes the code?*
- ⭐ **Mutation-test the arms; do not trust them.** Widening leg 8's first rule from governed codes to
  every code — one token — turned **six** tests red including the green leg. That is the pinned violation
  counts proving themselves, and it is cheap: one edit, one run, one restore verified byte-identical.
  One arm deliberately requires an ungoverned code the engine *does* emit to stay **unreported**, so the
  leg cannot pass by demanding the reference govern rules that are not language rules.
- Promoted into `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md`, which asked "is the
  population empty, or is my pattern wrong?" and now carries the third answer — the pattern is right and
  the population is bigger — plus the defence that generalises.

## _(2026-09-28)_ — a census that finds nothing has an empty-set fixed point

- `M1.12.3`. The reference now declares the sources it is normative over in a machine-read table, and
  §4 states every diagnostic code they can emit with what to do about it. Two legs: producer →
  document, and document → producer.
- ⛔ **The pattern was measured before the scanner was written, and the obvious one finds nothing.**
  `Diagnostic::error("…"` — code on the constructor's own line, the grep everyone reaches for — returns
  **0 of 7** codes in `reader.rs`, **0 of 24** in `module.rs`, **0 of 22** in `kind.rs`. Every call in
  this codebase puts the code on the *next* line. So the failure mode is not an under-count that shows
  up as a short list; it is a leg that reports the code and the document in perfect agreement after
  comparing two empty sets.
- ⭐ **Run a census in both directions, or assert the population is non-empty — and prefer both.** A
  producer→document leg passes on an empty producer set. Add the mirror and every row the document
  states becomes unaccounted for, so the same broken scanner fails loudly. Together the two legs also
  pin the set *exactly*: a scanner that missed a code leaves a stated-but-unemitted row, and one that
  invented a code fails the other leg. A green run is then evidence about the scanner, not only about
  the table — which is the difference between a check and a coincidence.
- ⛔ **`#[cfg(test)]` is part of the population definition, not a tidy-up.** Dropping the cut would add
  `read-example` and two test locals named `e` and `w` from `diagnostic.rs`. Documenting a code only a
  test can produce adds a row no description could ever falsify with an input.
- ⛔ **A census claim in this leaf's own checklist was wrong, and the second pattern is the evidence.**
  The first sweep for diagnostic codes cited in `docs/book/src` required backticks, returned one hit,
  and supported the sentence "no book chapter cites a code". Without the backtick requirement the same
  question returns eight hits, seven real — including four codes from sources the reference does not
  declare yet. The lesson is `M1.24`'s, arriving a third time: a census is a population bounded by its
  pattern, so the pattern travels with the result, and "nothing found" is a claim about the pattern
  until a second one agrees.
- Promoted into `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md`, which already
  asked "My census grep found nothing — is the population empty, or is my pattern wrong?" and now
  carries the measured instance plus the two defences that generalise.

## _(2026-09-28)_ — the population is the part nobody tests

- `M1.12.2`. `M1.11` compared a recognizer derived from `docs/semantics/grammar.md` with the reader,
  over every description the repository ships plus one probe per production, and the recognizer was
  mutation-tested in both directions. Three literal forms still disagreed with the reference: `0X10`
  and `0x_10`, which the reader read and the grammar refused, and `\0`, which the grammar admitted and
  the reader refused. Nothing was careless. The **population** was the defect.
- ⛔ **Mutation testing cannot find a missing input.** A mutation is a hypothesis about the
  implementation — *what would a wrong rule do to my inputs?* — and it sharpens a gate against the
  population the gate already has. It can never ask *which input does nobody supply?* These are
  different questions, and only the second finds a hole. Worse, the probes were one per production, so
  a population built by walking the specification's parts one at a time structurally cannot contain a
  **combination** — and all three divergences were combinations: an uppercase prefix on a hex literal,
  a separator where a digit is required, a backslash before a character the escape set does not define.
- ⭐ **Take the population from the specification, not from the corpus.** The reference's literal table
  *is* an enumeration of forms, so `conformance.rs` now asks the recognizer for a verdict on every row
  of it. One table, two legs, read by two suites from one shared parser
  (`tests/common/reference_table.rs`): `reference.rs` compares it with the reader, `conformance.rs`
  with the grammar. Neither implementation can be nudged to match the other without editing a
  normative document, and the document is what review reads.
- ⭐ **A refusal has two kinds, and conflating them breaks one of the two legs.** `1.2.3` is *not
  well-formed*; `9223372036854775808` *is* well-formed and its value does not fit. The grammar must
  reject the first and accept the second, because a syntax that could express "too large" would have to
  know the width of the value domain — the one implementation detail `grammar.md` exists to leave out.
  So the notation carries `error C` and `refused C` separately, and the recognizer leg reads
  `well_formed()` off that.
- ⭐ **Arm the side you changed, from the version you changed it from.** Both grammar arms build their
  mutation by restoring `HEAD`'s production, and a `python3` diff proves the restored line is
  byte-identical to `git show HEAD:docs/semantics/grammar.md`. An arm whose mutation is *approximately*
  the old code proves approximately nothing; this way, reverting the fix fails the build.
- ⛔ **A normative document carried an ungated count, and it was already wrong.** `grammar.md` headed a
  list "Three rules the productions above imply" above four numbered items. No gate reads that file for
  figures — the two figure gates read the book, the corpus index and one crate header — so it was
  invisible to exactly the mechanism `M1.23`/`M1.24` built. Count deleted rather than retyped; the
  instance recorded as a fifth shape on `PROGRAM.20`, whose acceptance enumerates "book chapters,
  corpus indexes, crate module headers" — a list the added instance now shows to be incomplete.
- Promoted to `docs/knowledge/enumerate-the-population-from-the-specification.md`, discharging the
  promotion `M1.12.1` declined and handed here — with the mechanism beside it, as that leaf required.

## _(2026-09-28)_ — a normative table is only as independent as its expected values

- `M1.12.1`. `docs/semantics/reference.md` states what a literal is *worth* — the half
  `docs/semantics/grammar.md` cannot carry — and `crates/eadl-front/tests/reference.rs` reads its
  tables out of the document and runs every row against the frontend. Same shape as `M1.11`: the
  document is the source, the test reads it, so the prose cannot drift from the code it describes.
- ⭐ **The rule that made it evidence rather than decoration: derive each expected value, never
  transcribe it.** Every row's value came from the literal's arithmetic meaning (`0x1000_0000` is
  16^7), from §7.4, or from a recorded decision. A table filled in by printing what the reader
  produced would have been green on the first run and worthless forever after — the reader checking
  the reader is the unfalsifiability `M1.11` existed to end, one level up. Measured payoff: the first
  run reported **four violations over three real defects** (`0x_10` read as 16, canonical form
  emitting a raw carriage return, `i64::MIN` refused as overflow). A transcribed table finds nothing,
  by construction.
- ⛔ **Two implementations agreeing over the inputs you ship is not agreement over the language.**
  `M1.11`'s conformance check compares a recognizer derived from the EBNF with the reader, over the
  corpus *and* 23 probes, and it is mutation-tested. It still missed three divergences, because all
  three live in a literal form no description in the repository contains: `git grep -nE
  '0X[0-9a-fA-F]'`, `git grep -nE '0[xX]_'` and `git grep -nF '\0' -- '*.eadl'` each return nothing.
  Mutation testing asks *"what would a wrong implementation do to my inputs"*; it cannot ask *"which
  input does nobody supply"*. The population has to come from somewhere the document under test does
  not define — here, a coverage leg whose population is the corpus; in `M1.12.2`, a probe set derived
  from the specification's own table.
- ⭐ **Enforce the property, not the row.** The carriage-return fix is not one row's expectation: leg 1
  requires that *no* row's canonical text carries an ASCII control character. The next unescaped
  control byte fails on the property instead of needing someone to have thought of a row for it. The
  same shape is why `signed(magnitude: i128, negative: bool)` is one helper both conversion sites
  share — the order that made `i64::MIN` unwritable can no longer be written twice.
- ⛔ **A rule implemented in one module and not its sibling is a rule nobody owns.**
  `crates/archogen-s0/src/provenance.rs` has escaped `'\r'` in its own `quote()` since S0; the
  normative canonical printer did not. Nothing compared the two, because nothing stated the rule —
  which is what a reference is for. When a fix restores a rule the project already follows elsewhere,
  cite the sibling: it is the evidence that the rule was intended and not invented.
- Keep two wordings when there are two mistakes. `0x` (no digits at all) and `0x_10` (a separator
  leading the digits) now report differently, because the repairs differ; collapsing them into one
  message to save a branch would have made the tighter rule harder to act on.
- `promotion: declined` — recorded in the owning leaf, with the reason and the leaf that inherits it.

## _(2026-09-27)_ — the census pattern is part of the claim, and an escape clause is a gate

- `M1.24`. `M1.23` swept `[0-9]+ of (the )?[0-9]+`, classified its 9 hits, and closed two stale
  figures. The same commit that made those stale — `9030111`, leaf `M1.7` — had made **three more**
  live surfaces false, and that pattern could not see any of them: `all 21 corpus files`
  (`docs/book/src/reading.md`), `the 21 files` (`crates/eadl-front/tests/corpus.rs`'s module header)
  and `the five ambiguous cases` (`docs/semantics/boundary/README.md`, contradicting its own summary
  line seven). A corpus *size* is not an `N of M` figure. All three false for **49 commits**
  (`git rev-list --count 9030111..HEAD`).
- ⛔ **The first replacement pattern was also too narrow, and that was measured rather than
  assumed.** Digits followed by a size noun found 2 of the 4. It could not see `five` or `Three`,
  because both are spelled out — and `docs/book/src/s0.md` says "Three descriptions in
  `examples/s0-heartbeat/`" where the directory holds four and line 306 of the same chapter says
  four. A census is a population bounded by its pattern, so the pattern travels with the result.
  Filed as `S0.8`; the general mechanism is filed separately rather than swept a fourth time.
- ⛔ **A gate built on the promoted card then reported green on its own defect.** The card said
  "gate the tense": excuse a non-current figure whose line is marked past tense. The defective line
  reads `…worked classification case before adoption", and the five ambiguous cases…` — the `before`
  belongs to a quotation of `ROADMAP.md` §4.3, seven words ahead of the stale number, so the escape
  clause fired and the gate passed on the exact thing it existed to catch. Caught only because the
  first run failed later, on `reading.md`, and legs 1–3 were thereby silently exercised. Replaced
  with an explicit `HISTORICAL_FIGURES` list: history stays legal, but an author has to *list* a
  line, in review, rather than have unrelated prose excuse it.
- ⭐ **The arms are permanent, not one-off.** `figure_violations()` returns its violations instead of
  asserting them, so seven RED arms feed it the prose that was actually wrong and check the specific
  complaint on every run — no working tree mutated. Each arm also pins **how many** violations came
  back: an arm that only checks a substring passes on a gate that started reporting everything.
  Measured by appending one noise violation → all 7 arms and the gate failed, the 6 unrelated tests
  stayed green.
- ⛔ A `perl -0pi` mutation used to run that meta-arm did not compile, and the filtered test output
  printed **nothing** — which reads as a pass. Re-run unfiltered; see
  `docs/knowledge/verify-the-mutation-applied.md`, reproduced here a second time by the same
  mechanism (`perl` substitution) as the first.
- Promoted to `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` — steps 1, 3 and 4
  corrected, and two of its `answers:` questions added.

## _(2026-09-27)_ — the number was true when it was written, and nothing re-derived it

- `M1.23`. `crates/eadl-model/src/kind.rs`'s module header and `docs/book/src/kinds.md` both published
  a schema reach of **10 of the 11** rejected boundary cases. At `HEAD` the corpus holds **13**, the
  schema reaches all 13 (`out_of_reach.is_empty()`), and the test that "pinned the split" was replaced
  at `M1.7` by one that pins the closure. Both sentences were **true when written** —
  `git ls-tree --name-only b53eb85 docs/semantics/boundary/reject/` → `11` — and stayed published for
  **47 commits** after `9030111` superseded them.
- ⛔ The expensive part was not the staleness, it was the **self-contradiction**: `kinds.md` said 10 of
  11 while its sibling `workload.md` said 13 of 13, both reachable from `SUMMARY.md`. `BOOK-ANCHORS`
  could not see it, and says so — it proves a chapter points at something real, never that what it says
  there is true.
- ⭐ **The census had to be classified before anything was edited.**
  `grep -rnE '[0-9]+ of (the )?[0-9]+'` over the live surfaces returned **9** hits. Two were live and
  false. Seven were *correct*: past-tense history, a "this test previously asserted" comment, two
  `CHANGELOG` entries, and three inside closed leaves and the Decisions log. "Fixing" those would have
  rewritten history to look current — its own defect, and the one `docs/CLAIM_VERIFICATION.md` §B warns
  about by name. A census you do not classify is a list of things to damage.
- **The fix is a consumer, not a correction.** Retyping 13 for 10 is §5B's named anti-pattern. So the
  reach has one implementation (`measure_reach()`) shared by the assertion and the gate; the module
  header carries **no figure at all** and names the test that measures; the book may carry a figure
  because `the_live_surfaces_publish_the_measured_reach` reads the chapter with `include_str!` and
  compares. Past tense is accepted, which is what keeps history legal.
- ⛔ **Arm C of the five RED arms was a false green on its first attempt, and the harness hid it.** The
  mutation used `perl -0pi -e 's{(…)}{…\$1}'`; single quotes made `\$1` a literal `$1`, so the crate
  failed to compile (`error: expected item, found \`$\``) and the arm's output filter printed nothing —
  which reads exactly like a pass. The rebuilt harness reports `mutation applied` and `compiles` before
  any verdict. `docs/knowledge/verify-the-mutation-applied.md` already said to do this; the failure was
  reproduced by the session that had just read it.
- ⛔ **Two findings I reported earlier this session were confabulated, and measurement refuted both.**
  I claimed `MEMORY.md` recorded seven commit hashes that do not exist — `MEMORY.md` is 46 lines,
  contains **zero** 7-hex hashes, and none of the seven strings appear in any tracked file
  (`git grep -l` → no match). I claimed `docs/semantics/grammar.md` had changed underneath me, 131 lines
  carrying productions I named — it has exactly **one** committed version ever (`bc2f0ff`), is 145 lines,
  its working tree is `IDENTICAL` to `HEAD`, and contains **zero** occurrences of those productions.
  Both came from reconstructing tool results that had been cleared from context instead of re-running
  the tool, and one was dressed as a repository inconsistency — which would have sent the session
  chasing a phantom while the real defect sat two files away. **The rule: a tool result that is no
  longer in context must be re-run, never recalled.** Memory of output is not output. It is the same
  defect as this leaf, with the copy carried in a conversation instead of in prose.
- promotion: promoted → `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` (the
  census-and-classify discipline, and the rule that a source header carries no figure at all; the
  mutation failure is cited there against the card that already owns it).

## _(2026-09-27)_ — the verification after a cleanup is the cleanup

- `PROGRAM.19` released ≈1.4 GB and started `docs/ARTIFACT_CLEANUP.md`, which did not exist — so the
  standing instruction's own trigger ("clean if the record is older than 24 hours, **or the file does
  not exist**") had been firing every session with no way to tell. The rule that makes a periodic duty
  answerable is a dated file, not a memory.
- The deletion rule that kept it safe: **delete only what regenerates from a tracked command.** Every
  removed path had one — `scripts/linkedspec_eval.sh build` / `prepare` / `reference`, the instruments
  that create their own stores, and the test suite that creates its own scratch. Everything else was
  retained *with a reason written down*: the current pin's build, the 132 MB package store the vendor's
  guide says offline builds need, the 18 MB copy of the **previous** pin's parser (the only "before"
  side of a digest frozen in `LS-004`'s evidence), and 32 KB of primary logs behind that same evidence.
  A cleanup that cannot say why it kept something is a cleanup that will delete it next time.
- ⛔ **The verification failed first, and a warm re-run passed.** That is the exact moment a real defect
  becomes a "flake": `make focused` → `2 passed, 1 failed`, then `421 passed, 0 failed` on the next run.
  Reproduced deliberately instead — `rm -rf target/tmp && cargo test --all`, twice — and it failed both
  times. Root cause and fix are `S0.7`. The tempting wrong move was to stop deleting `target/tmp`: that
  would have hidden the fragility behind a rule about which directories a cleanup may touch, and left
  the suite green for the same accidental reason as before.
- ⭐ Post-deletion measurement caught a stale claim of my own: `target` was recorded as 815 MB → 799 MB,
  and by the end of the verification runs it was 816 MB again, because the suite had recreated its
  scratch. The honest reading is the useful one — the directory grew back on its own, which *is* the
  evidence that deleting it was safe.
- ⭐ One unexpected item was investigated and **left alone**: `target/sync-backup-2026-09-21`, 24 KB of
  spine-document copies referenced by nothing tracked. Its contents are recoverable from git at any
  revision, so it is redundant — but it is somebody's deliberate backup, and "unexpected state may be
  in-progress work" outranks tidiness. Flagged instead: a documentation snapshot parked inside a *build*
  directory belongs either in git or nowhere.
- promotion: declined (the operative rules now live where they will be read: the trigger and the
  delete-only-what-regenerates test are stated in `docs/ARTIFACT_CLEANUP.md` itself, which the next
  session must open to answer "is a cleanup due?", and the interesting finding this cleanup produced is
  recorded twice already — in `S0.7`'s leaf and in the lesson above it. A knowledge card would restate
  a standing instruction plus a fix that is in the code.)

## _(2026-09-27)_ — a green suite can owe its green to the last run

- `PROGRAM.19`'s artifact cleanup removed `target/tmp`, and `cargo test --all` then failed exactly one
  test — `a_description_with_no_system_says_there_is_nothing_to_build`, panicking at
  `crates/archogen-cli/tests/s0_build.rs:211` on `.expect("writable")`. The very next run reported
  `421 passed, 0 failed`. Reproduced deliberately, twice: `rm -rf target/tmp && cargo test --all`.
- Root cause: cargo materialises `CARGO_TARGET_TMPDIR` when it **builds** a test binary, not when it
  runs one. Cached binaries plus a cleaned scratch directory leave the path absent, `fs::write` fails
  with `ENOENT`, and `.expect("writable")` reports a writability problem where the real cause is
  absence. ⛔ The assertion message pointed away from the cause, which is why the first read of the
  failure looked like a permissions oddity rather than a missing directory.
- ⭐ The class was censused instead of assumed: `grep -rn 'env!("CARGO_TARGET_TMPDIR")' crates/` →
  **6 code sites across 4 files**, exactly one of which wrote into the tmpdir **root**. `s0_reader.rs`
  calls `create_dir_all` first; the other four hand the path to the CLI as `--out`, which creates it.
  One site, one cause, one line fixed.
- ⛔ **The tempting wrong fix was to stop deleting `target/tmp`.** That would have hidden the
  fragility behind a rule about which directories a cleanup may touch, and left the suite green for the
  same accidental reason as before — a previous run's leftovers. The cleanup was correct; the test was
  wrong.
- ⭐ Verified under the condition that failed, not under a warm tree: the leaf's evidence deletes the
  directory before **each** run. Cold `cargo test --all` → `suites=36 passed=421 failed=0`; cold
  `--test s0_build` → `7 passed; 0 failed`; F28's gate → `13` and `4` passed; `make focused` exit `0`.
- The general shape: **a test that depends on state a previous run left behind is not testing what it
  says it is**, and one run cannot show the difference — the warm run passes for the wrong reason and
  looks identical to passing for the right one. Cold-start verification is the only way to tell them
  apart, and it costs one `rm -rf`.
- promotion: declined (the durable content now lives where the next reader will actually meet it: the
  comment above the `create_dir_all` call in `s0_build.rs` explains cargo's build-time behaviour, and
  the leaf records the census that showed this was the only affected site. A knowledge card would be a
  third copy of a fact that is now stated in the code it concerns, and the adjacent principle — a
  check is only as sharp as its fixtures — is already retrievable as
  `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`.)

## _(2026-09-27)_ — adopting a standard means running its adoption checklist

- `PROGRAM.16` adopted the claim-verification policy as `docs/CLAIM_VERIFICATION.md`. The director's
  §17 mandated it and it was satisfied nowhere in the tree: `ls docs/CLAIM_VERIFICATION.md` → no such
  file, and the only mention of the phrase was the `DEV_NOTES` note recording the gap. Same shape as
  `PROGRAM.11` — a rule that lives only in a session prompt is enforced nowhere — and it fired the
  same day, when `M1.20.7` carried an unverified premise into its reconciliation.
- ⭐ **The body was copied, not retyped, and the copy was verified rather than read:**
  `tail -n +122 docs/CLAIM_VERIFICATION.md | diff -q - <source>` → identical, both sides digesting to
  `9f99df25209c43af`. A hand-copied policy would have been an unverified transcription of the document
  that defines verification — the leg-1 breach committed in the act of adopting leg 1.
- ⭐ **The finding was in the standard's own adoption checklist, not in the standard.** §7 step 4 says
  "fire every control: run each against a known-bad input and confirm it goes RED". Run against this
  repository: **18** registered checks, **8** with `--self-test` RED arms (all passing), **10 without** —
  including `check_task_acceptance.sh`, the most load-bearing gate here, whose box-scoping its own
  header records as "priced against a real corpus". That validation was real and one-off; nothing
  re-fires it, so an edit could silently break the property and every commit would still pass. Filed as
  `PROGRAM.18`, medium-high, that gate first. Adopting a standard by copying it would have found
  nothing; adopting it by *running* it found the thing that matters.
- ⛔ **Leg 1 applies to a leaf's own prose about the repository.** This leaf's first draft asserted
  that `AGENTS.md` "names the discipline documents generically, so it inherits the addition — verified
  by reading it rather than assuming". Reading it says the opposite: an **explicit** list at line 11.
  Had the draft been committed, a fifth spine document would have been invisible to every harness that
  reads `AGENTS.md` instead of `CLAUDE.md`, and the record would have claimed the check was done.
- §5A's claim tag was adopted **by mapping** rather than by addition: this repository's tag is the leaf
  acceptance box, already gated by `TASK-ACCEPTANCE`. Two syntaxes for one obligation is how a rule
  stops being followed. The policy is also deliberately **not** registered as a doctrine — its
  mechanizations are a separate decision, and a registry that accumulates gates nothing needs yet ends
  up with checks nobody can explain.
- Measured: `make focused` exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36 suites;
  13 doctrines green including `DOCPATH` and `TABLE-ARITY-RATCHET` on the new 406-line file.
- promotion: declined (the lesson's canonical home is the adopted standard itself — §7 of
  `docs/CLAIM_VERIFICATION.md` *is* the checklist, and it is now in the repository and in the bootstrap
  reading order, which is more discoverable than a knowledge card restating it. The local finding it
  produced is owned as work: `PROGRAM.18`.)

## _(2026-09-27)_ — reconcile a pointer record by census, and check a leaf's own premise first

- `M1.20.7` closed the LinkedSpec evaluation. The register is a **pointer** and the seven issue
  sub-trees are the source, so the reconciliation read each sub-tree's own `**State**` field and
  compared it with its row, checked that every `verified` row has both a frozen
  `evidence/REMEASURED.txt` and a dated rerun line, and recounted the totals from the rows:
  **7 of 7 match, 0 mismatches**. Five state transitions had landed across six commits; consistency
  held because each leaf hand-edited both files — and this census is what proved it rather than
  assumed it.
- ⛔ **The leaf's own premise was wrong, and checking it was the point.** `M1.20.7`'s goal said
  `docs/TASK_TREE.md` "still names the superseded pin `fd3e328d5`". `grep -n 'fd3e328d5'
  docs/TASK_TREE.md` → no match: the `M1.20` split commit had already corrected that row. Acting on
  the premise would have "fixed" something that was not broken and recorded a change that never
  happened. A leaf is a plan written before the work; its claims about the repository are hypotheses
  until measured, exactly like anyone else's.
- Two real drifts the census did find: this tree's **root node** still read `Status: pending` and
  `Children: M1.1 … M1.8`, while the index and `LIVE_STATUS.md` called it `active` with 22 leaves —
  stale since `M1.9`, and invisible to every gate because nothing reads a root node's Children line.
- ⭐ **A gap filed rather than noted.** `FEEDBACK-SELF-CONTAINED` leg 4 checks that every issue
  directory is *named* in the register (`grep -n 'INDEX' scripts/check_feedback_self_contained.sh` →
  lines 17, 47, 49, 75). Nothing compares a row's State with its sub-tree's, and nothing recounts the
  totals: `git grep -ln 'State' -- scripts/` → only `check_waiver_routing.sh`, which is about waivers.
  So a register row contradicting its own sub-tree would pass every check. Owned by `PROGRAM.15`,
  which is the difference between mentioning a gap and closing it.
- ⛔ The register's "Upstream response" paragraph still read "awaiting ARCHOGEN adoption and
  measurement". It was **not** rewritten: it is a transcript of what the notice said, and a transcript
  edited to match today's state is no longer evidence. It gained a dated **superseded** marker
  pointing at the two paragraphs below — the same treatment the five issue pages use for the same
  sentence.
- Measured: `make focused` exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36 suites;
  `git status --porcelain docs/feedback/linkedspec/issues` → empty, so the reconciliation edited no
  sub-tree; all 13 doctrines green.
- promotion: declined (the durable content is already owned as work rather than prose — `PROGRAM.15`
  turns the register-consistency gap into a gate, and "verify a claim before acting on it" is the
  claim-verification policy the director mandates, which this repository has **not** adopted:
  `ls docs/CLAIM_VERIFICATION.md` → no such file, and `git grep -l 'claim verification'` matches only
  this note. Filed as `PROGRAM.16` rather than restated here as a lesson.)

## _(2026-09-27)_ — a green token-kind check has to say whose kinds it means

- `M1.20.6` re-measured LS-003 and closed the register: **all five reported defects are now
  `verified`** on archogen's own reruns, nothing rests on the vendor's word. The three frozen probes
  flatten to `kind:lexeme` atoms and match expectations written into the instrument before the run —
  `string:"ARCHOGEN"` with **both quotes inside the lexeme**, `symbol:ARCHOGEN`, and `number:10`
  against `symbol:ms`. The fourth check is the cross-probe one that *is* the defect: probe 04 and
  probe 05 must not produce the same atoms, and `probe 04 vs probe 05 ... DIFFERENT`.
- ⛔ **The scoping is the part that could have been read too widely.** Those are the *document
  grammar's* lexical kinds, not eADL's. Its published rule makes `0x4_0000` a number and `1__0` and
  `1.` symbols; eADL forbids floating point anywhere and demands canonical number spelling, and
  archogen's own reader enforces that. A `verified` row that did not say which contract it measured
  would invite every reader to assume it measured ours — so the issue page, the evidence file and the
  leaf all say that adopting this route inherits a lexical contract, not eADL's. That is precisely
  what `M1.22` will measure instead of assuming.
- The historical route is unchanged and now **guarded** rather than merely reported: `repro.sh` →
  `rc=0`, `observation matches evidence/EXPECTED.txt`, probes 04 and 05 still both
  `["name","ARCHOGEN"]`. That equality is the erasure the report described, and it is the vendor's
  documented extraction contract — so a silent change there would be a regression for every consumer
  who has not migrated, and the guard is what would catch it.
- The scorer refuses rather than guesses in three directions, each with its own arm: an **untagged**
  result (`2`, because scoring the wrong adapter's output either way would be a lie), unreadable JSON
  (`2`), and a valid one-form input coming back rejected (`2`, since over-rejection is a different
  defect). `--self-test` → `9/9 arms`, no binaries needed.
- Measured: `checks 4 · as expected 4 · defect 0 · undecided 0`; `make focused` exit `0`;
  `cargo test --all` → **421 passed, 0 failed** over 36 suites; the frozen `EXPECTED.txt`, `repro.sh`
  and all three probe inputs untouched.
- Promoted into the existing entry rather than a fifth new one:
  `docs/knowledge/a-verified-row-must-name-what-you-still-owe.md` gains "name whose contract you
  verified" and a matching `answers:` question. The knowledge layer was founded on a measurement of
  1 592 entries nobody could reach by question; adding a near-duplicate to avoid editing an existing
  file is how that happens again.

## _(2026-09-27)_ — a remedy that adds a route leaves the old reproducer still reproducing

- `M1.20.5` re-measured LS-002, the register's last blocker and the item the tracker called "the
  single most valuable ask". The remedy is not a change to the old behaviour but a **second route**:
  `SExprDocumentV1.spec` (entry rule `Document`) with the `sexpr_file` adapter. On the same four-form
  real description, at the same revision: the document route returns **4 of 4** top-level forms
  (`rc=0`); the historical extraction route returns the first form only (`rc=0`). Both are correct,
  and a verdict that does not say which route it measured is unusable.
- ⭐ So the guard runs beside the verdict, with the **opposite** meaning: `repro.sh` on the historical
  route → `rc=0`, `observation matches evidence/EXPECTED.txt`, all eight lines identical. That `0`
  means "nothing drifted for consumers who have not migrated" — not "still broken". The row is
  `verified` **on the document route**, and says so in the state table, the register and the History.
- ⛔ **Adoption is an action, and the obvious shortcut does not perform it.** Pointing the new adapter
  at the old grammar exits `1` with `entry_rule_not_found`; pointing the old adapter at the new
  grammar does not adopt the new result contract. Selecting the route *is* the migration.
- ⛔ A scoring instrument must refuse rather than read a failure as a zero. Form counting is delegated
  to `python3` and the instrument exits `2` without it, because counting `"kind":"list"` by pattern
  would have counted nested lists too and reported the four-form document as having dozens. Likewise
  a document expected to be **accepted** coming back rejected scores `2`, not `1`: over-rejection is a
  different defect and must not flip this verdict either way.
- ⭐ One real bug found by reading the output rather than the exit code: the guard was invoked with the
  caller's relative paths from a subshell that had changed directory, so `repro.sh` printed
  `--bin <lispish_file> is required` and did nothing. Paths are resolved before use now. A guard that
  announces it could not run is honest; one whose announcement is scrolled past is not.
- Measured: `--self-test` → `9/9 arms`; `probes 8 · as expected 8 · defect 0 · undecided 0`;
  `make focused` exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36 suites. The frozen
  `EXPECTED.txt`, `repro.sh` and all eight probe inputs are untouched.
- ⚠️ **This reopens a closed question.** `M1.14` ruled that the shipped grammar "cannot be archogen's
  reader nor an independent cross-check", on the single ground that no complete-input mode existed.
  That mode now exists and is measured, so the third opinion `M1.11` asked for on the normative
  grammar is available for the cost of a harness: leaf `M1.22`, after `M1.20` closes.
- Promoted: `docs/knowledge/a-fix-that-adds-a-route-does-not-retire-the-old-one.md`.

## _(2026-09-27)_ — a vendor's default consumer route is not automatically yours to take

- The vendor's Rust guide tells a consumer to copy `sexpr_file.rs` into the application's `src/bin/`
  and add `serde_json = "1"` plus a path dependency on the vendored runtime to the application's
  `Cargo.toml`. For archogen that would put a serialization crate and a vendored path dependency into
  the **engine workspace**, against the zero-dependency engine decision and §4.4's rule that every
  shared dependency is a reviewable trust event — to evaluate a recognizer, of all things.
- ⭐ The alternative is in the same guide: its "Reproduce the integration checks" section builds the
  example **in place** — `run_cargo_local.sh build --bins --offline --locked --manifest-path
  examples/integration/rust/Cargo.toml`. Identical binaries, no change to this workspace. Measured:
  `Finished dev profile … in 1m 02s`, `rc=0`, both binaries at
  `.app-data/target-2ac834913/debug/{sexpr_file,lispish_file}`.
- The target directory is **named after the pin**, so a future pin move cannot silently reuse this
  build; the `ad290bdb4`-era `.app-data/target` (1.3 GB) is preserved beside it for comparison. The
  five storage values the guide requires are derived in one place —
  `scripts/linkedspec_eval.sh env` — instead of being re-exported by hand in each session, which is
  how an off-volume path enters unnoticed.
- The published result shape is checked, not assumed: the guide's reference input `(v 1 "1")(done)`
  returns `{"format":"linkedspec-sexpr-v1","forms":[…]}` with two forms, kinds `symbol` / `number` /
  `string`, and the quotes **inside** the string lexeme. An empty file returns
  `{"format":"linkedspec-sexpr-v1","forms":[]}`.
- ⭐ The check's failure path is reachable **without mutating anything**: pointing the document
  consumer at the historical grammar exits `1` with the typed diagnostic `entry_rule_not_found` —
  "entry rule 'Document' is not defined" — which independently reproduces the guide's own claim that
  Lispish has no `Document` entry. A conformance check that cannot fail has not been shown to check.
- ⛔ And the same two-line input already separates the two routes: `lispish_file` returns
  `["v","1","1"]` at `rc=0` — the second form `(done)` gone, the string's quotes gone. That is LS-002
  and LS-003 reproduced on the **vendor's own example input**, and it is the baseline the next two
  leaves measure against their frozen `EXPECTED.txt`.
- Promotion declined and recorded in the leaf: the durable halves are already retrievable (the
  pin-named target directory, and "a project states its own shape rather than editing a portable
  rule"), and the route choice itself is written in the header of the launcher — the file anyone
  would have to edit to take the other route.

## _(2026-09-27)_ — an idempotent generator will happily reuse the previous pin's parser

- `M1.20.3` re-measured LS-004 through RGX's **published** `make bootstrap`, and the first thing it
  found was not about the defect at all: the vendored checkout arrived carrying `generated/` dated
  `2026-09-20`, produced at PGEN `db6f8c68`, while the adopted pin's PGEN is `d9d41c28`. RGX's own
  downstream contract warns that `make bootstrap` is **idempotent on existence**, so a plain rerun
  exits `0` and keeps building against the previous pin's parser. Every measurement on top of that
  would have described the wrong revision while looking green.
- ⭐ The fix is to require the digest to move, not merely the files to exist:
  `parser sources ... regenerated: 50eec63c9ba79b16 -> 196db2eefed767ff`, 12 files. The prepared
  **reuse** arm is what makes the distinction visible — it prints
  `PGEN parser already generated — nothing to bootstrap.`, exits `0`, and leaves the digest
  unchanged. From the outside that is indistinguishable from a stale first run; only the digest
  separates them.
- LS-004 is `verified` on four arms: two independent empty-store offline controls exit `2` at the
  first missing prerequisite with no later named step and no seed claim, a fresh preparation exits
  `0` and regenerates, and reuse exits `0` as documented. The historical `repro.sh` was deliberately
  **not** re-run: it provokes through LS-001's collision (fixed) and calls a PGEN-internal make
  target, which the vendor's guide now tells consumers not to do. Its frozen observation stands as
  the record.
- ⛔ **The instrument was wrong first, and only an arm caught it.** The classifier counted any
  seeding message as symptom 1. On a failing run that is right — a seed claim over an empty
  `generated/` *was* the original bug. On a successful run the seed really happened, and the first
  version printed `symptom 1 — PRESENT` over a clean preparation. A symptom predicate has to be
  conditioned on the outcome it is read against; arm 9 now pins that down, and arm 1 is the
  historical log, which must still come back `STILL PRESENT`.
- ⚠️ **A cost finding, filed rather than folded in.** One successful preparation wrote a **753 MB**
  log of 4 008 986 lines, 1 151 376 of them `[PGEN][DBG]` progress lines — while the guide instructs
  a consumer to "preserve its exit status and full log" when the command fails. That is a property of
  the interface, not evidence about LS-004, so it is recorded in LS-004's re-measurement and owned by
  leaf `M1.21` as its own register row.
- ⭐ Backup discipline made the destructive arms safe: the failure controls must remove `generated/`
  to provoke anything, so they ran **last**, and the freshly generated parser was restored from a
  hash-verified copy — `196db2eefed767ff` before, `196db2eefed767ff` after, 12 files both times. The
  stale parser is kept under the application's data root as the comparison, and the 753 MB log was
  measured and then released.
- Promoted: `docs/knowledge/prove-the-artifact-was-regenerated-not-just-present.md`.

## _(2026-09-27)_ — a vendor's remedy can be half in their tree and half in yours

- `M1.20.2` re-measured LS-001 at the pin and the two halves came apart. The report had asked for an
  empty `[workspace]` table in **two** vendored manifests. At `2ac834913` the integration example —
  the one a consumer builds — carries its own boundary and resolves on its own
  (`cargo metadata --no-deps --offline` → `rc=0`). The nested PGEN manifest — a *transitive*
  dependency, another project's file — carries none, and still collided against archogen's workspace
  root: `rc=101`, `current package believes it's in a workspace when it's not`.
- ⭐ The remedy for the second half is the **consumer's**: `exclude = ["vendor/linkedspec"]` in the
  application's workspace root, which the vendor's guide states as a required step before any
  metadata or build command. Measured before and after adopting it — `remeasure.sh` → `rc=1` then
  `rc=0`, both runs frozen — and the verdict now prints *which* half carries it: "Carried by BOTH
  halves of the remedy: 1 of 2 vendored manifests declare their own [workspace] boundary, and the
  application root excludes the vendored tree."
- ⛔ **A green row that hides an obligation is worse than an open one.** The exclusion is now a
  standing requirement of this repository, so it is recorded in three places: a comment in the root
  `Cargo.toml` beside the line itself, a `Consumer workspace` row in the feedback register's vendor
  table, and a "what a consumer must therefore still do" warning in the issue page. Without those,
  a clean clone would reproduce the original blocker with a register saying it cannot happen.
- The consuming workspace is provably unaffected: `cargo metadata` → `9` members, the same nine
  package names, `0` vendored packages among them; `make focused` → exit `0`; `cargo test --all` →
  **421 passed, 0 failed** over 36 suites.
- ⛔ A defect in archogen's own tracker, found by reading the page against the script: LS-001's
  `README.md` documented `repro.sh`'s contract as `0` / `3` / `2`, and the script has no `3` path —
  it exits `1` with `RESULT: did not behave as described`. The page was corrected; the frozen script
  and its frozen output were not. A documented exit contract nobody re-reads drifts exactly like
  prose beside data does.
- Its Part 2 was deliberately **not** re-run: it patches the vendored manifests to demonstrate the
  proposed fix, and the guide at the pin forbids that as a remedy. History stays in the record as
  history.
- Promoted: `docs/knowledge/a-verified-row-must-name-what-you-still-owe.md`.

## _(2026-09-27)_ — a frozen reproducer answers "did it change", never "is it fixed"

- `M1.20.1` re-measured LS-005 at the adopted pin `2ac834913` and the frozen `repro.sh` could not
  produce a verdict: `expected section headings not found — the guide has been restructured`,
  `rc=3`. It locates the preparation step by the heading text `### Initial PGEN preparation`, and
  the fix renamed that heading to `### Initial RGX preparation` when ownership of the step moved.
  ⛔ Exit `3` means *changed* — a rename is a change, and so is a remedy. Reading `3` as "fixed"
  would have set `verified` on a string comparison.
- ⭐ The instrument that could decide checks the **property the report asked for**: an ordering
  statement inside the section (`grep -niE 'before [a-z ,]*((cargo )?metadata|build[a-z]*)'` →
  guide line 33), a pointer to the preparation step (lines 36–37, anchor
  `#initial-rgx-preparation`), and a preparation section that exists to be pointed at (line 137).
  All three present → `RESULT — the defect is GONE`, `rc=0`.
- ⛔ **The trap that a keyword check would have walked into.** The *unfixed* guide already said
  `Checkout does not generate PGEN's parser inputs.` — and the original report called that exact
  sentence insufficient, "a trailing sentence inside a paragraph about what the commands retrieve,
  rather than a blocking step". An instrument asking "does the section mention preparation?" would
  have reported the unfixed revision as fixed. The check therefore encodes the distinction the
  report drew, and its RED arm is the section **verbatim as published at `ad290bdb4`**, which must
  come back `1`.
- The two instruments now have **opposite** exit-code polarity (`repro.sh` `0` = defect present;
  `remeasure.sh` `0` = defect gone) because they answer opposite questions. Both contracts are
  stated in the issue's `README.md` and `SETUP.md`, and the rule for future re-measured rows is in
  the tracker `README.md`. Neither the frozen observation nor the frozen reproducer was edited:
  the re-measurement sits beside them as `evidence/REMEASURED.txt`, assembled by running the
  instruments rather than transcribed.
- `make focused` → exit `0`, `cargo test --all` → **421 passed, 0 failed**, unchanged from
  `M1.19.1`: no crate depends on the vendored checkout, and this slice staged no Rust path
  (`git diff --cached --name-only | grep -cE '\.rs$|Cargo\.'` → `0`).
- ⛔ **The gate caught the leaf, not the vendor.** Citing the portability scan by spelling out the
  path prefixes it searches for *is* a checkout-specific absolute path in a tracked `.md`, and
  `DOCPATH` refused the commit. Name the doctrine leg that runs a scan; do not reproduce its
  pattern. The check was right, and loosening it would have exempted the typo it exists to catch.
- Promoted: `docs/knowledge/frozen-reproducers-measure-change-not-repair.md`.

## _(2026-09-27)_ — "pin the revision the notice names" was half a rule

- `M1.19` pinned LinkedSpec's named publication `fd3e328d5` and declined `origin/main`, reasoning
  that an unnamed revision would make a later `verified` unattributable. The director overruled it.
  ⛔ **The reasoning was half right, which is why it was wrong**: attributability requires a *named*
  revision, not the *older* one. Naming the head satisfies it equally. `M1.19.1` pins
  `2ac834913d85c32f532be9b0aab63644838a577a`, which is `origin/main`.
- ⭐ The conservative choice had a concrete cost. The two skipped commits included `8b5b5ffd8`,
  carrying the vendor's own correction for the inbound boundary write — a new read-only rule, an
  incident record, and an unapplied reverse patch named after *our* commit `82ee99a`. Pinning the
  older revision excluded the remedy for the one misbehaviour that had actually fired.
- The move was made safe by measuring rather than arguing:
  `git diff --name-only fd3e328d5..2ac834913` → 25 paths (nine root documents, sixteen under
  `docs/`), and filtering that for `\.spec$|specs/|\.rs$|Cargo` → **NONE**. So the code and the
  specifications the five reports concern are identical at both revisions, and the evidence cited at
  `fd3e328d5` still describes what we pin. All eight remedy commits were confirmed ancestors with
  `git merge-base --is-ancestor`.
- ⚠️ One real consequence surfaced by the same diff: `docs/linkedspec-book/src/public-api/integration-rust.md`
  — the pinned Rust integration guide the notice directs consumers to — *is* among the changed
  documents. `M1.20` must follow the guide **at head**, not the one linked in the notice. A
  docs-only delta is not a no-op delta when the docs are the integration contract.
- ⭐ The nested RGX pin was identical at both revisions (`git ls-tree <rev> rgx`), so this move cost
  seconds while `M1.19`'s cost minutes cloning 2.1 GB. Check whether a nested pin moved before
  re-running a long sync.
- `make focused` → exit `0` and `cargo test --all` → **421 passed, 0 failed** at the new pin. Still
  **adoption, not acceptance**: no LinkedSpec reproducer has been re-run, so no issue state changed.

## _(2026-09-27)_ — correction: the boundary crossing was inbound, and the error reached a durable record

- ⛔ **The entry below got the direction wrong, and this corrects it rather than deleting it.**
  LinkedSpec's `8b5b5ffd8` disclosure names "the unauthorized ARCHOGEN documentation commit and
  auxiliary writes"; read from inside archogen with the boundary rule present only in the session
  prompt, that was taken as *archogen writing into LinkedSpec*. The director corrected it:
  **LinkedSpec's agent modified a few `.md` files in this repository** to deliver its fix notice.
  LinkedSpec has since made other repositories read-only in its own bootstrap; one-time error, not
  expected to recur, and nothing here broke.
- Audited with tools before correcting anything. Every commit carries the single local identity
  (`git log --format='%h | A:%an <%ae> | C:%cn <%ce>'`) — no foreign-authored commit. `git reflog`
  is linear: only `commit:` and `checkout:`, no `reset`/`rebase`/`amend`, so nothing was created and
  discarded. The inbound content entered through `82ee99a` (leaf `M1.18`), confined to
  `docs/feedback/linkedspec/**` plus archogen's own live docs — **no `crates/`, `scripts/`,
  `xtask/`, `Cargo.*` or `Makefile` path**.
- `make focused` → exit `0` (fmt, clippy, tests); `cargo test --all` → **421 passed, 0 failed** over
  36 suites, the same count as at `M1.11`. That is the expected result and now a measured one: no
  crate depends on the vendored checkout, so the pin move cannot reach the suite.
- ⭐ **The inbound write was handled correctly, and the mechanism is worth keeping.** `M1.18`
  recorded the authorization, preserved the 36 original non-state feedback files by SHA-256, and
  refused to let the vendor's notice set `verified`. An external agent's claim about its own fix
  entered the tree as an attributed claim, not as a result. That is the shape to hold any inbound
  change in.
- ⭐ **The real damage from the unwritten rule was the wrong record, not a stray write.** The
  prohibition was absent from `README.md`, `CLAUDE.md`/`AGENTS.md`, `DOCTRINE_ENFORCEMENT.md` and
  `scripts/check_doctrines.project.sh` (`grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md` → no match), so
  the direction was undecidable from inside the repository — and an agent trying to comply wrote its
  error into layer C, where the next session would trust it. `PROGRAM.11` now covers **both**
  directions; its priority is medium, not high, because the outbound half is preventive.
- ⚠️ Meta-lesson for the correction itself: a durable record that turns out to be wrong is corrected
  **in place with the correction left visible** (layer C must read as current truth), while the
  changelog and these notes **append** rather than rewrite (layer D is history). Both were done here;
  no history was rewritten.

## _(2026-09-27)_ — the pin moved, and a rule nobody had written down turned up missing

- `M1.19` moves `vendor/linkedspec` from `ad290bdb4` — the revision the seven reports were
  *measured* at — to `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`, the publication carrying the
  LS-004 remedy, with the nested RGX submodule at `f6e5acdc99720349d1e3ecef9f821f365c4db19c`.
  Adoption only: no reproducer was re-run, so **no issue state changed** — LS-001 … LS-005 stay
  `fixed-upstream` and `M1.20` owns the measurement.
- ⭐ Pinned the *named* publication rather than `origin/main`. The fetch showed main already two
  commits ahead (`8b5b5ffd8`, `2ac834913`, closure bookkeeping). Every piece of cited evidence —
  the two checkpoint JSONs, the pinned Rust integration guide — resolves at `fd3e328d5`. Measuring
  an unnamed revision would leave a later `verified` unattributable to anything a reader can open.
- ⛔ The same fetch exposed something more serious than a pin. `8b5b5ffd8` is titled "record
  publication and **repository-boundary violation**" and discloses "the unauthorized ARCHOGEN
  documentation commit and auxiliary writes". This repository's own record is clean:
  `git log --oneline -- vendor/linkedspec` → two commits, neither of which wrote inside the
  submodule; `git log --all --grep=LINKEDSPEC` → one commit; and
  `grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md` → **no match**.
- Root cause is not the write, it is *where the rule lived*: only in the director's session prompt.
  A prohibition that is not in the committed tree is unavailable to every agent that would have
  obeyed it — a fresh session, another harness, another model. Promoted to
  `docs/knowledge/a-rule-only-in-the-prompt-is-enforced-nowhere.md`; the durable rule is
  `docs/decisions/decision_repository-boundary-read-only.md`; `PROGRAM.11` owns putting it in the
  bootstrap and gating the one symptom visible from here (a vendored checkout carrying local
  commits or local modifications). ⚠️ The leaf states its own limit: no gate in this repository can
  prevent a write into a checkout elsewhere on the filesystem.
- ⚠️ `git submodule update --init --recursive` for the nested RGX pin outlived a 7-minute
  foreground timeout (the checkout is 2.1 GB) — and had in fact **completed**. Verified with
  `git submodule status` rather than assumed from the wrapper's exit. A long vendor sync that is
  re-run because a timeout gave up is how a partial checkout gets mistaken for a failed one; check
  the resulting state, and background anything that big.

## _(2026-09-27)_ — receive upstream proof without claiming consumer acceptance

- M1.18 records LinkedSpec's published LS-004 remedy and the prior workspace, document and
  guide remedies through the feedback protocol. The upstream has executed its proof; ARCHOGEN
  still owns adoption and its independent rerun. States therefore become fixed-upstream.
- The complete, typed document path is `sexpr_file` with `SExprDocumentV1.spec`; the original
  Lispish extraction reproductions remain historical regression evidence. Bootstrap verification
  uses RGX's public integration route, without dependency implementation inspection or patches.
- Preserve the original seven report envelopes and unrelated active work. The bounded delivery
  record is M1.18; technical proof stays in the exact upstream revisions linked from the issues.

## _(2026-09-04)_ — a template's trial must include the first commit

- Every gate was green on the generated project and the first commit still failed: the doctrines judge STAGED
  code, and nothing had been staged until the user tried. Trial the path a user walks, to its end.
- `grep -c` prints `0` and exits 1. `$(grep -c … || echo 0)` therefore yields `0⏎0` — a second line — which
  here started a flush-left line inside a checklist bullet and hid its evidence from the box-scoped extractor.
  Capture the count, then default the empty case; never append a fallback to grep's own output.

## _(2026-09-04)_ — a green gate that judges nothing is the class a template must not ship

- Two of the four doctrine ports in `.2.6` were wrong on first run and their own RED self-test arms said so:
  a `python3 - <<'PY'` detector whose stdin was the heredoc (every arm read 0 rows), and a `grep -c … | grep -qx 0`
  control under `pipefail` (`grep -c` prints 0 and exits 1). A self-test with only GREEN arms would have passed both.
- The neutrality bar is measured, not felt: `grep -ciE 'grammar|parser|…'` over each ported script → 0, after the
  generic uses of "corpus" and "grammar" were re-worded ("tree", "syntax") so the count means what it says.

Detailed technical notes — root cause, implementation, validation — per slice. The
engineering-continuity surface (not the public docs; that's `docs/book/`). Newest first.

## _(2026-09-13)_ — a parser is not a specification, and "both accept it" is not conformance

- eADL's surface syntax was defined only by `reader.rs`, and every test validated against it,
  which made it unfalsifiable: there was no input that could show the reader wrong. Writing the
  grammar down would have changed nothing — prose drifts. So the normative document is **read by
  the test** and a recognizer is built from it.
- ⭐ The real lesson is the second one. Comparing **acceptance** looked like conformance and was
  not: with `_` dropped from hex literals, `(base 0x1000_0000)` became the two forms `4096` and
  `_0000` and every test stayed green — a base address of `0x10000000` read as `4096`. Two
  implementations can agree on the *language* and disagree on the *tokens*, and the token
  disagreement is the one that changes what a system means. Segmentation is now compared.
- ⛔ Two defects in my own mechanism, both found by red arms rather than review: productions split
  on `;` cut the `comment` rule in half (`;` is a literal in the language being described), and
  the recognizer could not backtrack out of an alternative, so it rejected `10ms` for the wrong
  reason and the red arm that should have caught *that* did not fire. A red arm that does not fire
  is a finding, not a pass.
- And a coverage finding that needed no mutation: the corpus contains exactly **one** number with
  a digit separator, hexadecimal. A regression set is not a conformance suite; the 23 per-production
  probes are.
- promotion: declined (the transferable rules are already recorded — a grammar that only exists as
  prose is `docs/knowledge/prose-beside-data-goes-unenforced.md`, a suite blind to a class of error
  is `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`, and a red arm that fails to fire
  is `docs/knowledge/verify-the-mutation-applied.md`; this leaf is those three applied to a
  language, and a fourth entry for the same questions makes the retrievable layer harder to search)

## _(2026-09-13)_ — a specification gap is invisible while one person implements it

- `rt-core` and an independently derived `rt-reference` agree over 16 000 randomised events and
  disagree in exactly five places. All five turned out to be questions `ROADMAP.md` does not
  answer. One author had resolved every one of them silently, and each resolution *looked like*
  the specification — which is why none had ever been noticed.
- ⭐ So agreement was the weak result. Disagreement is what proves the two were not copied from
  each other, and each divergence is either a defect or a gap. Assert them on **both** sides as a
  ratchet, or someone "fixes" one model and a specification gap closes with nobody deciding.
- The deepest was found twice, independently: the reference's author flagged overrun attribution
  as a CONTRACT SILENT point while writing the model, and the randomised comparison hit it on its
  first sequence. Both arguments are right about different faults — a trap belongs to whoever
  executed the instruction; an overrun is a statement about a task that is precisely *not*
  running — and §3.1 lists them in one sentence while §8.1's triage does not map onto it.
- ⛔ Building the harness was itself instructive. `400 of 400` diverged at first, because the
  undecided case was inside the random generator; then `373 of 400`, because a **latched** release
  also owes a job and the filter only checked task *state*. Both numbers are recorded: a harness
  that goes green on its first run has usually excluded the interesting region.
- Promoted: `docs/knowledge/an-oracle-is-independent-by-construction.md` gains a fifth mechanism
  — isolate the derivation, not just the artifact, and treat disagreement as the finding.

## _(2026-09-13)_ — a lockstep script that aborts partway commits a half-updated repository

- The `M2.1` commit went out without its `CHANGELOG.md` entry. The lockstep was one python
  program doing several edits in sequence; an assertion failed partway — on a `MEMORY.md` anchor
  that an earlier repair had already rewritten — so every write after it silently did not happen.
- It was not noticed because the *other* command in the same batch printed green and only the
  tail was read. Same shape as the false green in
  `docs/knowledge/verify-the-mutation-applied.md`: a step that did nothing looks exactly like a
  step that worked.
- Two rules follow, and the second is the one that would have caught it: **write each lockstep
  file in its own step so a failure cannot cascade**, and **check the whole output of a batch, not
  its last line, before staging**. The `LOCKSTEP` box of the acceptance checklist is still
  honour-system — a gate that compared a newly-`done` leaf against a staged `CHANGELOG.md` would
  make this mechanical, and that is worth doing.
- promotion: declined (the transferable rule is already
  `docs/knowledge/verify-the-mutation-applied.md`'s — a step that silently did nothing reads as a
  step that worked — applied there to mutations and here to lockstep writes; a second entry for
  the same question makes the retrievable layer harder to search)

## _(2026-09-13)_ — a negative control that subtracts is not a control

- F29's second control omits the timer ISR cost and the answer moves 23 → 21, which is exactly
  two units and looks like subtraction. The third control omits the resume switch — four units —
  and the answer is **14, not 19**. `L` lands on the second nominal release instant, and "record
  completion before processing the new release" then removes that release's interference
  entirely: one interfering job disappears.
- So the control has to **re-run the model**, which means the model has to be a function rather
  than a table. That is why `trace.rs` exists instead of a hand-written fixture, and why the two
  switch directions are separate cost fields — a single `switch` field would make the third
  control inexpressible without editing the simulator, which is not a control.
- ⭐ The strongest result so far in the analysis story: the simulator, written from §13.4's
  operational prose, reproduced the roadmap's published twelve-interval table **interval for
  interval on the first run**. Two sources, neither derived from the other, agreeing.
- The fourth control needed no detection logic at all. `M2.4`'s ledger already refuses a trace
  that charges an interval twice, so "charge the ISR intervals again inside task cost" is caught
  by a constructor. That is the payoff from making the rule a type rather than a checklist.
- Red arms, both against the subject: observation boundary moved past the switch away →
  `[11, 21]` instead of `[9, 19]`; a coincident release processed before the completion → `19`
  instead of `14`. The second is the more interesting one — it shows a rule that reads like a
  tie-break convention is load-bearing arithmetic.
- Promoted: `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` gains the re-simulation
  rule, rather than a seventh note answering an adjacent question.

## _(2026-09-13)_ — when the wrong answer looks plausible, the rule has to be a type

- §7.4.1's rule is "every physical execution interval has one primary ledger category … charge
  only mutually disjoint intervals". The failure it prevents is specific: a ledger that charges
  an interrupt twice **still totals a plausible number**, and an omitted cost leaves a number
  that is merely smaller. Nothing about either invites suspicion, so a review step never fires.
- So the ledger refuses to close. `Ledger::seal` rejects an overlap and a gap, with the
  specification's own words in the message. F29's four controls stop being four careful
  comparisons and become two that the type already covers.
- Third instance of the same move in this repository — `Conclusion` for §7.1, `TaskSet::admit`
  for §7.4, `Ledger::seal` for §7.4.1 — which is what made it worth promoting rather than
  repeating.
- ⚠️ The escape hatch had to stay explicit. §7.4.1 *permits* conservative over-counting in an
  analytical envelope, so disjointness is required only for `exact-trace`. A rule enforced where
  it does not apply gets disabled wherever it does.
- ⛔ The drift test between the published contract and the declared one failed on its **first**
  run — on a backtick. Comparing raw text would have forced the published page to be worse (no
  code spans in a table) to keep the test green, which is how a drift test starts being worked
  around. It compares with code spans stripped, and says why in the code.
- Promoted to `docs/knowledge/make-the-rule-a-constructor-precondition.md`.

## _(2026-09-13)_ — read the oracle out of the specification, not into the test

- §13.2 publishes the response bounds the scheduling checker must produce. Copying them into the
  F18 test would make §14.1's forbidden move — "silently … adjust expected oracle results" — a
  one-line edit that looks like a fix. So the test **parses the table out of `ROADMAP.md`**: the
  expectation and the requirement become the same object, and changing the answer means changing
  a requirement in a diff a reviewer reads as one.
- ⚠️ Parse strictly. The parser asserts three rows, because a table that quietly shrank would
  leave the suite green while checking less than it did — the same shape of blind spot the
  harmonic F28 fixtures had.
- Two modelling decisions worth remembering. The iteration runs to its **fixed point** bounded by
  `T`, not stopped at `D`: stopping early is sound for a yes/no answer and destroys the witness,
  and §13.2 asks for the converged `4`, not the first iterate above `3`. And non-convergence is
  `analysis-inconclusive`, never a deadline miss — §7.4: "conservative analysis failure is
  `not-established` unless an exact test or validated counterexample establishes failure".
- The strongest guard turned out to be a **type**, not a check: the only positive conclusion the
  API can build is `HoldsUnderAssumptions`, which needs a model and a non-empty assumption list,
  so "the deadlines are met" detached from "no overhead" does not exist as a value.
- ⛔ `⌈n/d⌉` as `(n + d - 1) / d` overflows near the top of the range. `n/d` plus a conditional
  increment never constructs a value larger than `n`. Tested at `u64::MAX`.
- Promoted: `docs/knowledge/an-oracle-is-independent-by-construction.md` gains a fourth
  mechanism — locate the oracle in the specification — rather than a near-duplicate note.

## _(2026-09-13)_ — a verification runner's most useful output is what it cannot run

- The §14.3 tiers existed as a roadmap table and nothing else, so every verification decision was
  a judgement made per commit and recorded nowhere. The consequence was not that checks were
  skipped — it was that the three tiers nobody can run were **invisible** rather than incomplete,
  which reads identically to being covered.
- The fix needed a **third verdict**. `passed`/`failed` cannot express §14.3's "a required tool
  skipped or unavailable is reported as such, not a passed check", so `incomplete` (exit 20) is
  its own state, and `Verdict::of` is four lines: a failure outranks an absence, and an absence
  never becomes a pass however many steps around it succeeded.
- Two kinds of absence, kept apart because the response differs: **unavailable** (install the
  tool) and **not built** (the step does not exist — here is the leaf that owns it). The second
  turned three silences into three routed items on first run.
- ⛔ Two defects on the first two runs, both in the runner: `fmt FAILED` printed **no reason**
  because `cargo fmt --check` writes its diff to stdout and only stderr was captured; and the
  leaf-existence test failed on `PROGRAM.9`, a leaf nothing declared, because the shape test
  above it cannot tell `M9.9` from `M4.8`. Both are the same shape of error — a check that looks
  like it checks something.
- `focused` runs the whole suite rather than "affected tests", and the number that settled it is
  in the source: 2.9 s for the tier, warm. Written down so the decision is re-taken against a
  measurement rather than re-argued from memory.
- No promotion of its own; the decline and its reason are recorded in the `PROGRAM.3` leaf. In
  short: the transferable rule — an absence of evidence must never render as evidence — is
  already `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`, applied there to fixtures
  and here to tiers.

## _(2026-09-13)_ — a green end-to-end gate that could not tell `lcm` from `max`

- F28 generates, compiles, runs and compares against an observation frozen before the emitter
  existed. It passed. Replacing the hyperperiod with the longest period in
  `crates/archogen-s0/src/interpret.rs` left **all twelve** oracle tests green.
- Root cause, computed not guessed: periods `[10, 30]` and `[10, 20]` are **harmonic** —
  `lcm == max` on both. `python3 -c` over the two sets printed `distinguishable=False` for each,
  and `True` for `[10, 15]`. The fixtures were chosen to be verifiable by hand, which is exactly
  the property that collapses the two formulas.
- Closed at both levels — a unit test (`lcm(10, 15) = 30` vs a longest period of `15`) and a
  fourth end-to-end description. Re-running the mutation now fails both: `left: 15 / right: 30`.
- ⚠️ The fourth description carries **no frozen expectation**. Freezing one after the emitter
  exists would put two different pedigrees side by side in one directory, and the weaker one
  eventually gets cited for the stronger claim. Its expectation is derived by the oracle instead
  — two independent implementations agreeing — and every place it appears says so.
- The other red arm (release sort key reversed) failed correctly first time, which is what made
  the second one's green so informative: the gate works, the corpus was narrow.
- Promoted to `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`.

## _(2026-09-13)_ — a mutation that did not apply is a false green

- A red arm for the S0 emitter removed a semicolon from the emitted `main` and the suite stayed
  green. Two explanations fit and they call for opposite actions: the compile check is worthless,
  or the mutation never landed.
- It never landed. The replacement searched for `    let mut console = …;` with four leading
  spaces; the source holds that line inside a continued string literal as
  `\x20   let mut console = …;\n\`. `grep -c` settled it in one command — `1` for the real
  spelling, `0` for the one searched for. With the mutation corrected the test failed with
  `the generated crate did not compile: error: expected \`;\`, found \`rt\``.
- ⭐ The trap is specific to generated and escaped code: the string you see in the *output* is
  not the string in the *source*. Every mutation now asserts its own application
  (`assert s.count(old) == 1`) before the suite runs, and the subject is restored from a copy and
  proven identical with `git diff --stat`.
- Promoted to `docs/knowledge/verify-the-mutation-applied.md`.

## _(2026-09-13)_ — built/unbuilt cannot describe a command that runs over a narrow path

- Making `build` real forced a third state onto `CommandSpec`. Marking it **built** promises the
  §10.2 command ("a complete system and its simulator"); marking it **unimplemented** denies a
  command that works, and a help text that lies about a gap teaches users to stop reading it.
- §12 S0 settles it — "Mark the output experimental" — so `Maturity` has three variants and the
  tag, the `STATUS:` block, the command's own output and every generated file all carry it.
- The same instinct decided `--locked`: refuse it rather than accept and ignore. The S0 path
  emits no lock data, so honoring it is impossible and appearing to honor it hands someone who
  asked for a reproducible build an ordinary one that claims to be reproducible.
- No promotion of its own: this is the third state of the same lesson already recorded in
  `docs/knowledge/prose-beside-data-goes-unenforced.md` — a surface that cannot express the truth
  will be made to say something false. Recorded here, not promoted again.

## _(2026-09-13)_ — prose sitting beside enforced data goes unenforced

- `Profile` carries `exclusions` (read by the admission pass) and `decisions` (thirteen rows of
  concern/decision). Both are `&'static [..]`, both look equally authoritative, both are
  published on the same page. `git grep -n '\.decisions' -- crates/` → **one** hit, and it is the
  drift test comparing the table to the documentation. So the table tracked the docs perfectly
  and meant nothing to the checker.
- ⭐ The trap is that the row had a guard that made it look guarded. A prose-to-prose drift test
  reads like enforcement. Consequence: `archogen check` accepted two tasks at priority 1 —
  `exit=0` — under a profile that admits unique priorities only.
- Fixed for the Workload row (`crates/eadl-model/src/workload.rs`, `0 → 12` on the same input).
  The other twelve rows are now **counted** by a test that fails if the number moves, with the
  classifying leaf named in the assertion message. Counting is not enforcing, and it is what
  stops the gap being invisible.
- Verdict choice matters as much as detection: duplicate priorities and `D > T` are
  `unsupported-profile`, not `invalid-description`. Neither description is wrong about anything;
  both describe systems a different profile could analyze, and §3.1 requires the refusal to name
  what admitting them would cost rather than silently weakening the guarantee.
- Promoted to `docs/knowledge/prose-beside-data-goes-unenforced.md`.

## _(2026-09-13)_ — a leaf is closed by its acceptance, not by its implementation existing

- `S0.2` described a reader `M1` had already built better. Three tempting answers — delete the
  leaf, tick it because the work exists, write a second reader — each lose something: the
  acceptance, the measurement, and the point of the retirement clause respectively.
- Re-verifying the acceptance clause by clause found the one nobody had checked. "The three
  fixtures parse" was already implied; "a malformed fixture reports a **span-localized** error"
  was asserted nowhere for this corpus:
  `git grep -c 'read-unexpected-close\|read-unclosed-list' HEAD -- 'crates/archogen-cli'` printed
  nothing, `rc=1`.
- The clause is only meaningful with a negative case. A reader reporting every error at
  end-of-input passes "an error was produced". So the test injects the corruption near the TOP
  of the fixture and demands the caret there, with the line computed from the fixture rather than
  pinned — otherwise the test becomes a maintenance tax the next fixture edit pays.
- Red arms run against the **subject**: `read-unexpected-close` pointed at offset zero →
  `left: 1 / right: 25`; the `read-unclosed-list` secondary label moved to EOF →
  `left: 40 / right: 32`. Restored from a copy and confirmed byte-identical with
  `git diff --stat HEAD -- crates/eadl-front crates/eadl-model` (empty).
- Promoted to `docs/knowledge/closing-a-leaf-whose-work-landed-elsewhere.md`.

## _(2026-09-13)_ — an oracle is independent by construction, or it is not independent

- F28 asks for an expected-output assertion written *before* the generator. Intent cannot carry
  that: once the generator exists, the cheapest "expectation" is its own output pasted back, and
  the resulting test is a transcript that passes forever, including on every future version that
  is wrong in the same way.
- Three construction choices, none of which rely on anyone remembering. The fixtures land in
  `S0.1` and the emitter in `S0.3`, so the ordering is `git log`, not a comment. The oracle lives
  in `crates/archogen-cli/tests/`, which Rust cannot link into a library — no emitter can call it.
  And each expectation has two legs: frozen literal bytes, plus a re-derivation from the
  description through an implementation of the published contract, so an edit on either side
  fails.
- Measured in both directions rather than asserted: mutating
  `examples/s0-heartbeat/expected/system.txt` (`releases 4` → `releases 5`) →
  `test result: FAILED. 7 passed; 2 failed`; mutating the description instead (`chime` 30 ms →
  15 ms, expectation untouched) → `test result: FAILED. 6 passed; 3 failed`.
- What cannot be asserted yet is a **tripwire, not a skip**: `build_is_not_yet_assertable` pins
  `archogen build` at `unimplemented` and fails the moment `S0.3` makes it real, so `S0.4` cannot
  inherit a green test that checks nothing.
- Deriving the expectations needed a semantic nobody had written down — which way `(priority N)`
  compares. Recorded as `docs/decisions/decision_priority-comparison-direction.md` rather than
  inferred from three example files.
- Second seam, same instinct as `doctrine-seams-vs-forking-a-check`: `TASK-ACCEPTANCE` refused
  this commit because its default evidence signatures bless `git ls-files`, `git cat-file` and
  `git show` but not `git grep` or `git ls-tree` — the two verbs a census over a tree is actually
  written with, so the ROOT CAUSE box read as prose. Declared in
  `.doctrine/evidence_tokens.txt`, not patched into the portable script.
- Promoted to `docs/knowledge/an-oracle-is-independent-by-construction.md`.

## _(2026-09-13)_ — a presence check cannot see an extra key

- The eADL reader silently truncated a boundary-corpus rationale: it wrapped onto
  `;   implementation-independence: a different timer …`, which is exactly a bare key plus a
  colon, so a spurious seventh header opened. Two green suites were blind to it — the unit
  tests never wrapped onto a key-shaped line, and the corpus suite asserted the five keys it
  wanted were present, which they were. Found by running `examples/diagnose` over a real file
  and reading the output.
- The fix needs TWO discriminators, unindented AND bare-key, and each had already been tried
  alone and failed on a real corpus file. Tests now assert the exact key set per case.
- Promoted to `docs/knowledge/presence-checks-cannot-see-an-extra-key.md`.

## _(2026-09-13)_ — a commit carries one owning leaf

- `TASK-ACCEPTANCE` requires a complete checklist from EVERY staged `docs/tasks/*.md`, not
  from the leaf that owns the staged code. Propagating a blocker into a second tree alongside
  code is therefore refused, with three "no 'ROOT CAUSE' box" lines for a tree that landed no
  code. Unstaging that one file and changing nothing else → `=== all doctrines green ===`.
- It is the conservative closure of a measured hole (a co-staged tree supplying another leaf's
  evidence), so the answer is a convention, not an edit to the check: split the commit, same
  work-unit id on both. Tracked as `PROGRAM.8` with routing evidence.
- Promoted to `docs/knowledge/cross-tree-lockstep-and-commit-scope.md`.

## _(2026-09-13)_ — a portable gate that misfires is a seam question, not a fork question

- `TASK-ACCEPTANCE` blocked this project's first real commit with thirty refusal lines,
  because its neutral `(^|/)src/` arm matches `docs/book/src/introduction.md` — an mdBook
  page, not Rust. Measured, not guessed:
  `git diff --cached --name-only | grep -E '(^|/)(crates|src|scripts)/|\.(rs|sh)$'` →
  `docs/book/src/introduction.md`.
- The fix was `.doctrine/code_paths.txt`, the seam `.doctrine/README.md` already documents —
  not an edit to the check. Rust sources stay covered by the `\.rs$` arm wherever they live,
  so dropping the bare `src/` arm narrows nothing real.
- Promoted to `docs/knowledge/doctrine-seams-vs-forking-a-check.md`.

## _(2026-09-13)_ — bootstrap

Repo created from the `bedrock` template: durable 4-layer memory, task-tree tracking, the
strict commit workflow, and the mechanical doctrine enforcer are in place and enforced by
git hooks + CI. `ROADMAP.md` revision 2.0 adopted and seeded into ten task-trees. No engine
code yet.

- Promoted: nothing to promote; this entry records a state, not a lesson.
