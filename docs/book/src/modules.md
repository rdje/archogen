# Modules and composition

A description is not one file. Reusable sub-hardware and sub-OS descriptions are imported with
namespaces, explicit exports, typed parameters and version constraints.

> **The normative rules are `docs/semantics/reference.md` §6, and every refusal this chapter shows is
> a row of its §4.** What an import binds, what an alias defaults to, when a version requirement is
> satisfied, why elaboration runs children before parents, and why a parameter with no default must be
> bound — those are stated there as rules, each cited to the test that enforces it, and the diagnostic
> codes are censused against the module reader in both directions. This chapter explains why the
> module system is shaped that way.

```text
(defmodule app.system
  (version 1 0)
  (import hw.soc  (as platform) (version (at-least 1 1)))
  (import os.time (as clock)    (version (at-least 2 0)))
  (export app.rt)
  (defsystem app.rt
    (requires (uses clock.time.monotonic))))
```

## What you can run today

`archogen check` **elaborates and type-checks** a module file. The example above is
`docs/semantics/modules/app.system.eadl`, and the modules it imports sit beside it — that directory is its
**module path**:

```console
$ archogen check docs/semantics/modules/app.system.eadl
docs/semantics/modules/app.system.eadl: accepted against profile `rt-static-up-v1` (4 declaration(s))
  elaborated from 4 instance(s): platform.timer = hw.timer 1.0, platform = hw.soc 1.2, clock = os.time 2.0, (root) = app.system 1.0
  this checks the description, not a system: no resolution, generation or analysis has run
$ echo $?
0
```

Two rules decide what an import reads, and both are normative (`docs/semantics/reference.md` §6):

- **Rule 7 — an import is found by a stated rule and verified by the name the module declares.**
  `(import hw.soc …)` reads `hw.soc.eadl` from the directory holding the description you gave the
  command. The file is *found* by the name you wrote and *checked* against the name its `defmodule`
  declares, so a file that calls itself something else is refused rather than trusted. A file that is
  missing is `module-not-found`; one that is there and cannot be read is a failure of the invocation
  (exit 2), exactly like an unreadable description — never "not found".
- **Rule 8 — a module name can only mean one file.** Dotted lowercase segments: `hw.soc`, `os.rt-core`.
  The name becomes a file name, so `../hw.soc` cannot leave the module path, and `HW.Soc` does not exist —
  on a filesystem that folds case it would be the same file as `hw.soc` on one machine and a different
  one on the next.

