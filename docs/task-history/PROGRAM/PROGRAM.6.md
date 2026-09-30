- ID: `PROGRAM.6`
  Status: `done`
  Goal: implement semantic versioning separation (§15) — language/profile semantics, engine
  implementation, catalog entries, model versions, evidence formats — as machine-checked
  version records with a compatibility corpus.
  Acceptance: a locked description retains its meaning across an engine upgrade; F25 has a
  home.
  Children: `PROGRAM.6.1`, `PROGRAM.6.2`, `PROGRAM.6.3` — decomposed `2026-09-30` on a census of what is versioned
  today: the language (`EADL_1 = "eadl/1"`, `crates/eadl-front/src/language_version.rs`), the profile
  (`rt-static-up-v1`, `crates/eadl-model/src/profile.rs`), the engine (`0.1.0` in all 9 member manifests), one stub
  catalog entry (`s0.hosted-playground.periodic` `0.1.0`, `crates/archogen-s0/src/provenance.rs`) and two evidence
  formats (`archogen-provenance/1`, `cost-accounting/1`). Catalogs proper and device/timing models do not exist
  yet (`M6`, `M2`); F25 — a locked rebuild after a catalog update — is `M6.4`'s, and `6.1` names that home.
  Verification: closed `2026-09-30` by its three children. Its acceptance, each half measured: **"a locked
  description retains its meaning across an engine upgrade"** — every description's verdict is frozen and checked
  on every run (`PROGRAM.6.2`: 120 descriptions; an engine change that moves 20 of them is named, line by line);
  **"F25 has a home"** — `docs/book/src/versions.md` names catalogs as `M6`'s and F25 at `M6.4` (`PROGRAM.6.1`). The
  register pins every versioned surface it lists, the two evidence formats to their identifiers (`PROGRAM.6.3`).
  Commit: `ARCHOGEN-PROGRAM-0133` (`.6.1`), `ARCHOGEN-PROGRAM-0134` (`.6.2`), `ARCHOGEN-PROGRAM-0135` (`.6.3`)

- ID: `PROGRAM.6.1`
  Status: `done`
  Goal: one register of everything versioned, what changes each version, and what pins it — derived from the
  code, so a new format identifier or a bump cannot land unannounced.
  Acceptance: a book chapter with one entry per versioned surface (version, where declared, when it changes,
  what pins it, what a locked description or artifact keeps), the not-yet-versioned surfaces named with their
  owners (F25 at `M6.4`); a gate deriving every version constant, profile id and the engine version from the
  code, each claimed by an entry carrying its value, and every entry's declaration still present; RED arms.
  Verification: see the checklist — the register, a gate over four derived populations (12 arms, 7 mutations),
  a falsification on the real code.
  Commit: `ARCHOGEN-PROGRAM-0133 (leaf PROGRAM.6.1)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the versions existed as constants scattered through four crates, and nothing
    recorded what changes each or what pins it:
    ```text
    $ git cat-file -e b387492:docs/book/src/versions.md            -> rc=128 (no register)
    $ git grep -nE 'pub const [A-Z0-9_]+: &str = "[a-z][a-z0-9-]*/[0-9]+' b387492 -- 'crates/*/src/*.rs'
      b387492:crates/archogen-s0/src/provenance.rs:43:pub const FORMAT: &str = "archogen-provenance/1";
      b387492:crates/eadl-front/src/language_version.rs:36:pub const EADL_1: &str = "eadl/1";
      b387492:crates/rt-analysis/src/cost.rs:36:pub const CONTRACT_VERSION: &str = "cost-accounting/1 (ROADMAP.md §7.4.1)";
      b387492:crates/rt-analysis/src/response.rs:44:pub const MODEL: &str = "idealized-zero-overhead/1 (ROADMAP.md §7.4)";
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — §15's separation was honoured constant by constant, each with a comment
    saying why it is versioned, but with no index the set had no edge: ⛔ the census that planned this leaf looked
    for constants **named** `VERSION|FORMAT|…` and found six surfaces; `MODEL` above, the analysis model every
    conclusion names, was the seventh, found only when the gate derived by the value's **shape** — its first run,
    against a register holding the six:
    ```text
    $ bash scripts/check_version_register.sh
      VERSION-REGISTER: the code declares const:crates/rt-analysis/src/response.rs:MODEL = 'idealized-zero-overhead/1', and no entry of docs/book/src/versions.md claims it — add one
      exit=1
    ```
  - [x] **FIX** — `docs/book/src/versions.md`, "What is versioned, and what changing it costs": seven entries
    (`language`, `profile`, `engine`, `catalog-s0`, `analysis-model`, `provenance-format`, `cost-format`) with
    version, declaration locator, what changes it, what pins it (`pending` stated where nothing does yet —
    `PROGRAM.6.2`, `PROGRAM.6.3`), and what it keeps; the not-yet-versioned surfaces named with their owners —
    catalogs `M6` (**F25 at `M6.4`**), models `M2`, the plan's identity `M4.7`. New doctrine **`VERSION-REGISTER`**.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_version_register.sh
      version-register: OK (7 entries; 7 declared version(s), each carried by its entry)     exit=0  (0.55 s)
    $ bash scripts/check_version_register.sh --self-test
      version-register self-test: 12 pass / 0 fail (12 arms)
    ```
    Seven mutations V-1–V-7, restored by `cmp`, each fails its own arms. On the real code: `FORMAT` bumped to
    `archogen-provenance/2` → `entry \`provenance-format\` does not carry the declared value … the code says
    'archogen-provenance/2'`, exit=1; restored by `git checkout`, matching HEAD.
  - [x] **NO REGRESSION** — `mdbook build docs/book` exit=0; `BOOK-ANCHORS` exit=0; `BOOK-COVERAGE` OK; the doctrine
    driver green at the commit; `make focused` passed.
  - [x] **LOCKSTEP** — the chapter in `SUMMARY.md`; `verification.md` gains "What is versioned is written down";
    `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md` rows; the live docs.

