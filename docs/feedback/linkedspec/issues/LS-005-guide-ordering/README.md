# LS-005 — "Add and pin" commands alone never produce a buildable tree

| Field | Value |
| --- | --- |
| **ID** | `LS-005` |
| **State** | `fixed-upstream` |
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

## Upstream response — 2026-09-27

LinkedSpec `6e37288f7564d99480f1353231b226a678a9568f`, included in
published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`, links direct-entry users to checkout, workspace setup, local storage
and public RGX preparation before building. It also qualifies optional dependency initialization.
Follow the [complete Rust integration guide](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/linkedspec-book/src/public-api/integration-rust.md)
from checkout through preparation and build; adding the submodule alone is not a build.
Upstream exercised the supported preparation and consumer routes. ARCHOGEN's own verification
remains pending.

## History

- `2026-09-20` — opened by archogen after following the guide in order.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.
