# Feedback to LinkedSpec — the Rust backend and the shipped Lispish grammar

- **From:** the archogen project, the first consumer of the LinkedSpec Rust backend.
- **About:** `docs/linkedspec-book/src/public-api/integration-rust.md` and `specs/Lispish.spec`.
- **Measured against:** LinkedSpec `ad290bdb427bc19a5af81de0f0b07e119c8999ff`, RGX
  `8763a0e6bea97879f027237439d57725f83ead23`, PGEN `db6f8c6836fefa5a57b1337d3ffbf6f15774089f`.
- **Toolchain:** `rustc 1.95.0 (59807616e 2026-04-14)`, `cargo 1.95.0`, Darwin arm64.
- **Status:** every claim below was executed, not read off the guide. Reproducers are included
  and are runnable **without archogen**.

The integration guide names ARCHOGEN in its own example (`settings.sexp` contains
`(name "ARCHOGEN")`), so this is offered in the spirit the guide invites: the first real
consumer reporting what the first integration actually did.

## Summary

| # | Finding | Kind | Severity | Reproducer |
| --- | --- | --- | --- | --- |
| A | The documented vendoring layout does not build inside a Cargo workspace | Build | Blocker | `repro/repro-workspace-collision.sh` |
| B | A multi-form file silently yields only its first form, exit `0` | Correctness | Blocker for eADL | `run-probes.sh` probes 01, 07, 09, 12 |
| C | A quoted string and a bare symbol are indistinguishable in the result | Correctness | Major for eADL | `run-probes.sh` probes 04, 05 |
| D | The PGEN bootstrap continues past a failed `cargo` call and reports a false seed | Robustness | Moderate | Part 1 of the finding-A reproducer |
| E | Guide ordering: the "Add and pin" commands alone never produce a working build | Docs | Minor | — |

Findings B and C are **not bugs against the shipped grammar's stated contract** — the guide
documents both. They are reported because the guide recommends this path to ARCHOGEN
specifically, and for ARCHOGEN's file format they are disqualifying. See "What this means for
the first consumer".

## A — The documented vendoring layout does not build inside a Cargo workspace

**Blocker.** The guide's setup is `git submodule add … vendor/linkedspec` inside an application
repository. When that application is a Cargo **workspace** — which archogen is, and which is the
ordinary shape of a multi-crate Rust application — Cargo's upward workspace auto-discovery walks
from each vendored manifest to the consumer's workspace root and binds it there. Every vendored
package that does not declare its own `[workspace]` then fails to build:

```text
error: current package believes it's in a workspace when it's not:
current:   <app>/vendor/linkedspec/examples/integration/rust/Cargo.toml
workspace: <app>/Cargo.toml
```

This is not confined to the example. It also stops **the PGEN bootstrap**, which the guide
requires before any build can succeed:

```text
current:   <app>/vendor/linkedspec/rgx/subs/pgen/rust/Cargo.toml
workspace: <app>/Cargo.toml
```

**Scope measured in this checkout:** 17 vendored `Cargo.toml` files, of which **3** declare
`[workspace]` — `rust/Cargo.toml`, `rgx/Cargo.toml`, `rgx/fuzz/Cargo.toml`. The consequence is
worth stating precisely, because it is good news and bad news:

- `rust/Cargo.toml` **is** a workspace root, so `linkedspec-runtime` — the crate a consumer adds
  as a path dependency — is insulated. The library route is sound.
- The packages a consumer is told to *build and copy from* — `examples/integration/rust` and
  `rgx/subs/pgen/rust` — are not. So the documented onboarding path fails at step one while the
  underlying product is fine.

**Proposed fix.** Add an empty `[workspace]` table to every manifest a consumer is expected to
build outside LinkedSpec's own workspaces — at minimum `examples/integration/rust/Cargo.toml`
and `rgx/subs/pgen/rust/Cargo.toml`. This is inert inside LinkedSpec's own CI and makes the
packages self-contained wherever they are vendored. Documenting `exclude = ["vendor"]` in the
consumer's workspace root is a reasonable *addition*, but it should not be the only remedy: it
requires every consumer to discover the failure first.

**Verification.** `repro/repro-workspace-collision.sh /path/to/linkedspec` builds the documented
layout from any LinkedSpec checkout, asserts both manifests fail with that exact error, applies
the proposed fix, and asserts both then pass. It exits `0` only if both halves behave as
described. Observed here:

```text
== Part 1: the documented layout, as shipped ==
  REPRODUCED: examples/integration/rust/Cargo.toml -> 'believes it's in a workspace when it's not'
  REPRODUCED: rgx/subs/pgen/rust/Cargo.toml -> 'believes it's in a workspace when it's not'
== Part 2: the proposed fix — an empty [workspace] table in each manifest ==
  FIXED: examples/integration/rust/Cargo.toml
  FIXED: rgx/subs/pgen/rust/Cargo.toml

RESULT: reproduced (2/2) and fixed by the proposal (2/2).
```

