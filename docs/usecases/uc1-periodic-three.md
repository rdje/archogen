# `uc1-periodic-three` — the straight-through case

**Status:** supported. **First complete gate:** M4 (a sketch of it drives S0).

Three periodic tasks on a single-core platform with one timer and one observable output. This
is the case the roadmap names in its own CLI examples (`examples/periodic-three/system.eadl`)
and the one §12 M4 requires to "build from a clean checkout without edits to output".

## What it requires of the toolchain

| Concern | Requirement |
| --- | --- |
| Workload | three periodic tasks, static unique priorities, constrained deadlines |
| Time | a monotonic time service and periodic release management |
| Output | one observable output device, enough to assert an event from outside |
| Analysis | fixed-priority response-time analysis with the runtime's declared overheads |
| Evidence | per-property statuses; no global verified flag |

## The synthetic workload

Higher priority first. Times are milliseconds; these are **synthetic**, chosen so the set is
schedulable with visible slack under the idealized model, leaving room for real overheads to
matter without dominating.

| Task | Priority | Period T | Deadline D | Release |
| --- | --- | --- | --- | --- |
| `control` | 1 (highest) | 10 | 10 | periodic |
| `sense` | 2 | 20 | 15 | periodic |
| `report` | 3 (lowest) | 100 | 100 | periodic |

Execution bounds are deliberately **absent here**. They are evidence about a binary and enter
through the build manifest, never through the description — see the boundary corpus case
[`execution-bound`](../semantics/boundary/reject/execution-bound.eadl).

## Why this case first

It is the smallest system that still exercises every stage: elaboration, joint resolution with
a real timer binding, lowering, emission, a build, an analysis with more than one interfering
task, and a report with separate statuses. A two-task system would let a scheduler with a
broken priority comparison pass.

## What it must not require

No patching of generated Rust, linker scripts, or startup code. §1 makes that an explicit
condition of the first convincing result, and §12 M4 repeats it as an exit gate.
