# The engine core carries no external Rust dependencies

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** repo-local, established at leaf `PROGRAM.2`

## The fact / decision

The archogen engine crates — the CLI, the frontend, the model, the resolver, the planner,
the emitter, the checker, and the analyses — depend on the Rust standard library and nothing
else. An external crate enters only through an explicit, reviewed decision record that names
the trust category it lands in and the claims it can contaminate.

This is not a purity exercise. It is the cheapest way to satisfy three obligations the
roadmap already imposes.

## Why

1. **§4.4 makes every dependency a trust artifact.** Each generator, configuration checker,
   scheduling checker, and reference-model build must emit a machine-readable
   dependency/provenance inventory, and every *newly shared* item between the generator and
   the checker blocks acceptance until reviewed. A shared transitive helper — an argument
   parser, a serializer, an error type — is exactly the "loss of independence" the F30 gate
   exists to surface. Every dependency not taken is a review that never has to happen and an
   independence argument that stays simple.

2. **§10.3 forbids acquisition during a build.** "Builds do not call an LLM or fetch new
   semantic knowledge to resolve missing inputs. Online dependency acquisition is a separate
   explicit operation. Locked builds fail on missing or mismatched inputs." An empty
   dependency graph makes a locked, offline, reproducible build the default state rather than
   a discipline to maintain.

3. **The diagnostics are the user contract.** §5.5 requires every diagnostic to carry source
   spans, requirement and offer IDs, the supported profile, a conflict set, and a concrete
   repair direction. A general-purpose argument or serialization library owns its own error
   wording, and that wording then leaks into a contract it knows nothing about. The surface
   here is small and fixed by §10.2 — seven commands — so owning the parser costs less than
   owning a dependency's phrasing.

The cost is real and accepted: hand-written argument parsing, S-expression reading, and
serialization. Each is a bounded, well-understood problem at this project's scale, and each
is covered by its own fixtures.

## How to apply

- Do not add a `[dependencies]` entry to any crate under `crates/` without a decision record
  that states: what it does, why the standard library is insufficient, which trust category
  of §4.4 it lands in, whether it is reachable from both the generator and the checker, and
  which claims its compromise would invalidate.
- Development-only tooling is judged separately from anything reachable at build or check
  time, but must still be declared: §4.4 requires build scripts, procedural macros, and
  generated-source inputs in the inventory, not only ordinary runtime dependencies.
- Vendored or copied code is a dependency. Different crate names do not establish
  independent derivation (§4.4), and neither does a copy-paste.
- When this decision is eventually relaxed for a specific crate, supersede this record rather
  than editing it, so the audit trail shows what was traded and when.
- **Tightened `2026-09-30` for the catalog's gate** (`decision_catalog-records.md` §3, leaf `M2.7.1`). The gate
  resolves the whole workspace offline with `cargo metadata`, so every member, dev dependencies included, must
  depend only by path. A crate admitted under this record anywhere in the workspace now also needs that rule
  amended in the same change.

Related: [[doctrine-seams-vs-forking-a-check]] — the same instinct applied to tooling: state
where a thing belongs instead of bending what enforces it.