After applying the same fix locally, the PGEN bootstrap and the `lispish_file` build both
succeed, and every observation in findings B and C was taken from that working build.

## B — A multi-form file yields only its first form, and exits 0

**Blocker for eADL.** Run the shipped `lispish_file` on a real, valid archogen description —
`examples/s0-heartbeat/system.eadl`, which contains four top-level forms (`defblock`,
`defplatform`, `defservice`, `defsystem`):

```json
["defblock","console.uart",["offers",["observable-output","true"]]]
```

Exit status `0`. The platform, the service, and the entire system — two tasks with periods,
deadlines, priorities and overrun policies — are **silently discarded**. Three of four
declarations vanish and the process reports success.

The guide does document this (`(a)(b)` → `["a"]`, "only the first form is returned"). The reason
it is still reported as a blocker is that the same guide recommends this path for ARCHOGEN, and
eADL's normative grammar defines a document as *all* of its forms plus explicit end-of-input:

```ebnf
document = { trivia } , { form , { trivia } } , end ;
```

A reader that returns one form and never requires `end` cannot implement that production. The
failure mode is the dangerous one: not an error, but a success carrying a truncated result.

**Measured probes** (`run-probes.sh`, frozen in `probes/EXPECTED.txt`):

| Probe | Input shape | Result | Exit |
| --- | --- | --- | --- |
| 01 | two top-level forms on separate lines | first form only | `0` |
| 12 | two top-level forms on one line | first form only | `0` |
| 07 | valid form followed by `this is not eADL at all &&& (((` | first form only | `0` |
| 09 | `(defblock console.uart))` — unbalanced close | form returned, stray `)` ignored | `0` |
| 06 | `(defsystem heartbeat` — unterminated | error | `1` |

Probe 06 is worth noting in LinkedSpec's favour: an unterminated form **is** rejected. The gap
is specifically *trailing* and *additional* input, not malformed input everywhere.

**Proposed fix — two independent asks, in priority order:**

1. **A complete-input mode.** An option (grammar-level or `ExecutionOptions`) that makes an
   unconsumed tail an error rather than a silent success. This is the single change that would
   make the Rust backend usable for document validation.
2. **A multi-form result.** A way to obtain *every* top-level form, not just the first.

Either one alone is a large improvement; (1) matters more, because it converts a silent wrong
answer into a diagnosable failure. We note the guide already tracks related work as
`SESSION-STARTUP-READING.83.1-.83.3` and says it is not implemented by the example — this
report is evidence for prioritising it, with a named consumer blocked on it.

**A smaller, zero-cost mitigation** while that work is pending: have `lispish_file` report the
consumed byte offset alongside the value, or warn on stderr when input remains. The adapter
comment already states it "cannot validate text the grammar already skipped"; surfacing *how
much* was skipped costs little and would have made this finding self-evident at first run.

## C — A quoted string and a bare symbol produce identical results

**Major for eADL.** Two different inputs, one output:

| Probe | Input | Result |
| --- | --- | --- |
| 04 | `(name "ARCHOGEN")` | `["name","ARCHOGEN"]` |
| 05 | `(name ARCHOGEN)` | `["name","ARCHOGEN"]` |

The guide is explicit that this is by design — `SExpression` has only `Atom(String)` and
`List(...)`, and "Lispish's parent rules discard distinctions between symbols, quoted strings and
numeric tokens". Reported because for eADL that distinction is semantic, not cosmetic: a quoted
string is a value, a bare symbol is an identifier that must resolve. Probe 03 shows the same
erasure for numbers — `(task beat (period 10 ms))` → `["task","beat",["period","10","ms"]]`,
where `10` and `ms` are both plain strings.

**Proposed fix.** Retain token kind in the result — either a tagged variant
(`Atom { text, kind }`) or, more cheaply, preserve the delimiters so the consumer can
reconstruct the distinction. Byte offsets per atom would serve both this and finding B.

## D — The PGEN bootstrap continues past a failed `cargo` invocation

**Moderate.** With finding A unfixed, `make … regex_parser_bootstrap` does not stop at the first
`cargo` failure. It ran two failing cargo commands, then printed:

```text
/bin/bash: ./target/debug/ast_pipeline: No such file or directory
🌱 generated/ebnf.rs seeded.
```

and only failed at the end with `Error 101`. Two problems:

1. **It proceeds after a failed prerequisite.** The exit code is eventually correct, but the run
   continues doing work on a foundation it knows is missing, so the reported error is far from
   the real cause. A consumer reads the tail of the log, sees the seeding message, and looks in
   the wrong place.
2. **`🌱 generated/ebnf.rs seeded.` was not true.** After that run, `generated/` existed and was
   **empty**. A success message with nothing behind it is worse than silence — this is the one
   line that most delayed diagnosis here.

