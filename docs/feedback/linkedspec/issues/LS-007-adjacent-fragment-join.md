# LS-007 — Adjacent fragments join into a single atom

| Field | Value |
| --- | --- |
| **ID** | `LS-007` |
| **State** | `no-action` |
| **Severity** | Informational |
| **Kind** | Correctness |
| **Component** | `specs/Lispish.spec` |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Reproducer** | [`repro/LS-002-003-lispish-probes.sh`](../repro/LS-002-003-lispish-probes.sh), probe 11 |
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

If [`LS-003`](LS-003-token-kind-erasure.md) is addressed by preserving delimiters or byte offsets,
this behaviour becomes observable to a consumer and stops mattering.

## History

- `2026-09-20` — measured on `ad290bdb4`; filed as `no-action` by archogen.
