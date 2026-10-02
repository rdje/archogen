# The fault paths' observation events: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-02`
- **Status:** `active` — drafted; round 1 next; the review closes on the first round that finds no defect
- **Owner / source:** leaf `M2.15` (`docs/tasks/M2.md`). The design under review is
  [`rt-static-up-v1-faults-observation.md`](../profiles/rt-static-up-v1-faults-observation.md), part of the profile's
  fault contract, with the routing it makes in the contract's *Still open* and the pointers in `M4.3` and `M4.9`.

## The draft

Step 1, `2026-10-02`: drafted from a quoted inventory a read-only context made of `ROADMAP.md` §6.3, the fault
contract, `M2.9`'s review findings (`docs/decisions/decision_runtime-contract-gaps.md`), `rt-core`'s and
`rt-reference`'s transitions, and the repository's traces — 161 entries, 21 ambiguities and 12 open questions. It
answers them by refusing what can be refused and delegating what another leaf owns: an arrival inside a region and a
discarded release produce no event; a start or a resume is stamped at the incoming context's first instruction, which
takes the target's decision-before-delivery out of the comparison; a delivery is compared task by task, the hosted
latch's coarser count forgiven within the target's; a fatal fault raised during a delivery is compared by its kind and
class, its attribution left to the port's fixtures; and a source that does not count arrivals makes its stretch
inconclusive.

## Why

The fault paths' traces are what `M4` compares between the hosted playground and the emulator; a rule stated after the
first mismatch would be fitted to it.
