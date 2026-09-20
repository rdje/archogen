# LS-003 — Quoted string, bare symbol and number are indistinguishable in the result

| Field | Value |
| --- | --- |
| **ID** | `LS-003` |
| **State** | `open` |
| **Severity** | Major for eADL |
| **Kind** | Correctness (documented behaviour; raised as a requirement) |
| **Component** | `specs/Lispish.spec`, the `SExpression` adapter |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Reproducer** | [`repro/LS-002-003-lispish-probes.sh`](../repro/LS-002-003-lispish-probes.sh), probes 03, 04, 05 |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

Two different inputs, one output:

| Probe | Input | Result |
| --- | --- | --- |
| 04 | `(name "ARCHOGEN")` | `["name","ARCHOGEN"]` |
| 05 | `(name ARCHOGEN)` | `["name","ARCHOGEN"]` |

Probe 03 shows the same erasure for numbers — `(task beat (period 10 ms))` becomes
`["task","beat",["period","10","ms"]]`, where `10` and `ms` are both plain strings.

## Why this is raised even though it is documented

The guide is explicit that this is by design: `SExpression` has only `Atom(String)` and
`List(...)`, and "Lispish's parent rules discard distinctions between symbols, quoted strings and
numeric tokens." It is raised because for eADL that distinction is **semantic, not cosmetic**: a
quoted string is a value; a bare symbol is an identifier that must resolve. A consumer cannot
recover the difference after the fact, because the delimiters are gone.

## Expected

The result distinguishes the token kinds the surface syntax distinguishes.

## Proposed fix

Retain token kind in the result — either a tagged variant (`Atom { text, kind }`) or, more
cheaply, preserve the delimiters so the consumer can reconstruct the distinction. **Byte offsets
per atom would serve both this and [`LS-002`](LS-002-multi-form-truncation.md)** and may be the
cheapest single change that addresses both.

## History

- `2026-09-20` — opened by archogen; reproduced on `ad290bdb4`.
