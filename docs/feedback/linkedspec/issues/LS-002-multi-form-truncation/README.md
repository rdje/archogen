# LS-002 — A multi-form file yields only its first form, and exits 0

| Field | Value |
| --- | --- |
| **ID** | `LS-002` |
| **State** | `open` |
| **Severity** | Blocker for eADL |
| **Kind** | Correctness (documented behaviour; raised as a requirement) |
| **Component** | `specs/Lispish.spec`, `examples/integration/rust/src/bin/lispish_file.rs` |
| **Affects** | LinkedSpec `ad290bdb4` |
| **Reproducer** | [`repro.sh`](repro.sh), inputs in [`evidence/`](evidence/) |
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
bash repro.sh --bin <path>/lispish_file --grammar <path>/Lispish.spec
```

| Item | Where |
| --- | --- |
| Inputs | [`evidence/`](evidence/) — one `.eadl` file per case |
| Frozen observation | [`evidence/EXPECTED.txt`](evidence/EXPECTED.txt) |
| Building the two arguments | [`../../SETUP.md`](../../SETUP.md) — once, for every issue |

**Exit code is the verdict:** `0` = the frozen observation still reproduces; `3` = behaviour
**changed**, which may mean this issue is fixed — the script prints the difference; `2` = could
not run.

## History

- `2026-09-20` — opened by archogen; reproduced on `ad290bdb4` against a real 4-form description.
