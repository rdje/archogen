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

```text
platform.timer.timer.counter     ← hw.timer, imported by hw.soc, imported by app.system
platform.soc.bus
clock.time.monotonic
app.rt                            ← the root is unqualified
```

An alias defaults to the module's last dotted segment, which is what an author means nine times
out of ten and is still explicit in the resulting names.

## Precise composition errors

"There is a cycle" is a puzzle. `a → b → c → a` is a diagnostic.

```text
error[module-circular-import]: circular import: a → b → c → a
  = hint: break the cycle by moving the shared declarations into a module that both import, or
          by removing one edge of a → b → c → a
```

A conflicting export names **both** sites, because the author looking at one of them cannot see
the other:

```text
error[module-conflicting-export]: `thing` is exported twice by module `dup`
  --> dup.eadl:4:20
  |
4 |            (export thing)
  |                    ^^^^^ exported again here
  --> dup.eadl:3:20
  |
3 |            (export thing)
  |                    ----- first exported here
  = hint: export each name once; two exports of one name give an importer two answers and no
          rule for choosing between them
```

Also refused: two imports bound to the same alias (every qualified name becomes ambiguous), an
export naming something the module does not declare, a module whose declared name differs from
the name it was imported by, a missing required parameter, and an unknown parameter — the last
listing the real ones.

Only a **cycle** stops elaboration, because continuing into one does not terminate. Everything
else is collected, so a description with three composition problems costs one edit cycle.

## Versions

`(version (at-least 1 0))` means the same major version and at least that minor. A **major**
difference is never satisfied, however much newer the module is:

```text
error[module-incompatible-version]: module `os.time` is version 2.0 but 1.0 or compatible is required
  = hint: major versions differ (2 vs 1), so no minor version can satisfy this import — §15 keeps
          a locked description's meaning, which a major bump is defined not to preserve
```

"Newer" is not "compatible". An unversioned module is refused outright: an importer cannot state
a requirement against it, and §15 needs a locked description to keep its meaning.

The relation sits in its own form — `(at-least 1 0)` rather than a flat `at-least 1 0` — so
`at-most` and `exactly` can be added later without changing the shape around them.
