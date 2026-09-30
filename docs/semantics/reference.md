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
| `crates/eadl-front/src/language_version.rs` | §8 the language version a description states, and every diagnostic in §4 whose code begins `language-version-` |

⚠️ **A source belongs here when a chapter of this file states rules about it**, not when it merely
happens to be nearby. `crates/eadl-model/src/kind.rs` is in a different crate from the frontend and is
declared anyway, because §7 states rules it enforces and §4 states its codes — the declaration is about
*what this document governs*, not about where the code lives. `crates/eadl-model/src/boundary.rs` is
deliberately **not** here: it classifies implementation syntax, and the boundary is a profile concern
documented in `docs/book/src/boundary.md`. Adding it to this table before a chapter of this file states
its rules would make the census demand rows for codes nothing here explains. Its rule and its code are
stated by `docs/semantics/model.md`.

⚠️ **So this file governs the surface, and the surface is not the whole toolchain.** The model layer
above it — quantities and units, presence and absence, refinement, the profile check — emits diagnostics
of its own, and `docs/book/src/` renders some of them to the reader. Those are stated by
`docs/semantics/model.md` (leaf `M1.26.1`), whose declaration and census mirror this file's and are read
by the same reader in `crates/eadl-front/tests/reference.rs`. A code has one home: the test refuses a code
both documents state. This file's scope does not widen to cover them, because a quantity's unit or a
refinement's direction is a rule about *meaning*, and §7 rule 5 is explicit that this reference does not
interpret meaning.

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
| `<N*C>` | in a **source** cell only: the one character `C`, `N` times, for an input whose point is its size — `<3*(>` is `(((` |
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

⭐ **§4's `fires on` column is an input, and it is run** (leaf `M1.26.2`). A row whose code no input can
reach any more would otherwise read exactly like a live rule, because the census only asks whether a source
still contains the constructor. So each cell names a door into the toolchain and what to feed it, and
`crates/archogen-cli/tests/fires_on.rs` executes every one and requires the row's code among what fires:

| Verb | Means |
| --- | --- |
| `check T` | `T` is a description, checked as `archogen check` checks one |
| `modules T <mod N> U …` | `T` is a description, and each `<mod N>` segment is the module file `N.eadl` beside it |
| `kinds K <validate> D` | `K` is a kind module, loaded as the toolchain loads its own; then each declaration in `D` is validated against the registry `K` built |
| `none: R` | no writable input fires the code, for the reason `R`, which may not be empty |

