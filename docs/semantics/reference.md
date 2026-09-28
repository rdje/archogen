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

## What this reference is normative over

A normative document has to say what it governs, or "every rule the frontend enforces is stated here"
is a claim nobody can check. These are the sources. The diagnostic census in §4 is taken over exactly
this list, in both directions, by `crates/eadl-front/tests/reference.rs`.

<!-- machine-read: normative-sources -->
| source | what this reference states about it |
| --- | --- |
| `crates/eadl-front/src/reader.rs` | §1 numbers, §2 strings and escapes, and every diagnostic in §4 whose code begins `read-` |
| `crates/eadl-front/src/form.rs` | §3 canonical form and §5 comment retention: what is printed, what is escaped, what structural equality compares, and which comments become headers |
| `crates/eadl-front/src/module.rs` | §6 modules and imports, and every diagnostic in §4 whose code begins `module-` |
| `crates/eadl-model/src/kind.rs` | §7 kinds and `defkind`, and every diagnostic in §4 whose code begins `schema-` |

⚠️ **A source belongs here when a chapter of this file states rules about it**, not when it merely
happens to be nearby. `crates/eadl-model/src/kind.rs` is in a different crate from the frontend and is
declared anyway, because §7 states rules it enforces and §4 states its codes — the declaration is about
*what this document governs*, not about where the code lives. `crates/eadl-model/src/boundary.rs` is
deliberately **not** here: it classifies implementation syntax, and the boundary is a profile concern
documented in `docs/book/src/boundary.md`. Adding it to this table before a chapter of this file states
its rules would make the census demand rows for codes nothing here explains.

⚠️ **So this file governs the surface, and the surface is not the whole toolchain.** The model layer
above it — quantities and units, presence and absence, refinement, the profile check — emits
diagnostics of its own, and `docs/book/src/` renders some of them to the reader. No normative document
states those codes yet, which means `crates/eadl-front/tests/reference.rs` can check that one the book
shows is a code the engine really emits, but not that its rule is written down anywhere. That is a gap
with an owner rather than a limit of this file's subject matter: leaf `M1.26` in `docs/tasks/M1.md`,
which carries the census. This file's scope does not widen to cover them, because a quantity's unit or
a refinement's direction is a rule about *meaning*, and §7 rule 5 is explicit that this reference does
not interpret meaning.

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
| `<0xNN>` | in a **source** cell only: the single character whose code point is `NN` in hexadecimal |
| `—` | no canonical text, because the literal yields no value |

⭐ **Why a source cell needs `<0xNN>`.** A row about a raw control byte cannot otherwise be written:
the byte is invisible in the cell, and a raw line feed would end the row and take the table with it. So
the input `read-control-character` and `read-unexpected-character` fire on was unwritable here, which
is why `M1.12.3` deferred an executable input column on §4's table and `M1.26`'s gap (b) inherited the
deferral. A marker that names no character — a surrogate, or a code point past `10ffff` — is a
**violation** and not a row to skip, the same contract `error` and `refused` hold.

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
| `-0x8000_0000_0000_0000` | `integer -9223372036854775808` | `-9223372036854775808` |
| `-0x8000_0000_0000_0001` | `refused read-number-overflow` | — |
| `0xFFFF_FFFF_C000_0000` | `refused read-number-overflow` | — |
| `-1073741824` | `integer -1073741824` | `-1073741824` |
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
9. ⭐ **The value domain is a property of this version of the language, not of an implementation.**
   `eadl/1` holds an integer in an exact signed 64-bit value, so what is writable runs from
   `-9223372036854775808` to `9223372036854775807`, and a well-formed literal outside that range is
   `refused read-number-overflow`. Both endpoints are rows above **in both spellings and in both
   directions** — the value that reads and the next one out, decimal and hexadecimal — so the boundary
   is executed rather than described. The sign is applied to a wider magnitude before the value is
   narrowed, in `crates/eadl-front/src/reader.rs`, and that order is load-bearing rather than an
   implementation detail: it is the only reason the two rows for the minimum are writable at all,
   since a magnitude one past the maximum *is* the minimum once signed.
   Widening the domain is a **compatible** language change — every literal this version reads keeps the
   value it has here, so no description changes meaning and none needs migrating. It therefore lands in
   a later version behind a migration note, and costs one added compatibility-corpus row: a literal
   refused under `eadl/1` and readable under the version that widens. It is not a change this version
   has to make pre-emptively, and the decision with its measurements is
   `docs/decisions/decision_eadl1-value-domain.md`.
