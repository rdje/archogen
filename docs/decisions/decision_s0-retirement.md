# The S0 retirement contract: every temporary assumption, and the leaf that removes it

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `retired` `2026-11-04` by leaf `M4.2.4`: `crates/archogen-s0` is deleted, and this record now says what became of each assumption
- **Owner / source:** leaf `S0.6`, closing the `S0` tree. Required by `ROADMAP.md` §12 S0's
  retirement clause.

## The fact / decision

The S0 path was a **prototype with a stated expiry**. Every hard-coded assumption it rested on was listed here with the
task-tree leaf that removes it, marked in the source at the line that made it, and held by a doctrine, `S0-RETIREMENT`, on every
commit. The prototype and that doctrine are both gone.

> ROADMAP.md §12 S0: *"The prototype implementation can be discarded or replaced as semantics
> settle. Keep its functional fixtures and documented pipeline lessons. By M4, the supported path
> replaces any temporary hard-coded assumptions with checked engine plans; **no hidden
> special-case generator is grandfathered into the release.**"*

## What became of each assumption

| id | What S0 hard-coded | What became of it |
| --- | --- | --- |
| `horizon-is-one-hyperperiod` | the generated system was observed for exactly one hyperperiod | **moved**: a named rule of the hosted build's scenario, not an assumption (`decision_hosted-build.md` §3); a run of a part of a hyperperiod is a scenario `M4.3` may add |
| `whole-millisecond-periods` | every period an exact whole number of milliseconds | **discharged**: exact integer ticks (§2), a test running `250 us` |
| `periodic-releases-only` | a task with a minimum separation refused, no event source modeled | **discharged**: the simulator's sources, a test counting a source's arrivals; device models stay `M4.5`'s |
| `fixed-observation-format` | the `system` / `release` / `summary` format fixed in the engine | **discharged**: `archogen-playground-trace`, registered; the rest of §6.3's event set stays `M4.3`'s |
| `stdout-is-observable-output` | `observable-output` realized by standard output, with no device model | **moved** to `M4.3`: no job writes anything yet, and the trace is the simulator's own |
| `single-fixed-realization` | one realization selected unconditionally | **discharged** by `M3.1`: the search |
| `plan-is-not-independently-checked` | the plan nothing re-validated | **discharged** by `M3.5`: the configuration checker, which `archogen build` runs |
| `catalog-record-is-a-stub` | provenance's `realization` a stub of a §9 catalog entry | **discharged**: the build's provenance has no such member; catalog records are `M2.7`'s |
| `no-assurance-report` | no per-property report, the build claiming nothing | **moved** to `M4.8`: the build still claims nothing, and says so |
| `no-lock-data` | no lock data, so `--locked` refused rather than honored | **moved** to `M4.7`: the supported path's own refusal, since `M4.2.3.1` |

## What survived, and what did not

| Artifact | Survived? |
| --- | --- |
| `examples/s0-heartbeat/*.eadl` — the four descriptions | **Yes.** §12 S0: "Keep its functional fixtures." They are feature-only descriptions and outlive any realization |
| `examples/s0-heartbeat/expected/*.txt` — the frozen observations | **Yes**, byte for byte but for the refusal, whose move is recorded in its own header. They judge the supported path |
| the oracle — now `crates/archogen-cli/tests/functional_fixtures.rs` | **Yes.** Its derivation of the contract and its shape, a frozen literal plus an independent re-derivation that no emitter can call, both |
| `crates/archogen-s0/` — the realization | **No.** Deleting it was the retirement |

## Why

The failure this prevented is quiet, not dramatic. A prototype that works becomes the thing
everybody builds on, and the assumptions that were obvious while it was being written become
invisible a milestone later. Three properties make that harder here:

1. **The crate is not named for a responsibility it does not own.** §4.2's names for the real
   pipeline are `archogen-plan` and `archogen-emit`; a prototype holding them would stop looking
   like a prototype. `archogen-s0` reads as temporary in every backtrace and every `Cargo.toml`.
2. **The assumptions are marked at the line that makes them**, not only listed in a document. A
   list drifts from the code; a marker cannot be moved without touching the code it annotates.
3. **The prototype may not acquire consumers.** The check enforces that only the S0 tests may
   import it (`crates/archogen-cli/src/build_cmd.rs` did until `M4.2.3.1` moved the command onto
   the plan), so M4 cannot build on it by accident — which is what "grandfathered" would actually
   look like in practice.

⚠️ **Honest limit.** None of this forces an author to *write* a marker when they hard-code
something new. It makes an unmarked assumption a thing someone chose not to record rather than a
thing nobody noticed, and it makes every recorded one impossible to lose. That is the same bargain
the acceptance checklist makes, and it is worth stating rather than overselling.

## How to apply

- Nothing to apply: the crate, its markers, its doctrine and `scripts/check_s0_retirement.sh` are gone, and a later prototype with
  an expiry can take this shape again. A task-tree or review that cites them reads as history.
- Related: [[decision_eadl-engine-boundary]] — S0 stayed on the right side of it; the assumptions above were all *engine*
  decisions, and none of them leaked into a description. [[decision_hosted-build]] is what replaced it.
