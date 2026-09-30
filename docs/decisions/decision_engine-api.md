# The engine API: one crate, one outcome vocabulary, and the CLI as its first consumer

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `API.3.1`, deciding the shape [[programmatic-interface]] and `ROADMAP.md` §10.4 require.
  The director ruled the surface (`2026-09-28`); this record fixes its engineering. Where it interprets
  the ruling, the interpretation is flagged in the director's findings rather than settled silently.

## The fact / decision

The engine API is a new library crate, **`crates/archogen-api`**, depending only on `eadl-front` and
`eadl-model`. It exposes the operations `archogen` has built, which today is one: **`check`**, a description
as text plus a profile in, a structured `Response` out. The CLI's `check` and `build` call it and do not
judge a description any other way.

1. **Placement.** The crate sits between the engine and every consumer, and the dependence runs one way:
   CLI → API → engine. It does no I/O, so `scripts/wasm_build.sh` derives it into the pure set, and the
   wasm binding (`API.5`) can link it as it is. The shipped kind modules move into it, because the
   language definition is what every binding needs and none should embed it again.
2. **The request.** `Request { name, text, profile, modules }`. `name` is how spans name the description.
   `profile` is optional, and its default is the API's, which the CLI's `--profile` default becomes.
   `modules` is the `ModuleSource` imports resolve through. The CLI passes a directory source, and an
   in-memory consumer passes `MemoryModules`. `ModuleSource` gains `unreadable()`, empty by default, so the
   API can tell an import that could not be read from one that does not exist, without knowing what a
   directory is.
3. **The response carries one outcome vocabulary: the exit contract's `Status`, moved into the API.** Its
   verdict variants are §5.5's. Its process-level variants, `usage` and `unimplemented`, cover a request
   the API did not judge. Every response has one, and a response cannot be built without one. Beside it
   the response has:
   - the diagnostics, with codes, spans and repair directions, and the `SourceMap` that renders them;
   - request-level notes, each a statement and a repair, for what has no span: a profile nobody supports,
     a kind module, an unreadable import, a language definition that failed to load;
   - on acceptance, the profile and language version it was judged under, the declarations, the
     module-tree instances and the closure.
4. **Rendering stays with the consumer.** The API returns structure. The CLI prints it as it prints today,
   and a binding serializes it. Serialization is a transport concern, and a dependency for it would live
   in the transport crate ([[zero-dependency-engine-core]]).
5. **Version.** `archogen_api::VERSION`, a major and a minor, versioned separately from the language
   (`eadl/1`) and the profiles, as §15 requires; a response names all three. **The promise:** within a
   major, an operation is never removed, a response field is never removed or given a new meaning, and the
   status vocabulary only grows. Adding an operation or a field is a minor. Anything else is a new major. A
   description whose verdict changes under the same language version is not an API change. It is a
   language change, and it goes through `docs/semantics/migrations/`. The first version is `1.0`. It is
   fixed when `API.3` closes; until then its children may still change the shape.
6. **Parity, both ways, gated** (`ROADMAP.md` §10.4: "a capability that exists only behind the CLI does not
   exist programmatically, and the reverse"):
   - *structurally:* the CLI's source calls no judging entry point of the engine (`check`,
     `check_program`, `shipped_registry`) except through the API;
   - *by operation:* the API's operations equal the commands `crates/archogen-cli/src/spec.rs` marks
     built, less the §10.4 exclusions (`build`), each named with its reason;
   - *by behaviour:* over every tracked description, `archogen check`'s exit code and printed codes equal
     the API's status and diagnostic codes.

## Why

- **One vocabulary rather than two.** The exit contract already sorts every outcome: `0` and `10`–`16`
  are §5.5 verdicts, and `2`, `20` and `70` are process-level (`docs/book/src/cli.md`). If the API had its
  own, parity would need a translation table, and a translation table is where a consumer's "ok" and the
  CLI's "unimplemented" drift apart. Moving `Status` into the API makes the CLI's exit code a
  projection of the response, not a second opinion.
- **"Carries §5.5's verdict for what it reports."** §10.4's wording. When the API judged the description,
  the status is that verdict. When it did not judge it, as for a kind module (`M1.32`) or an import that
  exists and cannot be read (§6 rule 7), no verdict about the description exists to carry. The status then
  says `unimplemented` or `usage`, never nothing and never `ok`. The failure the ruling guards against, an
  unanswered question read as `established`, needs a response with no outcome or with a default. This
  design has neither. ⚠️ This is an interpretation of `decision_programmatic-interface.md`'s stricter
  phrasing, "every programmatic response carries §5.5's verdict", and it is flagged for the director.
- **A crate rather than a module in `eadl-model`.** The API composes the registry, the profile table and
  the module-file routing that lives in `archogen-cli` today. Putting that in the engine would give the
  engine a front door. A separate crate gives the version one place to live, and the wasm binding one
  artifact to link.

## How to apply

- A new operation lands in the API first and in the CLI as its consumer. A command that judges a
  description any other way fails the structural parity leg.
- A new response field or operation bumps `VERSION`'s minor in the same change. Removing or redefining
  one is a major. Until `API.3` closes, the shape may change without a bump.
- `archogen build` stays outside: it calls the API's `check`, and its generation is the CLI's alone
  (§10.4, [[programmatic-interface]] ruling 3).
- The owning children are `API.3.2` (the crate), `API.3.3` (the CLI as its consumer, and the parity legs)
  and `API.3.4` (the book).
