# A module path, a module-name rule, and a worked case for every `module-` code a command can reach

- version: eadl/1
- date: 2026-09-29
- leaf: M1.29.2 (`docs/tasks/M1.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics, suite/docs/semantics/modules/app.system.eadl, suite/docs/semantics/modules/app.two-timers.eadl, suite/docs/semantics/modules/bad.bad-alias.eadl, suite/docs/semantics/modules/bad.bad-argument.eadl, suite/docs/semantics/modules/bad.bad-export.eadl, suite/docs/semantics/modules/bad.bad-import-name.eadl, suite/docs/semantics/modules/bad.bad-import.eadl, suite/docs/semantics/modules/bad.bad-param.eadl, suite/docs/semantics/modules/bad.bad-version-requirement.eadl, suite/docs/semantics/modules/bad.bad-version.eadl, suite/docs/semantics/modules/bad.bare-form.eadl, suite/docs/semantics/modules/bad.circular-import.eadl, suite/docs/semantics/modules/bad.conflicting-alias.eadl, suite/docs/semantics/modules/bad.conflicting-export.eadl, suite/docs/semantics/modules/bad.dangling-export.eadl, suite/docs/semantics/modules/bad.empty.eadl, suite/docs/semantics/modules/bad.incompatible-version.eadl, suite/docs/semantics/modules/bad.missing-argument.eadl, suite/docs/semantics/modules/bad.missing-name.eadl, suite/docs/semantics/modules/bad.missing-version.eadl, suite/docs/semantics/modules/bad.multiple-forms.eadl, suite/docs/semantics/modules/bad.name-mismatch.eadl, suite/docs/semantics/modules/bad.not-a-module.eadl, suite/docs/semantics/modules/bad.not-found.eadl, suite/docs/semantics/modules/bad.unknown-import-clause.eadl, suite/docs/semantics/modules/bad.unknown-parameter.eadl, suite/docs/semantics/modules/cycle.b.eadl, suite/docs/semantics/modules/cycle.c.eadl, suite/docs/semantics/modules/hw.misnamed.eadl, suite/docs/semantics/modules/hw.nothing.eadl, suite/docs/semantics/modules/hw.plain.eadl, suite/docs/semantics/modules/hw.sized.eadl, suite/docs/semantics/modules/hw.soc.eadl, suite/docs/semantics/modules/hw.timer.eadl, suite/docs/semantics/modules/os.time.eadl
- invalidates: none in this repository, measured — no tracked description imported anything before this change, because no command elaborated a module; outside it, an import whose module name is not dotted lowercase segments (§6 rule 8), which was never resolvable by any command either

## What changed

`docs/semantics/reference.md` §6 gained two rules and one stated limit. **Rule 7**: the module an import
names is read from `<name>.eadl` in the **module path** — the directory holding the description the command
was given — and verified by the name its `defmodule` declares, so rule 4's mismatch stays a live rule. A
file that exists and cannot be read is a failure of the invocation, not a statement about the description.
**Rule 8**: a module name is dotted lowercase segments, because rule 7 turns it into a file name; any other
name is `module-bad-import`. **The limit**: `module-too-large` has no fixture, because it fires on a
four-gigabyte source.

§4's `module-not-found` row is the one frozen construct that moved. It said "check the name, or add the
directory holding it to the module path", and there is no way to add a directory to a module path that is
defined as one directory; it now says where the module is read from and what to do.

The new suite root `docs/semantics/modules/` is the other 35 constructs: 9 library modules and 26 cases —
F01's two compositions (`app.system`, the book's opening example, and `app.two-timers`), F02's cycle and
conflicting export, and one case for every other `module-` code a command can reach. Each case's header
declares the verdict and the one code it must produce, and `crates/archogen-cli/tests/module_cases.rs`
drives every one through `archogen check` and `archogen build`.

## Why

No command elaborated a module tree (`M1.29`), so §6's rules were enforced only where a library test
called the elaborator directly, and none of its 24 codes was reachable from a command. Wiring the
elaborator into the commands needs a rule for how an import finds its file; the rule had to be written
before it was implemented, and a code a command reaches needs a case a reader can run.

## Which descriptions it invalidates

None that any command could have read before. The only behaviour an existing description could observe is
a module file given to `archogen check`: it was answered `invalid-description` (`schema-unknown-kind`)
before `M1.29.1`, `unimplemented` after it, and is elaborated now.

## Which version it lands in

`eadl/1`. Nothing that was accepted is refused and nothing that was refused changes meaning — the rules
added govern files no command read.
