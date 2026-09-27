---
slug: frozen-reproducers-measure-change-not-repair
answers:
  - "Upstream says the bug I reported is fixed — how do I verify that instead of accepting it?"
  - "My frozen reproducer exited 'behaviour changed' — does that mean the defect is gone?"
  - "What instrument do I need to move a report from fixed-upstream to verified?"
  - "A fix renamed the thing my reproducer greps for — how do I still get a verdict?"
type: knowledge
date: 2026-09-27
---

# A frozen reproducer measures *change*; verifying a fix needs a second instrument

## The question

You reported a defect with a reproducer whose exit code is its verdict, and froze the output.
The vendor publishes a fix and asks you to confirm. You re-run the reproducer at the new revision
and it says "behaviour changed". Is the defect gone?

## The answer

**You cannot tell, and the reproducer was never able to.** A frozen reproducer encodes the
*spellings* of the revision it was written against — heading names, error strings, file layout,
counts. Its "changed" verdict conflates a remedy with a rename, a reorder with a regression.

Write a **second instrument** that checks the *property* the report asked for, in terms that
survive the fix, and give it a RED arm built from the original observation. Keep both: the frozen
one is the historical record, the new one carries the verdict. Their exit-code polarity will
likely be opposite, so label both contracts where they are run.

## Why

Measured on this project, `2026-09-27`, re-measuring a documentation-ordering report at a vendor's
new pin. The frozen reproducer located a step by its heading text; the fix had renamed that heading
when ownership of the step moved to another component. Result:

```console
$ bash repro.sh <checkout at the new pin>
reproducer: expected section headings not found — the guide has been restructured.
$ echo $?
3        # "behaviour changed" — true, and useless as a verdict
```

The property instrument, checking three things the report actually asked for (an ordering statement
in the section, a pointer to the preparation step, and a preparation step that exists to point at),
returned `0 — the defect is gone`, with the exact line numbers that carry it.

⭐ **The trap worth naming.** The *unfixed* revision already contained a sentence mentioning the
missing step — and the original report had explicitly called that sentence insufficient, because it
was a trailing remark rather than a blocking instruction. An instrument written as "does the
section mention preparation?" would have reported the **unfixed** revision as fixed, and its RED
arm would have been the only thing standing between that bug and a false `verified` in the register.
Encode the distinction the report itself drew, not the keyword it happened to use.

## How to apply

- **Ask what the report demanded, not what the old output looked like.** Write the property as a
  small number of named conditions (here: order, pointer, target) and print each one's verdict, so
  a reader sees *which* condition carried the result.
- **Build the RED arm from the original artifact, verbatim.** Copy the section, the error text or
  the input from the revision under report — do not paraphrase it from memory. A property check
  that has only ever been seen green has not been shown to check anything.
- **Add arms for the failure modes of the instrument itself**: an unrecognisable shape must exit
  "could not run" rather than guess, and a condition that looks decorative must be shown to be
  load-bearing (a pointer to a target that does not exist is not a fix).
- **Never overwrite the frozen evidence.** The re-measurement goes beside it as its own dated
  file, assembled by running the instruments rather than transcribed by hand. A register whose
  `verified` rows cannot be traced to a preserved original is a register nobody can audit.
- **State what the re-measurement does not claim.** A documentation ordering check does not build
  anything; a build check does not prove the library correct. Naming the boundary is what keeps one
  green run from being read as acceptance of everything else.
- Related: [[a-gate-is-only-as-sharp-as-its-fixtures]] — the same argument about fixtures, and
  [[verify-the-mutation-applied]] — proving a check fires when the thing it checks is broken.
