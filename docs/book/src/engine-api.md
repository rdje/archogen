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
api 1.1 · engine 0.1.0 · status ok
  judged under eadl/1, profile rt-static-up-v1: 1 declaration(s)
api 1.1 · engine 0.1.0 · status invalid-description
  judged under eadl/1, profile rt-static-up-v1: 1 declaration(s)
  quantity-unknown-unit: `parsec` is not a known unit
api 1.1 · engine 0.1.0 · status unsupported-profile
  not judged: `rt-dynamic-mp` is not a supported profile
```

`crates/archogen-api/tests/book_example.rs` runs the example and compares its output with the block
above, so this page cannot go on showing output the API no longer gives.

## The version and what it promises

The API's version is `1.1`, apart from the language's (`eadl/1`) and the profile's, as §15 requires.
Every response names the API version and the engine version, and a judged one also names the language
version and the profile it was judged under. `1.0` was fixed when the API was declared, and `1.1` added
the engine version, a new field, so a minor, as the promise below says. Within a major, an operation is never removed, a response field
is never removed or given a new meaning, and the outcome vocabulary only grows. Adding an operation
or a field is a minor. A description whose verdict changes under the same language version is not an
API change: it is a language change, and it goes through the migration notes
([What is versioned](versions.md)).

## What an instance is

A consumer does not talk to a library. It talks to an **instance**: an MCP server it spawned, a wasm module
a page loaded, or, for the shortest life, one run of the command line
(`docs/decisions/decision_api-instance.md`). An instance is bound to the build it came from: the engine
version, the API version, the language it reads, the profiles it supports, and the kind modules built
into it. It keeps nothing between requests. So a response depends on two things only: its request (the
description, the profile, and the module texts it loaded) and the build, which the response names.
Asking the same instance the same question twice gives the same answer, and
`crates/archogen-api/tests/check.rs` asks twice and compares the whole response.

## How the command line is held to it

`crates/archogen-cli/tests/api_parity.rs` checks three things:

- **The command line judges nothing itself.** None of its production code names the engine's
  checking entry points. `archogen check` and `archogen build` read the file, ask the API, and print
  its answer.
- **The API offers exactly the commands that run**, less the ones §10.4 keeps out. Each exclusion
  names its reason.
- **They say the same thing.** Over every description in the repository, the command line's exit
  code, diagnostic codes and notes equal the API's status, diagnostic codes and notes.

## What is outside it

- **`archogen build`.** Generation writes a crate tree. §10.4 keeps it a human or CI action, since a
  remote consumer does not need that authority. `build` asks the API whether a description checks,
  and generates only from an accepted one.
- **`archogen verify`.** It runs verification tiers, which drive a toolchain and an emulator. It
  needs its own ruling before it is exposed, and is not exposed by analogy.
- **The commands not built yet.** They are no operation. The command table
  (`crates/archogen-cli/src/spec.rs`) names the leaf that owns each one, and the MCP server (leaf
  `API.6`) will report that leaf rather than failing.
- **Limits on an untrusted description.** A consumer handing the API text it did not write has no
  memory or time limit to rely on yet. That is leaf `API.4`, and until it lands the API is safe only
  for descriptions its caller trusts.
