# The eADL language reference

**Normative.** This file states the rules the surface grammar cannot carry: what a literal
*means*, what canonical form *guarantees*, and what the frontend refuses. `docs/semantics/grammar.md`
says what a well-formed description **is**; this file says what a well-formed description **is
worth**. The two are complementary and neither is a description of the reader — both are executed:
`crates/eadl-front/tests/reference.rs` reads the tables below **out of this document** and requires
`crates/eadl-front/src/reader.rs` and `crates/eadl-front/src/form.rs` to agree with every row.

> ⛔ **Why this file exists.** A grammar proves that two implementations accept the same language and
> cut it at the same token boundaries, and says nothing about the value behind a token. `M1.11`
> recorded that limit in its own words: *"a reader that tokenized identically and mis-nested, or read
> `1.5` as three halves, would pass."* Value exactness is the property `ROADMAP.md` §7.4 actually
> requires — "exact integer time units or checked rational arithmetic" — and a reader that produced a
> binary float would have lost it before any analysis ran. Leaf `M1.12`.

## How to read this document

Rules are written as statements about the language, not about an implementation. Each one is either
carried by a row of a table below — in which case a test executes it — or it cites the code that
enforces it. A rule that is neither is prose, and prose drifts; `M1.23` measured a figure that stayed
false on live surfaces long after the measurement behind it had moved.

⛔ **No count appears in this document.** Not of rows, escapes, codes or kinds. A count is a figure,
and a figure in prose is a figure nothing re-derives; the mechanisms that read this file enumerate the
population themselves, so a table that grows does not leave a stale number behind.

## Notation for the tables

A table introduced by a `<!-- machine-read: … -->` comment is **executed**:
`crates/eadl-front/tests/reference.rs` locates it by that comment, reads every row, and runs it against
the frontend. A table without that comment is prose and checks nothing.

| Form | Means |
| --- | --- |
| `integer N` | an exact integer whose value is `N` |
| `rational N/10^S` | an exact rational: the digits `N`, with the last `S` of them after the decimal point. Never a float |
| `symbol T` | not a number at all — the atom is the symbol whose text is `T` |
| `error C` | **not well-formed**, and refused with a diagnostic whose code is `C` |
| `refused C` | well-formed, but its value is outside the domain the language can hold; refused with code `C` |
| `—` | no canonical text, because the literal yields no value |

⭐ **`error` and `refused` are different claims, and the difference is not cosmetic.** `1.2.3` is not
a number and no conforming implementation may read it; `9223372036854775808` *is* a well-formed integer
whose value does not fit, and a grammar that could say so would have to know the width of the value
domain — which is precisely the implementation detail `docs/semantics/grammar.md` keeps out of itself.
So the recognizer derived from that grammar must reject every `error` row and accept every `refused`
one, and `crates/eadl-front/tests/conformance.rs` is what requires it.

A string's **decoded value** column is written in the language's own escape notation, so it is
unambiguous: a backslash in that column is always the start of one of the escapes this file defines.

## 1. Numbers are exact

A numeric literal denotes an exact rational number. There is no floating-point value in the language,
in the reader, or in any model above it: `crates/eadl-front/src/form.rs` has no float variant, and
§7.4 requires exact integer or checked rational arithmetic. A decimal literal is held as its digits
and a scale, so `0.1` is one tenth exactly and survives any number of round trips.

