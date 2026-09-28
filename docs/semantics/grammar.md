# The eADL surface grammar

**Normative.** This file defines what a well-formed eADL description *is*. It is the authority the
reader implements, not a description of the reader — and that direction is enforced:
`crates/eadl-front/tests/conformance.rs` derives a recognizer from the EBNF block below and
requires it and `crates/eadl-front/src/reader.rs` to agree on every description in the corpus. A
description one accepts and the other rejects fails the build.

> ⛔ **Why this file exists.** Before it, eADL's surface syntax was defined only by a 743-line
> hand-written parser, and every test in the repository validated against that parser — which
> made it unfalsifiable as a definition. `ROADMAP.md` §4.1 assigns the eADL project ownership of
> *syntax*; §12 M1 anticipates the syntax freezing at a compatibility baseline; §4.4 requires an
> independently derived checker whose parser is a declared trust dependency. None of those is
> possible against an implementation. Leaf `M1.11`.

## Scope

This grammar fixes the **surface**: what characters form tokens, and what shapes of form a
document may contain. It deliberately does **not** fix meaning. That `(counter-width 32 bit)` is a
quantity, that `defblock` introduces a hardware block, that `bit` is a unit — none of that is
here. Those are the kind registry (`docs/semantics/kinds/`) and the typed model, and keeping the
grammar free of them is what lets one reader serve the boundary corpus, the S0 fixtures and the
semantic corpus without any of them leaking assumptions into it.

Everything the grammar cannot carry — exactness of numbers, comment retention, canonical form,
module semantics — is stated normatively in the language reference,
[`docs/semantics/reference.md`](reference.md), which is executed against the frontend the same way
the EBNF below is.

## Notation

A small ISO-style EBNF. The recognizer in `crates/eadl-front/tests/conformance.rs` reads exactly
this dialect and nothing more, so the notation cannot quietly grow past what is checkable.

| Form | Means |
| --- | --- |
| `name = expr ;` | a production |
| `"text"` | a literal, matched exactly |
| `"a" .. "z"` | any single character in the inclusive range |
| `a , b` | `a` followed by `b` |
| `? a` | `a` must follow, but is **not consumed** (lookahead) |
| `end` | end of input |
| `a \| b` | `a` or `b`, first match wins |
| `( a )` | grouping |
| `[ a ]` | zero or one |
| `{ a }` | zero or more |
| `a - b` | `a`, but not if it also matches `b` (single characters only) |
| `any` | any single character |

## The grammar

```ebnf
document        = { trivia } , { form , { trivia } } , end ;

trivia          = whitespace | comment ;
whitespace      = " " | "\t" | "\r" | "\n" ;
comment         = ";" , { any - "\n" } , [ "\n" ] ;

form            = list | atom ;
list            = "(" , { trivia } , { form , { trivia } } , ")" ;

atom            = ( string | number | symbol ) , ? delimiter ;
delimiter       = whitespace | "(" | ")" | quote | ";" | end ;

string          = quote , { string_char } , quote ;
string_char     = escape | ( any - quote - "\\" ) ;
escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" | "0" ) ;
quote           = "\"" ;

number          = hexadecimal | decimal | integer ;
hexadecimal     = [ sign ] , "0x" , hex_digit , { hex_digit | "_" } ;
decimal         = [ sign ] , digit , { digit | "_" } , "." , digit , { digit | "_" } ;
integer         = [ sign ] , digit , { digit | "_" } ;
sign            = "+" | "-" ;
digit           = "0" .. "9" ;
hex_digit       = digit | "a" .. "f" | "A" .. "F" ;

symbol          = symbol_start , { symbol_char } ;
symbol_char     = any - whitespace - "(" - ")" - quote - ";" ;
symbol_start    = symbol_char - digit ;
```

### Three rules the productions above imply, stated because they are load-bearing

1. **A token ends where a delimiter begins**, and `atom`'s trailing `? delimiter` is what says
   so. `(`, `)`, `"`, `;` and whitespace terminate a token and are never part of one, so `(a)` is
   three tokens rather than one symbol `a)`.
2. **A symbol may not begin with a digit**, and the boundary rule is what gives that teeth.
   `1ms` cannot be an `integer` — the lookahead fails on `m` — and cannot be a `symbol` either,
   because `symbol_start` excludes digits. So it is a malformed number and must be reported as
   one. ⛔ Without the lookahead the grammar would happily read `1ms` as the two forms `1` and
   `ms`, and `(period 10ms)` would become a clause with one silent extra argument instead of a
   diagnostic. The rule is not decoration; it is the difference between a typo and a wrong system.
