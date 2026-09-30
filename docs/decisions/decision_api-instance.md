# An archogen instance: bound to one build, stateless between requests, reproducible from its request

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`
- **Owner / source:** leaf `API.4.1`. [[programmatic-interface]] requires that "instance" be defined before it
  is implemented, and the director's ruling made the MCP server "spawned per instance". [[engine-api]] is the
  contract an instance serves.

## The fact / decision

An **instance** is one running consumer-facing copy of archogen, built from one build of this repository: an MCP
server a consumer spawned (`API.6`), a wasm module a page instantiated (`API.5`), or, with the shortest life,
one run of the command line.

1. **Lifecycle.** A consumer creates it and ends it. In between it answers requests through the engine API.
   It reads no file and writes none in doing so; the CLI reads the one file it was given, and the module files
   beside it, before it asks.
2. **State.** None between requests. Every response is computed from its own request and from the build. No
   cache, no counter and no configuration carried from one request to the next can change an answer.
3. **Bound to its build.** The engine version (`archogen_api::ENGINE`, the workspace members' shared version),
   the API version (`archogen_api::VERSION`), the language version it reads (`eadl/1`), the profiles it
   supports, and the kind modules embedded in it. A different build is a different instance.
4. **Identity in the response.** Every response names the API version and the engine version. A judged one
   also names the language version and the profile it was judged under, and it carries the request's own
   texts in `sources`, so a response names what produced it.
5. **Reproducible against:** the request (the description's text and name, the profile, and the text of every
   module elaboration loaded) and the build. The same request to the same build gives the same response.
   `crates/archogen-api/tests/check.rs` asks twice and compares the whole response.

## Why

- **Statelessness is what makes a response evidence.** A response that depended on an earlier request could
  not be reproduced from what it names, and §15's "any changed behavior must be explicit" would have no
  object. With no state, the request plus the build is the whole cause of an answer.
- **Identity by build, not by process.** Two processes of one build answer alike, so the process is not
  worth naming. Two builds may not, so the build is.

## How to apply

- ⚠️ **Identity by version numbers is only as good as the release discipline.** A build from an unreleased
  tree carries the last release's numbers. The engine version is `0.1.0` today and has never been bumped. A
  response names its build truly only for a build made at a release. Stated here rather than hidden. A build
  digest in the response would close it and is a minor bump away. No leaf owns it yet.
- A new kind of consumer (an editor plugin, a build farm) is an instance by this definition, and needs no
  new record.
- Limits on what an untrusted consumer may send are a property of the instance's request handling, not of the
  language: [[engine-api]] leaf `API.4.2`.
