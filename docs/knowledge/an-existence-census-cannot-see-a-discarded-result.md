---
slug: an-existence-census-cannot-see-a-discarded-result
answers:
  - "My gate proves every diagnostic code is stated and emitted — what could still be wrong?"
  - "A test asserts the function returns the right error. Why does the user never see it?"
  - "Two commands read the same input and disagree about it — which one is wrong?"
  - "I censused the emitters of a rule. What population did the census not include?"
  - "The help text says the command does X and a library does X — how do I know the command calls it?"
type: knowledge
date: 2026-09-29
---

# An existence census reads the emitter, and the discard is in the caller

## The question

A gate proves something about a population of rules: every diagnostic code the declared sources emit is
stated in the normative document, every code the document states is emitted by one, every code a chapter
renders is one a production source really emits. All of it green, in both directions, with the population
enumerated at run time. And a user types a description the toolchain cannot realize and is told
`accepted`.

## The answer

**Every one of those legs is an existence census, and existence is a property of the emitter.** The code
is in the source, the row is in the table, the rendering is in the chapter — all three true, and all three
silent about the question that decides whether the rule reaches anybody: *does the value the emitter
produces survive its callers?*

Ask it as a second census, over the **consumers** rather than the emitters:

```text
census: grep -rn "<the producing call>" crates/ --include='*.rs' | grep -v tests
        → for EACH consumer, does it propagate the result or discard it?
        → `.ok()`, `.ok()?`, `.map_err(|_| ())`, `let _ =`, `if let Ok(…)` are all discards
```

Then drive one input through the **command a user runs**, not through the function a test calls. The gap
between those two is where this class lives, and no amount of both-directions pinning closes it, because
both directions are still only about existence.

## Five measured instances in this repository

| The rule | What the green gates proved | What they could not see |
| --- | --- | --- |
| the eight `quantity-*` codes (`M1.28`) | `quantity.rs` emits all eight; `f03_units.rs` asserts each refusal; leg 8 confirms the book renders two of them | **three of four consumers discard the diagnostic** (`workload.rs:178` `.map_err(|_| ())`, `refinement.rs:177` `.ok()?`, `:188` `.ok()`), so `archogen check` prints `accepted against profile rt-static-up-v1` for `(period 10 parsec)` while `archogen build` refuses the same bytes |
| `module-too-large`'s row (finding **F-H**) | legs 4 and 5 pin §4's 60 codes against the sources that emit them, exactly and in both directions | the row states "more addressable parts than an instance identifier can hold" and `grep` finds no such limit in the elaborator — the call site fires on a source of 2^32 bytes. A census reads the `Diagnostic::error("…")` literal and never its predicate |
| `quantity-invalid` (finding **F-J**) | the code is in a production half, so the census counts it as emitted | its only site is the `other =>` arm of a `map_err` over `Quantity::new`, which returns two variants and both have their own arm — a **totality** arm no input reaches |
| `docs/book/src/quantities.md`'s transcripts | leg 8: every rendered code is one a production source emits | the transcripts are rendered over `platform.eadl`, which exists nowhere tracked, and the command the chapter's reader would run cannot produce them |
| the whole module elaborator (`M1.29`) | legs 4 and 5 pin all 24 `module-*` codes; `f01_f02_modules.rs` drives F01 and F02; `BOOK-ANCHORS` resolves every path `modules.md` cites | **the emitter has no production caller at all** — `git grep "elaborate("` outside the module finds two hits, both in that test — while `archogen help check` said "elaborate and type-check". The limiting case of a discard: not dropped by a caller, never called. `archogen check` answered the book's own opening example with `invalid-description` |

## Why

- **A diagnostic is a value, and values get dropped.** Rust makes dropping an error cheap and often
  correct — `refinement.rs` skips a bound it cannot read rather than guessing at it — so the discard is
  usually a *reasonable local decision* whose global consequence is that the rule stops being enforced.
  Nothing at the discard site looks like a defect.
- **The emitter is where a census can see.** Codes are string literals in one file; consumers are
  call sites spread over crates, some of them in a prototype. A gate that enumerates a population
  enumerates the population it is cheap to enumerate.
- **A type-level test is not a pipeline-level test.** `f03_units.rs` proves the refusal exists and is
  raised before arithmetic, which is the property §13.1's F03 names — and `ROADMAP.md`'s gate reads the
  *fixture*, which is a description driven through the toolchain. The test is one level below the claim.

## How to apply

- **Census the consumers, not only the emitters, and propagate or justify each discard.** A discard that
  is deliberate belongs in a comment that says what reports the condition instead — and that comment is a
  claim worth checking, because `workload.rs:176` named "the schema/unit pass" and there was no such
  pass.
- ⭐ **Drive one input through the user-facing command.** `archogen check <fixture>; echo $?` costs
  seconds and is the only instrument whose population is "what an author sees". A defect that survives
  every both-directions leg did not survive it.
- **When two commands read the same bytes, the disagreement is the finding.** `check` said accepted and
  `build` said tool-failure; neither was lying about itself, and the pair is what proved the pipeline
  drops something. Ask which one a user is told to trust, and make that one right.
- ⛔ **Do not fix it by propagating from the sites you found.** That adds a route and retires nothing:
  the clauses those two passes happen to read get reported and every other quantity stays silent, which
  is worse than a uniform silence because it looks fixed. See
  [[a-fix-that-adds-a-route-does-not-retire-the-old-one]].
- ⭐ **Couple a claim about a capability to the capability, in both directions.** `M1.29.1`'s leg reads
  the `check` summary and runs a module file through the command, and asserts
  `summary.contains("elaborat") == (the module file is not refused)`. It fails if the word comes back
  without the elaborator *and* if the elaborator lands without the word, so the sentence a user reads can
  neither precede nor outlive what it describes. A refusal that names the leaf removing it gets the same
  treatment against the task tree: it fails if that leaf closes while the refusal still names it.
- **"The tool cannot read this yet" is not a verdict.** Every §5.5 verdict is a statement about the
  *system*, so a function that can only return a verdict will answer a missing capability with a false one
  — `invalid-description` for a well-formed module. Decide the capability before calling it, in the layer
  that owns process statuses, and say `unimplemented`.
- **A normative document has to be written against the pipeline, not the module.** A row saying "refused"
  over a pass that discards the refusal is born false, and it is the kind of false that every existence
  gate keeps green.

Related: [[a-gate-is-only-as-sharp-as-its-fixtures]] — the same blindness approached from the fixture
side; [[presence-checks-cannot-see-an-extra-key]] — a green assertion that never looked at the half that
was wrong; [[enumerate-the-population-from-the-specification]] — how to choose the population a census
walks, which is the decision that left the consumers out; [[prose-beside-data-goes-unenforced]] — the
comment that named a pass which did not exist.
