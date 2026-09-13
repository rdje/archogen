# Examples

The four use cases of `docs/usecases/`, expressed in eADL. They are the corpus the frontend is
tested against, and the systems the engine will eventually build.

| Example | Use case | Expected outcome |
| --- | --- | --- |
| [`periodic-three/`](periodic-three/) | `uc1` | builds |
| [`high-interference/`](high-interference/) | `uc2` | builds; the timing property may return `not-established` |
| [`alternative-timer/`](alternative-timer/) | `uc3` | refused before `M3.2`, builds after |

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

`uc4-bounded-queue` has no example yet: it must be refused by **profile admission**, which is
leaf `M1.8`. Adding the file before the refusal exists would put a case in this directory that
nothing refuses.
