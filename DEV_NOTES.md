# DEV_NOTES.md

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
