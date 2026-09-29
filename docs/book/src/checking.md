# Checking a description

```console
$ archogen check examples/periodic-three/system.eadl --profile rt-static-up-v1
examples/periodic-three/system.eadl: accepted against profile `rt-static-up-v1` (8 declaration(s))
  this checks the description, not a system: no resolution, generation or analysis has run
```

That second line is not modesty for its own sake. Acceptance means the description is
well-formed, in profile, and internally consistent. It is **not** a statement that any system
built from it will behave — the evidence categories start after this point.

## The passes, in the order a failure makes the next meaningless

| Pass | Owns | Verdict on failure |
| --- | --- | --- |
| read | syntax and spans | `invalid-description` |
| boundary | implementation content (F27) | `invalid-description` |
| schema | the declaration frame | `invalid-description` |
| profile | the capabilities the profile refuses | `unsupported-profile` |
| workload | the task model the profile admits | `unsupported-profile`, `missing-fact`, `invalid-description` |
| presence | offered / absent / undescribed | `missing-fact`, `infeasible-configuration`, `invalid-description` |
| refinement | the three obligations | `infeasible-configuration` |

Every pass runs. Every diagnostic is collected. A description with three problems costs one edit
cycle, not three.

The `profile` and `workload` passes are two halves of one contract and it is worth keeping them
apart. `profile` consults the capabilities `rt-static-up-v1` **refuses** — `general-ipc`,
`runtime-heap`, `dma`. `workload` consults the task model it **admits** — unique priorities,
constrained deadlines, a declared release model. The second half was prose in the profile table
and enforced nowhere until leaf `M1.9`, which is why a description with two tasks at priority 1
used to be accepted. See [Describing a workload](workload.md).

## The verdict is what to fix first

When several passes complain, the **verdict** is chosen by precedence — and precedence here
means *what to fix first*, not severity of consequence:

```text
tool-failure  >  invalid-description  >  unsupported-profile  >  infeasible-configuration
              >  missing-fact  >  counterexample  >  not-established  >  analysis-inconclusive
```

A malformed description makes every later answer meaningless, so it outranks everything. An
unsupported request outranks a missing fact, because describing that fact would be wasted work
on a system the profile will refuse anyway.

Precedence chooses the **headline**, not what the author gets to see. Both diagnostics are still
printed.

And an out-of-profile capability is not *also* reported as a missing fact. Telling someone that
`general-ipc` "is required and nothing describes it" invites them to go and describe it, on a
capability the profile refuses either way.

## Exit codes

The verdict maps to the process exit code through one table, and a test asserts the mapping is
total — so the number a script branches on and the word a human reads come from the same place:

```console
$ archogen check examples/alternative-timer/system.eadl ; echo $?
error[infeasible-configuration]: `absolute-deadline` is required by this system but declared absent
  --> examples/alternative-timer/system.eadl:51:25
   |
51 |   (needs time.monotonic absolute-deadline))
   |                         ^^^^^^^^^^^^^^^^^ required through this request
  --> examples/alternative-timer/system.eadl:26:11
   |
26 |   (absent absolute-deadline))
   |           ----------------- declared absent by `timer.delay`
  = hint: either the requirement or the platform is wrong; an explicitly absent fact is a
          definite answer, not a gap to be filled in
13
```

That is `uc3`, behaving exactly as the use case says it should *today*: the platform offers only
a relative delay timer, the workload requires absolute deadlines, and no indirect realization
exists yet. When `M3.2` lands, the same description must build — without changing.

## Refused by name

```console
$ archogen check examples/bounded-queue/system.eadl ; echo $?
error[unsupported-profile]: `general-ipc` is not admitted by profile `rt-static-up-v1`
  = hint: admitting it would add: needs queue capacity, overflow semantics, and their
          response-time effects; a later profile amendment. Request a profile that supports it,
          or remove the requirement — it is never silently reduced to a weaker guarantee
12
```

The refusal names the capability, the profile, and the **obligation admitting it would add** —
so it reads as a statement about work rather than a wall.

## The language definition travels with the binary

The kind modules are embedded at compile time, not read from the working directory. `archogen` must
behave identically wherever it is run from, and a language definition that could be shadowed by
a file in the current directory is a language definition an accident can change.

If those embedded modules ever fail to load, the result is `tool-failure` (exit 70) — never a
verdict about the user's description. §5.5: a tool failure is never reported as a valid system.

## The semantic corpus

`docs/semantics/cases/` holds 30 worked cases — §12 M1 asks for twenty — each declaring the
verdict it expects in its own header, and each run through this pipeline:

| Expected | Cases |
| --- | --- |
| `ok` | 6 |
| `invalid-description` | 11 |
| `unsupported-profile` | 7 |
| `infeasible-configuration` | 4 |
| `missing-fact` | 2 |

The expectation lives **in the case**, not in the driver. A driver that computed what to expect
would agree with itself forever.

The suite also enforces the §5.5 contract on every diagnostic the corpus produces — a located
span and a concrete repair direction — rather than on the handful a unit test happens to build.
And an accepted case must be **silent**: "accepted with three warnings" is a shape this pipeline
does not have, so an author never has to judge which messages mattered.
