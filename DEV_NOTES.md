# DEV_NOTES.md

Engineering lessons, newest first. This file is a rolling ledger: whenever it holds 20 notes, its oldest 10 are
sealed, byte for byte, into the next segment under `docs/history/dev-notes/`, listed with its digest in
`docs/history/INDEX.md` (`docs/decisions/decision_history-ledgers.md`).

## _(2026-09-29)_ — a fuzz harness found three defects before its first commit, and nearly hid a fourth problem in itself

- `PROGRAM.9.2`, a dependency-free seeded fuzz harness over the reader and the exact arithmetic. Designing
  its properties found `M1.34`, a saturating comparison, and `M1.35`, a reader panic. Its first run found
  `M1.36`, an overflowing power of ten in the printer. Each is fixed in its own `M1` commit. The harness was
  parked in `git stash` while `M1.36` landed, so the tree was clean when it changed trees.
- ⛔ Three of the harness's own mistakes, each caught before it could report anything:
  - A generator literal `(2 << 30) as u64` was computed as an `i32` and sign-extended, so "small" values
    were not small. Fixed with `u64` literals.
  - A read-back oracle parsed into `i128`, which cannot hold a correct magnitude of exactly 2^127. Now `u128`.
  - A mutation loop asked `git diff` whether an **untracked** file had changed, reported "did not apply",
    and skipped the restore.
- ⛔ The release run passed over `M1.36`, because release wraps where debug panics. The step runs with
  overflow checks, and the read-back property catches the wrong value even without them.