<!-- machine-read: number-values -->
| literal | value | canonical |
| --- | --- | --- |
| `0` | `integer 0` | `0` |
| `1` | `integer 1` | `1` |
| `2` | `integer 2` | `2` |
| `3` | `integer 3` | `3` |
| `4` | `integer 4` | `4` |
| `5` | `integer 5` | `5` |
| `8` | `integer 8` | `8` |
| `10` | `integer 10` | `10` |
| `15` | `integer 15` | `15` |
| `16` | `integer 16` | `16` |
| `20` | `integer 20` | `20` |
| `25` | `integer 25` | `25` |
| `30` | `integer 30` | `30` |
| `32` | `integer 32` | `32` |
| `50` | `integer 50` | `50` |
| `60` | `integer 60` | `60` |
| `64` | `integer 64` | `64` |
| `100` | `integer 100` | `100` |
| `150` | `integer 150` | `150` |
| `200` | `integer 200` | `200` |
| `850` | `integer 850` | `850` |
| `4294967296` | `integer 4294967296` | `4294967296` |
| `007` | `integer 7` | `7` |
| `1_000` | `integer 1000` | `1000` |
| `1__000` | `integer 1000` | `1000` |
| `1_` | `integer 1` | `1` |
| `+7` | `integer 7` | `7` |
| `-3` | `integer -3` | `-3` |
| `-` | `symbol -` | `-` |
| `+` | `symbol +` | `+` |
| `9223372036854775807` | `integer 9223372036854775807` | `9223372036854775807` |
| `-9223372036854775808` | `integer -9223372036854775808` | `-9223372036854775808` |
| `9223372036854775808` | `refused read-number-overflow` | — |
| `-9223372036854775809` | `refused read-number-overflow` | — |
| `99999999999999999999` | `refused read-number-overflow` | — |
| `0x0` | `integer 0` | `0` |
| `0x10` | `integer 16` | `16` |
| `0X10` | `integer 16` | `16` |
| `0x1000_0000` | `integer 268435456` | `268435456` |
| `0xdead_BEEF` | `integer 3735928559` | `3735928559` |
| `0x7fff_ffff_ffff_ffff` | `integer 9223372036854775807` | `9223372036854775807` |
| `0x8000_0000_0000_0000` | `refused read-number-overflow` | — |
| `0x` | `error read-malformed-number` | — |
| `0xg` | `error read-malformed-number` | — |
| `0x_10` | `error read-malformed-number` | — |
| `1.5` | `rational 15/10^1` | `1.5` |
| `0.1` | `rational 1/10^1` | `0.1` |
| `10.50` | `rational 1050/10^2` | `10.50` |
| `1.05` | `rational 105/10^2` | `1.05` |
| `1.50` | `rational 150/10^2` | `1.50` |
| `-0.25` | `rational -25/10^2` | `-0.25` |
| `0.000` | `rational 0/10^3` | `0.000` |
| `1_0.5_0` | `rational 1050/10^2` | `10.50` |
| `1.` | `error read-malformed-number` | — |
| `1.2.3` | `error read-malformed-number` | — |
| `1e9` | `error read-malformed-number` | — |
| `3ms` | `error read-malformed-number` | — |
| `.5` | `symbol .5` | `.5` |
| `_1000` | `symbol _1000` | `_1000` |

### The rules those rows state

1. **An integer is exact and 64-bit signed.** Every value from `-9223372036854775808` to
   `9223372036854775807` is writable, and the endpoints are rows above rather than a claim in prose.
   A literal outside that range is refused as `read-number-overflow`; it is never wrapped, truncated
   or silently reduced, because a wrapped magnitude is a wrong system rather than an error. Such a
   literal is still **well-formed** — the grammar accepts it and the value domain refuses it — which
   is why those rows read `refused` and not `error`.
2. **A decimal is a rational, not a quotient.** `rational 15/10^1` is fifteen tenths exactly. The
   scale — how many digits sit after the point — is part of the value's identity, so `1.5` and `1.50`
   are **different literals with different canonical text**, even though the model layer above the
   frontend makes them the same magnitude again — `crates/eadl-model/src/rational.rs` holds an exact
   rational, so `15/10` and `150/100` are equal there. That is deliberate: an author who writes `1.50`
   has stated a precision, and erasing it in the reader would erase it everywhere.
3. **`_` separates digits and is ignored.** It may repeat and it may trail. It may **not** lead: a
   hexadecimal literal must begin with a hexadecimal digit, exactly as a decimal one must begin with a
   decimal digit, so `0x_10` is `read-malformed-number`. The scale of a decimal counts the digits
   after the point *with separators removed*, which is why `1_0.5_0` is `rational 1050/10^2`.
4. **A sign belongs to a number only when a digit follows it.** `-3` is a negative integer; a lone `-`
   or `+` is a symbol, because an identifier may legitimately be spelled that way. A leading `+` is
   part of the spelling and not of the value, so canonical text does not carry it.
5. **Leading zeros are decimal.** `007` is seven. There is no octal literal in the language, and no
   exponent form either: `1e9` is refused rather than read as a float, as two atoms, or as a symbol.
6. **A digit-led atom that is not a number is refused, never re-read as a symbol.** `3ms` is a typo
   for `3 ms`; accepting it as a symbol would lose the magnitude and resurface much later as a missing
   field. `read-malformed-number` names the offending text and the repair.
7. **A number must begin with a digit.** `.5` is therefore the symbol `.5`, not a rational — and a
   later stage that wanted a quantity there reports a type error naming what it found, which is a
   truer diagnostic than a silently accepted half.
8. **Hexadecimal digits and the `0x` prefix are case-insensitive.** `0X10` and `0xdead_BEEF` are
   values; canonical text prints them in decimal, because canonical form is a function of value.
9. **⚠️ Nothing at or above 2^63 is writable.** An integer is signed, so a base address of
   `0x8000_0000_0000_0000` is refused. This is a limit of the value domain rather than a rule about
   literals, it is stated here so that nobody reads row `0x8000_0000_0000_0000` as an accident, and
   widening it is a language change owned by the freeze (`M1.13`, finding F-F).

## 2. Strings and escapes

A string is a double-quoted run of characters. It is a **value**, not a name: `docs/semantics/grammar.md`
gives it its own token kind, and the erasure of that distinction is what the LinkedSpec evaluation's
`LS-003` was about.

