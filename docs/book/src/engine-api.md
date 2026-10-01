# The engine API

The command line is how a person uses archogen. A browser, an editor, a build farm or an agent needs
to use it without a shell and without parsing prose, and `ROADMAP.md` §10.4 gives them the same
operations through one declared **engine API**. The CLI is its first consumer rather than a second
implementation, so a capability behind one is a capability behind the other.

The API is the library `archogen-api` (`crates/archogen-api/src/lib.rs`, leaf `API.3`). Its design is
recorded in `docs/decisions/decision_engine-api.md`. It reads no file, writes none, and runs nothing,
and it compiles for `wasm32-unknown-unknown` with the rest of the engine
([Verifying the toolchain](verification.md)).

## One operation: check

The API offers the operations the command line has built, which today is one: `check`, a
description as text plus a profile in, a structured response out. A `Request` holds:

| Field | What it is |
| --- | --- |
| `name` | how spans and notes name the description: a path for the CLI, any label for another consumer |
| `text` | the description itself |
| `profile` | the profile's identifier, or none for the default, `rt-static-up-v1` |
| `modules` | where an import resolves: a directory for the CLI, `MemoryModules` for a consumer with no filesystem, `NoModules` for a description that imports nothing |

A `Response` holds:

| Field | What it is |
| --- | --- |
| `version` | the API version that answered |
| `engine` | the engine version that answered: with `version`, the build the response came from |
| `status` | the outcome, always present |
| `diagnostics` | every finding, each with its code, the spans it points at, and a repair direction |
| `sources` | the texts those spans point into, so any consumer can render them |
| `notes`, `hint` | what the API says about a request it did not judge, and the repair |
| `judged` | present exactly when the description was judged: the verdict, the language version and profile it was judged under, the declarations it read, a module tree's instances, and the closure |

## One outcome vocabulary

The status is the exit contract the command line already has ([The `archogen` command
line](cli.md)). The CLI's exit code is that status's number, so the two cannot drift apart. When the
description was judged, the status is its `ROADMAP.md` §5.5 verdict. When it was not, the status says
why, and `notes` explain it:

- a profile nobody supports: `unsupported-profile`;
- a kind module, which nothing loads yet (leaf `M6.3`): `unimplemented`;
- an import that exists and cannot be read: `usage`, never `module-not-found`, which would be a
  false statement about the description;
- a language definition that fails to load, or a description too large to address: `tool-failure`.

A response is never missing its status, and a response that was not judged is never `ok`. The
failure §10.4 guards against is an unanswered question read as an answer, and it needs a response
with no outcome, or with a default one. This one has neither. ⚠️ The director's ruling says every
response carries a §5.5 verdict. §10.4 says "for what it reports". This chapter follows the second
reading, which is flagged for the director's call.

## An example that runs

`crates/archogen-api/examples/in_memory.rs` sends three requests, all held in memory: one accepted,
one judged and refused, and one the API does not judge.

```console
$ cargo run -q -p archogen-api --example in_memory
api 1.2 · engine 0.1.0 · status ok
  judged under eadl/1, profile rt-static-up-v1: 1 declaration(s)
api 1.2 · engine 0.1.0 · status invalid-description
  judged under eadl/1, profile rt-static-up-v1: 1 declaration(s)
  quantity-unknown-unit: `parsec` is not a known unit
api 1.2 · engine 0.1.0 · status unsupported-profile
  not judged: `rt-dynamic-mp` is not a supported profile
```

`crates/archogen-api/tests/book_example.rs` runs the example and compares its output with the block
above, so this page cannot go on showing output the API no longer gives.

## The version and what it promises

The API's version is `1.2`, apart from the language's (`eadl/1`) and the profile's, as §15 requires.
Every response names the API version and the engine version, and a judged one also names the language
version and the profile it was judged under. `1.0` was fixed when the API was declared. `1.1` added the
engine version, a new field, and `1.2` added `check_with`, below. Both are minors, as the promise says. Within a major, an operation is never removed, a response field
is never removed or given a new meaning, and the outcome vocabulary only grows. Adding an operation
or a field is a minor. A description whose verdict changes under the same language version is not an
API change: it is a language change, and it goes through the migration notes
([What is versioned](versions.md)).

## What an instance is

