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

## `catalog-s0`

| Field | Value |
| --- | --- |
| Surface | the one catalog entry so far: S0's fixed realization, `s0.hosted-playground.periodic`, a stub until the catalogs of `M6` |
| Version | `0.1.0` |
| Declared at | `const:crates/archogen-s0/src/provenance.rs:REALIZATION_VERSION` |
| Changes when | the observation contract changes, since each frozen expectation in `examples/s0-heartbeat/expected/` holds for one contract |
| Pinned by | the frozen observations, compared by `crates/archogen-cli/tests/s0_oracle.rs` |
| Keeps | the same description built by this entry gives the same observation |

## `provenance-format`

| Field | Value |
| --- | --- |
| Surface | the provenance a build writes beside what it generated |
| Version | `archogen-provenance/1` |
| Declared at | `const:crates/archogen-s0/src/provenance.rs:FORMAT` |
| Changes when | a field is added, removed or reinterpreted. A consumer that does not recognize the identifier must refuse rather than guess |
| Pinned by | its shape — every key path and the kind of value at it — frozen in `crates/archogen-cli/tests/goldens/archogen-provenance-1.golden` and compared by `crates/archogen-cli/tests/format_golden.rs`; a golden is never rewritten, so a shape change fails until the identifier moves |
| Keeps | an artifact labelled `archogen-provenance/1` means the same to every reader of `/1` |

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
| Version | `1.3` — `1.0` fixed `2026-09-30` when leaf `API.3` closed; `1.1` added the response's engine version (`API.4.1`); `1.2` added `check_with` and its `Limits` (`API.4.2`); `1.3` added the status `stale-input`, the vocabulary growing (`M3.5.1.1`) |
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
| Surface | the record a checker writes for the front that ran it and the build that consumes it: who wrote it and its sha256, what it judged, the profile, the status and every finding (`M3.5.1`) |
| Version | `archogen-result/1` |
| Declared at | `const:crates/archogen-outcome/src/record.rs:FORMAT` |
| Changes when | a member is added, removed, renamed, re-typed or reinterpreted. A reader that meets another identifier refuses the record rather than guess |
| Pinned by | its shape — every key path and the kind of value at it — frozen in `crates/archogen-outcome/tests/goldens/archogen-result-1.golden` and compared by `crates/archogen-outcome/tests/record.rs`, never rewritten; the same test reads every record back as itself and reads no damage to a refusal as an acceptance |
| Keeps | a record labelled `archogen-result/1` has one text, and means the same to every reader of `/1` |

## `assignment-format`

| Field | Value |
| --- | --- |
| Surface | the record of what the search chose for one system — the profile, the system, the elaborated program's digest, the placement memory and one binding per requester and fact — which the configuration checker judges against the description and the plan carries (`M3.5.4.1`) |
| Version | `archogen-assignment/1` |
| Declared at | `const:crates/archogen-plan-check/src/assignment.rs:FORMAT` |
| Changes when | a clause is added, removed, renamed, re-typed, reordered or reinterpreted, a provider or requester kind is added, or the program's digest is computed otherwise. A reader that meets another identifier refuses the record rather than guess |
| Pinned by | its canonical text — every clause, the bindings' order and the digest of a fixed program — frozen in `crates/archogen-plan-check/tests/goldens/archogen-assignment-1.golden` and compared by `crates/archogen-plan-check/tests/record.rs`, never rewritten; the same test reads every record back as itself and refuses another format, a provider kind `/1` does not name and a binding out of order or twice. The search's writer (`M3.4`) is held to the same golden |
| Keeps | a record labelled `archogen-assignment/1` has one text, and names the same program and bindings to every reader of `/1` |

## `install-format`

| Field | Value |
| --- | --- |
| Surface | the manifest an install writes beside the programs it placed: the directory, the build they came from, the inventory's sha256, the front and each sibling with its role and sha256 (`M3.5.1.3`, [Installing](cli.md#installing)) |
| Version | `archogen-install/1` |
| Declared at | `const:crates/archogen-install/src/lib.rs:FORMAT` |
| Changes when | a member is added, removed, renamed, re-typed or reinterpreted. A front that meets another identifier refuses the install rather than guess |
| Pinned by | its shape — every key path and the kind of value at it — frozen in `crates/archogen-install/tests/goldens/archogen-install-1.golden` and compared by `crates/archogen-install/tests/manifest.rs`, never rewritten; the same test reads every manifest back as itself |
| Keeps | a manifest labelled `archogen-install/1` has one text, and names the same programs to every reader of `/1` |

## `wasm-request-format`

| Field | Value |
| --- | --- |
| Surface | the request a page writes to the wasm binding: length-prefixed fields for the description, its profile and its modules ([The engine API](engine-api.md), `docs/decisions/decision_wasm-binding.md` §4) |
| Version | `archogen-wasm-request/1` |
| Declared at | `const:crates/archogen-wasm/src/lib.rs:REQUEST_FORMAT` |
| Changes when | a field is added, removed, reordered or reinterpreted. A request naming another format is answered `usage`, never read |
| Pinned by | the framing tests of `crates/archogen-wasm/tests/binding.rs`: a request reads back as it was framed, and each refusal names its field and byte offset |
| Keeps | a loader that writes `archogen-wasm-request/1` is read the same way by every module that accepts it |

## `wasm-response-format`

| Field | Value |
| --- | --- |
| Surface | the JSON the wasm binding answers with, one fixed encoding of the engine API's response (`docs/decisions/decision_wasm-binding.md` §6) |
| Version | `archogen-wasm-response/1` |
| Declared at | `const:crates/archogen-wasm/src/lib.rs:RESPONSE_FORMAT` |
| Changes when | a key is added, removed, renamed or re-typed, or the encoding of a value changes |
| Pinned by | its shape — every key path and the kinds of value at it — frozen in `crates/archogen-wasm/tests/goldens/archogen-wasm-response-1.golden` and compared by `crates/archogen-wasm/tests/binding.rs`, never rewritten; the same test compares every field with the API's response |
| Keeps | a page that reads `archogen-wasm-response/1` finds every key where it was, with the same kind of value |

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
