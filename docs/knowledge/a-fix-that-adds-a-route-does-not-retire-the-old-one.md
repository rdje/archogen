---
slug: a-fix-that-adds-a-route-does-not-retire-the-old-one
answers:
  - "Upstream fixed my bug by adding a new mode — is my old reproducer now wrong?"
  - "The vendor says it is fixed but my reproducer still reproduces. Who is right?"
  - "How do I record a verdict when the remedy adds a route instead of changing behaviour?"
  - "Should I keep testing the old, still-truncating path after the fix?"
type: knowledge
date: 2026-09-27
---

# A fix that adds a route does not retire the old one

## The question

You reported a defect. The vendor's remedy is a **new** grammar, mode, flag or endpoint, and the old
path keeps behaving exactly as it did — deliberately. Your frozen reproducer still reproduces. Is the
defect fixed, and what do you write in the register?

## The answer

**Both are true, and the verdict has to name the route it measured.** "Verified" without a route is a
claim nobody can act on: the next consumer reads it, keeps using the old path, and gets the old
behaviour from a register that says it cannot happen.

Then **keep the old path under test** — but as a *guard*, with the opposite meaning. Its expected
result is now "unchanged", and a change in it is a regression against every consumer who has not
migrated, not a fix.

## Why

Measured on this project, `2026-09-27`. A recognizer returned only the first top-level form of a
multi-form document and exited `0`. The vendor's remedy was a second grammar with a `Document` entry
rule plus a second adapter; the original grammar is documented as an extraction example and keeps its
behaviour by design. On the same four-form input at the same revision:

| Route | Result | Meaning |
| --- | --- | --- |
| new document grammar | 4 of 4 top-level forms, `rc=0` | the reported defect is gone |
| historical extraction grammar | first form only, `rc=0` | unchanged — now its contract |

Eight frozen probes on the new route behaved exactly as archogen's pre-written expectations said,
including typed rejection of leading, intervening and trailing junk with no partial value. The
historical route re-run against its frozen observation matched **line for line**, exit `0` — and that
`0` means "nothing drifted", not "still broken".

⭐ **Adoption is a consumer action, and the obvious shortcut does not perform it.** Pointing the new
adapter at the old grammar fails with a typed diagnostic (`entry_rule_not_found` — the old grammar
has no `Document` rule); pointing the old adapter at the new grammar does not adopt the new result
contract either. Selecting the new route is the migration. A verdict recorded without saying that
leaves the next reader one silent failure away from believing they migrated.

⛔ **A scoring instrument must refuse rather than read a failure as a zero.** Counting top-level
forms by pattern-matching the result text would have counted nested lists as well and reported the
four-form document as having dozens; and a result that cannot be parsed is *not* the same verdict as
"zero forms". Delegating the count to a real parser, and exiting "could not decide" when that parser
is absent, is what keeps a green run meaningful.

## How to apply

- **Name the route in the state, the register row and the History line.** "Verified on the document
  route; the extraction route is unchanged by design" is a sentence a consumer can act on.
- **Run both, and report both.** The new route carries the verdict; the old route carries the guard.
  Print the guard's result next to the verdict so nobody has to wonder whether it was skipped.
- **Write the expectation before the run**, per probe, in the instrument. A count that is read back
  from the tool's own output is not a measurement.
- **Distinguish over-rejection from the defect you reported.** A document you expect to be accepted
  coming back rejected is a *different* defect; score it "could not decide" and say so, rather than
  letting it flip the verdict either way.
- **Let the guard's failure be visible.** Ours printed "binary required" and did nothing, because a
  relative path stopped resolving inside a subshell that changed directory. Resolve paths before
  handing them to another script, and read the guard's output instead of trusting its silence.
- Related: [[frozen-reproducers-measure-change-not-repair]] — why the verdict needs its own
  instrument, and [[a-verified-row-must-name-what-you-still-owe]] — the consumer's half of a remedy.