10. ⚠️ **Honest limit — an address whose most significant bit is set has no unsigned spelling here.**
    A canonical high-half virtual address, which is the shape every RV64 paging scheme gives kernel
    space because such an address has its upper bits all set, is a magnitude above this domain: row
    `0xFFFF_FFFF_C000_0000` is refused. ⛔ The address is **not** lost — the row below it,
    `-1073741824`, is the same 64-bit pattern read as signed, and it reads and round-trips. What is
    refused is the *spelling* a datasheet, a linker script or a device tree prints, and that is a real
    cost rather than a curiosity: it is the named trigger for rule 9's widening, because the first
    target description that has to state a high-half address is the description that needs the unsigned
    spelling. Physical addresses are unaffected — the widest one a standardized RV64 target can have
    sits far inside the domain — and both measurements live in the decision record rather than being
    restated here, because a figure in this document is a figure nothing re-derives.

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
| `"a\u{1b}b"` | `a\u{1b}b` |
| `"\u{0}"` | `\u{0}` |
| `"\u{7f}"` | `\u{7f}` |
| `"\u{10ffff}"` | `\u{10ffff}` |
| `"\u{d800}"` | `refused read-escape-out-of-range` |
| `"\u{110000}"` | `refused read-escape-out-of-range` |
| `"\u{}"` | `error read-bad-escape` |
| `"\u{1bx}"` | `error read-bad-escape` |
| `"a<0x1b>b"` | `error read-control-character` |
| `"a<0x0>b"` | `error read-control-character` |
| `"a<0x9f>b"` | `error read-control-character` |
| `"a<0xa>b"` | `error read-unterminated-string` |
| `"a<0x9>b"` | `a\tb` |
| `"unterminated` | `error read-unterminated-string` |

### The rules those rows state

1. **A string with no backslash in it denotes exactly its characters.** Any character may appear
   literally except `"` — which ends the string — `\`, which starts an escape, and **a control
   character other than tab**, which is refused as `read-control-character`. Row `"§ multi-byte"` is
   that rule with a character outside ASCII in it, and it is why the coverage leg of
   `crates/eadl-front/tests/reference.rs` can require a table row only of a string that carries a
   backslash: every other string in a description is already covered by this sentence. Tab is the one
   control character that may appear raw, because it is whitespace the grammar already names and §3
   prints it as `\t`; a raw line feed keeps its own more specific diagnostic, `read-unterminated-string`
   (rule 3), which is what row `"a<0xa>b"` pins.
2. **The escapes are exactly these:** `\n` line feed, `\t` tab, `\r` carriage return, `\"` a quote,
   `\\` a backslash, and `\u{…}` the character whose code point is written in hexadecimal — one to six
   digits, so `\u{1b}` is ESCAPE and `\u{10ffff}` the last character there is. They are enumerated in
   `crates/eadl-front/src/reader.rs` and restated here because a grammar can say that `\0` is *shaped*
   like an escape without knowing whether it means anything. Anything else after a backslash is
   `read-bad-escape`, and the diagnostic names the supported set — §5.5 requires a repair direction,
   not just a refusal. ⭐ `\u{…}` has **two** refusals and they are not interchangeable: an escape that
   is not shaped like one (`\u{}`, `\u{1bx`) is not well-formed at all, while one that is shaped
   correctly but names no character — a surrogate, or a code point past `10ffff` — is well-formed and
   outside the domain, so it is `refused` rather than `error`. That is the same distinction §1 draws
   between `read-malformed-number` and `read-number-overflow`, and it is load-bearing for the same
   reason: `crates/eadl-front/tests/conformance.rs` requires the recognizer derived from the grammar to
   reject every `error` row and accept every `refused` one.
