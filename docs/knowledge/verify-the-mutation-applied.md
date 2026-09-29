---
slug: verify-the-mutation-applied
answers:
  - "I broke the code to prove my test catches it and the test still passed — what now?"
  - "How do I know a red arm actually exercised anything?"
  - "Why did my sed/python patch silently do nothing?"
  - "My mutation applied, its count assertion passed, and the arm still proves nothing — what did I miss?"
  - "My RED arm reported a pass, but did the thing it tests actually run?"
  - "My before→after census shows no difference — did nothing change, or did I record too little?"
  - "My census sized a defect — how do I know it measured the defect and not something containing it?"
  - "The count I published came from a saved census file — was the file complete?"
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

## An oracle that accepts "not success" accepts "never ran"

Added `2026-09-29`, measured on leaf `PROGRAM.21`.

The same weakness appears on the other side of the arm — in its **pass condition**. Nine RED arms for a
doctrine check were written as *"expect a non-zero exit"*, and the first run scored **four passes on
`exit 127`**: the check under test could not be *found*, because the arms invoked it by a relative path
and then `cd`'d into a throwaway repository. Nothing was verified. The four arms that expected failure
"passed" on a command that never executed, and they passed *loudly enough to look right* — the harness
printed ✅ and a tally of `4 pass / 5 fail`, so the number that should have been `0 pass` read as
partial success.

⛔ **A refusal and a failure to exec are different claims.** `127` is the shell's, not the subject's. An
oracle written as `rc != 0` cannot distinguish "the gate correctly refused" from "the gate was not
found", "the gate crashed", "the fixture was empty", or "the interpreter is missing" — and every one of
those is a green tick on an arm that proved nothing. Require the subject's **own** verdict: the exact
exit code it documents, *and* its own identifying text in the output.

```sh
# ✗ passes on 127, on a crash, on an empty fixture — on anything that is not success
if [ "$rc" -ne 0 ]; then ok=$((ok + 1)); fi

# ✓ the subject's documented refusal code AND the subject's own voice
if [ "$rc" -eq 1 ] && printf '%s' "$out" | grep -q 'TASK-ACCEPTANCE'; then verdict=good; fi
```

⭐ **The general rule, and it is the same rule as the sections above.** An arm has three parts that can
each be weaker than the property: the **needle** (does it match?), the **mutation** (did it destroy the
property, or one instance of it?), and the **oracle** (does its pass condition exclude the ways the arm
can fail to run?). A count assertion covers the first, a post-mutation assertion the second, and only an
exact verdict covers the third. Check all three, or the arm is decoration that prints ✅.

⭐⭐ **And a before→after census has a fourth part: the field it records.** `M1.28.2` measured a language
change by driving all 76 tracked descriptions through `archogen check` at the parent commit and at the new
one, diffing the two — and the diff was **empty**, which read as "nothing changed". The instrument recorded
only the last line of output, which carries the *verdict*. The one description that had changed kept its
verdict (`invalid-description`) and changed only *which code* produced it — from `schema-unknown-clause`,
a clause-name typo, to `quantity-non-positive-frequency`, the zero clock frequency its header always
claimed. Re-run recording the verdict **and every code**, the same census reported `1 changed, 0
acceptances lost`, which is the figure the leaf publishes. ⛔ The failure mode is worse than a weak oracle
because it reports *success*: a measurement that under-records does not fail, it agrees with you.

⭐⭐ **And a census that *sizes* a defect has two more: the quantity it measures, and the file it saved.**
`M1.29.1` sized a renderer defect — a caret overrunning its line — at "23 of 78 tracked descriptions" and
published that figure on a new leaf. `M1.31`, fixing it, found the figure wrong in **both** directions.
The instrument measured the whole marker line, *label text included*, so a long label under a short line
counted as an overrun: run to completion it reported **38** files, and 38 again after the fix — the tell,
because a census that does not move when its defect is fixed was never measuring the defect. And the
saved list was exactly **2048 bytes**: it had been written by `… | tee census.txt | head -20`, `head`
exited after twenty lines, SIGPIPE stopped `tee`, and the count was taken from the alphabetical head of
the list — which cut off both of the largest overruns there were. Measured correctly, the figure is **10**.
Neither error announced itself; each produced a plausible number.

## How to apply

- Never write `sed -i` / `s.replace(...)` for a mutation without a count assertion. `sed` in
  particular reports success when it changed nothing.
- ⛔ **And never stop there: assert the post-mutation state, not only that the needle matched.** A
  partial mutation satisfies a count assertion and still leaves the property intact, so the arm proves
  nothing while looking exactly like one that does.
- ⛔ **And make the oracle exact: require the subject's own documented exit code AND its own
  identifying output — never merely `rc != 0`.** An arm that accepts any non-zero exit passes on `127`
  (command not found), on a crash, and on an empty fixture: three ways to print ✅ while verifying
  nothing, and the tally still reads like partial success.
- ⛔ **And a census you diff must record every field the change could move.** A verdict is a summary; if
  the change can move the *reason* and leave the summary alone, record the reason too. The empty diff is
  indistinguishable from a correct one unless you can say which fields were compared.
- ⛔ **A census that sizes a defect must be re-run after the fix, and must move to zero.** If it does not
  move, it measured something else. Measure the narrowest quantity that *is* the defect — the marker run,
  not the line it sits on.
- ⛔ **Never save a census through a pipe you also truncate.** `tee file | head` writes a file exactly as
  long as `head` let it be. Write the census to the file with nothing downstream, and read the file;
  a figure is taken from the complete population or it is not taken.
- Back the file up and **restore from the copy**, then prove the restore was exact —
  `git diff --stat <paths>` must be empty. A red arm that leaves the subject modified is worse
  than no red arm.
- Prefer mutating the **subject** over the test. Editing the assertion proves the assertion is
  load-bearing; editing the code proves the behavior is.
- Record what the failure actually said, not that it failed. `left: 1 / right: 25` is evidence
  someone can chase; "the test went red" is a claim ([[an-oracle-is-independent-by-construction]]
  is the same standard applied to expectations).
