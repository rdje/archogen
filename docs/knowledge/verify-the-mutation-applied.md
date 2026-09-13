---
slug: verify-the-mutation-applied
answers:
  - "I broke the code to prove my test catches it and the test still passed — what now?"
  - "How do I know a red arm actually exercised anything?"
  - "Why did my sed/python patch silently do nothing?"
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

## How to apply

- Never write `sed -i` / `s.replace(...)` for a mutation without a count assertion. `sed` in
  particular reports success when it changed nothing.
- Back the file up and **restore from the copy**, then prove the restore was exact —
  `git diff --stat <paths>` must be empty. A red arm that leaves the subject modified is worse
  than no red arm.
- Prefer mutating the **subject** over the test. Editing the assertion proves the assertion is
  load-bearing; editing the code proves the behavior is.
- Record what the failure actually said, not that it failed. `left: 1 / right: 25` is evidence
  someone can chase; "the test went red" is a claim ([[an-oracle-is-independent-by-construction]]
  is the same standard applied to expectations).
