- ID: `PROGRAM.47`
  Status: `done` — `2026-10-02`, filed that day on the director's ruling
  Children: `PROGRAM.47.1` … `PROGRAM.47.6`
  Goal: the book written in layers, as [`decision_book-in-layers.md`](../decisions/decision_book-in-layers.md)
  rules: plain words first for a student, a one-minute summary and the precise rules for an expert, annexes for the
  gory details, a live glossary and a generated index.
  Why: the director, `2026-10-02`: "A student or newbie shall be able to read the book without being scared away. An
  expert SW engineer or embedded SW engineer shall not be bored"; and "be sure to include and keep a live glossary for
  acronyms … have annexes … keep in index at the end of the book".
  Acceptance: every chapter in layers; the glossary and the index in the book, each held by its gate; the annexes
  holding what the chapters point to; the book builds.
  Verification: closed by its children, `.1` to `.5`; the glossary and the index held by `BOOK-GLOSSARY` and
  `BOOK-INDEX` on every commit from here on
  Commit: the children's, `ARCHOGEN-PROGRAM-0296` to `-0327`

- ID: `PROGRAM.47.1`
  Status: `done` — `2026-10-02`
  Goal: the ruling recorded before any chapter changes.
  **Done.** [`decision_book-in-layers.md`](../decisions/decision_book-in-layers.md) and its index row; the Knowledge
  Map regenerated.
  Verification: `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0296 (leaf PROGRAM.47.1)`

- ID: `PROGRAM.47.2`
  Status: `done` — `2026-10-02`
  Goal: the glossary, *Words this book uses*: every acronym the book uses, spelled out and explained, and the
  recurring terms in plain words, each linked to where it is treated; `BOOK-GLOSSARY`, refusing an acronym in the
  book that the glossary does not define, and an entry no chapter uses.
  Acceptance: the gate's RED arms; the chapter in `SUMMARY.md`; the book builds.
  **Measured before writing:** the chapters used 61 words of two or more capitals outside code, links and comments,
  among them the RISC-V interrupt names the ledger quotes, units, project names (`RGX`, `PGEN`), a chip's part
  number and a Roman numeral. Every one but the numeral is defined — a reader meeting `TL16C550C` needs it as much
  as `UART` — so the gate needs no exception list beyond the project's identifiers and Roman numerals.
  ⛔ *Corrected `2026-10-02`:* this leaf said eADL is "never spelled out in this repository", from a census of the README,
  the roadmap, the book and the semantics only. The director answered "eADL = Extended ADL", pointing to the task trees:
  leaf `M1.13.3` had recorded "Extended Architecture Description Language", read as extensible, since `2026-09-29`. The
  entry now says so (`ARCHOGEN-PROGRAM-0300`), and a search now starts with the task trees, the Knowledge Map cards and
  the decision records, then the book, as the director directed.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — no glossary: `git ls-tree HEAD docs/book/src/ | grep -c "glossary"` → 0, and the
    measure above found 61 undefined acronyms across the chapters.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the book defined terms where it used them, or not at all, and nothing held
    a list to the chapters: `git show HEAD:scripts/check_doctrines.project.sh | grep -c "BOOK-GLOSSARY"` → 0.
    WHERE: `docs/book/src/`, the enforcer's project slot.
  - [x] **FIX** — `docs/book/src/glossary.md` (acronyms, then the project's own words, each linked), last in
    `SUMMARY.md`; `scripts/check_book_glossary.sh`, registered; `DOCTRINE_ENFORCEMENT.md`'s row; each entry naming a
    ledgered source linked to its entry; `docs/book/`'s ceilings raised to 48 files and 384 KiB by the ruling's record,
    since the glossary took the book to 295 147 bytes, past its 288 KiB, and `README_POLICY.md` changed with it.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_book_glossary.sh` → rc=0, `book-glossary: OK (60 acronym(s)
    used and defined; 17 term(s), each in a chapter)`; `--self-test` → `9 pass / 0 fail (9 arms)`; with the
    undefined-acronym rule removed, and with the stale-entry rule removed, an arm each → `expected exit 1, got 0`.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/build_book.sh` → rc=0;
    `bash scripts/run_self_tests.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the book's glossary and `SUMMARY.md`; `DOCTRINE_ENFORCEMENT.md`; this leaf, the frontier and
    the log; `CHANGELOG.md`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0297 (leaf PROGRAM.47.2)`

- ID: `PROGRAM.47.3`
  Status: `done` — `2026-10-02`
  Goal: the index at the end of the book, generated from the glossary's terms and every chapter's and annex's
  headings by `scripts/check_book_index.sh`; `BOOK-INDEX`, refusing an index that differs from what the generator writes.
  Acceptance: the generator's and the gate's RED arms; the index last in `SUMMARY.md`; the book builds.
  **Measured before writing:** mdBook 0.5.2's anchors, on a probe book of awkward headings (code, emoji, a section
  sign, an em dash, accents, a repeat): the heading's text lower-cased, letters, digits, `-` and `_` kept, a space a
  `-`, the rest dropped, a repeat `-1`. The generator does exactly that, and its self-test pins the measured cases.
  The gate is `scripts/check_book_index.sh`, not `book_index.sh` as first written: the spine's self-test stubs every
  gate by the `check_` name, and the first name broke it (`spine self-test: 38 pass / 1 fail`).

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — no index: `git ls-tree HEAD docs/book/src/ | grep -c "book-index"` → 0.
  - [x] **ROOT CAUSE (WHY + WHERE)** — nothing produced one; an index kept by hand is wrong the day a heading
    moves: `git show HEAD:scripts/check_doctrines.project.sh | grep -c "BOOK-INDEX"` → 0. WHERE: `docs/book/src/`,
    `scripts/`.
  - [x] **FIX** — `scripts/check_book_index.sh`: `--write` generates `docs/book/src/book-index.md` from `SUMMARY.md`,
    the chapters' headings and the glossary; with no argument it is `BOOK-INDEX`, registered; the index last in
    `SUMMARY.md`; `DOCTRINE_ENFORCEMENT.md`'s row.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_book_index.sh` → rc=0, `book-index: OK (269 entries, as the
    chapters and the glossary give them)`; `--self-test` → `7 pass / 0 fail (7 arms)`; on the real build, 192 section
    links checked against the HTML's ids, 0 missing; a mutant without the space rule and one passing a stale index,
    each red.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/build_book.sh` → rc=0;
    `bash scripts/run_self_tests.sh` → rc=0, `self-tests: OK — 42 self-test(s) passed`;
    `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the index and `SUMMARY.md`; `DOCTRINE_ENFORCEMENT.md`; the ruling's record names the script;
    this leaf, the frontier and the log; `CHANGELOG.md`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0298 (leaf PROGRAM.47.3)`

