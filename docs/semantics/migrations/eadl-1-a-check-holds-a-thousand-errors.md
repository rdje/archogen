# A check holds a thousand errors, and what it holds back still counts for its verdict

- version: eadl/1
- date: 2026-11-06
- leaf: M4.2.3.3.20 (`docs/tasks/M4.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics, docs/semantics/reference.md#normative-sources
- invalidates: none

## What changed

§4's row for `too-many-errors` says a check or a search, and not a pass, and that what is unsaid counts for the verdict. The normative source's line says one check.

## Why

`eadl-1-a-pass-holds-a-thousand-errors.md` said a pass, and that no description changed meaning. Neither held of the commit that landed it. The hosted build's round 20 found its cap read a system's first 2,000 tasks and no more, so two tasks sharing a name past that were accepted, and that a pass's early stop could hide a later verdict of higher precedence, changing an exit code. Both are corrected in this commit: a pass stops gathering early only where every error of it carries one verdict, and nothing stops reading for a count of what it has read.

## Which descriptions it invalidates

None. This commit restores what that one moved: a description with two tasks of one name past the 2,000th is refused again, as it was before it, and a file past a thousand errors has the status it had before it. The frozen verdicts in `crates/archogen-cli/tests/verdicts.txt` are unchanged.

## Which version it lands in

`eadl/1`, as a correction.
