# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. The
`bedrock-scaffold` entries below the separator are the provenance of the discipline spine
this repository was created from, not archogen's own history.

## archogen — a runtime that decides and does not act

`ARCHOGEN-M2-0034` (leaf `M2.1`).

⚠️ *This entry was written after the fact: the `M2.1` commit's lockstep script aborted on an
assertion before reaching `CHANGELOG.md`, and the failure was not noticed because the rest of the
batch reported green. Restored in its own commit rather than folded into a later one.*

- `crates/rt-core` is the shared runtime state machine: task lifecycle, fixed-priority ready
  structure, nested interrupt masking, and the bounded fault path. `no_std` outside its own tests,
  and allocation-free — the task set is a fixed-capacity array sized by a const parameter.
- ⭐ **It never performs a context switch.** Every operation returns a `Decision`; saving
  registers and returning from an interrupt belong to the architecture port. That is §8's
  "separate policy state transitions from the execution substrate", and two things follow: the
  policy is testable on a host, which is the leaf's acceptance, and a scheduling bug stops being
  the same bug as a context-save bug.
- Masking **nests**, and a release arriving while masked is **latched**, delivered on the
  outermost unmask in priority order. ⛔ A second release while one is pending is an **overrun**,
  not a second pending job: there is nowhere to put it, and inventing somewhere would be a queue
  in a profile that excludes queues.
- The fault path is a **value, not a panic**, so a hosted test can observe it and the target port
  can route it to a defined fatal handler. Faults split by whether the runtime's own state is
  still trustworthy — only an overrun is. There is no "ignore" overrun policy.
- ⚠️ `#![no_std]` is **verified**, not asserted — using `String` outside `cfg(test)` fails to
  compile. The integration tier gained a `no-std-build` step for the stronger claim, which the
  installed `riscv64imac-unknown-none-elf` target now makes pass.
- Validation: `cargo test -p rt-core` → **3 + 14 passed, 0 failed**. `cargo test --all` → **374**
  passed (357 before, `+17`); `make focused` → `passed`; all doctrines green; `mdbook build` OK.

## archogen — the book gets a shape, and its citations get checked

`ARCHOGEN-PROGRAM-0033` (leaf `PROGRAM.4`).

- Eighteen chapters had accreted into a flat list — one per leaf, no structure. `SUMMARY.md` now
  carries five parts that mirror the programme: what eADL describes · writing a description ·
  what the engine may claim · generating and running a system · using the toolchain.
- ⭐ **The second half of this leaf's acceptance is now mechanical.** `BOOK-ANCHORS` checks that
  every chapter describing behavior **cites** a repository path, and that every path any chapter
  cites **exists**. The book is the project's public surface and, for its director, the only
  window into it — the code is not read, the book is — which makes a confident chapter describing
  something the engine no longer does the most expensive drift available here.
- The census found a real one: **`presence.md` cited nothing at all.** A chapter with no anchor
  cannot be checked against anything, by a script or by a reader; it is an essay about a system
  rather than a description of one. It now names `crates/eadl-model/src/presence.rs`, its F04–F06
  test file, and the worked-case directory.
- ⛔ The false positive was designed out rather than discovered: chapters legitimately name
  `src/main.rs` of a **generated** crate and `os-rt.eadl` by basename, and requiring those to
  exist at the repository root would make writing about generated output cost a doctrine breach.
  That is the lesson from `S0-RETIREMENT`'s first run, applied in advance.
- Validation: `scripts/check_book_anchors.sh` →
  `OK (18 chapter(s); every cited repository path resolves)`; `--self-test` → `3 pass / 0 fail`
  across three RED arms — an unanchored chapter, a rotted citation, and a **well-formed** chapter,
  because a check that always fails is not discriminating either. `cargo test --all` → **357**
  passed, unchanged (no Rust touched); all doctrines green; `mdbook build` OK with the new parts.

## archogen — F29 is green, and the two independent sources agree interval for interval

`ARCHOGEN-M2-0032` (leaf `M2.5`).

- §13.4's repeated-preemption fixture is checked **two ways that are not derived from each
  other**: the roadmap's own expected-trace table, parsed out of `ROADMAP.md`, and a simulator
  written from the operational rules in the prose above it. They agree on all twelve intervals of
  `[0, 23)` — per-category totals `L 8`, `H 4`, dispatch `1`, ISRs `2`, switches `8`, total **23**
  — with `H` completing at 9 and 19 (response 5 from *nominal* release) and `L` at 23.
- All four controls behave as §13.4 states. Omit the timer ISR cost → `L` at **21**, the false
  pass at deadline 22. Omit the `H`→`L` resume switch → `L` at **14**. Charge the two ISR
  intervals again inside task cost → a would-be total of **25** that cannot seal as an exact
  trace — no detection logic needed, the ledger from `M2.4` simply refuses.
- ⛔ **The controls re-simulate, and the third one proves why that matters.** Deleting four units
  of resume cost does not give `19`: `L` finishes at exactly 14, the instant of the second nominal
  release, and "record completion before processing the new release" then removes that release's
  interference altogether — one interfering job vanishes. §13.4 warns about this directly
  ("subtracting a fixed number from the original response is not generally valid"), and a control
  built by arithmetic would have agreed on the second row by luck and been wrong here.
- The two switch directions are separate cost fields precisely because one control deletes one of
  them. A single `switch` field would make that control inexpressible without editing the
  simulator, which is not a control at all.
- ⚠️ F29 remains a **synthetic accounting fixture**: concrete cost coverage and three known
  mistakes detected. Not a benchmark, not a claim about any board, and not a substitute for
  `M2.6`'s review of the runtime accounting model and its theorem conditions.
- Validation: `cargo test -p rt-analysis --test f29_preemption` → **8 passed, 0 failed**; proven
  able to fail by mutating the simulator — moving the observation boundary past the switch away
  (`[11, 21]` instead of `[9, 19]`) and processing a coincident release before the completion
  (`19` instead of `14`), each restored and re-run green. `cargo test --all` → **357** passed
  (349 before, `+8`); `make focused` → `passed`; all doctrines green; `mdbook build` OK.

## archogen — an accounting rule you cannot construct a violation of

`ARCHOGEN-M2-0031` (leaf `M2.4`).

- `cost-accounting/1` is published (`docs/analysis/cost-accounting-v1.md`) and declared as data
  beside the code, with a drift test holding the two together. §7.4.1 requires a **versioned**
  contract because a total is only meaningful under the rules it was computed with, and §15
  versions evidence formats separately for that reason.
- ⭐ **"One interval, one category" is a constructor precondition, not a review instruction.** An
  exact-trace ledger with an **overlap** (time charged twice — §7.4.1's own example, and F29's
  fourth control) or a **gap** (time charged to nothing — which is what an omission *is*) does not
  seal. Both matter for the same reason: when they happen the total still looks plausible, so a
  check that depends on someone noticing never fires.
- §7.4.1's permitted pessimism is preserved rather than legislated away. A `safe-envelope` may
  over-count — that is what makes it safe — and the kind of total is named, so the conservatism is
  declared instead of implied. An `observed-maximum` is neither, whatever it is multiplied by.
- The drift test **caught a real mismatch on its first run** — a code span around `C` — and was
  then taught to compare meaning rather than markup, because a drift test that forces the page to
  be worse in order to stay green is one people work around instead of satisfying.
- This is the third time the project has answered a "must not" with a type rather than a review
  step (`Conclusion` for §7.1, `TaskSet::admit` for §7.4, `Ledger::seal` here), so the pattern is
  now `docs/knowledge/make-the-rule-a-constructor-precondition.md`.
- Validation: `cargo test -p rt-analysis` → **27 + 5 passed, 0 failed**; proven able to fail by
  disabling overlap detection (`25 passed; 2 failed`) and by editing the published contract away
  from the declared one (`26 passed; 1 failed`), each restored and re-run green.
  `cargo test --all` → **349** passed (336 before, `+13`); `make focused` → `passed`; all
  doctrines green; `mdbook build` OK.

## archogen — the first analysis, and it establishes less than it looks like it does

`ARCHOGEN-M2-0030` (leaf `M2.3`). **F18 is green.**

- `crates/rt-analysis` implements §7.4's idealized zero-overhead response-time recurrence with
  exact integer arithmetic, checked at every operation, upward rounding written so it cannot
  overflow where the textbook `(n + d - 1) / d` does, two explicitly named limits, and the full
  iterate sequence kept as a witness.
- The §13.2 baseline yields exactly the published bounds — A `1`, B `2`, C `4` — and C's witness
  is `C: 2 → 4 → 4`, which is the sequence §13.2's prose describes rather than merely its answer.
  Changing only C's deadline to 3 produces a counterexample carrying that same sequence.
