# Every diagnostic row names an input that fires it, and `module-too-large` states its real mechanism

- version: eadl/1
- date: 2026-09-30
- leaf: M1.26.2.2 (`docs/tasks/M1.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: none — no description changes meaning; the table gained a column, and one row's account of when it fires was corrected to what the code does

## What changed

§4's diagnostics table gained a fourth column, **`fires on`**. Each cell is an input the toolchain runs:
`check` (a description), `modules` (a description and the module files beside it), `kinds` (a kind
module, and declarations validated against it) or `none:` with the reason no writable input can fire
the code. `crates/archogen-cli/tests/fires_on.rs` executes every row and requires the row's code among
the diagnostics its input produces.

One row's "when it fires" was wrong, and the column is what exposed it (`F-H`). `module-too-large` said
"a module has more addressable parts than an instance identifier can hold", a mechanism that exists
nowhere in the code. It fires where `SourceMap::add` refuses a text whose length does not fit a `u32`,
the width of a span's byte offset: a module over 4 GiB. The row now says so, and its `fires on` cell is a
stated limit, because no fixture of any sane size carries such a module.

## Why

The census legs pinned each code to the source that emits it, and never to an input. A call site that
could no longer be reached satisfied them for ever, while §4 kept stating a rule the toolchain no longer
enforces. `F-H` was a live instance: a row describing a mechanism nobody had built, with both legs green.
An executed cell makes an unreachable row fail the build.

## Which descriptions it invalidates

None. No code changed, no rule was added or removed, and every description reads and checks as it did.
The correction is to the table's account of one refusal, not to the refusal.

## Which version it lands in

`eadl/1`, as a correction and an addition to its specification. It is not a new language version.
