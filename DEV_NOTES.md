# DEV_NOTES.md

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