3. **A string ends at the end of its line.** A raw line break inside a string is
   `read-unterminated-string`, as is running out of input; both label the opening quote as a secondary
   span, so the author sees the two ends of the mistake. There is no multi-line string literal, and
   therefore no rule about which line ending a multi-line string would keep.
4. **Characters outside ASCII survive intact.** A description carries prose — `§`, em dashes, non-Latin
   text — and a byte-oriented reader would cut one in half. Spans and rendered columns count
   **characters**, not bytes, which is why a caret still lands under the right text on a line
   containing `§`.
5. ⭐ **The escape set is closed, and it is sufficient — the two halves are what make rule 1 safe.**
   Closed: a backslash followed by anything the set does not define is `read-bad-escape`, so the set
   cannot grow by accident. Sufficient: every character is writable, because `\u{…}` reaches any
   Unicode scalar value, so §3 can escape anything canonical form cannot carry and still produce text
   this language reads back (rule 5 of §3). Neither half holds without the other. A set that were only
   closed would leave characters that cannot be written *or* printed, which is what finding F-G was:
   `M1.12` measured a raw control byte reaching canonical text, and until `M1.13.1` this rule claimed
   it could not. ⛔ A raw control character is refused rather than normalized, so an invisible byte
   cannot enter a description at all — and a character that must appear is written by name, where a
   reviewer can see it. Adding an escape is still a language change; after the freeze it is also a
   migration note (`M1.13`).

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
3. **It carries no control character — all of them, not the three that have names.** Line feed, tab
   and carriage return print as `\n`, `\t` and `\r`; every other character in Unicode `Cc`, which is
   what "control character" means in this rule, prints as `\u{…}`. So canonical text is safe to diff,
   to hash and to put in a report whatever a string value holds. ⭐ The guarantee is enforced **in the
   printer** and not at the door, because `Form::Str` is constructible outside the frontend — `lib.rs`
   re-exports it and `Span::new` is public — so a value that never passed a reader still has to print
   safely. `crates/archogen-s0/src/provenance.rs` shares the *property*, not the escape set: it writes
   **JSON**, where the short forms are `\b` and `\f` and a code point is `\u00XX`, so its output is
   deliberately not eADL text and is not held to this rule.
4. **A decimal keeps its scale.** `1.5` and `1.50` print differently, per §1 rule 2.
5. **Reading canonical text back yields a structurally equal document.** That is what "round-trips
   semantically" means: same forms, same values, spans necessarily different.

Canonical form does **not** guarantee that two descriptions with the same meaning print identically —
`(a 1.5)` and `(a 1.50)` are the same magnitude and different canonical text — nor that canonical text
is stable across a change to these rules, which is what freezing the language at `M1.13` is for.

⚠️ **Honest limit.** Rule 3 is about control characters, and "invisible" is a wider set than
"control". A zero-width space, a byte-order mark or a bidirectional override is not in Unicode `Cc`, so
a string holding one prints it raw: canonical text stays one form per line and stays readable by this
language, but a report that renders it can still mislead whoever reads it. Nothing refuses one, and
this section does not claim otherwise. Measured rather than assumed, so the limit is sized: **0 of 75**
tracked descriptions hold such a character, and every non-ASCII character the corpus does hold is a
visible one (`§`, `—`, `…`, `≤`, `⛔`, `⭐`). Whether the language should refuse them anyway is a scope
question for the freeze, owned by `M1.13.3`, not a rule left quietly unstated here.

⭐ The limit this section used to carry is closed. Finding F-G — a raw control byte reaching canonical
text with nothing to escape it into — was measured by `M1.12` and closed by `M1.13.1`: the escape set
gained `\u{…}`, the printer became total, and the reader now refuses the raw spelling (§2 rules 1, 2
and 5). What closed it is a row, not a sentence: `"a<0x0>b"` in §2's table is a **NUL**, and it is
executed.

## 4. Diagnostics

A description that does not read is refused with diagnostics, and §5.5 requires each one to carry a
repair direction: a refusal that does not say what to do costs an author an edit cycle, and the
cheapest diagnostic to write is the most expensive to receive.

