# Compiler provenance repair contract and prototype

`M3.6.9.6.3.2.2.15.2.1.3.3.1` (`2026-10-18`) selects Cargo's published
`RUSTC_WRAPPER` interface for a separate instrument-owned compiler record. The documented first
argument is the actual compiler, followed by its arguments. The global wrapper covers dependencies;
the workspace-only wrapper is not used. The [toolchain ledger](../../book/src/ledger.md#rust-toolchain)
pins the public documentation and actual producer; this is an implementation contract pending repair,
not a claim that current `compile` has stopped reading human output.

| Boundary | Required behavior |
| --- | --- |
| Producer | Compile an embedded, tracked, std-only standalone observer with the pinned rustc, once per inventory invocation. Install verified bytes under each role's own target directory. No user wrapper, workspace wrapper or compiler override is inherited. |
| Invocation | Retain the base environment constructor and fixed `RUSTC=rustc`; add only explicit observer/session/role fields. Forward native arguments and stdio, remove instrument fields before the real compiler, retain working directory and exit/signal outcome. |
| Lifecycle | Fresh same-volume session per root, harness or generator build; no reuse of previous records. Create each partial record exclusively before running the compiler; publish a completed record without replacement only after the compiler finishes. Any setup/write/publication error fails the build. |
| Consumer | Read the private directory after Cargo succeeds. Require exact version/schema, current session/role, regular non-symlink records, invocation identities, byte framing, working directory and successful compiler status. Refuse empty, partial, stale, malformed or unknown records explicitly. No fallback to diagnostic text. |
| Queries | Distinguish the pinned version/target-information queries from actual compilation. Unknown invocation shapes refuse. Queries produce no inventory units. Every compilation must retain complete argv and have the existing metadata owner, dep-info, source/file, environment and configuration checks. |
| Artifact | Bind the selected role target to an observed compilation and its actual output. Query pinned rustc with empty input and only observed naming arguments; validate complete basename outputs under the observed out-dir, regular files and hashes. No root artifact is selected from arbitrary Cargo stdout text. |
| Determinism | Ephemeral observer/session paths never enter inventoried configuration or shared-item identity. Preserve full compiler arguments, existing normalization and sorted units. Raw native evidence may record runtime path bytes; no project lookup or persisted configuration depends on them. |
| Scope | Use the same protocol for binary/library roots, comparison test harnesses and generator entry points. Keep manifest/configuration/dynamic-target preflight and source admissions. Existing supported path geometry and copied execution must retain their contracts. |
| Failure | Refuse provenance unable to be judged, leave no inventory and retain useful Cargo diagnostics on a genuine Cargo failure. Wait for all children; preserve partial evidence for diagnosis. The next invocation starts fresh. |

The durable instrument is `scripts/trust_compiler_provenance_probe.py`, with
`scripts/fixtures/compiler_provenance_observer.rs` and `compiler_provenance_receiver.c`.
It compiles the current actual environment constructor, then the separate observer and native C argv
receiver, entirely under a new repository-relative `target/` directory. The receiver independently
records NUL-separated native arguments, including empty, quoting, line-break, Unicode and non-UTF8
cases; exit7 and exact stdio agree, and private fields are absent. Actual signal15 maps to143 with a
signal record; a failed compiler spawn leaves a refused partial record. Setup failure exits2 before
the requested program executes. Concurrent native writers retain separate complete records.

The dated probe passes20 actual pinned Cargo builds: ordinary source, non-compiler warning,
rustc-looking warning, complete command-shaped warning and escaped JSON-looking stdout, each across
binary/library/harness/generator roles. Literal unit sets agree with Cargo artifacts;45 naming queries
agree on artifact bytes and leave every build file unchanged. The stdout experiment prints escaped
text, not valid artifact JSON: it demonstrates no artifact-report exploit and establishes no universal
absence. Fourteen separate malformed/stale/identity controls reject; the book lists them all.
Installed pinned public wrapper/print documentation hashes are recorded in the receipt.

All1067 held regular inputs restore at the activation epoch. Original production/test bytes, operators,
catalogue and equivalent reasons are unchanged. Exploratory errors (binary emit shape, assembly brace
escaping and JSON-looking text) remain retained and corrected in the durable instrument. The
[book contract](../../book/src/annex-trust-proofs.md#compiler-provenance-contract-and-prototype)
gives the replay command, exact matrix, evidence and limits. `.3.3.2.1` owns the bounded producer/unit
repair and actual public red/green control; `.2.2` artifact binding; `.2.3` compatibility/failure controls.
Each production change must retire or revalidate affected old helper/reason premises in the same slice.
`.3.3.3` owns complete current source/mutation/premise and milestone integration verification.
Independent review and final F30 adequacy remain open; this prototype is not production signoff.
