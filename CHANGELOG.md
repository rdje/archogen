# CHANGELOG.md

Changelog-style summary of completed work and its validation. Newest first. The
`bedrock-scaffold` entries below the separator are the provenance of the discipline spine
this repository was created from, not archogen's own history.

## archogen — `TASK-ACCEPTANCE` verifies the leaf that owns the change, and refuses when it cannot tell which one that is

`ARCHOGEN-PROGRAM-0086` (leaf `PROGRAM.21`). **Fixed, not filed** — the gate that enforces root-cause,
effect and no-regression evidence on every code change had been reading the wrong checklist since the
repository was created, and saying it had checked the staged one.

- ⛔ **The hole was cross-*leaf* leakage.** One `awk` ran over the whole tree **file** and stopped at the
  first box it found (`if (inbox) exit`, line 111 at `HEAD`), so a file of N leaves verified exactly one
  — whichever came first. `docs/tasks/M1.md` carries **32** ticked ROOT CAUSE boxes; the awk captured
  **line 53**, leaf `M1.1`, written `2026-09-13`. The check had already been hardened twice, for
  cross-*file* and incidental-*prose* leakage, and its header called box-scoping "the soundness
  property" — this was the same hole one directory level down.
- ⭐ **Both real commits it mis-gated were replayed as fixtures**, not recalled: each commit's own
  `docs/tasks/M1.md` and one staged source file extracted with `git show <commit>:<path>` into a
  throwaway repo, run pristine and then with the committing leaf's ROOT CAUSE box unticked, old check
  against new. `cd355ef` and `3a6bbb9` both give the **old** check a byte-identical `exit=0 OK` either
  way — so neither commit's own boxes could have changed its verdict — while the new check passes
  pristine naming `M1.13.1`/`M1.13.2` and refuses when that leaf's box is unticked.
- **The owner is declared, never inferred.** It comes from `TASK_ACCEPTANCE_LEAF`, else the `(leaf <ID>)`
  token in the pending message's subject through the new `.doctrine/commit_message_file` seam, else the
  check **refuses** — because falling back to the first checklist in the file *is* the defect, and a
  green verdict about a leaf nobody claimed is worse than no verdict. The inference route was priced
  first and is dead: the leaf sections a commit's diff touches agree with the subject on **1 of 7** real
  code commits. Leaf sections are sliced by **exact string** match, not a regular expression, so an id's
  `.` cannot match another leaf's.
- ⛔ **The new arms' first oracle was unsound in the same way the check was.** Written as
  `[ "$rc" -ne 0 ]`, the first run scored **`4 pass / 5 fail`** — and all four passes were on
  **`exit 127`**: `$0` was a relative path and every arm `cd`s into a throwaway repo, so the check was
  never found. The tally read like partial progress; the truth was that nothing had been exercised. Now
  each arm requires the subject's **own** exit code and its **own** identifying output, and mutation B2
  reproduces the false green deliberately (`4 pass / 5 fail` weak oracle vs `0 pass / 9 fail` exact).
- Three mutations, each restoration proven byte-identical: A restores the first-leaf fallback →
  `8 pass / 1 fail`, arm 4 only, so the refusal is load-bearing; B breaks the invocation path with the
  exact oracle → `0 pass / 9 fail`; B2 the same with the weak oracle → the false green. Restored:
  **9 pass / 0 fail**, `exit=0`.
- ⛔ **Two silent corruptions in the rewrite, caught by diffing the blocks meant to be preserved:**
  `DEFAULT_SIG` had been widened with a token that does not exist (`\bspindb\b` beside `\bspindump\b`),
  which would have let a box pass on evidence that is not evidence, and a historical comment had been
  re-dated `awk version 20200816` → `20260816`, falsifying a record of what an earlier cut rejected.
  Both restored byte-identical to `HEAD`. Neither is visible reading a rewrite top to bottom.
- Lockstep: the check's header now names three leakage holes and three honest limits; the new seam is
  documented in `.doctrine/README.md` with its measurement; `DOCTRINE_ENFORCEMENT.md` §4's row; a
  `TOOLBOX.md` row; `docs/knowledge/verify-the-mutation-applied.md` gains the oracle section, a fifth
  `answers:` line and a How-to-apply bullet — an arm has **three** parts that can each be weaker than
  the property (needle, mutation, oracle). ⭐ `PROGRAM.18`'s population moved from ten controls without
  a RED arm to **nine**, and its frontier row says to re-run the census rather than reuse the figure.
- `make focused` → `passed — 3 / 0 / 0`; `make gate` → 13 doctrines green; `bash -n` clean. **This
  commit is the new check's first real exercise: it gated itself**, so the verdict it printed is about
  `PROGRAM.21`'s boxes and not `PROGRAM.1`'s.

## archogen — two findings raised in conversation are now owned by a task tree, and one of them was false

`ARCHOGEN-PROGRAM-0083` (leaf `PROGRAM.25`). Docs only. §15 makes raising an issue the *first* step and
a verified fix the goal; both of these stopped at a reply, so neither would have survived the session.

- ⛔ **The PDF finding was false as raised, and measuring before filing is what caught it.**
  `read_file`'s PDF bridge answered `pdftotext is not installed` for chipdoc's SiFive datasheets, and
  that was reported onward as "this machine has no PDF text extractor, so §3.2's `board-first` facts
  cannot be read here". `command -v pdftotext` resolves to `/opt/homebrew/bin/pdftotext` (Xpdf **4.06**)
  and `pdftotext -f 1 -l 6 <the FE310-G002 datasheet> -` prints its first page. The bridge cannot see
  the host `PATH`, so its message describes the bridge and not the machine. **Consequence: the opposite
  of the one reported** — those datasheets are readable, and `M5` is blocked on procurement alone. A
  second blocker on `M5`, drafted on that premise, is **not** filed. What is filed is the route: a
  `TOOLBOX.md` row and a note beside the chipdoc record's inventory, so the next session does not
  re-conclude the documents are unreachable.
- ⭐ **`docs/decisions/reference_sibling-project-semulith.md`** records that `../semulith` exists, is
  **read-only** under §21, builds CPU/DSP models, and names archogen as a concrete consumer with an
  integration contract and a tree of its own waiting on archogen's real eADL interface. A pointer,
  deliberately **not** an analysis — that was offered and declined. Indexed, and the Knowledge Map
  regenerated.
- ⛔ **The census needed both halves, and one of them is a trap.** `git grep -il semulith -- ':!vendor'`
  → **0**: archogen's own tracked files named it nowhere. `grep -ril semulith vendor/` → **40**. The
  difference is that `vendor/linkedspec` is a **submodule**, which `git grep` skips entirely — so the
  tracked-file census alone would have supported "the two projects have never met", when in fact they
  met through the vendor's issue ledger during `M1.20`. A census that cannot see part of its population
  is worse than no census.
- `GAP-CLAIM-CENSUS` fired on the first draft of this filing — a log row saying "owned by nothing" with
  its census on the leaf rather than beside the claim. The census went inline rather than the wording
  going soft, because rewording around a gate leaves the gate unexercised.

## archogen — `eadl/1`'s value domain becomes a stated property of the language version, and both measurements it was routed on turn out to be false

`ARCHOGEN-M1-0082` (leaf `M1.13.2`). Finding F-F settled: **the domain stays exact signed 64-bit**, and
widening it stays deferred — but now on a reason that survived being re-measured, which the original one
did not.

- ⛔ **Both measurements `M1.13` routed this leaf on were false.** Re-derived rather than carried, per
  `docs/CLAIM_VERIFICATION.md` leg 1 and its auditor's asymmetry:
  - the census said "**27** distinct integer literals; the largest is `0x1000_0000` (2^28) and the largest
    decimal is 2^32" — an **unstated scope** (27 is the *non-negative* count) and a **radix-scoped maximum
    published as the overall one**, in one self-contradictory sentence. The frontend measures **24**
    distinct values over **194** occurrences, largest **4294967296** (2^32, 33 of 63 magnitude bits),
    smallest `-1`, and **0** literals refused by the domain. Census of the copies, both directions:
    **7** restatements over **3** live surfaces, all corrected at their sources with the originals quoted
    rather than retyped.
  - "no address a standardized target can have reaches 2^57" is true of every **physical** address and
    false of every canonical **high-half virtual** one. `0xFFFF_FC00_0000_0000`, `0xFFFF_FFFF_C000_0000`
    and `0xFFFF_FF80_0000_0000` are each refused today, because an Sv39 kernel address has bits 63-39 all
    set — a magnitude of 2^64−2^38 against a limit of 2^63−1. No value is lost (`-1073741824`, the same
    64-bit pattern, reads and round-trips); what is refused is the **unsigned spelling** a datasheet, a
    linker script or a device tree prints. Sv64 is also stronger than "Reserved": the pin says it "**will
    be defined** in a later version of this specification".
- ⭐ **What closed it is rows and an instrument, not sentences.** §1 rule 9 states the domain as a property
  of `eadl/1` and cites the code that enforces it; new rule 10 states the honest limit and names the
  trigger that would end it. **Four new executed rows** carry both, including a hexadecimal lower boundary
  that nothing had pinned — the spelling finding F-E's `i64::MIN` defect originally lived in. And
  `crates/eadl-front/examples/literals.rs` makes the census a **command** rather than a description of
  one, asking the frontend instead of a second regular expression, which is the mechanism by which the
  original figure went wrong; it names its population as its arguments and counts every atom category, so
  one holding nothing is visibly zero.
- ⛔ **One code carried three repair wordings** — "split the quantity" in the hexadecimal arm, "reduce the
  digits" in the decimal one, and a third merged in §4's column — and none of them told the author of a
  kernel address anything usable, which §4's own preamble calls the most expensive diagnostic to receive.
  Now one shared `OVERFLOW_REPAIR` naming the spelling that works.
- Two new reader tests, and **two mutations seen firing** with both restorations proven byte-identical:
  dropping the repair clause fails the high-half test; parsing the hexadecimal magnitude as `i64` fails
  the new unit test **and** `reference.md:145`'s row — one plausible regression now caught in two places
  where before it was caught in neither.
- ⭐ `arm_24` fired on the real tree for a reason worth keeping: its mutation removed **one sentence** from
  a chapter this leaf had given a second citation, so it no longer produced the state its name claims.
  Widened to every occurrence with a post-mutation assertion, and promoted into
  `docs/knowledge/verify-the-mutation-applied.md` — **assert the post-mutation state, not only that the
  needle matched**.
- ⚠️ Routed onward, measured and sized: the census reports **`decimal literals : 0`** over all 75 tracked
  descriptions, independently confirmed by `grep`. §1 rule 2's exact rationals are pinned by table rows
  and unit tests but exercised by **no shipped description**, so the conformance suite `M1.13.4` builds
  would not test them. Both commands are on the leaf and on the frontier row.
- `docs/decisions/decision_eadl1-value-domain.md` carries the decision, both censuses as re-runnable
  commands, the ISA measurements with their pin and digests, and **three named triggers** that would make
  it wrong — the first description needing a high-half address, Sv64 being defined, and any layer that
  starts treating an integer as a bit pattern (the only one that would make deferral *expensive* rather
  than merely late). `476 passed / 0 failed` over 37 suites (baseline 474; delta = the two new tests).

## archogen — `TASK-ACCEPTANCE` is measured active-unsound, not latent: a real code commit was gated on a different leaf's checklist

`ARCHOGEN-PROGRAM-0081` (leaf `PROGRAM.21`). **Filed, not fixed** — docs only, no code, so the gate
stays unsound and the fix is still owed.

- `PROGRAM.21` was written with the severity measured as **latent**: five committed leaves carried no
  ticked ROOT CAUSE box, and `git show --stat` on each reported zero code files, so no code change had
  landed unboxed. That claim is now superseded by an instance rather than an argument.
- ⛔ **Commit `cd355ef`** (`ARCHOGEN-M1-0080`, leaf `M1.13.1`) staged five Rust files and the gate
  printed `task-acceptance: OK (every staged code-change leaf carries a ticked, evidence-backed
  checklist)` with `exit=0`. The box it read was **`docs/tasks/M1.md` line 53 — leaf `M1.1`, written
  `2026-09-13`**, thirteen days stale and about a different subject.
- Three measurements, all re-runnable: a `grep -c` census returning **31** ticked ROOT CAUSE boxes in
  the file where the leaf recorded 24, of which exactly one is ever read; the check's **own awk**
  extracted and run over the file, printing `captures line 53 ticked=1`; and a **mutation** — the
  committing leaf's box unticked in a scratch copy, the same awk re-run over both files, giving an
  **identical capture**.
- ⭐ The mutation is the finding. `M1.13.1`'s checklist was in fact written, ticked and evidence-backed,
  so the commit was honest and the verdict was right — **for the wrong reason**. Had those boxes been
  empty the verdict would have been byte-identical. The gate supplied that commit no protection and
  reported that it had. Every code commit on every leaf of `M1.md` after `M1.1` is in the same
  position, and `M1.md` is the tree the project is actively working in.
- ⛔ **Interim mitigation: none inside the gate.** The only thing between this and an unboxed code
  commit is author discipline, which is what a gate exists to not depend on. The fix is bounded and
  already specified on the leaf — scope the boxes to the leaf that owns the staged change, make the
  success message true of what was examined, refuse with the leaf's ID in the message, and arm
  `--self-test` with the *second-leaf-with-no-boxes* shape measured here — but it needs the owning-leaf
  question answered honestly, since a pre-commit hook has no commit message to read.

