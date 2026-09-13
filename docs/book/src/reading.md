# Reading a description

Before anything can be checked, resolved or generated, it has to be read — and when it cannot
be read, the message has to say where and what to do. `ROADMAP.md` §5.5 makes that part of the
user contract: every diagnostic carries source spans and a concrete repair direction.

## What the reader produces

S-expressions, with spans on everything:

```console
$ cargo run -q -p eadl-front --example diagnose -- docs/semantics/boundary/accept/counter-width-and-rate.eadl
read cleanly: 1 top-level form(s)

headers:
  case: counter-width-and-rate
  verdict: accept
  ambiguous: yes
  tests: externality=pass implementation-independence=pass non-prescription=pass
  rationale: AMBIGUOUS, resolved ACCEPT. The width and rate of an offered counter loo…
  other-side: engine: whether to extend the epoch, how often to read, and the arithmet…

canonical:
  (defblock timer.counter (offers (counter-width 32 bit) (counter-modulus 4294967296) (tick-rate 10 MHz)))
```

The reader is **purely syntactic**. `(tick-rate 10 MHz)` is a symbol, an integer and a symbol;
that `MHz` is a unit is the model layer's business. That separation is what lets one reader
serve the boundary corpus, the S0 fixture and the M1 semantic corpus without any of them
leaking assumptions into the others.

## Numbers are exact — there is no float anywhere

§7.4 requires exact integer or checked rational arithmetic. A reader that produced `f64` would
lose that before any analysis ran: a `0.1 ms` in a description would silently become a value
that is not one tenth of a millisecond. So `1.5` is kept as 15 with a scale of 1, and prints
back as `1.5`.

A literal that does not fit in a 64-bit signed integer is **refused**, not wrapped.

## Diagnostics point at the problem

```text
error[read-malformed-number]: `3ms` is not a number
  --> examples/sensor.eadl:4:13
  |
4 |     (period 3ms)
  |             ^^^ a number cannot continue like this
  = hint: write an integer or a decimal such as `10` or `1.5`; units go in a following atom, e.g. `10 ms`
```

`3ms` could have been read as a symbol — most S-expression readers would. It is refused instead,
because it is a typo for `3 ms`, and reading it as a symbol would lose the magnitude and surface
much later as a mysteriously missing field.

When a list is left open, both ends are shown:

```text
error[read-unclosed-list]: this list is never closed
  --> examples/time.eadl:3:1
  |
3 |
  | ^ input ends here, still inside a list
  --> examples/time.eadl:1:1
  |
1 | (defservice time.monotonic
  | - opened here
  = hint: add the matching `)`
```

Columns count **characters**, not bytes, so the caret still lands under the right text on a
line containing `§` or an em dash — and eADL descriptions carry prose in their comments.

Reading does not stop at the first error: three malformed numbers cost one edit cycle, not
three.

## Comments survive

They are semantically inert to the language and are kept anyway, with their spans, because the
boundary corpus carries its case metadata in them. Discarding them would force a second,
divergent parser to exist just to read those headers.

## Canonical form

Two descriptions differing only in whitespace print identically. That is the first link in the
chain §12 M4 needs — "repeated generation produces identical canonical plans and generated
sources" — and it is tested as a round trip on all 21 corpus files: read, print, read again,
and the structure must be unchanged.

## A lesson from the corpus

The header format needs **two** independent discriminators — indentation *and* key shape — and
each was added only after the other alone failed on a real file in the corpus. The second
failure is the instructive one: a rationale wrapped onto a line beginning
`implementation-independence: a different timer …`, which is exactly a bare key followed by a
colon. It opened a spurious header and silently truncated the value.

The suite did not catch it, because it checked that the keys it *wanted* were present. Presence
checks cannot see an extra key. It was found by running the `diagnose` tool over a real file and
reading the output — and the test now asserts the exact key set for every case.
