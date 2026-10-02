- ID: `API.6`
  Status: `done` — `2026-10-02`; its six children done, and the integration tier passed
  Children: `API.6.1` … `API.6.6`
  Goal: the **MCP server** — any agent drives a running archogen instance through it.
  Acceptance: the tool list is derived from `crates/archogen-cli/src/spec.rs`, so a documented operation
  is always an offered one and an unimplemented one names its owning leaf instead of failing at runtime;
  the three-state `Maturity` is visible in capability discovery, so a consumer learns at the handshake
  that `resolve` does not exist yet rather than after calling it; every response carries §5.5's verdict;
  neither build is exposed, and a test asserts that rather than leaving it to convention; any external
  dependency arrives with a decision record naming its §4.4 trust category and the claims its compromise
  would invalidate, and is reachable from the transport and **not** from the generator or the checker;
  the book documents it; `make integration` exit `0` or naming what is incomplete.
  Priority: **the point of the tree**, and last because everything above is what makes it safe.
  ⚠️ Read at the primary source before designing, `2026-10-01`: MCP's current revision, `2026-07-28`, has no
  `initialize` handshake — every request carries its version and capabilities in `_meta`, and `server/discover` is
  mandatory — and a legacy client cannot reach a server that speaks only it. The design answers both eras
  ([`decision_mcp-server.md`](../decisions/decision_mcp-server.md)).
  Verification: each clause of the acceptance, `2026-10-02`. **The tool list** is `spec::tools()`, the command table
  less its exclusions (`the_tool_list_is_the_command_tables_less_its_exclusions`), and a tool not built answers with
  the leaf that owns it (`a_tool_that_is_not_built_answers_with_the_leaf_that_owns_it`). **Maturity at discovery**:
  each tool carries its maturity and owner in its `_meta`, which the same test reads. **The verdict**: `check` answers the wasm
  binding's bytes, `status` and `exit` included, and the command line's verdict on every single-file conformance
  case in both revisions (`every_verdict_is_the_command_lines_for_the_same_description`). **Neither build exposed**:
  `neither_build_nor_verify_is_offered_programmatically` reads the table, and `a_call_that_is_no_call_is_a_protocol_error`
  calls `build`, `verify` and `mcp` and is refused each time. **No external dependency**: the server is built from
  this workspace's crates alone, its JSON reader its own (`API.6.3`), so no §4.4 trust category arises. **The book**:
  *The server an agent spawns* in `engine-api.md`, its transcript a real run (`API.6.5`). **`make integration`**:
  exit `0`, 11 passed, 0 failed, 0 unavailable, 0 not built, 0 quarantined.
  Commit: `ARCHOGEN-API-0341 (leaf API.6)`

- ID: `API.6.1`
  Status: `done` — `2026-10-01`
  Goal: the server's design recorded and the protocol it implements ledgered, before any code.
  Acceptance: a decision record answering the protocol revision, the eras, the transport, how the tool list derives
  from `spec.rs` with the builds excluded, what a tool takes and returns, `isError` against the verdict, the input
  bounds and the dependency rule; the specification's ledger entry with what was read, its commit and its hashes.
  **Done.** [`decision_mcp-server.md`](../decisions/decision_mcp-server.md) and its index row; the ledger's
  `mcp-specification` entry, both schemas pinned by commit `3098fe9` and sha256; the four documents that named MCP
  now cite it (`source-ledger: OK (19 entries …)`).
  Verification: `bash scripts/check_source_ledger.sh` → rc=0; `bash scripts/check_doctrines.sh` → all green
  Commit: `ARCHOGEN-API-0293 (leaf API.6.1)`

