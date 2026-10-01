# A task's overrun policy is one the profile's runtime performs

- version: eadl/1
- date: 2026-10-01
- leaf: M2.14 (`docs/tasks/M2.md`)
- status: applied
- constructs: suite/docs/semantics/cases/positive-skip-late-job.eadl, suite/docs/semantics/cases/unsupported-overrun-policy.eadl
- invalidates: none in this repository, measured; outside it, a description whose task declares an `on-overrun` policy other than `fault` or `skip-late-job` against `rt-static-up-v1`

## What changed

`archogen check` refuses a task whose `(on-overrun …)` names a policy `rt-static-up-v1`'s runtime does not
perform, with `unsupported-profile` (`docs/semantics/model.md` §4 rule 6). The profile performs two, spelled
`fault` and `skip-late-job` in eADL (`ROADMAP.md` §3.1.1 rule 5), and a task without the clause has `fault`, the
profile's default. The kind still declares `(holds values symbol)`: the domain is the profile's, not the
language's, because another profile could perform more — which is why the verdict is `unsupported-profile`,
the same as for two tasks sharing a priority.

The frozen constructs that move are the two new worked cases, one admitted and one refused. The kind file's comment
changed too, which the baseline does not see: canonical form carries no comment.

## Why

`ROADMAP.md` §3.1 requires a *defined* overrun policy, and §3.1.1 rule 5 defines two. Measured on `2026-10-01`
before this change, with the clause added to `docs/semantics/cases/positive-minimal-system.eadl`:

```console
$ archogen check <the case, with `(on-overrun banana)`>
<…>: accepted against profile `rt-static-up-v1` (4 declaration(s))
```

A lowering has nothing to map an unknown policy to, so a description the runtime cannot perform passed the check.
Found by the independent review of §3.1.1 (`decision_runtime-contract-gaps.md`, finding 3).

## Which descriptions it invalidates

None in this repository: the 20 `on-overrun` clauses in tracked descriptions before this change all say `fault`,
as do all but the two new cases' after it, measured by
`grep -rhoE "\(on-overrun [a-z-]+\)" --include=*.eadl .` outside `target/` and `vendor/`, and the frozen verdicts
in `crates/archogen-cli/tests/verdicts.txt` gained the two new cases' lines and changed none. Outside it, a
description declaring any other policy against `rt-static-up-v1`.

## Which version it lands in

`eadl/1`, as a correction: no description with a policy the runtime performs changes meaning, and the absent
clause keeps the meaning the profile always gave it.
