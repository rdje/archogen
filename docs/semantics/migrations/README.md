# Migration notes — how a frozen construct of `eadl/1` changes

`docs/semantics/BASELINE.txt` digests every construct the language freezes: the EBNF fence of
[`../grammar.md`](../grammar.md), every machine-read table in [`../reference.md`](../reference.md), and
the canonical form of every description [`../conformance.md`](../conformance.md) declares.
`scripts/check_language_freeze.sh` recomputes it and refuses a difference no note here covers.

⭐ **The point is not to prevent change.** §15 of `ROADMAP.md` requires that "any changed behavior must
be explicit", and a language that cannot change is a language that cannot be fixed. The point is that the
change is *written down at the moment it happens*, by the person who knows why — rather than reconstructed
later from a digest difference by someone who does not.

## Two rules the gate enforces

1. **Any movement of the baseline requires a note, including a correction.** A note may say *"correction:
   the specification was wrong and no description changes meaning"* — that is still explicit, which is the
   whole requirement, and it costs one paragraph. A gate that tried to distinguish a bug fix from a
   language change would need judgement it cannot have, so it does not try.
2. **Amending the baseline is an explicit act, never a side effect.** Regenerating it without a note
   **fails** rather than passes: the gate compares the tracked baseline with `HEAD`'s as well as with a
   fresh run, so a regeneration nobody wrote down is a movement nobody covered. Without this, the first
   legitimate language change would be impossible and the gate would get waived.

## The form

One file per note, named `<version>-<slug>.md` — for example `eadl-2-widen-value-domain.md`. The header
lines are the machine-read part; the sections are what a reader gets.

```text
# <one-line title>

- version: <the version this change lands in>
- date: <YYYY-MM-DD>
- leaf: <the task-tree leaf that owns the change>
- status: pending | applied
- constructs: <construct id>, <construct id>   — or: all
- invalidates: <which descriptions, or `none`>

## What changed

## Why

## Which descriptions it invalidates

## Which version it lands in
```

- **`status: pending`** while the change is being made. The gate only accepts a **pending** note, so a
  note from an earlier migration cannot cover a later one — flip it to `applied` after regenerating the
  baseline, which is what makes the note a record rather than a permission that stays open.
- **`constructs:`** names what the note covers, using the baseline's own ids
  (`docs/semantics/grammar.md#ebnf`, `docs/semantics/reference.md#number-values`,
  `suite/examples/periodic-three/system.eadl`). Every construct the gate reports must be named, or the
  line must say `all` — and `all` is a claim that the note explains every movement, which the sections
  below it have to support.

## The workflow

```console
$ scripts/check_language_freeze.sh            # says what moved, and what would cover it
$ scripts/language_baseline.sh --print | diff docs/semantics/BASELINE.txt -   # the raw difference
$ $EDITOR docs/semantics/migrations/<version>-<slug>.md    # write the note, status: pending
$ scripts/check_language_freeze.sh            # green: the movement is covered
$ scripts/language_baseline.sh --emit         # amend the baseline — the explicit act
$ $EDITOR docs/semantics/migrations/<version>-<slug>.md    # status: applied
```

## Honest limits

⚠️ **A digest proves a construct *moved*.** It cannot prove that the note covering the movement is
correct, or complete, or that every description the change invalidates was found. That residue is review,
exactly as `BOOK-ANCHORS` states its own: the gate removes the cheapest failure — a frozen construct
edited silently — and leaves the expensive one to the reader.

⚠️ **Canonical form carries no comment**, so a corpus file's *header* is invisible to the baseline. The
headers that carry data are pinned elsewhere: by `crates/eadl-front/tests/corpus.rs` against
[`../boundary/README.md`](../boundary/README.md)'s counts, and by
`crates/eadl-front/tests/reference.rs` against the reference's `comment-headers` table.