3. **`+` and `-` begin a number only when a digit follows.** `-` alone is a symbol; `-4` is a
   number. `sign` therefore belongs to `number`, and `symbol_char` admits `-` as an ordinary
   character — a reader that resolved this the other way would make `-4` an identifier.
4. **A symbol is anything that is not a delimiter.** This is deliberately permissive: an
   S-expression reader's job is to find token boundaries, not to police spelling. Restricting the
   character set would be a rule invented here rather than one the language needs, and
   `docs/semantics/kinds/` is where a name is actually checked.

## What is deliberately not here

| Not in the grammar | Where it lives | Why |
| --- | --- | --- |
| which head symbols are declarations | `docs/semantics/kinds/` | the language is extended by `defkind`, not by editing this file |
| what a clause means | `crates/eadl-model/` | the grammar is syntax; §5.6 keeps interpretation in the engine |
| units, quantities, contract IDs | `crates/eadl-model/src/quantity.rs` | `bit` is a symbol here and a unit there |
| that numbers are **exact** | [`docs/semantics/reference.md`](reference.md) §1 | a grammar cannot express "and never a float" |
| what each escape **denotes** | [`docs/semantics/reference.md`](reference.md) §2 | a grammar says what is shaped like an escape, not what it means |
| that comments are **retained** | the language reference (`M1.12.4`) | a grammar says what is skipped, not what is kept |
| canonical form | [`docs/semantics/reference.md`](reference.md) §3 | it is a property of printing, not of parsing |

⚠️ **Two spellings below are being reconciled with the reference, and the reference is the authority
on what a literal means.** `hexadecimal` spells its prefix `"0x"` where the reference admits `0X10`
as a value, and `escape` admits `"0"` where the reference refuses `\0` as `read-bad-escape`. Neither
spelling appears in any description the repository ships, so the conformance check below cannot reach
them; leaf `M1.12.2` runs the reference's own literal table against the recognizer derived from this
file, which is what makes both divergences impossible rather than merely noted.

⚠️ **No float, anywhere.** The grammar admits a decimal literal; §7.4 requires "exact integer time
units or checked rational arithmetic", so a conforming reader holds `1.5` as fifteen tenths and
never as an IEEE double. The grammar cannot say that, which is exactly why
[`docs/semantics/reference.md`](reference.md) is a separate normative document rather than a section
of this one — and its §1 states the value of every literal form as a table that
`crates/eadl-front/tests/reference.rs` executes.

## Conformance

A description **conforms** when the recognizer derived from this file accepts it *and* segments it
into the same tokens the reader does. `crates/eadl-front/tests/conformance.rs` checks four things:

| Check | Over |
| --- | --- |
| both accept | every `.eadl` file in `docs/semantics/` and `examples/` |
| both reject | the malformed fixtures |
| both produce the **same token spans** | every corpus file |
| both accept | 23 probes, one per production |

⛔ **Acceptance agreement alone is too weak, and that is measured rather than argued.** Dropping
`_` from hexadecimal literals in the reader left every acceptance test green while
`(base 0x1000_0000)` silently became the two forms `4096` and `_0000` — a memory-mapped base
address of `0x10000000` read as `4096`. Two implementations can agree on the *language* and
disagree on the *tokens*, and the token disagreement is the one that changes what a system means.
With segmentation compared, the same mutation fails with the offending byte offsets named.

⭐ **The probes exist because the corpus is not a conformance suite.** It contains exactly one
number with a digit separator, and that one is hexadecimal — so nothing in it reached decimal
separators, signs, escapes, CRLF, or several other productions. A suite that never exercises a
production is not evidence about it. `M1.12` measured that limit harder and found three literal
forms this file and the reader disagreed about — `0X10`, `0x_10` and `\0` — none of which any
description in the repository contains, so no check here could reach them. The census is in
`docs/tasks/M1.md`; `M1.12.2` derives the probe set from the reference's literal table so the gap
cannot reopen.

⚠️ **Honest limit.** This establishes that the two agree on the language and on where tokens
begin and end. It does **not** establish that the reader builds the right *tree* from those
tokens, or that it assigns the right *value* to each — a reader that tokenized identically and
mis-nested, or read `1.5` as three halves, would pass here. Nesting is the corpus suites'
business (`corpus.rs`, the semantic cases); value exactness is
[`docs/semantics/reference.md`](reference.md)'s, and
`crates/eadl-front/tests/reference.rs` is the leg that closed it.