- ID: `API.6.2`
  Status: `done` — `2026-10-01`
  Goal: each command in `crates/archogen-cli/src/spec.rs` declares whether it is offered programmatically, and the
  tool list derives from that: `check` a tool; `resolve`, `analyze`, `explain`, `replay` tools that name their
  owners; `build`, `verify` and the server's own command excluded, each with its reason.
  Acceptance: a test that the builds and `verify` are excluded, read from the table; the tool list, each tool's
  `_meta` and annotations derived, not written; `archogen --help` unchanged but for the new command.
  **Done in part, by design:** the exposure and the list. The tool objects — `_meta`, annotations, input schemas —
  are built where they are sent, in `API.6.4`, from this list; and the server's own command, which would change
  §10.2's seven, lands with the server and its §10.4 sentence.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — no command said whether it is offered: `git show HEAD:crates/archogen-cli/src/spec.rs
    | grep -c "exposure"` → 0; the builds' exclusion lived in prose (§10.4), where a tool list could miss it.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `CommandSpec` carried `Maturity` only: `git show HEAD:crates/archogen-cli/
    src/spec.rs | grep -c "pub maturity: Maturity"` → 1 and no other per-command state, so the §10.4 rulings had no
    place in the table §10.4 says the tool list derives from. WHERE: `crates/archogen-cli/src/spec.rs`.
  - [x] **FIX** — `Exposure { Tool, Excluded { reason } }` on every command, `build` and `verify` excluded citing
    §10.4; `tools()`, the offered commands in table order; two tests reading the exclusions from the table.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --lib` → `30 passed`; with `build` flipped to
    `Tool`, `neither_build_nor_verify_is_offered_programmatically` → `FAILED. 29 passed; 1 failed`.
  - [x] **NO REGRESSION** — `cargo test --all -q` → 958 passed, 0 failed; `cargo clippy -q -p archogen-cli
    --all-targets -- -D warnings` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — this leaf, the frontier and the log; the book's account is `API.6.5`'s, with the server.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0294 (leaf API.6.2)`

- ID: `API.6.3`
  Status: `done` — `2026-10-02`
  Goal: the server's own JSON reader: bounded nesting and size, refusing every malformed text, and a writer with one
  way to write each value.
  Acceptance: RED arms for each refusal and both edges of each bound; a round trip of every message the server
  sends; no dependency.
  **Done.** `crates/archogen-cli/src/json.rs`: `read(bytes, limit)` refuses, each with the byte where it stopped, a
  text over its bound before reading it, bytes that are not UTF-8, a byte order mark, everything outside RFC 8259's
  grammar, an escaped lone surrogate, a name given twice in one object, and nesting deeper than `MAX_DEPTH` (64),
  recursing no deeper than that; a number keeps its text, so a request's `id` is answered as it came. `write` has
  one way to write each value, a string as the wasm binding writes one. The messages the server will send are
  `API.6.4`'s, so the round trip here is of their shapes, as `docs/decisions/decision_mcp-server.md` gives them, and
  of 2 000 generated values; `API.6.4`'s acceptance takes the round trip of the messages it actually sends.
  **Found on the way:** `crates/archogen-cli/src/spec.rs` had been committed unformatted by `ARCHOGEN-API-0294`, so
  the `focused` tier's `fmt` step failed from that commit; no pre-commit check runs `cargo fmt`. Formatted here.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the server had no reader: `git ls-files crates/archogen-cli/src | grep -c json` → 0;
    and `git show HEAD:crates/archogen-cli/src/spec.rs > spec_head.rs; rustfmt --check --edition 2021 spec_head.rs`
    → rc=1.
  - [x] **ROOT CAUSE (WHY + WHERE)** — the record's §5 and §6 ask for the server's own bounded reader, with no
    dependency, and nothing in the workspace reads JSON: the wasm binding's requests are length-framed
    (`git show HEAD:docs/decisions/decision_wasm-binding.md | grep -c "is length-framed, not JSON"` → 1). WHERE:
    `crates/archogen-cli/src/`.
  - [x] **FIX** — `json.rs`: `Value`, `Number`, `read`, `write`, `Refusal`; `pub mod json` in `lib.rs`; `spec.rs`
    formatted.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --lib json` → `14 passed; 0 failed`; three mutants
    each red: the duplicate check removed → `FAILED. 12 passed; 2 failed`, the depth bound one higher → `FAILED. 13
    passed; 1 failed`, the leading-zero rule removed → `FAILED. 13 passed; 1 failed`.
  - [x] **NO REGRESSION** — `cargo xtask verify --tier focused` → `tier focused: passed — 3 passed, 0 failed`;
    `cargo clippy -q -p archogen-cli --all-targets -- -D warnings` → rc=0; `bash scripts/check_doctrines.sh` →
    `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — this leaf, `API.6.4`'s acceptance, the frontier and the log; `CHANGELOG.md`. The book's
    account of the server is `API.6.5`'s.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0332 (leaf API.6.3)`

