# DEV_NOTES.md

Engineering lessons, newest first. This file is a rolling ledger: whenever it holds 20 notes, its oldest 10 are
sealed, byte for byte, into the next segment under `docs/history/dev-notes/`, listed with its digest in
`docs/history/INDEX.md` (`docs/decisions/decision_history-ledgers.md`).

## _(2026-10-22)_ — a deadline sized from a quiet run reads a busy host as a hang

- `PROGRAM.99`. The `xtask` suite failed 31 and 73 tests beside other builds and passed alone. The 30 s limit on a
  bounded child had been set against a 6.23 s slowest body; eleven days later the slowest took 11.78 s, a 2.6× margin.
- ⛔ The hypothesis handed over — macOS's signature checks serialising every exec — did not survive its first
  measurement: a fresh program's exec took ~140 ms quiet and the same under 12 workers. The tool refused to claim that
  load (exit 1) instead of letting a weak load stand in for the real one, and 64 workers measured ×3.9 and reproduced.
- ⛔ The failures were lost twice before they were read: libtest captures a passing test's stderr (the timings needed
  `--nocapture`), and the tier runner keeps only 40 lines of a failed step (`PROGRAM.100`).
- The rule that came out of it: a deadline on what must finish is a safety net only a hang reaches; a deadline on what
  must be stopped is tight, against a hang that outlasts every bound the test asserts. One limit shared by both sides
  — the sweep's `--timeout 1` over a baseline that must pass and a mutant that must hang — is the defect's own shape.

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

