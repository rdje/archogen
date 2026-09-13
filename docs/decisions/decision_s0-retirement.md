# The S0 retirement contract: every temporary assumption, and the leaf that removes it

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** leaf `S0.6`, closing the `S0` tree. Required by `ROADMAP.md` §12 S0's
  retirement clause.

## The fact / decision

The S0 path (`crates/archogen-s0`) is a **prototype with a stated expiry**. Every hard-coded
assumption it rests on is listed below with the task-tree leaf that removes it, and the list is
**mechanically checked** rather than maintained by memory — `scripts/check_s0_retirement.sh` runs
on every commit and in CI.

> ROADMAP.md §12 S0: *"The prototype implementation can be discarded or replaced as semantics
> settle. Keep its functional fixtures and documented pipeline lessons. By M4, the supported path
> replaces any temporary hard-coded assumptions with checked engine plans; **no hidden
> special-case generator is grandfathered into the release.**"*

## The assumptions

Each row is marked in the source with an `S0-ASSUMPTION: <id>` comment at the line that makes
the assumption, and the check requires every marker to appear here and every leaf named here to
exist.

| id | What S0 hard-codes | Removed by |
| --- | --- | --- |
| `horizon-is-one-hyperperiod` | the generated system is observed for exactly one hyperperiod, because any other run length would be arbitrary | `M4.3` |
| `whole-millisecond-periods` | every declared period must be an exact whole number of milliseconds; S0's modeled clock has no finer tick | `M4.1` |
| `periodic-releases-only` | a task with a minimum separation is refused; no event source is modeled | `M4.3` |
| `fixed-observation-format` | the `system` / `release` / `summary` line format is fixed inside the engine and is not the §6.3 observation event set | `M4.3` |
| `stdout-is-observable-output` | `observable-output` is realized by writing to standard output, with no device model behind it | `M4.3` |
| `single-fixed-realization` | one realization is selected unconditionally; there is no candidate enumeration and no provider search | `M3.1` |
| `no-lock-data` | no lock data is emitted, so `--locked` is refused rather than honored | `M4.1` |
| `plan-is-not-independently-checked` | the plan the emitter consumes is the one the interpreter produced; nothing re-validates it against the description | `M3.5` |
| `catalog-record-is-a-stub` | `provenance.json`'s `realization` carries four of §9's fourteen catalog fields | `M2.7` |
| `no-assurance-report` | no per-property report is produced; the build makes no claim of any kind | `M4.8` |

## What survives, and what does not

| Artifact | Survives S0's retirement? |
| --- | --- |
| `examples/s0-heartbeat/*.eadl` — the four descriptions | **Yes.** §12 S0: "Keep its functional fixtures." They are feature-only descriptions and outlive any realization |
| `examples/s0-heartbeat/expected/*.txt` — the frozen observations | **Only while the observation contract holds.** They describe what *this* realization produces, which is why `provenance.rs` carries `REALIZATION_VERSION`: changing the contract must bump it and re-derive them, not quietly edit them |
| `crates/archogen-cli/tests/s0_oracle.rs` — the oracle | **Partly.** Its independent derivation of the contract retires with the contract; its *shape* — frozen literal plus independent re-derivation, and no ability for the subject to call it — is the lesson `docs/knowledge/an-oracle-is-independent-by-construction.md` keeps |
| `crates/archogen-s0/` — the realization | **No.** Deleting it is the retirement |

## Why

The failure this prevents is quiet, not dramatic. A prototype that works becomes the thing
everybody builds on, and the assumptions that were obvious while it was being written become
invisible a milestone later. Three properties make that harder here:

1. **The crate is not named for a responsibility it does not own.** §4.2's names for the real
   pipeline are `archogen-plan` and `archogen-emit`; a prototype holding them would stop looking
   like a prototype. `archogen-s0` reads as temporary in every backtrace and every `Cargo.toml`.
2. **The assumptions are marked at the line that makes them**, not only listed in a document. A
   list drifts from the code; a marker cannot be moved without touching the code it annotates.
3. **The prototype may not acquire consumers.** The check enforces that only
   `crates/archogen-cli/src/build_cmd.rs` and the S0 tests import it, so M4 cannot build on it by
   accident — which is what "grandfathered" would actually look like in practice.

⚠️ **Honest limit.** None of this forces an author to *write* a marker when they hard-code
something new. It makes an unmarked assumption a thing someone chose not to record rather than a
thing nobody noticed, and it makes every recorded one impossible to lose. That is the same bargain
the acceptance checklist makes, and it is worth stating rather than overselling.

## How to apply

- Adding a hard-coded decision to `crates/archogen-s0/`? Add `S0-ASSUMPTION: <id>` on the line,
  and a row here naming the leaf that removes it. The check fails until both exist.
- Removing one? Delete the marker, the row, and the code — in that order, so the check is green
  at every step.
- Retiring S0 entirely: this record's assumption table must be empty, at which point
  `crates/archogen-s0/` has no reason to exist. Delete it and set this record to `retired`.
- Related: [[decision_eadl-engine-boundary]] — S0 stays on the right side of it; the assumptions
  above are all *engine* decisions, and none of them leaked into a description.
