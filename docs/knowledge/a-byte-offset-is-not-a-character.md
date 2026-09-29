---
slug: a-byte-offset-is-not-a-character
answers:
  - "Where can my reader or renderer panic on text that is not ASCII?"
  - "My scanner advances one byte at a time — when is that safe?"
  - "Every test passes and the reader still crashes on a user's file — what did the tests never contain?"
type: knowledge
date: 2026-09-29
---

# A byte offset is not a character: every advance and every slice over user text lands on a boundary

## The question

The reader and the diagnostic renderer work in byte offsets, because spans are byte ranges. Where does
that go wrong?

## The answer

**Wherever a step of one byte, or a byte turned into a `char`, can meet a character written in more than
one byte.** Rust's `&str` slicing panics off a character boundary, and it panics at the *next* slice, far
from the step that went astray. That is why the crash message points at an innocent line.

It happened twice in one day here, in two different places:

- `M1.31`: a renderer found a span's last line with `position(end - 1)`. When the span ended on `é`,
  `end - 1` was inside `é`, and twelve reference legs panicked at `source.rs:119`.
- `M1.35`: the string reader read an unknown escape as one byte (`other as char`) and advanced one byte.
  After `"a\éb"` it was left inside `é`, the diagnostic's span ended mid-character, and the next slice
  panicked. `archogen check` exited 101 where the contract is a diagnostic and exit 10.

Neither was caught by a test, because the tests were written in ASCII and the corpus is ASCII.

## How to apply

- **Census the shape, not the symptom.** `grep -n "as char\|+= 1" reader.rs`, and for each hit ask whether
  the byte there can be `≥ 0x80`. A one-byte step is safe only after an ASCII byte has been confirmed.
- **Read a character whole where one can occur**: `self.raw[at..].chars().next()` and
  `at += ch.len_utf8()`, from a position already known to be a boundary.
- **Put multi-byte text in the fixtures**, at the ends of spans, after escapes, and in every position a
  byte-wise step exists: `é` (two bytes) and `🙂` (four), since a width-two fix can still be wrong at four.
- **Make a generator reach it.** `PROGRAM.9.2`'s fuzz step carries a known-false arm, "no input contains a
  multi-byte character", which it must refute, so its reader properties cannot pass on ASCII alone.

Related: [[a-gate-is-only-as-sharp-as-its-fixtures]] — the same blind spot, seen from the fixture side.
