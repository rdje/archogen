# LS-002 — A multi-form file yields only its first form, and exits 0

| Field | Value |
| --- | --- |
| **ID** | `LS-002` |
| **State** | `verified` |
| **Severity** | Blocker for eADL |
| **Kind** | Correctness (documented behaviour; raised as a requirement) |
| **Component** | `specs/Lispish.spec`, `examples/integration/rust/src/bin/lispish_file.rs` |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Fixed in** | new route: `specs/SExprDocumentV1.spec` + `sexpr_file` (`77d7b3db1`, `df845ce61`), published in `fd3e328d5`; re-measured by archogen at `2ac834913` |
| **Reproducer** | [`repro.sh`](repro.sh) — the historical extraction route, inputs in [`evidence/`](evidence/) |
| **Re-measurement** | [`remeasure.sh`](remeasure.sh) — the document route, same inputs |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

Run the shipped `lispish_file` on a real, valid archogen description —
[`evidence/system.eadl`](evidence/system.eadl), which contains four top-level forms
(`defblock`, `defplatform`, `defservice`, `defsystem`):

```json
["defblock","console.uart",["offers",["observable-output","true"]]]
```

Exit status `0`. The platform, the service, and the entire system — two tasks with periods,
deadlines, priorities and overrun policies — are **silently discarded**. Three of four
declarations vanish and the process reports success.

## Why this is raised even though it is documented

The upstream guide documents it (`(a)(b)` → `["a"]`, "only the first form is returned"), and is
commendably clear that "a successful value does not establish that the complete file is valid".
It is raised because the same guide recommends this path for ARCHOGEN by name, and eADL's
normative grammar defines a document as *all* of its forms plus explicit end-of-input:

```ebnf
document = { trivia } , { form , { trivia } } , end ;
```

A reader that returns one form and never requires `end` cannot implement that production. The
failure mode is the dangerous one: not an error, but a **success carrying a truncated result**.

## Observed

From `evidence/EXPECTED.txt`:

| Probe | Input shape | Result | Exit |
| --- | --- | --- | --- |
| 01 | two top-level forms on separate lines | first form only | `0` |
| 12 | two top-level forms on one line | first form only | `0` |
| 07 | valid form then `this is not eADL at all &&& (((` | first form only | `0` |
| 09 | `(defblock console.uart))` — unbalanced close | form returned, stray `)` ignored | `0` |
| 06 | `(defsystem heartbeat` — unterminated | error | `1` |

Probe 06 is noted in LinkedSpec's favour: an unterminated form **is** rejected. The gap is
specifically *trailing* and *additional* input, not malformed input everywhere.

## Expected

A way to require that the grammar consumed the whole input, and a way to obtain every top-level
form.

## Proposed fix — two independent asks, in priority order

1. **A complete-input mode.** An option (grammar-level or on `ExecutionOptions`) that makes an
   unconsumed tail an error rather than a silent success. This is the single change that would
   make the Rust backend usable for document validation, and **the most valuable item in this
   tracker**.
2. **A multi-form result.** A way to obtain *every* top-level form, not just the first.

Either alone is a large improvement; (1) matters more, because it converts a silent wrong answer
into a diagnosable failure.

**A zero-cost mitigation** while that work is pending: have `lispish_file` report the consumed
byte offset alongside the value, or warn on stderr when input remains. The adapter comment
already says it "cannot validate text the grammar already skipped"; surfacing *how much* was
skipped costs little and would have made this self-evident on the first run.

## Related upstream work

The guide states that applications requiring strict document validation or multiple top-level
forms "need an explicit grammar/contract for those requirements", tracked as
`SESSION-STARTUP-READING.83.1-.83.3`, and not implemented by the example. This issue is evidence
for prioritising it, with a named consumer blocked on it.

## Reproduce

Everything needed is in this directory.

```sh
# the document route — the one this row is verified on
bash remeasure.sh --sexpr-bin <bin>/sexpr_file --sexpr-grammar <checkout>/specs/SExprDocumentV1.spec \
                  --lispish-bin <bin>/lispish_file --lispish-grammar <checkout>/specs/Lispish.spec
bash remeasure.sh --self-test      # 9 evaluator arms; no binaries needed

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
**gone** on the document route; `1` = **still present** — a form was dropped on a success exit, or
unusable input was accepted; `2` = could not run, or could not decide (including a document archogen
expects to be accepted being *rejected*, which is a different defect and is not scored as either).
`repro.sh`: `0` = the frozen observation still reproduces, i.e. the historical route still
truncates **as documented**; `3` = behaviour changed; `2` = could not run.

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

Both requested capabilities are available through the separate
`SExprDocumentV1.spec` grammar and native `sexpr_file` adapter: every top-level list is returned
and invalid leading, intervening or trailing input rejects the whole document. Grammar commit
`77d7b3db1b65a2c83072447a6aec77456ca7aede` and adapter commit
`df845ce615df20929ac501b61984fbf9d29225ca` are included in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`.
That delivery also closes the wrapper-skipped prefix gap, with 51 cases on six runtime routes
and native file delivery. Select `sexpr_file --grammar SExprDocumentV1.spec`; retain the
historical `lispish_file`/`Lispish.spec` reproduction as an extraction regression guard.
Changing only the old adapter's grammar argument does not adopt the new result contract.
See the [document contract](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/linkedspec-book/src/specs-and-corpora/sexpr-document-v1.md)
and [Rust delivery guide](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/linkedspec-book/src/public-api/integration-rust.md).
ARCHOGEN's own adoption and verification remain pending.

