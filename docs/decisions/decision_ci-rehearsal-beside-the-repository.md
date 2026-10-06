# The CI rehearsal checks out beside the repository, and `.archogen-data/` is a store inside it

- **Type:** `decision`
- **Date:** `2026-10-06`
- **Status:** `active`
- **Owner / source:** the director's two rulings of `2026-10-06`, taken by leaf `PROGRAM.65` (`docs/tasks/PROGRAM.md`);
  an exception to [[decision_scratch-on-the-repository-volume]]
- **External sources:** [the Rust toolchain](../book/src/ledger.md#rust-toolchain) — Cargo's configuration discovery,
  which reads every `.cargo/config.toml` on a build's directory path; version and limits in the ledger

## The fact / decision

1. **The CI rehearsal's checkout lives beside this repository, on the same volume:** `../.archogen-ci-rehearsal/`,
   which `scripts/ci_rehearse.sh` makes before a run and removes after it (`CI_REHEARSE_KEEP=1` keeps it). It is the one
   exception to "scratch under `target/`".
2. **A git-ignored `.archogen-data/` at the repository's root is a store** this repository's tooling may write and
   manage, in the director's words: *"At the root of the ARCHOGEN repo you can create a git ignored .archogen-data/
   where you can store and manage stuff there."*

## Why

A checkout inside this repository has this repository's tracked `.cargo/config.toml` on every one of its builds'
directory paths. Cargo reads it, and the trust instrument refuses it, rightly, by the catalog's §3 rule it adopts: *"a
cargo configuration above the repository, on a build's path (§3)"*. The runner's checkout has none above it. Measured
by `PROGRAM.64`, `2026-10-06`: a whole rehearsal of `a3c0cbd` under `target/ci/rehearsal/` failed 43 tests for that
reason alone — 38 `trust`, 5 `catalog_build` — with the machine's `HOME` as with the job's; no whole rehearsal had
passed since those tests landed (`M2.7.4.3`, `2026-10-03`; `M3.6.2`, `2026-10-05`).

`.archogen-data/` was the director's first offer. It is the right place for data this repository keeps, but not for
the checkout: being inside the repository, it has the root's configuration above it too. Of the options put to the
director — a sibling directory, the trust design admitting an alias-only configuration above the repository (the
catalog harness's open `M2.7.6.4` proposal, a normative change to a reviewed record), or leaving the rehearsal red —
the sibling directory was ruled: faithful to the runner and no design change. The bytes stay on this repository's
volume, which is what the data-locality rule protects.

## How to apply

- `scripts/ci_rehearse.sh` is the only thing that writes outside the repository's tree. Before anything runs it checks
  that no `.cargo/config` or `.cargo/config.toml` lies on the checkout's directory path, and says "could not run" if
  one does — the precondition a fixture modelling "outside a repository" checks
  ([[decision_scratch-on-the-repository-volume]]).
- Anything else this repository keeps lives under `target/` or `.archogen-data/`, both ignored by git.
- Related: [[decision_incomplete-blocking-policy]], the CI job the rehearsal imitates.