<!-- machine-read: diagnostics -->
| code | when it fires | what to do |
| --- | --- | --- |
| `read-unclosed-list` | a `(` is never matched and the input ends inside it | add the matching `)`; the diagnostic also labels where the list opened |
| `read-unexpected-close` | a `)` appears with no list open | remove it, or add the `(` that was meant to open a list |
| `read-unexpected-character` | a byte that can start nothing **between forms**, which in practice means a stray control character; inside a string the same byte is `read-control-character` | delete it — a form is a list `(…)`, a symbol, a number or a string |
| `read-unterminated-string` | a `"` is not closed before the end of its line, or before the end of the input | close the string on its own line, or escape the newline as `\n`; there is no multi-line string (§2 rule 3) |
| `read-bad-escape` | a backslash inside a string is followed by anything other than the escapes §2 defines, including a `\u` escape that is not shaped like one | use one of those; the diagnostic names the set |
| `read-control-character` | a **raw** control character inside a string: anything in Unicode `Cc` except tab, which is whitespace the grammar names (§2 rule 2) | write the escape instead — `\n`, `\t`, `\r`, or `\u{…}` for any other character |
| `read-escape-out-of-range` | a **well-formed** `\u{…}` escape that names no Unicode scalar value — a surrogate, or a code point past `10ffff` (§2 rule 2) | write a code point that denotes a character; a surrogate denotes one only inside a UTF-16 encoding |
| `read-malformed-number` | an atom that begins with a digit, or with a sign and a digit, and is not a number — a second decimal point, an exponent, a unit glued to the magnitude, `0x` with no digits after it, or a separator leading them | write an integer or a decimal; a unit goes in a following atom, `10 ms` |
| `read-number-overflow` | a **well-formed** literal whose value lies outside the 64-bit signed range, which §1 rule 9 makes a property of this version of the language rather than of an implementation | reduce the magnitude or change its units; an address with its most significant bit set is writable as the negative value it is two's-complement equal to (§1 rule 10) |
| `module-not-a-module` | a file resolved as a module does not begin with a `defmodule` declaration | write `(defmodule <name> (version <major> <minor>) …)` |
| `module-missing-name` | a `defmodule` carries no name | write `(defmodule platform.timer (version 1 0) …)` |
| `module-bad-version` | `version` is not a major and a minor integer | write `(version 1 0)` |
| `module-missing-version` | a module declares no version | write `(version 1 0)` — an unversioned module cannot be required by an importer, and §15 needs a version to lock |
| `module-bad-param` | a `param` clause does not name its parameter | write `(param tick-rate (default 10 MHz))` |
| `module-bad-export` | an `export` entry is not a name | write `(export timer.counter timer.compare)` |
| `module-conflicting-export` | one name is exported twice by one module | export each name once — two exports give an importer two answers and no rule for choosing |
| `module-dangling-export` | a module exports a name it does not declare | export only what the module declares, or declare it |
| `module-bare-form` | a module holds a form that is neither a declaration nor a module clause | every item in a module is a declaration or a clause; move anything else out |
| `module-empty` | a module file holds no declaration | a module file holds exactly one `(defmodule …)` form |
| `module-multiple-forms` | a module file holds more than one top-level form | keep one `(defmodule …)` per file and move the rest into their own modules |
| `module-not-found` | an import names a module the module path cannot resolve | check the name, or add the directory holding it to the module path |
| `module-name-mismatch` | the name a module declares is not the name it was imported by | make them match — a locked build cannot otherwise tell which module it locked |
| `module-too-large` | a module has more addressable parts than an instance identifier can hold | split the module |
| `module-circular-import` | a module imports something that imports it back | break the cycle: elaboration is children-before-parents, so a cycle has no first instance |
| `module-bad-import` | an `import` does not name a module | write `(import platform.timer (as timer))` |
| `module-unknown-import-clause` | an import holds a clause that is not `as`, `version` or `with` | an import holds `as`, `version` and `with` |
| `module-bad-alias` | `as` is not given a namespace name | write `(as timer)` |
| `module-conflicting-alias` | two imports in one module bind the same alias | give one a different namespace, e.g. `(as timer_2)` — one alias for two imports makes every qualified name ambiguous |
| `module-bad-version-requirement` | an import's `version` is not `(version (at-least <major> <minor>))` | write `(version (at-least 1 0))` |
| `module-incompatible-version` | the module found does not satisfy the requirement | the majors must be equal and the minor at least the required one; a major bump is never silently accepted |
| `module-unknown-parameter` | an import binds a parameter the module does not declare | the diagnostic lists the parameters it does declare |
| `module-missing-argument` | an import leaves a parameter that has no default unbound | add `(with (<param> <value>))` to the import |
| `module-bad-argument` | a `with` binding is not written `(<param> <value>)` | write `(with (tick-rate 20 MHz))` |
| `schema-not-a-kind` | a form read as a kind definition is not a `defkind` | write `(defkind <head> (doc "…") (name …) (clause …) …)` |
| `schema-missing-kind-head` | a `defkind` does not name the declaration head it defines | write `(defkind defservice …)` |
| `schema-duplicate-kind` | one declaration head is defined twice | remove one — redefining a kind would silently change what already-written descriptions mean |
| `schema-missing-doc` | a kind definition does not say what the kind is for | write `(doc "one line saying what this kind describes")` |
| `schema-bad-doc` | `doc` is not one quoted string | write `(doc "what this kind is for")` |
| `schema-unknown-kind-field` | a kind definition holds a field that is not `doc`, `name` or `clause` | it holds those and nothing else, because it defines well-formedness rather than behavior |
| `schema-bad-name-rule` | `name` is neither `required` nor `forbidden` | write `(name required)` when the declaration is written `(<head> <name> …)` |
| `schema-missing-clause-head` | a `clause` does not name the clause it declares | write `(clause <name> (cardinality …) (holds …))` |
| `schema-duplicate-clause` | one clause is declared twice in a kind | declare each once, and use `(cardinality any)` to allow repetition in a description |
| `schema-unknown-clause-field` | a clause declaration holds a field that is not `cardinality` or `holds` | a clause declaration holds those two |
| `schema-bad-cardinality` | `cardinality` is not one of `one`, `at-most-one`, `one-or-more`, `any` | use one of those spellings; the diagnostic lists them |
| `schema-bad-holds` | `holds` is not `forms`, `values <type>…` or `kind <name>` | write `(holds kind task)` to have each occurrence validated as a declaration of that kind |
| `schema-bad-value-type` | a `holds values` type is not one the schema knows | the value types are `symbol`, `integer`, `decimal`, `number`, `string`, `any` |
| `schema-unknown-referenced-kind` | a clause holds a kind that is not registered | register the module that defines it — the workload kinds live in `docs/semantics/kinds/os-rt.eadl` |
| `schema-not-a-declaration` | a top-level form is not a declaration | declarations are written `(defservice time.monotonic …)` |
| `schema-unknown-kind` | a declaration's head is not a registered kind | the diagnostic names the kinds that are |
| `schema-missing-name` | a declaration of a kind that requires a name carries none | write `(<head> <name> …)` |
| `schema-not-a-clause` | a declaration holds a form that is not one of its kind's clauses | the diagnostic lists the clauses that are |
| `schema-unknown-clause` | a declaration holds a clause its kind does not define | the diagnostic lists the clauses it does define |
| `schema-cardinality` | a clause appears a number of times its kind's cardinality forbids | keep the number of `(clause …)` occurrences the cardinality allows |
| `schema-arity` | a clause does not hold the number of values its kind requires | the diagnostic names the clause, the number required and the number found |
| `schema-type` | a value in a clause is not of the type the kind declares for it | write a value of the declared type |