- ⭐ **A positive answer cannot be detached from the eight conditions that make it true.** There
  is no function returning "schedulable": the only positive conclusion available is §7.1's
  conditional form, which will not exist without a named model and a non-empty assumption list.
  A reader who quotes it quotes "no overhead" with it — the best defence available against the
  misuse §7.4 warns about most loudly, since no type can tell an idealized model from a board.
- A task set the model does not cover is **refused at construction**, not analyzed and caveated.
- Three outcomes, and the third is the one that is easy to get wrong: converged-within-deadline,
  converged-past-deadline (a witness), and **inconclusive**. §7.4 is explicit that a conservative
  failure is `not-established` unless an exact test or validated counterexample establishes
  failure — so non-convergence and overflow never become a deadline miss.
- ⭐ **The F18 oracle is parsed out of `ROADMAP.md` §13.2, not copied into the test.** §14.1
  forbids implementation changes that "silently … adjust expected oracle results", and a test
  holding its own copy makes exactly that a one-line edit that looks like a fix. The parser
  asserts the row count, because a table that quietly shrank would leave a green test checking
  less than it did.
- ⚠️ Nothing in this crate may be cited for a claim about a **running** system. §7.4 requires the
  variant that charges critical sections, jitter, interrupt interference and switch costs first —
  leaf `M2.6`, controlled by F29.
- Validation: `cargo test -p rt-analysis` → **14 + 5 passed, 0 failed**; proven able to fail by
  mutating the subject — ceiling → floor gives `B … gave 1 (witness B: 1 → 1)`, and `hp(i)` →
  the whole set turns `A` inconclusive; restored and re-run green after each. `make focused` →
  `passed`; `cargo test --all` → **336** passed (317 before, `+19`); all doctrines green;
  `mdbook build` OK.

## archogen — the five verification tiers become five commands

`ARCHOGEN-PROGRAM-0029` (leaf `PROGRAM.3`).

- `cargo xtask verify --tier <focused|integration|extended|hardware|assurance>`, also reachable
  as `make focused` / `make integration` / `make tiers`. Tiers are declared as data, the idiom
  `spec.rs` already uses here.
- ⭐ **The verdict has three states, and the third is the point.** §14.3 says "a required tool
  skipped or unavailable is reported as such, **not a passed check**", which two states cannot
  express. So: `passed` (exit 0), `failed` (1), and `incomplete` (20) — nothing failed, and
  something could not be run. The two reasons are kept apart because the response differs:
  **unavailable** (a tool is missing from this machine) and **not built** (the step does not
  exist, and names the leaf that owns building it).
- ⭐ **Four of the five tiers report `incomplete`, and that is the runner's most useful output.**
  Before it, the fuzz corpus, the mutation harness, the Miri wiring, the board and the entire
  assurance story were not reported as missing — they were simply not mentioned, which reads
  identically to being covered. Each now names an owner: `PROGRAM.9` (the extended tier's three
  steps, opened by this leaf), `M5.1` (no board), `M3.6` / `M4.8` / `M4.7` (assurance).
- CI now runs the **same object** the developer runs: `.github/workflows/rust.yml` calls
  `cargo xtask verify --tier focused` instead of listing three steps that someone had to keep
  identical to the Makefile's three. Wiring the integration tier into CI is `PROGRAM.10`, and the
  deliverable there is a decided answer to whether `incomplete` should block a build — wiring it
  up first would produce either a permanently red CI people learn to ignore or a green one that
  hides the gap.
- `COMMIT.md` adopts the director's CI policy explicitly: `make focused` per ordinary commit,
  `make integration` before a push or a milestone close.
- ⛔ The runner found two defects on its first two runs, which is the argument for it. It
  reported `❌ fmt … FAILED` **with no reason** — `cargo fmt --check` writes its diff to stdout
  while only stderr was captured — and its own leaf-existence test failed on `PROGRAM.9`, a leaf
  no tree declared, because the shape test above it cannot tell `M9.9` from `M4.8`.
- Validation: `cargo test -p xtask` → **8 passed, 0 failed**; `focused` → `passed`, exit 0;
  `integration` → `incomplete`, exit 20, naming QEMU. `cargo test --all` → **317** passed (309
  before, `+8`); fmt and clippy clean; all doctrines green; `mdbook build` OK.

## archogen — the S0 prototype gets an enforced expiry — **S0 complete**

`ARCHOGEN-S0-0028` (leaf `S0.6`). The `S0` tree is closed and **F28 is green end to end**.

- `docs/decisions/decision_s0-retirement.md` lists the **ten** hard-coded assumptions the S0 path
  rests on, each with the task-tree leaf that removes it, plus what survives retirement and what
  does not.
- ⭐ **The list is enforced, not maintained.** §12 S0 names a slow failure — nobody *decides* to
  grandfather a prototype, it just stops being noticed — and a document kept by memory drifts from
  the code it describes. So each assumption is marked `S0-ASSUMPTION: <id>` **at the line that
  makes it**, and the new `S0-RETIREMENT` project doctrine checks three things on every commit:
  every marker is listed; every listed assumption names a leaf some tree actually declares
  ("removed later" is not an owner); and the prototype has acquired **no consumers** beyond its
  declared ones. The third is what grandfathering actually looks like in practice — `M4` building
  on the prototype rather than replacing it.
- ⛔ The check's **first run was a false positive**: it flagged `crates/archogen-cli/src/lib.rs`,
  whose module documentation names `archogen_s0` in a sentence. Matching a mention rather than a
  `use` would have made documenting the prototype cost a doctrine breach, which is how a gate
  teaches people to route around it. The consumer test now matches `use`, `::` and
  `extern crate`, and the reason is written into the check.
- ⚠️ Honest limit, stated in the check itself: nothing forces an author to *write* a marker for
  something new. It makes an unmarked assumption a thing someone chose not to record, rather than
  a thing nobody noticed.
- Validation: `scripts/check_s0_retirement.sh` →
  `OK (10 assumption(s), each marked, listed and owned)`; `--self-test` → `3 pass / 0 fail`
  across three RED arms (an unlisted marker, a leaf no tree declares, a listed assumption with no
  marker), each restoring the record afterwards. `cargo test --all` → **309** passed, unchanged —
  the markers are comments. Fmt and clippy clean; all doctrines green; `mdbook build` OK.

## archogen — generated output that says where it came from

`ARCHOGEN-S0-0027` (leaf `S0.5`).

- `archogen build` now writes `provenance.json` beside the generated crate: for each generated
  declaration, the line it was written to, the byte span of the source form it came from, and the
  engine rule that produced it. That closes F28's last clause — "failure points have useful
  diagnostics **and basic source provenance**".
- ⭐ **The failure mode is not an absent record; it is a record that points somewhere wrong.** A
  line number that does not contain the declaration it claims sends a reader somewhere
  confidently wrong. So every record is resolved from **both ends** by the suite — the named line
  of `src/main.rs` must carry that declaration, and the named byte span of the description must
  cover it — and both checks are mutation-tested. Pointing every task at the first task's span,
  and shifting one line number by one, each turn them red.
- `main.rs` is now built as a **line vector** rather than a string, which is what makes the
  recorded line numbers exact rather than approximately right.
- ⚠️ `realization` is a **stub** of a §9 catalog entry, carrying four of the fourteen fields §9
  requires — and the artifact names what it is missing, in its own text. A record that looks like
  a catalog entry is one somebody will eventually cite as if it were one, and the citation will
  be made by someone who never read the task tree.
- The JSON writer is deliberately small — flat records, one escaping routine, unit tests for the
  control characters naive escapers miss. It is a writer, not a serialization library.
- Validation: `cargo test -p archogen-cli --test s0_provenance` → **4 passed, 0 failed**; the
  artifact parses under a real JSON parser (`python3 -c "import json; json.load(...)"` →
  `records = 4`); record `beat` resolves by hand to `src/main.rs:24` and to source bytes
  `1511..1639`. `cargo test --all` → **309** passed (301 before, `+8`); fmt and clippy clean;
  all doctrines green; `mdbook build` OK.

## archogen — F28 is green, and mutation testing found what it could not see

`ARCHOGEN-S0-0026` (leaf `S0.4`).

- **F28 runs as an automated test.** For each runnable case it removes the build directory,
  generates, compiles with `cargo`, runs the produced executable, and compares standard output to
  the bytes frozen in `S0.1`. Plus the unsupported case's refusal, plus a repeat pass proving that
  deleting the build directory changes nothing, plus a check that no generated file differed
  between generation and execution — so §12 S0's "without editing generated output" is a fact
  about the run, not an assurance about the author.
