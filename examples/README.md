# Examples

The four use cases of `docs/usecases/`, expressed in eADL. They are the corpus the frontend is
tested against, and the systems the engine will eventually build.

| Example | Use case | Expected outcome |
| --- | --- | --- |
| [`periodic-three/`](periodic-three/) | `uc1` | builds |
| [`high-interference/`](high-interference/) | `uc2` | builds; the timing property may return `not-established` |
| [`alternative-timer/`](alternative-timer/) | `uc3` | refused before `M3.2`, builds after — `infeasible-configuration` today |
| [`bounded-queue/`](bounded-queue/) | `uc4` | **must stay refused** — `unsupported-profile` |

All numbers are **synthetic** — not measurements, not attributed to any published task set, not
claims about any board.

## What is not here

No execution bounds, no entry points, no stack sizes. §7.3 splits the task record three ways and
only one third is a description:

| Lives in | What |
| --- | --- |
| eADL | the functional requirements and constraints |
| the build manifest | application-code association, execution bounds, and their evidence |
| the engine | concrete allocations, stack placement, other implementation fields |

Each of the three exclusions has a worked rejection case in `docs/semantics/boundary/reject/`:
[`execution-bound`](../docs/semantics/boundary/reject/execution-bound.eadl),
[`application-entry-point`](../docs/semantics/boundary/reject/application-entry-point.eadl),
[`stack-allocation`](../docs/semantics/boundary/reject/stack-allocation.eadl). A test asserts
that every example here passes the boundary classifier, so the claim is checked rather than
asserted.

Every example runs through `archogen check`, and a test asserts the verdict each one produces:

```console
$ archogen check examples/periodic-three/system.eadl  ; echo $?   # 0  accepted
$ archogen check examples/alternative-timer/system.eadl ; echo $? # 13 infeasible-configuration
$ archogen check examples/bounded-queue/system.eadl   ; echo $?   # 12 unsupported-profile
```

`uc3` and `uc4` are refused for different reasons, and the difference matters. `uc3`'s refusal is
**temporary**: the same description must build once `M3.2` supplies indirect realization, without
changing. `uc4`'s refusal is **permanent** for this profile: admitting it requires a new named
profile with its own analysis obligations, not a wider `rt-static-up-v1`.
