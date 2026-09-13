# What a report may claim

archogen reports do not say "verified". They cannot: there is no such word in the vocabulary,
and the absence is enforced by the code that builds them.

`ROADMAP.md` §7.1 opens with a prohibition rather than a feature:

> The report contains separate statuses for configuration validity, runtime functional
> behavior, timing, memory bounds, startup behavior, and any future isolation property.
> **One global "verified" flag is prohibited.**

A prohibition written in a document is one somebody violates under deadline. This one is
encoded three ways (`crates/archogen-evidence/`).

## 1. There is no aggregate verdict

A report is a set of per-property claims. It offers no method that collapses them, so you
cannot ask whether "it" passed — the question has no referent.

## 2. Silence about a property is not a pass

A report refuses to render while any property lacks a claim:

```text
report is incomplete: no claim for configuration-validity, runtime-functional-behavior,
memory-bounds, startup-behavior — a property with no claim is not a pass
```

The only way to say nothing about a property is `not applicable`, which must still say *why*.

## 3. Every positive conclusion carries its qualifier

There is no `Verified`, no `Passed`, no bare `Holds`. Each conclusion names what makes it
true, and cannot be constructed without it:

| Evidence category | What it may conclude |
| --- | --- |
| structural check | "the checked configuration satisfies **these named constraints**" |
| conditional analysis | "holds in **model M** under **these listed assumptions**" — an empty list is refused |
| tested conformance | "no violation was observed within **this recorded coverage**" |
| model proof | "**invariant I** holds for the stated formal **model M**" |
| implementation refinement | "transfers through **refinement R** and **its assumptions**" — an empty list is refused |
| target evidence | "applies only to **target T**, **binary B**" |

A conditional analysis with an empty assumption list renders as "holds", full stop — the same
overstatement wearing a different hat. It is refused:

```text
claim for `timing` is malformed: a conditional analysis with no listed assumptions is an
unconditional claim
```

Three sentences from §7.1 shape the whole table:

> Testing does not become proof through repetition. A published algorithm proof does not
> automatically verify its Rust implementation. A declared capability does not constitute
> hardware evidence.

So `tested conformance` has no route to a stronger conclusion, no matter how many runs.

## Bounds remember where they came from

Every numerical bound records its units, scope, target, **binary identity**, and origin.

| Origin | Established bound? |
| --- | --- |
| assumed | no |
| observed maximum | no |
| externally supplied | yes, within the source's own scope |
| analytically established | yes |

§7.3: *"An observed maximum with a safety multiplier remains an empirical assumption unless a
valid argument establishes a bound."* So the safety factor lives **inside** the observation,
and cannot promote it — a factor of 1000 leaves it an observation. Multiplying a measurement
by 1.5 and calling the result a bound is the most common way a timing claim becomes untrue,
and it is untrue in a way that looks like diligence.

Safety factors are exact rationals (`3/2`), not floats: §7.4 requires exact integer or checked
rational arithmetic, and a binary float would introduce a rounding question in the one place
nobody would look for one.

A bound with no binary identity is refused outright:

```text
`binary`: a bound not tied to a binary is invalidated by any rebuild and nobody can tell
```

That is §15's point — a compiler flag change can invalidate a timing bound while the eADL
description is byte-identical.

## Trust dependencies

The generator and the independent checker are meant to reach their verdicts separately. §4.4's
position is not that sharing is forbidden — it is that sharing must be **visible**.

Each inventoried item records its identity, version, **content hash**, role, and which roots
reach it. The hash is what catches a change behind an unchanged name and version. The role is
what keeps the gate meaningful:

| Role | Shared with the generator costs independence? |
| --- | --- |
| infrastructure | no — a shared allocator cannot make two implementations agree on a wrong answer |
| interpretation / normalization | yes — it shapes what both sides *see* |
| semantic analysis | yes — it shapes what both sides *conclude* |
| authoritative data | yes |
| reference derivation | yes |

An unrelated change produces **no** warning. §14.4 requires that too, and a gate that cries
wolf is a gate that gets disabled.

⚠️ The honest limit, from §4.4 itself: *"This check enforces disclosure and change control; it
does not prove semantic independence."* Two separately written implementations of the same
misread specification share nothing any inventory can see.
