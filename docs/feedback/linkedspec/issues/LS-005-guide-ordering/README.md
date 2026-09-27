# LS-005 — "Add and pin" commands alone never produce a buildable tree

| Field | Value |
| --- | --- |
| **ID** | `LS-005` |
| **State** | `verified` |
| **Severity** | Minor |
| **Kind** | Documentation |
| **Component** | `docs/linkedspec-book/src/public-api/integration-rust.md` |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Fixed in** | LinkedSpec `6e37288f7`, published in `fd3e328d5`; re-measured by archogen at `2ac834913` |
| **Reproducer** | [`repro.sh`](repro.sh) — the frozen original observation, mechanically |
| **Re-measurement** | [`remeasure.sh`](remeasure.sh) — the reported property, at any revision |
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
bash repro.sh     /path/to/linkedspec   # the frozen original observation
bash remeasure.sh /path/to/linkedspec   # is the reported defect gone at THIS revision?
```

| Item | Where |
| --- | --- |
| Reproducer | [`repro.sh`](repro.sh) |
| Recorded run | [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt) |
| Re-measurement instrument | [`remeasure.sh`](remeasure.sh) |
| Re-measurement at `2ac834913` | [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt) |
| Environment and pins | [`SETUP.md`](SETUP.md) — in this directory |

⚠️ **The two instruments have opposite exit-code meanings, on purpose.** `repro.sh` answers
"does the recorded observation still reproduce?", so `0` = the defect is **present**, `3` =
behaviour **changed**, `2` = could not run. `remeasure.sh` answers "is the reported defect
gone?", so `0` = **gone**, `1` = **still present**, `2` = could not run. Read the label on the
output, not just the number.

`remeasure.sh --self-test` runs four arms, including one built from the section **verbatim as
published at `ad290bdb4`**, and proves the verdict discriminates: the original text must come back
`1` and the remedied shape `0`. An instrument that has only ever been seen green has not been
shown to check anything.

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

⚠️ The last line above was true when the notice was recorded and is **superseded** by the
re-measurement below, which is archogen's own.

## Re-measurement at `2ac834913` — 2026-09-27 (archogen)

**Verdict: the reported defect is gone.** State moved `fixed-upstream` → `verified` on archogen's
own rerun, not on the notice. Revision measured: LinkedSpec
`2ac834913d85c32f532be9b0aab63644838a577a` — the consumer's adopted pin, which is documentation-only
ahead of the notice's `fd3e328d5`, so the guide under test carries the same remedy.

| Command | Exit | Meaning |
| --- | --- | --- |
| `bash remeasure.sh --self-test` | `0` | 4/4 arms, including the original section text verbatim |
| `bash remeasure.sh <checkout at 2ac834913>` | `0` | the defect is **gone** |
| `bash repro.sh <checkout at 2ac834913>` | `3` | behaviour **changed**; see the caveat below |

What was measured, as three properties of the guide's "Add and pin the source dependency" section
(lines 22–110 at this revision) rather than as heading spellings:

| Property | Result | Where |
| --- | --- | --- |
| **P1 order** — the section states that preparation comes *before* metadata/build | present | line 33: `**Before running Cargo metadata or building:** complete` |
| **P2 pointer** — the section names the preparation to complete | present | lines 36–37: `[RGX preparation](#initial-rgx-preparation). The checkout commands above obtain source; the preparation section is a required part of this setup sequence.` |
| **P3 target** — that section exists, so the pointer resolves | present | line 137: `### Initial RGX preparation` |

This is the first of the two remedies the report proposed, so the second (moving the bootstrap into
the section) is neither measured nor required.

⚠️ **Why the verdict does not rest on `repro.sh`.** At this revision `repro.sh` exits `3` with
`expected section headings not found — the guide has been restructured`, because it keys on
`### Initial PGEN preparation` and the heading was renamed to `### Initial RGX preparation`. Exit
`3` reports *change*; it cannot distinguish "reordered so the reader is warned" from "renamed and
still misleading". `remeasure.sh` exists to close that gap, and its `--self-test` proves it would
still report the original guide as defective.

**Not claimed here:** nothing in this re-measurement builds anything. The original observation's
symptom 1 — preparation living in a later top-level section — is still true and is no longer a
defect, because the reader is now sent there first. The workspace route is `LS-001`'s subject and
the preparation itself is `LS-004`'s.

Full output, including the commands and both instruments' verdicts:
[`evidence/REMEASURED.txt`](evidence/REMEASURED.txt). The original frozen observation,
[`evidence/OBSERVED.txt`](evidence/OBSERVED.txt), is preserved unchanged as historical evidence.

## History

- `2026-09-20` — opened by archogen after following the guide in order.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.

- `2026-09-27` — archogen: re-measured at `2ac834913` with a new property instrument (`remeasure.sh`, 4/4 self-test arms); exit `0`, the defect is gone. State `fixed-upstream` → **`verified`**. Evidence: `evidence/REMEASURED.txt`.
