# LS-006 — Hex literal underscores corrupted by atom joining — WITHDRAWN

| Field | Value |
| --- | --- |
| **ID** | `LS-006` |
| **State** | `withdrawn` |
| **Severity** | — (not a defect) |
| **Kind** | Correctness |
| **Component** | `specs/Lispish.spec` |
| **Affects** | — |
| **Reproducer** | [`repro.sh`](repro.sh), input in [`evidence/`](evidence/) |
| **Reported by** | archogen — **raised in error, corrected here** |

## What archogen expected

That Lispish's atom-fragment joining would corrupt eADL hex literals containing underscores.
archogen's own grammar admits them —
`hexadecimal = [ sign ] , "0x" , hex_digit , { hex_digit | "_" }` — and a separate defect in
archogen's own reader had previously split `(base 0x1000_0000)` into two forms, so the concern
was that Lispish would mask that class of error.

## What actually happens

It does not. Probe 02 returns the literal intact:

```json
["region","device.timer",["base","0x1000_0000"]]
```

The underscore survives. The concern was unfounded.

## Why this issue exists at all

It is kept, rather than deleted, so the tracker records what was checked and dismissed. A reader
who has the same worry can see it was measured, and nobody re-raises it. A tracker that silently
drops its own false positives gives no signal about the ones that remain.

## Reproduce

Everything needed is in this directory.

```sh
bash repro.sh --bin <path>/lispish_file --grammar <path>/Lispish.spec
```

| Item | Where |
| --- | --- |
| Inputs | [`evidence/`](evidence/) — one `.eadl` file per case |
| Frozen observation | [`evidence/EXPECTED.txt`](evidence/EXPECTED.txt) |
| Building the two arguments | [`../../SETUP.md`](../../SETUP.md) — once, for every issue |

**Exit code is the verdict:** `0` = the frozen observation still reproduces; `3` = behaviour
**changed**, which may mean this issue is fixed — the script prints the difference; `2` = could
not run.

## History

- `2026-09-20` — raised, measured, and withdrawn the same day; probe 02 retained as the record.
