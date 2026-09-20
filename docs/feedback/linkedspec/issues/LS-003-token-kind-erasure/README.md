# LS-003 — Quoted string, bare symbol and number are indistinguishable in the result

| Field | Value |
| --- | --- |
| **ID** | `LS-003` |
| **State** | `open` |
| **Severity** | Major for eADL |
| **Kind** | Correctness (documented behaviour; raised as a requirement) |
| **Component** | `specs/Lispish.spec`, the `SExpression` adapter |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Reproducer** | [`repro.sh`](repro.sh), inputs in [`evidence/`](evidence/) |
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
per atom would serve both this and `LS-002`** and may be the
cheapest single change that addresses both.

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

- `2026-09-20` — opened by archogen; reproduced on `ad290bdb4`.
