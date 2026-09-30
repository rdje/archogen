# What is versioned, and what changing it costs

`ROADMAP.md` §15 separates the version of the language from the versions of the engine, catalog
entries, device and timing models, and evidence formats. It also requires that "a source description
retains its meaning under its locked semantic version". This page lists every versioned surface this
repository has today. For each one it gives where the version is declared, what change forces a new
version, what refuses an unannounced change, and what a description or artifact written against it can
rely on.

`VERSION-REGISTER` (`scripts/check_version_register.sh`) keeps this page in step with the code. It reads
every version the code declares: each format identifier, each version constant, each profile id, the
engine API's version, and the engine version in the workspace's manifests. Each one must belong to an entry below with the same value,
and every entry's declaration must still exist. A new format, or a bump, therefore cannot land without
this page changing too.

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

## `analysis-model`

| Field | Value |
| --- | --- |
| Surface | the model the scheduling analysis reasons in, which every conclusion names ([What the scheduling checker establishes](analysis.md)) |
| Version | `idealized-zero-overhead/1` |
| Declared at | `const:crates/rt-analysis/src/response.rs:MODEL` |
| Changes when | an assumption of the model changes: what costs nothing, what is atomic, what is known. A changed model invalidates every conclusion stated in the old one |
| Pinned by | the tests in `crates/rt-analysis/src/response.rs`, and the conclusions printed in `docs/book/src/analysis.md`, which quote the model by name |
| Keeps | a conclusion "in `idealized-zero-overhead/1`" means the same bound under the same assumptions |

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
| Surface | the engine API: the one contract every consumer judges a description through, the CLI today and the wasm and MCP bindings to come ([The engine API](engine-api.md)) |
| Version | `1.2` — `1.0` fixed `2026-09-30` when leaf `API.3` closed; `1.1` added the response's engine version (`API.4.1`); `1.2` added `check_with` and its `Limits` (`API.4.2`) |
| Declared at | `const:crates/archogen-api/src/lib.rs:VERSION` |
| Changes when | an operation or a response field is added, which is a minor; an operation or a field removed, or a field given a new meaning, which is a new major |
| Pinned by | the exhaustive reading of `Response` and `Judgement` in `crates/archogen-api/tests/check.rs`, which stops compiling when a field is added or removed and names the version its shape belongs to; the three parity legs of `crates/archogen-cli/tests/api_parity.rs`. The bump itself is the author's act: nothing can tell a new field from a renamed one |
| Keeps | within a major, what a consumer reads from a response stays true, and its outcome vocabulary only grows |

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

- **Catalog records**, the engine knowledge that realizes descriptions, are being designed in `M2.7`:
  `docs/decisions/catalog/decision_catalog-records.md`, under independent review, versions each part of a record apart.
  Catalogs at scale belong to `M6`. F25, "a locked rebuild after a catalog update", lives at `M6.4`.
- **Device and timing models** belong to `M2`.
- **The build plan and its identity**, the plan hash and the binary hash, belong to `M4.7`.
