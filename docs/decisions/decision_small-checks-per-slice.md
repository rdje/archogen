# Small checks per slice; full verification after the feature set

- **Type:** `decision`
- **Date:** `2026-10-26`
- **Status:** `active`
- **Owner / source:** the director's verification doctrine, number 22 in the director's own numbering, given to the
  session on `2026-10-26` and recorded here word for word by leaf `PROGRAM.110`

## The fact / decision

The director's text, verbatim:

> ## 22. Verification Doctrine: Small Checks per Slice, Full Verification After Feature Completion
>
> Prioritize implementing the agreed feature set. Apply these rules:
>
> 1. After each slice, run only the smallest meaningful check of the changed behavior and directly affected functionality.
> 2. Fix failures exposed by that check, update the task tree and relevant documentation, commit, then continue immediately to the next slice.
> 3. Do not run full CI, broad regression suites, exhaustive audits, or verification matrices between implementation slices. Do not expand a focused check into a verification campaign.
> 4. Track deferred verification in a separate task-tree task. Activate it only after the agreed feature set is implemented.
> 5. Clearly distinguish implemented with focused checks passing from fully verified. Never imply that deferred verification has passed.
> 6. Preserve mandatory commit-integrity checks and post-push CI monitoring. This doctrine supersedes earlier requirements for broad local verification after every slice.

What it means here, slice by slice:

- **The smallest meaningful check** is the changed crate's own tests and its `cargo clippy -p <crate>`, and the trust
  inventory on a snapshot when a slice changes what a trust root compiles — seconds, and the gate CI runs on push.
- **Commit integrity stays mandatory**: the pre-commit hooks and every doctrine they run, and `make focused`, which
  `UNTRACKED-CODE` requires stamped on the staged tree.
- **What `make focused` runs stays at every commit**, since commit integrity is mandatory: its tier in
  `xtask/src/main.rs` is `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features -- -D warnings` and
  `cargo test --all` — the workspace's clippy, the whole xtask suite and the mutation sweep's premise tests
  (`xtask/mutation_sweep_tests.py`) among them. No slice runs them a second time beside it, and a pin a slice moves —
  the lockfile's, in that file — is re-pinned by that slice, its reason on the pin's line. Corrected `2026-10-26` by
  `PROGRAM.112`: this bullet first listed them as deferred, and `M3.5.4.5`'s focused run refused the pin it had left.
- **Deferred** until the agreed feature set is implemented: what `make focused` does not run — the integration tier and
  full CI, the mutation sweeps themselves, the census's oracle beyond the changed legs, and every other broad audit.
  Today's feature set is `M3.5`'s, the configuration checker, and its deferred verification is `M3.5.9`, pending until
  the last of `M3.5`'s feature leaves lands.
- **What a leaf says**: "implemented, focused checks passing" until `M3.5.9` has run; never "verified" before.
- A daily artifact cleanup keeps what makes it safe — the handoff census, regenerable caches alone, each removal
  proven gone — and drops the exhaustive identity audit of everything it kept.

## Why

- The director's goal, in their words: "The goal of this new doctrine is to ensure all the feature set is implemented as
  fast as possible with production-grade code." Speed is the aim; the code each slice lands is production grade —
  tested where it changed, lint-clean — not verified into shape later. And, in their words, "that full verification
  will follow after that's achieved": `M3.5.9` runs, whole, once the feature set is in.
- Before it, every leaf ran the trust inventory, the full xtask suite, workspace clippy and the fifty proofs, about ten
  minutes before each commit, and most of a leaf's time; the director puts the feature set first.
- It supersedes, for slices, the per-leaf chain the session's earlier practice ran, and nothing in
  `DOCTRINE_ENFORCEMENT.md`'s commit-time gates.

## How to apply

Run the changed crate's checks and the mandatory commit-integrity checks; commit; go on. File what a slice leaves
unverified under the feature set's deferred-verification task, and run that task, whole, only after the feature set.
