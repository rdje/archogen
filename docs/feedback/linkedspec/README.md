# LinkedSpec issue tracker (maintained by archogen)

A small, self-contained bug tracker for defects archogen found in the **LinkedSpec Rust backend**
and the **shipped Lispish grammar**. archogen is the first consumer of that backend, and the
upstream integration guide names ARCHOGEN in its own example, so this is offered in the spirit
the guide invites: the first real consumer reporting what the first integration actually did.

> **The register of every reported bug and its state is [`INDEX.md`](INDEX.md).**
> Currently **7 issues — 5 open, 2 of them blockers**, 1 withdrawn, 1 no-action.

**Everything LinkedSpec needs is here.** Each issue is a self-contained sub-tree holding its own
inputs, its own reproducer and its own frozen observation. Nothing outside this directory is
read, and nothing from archogen is required.

## The rule this tracker follows

**No claim leaves this repository unexecuted.** Every issue was reproduced by running commands on
a named revision, not by reading the upstream guide. Where archogen's first reading was wrong,
the issue stays in the register with a `withdrawn` or `no-action` state and the correction
written out — see `LS-006` and `LS-007`. A tracker that hides its own false positives gives no
signal about the ones that remain.

## Layout

```text
linkedspec/
├── INDEX.md                     the register — every bug, its state, the totals
├── README.md                    this file — how the tracker works
└── issues/
    └── LS-00N-<slug>/           one self-contained sub-tree per bug
        ├── README.md            the issue, the state model, and its History
        ├── SETUP.md             the environment and pins THIS issue needs, from scratch
        ├── repro.sh             its own reproducer; exit code is the verdict
        └── evidence/            its own inputs and its own frozen observation
```

**Nothing is shared between issues.** Each directory carries its own setup instructions, its own
inputs and its own frozen observation, and no file in it references a path outside it — so one
issue directory can be extracted, mailed or attached on its own and still be complete. Fixing
`LS-003` never requires reading `LS-002`'s directory, and the four issues that need a built
binary each describe that build themselves.

This is enforced, not promised: `scripts/check_feedback_self_contained.sh` in the archogen
repository fails the commit if any issue directory is incomplete, references a path outside
itself, carries a machine-specific absolute path, or is missing from `INDEX.md`.

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

To respond: edit the `State` field in the issue's `README.md`, add a dated line to its
**History**, and update the row in [`INDEX.md`](INDEX.md) — same commit, so the register never
drifts from the issues.

## Running a reproducer

Every `repro.sh` reports its verdict as an **exit code**, so a fix can be confirmed
mechanically:

| Exit | Meaning |
| --- | --- |
| `0` | The recorded observation still reproduces — the defect is present |
| `3` | Behaviour **changed** — possibly fixed; the script prints the difference |
| `2` | Could not run (missing argument or prerequisite) |

Three issues need only a checkout:

```sh
bash issues/LS-001-cargo-workspace-collision/repro.sh /path/to/linkedspec
bash issues/LS-004-bootstrap-false-success/repro.sh  /path/to/linkedspec
bash issues/LS-005-guide-ordering/repro.sh           /path/to/linkedspec
```

Four need the Lispish example binary. Each describes that build in its own `SETUP.md`:

```sh
# see issues/LS-002-multi-form-truncation/SETUP.md for the build, then:
bash issues/LS-002-multi-form-truncation/repro.sh \
  --bin <path>/lispish_file --grammar <path>/Lispish.spec
```

`LS-001`'s reproducer is **self-checking** in both directions: it asserts the defect reproduces
*and* that the proposed fix resolves it, so a green run is evidence for both halves of the report.

## The single most valuable ask

If only one thing changes upstream, make it **`LS-002`'s complete-input mode** — an option that
turns unconsumed input into an error rather than a silent success. It converts a wrong answer
into a diagnosable failure, and it is what decides whether an `eadl.spec` could ever serve as an
independent recognizer of archogen's grammar. Everything else in this tracker is smaller.

## What works well

Worth stating, since the register is otherwise all problems:

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
