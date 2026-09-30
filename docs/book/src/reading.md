# Reading a description

> **The surface has two normative halves, and this chapter is neither of them.**
> `docs/semantics/grammar.md` says what a well-formed description *is*;
> `docs/semantics/reference.md` says what a well-formed description *is worth* — the exact value
> behind every literal, the escapes a string may carry, what canonical form guarantees, which
> comments become headers, and every diagnostic the reader can emit with the repair it owes you.
> This chapter explains what the reader *produces* and why.
>
> Both halves are executed rather than merely written: a recognizer derived from the grammar and the
> reader are checked against each other over the whole corpus — they must accept the same language
> and agree on where every token begins and ends, or the build fails — and the reference's tables are
> read back out of the document and run against the reader row by row, so a value it states and a
> value the reader produces cannot disagree quietly.
>
> The probes behind the first check reach every production of the grammar, and that is measured: the
> recognizer reports the productions of each derivation it accepts, and a production no probe reaches
> fails the build.


Before anything can be checked, resolved or generated, it has to be read — and when it cannot
be read, the message has to say where and what to do. `ROADMAP.md` §5.5 makes that part of the
user contract: every diagnostic carries source spans and a concrete repair direction.

## What the reader produces

S-expressions, with spans on everything:

```console
$ cargo run -q -p eadl-front --example diagnose -- docs/semantics/boundary/accept/counter-width-and-rate.eadl
read cleanly: 2 top-level form(s)

headers:
  case: counter-width-and-rate
  verdict: accept
  ambiguous: yes
  tests: externality=pass implementation-independence=pass non-prescription=pass
  rationale: AMBIGUOUS, resolved ACCEPT. The width and rate of an offered counter loo…
  other-side: engine: whether to extend the epoch, how often to read, and the arithmet…

canonical:
  (eadl-version eadl/1)
  (defblock timer.counter (offers (counter-width 32 bit) (counter-modulus 4294967296) (tick-rate 10 MHz)))
```

⭐ **Two forms, and only one of them is a declaration.** The first states the language version this
description is written in — every description in this repository carries one, and §8 of
`docs/semantics/reference.md` says why it is a form rather than a comment. Note what the two halves of
this output do *not* share: the `headers:` above come from the comment block, and the `canonical:` text
carries no comment at all, which is exactly why a version could not live in one.

The reader is **purely syntactic**. `(tick-rate 10 MHz)` is a symbol, an integer and a symbol;
that `MHz` is a unit is the model layer's business. That separation is what lets one reader
serve the boundary corpus, the S0 fixture and the M1 semantic corpus without any of them
leaking assumptions into the others.

## Numbers are exact — there is no float anywhere

§7.4 requires exact integer or checked rational arithmetic. A reader that produced `f64` would
lose that before any analysis ran: a `0.1 ms` in a description would silently become a value
that is not one tenth of a millisecond. So `1.5` is kept as 15 with a scale of 1, and prints
back as `1.5`.

A literal that does not fit in a 64-bit signed integer is **refused**, not wrapped. That range is a
property of the **language version** rather than of this build: `eadl/1` writes every value from
`-9223372036854775808` to `9223372036854775807`, in decimal or in hexadecimal, and both endpoints are
executed rows of `docs/semantics/reference.md` §1 rather than a claim in prose. A later version may
widen the domain, and that costs a migration note rather than a rewritten description — every literal
this version reads keeps the value it has here, so nothing changes meaning underneath you.

⚠️ **One thing the domain costs today.** An address whose most significant bit is set — a kernel-space
virtual address under any RV64 paging scheme, whose upper bits the scheme requires to be all set — has
no unsigned spelling. `0xFFFF_FFFF_C000_0000` is refused, while `-1073741824`, the same 64-bit pattern
read as signed, reads and round-trips. The address is not lost; the spelling a datasheet or a linker
script prints is. The refusal says what to write instead, and §1 rule 10 of the reference states the
limit and names the trigger that would end it.

How much of the domain the descriptions in this repository actually use is a command rather than a
figure somebody typed, so it cannot go stale:

```console
$ cargo run -q -p eadl-front --example literals -- docs/semantics examples
```

It prints every distinct integer value the population writes, the largest of them and how many of the
domain's magnitude bits it needs, and — the number a decision about the domain rests on — how many
literals the domain **refused**.

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

A label can cover more than one line — the boundary check points at a whole `(implementation …)` body,
which here runs from line 15 to line 20. The caret underlines the part on the line shown and **stops at
the end of that line**; the label then says where the span ends:

```console
$ archogen check docs/semantics/boundary/reject/register-programming-sequence.eadl
error[boundary-implementation-in-description]: `implementation` is implementation, and eADL contains no implementation
  --> docs/semantics/boundary/reject/register-programming-sequence.eadl:15:3
   |
15 |   (implementation
   |   ^^^^^^^^^^^^^^^ this is an implementation body — an algorithm, a sequence, or a data structure (continues to line 20)
  = hint: it fails the `non-prescription` test — can its interpretation be stated without prescribing an algorithm, instruction sequence, code provider, data structure, or executable model body? It belongs to engine knowledge: the provider record selected against this declaration's contract.
archogen: invalid-description: 1 diagnostic(s) in docs/semantics/boundary/reject/register-programming-sequence.eadl
```

Until leaf `M1.31` the caret was sized by the whole span, so this one drew **183** carets under a
17-character line, and ten of the repository's tracked descriptions rendered a caret that wrapped the
terminal and pointed at nothing. `crates/eadl-model/tests/rendering.rs` now renders every diagnostic
`check` produces over the whole conformance suite and fails on any marker that runs past its line.

