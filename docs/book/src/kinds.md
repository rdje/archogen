# Kinds and schemas

A **kind** says what a declaration may contain. `defservice`, `defblock`, `defplatform`,
`defpolicy` and `defsystem` are kinds, and so is anything a user adds tomorrow.

## Exactly one primitive is trusted

`ROADMAP.md` §2 settles where the trust sits:

> Surface kinds can share the registry; **a small trusted semantic foundation remains
> explicit**. The registry cannot silently introduce new trusted axioms.

So archogen has exactly one declaration whose meaning is Rust code: `defkind` itself. Every
other kind — including all five surface kinds — is declared **in eADL**, in
`docs/semantics/kinds/core.eadl`:

```text
(defservice
  (doc "a required OS service and its externally observable contract")
  (name required)
  (clause requires (cardinality one-or-more) (holds forms))
  (clause offers (cardinality any) (holds forms)))
```

A test asserts that the five surface kinds come from that file and that `defkind` is *not* a
registry entry. If a future change needs a second trusted primitive, it has to be written in
Rust, where it is visible — which is what "cannot silently introduce new trusted axioms" means
once it stops being a sentence and starts being a build.

## A kind defines well-formedness, never behavior

§5.6 is explicit that `defkind` "must not become a host-code evaluator or an implementation
template language". A kind definition holds `doc`, `name` and `clause` — and nothing else.

The boundary classifier runs over kind definitions too, so there is no back door through the
facility that declares the language:

```text
(defkind deftimer (doc "a timer") (name required) (implementation (emit-template "t.rs")))
→ error[boundary-implementation-in-description]: `implementation` is implementation, and eADL
  contains no implementation
```

A kind must also carry a `doc`. A kind nobody can explain is a kind nobody should be adding.

## What the schema checks

The **declaration frame**: known kind, name present when required, known clauses, right number
of them, right value shapes.

```text
(defservice s (requires (x)) (requries (y)))
→ error[schema-unknown-clause]: `defservice` declarations have no `requries` clause
  = hint: did you mean `requires`? the clauses of `defservice` are `requires`, `offers`
```

The "did you mean" is edit-distance bounded on purpose. Suggesting `defsystem` for
`implementation` would send an author to rename rather than to reconsider.

Every problem in a kind definition is reported, not just the first — a malformed definition
usually has several things wrong with it.

## What it does not check, and the measured gap

A clause declared `(holds forms)` is **opaque**. `(at-least 60 s)` is nested forms at this
layer and becomes a checked quantity later.

That has a consequence worth naming rather than discovering: a forbidden construct nested
*inside* an opaque clause is invisible to the schema. Measured on the boundary corpus, the
schema refuses **10 of the 11** rejected cases; the eleventh, `execution-bound`, hides `wcet`
inside `(task …)` and is caught by the boundary classifier instead, which walks the whole tree.

The two mechanisms are independent and their reach is not identical. The test pins the split in
both directions, so when `(task …)` gets a real schema the count changes and someone has to say
so deliberately.

Two independent refusals for the same content is a feature, not redundancy: a mistake in one is
caught by the other. Claiming they were equivalent would have been the more dangerous kind of
green.