⚠️ The last line above was true when the notice was recorded and is **superseded** by the
re-measurement below, which is archogen's own.

## Re-measurement at `2ac834913` — 2026-09-27 (archogen)

**Verdict: the reported defect is gone on the document route**, and both of this report's asks are
met. State moved `fixed-upstream` → `verified` on archogen's own rerun. Measured with the native
document consumer `sexpr_file` and `specs/SExprDocumentV1.spec` (entry rule `Document`) at LinkedSpec
`2ac834913`, RGX `f6e5acdc9`, PGEN `d9d41c28`, with the generated parser **regenerated at this pin**
rather than the one the checkout arrived carrying.

All eight frozen probes, each against an expectation archogen wrote down before running:

| Probe | Expectation | Result |
| --- | --- | --- |
| `01-multiple-top-level-forms` | 2 forms | **2 of 2**, `rc=0` |
| `12-two-forms-one-line` | 2 forms | **2 of 2**, `rc=0` |
| `system.eadl` — the real four-form description | 4 forms | **4 of 4**, `rc=0` |
| `08-comment-no-newline` | 1 form | 1 of 1, `rc=0` |
| `10-semantically-invalid` | 1 form | 1 of 1, `rc=0` |
| `07-trailing-garbage` | reject | **rejected**, `rc=1`, no partial value |
| `09-unbalanced-close` | reject | **rejected**, `rc=1`, no partial value |
| `06-unterminated-form` | reject | rejected, `rc=1`, no partial value |

`remeasure.sh` → `rc=0`, `probes 8 · as expected 8 · defect 0 · undecided 0`. `--self-test` →
`9/9 arms passed`, including the historical truncation (one form where two are expected), which must
come back `1`, and an unreadable result, which must be refused rather than counted as zero forms.

**The decisive probe.** `system.eadl` is the real four-form description that decided the original
report. It now returns all four top-level forms, headed `defblock`, `defplatform`, `defservice`,
`defsystem`, where the historical route returned the first and silently discarded three on a success
exit.

⭐ **Both asks were met — by a new route, not by changing the old one.** Ask (1), a complete-input
mode: trailing, intervening and leading junk now reject the whole document. Ask (2), a multi-form
result: every top-level form, in order. Neither was delivered by modifying `Lispish.spec`, which
keeps its extraction behaviour by design. So a consumer must **select** the document grammar;
changing only the old adapter's grammar argument does not adopt the new contract — the old adapter
has no `Document` entry and fails with the typed diagnostic `entry_rule_not_found`.

**The regression guard, run as the notice asks.** `repro.sh` on the historical route at this pin →
`rc=0`, `observation matches evidence/EXPECTED.txt`: the extraction route still returns one form per
file and still ignores trailing junk. That is now its documented contract rather than a defect, and
this row does not claim otherwise.

⚠️ **What this does not settle.** It measures syntax — how many forms there are, and whether unusable
input is refused. `10-semantically-invalid` is accepted here and must be: a document grammar is not a
semantic checker, and the consumer's own checking owns meaning. Nor does it establish that
`SExprDocumentV1.spec` can serve as an **independent recognizer** of archogen's normative surface
syntax. archogen's earlier evaluation closed that question on the grounds that no complete-input mode
existed; this result reopens it rather than answering it, and it is tracked separately by the
consumer. One platform, one build profile, one pin.

Full output: [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt). The frozen original observation,
[`evidence/EXPECTED.txt`](evidence/EXPECTED.txt), is preserved unchanged.

## History

- `2026-09-20` — opened by archogen; reproduced on `ad290bdb4` against a real 4-form description.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.

- `2026-09-27` — archogen: re-measured at `2ac834913` on the document route (`sexpr_file` + `SExprDocumentV1.spec`, `remeasure.sh`, 9 evaluator arms) — **8 of 8 probes as expected**, including all four top-level forms of the real description and typed rejection of trailing, unbalanced and unterminated input. The historical extraction route re-run as the guard and unchanged (`repro.sh` → `rc=0`, matches the frozen observation). State `fixed-upstream` → **`verified`**. Evidence: `evidence/REMEASURED.txt`.
