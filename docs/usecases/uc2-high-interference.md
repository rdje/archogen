# `uc2-high-interference` — interference, and honest refusal

**Status:** supported; the timing property may legitimately return `not-established`.
**First gates:** M2 (analysis), M4 (execution).

The same shape as [`uc1`](uc1-periodic-three.md) with the interference turned up: a shorter
timer period, a second explicitly modeled interrupt source, and a low-priority task whose
deadline leaves little slack once switch and ISR costs are charged.

## What it exercises

1. **That overheads are actually charged.** The §7.4 idealized model ignores context switches,
   ISR costs and jitter. This case is constructed so that the idealized model says "fits" and
   the runtime-applicable model of `M2.6` may not. A toolchain that reports the idealized
   answer for a real build fails here — which is the same defect F29's controls hunt for at the
   ledger level.
2. **That refusal is available and used.** If the analysis cannot establish the deadline, the
   required answer is `not-established` (exit 15) — not a quiet pass, and not a claimed
   failure either. §7.4: "conservative analysis failure is `not-established` unless an exact
   test or validated counterexample establishes failure."
3. **That an unmodeled source is refused, not absorbed.** Add an interrupt source without
   declared arrival assumptions and the profile's `unmodeled-interrupt-load` exclusion must
   fire (F17), because "no source that can run during the analyzed interval is omitted just
   because it belongs to the kernel or instrumentation" (§7.3).

## The synthetic workload

| Task | Priority | Period T | Deadline D |
| --- | --- | --- | --- |
| `control` | 1 (highest) | 5 | 5 |
| `sense` | 2 | 10 | 8 |
| `log` | 3 (lowest) | 50 | 20 |

| Interrupt source | Arrival | Declared |
| --- | --- | --- |
| timer tick | every 5 ms | yes |
| external event | minimum separation 8 ms | yes |
| *(variant)* undeclared source | — | **no — must be refused** |

## The trap this case is built to catch

A system that "passes" because a cost category was omitted. §7.4.1 requires every physical
execution interval in a fixed trace to have exactly one primary ledger category, and F29
builds the detection fixture for it. `uc2` is the system-level companion: the same omission,
seen from the user's side, where the only visible symptom is a deadline claim that should not
have been made.
