# Compiler provenance repair contract and prototype

`M3.6.9.6.3.2.2.15.2.1.3.3.1` (`2026-10-18`) selects Cargo's published
`RUSTC_WRAPPER` interface for a separate instrument-owned compiler record. The documented first
argument is the actual compiler, followed by its arguments. The global wrapper covers dependencies;
the workspace-only wrapper is not used. The [toolchain ledger](../../book/src/ledger.md#rust-toolchain)
pins the public documentation and actual producer. Unit collection implements this contract in
`.3.3.2.1`; artifact binding in `.3.3.2.2`, with complete compatibility and final
source/integration verification afterward. The table describes the complete target contract.

| Boundary | Required behavior |
| --- | --- |
| Producer | Compile an embedded, tracked, std-only standalone observer with the pinned rustc, once per inventory invocation. Install verified bytes under each role's own target directory. No user wrapper, workspace wrapper or compiler override is inherited. |
| Invocation | Retain the base environment constructor and fixed `RUSTC=rustc`; add only explicit observer/session/role fields. Forward native arguments and stdio, remove instrument fields before the real compiler, retain working directory and exit/signal outcome. |
| Lifecycle | Fresh same-volume session per root, harness or generator build; no reuse of previous records. Create each partial record exclusively before running the compiler; publish a completed record without replacement only after the compiler finishes. Any setup/write/publication error fails the build. |
| Consumer | Read the private directory after Cargo succeeds. Require exact version/schema, current session/role, regular non-symlink records, invocation identities, byte framing, working directory and successful compiler status. Refuse empty, partial, stale, malformed or unknown records explicitly. No fallback to diagnostic text. |
| Queries | Distinguish the pinned version/target-information queries from actual compilation. Unknown invocation shapes refuse. Queries produce no inventory units. Every compilation must retain complete argv and have the existing metadata owner, dep-info, source/file, environment and configuration checks. |
| Artifact | Bind the selected role target to an observed compilation and its actual output. Query pinned rustc with empty input and only observed naming arguments; validate complete basename outputs under the observed out-dir, regular files and hashes. No root artifact is selected from arbitrary Cargo stdout text. Implemented `.3.3.2.2`. |
| Determinism | Ephemeral observer/session paths never enter inventoried configuration or shared-item identity. Preserve full compiler arguments, existing normalization and sorted units. Raw native evidence may record runtime path bytes; no project lookup or persisted configuration depends on them. |
| Scope | Use the same protocol for binary/library roots, comparison test harnesses and generator entry points. Keep manifest/configuration/dynamic-target preflight and source admissions. Existing supported path geometry and copied execution must retain their contracts. |
| Failure | Refuse provenance unable to be judged, leave no inventory and retain useful Cargo diagnostics on a genuine Cargo failure. Wait for all children; preserve partial evidence for diagnosis. The next invocation starts fresh. |

The [prototype record](../../book/src/annex-trust-proofs.md#compiler-provenance-contract-and-prototype)
retains replay commands, native controls, source identities, corrections and limits.

## Production unit collector and current premise revalidation

`2026-10-18`, `.15.2.1.3.3.2.1`: the controller/embedded observer implement the collection
rows above for both compile callers. Base environment/preflight and argv remain. Records require
current schema/session/role/cwd, matching PID/filename and successful status. Unknown, malformed,
partial, stale or unsuccessful provenance stops the inventory. UTF-8 interpretation is explicit.
Artifact binding is implemented; compatibility and source/integration gates follow.

The production record retains the verified P2/native controls, bounded diagnostic matrix,
legacy catalogue/reason retirements, historical controls and current premise revalidation.
Baseline evidence repair is .3.3.3.0; bounded results do not establish full source adequacy.

Exact commands, controls, outcomes, failed attempts and scope are in the
[production record](../../book/src/annex-trust-proofs.md#production-compiler-invocation-collection-repair).
All new judgment files join .3's scope. The
[artifact record](../../book/src/annex-trust-proofs.md#observed-compiler-artifact-binding) owns metadata/source
selection, empty-input naming, direct relative paths, output refusals and actual/copy controls.
The [role-warning matrix](../../book/src/annex-trust-proofs.md#compiler-warning-data-across-harness-and-generator-roles)
holds harness/generator success, source refusal, compiler failure and recovery; other compatibility stays open.
