# The `priority` clause: a numerically lower value is a higher priority

- **Type:** `decision`
- **Date:** `2026-09-13`
- **Status:** `active`
- **Owner / source:** established by leaf `S0.1`, which needed a defined order for two
  releases at the same instant and found the direction was nowhere recorded.

## The fact / decision

In an eADL `(task … (priority N))` clause, **`N` is a rank, not a weight: a numerically lower
`N` is a higher priority, and `1` is the highest.** Where anything must order tasks — dispatch,
a release trace, the `hp(i)` set of a response-time recurrence — it sorts by ascending `N`.

## Why

The direction was **undeclared**, and both readings were live in the repository:

- `ROADMAP.md` §13.2 and §13.4 list their fixture tasks "higher priority first" (`A`, `B`, `C`;
  `H` before `L`) but never connect that ordering to a number a description could carry.
- `docs/semantics/kinds/os-rt.eadl` declares the clause as `(holds values integer)` and says
  "the number here is the policy, not the implementation" — which fixes its type and its side of
  the boundary, and says nothing about its comparison direction.
- The M0 examples imply the answer without stating it: in `examples/periodic-three/system.eadl`
  the 10 ms task is `priority 1`, the 20 ms task `priority 2` and the 100 ms task `priority 3`,
  which is rate-monotonic only if smaller means higher.

Census over the tracked tree at the commit that precedes this record
(`ARCHOGEN-PROGRAM-0021`, the parent of the commit that adds it):

```console
$ git grep -n 'priority' HEAD -- '*.md' '*.rs' '*.eadl' \
    | grep -Ei 'higher|lower|ascend|descend|rank' | wc -l
       5
```

Five lines: four of `ROADMAP.md` prose (§7.4, §13.2, §13.4) and one of
`docs/usecases/uc1-periodic-three.md`. **Every one of them orders a table** — "higher priority
is listed first", "`H` has higher fixed priority". Not one connects that ordering to the number
an eADL description carries, so nothing in the tree answered the question this record answers.

The reason this could not be left to inference: §15 says *"Do not silently reinterpret a
negative fact or change a parameter's comparison direction."* A rule that is only implied by
three example files is one refactor away from being changed silently, and the change would be
invisible: every description stays byte-identical while every ordering result inverts. The
first consumer that would have depended on it unstated is `M2.3`'s response-time recurrence,
where `hp(i)` is exactly "the tasks whose `N` is smaller".

The choice itself follows the examples rather than overruling them, and matches the ordinary
meaning of "priority 1" in a queue.

## How to apply

- **Sorting.** Ascending `N` is highest-first. A release trace emits coincident releases in
  ascending `N`; `hp(i)` is `{ j : N_j < N_i }`.
- **Uniqueness.** §3.1 requires static *unique* priorities, so ascending `N` is a total order
  within a system — no tie-break below this one is ever needed. The uniqueness half is already
  tested (`crates/eadl-model/tests/examples.rs::task_priorities_are_unique_within_a_system`).
- **Changing it is a language change, not an implementation change.** §15 puts a comparison
  direction under migration discipline: reversing this needs a new language/profile semantic
  version, a migration note, and a compatibility-corpus entry — never a quiet edit.
- Related: [[decision_eadl-engine-boundary]] — the *number* is policy and belongs in eADL; the
  ready-queue structure that realizes the order is engine-owned and does not.
