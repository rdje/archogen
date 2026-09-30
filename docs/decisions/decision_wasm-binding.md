# The wasm binding: three exports, a framed request and a versioned JSON response

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **External sources:** [the Rust toolchain](../book/src/ledger.md#rust-toolchain), for the `wasm32-unknown-unknown`
  target it already pins
- **Owner / source:** leaf `API.5.1` (`docs/tasks/API.md`), deciding the transport that
  [[decision_programmatic-interface]] names and [[decision_engine-api]] leaves open: "Serialization is a transport
  concern, and a dependency for it would live in the transport crate."

## The fact / decision

The binding is a transport crate over the engine API. It exports three functions, reads a length-framed request
and writes a versioned JSON response. It depends on nothing outside the workspace, holds no `unsafe` block, and
the module it builds imports nothing, so a page that loads it gives it no way to reach anything.

### 1. Where it lives

- **`crates/archogen-wasm`**, a library with `crate-type = ["cdylib", "rlib"]`. The `cdylib` is the artifact a
  page loads. The `rlib` lets the host's tests call the same functions, so the crate is tested without a wasm
  toolchain.
- It depends on `archogen-api`, and on `eadl-front` for `MemoryModules`, both by path and without features.
- It has no other dependency. `wasm-bindgen` is not used: its code generator would be a second program standing
  between the API and the page, and its trust category would need a record of its own. The interface is three
  functions, and the loader that calls them is short enough to read.

### 2. What it exports

Exactly three functions, and the module's linear memory:

| Export | Does |
| --- | --- |
| `archogen_input(len) -> address` | replaces the module's input buffer with `len` zero bytes and returns its address. When `len` is above the transport's cap, it allocates nothing and returns `0` |
| `archogen_check() -> length` | reads the input buffer as a request (§4), answers it (§5), writes the response (§6) into the output buffer, empties the input buffer, and returns the response's length in bytes |
| `archogen_output() -> address` | the output buffer's address, valid until the next call of any export |

- Sizes and addresses are `usize`, which is 32 bits on `wasm32`.
- A call goes: the page calls `archogen_input` with the request's length, writes the request at the address it
  gets back, calls `archogen_check`, and reads that many bytes at `archogen_output()`.
- **The transport's cap** is `INPUT_CAP`, four times `archogen_api::DEFAULT_BYTES`. It only stops an allocation
  that the API's budget would refuse anyway. The budget itself is §5's.
- **The linker adds two exports**, the globals `__data_end` and `__heap_base`. This was measured with the pinned
  `rustc` 1.95.0: a `cdylib` built for `wasm32-unknown-unknown` lists `memory`, its functions and those two globals
  under `WebAssembly.Module.exports`, and lists nothing under `WebAssembly.Module.imports`. They are addresses and
  carry no authority. `API.5.3`'s check admits exactly them and the four above.

### 3. No `unsafe` block

- The crate denies `unsafe_code`.
- Rust hands out the addresses of buffers it owns, which is a safe cast. The host writes and reads linear memory
  between calls, when no Rust borrow is live.
- **The lint's only exceptions are the three `#[no_mangle]` attributes**, each allowed where it is written. The
  compiler counts an unmangled symbol as unsafe code, because two of them could collide. This was measured:
  under `deny(unsafe_code)`, rustc 1.95.0 refuses each with "declaration of a `no_mangle` function".

### 4. The request

A request is length-framed, not JSON, so the module needs no parser for a format only its own loader writes.

- **A field** is a 4-byte little-endian length, then that many bytes of UTF-8.
- **A request** is, in order:
  1. a field holding exactly `archogen-wasm-request/1`;
  2. a field holding the description's name;
  3. a field holding its text;
  4. a field holding the profile's identifier, or an empty field for the API's default;
  5. the number of modules, as a 4-byte little-endian integer;
  6. for each module, a field holding its name, then a field holding its text.
- Nothing may follow the last module.
- **A request that breaks this is answered, not judged.** A length that runs past the end, bytes that are not
  UTF-8, another format, a module named twice, and bytes after the last module each get the status `usage`. The
  note names the field and its byte offset, and the description is not read.

### 5. The answer

- It is `archogen_api::check_with(request, Limits::DEFAULT)`, with the modules as a `MemoryModules`.
- **The instance sets the budget, never the page**, as [[decision_api-instance]] requires. A page cannot raise
  it. A request over it gets the API's own answer, `tool-failure`, naming the budget.
- A module is named `<module>.eadl` in spans, as `MemoryModules` names it.

### 6. The response

One JSON object (RFC 8259), in UTF-8, with its keys in this order:

```text
{"format":"archogen-wasm-response/1","api":"<major>.<minor>","engine":"<engine version>",
 "status":"<slug>","exit":<exit code>,"notes":["…"],"hint":"…" or null,
 "diagnostics":[{"code":"…","severity":"error" or "warning","message":"…",
                 "primary":<label>,"secondary":[<label>, …],"repair":"…"}, …],
 "rendered":"…",
 "judged":null or {"verdict":"<slug>","language":"…","profile":"…","declarations":<count>,
                   "instances":["…", …] or null,
                   "closure":{"inside":[["<fact>","<needed by>" or null], …],"outside":["…", …]}}}
```

- **A label** is `{"source":"<name>","line":<line>,"column":<column>,"message":"…"}`. Line and column are
  1-based, and the column counts characters, as `eadl_front::SourceMap` gives them for the span's start.
- `status` is `Status::slug`, and `exit` is `Status::code`. `verdict` is the verdict's slug.
- **`rendered` is `Response::render_diagnostics()`, byte for byte**, so a page shows exactly what the CLI prints
  for the same diagnostics.
- **The encoding is fixed**, so two builds of the same answer produce the same bytes:
  - there is no whitespace between tokens (the layout above is for reading only);
  - a string escapes `"` as `\"`, `\` as `\\`, and every character below `0x20`: as `\b`, `\t`, `\n`, `\f` or
    `\r` where one of those names it, and otherwise as `\u00XX` with lowercase hexadecimal digits. Nothing else
    is escaped, and other characters are written as their UTF-8 bytes;
  - an integer is decimal;
  - an absent value is `null`.
- `related_ids` and `profile` on a diagnostic are not carried, because nothing sets them yet. Carrying one later
  moves the format.

### 7. No ambient authority, as something a test checks

The artifact imports nothing: `WebAssembly.Module.imports` returns an empty list. A WebAssembly module can reach
outside itself only through its imports. So a module with none has no filesystem, clock, network or process,
whatever page or runtime instantiates it. `API.5.3` asserts this with the platform's own parser of the binary,
not with a reading of the source. That the crate names no `std::fs`, `std::process` or `std::net` is already
`scripts/wasm_build.sh`'s check of the pure set, which this crate joins.

### 8. What it is checked against

- **On the host (`API.5.2`):** the encoder's output is parsed by an independent JSON reader and compared with the
  API's `Response` field by field. Every framing refusal of §4 is constructed.
- **As an artifact (`API.5.3`):** the population is every tracked `*.eadl` outside `docs/feedback/`, with the
  modules of its own directory supplied the way the CLI's directory source finds them (`<dir>/<module>.eadl`). The
  artifact is driven by the loader a page uses. For each description:
  - its JSON is byte-identical to what the host build of the same crate writes for the same request;
  - its `exit` equals `archogen check`'s exit code for the same file.
  The first proves the compilation and the transport; the second ties the answer to the CLI, whose verdict comes
  from the same API.
- **A JavaScript runtime that is absent** makes the step unavailable, which the tier reports as incomplete (`20`)
  and never as passed.

### 9. The loader

- An ES module in the crate, `crates/archogen-wasm/js/archogen.mjs`. It exports `instantiate(bytes)`, whose result
  has `check({ name, text, profile, modules })`. `modules` maps a module's name to its text.
- It encodes the request, calls the three exports, and decodes the response with `JSON.parse`.
- It uses only `WebAssembly`, `TextEncoder` and `TextDecoder`, which browsers and Node both provide. The page
  (`API.5.4`) and the harness (`API.5.3`) load this one file, so what is checked is what a page runs.

### 10. Versions

- The request and response formats are `pub const`s of the crate, `archogen-wasm-request/1` and
  `archogen-wasm-response/1`, so the version register holds them (`VERSION-REGISTER`).
- A response also names the API version and the engine version, as every API response does.
- Any change to what §4 or §6 says moves the format's number, and the register's entry says what changed.

## Why

- **Three raw exports rather than generated bindings.** The workspace takes no dependency
  ([[decision_zero-dependency-engine-core]]), and the interface is small enough to state in a table. What would
  replace it is a generator whose output nobody here reviews.
- **A framed request rather than JSON.** Only the loader writes requests. A JSON parser in the module would be new
  code whose only job is to parse what our own code produced, and parsing it has failure modes that a length
  prefix does not.
- **A JSON response.** Any page parses it natively, and encoding it is total: every `Response` has exactly one
  encoding.
- **"No imports" as the authority test.** The platform checks it, not a reading of the source. A module without
  imports has nothing to call, whoever loads it.
- **Checked against the host build and the CLI.** The judgement is the API's, and it is already tested. What is
  new is the compilation to wasm, the transport and the loader, and a byte comparison of the same request both
  ways isolates exactly those.

## How to apply

- **Changing the request or the response:** change §4 or §6 here, move the format's number in the crate, and
  update the register's entry.
- **Adding an export, or any import:** that is a change to this record first. An import gives the module a way out,
  and §7's check refuses it until the record admits it.
- Related: [[decision_engine-api]], [[decision_api-instance]], [[decision_programmatic-interface]],
  [[decision_zero-dependency-engine-core]].
