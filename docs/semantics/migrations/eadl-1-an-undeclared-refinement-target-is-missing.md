# An undeclared refinement target is a missing fact, and its case is named for it

- version: eadl/1
- date: 2026-09-30
- leaf: M1.26.3 (`docs/tasks/M1.md`)
- status: applied
- constructs: suite/docs/semantics/cases/infeasible-refinement-target-missing.eadl, suite/docs/semantics/cases/missing-refinement-target.eadl
- invalidates: none — no description changes whether it is accepted; a refinement naming an undeclared target is refused with `missing-fact`, exit 11, where it was `infeasible-configuration`, exit 13

## What changed

The engine refuses a refinement whose target the description does not declare as `missing-fact`
(`docs/semantics/model.md` §3 rule 4). It used to be `infeasible-configuration`, which is §2 rule 2's
code for a required fact declared absent. So one code named two rules, which `model.md` §3 recorded as
finding `M1.26.3`.

The conformance suite's case for it was named for the old verdict, as every case is named for the verdict
it expects. It is renamed from `infeasible-refinement-target-missing.eadl` to
`missing-refinement-target.eadl`, and its header now says `expect: missing-fact`. Its forms are unchanged,
so its canonical form is unchanged. The baseline records the move as a construct removed and one added,
and this note covers both.

## Why

An undeclared name is not an infeasibility. `infeasible-configuration` says the constraints have no
satisfying assignment. A refinement against a platform described nowhere has no constraints to satisfy,
because the contract its obligations are owed to is unavailable. That is §5.5's definition of
`missing-fact`, and the repair is the same: supply the missing description, or correct the name.

## Which descriptions it invalidates

None changes whether it is accepted. A description whose only problem is an undeclared refinement target
now exits `11` rather than `13`. With another refusal as well, the verdict of higher precedence still
heads the report (`Verdict::precedence`: `infeasible-configuration` 70, `missing-fact` 60). So a description
that also violates an obligation still exits `13`. One whose other problem is a `missing-fact` used to exit
`13` because of the target alone, and now exits `11`. Measured over every tracked description on
`2026-09-30`: one moved, this case. Its frozen verdict in `crates/archogen-cli/tests/verdicts.txt` went
from `13 infeasible-configuration` to `11 missing-fact`.

## Which version it lands in

`eadl/1`, as a correction: a code now names one rule. It is not a new language version.