## archogen — the escape set is closed *and* sufficient, so canonical form can no longer carry a raw control byte

`ARCHOGEN-M1-0080` (leaf `M1.13.1`, the first of the language freeze's five children). Closes finding
F-G, which `M1.12` routed to the freeze because settling it changes what the language can express.

- ⛔ **The defect was worse than the finding said, and the reason it survived is the interesting part.**
  `(probe "a<ESC>b")`, `(probe "a<BEL>b")` and `(probe "a<NUL>b")` each read cleanly with **no
  diagnostic** and each printed the byte raw into canonical text — the artifact §12 M4 hashes and
  compares, and which §3 defines as one form per line. The NUL case truncated the output of the very
  test that reported it, and `grep` declared its own input a *binary file*. Two legs checked §3's "no
  control character" rule and both were green, for two reasons: **0 of 75** tracked descriptions hold
  such a byte, so the coverage leg's population could not contain the case — and the second
  implementation the legs compared against, `encode_canonical` in the shared test helper, had the
  printer's same four arms and same fall-through. It was not a copy of the printer, it was a copy of
  the printer's *omission*.
- **The reader already had the rule and stopped applying it at the opening quote.** The same byte
  *between* forms was refused as `read-unexpected-character` — "a byte that can start nothing … a stray
  control character". So this was an inconsistency, not a missing rule, and the fix applies the existing
  rule uniformly rather than inventing one.
- **Fix, at the producer rather than the door.** `form.rs`'s printer now escapes every Unicode `Cc`
  character — `\n`, `\t`, `\r` keep their names, everything else prints as `\u{…}`. That placement is
  forced, not preferred: `Form::Str` is constructible outside the frontend (`lib.rs` re-exports it, an
  enum's variant fields carry its visibility, `Span::new` is public), so a value that never passed a
  reader still has to print safely. `reader.rs` gains `\u{…}` — one to six hexadecimal digits — and
  refuses a raw control character other than tab as `read-control-character`.
- **`\u{…}` has two refusals, and collapsing them would break a leg.** An escape that is not shaped like
  one (`\u{}`, `\u{1bx}`) is not well-formed: `read-bad-escape`. One shaped correctly but naming no
  character — a surrogate, or a code point past `10ffff` — is well-formed and outside the domain:
  `read-escape-out-of-range`, `refused` in the reference's notation. That is the same distinction §1
  draws between `read-malformed-number` and `read-number-overflow`, and it is load-bearing because the
  recognizer derived from the grammar must reject every `error` row and accept every `refused` one.
- **The grammar moves with it.** `escape` gains `unicode_escape`; `string_char` gains `- control`, a new
  production naming `Cc` minus tab in the language's own escape notation. That also closed a live
  disagreement nobody could reach: the grammar accepted a raw line feed inside a string that the reader
  refused as `read-unterminated-string`, and no probe or corpus file could contain the input.
- **A notation the reference was missing, which unblocks `M1.26`.** `<0xNN>` in a source cell names a
  character that cannot be written into a markdown table — invisible in the cell, and a raw line feed
  would end the row. That is why `M1.12.3` deferred an executable input column on §4's table and
  `M1.26`'s gap (b) inherited the deferral; it is now solved rather than routed around, and a raw NUL
  is an executed table row for the first time.
- **Four false statements found in normative surfaces and corrected in place**, none of them in this
  leaf's acceptance criteria: §2 rule 5 claimed a character with no escape cannot be written into a
  string; §3 rule 3 named `provenance.rs` as held to the same escape set when that `quote()` writes
  **JSON** (`\b`, `\f`, `\u00XX`); `grammar.md`'s notation table said `a - b` excludes single characters
  only, which `symbol_char = any - whitespace - …` already contradicted; and the `\0` rationale there
  ("a NUL byte has no use in a description") needed narrowing, since a NUL is now writable *by name* —
  six visible characters, which is the point.
- ⭐ **Honest limit, measured and routed rather than hidden.** `Cc` is not the whole of "invisible": a
  zero-width space, a BOM or a bidirectional override still prints raw and nothing refuses it. Census:
  **0 of 75** tracked descriptions hold one, and every non-ASCII character the corpus does hold is
  visible. Stated in §3 and routed to `M1.13.3`, the last child that can narrow the language for free.
- **Validation.** BEFORE, measured with the rows added and no production code moved:
  `the_grammar_and_the_reference_agree_on_the_literal_space` → `FAILED` with **4 violations** naming
  exactly the raw ESC, NUL, C1 and line-feed rows, and the reader leg with **3**. AFTER: conformance
  `12 passed / 0 failed` (was 6/4), reference `34 passed / 0 failed`, **474 passed / 0 failed over 37
  suites** (baseline 471 over 37), `make focused` → `passed — 3 / 0 / 0`, `book-anchors: OK (19
  chapter(s), 2 normative document(s))` with self-test `6/6`. Two mutations of the fix were each seen
  firing and each restoration proven byte-identical with `diff -q`: deleting the printer's control arm
  gives `49 passed; 1 failed`, the one failure being the new test that constructs a `Form::Str` holding
  a NUL, ESC, DEL and a C1 control **without a reader** and requires canonical text to escape all four
  and read back structurally equal. No corpus file, example or fixture changed.

## archogen — a programmatic interface is now part of the roadmap: one engine API, a wasm binding, and an MCP server

`ARCHOGEN-API-0078` (leaf `API`). Docs only — no code, no behaviour change.

- **`ROADMAP.md` §10.4 added by director ruling `2026-09-28`.** §10.2's CLI stays the human interface;
  the same operations are also exposed programmatically so a browser, an editor, a build farm or an agent
  can drive archogen without a shell. One declared, versioned, **transport-neutral engine API** — a
  description as text plus a profile in, a structured result out — with the CLI as a *consumer* of it, so
  a capability cannot exist behind one surface and not another.
- ⛔ **Both builds are outside it, by ruling.** Neither archogen's own compilation nor `archogen build`
  (system generation, §10.3) is controllable programmatically. That is not only policy: a server must
  exist before it can be controlled, so "MCP-controllable during the build" is a bootstrapping fixed
  point. Excluding generation is also what keeps the whole surface pure — computation over an in-memory
  description, no filesystem, no subprocess, no ambient authority — which is what makes it safe to hand
  to an arbitrary agent, and what makes wasm and MCP one API with two transports rather than two projects.
- **Sequenced behind the `M1` language freeze.** `M1.13` settles the integer domain (F-F) and the escape
  set (F-G), which are exactly the API's numeric types and its string encoding. Declaring first means
  declaring twice.
- **The evidence vocabulary crosses the boundary unchanged.** Every programmatic response carries §5.5's
  verdict; a result without one is how `tool-failure` becomes `established` in a consumer that never saw
  the distinction. §7.1 applies to a machine-readable response exactly as to a printed one.
- **Feasibility measured before it was promised.** A census over every crate's production half: six of
  eight crates touch no `std::fs`, `std::process` or `std::env`; `rt-core` and `rt-reference` are already
  `#![cfg_attr(not(test), no_std)]` and the integration tier already compiles for bare metal; the
  workspace has **zero** third-party dependencies, every `[dependencies]` entry being a `path =` sibling;
  and every library crate already has a `lib.rs` with public items, so a de facto API exists that was
  simply never declared.
- ⛔ **A claim made in conversation and then corrected by measurement.** `archogen build` was described as
  shelling out to `cargo` and QEMU, making an MCP-exposed build a remote-code-execution surface. It does
  not: `Command::new` appears in **no production half** anywhere in the workspace, `archogen-cli`'s only
  `std::process` use is `use std::process::ExitCode;`, and the one place that compiles a generated
  artifact is `crates/archogen-cli/tests/s0_oracle.rs:609` — a test. `archogen build` writes a crate tree
  and stops. The corrected finding is stronger and is now `API.2`: **the invariant that makes agent
  control safe is written down nowhere** — `git grep -niE 'spawns? no|no subprocess|does not execute|never
  executes|no child process'` over tracked files returns nothing — so a `Command::new` added to a product
  crate tomorrow passes every gate in the tree.
- ⛔ **A second correction, this one against a recorded decision.** An early framing held that a
  "non-core" transport crate would sit outside `decision_zero-dependency-engine-core.md`. Reading the
  record says otherwise: its "How to apply" forbids a `[dependencies]` entry in **any** crate under
  `crates/` without a decision record naming the §4.4 trust category and the claims its compromise would
  invalidate. The shape that satisfies both is a one-way dependence, transport → engine, so the generator
  and the checker share nothing new and F30's independence argument is untouched.
- **Seven leaves, two of them unblocked.** `API.1` makes the wasm build a §14.3 tier step in the shape of
  the existing `no-std-build`, so feasibility stays a measurement rather than becoming an opinion; `API.2`
  states and gates the no-subprocess invariant. `API.3` (the declared API) is the load-bearing leaf and
  waits on `M1.13`; `API.4` the instance model and the resource limits an untrusted consumer makes
  necessary; `API.5` the wasm binding; `API.6` the MCP server, deriving its tool list from
  `crates/archogen-cli/src/spec.rs` — which already declares the command surface as data with a
  three-state `Maturity`, renders help from it, validates the parser against it and names the owning leaf
  for every non-built state, so the tool list is a fourth consumer of one table and not a second list;
  `API.7` the book chapter, its own leaf so it cannot be quietly skipped when the server lands.
- **Three open questions recorded with owners rather than decided by analogy** — whether `archogen verify`
  may be exposed at all (it invokes a toolchain and an emulator through `xtask`, unlike everything else on
  this surface, and needs its own ruling); what an instance is bound to and for how long; and whether the
  API surfaces the model layer's diagnostics, which `M1.26` has just measured are stated normatively
  nowhere.
- **Validation:** docs only, so no Rust tier was required; run anyway. `make focused` →
  `passed — 3 passed, 0 failed, 0 unavailable`; `=== all doctrines green ===`;
  `check_knowledge_map.sh` → `rc=0` after regenerating for the new tree and decision record;
  `cargo test --all` unchanged at **471 passed, 0 failed over 37 suites**.

## archogen — the language reference is closed, and its instruments now read the book

`ARCHOGEN-M1-0077` (leaf `M1.12.5`, closing `M1.12`).

- ⭐ **Two new legs turn the reference's instruments on the book**, which is the only view of this
  project its director has. **Leg 8** requires every diagnostic a chapter renders as `error[<code>]` to
  be a code a production source really emits, and every one whose prefix §4 governs to be a row of §4;
  a rendered severity other than `error` violates §4 rule 1. **Leg 9** requires every chapter that
  publishes the surface to cite `docs/semantics/reference.md`, the population *derived* — it renders a
  governed diagnostic, or it cites the grammar — rather than listed, so a chapter written tomorrow is
  in scope without anyone editing a test. This is `BOOK-ANCHORS` one level deeper: not "does the
  citation resolve" but "is the thing the chapter shows still a thing the engine does".
- **Four chapters cited the grammar and nothing anywhere in the book cited the reference** —
  `reading.md`, `modules.md`, `kinds.md` and `s0.md`. The grammar says what a description *is* and the
  reference says what it is *worth*; a chapter pointing at one made it look like the whole definition.
  Each now carries a pointer that says something true about the reference's content rather than a bare
  path. Leg 9's first run named all four, with the code that put each in the population.
- **`BOOK-ANCHORS` walks two populations instead of one**: the book's chapters *and* the normative
  documents under `docs/semantics/`, because the grammar and the reference are prose about the code in
  the same sense and are the authority the code is held to — a rotted citation there sends an
  implementer to nothing. An empty normative population is a **breach** rather than a "not applicable",
  since the frontend `include_str!`s the reference and cannot compile without it. Three RED arms became
  six, three per population, and the chapter count was deliberately held at 19 so a refactor did not
  move a reported figure.
- ⛔ **The closure review measured two gaps outside the parent's criteria and filed them rather than
  folding them in** — `M1.26`. **(a)** The book renders fourteen distinct diagnostic codes and §4
  governs six; the other eight come from the model layer, which no normative document states. `M1.12.3`
  had censused this with a prefix-shaped pattern and reported seven citations — correct, and a subset of
  a fourteen-code population it could not see. **(b)** No input is pinned to any row of §4, so
  **25 of the 53** stated codes are named by no test, which **falsifies the reason `M1.12.3` recorded
  for deferring** the `fires on` column: it holds for `read-` (7 of 7) and not for the 46 codes `M1.12.4`
  added (`module-` 11 of 24, `schema-` 10 of 22). The parent still closes, because its criteria ask that
  rules be *stated* and legs 4/5 pin that exactly at 53 stated / 53 emitted / 0 rotted rows — but a
  deferral is a claim, and this one was re-measured when the population moved.
- **`M1.12`'s acceptance re-checked criterion by criterion against measurement.** The frontend's six
  source files emit from exactly two (`reader.rs` 7 codes over 11 call sites, `module.rs` 24);
  `diagnostic.rs`, `form.rs`, `lib.rs` and `source.rs` emit none. Every `Diagnostic::` constructor in a
  production half takes a **string literal**, so no code reaches a diagnostic dynamically and the
  literal-shaped census is sound rather than lucky — which is the assumption both legs rest on.
- ⛔ **The new arms were mutation-tested, not trusted.** Widening leg 8's first rule from governed codes
  to every code — one token — turned **six** tests red, including the green leg, which is the pinned
  violation counts proving themselves. One arm requires an ungoverned code the engine *does* emit to
  stay unreported, so the leg cannot pass by demanding the reference govern rules that are not language
  rules.
- **Two defects fixed in passing, both small:** `check_book_anchors.sh`'s self-test opened a `mktemp -d`
  it never used — dead code writing off-volume, against §13 — replaced by a `trap` over the in-repo
  victims; and `TOOLBOX.md`'s reference-value row said "its **four** declared sources", an ungated figure
  of exactly the class `M1.23`/`M1.24` exist to end, removed rather than retyped.
- **Also filed: `PROGRAM.24`**, the mirror direction of `BOOK-ANCHORS` that nothing covers — does a
  capability the codebase has get described in the book? One live instance measured: the `rt-analysis`
  crate is named nowhere in `docs/book/`, and `analysis.md` is the chapter about what it establishes. It
  passes both `BOOK-ANCHORS` legs, because a chapter can be perfectly anchored and still never say where
  its subject is implemented.
- **Validation:** `cargo test -p eadl-front --test reference` → **34 passed / 0 failed** (was 24);
  `book-anchors: OK (19 chapter(s), 2 normative document(s))` with **6/6** self-test arms; chapters
  citing the reference **0 → 4**; `make focused` → `passed — 3 passed, 0 failed, 0 unavailable`;
  `cargo test --all` → **471 passed, 0 failed over 37 suites** (baseline 461 over 37);
  `=== all doctrines green ===` over 13 checks; `check_knowledge_map.sh` → `rc=0`.
  `git diff --stat HEAD -- 'crates/*/src'` is empty: no production code changed.
  ⚠️ `make integration` → `failed — 6 passed, 1 failed`, the one failure being `emulator`
  (`TARGET_VERIFIED=no`), pre-existing and owned by `M2.8.2`; `git diff --stat HEAD -- crates/rt-core
  crates/rt-analysis crates/rt-reference targets scripts/target_emulator.sh xtask` is empty, so nothing
  that step reads was touched. `ROADMAP.md` needed no edit, measured rather than assumed: its only
  relevant claim is "`docs/semantics/` with at least twenty worked cases", and the corpus holds 62.

## archogen — the language reference is complete, except for the version identifier

`ARCHOGEN-M1-0076` (leaf `M1.12.4`).

- **§5 comments and the header convention, executed.** A comment runs from `;` to the end of its line
  and is retained; a `; key: value` line at one space of indentation with a lowercase-kebab key is a
  **header**, which is how the boundary corpus states its own verdict and how F27 reads it. The rules
  are a machine-read table run against `Document::comment_headers`, including the cases that must yield
  **no header at all** — a table of successful parses alone would pass on an implementation that treated
  every comment as one.
- ⭐ **Both discriminators are proved load-bearing from the document.** One arm takes the indentation
  off a wrapped rationale line whose text is exactly a well-shaped key and a colon, and the leg reports
  the spurious header that appears; the other lowercases `Expected` in a line of prose, and the leg
  reports a header where §5 says there is none. Neither arm touches the code, so each shows it is the
  *row* pinning the rule. Both cases are real: `counter-width-and-rate.eadl:10` and
  `examples/bounded-queue/system.eadl:8`.
- **§6 modules and imports** states what an import produces — an *instance* addressed by its dotted
  alias path, so one module imported twice under two aliases is two instances with their own parameter
  bindings — plus alias defaulting to the last dotted segment, version satisfaction (same major, at
  least the stated minor, and a major bump is never silently accepted because §15's promise is that a
  description retains its meaning), children-before-parents elaboration, and why a cycle is refused
  rather than resolved.
- **§7 kinds and `defkind`** states the trusted foundation: `defkind` is the only primitive whose
  meaning is Rust, every other kind is declared in eADL with it, and §5.6's prohibition is enforced
  *against the facility that declares the language* — the boundary classifier runs over the `defkind`
  form itself, so `(defkind x (implementation …))` is refused by the same machine that refuses it
  anywhere else. It also states what a schema does **not** check: the declaration frame, not the
  constraint vocabulary inside a clause.
- **The reference now declares four normative sources**, so §4's census covers **53** diagnostic codes
  rather than the seven the reader emits — 46 rules the language enforced and no document stated,
  extracted by instrument (code, message and repair hint) and then restated normatively. `boundary.rs`
  is named as deliberately *not* declared: it classifies implementation syntax, which is a profile
  concern, and a normative document's scope is worth less for being wider than the document.
- **Every literal row now round-trips**: canonical text is read back and must be structurally equal to
  what produced it. Comparing the text pins the spelling; re-reading it pins the contract — and finding
  F-D violated exactly that, where the control-character property catches only the control-character
  case.
- **Validation:** `cargo test -p eadl-front --test reference` → **24 passed / 0 failed** (was 19);
  `make focused` → `passed — 3 passed, 0 failed, 0 unavailable`; `cargo test --all` → **461 passed,
  0 failed over 37 suites** (baseline 456 over 37); `=== all doctrines green ===`.
  `git diff --stat HEAD -- crates/eadl-front/src crates/eadl-model/src` is empty: no production code
  changed, so the rules were written down and checked rather than fitted to an implementation. No book
  chapter became false — `reading.md:99-103` already stated that the header format needs two independent
  discriminators; pointing the book at the reference is `M1.12.5`'s.

## archogen — a diagnostic code is now a stated rule, not a string in a call site

`ARCHOGEN-M1-0075` (leaf `M1.12.3`).

- **`docs/semantics/reference.md` §4 states every code the frontend can emit** — when it fires and what
  to do about it — and the reference now *declares the sources it is normative over* in a machine-read
  table. The census takes its population from that declaration, so widening the reference's scope is a
  reviewable edit to a normative document rather than a change to a list inside a test.
- ⛔ **The obvious census pattern finds nothing, and that was measured before the scanner was written.**
  A grep requiring the code on its constructor's own line — `Diagnostic::error("…"` — returns **0 of 7**
  codes for `reader.rs`, **0 of 24** for `module.rs` and **0 of 22** for `kind.rs`, because every call
  in this codebase puts the code on the *next* line. A producer→document leg built on it would not have
  reported a short list; it would have reported that the code and the document agree perfectly, having
  compared two empty sets. So the census runs in **both** directions and refuses an empty population
  outright.
- ⭐ **The two directions pin the code set exactly.** Every emitted code has a row and every row is an
  emitted code, so a scanner that missed one leaves a stated-but-unemitted row and a scanner that
  invented one fails the other leg — a green run is evidence about the scanner, not only about the
  table. The `#[cfg(test)]` cut is load-bearing too: without it, `diagnostic.rs` would contribute
  `read-example` and two test locals named `e` and `w`.
- **Severity is censused with the code**, which makes §4's rule "every diagnostic the frontend emits is
  an error" a checked claim rather than an observation. §5.5's repair-direction requirement is checked
  per row for the same reason.
- ⛔ **A census claim in this leaf's own checklist was wrong once, and the correction is recorded
  rather than quietly fixed.** The first sweep of `docs/book/src` for code citations returned one hit
  and supported the sentence "no book chapter cites a diagnostic code". A second sweep without the
  backtick requirement returned **eight**, seven of them real — `read-malformed-number`,
  `read-unclosed-list` (twice), `schema-unknown-clause`, `module-circular-import`,
  `module-conflicting-export`, `module-incompatible-version` — and one false positive (`read-sequence`,
  a protocol name in prose). The book therefore cites four codes from sources this reference does not
  declare yet; that is `M1.12.4`'s, and the cross-surface leg it makes writable is `M1.12.5`'s.
- 📌 **Deferred with a reason, not dropped:** an executable `fires on` column giving each code an input
  that must produce it. It needs one more piece of notation (a control character cannot be written
  literally in a table cell, and `read-unexpected-character` fires on exactly that), and per-code
  reachability is already covered by `reader.rs`'s unit tests. `M1.12.5`'s closure review decides
  whether the parent's acceptance needs it.
- **Validation:** `cargo test -p eadl-front --test reference` → **19 passed / 0 failed** (was 11 — two
  legs and six arms); `make focused` → `passed — 3 passed, 0 failed, 0 unavailable`;
  `cargo test --all` → **456 passed, 0 failed over 37 suites** (baseline 448 over 37);
  `=== all doctrines green ===`; `git diff --stat HEAD -- crates/eadl-front/src crates/eadl-model/src`
  empty, so the rule was written down and checked rather than fitted to the code.

## archogen — the second artifact cleanup, and the two deletions it declined to make

`ARCHOGEN-PROGRAM-0074` (leaf `PROGRAM.19`, recorded as a run under the standing owner rather than a
new leaf — one leaf per day forever is not a task tree).

- **Deleted, each with a tracked regeneration path:** the test scratch under `target/tmp` (`f28`,
  `s0-build`, `s0-oracle`, `s0-provenance`, `s0-reader`, `s0-build-library.eadl`), all recreated by
  `cargo test`, and one stale `-working` incremental directory left by an interrupted build. `target`
  873 MB → 855 MB; `.bin` count 715 → 693; residue census reports all seven paths `gone`.
- ⛔ **`.app-data/pgen-generated-before-remeasure` was investigated and kept.** It looks like a
  superseded snapshot. It is not: `LS-004`'s `remeasure.sh` treats an existing backup as a reason to
  keep it and prints `a backup already exists … — kept`, so deleting it would change what a **frozen**
  instrument prints on its next run — not just lose a digest.
- ⛔ **`target/debug/incremental` (614 MB, most of the repository's `.bin` files) was measured and
  kept.** At most four `s-*` generations per crate across 121 crate directories is cargo's own
  retention, not residue; its "regeneration path" is a full rebuild of a 37-suite workspace. The
  instruction says to *check* that directory, and checking it produced a reason to keep it.
- **Verified cold**, which is the only verification that means anything here: `make focused` →
  `passed — 3 passed, 0 failed, 0 unavailable` with the scratch it consumes already deleted. The
  previous cleanup's first run failed on exactly this and a warm re-run would have hidden it again —
  `S0.7`'s lesson applied rather than remembered. `=== all doctrines green ===`; nothing tracked
  touched, `git status --porcelain` empty after the deletions.

## archogen — the literal space now has one definition, and three surfaces agreeing on it

`ARCHOGEN-M1-0073` (leaf `M1.12.2`).

- **`conformance.rs` asks the recognizer for a verdict on every literal the reference enumerates** —
  not on the corpus, and not on one probe per production. That is the only reason it can see the two
  divergences it was written for: no description the repository ships contains either spelling, so
  `M1.11`'s mutation-tested conformance check agreed on everything it was ever given.
- **Both were settled by changing the grammar, never by relaxing the reader** — `crates/eadl-front/src`
  is unmodified in this commit:
  - **F-A: `0X10` is a hexadecimal literal.** `hexadecimal`'s prefix is now `( "0x" | "0X" )`. The
    reader has always read it, its digits were already case-insensitive, and a case-*sensitive* prefix
    would be the one inconsistency in the literal; canonical form prints `16` either way.
  - **F-C: `\0` is not an escape.** `escape` no longer admits `"0"`. The reader never implemented it,
    a NUL byte has no use in a description, and `read-bad-escape`'s own hint already enumerated the set
    that exists.
- ⭐ **The two RED arms are the original defect restored, verified byte-for-byte.** A `python3` diff of
  each arm's mutated production against `git show HEAD:docs/semantics/grammar.md` reports the lines
  identical, so reverting either fix fails the build. The leg itself was seen firing on the real tree
  first: with `HEAD`'s grammar temporarily restored it reported exactly two violations, naming `0X10`
  as well-formed-but-rejected and `"nul\0here"` as not-well-formed-but-accepted.
- **`error` and `refused` are now distinct claims in the reference**, because the leg needed the
  difference and the language always had it. `1.2.3` is *not well-formed* and the grammar must reject
  it; `9223372036854775808` *is* well-formed and its value does not fit, so the grammar must accept it
  and the frontend refuse it. A syntax that could express "too large" would have to know the width of
  the value domain — exactly the implementation detail `grammar.md` keeps out of itself.
- **One normative document gets one parser.** The table reader moved to
  `crates/eadl-front/tests/common/reference_table.rs`, shared by `reference.rs` and `conformance.rs`;
  two parsers for one normative table would be two things that can disagree about what a rule *says*
  before either asks what it means. `Grammar::load()` became `Grammar::from_document(text)` over an
  `include_str!` constant, which is what makes the grammar side mutable by an arm at all.
- ⛔ **A defect found and fixed on the way:** `grammar.md` headed a list "**Three** rules the
  productions above imply" above *four* numbered items — a count in a normative document that no gate
  watches. The count was deleted rather than retyped, and the instance recorded as a fifth shape on
  `PROGRAM.20`, whose surface list ("book chapters, corpus indexes, crate module headers") does not
  include normative language documents.
- **Promoted:** `docs/knowledge/enumerate-the-population-from-the-specification.md` — *agreement over
  your inputs is not agreement over the specification*. This discharges the promotion `M1.12.1`
  declined and handed on, in the same commit as the mechanism it describes.
- **Validation:** `cargo test -p eadl-front --test conformance` → **10 passed / 0 failed** (was 6);
  `make focused` → `passed — 3 passed, 0 failed, 0 unavailable`; `cargo test --all` → **448 passed,
  0 failed over 37 suites** (baseline 444 over 37); `scripts/check_doctrines.sh` → `=== all doctrines
  green ===`. A census for other surfaces stating the escape set or the hex prefix returned one
  unrelated hit, so no book chapter had drifted. The one surface still quoting the old production is
  the **frozen** `LS-006` record, left untouched on purpose.

## archogen — the language reference exists, and it is executed rather than read

`ARCHOGEN-M1-0072` (leaf `M1.12.1`).

- **`docs/semantics/reference.md` is now the normative statement of what a literal is *worth*** — the
  half `docs/semantics/grammar.md` cannot carry. It states number exactness (no float anywhere, per
  §7.4), the string escape set, and what canonical form guarantees, as rules plus two machine-read
  tables of `literal → value → canonical text`.
- **`crates/eadl-front/tests/reference.rs` reads those tables out of the document and runs every row
  against the frontend.** A row the reader disagrees with fails naming the literal, the stated value
  and the produced one. Two further legs: the population for "did we forget a literal" is the **corpus**
  rather than the table, so a deleted row is caught by something the document does not define; and a
  table that stops being a table is reported instead of silently gating nothing. Eight RED arms, each
  pinning its violation count.
- ⭐ **The expected values were derived arithmetically, never printed from the reader** — and that is
  why the first run found three defects instead of confirming an implementation:
  - **`0x_10` read as 16.** A separator separates digits, it does not lead them; the grammar already
    required a leading hex digit and the reader's decimal path could never begin with `_`. The reader
    was tightened to the grammar, with its own diagnostic wording (`a separator cannot lead the
    digits`) distinct from `0x`'s.
  - **Canonical form emitted a raw carriage return.** `diagnose` on `(probe "a\rb")` piped to `od -c`
    showed the control byte inside the text §12 M4 hashes and compares; a terminal read it as "back to
    column zero" and overwrote the start of its own line. `archogen-s0`'s provenance quoting already
    escaped `\r` — the rule existed in the project and was missing from the normative printer. The leg
    that now forbids it is a property over every row, not one row's expectation.
  - **`i64::MIN` was unwritable.** `-9223372036854775808` was refused as overflow because the
    magnitude was parsed as an `i64` *before* the sign was applied. Both conversion sites now share one
    helper that parses wide and narrows once, so the wrong order cannot be written twice; one past
    either endpoint is still refused, never wrapped.
- ⚠️ **Two normative documents disagree on purpose, and say so.** The reader accepts `0X10` where the
  grammar spells the prefix `"0x"`, and the grammar admits `\0` where the reader refuses it. Neither
  spelling appears in any description the repository ships, so no existing check could reach them —
  three `git grep` censuses, three empty results. `grammar.md` carries a ⚠️ block naming both and
  stating that the reference is the authority on what a literal means; `M1.12.2` runs the same table
  against the recognizer derived from the grammar, which is what turns the note into an impossibility.
- **Validation:** `cargo test -p eadl-front --test reference` → `11 passed; 0 failed`;
  `eadl-front` lib `46` → `49`; `make focused` → `passed — 3 passed, 0 failed, 0 unavailable`;
  `cargo test --all` → **444 passed, 0 failed over 37 suites** (baseline 430 over 36);
  `scripts/check_doctrines.sh` → `=== all doctrines green ===`; `conformance.rs` and `corpus.rs`
  unmodified, so the grammar leg and the corpus-figure gate still pass on their own terms.

## archogen — QEMU pinned at 11.1.1, and what pinning does *not* unblock

`ARCHOGEN-M2-0070` (leaf `M2.8.1`).

- **`targets/riscv-virt-up.env` carries `QEMU_VERSION_PINNED=11.1.1`**, on evidence from the installed
  tool rather than from a package manager: `--version` → `QEMU emulator version 11.1.1`;
  `-machine help` → `virt  RISC-V VirtIO board`; `-cpu help` → `rv64`. §3.2's "do not rely on changing
  defaults" now has a value behind it.
- **The pin's RED arm was measured, because a pin nothing compares is prose.** Setting
  `QEMU_VERSION_PINNED=99.99.99` and re-running `--check` gives
  `PINNED RELEASE MISMATCH: pinned '99.99.99', installed 'QEMU emulator version 11.1.1'`, exit `1`;
  the correct pin was restored and re-verified afterwards.
- ⛔ **Pinning is not verification, and the tier is still red — correctly.** `--check` has four
  independent `rc=1` causes and this removed one: `NO RELEASE IS PINNED YET` is gone, and what remains
  is `TARGET_VERIFIED=no — this configuration is still a PROPOSAL`. `make integration` →
  `failed — 6 passed, 1 failed`. Flipping that flag is §3.2's agreement check, and the check currently
  has **neither side**:
  - `DEVICE_TREE_FIXTURE=docs/targets/riscv-virt-up.dtb.summary.md` is named at
    `targets/riscv-virt-up.env:45` and **does not exist** — `ls docs/targets/` returns one file;
  - no eADL description of this target exists — `git grep -ln 'riscv|virt|qemu' -- '*.eadl'` returns a
    single boundary *rejection* case, and every `defplatform` in the repository is a semantic fixture
    (`host.playground`, `soc.abstract`, `soc.concrete`, `soc.p`).
  So `M2.8` is split: **`M2.8.1`** (this pin, done) → **`M2.8.2`** (write the fixture from a measured
  dump and make re-dumping it a test) → **`M2.8.3`** (write the target's `defplatform`, the agreement
  test, and only then flip the flag). `M2.8.3` is sequenced **after `M1.12`/`M1.13`**: deciding what
  `defplatform` means for a real target and *then* freezing `eadl/1` is the cheap order, and the
  reverse is not.
- **`docs/targets/first-target.md` was rewritten to the measured state, because pinning falsified it on
  the spot.** Its fact 2 said "No QEMU release is pinned yet"; its fact 1, status-table row, section
  heading ("pinned, unverified, **uninstalled**") and "as soon as QEMU is installed" sentence were
  already false from the install. All five now state what `--check` reports, and fact 2 states the
  sharper truth: a pinned release is a fact about the *tool*, while verification is a claim about the
  *platform*. Three stale surfaces remain (`docs/book/src/verification.md`'s transcript, which is wrong
  in *shape* — `incomplete`/exit `20` where the tier reports `failed`/exit `1` — plus
  `docs/book/src/targets.md` and the Blockers sections of `M0.md`/`M4.md`); `M2.8.3` owns them.
- ⛔ **An edit dropped `QEMU_BINARY` from the `.env` and was caught before commit** by reading
  `git diff` line by line and diffing the `KEY=` set against `HEAD`. The same class of self-inflicted
  breakage as `M1.23`'s arm C and `M1.24`'s `perl` mutation: a change that did not do what its author
  intended, visible only to a tool that compared before and after rather than to a read of the intent.
- `M2`'s root node was also stale — `Status: pending` with `Children: M2.1 … M2.8` while `M2.9` exists
  and five leaves are closed. Corrected to `active` / `M2.1 … M2.9` plus sub-leaves. This is the second
  tree whose root node drifted behind its own leaves (`M1.20.7` found the same in `M1`), which is
  `PROGRAM.15`'s register-consistency gap.
- **Validation:** `scripts/target_emulator.sh --check` → exit `1` on one remaining cause;
  `make integration` → `failed — 6 passed, 1 failed, 0 unavailable`; `make focused` →
  `tier focused: passed — 3 passed, 0 failed`, exit `0`, baseline unchanged at **430 passed / 0
  failed** (no Rust touched); `make gate` → `=== all doctrines green ===` over 13 checks.

## archogen — the director ruled a push cadence, and the slot for it had never been filled

`ARCHOGEN-PROGRAM-0069` (docs). No code changed.

- **Ruled `2026-09-28`: push at `400` commits ahead of `origin/main`.** Recorded as
  `docs/decisions/decision_push-cadence.md` (+ its `INDEX.md` row), and `MEMORY.md`'s layer-A block now
  carries it. `MEMORY_ARCHITECTURE.md:193` has provided the slot since the spine was adopted —
  `(ahead of origin: <N>; push at ~<threshold>)` — and archogen never filled it in.
- "What is N in this project?" had no answer, measured rather than assumed:
  `grep -rniE 'rev-list --count|origin/main|push.threshold|ahead of origin' scripts/*.sh
  xtask/src/main.rs` → **no match**, so nothing mechanical has ever read a threshold. The doctrine is
  qualitative only ("**Push regularly** — the remote is your crash insurance; an unpushed commit dies
  with the machine"). The sole numeric threshold anywhere was the operator's *batch* rule (BWFSC,
  default 100 slices), which the PNT loop can never trigger because PNT has no fixed BWFSC by
  definition.
- ⭐ **The recommendation was `25` commits or `7` days, whichever came first, and the record says so.**
  Crash insurance scales with elapsed time, not commit count: a fixed N halves the window if the rate
  doubles and never fires at all if work stalls. At the measured ≈4.9 commits/day, `400` is roughly
  **82 days** of work on one volume — what `MEMORY_ARCHITECTURE.md:104` calls "survives nothing". The
  director chose `400`, which is a legitimate trade of durability against push overhead and review
  churn; it is recorded so a future session neither re-litigates it nor mistakes the number for a
  measurement.
- ⛔ **The check that enforces it must not be able to deadlock the repository**, and that hazard is
  recorded in `PROGRAM.23` now rather than discovered at 400: a blocking verdict at N while
  `make integration` fails leaves the tree able neither to commit nor to push, because `COMMIT.md`
  step 2 requires `make integration` before a push. `PROGRAM.23` is therefore sequenced behind
  `PROGRAM.10`, or ships as a loud non-blocking report until it has.
- `PROGRAM.10` gained the measured reason it now owns the push precondition: the emulator step's
  verdict **changed shape** when QEMU was installed — `Unavailable` → tier `incomplete` → exit `20`
  became `Failed` → tier `failed` → exit `1`, because `--check` now reaches the unpinned-config
  branch. `COMMIT.md` permits proceeding past `incomplete` after reading what it names, and does not
  permit proceeding past `failed`. §14.3's quarantine clause and the runner's existing
  `Action::NotBuilt { owner, note }` — five steps already use it under `PROGRAM.9` — supply the fix.
- Two stale `73 commits` counts written earlier today were replaced with the **live-number idiom**
  rather than retyped to `74`: the count moves with every commit, so any written copy is false by the
  next one. `MEMORY.md` already pointed at `git rev-list --count origin/main..HEAD`; `PROGRAM.10` and
  this tree's Open Questions now do too.
- **Validation:** docs-only; `make gate` → `=== all doctrines green ===` over 13 checks;
  `KNOWLEDGE_MAP.md` regenerated through its published generator (one added row). `MEMORY.md` stays at
  its 50-line budget. Baseline unchanged at **430 passed / 0 failed**.

## archogen — the director ruled: the independent emulator stays, and `M2.8` is reframed

`ARCHOGEN-PROGRAM-0066` (docs). No code changed.

- The director asked why QEMU is needed at all and whether the programme could do without it, heard
  the argument and its limits, and ruled: **keep it** — §3.2's three environments are not re-scoped.
  Recorded as `docs/decisions/decision_emulator-independence-retained.md` (+ its `INDEX.md` row) so a
  future session does not re-litigate it.
- The argument in one line: archogen is a generator, so half of its correctness is "the emitted
  artifact works on a machine", and that half is where the **facts** live — a timer's base address, a
  controller's claim/complete registers, `mtvec` alignment, the callee-saved set. Nothing in the
  repository can check them, and a wrong one is *quiet*: the hosted device model reads the same
  catalog, so both sides agree and the test passes, while every WCET derived from a tick that never
  arrives is wrong by an unknown factor inside an assurance report. `M2.2`'s independently derived
  reference is the precedent — it agreed over 16 000 events and disagreed in five places, every one a
  question the roadmap had not answered.
- The limits are recorded *with* the decision so it is not over-read: QEMU is an independent
  **implementation**, not independent **truth** (a shared misreading of the ACLINT/PLIC/16550 specs
  would agree); only ISA/ABI/startup/interrupt mechanics transfer to a board; **no timing claim rests
  on it** (§3.2 "not a cycle-accurate timing reference", §18 "not a WCET oracle"); and it is
  prospective — there is no bare-metal generated image yet, since `no-std-build` only proves
  `rt-core` *compiles* for the target and S0 emits a host crate.
- `M2.8` reframed accordingly: not "install and pin a tool" but **make the platform facts checked
  rather than asserted**. Measured while answering — `--dump-dtb` on QEMU 11.1.1 with the pinned
  options gives RAM `0x8000_0000`/128 MiB, `ns16550a` UART `0x1000_0000`, `sifive,clint0` timer
  `0x0200_0000`/64 KiB, `sifive,plic-1.0.0` interrupt controller `0x0C00_0000`/6 MiB, and
  `riscv,isa = "rv64imafdch_…"` — which **offers F and D** while archogen builds `riscv64imac`, a
  deliberate subset that must be written down so nobody later "fixes" the mismatch by enabling float
  in generated code. And `docs/targets/riscv-virt-up.dtb.summary.md`, named by `DEVICE_TREE_FIXTURE`
  at `targets/riscv-virt-up.env:45`, **does not exist** — so the §3.2 agreement check has one side
  and no other, which is the real first step of that leaf.
- **Validation:** docs-only; `make gate` → `=== all doctrines green ===` over 13 checks;
  `KNOWLEDGE_MAP.md` regenerated through its published generator (one added row, the new record). No
  code touched, so the baseline stays **430 passed / 0 failed**.

## archogen — an external document source, and a moved fact with eight uncensused copies

`ARCHOGEN-PROGRAM-0064` (docs). No code changed.

- **A read-only external document source is now recorded.**
  `docs/decisions/reference_external-document-source-chipdoc.md` (+ its `INDEX.md` row): ISA, RISC-V,
  devicetree and peripheral specifications come from the `chipdoc` corpus, mapped by `ARCHOGEN.md` at
  its root and queried through that repository's own index rather than through anyone's memory. The
  checkout location is deliberately **not** recorded here — §12 forbids checkout-specific absolute
  paths in tracked files — so it is supplied at runtime as `ARCHOGEN_CHIPDOC_ROOT`. Read-only in both
  directions; a document archogen needs and the corpus lacks is *requested*, operator-relayed, never
  fetched around it.
- **Every claim in that mapping was verified against the checkout rather than trusted**, because an
  external document's own records are data: the index tool answers queries; `risc-v/isa/pinned`
  holds a `v20260120` snapshot of 73 files across `priv`/`unpriv`/`biblio`; ACLINT, PLIC, AIA,
  TL16C550C, Devicetree v0.4, the ELF psABI and CMSIS-SVD are present; `sifive/fe310/current`
  (3 PDFs) and `sifive/hifive1/current` (2 PDFs) are present. Two requests already stand in that
  repository's ledger on archogen's behalf: `REQ-006` (FE310 / HiFive1 Rev B board documents)
  **fulfilled**, and `REQ-007` (QEMU `virt` machine documentation and its device-tree bindings)
  **requested** — the latter a dependency of `M2.8`'s §3.2 agreement check.
- ⛔ **A moved *fact*, not a moved number: eight files published "QEMU is not installed" after it
  was.** *(This entry first said "five files"; the count was corrected on `2026-09-28` — see the
  correction note at the end of this entry.)* `scripts/target_emulator.sh --check` reports
  `found: QEMU emulator version 11.1.1` and exits
  **`1`** with `NO RELEASE IS PINNED YET`, not `20`. Three were corrected here because each was
  actively routing someone to work already done: the director-facing register's §2 (marked resolved,
  its original text kept verbatim — a register that silently edits its own history cannot be
  audited), its `INDEX.md` hook, and `PROGRAM.10`'s parenthetical. The other five are recorded in
  `M2.8`'s leaf rather than fixed here, because correcting them *is* that leaf:
  `docs/targets/first-target.md`, `docs/book/src/targets.md`, `docs/book/src/verification.md`, and
  the Blockers sections of `docs/tasks/M0.md` and `docs/tasks/M4.md`. No gate compares a prose
  statement about tool availability with the tool's own verdict; whether a state fact is gateable at
  all is `PROGRAM.20`'s open question.
- ⛔ **The register's index hook was under-reporting the decisions it exists to route.** It read
  "four items for the director: no board, no QEMU, a spine gate with no seam, and a green gate that
  was blind" — the record carries **six**, and the two it omitted (§5's five runtime-contract gaps,
  §6's two behaviour-changing criticisms of the §3.1.1 amendment) are exactly the ones `MEMORY.md`
  flags as waiting on the director. It now names them.
- ⭐ `REQ-006` being fulfilled changes what `M5`'s blocker *means* without changing its status: the
  first `board-first` candidate whose §3.2 criteria can be checked against a datasheet now has one,
  so the documentation half of "a board named without its datasheet in hand is a proposal wearing a
  fact's clothes" is dischargeable. **Procurement is still the director's decision and `M5` is still
  blocked** — recorded in the decision record, not in `M5`'s blocker row, because nothing about the
  blocker changed.
- `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` step 1 gains the generalisation:
  the population a census must cover is not only numbers.
- **Validation:** docs-only, so `make focused` was re-run for form rather than necessity —
  `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not built`, exit `0`, baseline
  unchanged at **430 passed / 0 failed**; `make gate` → `=== all doctrines green ===` over 13 checks.
  `MEMORY.md` sits at its ~50-line budget (50): the new material went to the decision record, and the
  resume pointer names that layer rather than restating it.
- ⛔ **Correction, `2026-09-28` — this entry's own count was wrong, and how it was wrong is the
  finding.** It said "five files". The census behind that number enumerated **phrasings already
  seen** (`not installed|no QEMU|unavailable|install it, then re-run|as soon as QEMU is
  installed|needs an installed QEMU`), which cannot see `currently absent on this machine`
  (`docs/tasks/M4.md`) or `UNAVAILABLE — not on PATH` inside a fenced console block
  (`docs/book/src/verification.md`), and which missed `docs/tasks/M0.md`'s Blockers section entirely.
  Re-measured by **concept** instead — `git grep -il qemu` → 24 files, each classified as *live*,
  *conditional* or *record* — the population is **eight**: the three corrected here plus five recorded
  in `M2.8`'s leaf, whose table now carries all of them with the classification of every file that was
  deliberately left alone. This is the second phrasing-bounded under-count in two commits, in the
  entry that documents the first, and
  `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` step 1 now says so: enumerate
  the files that could state the fact, not the wordings it might use.

## archogen — a figure false for 49 commits, and the escape clause that excused it

`ARCHOGEN-M1-0063` (leaf `M1.24`).

- **Three more live surfaces published a corpus figure `M1.7` had superseded 49 commits earlier.**
  `docs/book/src/reading.md` said the semantic round trip runs over "all **21** corpus files" and
  `crates/eadl-front/tests/corpus.rs`'s module header said the suite "uses the **21** files", where the
  suite measures **23**; `docs/semantics/boundary/README.md` said "the **five** ambiguous cases" on line
  94 while line 9 of the *same file* said "**7** are ambiguous" and the case headers measure 7. All
  three were true at `152d188` (`M0.2`, whose commit subject is literally "the 21-case boundary
  corpus") and were superseded at `9030111` — the same commit that made `M1.23`'s two figures stale.
- ⛔ **`M1.23`'s census pattern could not see any of these, and neither could the first replacement.**
  `[0-9]+ of (the )?[0-9]+` cannot see a corpus *size*, which is not an `N of M` figure; a
  digits-plus-size-noun pattern then found 2 of the 4 and could not see `the five ambiguous cases` or
  `Three descriptions`, because both are spelled out. **Three sweeps with three patterns** were needed
  for one defect class. A census is a population bounded by its pattern, so the pattern now travels
  with the result — recorded as a decision, and as the reason `PROGRAM.20` exists rather than a fourth
  sweep.
- ⛔ **The gate built to fix this reported green on its own defect.** Its first cut excused a
  non-current figure whose line carried a past-tense marker, copying the reach gate's idiom. The
  defective line reads `…worked classification case before adoption", and the five ambiguous cases…` —
  the `before` belongs to a quotation of `ROADMAP.md` §4.3, seven words ahead of the stale number.
  Measured on the first run: it failed on `reading.md` while the index leg **passed** with `five` still
  in place. A substring anywhere in a line is not a tense, it is a coincidence; history is now an
  explicit `HISTORICAL_FIGURES` list an author has to add a line to, in review. The reach gate still
  uses the marker scan and is `M1.25`'s.
- ⛔ **`TASK-ACCEPTANCE` verifies one checklist per tree file, not the owning leaf's.** Its per-file
  `awk` enters the first box matching a keyword and exits at the next box bullet. `docs/tasks/M1.md`
  carries **24** ticked ROOT CAUSE boxes; the check reads **line 52** — leaf `M1.1`, `2026-09-13`. With
  `corpus.rs` staged and leaf `M1.24` carrying **zero** boxes, the gate printed `task-acceptance: OK
  (every staged code-change leaf carries a ticked, evidence-backed checklist)` and `exit=0`. Severity
  measured rather than assumed — **latent, not active**: every leaf recording a commit was audited,
  five carry no ticked box, and `git show --stat` on each reports **0** code files. Filed as
  `PROGRAM.21`, priority high, ahead of `PROGRAM.18`.
- **The fix is a consumer, not a correction.** `measure_corpus()` is now the corpus's one producer,
  shared by the census test — which gained the `ambiguous == 7` pin it never had — and by
  `the_live_surfaces_publish_the_measured_corpus_size`, which compares three surfaces read with
  `include_str!` so a moved or deleted one stops the crate compiling. It also discharges a name that
  overstated itself: `the_corpus_is_the_size_the_index_claims` had never read the index it is named
  after.
- **Seven permanent RED arms**, not one-off mutations: `figure_violations()` returns its violations, so
  each arm feeds it the prose that was *actually* wrong and asserts the specific complaint on every
  run. Each arm also pins the violation **count** via `assert_reported`, because an arm that only
  checks a substring passes on a gate that started reporting everything — measured by appending one
  noise violation: `6 passed; 8 failed`, all seven arms and the gate, the six unrelated tests green.
- ⛔ The mutation-applied discipline failed a **second** time by the same mechanism as `M1.23`'s arm C:
  a `perl -0pi` substitution produced `error: could not compile`, while the filtered test output
  printed **nothing at all** — which reads as a pass. Caught only by re-running unfiltered.
- **Validation:** `make focused` → `tier focused: passed — 3 passed, 0 failed, 0 unavailable, 0 not
  built`, exit `0`; `cargo test --all` → **430 passed / 0 failed** over 36 suites (baseline 422, delta
  = the gate and its seven arms); the corpus suite `6` → `14 passed; 0 failed`; `make gate` →
  `=== all doctrines green ===` over 13 checks. Both drift legs were seen firing on the **real tree**
  before any prose was edited.
- **Filed from the same census:** `S0.8` (`docs/book/src/s0.md` says three descriptions where the
  directory holds four, and four later in the same chapter — false for 38 commits), `PROGRAM.20` (the
  carried-figure register), `PROGRAM.21` (above) and `M1.25` (converge the reach gate's escape clause).
  `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` is corrected in place — steps 1,
  3 and 4 — which is the promotion `LESSON-PROMOTION` asks for. The `M0.2` commit subject, this file's
  own history and the closed leaves are deliberately untouched: rewriting a record to look current is
  the defect `docs/CLAIM_VERIFICATION.md` §5B names.

## archogen — a figure that was false for 47 commits, and the gate that makes the next one derived

`ARCHOGEN-M1-0061` (leaf `M1.23`).

- **Two live surfaces published a measurement `M1.7` had superseded.** `crates/eadl-model/src/kind.rs`'s
  module header and `docs/book/src/kinds.md` both said the schema refuses **10 of the 11** rejected
  boundary cases, that `execution-bound` is caught *only* by the classifier, and that a test "pins the
  split in both directions". At `HEAD` the reject corpus holds **13** cases, the schema reaches all 13
  (`out_of_reach.is_empty()`), and the test that pinned a split was replaced by one that pins the
  closure. Both sentences were true when written at `b53eb85` — the corpus really was 11 — and were
  superseded at `9030111`, **47 commits** earlier (`git rev-list --count 9030111..HEAD`).
- ⛔ **The book contradicted itself.** `kinds.md` said 10 of 11 while its sibling `workload.md` said
  13 of 13, both reachable from `SUMMARY.md` — and the book is the director's only window into the
  project. `M1.7` had updated the test, the corpus and `workload.md`, and never touched `kinds.md`.
  `BOOK-ANCHORS` could not see it, and says so: it proves a chapter points at something real, never that
  what it says there is true.
- ⭐ **The census was classified before anything was edited.** `grep -rnE '[0-9]+ of (the )?[0-9]+'` over
  the live surfaces returned **9** hits: 2 live and false, and **7 correct** — past-tense history, a
  "this test previously asserted" comment, two `CHANGELOG` entries, and three inside closed leaves and
  the Decisions log. Editing those would have rewritten history to look current, which is its own defect
  and the one `docs/CLAIM_VERIFICATION.md` §B names.
- **The fix is a consumer, not a correction.** §5B of that policy names "correcting a stale constant to
  a fresh constant" as an anti-pattern, so the number became derived: the corpus walk was extracted into
  `measure_reach()` giving the reach **one** implementation shared by the assertion and the gate; the
  module header now carries **no figure at all** and names the test that measures; and
  `the_live_surfaces_publish_the_measured_reach` reads all three surfaces with `include_str!` — so a
  moved file stops the crate compiling instead of silently gating nothing — and compares every figure to
  the measurement, accepting a figure only as the measurement or as marked past tense.
- ⛔ **One of the five RED arms was a false green on its first attempt.** The mutation was applied with
  `perl -0pi -e 's{(…)}{…\$1}'`, where single quotes made `\$1` literal, so the crate failed to compile
  (`error: expected item, found \`$\``) and the arm's output filter printed nothing — which reads exactly
  like a pass. The harness was rebuilt to report `mutation applied` and `compiles` before any verdict,
  and the arm re-run. `docs/knowledge/verify-the-mutation-applied.md` already prescribed this.
- New knowledge card `docs/knowledge/a-moved-measurement-needs-a-census-of-its-copies.md` + its `INDEX.md`
  row; `TOOLBOX.md` names the reach drift gate and its five arms; `M1`'s Decisions log gains the rule.
- Validation: `make focused` → exit `0`, `3 passed, 0 failed, 0 unavailable`; `cargo test --all` →
  **422 passed, 0 failed** over 36 suites against a baseline of 421, the delta being exactly the one new
  gate; `kinds` suite `15` → `16 passed`; `make gate` → `=== all doctrines green ===`.

## archogen — 1.4 GB of regenerable artifacts released, and the cleanup record started

`ARCHOGEN-PROGRAM-0060` (leaf `PROGRAM.19`).

- `docs/ARTIFACT_CLEANUP.md` did not exist, so the standing instruction's own trigger — clean if the
  record is older than 24 hours **or missing** — had been firing on every session with no way to tell.
  The file now carries the date, one entry and the mechanism; it is overwritten, never appended to.
- Released ≈1.4 GB, each item deleted only because its regeneration path is a **tracked command**: the
  previous pin's LinkedSpec build (`.app-data/target`, 1.3 GB), a digest-verified duplicate of the
  checkout's generated parser (70 MB), two empty cargo stores, the reference-check scratch, a prior
  session's notice scratch, and `target/tmp` (17 MB of test scratch carrying most of 703 stale
  incremental `.bin` files). `.app-data` 3.5 GB → 2.2 GB; `target` 815 MB → 799 MB, then 816 MB once
  the suite recreated its scratch — which is itself the proof that deleting it was safe.
- Retained **with reasons**: the current pin's build (every remaining measurement uses it), the
  132 MB package store (the vendor's guide says offline builds depend on it), the 18 MB copy of the
  *previous* pin's parser (the only "before" side of a digest frozen in `LS-004`'s evidence), the
  32 KB of primary bootstrap logs behind that same evidence, the tiers' live build products, and the
  built book.
- ⛔ **The verification did not pass first, and that is why it was run.** The first post-cleanup
  `make focused` reported `2 passed, 1 failed`; the warm re-run passed, which is how this becomes a
  "flake" in most repositories. It was reproduced deliberately instead — twice, cold — and fixed at the
  root in `S0.7`: cargo creates `CARGO_TARGET_TMPDIR` at build time, not run time, and one test wrote
  into the tmpdir root without creating it. The cleanup was correct; the test was wrong.
- ⭐ One unexpected item investigated and **left alone**: `target/sync-backup-2026-09-21`, 24 KB of
  spine-document copies dated `2026-09-21`, referenced by nothing tracked at the time. Its contents are
  recoverable from git at any revision, so it is redundant — but it is somebody's deliberate backup,
  and "unexpected state may be in-progress work" outranks tidiness. Flagged instead: a documentation
  snapshot parked inside a *build* directory is in the wrong place whatever its size.
- Validation after the deletions, measured rather than assumed: `linkedspec_eval.sh bins` → both
  binaries resolve; `reference` → the documented two-form tagged result; `LS-002 --self-test` →
  `9/9 arms`; `make focused` → exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36 suites;
  all 13 doctrines green; `git status --porcelain` shows only this leaf and the new record.

## archogen — S0 reopened and closed: a test that was green only because a previous run left a directory behind

`ARCHOGEN-S0-0059` (leaf `S0.7`).

- **One test failed on a cold run and passed on the warm one.** An artifact cleanup removed
  `target/tmp`; with the test binaries cached, `cargo test --all` then failed exactly one test —
  `a_description_with_no_system_says_there_is_nothing_to_build`, panicking at
  `crates/archogen-cli/tests/s0_build.rs:211` on `.expect("writable")` — and the next run reported
  `421 passed, 0 failed`. Reproduced deliberately, twice: `rm -rf target/tmp && cargo test --all`.
- **Root cause.** Cargo materialises `CARGO_TARGET_TMPDIR` when it *builds* a test binary, not when it
  runs one, so a cached binary plus a cleaned scratch directory leaves the path absent and `fs::write`
  fails with `ENOENT` — which `.expect("writable")` reported as a writability problem rather than an
  absence. The class was censused, not assumed: `grep -rn 'env!("CARGO_TARGET_TMPDIR")' crates/` →
  **6 code sites across 4 files**, and this was the only one writing into the tmpdir **root**;
  `s0_reader.rs` calls `create_dir_all` first and the other four pass the path to the CLI as `--out`.
- **Fix:** create the directory before writing into it, with the mechanism written down beside it so
  it is not "simplified" away. ⛔ The tempting wrong fix was to stop the cleanup from deleting
  `target/tmp`, which would have hidden the fragility behind a rule about which directories may be
  cleaned and left the suite green for the same accidental reason as before.
- Verified under the condition that failed, not under a warm tree: cold `cargo test --all` → no
  `FAILED` line, `suites=36 passed=421 failed=0`; cold `--test s0_build` → `7 passed; 0 failed`.
  F28's own gate green (`s0_oracle` 13 passed, `s0_provenance` 4 passed); `make focused` exit `0`;
  `fmt` and `clippy -D warnings` clean; all 13 doctrines green.
- The tree's status stays `done`: reopened for this defect and closed with it, all seven leaves green,
  and the reopening recorded in the frontier prose rather than left implicit.
- ⭐ The general shape, recorded in `DEV_NOTES.md`: **a test that depends on state a previous run left
  behind is not testing what it says it is**, and a single run cannot show the difference.

## archogen — the claim-verification policy adopted: the spine is five architectures, not four

`ARCHOGEN-PROGRAM-0058` (leaf `PROGRAM.16`).

- **`docs/CLAIM_VERIFICATION.md` now exists here.** The director's §17 mandates adopting the policy,
  and it was satisfied only inside a session prompt: `ls docs/CLAIM_VERIFICATION.md` → no such file,
  and `git grep -lni 'claim verification' -- '*.md'` matched only the `DEV_NOTES.md` note recording
  the gap. Same shape as `PROGRAM.11` — a rule that lives only in a prompt is enforced nowhere — and
  it fired the same day: leaf `M1.20.7` carried an unverified premise about `docs/TASK_TREE.md` and
  caught it only by running the grep.
- **Copied, not retyped, and the copy was verified rather than read.** The 285-line body came from the
  read-only source by `cat`, and `tail -n +122 docs/CLAIM_VERIFICATION.md | diff -q - <source>` →
  identical, both sides digesting to `9f99df25209c43af`. Result: 406 lines / 27 263 bytes. Nothing
  outside this repository was written to; §12's exception permits copying a policy **in** and forbids
  editing its source. A hand-copied policy would have been an unverified transcription of the document
  that defines verification.
- **Restated locally, as the policy's own §7.6 demands** — a rule you cannot restate in your project's
  terms is under-specified for you. §A maps the five portable architectures onto this repository and
  gives each leg archogen's own measured instances: the vacuous `xargs sha` digest that would have made
  "regenerated" indistinguishable from "UNCHANGED"; the classifier that reported a symptom over a clean
  run; the leaf premise a grep disproved; the granularity rule evidenced by `cargo metadata`'s member
  list rather than "the build passed".
- ⭐ **§7's adoption sweep was run, not skipped, and step 4 found a real gap.** Eighteen registered
  controls in `scripts/check_*.sh`; **8** carry `--self-test` RED arms and all 8 pass; **10 do not** —
  including `check_task_acceptance.sh`, the most load-bearing gate here, whose box-scoping was
  validated once during development and is re-fired by nothing. Filed as `PROGRAM.18`, medium-high,
  that gate first. Publishing the adoption while burying that result would have been the exact failure
  the policy describes.
- Registered in **all three** entrypoints: `CLAUDE.md` (spine sentence + reading order step 5),
  `AGENTS.md`, and `DOCTRINE_ENFORCEMENT.md`'s E1 discovery list with the sibling question stated.
  ⛔ `AGENTS.md` was nearly missed: the leaf's first draft asserted it "names the discipline documents
  generically", and reading it showed an **explicit** list at line 11 — so a fifth spine document
  absent from it would be invisible to every harness that reads `AGENTS.md` instead of `CLAUDE.md`.
  The claim was written from the file as remembered, not as it is; leg 1 applies to a leaf's own prose.
- §5A's claim tag is adopted **by mapping**: this repository's tag is the leaf acceptance box, already
  gated by `TASK-ACCEPTANCE`, so no second syntax was added. The policy is deliberately **not**
  registered as a doctrine — its mechanizations are a separate decision, and adding a gate nothing
  needs yet is how a registry accumulates checks nobody can explain.
- Validation: `make focused` → exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36 suites;
  all 13 doctrines green, `DOCPATH` and `TABLE-ARITY-RATCHET` included on the new file.

## archogen — the LinkedSpec evaluation is closed: register reconciled by census

`ARCHOGEN-LINKEDSPEC-0057` (leaf `M1.20.7`, closing `M1.20`).

- **`M1.20` is closed.** Seven sub-leaves, six measurement commits, one census. All five reported
  defects — `LS-001` … `LS-005`, both blockers among them — were re-measured by archogen at LinkedSpec
  `2ac834913` and are `verified`; nothing in the register rests on the vendor's word any more.
- **Reconciled by census, not by trust.** Each of the seven issue sub-trees' own `**State**` field was
  read and compared with its register row, each `verified` row checked for a frozen
  `evidence/REMEASURED.txt` **and** a dated archogen rerun line, and the totals recounted from the
  rows: **7 of 7 match, 0 mismatches**; `verified 5`, `withdrawn 1`, `no-action 1`, everything else
  `0`. No issue sub-tree was edited by the reconciliation —
  `git status --porcelain docs/feedback/linkedspec/issues` → empty.
- ⛔ **Two real drifts found and fixed.** This tree's root node still read `Status: pending` and
  `Children: M1.1 … M1.8` while the index and `LIVE_STATUS.md` both called it `active` with 22 leaves.
  And `M1.20.7`'s own goal claimed `docs/TASK_TREE.md` still named the superseded pin `fd3e328d5` —
  `grep` disproved it, because the split commit had already corrected that row; the claim was fixed
  before anything was done about it. A leaf that acts on its own unverified premise is how drift gets
  "fixed" into a new shape.
- ⭐ **A gap filed rather than noted.** `FEEDBACK-SELF-CONTAINED` checks that every issue directory is
  *named* in the register; nothing anywhere compares a row's State with its sub-tree's, or recounts
  the totals. Five state transitions in six commits stayed consistent by hand-editing plus this
  census, and a commit that got one wrong would have passed every gate. Owned by `PROGRAM.15`.
- The register's "Upstream response" paragraph keeps its original wording — it is a transcript of what
  the notice said — and gains a dated **superseded** marker, the same treatment the five issue pages
  use. ⛔ Nothing was rewritten to look tidy.
- Validation: `make focused` → exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36 suites;
  no Rust or manifest path staged; all 13 doctrines green, `FEEDBACK-SELF-CONTAINED` included.

## archogen — LS-003 verified: the register is closed, every report re-measured

`ARCHOGEN-LINKEDSPEC-0056` (leaf `M1.20.6`).

- **LS-003 is `verified`, and with it the whole register.** All five reported defects were re-run by
  archogen at LinkedSpec `2ac834913` on `2026-09-27`; nothing rests on the vendor's word any more.
  Totals: 5 `verified`, 1 `withdrawn`, 1 `no-action`, 0 `open`, 0 `fixed-upstream`.
- The three frozen probes, flattened to `kind:lexeme` atoms in document order and compared with
  expectations written into the instrument **before** the run: `04` → `symbol:name`,
  `string:"ARCHOGEN"` with **both quotes inside the lexeme**; `05` → `symbol:name`,
  `symbol:ARCHOGEN`; `03` → `… symbol:period number:10 symbol:ms …`. The fourth check is the
  cross-probe one that *is* this defect — probes 04 and 05 must not collapse — and they no longer do.
  `checks 4 · as expected 4 · defect 0 · undecided 0`, `rc=0`; `--self-test` → `9/9 arms`.
- The scorer refuses rather than guesses in three directions, each with its own arm: an **untagged**
  result (`2`, since scoring the wrong adapter's output either way would be a lie), unreadable JSON
  (`2`), and a valid one-form input coming back rejected (`2`, because over-rejection is a different
  defect and must not flip this verdict).
- ⛔ **The verdict is scoped, and the scope is written down.** These are the document grammar's
  lexical kinds, not eADL's: its published rule makes `0x4_0000` a number and `1__0` and `1.` symbols,
  while eADL forbids floating point anywhere and demands canonical spelling. The issue page, the
  evidence and the leaf all say that adopting this route inherits a lexical contract, not eADL's —
  which is exactly what `M1.22` will measure instead of assuming.
- The historical route is unchanged and now **guarded**: `repro.sh` → `rc=0`, matches
  `EXPECTED.txt` line for line, probes 04 and 05 still both `["name","ARCHOGEN"]`. A silent change
  there would be a regression for every consumer who has not migrated.
- Validation: `make focused` → exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36
  suites; the frozen `EXPECTED.txt`, `repro.sh` and all three probe inputs untouched;
  `FEEDBACK-SELF-CONTAINED` green on the new files; all 13 doctrines green.
- Promoted **into an existing entry** rather than adding a fifth near-duplicate:
  `docs/knowledge/a-verified-row-must-name-what-you-still-owe.md` gains "name whose contract you
  verified" plus a matching `answers:` question.

## archogen — LS-002 verified: the last reported blocker, on the new document route

`ARCHOGEN-LINKEDSPEC-0055` (leaf `M1.20.5`).

- **LS-002 is `verified`, and no reported blocker is outstanding.** All eight frozen probes were run
  at LinkedSpec `2ac834913` through `sexpr_file` with `specs/SExprDocumentV1.spec` (entry rule
  `Document`), each against an expectation written into the instrument **before** the run:
  `probes 8 · as expected 8 · defect 0 · undecided 0`, `rc=0`.
- **The decisive probe.** `system.eadl`, the real four-form S0 description whose truncation decided
  the original report, now returns all four top-level forms — heads `defblock`, `defplatform`,
  `defservice`, `defsystem` — re-parsed independently of the instrument from the frozen JSON. The
  historical route returned the first and silently discarded three, on a success exit.
- Both of the report's asks are met: a complete-input mode (leading, intervening and trailing junk
  reject the whole document with a typed error and **no partial value**) and a multi-form result
  (every top-level form, in order).
- ⭐ **The remedy is a new route, not a change to the old one**, so the verdict names the route and
  the old path stays under test as a guard: `repro.sh` on `lispish_file` + `Lispish.spec` → `rc=0`,
  `observation matches evidence/EXPECTED.txt`, all eight lines identical. That `0` means "nothing
  drifted for consumers who have not migrated", not "still broken".
- ⛔ Adoption is an action, and the shortcut does not perform it: the new adapter pointed at the old
  grammar exits `1` with `entry_rule_not_found`, and the old adapter pointed at the new grammar does
  not adopt the new result contract.
- New instrument `remeasure.sh`, `--self-test` → `9/9 arms`, with a pure evaluator at its centre:
  one form where two are expected must be caught, accepting trailing junk must be caught, rejecting
  while still printing a value must be caught, an unreadable result must be **refused** rather than
  read as zero forms, and a document expected to be accepted coming back rejected scores "could not
  decide" — over-rejection is a different defect and must not flip this verdict.
- ⭐ A real bug found by reading output rather than exit codes: the guard was invoked with the
  caller's relative paths from a subshell that had changed directory, so it printed
  `--bin <lispish_file> is required` and did nothing. Paths are resolved before use.
- ⚠️ **This reopens a closed question.** `M1.14` ruled the shipped grammar unfit as an independent
  cross-check on the single ground that no complete-input mode existed. That mode now exists and is
  measured, so leaf `M1.22` is logged: settle by differential harness whether `SExprDocumentV1.spec`
  can be the third opinion on archogen's normative surface syntax — as a development-time tool, with
  no dependency added to the engine workspace.
- Validation: `make focused` → exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36
  suites; the frozen `EXPECTED.txt`, `repro.sh` and all eight probe inputs untouched;
  `FEEDBACK-SELF-CONTAINED` green on the new files; all 13 doctrines green.
- Promoted: `docs/knowledge/a-fix-that-adds-a-route-does-not-retire-the-old-one.md`.

## archogen — both native consumers built at the pin, behind one launcher

`ARCHOGEN-LINKEDSPEC-0054` (leaf `M1.20.4`).

- New tool `scripts/linkedspec_eval.sh` (`env` · `prepare` · `build [--network]` · `bins` ·
  `reference` · `run`) derives the five storage values the vendor's guide requires from
  `git rev-parse --show-toplevel`, so no session re-exports them by hand and nothing lands
  off-volume. Named in `TOOLBOX.md` with its question and its exit contract.
- **The target directory is named after the pin** — `.app-data/target-2ac834913` — so a future pin
  move cannot silently reuse this build. The `ad290bdb4`-era `.app-data/target` (1.3 GB) is preserved
  beside it; the new build is 2.1 GB.
- `build` → `Finished dev profile … in 1m 02s`, `rc=0`, `--offline --locked`, producing
  `sexpr_file` and `lispish_file`. Nothing was written into the vendored checkout:
  `vendor/linkedspec/rust/target` is still dated `2026-09-20`, `examples/integration/rust/target`
  does not exist, and `git submodule status` is unmoved.
- `reference` → `rc=0`: the guide's own input `(v 1 "1")(done)` returns
  `{"format":"linkedspec-sexpr-v1","forms":[…]}` with two top-level forms, kinds `symbol` / `number` /
  `string`, and the quotes inside the string lexeme. An empty file returns `{"forms":[]}`.
- ⭐ The check can fail, proved without mutating anything: pointing the document consumer at
  `Lispish.spec` exits `1` with the typed diagnostic `entry_rule_not_found` — independently
  reproducing the guide's claim that Lispish has no `Document` entry.
- ⛔ **A documented route was declined.** The guide's consumer route copies `sexpr_file.rs` into the
  application and adds `serde_json` plus a path dependency on the vendored runtime to its
  `Cargo.toml`. That would put a serialization crate and a vendored dependency into the engine
  workspace, against the zero-dependency decision and §4.4. The example is built **in place**
  instead — the route the guide's own check section uses — with identical binaries and no change
  here: `cargo metadata` still reports 9 members and 0 vendored packages.
- ⭐ The same reference input already separates the two routes: `lispish_file` returns
  `["v","1","1"]` at `rc=0` — the second form and the string's quotes gone. LS-002 and LS-003, on the
  vendor's own example.
- Validation: `make focused` → exit `0`; `cargo test --all` → **421 passed, 0 failed** over 36
  suites; `bash -n` clean; all 13 doctrines green. Promotion of this leaf's lesson declined with a
  recorded reason: the durable halves are already in `docs/knowledge/`, and the route choice is
  written in the launcher's header.

## archogen — LS-004 verified through RGX's published interface, and a stale parser caught

`ARCHOGEN-LINKEDSPEC-0053` (leaf `M1.20.3`).

- **LS-004 is `verified`** on four arms at LinkedSpec `2ac834913` / RGX `f6e5acdc9` / PGEN
  `d9d41c28`, driven only through RGX's **published** `make -C <checkout>/rgx bootstrap` and the
  vendor's documented storage wrapper. Two independent empty-store offline controls exit `2` at the
  first missing prerequisite — one `error:` line, no later named step, no seed claim, `generated/`
  empty; a fresh preparation exits `0` with 12 files and `✅ Bootstrap complete.`; prepared reuse
  exits `0` printing `PGEN parser already generated — nothing to bootstrap.` Both reported symptoms
  are absent, so `🌱 generated/ebnf.rs seeded.` over an empty directory cannot occur.
- ⛔ **The checkout arrived carrying a parser from the previous pin**, and this is the finding that
  mattered most: `generated/` was dated `2026-09-20` and built at PGEN `db6f8c68`, while the adopted
  pin carries `d9d41c28`. RGX's published contract warns that `make bootstrap` is idempotent **on
  existence**, so a plain rerun exits `0` and keeps the old parser — every later measurement would
  have described the wrong revision while looking green. Regenerated per the contract: digest
  `50eec63c9ba79b16` → `196db2eefed767ff`.
- ⭐ The **reuse** arm is what makes staleness visible at all: it exits `0`, says "already
  generated", and leaves the digest unchanged — from the outside identical to a stale first run.
- ⛔ The instrument was wrong first and an arm caught it: the classifier counted any seeding message
  as a symptom, and printed `symptom 1 — PRESENT` over a clean, successful preparation. A seed claim
  indicts a run only if the run failed; `self-test` now has **9 arms**, including the historical log
  from `evidence/OBSERVED.txt`, which must still come back `STILL PRESENT`.
- The historical `repro.sh` was deliberately **not** re-run: it provokes through LS-001's collision
  (fixed) and calls a PGEN-internal make target, which the guide now tells consumers not to do. Its
  frozen observation stands as the record of the original defect.
- ⚠️ **New finding, filed not folded in:** one successful preparation wrote a **753 MB** log of
  4 008 986 lines, 1 151 376 of them `[PGEN][DBG]` progress lines, while the guide instructs a
  consumer to preserve the full log when the command fails. Owned by leaf `M1.21` as its own
  register row; recorded inside LS-004's re-measurement.
- Destructive arms made safe: the controls remove `generated/`, so they ran last and the fresh parser
  was restored from a hash-verified copy (`196db2eefed767ff` before and after, 12 files). The stale
  parser is kept under the application's data root for comparison; the 753 MB log was measured and
  released. No tracked content of either submodule was modified.
- Promoted: `docs/knowledge/prove-the-artifact-was-regenerated-not-just-present.md`.

## archogen — LS-001 verified at the adopted pin, and the consumer's half of the remedy adopted

`ARCHOGEN-LINKEDSPEC-0052` (leaf `M1.20.2`).

- **One of the two reported blockers is now `verified`** on archogen's own rerun at LinkedSpec
  `2ac834913`. The remedy turned out to be **split**: the integration example — the manifest a
  consumer builds — now carries its own `[workspace]` boundary and resolves on its own, while the
  nested PGEN manifest, a transitive dependency in another project's tree, carries none and still
  collided against this repository's workspace root (`rc=101`,
  `current package believes it's in a workspace when it's not`).
- **archogen's half:** `exclude = ["vendor/linkedspec"]` in the root `Cargo.toml`, with the reason
  beside it. The vendor's guide states it as a required step before any cargo metadata or build
  command, and it is what makes the nested manifest resolvable — a standing requirement of this
  consumer, not a one-off fix. Measured before and after adopting it: `remeasure.sh` → `rc=1`
  (`STILL PRESENT`, 1 collision of 2 probes) then `rc=0` (`GONE`, "carried by BOTH halves").
- New instrument `remeasure.sh`, `--self-test` → `5/5 arms`: the reported shape must collide, the
  app-root exclusion must resolve it, a vendored manifest with its own boundary must resolve it, a
  manifest failing for an unrelated reason must be **refused** rather than counted as this defect,
  and an application root with no workspace must be declined as not applicable. Every probe is
  `cargo metadata --no-deps --offline`: no build, no network, nothing written into the checkout.
- ⛔ The historical reproducer's Part 2 was deliberately **not** re-run — it patches the vendored
  manifests, which the guide at the pin forbids as a remedy. Reading the page against the script
  also exposed a defect in archogen's own tracker: it documented an exit `3` the script never
  returns. Page corrected; the frozen script and its frozen output were left untouched.
- Validation: the consuming workspace is unchanged by the exclusion — `cargo metadata` → `9`
  members, the same nine package names, `0` vendored packages among them; `make focused` → exit `0`;
  `cargo test --all` → **421 passed, 0 failed** over 36 suites; `FEEDBACK-SELF-CONTAINED` green on
  the new files; all 13 doctrines green.
- Promoted: `docs/knowledge/a-verified-row-must-name-what-you-still-owe.md`.

## archogen — LS-005 verified at the adopted pin, on archogen's own rerun

`ARCHOGEN-LINKEDSPEC-0051` (leaf `M1.20.1`).

- **The register's first `verified` row.** LS-005 ("add and pin" commands alone never produce a
  buildable tree) was re-measured at LinkedSpec `2ac834913d85c32f532be9b0aab63644838a577a` and the
  reported defect is **gone**: the guide's "Add and pin the source dependency" section now states
  `**Before running Cargo metadata or building:** complete` workspace setup, application-local
  storage and RGX preparation (line 33), points at `[RGX preparation](#initial-rgx-preparation)`
  and calls it "a required part of this setup sequence" (lines 36–37), and the target exists
  (`### Initial RGX preparation`, line 137). State `fixed-upstream` → `verified`; the register now
  reads 4 `fixed-upstream`, 1 `verified`, 1 `withdrawn`, 1 `no-action`.
- ⛔ **The frozen reproducer could not decide, and that is the finding.** `repro.sh` at the same pin
  exits `3` with `expected section headings not found — the guide has been restructured`, because it
  keys on `### Initial PGEN preparation` and the fix renamed that heading. Exit `3` means *changed*;
  a rename is a change and so is a remedy, so the row could not move on that output.
- ⭐ New instrument `remeasure.sh`, checking the **property** rather than the spellings, with
  `--self-test` → `4/4 arms passed`: the section **verbatim as published at `ad290bdb4`** must come
  back `STILL PRESENT`, the remedied shape `GONE`, a restructured guide must be refused rather than
  guessed, and a pointer whose target does not exist must not count as a fix.
- ⛔ The trap it had to avoid: the *unfixed* guide already carried
  `Checkout does not generate PGEN's parser inputs.` — the very sentence the report called
  insufficient, because it is a trailing remark and not a blocking step. A keyword check would have
  reported the unfixed revision as fixed.
- The two instruments have **opposite** exit-code polarity on purpose, and both contracts are stated
  in the issue's `README.md`, its `SETUP.md` and the tracker `README.md`, which now carries the rule
  for every future re-measured row.
- Validation: `remeasure.sh` → `rc=0`; `repro.sh` → `rc=3`; `evidence/OBSERVED.txt` and `repro.sh`
  untouched (`git status --porcelain` on both → empty); `FEEDBACK-SELF-CONTAINED` self-test `6/6`
  red arms and the doctrine green on the new files; `bash -n` clean; `make focused` → exit `0`,
  `cargo test --all` → **421 passed, 0 failed**, unchanged from `M1.19.1`.
- Promoted: `docs/knowledge/frozen-reproducers-measure-change-not-repair.md`.

## archogen — pin LinkedSpec's latest published head

`ARCHOGEN-LINKEDSPEC-0048` (leaf `M1.19.1`).

- Move `vendor/linkedspec` from the notice's named publication `fd3e328d5` to LinkedSpec's **latest
  published head** `2ac834913d85c32f532be9b0aab63644838a577a`, on the director's ruling that the
  latest pushed work is what should be pinned unless a report proves unfixed or a new misbehaviour
  appears. `git submodule status` now equals `origin/main`.
- ⭐ **Every remedy commit is an ancestor of it — measured, not assumed.** `git merge-base
  --is-ancestor <c> HEAD` returns YES for all eight: `8259719f8`, `effe3e7b2`, `01b04138a`,
  `a8d34c845`, `4e2598d1e`, `fd3a2444e`, `fd3e328d5`, `8b5b5ffd8`. All five fixes are in, plus
  LinkedSpec's own boundary correction.
- ⛔ **`M1.19`'s reasoning was half right, which is why it was wrong**, and is marked superseded
  rather than deleted: attributability does require a *named* revision, but nothing required the
  *older* one. Skipping `8b5b5ffd8` had excluded the remedy for the one misbehaviour that actually
  fired — the conservative pin was the less safe one.
- ⭐ The delta was measured before moving: `git diff --name-only fd3e328d5..2ac834913` → 25 paths,
  nine root documents and sixteen under `docs/`, and filtering for `\.spec$|specs/|\.rs$|Cargo` →
  **NONE**. The Rust backend and the specifications the five reports concern are identical at both
  revisions, so the evidence the notice cites at `fd3e328d5` still describes what archogen pins.
- ⚠️ One consequence for `M1.20`: the pinned Rust integration guide
  (`docs/linkedspec-book/src/public-api/integration-rust.md`) *is* among the changed documents, so
  the re-measurement follows the guide **at `2ac834913`**. A docs-only delta is not a no-op delta
  when the docs are the integration contract.
- Validation: nested `rgx` still `f6e5acdc99720349d1e3ecef9f821f365c4db19c`, matching
  `git ls-tree HEAD rgx` — identical at both revisions, so no re-sync was needed; vendored worktree
  clean; `make focused` → `tier focused: passed — 3 passed, 0 failed`, exit `0`;
  `cargo test --all` → **421 passed, 0 failed** over 36 suites. Still **adoption, not acceptance**:
  no reproducer re-run, no issue state changed, `verified` still zero.
- Promoted: `docs/knowledge/pin-the-vendor-head-and-measure-the-delta.md`.

## archogen — correct the direction of the LinkedSpec boundary crossing

`ARCHOGEN-PROGRAM-0047` (leaf `PROGRAM.11`).

- ⛔ **The previous entry recorded the incident backwards, and this corrects it.** Upstream
  `8b5b5ffd8` discloses "the unauthorized ARCHOGEN documentation commit and auxiliary writes"; read
  from inside archogen, with the boundary rule present only in a session prompt, that was taken as
  *archogen writing into LinkedSpec*. The director corrected it on `2026-09-27`: **LinkedSpec's
  agent modified a few `.md` files in this repository** to deliver its fix notice. LinkedSpec has
  since made other repositories read-only in its own bootstrap; a one-time error, not expected to
  recur, and nothing here broke.
- Audited with tools before correcting anything: every commit carries the single local identity (no
  foreign-authored commit); `git reflog` is linear — no `reset`, `rebase` or `amend`, so nothing was
  created and discarded; the inbound content entered through `82ee99a` (leaf `M1.18`) confined to
  `docs/feedback/linkedspec/**` plus archogen's own live docs, with **no** `crates/`, `scripts/`,
  `xtask/`, `Cargo.*` or `Makefile` path touched.
- Validation: `make focused` → exit `0` (fmt, clippy, tests); `cargo test --all` → **421 passed,
  0 failed** over 36 suites — the same count as at `M1.11`, which is the expected result now
  measured: no crate depends on the vendored checkout, so the pin move cannot reach the suite.
- ⭐ The inbound write was **handled correctly**, and the mechanism is worth keeping: `M1.18`
  recorded the authorization, preserved the 36 original non-state feedback files by SHA-256, and
  refused to let the vendor's notice set `verified`. An external agent's claim about its own fix
  entered the tree as an attributed claim, not as a result.
- `PROGRAM.11` is re-scoped to cover the boundary **in both directions**, and its priority drops
  from high to medium — the high rating rested on the false premise that the outbound defect class
  had fired. The outbound half is preventive; the inbound half fired once and was handled.
- Records were corrected **in place where they must read as current truth** (the decision record,
  the promoted lesson, the leaf) and **appended where they are history** (this file, `DEV_NOTES.md`).
  No history was rewritten.
- ⭐ What survives the correction, unchanged and still the reason the leaf exists:
  `grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md` → **no match**. The rule was absent from the committed
  tree, so its direction was undecidable from inside it — and the measured damage was a wrong
  durable record in layer C, not a stray write.

## archogen — adopt the published LinkedSpec pin

`ARCHOGEN-LINKEDSPEC-0046` (leaf `M1.19`).

- Move `vendor/linkedspec` from `ad290bdb4` — the revision the seven reports were **measured** at —
  to `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`, the publication that carries the LS-004 remedy,
  with the nested `rgx` submodule at `f6e5acdc99720349d1e3ecef9f821f365c4db19c`, the revision that
  publication pins. Director-instructed, and the step `M1.18` left pending: "awaiting ARCHOGEN
  adoption and verification".
- ⛔ **Adoption is not acceptance.** No reproducer was re-run, so **no issue state changes**:
  LS-001 … LS-005 remain `fixed-upstream`, LS-006 `withdrawn`, LS-007 `no-action`, and `verified`
  is still zero. Leaf `M1.20` owns the re-measurement and is the only thing that may move a row.
- ⭐ Pinned the **named publication**, not `origin/main`, which the fetch showed had already
  advanced two commits (`8b5b5ffd8`, `2ac834913`). Every piece of cited upstream evidence resolves
  at `fd3e328d5`; measuring an unnamed revision would make a later `verified` unattributable.
- ⛔ **A repository-boundary violation surfaced in the same fetch, and it is owned rather than
  noted.** Upstream `8b5b5ffd8` ("record publication and repository-boundary violation") discloses
  "the unauthorized ARCHOGEN documentation commit and auxiliary writes". This repository's own
  record is clean — two commits ever touched `vendor/linkedspec`, neither wrote inside it — and
  `grep -rn 'READ-ONLY' CLAUDE.md AGENTS.md` → no match. **The rule existed only in the director's
  session prompt**, so it was unavailable to any agent resuming from git alone. Recorded in
  `docs/decisions/decision_repository-boundary-read-only.md`, promoted to
  `docs/knowledge/a-rule-only-in-the-prompt-is-enforced-nowhere.md`, and owned by `PROGRAM.11`
  (state it in the bootstrap; gate the observable symptom — a vendored checkout with local commits
  or local modifications). The leaf states its own limit: no gate here can stop a write into a
  checkout elsewhere on the filesystem.
- Validation: `git submodule status` → `+fd3e328d5dd5c80981a1c3b8496a27270291f7b8
  vendor/linkedspec`; nested `rgx` → ` f6e5acdc99720349d1e3ecef9f821f365c4db19c`, matching
  `git ls-tree HEAD rgx`; vendored worktree clean; `git merge-base --is-ancestor ad290bdb4…
  fd3e328d5…` → yes. No archogen crate depends on the vendored checkout, and no `Cargo.toml`,
  `Cargo.lock` or `crates/` path is touched, so no application test is claimed or needed for the
  pin move itself. Submodule internals were not inspected, reconstructed or modified (§20).

## archogen — receive LinkedSpec's published fixes

`ARCHOGEN-LINKEDSPEC-LS004` (leaf `M1.18`).

- Receive the director-authorized completion notice for published LinkedSpec `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`.
  LS-004 adopts verified RGX f6e5acdc; each of the five addressed reports names its remedy.
- Synchronize five fixed-upstream states, zero open states, one withdrawn and one no-action.
  ARCHOGEN verification remains pending, including the two originally reported blockers.
- Preserve all original reproducers, setup, fixtures, measured revisions, application code and
  vendor pins. M1.12 and the M2.9 checkpoint/director rulings remain unchanged.
- Validation: feedback closure passes; issue/index census agrees on all seven states; all36
  original non-state feedback files remain SHA-256 identical; four prior fix commits are ancestors
  of the publication; MEMORY43lines and diff checks pass. Required doctrine hooks govern landing.
  LIVE_STATUS milestone states are unchanged; no application-test rerun is claimed.

## archogen — the eADL surface syntax becomes normative

`ARCHOGEN-M1-0036` (leaf `M1.11`).

- ⛔ **Before this, eADL's format had no definition other than its implementation.** A 743-line
  hand-written reader was the sole authority on what the language *is*, and every test in the
  repository validated against it — so no input could show it wrong, because whatever it did was
  correct by definition. §4.1 assigns the project ownership of *syntax*; §12 M1 anticipates the
  syntax freezing at a compatibility baseline; §4.4 requires an independently derived checker
  whose parser is a declared trust dependency. None is possible against an implementation.
- `docs/semantics/grammar.md` now states the lexical and phrase structure as productions, and it
  is **executed**: `crates/eadl-front/tests/conformance.rs` reads the EBNF block out of that
  document, builds a recognizer from it, and requires the recognizer and the reader to agree
  across all **62** corpus descriptions, six malformed fixtures, and **23 probes** covering every
  production. The document is the source; there are not two definitions to drift apart.
- ⭐ **Acceptance agreement turned out to be too weak a claim, and that was measured.** Dropping
  `_` from hexadecimal literals in the reader left every acceptance test green while
  `(base 0x1000_0000)` silently became the two forms `4096` and `_0000` — a memory-mapped base
  address of `0x10000000` read as `4096`. Conformance now compares **token segmentation**, and the
  same mutation fails naming byte offsets: `grammar sees "0x1000_0000" at 953..964, reader sees
  "0x1000" at 953..959`.
- ⛔ Two defects in the mechanism itself, both found by red-arming it. Splitting productions on
  `;` cut the `comment` rule in half, because `;` is a literal *in* the language being described.
  And the recognizer originally returned one end position per expression, so it could not
  backtrack out of an alternative — on `10ms` it matched `number`, failed the delimiter lookahead,
  and never tried `symbol`. It gave the right verdict for the wrong reason, and the red arm that
  should have caught it **did not fire**. Rewritten continuation-passing.
- ⭐ **The corpus is not a conformance suite**, which is why the probes exist: it contains exactly
  one number with a digit separator and that one is hexadecimal, so nothing in it reached decimal
  separators, signs, escapes or CRLF.
- ⚠️ What this establishes: the grammar and the reader accept the same language and agree on every
  token boundary. What it does not: that the reader builds the right *tree*, or assigns the right
  *value* — a reader that read `1.5` as three halves would still pass. That is leaf `M1.12`.
- `reader.rs` is unchanged. The grammar documents the language the reader already implemented,
  which is itself a result worth stating.
- Validation: `cargo test -p eadl-front --test conformance` → **6 passed, 0 failed**;
  `cargo test --all` → **421** passed (415 before, `+6`); fmt and clippy clean; all doctrines
  green.

## archogen — checked against a model that never saw it, and the disagreements are the finding

`ARCHOGEN-M2-0035` (leaf `M2.2`).

- `crates/rt-reference` is an **independently derived** model of the same runtime semantics,
  written by a separate context instructed not to read `crates/rt-core`, the runtime book chapter
  or the task tree — working from `ROADMAP.md` §3.1/§8/§8.1 and the priority decision record. Its
  API came out visibly different (explicit `Priority` newtypes, a separate `Processor` state,
  `Result`-returning operations, eager dispatch inside `release`), which is evidence rather than
  friction. ⛔ The dependency points implementation → reference and never the reverse.
- `crates/rt-core/tests/differential.rs` drives both through **400 randomised sequences of 40
  events** and compares what a user of either could observe — with **coverage floors**, so the
  agreement cannot be vacuous: the sequences must actually reach preemption, latched delivery,
  completion and idle. In the region the contract decides, the two agree exactly.
- ⭐ **The agreement is the weak result.** Two models that disagree cannot have been copied from
  each other, so each divergence is simultaneously proof of independence and a real defect. There
  are **five**, and every one is a question `ROADMAP.md` does not answer — where an overrun's
  fault attaches, whether an empty task set is admissible, what priority rank `0` means, what a
  containable fault inside a masked region does, and what bounds mask nesting. A single author had
  resolved all five silently, and the resolutions looked like the specification.
- ⛔ Among them, a concrete one worth naming: **the runtime's highest priority is index `0` while
  the language's is `(priority 1)`**, and that off-by-one is written down nowhere.
- The five are asserted on **both** sides as ratchet tests, so neither model can drift and the
  list cannot shrink by "fixing" one side — §14.1 forbids silently weakening a requirement. Four
  of them change the roadmap and are the director's call; they are set out with recommendations in
  `docs/decisions/decision_runtime-contract-gaps.md` and owned by the new leaf `M2.9`.
- ⚠️ What remains **shared** is disclosed, because §4.4 requires it and the leaf's acceptance
  demands it: the contract text, the priority decision record, the adapter, and the fact that the
  same model family produced both — §14: "a second model agreeing with the first is not ground
  truth". Including one incidental leak the reference's author disclosed unprompted.
- Two intermediate states are recorded because they show the harness was measured rather than
  assumed: the first run reported `400 of 400` sequences diverging (the undecided region was
  inside the generator), and after the first filter `373 of 400` — a **latched** release also owes
  a job, which the filter missed.
- Validation: `cargo test -p rt-reference` → **34 passed**; `--test differential` → **7 passed**;
  `cargo test --all` → **415** passed (374 before, `+41`); fmt and clippy clean; all doctrines
  green; `mdbook build` OK.

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
