- ID: `API.5`
  Status: `done`
  Children: `API.5.1`, `API.5.2`, `API.5.3`, `API.5.4`, `API.5.5` — decomposed `2026-09-30`: the design decided
  first, then the crate tested on the host, then the artifact checked against the CLI, then the page and the book,
  then the page run in a real browser
  ⚠️ From `API.1`'s measurement: the crates this binding needs most do I/O today — `eadl-front`'s module loader reads
  files through `DirectoryModules` (the `ModuleSource` trait is the seam to supply another), and `archogen-s0`'s
  emitter writes the generated crate to a directory. Both compile for wasm32; neither would work there as written.
  Goal: the **wasm binding** over the declared API, so the description-side toolchain runs in a browser
  or a worker.
  Acceptance: the API is reachable from `wasm32-unknown-unknown` as an artifact a page can load; a
  worked example checks a real description in a browser and shows the verdict; no filesystem, no
  subprocess and no ambient authority is required, and a test asserts the binding does not reach for one;
  the book documents it; `make integration` exit `0` or naming what is incomplete.
  Priority: **medium** — behind `API.1` (which measures whether this is possible at all) and `API.3`
  (which decides what it exposes).
  Verification: through its five children:
  - the design (`API.5.1`);
  - the host tests, and the same under Miri (`API.5.2`);
  - the artifact answering all 108 tracked descriptions byte for byte as the host build and the CLI do, in the
    integration tier (`API.5.3`);
  - the page and the book's transcript, reproduced by the tier (`API.5.4`);
  - the director's run in Chrome 154 (`API.5.5`).
  Commit: `ARCHOGEN-API-0220` closes it; its children's commits are in the log below.

- ID: `API.5.1`
  Status: `done`
  Goal: the binding decided in a durable record: where it lives, what it exports, how a request crosses into the
  module and a response out, the response's versioned format, who sets the budget, what "no ambient authority"
  means as something a test can check, and what the binding is checked against.
  Acceptance: `docs/decisions/decision_wasm-binding.md`; no dependency and no `unsafe`, or a stated reason for
  either; nothing left to the implementation that changes what a page receives.
  Verification: the record, with its two facts about the toolchain measured on a throwaway `cdylib` built under
  `target/` for `wasm32-unknown-unknown` with the pinned rustc 1.95.0, then removed:
  `WebAssembly.Module.imports` → `[]`; `WebAssembly.Module.exports` → `memory`, the functions, `__data_end` and
  `__heap_base`; and `#[no_mangle]` under `deny(unsafe_code)` → `error: declaration of a no_mangle function`, so the
  record states that exception rather than promising no unsafe code at all. The module-file rule the harness
  must reproduce was read from `eadl-front` (`DirectoryModules::file_for` → `<dir>/<module>.eadl`).
  Commit: `ARCHOGEN-API-0201 (leaf API.5.1)`

