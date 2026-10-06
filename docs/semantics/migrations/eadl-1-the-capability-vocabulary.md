# The capability vocabulary is part of the language's definition

- version: eadl/1
- date: 2026-10-06
- leaf: M3.1.2.1 (`docs/tasks/M3.md`)
- status: applied
- constructs: suite/docs/semantics/kinds/deffact.eadl, suite/docs/semantics/vocabulary/vocabulary.eadl
- invalidates: none, measured — every tracked description's `archogen check` verdict, frozen in `crates/archogen-cli/tests/verdicts.txt`, is unchanged; the table gains the two new files at exit 20; outside this repository, a description that writes a `deffact` outside a module, which was `invalid-description` (`schema-unknown-kind`) and is now answered exit 20 as the language's own definition

## What changed

The suite gained two of the language's own definitions. `docs/semantics/kinds/deffact.eadl` declares the kind
`deffact` with `defkind`, its clauses as `docs/decisions/decision_substitutability-relation.md` §1.1 sets them.
`docs/semantics/vocabulary/vocabulary.eadl` is `/1`, the capability vocabulary: one `deffact` entry per row of
§1.1's table, each fixing a fact's value kind, whom it speaks for, and which way an offered value must lie against a
required one. `docs/semantics/conformance.md` gained the root `docs/semantics/vocabulary`, and the kinds root's
purpose now names `deffact.eadl` beside the `defkind` modules a description's registry is built from.

No description's registry holds `deffact`: the shipped registry stays `core.eadl` and `os-rt.eadl`. So a `deffact`
inside a module is `schema-unknown-kind`, as before, and one beside a `defmodule` `module-multiple-forms`; any other
file that writes one is the language's own definition, which `archogen check` answers exit 20, as it answers a kind
module (`SR-H3`).

## Why

`ROADMAP.md` §5.3 asks for a "versioned capability vocabulary", and §5.2 for "a documented comparison direction" for
every parameter the engine compares. The record decides the vocabulary is an eADL module the engine ships beside its
kinds, versioned with the language, an entry changing only by §15's migration process: so the file is frozen here,
and this note is the first such change. The relation that reads it, `crates/eadl-resolve`, is a library in `M3.1`;
`archogen check` does not call it, so no verdict moves with it.

## Which descriptions it invalidates

None in this repository. Outside it, a description with a `deffact` outside any module, which was refused
`schema-unknown-kind` and is now answered as the language's own definition, exit 20 — a file nobody could have
meant as a system.

## Which version it lands in

`eadl/1`: the language grew by its vocabulary, and no description that meant one thing now means another.