<!-- machine-read: string-values -->
| source | decoded value |
| --- | --- |
| `"a"` | `a` |
| `"a\nb"` | `a\nb` |
| `"a\tb"` | `a\tb` |
| `"a\rb"` | `a\rb` |
| `"say \"hi\""` | `say \"hi\"` |
| `"back\\slash"` | `back\\slash` |
| `"§ multi-byte"` | `§ multi-byte` |
| `"nul\0here"` | `error read-bad-escape` |
| `"bad\qescape"` | `error read-bad-escape` |
| `"unterminated` | `error read-unterminated-string` |

### The rules those rows state

1. **A string with no backslash in it denotes exactly its characters.** Any character may appear
   literally except `"` — which ends the string — and `\`, which starts an escape. Row
   `"§ multi-byte"` is that rule with a character outside ASCII in it, and it is why the coverage leg
   of `crates/eadl-front/tests/reference.rs` can require a table row only of a string that carries a
   backslash: every other string in a description is already covered by this sentence.
2. **The escapes are exactly these:** `\n` line feed, `\t` tab, `\r` carriage return, `\"` a quote,
   and `\\` a backslash. They are enumerated in `crates/eadl-front/src/reader.rs` and restated here
   because a grammar can say that `\0` is *shaped* like an escape without knowing whether it means
   anything. Anything else after a backslash is `read-bad-escape`, and the diagnostic names the
   supported set — §5.5 requires a repair direction, not just a refusal.
3. **A string ends at the end of its line.** A raw line break inside a string is
   `read-unterminated-string`, as is running out of input; both label the opening quote as a secondary
   span, so the author sees the two ends of the mistake. There is no multi-line string literal, and
   therefore no rule about which line ending a multi-line string would keep.
4. **Characters outside ASCII survive intact.** A description carries prose — `§`, em dashes, non-Latin
   text — and a byte-oriented reader would cut one in half. Spans and rendered columns count
   **characters**, not bytes, which is why a caret still lands under the right text on a line
   containing `§`.
5. **The escape set is closed on purpose.** A character that has no escape cannot be written into a
   string, and cannot be written *out* of one either: §3 escapes what canonical form cannot carry.
   Adding an escape is a language change (`M1.13`, finding F-G), not a reader fix.

## 3. Canonical form

Canonical text is what the toolchain compares, hashes and reports. §12 M1 requires a description to
round-trip semantically, and §12 M4 requires repeated generation to produce identical canonical plans
— both of which start here, in `crates/eadl-front/src/form.rs`.

Canonical form **guarantees**:

1. **It is a function of value, not of spelling.** `+7`, `007` and `0x7` all print `7`; `0x1000_0000`
   prints `268435456`. Two descriptions that differ only in how a value was written, in whitespace, or
   in comments print identically.
2. **It is one form per line, with no comment and no line break inside a form.** Siblings are separated
   by exactly one space.
3. **It carries no control character.** Every character that would break rule 2 — line feed, tab,
   carriage return — is printed as its escape instead, so canonical text is safe to diff, to hash and
   to put in a report. `crates/archogen-s0/src/provenance.rs` escapes the same set in its own quoting:
   one rule, and every surface that prints a string is held to it.
4. **A decimal keeps its scale.** `1.5` and `1.50` print differently, per §1 rule 2.
5. **Reading canonical text back yields a structurally equal document.** That is what "round-trips
   semantically" means: same forms, same values, spans necessarily different.

Canonical form does **not** guarantee that two descriptions with the same meaning print identically —
`(a 1.5)` and `(a 1.50)` are the same magnitude and different canonical text — nor that canonical text
is stable across a change to these rules, which is what freezing the language at `M1.13` is for.

⚠️ **Known limit (finding F-G).** Rule 3 holds for every control character the language has an escape
for. A raw control byte copied into a string from its source has no escape to use and prints as
itself; closing that needs either a general escape or a rule that a description may not contain one,
and both are language changes owned by `M1.13`.

## What this reference does not yet carry

Stated rather than left implicit, because a reference that quietly omits a rule reads as though the
rule does not exist:

| Not here yet | Where it will be | Leaf |
| --- | --- | --- |
| the census of diagnostic codes the frontend can emit, and the repair each carries | a chapter of this file, checked in both directions against the code that emits them | `M1.12.3` |
| comment retention, and the `; key: value` header convention with its two discriminators | a chapter of this file, with a table executed against `Document::comment_headers` | `M1.12.4` |
| module and import semantics — instances rather than modules, version satisfaction, alias defaulting, the one failure that stops | a chapter of this file | `M1.12.4` |
| the `defkind` facility's meaning and its limits (§5.6: it must not become a host-code evaluator) | a chapter of this file | `M1.12.4` |
| a version identifier on the surface, and the frozen compatibility baseline | `eadl/1` | `M1.13` |
