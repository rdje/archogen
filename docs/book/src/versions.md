# What is versioned, and what changing it costs

## The idea, in plain words

A description written today must still mean the same thing years from now, though archogen will have changed many
times by then. So everything whose meaning someone relies on carries a **version**: the language itself, the
supported profile, the engine, and the formats of the files archogen writes. A version is a promise that, while it
stays the same, whatever was built on it stays valid. Changing what something means without changing its version
would break that promise without anyone noticing, so archogen does not let it happen: each version is declared once
in the code, listed on this page, and held there by a check that fails when the two disagree.

> **In one minute, for engineers.** `ROADMAP.md` §15 versions the language apart from the engine, catalog entries,
> device and timing models and evidence formats, and requires a description to keep its meaning under its locked
> semantic version. Each surface below gives its version, where it is declared, what forces a new one, what refuses
> an unannounced change, and what it keeps. `VERSION-REGISTER` (`scripts/check_version_register.sh`) requires the
> Rust declarations in its population to be registered here with the same value; frozen verdicts hold the engine to what
> descriptions mean, and golden files that are never rewritten hold each format to its identifier.

## How it works

`ROADMAP.md` §15 separates the version of the language from the versions of the engine, catalog
entries, device and timing models, and evidence formats. It also requires that "a source description
retains its meaning under its locked semantic version". This page lists every versioned surface this
repository registers from Rust declarations. For each one it gives where the version is declared, what change forces a new
version, what refuses an unannounced change, and what a description or artifact written against it can
rely on. Project-local diagnostic schemas are documented with their producers and readers, including
[tracked-input snapshots and checks](annex-repository.md#checking-restored-inputs).

`VERSION-REGISTER` (`scripts/check_version_register.sh`) keeps this page in step with the code. It reads
public Rust format/version constants, profile identifiers and API version declarations under `crates/*/src`,
and the engine version in workspace-member manifests — a declaration whole, wherever rustfmt breaks its lines. Each
one must belong to an entry below with the same value,
and every entry's declaration must still exist. A new format, or a bump, therefore cannot land without
this page changing too within that population. The checker does not enumerate Python diagnostic schema strings;
their versioned readers and permanent refusal controls enforce their local contracts.

## `language`

| Field | Value |
| --- | --- |
| Surface | the eADL language: what a description may say, and what it means |
| Version | `eadl/1` |
| Declared at | `const:crates/eadl-front/src/language_version.rs:EADL_1` |
| Changes when | a construct changes meaning. That makes a new version, never an edit of `eadl/1` (§15: "any changed behavior must be explicit") |
| Pinned by | `LANGUAGE-FREEZE` (`docs/semantics/BASELINE.txt`, with a migration note for each amendment) and the conformance suite (`docs/semantics/conformance.md`) |
| Keeps | a description that states `(eadl-version eadl/1)` reads and checks the same under every engine that accepts `eadl/1` |

## `profile`

| Field | Value |
| --- | --- |
| Surface | the supported profile: which systems the engine admits, and what it may claim about them |
| Version | `rt-static-up-v1` |
| Declared at | `profile:crates/eadl-model/src/profile.rs:rt-static-up-v1` |
| Changes when | an admitted construct or a claim changes. That makes `-v2`, not an edit |
| Pinned by | `docs/profiles/rt-static-up-v1.md`, which the tests in `crates/eadl-model/src/profile.rs` compare with the code |
| Keeps | a system admitted under `rt-static-up-v1` stays admitted, with the same claims |

## `engine`

| Field | Value |
| --- | --- |
| Surface | the engine: the workspace's crates, which read, check, realize and analyse, as `archogen --version` reports them |
| Version | `0.1.0` |
| Declared at | `manifests` |
| Changes when | a release. The engine may realize a description better without changing what it means (§15) |
| Pinned by | the frozen verdict of every description in the repository (`crates/archogen-cli/tests/verdicts.rs`, 120 today), and the conformance suite |
| Keeps | the language's and the profile's guarantees above, which do not depend on the engine's version |

## `configuration-checker`

| Field | Value |
| --- | --- |
| Surface | the configuration checker's own version, as its [MCP](ledger.md#mcp-specification) server names itself in `serverInfo` ([Checking a plan](plan-check.md#as-an-mcp-tool)) |
| Version | `0.1.0` |
| Declared at | `const:crates/archogen-plan-check/src/lib.rs:VERSION` |
| Changes when | the checker's package version does, at a release: a literal, because the trust gate refuses a value compiled in through `env!` in a root |
| Pinned by | `crates/archogen-plan-check/tests/mcp.rs`, which holds it equal to the package manifest's version (`M3.5.1.6.4`) |
| Keeps | a client reads which checker answered; what it judges and writes is pinned by `result-format` and `assignment-format`, not by this number |

## `catalog-rules`

| Field | Value |
| --- | --- |
| Surface | the catalog's hash grammar and its ledger-time checks, which every catalog hash begins with and the lock's first line names (`docs/specs/catalog/decision_catalog-records.md` §3 and §5) |
| Version | `archogen-catalog/1` |
| Declared at | `const:crates/archogen-catalog/src/hash.rs:IDENTIFIER` |
| Changes when | any part of the hash grammar or of the ledger-time checks changes. Every hash moves with it, and the change's migration note says how the lock's earlier lines compare under it (`ROADMAP.md` §15) |
| Pinned by | the worked example's 23 digests, read from `docs/specs/catalog/decision_catalog-records-example.md` and reproduced by `crates/archogen-catalog/tests/hash.rs` |
| Keeps | a hash labelled `/1` covers the same bytes for every reader, and a line ledgered under `/1` is verified under `/1` for good |

## `analysis-model`

| Field | Value |
| --- | --- |
| Surface | the model the scheduling analysis reasons in, which every conclusion names ([What the scheduling checker establishes](analysis.md)) |
| Version | `idealized-zero-overhead/1` |
| Declared at | `const:crates/rt-analysis/src/response.rs:MODEL` |
| Changes when | an assumption of the model changes: what costs nothing, what is atomic, what is known. A changed model invalidates every conclusion stated in the old one |
| Pinned by | the tests in `crates/rt-analysis/src/response.rs`, and the conclusions printed in `docs/book/src/analysis.md`, which quote the model by name |
| Keeps | a conclusion "in `idealized-zero-overhead/1`" means the same bound under the same assumptions |

## `runtime-analysis-model`

| Field | Value |
| --- | --- |
| Surface | the model the runtime-applicable analysis variant reasons in, which each of its conclusions names, apart from the baseline's ([What the scheduling checker establishes](analysis.md)) |
| Version | `fixed-priority-with-overheads/2` — `/1` until `2026-10-02`, when leaf `M2.11` changed its admission conditions |
| Declared at | `const:crates/rt-analysis/src/runtime.rs:MODEL` |
| Changes when | the recurrence, the charging or the admission conditions change after a conclusion has been stated: the next is `/3`, and a changed model invalidates every conclusion stated in the one before (`docs/specs/analysis/decision_runtime-analysis-variant.md`, *How to apply*) |
| Pinned by | `crates/rt-analysis/tests/runtime_variant.rs`, which holds the constant to the identifier the record states and to the record's path, and each conclusion its tests check to the constant |
| Keeps | a conclusion "in `fixed-priority-with-overheads/2`" means the same bound under the same conditions |

## `cost-format`

| Field | Value |
| --- | --- |
| Surface | the cost-accounting contract the scheduling analysis reports against (§7.4.1) |
| Version | `cost-accounting/1` |
| Declared at | `const:crates/rt-analysis/src/cost.rs:CONTRACT_VERSION` |
| Changes when | a cost term is added, removed or reinterpreted |
| Pinned by | everything it says — its kinds of total, ledger categories, seven identifications and the table a ledger renders as — frozen in `crates/rt-analysis/tests/goldens/cost-accounting-1.golden` and compared by `crates/rt-analysis/tests/format_golden.rs`; never rewritten |
| Keeps | a bound computed under `cost-accounting/1` counts the same costs every time |

## `engine-api`

| Field | Value |
| --- | --- |
| Surface | the engine API: the one contract every consumer judges a description through, the CLI today and the wasm and [MCP](ledger.md#mcp-specification) bindings to come ([The engine API](engine-api.md)) |
| Version | `1.10` — `1.0` fixed `2026-09-30` when leaf `API.3` closed; `1.1` added the response's engine version (`API.4.1`); `1.2` added `check_with` and its `Limits` (`API.4.2`); `1.3` added the status `stale-input`, the vocabulary growing (`M3.5.1.1`); `1.4` added the operation `resolve`, the search alone (`M3.4.4`); `1.5` added the operation `explain`, one requirement's account (`M3.4.13.2`); `1.6` added the definition `defadapter`, a file of the engine's adapter catalog answered as the language's own (`M3.2.2`); `1.7` added the plan's `obligations`, what a plan binding an engine adapter owes (`M3.2.6.1`); `1.8` added the resolution's `elaborated`, what a plan was written from, for the command line's build (`M4.2.3.1`), `1.9` `is_a_plan`, which the command line asks before a plan replaces what stands at its name (`M4.2.3.3.10`), `1.10` the request's `target`, the machine a description's systems are judged on (`M4.1.7.4.1`) |
| Declared at | `const:crates/archogen-api/src/lib.rs:VERSION` |
| Changes when | an operation or a response field is added, which is a minor; an operation or a field removed, or a field given a new meaning, which is a new major |
| Pinned by | the exhaustive reading of `Response` and `Judgement` in `crates/archogen-api/tests/check.rs`, which stops compiling when a field is added or removed and names the version its shape belongs to; the three parity legs of `crates/archogen-cli/tests/api_parity.rs`. The bump itself is the author's act: nothing can tell a new field from a renamed one |
| Keeps | within a major, what a consumer reads from a response stays true, and its outcome vocabulary only grows |

## `status-vocabulary`

| Field | Value |
| --- | --- |
| Surface | the outcome vocabulary: every status an API response, an exit code or a checker's result record carries, with its code and its family, apart from the engine API since a checker carries it without the API ([The `archogen` command line](cli.md), exit codes) |
| Version | `1.1` — `1.0`, the eleven statuses the engine API carried from leaf `API.3.2`; `1.1` added `stale-input`, exit `3` (`M3.5.1.1`) |
| Declared at | `const:crates/archogen-outcome/src/status.rs:VOCABULARY` |
| Changes when | a status is added, which is a minor; a status removed, renamed, renumbered, moved between families or given a new meaning, which is a new major |
| Pinned by | one golden per minor, `crates/archogen-outcome/tests/goldens/status-vocabulary-<major>.<minor>.golden`, each listing what that minor holds and compared by `crates/archogen-outcome/tests/vocabulary.rs`, never rewritten; each status's `since`, the minor it entered at |
| Keeps | within a major, a status means what it meant and keeps its code, and a reader of an earlier minor refuses only a status it does not know |

## `result-format`

| Field | Value |
| --- | --- |
| Surface | the record a checker writes for the front that ran it and the build that consumes it: who wrote it and its sha256, what it judged, the profile, the status and every finding, up to `too-many-errors` (`M3.5.1`) |
| Version | `archogen-result/1` |
| Declared at | `const:crates/archogen-outcome/src/record.rs:FORMAT` |
| Changes when | a member is added, removed, renamed, re-typed or reinterpreted. A reader that meets another identifier refuses the record rather than guess |
| Pinned by | its shape — every key path and the kind of value at it — frozen in `crates/archogen-outcome/tests/goldens/archogen-result-1.golden` and compared by `crates/archogen-outcome/tests/record.rs`, never rewritten; the same test reads every record back as itself and reads no damage to a refusal as an acceptance |
| Keeps | a record labelled `archogen-result/1` has one text, and means the same to every reader of `/1` |

## `assignment-format`

| Field | Value |
| --- | --- |
| Surface | the record of what the search chose for one system — the profile, the system, the elaborated program's digest, the adapter catalog it was resolved with, the placement memory and one binding per requester and fact, an engine adapter's instance a provider and a requester among them — which the configuration checker judges against the description and the plan carries (`M3.5.4.1`; `/2`, `M3.2.5`) |
| Version | `archogen-assignment/2` — `/1` fixed by `M3.5.4.1`; `/2` added the `adapters` clause and the adapter instance, a provider and requester kind (`docs/specs/adapters/decision_engine-adapters.md` §5) — and `archogen-assignment/3`, the plan of a system on a target: `(target NAME "SHA256")` after the program and the release requester, `(release SYSTEM)`, last among requesters (`M4.1.7.5.2`, `docs/decisions/decision_runtime-plan-format-2.md` §1); `/2` stays the hosted one |
| Declared at | `const:crates/archogen-plan-check/src/assignment.rs:FORMAT`, the checker's; `const:crates/eadl-resolve/src/plan.rs:FORMAT`, the search's — two constants of one format, one golden; `const:crates/eadl-resolve/src/plan.rs:FORMAT_3`, `/3`'s, the checker's own constant of it `M4.1.7.5.4`'s |
| Changes when | a clause is added, removed, renamed, re-typed, reordered or reinterpreted, a provider or requester kind is added, or the program's digest is computed otherwise. A reader that meets another identifier refuses the record rather than guess |
| Pinned by | its canonical text — every clause, the bindings' order and the digest of a fixed program — frozen in `crates/archogen-plan-check/tests/goldens/archogen-assignment-2.golden` and compared by `crates/archogen-plan-check/tests/record.rs`, never rewritten, beside `/1`'s golden, kept unchanged and refused as another format; the same test reads every record back as itself and refuses another format, a kind the checker's table does not hold and a binding out of order or twice. A plain description — no `defmodule` — is one root instance, written `instance . - 0.0`, which `crates/archogen-plan-check/tests/executable.rs` holds (`M3.5.6.1`, completing `/1` before any plan of one could be checked). `/3`'s golden, `crates/archogen-plan-check/tests/goldens/archogen-assignment-3.golden`, written apart, holds the search's writer of it in `crates/eadl-resolve/tests/plan.rs`, which reads it back and refuses a release requester in `/2` and a `/3` naming no target. The search's writer, `crates/eadl-resolve/src/plan.rs` with its own `FORMAT` constant, is held to the same golden byte for byte, and its program text to the checker's over every tracked description, by `crates/eadl-resolve/tests/plan.rs` (`M3.4.3`) |
| Keeps | a record labelled `archogen-assignment/2` has one text, and names the same program, adapter catalog and bindings to every reader of `/2`; the generator's own reader, `plan::read`, accepts only what its writer writes, every record the search writes reading back as itself (`M4.1.3`) |

## `runtime-plan-format`

| Field | Value |
| --- | --- |
| Surface | the runtime plan a checked assignment is lowered into, for a system run on the hosted playground: its tasks and what releases each, its bindings and adapter instances, its interrupts ranked, each part's execution contexts, its initialisation graph and steps, and what it owes (`docs/decisions/decision_runtime-plan.md`, `M4.1`) |
| Version | `archogen-runtime-plan/1`, its design accepted `2026-10-29` (`M4.1.1`), for the hosted playground; a target's layout is `/2`'s (`M4.1.7`) |
| Declared at | `const:crates/archogen-plan/src/lib.rs:FORMAT` |
| Changes when | any rule of the record's §2 changes — an order, the topological choice — or what §2 reads, the joint rules, the vocabulary or the adapter catalog, moves a plan for the same inputs |
| Pinned by | three goldens, lowered by the production entry from frozen inputs and a fixed result record, each held to its sha256 and never rewritten (`crates/archogen-plan/tests/goldens.rs`); the record's three worked texts, compared with what the code writes by `crates/archogen-plan/tests/worked.rs`, and the review's probe systems with their outcomes, `crates/archogen-plan/tests/corpus.rs`; the frozen goldens come with `M4.1.4`'s production half |
| Keeps | a plan labelled `archogen-runtime-plan/1` has one text for one record and one description |

## `application-inputs-format`

| Field | Value |
| --- | --- |
| Surface | what a description may not state of a task's own code, in one file per system: what holds its code, each task's entry point, bounds with their evidence, calls and facts, and the application's quoted declaration (`M4.12`) |
| Version | `archogen-application/1` |
| Declared at | `const:crates/archogen-plan/src/application.rs:FORMAT` |
| Changes when | a clause is added, removed, renamed or reinterpreted, or the words `CLAUSE_PARTS` quotes move. A reader that meets another identifier refuses the file rather than guess |
| Pinned by | `crates/archogen-plan/tests/code_digest.rs`, which holds the `code` digest's recipe to the pinned compiler, and `crates/archogen-plan/tests/application.rs`: a whole file reads as it says, each malformed input is refused saying what is wrong, the quoted clause reads back from the composite-inputs record with what stands beside it and stands whole in the book, and every fact the three golden plans owe the application is held |
| Keeps | a file labelled `archogen-application/1` means the same to every reader of `/1`, and declares the words it quotes |

## `adapter-catalog`

| Field | Value |
| --- | --- |
| Surface | the adapters the engine knows — what each requires of the provider it programs, what it offers over it, the level its code runs at and what it owes — which the search binds and the configuration checker judges (`docs/specs/adapters/decision_engine-adapters.md` §1, §2) |
| Version | `adapters/1` |
| Declared at | `const:crates/eadl-resolve/src/adapter.rs:VERSION`, the generator's; `const:crates/archogen-plan-check/src/adapters.rs:VERSION`, the checker's — two constants of one catalog, held equal |
| Changes when | a kind is added, removed or renamed, or what one requires, offers, owes or runs at changes. The file's canonical form is frozen in `docs/semantics/BASELINE.txt`, so a change needs a migration note, and a plan naming another catalog is refused by the checker (record §5) |
| Pinned by | `docs/semantics/adapters/adapters.eadl`, read by `crates/eadl-resolve/src/adapter.rs` and held kind by kind to the checker's own table by `crates/eadl-resolve/tests/adapters.rs` (`M3.2.2`) |
| Keeps | a plan resolved with `adapters/1` binds instances whose requirements, offers and obligations mean the same to the generator and the checker |

## `install-format`

| Field | Value |
| --- | --- |
| Surface | the manifest an install writes beside the programs it placed: the directory, the build they came from, the inventory's sha256, the front and each sibling with its role and sha256 (`M3.5.1.3`, [Installing](cli.md#installing)) |
| Version | `archogen-install/1` |
| Declared at | `const:crates/archogen-install/src/lib.rs:FORMAT` |
| Changes when | a member is added, removed, renamed, re-typed or reinterpreted. A front that meets another identifier refuses the install rather than guess |
| Pinned by | its shape — every key path and the kind of value at it — frozen in `crates/archogen-install/tests/goldens/archogen-install-1.golden` and compared by `crates/archogen-install/tests/manifest.rs`, never rewritten; the same test reads every manifest back as itself |
| Keeps | a manifest labelled `archogen-install/1` has one text, and names the same programs to every reader of `/1` |

## `playground-trace-format`

| Field | Value |
| --- | --- |
| Surface | the trace a system prints on the hosted playground: one line an event, its time exact, in the one order `docs/decisions/decision_hosted-build.md` §3 fixes for an instant |
| Version | `archogen-playground-trace/4` — `/2` moved the order of one instant, a job with nothing left that takes time finishing before what arrives then (`M4.3.2`); `/3` made the fault paths' events the ones `docs/profiles/rt-static-up-v1-faults-observation.md` §2 fixes, added a release's nominal instant and idle entry, and wrote a programmed instant as a time (`M4.3.3`); `/4` added each device to the header, a job's access to one, and an alarm's programming, arrival and service (`M4.3.5`) |
| Declared at | `const:crates/sim-playground/src/lib.rs:TRACE_FORMAT` |
| Changes when | an event's words change, an event is added or removed, or the order of one instant moves |
| Pinned by | the exact traces of `crates/sim-playground/tests/harness.rs` |
| Keeps | one system, run twice, writes one trace |

## `build-provenance-format`

| Field | Value |
| --- | --- |
| Surface | what a generated system says it was lowered from, and what each of its table entries came from: `provenance.json`, beside the plan's text |
| Version | `archogen-build-provenance/5` — `/2` added the name of the description the build records (`M4.2.3.2`), `/3` every file it wrote with its digest and the entries of `SOURCES` and `INITIALISATION`, `/4` its own digest (`M4.2.3.3`), `/5` each file's length, so that a file grown to any size is changed without being read (`M4.2.3.3.11`), each days or less after the last, which no reader had met |
| Declared at | `const:crates/archogen-emit/src/lib.rs:PROVENANCE_FORMAT` |
| Changes when | a member is added, removed or reinterpreted |
| Pinned by | its shape — every key path and the kind of value at it — frozen in `crates/archogen-cli/tests/goldens/archogen-build-provenance-5.golden` and compared by `crates/archogen-cli/tests/format_golden.rs`, a golden never being rewritten; each entry followed from both ends by `crates/archogen-cli/tests/build.rs` |
| Keeps | a generated table entry can be followed back to the plan's line and the description's form |

## `wasm-request-format`

| Field | Value |
| --- | --- |
| Surface | the request a page writes to the wasm binding: length-prefixed fields for the description, its profile, its target and its modules ([The engine API](engine-api.md), `docs/decisions/decision_wasm-binding.md` §4) |
| Version | `archogen-wasm-request/2`, which the loader writes — `/1` fixed by `API.5`; `/2` names a target after the profile, at most one (`M4.1.7.4.2.3`) — and `archogen-wasm-request/1`, still read as `/2` naming none |
| Declared at | `const:crates/archogen-wasm/src/lib.rs:REQUEST_FORMAT`, `const:crates/archogen-wasm/src/lib.rs:REQUEST_FORMAT_1` |
| Changes when | a field is added, removed, reordered or reinterpreted. A request naming another format is answered `usage`, never read |
| Pinned by | the framing tests of `crates/archogen-wasm/tests/binding.rs`: a request reads back as it was framed, a `/1` request is answered as `/2` naming no target, and each refusal names its field and byte offset |
| Keeps | a loader that writes `archogen-wasm-request/1` or `/2` is read the same way by every module that accepts it |

## `wasm-response-format`

| Field | Value |
| --- | --- |
| Surface | the JSON the wasm binding answers with, one fixed encoding of the engine API's response (`docs/decisions/decision_wasm-binding.md` §6) |
| Version | `archogen-wasm-response/1` |
| Declared at | `const:crates/archogen-wasm/src/lib.rs:RESPONSE_FORMAT` |
| Changes when | a key is added, removed, renamed or re-typed, or the encoding of a value changes |
| Pinned by | its shape — every key path and the kinds of value at it — frozen in `crates/archogen-wasm/tests/goldens/archogen-wasm-response-1.golden` and compared by `crates/archogen-wasm/tests/binding.rs`, never rewritten; the same test compares every field with the API's response |
| Keeps | a page that reads `archogen-wasm-response/1` finds every key where it was, with the same kind of value |

## `plan-check-wasm-request-format`

| Field | Value |
| --- | --- |
| Surface | the request a page writes to the configuration checker's own wasm module: length-prefixed fields for the module's digest, the description, its profile, its modules and the plan ([Checking a plan](plan-check.md#in-a-browser), `docs/decisions/decision_wasm-binding.md` §11) |
| Version | `archogen-plan-check-wasm-request/1` |
| Declared at | `const:crates/archogen-plan-check-wasm/src/lib.rs:REQUEST_FORMAT` |
| Changes when | a field is added, removed, reordered or reinterpreted. A request naming another format is refused `usage`, never judged |
| Pinned by | the framing tests of `crates/archogen-plan-check-wasm/tests/module.rs`: a request reads back as it was framed, and each refusal names its field and byte offset |
| Keeps | a loader that writes `archogen-plan-check-wasm-request/1` is read the same way by every module that accepts it |

## `plan-check-wasm-refusal-format`

| Field | Value |
| --- | --- |
| Surface | the answer the configuration checker's wasm module gives a request it cannot judge — its format, status, exit code and a note — where a request it judges is answered with the result record, `result-format` (`docs/decisions/decision_wasm-binding.md` §11) |
| Version | `archogen-plan-check-wasm-refusal/1` |
| Declared at | `const:crates/archogen-plan-check-wasm/src/lib.rs:REFUSAL_FORMAT` |
| Changes when | a key is added, removed, renamed, reordered or re-typed |
| Pinned by | `crates/archogen-plan-check-wasm/tests/module.rs`, which reads every refusal it makes and holds its keys, in their order, and their kinds |
| Keeps | a page that meets `archogen-plan-check-wasm-refusal/1` finds why nothing was judged where it was |

## How an engine change is held to what descriptions mean

The engine changes more often than the language. Each time it does, every description in the
repository must keep its verdict: the exit code of `archogen check`, and the codes of every diagnostic
it reports. Those verdicts are frozen in `crates/archogen-cli/tests/verdicts.txt`, and a test compares
all of them on every run.

A change that moves one fails the test. It passes only if the table is regenerated in the same change:

```console
$ ARCHOGEN_BLESS_VERDICTS=1 cargo test -p archogen-cli --test verdicts
```

Regenerating puts every moved verdict in that change's diff, where a reviewer sees it, instead of
nowhere. Three engine fixes in one day checked this by hand, each by building the tool twice and
comparing the output for every description. The frozen table does the same check on every run. As a
test of the test, making the deadline rule refuse deadline = period moves **20** of the 120 verdicts,
and the table names each one with its old and new verdict.

## How a format is held to its identifier

Each evidence format has a golden file named after its identifier, and a test compares what the format
emits, or says, with it on every run. A golden is **never rewritten**. Change the provenance's keys
or the cost contract's wording under the same identifier, and the test fails until the identifier
moves. A new identifier gets its golden only deliberately:

```console
$ ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-cli --test format_golden
$ ARCHOGEN_BLESS_FORMATS=1 cargo test -p rt-analysis --test format_golden
```

Blessing cannot overwrite an existing golden, so it cannot be used to relabel a changed shape as the
old one. The old golden stays behind as the record of what the old identifier meant.

## Not versioned yet

These surfaces have no version yet because they do not exist yet. Each is named with the tree that
owns it, so the gap is visible rather than implied:

- **Catalog records**, the engine knowledge that realizes descriptions, are designed in `M2.7`:
  `docs/specs/catalog/decision_catalog-records.md`, accepted after its independent review, versions each part of
  a record apart, and its lock names the version of its own rules (see [the catalog chapter](catalog.md)).
  Catalogs at scale belong to `M6`. F25, "a locked rebuild after a catalog update", lives at `M6.4`.
- **Device and timing models** belong to `M2`.
- **The build plan and its identity**, the plan hash and the binary hash, belong to `M4.7`.
