# LS-003 — Quoted string, bare symbol and number are indistinguishable in the result

| Field | Value |
| --- | --- |
| **ID** | `LS-003` |
| **State** | `verified` |
| **Severity** | Major for eADL |
| **Kind** | Correctness (documented behaviour; raised as a requirement) |
| **Component** | `specs/Lispish.spec`, the `SExpression` adapter |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Fixed in** | new route: `specs/SExprDocumentV1.spec` + `sexpr_file` (`77d7b3db1`, `df845ce61`), published in `fd3e328d5`; re-measured by archogen at `2ac834913` |
| **Reproducer** | [`repro.sh`](repro.sh) — the historical extraction route, inputs in [`evidence/`](evidence/) |
| **Re-measurement** | [`remeasure.sh`](remeasure.sh) — the document route, same inputs |
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
# the document route — the one this row is verified on
bash remeasure.sh --sexpr-bin <bin>/sexpr_file --sexpr-grammar <checkout>/specs/SExprDocumentV1.spec \
                  --lispish-bin <bin>/lispish_file --lispish-grammar <checkout>/specs/Lispish.spec
bash remeasure.sh --self-test      # 9 scorer arms; no binaries needed

# the historical extraction route — the frozen original observation
bash repro.sh --bin <bin>/lispish_file --grammar <checkout>/specs/Lispish.spec
```

| Item | Where |
| --- | --- |
| Inputs | [`evidence/`](evidence/) — one `.eadl` file per case |
| Frozen observation (historical route) | [`evidence/EXPECTED.txt`](evidence/EXPECTED.txt) |
| Re-measurement instrument | [`remeasure.sh`](remeasure.sh) |
| Re-measurement at `2ac834913` | [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt) |
| Building the two arguments | [`SETUP.md`](SETUP.md) — in this directory |

**Exit codes are the verdict, and the two instruments differ.** `remeasure.sh`: `0` = the defect is
**gone** on the document route; `1` = **still present** — a kind was erased, a lexeme was converted
or unquoted, or the two probes still collapse to one result; `2` = could not run, or could not decide
— including an untagged or unreadable result, which means the route under test is not the tagged
document route and scoring it either way would be a lie. `repro.sh`: `0` = the frozen observation
still reproduces, i.e. the historical route still erases kinds **as documented**; `3` = behaviour
changed; `2` = could not run.

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

The separate document grammar and adapter retain tagged symbol,
string and number nodes with exact token lexemes, including string delimiters and escapes.
Grammar `77d7b3db1b65a2c83072447a6aec77456ca7aede` and adapter
`df845ce615df20929ac501b61984fbf9d29225ca` are included in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`.
Use `sexpr_file` with `SExprDocumentV1.spec` (public entry `Document`); the historical
`SExpression`/Lispish extraction result keeps its documented behavior. The tagged document
is a syntax representation, not eADL semantic validation. See the
[versioned document contract](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/linkedspec-book/src/specs-and-corpora/sexpr-document-v1.md).
ARCHOGEN's own adoption and verification remain pending.

⚠️ The last line above was true when the notice was recorded and is **superseded** by the
re-measurement below, which is archogen's own.

## Re-measurement at `2ac834913` — 2026-09-27 (archogen)

**Verdict: the reported defect is gone on the document route.** State moved `fixed-upstream` →
`verified` on archogen's own rerun. Measured with `sexpr_file` and `specs/SExprDocumentV1.spec` at
LinkedSpec `2ac834913`, RGX `f6e5acdc9`, PGEN `d9d41c28`, with the generated parser regenerated at
this pin.

Each probe's document is flattened to its atoms in order as `kind:lexeme` and compared with an
expectation written into the instrument before the run:

| Probe | Input | Expected atoms | Result |
| --- | --- | --- | --- |
| 04 | `(name "ARCHOGEN")` | `symbol:name`, `string:"ARCHOGEN"` | **exact match** |
| 05 | `(name ARCHOGEN)` | `symbol:name`, `symbol:ARCHOGEN` | **exact match** |
| 03 | `(task beat (period 10 ms) (deadline 10 ms))` | `symbol:task`, `symbol:beat`, `symbol:period`, `number:10`, `symbol:ms`, `symbol:deadline`, `number:10`, `symbol:ms` | **exact match** |

`remeasure.sh` → `rc=0`, `checks 4 · as expected 4 · defect 0 · undecided 0`; the fourth check is
the cross-probe one that **is** this defect: probe 04 and probe 05 must not produce the same atoms,
and they no longer do. `--self-test` → `9/9 arms passed`, including the historical erasure of a
quoted string (must be `1`), a numeric conversion such as `10` → `10.0` (must be `1`), and an
**untagged** result (must be refused as `2`, because scoring the wrong adapter's output either way
would be a lie).

⭐ **The report's exact complaint, both routes side by side.** Probe 04 and probe 05 still produce one
identical result on the historical route — `["name","ARCHOGEN"]` for both — and that is now its
documented contract. On the document route they differ, and the string keeps **both quotes** inside
its lexeme, so a consumer can reconstruct the distinction without re-lexing the source. The
number/unit case the report called semantic rather than cosmetic also separates: `10` is a `number`
and `ms` is a `symbol`, neither converted, no unit folded into a value.

**The regression guard, run as the notice asks.** `repro.sh` on the historical route → `rc=0`,
`observation matches evidence/EXPECTED.txt`: all three lines identical to the frozen observation.
Nothing about this verdict rests on that route having changed.

⚠️ **What this does not claim.** The kinds are the *document grammar's* lexical classification, not
eADL's. Its published rule makes `0x4_0000` a number and `1__0` and `1.` symbols; eADL's exactness
rules — no floating point anywhere, canonical number spelling — are archogen's own, stricter, and
enforced by archogen's reader. A consumer adopting this route inherits a lexical contract, not
eADL's, and the notice says the same: the tagged document is a syntax representation, not semantic
validation. One platform, one build profile, one pin.

Full output: [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt). The frozen original observation,
[`evidence/EXPECTED.txt`](evidence/EXPECTED.txt), is preserved unchanged.

## History

- `2026-09-20` — opened by archogen; reproduced on `ad290bdb4`.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.

- `2026-09-27` — archogen: re-measured at `2ac834913` on the document route (`sexpr_file` + `SExprDocumentV1.spec`, `remeasure.sh`, 9 scorer arms) — **4 of 4 checks as expected**: `string:"ARCHOGEN"` with both quotes, `symbol:ARCHOGEN`, `number:10` against `symbol:ms`, and probes 04 and 05 no longer collapse. The historical extraction route re-run as the guard and unchanged (`repro.sh` → `rc=0`, matches the frozen observation). State `fixed-upstream` → **`verified`**. Evidence: `evidence/REMEASURED.txt`.