A consumer does not talk to a library. It talks to an **instance**: an [MCP](ledger.md#mcp-specification) server it spawned, a wasm module
a page loaded, or, for the shortest life, one run of the command line
(`docs/decisions/decision_api-instance.md`). An instance is bound to the build it came from: the engine
version, the API version, the language it reads, the profiles it supports, and the kind modules built
into it. It keeps nothing between requests. So a response depends on two things only: its request (the
description, the profile, and the module texts it loaded) and the build, which the response names.
Asking the same instance the same question twice gives the same answer, and
`crates/archogen-api/tests/check.rs` asks twice and compares the whole response.

## What one request may cost

The language already bounds the shapes that multiply work. A list nests at most 256 deep, and a module tree
has at most 1 024 instances and 16-module import chains ([Reading a description](reading.md),
[Modules and composition](modules.md)). What is left grows with the text, and in proportion to it: measured
on a debug build, 1 MB of description checked in 0.36 s and held 82 MB, and 5 MB took 1.95 s and 383 MB.

So an instance gives each request a **byte budget**, counted over the description and every module text
elaboration loads. Each instance reloads its module, so a module imported a hundred times counts a hundred
times. `check` applies the default, 1 MiB, which is two hundred times the largest description in the
repository. `check_with(request, limits)` applies the budget the instance chose. The instance sets it and
the consumer cannot, because its purpose is to protect the host from what a consumer sends. A request over
its budget is `tool-failure`, with a note naming the budget and what went past it, and nothing about the
description, because a judgement cut short is not one. A request within its budget gets exactly the answer
it would get with no budget. `crates/archogen-api/tests/budget.rs` pins both edges, through the description
and through the modules.

The command line asks with no budget. It is its own consumer and trusts the files it was given. The
language's limits still apply to it, because they belong to the language.

## How the command line is held to it

`crates/archogen-cli/tests/api_parity.rs` checks three things:

- **The command line judges nothing itself.** None of its production code names the engine's
  checking entry points. `archogen check` and `archogen build` read the file, ask the API, and print
  its answer.
- **The API offers exactly the commands that run**, less the ones §10.4 keeps out. Each exclusion
  names its reason.
- **They say the same thing.** Over every description in the repository, the command line's exit
  code, diagnostic codes and notes equal the API's status, diagnostic codes and notes.

## The binding a web page loads

A page reaches the API through `archogen-wasm` (`crates/archogen-wasm/src/lib.rs`), a module built for
`wasm32-unknown-unknown` (`docs/decisions/decision_wasm-binding.md`). A page asks it three things: a buffer for
its request, an answer, and where the answer is. The request carries the description, its profile and its
modules as length-prefixed text. The answer is JSON in one fixed encoding, and it carries every field of the
API's response, including the diagnostics rendered exactly as the command line prints them:

```text
{"format":"archogen-wasm-response/1","api":"1.2","engine":"0.1.0","status":"ok","exit":0,"notes":[],"hint":null,
 "diagnostics":[],"rendered":"","judged":{"verdict":"ok","language":"eadl/1","profile":"rt-static-up-v1", …}}
```

The instance, not the page, sets what one request may cost, so a page cannot raise the budget above. A request
the module cannot read is answered `usage`, with a note naming the field and the byte where it went wrong, and
nothing is judged. The module imports nothing, so whatever page loads it, it cannot read a file or send anything
anywhere. [Verifying the toolchain](verification.md) shows how that, and its answers, are checked.

### Opening the page

`crates/archogen-wasm/page/index.html` is a page that checks a description typed into it. A browser loads
modules only over HTTP, so build the module and serve the repository's root:

```console
$ cargo build --release -p archogen-wasm --target wasm32-unknown-unknown
$ python3 -m http.server 8000
```

Then open `http://localhost:8000/crates/archogen-wasm/page/`. The page loads the module through
`crates/archogen-wasm/js/archogen.mjs`, the same loader the checks use, and answers each time Check is pressed.
It shows the outcome and its exit code first. For a description that is refused, it then shows the diagnostics
exactly as `archogen check` prints them. For one that is accepted, it shows what the description was judged
under. Typed into the page, this description:

<!-- wasm-page-transcript: the harness (scripts/wasm_binding.sh) types the first block into the page's logic and
     compares its answer with the second -->
```eadl
(defblock console.uart (offers observable-output))
```

is answered:

```text
ok (exit 0)
accepted against profile rt-static-up-v1 (eadl/1), 1 declaration(s)
```

## What is outside it

- **`archogen build`.** Generation writes a crate tree. §10.4 keeps it a human or CI action, since a
  remote consumer does not need that authority. `build` asks the API whether a description checks,
  and generates only from an accepted one.
- **`archogen verify`.** It runs verification tiers, which drive a toolchain and an emulator. It
  needs its own ruling before it is exposed, and is not exposed by analogy.
- **The commands not built yet.** They are no operation. The command table
  (`crates/archogen-cli/src/spec.rs`) names the leaf that owns each one, and the MCP server (leaf
  `API.6`) will report that leaf rather than failing.
- **Wall-clock time.** The budget bounds the work, and the work is linear in it. A host that needs a
  deadline as well enforces it around the instance. The API reads no clock, and on `wasm32-unknown-unknown`
  the standard library has none to read.