- ⛔ **And it was blind.** Replacing the hyperperiod with the longest period
  (`lcm` → `max`) left **all twelve** oracle tests green. Root cause, computed rather than
  guessed: both fixtures are **harmonic** — periods 10 and 30, then 10 and 20 — so one period
  divides the other and the two formulas agree on every input the corpus had. The fixtures were
  not wrong; they were chosen to be checkable by hand, which is the same property that collapses
  the formulas onto one answer.
- Closed at both levels: a unit test where `lcm(10, 15) = 30` against a longest period of `15`,
  and `examples/s0-heartbeat/system-non-harmonic.eadl` running the whole path on non-dividing
  periods. The same mutation now fails both.
- ⚠️ That fourth description has **no frozen expectation**, deliberately. The three F28 cases
  predate the emitter, which is the whole basis of the independence claim; one frozen afterwards
  would sit beside them with a weaker pedigree nobody could later tell apart — and the likeliest
  outcome is the weaker one being credited with the stronger claim. Its expectation is derived by
  the oracle instead, and labelled as such in the file, the README and the test.
- The generalisation is worth more than the bug, and is recorded for the director: a fixture set
  can be complete against its own specification and blind to a class of error, and a green gate
  says nothing about which. The acceptance matrix is thirty such fixtures.
- Validation: `cargo test -p archogen-cli --test s0_oracle` → **13 passed, 0 failed**; proven able
  to fail by reversing the release sort key in the runtime → `2 failed`, restored byte-identical
  and re-run green. `cargo test --all` → **301** passed (287 before, `+14`); fmt and clippy clean;
  all doctrines green; `mdbook build` OK.

## archogen — `archogen build` becomes real, and prints exactly what was frozen before it existed

`ARCHOGEN-S0-0025` (leaf `S0.3`).

- `crates/archogen-s0` interprets a checked description into a plan and emits a Rust crate;
  `archogen build` runs it. The generated crate compiles and runs, and its output is
  **byte-identical** to the observations leaf `S0.1` froze two commits earlier, before any
  emitter existed — compared with `diff`, not by eye, for both runnable fixtures.
- ⛔ The crate is deliberately **not** named `archogen-plan` or `archogen-emit`. Those are the
  §4.2 responsibility names `M4` builds under, and a prototype squatting on them is precisely how
  §12 S0's "no hidden special-case generator is grandfathered into the release" gets violated —
  it stops looking like a prototype. Retiring S0 is then one visible operation: delete the crate.
- ⭐ The engine-owned runtime (`rt.rs`, `service.rs`) is a **module of the engine**, read into the
  output with `include_str!`, not a string template — and a test asserts the emitted bytes equal
  the reviewed source. A template held as a string is Rust that nothing type-checks until a user
  compiles the output. Generation emits a **table**; behavior is copied from code that is
  compiled, linted and tested here.
- The unsupported fixture is refused by *generation*, after `check` accepted it: the diagnostic
  names `min-separation`, the missing capability (a modeled event source) and the leaf that
  supplies it (`M4.3`), per §5.4's "explain the missing engine capability; do not declare the
  requested function logically impossible". Nothing is written — a refusal that had already
  created a directory would leave a half-built artifact for someone to mistake for output.
- `--locked` is **refused**, not accepted and ignored: the S0 path emits no lock data, so a build
  that took the flag silently would be an unlocked build wearing a locked build's label.
- The command surface gained a third maturity state. `build` runs, but over a narrower path than
  its §10.2 contract, and both other states would have been false: *built* promises a complete
  system and its simulator, *unimplemented* denies a command that works. `archogen --help`,
  `archogen help build`, the command's own output and every generated file all say
  **experimental**, which §12 S0 requires.
- Validation: `cargo test -p archogen-cli --test s0_build` → **7 passed, 0 failed**, including
  the generated crate actually compiling; proven able to fail by removing one semicolon from the
  emitted `main` → `error: expected \`;\`, found \`rt\``. ⛔ The *first* attempt at that control
  was a false green — the replacement searched for a spelling the escaped source does not contain
  — which is now its own lesson. `cargo test --all` → **287** passed (272 before, `+15`); fmt and
  clippy clean; all doctrines green; `mdbook build` OK.

## archogen — the profile's admitted task model stops being prose

`ARCHOGEN-M1-0024` (leaf `M1.9`).

- ⛔ **`archogen check` accepted two tasks at the same priority**, against a profile whose own
  table says "static **unique** task priorities". Found from outside M1, by `S0.3` needing to
  order two coincident releases by priority rank — which is a total order only if that rule
  holds.
- Root cause: `Profile` carries two lists and only one was ever consulted.
  `git grep -n '.decisions' -- crates/` returned a single hit — the drift **test** that compares
  the table to the published documentation page. So all thirteen decision rows were kept
  scrupulously in sync with the docs while meaning nothing to the checker. A guard that makes a
  rule *look* guarded is why nobody noticed.
- `crates/eadl-model/src/workload.rs` enforces the §3.1 **Workload** row as four rules, each with
  the §5.5 verdict its kind of failure deserves: an undeclared release model is a `missing-fact`,
  two release models is `invalid-description`, and duplicate priorities and `D > T` are
  `unsupported-profile` — refused with the cost of admitting them named, never silently reduced.
- One mistake produces one message: a task with no release model is not also reported for a
  deadline it has nothing to compare against, and a missing `priority` is left to the schema.
  The deadline comparison crosses units, so `1 s` against a `100 ms` period is caught.
- The remaining twelve decision rows are **counted** by a census test that fails if the number
  moves, and classifying them by the stage that can enforce each is tracked as leaf `M1.10`.
  Twelve unenforced rows is not twelve defects — "Rust `no_std` core" is a property of the
  engine — but twelve rows nobody had counted was the state worth leaving behind.
- Validation: `0 → 12` on the identical duplicate-priority description; four new semantic corpus
  cases each produce the verdict they declare (corpus now **29**); every existing example keeps
  the exact verdict `examples/README.md` publishes (`0`, `0`, `13`, `12`); `cargo test --all` →
  **272** passed (259 before, `+13`); fmt and clippy clean; all doctrines green; `mdbook build`
  OK.

## archogen — a leaf closed by its acceptance, not by its implementation

`ARCHOGEN-S0-0023` (leaf `S0.2`).

- `S0.2` asked for "a minimal S-expression reader with source spans, sufficient for the S0
  fixture only". `M1` had already built the whole frontend, and §12 S0 anticipates exactly that:
  "the prototype implementation can be discarded or replaced as semantics settle. Keep its
  functional fixtures."
- So the leaf closed **without writing a reader** — and not by deleting it or ticking it either.
  What a leaf carries is its acceptance, so both clauses were re-verified against `eadl-front`.
  The first ("the three fixtures parse") was already implied by other tests. The second
  ("a malformed fixture reports a **span-localized** error") was asserted **nowhere** for this
  corpus, so closing the leaf still produced a test file.
- ⭐ That clause needs a negative case to mean anything: a reader that reported every syntax
  error at end-of-input would satisfy "an error was produced" and be useless on a long
  description. `crates/archogen-cli/tests/s0_reader.rs` therefore injects a stray `)` near the
  **top** of the fixture and requires the caret on that line — with the expected line computed
  from the fixture text, so rearranging the declarations moves it automatically. Every
  corruption is built in memory; no broken description enters the corpus.
- Validation: `cargo test -p archogen-cli --test s0_reader` → **4 passed, 0 failed**, and proven
  able to fail by mutating the **reader**, not the test: pointing `read-unexpected-close` at
  offset zero → `left: 1 / right: 25`, one test red; moving the `read-unclosed-list` secondary
  label to end-of-input → `left: 40 / right: 32`, one test red. Restored and re-run green both
  times. `cargo test --all` → **259** passed (255 before, `+4`); fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — the S0 corpus and its oracle, written before the emitter

`ARCHOGEN-S0-0022` (leaf `S0.1`).

- `examples/s0-heartbeat/` holds the three descriptions of fixture **F28** — a base system, the
  same system with one period changed, and the same system with one task made sporadic — and the
  observations each must produce, frozen in `expected/`.
- The point of the leaf is the **order**: this lands before any emitter exists (`S0.3`), so
  §12 S0's "write an independent expected-output assertion *before* generating" is a fact of the
  commit history rather than a claim. The oracle,
  `crates/archogen-cli/tests/s0_oracle.rs`, additionally lives where Rust cannot link it into a
  library, so no future emitter can call the derivation that judges it.
- The unsupported case is a **realization** gap, not a profile violation: §3.1 admits sporadic
  releases, so `archogen check` accepts the description (exit `0`) and generation must refuse it
  (exit `12`), naming the missing engine capability and the leaf that supplies it. That is F10's
  shape — "missing engine support, not universal impossibility" — reached early, and it exercises
  S0's own path instead of re-testing M1's.
