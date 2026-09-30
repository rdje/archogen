# A string must be followed by a delimiter, as the grammar always said

- version: eadl/1
- date: 2026-09-30
- leaf: M1.37 (`docs/tasks/M1.md`)
- status: pending
- constructs: docs/semantics/reference.md#diagnostics
- invalidates: a description with a string followed directly by a symbol character; the tracked corpus holds one, a bug report's evidence file, and no system description

## What changed

§4's diagnostics table gained a row, `read-missing-delimiter`. The reader now refuses a string followed
directly by a character a symbol may contain, such as `("b"c)`. Until now it read that as two atoms,
`"b"` and `c`.

The grammar is unchanged. `atom = ( string | number | symbol ) , ? delimiter` has always required a
delimiter after every atom, and the grammar-derived recognizer in `crates/eadl-front/tests/conformance.rs`
has always refused `("b"c)`. The reader was more permissive than its own specification, and no
conformance probe covered the case. A byte that can start nothing after a string, such as a control
character, keeps its own code, `read-unexpected-character`.

## Why

It was found by the third reader (`M1.22`). LinkedSpec's recognizer refused a document that archogen's
reader accepted, and the grammar sided with the recognizer. A reader that silently splits `"b"c` into two
atoms turns a typo into an extra argument. That is the failure the grammar's rule 2 exists to prevent for
`10ms`, and it applies to a string for the same reason.

## Which descriptions it invalidates

Any description with a string followed directly by a symbol character. It is now refused, exit `10`,
where before it read as two atoms and was judged on those. A census of every tracked description outside
`vendor/`, taken with the fixed reader on `2026-09-30`, found one: `LS-007`'s evidence file
`11-adjacent-fragment-join.eadl`, written as input for LinkedSpec and never a system description. Its
frozen verdict in `crates/archogen-cli/tests/verdicts.txt` moved from `10 schema-unknown-kind` to
`10 read-missing-delimiter`. Nothing else moved.

## Which version it lands in

`eadl/1`, as a correction: the reader now enforces a rule its grammar already stated. It is not a new
language version.