The `<0xNN>` and `<N*C>` markers work in these cells as in a source cell. A cell nothing can parse is a violation, and
never a skipped row.

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
| `12.5` | `rational 125/10^1` | `12.5` |
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
   ⭐ **And a description in the conformance suite writes one.**
   `docs/semantics/cases/positive-decimal-quantity.eadl` declares `(period 12.5 ms)` — an 80 Hz control
   loop, which no integer number of milliseconds can spell — so this rule is exercised through a real
   declaration and not only through the rows above. Until that case existed the census instrument
   reported `decimal literals : 0` over every description the repository ships, against 194 integer
   occurrences and 13 strings: a rule the specification states and no description uses is a rule the
   suite does not conform-test, and §12 M2's standard for an independent checker is not met by rows
   alone.
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
| code | when it fires | what to do | fires on |
| --- | --- | --- | --- |
| `read-unclosed-list` | a `(` is never matched and the input ends inside it | add the matching `)`; the diagnostic also labels where the list opened | `check (defsystem heartbeat` |
| `read-nesting-too-deep` | a list opens inside 256 others. The reader refuses it and reads nothing inside it, so no later pass meets the depth. No construct of the language nests more than a few levels, and the limit is a property of this version of the language, as the value domain is (§1 rule 9), so every reader refuses the same descriptions | flatten the description: a list may nest at most 256 deep | `check <257*(><257*)>` |
| `read-unexpected-close` | a `)` appears with no list open | remove it, or add the `(` that was meant to open a list | `check (defsystem heartbeat))` |
| `read-unexpected-character` | a byte that can start nothing **between forms**, which in practice means a stray control character; inside a string the same byte is `read-control-character` | delete it — a form is a list `(…)`, a symbol, a number or a string | `check (defsystem heartbeat)<0x1b>` |
| `read-missing-delimiter` | a string is followed directly by a character a symbol may contain, with no delimiter between them. The grammar requires a delimiter after every atom, and a symbol or a number runs until one, so a string is the only atom that can end without one. A byte that can start nothing there is `read-unexpected-character` instead | separate the two with a space, or move the text inside the string | `check (defsystem heartbeat (doc "a"b))` |
| `read-unterminated-string` | a `"` is not closed before the end of its line, or before the end of the input | close the string on its own line, or escape the newline as `\n`; there is no multi-line string (§2 rule 3) | `check (defsystem heartbeat (doc "no end))` |
| `read-bad-escape` | a backslash inside a string is followed by anything other than the escapes §2 defines, including a `\u` escape that is not shaped like one | use one of those; the diagnostic names the set | `check (defsystem heartbeat (doc "\q"))` |
| `read-control-character` | a **raw** control character inside a string: anything in Unicode `Cc` except tab, which is whitespace the grammar names (§2 rule 2) | write the escape instead — `\n`, `\t`, `\r`, or `\u{…}` for any other character | `check (defsystem heartbeat (doc "a<0x07>b"))` |
| `read-escape-out-of-range` | a **well-formed** `\u{…}` escape that names no Unicode scalar value — a surrogate, or a code point past `10ffff` (§2 rule 2) | write a code point that denotes a character; a surrogate denotes one only inside a UTF-16 encoding | `check (defsystem heartbeat (doc "\u{d800}"))` |
| `read-malformed-number` | an atom that begins with a digit, or with a sign and a digit, and is not a number — a second decimal point, an exponent, a unit glued to the magnitude, `0x` with no digits after it, or a separator leading them | write an integer or a decimal; a unit goes in a following atom, `10 ms` | `check (defsystem heartbeat (priority 1.2.3))` |
| `read-number-overflow` | a **well-formed** literal whose value lies outside the 64-bit signed range, which §1 rule 9 makes a property of this version of the language rather than of an implementation | reduce the magnitude or change its units; an address with its most significant bit set is writable as the negative value it is two's-complement equal to (§1 rule 10) | `check (defsystem heartbeat (priority 9223372036854775808))` |
| `module-not-a-module` | a file resolved as a module does not begin with a `defmodule` declaration | write `(defmodule <name> (version <major> <minor>) …)` | `modules (defmodule app (version 1 0) (import hw.plain)) <mod hw.plain>(defblock plain (offers (p 1 bit)))` |
| `module-missing-name` | a `defmodule` carries no name | write `(defmodule platform.timer (version 1 0) …)` | `check (defmodule (version 1 0))` |
| `module-bad-version` | `version` is not a major and a minor integer | write `(version 1 0)` | `check (defmodule app (version one zero))` |
| `module-missing-version` | a module declares no version | write `(version 1 0)` — an unversioned module cannot be required by an importer, and §15 needs a version to lock | `check (defmodule app (export thing) (defblock thing (offers (p 1 bit))))` |
| `module-bad-param` | a `param` clause does not name its parameter | write `(param tick-rate (default 10 MHz))` | `check (defmodule app (version 1 0) (param (default 1)))` |
| `module-bad-export` | an `export` entry is not a name | write `(export timer.counter timer.compare)` | `check (defmodule app (version 1 0) (export "thing"))` |
| `module-conflicting-export` | one name is exported twice by one module | export each name once — two exports give an importer two answers and no rule for choosing | `check (defmodule app (version 1 0) (export thing) (export thing) (defblock thing (offers (p 1 bit))))` |
| `module-dangling-export` | a module exports a name it does not declare | export only what the module declares, or declare it | `check (defmodule app (version 1 0) (export absent) (defblock present (offers (p 1 bit))))` |
| `module-bare-form` | a module holds a form that is neither a declaration nor a module clause | every item in a module is a declaration or a clause; move anything else out | `check (defmodule app (version 1 0) stray)` |
| `module-empty` | a module file holds no declaration | a module file holds exactly one `(defmodule …)` form | `modules (defmodule app (version 1 0) (import hw.nothing)) <mod hw.nothing>(eadl-version eadl/1)` |
| `module-multiple-forms` | a module file holds more than one top-level form | keep one `(defmodule …)` per file and move the rest into their own modules | `check (defmodule app (version 1 0)) (defblock stray (offers (p 1 bit)))` |
| `module-not-found` | an import names a module no file in the module path holds — `a.b` is read from `a.b.eadl` in the directory holding the description (§6 rule 7) | check the name, or put the module beside the description as `<name>.eadl` | `check (defmodule app (version 1 0) (import hw.absent))` |
| `module-name-mismatch` | the name a module declares is not the name it was imported by | make them match — a locked build cannot otherwise tell which module it locked | `modules (defmodule app (version 1 0) (import hw.misnamed)) <mod hw.misnamed>(defmodule hw.clock (version 1 0))` |
| `module-too-large` | a module's text is larger than a span can address: over 4 GiB, because a span is a 32-bit byte offset (`SourceMap::add`) | split the module | `none: a module text over 4 GiB, which a span, a 32-bit byte offset, cannot address; no fixture of any sane size carries one (SourceMap::add in crates/eadl-front/src/source.rs)` |
| `module-circular-import` | a module imports something that imports it back | break the cycle: elaboration is children-before-parents, so a cycle has no first instance | `modules (defmodule app (version 1 0) (import cycle.b)) <mod cycle.b>(defmodule cycle.b (version 1 0) (import cycle.c)) <mod cycle.c>(defmodule cycle.c (version 1 0) (import cycle.b))` |
| `module-bad-import` | an `import` does not name a module | write `(import platform.timer (as timer))` | `check (defmodule app (version 1 0) (import (as timer)))` |
| `module-unknown-import-clause` | an import holds a clause that is not `as`, `version` or `with` | an import holds `as`, `version` and `with` | `modules (defmodule app (version 1 0) (import hw.timer (alias timer))) <mod hw.timer>(defmodule hw.timer (version 1 0))` |
| `module-bad-alias` | `as` is not given a namespace name | write `(as timer)` | `modules (defmodule app (version 1 0) (import hw.timer (as "timer"))) <mod hw.timer>(defmodule hw.timer (version 1 0))` |
| `module-conflicting-alias` | two imports in one module bind the same alias | give one a different namespace, e.g. `(as timer_2)` — one alias for two imports makes every qualified name ambiguous | `modules (defmodule app (version 1 0) (import hw.timer (as t)) (import os.time (as t))) <mod hw.timer>(defmodule hw.timer (version 1 0)) <mod os.time>(defmodule os.time (version 1 0))` |
| `module-bad-version-requirement` | an import's `version` is not `(version (at-least <major> <minor>))` | write `(version (at-least 1 0))` | `modules (defmodule app (version 1 0) (import hw.timer (version 1 0))) <mod hw.timer>(defmodule hw.timer (version 1 0))` |
| `module-incompatible-version` | the module found does not satisfy the requirement | the majors must be equal and the minor at least the required one; a major bump is never silently accepted | `modules (defmodule app (version 1 0) (import hw.timer (version (at-least 2 0)))) <mod hw.timer>(defmodule hw.timer (version 1 0))` |
| `module-unknown-parameter` | an import binds a parameter the module does not declare | the diagnostic lists the parameters it does declare | `modules (defmodule app (version 1 0) (import hw.timer (with (tickrate (tick-rate 1 MHz))))) <mod hw.timer>(defmodule hw.timer (version 1 0) (param tick-rate (default (tick-rate 10 MHz))))` |
| `module-missing-argument` | an import leaves a parameter that has no default unbound | add `(with (<param> <value>))` to the import | `modules (defmodule app (version 1 0) (import hw.sized)) <mod hw.sized>(defmodule hw.sized (version 1 0) (param size))` |
| `module-bad-argument` | a `with` binding is not written `(<param> <value>)` | write `(with (tick-rate 20 MHz))` | `modules (defmodule app (version 1 0) (import hw.timer (with 5))) <mod hw.timer>(defmodule hw.timer (version 1 0) (param tick-rate (default (tick-rate 10 MHz))))` |
| `module-not-exported` | a name written through an import's alias is not one that import's module exports (§6 rule 10) | export it from that module, or name one of its exports — the diagnostic lists them | `modules (defmodule app (version 1 0) (import hw.private (as parts)) (defsystem app.rt (requires (uses parts.private.part)))) <mod hw.private>(defmodule hw.private (version 1 0) (export public.part) (defblock public.part (offers (p 1 bit))) (defblock private.part (offers (p 1 bit))))` |
| `schema-not-a-kind` | a form read as a kind definition is not a `defkind` | write `(defkind <head> (doc "…") (name …) (clause …) …)` | `kinds (defservice x)` |
| `schema-missing-kind-head` | a `defkind` does not name the declaration head it defines | write `(defkind defservice …)` | `kinds (defkind (doc "a thing") (name required))` |
| `schema-duplicate-kind` | one declaration head is defined twice | remove one — redefining a kind would silently change what already-written descriptions mean | `kinds (defkind defthing (doc "a thing") (name required)) (defkind defthing (doc "a thing") (name required))` |
| `schema-missing-doc` | a kind definition does not say what the kind is for | write `(doc "one line saying what this kind describes")` | `kinds (defkind defthing (name required))` |
| `schema-bad-doc` | `doc` is not one quoted string | write `(doc "what this kind is for")` | `kinds (defkind defthing (doc 42) (name required))` |
| `schema-unknown-kind-field` | a kind definition holds a field that is not `doc`, `name` or `clause` | it holds those and nothing else, because it defines well-formedness rather than behavior | `kinds (defkind defthing (doc "a thing") (name required) (colour red))` |
| `schema-bad-name-rule` | `name` is neither `required` nor `forbidden` | write `(name required)` when the declaration is written `(<head> <name> …)` | `kinds (defkind defthing (doc "a thing") (name sometimes))` |
| `schema-missing-clause-head` | a `clause` does not name the clause it declares | write `(clause <name> (cardinality …) (holds …))` | `kinds (defkind defthing (doc "a thing") (name required) (clause (cardinality one) (holds forms)))` |
| `schema-duplicate-clause` | one clause is declared twice in a kind | declare each once, and use `(cardinality any)` to allow repetition in a description | `kinds (defkind defthing (doc "a thing") (name required) (clause c (cardinality one) (holds forms)) (clause c (cardinality one) (holds forms)))` |
| `schema-duplicate-name` | two declarations carry one name — in a module tree, after §6 rule 9 has named them (§7 rule 6) | rename one: a name means one declaration, and the diagnostic names both sites | `check (defblock b (offers (p 1 bit))) (defblock b (offers (p 1 bit)))` |
| `schema-unknown-clause-field` | a clause declaration holds a field that is not `cardinality` or `holds` | a clause declaration holds those two | `kinds (defkind defthing (doc "a thing") (name required) (clause c (cardinality one) (holds forms) (colour red)))` |
| `schema-bad-cardinality` | `cardinality` is not one of `one`, `at-most-one`, `one-or-more`, `any` | use one of those spellings; the diagnostic lists them | `kinds (defkind defthing (doc "a thing") (name required) (clause c (cardinality lots) (holds forms)))` |
| `schema-bad-holds` | `holds` is not `forms`, `values <type>…` or `kind <name>` | write `(holds kind task)` to have each occurrence validated as a declaration of that kind | `kinds (defkind defthing (doc "a thing") (name required) (clause c (cardinality one) (holds everything)))` |
| `schema-bad-value-type` | a `holds values` type is not one the schema knows | the value types are `symbol`, `integer`, `decimal`, `number`, `string`, `any`, `quantity` | `kinds (defkind defthing (doc "a thing") (name required) (clause c (cardinality one) (holds values colour)))` |
| `schema-unknown-referenced-kind` | a clause holds a kind that is not registered | register the module that defines it — the workload kinds live in `docs/semantics/kinds/os-rt.eadl` | `kinds (defkind defthing (doc "a thing") (name required) (clause c (cardinality one) (holds kind nosuch))) <validate>(defthing x (c (y)))` |
| `schema-not-a-declaration` | a top-level form is not a declaration | declarations are written `(defservice time.monotonic …)` | `check stray` |
| `schema-unknown-kind` | a declaration's head is not a registered kind | the diagnostic names the kinds that are | `check (defthing x)` |
| `schema-missing-name` | a declaration of a kind that requires a name carries none | write `(<head> <name> …)` | `check (defservice (requires (needs counter-width)))` |
| `schema-not-a-clause` | a declaration holds a form that is not one of its kind's clauses | the diagnostic lists the clauses that are | `check (defservice time.monotonic stray)` |
| `schema-unknown-clause` | a declaration holds a clause its kind does not define | the diagnostic lists the clauses it does define | `check (defservice time.monotonic (requries (needs counter-width)))` |
| `schema-cardinality` | a clause appears a number of times its kind's cardinality forbids | keep the number of `(clause …)` occurrences the cardinality allows | `check (defblock b (absent debug-port))` |
| `schema-arity` | a clause does not hold the number of values its kind requires | the diagnostic names the clause, the number required and the number found | `check (defsystem s (task t (period 10 ms) (deadline 10 ms) (priority 1 2)))` |
| `schema-type` | a value in a clause is not of the type the kind declares for it | write a value of the declared type | `check (defsystem s (task t (period 10 ms) (deadline 10 ms) (priority fast)))` |
| `language-version-unknown` | a **well-formed** `(eadl-version …)` names a version this toolchain does not read (§8 rule 6) | write `(eadl-version eadl/1)`; a description is never silently re-read as another version | `check (eadl-version eadl/9)` |
| `language-version-not-an-identifier` | the identifier is not a bare symbol — a string, a number or a list (§8 rule 5) | write `(eadl-version eadl/1)`; `"eadl/1"` is a string and is not the same atom | `check (eadl-version "eadl/1")` |
| `language-version-missing` | an `(eadl-version …)` form carries no identifier at all | write `(eadl-version eadl/1)` | `check (eadl-version)` |
| `language-version-extra-argument` | an `(eadl-version …)` form carries more than the one identifier | keep `(eadl-version eadl/1)` and delete what follows it | `check (eadl-version eadl/1 eadl/2)` |
| `language-version-duplicated` | a description states its language version more than once (§8 rule 5) | keep one `(eadl-version eadl/1)` and delete the other | `check (eadl-version eadl/1) (eadl-version eadl/1)` |

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
7. **An import is found by a stated rule, and verified by the name the module declares.** The module
   `(import platform.timer …)` names is read from the file `platform.timer.eadl` in the **module path**,
   which is the directory holding the description the command was given — one directory, so no two files
   can compete for one name and no search order has to be chosen. The file is *found* by the name the
   importer wrote and *verified* by the name its `defmodule` declares, and a disagreement is refused by
   rule 4: two independent statements that must agree, which is what lets a locked build say which module
   it resolved. A file that does not exist is `module-not-found`. A file that exists and cannot be read is
   a failure of the invocation, reported the way an unreadable description is, and not a statement about
   the description. The description the command was given is never looked up, so its own declared name is
   not compared with its file name. `crates/eadl-front/src/module.rs`'s `DirectoryModules` implements this
   rule, and `crates/archogen-cli/tests/module_cases.rs` drives every case under `docs/semantics/modules/`
   through it.
