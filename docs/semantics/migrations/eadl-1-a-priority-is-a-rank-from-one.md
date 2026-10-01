# A task priority below 1 is refused, because a priority is a rank from 1

- version: eadl/1
- date: 2026-10-01
- leaf: M2.13 (`docs/tasks/M2.md`)
- status: pending
- constructs: suite/docs/semantics/cases/invalid-priority-below-one.eadl
- invalidates: none in this repository, measured; outside it, a description giving a task `(priority 0)` or a negative priority

## What changed

`archogen check` refuses a task whose `priority` is below 1, with the model layer's new code
`priority-below-one` (`docs/semantics/model.md` §4 rule 5, and its row in §6), whose verdict is
`invalid-description`. The rule is the language's, not a profile's: a priority is a rank, an integer from 1,
where 1 is the highest and a larger number a lower priority. The ranks of a system need not be contiguous.

The one frozen construct that moves is the new worked case, `docs/semantics/cases/invalid-priority-below-one.eadl`;
no table of `docs/semantics/reference.md` changes, because the model layer's codes are stated in `model.md`.

## Why

`docs/decisions/decision_priority-comparison-direction.md` decided on `2026-09-13` that "a description carrying
`(priority 0)` is refused", and nothing refused it. Measured on the parent commit, with the clause of
`docs/semantics/cases/positive-minimal-system.eadl` changed:

```console
$ archogen check <the case, with `(priority 0)`>
<…>: accepted against profile `rt-static-up-v1` (4 declaration(s))
$ archogen check <the case, with `(priority -3)`>
<…>: accepted against profile `rt-static-up-v1` (4 declaration(s))
```

while the runtime's lowering, `rt_core::Scheduler::from_eadl_ranks`, refuses both at boot. A description the
runtime cannot build passed the check — the shape `M1.28.2` fixed for units.

No description changes meaning. The record never gave a rank below 1 one — it said "`1` is the highest" — so a
description carrying one had no defined meaning to lose; refusing it states what the language already was.

## Which descriptions it invalidates

None in this repository, measured by `grep -rnoE "\(priority -?[0-9]+\)" --include=*.eadl .` outside `target/`
and `vendor/`: every task's priority is from 1 to 3. One other clause carries `(priority -1)`, in
`docs/feedback/linkedspec/issues/LS-002-multi-form-truncation/evidence/10-semantically-invalid.eadl`, but on a
`defsystem`, whose kind has no such clause, so the schema refuses it first (`schema-unknown-clause`) and its frozen
verdict in `crates/archogen-cli/tests/verdicts.txt` did not move: the table gained one line, the new case's, and
changed none. Outside the repository, a description giving a task `(priority 0)` or a negative priority, which is
now refused with a repair direction.

## Which version it lands in

`eadl/1`, as a correction: the version's meaning is unchanged, and only an input it never gave a meaning is
refused.
