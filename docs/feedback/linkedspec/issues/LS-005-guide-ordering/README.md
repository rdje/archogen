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
([`LS-004`](../LS-004-bootstrap-false-success/README.md)) points away from the real cause.

## Proposed fix

End the "Add and pin" section with an explicit forward pointer — *"these commands do not produce
a buildable tree; continue to Initial PGEN preparation before building"* — or move the bootstrap
into that section.

Adding [`LS-001`](../LS-001-cargo-workspace-collision/README.md)'s `[workspace]` note in the same place
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
| Environment and pins | [`../../SETUP.md`](../../SETUP.md) |

**Exit code is the verdict:** `0` = reproduces as recorded; `3` = behaviour **changed**, which
may mean this issue is fixed; `2` = could not run.

## History

- `2026-09-20` — opened by archogen after following the guide in order.