8. **A module name is one that can only mean one file.** One or more segments joined by single dots, each
   segment a lowercase ASCII letter followed by lowercase letters, digits, `-` and `_`: `platform.timer`,
   `os.rt-core`. Rule 7 turns the name into a file name, so the name has to denote the same file on every
   machine — no separator can leave the module path (`../x` names no module), no segment is empty
   (`a..b`), and no uppercase letter exists, because on a case-insensitive filesystem `HW.Timer` and
   `hw.timer` would be one file on one machine and two on another. An import naming anything else does not
   name a module, which is `module-bad-import`.
9. **An elaborated declaration is named by the path of its instance.** A declaration named `n` in the
   instance at alias path `p` is `p.n` in the elaborated program — `timer.counter`, in `hw.timer` imported
   as `timer` by `hw.soc` imported as `platform`, is `platform.timer.timer.counter` — and a declaration of the
   description the command was given keeps its own name. Two instances of one module therefore declare
   different names, which is what rule 1 needs of them.
10. **A name written inside an instance resolves in that instance's scope, and an import shows only what its
    module exports.** The operands of `uses`, `needs` and `refines` are the positions that name another
    declaration, and each is resolved in order: a declaration of the same instance, by its local name; then a
    name written through one of the instance's own aliases, `alias.n`, where `n` must be one of the names
    that import's module **exports** — anything else written through an alias is `module-not-exported`;
    then, neither, the name is left as written, because it is a name of the capability vocabulary
    (`counter-width`, `absolute-deadline`) and not of a declaration. An alias never shadows a local
    declaration. There is no re-export, so an importer sees exactly the exports of the modules it imports
    directly: `app.system` cannot name `platform.timer.timer.counter`, because `hw.soc` does not export it.
    `crates/eadl-model/src/check.rs`'s `check_program` applies this rule and then runs every pass a single
    description gets, so a module tree is judged by exactly the same rules.