- ID: `API.5.2`
  Status: `done`
  Goal: `crates/archogen-wasm` — the exported functions, the request framing and the response encoder — tested on
  the host, where no wasm toolchain is needed.
  Acceptance: every framing refusal constructible; the encoder's output parsed back by an independent JSON reader
  and equal field by field to the API's response; the book-coverage, version-register and wasm-build gates green.
  Verification: see the checklist — 8 tests on the host and under Miri, two catalogued mutations, the shape frozen.
  Commit: `ARCHOGEN-API-0202 (leaf API.5.2)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — no transport existed: `git ls-files crates/archogen-wasm` → nothing, and
    `git grep -n "RESPONSE_FORMAT\|archogen_check" -- crates` → no match before this leaf. The engine API returns
    structure, and `decision_engine-api.md` §4 leaves its serialization to a transport crate.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: between `archogen_api::check_with` and a page there was no byte
    format in either direction; `decision_wasm-binding.md` §4 and §6 now define both. WHY each piece has its own
    test, measured on this leaf's own first run rather than assumed: the first blessed shape
    (`ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-wasm --test binding` → `test result: ok. 8 passed`, then the
    golden read back) listed `diagnostics[].secondary: array` and no `diagnostics[].secondary[]` path, because none
    of the four cases produced a secondary label, so the golden would not have pinned that label's shape. A case
    that does, a name declared twice (`archogen check` shows `first declared here`), was added, and the uncommitted
    golden was deleted and blessed again with the six `secondary[]` paths.
  - [x] **FIX** — `src/request.rs` reads and writes the framing and refuses each §4 breach with its field and byte
    offset; `src/json.rs` writes §6's encoding; `src/lib.rs` holds the two format constants, `INPUT_CAP`, the pure
    `answer`, and the three `#[no_mangle]` exports over thread-local buffers, with `deny(unsafe_code)` and no
    `unsafe` block. `tests/binding.rs` holds a strict RFC 8259 reader written for the purpose, the field-by-field
    comparison with the API's `Response` over five cases (accepted; refused; not judged; a secondary label; a
    module tree read from `docs/semantics/modules`), the exact escaping of every character below `0x20`, every
    framing refusal, the exports driven with raw writes as the loader drives them, and the shape golden.
  - [x] **ADDRESSED (verified)** — `cargo test -p archogen-wasm` → `test result: ok. 8 passed; 0 failed`. The
    golden was blessed once, deliberately (`ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-wasm --test binding`),
    and passes unblessed. `cargo +nightly miri test -p archogen-wasm` → `test result: ok. 8 passed; 0 failed`, the
    raw-write exports test among them, rc=0. The book's JSON excerpt equals the binding's real answer for the same
    description. Mutations:
    ```text
    cargo xtask mutate --only wasm-json-long-escape               → killed by every_control_character_…
    cargo xtask mutate --only wasm-framing-accepts-a-module-twice → killed by every_framing_refusal_…
    ```
  - [x] **NO REGRESSION** — `make focused` → `tier focused: passed — 3 passed, 0 failed`, rc=0;
    `bash scripts/wasm_build.sh` → rc=0, `archogen-wasm` compiled for `wasm32-unknown-unknown` with the pure set;
    `bash scripts/check_no_subprocess.sh` → `51 production source file(s)`, OK; `check_version_register.sh` →
    `10 entries; 10 declared version(s)`; `check_book_coverage.sh` → `11 workspace member(s)`, OK.
  - [x] **LOCKSTEP** — `docs/book/src/engine-api.md` gains "The binding a web page will load";
    `docs/book/src/versions.md` gains `wasm-request-format` and `wasm-response-format`; `xtask/mutations.txt` gains
    two entries; `Cargo.lock` gains the package; this leaf, the frontier, both logs and `CHANGELOG.md`.