### The rules those rows state

1. **Every diagnostic these sources emit is an error.** There is no warning and no note anywhere in the
   toolchain's diagnostics: a description that does not read cannot be used, and a severity that said
   "probably fine" would be a lie. `crates/eadl-front/tests/reference.rs` takes the census over
   severities as well as codes, so adding a warning is a change to this sentence and not only to the
   code.
2. **Reading does not stop at the first error.** Every malformed literal in a description is reported
   in one pass, so a file with three typos costs one edit cycle rather than three.
3. **A code names a rule, not a call site.** `read-malformed-number` fires wherever a digit-led atom
   turns out not to be a number, and the message names the offending text and which shape of mistake
   it was — `1.2.3`, `0x` and `0x_10` are refused for different reasons and say so. A new call site
   that enforces the same rule adds no row here; a new *rule* adds a code and a row.
4. **The set above is complete for the sources this file is normative over, in both directions.** A
   code the frontend can emit and this file does not state is a rule nobody has written down; a code
   this file states and no declared source emits is a rotted row that reads like a live rule. Both are
   build failures, and both are checked against the declaration above rather than against a list
   inside the test.

## 5. Comments are retained, and one shape of comment is data

A comment runs from `;` to the end of its line. Comments are **kept**: `crates/eadl-front/src/form.rs`
holds every one in source order with its span, because the boundary corpus carries its verdict in them
and F27 reads exactly that. Canonical form drops them (§3 rule 2): a comment is not part of a
description's value, so two descriptions differing only in commentary print identically.