- The observation contract is published in `examples/s0-heartbeat/README.md` and the new book
  chapter: the horizon is the system's hyperperiod, a task of period `T` is released at every
  `t = k·T` inside it, coincident releases are ordered by priority rank. No run length and no
  message text appear in any description — both would be the engine's decisions smuggled into
  eADL, which §12 S0 says does not satisfy the gate.
- ⭐ Found on the way through: **the `priority` clause's comparison direction was nowhere
  recorded.** Five tracked lines mention higher/lower priority and every one of them orders a
  prose table. Recorded as a decision (lower number = higher priority) and cited from the
  language module, because §15 puts a parameter's comparison direction under migration
  discipline — the first consumer that would have depended on it unstated is `M2.3`'s
  response-time recurrence.
- Validation: `cargo test -p archogen-cli --test s0_oracle` → **9 passed, 0 failed**, and measured
  *sensitive* in both directions — mutating the frozen expectation fails 2 tests, mutating the
  description fails 3. `cargo test --all` → **255** passed (246 before, `+9`); fmt and clippy
  clean; all doctrines green; `mdbook build` OK. What F28 still owes — generate, compile, run —
  is held by a tripwire that fails the moment `archogen build` becomes real.

## archogen — the command is `archogen`, not `osgen`

`ARCHOGEN-PROGRAM-0021` (leaf `PROGRAM.2.1`).

- The project is `archogen` and its command announced itself as `osgen`, so the toolchain carried
  two names. Measured before the change: **169 occurrences** across 33 tracked files, in six
  spellings. After: none, outside the two places that record the rename.
- `crates/osgen-cli` → `crates/archogen-cli` (binary `archogen`), `crates/osgen-evidence` →
  `crates/archogen-evidence`, both via `git mv` so history follows the files. `Cargo.lock` was
  regenerated rather than edited.
- The `ROADMAP.md` §4.2 component names (`archogen-plan`, `archogen-emit`, `archogen-check`) and
  every §10.2 command went with it. **Scope call, recorded in the leaf:** the narrow reading —
  the binary alone — would have preserved exactly the inconsistency that prompted the request,
  leaving a project called `archogen` whose evidence crate was `osgen-evidence`. Narrowing it
  back is a small mechanical revert if that is what was meant.
- `ROADMAP.md` carries a dated **migration note**, which §15 requires of a rename, stating
  plainly that nothing but the name changed.
