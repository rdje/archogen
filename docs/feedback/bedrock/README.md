# bedrock issue tracker (maintained by archogen)

A small, self-contained tracker for what archogen found in **bedrock**, the template archogen's portable spine came
from (`scripts/update_scaffold.sh`). archogen keeps bedrock's files as bedrock wrote them and changes none of them, so a
defect in one is reported here rather than patched there.

> **The register of every report and its state is [`INDEX.md`](INDEX.md).** Currently **1 issue, `open`**.

**Everything bedrock needs is here.** Each issue is a self-contained sub-tree holding its own reproducer and its frozen
observation. Nothing outside it is read, and nothing from archogen is required.

## The rule this tracker follows

**No claim leaves this repository unexecuted.** Every issue was reproduced by running commands on a named revision of
bedrock. Where archogen's first reading proves wrong, the issue stays in the register as `withdrawn`, the correction
written out.

## Layout

```text
bedrock/
├── INDEX.md                     the register — every report, its state, the totals
├── README.md                    this file
└── issues/
    └── <ID>-<slug>/             one self-contained sub-tree per report
        ├── README.md            ID, State, Severity, Kind, Component, Reproducer, History
        ├── SETUP.md             the environment it needs, from scratch
        ├── repro.sh             exit 0 = reproduces · 3 = changed · 2 = could not run
        └── evidence/            the frozen observation
```

## State model

| State | Meaning | Who sets it |
| --- | --- | --- |
| `open` | Reported with a reproducer; no upstream response yet | archogen |
| `acknowledged` | bedrock has confirmed the behaviour | bedrock |
| `by-design` | Confirmed intentional; archogen must adapt or route around it | bedrock |
| `fixed-upstream` | Fixed in a named revision, not yet re-measured here | bedrock |
| `verified` | archogen re-ran the reproducer against that revision and it passes | archogen |
| `withdrawn` | archogen raised it in error; the correction is recorded in the issue | archogen |
| `no-action` | Reproduces as described, but no change is requested | archogen |

To respond: edit the `State` field in the issue's `README.md`, add a dated line to its **History**, and update the row
in [`INDEX.md`](INDEX.md) — same commit, so the register never drifts from the issues.
