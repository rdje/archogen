# `eadl/1`'s integer value domain is exact signed 64-bit, and widening it is deferred

- **Type:** `decision`
- **Date:** `2026-09-28`
- **Status:** `active`
- **Owner / source:** leaf `M1.13.2` (tree `M1`), settling finding **F-F** that `M1.12` routed to the
  language freeze. Measured, not preferred — and re-measuring it corrected the routing twice, in both
  of the figures the routing rested on.
- **External sources:** [LinkedSpec](../book/src/ledger.md#linkedspec) · [chipdoc](../book/src/ledger.md#chipdoc) — version, scope and limits in the ledger

## The decision

`eadl/1` holds an integer in an **exact signed 64-bit value**: every value from `-9223372036854775808`
to `9223372036854775807` is writable, and a well-formed literal outside that range is
`refused read-number-overflow` — never wrapped, truncated, or re-read as a symbol. The domain is a
**property of the language version**, not of an implementation: it is stated normatively in
`docs/semantics/reference.md` §1 rules 9 and 10 and executed by table rows in **both spellings and both
directions**, so a reader that narrows or widens it fails the build rather than drifting.

**Widening is deferred, and deferring it is cheap** — because a widening is *backward compatible*:
every literal `eadl/1` reads keeps the value it has here, so no description changes meaning and none
needs migrating. It lands in a later version behind a migration note and **one added
compatibility-corpus row** (a literal refused under `eadl/1` and readable under the version that
widens), which is what §15's "any changed behavior must be explicit" costs in this case.

## Why — three measurements, each with the command that reproduces it

### 1. No requirement reaches the limit, and nothing in the corpus does either

```console
$ cargo run -q -p eadl-front --example literals -- docs/semantics examples docs/feedback
```

Over all **75** tracked descriptions: **194** integer-literal occurrences, **24** distinct values, and
**0** literals the domain refused. Over the **62**-file population the conformance suite walks
(`docs/semantics` and `examples`, with the frozen LinkedSpec evidence excluded): **180** occurrences,
**23** distinct values, **0** refusals and **0** diagnostics of any kind — so the census loses nothing
to a file that failed to read.

The largest value in either population is `4294967296` = 2^32, a counter modulus in
`docs/semantics/boundary/accept/counter-width-and-rate.eadl`; the largest hexadecimal is `0x1000_0000`
= 2^28, a memory-mapped base in `examples/periodic-three/system.eadl`. The largest needs **33** of the
domain's **63** magnitude bits, so 2147483647× the largest literal the project actually writes still
fits. The smallest is `-1`.

⭐ **The instrument asks the frontend, not a regular expression.** `crates/eadl-front/tests/reference.rs`
already states why: "a second tokenizer here would be a second thing to be wrong about". That is not a
taste — see correction A below, where a regular-expression census produced a figure that was wrong
twice over and was then restated seven times.

### 2. No *physical* address a standardized RV64 target can have reaches it either

Read out of the pinned specification in the read-only `chipdoc` corpus at revision `v20260120`
([[reference_external-document-source-chipdoc]]; never a build dependency, and the paths below are
relative to `ARCHOGEN_CHIPDOC_ROOT`):

| Source (digest from that pin's `SHA256SUMS`) | States |
| --- | --- |
| `risc-v/isa/pinned/v20260120/priv/machine.html` — `b63fe33517fef2dabe53cdbc45b05d656bf808eebf83f0af5a4ab74549617fea` | "For RV64, each PMP address register encodes bits 55-2 of a **56-bit physical address**" |
| the same file | "The Sv39 and Sv48 page-based virtual-memory schemes … support a **56-bit physical address space**, so the RV64 PMP address registers impose the same limit" |
| the same file, for the **narrower** base ISA | "Each PMP address register encodes bits 33-2 of a **34-bit physical address for RV32**"; "The Sv32 … scheme … supports 34-bit physical addresses for RV32" |
| `risc-v/isa/pinned/v20260120/priv/supervisor.html` — `d5186efc3ead1c1f693ac2b16aeccdcab95d990264f49b559252d2dc413a651a` | Sv57 "supports **57-bit** virtual address spaces"; Sv32 "support a **32-bit** virtual address space"; the Sv39 chapter requires bits 63-39 of an effective address to equal bit 38 |

So the widest physical address is 2^56−1 against a domain limit of 2^63−1: seven bits of headroom, and
measurement 1 says the project writes 2^32. A memory-mapped base address in the upper half of a
*physical* address space is writable, which is what F-F's original impact claim doubted.

⭐ **Both base ISA widths are measured, not only the one the emulator target uses.** archogen builds
`riscv64imac-unknown-none-elf` today (`xtask/src/main.rs`), but `board-first` names no processor yet —
every §3.2 row in `docs/targets/first-target.md` is `unrecorded` — and the first board candidate whose
documents are actually in hand is a SiFive FE310 family part ([[reference_external-document-source-chipdoc]],
`REQ-006` fulfilled). Its ISA width is stated in those datasheets and is deliberately **not** restated
here, because an ISA figure from memory is exactly what the chipdoc record forbids. What matters for
this decision is that the RV32 line is *narrower still* — 34-bit physical, 32-bit virtual — so whichever
width a board is eventually named at, the domain is not what limits it.

### 3. Widening moves the hole rather than closing it

`Form::Integer` holds an `i64` and `Form::Decimal` holds digits plus a scale, so moving to `i128`
leaves the same class of limit one width up. Only arbitrary precision removes it, and the workspace
carries **zero** third-party dependencies — [[decision_zero-dependency-engine-core]], which §4.4's
trust argument and §10.3's locked builds rest on. Arbitrary precision therefore means hand-written
arithmetic in the one crate every other layer's exactness depends on: a real cost, taken on for a
requirement measurement 1 says does not exist yet.

## ⛔ Two corrections, because the routing this decision settles was wrong in both of its figures

The auditor's asymmetry in `docs/CLAIM_VERIFICATION.md` §4 applies and is honoured here: a
re-derivation contradicts a figure that has at least been read, so each disagreement was attributed by
measurement before being published.

### A. The corpus census figure was false, and had been restated seven times

`M1.13`'s measurement M-G recorded "**27** distinct integer literals in the whole tracked corpus; the
largest is `0x1000_0000` (2^28, a memory-mapped base) and the largest decimal is 2^32". Two defects in
one sentence:

- **The population's scope was never stated.** 27 is the count of *non-negative* distinct values; the
  whole population holds **28**, the extra one being `-1`. Neither figure was labelled.
- **A radix-scoped maximum was published as the overall one.** 2^32 > 2^28, so "the largest is
  `0x1000_0000`" is false on the row's own terms — the row names both maxima and then calls the
  smaller one the largest.

Every restatement dropped the radix qualifier, so the false half is what propagated. Census of the
copies, run in the direction that finds them (`grep -rn '2\^28\|27 distinct\|largest literal'
--include='*.md'`, outside `target/` and `vendor/`): **7** occurrences over **3** live surfaces —
`docs/tasks/M1.md` (5), `docs/TASK_TREE.md` (1), `MEMORY.md` (1). None in the book, the grammar or the
reference. All 7 are corrected in the commit that lands this record.
[[a-moved-measurement-needs-a-census-of-its-copies]] is the card, and this is its third instance in
this repository.

⛔ **Recorded so nobody "fixes" the figure back:** the first re-derivation here was itself a regular
expression over file text, and it reported **28** distinct values over **303** occurrences. Both of its
figures are wrong for this question, because text in a *comment* is prose and not a literal — the
frontend's count is **24** over **194**. The disagreement is the reason the instrument is tracked: a
census whose producer is a description of a command cannot be re-run and checked, and a census whose
producer is a second tokenizer measures the wrong population confidently.

### B. The address argument was true of physical addresses and false of virtual ones

M-D concluded that "no address a standardized target can have reaches 2^57". Measured, that holds for
every **physical** address and fails for every canonical **high-half virtual** one:

```console
$ cargo run -q -p eadl-front --example diagnose -- <a file holding (base 0xFFFF_FFFF_C000_0000)>
error[read-number-overflow]: this hexadecimal literal does not fit in a 64-bit signed integer
```

An Sv39 kernel-space address has bits 63-39 all set, so its magnitude is 2^64−2^38 — above 2^63−1.
`0xFFFF_FC00_0000_0000` (the smallest Sv39 high-half address), `0xFFFF_FFFF_C000_0000` and
`0xFFFF_FF80_0000_0000` (the Sv57 equivalent) are all refused, under the paging schemes the pinned spec
ships today and with no Sv64 required.

⛔ **No value is lost, and the distinction is the whole finding.** The same 64-bit pattern *is* an exact
signed value — `-1073741824` reads cleanly and round-trips — so the address is representable. What the
domain refuses is the **unsigned spelling** a datasheet, a linker script or a device tree prints. The
decision therefore stands, but on an honest reason: not "nothing needs more", which is false, but
"everything that needs more is already reachable as a signed value, and the spelling is a compatible
widening". Three consequences landed with it rather than being left as prose:

- §1 rule 10 states the limit, and rows `0xFFFF_FFFF_C000_0000` (refused) and `-1073741824` (reads)
  execute it, so the limit is falsifiable instead of reassuring.
- `read-number-overflow`'s repair direction names the spelling that works. Measured, the two call
  sites disagreed with each other and with §4 — "split the quantity", "reduce the digits", and a third
  merged wording — and none of the three told this author anything usable. One code, one repair.
- The trigger below is concrete and checkable, instead of "if it is ever needed".

## What would make this decision wrong — the named triggers

1. ⭐ **The first target description that has to state a high-half address.** A kernel link address, a
   page-table base, a direct-map window: the moment a description must carry one, the unsigned spelling
   is needed and the widening stops being deferrable. This is the trigger to check against, and it is
   owned by `M2.8.2` (the device-tree fixture) and by any `board-first` target row, because that is
   where real addresses first enter a description.
2. **Sv64 being defined.** Not hypothetical: the pinned spec says "One additional scheme, Sv64, **will
   be defined in a later version of this specification**", and its satp `MODE` encoding is already
   reserved. A 64-bit virtual address space makes the high-half case ordinary rather than exceptional.
3. ⛔ **Any layer treating an integer value as a bit pattern.** Today `-1073741824` and the address
   `0xFFFF_FFFF_C000_0000` are the same 64-bit pattern, and a widening keeps the *number* identical
   while separating the two — which is exactly what makes the widening backward compatible at the level
   §15 cares about. But if any consumer ever starts reading an eADL integer as a bit pattern rather
   than as a number, a widening changes what that consumer holds and the change stops being
   compatible. §7 rule 5 of the reference keeps interpretation out of the frontend, so nothing does
   today; that is a reason to keep it that way, not a fact that survives carelessness. **This is the
   one trigger that would make the deferral expensive rather than merely late**, and it is recorded
   here because it is a constraint on other leaves rather than on this one.

## How to apply

- **Do not restate these figures — re-run the instrument.** Every count above is a record of this run,
  and the corpus moves. `cargo run -q -p eadl-front --example literals -- <roots>` is the producer; a
  figure copied out of this record into a chapter or a leaf is the defect correction A describes.
- **The domain is changed by changing the reference, not the reader.** §1 rules 9 and 10 are the
  statement and the rows are the enforcement; `crates/eadl-front/tests/reference.rs` executes every row
  and `crates/eadl-front/tests/conformance.rs` requires the grammar's recognizer to accept each
  `refused` row and reject each `error` one. After `M1.13.4` writes the baseline, changing any of it is
  a language change requiring a migration note (`M1.13.5`).
- **A description that needs an unsigned high-half address opens a widening against this record.** Do
  not widen locally, and do not work around it by writing the signed spelling silently: the author has
  to know they wrote an address as a negative number, which is why §1 rule 10 says so out loud.
- **Sequence `M1.26`'s gap (a) before any widening.** Widening forces `eadl-model` and `rt-analysis` to
  narrow explicitly at their own boundaries, which adds diagnostics to a layer whose codes gap (a)
  measures as stated normatively nowhere. Deciding the domain now is what keeps that ordering honest:
  the model layer's rules get written down before its population grows.
- `chipdoc` is read-only in both directions ([[decision_repository-boundary-read-only]]). Cite the pin
  and the digest as above; never restate an ISA figure from memory.

## Related

[[decision_zero-dependency-engine-core]] — why arbitrary precision is expensive here ·
[[reference_external-document-source-chipdoc]] — where the ISA measurements come from ·
[[a-moved-measurement-needs-a-census-of-its-copies]] — correction A's shape ·
[[enumerate-the-population-from-the-specification]] — why the instrument asks the frontend ·
[[a-verified-row-must-name-what-you-still-owe]] — what rule 10's honest limit leaves outstanding