⚠️ **Rule 9 does not by itself make a name mean one declaration**: a module that declares `inner.x` and
also imports a module as `inner` that declares `x` gives both the name `p.inner.x`. That is refused by §7
rule 6 — a name is declared once — which covers a single description and a module tree alike.

⚠️ **One code has no fixture, and the reason is stated rather than hidden.** `module-too-large` fires when
a module's source cannot be given an address — at 2^32 bytes — and no tracked fixture carries a
four-gigabyte file. Every other `module-` code in §4 is produced by a case under
`docs/semantics/modules/` through `archogen check`. The row's own wording is finding **F-H**, owned by
leaf `M1.26.2` in `docs/tasks/M1.md`.

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
   `symbol`, `integer`, `decimal`, `number`, `string`, `any` or `quantity` — and `quantity` is the one
   type that consumes **two** values, a number and a unit, because a clause declared
   `(holds values number symbol)` says nothing about whether the pair measures anything), or
   `kind <name>` (each occurrence is
   itself validated as a declaration of that kind — which is what makes the schema recursive, and what
   let `(clause task (holds kind task))` close a gap the corpus had not reached).
5. ⚠️ **What a schema does not check.** It validates the declaration *frame*: is this a known kind, does
   it carry a name when its kind requires one, are its clauses known, do they appear an allowed number
   of times, are their values the declared shape. It does **not** interpret the constraint vocabulary
   inside a clause — `(at-least 60 s)` is nested forms at this stage and becomes a checked quantity
   later. Claiming otherwise would be the more dangerous kind of green.
