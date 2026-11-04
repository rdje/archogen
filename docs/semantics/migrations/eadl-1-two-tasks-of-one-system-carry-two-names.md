# Two tasks of one system carry two names

- version: eadl/1
- date: 2026-11-04
- leaf: M1.50 (`docs/tasks/M1.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: a description one system of which declares two tasks of one name; two tracked ones, the runtime plan's own corpus's `dup-task-name.eadl` and `dup-task-three.eadl`

## What changed

§7 rule 6, "a name is declared once", held over top-level declarations alone, so a system declaring two tasks of one
name passed `archogen check` and the configuration checker. It now holds over one system's tasks too, with the same
code, `schema-duplicate-name`, and both sites named. Two systems' tasks may still share a name: a task is named within
its system.

## Why

Every rule keyed by a requester's name reads two such tasks as one: the search and the runtime plan merged them, and the
configuration checker resolved the name by order, the first task declared with it, which is what §7 rule 6 forbids
("never merged and never resolved by order"). The runtime plan's lowering refused it `tool-failure` meanwhile (`RTP-H23`).

## Which descriptions it invalidates

One whose system declares two tasks of one name, which is now `invalid-description`. The repair is to rename one. Two
frozen verdicts move, in `crates/archogen-cli/tests/verdicts.txt`, each from `0 -` to `10 schema-duplicate-name`:
`crates/archogen-plan/tests/corpus/dup-task-name.eadl` and `dup-task-three.eadl`, written to reach this refusal, which
the lowering gave them until now. Each is the case this rule refuses, so each keeps its description.

## Which version it lands in

`eadl/1`, as a correction of its specification. It is not a new language version.