- ID: `API.6.4`
  Status: `done` — `2026-10-02`
  Goal: the server, `archogen mcp`: `server/discover`, `tools/list`, `tools/call` under `2026-07-28`'s per-request
  `_meta`; `initialize`, `ping`, `tools/list`, `tools/call` under `2025-11-25`; the errors the record names; exit at
  the end of stdin.
  Acceptance: an integration test that spawns the built binary and speaks both eras over its stdio, a refusal and
  an unimplemented tool among them; every result's verdict checked against the CLI's for the same description;
  `NO-SUBPROCESS` green; every message it sends read back by `API.6.3`'s reader and written to the same bytes.
  **Done.** `crates/archogen-cli/src/mcp.rs` and `archogen mcp`: one message per line, both eras as the record
  decides — `server/discover`, `tools/list` and `tools/call` per request under `2026-07-28` (`-32602` for a missing
  `_meta` key, `-32022` with `supported` and `requested` for another version), and `initialize`, `ping`,
  `tools/list` and `tools/call` under `2025-11-25`. The tools are `spec::tools()`; `check` answers the wasm
  binding's bytes as `structuredContent` and its text block, `isError` false only on acceptance; an unbuilt tool
  names its leaf. Read at the pinned schemas (sha256 `742750af…`, `e74b56e7…`, both matching the ledger) and the
  `2025-11-25` lifecycle: the namespaced `_meta` keys, `resultType`, `ttlMs` and `cacheScope`, a `ping` allowed
  before `initialize`, `2026-07-28` having no `ping`; the record's *As built* states each choice. `ROADMAP.md` §10.4
  names the command, `spec.rs` lists it as no tool, and the surface test reads §10.4 for it.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — no server: `git show HEAD:crates/archogen-cli/src/lib.rs | grep -c "pub mod mcp"` →
    0, and `archogen mcp` was `usage: unknown command`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `API.6.1`–`.3` gave the design, the tool list and the reader; nothing served
    them: `git show HEAD:crates/archogen-cli/src/spec.rs | grep -c '"mcp"'` → 0. WHERE: `crates/archogen-cli/src/`.
  - [x] **FIX** — `mcp.rs`; `run_with_input` and the `mcp` arm; `main.rs` passing stdin; `mcp` in `spec.rs`,
    excluded with its reason; `archogen-wasm` a dependency, for the response's encoding; `api_parity.rs`'s
    exclusions; §10.4's sentence; the record's *As built*; the book's command table and help transcript.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --lib mcp` → `12 passed`; `--test mcp_stdio` → `2
    passed`, the built binary spoken to in both eras and its verdict the command line's on all 36 single-file cases;
    three mutants red — `isError` inverted → `11 passed; 1 failed` and the stdio suite `0 passed; 2 failed`, the
    version check removed → `11 passed; 1 failed`, `resultType` dropped → `11 passed; 1 failed`.
  - [x] **NO REGRESSION** — `cargo xtask verify --tier focused` → `tier focused: passed — 3 passed, 0 failed`;
    `cargo test -q --workspace` → 986 passed, 0 failed; `bash scripts/check_doctrines.sh` → `=== all doctrines green
    ===`, `NO-SUBPROCESS` among them.
  - [x] **LOCKSTEP** — `ROADMAP.md` §10.4; the record; `docs/book/src/cli.md`; this leaf, the frontier and the log;
    `CHANGELOG.md`. The server's own chapter and its review are `API.6.5`'s.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0333 (leaf API.6.4)`