**Proposed fix.** Fail fast on a non-zero `cargo` status in the bootstrap recipe, and make the
seeding message conditional on the file existing afterwards.

## E — Guide ordering

**Minor, but it costs every first consumer the same hour.** "Add and pin the source dependency"
gives five commands and ends with `git add`. A reader reasonably tries to build next. The build
cannot work: PGEN's generated sources do not exist yet, and the preparation lives two sections
later under "Keep preparation and build products local → Initial PGEN preparation". The
dependency is stated ("Checkout does not generate PGEN's parser inputs") but as a trailing
sentence rather than a blocking step.

**Proposed fix.** End the "Add and pin" section with an explicit forward pointer — *"these
commands do not produce a buildable tree; continue to Initial PGEN preparation before building"*
— or move the bootstrap into that section. Adding the `[workspace]` note from finding A in the
same place would remove both first-run failures at once.

## What this means for the first consumer

archogen needs a reader that **rejects**. Its conformance test compares acceptance *and* token
segmentation against a normative EBNF grammar; a reader that accepts by ignoring agrees on every
accept and catches nothing it should refuse. So, concretely:

- The **shipped `Lispish.spec` cannot be archogen's eADL reader**, and cannot serve as an
  independent cross-check of archogen's own recognizer, for the reasons in B and C.
- **The engine is a different question, and the answer there looks positive.** LinkedSpec is
  grammar-driven, `linkedspec-runtime` is correctly insulated as a path dependency, and the
  limitation is in one shipped grammar. An `eadl.spec` authored from archogen's own normative
  grammar — requiring complete-input consumption and all top-level forms — would be a genuine
  *independent* recognizer: different engine, different author, same normative grammar. That is
  valuable to archogen in a way a second in-house parser would not be.

Whether that is reachable depends almost entirely on **B(1)**, complete-input mode. That is the
one item we would ask to see prioritised.

## Corrections to our own earlier reading

Stated plainly, because two of our initial concerns did **not** survive measurement:

1. We expected `0x1000_0000` to be corrupted by Lispish's atom-fragment joining. It is not —
   probe 02 returns `["region","device.timer",["base","0x1000_0000"]]`, underscore intact. The
   hex-literal concern was unfounded.
2. We flagged fragment joining (`(a" b"[c]{d})` → `["a b[c]d"]`, probe 11) as a risk. It
   reproduces exactly as documented, but eADL uses neither square brackets nor braces in its
   surface syntax, so the impact on this consumer is negligible. Recorded for completeness, not
   as an ask.

## What works well

Worth saying, since the rest of this document is problems:

- **The limits table in the guide is unusually honest.** "Reading an entire file into a string is
  not proof that the grammar consumed it" is exactly right, and it predicted findings B and C
  before we measured them. Most projects do not document their extraction semantics this
  candidly.
- **The local-data discipline** (`LINKEDSPEC_PROJECT_DATA_ROOT`, `run_cargo_local.sh`,
  `project_data_run.sh`) matches a rule archogen enforces independently. It worked first try.
- **The runtime crate is correctly insulated** by `rust/Cargo.toml`'s own `[workspace]`.
- **The engine compiles and runs quickly** once bootstrapped — 34s for the example from cold,
  and the grammar compiles once and parses many inputs as advertised.

## How to reproduce

Everything below runs against a bare LinkedSpec checkout; archogen is not required.

**Finding A and D** — no build needed, ~1 minute:

```sh
bash repro/repro-workspace-collision.sh /path/to/linkedspec
# exit 0 means: reproduced 2/2 as shipped, fixed 2/2 by the proposal
```

**Findings B and C** — needs a working build. From the LinkedSpec checkout, apply finding A's
fix, run the bootstrap, build the example, then:

```sh
bash run-probes.sh \
  --bin     <target-dir>/debug/lispish_file \
  --grammar /path/to/linkedspec/specs/Lispish.spec \
  --probes  ./probes
diff <(bash run-probes.sh ...) probes/EXPECTED.txt
```

`probes/` holds twelve `.eadl` inputs drawn from real archogen constructs;
`probes/EXPECTED.txt` is the frozen output observed here, with absolute paths replaced by
`<PROBE>`. A difference is either a fix on your side or an environment difference worth
knowing about — both are useful to us.

**Finding B's headline** needs one real file rather than a probe:

```sh
lispish_file --grammar specs/Lispish.spec examples/s0-heartbeat/system.eadl
# observed: only ["defblock","console.uart",...] — three of four declarations dropped, exit 0
```

That file is in archogen at `examples/s0-heartbeat/system.eadl`; any file with two or more
top-level forms shows the same thing, and probe 01 is the minimal version.

## Contact

Raised by archogen as the first consumer of the LinkedSpec Rust backend. Findings A, D and E are
integration defects we would expect to be uncontroversial. Findings B and C are documented
behaviour, reported as requirements a named consumer is blocked on rather than as bugs.
