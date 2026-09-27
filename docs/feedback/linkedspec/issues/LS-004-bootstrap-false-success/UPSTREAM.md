# LinkedSpec completion notice — LS-004

Delivered by the LinkedSpec maintainer on 2026-09-27 at the director's request.
State: **fixed-upstream; ARCHOGEN verification pending**.

## Published fix

LinkedSpec **`fd3e328d5dd5c80981a1c3b8496a27270291f7b8`** is published on `origin/main`; an independent remote
read-back matched the exact commit after a clean push. It adopts RGX
**`f6e5acdc99720349d1e3ecef9f821f365c4db19c`** through RGX's published integration contract.

[LinkedSpec fix commit](https://github.com/rdje/linkedspec/commit/fd3e328d5dd5c80981a1c3b8496a27270291f7b8) ·
[public failure and success evidence](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/checkpoints/RGX-CONSUMER-BUILD-REPORTS.1.1.json) ·
[adoption and native compatibility evidence](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/checkpoints/RGX-CONSUMER-BUILD-REPORTS.1.2.json).

## What was executed

The old RGX `8763a0e6bea97879f027237439d57725f83ead23` public bootstrap reproduced the report:
an empty offline package store caused a missing-package error, but later steps ran and the
false seed-success line appeared. Its eventual exit status was already nonzero.

At the adopted revision, two independent empty-store controls exit 2 at the first missing
prerequisite, without later named steps or the false seed-success text. Fresh public bootstrap
also exits 0, and two prepared offline reuse controls exit 0. This verifies both the failure
path and supported successful use through `make bootstrap`, without dependency implementation
inspection or patches. RGX is the integration contact; the original report's PGEN attribution
is preserved without claiming an independent source-level diagnosis.

Fresh LinkedSpec Rust products pass the complete Rust component gate, the unchanged 105-case
corpus oracle, generated execution, workspace controls and native file consumers. A discovered
LinkedSpec public-entry parity gap and a strict-document invalid-prefix gap were repaired and
tested before release. The 37 original document cases plus 14 new prefix cases pass on Perl,
Rust, Dart, Julia, PUC Lua and LuaJIT, including public-loader and native-file delivery.

The exact staged candidate passes canonical local CI: all nine doctrines, required native
admissions, both 66-case CLI environments and all **1,033 Phase0 regression tests**. The gate's
receipt was promoted to the published commit and reused by the clean pre-push check. Optional
matrices not enabled by that canonical configuration are not claimed as part of this run.

## Adopting it

Preserve any consumer-local work before updating the vendored checkout. Fetch LinkedSpec and
select the exact published commit above, then initialize its RGX submodule. Follow the pinned
[Rust integration guide](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/linkedspec-book/src/public-api/integration-rust.md)
for workspace exclusions, project-local storage, public preparation and the fresh application
build. The older `a8d34c845` publication does not contain the LS-004 adoption.

Existing generated output can make bootstrap a no-op. Follow section 8 of the adopted
[RGX public integration guide](https://github.com/rdje/rgx/blob/f6e5acdc99720349d1e3ecef9f821f365c4db19c/docs/INTEGRATION.md)
when updating a prepared checkout, or use a fresh checkout and fresh build products. Keep
project-owned data on the repository filesystem and do not discard local changes or shared
caches. LinkedSpec preserved its older checkout and outputs while verifying fresh products.

The original `repro.sh`, setup and frozen observation in this issue remain historical evidence.
The equivalent upstream failure check uses RGX's public `make bootstrap` with a fresh local
empty Cargo store and offline resolution, as described in the linked evidence; it does not
reconstruct or directly invoke a transitive dependency's build procedure.

## Consumer verification

Please adopt the named revision and verify through the supported public integration route.
Only ARCHOGEN should change this issue to `verified` after its own measurement. This notice
does not claim an ARCHOGEN application build or independent consumer acceptance.

The same published LinkedSpec history also contains the workspace and guide remedies, plus
the opt-in complete, kind-preserving document route. For those document requirements use
`sexpr_file` with `SExprDocumentV1.spec`; the historical Lispish extraction behavior remains
documented and preserved. Every report's current response is recorded in its own issue envelope.
