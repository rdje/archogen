# The MCP server: both protocol eras over stdio, its tools declared in the command table

- **Type:** `decision`
- **Date:** `2026-10-01`
- **Status:** `active` — the design for leaf `API.6`, reviewed under `API.6.5` before the server is relied on
- **Owner / source:** leaf `API.6` (`docs/tasks/API.md`), under `ROADMAP.md` §10.4 and
  [[decision_programmatic-interface]]. The protocol is read at its primary source, the [MCP
  specification](../book/src/ledger.md#mcp-specification), on `2026-10-01`.

## The fact / decision

1. **Both protocol eras, over stdio.** The current revision of MCP is `2026-07-28` ("The **current** protocol
   version is 2026-07-28"). It has no handshake: every request carries `io.modelcontextprotocol/protocolVersion` and
   `io.modelcontextprotocol/clientCapabilities` in its `_meta`, a request missing either is refused with `-32602`, an
   unsupported version with `-32022` and `data: {supported, requested}`, and every server "**MUST** implement
   `server/discover`". The revisions up to `2025-11-25` open with `initialize`. The specification's compatibility
   matrix says a legacy client against a modern-only server "Fails … Legacy clients have no fall-forward mechanism",
   and that a dual-era server "selects its behavior from how the client opens". archogen's server is dual-era: a
   request carrying the per-request `_meta` is served statelessly under `2026-07-28`; an `initialize` selects
   `2025-11-25`, scoped to the process, with `tools/list`, `tools/call` and `ping` after it. It supports exactly those
   two versions and names both in every version error. Messages are one JSON-RPC 2.0 object per line on stdin and
   stdout, nothing else is written to stdout, and the server exits when stdin ends.
2. **The tool list is a fifth consumer of `crates/archogen-cli/src/spec.rs`.** Each command declares whether it is
   offered programmatically, as data beside its `Maturity`: `check` is a tool; `resolve`, `analyze`, `explain` and
   `replay` are tools that are not built, listed so that a consumer sees them at discovery, and answered when called
   with the leaf that owns them; `build` is excluded by ruling (§10.4, "Both builds are outside the programmatic
   interface"); `verify` is excluded until it has its own ruling (the open question on the `API` tree); and the
   server's own command is no tool. A test asserts the exclusions from the table, so a command added tomorrow is a
   deliberate choice, not an accident of a name list. Each tool carries its maturity and owner in its `_meta` under
   the prefix `io.github.rdje.archogen/`, and the annotations `readOnlyHint: true`, `destructiveHint: false`,
   `idempotentHint: true`, `openWorldHint: false`, which are true of every operation the surface offers.
3. **A tool takes the engine API's request, not the command line's.** The command line names a file; the server
   has no filesystem authority (§10.4), so `check` takes the description's text, its profile and the texts of its
   modules — `archogen_api::Request` — under the API's byte budget, and answers `archogen_api::Response`.
4. **Every result carries §5.5's verdict, and `isError` cannot overstate it.** A tool result's `structuredContent`
   is the response in the wasm binding's encoding (`docs/decisions/decision_wasm-binding.md` §6), the same bytes
   for the same answer, and its one text block carries that JSON, as the specification asks of structured content.
   `isError` is `false` only when the response accepts the description; a refusal, an inconclusive answer or a
   tool failure is `true`, so a client that reads nothing but `isError` never takes a refusal for a pass. A call
   naming no tool, or with arguments that are not the tool's, is a protocol error, `-32602`.
5. **Bounded on untrusted input.** A line longer than the API's byte budget plus a fixed allowance for the
   envelope is refused with `-32700` without being parsed; the JSON reader is the server's own, refuses nesting
   deeper than a fixed bound and every malformed text, and reads nothing it does not need. Work per request is linear
   in its size (`API.4.2`), and requests are served one at a time; that, for a stdio server with one client, is how
   the specification's "Rate limit tool invocations" is met, and the record says so rather than claiming more.
6. **No dependency, no process.** The server, its reader and its writer are code in `crates/archogen-cli`, which
   depends on the engine API and not the reverse, so the checker and the generator share nothing new (§10.4,
   F30). It spawns nothing (`NO-SUBPROCESS`).

## Why

- **Dual-era, not modern-only.** The specification changed its model on `2026-07-28`, and a client built for the
  earlier revisions cannot reach a server that only speaks the new one. A server that answers one era is a server
  some agents cannot use, and the cost of the second is three small methods.
- **Exposure as data.** §10.4 asks that "its tool list is derived from the same command table the CLI help is
  derived from". A list of excluded names in the server would be the second list that requirement forbids; a
  field each command carries is the same table answering one more question.
- **`isError` from the verdict.** The specification's `isError` marks "Tool Execution Errors"; a description the
  checker refuses is exactly the actionable feedback it describes. Leaving it `false` on a refusal would let a
  client that reads one boolean overstate the evidence, which §10.4 names as the risk the verdict exists to stop.

## How to apply

- The leaves: `API.6.1` this record and the ledger entry; `API.6.2` the exposure field and the tool list; `API.6.3`
  the bounded JSON reader; `API.6.4` the server, both eras, and `archogen mcp`; `API.6.5` the book and an
  independent review.
- A new revision of MCP is a change to this record first: the versions the server names are this record's.
- `docs/book/src/ledger.md#mcp-specification` pins what was read; a decision resting on more of the specification
  reads it again there.

Related: [[decision_programmatic-interface]], [[decision_api-instance]], [[decision_wasm-binding]].
