# The engine's adapter catalog joins the language's definitions

- version: eadl/1
- date: 2026-10-27
- leaf: M3.2.2 (`docs/tasks/M3.md`)
- status: pending
- constructs: suite/docs/semantics/kinds/defadapter.eadl, suite/docs/semantics/adapters/adapters.eadl
- invalidates: none, measured — every tracked description's `archogen check` verdict, frozen in `crates/archogen-cli/tests/verdicts.txt`, is unchanged; the table gains the two new files at exit 20; outside this repository, a description that writes a `defadapter` outside a module, which was `invalid-description` (`schema-unknown-kind`) and is now answered exit 20 as the engine's own knowledge

## What changed

The suite gained the engine's adapter catalog, as it gained the vocabulary. `docs/semantics/kinds/defadapter.eadl`
declares the kind `defadapter` with `defkind`, its clauses as `docs/specs/adapters/decision_engine-adapters.md` §1.3
sets them. `docs/semantics/adapters/adapters.eadl` is `adapters/1`: one `defadapter` entry per row of the record's
§2 table, `deadline-over-delay` and `epoch-extender`, each fixing what an instance requires of the provider it
programs, what it offers over it, the level its code runs at, and what it owes. `docs/semantics/conformance.md` gained
the root `docs/semantics/adapters`, and the kinds root's purpose names `defadapter.eadl`.

No description's registry holds `defadapter`. So a `defadapter` inside a module is `schema-unknown-kind`, as before;
any other file that writes one is the engine's own knowledge, which `archogen check` answers exit 20, as it answers
the vocabulary — the engine API's `Definition::Adapter`, API `1.6`.

## Why

`ROADMAP.md` §5.2 and §5.4 make an adapter "a real provider with costs and proof obligations", supplied by the engine.
The record decides its contract is data, read twice — this file by the generator, the configuration checker's own
table beside it, held equal by `crates/eadl-resolve/tests/adapters.rs` — so the file is frozen here, a kind changing
only by a note like this one. The search binds no instance yet (`M3.2.3`), so no verdict moves with it.

## Which descriptions it invalidates

None in this repository. Outside it, a description with a `defadapter` outside any module, which was refused
`schema-unknown-kind` and is now answered as the engine's own knowledge, exit 20 — a file nobody could have meant as a
system.

## Which version it lands in

`eadl/1`: the language's definitions grew by the engine's catalog, and no description that meant one thing now means
another.