- ID: `API.5.3`
  Status: `done`
  Goal: the artifact — built for `wasm32-unknown-unknown` in a tier step, its imports and exports checked by the
  platform's own `WebAssembly.Module`, and every tracked description run through it and compared with the CLI.
  Acceptance: an artifact with no imports and exactly the decided exports; for the whole population, JSON
  byte-identical to the host build's and an `exit` equal to `archogen check`'s (`decision_wasm-binding.md` §8); a
  JavaScript runtime that is absent makes the step unavailable, never passed; that runtime gets a ledger entry at
  the version the step uses, since the tier then relies on it.
  Verification: see the checklist — 108 descriptions, six exit statuses among them, agreeing both ways; six RED
  arms and a broken loader refused on the real tree; the integration tier 11 of 11.
  Commit: `ARCHOGEN-API-0203 (leaf API.5.3)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — after `API.5.2` the module had only been compiled, never run: `wasm-build`
    compiles the pure set and executes nothing, and `git ls-files 'crates/archogen-wasm/js/*' 'scripts/wasm_binding*'`
    → nothing. Whether the artifact imports nothing, and answers as the host build does, was unmeasured.
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE: the three things only the artifact can show are the compilation to
    WebAssembly, the transport through linear memory, and the loader. WHY the comparison is two-sided: the byte
    comparison with the host build isolates exactly those three, and `archogen check`'s exit code ties the answer
    to the command line, which reaches the same API through a directory source. The harness reproduces that
    source's rule, `DirectoryModules::file_for` → `<dir>/<module>.eadl`, read from `eadl-front`, so both sides ask
    the same question. Measured on the first run: `bash scripts/wasm_binding.sh` → rc=0 over the population,
    whose exit codes span six statuses (`cut -d, -f5` of the responses: 34 × `0`, 57 × `10`, 3 × `11`, 8 × `12`,
    4 × `13`, 2 × `20`).
  - [x] **FIX** — `crates/archogen-wasm/js/archogen.mjs`, the loader: it frames the request, reads addresses
    unsigned (`>>> 0`, since wasm32's `usize` reaches JavaScript as a signed `i32`), and takes a new view of memory
    after every call, since a call that grows memory detaches older views. `crates/archogen-wasm/examples/answers.rs`,
    the host side. `scripts/wasm_binding.mjs`, which inspects the artifact with `WebAssembly.Module` and answers
    through the loader. `scripts/wasm_binding.sh`, which builds the artifact, derives the population
    (`git ls-files '*.eadl'` outside `docs/feedback/`), compares the three sides, and holds six RED arms. The
    `wasm-binding` step of `integration`, requiring `node`. The ledger's `node` entry.
  - [x] **ADDRESSED (verified)** — `bash scripts/wasm_binding.sh` → `OK — the artifact imports nothing and exports
    what the record lists; 108 description(s) answered byte for byte as the host build answers them, each with
    archogen check's exit code (v26.8.1)`, rc=0. RED on the real tree: the loader edited to send a profile nobody
    supports → rc=1, all 108 descriptions named as differing, then restored (`cmp`) → rc=0.
    `bash scripts/wasm_binding.sh --self-test` → `6 pass / 0 fail (6 arms)`: agreeing sides pass; a differing
    response, a differing exit and a missing answer are each refused naming the description; a hand-written module
    importing `env.f`, and one exporting `x` and none of the record's functions, are each refused.
  - [x] **NO REGRESSION** — `cargo xtask verify --tier integration` → `tier integration: passed — 11 passed, 0 failed,
    0 unavailable`, the new step and `self-tests` among them; `cargo test -q -p archogen-cli --test book_transcripts`
    → `6 passed`; `cargo test -q -p eadl-front --test reference` → `55 passed`; `check_source_ledger.sh` →
    `13 entries`, OK; `check_book_anchors.sh` → OK.
  - [x] **LOCKSTEP** — `docs/book/src/verification.md`: the tier's transcript re-rendered from this run, the
    browser section brought up to date, and "The browser module answers as the command line does";
    `docs/book/src/ledger.md`: `node` added, and `miri` revalidated, since its trigger, the first `unsafe` block in
    the workspace, fired with `API.5.2`'s test; `decision_wasm-binding.md` cites Node's entry; `TOOLBOX.md`; this
    leaf, the frontier, both logs and `CHANGELOG.md`.