One shape of comment *is* data. A `; key: value` line at the top of a description is a **header**, and
headers are how a description states facts about itself that are not declarations — which case it is,
what verdict it carries, why.

<!-- machine-read: comment-headers -->
| source | key | value |
| --- | --- | --- |
| `; case: counter-width-and-rate` | `case` | `counter-width-and-rate` |
| `; other-side: engine: whether to extend the epoch` | `other-side` | `engine: whether to extend the epoch` |
| `; rationale: AMBIGUOUS\n;   implementation-independence: a different timer is\n;   interchangeable here` | `rationale` | `AMBIGUOUS implementation-independence: a different timer is interchangeable here` |
| `; rationale: settled\n; and the reason is this` | `rationale` | `settled and the reason is this` |
| `; a: 1\n;\n; b: 2` | `a` | `1` |
| `; a: 1\n;\n; b: 2` | `b` | `2` |
| `; Expected: unsupported-profile` | — | — |
| `;   indented-with-no-open-block: yes` | — | — |
| `; spaced key : value` | — | — |

A `—` in both the key and the value means the block yields **no header at all**. Rows sharing a source
are one case, and their order is the order the headers come out in. The source column is written in the
escape notation §2 defines, so `\n` is a line break between two comment lines.

### The rules those rows state

1. **A comment is a header when two conditions hold together.** Its text after `;` is indented by *at
   most one* whitespace character, **and** the part before its first `:` is a non-empty key of
   lowercase ASCII letters, digits and hyphens only. The key and the value are both trimmed.
2. **Only the first colon splits.** `; other-side: engine: whether to extend the epoch` is one header
   whose value contains a colon, because a rationale that could not mention a time, a ratio or a
   quotation would force authors to paraphrase their own reasoning.
3. **Anything else continues the open header.** A comment that is not a header appends its trimmed text
   to the most recent header's value, separated by one space — whether it was rejected for being
   indented or for having no colon at all. That is what lets a long rationale wrap across comment lines
   and still read as one value.
4. **A bare `;` closes the block.** After it, a following comment cannot glue itself onto the last
   value; it either opens a header of its own or is dropped. Without this, a blank comment line used as
   a paragraph break would silently merge two paragraphs into one field.
5. ⛔ **Neither condition alone works, and both failures are measured on descriptions the repository
   ships.** `docs/semantics/boundary/accept/counter-width-and-rate.eadl` wraps a rationale through a
   line reading `;   implementation-independence: a different timer with the same width and rate is` —
   text that is *exactly* a well-shaped key followed by a colon, and only its indentation stops it
   opening a spurious header and truncating the rationale there. `examples/bounded-queue/system.eadl`
   carries a line beginning `; Expected:` at one space of indentation, where only the capital in
   `Expected` stops prose becoming a field. `crates/eadl-front/tests/corpus.rs` holds a test for each,
   and the table above holds a row for each, so dropping either condition fails in two places rather
   than in a description somebody reads later.

## 6. Modules and imports

A module is a file holding exactly one `(defmodule …)` form. Its clauses are `version`, `param`,
`export`, `import` and the declarations themselves; anything else is `module-bare-form`.

1. **An import makes an instance, not a link to a module.** Importing binds a name in the importer's
   namespace, and what it brings in is addressed by the dotted path of aliases from the root — `soc.timer`.
   Importing the same module twice under two aliases therefore yields **two instances with their own
   parameter bindings**, not one shared module: `crates/eadl-front/tests/f01_f02_modules.rs` has a test
   whose whole job is that a binding given to one instance does not leak into the other.
