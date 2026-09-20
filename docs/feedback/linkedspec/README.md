# LinkedSpec issue tracker (maintained by archogen)

A small, self-contained bug tracker for defects archogen found in the **LinkedSpec Rust backend**
and the **shipped Lispish grammar**. archogen is the first consumer of that backend, and the
upstream integration guide names ARCHOGEN in its own example, so this is offered in the spirit
the guide invites: the first real consumer reporting what the first integration actually did.

**Everything LinkedSpec needs is in this directory.** The reproducers run against a bare
LinkedSpec checkout and require nothing from archogen — no archogen crate, build, or test.

## The rule this tracker follows

**No claim leaves this repository unexecuted.** Every issue below was reproduced by running
commands on a named revision, not by reading the upstream guide. Where archogen's first reading
was wrong, the issue stays in the tracker with a `withdrawn` or `no-action` state and the
correction written out — see `LS-006` and `LS-007`. A tracker that hides its own false positives
is not worth reading.

## Status board

| ID | Title | Kind | Severity | State |
| --- | --- | --- | --- | --- |
| [LS-001](issues/LS-001-cargo-workspace-collision.md) | Documented vendoring layout does not build inside a Cargo workspace | Build | Blocker | `open` |
| [LS-002](issues/LS-002-multi-form-truncation.md) | A multi-form file yields only its first form, exit `0` | Correctness | Blocker | `open` |
| [LS-003](issues/LS-003-token-kind-erasure.md) | Quoted string, bare symbol and number are indistinguishable | Correctness | Major | `open` |
| [LS-004](issues/LS-004-bootstrap-false-success.md) | PGEN bootstrap continues past a failed `cargo` and reports a false seed | Robustness | Moderate | `open` |
| [LS-005](issues/LS-005-guide-ordering.md) | "Add and pin" commands alone never produce a buildable tree | Docs | Minor | `open` |
| [LS-006](issues/LS-006-hex-underscore-withdrawn.md) | Hex literal underscores corrupted by atom joining | Correctness | — | `withdrawn` |
| [LS-007](issues/LS-007-adjacent-fragment-join.md) | Adjacent fragments join into a single atom | Correctness | Informational | `no-action` |

`LS-002` and `LS-003` are **documented upstream behaviour**, not bugs against the shipped
grammar's stated contract. They are tracked because the guide recommends this path to ARCHOGEN
specifically, and for eADL they are disqualifying. They are requirements a named consumer is
blocked on. `LS-001`, `LS-004` and `LS-005` are ordinary defects.

## State model

| State | Meaning | Who sets it |
| --- | --- | --- |
| `open` | Reported with a reproducer; no upstream response yet | archogen |
| `acknowledged` | LinkedSpec has confirmed the behaviour | LinkedSpec |
| `by-design` | Confirmed intentional; archogen must adapt or route around it | LinkedSpec |
| `fixed-upstream` | Fixed in a named revision, not yet re-measured here | LinkedSpec |
| `verified` | archogen re-ran the reproducer against that revision and it passes | archogen |
| `withdrawn` | archogen raised it in error; the correction is recorded in the issue | archogen |
| `no-action` | Reproduces as described, but no change is requested | archogen |

To respond, edit the `State` field in the issue file and add a dated line to its **History**
section. Every issue carries both.

## The single most valuable ask

If only one thing changes upstream, make it **`LS-002`'s complete-input mode** — an option that
turns unconsumed input into an error rather than a silent success. It converts a wrong answer
into a diagnosable failure, and it is what decides whether an `eadl.spec` could ever serve as an
independent recognizer of archogen's grammar. Everything else in this tracker is smaller.

## Layout

```text
linkedspec/
├── README.md                    this file — status board, state model
├── SETUP.md                     exact environment, pins, and build from scratch
├── issues/                      one file per bug, each with ID, State and History
│   └── LS-001 … LS-007
├── repro/
│   ├── LS-001-workspace-collision.sh      self-checking; needs only a LinkedSpec checkout
│   └── LS-002-003-lispish-probes.sh       replays the probe corpus
└── evidence/
    ├── system.eadl              the real 4-form eADL file behind LS-002's headline
    └── probes/                  12 eADL inputs + EXPECTED.txt (frozen observed output)
```

## Reproducing everything

Full instructions, including the toolchain and the exact pins measured, are in
[`SETUP.md`](SETUP.md). The short version:

```sh
# LS-001 and LS-004 — no build required, about a minute
bash repro/LS-001-workspace-collision.sh /path/to/linkedspec

# LS-002, LS-003, LS-006, LS-007 — needs a working build (SETUP.md)
bash repro/LS-002-003-lispish-probes.sh \
  --bin     <target>/debug/lispish_file \
  --grammar /path/to/linkedspec/specs/Lispish.spec \
  --probes  ./evidence/probes
```

`repro/LS-001-workspace-collision.sh` is **self-checking**: it exits `0` only if the defect
reproduces *and* the proposed fix resolves it, so a green run is evidence for both halves of the
report. For the probe runner, diff its output against `evidence/probes/EXPECTED.txt`; a
difference is either a fix on LinkedSpec's side or an environment difference, and both are worth
knowing.

## What works well

Worth stating, since the rest of this tracker is problems:

- **The upstream limits table is unusually honest.** "Reading an entire file into a string is not
  proof that the grammar consumed it" is exactly right, and it predicted `LS-002` and `LS-003`
  before archogen measured them. Most projects do not document extraction semantics this candidly.
- **The local-data discipline** (`LINKEDSPEC_PROJECT_DATA_ROOT`, `run_cargo_local.sh`,
  `project_data_run.sh`) matches a rule archogen enforces independently. It worked first try.
- **The runtime crate is correctly insulated** — `rust/Cargo.toml` is its own workspace root, so
  `linkedspec-runtime` as a path dependency is unaffected by `LS-001`.
- **The engine is fast once bootstrapped** — 33.94s cold build of the example, and the grammar
  compiles once and parses many inputs as advertised.

The engine is not what is being criticised here. `LS-002` and `LS-003` are properties of one
shipped grammar, and archogen's interest in LinkedSpec survives both.