- ID: `API.5.4`
  Status: `done`
  Goal: the page and the book — a page that loads the artifact, checks a description typed into it and shows the
  verdict; the book documents the binding and how to open the page.
  Acceptance: the page's loader is the one `API.5.3` checks; the chapter's transcript is reproduced by a test;
  `make integration` exit `0` or naming what is incomplete.
  Verification: see the checklist. ⚠️ The page has not been run in a browser: that is `API.5.5`, filed rather than
  claimed.
  Commit: `ARCHOGEN-API-0205 (leaf API.5.4)`

  ### Acceptance Checklist (enforced by `TASK-ACCEPTANCE`)

  - [x] **REPRODUCE / ISSUE** — the artifact existed and was checked, and no page loaded it:
    `git ls-files crates/archogen-wasm/page` → nothing, and the book said "the page and its instructions come last".
  - [x] **ROOT CAUSE (WHY + WHERE)** — WHERE the page can drift from what is checked: its request, its display and
    its loader. So all three are shared rather than restated. `page.mjs` exports `request` and `show`, which
    `node scripts/wasm_binding.mjs show` imports, and it loads the binding through `js/archogen.mjs`, which
    `API.5.3` checks. WHY the book's transcript shows an accepted description: a block holding an `error[` line is
    claimed by `crates/archogen-cli/tests/book_transcripts.rs`, whose backlog of unreproducible blocks may not grow,
    and a refused answer's diagnostics are already byte-identical to the command line's for every tracked
    description (`API.5.3`). `cargo test -q -p archogen-cli --test book_transcripts` → `6 passed` with the new
    section in place.
  - [x] **FIX** — `crates/archogen-wasm/page/index.html` and `page.mjs` (`request`, `show`, `wire`); the book's "The
    binding a web page loads" with "Opening the page" and a marked transcript; `scripts/wasm_binding.sh`'s fourth
    leg, which types the transcript's input into the page's logic and requires the chapter's answer, with five new
    RED arms, and its fifth, which runs the page's own wiring.
  - [x] **ADDRESSED (verified)** — `bash scripts/wasm_binding.sh` → rc=0, `… the book's page transcript is what the page
    shows`. RED on the real tree: the book's answer edited to `2 declaration(s)` → rc=1 with the difference shown,
    then restored → rc=0. `bash scripts/wasm_binding.sh --self-test` → `11 pass / 0 fail (11 arms)`. The page's
    own wiring is the harness's fifth leg (`node scripts/wasm_binding.mjs page`): `page.mjs` exports `wire`, which
    is run against a stand-in document whose `fetch` reads the file the page's relative URL names. It must fetch
    the module under test, show `index.html`'s default description's answer on load (`invalid-description
    (exit 10)` with its `parsec` diagnostic), and show the book's answer after Check. Its first run failed
    (rc=1, `ENOENT … open ''`): the harness's static import had loaded `page.mjs` before the stand-in existed, so
    the page never wired itself. Hence `wire` is exported and called, not triggered by a global. RED: the wiring
    edited to show raw JSON → rc=1, `on load the page showed: …`, then restored → rc=0.
    Served by the book's own command (`python3 -m http.server`, Python 3.14.7), `curl` → the page `200 text/html`,
    both modules `200 text/javascript`, the artifact `200 application/wasm`; the server was stopped afterwards.
  - [x] **NO REGRESSION** — `cargo xtask verify --tier integration` → `tier integration: passed — 11 passed, 0 failed,
    0 unavailable`; `cargo test -q -p eadl-front --test reference` → `55 passed`; `check_book_anchors.sh` → OK.
  - [x] **LOCKSTEP** — `docs/book/src/engine-api.md`; `TOOLBOX.md`'s browser-module row; this leaf, `API.5.5`, the
    frontier, both logs and `CHANGELOG.md`.

- ID: `API.5.5`
  Status: `done`
  Goal: the page run in a real browser, as `API.5`'s acceptance asks: "a worked example checks a real description in
  a browser and shows the verdict".
  Acceptance: the book's "Opening the page" followed in a browser; the answer on load is the default description's
  `invalid-description (exit 10)` with its `parsec` diagnostic; after Check with the book's description, the
  book's transcript; the browser's name and version recorded here.
  Unblocked by: a session with browser tools, or the director running those steps and reporting what the page shows.
  Observed `2026-09-30`, **by the director**, in a browser: after Check with the book's description, the page showed
  `ok (exit 0)` / `accepted against profile rt-static-up-v1 (eadl/1), 1 declaration(s)`. That is the book's
  transcript line for line (`docs/book/src/engine-api.md`, the block marked `wasm-page-transcript`). So the page
  loaded, fetched and instantiated the module, and answered in a real browser. **Not yet reported:** the answer
  shown on load, and the browser's name and version.
  Observed `2026-09-30`, **by the director**, the rest: on load the page showed `invalid-description (exit 10)`, in
  **Chrome `154.0.8037.58` (Official Build) (arm64)**. The director reported the status line and not the diagnostic
  under it. That line comes from the same response the page renders whole, and the harness's page leg checks it
  against a stand-in document on every integration run (`API.5.4`). So the browser run confirms that the module
  loads, answers on load and answers on Check in a real browser. The diagnostic's wording is the harness's to hold.
  Verification: the director's two observations in Chrome 154, against the book's "Opening the page" — the status on
  load and the full answer after Check, each as the book says.
  Commit: `ARCHOGEN-API-0209` (the Check leg), `ARCHOGEN-API-0220` (the load leg and the browser; the leaf closed)