- Promoted: `docs/knowledge/verify-the-mutation-applied.md` (the untracked-file case) and
  `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` (a release build's blindness).

## _(2026-09-29)_ — the reader crashed on `"a\éb"`: the second byte-for-character defect in one day

- `M1.35`, found by reading the reader while choosing `PROGRAM.9.2`'s fuzz targets. An unknown escape was
  read as one byte and the reader advanced one byte, leaving it inside `é`. The next slice panicked and
  `archogen check` exited 101. Probed alongside with 13 other multi-byte placements; none crashed.
- Fixed by reading the escaped character whole. Tests at the reader (`é` and `🙂`) and in-process through
  the CLI. The byte-wise read restored as a mutation fails both, the CLI one through the real panic.
  120 descriptions keep their verdicts.
- The same class as `M1.31`'s `position(end - 1)`. Promoted as its own note:
  `docs/knowledge/a-byte-offset-is-not-a-character.md`.

## _(2026-09-29)_ — a comparison that saturated, a comment that said why that was safe, and a test named for exactness

- `M1.34`, found while designing `PROGRAM.9.2`'s fuzz properties. `Rational`'s `Ord` multiplied across with
  `saturating_mul`, on the stated ground that saturation "preserves the sign of the comparison". Not when both
  products saturate: two different values compared `Equal` while `==` said they differ. Through
  `Quantity::compare`, that let `(deadline 1.00000000000000003 ns)` pass `(period 1.00000000000000001 ns)`,
  exit 0, where the same relation in short decimals exits 12.
- Fixed with an exact 256-bit cross product from 64-bit limbs, no dependency. The saturating version restored
  as a mutation fails the three new tests; 120 tracked descriptions keep their verdicts.
- The existing `ordering_is_exact_and_total` never had a value large enough to tell the two apart. Promoted:
  `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`.

## _(2026-09-29)_ — a tool reported absent was installed, and an exclusion written from a story was wrong twice

- `PROGRAM.9.1`. The `extended` tier's `miri` step said **UNAVAILABLE** on a machine where
  `cargo +nightly miri --version` answered. Its probe ran `cargo miri --version` under the repository's
  pinned `stable` toolchain, which has no Miri. It asked the wrong question and got a true answer to it. The
  probe now asks the named toolchain for the component, and a test refuses any requirement the probe does
  not understand.
- ⛔ The first population left out three crates on a plausible story ("their tests spawn processes"). Two
  were wrong. Measuring each of the 34 test targets under Miri on its own gave 541 tests passed, 4 ignored,
  none failed. The four genuine process-spawning tests are marked where they are written, and the five
  corpus walks that cost more than 300 s each are left out on their measured cost, with `--all` to run them.
  Promoted: `docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md`.
- By default Miri puts its sysroot in `~/Library/Caches`, off this volume. `MIRI_SYSROOT` with
  `cargo miri setup` puts it under `target/`, as Miri's own README documents.

## _(2026-09-29)_ — a census scoped to a folder, and a fixture moved out of its environment

- `PROGRAM.29`, moving gate scratch out of the system temporary directory. The leaf was filed on a census of
  `scripts/`: 8 sites in 6 files. A second census, over every tracked file but dropping `docs/` to shed prose,
  found the same 8. The gate written for the leaf reads every tracked shell script by extension and found **18
  in 15**. The ten missing were all under `docs/`, the vendor-feedback scripts and two probe artifacts. Both
  filters removed a *place*, and code lives in places you file as documentation. Promoted:
  `docs/knowledge/scope-a-census-by-the-rule-not-by-the-folder.md`.
- ⛔ **Moving a fixture moved what surrounded it.** Two self-tests changed meaning without a line changing.
  `FEEDBACK-SELF-CONTAINED`'s went **partly vacuous**: two red arms stopped firing because its lister asked
  git, which ignores all of `target/`. LS-001's re-measurement false-failed an arm because Cargo, walking up
  past an excluding workspace, found this repository's root. The vacuous one is the dangerous kind: it was
  seen only because `PROGRAM.28`'s runner counts every gate's fired arms. Both fixed at the cause (a `find`
  for `target/` fixtures; `exclude = [..., "target"]` in the root manifest; a precondition probe in LS-001).
  Promoted into `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md`.
- Measured, not read: a logging `mktemp` first on `PATH` recorded 26 calls, 26 off the volume, and 0 left
  behind. On macOS `$TMPDIR` is `/var/folders/…/T/`, so a grep for `/tmp` would have found nothing to fix.

## _(2026-09-29)_ — an `or_insert` over a fact map is the code shape of "choosing one"

- `M1.33`. A single description declaring `timer.counter` twice, offering 32 and 16 bits, was accepted. Two
  reasons, and they are the same reason: every schema check took **one** declaration (`kind::validate`), so
  nothing in the schema could see a second one; and the one pass that spans declarations,
  `presence.rs`'s `record`, stored an offer with `entry(name).or_insert(…)` — keeping the first and dropping
  the second without a word. §5.3 of `ROADMAP.md` says "reject contradictory declarations rather than choosing
  one", and `or_insert` *is* choosing one, spelled as a map operation.
- The rule went into §7 as rule 6 (the declaration frame, one level across declarations) with the code
  `schema-duplicate-name`, and runs once, inside the shared `passes` — so the single-file defect and §6 rule
  9's collision in a module tree are one check with one message, and the message explains rule 9 only when the
  two sites are in different files.
- Measured, not assumed: every tracked description through `archogen check` before (from a stash) and after —
  118, 0 changed. The book's two corpus figures moved and their gates refused the old ones until they were
  re-derived from the walk.
- Promotion declined on the leaf, with the reason: the lesson is the rule, and the rule is now written where a
  reader looks for it.

## _(2026-09-29)_ — the fixture that passes with and without a rule is not a fixture for the rule

- `M1.29.3`, giving an elaborated module tree a name rule (§6 rules 9–10) so the model passes can read it.
  The two F01 trees were the obvious fixtures, and both went green the moment the rule landed.
- ⛔ **They would have gone green without half of it.** Mutation N-2 kept rule 9 (declarations renamed by
  instance path) and dropped rule 10 (names resolved where written): both F01 trees still passed, because
  their root writes names that *already equal* the global ones — `clock.time.monotonic` is the same string
  whether or not anything resolved it. Only a module that names **its own sibling** by a local name, and a
  name an export **hides**, can tell the rule from its absence. So `app.sibling` names siblings in `uses`,
  `needs` and `refines`, and removing any one head from the name-clause list fails it — each head is
  individually load-bearing, measured, not argued.
- ⭐ **Writing a rule's legs is how its holes show.** Building the "an alias never shadows a local name" leg
  needed a module with a local dotted name beside an alias — which is exactly the case where rule 9 gives two
  declarations one name. Asking what catches duplicates at all found that nothing does, in a single
  description either: two `(defblock timer.counter …)` with 32 and 16 bits are accepted, and `presence.rs`'s
  `or_insert` keeps the first. Filed as `M1.33`, high, with §6 stating the limit instead of implying it away.
- Promoted into `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` as an instance of its question
  "How do I know a fixture set is discriminating and not just complete?", with an `answers:` line.

## _(2026-09-29)_ — a code a command cannot reach hides the defects its own tests assert around

- `M1.29.2`, wiring the module elaborator into `archogen check` and `archogen build`. The elaborator had 24
  codes, every one stated in §4 and asserted by a library test, and no command could reach any of them.
- ⛔ **The first run of the fixture census found two defects those tests had asserted around since the
  elaborator was written** (`27a7541`, `M1.4`, `2026-09-13` — 101 commits earlier).
  The census drove 26 tracked cases through the command, each declaring the *one* code it must produce, and
  compared the set. `(version one zero)` produced two: `module-bad-version`, and `module-missing-version:
  … declares no version` about the clause right above it. The library test checked
  `rendered.contains("module-bad-version")` — presence — and passed. And `module-empty` pointed at
  `docs/semantics/kinds/core.eadl:1:1`: its label hard-coded source 0, which in a library test is whatever
  was added first and in a command is the first shipped kind module.
- ⭐ **Reachability is a property of the pipeline, and it is where these live.** A library test controls
  what is in the source map and asserts the code it expects; a command decides both, and shows the author
  every message. Neither defect could be seen from inside the library.
- The rule had to be written before the loader, and it had one constraint worth recording: `M1.29`'s
  acceptance asked for resolution "by declared name and not by filename". Read literally — index the
  directory by what each file declares — §6 rule 4 (a declared name that differs from the imported one is
  refused) can never fire. So the file is *found* by the imported name and *verified* by the declared one.
- The freeze gate's first real client: `LANGUAGE-FREEZE` stayed red until `--emit`, went green with the
  pending note, and went red again with the note moved out — the behaviour `PROGRAM.27` had just made real.
- Promoted into `docs/knowledge/presence-checks-cannot-see-an-extra-key.md` as its second measured instance,
  with an `answers:` line.

## _(2026-09-29)_ — an arm that expects a refusal has to say what the refusal is about

- `PROGRAM.27`, making `LANGUAGE-FREEZE`'s explicitness leg able to fail. The defect was the documented
  one — the notes directory's README read as a pending note covering every construct — and every one of
  the gate's nine arms had been green through it, because they all pointed at a scratch notes directory.
- ⛔ **Arming the fix found two more arms that had never tested their names.** A new arm built the same
  way as "a construct nobody declared is refused" stayed green under a mutation that should have turned it
  red. Debugged directly: the classifier printed *"… follows …, and the file is generated sorted by id"*
  and exited 2 — the fixture appended its row at the end, so the whole file was refused as unreadable and
  the arm passed on leg A's "not a readable baseline file" note. Both pre-existing arms of that shape had
  done this since `2ae744a`.
- ⭐ The fix to the *class* is in the oracle, not the fixture: every refusing arm now carries the text its
  refusal must contain — the construct id, or the note file — so an arm cannot be satisfied by a refusal
  for another reason. Mutation P-F (the unsorted fixture put back) now reports *"refused, but not about
  `…`, so it refused for another reason"* for exactly those two arms.
- And the message that made it slow to see: the classifier stamped every problem with
  `docs/semantics/BASELINE.txt`, whatever file it had parsed, so a malformed scratch fixture read as a
  defect in the tracked baseline. `parse_named` carries the real name.
- A third hole of the same leg, measured rather than supposed: `M1.28.2`'s note stayed `status: pending`
  for five commits, silently covering any later movement of its five constructs. The new spent-notes leg
  refused it on its first run.
- Promoted into `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` — two lessons (point an arm at
  the deployed population; make a refusing arm name what it refuses) and a new `answers:` line.

## _(2026-09-29)_ — the census that sized a defect was wrong in both directions, and each error looked plausible

- `M1.31`, fixing a caret that overran its line. `M1.29.1` had sized the defect by census and published
  "23 of 78 tracked descriptions" on the leaf, in `MEMORY.md`, `LIVE_STATUS.md`, `CHANGELOG.md` and the
  task-tree index. Re-run after the fix, the same census still reported **38** files.
- ⛔ **Two independent errors, each pinned with a tool.** (1) The awk measured the whole marker line,
  *label text included*: a long label under a short line is not an overrun, and the instrument counted it
  as one — 38 before, 38 after. (2) The saved list was exactly 2048 bytes: `| tee census.txt | head -20`
  truncated it when `head` exited, and "23" was counted from the alphabetical head of the list, which cut
  off both kind modules — the two largest overruns in the repository.
- The corrected instrument measures indent + marker run only, writes to a file with nothing downstream,
  and was re-run on the *before* state from a stash: **14 runs over 10 files → 0**. Mutation M-E puts the
  first instrument's error back inside the new leg and it reports **53** phantom overruns on the fixed
  renderer, so the leg is proven to measure the run and not the label.
- Every copy of "23" is corrected in `M1.31`'s commit, and the historical copies say they were corrected
  rather than being silently rewritten (`docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md`).
- A second defect on the way: the first cut of the fix found the last line with `position(end - 1)`,
  which lands inside a multi-byte character, and twelve reference legs panicked at `source.rs:119`. The
  continuation is now counted over the covered text, and a leg whose span *ends* on `é` pins it.
- Promoted into `docs/knowledge/verify-the-mutation-applied.md` as the fifth and sixth parts of a census —
  the quantity it measures and the file it saved — with two new `answers:` lines.

## _(2026-09-29)_ — a help line is a claim about a capability, and it needs a leg in both directions

- `M1.29.1`. `archogen help check` said "elaborate and type-check" from the day `spec.rs` was created (`af4e6dc`, `2026-09-13`), and
  no production code called the elaborator: `git grep "elaborate("` outside `module.rs` finds two hits,
  both in `f01_f02_modules.rs`. Every gate near it was green — legs 4 and 5 pin all 24 `module-*` codes,
  `BOOK-ANCHORS` resolves every path `modules.md` cites — because each is an existence census, and the
  elaborator *exists*. It is the limiting case of `M1.28`'s shape: not a result discarded by its caller,
  but an emitter with no caller at all.
- ⭐ **The fix to the sentence is a leg, not an edit.** `module_files.rs` reads the `check` summary, runs
  a module file through the command, and asserts `summary.contains("elaborat") == (not refused)`. Mutation
  A (the old summary restored) fails it; so will wiring the elaborator without saying so. The refusal's
  owning leaf is held to the task tree the same way, so it cannot name `M1.29.2` after `M1.29.2` closes.
- ⛔ **The first honest answer was not available inside the pipeline.** `eadl_model::check` can only
  return a §5.5 verdict, and every verdict is about the *system*; "this tool cannot read a module yet" is
  about the tool. So a module file got the only verdict the passes could produce — `invalid-description`,
  exit 10, for a well-formed module — and the classification had to move ahead of the call, into the CLI,
  which owns the process statuses. `unimplemented`, exit 20, the same status `build --locked` returns.
- **A race found by the suite before any mutation ran.** Three legs wrote the same scratch file name;
  tests run in parallel and `fs::write` truncates first, so one `check` read the empty file, found no
  declarations and *accepted* it. Here that surfaced as a red leg; with an assertion that expected
  acceptance it would have been a green one for the wrong reason. One name per leg, recorded on the helper.
- Promoted into `docs/knowledge/an-existence-census-cannot-see-a-discarded-result.md` as its fifth
  instance, with a new `answers:` line — "The help text says the command does X and a library does X — how
  do I know the command calls it?"

## _(2026-09-29)_ — an empty diff from a census is a claim about the census, not about the tree

- `M1.28.2`, making `archogen check` refuse a quantity it cannot read. The change was a language change, so
  the acceptance needed "no description that reads today is refused tomorrow" — and the way to say that is
  a census, not an argument.
- The instrument: every tracked `.eadl` driven through `archogen check` at the parent commit and at this
  one, the two outputs diffed. The first cut recorded `tail -1` of each run. Its diff was **empty**.
- ⛔ **The empty diff was not a result.** The one description that had changed kept its *verdict*
  (`invalid-description`) and changed only *which code* produced it — `schema-unknown-clause` →
  `quantity-non-positive-frequency` — and the last line carries the verdict and a count, both of which
  were identical. Re-run recording the verdict **and every code**, the same census reported
  **76 descriptions, 1 changed, 0 acceptances lost**, which is the figure the leaf publishes.
- ⭐ **Why this is worse than a weak oracle.** A weak arm fails to fire and you can see that it did not.
  An under-recording census *succeeds*: it prints the answer you were hoping for, and the only way to
  catch it is to ask which fields were compared. Promoted into
  `docs/knowledge/verify-the-mutation-applied.md` as the **fourth** part of the general rule beside the
  needle, the mutation and the oracle, with a new `answers:` line — "My before→after census shows no
  difference — did nothing change, or did I record too little?"
- The second lesson is the one the census was written for, and it is about *which mechanism a leg asks*.
  Reading every offered value as a quantity refused
  `docs/semantics/boundary/accept/counter-width-and-rate.eadl` — verdict `accept`, one of the ten F27 reads
  — because it holds `(counter-modulus 4294967296)`, a count. No leg saw it: `f27_boundary.rs` asks the
  **classifier** and the **schema**, and neither reads a fact's value, because `(offers …)` is
  `(holds forms)` and `core.eadl` is explicit that this layer does not interpret it. The pipeline does, and
  nothing ran the pipeline over that corpus. `f27_every_accepted_case_is_accepted_by_the_whole_pipeline`
  exists now; mutation **F** (the guard widened back) fails exactly it and the lone-number leg, which is
  what identifies them as arms rather than decoration.
- ⛔ The discard that caused all of this was also hiding a contradiction between two shipped surfaces:
  `quantities.md` said "a number in an eADL description **always** carries a unit" and rendered the
  refusal, while that accepted case shipped a bare number. Both were true only because no refusal ever
  reached an author. Filed as **F-N** and corrected in the chapter, which now re-renders all three of its
  transcripts from `archogen check` over three **tracked** cases instead of over a `platform.eadl` that
  exists nowhere — `PROGRAM.20`'s seventh shape, closed rather than registered.
- promotion: promoted (`docs/knowledge/verify-the-mutation-applied.md` gains the fourth part of its general
  rule, a worked instance, a `How to apply` bullet and an `answers:` line; the INDEX row widened to match.
  The `f27` lesson is deliberately **not** a new card — it is the fifth instance of
  `docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md` and is recorded on `M1.28.2`'s leaf beside
  the mutation that proves the new leg bites, because a sixth card answering the same question makes the
  retrievable layer harder to search)

## _(2026-09-29)_ — the consumer facing the author was the one that had re-implemented the rule

- `M1.28.1`, closing F-M: `archogen build` classified a description defect as a toolchain failure and
  exited **70** for a mistyped unit symbol.
- ⛔ The rule "which verdict does a diagnostic code that is not a §5.5 verdict slug carry?" had **three**
  consumers and **two** answers, and `grep -rn "Verdict::parse" crates/ --include='*.rs'` shows the split
  in one screen: `crates/eadl-model/src/check.rs:212` and `:223` both
  `unwrap_or(Verdict::InvalidDescription)`, `crates/archogen-cli/src/build_cmd.rs:136`
  `map_or(Status::ToolFailure, …)`. The two that were right are internal — they feed a precedence
  comparison. The one that was wrong prints to stderr and sets the exit code a script branches on.
- ⭐ **That is the shape worth naming: the consumer facing the author is the one most likely to have
  re-implemented the rule**, because it is the furthest from the crate that owns it and the least likely to
  be reached by that crate's tests. `grep -rn "quantity-" crates/archogen-cli/tests/` → **no match**: no
  fixture ever drove a non-verdict-shaped code through `build`, so the divergence had nothing to fail on.
  Second instance of `M1.13.4.1`'s lesson in four commits, and the prescription is identical — extract the
  rule into one named accessor at the lowest level that can carry it, so a fourth consumer cannot pick its
  own default without deleting the function.
- **Four mutations, because three of them test different hazards.** Restoring the old default fails only the
  new end-to-end arm; setting the accessor's default to `ToolFailure` reproduces the original defect as a
  mutation; setting it to `Ok` fails two arms, which is the one that pins *silent acceptance* — a hazard
  nobody reported and the more dangerous of the two, because an unrecognized code that meant `ok` would
  have made `archogen check` print nothing at all; and making the accessor ignore its argument fails the
  totality arm, which is what proves `of_code` does not simply shadow `parse`.
- ⚠️ The before→after was taken from a **stashed tree**, not from memory or from the leaf's own text: the
  leaf quoted `exit=70` from the filing measurement, and re-running it against `git stash` confirmed the
  number rather than carrying it. `git stash pop` restored four modified files and
  `git status --short` agreed with the pre-stash listing.
- promotion: declined (recorded on `M1.28.1`'s leaf with the reason — a second instance of a shape whose
  prescription is now a named function, cited in that function's own doc comment where the next consumer
  reads it; this repository promotes at recurrence, and
  `docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md` was declined at one instance and
  promoted at four)

## _(2026-09-29)_ — every leg was an existence census, and the discard was in the caller

- Found by picking up `M1.26.1`, not by looking for it. The first question a normative document forces is
  "which pass surfaces each of these codes?", and for the eight `quantity-*` codes the answer is: one,
  and it is the prototype.
- ⛔ **`archogen check` accepts a description `archogen build` cannot realize.** `examples/s0-heartbeat/system.eadl`
  with `(period 10 ms)` → `(period 10 parsec)` gives
  `accepted against profile rt-static-up-v1 (4 declaration(s))` from `check` and
  `error[quantity-unknown-unit]: `parsec` is not a known unit` from `build`. A zero tick rate —
  `ROADMAP.md` §13.1's **F03** — gives no diagnostic from `check` at all.
- Root cause is three discards and one absent notion, and each discard is locally reasonable:
  `crates/eadl-model/src/workload.rs:178` (`.map_err(|_| ())`), `crates/eadl-model/src/refinement.rs:177`
  (`.ok()?`) and `:188` (`.ok()`) all read a quantity and drop what they find; `crates/archogen-s0/src/interpret.rs:200`
  is the only consumer that propagates. And the schema *cannot* ask the question: `period`,
  `min-separation`, `deadline` and `jitter` are declared `(holds values number symbol)`, so `parsec` is a
  symbol and passes correctly, while `ValueType::parse` (`crates/eadl-model/src/kind.rs:156`) has no
  quantity. `workload.rs:176` names "the schema/unit pass" as the reporter — there is no such pass, and
  the comment is why the discard reads as safe.
- ⭐ **Why every gate stayed green, which is the lesson and not the incident.** Legs 4 and 5 of
  `crates/eadl-front/tests/reference.rs` pin §4's codes against the sources that emit them, exactly and in
  both directions; leg 8 pins every code the book renders against the codes a production half emits;
  `BOOK-ANCHORS` pins every citation. All of them are **existence** censuses, and existence is a property
  of the *emitter*. `f03_units.rs` proves the refusal exists and precedes arithmetic — the property F03
  names — and it drives `Quantity::read` directly, one level below the fixture §13.1's gate reads. Nothing
  in the repository drove a description through the command an author runs.
- Three more instances of the same shape fell out of the same hour, and all four are on the leaves:
  **F-H** (§4's `module-too-large` row states a mechanism `grep` cannot find in the elaborator, and both
  census legs are green because they read the code literal and never its predicate), **F-J**
  (`quantity-invalid`'s only site is a `map_err` catch-all over a function that returns two variants, so
  the arm is totality and no input reaches it), and **F-K**
  (`docs/semantics/cases/invalid-zero-clock-frequency.eadl` gets its expected `invalid-description` from
  `schema-unknown-clause`, because `refines` is `defplatform`'s clause and the case puts it on a
  `defblock` — so the suite's one F03 case exercises a clause-name typo and passes).
- Filed as **`M1.28`**, priority **high**, and sequenced *ahead* of the leaf that found it: `M1.26.1`
  cannot state the quantity rules normatively while `check` does not enforce them, because a normative
  document that says "refused" over a pipeline that accepts is born false rather than becoming false.
  The fix is a language change and costs a migration note — `ValueType`'s list is §4's
  `schema-bad-value-type` repair direction and `docs/semantics/kinds/os-rt.eadl` is digested at
  `docs/semantics/BASELINE.txt:76` — and the narrow fix (propagate from the three sites) is recorded on
  the leaf as **worse than doing nothing**, because it reports the clauses those two passes happen to
  read and leaves every other quantity silent, which looks fixed.
- promotion: promoted (`docs/knowledge/an-existence-census-cannot-see-a-discarded-result.md`, a new card
  with the four instances in a table and an `answers:` list headed "My gate proves every diagnostic code
  is stated and emitted — what could still be wrong?". The prescription is a second census over the
  **consumers** plus one input driven through the user-facing command, and the card names the trap in the
  obvious fix)

## _(2026-09-29)_ — an existence census cannot see a rule that is stated falsely

- `M1.26`'s decomposition, measuring the two gaps it inherited before writing either child. The census
  was run to size the work; it found a defect instead.
- ⛔ **`docs/semantics/reference.md` §4 states a mechanism that does not exist.** The row reads
  `module-too-large` | "a module has more addressable parts than an instance identifier can hold" |
  "split the module". `grep -rn "module-too-large" --include='*.rs' --include='*.md' .` → exactly one
  call site, `crates/eadl-front/src/module.rs:605`, and it fires when `SourceMap::add` returns `Err`;
  `crates/eadl-front/src/source.rs:198` shows `add` fails only on `u32::try_from(text.len()).is_err()`,
  i.e. a source of **2^32 bytes**. `grep -rn "addressable\|instance identifier" crates/eadl-front/src/module.rs`
  → **no match**: there is no instance-identifier width and no addressable-part count anywhere in the
  elaborator. The same `SourceError::TooLarge` condition surfaces **four** ways — `module-too-large`,
  `tool-failure` (`crates/eadl-model/src/check.rs:95`), and two bare `archogen:` lines that are not
  diagnostics at all (`crates/archogen-cli/src/{build_cmd.rs:113,check_cmd.rs:90}`).
- ⭐ **Why both existing legs are green on it, and why that is the argument for gap (b) rather than an
  objection to it.** Legs 4 and 5 of `crates/eadl-front/tests/reference.rs` pin §4's code set against the
  sources that emit it, in both directions — an *existence* census, and an exact one. A row can be
  emitted, stated, repair-directed and still describe a rule the code does not implement, because the
  census reads the code's `Diagnostic::error("…")` literal and never its predicate. What would have seen
  it is the thing §4 does not have: an **input** per row. Asking "what input fires this?" answers
  "nothing writable" for `module-too-large`, and the follow-up question "then what does the call site
  actually test?" is one `sed` away from the false sentence.
- Filed as **F-H** on `M1.26`'s leaf with its four censuses and owned by `M1.26.2`, which is the leaf that
  writes the column and therefore the one that has to decide between correcting the row, implementing the
  rule the row states (a language change, so a migration note), and removing the code. Not fixed here:
  the choice changes what an author is told, and `M1.26.2` is where the evidence for it lands.
- ⛔ **Three of the leaf's own recorded figures were stale on pickup**, which is the defect class the leaf
  exists to close: §4 is **60** rows and not 55 (`M1.13.3` added five after `M1.13.1` measured); the
  ungoverned population is **16** and not 15, the extra being `analysis-inconclusive` from
  `crates/archogen-s0/src/interpret.rs`, which a census over six *named* files could not see; and the
  "27 codes named by no test" count survives with a composition that does not. Fifth instance of
  `docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md`, recorded there.
- promotion: promoted (`docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md` gains the
  fifth measured instance and a note that the class compounds at a leaf whose *subject* is stale figures.
  F-H's own lesson — an existence census is blind to a falsely stated rule — is deliberately **not**
  promoted yet: it is promoted by `M1.26.2`, which lands the instrument that sees it, because a card
  written before the fix exists would state the blind spot without the mechanism that closes it)

## _(2026-09-29)_ — the arms a leaf says exist are worth counting

- `M1.25`, converging the reach gate on an explicit list of historical lines instead of a past-tense
  substring scan. The leaf's acceptance said: "all **five** existing arms are re-expressed against the
  new mechanism — including the one that strips the past tense off a legitimate historical figure".
- ⛔ Measured before designing: `grep -c '^fn arm_' crates/eadl-model/tests/kinds.rs` → **0**. The gate
  this leaf was about had **no arms at all**, and the seven arms the leaf was probably describing belong
  to `corpus.rs`'s *boundary-corpus* figure gate — a different gate, over different surfaces, written by
  `M1.24`. So the slice was not a refactor of five arms but a build from nothing, and the gate that closed
  `M1.23` had never been observed firing.
- That changed what the work had to prove. Arm 1 is now the original defect replayed as a fixture, and
  the discriminating evidence is a **meta-mutation**: putting the marker scan back makes exactly two arms
  fail — the one for the explicit list and the one for the past-tense escape — while the other four still
  pass, so those two arms demonstrably test the mechanism this leaf replaced rather than decorating it.
- ⭐ Fourth instance of the same shape, which is why it is promoted this time: `M1.13.3`'s "a new
  top-level form is a grammar change" (the grammar names no construct vocabulary), `M1.13.4`'s
  content-identity exclusion (the frozen evidence copy was byte-identical to a suite file),
  `M1.13.4.4`'s "minimal **glob** semantics" (a path prefix is the whole matcher), and this one. A leaf is
  a plan written before the work; its claims about the repository are hypotheses until measured.
- promotion: promoted (`docs/knowledge/a-leafs-claims-about-the-repository-are-hypotheses.md`, new card
  with four measured instances in a table and an `answers:` list headed "My task leaf says the mechanism /
  arms / figure already exists — do I trust it?". The `2026-09-27` entry declined this lesson on the
  grounds that "verify a claim before acting on it" is the claim-verification policy — which this
  repository had **not** adopted then and has since, and a policy document is not searchable by the
  question a session actually has. Four instances is the recurrence the decline named as the signal.)

## _(2026-09-29)_ — an independent re-computation that disagrees is usually the one that is wrong

- `M1.13.4.5`, writing `eadl/1`'s frozen-construct baseline: Rust extracts each construct's text and
  emits NUL-framed records, shell hashes them, because the workspace carries zero dependencies and so has
  no hasher. Two processes agreeing on bytes is a claim, so it was cross-checked twice — the grammar's
  EBNF fence re-extracted and hashed in python, and one machine-read table the same way.
- ⛔ The fence agreed and **the table did not**. The instrument was right: the cross-check kept consuming
  blank lines after the table ended and so hashed trailing newlines the instrument's rule stops before.
  The rule (contiguous `|` lines after the marker, blanks allowed only before the header) was in the code
  and not in the check — and a cross-check that reimplements a rule from memory is not independent, it is
  a second guess with a confident output format. Re-written from the stated rule, both agreed; a third
  check crossed an implementation boundary rather than a language one (a suite file's canonical form
  re-printed by `diagnose` and hashed) and agreed too.
- ⛔ **The pipeline also shipped a bug the digests hid.** `records+="$(printf '%s  %s\n' …)"` — command
  substitution strips the trailing newline, so all 70 records reached `sort` as a single line and
  `sort -k2` on one line returns one line. Every digest was correct and the file was unusable. It was
  visible only by reading the emitted text rather than the exit code, and it is now a comment in
  `scripts/language_baseline.sh` naming the failure it prevents.
- ⭐ The framing byte is NUL because the payload is canonical text and §3 rule 3 of
  `docs/semantics/reference.md` forbids a control character in it — a documented reason, not a lucky
  choice. A frame the payload can contain silently merges two constructs and both digests still look
  fine.
- promotion: promoted (`docs/knowledge/an-oracle-is-independent-by-construction.md` gains the section
  "When the two sides are byte streams, suspect the cross-check first" and a fifth `answers:` line —
  *"My digest pipeline and an independent re-computation disagree — which one is wrong?"*. Same card as
  the oracle-independence lesson, because the statement is the same: independence is a property of how
  the second opinion was produced, not of the fact that there are two.)

## _(2026-09-29)_ — the third consumer of a rule is in a file kind no fixture uses

- `M1.13.4.1`. `M1.13.3` put `(eadl-version eadl/1)` on the surface as a top-level **form** and taught
  the two consumers that refused it: `module.rs` (which counts a module file's forms) and `check.rs`'s
  schema pass (which validates each form against the kind registry). Both were found the right way — by
  running the real entry point and reading the refusal — and that leaf closed green with 16 new tests.
- ⛔ There was a **third** consumer, in the same file: `shipped_registry`, the loader that reads
  `docs/semantics/kinds/`. It handed every top-level form to `read_kind`, so a kind module stating its
  own version was refused as `schema-not-a-kind` and the whole registry failed to build —
  `archogen: tool-failure: the shipped kind modules could not be loaded` for *every* description. No
  description-level test can reach it, because nothing reads a kind module except that loader, and no
  fixture had ever put the new construct in one.
- What found it was not a search for consumers. It was writing the identifier into **all 62** shipped
  descriptions — every file kind the toolchain reads — and running the whole suite: `45 failed / 447
  passed over 9 suites`, of which **36** were this one consumer and 9 were legs that treat every
  top-level form as a declaration. The measurement was a temporary edit, reverted, with the restoration
  proven (`git status --porcelain` → 0 entries).
- The rule: **when a language gains a construct, put it in one file of every kind the toolchain reads
  before deciding the change is complete.** A rule's consumers are a population, and only the ones your
  fixtures reach will tell you they are missing. Census them by *shape* (`for form in &document.forms`),
  never by the construct's name — a consumer that has never seen the construct cannot be found by
  searching for it.
- ⭐ The fix is an accessor, not a third hand-rolled filter: `language_version::declarations()` sits
  beside `is_identifier` and every pass that treats a form as a declaration goes through it, including
  the field `archogen check` counts when it prints `(N declaration(s))` — a line
  `docs/book/src/checking.md` publishes, so a consumer that counted forms would have printed one too
  many for every description that states its version. The same run found the shape's test-side copy:
  three test helpers hand-rolled a kind-module loader, so the suite could have stayed green on a loader
  that could not read the shipped files. They now call the production one.
- promotion: promoted (`docs/knowledge/enumerate-the-population-from-the-specification.md` gains the
  section "The population has a second axis: the kinds of file a rule reaches" and a fifth `answers:`
  line — *"I added a construct to the language and every test passed — which reader have I not told?"*.
  Same card as the input-population lesson, because both are one statement about two populations: green
  over the population you supplied says nothing about the population you did not.)

## _(2026-09-29)_ — a fix in a caller is invisible to a test of the callee

- `M1.13.3`, adding the language-version identifier. §8 made `(eadl-version eadl/1)` a top-level form;
  `crates/eadl-model/src/check.rs` validates every top-level form against the kind registry, so the new
  form was refused as `schema-unknown-kind` — the rule contradicted end-to-end by the layer furthest
  from it, and invisibly: no frontend test can see a model-layer refusal.
- The exemption went in `check.rs`, in front of `validate`. My first test of the break called
  **`validate` directly**, so it kept failing after the fix landed and would have kept failing whatever
  `check.rs` did. ⛔ A test that bypasses the layer under test measures the wrong thing *confidently* —
  it was red before the fix and red after, which reads as "the fix did not work" and is actually "the
  test is not looking at the fix".
- The rule: **test through the entry point the real consumer uses.** Here that is `check`, which is what
  S0 and the corpus suite call. Calling the inner function is fine for unit-testing that function; it is
  not evidence about a caller's behaviour, and an exemption is always a caller's behaviour.
- Same slice, same shape, one level down: the leaf's own design premise was false. It said a new
  top-level form is a grammar change. `grammar.md:109` says the grammar names no construct vocabulary at
  all — "the language is extended by `defkind`, not by editing this file" — so the form was already
  well-formed and nothing downstream of the grammar moved. **A cost argument is a claim; read the thing
  it claims about.**
- promotion: declined (one instance, and the transferable half — "test through the entry point the
  consumer uses" — is a practice this repository already follows everywhere else, so a card would be
  searchable by a question nobody asks. Recorded here instead, where the next session reads it. If it
  recurs, that is the signal to promote it, and `a-gate-is-only-as-sharp-as-its-fixtures` is the card it
  would join rather than a new one.)

