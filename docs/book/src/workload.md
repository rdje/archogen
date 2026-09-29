# Describing a workload

A task says what it must do and when. It does not say how, what code does it, or where its
stack goes.

```text
(defsystem periodic-three
  (task control
    (period 10 ms)
    (deadline 10 ms)
    (deadline-from release)
    (priority 1)
    (uses time.periodic-release)
    (on-overrun fault))
  …
  (platform (uses soc.playground)))
```

The `task` kind lives in the **`os/rt` feature module**
(`docs/semantics/kinds/os-rt.eadl`), declared with the same `defkind` primitive as everything
else — an extension, not a privileged addition.

## What a task carries

| Clause | Why |
| --- | --- |
| `period` / `min-separation` | the release model. The profile admits periodic or sporadic with a declared minimum separation |
| `deadline` + `deadline-from` | §7.3 requires "the relative deadline **and its reference event**" — a deadline measured from an unnamed instant is not checkable |
| `jitter` | bounded release jitter. Absent means zero, which is a claim the analysis relies on |
| `priority` | static and unique. The number is the policy; the ready-queue that realizes it is the engine's |
| `needs` / `uses` | what the task requires, functionally |
| `on-overrun` | a named response, not a handler body |

## What a task does not carry

§7.3 splits the task record three ways, and only one third is a description:

| Lives in | What |
| --- | --- |
| **eADL** | the functional requirements and constraints |
| **the build manifest** | application-code association, execution bounds, and their evidence |
| **the engine** | concrete allocations, stack placement, other implementation fields |

So three fields that *feel* like task properties are refused, each with a worked case in the
boundary corpus:

- **`wcet`** — an execution bound is evidence about a binary, valid only for one target and
  toolchain. §15 notes a compiler flag change invalidates it while the description is
  byte-identical.
- **`entry-point`** — naming the function a task runs makes the workload unbuildable against a
  different implementation of the same tasks, which is precisely the reuse the program exists to
  demonstrate.
- **`stack-allocation`** — the *requirement* is that each task has its own bounded stack; the
  *number* is an allocation the engine derives and §7.6 then checks against the actual linked
  image.

## Two mechanisms now catch all three

Before the `os/rt` module, `task` was declared `(holds forms)` — opaque — so a `wcet` hidden
inside one was caught only by the boundary classifier walking the whole tree. The measured reach
was **10 of 11** rejected corpus cases.

Giving `task` a real kind and writing `(holds kind task)` makes the schema recurse into it. The
reach is now **13 of 13**, and the refusal carries the boundary's wording rather than a generic
"unknown clause" — because `wcet` is not a typo, it is content in the wrong layer:

```text
error[boundary-implementation-in-description]: `wcet` is implementation, and eADL contains no implementation
  = hint: it fails the `externality` test … It belongs to the engine build manifest: §7.3 keeps
          bounds, their origin, and their target and binary identity outside eADL.
```

A registry that has not loaded `os-rt.eadl` **says so** rather than silently accepting whatever
is inside a task. Silently accepting an unvalidatable clause is the failure mode that let the
gap exist in the first place.

## The admitted task model, enforced

The profile does not only *refuse* capabilities; it also *admits* a particular shape of task
set, and that shape is checked:

| Rule | §3.1 wording | Refused as |
| --- | --- | --- |
| a release model is declared | "periodic or sporadic releases with declared minimum separation" | `missing-fact` |
| one release model, not two | — (§5.3: contradictions are rejected, not resolved) | `invalid-description` |
| priorities are unique | "static **unique** task priorities" | `unsupported-profile` |
| deadlines are constrained | "constrained deadlines" — `D ≤ T` | `unsupported-profile` |

```console
$ archogen check system.eadl
error[unsupported-profile]: tasks `beat` and `chime` share priority 1
  --> system.eadl:40:15
   |
40 |     (priority 1) (uses console.write) (on-overrun fault))
   |               ^ this priority is already taken
  --> system.eadl:37:15
   |
37 |     (priority 1) (uses console.write) (on-overrun fault))
   |               - first declared here
  = hint: `rt-static-up-v1` admits **static unique** task priorities. Give one of them a
          different number. Equal priorities need a documented tie-break — FIFO, round-robin —
          which is a different scheduling policy with its own analysis, so it belongs to a
          different profile rather than to a looser reading of this one
```

The last two are `unsupported-profile` and not `invalid-description`, and the distinction is not
cosmetic. Neither description is *wrong* about anything — both describe a system a different
profile could analyze. §3.1 requires such a request to be refused "rather than silently reducing
the requested guarantee", so the refusal says what admitting it would cost: equal priorities need
a tie-break policy, and `D > T` needs a busy-period analysis rather than the response-time
recurrence.

⚠️ **These four rules are the profile's *Workload* row, and that row is one of thirteen.** The
other twelve are still prose that nothing consults. That is not twelve defects — "Rust `no_std`
core" is a property of the engine, not of a description — but it was twelve rows nobody had
counted, and a test now counts them so the number cannot drift quietly. Classifying each by the
stage that can enforce it is tracked as leaf `M1.10`.

## The examples

`examples/` holds the three use cases as real descriptions, and they are checked, not just
stored:

- every declaration validates against the shipped kinds;
- every task has a deadline with its reference event;
- and **no example carries an execution bound, a code reference, or an allocation** — asserted
  by running the boundary classifier over each one, with all three constructs registered and all
  three exercised by worked corpus cases. That is what stops someone adding a WCET to an example
  to make an analysis pass.

`uc3` is checked for something unusual: that it still *contains its own contradiction*. Its
platform declares `absolute-deadline` absent while its service requires it — that gap is the
case. A test asserts both halves are still there, because the tempting "fix" is to delete one,
and the case would go quiet and stop testing anything.
