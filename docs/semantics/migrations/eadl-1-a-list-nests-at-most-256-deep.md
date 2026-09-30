# A list nests at most 256 deep, and a deeper one is refused rather than crashing the reader

- version: eadl/1
- date: 2026-09-30
- leaf: M1.38 (`docs/tasks/M1.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: a description with a list opened inside 256 others; the deepest tracked description nests 6 levels, so none in this repository

## What changed

§4's diagnostics table gained a row, `read-nesting-too-deep`. The reader refuses a list that opens inside 256
others and reads nothing inside it. The limit is `crates/eadl-front/src/reader.rs`'s `MAX_NESTING`.

The table notation gained a marker, `<N*C>`, the one character `C` repeated `N` times. The new row's
`fires on` cell is 257 nested parentheses, and written out it would be 514 characters nobody could check by
eye. The notation table is not frozen. It is recorded here because the frozen row depends on it.

## Why

Before this, the reader recursed once per level with nothing to stop it. Measured on `2026-09-30` with a debug
build: 6 000 nested parentheses read, and 8 000 overflowed the stack and aborted the process, exit 134. That is
outside the exit contract, and it is neither a verdict nor `tool-failure`. A 16 KB file ended `archogen check`,
and it would end any server built on the engine API with one message. Every pass after the reader walks forms
recursively too, so the depth has to be refused where it is first seen.

The limit belongs to `eadl/1` rather than to a machine, as the 64-bit value domain does (§1 rule 9). A limit that
was whatever a reader's stack could hold would accept a description on one host and crash on another. 256 is far
above any real description, and far below the crash.

## Which descriptions it invalidates

A description with a list nested past 256 levels. It was either read, if the host's stack held it, or it crashed
the reader. It is now refused, exit `10`. Measured over every tracked description outside `vendor/` on
`2026-09-30`: the deepest nests 6 levels, so none moved, and the frozen verdicts in
`crates/archogen-cli/tests/verdicts.txt` are unchanged.

## Which version it lands in

`eadl/1`, as a correction: a limit that existed as a crash is now a stated refusal. It is not a new language
version.
