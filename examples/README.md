# Examples

Two corpora live here. The **use cases** are the systems the engine will eventually build; the
**S0 corpus** is the tiny early-generation fixture set of `ROADMAP.md` §12 S0.

## The use cases

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

## The S0 corpus

[`s0-heartbeat/`](s0-heartbeat/) holds the three descriptions of fixture **F28** and the
observations they must produce, frozen before any emitter existed. It is a different kind of
object from the use cases: not a system anyone wants, but the smallest one that can prove the
pipeline's shape. Its README carries the observation contract and the list of temporary S0
assumptions `S0.6` must retire.

| File | Case | `archogen check` | `archogen build` |
| --- | --- | --- | --- |
| [`s0-heartbeat/system.eadl`](s0-heartbeat/system.eadl) | base | `0` | `0` + 4 releases over a 30 ms hyperperiod |
| [`s0-heartbeat/system-changed.eadl`](s0-heartbeat/system-changed.eadl) | one period changed | `0` | `0` + 3 releases over a 20 ms hyperperiod |
| [`s0-heartbeat/system-unsupported.eadl`](s0-heartbeat/system-unsupported.eadl) | sporadic release | `0` | `12` — no engine realization |

The unsupported case is **accepted** by `archogen check` and refused by generation, which is the
point of it: §3.1 admits sporadic releases, so the gap is in the engine, not in the description.

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