6. **A name is declared once.** Two declarations carrying one name are refused with both sites named,
   never merged and never resolved by order: choosing one would silently decide which half of the
   description the author meant, which is what §5.3 of `ROADMAP.md` forbids for facts. The rule is over
   the declarations the pipeline checks, so it covers a module tree after §6 rule 9 has named them — where
   a module that declares `inner.x` and imports a module as `inner` that declares `x` has made one name of
   two. `crates/eadl-model/src/kind.rs`'s `duplicate_names` enforces it.

## 8. The language version a description states

A description says which version of this language it is written in, so that `ROADMAP.md` §15's promise
— *a source description retains its meaning under its locked semantic version* — is something a reader
can **determine from the description** rather than assume from whichever toolchain happens to be
reading it.

<!-- machine-read: language-version -->
| source | verdict | identifier |
| --- | --- | --- |
| `(eadl-version eadl/1)` | clean | `stated` |
| `(defsystem s)` | clean | `absent` |
| `(defsystem s (eadl-version eadl/1))` | clean | `absent` |
| `(eadl-version eadl/1) (defmodule m (version 1 0))` | clean | `stated` |
| `(eadl-version eadl/2)` | `refused language-version-unknown` | — |
| `(eadl-version eadl)` | `refused language-version-unknown` | — |
| `(eadl-version "eadl/1")` | `error language-version-not-an-identifier` | — |
| `(eadl-version 1)` | `error language-version-not-an-identifier` | — |
| `(eadl-version)` | `error language-version-missing` | — |
| `(eadl-version eadl/1 eadl/1)` | `error language-version-extra-argument` | — |
| `(eadl-version eadl/1) (eadl-version eadl/1)` | `error language-version-duplicated` | — |