2. **An alias defaults to the last dotted segment of the module name.** `(import platform.timer …)` is
   reachable as `timer` without saying so, which is what an author means by it; `(as …)` overrides that,
   and two imports binding one alias are refused rather than resolved by order.
3. **A version requirement is satisfied by the same major and at least the stated minor.**
   `(version (at-least 1 2))` accepts `1.2` and `1.7` and refuses `1.1` — and refuses `2.0` too,
   however much newer it is, because §15's promise is that a source description *retains its meaning*
   under its locked version and a major bump is by definition a change of meaning.
4. **A module without a version cannot be imported**, and a module whose declared name is not the name
   it was imported by is refused: a locked build has to be able to say which module it locked.
5. **Elaboration is children before parents**, so an instance's imports always precede it. A cycle
   therefore has no first instance and is refused outright rather than resolved by picking one.
6. **Parameters are bound at the import, and every one of them is bound.** A `param` with a `default`
   may be left alone; one without may not, and a binding for a parameter the module does not declare is
   refused rather than ignored — a typo in a parameter name is a description that does not mean what its
   author thinks, which is the most expensive thing a toolchain can accept silently.

## 7. Kinds, and the one primitive that is not declared in eADL

A **kind** says what a declaration may contain: whether it carries a name, which clauses it has, how
often each may appear, and what shape their values have. Kinds are declared in eADL, with `defkind`,
and registered; `docs/semantics/kinds/core.eadl` declares the surface kinds and
`docs/semantics/kinds/os-rt.eadl` the workload ones.

1. ⭐ **`defkind` is the only trusted primitive.** Every other kind — `defblock`, `defplatform`,
   `defservice`, `defpolicy`, `defsystem` — is declared in eADL using `defkind`, and is therefore no
   more privileged than a kind a user adds tomorrow. `defkind`'s own meaning is Rust, in
   `crates/eadl-model/src/kind.rs`, and that is the whole of the trusted foundation. If a future change
   needs a second trusted primitive it has to be written there, where it is visible — which is what
   §2's "the registry cannot silently introduce new trusted axioms" means in practice.
2. ⛔ **§5.6's prohibition is enforced against the facility that declares the language, not around
   it.** "It must not become a host-code evaluator or an implementation template language" is not a
   comment: the boundary classifier runs over the `defkind` form itself, so
   `(defkind x (implementation …))` is refused by the same machine that refuses it in an ordinary
   declaration. There is no back door through the thing that defines the doors.
3. **A kind defines well-formedness and nothing else.** Its fields are `doc`, `name` and `clause`, and a
   field outside those is refused — because a kind that could carry behavior would be a template
   language wearing a schema's clothes. A kind must say what it is for: a kind nobody can explain is a
   kind nobody should have to guess at.
4. **A clause's cardinality is `one`, `at-most-one`, `one-or-more` or `any`**, and its content is
   `forms` (any nested forms), `values <type>…` (a fixed number of typed values, where a type is
   `symbol`, `integer`, `decimal`, `number`, `string` or `any`), or `kind <name>` (each occurrence is
   itself validated as a declaration of that kind — which is what makes the schema recursive, and what
   let `(clause task (holds kind task))` close a gap the corpus had not reached).
5. ⚠️ **What a schema does not check.** It validates the declaration *frame*: is this a known kind, does
   it carry a name when its kind requires one, are its clauses known, do they appear an allowed number
   of times, are their values the declared shape. It does **not** interpret the constraint vocabulary
   inside a clause — `(at-least 60 s)` is nested forms at this stage and becomes a checked quantity
   later. Claiming otherwise would be the more dangerous kind of green.

## What this reference does not yet carry

Stated rather than left implicit, because a reference that quietly omits a rule reads as though the
rule does not exist:

| Not here yet | Where it will be | Leaf |
| --- | --- | --- |
| a version identifier on the surface, and the frozen compatibility baseline | `eadl/1`, with the corpora as that version's conformance suite | `M1.13` |