After elaboration the tree's declarations go through **every pass a single description gets** — the same
code, in the same order ([Checking a description](checking.md)) — once two more rules have said what their
names mean. They are the subject of [Names carry their whole path](#names-carry-their-whole-path) below.
`archogen build` answers a module file exactly as `check` does, and then meets the S0 path's own limits.

Still open, owned by leaf `M1.29.4`: module **parameters** reach no declaration yet — a `(with …)`
binding is recorded and used by nothing.

Every refusal this chapter shows is rendered by the command from a file in `docs/semantics/modules/`,
which holds one case for every `module-` code a command can reach; the one code without a case,
`module-too-large`, fires only on a four-gigabyte source.

## Elaboration produces instances, not modules

This is the design decision the whole module system turns on. `ROADMAP.md` §5.1.1:

> **Instantiate a module more than once without sharing mutable elaboration state.**

So importing the same module twice gives **two instances**, each with its own parameter
bindings and its own qualified names, and neither can observe the other:

```text
(import hw.timer (as fast) (with (tick-rate (tick-rate 100 MHz))))
(import hw.timer (as slow) (with (tick-rate (tick-rate   1 MHz))))
```

A design that cached one elaborated module per name would be smaller, and would silently make
the second import a no-op — so a system with two timers would have one, and nothing would say
so.

Instances come out in dependency order, children before parents. Every later pass —
initialization order, resolution, emission — relies on that.

## Names carry their whole path

**Rule 9.** A declaration in an imported instance is named by the path of aliases that reached it, and the
root's declarations keep their own names:

```text
platform.timer.timer.counter     ← hw.timer, imported by hw.soc, imported by app.system
platform.soc.bus
clock.time.monotonic
app.rt                            ← the root is unqualified
```

An alias defaults to the module's last dotted segment, which is what an author means nine times
out of ten and is still explicit in the resulting names. Two instances of one module therefore declare
different names — `fast.timer.counter` and `slow.timer.counter` — which is what lets them coexist.

**Rule 10.** A name a module *writes* — in `uses`, `needs` or `refines`, the clauses that name other
declarations — is resolved where it was written: first among that module's own declarations, by their
local names; then as `alias.name` through one of its own imports, which must be a name that import's
module **exports**; and otherwise it is a word of the capability vocabulary, like `counter-width`, and
is left as it is. So a reusable module names its own parts without knowing where it will be imported:

```text
(defmodule hw.bus
  (version 1 0)
  (export bus.main bus.fast)
  (defplatform bus.main
    (uses bus.timer)
    (offers (bus-width 32 bit)))
  (defplatform bus.fast
    (refines bus.main)
    (needs bus.clock)
    (offers (bus-width 32 bit)))
  (defblock bus.timer
    (offers (counter-width 32 bit)))
  (defblock bus.clock
    (offers (clock-rate 10 MHz))))
```

Imported as `board`, `bus.fast`'s `(refines bus.main)` means `board.bus.main` and its `(needs bus.clock)`
means `board.bus.clock` — without the rule both would be looked for at the root and reported missing:

```console
$ archogen check docs/semantics/modules/app.sibling.eadl
docs/semantics/modules/app.sibling.eadl: accepted against profile `rt-static-up-v1` (5 declaration(s))
  elaborated from 2 instance(s): board = hw.bus 1.0, (root) = app.sibling 1.0
  this checks the description, not a system: no resolution, generation or analysis has run
```

An **export** is what an importer may name, and there is no re-export: `app.system` imports `hw.soc`,
which imports `hw.timer`, but `hw.soc` exports only `soc.bus`, so the timer is not `app.system`'s to name:

```console
$ archogen check docs/semantics/modules/bad.not-exported-transitive.eadl
error[module-not-exported]: module `hw.soc`, imported as `platform`, exports no `timer.timer.counter`
  --> docs/semantics/modules/bad.not-exported-transitive.eadl:12:21
   |
12 |     (requires (uses platform.timer.timer.counter))))
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not exported by that import
  = hint: module `hw.soc` exports `soc.bus`, and declares no `timer.timer.counter` — a module's own imports are not visible to its importer, because §6 has no re-export (rule 10); import the module that declares it, or name one of those
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/modules/bad.not-exported-transitive.eadl
```

**A name means one declaration** (`docs/semantics/reference.md` §7 rule 6), and rule 9 is where a module
tree can break that: a module that declares a local `parts.public.part` while importing `hw.private` as
`parts` — which declares `public.part` — has given two declarations one name. It is refused, with both
sites named, in the two files they are in:

```console
$ archogen check docs/semantics/modules/bad.name-collision.eadl
error[schema-duplicate-name]: `parts.public.part` is declared twice
  --> docs/semantics/modules/bad.name-collision.eadl:11:13
   |
11 |   (defblock parts.public.part
   |             ^^^^^^^^^^^^^^^^^ declared again here
  --> docs/semantics/modules/hw.private.eadl:10:13
   |
10 |   (defblock public.part
   |             ----------- first declared here
  = hint: rename one — a name means one declaration (§7 rule 6). In a module tree a declaration is named by its instance path and its local name (§6 rule 9), so a local `a.x` beside an import aliased `a` that declares `x` is one name: `parts.public.part`
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/modules/bad.name-collision.eadl
```

## Precise composition errors

"There is a cycle" is a puzzle. The whole chain is a diagnostic — and it is reported at the import
that closes it, in the file where that import is written:

```console
$ archogen check docs/semantics/modules/bad.circular-import.eadl
error[module-circular-import]: circular import: bad.circular-import → cycle.b → cycle.c → bad.circular-import
  --> docs/semantics/modules/cycle.c.eadl:9:3
  |
9 |   (import bad.circular-import))
  |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ this import closes the cycle
  = hint: break the cycle by moving the shared declarations into a module that both import, or by removing one edge of bad.circular-import → cycle.b → cycle.c → bad.circular-import
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/modules/bad.circular-import.eadl
```

A conflicting export names **both** sites, because the author looking at one of them cannot see
the other:

```console
$ archogen check docs/semantics/modules/bad.conflicting-export.eadl
error[module-conflicting-export]: `thing` is exported twice by module `bad.conflicting-export`
  --> docs/semantics/modules/bad.conflicting-export.eadl:11:11
   |
11 |   (export thing)
   |           ^^^^^ exported again here
  --> docs/semantics/modules/bad.conflicting-export.eadl:10:11
   |
10 |   (export thing)
   |           ----- first exported here
  = hint: export each name once; two exports of one name give an importer two answers and no rule for choosing between them
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/modules/bad.conflicting-export.eadl
```

Also refused: two imports bound to the same alias (every qualified name becomes ambiguous), an
export naming something the module does not declare, a module whose declared name differs from
the name it was imported by, a missing required parameter, and an unknown parameter — the last
listing the real ones.

Only a **cycle** stops elaboration, because continuing into one does not terminate. Everything
else is collected, so a description with three composition problems costs one edit cycle.

## Versions

`(version (at-least 1 0))` means the same major version and at least that minor. A **major**
difference is never satisfied, however much newer the module is — and the refusal points at both the
requirement and the declaration it cannot be met by:

```console
$ archogen check docs/semantics/modules/bad.incompatible-version.eadl
error[module-incompatible-version]: module `hw.timer` is version 1.0 but 2.0 or compatible is required
  --> docs/semantics/modules/bad.incompatible-version.eadl:10:3
   |
10 |   (import hw.timer (version (at-least 2 0))))
   |   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required here
  --> docs/semantics/modules/hw.timer.eadl:7:1
  |
7 | (defmodule hw.timer
  | ------------------- declared here (continues to line 12)
  = hint: major versions differ (1 vs 2), so no minor version can satisfy this import — §15 keeps a locked description's meaning, which a major bump is defined not to preserve
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/modules/bad.incompatible-version.eadl
```

"Newer" is not "compatible". An unversioned module is refused outright: an importer cannot state
a requirement against it, and §15 needs a locked description to keep its meaning.

The relation sits in its own form — `(at-least 1 0)` rather than a flat `at-least 1 0` — so
`at-most` and `exactly` can be added later without changing the shape around them.