- ID: `API.6.5`
  Status: `done` — `2026-10-02`; the book written (`-0335`), the review answered (`-0337`), its D3 filed as `API.6.6`
  Goal: the book documents the server, and a context that did not write it reviews it against the specification.
  **The book (`-0335`).** *The engine API* gains *The server an agent spawns*: how an agent starts and speaks to
  `archogen mcp`, both eras, what `check` takes and answers, an unbuilt tool and a refused version; its transcript is
  a real run, which `crates/archogen-cli/tests/mcp_stdio.rs` replays against the built binary and compares byte for
  byte, red with one character of an answer changed. Its three sentences that said the server was not built are
  corrected.

  **Acceptance checklist, the book (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the chapter said the server was not built after it was:
    `git show HEAD:docs/book/src/engine-api.md | grep -c "It is not built yet"` → 1.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `API.6.4` changed the code and the command-line chapter but left the engine
    API's chapter, where the server is described, to this leaf: `git show HEAD:docs/book/src/engine-api.md | grep -c
    "mcp-transcript"` → 0. WHERE: `docs/book/src/engine-api.md`.
  - [x] **FIX** — the section and its transcript; the three sentences corrected; the replay test in
    `crates/archogen-cli/tests/mcp_stdio.rs`.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --test mcp_stdio` → `3 passed`; with `M3.4` changed
    to `M3.5` in the book's answer line, the replay → `FAILED. 0 passed; 1 failed`.
  - [x] **NO REGRESSION** — `cargo clippy -q -p archogen-cli --all-targets -- -D warnings` → rc=0; `bash
    scripts/build_book.sh` → rc=0; `bash scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the chapter, the index; this leaf and the log.

  **The review (`-0337`).** A context that had not written the server read it against the pinned specification and
  attacked it — a 30 000-line fuzz, RFC 8259 vectors, hostile sizes — and found 7 defects, 10 drafting points and 8
  nits (the protocol's shapes conformant, the input bounds not). Answered: **D1** the duplicate check a set lookup,
  linear (an object of 238 000 members had taken 81 s); **D2** the line bound six times the budget, since a JSON
  writer may spell any byte as six (Python's `json.dumps` wrote a 0.8 MB description as 2.4 MB, refused); **D4** an
  unreadable request answered with no id, never `null`, which neither schema allows; **D5** a request's id a string
  or an integral number; **D6** an argument `check` cannot use a tool error with what to change, as both revisions'
  tools pages ask of "Input validation errors", read at the commit (`-32602` kept for no tool, an unknown tool or
  arguments that are no object); **D7** the test the code cited, written over the whole suite and hostile texts.
  **D3**, a 1 MB description of unclosed forms answered in 1.5 GB because each diagnostic quotes its one long line —
  the command line too — is the engine's renderer's, filed as `API.6.6`. The drafting points and nits: the record's
  §4, §5 and *As built* corrected (the line bound, the ping before `initialize` a leniency, `resultType` only under
  `2026-07-28`); the pages read at the commit ledgered with their hashes; a version that is no string named so; a
  `2026-07-28` method in a `2025-11-25` session not found; an unbuilt tool's description saying so; the suite run in
  both revisions; a built tool without a handler `-32603`; client responses ignored, blank lines skipped; the stdio
  test's input on its own thread; `json.rs`'s departure from RFC 8259 §9 stated. Chosen and kept: U+2028 unescaped,
  and a per-request `2025-11-25` refused with `-32022` (the record says why).

  **Acceptance checklist, the review (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — the reviewer's runs, each reproduced as a test against a mutant restoring the flaw:
    the quadratic check → `a_wide_object_is_read_in_time_linear_in_its_size` `FAILED` after 30.09 s; the 2× bound
    → `FAILED. 64 passed; 1 failed`; `id: null` restored → `FAILED. 64 passed; 1 failed`; `1.5` accepted → `FAILED.
    64 passed; 1 failed`.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `members.iter().any(…)` per name in `json.rs`; a line bound reasoned from
    `\n` alone; `error` writing `id.clone()` of a `Value::Null`: `git show HEAD:crates/archogen-cli/src/mcp.rs | grep
    -c "2 \* archogen_api::DEFAULT_BYTES"` → 1. WHERE: `crates/archogen-cli/src/json.rs`, `mcp.rs`.
  - [x] **FIX** — as above; `Failure` separating `-32602` from `-32603`; `integral`; the record, the ledger, the book.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p archogen-cli --lib` → `65 passed`; `--test mcp_stdio` → `3
    passed`, the suite now in both revisions; the four mutants above red, the restored code green.
  - [x] **NO REGRESSION** — `cargo xtask verify --tier focused` → `tier focused: passed — 3 passed, 0 failed`;
    `cargo test -q --workspace` → 996 passed, 0 failed; `bash scripts/check_doctrines.sh` → `=== all doctrines green
    ===`.
  - [x] **LOCKSTEP** — the record, the ledger's `mcp-specification`, the book's section; `API.6.6` filed; this leaf,
    the frontier and the log; `CHANGELOG.md`.
  Verification: see the checklists.
  Commit: `ARCHOGEN-API-0335 (leaf API.6.5)` for the book; `ARCHOGEN-API-0337 (leaf API.6.5)` for the review

- ID: `API.6.6`
  Status: `done` — `2026-10-02`
  Goal: an answer as bounded as its request. A description of a million unclosed forms is answered — by
  `archogen check`, the engine API and `archogen mcp` alike — with diagnostics that each quote its one long line:
  1.5 GB and 4.8 GB of memory for a 1 MB request (`API.6.5`'s review, D3). The renderer quotes a window of a long
  line around its span, so an answer's size is linear in its request's.
  Acceptance: a RED arm with the reviewer's input; the window's edges; every transcript the book shows unchanged.
  **Done.** `crates/eadl-front/src/diagnostic.rs`: a line longer than `EXCERPT` (160) characters is quoted as a
  window of that many around the span, 40 before it, cut with `…`, the caret moved with it and never past the
  window; a shorter line is quoted as before, so no transcript moved. The reviewer's 1 MB line, measured on a release
  build: `archogen check` from 770 MB of output and 1.5 GB of memory to 0.25 MB and 7 MB; `archogen mcp` from a
  1.5 GB answer and 4.8 GB of memory to 0.57 MB and 10 MB.

  **Acceptance checklist (`DOCTRINE_ENFORCEMENT.md`):**
  - [x] **REPRODUCE / ISSUE** — with the window disabled, `cargo test -q -p archogen-api --test check an_answer` →
    `FAILED. 0 passed; 1 failed`, a 100 000-character line of `(` rendered far past four times its size.
  - [x] **ROOT CAUSE (WHY + WHERE)** — `render_label` printed `source.line_text(position.line)` whole, and an
    indent to the column, for every diagnostic: `git show HEAD:crates/eadl-front/src/diagnostic.rs | grep -c "let
    line_text = source.line_text(position.line);"` → 1. WHERE: `crates/eadl-front/src/diagnostic.rs`.
  - [x] **FIX** — `excerpt`, `EXCERPT`; the caret bounded by the window; the book's *What one request may cost* and
    the record's §5.
  - [x] **ADDRESSED (verified)** — `cargo test -q -p eadl-front --lib` → `77 passed`, the window's edges and the
    caret at a long line's start, middle and end; `-p archogen-api --test check` → `11 passed`; the window disabled
    → `FAILED. 75 passed; 2 failed` and the API test red.
  - [x] **NO REGRESSION** — every transcript test green, the book's unchanged; `cargo xtask verify --tier focused`
    → `tier focused: passed — 3 passed, 0 failed`; `cargo test -q --workspace` → 999 passed, 0 failed; `bash
    scripts/check_doctrines.sh` → `=== all doctrines green ===`.
  - [x] **LOCKSTEP** — the book, the record, the figure register; this leaf, the frontier and the log;
    `CHANGELOG.md`.
  Verification: see the checklist.
  Commit: `ARCHOGEN-API-0338 (leaf API.6.6)`