- ID: `PROGRAM.6.2`
  Status: `done`
  Goal: "a locked description retains its meaning across an engine upgrade", made mechanical — the verdict of
  every tracked description, frozen, so an engine change that moves one is refused unless the change says so.
  Acceptance: a frozen table of every tracked description's verdict and diagnostic codes, checked by a test on
  every run; the census `M1.34`–`M1.36` each ran by hand (120 descriptions before and after) becomes that test.
  Verification: see the checklist — the table cross-checked against the binary census, three falsifications, a
  permanent mutation-catalog entry.
  Commit: `ARCHOGEN-PROGRAM-0134 (leaf PROGRAM.6.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the register's own `engine` entry said nothing pinned meaning across an engine
    change outside the conformance suite:
    ```text
    $ git grep -n "PROGRAM.6.2" 2883035 -- docs/book/src/versions.md
      2883035:docs/book/src/versions.md:46:| Pinned by | the conformance suite today. ⚠️ A frozen verdict for every tracked description is `PROGRAM.6.2`, pending: …
    $ git cat-file -e 2883035:crates/archogen-cli/tests/verdicts.rs      -> rc=128
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the conformance suite pins its own cases; the other descriptions (examples,
    module trees, the boundary corpus, the feedback evidence) kept their verdicts only while someone re-ran the
    census by hand — `M1.34`, `M1.35` and `M1.36` each did, building the CLI from a stash and from the change:
    ```text
    $ diff m136_before.txt m136_after.txt      (M1.36's census: exit code and output digest per description)
      rc=0
    ```
  - [x] **FIX** — `crates/archogen-cli/tests/verdicts.rs`: every `*.eadl` under the root (walked, skipping
    `target/`, `build/`, `vendor/` and hidden directories — no `git` dependency), run through `archogen check`
    in-process, its exit code and sorted diagnostic codes compared with `crates/archogen-cli/tests/verdicts.txt`;
    a moved, new or vanished verdict fails and is named; `ARCHOGEN_BLESS_VERDICTS=1` regenerates deliberately.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ ARCHOGEN_BLESS_VERDICTS=1 cargo test -q -p archogen-cli --test verdicts; cargo test -q -p archogen-cli --test verdicts
      test result: ok. 1 passed; 0 failed      120 descriptions (35 × 0, 70 × 10, 2 × 11, 8 × 12, 5 × 13), 0.05 s
    $ diff <binary census exit codes> <table exit codes>      -> rc=0, 120 identical (re-derived two ways)
    ```
    Falsified: the D ≤ T rule made to refuse equality → `20 of 120 description(s) differ` with each `MOVED: … was
    \`0 -\`, is \`12 unsupported-profile\``; a new `examples/zz-probe.eadl` → `new, with no frozen verdict`; a table
    row for a missing file → `frozen but gone`; each restored (`git checkout`, `cmp`). Kept as catalog entry
    `constrained-deadline-seen-by-the-frozen-verdicts` → `killed by every_description_keeps_its_frozen_verdict`.
  - [x] **NO REGRESSION** — the test adds 0.08 s; ignored under Miri with the other corpus walks:
    ```text
    $ cargo test --all -q
      test result: passed=617 failed=0 ignored=1 over 45 suites      (the 1 ignored: fuzz_extended, by design)
    ```
    The doctrine driver green at the commit; `make focused` passed.
  - [x] **LOCKSTEP** — `versions.md`'s `engine` entry is pinned now, and the chapter gains "How an engine change is
    held to what descriptions mean"; the mutation catalog gains its ninth entry.

