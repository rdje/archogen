# archogen

**archogen** turns a functional description of a platform and a workload — written in
**eADL** — into a complete specialized operating system, a matching development simulator,
and a report stating exactly what has been checked and under which assumptions.

## The controlling boundary

eADL describes **functionality**: what a platform offers, what a system requires, and the
constraints that must hold. It contains **no implementation**. Algorithms, device
implementations, register-programming sequences, simulator models, provider selection,
lowering rules, and code generation belong to the engine and its versioned knowledge bases.

This is not a stylistic preference. It is the property that makes a description reusable
across implementations, and it is enforced mechanically rather than by review habit.

| eADL describes | The engine decides and implements |
| --- | --- |
| Available timer functionality and required time-service properties | Counter extension, compare programming, interrupt handling, simulator behavior |
| Task-execution functionality, priority semantics, deadline constraints | Scheduler data structures, dispatch code, context saving, cost analysis |
| Memory capacity and required protection functionality | Concrete layout, region tables, privileged operations |
| Hardware connectivity and supported operating states | Drivers, initialization order, transition sequences, state-dependent models |
| Reusable sub-HW/sub-OS feature compositions | Realization graph, provider bindings, generated Rust, verification execution |

## What is being built first

The first supported family is the `rt-static-up-v1` profile: one active core, static unique
task priorities, a finite static task set, statically allocated stacks, no runtime heap, a
timer interrupt plus explicitly modeled bounded interrupt sources. Requests outside that
profile receive an `unsupported-profile` diagnostic — never a silently weakened guarantee.

## What this project does not claim

- No global "verified" flag. Configuration validity, functional behavior, timing, memory
  bounds, and startup behavior each carry their own status and their own evidence category.
- Testing does not become proof through repetition, a published algorithm proof does not
  verify its Rust implementation, and a declared capability is not hardware evidence.
- The first release may accurately provide *conditional* timing analysis plus *tested*
  implementation conformance, and must list the links that remain unproved.

## Reading order

This book is the project's public surface and is kept in lockstep with the code. Where a
chapter describes behavior, it names the code or fixture that implements it. The direction
and its exit gates live in `ROADMAP.md`; execution is tracked as task-trees under
`docs/tasks/`.

> Status: early. Chapters appear as the milestones that own them land.