Reading does not stop at the first error: three malformed numbers cost one edit cycle, not
three.

A list may nest at most 256 deep (leaf `M1.38`). The deepest description in the repository nests six
levels, so the limit only ever meets input that is not a description. Such input used to crash the
reader. Ten thousand nested parentheses, a 20 KB file, overflowed its stack and aborted the process with
no verdict at all. Now the first list past the limit is `read-nesting-too-deep`, the reader steps over it
without descending, and nothing after the reader meets the depth. The limit belongs to `eadl/1`, like the
64-bit value domain, so every reader refuses the same descriptions.

## Comments survive

They are semantically inert to the language and are kept anyway, with their spans, because the
boundary corpus carries its case metadata in them. Discarding them would force a second,
divergent parser to exist just to read those headers.

## Canonical form

Two descriptions differing only in whitespace print identically. That is the first link in the
chain §12 M4 needs — "repeated generation produces identical canonical plans and generated
sources" — and it is tested as a round trip on all 23 corpus files: read, print, read again,
and the structure must be unchanged.

Canonical text also never carries a **raw control character**. `\n`, `\t` and `\r` print as the
escapes you would write yourself, and every other control character — the ones with no short
name, a NUL included — prints as `\u{…}`, which the language reads back. That is worth more than
it sounds: canonical text is what gets hashed, diffed and pasted into a report, and an invisible
byte is merely confusing in the first two and destructive in the third, where a carriage return
sends a terminal back to column zero and overwrites what it already printed.

The same rule runs in the other direction. A description may not contain a raw control character
either — except a tab, which is whitespace — and the reader says so with
`error[read-control-character]`, naming the escape that writes the character you meant. Nothing
writable is lost by that: `\u{…}` reaches every character there is, so an unusual one costs six
visible characters instead of one invisible byte.

An escape the language does not know, such as `\q` or `\é`, is reported as `error[read-bad-escape]`,
naming the whole character. The string keeps both characters as written, so one typo does not
cascade into more errors. ⛔ Until leaf `M1.35`, a backslash before a character written in more than
one byte (`"a\éb"`) crashed the reader, and `archogen check` exited 101 where the contract is a
diagnostic and exit 10 (`crates/eadl-front/src/reader.rs`).

## Which language version a description is written in

A description can say so:

```eadl
(eadl-version eadl/1)

(defsystem periodics
  (name "three periodic tasks"))
```

`docs/semantics/reference.md` §8 states the rules, and its table is executed by the same legs that
cover numbers and strings — so what follows is what the reader does, not a description of it.

**Why a form and not a comment.** The obvious home for a version is the comment header at the top of
the file, and it is wrong here: canonical form carries no comment, and §12 M4 hashes canonical text. A
version written in trivia would be a version the hashed artifact does not capture — two descriptions
differing only in the language they claim would hash identically, and any tool that strips comments
would strip the lock along with them.

**Absence means `eadl/1`, by rule.** A description carrying no identifier is not missing information;
§8 rule 2 says what absence *denotes*, which is what makes the version determinable from the
description rather than from whichever toolchain happens to be reading it. That rule carries a stated
expiry: `eadl/1` is the last version for which absence is permitted, because a default that outlives
the version it defaults to is how a description silently changes meaning.

⭐ **And its population is a gate, not a sentence.** Every description this repository ships states its
version — the whole conformance suite, enumerated by the tests that walk it — so the only descriptions
relying on absence are the frozen [LinkedSpec](ledger.md#linkedspec) evidence files under `docs/feedback/`, whose bytes *are*
the reproduction of another project's defect and which nothing here reads.
`crates/eadl-front/tests/reference.rs` requires that, by
asking the frontend rather than a text search: a nested, quoted or unread identifier does not satisfy
it. A rule whose justifying population can grow silently is a rule that has already expired, and the
sentence in §8 was written while 62 live descriptions contradicted it.

**What is refused**, and why each is a different mistake rather than one:

| you wrote | what the reader says |
| --- | --- |
| `(eadl-version eadl/2)` | `error[language-version-unknown]` — this toolchain reads one version, and a description is never silently re-read as another |
| `(eadl-version "eadl/1")` | `error[language-version-not-an-identifier]` — a string is a different atom from a symbol, not an interchangeable spelling of one |
| `(eadl-version)` | `error[language-version-missing]` |
| `(eadl-version eadl/1 eadl/1)` | `error[language-version-extra-argument]` |
| the same identifier twice | `error[language-version-duplicated]`, pointing at the second and labelling the first |

⚠️ **What this does not do yet.** There is one version, so nothing behaves differently between stating
it and omitting it: the identifier is read, checked and refused, but not yet *acted on*. Saying
otherwise would be the more dangerous kind of green. What makes it more than a placeholder is that the
corpora are about to be digested into `eadl/1`'s frozen baseline, which is what turns "a later version
differs" from a promise into something a gate can check.

## A lesson from the corpus

The header format needs **two** independent discriminators — indentation *and* key shape — and
each was added only after the other alone failed on a real file in the corpus. The second
failure is the instructive one: a rationale wrapped onto a line beginning
`implementation-independence: a different timer …`, which is exactly a bare key followed by a
colon. It opened a spurious header and silently truncated the value.

The suite did not catch it, because it checked that the keys it *wanted* were present. Presence
checks cannot see an extra key. It was found by running the `diagnose` tool over a real file and
reading the output — and the test now asserts the exact key set for every case.
