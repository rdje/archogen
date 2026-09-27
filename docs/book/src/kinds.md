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

## What it does not check, and the gap that closed

A clause declared `(holds forms)` is **opaque**. `(at-least 60 s)` is nested forms at this
layer and becomes a checked quantity later.

That has a consequence worth naming rather than discovering: a forbidden construct nested
*inside* an opaque clause is invisible to the schema. This was not hypothetical. The boundary
corpus carried a case, `execution-bound`, that hides `wcet` inside `(task …)` — and while `task`
was declared `(holds forms)`, only the boundary classifier caught it, because it walks the whole
tree. The measured reach was **10 of 11** rejected cases.

`M1.7` gave `task` a real kind and wrote `(holds kind task)`, which makes the schema validate
each task as a full declaration and so see inside it. The reach is now **13 of 13**: every
rejected case is refused by the schema *and* by the classifier.

The two mechanisms remain independent, and that is the point rather than a redundancy: a mistake
in one is caught by the other. What changed is the *measurement*, not the argument — and equal
reach on one corpus is not equivalence. A construct hidden inside a clause that is still
`(holds forms)` would be invisible to the schema again, which is why the test asserts
`out_of_reach.is_empty()` over the corpus rather than asserting a number, and why moving a clause
back would fail the build.

⭐ The figures above are **gated**, not typed. `the_live_surfaces_publish_the_measured_reach`
measures the reach from the corpus and fails if this chapter publishes a different current
figure. It exists because this chapter published the stale one for 47 commits: the test that
measured the closure landed, the sibling chapter `workload.md` was updated, and this one was
not. A number copied into prose is copied out of the reach of the test that took it.
