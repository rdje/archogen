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

`archogen check` **elaborates** a module file. The example above is
`docs/semantics/modules/app.system.eadl`, and the modules it imports sit beside it — that directory is its
**module path**:

```console
$ archogen check docs/semantics/modules/app.system.eadl
archogen: unimplemented: docs/semantics/modules/app.system.eadl elaborated into 4 instance(s), and no command type-checks an elaborated module tree yet
  instances: platform.timer = hw.timer 1.0, platform = hw.soc 1.2, clock = os.time 2.0, (root) = app.system 1.0
  hint: every composition rule of docs/semantics/reference.md §6 held. The declarations of an elaborated tree need the name rule of task-tree leaf M1.29.3 (docs/TASK_TREE.md) before the later passes can read them — how a name written inside an imported module resolves, and what an `export` hides
$ echo $?
20
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

**Every composition rule of §6 is enforced, and the tree is not yet type-checked.** Exit 20 is a
statement about the tool: the four instances above are exactly what the rules produce, and what stops the
command is the next step. The declarations of an elaborated tree need a **name rule** — how a name
written *inside* an imported module resolves, and what an `export` hides from its importer — before the
schema, presence and refinement passes can read them, and §6 does not state one yet. That is leaf
`M1.29.3`; checking the tree with names nothing resolves would report missing facts that are not missing.
Module parameters reaching the declarations they parameterize is `M1.29.4` — today a `(with …)` binding
is recorded and then used by nothing. `archogen build` answers a module file exactly as `check` does.

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

```text
platform.timer.timer.counter     ← hw.timer, imported by hw.soc, imported by app.system
platform.soc.bus
clock.time.monotonic
app.rt                            ← the root is unqualified
```

An alias defaults to the module's last dotted segment, which is what an author means nine times
out of ten and is still explicit in the resulting names.

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