### The rules those rows state

1. ⛔ **The identifier is a form, not a comment header.** §3 states that canonical form carries **no
   comment**, and §12 M4 hashes canonical text — so a version written in trivia is a version the hashed
   artifact does not capture: two descriptions differing only in the language version they claim would
   hash identically, and any tool that strips comments would strip the lock along with them. The header
   route is genuinely available (§5 makes one shape of comment data, and
   `crates/eadl-front/src/form.rs` reads it) and is still wrong here, for that reason and no other.
2. ⭐ **Absence denotes `eadl/1` — by rule, not by default.** A description that carries no identifier
   is not missing information; the rule above says what it denotes, which is what makes the version
   determinable from the description alone. ⛔ **And `eadl/1` is the last version for which that is
   true.** A default that outlives the version it defaults to is exactly how a description silently
   changes meaning, so the day a second version exists, absence must be **refused** and this rule
   retired with a migration note. What makes the rule acceptable now is that the descriptions relying
   on it are known rather than assumed: the frozen LinkedSpec evidence under `docs/feedback/`, whose
   bytes *are* the reproduction of another project's defect and which nothing in `crates/` reads.
   ⭐ **That population is measured, and it is a gate rather than a sentence.**
   `crates/eadl-front/tests/reference.rs` requires every description the repository ships outside
   `docs/feedback/` to state its version, asked of the frontend through `Document::stated_version`
   rather than of a text search — so a nested, quoted or unread identifier does not satisfy it. The
   rule was written while **62** live descriptions relied on absence and this sentence named only the
   frozen evidence; `M1.13.4.2` wrote the identifier into all 62, and the leg is what stops the next
   case added to a corpus from silently widening a population a normative rule names. Retiring the rule
   therefore costs one migration and not two: the frozen evidence keeps reading, because nothing here
   reads it, and what starts being refused is a *new* description that omits its version — which is the
   entire point of the expiry.
