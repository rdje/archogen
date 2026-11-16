# A name is at most 256 characters

- version: eadl/1
- date: 2026-11-06
- leaf: M4.2.3.3.21 (`docs/tasks/M4.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: a description with a symbol of more than 256 characters; the longest tracked symbol is 57 characters, so none in this repository

## What changed

§4's diagnostics table gained a row, `read-name-too-long`. The reader refuses a symbol of more than 256 characters. The limit is `crates/eadl-front/src/reader.rs`'s `MAX_NAME`.

## Why

Every answer quotes names: each error, and on an accepted description each line of the closure. One name of any length multiplied what a request could cost: the hosted build's rounds 20 and 21 measured an answer 1,508 times its request through `archogen mcp`, within budget, and 65 MB on stdout from a 136 KB file that `check` accepted. The limit belongs to `eadl/1`, as the nesting limit does, and for the same reason: a limit a host's memory set would accept a description on one machine and fail on another. 256 is far above any real name.

## Which descriptions it invalidates

A description with a symbol past 256 characters, which was read and is now refused, exit `10`. Measured over every tracked description outside `vendor/` and the sealed set on `2026-11-06`: the longest symbol is 57 characters, so none moved, and the frozen verdicts in `crates/archogen-cli/tests/verdicts.txt` are unchanged.

## Which version it lands in

`eadl/1`, as a correction: a limit that existed as a cost is now a stated refusal.