- ID: `PROGRAM.47.4`
  Status: `done` — `2026-10-02`
  Goal: the runtime chapter in layers, its full mechanics moved to an annex.
  **Done.** `docs/book/src/runtime.md` opens with the idea in plain words — one cook, a row of orders with deadlines,
  a referee who decides beside a player who acts — then a one-minute summary for engineers, then how it works (a
  task's life, who runs next, holding interrupts back, the four faults), then the precise rules, how we know they are
  right, and today against ahead. *Annex A: The runtime's rules in detail* (`annex-runtime.md`), in a new *Annexes*
  part, takes the case-by-case mechanics — latched arrivals and completions inside a region, panics, the record a
  board keeps, the masked-overrun rule — and the history of how the two models were made to agree. The rewrite also
  corrected a stale claim: the chapter said the bare-metal build was unavailable until the target was installed, and
  `bash scripts/no_std_build.sh` builds `rt-core` for `riscv64imac-unknown-none-elf`, rc=0.
  Verification: `bash scripts/build_book.sh` → rc=0; 231 anchored links across the book checked against the built
  HTML, 0 missing; `book-glossary: OK`, `book-index: OK (280 entries …)`, `book-anchors: OK`, `book-coverage: OK`;
  `cargo test --all -q` → 958 passed, 0 failed; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0301 (leaf PROGRAM.47.4)`

- ID: `PROGRAM.47.5`
  Status: `done` — `2026-10-02`; one chapter per child: `.1` the introduction, `.2` the tour, `.3` reading a description, `.4` describing a workload, `.5` checking a description, `.6` what a report may claim, `.7` the scheduling checker, `.8` the catalog, `.9` where systems run, `.10` the command line, `.11` the engine API, `.12` verifying the toolchain, with Annex B, `.13` the boundary, `.14` the supported profile, `.15` the use cases, `.16` kinds and schemas, `.17` quantities and units, `.18` modules and composition, `.19` presence, absence and relevance, `.20` refinement, `.21` what is versioned, `.22` the S0 path, `.23` what the project relies on from outside
  Goal: every other chapter in layers, one leaf and commit each when started, the chapters a newcomer meets first
  leading — the introduction and the tour, reading and workload, checking, evidence and analysis, the catalog,
  targets, the command line, the engine API, verification — and the gory parts of each moved to annexes.
  Verification: every chapter `SUMMARY.md` lists opens in plain words — the tour by its own opening, with the
  summary `.2` added — the two annexes, the glossary and the index apart by design; Annex A holds the runtime's
  mechanics and Annex B the repository's checks; the book gates and `bash scripts/check_doctrines.sh` green
  Commit: the children's, `ARCHOGEN-PROGRAM-0302` to `-0327`

- ID: `PROGRAM.47.5.1`
  Status: `done` — `2026-10-02`
  Goal: the introduction, the book's first page, in layers.
  **Done.** It opens with the idea in plain words — the small devices such a system runs, describing what is needed
  and letting archogen build it, the what-not-how rule — then a one-minute summary for engineers, the four steps,
  the precise rules it held before (the boundary table, the first profile, what the project does not claim), today
  against ahead, and how to read the book with its annexes, glossary and index.
  Verification: `bash scripts/build_book.sh` → rc=0; the book gates and `bash scripts/check_doctrines.sh` → `=== all
  doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0302 (leaf PROGRAM.47.5.1)`

- ID: `PROGRAM.47.5.2`
  Status: `done` — `2026-10-02`
  Goal: the tour in layers.
  **Done.** The tour already opened in plain words and kept today apart from tomorrow; it gains the one-minute
  summary for engineers, so an expert can see what is real in a paragraph.
  Verification: `bash scripts/build_book.sh` → rc=0; `cargo test -q -p archogen-cli --test book_tour` → rc=0;
  `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0303 (leaf PROGRAM.47.5.2)`

- ID: `PROGRAM.47.5.3`
  Status: `done` — `2026-10-02`
  Goal: *Reading a description* in layers.
  **Done.** It opened with a block on the language's two normative halves, a reviewer's preface in front of a
  newcomer; it now opens with what reading is and what an S-expression looks like, a one-minute summary for
  engineers, and three steps, and the preface became the first of the precise rules. The console transcripts and
  the corpus counts two tests read are unchanged.
  Verification: `cargo test -q -p archogen-cli --test book_transcripts` → `6 passed`; `cargo test -q -p eadl-front
  --test corpus` → `14 passed`; `--test reference` → `55 passed`; `bash scripts/build_book.sh` → rc=0;
  `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0304 (leaf PROGRAM.47.5.3)`

- ID: `PROGRAM.47.5.4`
  Status: `done` — `2026-10-02`
  Goal: *Describing a workload* in layers.
  **Done.** It opens with what a workload and a task are and the four things a task says — how often, how quickly,
  how urgent, what if late — and what it deliberately does not say; then the one-minute summary and the precise
  rules, every section kept. The reach line `crates/eadl-model/tests/kinds.rs` quotes is unchanged.
  Verification: `cargo test -q -p eadl-model --test kinds` → `37 passed`; `cargo test -q -p archogen-cli --test
  book_transcripts` → `6 passed`; `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `===
  all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0305 (leaf PROGRAM.47.5.4)`

- ID: `PROGRAM.47.5.5`
  Status: `done` — `2026-10-02`
  Goal: *Checking a description* in layers.
  **Done.** It opened with a console transcript; it now opens with the one question the check answers, its passes
  as a proof-reader's, and the four verdicts a newcomer meets, each with what it means; then the one-minute summary,
  the transcript as how it works, and the precise rules, every section kept. The command, the declaration count and
  the corpus size three tests read are unchanged.
  Verification: `cargo test -q -p archogen-cli --test kind_modules` → `7 passed`; `-p eadl-model --test
  semantic_corpus` → `16 passed`; `--test kinds` → `37 passed`; `bash scripts/build_book.sh` → rc=0;
  `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0306 (leaf PROGRAM.47.5.5)`

- ID: `PROGRAM.47.5.6`
  Status: `done` — `2026-10-02`
  Goal: *What a report may claim* in layers.
  **Done.** It opens with a medical check-up's separate results, never "you are healthy", as the picture of a report
  in which each property has its own answer and its own kind of evidence; then the one-minute summary, the three
  encodings of the prohibition as how it works, and bounds, trust and hashes as the precise rules.
  Verification: `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0307 (leaf PROGRAM.47.5.6)`

- ID: `PROGRAM.47.5.7`
  Status: `done` — `2026-10-02`
  Goal: *What the scheduling checker establishes* in layers.
  **Done.** It opened with the recurrence's formula; it now opens with what a response time is and a two-task example
  worked in words — `chime`'s 3 ms plus `beat`'s 2 ms, no second interruption before `beat`'s next release at 10 ms,
  so 5 ms against 30 — and the chapter's point, that the answer holds only in a model under listed assumptions; then
  the one-minute summary, the formula as how it works, and the precise rules, every section kept.
  Verification: `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0308 (leaf PROGRAM.47.5.7)`

- ID: `PROGRAM.47.5.8`
  Status: `done` — `2026-10-02`
  Goal: *The catalog* in layers.
  **Done.** It opened with one thirty-line paragraph of everything the crate does; it now opens with a cookbook whose
  recipes are tested, signed and sealed, as the picture of records, hashes and reviews; then the one-minute summary,
  five steps from a record to a claim, the precise rules — the crate's paragraph first, as *What the crate does
  today* — and *Today and ahead*, every section kept.
  Verification: `bash scripts/build_book.sh` → rc=0; `book-glossary: OK`, `book-anchors: OK`; `bash scripts/
  check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0310 (leaf PROGRAM.47.5.8)`

- ID: `PROGRAM.47.5.9`
  Status: `done` — `2026-10-02`
  Goal: *Where generated systems run* in layers, with the director's question of which boards.
  **Done.** It opens with the three places a system runs, in order, and what each can and cannot tell; then *Which
  board?*, answering the director's question of `2026-10-02` with only what is verified: the first board a RISC-V
  microcontroller, none chosen (`M5`); the Pico 2's RP2350 as an example of the kind, quoted from its datasheet, read
  and ledgered (`rp2350`, sha256 `2877d0f2…`, build-date `2025-07-29`) since the product page refuses a scripted read;
  its cores 32-bit against the 64-bit emulated target; other processor families a port each, none planned. Then the
  one-minute summary and the precise rules. Two names the live glossary refused, `RP2350` and `RV32IMAC`, are defined.
  Verification: `source-ledger: OK (20 entries …)`; `book-glossary: OK (62 acronym(s) …)`; `bash scripts/
  build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0311 (leaf PROGRAM.47.5.9)`

- ID: `PROGRAM.47.5.10`
  Status: `done` — `2026-10-02`
  Goal: *The `archogen` command line* in layers.
  **Done.** It opens with what the program is for, how each command says how finished it is, and what an exit code
  tells a script; then the one-minute summary — the command table, three maturities, the programmatic exposure —
  the help transcript as how it works, and the precise rules, every section kept.
  Verification: `cargo test -q -p archogen-cli` → 0 failed; `bash scripts/build_book.sh` → rc=0;
  `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0312 (leaf PROGRAM.47.5.10)`

- ID: `PROGRAM.47.5.11`
  Status: `done` — `2026-10-02`
  Goal: *The engine API* in layers.
  **Done.** It opens with why programs — a web page, an editor, a build server, an AI assistant — need a way in, and
  that their answer carries the verdict a person sees; then the one-minute summary, the chapter's own opening as how
  it works, and the precise rules, every section and the example the tests read kept.
  Verification: `cargo test -q -p archogen-api` → 0 failed; `cargo test -q -p archogen-cli --test api_parity` → `6
  passed`; `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0313 (leaf PROGRAM.47.5.11)`

- ID: `PROGRAM.47.5.12`
  Status: `done` — `2026-10-02`
  Goal: *Verifying the toolchain* in layers, its repository checks moved to an annex.
  **Done.** At 48.9 KB it stood at the book's per-file ceiling of 48 KiB. It now opens with the tiers from quick to
  thorough, fuzzing, mutation and Miri each in one plain clause, and the three outcomes, because "nothing failed" is
  not "everything passed"; then the one-minute summary and the precise rules. The thirteen sections about the
  repository's own integrity — among them book coverage, other repositories, the ledgers, the sealed histories, the
  figure register and the real-run diagnostics — move to **Annex B: The checks
  that keep the repository honest** (`annex-repository.md`), unchanged but for two ledger links the source-ledger gate
  asked for; the chapter now cites LinkedSpec's ledger entry, which only the moved text cited. The figure register's rows follow the moved
  lines. Chapter 34,888 bytes, annex 16,595; every anchored link in the book resolves.
  Verification: `source-ledger: OK (20 entries …)`; `book-glossary: OK`; `book-index: OK (313 entries …)`;
  `readme-routes: OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines
  green ===`
  Commit: `ARCHOGEN-PROGRAM-0314 (leaf PROGRAM.47.5.12)`

- ID: `PROGRAM.47.5.13`
  Status: `done` — `2026-10-02`
  Goal: *The boundary* in layers.
  **Done.** It opens with ordering a meal — say what you want, leave the cooking to the kitchen — as the picture of
  what against how; why only a description of *what* can be reused; and that the line is drawn by meaning, not
  vocabulary. Then the one-minute summary, the rule and its three tests as how it works, and the precise rules, every
  section kept; the summary names the tests rather than counting them, and separates the classifier
  (`crates/eadl-model/src/boundary.rs`) from its fixture F27 (`tests/f27_boundary.rs`).
  Verification: `cargo test -q -p eadl-model --test f27_boundary` → `7 passed`; `-p archogen-cli --test
  book_transcripts` → `6 passed`; `figure-register: OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/
  check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0315 (leaf PROGRAM.47.5.13)`

- ID: `PROGRAM.47.5.14`
  Status: `done` — `2026-10-02`
  Goal: *The supported profile* in layers.
  **Done.** It opens with a bridge's load-limit sign, behind which its engineers stand and beyond which they promise
  nothing, as the picture of a profile; then `rt-static-up-v1` read from its name — real-time, static, one processor
  core, first version — in concrete terms, and what is left out, each refusal a to-do list. Then the one-minute
  summary, the definition as how it works, the refusal rules as the precise rules, and the later profiles as today
  and ahead, every section kept. Its exclusion count and exit code are the code's: 18 slugs in
  `crates/eadl-model/src/profile.rs`, `UnsupportedProfile => 12` in `crates/archogen-api/src/status.rs`.
  Verification: `cargo test -q -p eadl-model profile` → 0 failed; `figure-register: OK`; `book-glossary: OK`;
  `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0316 (leaf PROGRAM.47.5.14)`

- ID: `PROGRAM.47.5.15`
  Status: `done` — `2026-10-02`
  Goal: *The use cases* in layers.
  **Done.** It opens with an exam written before the course is taught, as the picture of example systems fixed before
  the engine existed; each case's required answer in one plain line — build, "cannot be sure", bridge the gap or say
  what is missing, stay refused — and the sealed set as the end-of-course measure. Then the one-minute summary, the
  table as how it works, and the precise rules, every section kept; the plain layer names the cases rather than
  counting them.
  Verification: `figure-register: OK`; `book-glossary: OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/
  check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0317 (leaf PROGRAM.47.5.15)`

- ID: `PROGRAM.47.5.16`
  Status: `done` — `2026-10-02`
  Goal: *Kinds and schemas* in layers.
  **Done.** It opens with a printed form — which boxes, which required, what goes in each — as the picture of a kind,
  a period measured in parsecs as what the form catches (`crates/eadl-model/tests/kinds.rs`, `f03_units.rs` refuse
  it), `defkind` as the one form for making forms, and shape against truth. Then the one-minute summary, the
  chapter's opening and its normative pointer as how it works, and the precise rules, every section kept. The
  reach lines `crates/eadl-model/tests/kinds.rs` gates are unchanged, and the new layers add no reach figure.
  Verification: `cargo test -q -p eadl-model --test kinds` → `37 passed`; `figure-register: OK`; `book-glossary:
  OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0318 (leaf PROGRAM.47.5.16)`

- ID: `PROGRAM.47.5.17`
  Status: `done` — `2026-10-02`
  Goal: *Quantities and units* in layers.
  **Done.** It opens with "wait 10" meaning nothing until you say 10 what; a unit as part of a number's meaning, checked
  before any arithmetic — parsecs, zero hertz, a time against an amount of memory refused; exact fractions, because an
  analysis adds its own results up; and a direction on every comparison. Then the one-minute summary, the shape rule
  as how it works, and the precise rules, every section and transcript kept.
  Verification: `cargo test -q -p eadl-model --test f03_units` → `23 passed`; `-p archogen-cli --test
  book_transcripts` → `6 passed`; `figure-register: OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/
  check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0320 (leaf PROGRAM.47.5.17)`

- ID: `PROGRAM.47.5.18`
  Status: `done` — `2026-10-02`
  Goal: *Modules and composition* in layers.
  **Done.** It opens with why nobody describes a device in one file, an import as a program imports a library — a
  local name, only what is exported, a version or later — elaboration as assembling the pieces and checking the whole,
  and the troubles of assembly it refuses. Then the one-minute summary, the chapter's opening and *What you can run
  today* as how it works, and the precise rules, every section and transcript kept; the in-page link to *Names carry
  their whole path* resolves, since a heading's level does not change its anchor.
  Verification: `cargo test -q -p archogen-cli --test module_cases --test module_files --test book_transcripts` →
  `6`, `8`, `9 passed`; `figure-register: OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh`
  → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0321 (leaf PROGRAM.47.5.18)`

- ID: `PROGRAM.47.5.19`
  Status: `done` — `2026-10-02`
  Goal: *Presence, absence, and relevance* in layers.
  **Done.** It opens with asking whether a car has air conditioning — yes, no, or "I don't know", which is not a no —
  as the picture of offered, absent and undescribed; a gap mattering only inside what the system depends on; and a
  yes-and-no refused rather than guessed. Then the one-minute summary, the three-state table as how it works, and the
  precise rules, every section and transcript kept.
  Verification: `cargo test -q -p eadl-model --test f04_f06_presence` → `10 passed`; `-p archogen-cli --test
  closure_report --test book_transcripts` → `6`, `4 passed`; `figure-register: OK`; `bash scripts/build_book.sh` →
  rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0322 (leaf PROGRAM.47.5.19)`

- ID: `PROGRAM.47.5.20`
  Status: `done` — `2026-10-02`
  Goal: *Refinement* in layers.
  **Done.** It opens with a family of chips that starts as one general description and concrete chips that claim to
  fit it; the claim checked, not trusted — every guarantee kept, every bound met, nothing ruled out offered — and
  additions shown, not hidden. Then the one-minute summary, the example and §5.1.1 as how it works, and the precise
  rules, every section kept.
  Verification: `cargo test -q -p eadl-model --test f07_refinement` → `13 passed`; `figure-register: OK`; `bash
  scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0323 (leaf PROGRAM.47.5.20)`

- ID: `PROGRAM.47.5.21`
  Status: `done` — `2026-10-02`
  Goal: *What is versioned, and what changing it costs* in layers.
  **Done.** It opens with a description that must mean the same years from now, a version as a promise, and the check
  that holds each version the code declares to this page. Then the one-minute summary and the chapter's opening as how
  it works. The entries stay `##` headings, since `VERSION-REGISTER` reads a section as `## \`<id>\``; so the
  precise rules follow as they stand rather than under a heading of their own.
  Verification: `version-register: OK (11 entries …)`; `figure-register: OK`; `bash scripts/build_book.sh` → rc=0;
  `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0324 (leaf PROGRAM.47.5.21)`

- ID: `PROGRAM.47.5.22`
  Status: `done` — `2026-10-02`
  Goal: *The S0 early generation path* in layers.
  **Done.** It opens with an architect's cardboard model as the picture of S0: two tasks turned into a program that
  runs on your computer and prints its releases, so the pipeline's shape is tested while it is cheap to change; what
  it does not prove, and what it does. Then the one-minute summary; the gate, the corpus, the observation contract and
  a build as how it works; provenance, `--locked`, broken descriptions and F28's blind spot as the precise rules; and
  what S0 owes the supported path as today and ahead. The new layers name no count of descriptions or files, which
  `crates/archogen-cli/tests/s0_chapter.rs` would compare with the directory and the build.
  Verification: `cargo test -q -p archogen-cli --test s0_chapter --test book_transcripts` → `6`, `7 passed`;
  `figure-register: OK`; `bash scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines
  green ===`
  Commit: `ARCHOGEN-PROGRAM-0325 (leaf PROGRAM.47.5.22)`

- ID: `PROGRAM.47.5.23`
  Status: `done` — `2026-10-02`
  Goal: *What this project relies on from outside* in layers, the last chapter.
  **Done.** It opens with no project being built from nothing — an emulator, a compiler, a book generator, other
  people's specifications — and the page as the list of what is relied on, what for, what it cannot tell, and when to
  look again, so a claim about an outside tool is dated here rather than timeless elsewhere. Then the one-minute
  summary and the chapter's opening as how it works. The entries stay `##` headings, which `SOURCE-LEDGER` reads.
  Verification: `source-ledger: OK (20 entries …)`; `figure-register: OK`; `readme-routes: OK`; `bash scripts/
  build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`
  Commit: `ARCHOGEN-PROGRAM-0327 (leaf PROGRAM.47.5.23)`

- ID: `PROGRAM.47.6`
  Status: `done` — `2026-10-02`
  Goal: the index without the layer headings, which name a chapter's part and no topic.
  Acceptance: the generator leaves the four layer headings out, a RED arm proving it; every index link resolves on
  the built book; the ledger chapter's own layers no longer read as citations.
  **Found by** `SOURCE-LEDGER` refusing the ledger chapter's layers: the index linked `ledger.md#how-it-works`, and
  every link into the ledger is read as a citation of an entry. The cause was the index, not the gate: by then it
  listed *How it works* and *The idea, in plain words* 23 times each, *The precise rules* 21 and *Today and ahead* 5,
  about 70 lines of the index under four words that name no topic.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — `git show 8e64d68:docs/book/src/book-index.md | grep -c "^- How it works —"` → 22,
    and 23 with the ledger chapter layered, when `SOURCE-LEDGER` refused it: `docs/book/src/book-index.md cites
    ledger.md#how-it-works, which is no entry`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the generator indexed every `##` and `###` heading, and the book's layering
    (`docs/decisions/decision_book-in-layers.md`) gives every chapter the same four:
    `git show 8e64d68:scripts/check_book_index.sh | grep -c "if len(m.group(1)) in (2, 3):$"` → 1, the one
    condition, with no exclusion. WHERE: `scripts/check_book_index.sh`, the heading loop.
  - [x] **FIX** — `LAYERS`, the four headings, left out of the entries while their anchors are still counted, so a
    later repeat's `-1` stays mdBook's; a self-test arm with a layer heading in its chapter.
  - [x] **ADDRESSED (verified)** — `bash scripts/check_book_index.sh --write` → `(270 entries)`, from 342;
    `--self-test` → `8 pass / 0 fail (8 arms)`; a mutant without the exclusion → `6 pass / 2 fail`; 190 index links
    checked against the built book's ids, 0 missing.
  - [x] **NO REGRESSION** — no Rust source changed; `bash scripts/build_book.sh` → rc=0;
    `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md`'s `BOOK-INDEX` row; the ruling's record; this leaf and the log;
    `CHANGELOG.md`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-PROGRAM-0326 (leaf PROGRAM.47.6)`