3. **This is not a grammar change, and that is measured rather than convenient.**
   `docs/semantics/grammar.md` fixes *shape* and names no construct vocabulary — its own table says "the
   language is extended by `defkind`, not by editing this file", and `document` already admits any
   s-expression. So `(eadl-version eadl/1)` was well-formed before this section existed and the
   recognizer `conformance.rs` derives from that grammar accepts it unchanged. What this section adds
   is **meaning**, which is the half the two normative documents exist to keep apart.
4. ⛔ **The head symbol is `eadl-version`, not `version`, because `version` is taken.** §6 uses
   `version` as a clause of `defmodule` for the *module's* own version, and an import's requirement
   spells it `(version (at-least 1 2))`. One name carrying two different versions in one file is a
   collision a reader could only resolve by nesting depth, which is the kind of rule that looks
   unambiguous until some tool forgets the depth. Row 4 states the two together, so the combination is
   executed and not merely permitted.
5. **The identifier is one bare symbol.** `/` is an ordinary `symbol_char`, so `eadl/1` needs no
   quoting — and `"eadl/1"` is a *string*, a different value under §2, refused rather than accepted as
   an interchangeable spelling of the same thing. Row 3 is the mirror image: only a **top-level** form
   states the version, so a clause named `eadl-version` inside a declaration denotes nothing and cannot
   be mistaken for a statement.
