# A module tree is bounded: 1 024 instances, and import chains 16 modules long

- version: eadl/1
- date: 2026-09-30
- leaf: M1.39 (`docs/tasks/M1.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: a module tree past either limit; the largest tracked tree has 4 instances and a chain of 3, so none in this repository

## What changed

§6 gained rule 11, and §4 two rows. A module tree elaborates into at most 1 024 instances
(`module-too-many-instances`), and an import chain is at most 16 modules long, the root included
(`module-import-too-deep`). Elaboration stops at the first import past either limit and imports nothing more.
The limits are `MAX_INSTANCES` and `MAX_IMPORT_DEPTH` in `crates/eadl-front/src/module.rs`.

## Why

Measured `2026-09-30` with `archogen check`. Before the limits, a module imported twice at every link of a chain
doubled the tree at every link: 19 files of about 100 bytes elaborated into 524 287 instances in 12.8 s and held
1.8 GB. A chain of 3 000 modules overflowed the elaborator's stack and aborted the process, exit 134. Through the
engine API a consumer can supply the modules in memory, so either would take one request.

Both limits belong to `eadl/1`, as the list-nesting limit does (`eadl-1-a-list-nests-at-most-256-deep.md`), so
every elaborator refuses the same trees. They are far above the repository's largest tree and far below either
failure.

## Which descriptions it invalidates

A module tree past either limit. It was elaborated if the host could hold it, and otherwise it crashed or
exhausted memory. It is now refused, exit `10`. Measured over the tracked module fixtures on `2026-09-30`: the
largest tree has 4 instances and a chain of 3 modules. The frozen verdicts in
`crates/archogen-cli/tests/verdicts.txt` are unchanged.

## Which version it lands in

`eadl/1`, as a correction: limits that existed as a crash and an exhaustion are now stated refusals. It is not a
new language version.
