# LS-007 — Adjacent fragments join into a single atom

| Field | Value |
| --- | --- |
| **ID** | `LS-007` |
| **State** | `no-action` |
| **Severity** | Informational |
| **Kind** | Correctness |
| **Component** | `specs/Lispish.spec` |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Reproducer** | [`repro.sh`](repro.sh), input in [`evidence/`](evidence/) |
| **Reported by** | archogen |

## Summary

Adjacent quoted, bracketed and braced fragments join into one atom, exactly as the upstream guide
documents. Probe 11:

| Input | Result |
| --- | --- |
| `(a" b"[c]{d})` | `["a b[c]d"]` |

Nonempty square-bracket content keeps its brackets; brace content loses its outer braces; the
pieces concatenate.

## Why no action is requested

It reproduces precisely as documented, and **eADL uses neither square brackets nor braces in its
surface syntax**, so the impact on this consumer is negligible. archogen flagged it as a risk on
first reading and is recording the measured outcome rather than carrying an ask it does not have.

Another consumer whose surface syntax *does* use those delimiters would likely disagree, which is
why the issue is filed as `no-action` rather than closed.

## Relationship to other issues

If `LS-003` is addressed by preserving delimiters or byte offsets,
this behaviour becomes observable to a consumer and stops mattering.

## Reproduce

Everything needed is in this directory.

```sh
bash repro.sh --bin <path>/lispish_file --grammar <path>/Lispish.spec
```

| Item | Where |
| --- | --- |
| Inputs | [`evidence/`](evidence/) — one `.eadl` file per case |
| Frozen observation | [`evidence/EXPECTED.txt`](evidence/EXPECTED.txt) |
| Building the two arguments | [`SETUP.md`](SETUP.md) — in this directory |

**Exit code is the verdict:** `0` = the frozen observation still reproduces; `3` = behaviour
**changed**, which may mean this issue is fixed — the script prints the difference; `2` = could
not run.

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

- `2026-09-20` — measured on `ad290bdb4`; filed as `no-action` by archogen.