- ID: `PROGRAM.6.3`
  Status: `done`
  Goal: an evidence format's shape is pinned to its version: `archogen-provenance/1` and `cost-accounting/1`
  cannot change what they emit without the identifier moving.
  Acceptance: a golden sample per format, compared on every run; a shape change with the same identifier is
  refused, with a bump accepted only alongside a new golden.
  Verification: see the checklist — two goldens, a shape lexer with its own test, four falsifications, two
  permanent catalog entries.
  Commit: `ARCHOGEN-PROGRAM-0135 (leaf PROGRAM.6.3)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the register said nothing refused a format change under the same identifier:
    ```text
    $ git grep -n "PROGRAM.6.3" eac86bd -- docs/book/src/versions.md
      eac86bd:docs/book/src/versions.md:68:| Pinned by | `crates/archogen-cli/tests/s0_provenance.rs`. ⚠️ A golden sample that refuses a shape change under …
      eac86bd:docs/book/src/versions.md:90:| Pinned by | the tests in `crates/rt-analysis/src/cost.rs`. ⚠️ The golden sample is `PROGRAM.6.3`, pending |
    ```
  - [x] **ROOT CAUSE (WHY + WHERE)** — the provenance test reads the handful of fields it asserts and checks the
    JSON is well formed; nothing compared the *set* of keys, so a key added under `archogen-provenance/1` passed.
    The cost contract's drift test compares code with its page, not with its version — both could move together
    under `/1`. Measured, with F-1's extra key in the artifact:
    ```text
    $ cargo test -q -p archogen-cli --test s0_provenance      -> test result: ok. 4 passed; 0 failed
    $ cargo test -q -p archogen-s0                            -> test result: ok. 20 passed; 0 failed
    ```
  - [x] **FIX** — a golden per identifier, **never rewritten**: `crates/archogen-cli/tests/format_golden.rs` builds
    the S0 base fixture and freezes the provenance's shape (every key path and value kind — a lexer with a key
    stack, deliberately not a parser, and a test of its own) in `tests/goldens/archogen-provenance-1.golden`;
    `crates/rt-analysis/tests/format_golden.rs` freezes what `cost-accounting/1` says (kinds of total and ledger
    categories through exhaustive `match`es, the seven identifications, a rendered ledger) in
    `tests/goldens/cost-accounting-1.golden`. `ARCHOGEN_BLESS_FORMATS=1` creates a golden for a new identifier
    only.
    ⛔ The first cut imported the identifier from the S0 prototype and `S0-RETIREMENT` refused the commit (`imports
    the S0 prototype. Only its declared consumers may`); the test now reads it from the artifact, which carries it —
    the stronger pin, since it is what a consumer reads. F-1 and F-2 re-run after the change: both still refused.
  - [x] **ADDRESSED (verified)** —
    ```text
    $ cargo test -q -p archogen-cli --test format_golden; cargo test -q -p rt-analysis --test format_golden
      test result: ok. 2 passed; 0 failed          test result: ok. 1 passed; 0 failed
    provenance shape: 20 key paths — re-derived from the renderer's source: 6 top-level, 5 realization, 9 records[]
    ```
    Falsified, sources restored by `git checkout`: **F-1** a key added, same identifier → `the shape of
    \`archogen-provenance/1\` has changed`; **F-2** the identifier bumped with no golden → `no golden for
    \`archogen-provenance/2\``; **F-3** blessing over the changed shape → still `has changed`, `test result: FAILED`
    (a golden is not overwritten); **F-4** a contract statement reworded → `what \`cost-accounting/1\` says has
    changed`. F-1 and F-4 kept as catalog entries → `cargo xtask mutate` → `mutate: OK — 11 mutation(s)`.
    ⚠️ The lexer's own test caught a bug in it on the first bless (empty frames gave `.realization.id`); the bad
    golden was deleted and re-blessed.
  - [x] **NO REGRESSION** —
    ```text
    $ cargo test --all -q
      test result: passed=620 failed=0 ignored=1 over 47 suites
    ```
    `VERSION-REGISTER` OK; the doctrine driver green at the commit; `make focused` passed.
  - [x] **LOCKSTEP** — `versions.md`: both format entries pinned, and a section "How a format is held to its
    identifier"; the catalog's tenth and eleventh entries.
