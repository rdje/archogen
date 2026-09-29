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

- **`status: pending`** in the commit that lands the movement, and **`applied`** from the next commit on.
  A pending note is a file here with a line that is **exactly** `- status: pending` — the value, and
  nothing after it. That is a rule and not a filename: the form above writes `pending | applied`, which is
  not a status, so this file is not a note (before `PROGRAM.27` it was, and through `— or: all` it covered
  every construct there is, which left the explicitness leg unable to fail).
- **A note `HEAD` already carries as pending is spent.** Its movement has landed, so it covers nothing,
  and the gate refuses it until it says `applied` — a pending note left in the tree is a permission that
  stays open, and would cover a later movement of the same constructs with nothing written down about it.
- **`constructs:`** names what the note covers, as a comma-separated list of the baseline's own ids
  (`docs/semantics/grammar.md#ebnf`, `docs/semantics/reference.md#number-values`,
  `suite/examples/periodic-three/system.eadl`), each compared **exactly** — a note naming `…#ebnf-v2` does
  not cover `…#ebnf`. Every construct the gate reports must be named, or the whole value must be `all` —
  a claim that the note explains every movement, which the sections below it have to support. `all`
  inside a longer value is not that claim.

## The workflow

A migration is **two commits**, and the gate enforces both.

```console
# commit 1 — the movement and its note
$ $EDITOR docs/semantics/migrations/<version>-<slug>.md    # the note, status: pending
$ $EDITOR <the construct>                                   # the change itself
$ scripts/check_language_freeze.sh            # RED: the integrity leg compares the tracked baseline with a
                                              #   fresh run and does not consult notes, so it stays red —
                                              #   and says what moved — until the baseline is amended
$ scripts/language_baseline.sh --print | diff docs/semantics/BASELINE.txt -   # the raw difference
$ scripts/language_baseline.sh --emit         # amend the baseline — the explicit act
$ scripts/check_language_freeze.sh            # green: the amendment is covered by the pending note
$ git commit …                                # note, construct and baseline together

# commit 2 — the note becomes a record
$ $EDITOR docs/semantics/migrations/<version>-<slug>.md    # status: applied
$ scripts/check_language_freeze.sh            # green; while the note still said pending it was refused
                                              #   as spent, because HEAD carries it that way
```

⛔ **The flip cannot share commit 1.** The explicitness leg compares the working tree's baseline with
`HEAD`'s, and an `applied` note covers nothing — so flipping in the amending commit refuses that commit.
And it cannot be skipped: from the moment commit 1 lands, `HEAD` carries the note as pending and the gate
refuses it until it is flipped. `eadl-1-quantity-value-type.md` is the first note to go through both
commits: it stayed pending for five commits before `PROGRAM.27`, which is the open permission the second
rule now closes.

## Honest limits

⚠️ **A digest proves a construct *moved*.** It cannot prove that the note covering the movement is
correct, or complete, or that every description the change invalidates was found. That residue is review,
exactly as `BOOK-ANCHORS` states its own: the gate removes the cheapest failure — a frozen construct
edited silently — and leaves the expensive one to the reader.

⚠️ **Canonical form carries no comment**, so a corpus file's *header* is invisible to the baseline. The
headers that carry data are pinned elsewhere: by `crates/eadl-front/tests/corpus.rs` against
[`../boundary/README.md`](../boundary/README.md)'s counts, and by
`crates/eadl-front/tests/reference.rs` against the reference's `comment-headers` table.