6. **A version this toolchain does not read is refused, never re-interpreted.** Reading `eadl/2` as
   `eadl/1` would be the precise failure §15 exists to prevent: a description whose meaning changed
   without anyone editing it. Refusing is also why the diagnostic is a `refused` row and not an `error`
   one — the form is well-formed, and what is out of range is its *value*, the same distinction §1 draws
   for a literal outside the integer domain.
7. **In a module file the identifier precedes the declaration and is not a stray form.** §6's "exactly
   one top-level form" is about the declaration; `crates/eadl-front/src/module.rs` filters the
   identifier before counting, so a module can state its language version without being refused as
   `module-multiple-forms`. Without that, the one file kind that most needs a locked version would be
   the one unable to carry it.
8. ⛔ **The identifier is not a declaration, and three layers had to be told.** §7's schema layer
   validates every top-level form it is given against the kind registry, and `eadl-version` is not a
   kind — so without an exemption a description that states its version reads cleanly and is then
   refused as
   `schema-unknown-kind`, contradicting this section from the layer furthest from it, where no frontend
   test can see it happen. Rule 7 is the same problem in miniature. The **third** is the kind registry
   itself: `crates/eadl-model/src/check.rs` reads the kind modules under `docs/semantics/kinds/` and
   handed every top-level form to `read_kind`, so a kind module stating its own version was refused as
   `schema-not-a-kind` and took the whole registry with it — `tool-failure`, not a verdict about
   anybody's description. No description-level test could reach it, because nothing reads a kind module
   except that loader; it was found by writing the identifier into all 62 corpus descriptions and
   running the suite, which is `M1.13.4`'s measurement and 36 of its 45 failures.
   ⭐ **So the rule now has one accessor and not only one predicate**:
   `crates/eadl-front/src/language_version.rs` exports `declarations`, the top-level forms that *are*
   declarations, and every pass that treats a form as one goes through it — because a rule each
   consumer re-implements is a rule the next consumer lacks, the reasoning that put §2's escape rule in
   the printer. That includes the count a reader sees: `archogen check` reports how many declarations
   it accepted, so a consumer that counted forms would report one too many for every description that
   states its version. Measured, not anticipated: both refusals were reproduced through the real entry
   points — `check` for the schema layer, `shipped_registry` for the kind loader — before either
   exemption existed, and each fix carries a mutation arm showing the leg that fails without it.
   ⚠️ `crates/eadl-model/src/check.rs` is **not** a declared source in the table above, and that is not
   an oversight: §4's census runs in both directions over exactly that list, so declaring a source
   demands a row for every code it can emit, and this one emits the model layer's diagnostics — which
   no normative document states yet. That is leaf `M1.26`'s gap (a), and it is what has to close first.

⚠️ **Honest limit: the identifier is read and refused, and nothing yet *acts* on it.** There is one
version, so no behaviour differs between stating it and omitting it, and saying otherwise would be the
more dangerous kind of green. What makes it more than a placeholder is that `M1.13.4` digests the
corpora into `eadl/1`'s frozen baseline, so "differs" becomes checkable the moment a second version
exists — and rule 2's expiry is what forces that question to be answered rather than defaulted.

## What this reference does not yet carry

Stated rather than left implicit, because a reference that quietly omits a rule reads as though the
rule does not exist:

| Not here yet | Where it will be | Leaf |
| --- | --- | --- |
| the frozen compatibility baseline, and the corpora as one version's conformance suite | §8 states the identifier; the baseline that makes a second version's differences checkable is next | `M1.13` |
