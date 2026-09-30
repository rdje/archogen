- ID: `PROGRAM.5`
  Status: `done`
  Goal: implement the dependency/evidence ledger of §15 and §19 as a tracked, checkable
  record (external source versions, retrieval dates, scope, limitations, revalidation
  triggers).
  Acceptance: every external source claim in the book and decisions resolves to a ledger row.
  Made mechanical `2026-09-29` (census: 0 external URLs in the book or the decisions — sources are named in
  prose, so a claim is found by the source's **name**; 12 files name one):
  - the ledger is a book chapter, `docs/book/src/ledger.md` — one section per source, id as its heading, and a
    fixed field table: version, where it is pinned, retrieved (absolute date), hash or "not captured",
    scope, known limitations, revalidation trigger, and the names documents use for it;
  - **PINS**, derived and never listed: every gitlink, every `*_VERSION_PINNED` in a tracked `targets/*.env`,
    `DOCTRINE_VERSION`, `rust-toolchain.toml`'s channel and every CI `uses:` ref is claimed by exactly one row
    whose version carries the pinned value — so a pin that moves without its row is refused, and a row whose
    pin is gone is stale;
  - **CITATION**: every book chapter and decision record that names a ledgered source cites its row
    (`ledger.md#<id>`), and every such citation resolves;
  - **ROWS**: every field present, dates absolute, no version given as "latest" (§19);
  - a `--self-test` with RED arms; ⚠️ honest limit — a claim about a source the ledger does not name at all is
    not seen; that residue is review.
  Verification: see the checklist — a ledger chapter of 11 entries, a gate over five kinds of pin and every naming
  document (20 arms, 15 mutations, two falsifications on the real tree), 21 citations added across 12 files.
  Commit: `ARCHOGEN-PROGRAM-0126 (leaf PROGRAM.5)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — no ledger, and nothing a ledger could hang from:
    ```text
    $ git grep -lE "https?://" cb8180d -- docs/book/src docs/decisions        -> no match, rc=1
    $ git grep -n "ledger\.md" cb8180d -- docs/book/src docs/decisions        -> no match, rc=1
    $ git grep -n 'channel = \|uses: ' cb8180d -- rust-toolchain.toml .github/workflows
      cb8180d:.github/workflows/rust.yml:14:      - uses: dtolnay/rust-toolchain@stable
      cb8180d:rust-toolchain.toml:2:channel = "stable"      (and actions/checkout@v4 twice)
    ```
    External sources were named in prose only — QEMU, LinkedSpec, RGX, chipdoc, bedrock, semulith, Cargo,
    mdBook, Miri — in **12** files, with **21** uncited mentions once the gate could see them.
  - [x] **ROOT CAUSE (WHY + WHERE)** — §15's ledger was a roadmap sentence with no file and no check, so a
    version lived wherever a sentence happened to state it (`decision_emulator-independence-retained.md`
    "QEMU 11.1.1", the feedback register's commits), and a pin could move without any of them moving:
    ```text
    $ git grep -n "QEMU 11.1.1" cb8180d -- docs/decisions
      cb8180d:docs/decisions/decision_emulator-independence-retained.md:41:QEMU 11.1.1 with the pinned options gives:
    ```
  - [x] **FIX** — `docs/book/src/ledger.md`, "What this project relies on from outside": 11 entries (`qemu`,
    `linkedspec`, `rgx`, `pgen`, `chipdoc`, `bedrock`, `semulith`, `rust-toolchain`, `mdbook`,
    `github-actions`, `miri`), each with version, pin locator, retrieval date, hash or "not captured", scope,
    limits, revalidation trigger and names. A QEMU binary digest captured. ⚠️ `chipdoc`'s revision was never
    recorded when it was read, and its entry says so. New doctrine **`SOURCE-LEDGER`**
    (`scripts/check_source_ledger.sh`) with the three legs above; 21 citations added (inline in chapters, an
    **External sources** header bullet in decision records).
  - [x] **ADDRESSED (verified)** —
    ```text
    $ bash scripts/check_source_ledger.sh
      source-ledger: OK (11 entries; 7 pin(s), each carried by its entry; every naming document cites it)   exit=0  (0.44 s)
    $ bash scripts/check_source_ledger.sh --self-test
      source-ledger self-test: 20 pass / 0 fail (20 arms)
    $ mdbook build docs/book  -> exit=0; all 11 ids present as anchors in ledger.html (mdBook v0.5.2)
    ```
    Every pin kind has an arm (gitlink via a synthetic index entry, env, file, toml, uses). Fifteen mutations
    M-1–M-15, each restored and checked by `cmp`, and each fails its own arm. On the **real tree**: moving
    `QEMU_VERSION_PINNED` to `11.2.0` → refused naming entry `qemu`; deleting one decision's citation →
    refused at `decision_findings-for-director-review.md:33`; both restored by `cmp`. One false positive met
    and fixed on the way: a `` `Cargo.*` `` glob read as a mention of Cargo — a `.` now ends a sentence only
    before a space or the line end. First cut took 4.5 s (a process per lookup); one awk pass per document
    brought it to 0.44 s with the arms unchanged.
  - [x] **NO REGRESSION** — `BOOK-ANCHORS` exit=0 over the new chapter; `SCRATCH-LOCALITY` green over the new
    script; the doctrine driver green at the commit; `make focused` passed.
  - [x] **LOCKSTEP** — the chapter in `SUMMARY.md`; `verification.md` gains "What comes from outside is written
    down" and links its tools' entries; `DOCTRINE_ENFORCEMENT.md` and `TOOLBOX.md` rows; the live docs.
    **`PROGRAM.30` filed** for what deriving the pins exposed: the Rust channel, mdBook and the CI actions are
    not pinned to exact versions.
