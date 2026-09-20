# LS-005 — "Add and pin" commands alone never produce a buildable tree

| Field | Value |
| --- | --- |
| **ID** | `LS-005` |
| **State** | `open` |
| **Severity** | Minor |
| **Kind** | Documentation |
| **Component** | `docs/linkedspec-book/src/public-api/integration-rust.md` |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Reproducer** | [`repro.sh`](repro.sh) — checks the guide's section order mechanically |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

"Add and pin the source dependency" gives five commands and ends with `git add`. A reader
reasonably tries to build next. The build cannot work: PGEN's generated sources do not exist yet,
and the preparation lives two sections later, under "Keep preparation and build products local →
Initial PGEN preparation".

The dependency **is** stated — "Checkout does not generate PGEN's parser inputs" — but as a
trailing sentence inside a paragraph about what the commands retrieve, rather than as a blocking
step.

## Why it is worth fixing

It costs every first consumer the same wasted attempt, and the resulting failure
(`LS-004`) points away from the real cause.

## Proposed fix

End the "Add and pin" section with an explicit forward pointer — *"these commands do not produce
a buildable tree; continue to Initial PGEN preparation before building"* — or move the bootstrap
into that section.

Adding `LS-001`'s `[workspace]` note in the same place
would remove both first-run failures at once.

## Reproduce

Everything needed is in this directory. No build required.

```sh
bash repro.sh /path/to/linkedspec
```

| Item | Where |
| --- | --- |
| Reproducer | [`repro.sh`](repro.sh) |
| Recorded run | [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt) |
| Environment and pins | [`SETUP.md`](SETUP.md) — in this directory |

**Exit code is the verdict:** `0` = reproduces as recorded; `3` = behaviour **changed**, which
may mean this issue is fixed; `2` = could not run.

## State values

This issue's **State** field is one of:

| State | Meaning | Who sets it |
| --- | --- | --- |
| `open` | Reported with a reproducer; no upstream response yet | archogen |
| `acknowledged` | LinkedSpec has confirmed the behaviour | LinkedSpec |
| `by-design` | Confirmed intentional; archogen must adapt or route around it | LinkedSpec |
| `fixed-upstream` | Fixed in a named revision, not yet re-measured by archogen | LinkedSpec |
| `verified` | archogen re-ran the reproducer against that revision and it passes | archogen |
| `withdrawn` | archogen raised it in error; the correction is recorded above | archogen |
| `no-action` | Reproduces as described, but no change is requested | archogen |

To respond, edit the **State** field in the table at the top of this file and add a dated line
to the History below.

## History

- `2026-09-20` — opened by archogen after following the guide in order.
