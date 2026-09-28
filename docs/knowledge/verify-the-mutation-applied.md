---
slug: verify-the-mutation-applied
answers:
  - "I broke the code to prove my test catches it and the test still passed — what now?"
  - "How do I know a red arm actually exercised anything?"
  - "Why did my sed/python patch silently do nothing?"
  - "My mutation applied, its count assertion passed, and the arm still proves nothing — what did I miss?"
type: knowledge
date: 2026-09-13
---

# A mutation that did not apply is indistinguishable from a test that does not work

## The question

You seed a defect to prove a test catches it. The suite stays green. Two explanations fit, and
they call for opposite actions: the test is worthless, or the mutation never landed.

## The answer

**Assert the mutation applied, in the same command that applies it, before running anything.**

```python
old = 'let mut console = service::Console::new();'
assert s.count(old) == 1          # ← the line that would have caught it
open(p, 'w').write(s.replace(old, new))
```

Until that assertion exists, a green suite after a mutation is *no information at all*. With it,
green means exactly one thing: the test does not catch the defect.

## Why

This is not hypothetical — it happened here, twice in one session, and the first time it went
unnoticed for a full cycle. A red arm for the S0 emitter searched for

```text
    let mut console = service::Console::new();
```

with four leading spaces, while the source holds that line inside a continued string literal as

```text
         \x20   let mut console = service::Console::new();\n\
```

`str.replace` found nothing, wrote the file back unchanged, and the suite passed — which read
exactly like "the compile check does not catch a missing semicolon". Counting the anchor settled
it in one command: `grep -c` returned `1` for the real spelling and `0` for the one searched for.
With the mutation corrected, the same test failed with
`the generated crate did not compile: error: expected ';', found 'rt'`.

⭐ The trap is specific to *generated* and *escaped* code: the string you see rendered in the
output is not the string in the source. A template's literal is escaped, indented by the
continuation, and often split across lines, so a search written from the rendered form misses.

## A mutation that applied *partially* is the same failure in a greener disguise

Added `2026-09-28`, measured on leaf `M1.13.2`.

The count assertion above answers "did my needle match". It does not answer "is the property under
test now false". Those come apart the moment the subject holds **more than one instance** of the thing
the mutation removes.

`crates/eadl-front/tests/reference.rs`'s `arm_24` proves that a book chapter which stops citing the
normative reference is reported. It mutated one chosen sentence — the chapter's opening blockquote —
and pinned the leg at exactly one violation. That was sound while the chapter cited the reference
once. A leaf that added a **second** citation, in an unrelated paragraph about the value domain, left
the arm removing one of two: the chapter still cited the reference, the leg correctly reported
nothing, and the arm failed on its own count.

⭐ **The failure was loud, and that is the design working rather than luck** — `edited` asserts its
needle is present, so the arm could not pass vacuously. But notice what that assertion did *not*
catch: the needle was there, it matched, and the mutation still did not produce the state the arm's
name claims. A count assertion on the needle proves the needle exists; only an assertion on the
**post-mutation state** proves the property was destroyed. The arm now mutates the cited *path* rather
than one sentence containing it, replacing every occurrence, and asserts the result — that the chapter
it edits cites nothing.

⛔ **Generalise it: mutate the property, not an instance of it.** An arm whose mutation is scoped to
one occurrence weakens silently every time the population grows, and it weakens in the direction that
looks like nothing happened — the arm keeps passing, on a subject that has since acquired a second
route to satisfying the rule. Ask of every arm: *if the subject gained one more instance of the thing I
remove, would this arm still test what its name says?* If the answer is no, the needle is too
specific.

This is [[a-gate-is-only-as-sharp-as-its-fixtures]] one level up: that card is about a population that
cannot contain the case; this one is about a mutation that cannot remove all of it.

## How to apply

- Never write `sed -i` / `s.replace(...)` for a mutation without a count assertion. `sed` in
  particular reports success when it changed nothing.
- ⛔ **And never stop there: assert the post-mutation state, not only that the needle matched.** A
  partial mutation satisfies a count assertion and still leaves the property intact, so the arm proves
  nothing while looking exactly like one that does.
- Back the file up and **restore from the copy**, then prove the restore was exact —
  `git diff --stat <paths>` must be empty. A red arm that leaves the subject modified is worse
  than no red arm.
- Prefer mutating the **subject** over the test. Editing the assertion proves the assertion is
  load-bearing; editing the code proves the behavior is.
- Record what the failure actually said, not that it failed. `left: 1 / right: 25` is evidence
  someone can chase; "the test went red" is a claim ([[an-oracle-is-independent-by-construction]]
  is the same standard applied to expectations).
