# An elaborated declaration is named by its instance path, and a name resolves in the scope that wrote it

- version: eadl/1
- date: 2026-09-29
- leaf: M1.29.3 (`docs/tasks/M1.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics, suite/docs/semantics/modules/app.sibling.eadl, suite/docs/semantics/modules/bad.not-exported-transitive.eadl, suite/docs/semantics/modules/bad.not-exported.eadl, suite/docs/semantics/modules/hw.bus.eadl, suite/docs/semantics/modules/hw.private.eadl
- invalidates: none in this repository, measured — no command read a module tree's declarations before this change; outside it, a module that names an import's declaration the import does not export, which now is `module-not-exported` where before no command resolved it at all

## What changed

`docs/semantics/reference.md` §6 gained **rule 9** — a declaration named `n` in the instance at alias path
`p` is `p.n`, and a root declaration keeps its name — and **rule 10**: the operands of `uses`, `needs` and
`refines` resolve in the writing instance's scope, its own declarations first, then `alias.n` where `n` must
be one of that import's module's exports, and otherwise the name is capability vocabulary, left as written.
There is no re-export. §4 gained one row, `module-not-exported`, which is the frozen construct that moved.
§6 also states, as an open limit owned by leaf `M1.33`, that nothing yet makes a name mean one declaration.

The suite gained `docs/semantics/modules/hw.bus.eadl` and `hw.private.eadl` (library) and three cases:
`app.sibling` (a module naming its own siblings in all three positions — accepted, and refused without rule
10), `bad.not-exported` and `bad.not-exported-transitive`.

## Why

`M1.29.2` wired elaboration into the commands and had to stop there: the model passes read declarations by
name, and a name written inside an imported module meant nothing until a rule said what it resolves to.
Checking an elaborated tree without one would have reported a module's own sibling as a missing fact.

## Which descriptions it invalidates

None that any command could read before: every module tree was answered `unimplemented` after elaboration.

## Which version it lands in

`eadl/1`. The rules govern module trees, which no command type-checked before this change.