- ⛔ The first draft of this leaf's acceptance criterion was self-defeating — "`git grep -ci
  osgen` returns nothing" — because the record of a rename necessarily names the old thing.
  Rescoped to exclude the two records, and the exclusion is verified rather than assumed.
- Earlier entries below were rewritten to the new name so they point at artifacts that exist;
  the commit messages in `git log` still carry the old one, which is where the history lives.
- Validation: behaviour byte-identical — `archogen check` returns the same verdicts and the same
  exit codes (0, 12, 13) on the same examples. **246** tests passing, the same count as before,
  so no test was lost to a renamed path; fmt and clippy clean; all doctrines green;
  `mdbook build` OK.

## archogen — the frontend pipeline, and `archogen check` becomes real — **M1 complete**

`ARCHOGEN-M1-0019` (leaf `M1.8`).

- `crates/eadl-model/src/check.rs` composes the six passes built one at a time by `M1.1`–`M1.7`
  and `M0.3`, and returns a `ROADMAP.md` §5.5 verdict. `archogen check` runs it — the first command
  of the §10.2 surface to stop being a signpost.
- ⭐ **The verdict is what to fix first**, not what was found first and not severity of
  consequence. A malformed description makes every later answer meaningless, so it outranks
  everything; an unsupported request outranks a missing fact, because describing that fact would
  be wasted work on a system the profile refuses anyway. Precedence chooses the headline, never
  what the author gets to see — every diagnostic is still printed.
- An **accepted description is silent**. "Accepted with three warnings" is a shape this pipeline
  does not have, so an author never has to judge which messages mattered. And acceptance says so
  plainly: *"this checks the description, not a system: no resolution, generation or analysis has
  run"*.
- `docs/semantics/cases/` holds **25** worked cases against §12 M1's minimum of twenty — 5 `ok`,
  10 `invalid-description`, 5 `unsupported-profile`, 4 `infeasible-configuration`, 1
  `missing-fact` — each declaring its expected verdict **in its own header**. A driver that
  computed the expectation would agree with itself forever.
- The §5.5 contract is enforced over **every diagnostic the corpus produces** — a located span
  and a concrete repair direction — rather than over the handful a unit test happens to build.
- `Verdict` is the single §5.5 vocabulary, and its mapping to exit codes is asserted **total**.
  A second enum spelling the same seven words is the drift the test exists to prevent.
- `examples/bounded-queue/` (uc4) joins the examples and is refused by name with the obligation
  admitting it would cost. uc3 and uc4 are both refused for reasons that must not be confused:
  uc3's refusal is temporary and must become a build when `M3.2` lands, without the description
  changing; uc4's is permanent for this profile.
- The kind modules are embedded in the binary, so `archogen` cannot be shadowed by a file in the
  working directory. If they ever fail to load the result is `tool-failure`, never a verdict about
  the user's description.
- ⭐ **Five defects were found by running the corpus.** `needs`/`uses` were readable by the
  checker and unwritable in the language; a refinement pair was reported as a contradiction
  before the refinement checker saw it; uc3 passed silently because its dependency was a
  constraint rather than a `needs` edge; a case used `dma`, itself a profile exclusion, and would
  have stopped testing refinement; and `check_cmd` read the file before resolving the profile,
  contradicting its own comment — the code was changed to match the comment, not the reverse.
- New book chapter `docs/book/src/checking.md`.
- Validation: 246 tests across the workspace, 0 failed; fmt and clippy clean; all doctrines
  green; `mdbook build` OK.

## archogen — the `os/rt` workload module, and a measured gap closed

`ARCHOGEN-M1-0018` (leaf `M1.7`).

- `docs/semantics/kinds/os-rt.eadl` declares the `task` kind — release model, deadline with its
  reference event, bounded jitter, static unique priority, functional needs, overrun response —
  using the same `defkind` primitive as everything else.
- ⭐ **The 10-of-11 schema gap `M1.2` measured is now 13 of 13.** `Holds::Kind` makes a clause
  validate recursively, so `(clause task … (holds kind task))` lets the schema see inside a task.
  `execution-bound` is now caught by the schema *and* the boundary classifier, and the refusal
  carries the boundary's wording — `wcet` is not a typo, it is content in the wrong layer. The
  pinned assertion was replaced, not loosened: it now asserts the out-of-reach set is **empty**.
- A registry that has not loaded `os-rt.eadl` **says so**. Silently accepting whatever is inside
  an unvalidatable clause is the failure mode that let the gap exist in the first place.
- `entry-point` and `stack-allocation` join the forbidden registry, each with a worked
  ambiguous case — both *feel* like task properties and both belong elsewhere. The corpus grew
  from 21 to 23 cases and every census assertion with it.
- `examples/` holds the three M0 use cases as real descriptions, checked by a 7-arm suite: they
  validate, every task has an arrival model and a deadline with its reference event, priorities
  are unique, and **none carries an execution bound, a code reference, or an allocation** —
  asserted by the classifier rather than by anyone remembering. `uc3` is additionally checked for
  still containing its own contradiction, because the tempting "fix" would make the case go quiet.
- ⭐ **Three defects were found by running the new tests.** (a) `defblock` had no `absent`
  clause: §5.3's three presence states were implemented in the checker and missing from the
  language. (b) A composition reference to a declared block was reported as `missing-fact`;
  a `uses` target that is declared is satisfied by its declaration, and the absence check was
  reordered so that being declared does not repeal an explicit absence. (c) `diagnose | head -3`
  panicked with `Broken pipe` — a closed pipe is not a failure.
- New book chapter `docs/book/src/workload.md`.
- Validation: 234 tests across 17 suites, 0 failed; fmt and clippy clean; all doctrines green;
  `mdbook build` OK.

## archogen — refinement as an obligation to check: F07

`ARCHOGEN-M1-0017` (leaf `M1.6`).

- `crates/eadl-model/src/refinement.rs`: three obligations — **guarantee**, **constraint**,
  **exclusion** — and a violation names which one broke. §5.1.1: a refinement declaration is "an
  obligation to check, not permission to trust a claim blindly".
- ⭐ **Two roadmap sentences pull against each other here, and both directions are tested.**
  §5.1.1 demands the obligations be checked; §5.3 says "adding an unused device is not
  automatically an invalid refinement". A checker that demanded equality passes the exclusion arm
  and rejects every real refinement; one that allowed any addition passes the addition arm and
  silently drops the exclusion obligation — which is the one that matters most, because an
  abstract description declares a fact absent *because* something depends on its absence.
- Additions are reported even though they are allowed: allowed is not the same as invisible, and
  the author should be able to see what grew.
- An abstract description states **bounds with a direction**, not bare values. The same numeric
  relationship satisfies `(at-least 32 bit)` and violates `(at-most 50 us)`, so a bare value
  would leave the checker guessing. `exactly` refuses a merely "better" value but accepts the
  same amount written differently, because the comparison is on the amount, not the spelling.
- A bound checked against the wrong dimension is a type error, not a `false` — reusing F03's
  refusal.
- New book chapter `docs/book/src/refinement.md`.
- Validation: 15 new arms, 223 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — presence, relevance, and the closure: F04, F05, F06

`ARCHOGEN-M1-0016` (leaf `M1.5`).

- `crates/eadl-model/src/presence.rs`: offered / explicitly absent / undescribed, and the
  transitive dependency closure that decides which unknowns matter.
- ⭐ **F04 and F05 are run against the same description**, deliberately. They are two halves of
  one decision: §2's correction table revised "unknown capability anywhere blocks generation"
  into "required facts in the selected dependency closure must be known", so that "irrelevant
  unknown facts do not invalidate unrelated systems". A checker that always blocks passes F04
  and fails F05; one that never blocks passes F05 and fails F04. Two separate descriptions could
  have been passed by two different bugs.
- **Absent is not undescribed.** A required absent fact is `infeasible-configuration` — "a
  definite answer, not a gap to be filled in" — while a required undescribed one is
  `missing-fact`. Collapsing them would send an author to describe something the platform has
  already said it does not have.
- A contradiction is `invalid-description` whether or not it is reachable, and names **both**
  sites: §5.3 qualifies the unknown-fact rule by relevance and states the contradiction rule
  without a qualifier. It is never resolved by preferring one side, because only the author knows
  which half was meant.
- A `needs` edge nested inside a `requires` clause is still followed. A closure that only read
  top-level clauses would miss most real descriptions and would fail **open**.
- `offered` remains a claim, not evidence: nothing in this analysis says a platform really has a
  capability.
- New book chapter `docs/book/src/presence.md`.
- Validation: 14 new arms, 208 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — modules and instances, F01 and F02

`ARCHOGEN-M1-0015` (leaf `M1.4`).

- `crates/eadl-front/src/module.rs`: namespaced imports, explicit exports, typed parameters with
  defaults, version constraints, and elaboration into a program.
- ⭐ **Elaboration produces instances, not modules.** §5.1.1 requires instantiating a module more
  than once "without sharing mutable elaboration state", so importing `hw.timer` twice yields two
  instances with independent bindings and independent qualified names. A cache keyed on module
  name would have been smaller and would have silently made the second import a no-op — a system
  with two timers would have had one, and nothing would have said so.
- Names carry their whole alias path (`platform.timer.timer.counter`), and instances come out in
  dependency order, children before parents.
- **F01 and F02 green, 18 arms.** A cycle reports its whole chain — `circular import: a → b → c →
  a` — because "there is a cycle" is a puzzle and the chain is a diagnostic. A duplicated export
  and a duplicated alias each name **both** sites, since the author looking at one cannot see the
  other. Also refused: a dangling export, a name mismatch, a missing required parameter, an
  unknown parameter (listing the real ones), and an unversioned module.
- A **major** version difference is never satisfied, however much newer the module is. "Newer" is
  not "compatible", and §15 keeps a locked description's meaning, which a major bump is defined
  not to preserve.
- A cycle is the only failure that stops elaboration; everything else is collected, so three
  problems cost one edit cycle.
- ⭐ A parser defect was caught by the fixture, not by inspection: the import version clause was
  read flat while the syntax is nested, so every versioned import in the valid composition was
  refused — five of eighteen arms red, including the F01 happy path. The nested shape was kept
  rather than flattened to match the bug, because the relation being its own form leaves room for
  `at-most` and `exactly`.
- New book chapter `docs/book/src/modules.md`.
- Validation: 18 new arms, 194 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — exact quantities, and F03

`ARCHOGEN-M1-0014` (leaf `M1.3`).

- `src/rational.rs`: exact `i128/i128` rationals, normalized on construction so equality and
  ordering are structural, with **every** operation checked. §7.4 requires overflow detection,
  and a wrapped numerator turns an unschedulable system into a schedulable-looking one with
  nothing in the output to say so. `ceil` rounds up, because that is the direction response-time
  analysis needs — rounding the other way understates interference and turns a missed deadline
  into a reported pass.
- No floating point anywhere. `1/3 + 1/3 + 1/3` is exactly `1`; `1 ns` in milliseconds is
  exactly `1/1000000`, not zero; values with no short decimal form print as fractions rather
  than being rounded into something that reads like a measurement.
- `src/quantity.rs`: 13 units over four dimensions, exact conversion, and declared comparison
  directions with **no default** — §5.2 warns that "more bits or a faster clock is not
  universally better", and an undeclared direction is a bug waiting for a substitution to expose
  it. The unit table is small on purpose: one accepting arbitrary SI prefixes accepts `Ps` too,
  and a typo that parses is worse than one that does not.
- **F03 green, 15 arms.** ⭐ The "before arithmetic" property is *demonstrated*, not asserted:
  the fixture compares two quantities whose magnitudes would overflow if touched — asserting
  first that they would — and the comparison still returns the dimension error, which is only
  possible if the dimension check precedes any read. A zero clock frequency is refused at
  construction, so no such quantity exists for anything to divide by later.
- New book chapter `docs/book/src/quantities.md`.
- Validation: 15 new arms, 176 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — kinds and schemas, with exactly one trusted primitive

`ARCHOGEN-M1-0013` (leaf `M1.2`).

- `crates/eadl-model/src/kind.rs` implements the kind registry, the `defkind` facility, and
  schema validation of the declaration frame: known kind, name when required, known clauses,
  cardinality, value shapes.
- ⭐ **Exactly one declaration is a trusted primitive.** §2 requires that "a small trusted
  semantic foundation remains explicit" and that "the registry cannot silently introduce new
  trusted axioms". So `defkind` is Rust, and all five surface kinds are declared *in eADL* in
  `docs/semantics/kinds/core.eadl` — no more privileged than a kind added tomorrow. A test
  asserts both halves, including that `defkind` is **not** a registry entry.
- §5.6's prohibition has no back door: the boundary classifier runs over kind definitions too,
  so `(defkind deftimer … (implementation …))` is refused by the same machine that refuses it in
  an ordinary declaration. A kind must also carry a `doc` — a kind nobody can explain is a kind
  nobody should be adding.
- A forbidden construct gets the **boundary's** wording, not "unknown clause", which is true and
  useless. A typo gets an edit-distance-bounded suggestion; the bound exists so that
  `implementation` is never "corrected" to `defsystem`, which would send an author to rename
  rather than to reconsider.
- ⭐ **A claimed equivalence was refused by its own test and replaced with a measurement.** The
  first version asserted that the schema refuses every rejected boundary case too;
  `execution-bound` broke it, because `wcet` hides inside `(task …)`, which is opaque at this
  layer. The honest result — schema 10 of 11, classifier 11 of 11 — is now pinned in both
  directions, with `M1.7` named as the leaf that closes the gap.
- New book chapter `docs/book/src/kinds.md`.
- Validation: 13 new arms, 150 tests across the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — F27: the boundary gets a machine

`ARCHOGEN-M0-0011` (leaf `M0.3`). **M0 is complete.**

- `crates/eadl-model/src/boundary.rs` registers nine constructs that are implementation by
  definition — `implementation`, `model`, `provider`, `emit`, `permits`, `wcet`, `init-order`,
  `save-order`, `read-sequence` — each with the boundary test it fails, what it actually is, and
  where it belongs instead. A refusal that does not say where the content belongs is an obstacle
  rather than guidance.
- **A construct registry, not a keyword scan**, and the corpus is why: the *accepted*
  `required-ordering-guarantee` declaration contains `write` twice, naming observable effects an
  ordering requirement is stated over. A word-based classifier refuses a contract the roadmap
  explicitly permits. What separates a step from a reference is the construct it sits inside.
- F27 runs six arms over all 21 cases, `6 passed 0 failed`. The load-bearing one is **mutation**:
  seeding each of the nine constructs into each of the ten accepted cases — 90 mutations — must
  flip every one to rejected with the matching test, and stripping the offending construct from
  each rejected case must flip it back. Agreement alone is passed by a classifier that accepts
  everything.
- ⭐ **The coverage arm found a real gap in the corpus, and it was fixed at the corpus**:
  `save-order` and `read-sequence` were registered but never reached, because both cases wrapped
  their procedure in an `implementation` block that was blamed first. Both now carry the
  procedure as an ordinary clause — the realistic shape, since an author who believes a retry
  loop belongs in the description does not label it "implementation" beforehand.
- The honest limit is in the module, the book and the corpus README: acceptance means "contains
  no construct the registry knows to be implementation", not "is sound". §4.3 requires human
  review for intent, because a field can hide an algorithm behind an innocent name.
- Book: `docs/book/src/boundary.md` gains "How much of this a machine can check".
- Validation: 137 tests across 11 suites, 0 failed; fmt and clippy clean; all doctrines green.

## archogen — the eADL reader

`ARCHOGEN-M1-0010` (leaf `M1.1`).

- New crate `crates/eadl-front`: S-expressions, byte spans with **character** columns, and
  caret diagnostics that carry a repair direction, as §5.5 requires of every diagnostic.
- Measured on the real corpus rather than on toys: all 21 boundary-corpus files read with zero
  diagnostics, round-trip semantically through canonical form, and yield their exact metadata
  key set. Those files were written for a different purpose before the reader existed, which is
  what makes the suite evidence rather than confirmation.
- **No float anywhere.** §7.4 requires exact integer or checked rational arithmetic, so a
  decimal literal is kept as an integer and a scale — `0.1` survives a round trip exactly — and
  an out-of-range literal is refused rather than wrapped.
- `3ms` is refused, not read as a symbol: it is a typo for `3 ms`, and accepting it would lose
  the magnitude and surface much later as a mysteriously missing field. Reading does not stop at
  the first error, so three malformed numbers cost one edit cycle.
- New diagnostic tool `cargo run -q -p eadl-front --example diagnose -- <file>`, registered in
  `TOOLBOX.md`.
- ⭐ **That tool immediately found a real defect.** The header parser silently truncated a
  corpus rationale that wrapped onto a line beginning `implementation-independence:` — exactly a
  bare key plus a colon. Two green suites were blind to it, because both asserted only that the
  keys they *wanted* were present, and a presence check cannot see an extra key. The rule now
  requires two independent discriminators, each of which had already been tried alone and failed
  on a real file; the corpus test asserts the exact key set for every case. Promoted to
  `docs/knowledge/presence-checks-cannot-see-an-extra-key.md`.
- New book chapter `docs/book/src/reading.md`.
- Validation: 52 tests in the new crate, 119 across the workspace, 0 failed; fmt and clippy
  clean; all doctrines green; `mdbook build` OK.

## archogen — the emulator pinned, and the board recorded as absent

`ARCHOGEN-M0-0009` (leaf `M0.5`).

- `targets/riscv-virt-up.env` pins the emulator configuration as data, and
  `scripts/target_emulator.sh` is the only thing that renders it — so no two callers can type
  it slightly differently. §3.2: "Pin the emulator configuration … do not rely on changing
  defaults." `--dump-dtb` writes QEMU's generated device tree for the agreement check against
  the eADL platform fixture, which is the difference between describing a platform and
  describing *this* platform.
- Absence is reported, never skipped: with QEMU not installed, `--check` exits `20` saying
  `do NOT record an emulator result without it`. §14.3 requires exactly that. The configuration
  also carries `TARGET_VERIFIED=no` until an installed QEMU confirms it.
- 🔎 **No physical board has been selected or procured.** All seven facts §3.2 requires are
  recorded as `unrecorded`, with the selection criteria written out. Naming a plausible board
  from memory would be worse than naming none — each row is a fact a later timing claim would
  rest on. Tree `M5` is blocked at the root; S0–M4 are not. This is a director decision.
- New book chapter `docs/book/src/targets.md`; `TOOLBOX.md` gains the emulator tool row.
- Validation: `bash -n` clean; docpath clean; all doctrines green; 67 tests pass;
  `mdbook build` OK.

## archogen — a report that cannot say "verified"

`ARCHOGEN-M0-0008` (leaf `M0.7`).

- New crate `crates/archogen-evidence` — the evidence, claim and trust vocabulary of §7.1, §7.3
  and §4.4. A separate crate on purpose: it is shared between the generator and the independent
  checker, and §4.4 requires such sharing to be visible rather than buried.
- §7.1's "one global verified flag is prohibited" is encoded **three ways**, not documented
  once: there is no aggregate verdict type; a report refuses to render while any property is
  unanswered (`a property with no claim is not a pass`); and every positive conclusion carries
  its qualifier by construction — a conditional analysis with an empty assumption list is
  refused as `an unconditional claim`.
- Bounds remember where they came from. An observed maximum stays an observation whatever the
  safety factor — tested at 1/1, 3/2, 10/1 and 1000/1 — because §7.3 says so and because
  multiplying a measurement by 1.5 and calling it a bound is the most common way a timing claim
  becomes untrue while looking like diligence. A bound with no binary identity is refused.
- ⭐ The compiler caught a design defect: `f64` is not `Eq`, so a float safety factor would
  have cost the whole vocabulary comparability — and a bound that cannot be compared to its
  baseline cannot be checked for drift. Replaced with an exact rational, which §7.4's "exact
  integer or checked rational arithmetic" wanted anyway. The type error was a semantic error.
- §4.4 trust vocabulary: roots, roles, and drift. Shared infrastructure is recorded but costs
  no independence; a shared semantic helper does. An unrelated change produces no warning
  (§14.4 case 5) — a gate that cries wolf is a gate that gets disabled. The honest limit is
  carried in the module: this enforces disclosure, not semantic independence.
- New book chapter `docs/book/src/evidence.md`.
- Validation: 33 new contract tests, 67 in the workspace, 0 failed; fmt and clippy clean; all
  doctrines green; `mdbook build` OK.

## archogen — four use cases, and a seal that is a check rather than a promise

`ARCHOGEN-M0-0007` (leaf `M0.6`).

- `docs/usecases/` — the three cases §12 M0 requires plus a fourth: `uc1-periodic-three`
  (succeeds by building), `uc2-high-interference` (may succeed by refusing to conclude),
  `uc3-alternative-timer` (refused before M3, supported after), `uc4-bounded-queue` (succeeds
  by refusing). A profile that has never refused anything has not been tested as a profile.
- `uc3` is the only case whose expected answer *changes* when a capability lands. That is how
  the project distinguishes adding an engine capability from weakening a requirement until it
  passes — both turn a red fixture green, only one is progress.
- Five evaluation cases sealed under `docs/evaluation/frozen/` with SHA-256 digests, and a new
  project doctrine `FROZEN-EVALUATION` that enforces three legs: **integrity** (no post-seal
  edit), **completeness** (nothing added or removed unlisted), and **non-contamination** (no
  tracked file outside the sealed directory names a sealed case). All three proven in both
  directions. §16's reuse claim is about *unseen* systems; measured on cases that were in view
  while the catalog was designed, it measures how well the catalog was fitted to them.
- ⭐ A latent defect in the first cut of the checker was caught before it landed: the
  contamination scan word-split tracked paths containing spaces and put the whole file list on
  one command line. Replaced with `git grep --fixed-strings` and re-proven against a path with
  a space in it.
- `scripts/check_doctrines.project.sh` turned from a shipped no-op into a real registry;
  `DOCTRINE_ENFORCEMENT.md` gains the project-doctrine mirror; `TOOLBOX.md`'s placeholder
  table replaced with seven real instruments and the rule that a new tool ships a RED arm.
- New book chapter `docs/book/src/usecases.md`.
- Validation: `bash -n` clean on both scripts (shellcheck not installed on this machine);
  `cargo test --all` → 34 passed, 0 failed; all doctrines green; `mdbook build` OK.

## archogen — the 21-case boundary corpus

`ARCHOGEN-M0-0006` (leaf `M0.2`).

- `docs/semantics/boundary/` — 21 worked cases against minimums of 12 and 3: 10 accepted, 11
  rejected, 5 of them ambiguous and argued rather than asserted. Every rejected case names the
  test that failed.
- Cases are written in **pairs** wherever possible — the accepted contract and the rejected
  procedure that satisfies it — so the corpus shows where the boundary runs rather than only
  recording verdicts: `atomic-observation` vs `atomic-read-retry-loop`, `addressable-region`
  vs `register-programming-sequence`, `required-ordering-guarantee` vs `initialization-order`.
- The ambiguous five are the ones a reasonable author gets wrong in both directions:
  `counter-width-and-rate` and `addressable-region` look like implementation and are accepted;
  `execution-bound`, `initialization-order` and `retry-permitted` look like requirements and
  are rejected. A WCET is evidence about a binary (§7.3), not a description field.
- `docs/semantics/boundary/README.md` fixes the case format F27 will consume, and states the
  honest limit: the mechanical check is a floor, because a field can hide an algorithm behind
  an innocent name.
- Sequencing decision recorded: F27 (`M0.3`) lands after the reader (`M1.1`) rather than
  before it — mechanizing the corpus needs real parsing, and two throwaway tokenizers would
  buy no earlier signal.

## archogen — `rt-static-up-v1` as checked data, not prose

`ARCHOGEN-M0-0005` (leaf `M0.4`).

- New crate `crates/eadl-model` — the §4.2 home for typed declarations, units, contract IDs
  and profile definitions. It holds `rt-static-up-v1`: 13 concern decisions and 18 named
  exclusions, each carrying the obligation admitting it would add.
- §3.1 requires an out-of-profile request to be refused *by name* rather than silently
  weakened. That is only enforceable if the engine holds the list; before this it was prose in
  the roadmap (`git grep -ln 'rt-static-up-v1' -- crates/` → no match).
- `docs/profiles/rt-static-up-v1.md` is the published form, and a test fails if it drifts.
  The gate was proven with a RED arm: misspelling `posix` as `posiks` on the page →
  `docs/profiles/rt-static-up-v1.md has drifted from the profile data`, 1 failed.
- An unknown capability is not admitted by silence — a known exclusion is refused by name,
  an unknown one is a `missing-fact`.
- New book chapter `docs/book/src/profile.md`.
- Validation: fmt clean; clippy -D warnings clean; `cargo test --all` → 34 passed, 0 failed;
  all doctrines green; `mdbook build` OK.

## archogen — the controlling eADL/engine boundary

`ARCHOGEN-M0-0004` (leaf `M0.1`).

- Recorded `docs/decisions/decision_eadl-engine-boundary.md`: eADL describes functionality and
  contains no implementation. This is the precedence rule — it overrides ambiguous wording in
  any source draft and any later convenience argument.
- The record fixes the three classification tests (externality, implementation-independence,
  non-prescription), all eight worked cases of `ROADMAP.md` §4.3 with both sides named, the
  direction of every obligation (a declared capability is not evidence; a refinement is an
  obligation to check), and what the boundary rules out — including `defkind` becoming a
  template language, and implementation syntax as an escape hatch for missing engine support.
- New book chapter `docs/book/src/boundary.md`, because this is the concept a reader must
  have before any other chapter makes sense.
- Validation: 8/8 §4.3 rows present; layer-C index in sync
  (`scripts/check_memory_architecture.sh`, `rc=0`); `mdbook build` OK; all doctrines green.

## archogen — the `archogen` command-line shell

`ARCHOGEN-PROGRAM-0003` (leaf `PROGRAM.2`).

- Replaced the bedrock starter crate with `crates/archogen-cli` — library `archogen_cli` plus the
  `archogen` binary. The workspace now has a real entry point.
- The `ROADMAP.md` §10.2 command surface (`check`, `resolve`, `build`, `analyze`, `verify`,
  `explain`, `replay`) is declared **once, as data**; `--help` renders from that table and the
  parser validates against it, so a documented option is always an accepted option.
- The §5.5 outcome vocabulary is implemented as a stable exit-code contract, with diagnostic
  results (`invalid-description` … `tool-failure`) kept separate from process-level statuses
  (`ok`, `usage`, `unimplemented`). Every refusal carries a concrete repair direction, as §5.5
  requires.
- Every command is unimplemented and says so precisely, naming the task-tree leaf that owns
  building it and exiting 20 — a gap in this toolchain is tracked work, not an unknown.
- Zero external dependencies, recorded as a decision
  (`docs/decisions/decision_zero-dependency-engine-core.md`): §4.4 trust inventories, §10.3
  locked offline builds, §5.5 diagnostic wording.
- New book chapter: `docs/book/src/cli.md`.
- Validation: `cargo fmt --check` clean; `clippy -D warnings` clean; `cargo test --all` →
  28 passed, 0 failed; `scripts/check_doctrines.sh` → all doctrines green; `mdbook build` OK.

## archogen — roadmap seeded into task-trees

`ARCHOGEN-PROGRAM-0002` (leaf `PROGRAM.1`).

- `ROADMAP.md` revision 2.0 (eADL and OS Generation — Consolidated Roadmap) adopted as the
  project's direction and converted, in full, into ten task-trees: `PROGRAM` for the
  cross-cutting engineering substrate and one tree per roadmap milestone (`M0`, `S0`,
  `M1`–`M7`).
- Every roadmap unit (§11–§20) and every mandatory fixture F01–F30 now names an owning tree
  and leaf; the two coverage maps live in `docs/tasks/PROGRAM.md` so "where does roadmap
  item X live?" has a single mechanical answer.
- `README.md` rewritten from the template landing page to archogen's, within the
  `README-STABILITY` caps (77/300 lines, 3 637/16 384 bytes).
- Live docs brought into lockstep: `MEMORY.md` resume pointer, `LIVE_STATUS.md` (twelve
  rows, one per tree plus the spine), `docs/TASK_TREE.md` index, the derived Knowledge Map,
  and the book introduction.
- Validation: `scripts/check_doctrines.sh` → `=== all doctrines green ===`, 13/13.

---

## Provenance — the `bedrock` discipline spine


## bedrock-scaffold 0.6.1 — creating a project is foolproof through its first commit

`BEDROCK-MAINTENANCE.2.7`.

- ⛔ **Measured on a fresh clone of 0.6.0:** `bootstrap.sh` left the crate rename — a CODE change — with no owning
  leaf, so the new project's FIRST commit was refused by `TASK-TREE-OWNERSHIP` and `TASK-ACCEPTANCE`. A new user's
  first contact with the discipline was a refusal about a rename the tool made.
- **`bootstrap.sh` now seeds `docs/tasks/BOOTSTRAP.md`** on a fresh de-template: a done leaf that owns the bootstrap,
  its ticked checklist carrying the evidence of that very run (crate-name count before/after, hooks path, the
  enforcer's summary and verdict with `rc=0`), registered in `docs/TASK_TREE.md`, pointed to by `MEMORY.md`; and it
  prints the exact first-commit command as step 0. Idempotent.
- Proven: clone → `bootstrap.sh <name>` → the printed commit → hooks green → `make gate` green → `make check` green,
  with no hand edits. Two defects in the fix were caught by the trial itself (an enforcer run before the map
  existed; a `grep -c` fallback that split a checklist bullet).

## bedrock-scaffold 0.6.0 — four evidence and ratchet doctrines: lessons reach the retrievable layer, routings carry evidence, gap claims carry their census, tables keep their columns

`BEDROCK-MAINTENANCE.2.6`.

- **Added `LESSON-PROMOTION`**: a new dated lesson heading staged in `DEV_NOTES.md` must be promoted (a
  `docs/knowledge/` change or a `docs/decisions/` record gaining `answers:`) or explicitly declined
  (`promotion: declined (<reason>)` in the owning leaf). Pure verdict with 9 controls at import.
- **Added `ROUTING-EVIDENCE`**: a leaf that routes a finding out to another tree carries a `ROUTING EVIDENCE`
  section. Keyed on the semantics of leaving the tree; 5-arm `--self-test`.
- **Added `GAP-CLAIM-CENSUS`**: a leaf that ADDS a "nothing checks X" claim records the census it rests on in
  the same section (or `census: not run (<why>)`). Staged-diff-scoped; `--all` reports the backlog; 10-arm
  `--self-test` pinning the founding active and passive sentences.
- **Added `TABLE-ARITY-RATCHET`** (a fresh minimal implementation): a staged `.md` may not raise the number of
  table rows whose cell count disagrees with their header; code spans and escaped pipes respected; 8-arm
  `--self-test`.
- ⛔ Two defects in the ports were caught by their own RED arms before the gate ran: a heredoc that consumed
  the table detector's stdin (every arm read 0), and a `pipefail` control in lesson promotion.
- All four scripts join the `NEUTRAL` allow-list of `scripts/update_scaffold.sh`. Backlog notes record the
  input-bound principles (`BASELINE-IDENTITY`, `IDENTITY-CARRIER-CURRENCY`, `SCRATCH-SLOT-HEADER`, the full
  `LIVE-DOC-CURRENCY` instrument) for a future seam.

## bedrock-scaffold 0.5.0 — the day-one batch: no agent trailers, a handoff census, no self-reported dates

`BEDROCK-MAINTENANCE.2.5`.

- ⛔ **`COMMIT.md` had the trailer rule backwards.** It told every generated project to *end commit
  messages with the project's co-authorship trailer*; the upstream maintainer ruled the opposite on
  2026-08-22 (a commit message ends with its own last line — no agent/tool attribution trailers,
  harness-agnostic). The rule is rewritten and `.githooks/commit-msg` now refuses the known
  agent-attribution shapes mechanically; a human co-author's `Co-Authored-By:` still passes.
- **Added `scripts/check_no_background_jobs.sh`**, the handoff census: pattern-free (`lsof` over the
  caller's uid — an open handle under the repo, or a command line naming the checkout), run before
  a session ends; deliberately not a commit gate. Named in `CLAUDE.md`'s non-negotiables.
- **Added the `LIVE-DOC-CURRENCY` doctrine** (principle): no tracked `.md` reports its own currency
  (`Last updated:` and kin) — git carries it, a hand-kept date is false the day after. The field is
  deleted from `docs/tasks/TEMPLATE.md` and the maintenance tree; `scripts/check_live_doc_currency.sh`
  is structural over `git ls-files '*.md'` with a 3-arm `--self-test`.
- Both scripts join the `NEUTRAL` allow-list of `scripts/update_scaffold.sh`.
- Part 2 of the same transfer (`LESSON-PROMOTION`, `ROUTING-EVIDENCE`, `GAP-CLAIM-CENSUS`, a fresh
  `TABLE-ARITY-RATCHET`) is classified in the `.2.5` leaf and queued as `.2.6`, paused by the maintainer.

## bedrock-scaffold 0.4.0 — TASK-ACCEPTANCE: a change lands with evidence, not with a claim

`BEDROCK-MAINTENANCE.2.4`.

- **Added the `TASK-ACCEPTANCE` doctrine**: a staged CODE change must be owned by a task-tree leaf
  whose checklist has ROOT CAUSE / ADDRESSED / NO REGRESSION **ticked**, each backed by output from
  a tool that was actually run — **inside that box's own bullet**.
- ⭐⭐ **Box-scoping is the soundness property**, not a nicety. It closes two measured leakage
  holes: a co-staged, unrelated leaf supplying the evidence, and a token matched anywhere in the
  file rather than in the box it backs. `CTRL-1` demonstrates it directly — a whole-file grep
  PASSES the fixture that the shipped check REJECTS.
- **Neutral by seam, not by rename.** Default signatures are universal to any Rust project
  (`error[E1234]`, `could not compile`, `clippy::…`, `test result: ok`, panics, profilers) plus any
  project's build-flow forensics (`git log -S`, `shellcheck`, `bash -n`, `make -n`, `ENOSPC`…).
  Project-specific tooling is declared in `.doctrine/evidence_tokens.txt`, and what counts as a
  code change in `.doctrine/code_paths.txt` — both optional, both defaulted, both documented in
  `.doctrine/README.md`. ⭐ `CTRL-4`/`CTRL-4b` prove the seam is load-bearing: the same leaf passes
  WITH the declaration and fails WITHOUT it.
- ⛔ **Fixed a portability defect the probes caught**: the box extractor used `IGNORECASE`, a gawk
  extension that BSD awk silently ignores — every leaf would have been reported as having no
  checklist. Rewritten with POSIX `tolower()`.
- ⚠️ Honest limit, stated in the check itself: it proves the author cited something re-runnable,
  never that the output is true. The un-fakeable leg is re-running the cited command in CI.
- Probes 9/0; `make gate` 8/8.

## unreleased — the admission test asks about VALUE first, not vocabulary

`BEDROCK-MAINTENANCE.2.3`. Process only; no check changed, so `DOCTRINE_VERSION` is unmoved
(`MAINTAINING.md` and the maintenance tree are maintainer-only, not re-syncable spine files).

- **The admission test is now two ordered questions.** Q1 (primary, about VALUE): *does this
  objectively benefit any present and any future project?* — answered by stating what the check
  prevents using no project's nouns, then asking whether a brand-new project is better off with it
  on day one. Q2 (secondary, a filter): *can it be expressed without domain nouns?*
- ⛔ **Q2 cannot substitute for Q1.** A check can score 0 domain nouns and still encode a workflow
  only one project needs — neutral vocabulary, project-shaped substance. Q2 measures whether a
  thing CAN be neutralized; Q1 asks whether it SHOULD be. Running Q2 first waves impostors through.
- ⭐ **Measured worked example, which changed a verdict.** A "destructive automation must require
  confirmation" check scored well on Q2 and was ranked an easy win; its logic hardcodes a Makefile
  path and a `clean:` recipe, so it really offers *"benefits any project that builds with make"* —
  a conditional. **Rejected as-is.** Meanwhile `ROUTING-EVIDENCE` measures 0 build-system
  references and presumes only the task-tree system this template ships ⇒ promoted to top.
- **The portability seam to look for:** does the check presume anything beyond what bedrock ships?
  If yes, give it a project-declared seam or leave it upstream — never hardcode one project's
  answer and call it neutral.
- ✅ Retroactive audit: all four already-ported items PASS Q1. Nothing retracted.

## bedrock-scaffold 0.3.0 — WAIVER-ROUTING, and the neutrality bar for every future port

`BEDROCK-MAINTENANCE.2.2`.

- **Added the `WAIVER-ROUTING` doctrine** (`scripts/check_waiver_routing.sh`): a task leaf saying a
  gate DOES NOT APPLY must name the leaf that owns fixing the gate. ⭐ An author writing a waiver
  IS the gate reporting a missing capability — the highest-signal defect report a gate can get.
  Deliberately does **not** punish honesty: the waiver stays legal, it just has to name an owner.
- **Chosen by measurement.** All 15 upstream doctrines were classified by domain-dependence of
  their LOGIC (comments stripped). `WAIVER-ROUTING` scored **0** — portable essentially unchanged.
  The ranked remainder is now a frontier in `docs/tasks/BEDROCK-MAINTENANCE.md`, not a wish list.
- ⭐⭐ **The port FIXED a defect rather than inheriting one**: the origin's `printf … | grep -q …
  || continue` returns failure ON SUCCESS past the pipe buffer under `pipefail`, silently SKIPPING
  the file — a **fail-open**. Both sites here read a file instead. Threshold measured, not assumed:
  65,606 B → no SIGPIPE; 131,139 B → SIGPIPE.
- **Wrote down the neutrality bar** (`MAINTAINING.md`): every doctrine here must be objectively
  applicable to ANY project, with a measurable admission test and its honest bound — plus the rule
  that **transfer runs both ways**, after this repo's layer-C check turned out to be stronger than
  the reference deployment's.
- Probes 5/0; `make gate` 7/7; added to the `update_scaffold.sh` NEUTRAL allow-list.

## bedrock-scaffold 0.2.0 — README Stability Policy + a layer-A byte cap

`BEDROCK-MAINTENANCE.2.1`. Transferred from the reference deployment by maintainer order.

- **Added `README_POLICY.md`** (project-neutral, verbatim) — keeps `README.md` a stable landing
  page instead of a changelog/roadmap/catalogue, and states the caps rule.
- **Added the `README-STABILITY` doctrine** (`scripts/check_readme_stability.sh`): a line cap
  AND a byte cap, a dated-line (release-history) tripwire, and a required link back to the
  policy. Non-mutating; REFUSES (exit 2) rather than passing when the README or policy is
  absent. Template defaults 300 lines / 16384 bytes — generous on purpose, because they ship to
  a project whose README is not this one; tighten after your own trim.
- ⛔ **Closed a bypass the spine was itself shipping.** `scripts/check_memory_architecture.sh`
  capped layer-A `MEMORY.md` by LINES only (cap 120, no byte bound), exactly as
  `MEMORY_ARCHITECTURE.md` §9's reference check prescribed — so **every adopting project
  inherited a bound that does not bind.** Measured on a real project running this spine:
  60 lines (passing, exactly at its cap) carrying **138,403 bytes** — 2,306 B/line, one line of
  18,816 B. Now both caps, in the check **and** in the standard (§6 / §9 / §9.1).
  Layer-A caps: **50 lines** (tightened from 120, to match the "≤ ~50 lines" §6 already stated)
  and **7168 bytes**. Both env-overridable.
- Both new files added to the `update_scaffold.sh` NEUTRAL allow-list, so existing projects
  pull them with `scripts/update_scaffold.sh <bedrock-url>`.
- Verified: `make gate` 6/6 green; a 13-line / 19,304-byte fixture is REJECTED by the byte cap
  while being well under the line cap; the **retired** layer-A guard PASSES that same file
  (exit 0) — the change is proven necessary by execution, not by argument.

Changelog-style summary of completed work + its validation (internal continuity surface;
the immutable audit trail proper is `git log` — memory layer D). Newest first.

## _(YYYY-MM-DD)_ — bootstrap

Instantiated from the `bedrock` discipline-spine template. Next: replace `ROADMAP.md` and
seed the first task-tree.
