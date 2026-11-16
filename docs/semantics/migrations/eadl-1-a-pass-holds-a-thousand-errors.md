# A pass holds a thousand errors, and one more says the rest went unsaid

- version: eadl/1
- date: 2026-11-06
- leaf: M4.2.3.3.18 (`docs/tasks/M4.md`)
- status: applied
- constructs: docs/semantics/reference.md#diagnostics, docs/semantics/reference.md#normative-sources
- invalidates: none

## What changed

§4's diagnostics table gained a row, `too-many-errors`. One set of diagnostics holds a thousand errors. The next one is replaced by this, which stands where it would have been, and any after it go unsaid. The limit is `crates/eadl-front/src/diagnostic.rs`'s `MOST_ERRORS`.

The reference's list of normative sources gained `crates/eadl-front/src/diagnostic.rs`, since §4's census is taken over that list in both directions and this is where the code is emitted.

## Why

A file of junk got one error for each byte, each of them about 2 KiB once said. The hosted build's round 18 measured 131.3 MiB for 64 KiB of NUL bytes, and 3.46 GiB for 64 MiB before the host guard stopped it, on a machine other projects share. Nobody reads the thousandth. The same 64 KiB now costs 6.3 MB.

The cap is where every pass records and not in the reader, so what a later pass says of a file of forms is bounded too. Warnings are not counted: a warning refuses nothing, and a description with many of them must not be refused for that.

## Which descriptions it invalidates

None. A file reaches this only past a thousand errors, so it was refused before and it is refused now, with the same status. What changes is how much it is told. The frozen verdicts in `crates/archogen-cli/tests/verdicts.txt` are unchanged.

## Which version it lands in

`eadl/1`, as a correction: no description changes meaning, and the language accepts exactly what it accepted.
