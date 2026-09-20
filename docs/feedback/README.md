# Outbound feedback — vendor issue trackers

Defects archogen has reported to the projects it depends on. Each vendor gets a directory that is
a small, self-contained **issue tracker**: a register of every bug with its state, and one
sub-tree per bug holding its issue, its inputs, its reproducer and its frozen observation.

The recipient needs nothing from archogen to reproduce anything, and responds by editing the
state and history in the issue — so a report stays live instead of becoming a stale email.

## Vendors

| Vendor | Tracker | Component | Bugs | Open | Blockers |
| --- | --- | --- | --- | --- | --- |
| LinkedSpec | [`linkedspec/`](linkedspec/INDEX.md) | Rust backend + shipped Lispish grammar | 7 | 5 | 2 |

Per-vendor detail — severity, reproducibility and totals by state — is in that vendor's
`INDEX.md`. This table is a pointer; the vendor registers are the source, and both move in the
same commit.

## The rule every tracker here follows

**No claim leaves this repository unexecuted.** A report that goes out carries archogen's name,
so every observation in it is measured on a named revision with a runnable reproducer whose exit
code is its verdict. Anything archogen got wrong on first reading stays in the register in a
`withdrawn` state with the correction written out, rather than being quietly deleted — a tracker
that hides its own false positives gives no signal about the ones that remain.

## Shape of a vendor tracker

```text
<vendor>/
├── INDEX.md                     the register — every bug, its state, the totals
├── README.md                    how this tracker works, and the state model
├── SETUP.md                     environment and pins needed to reproduce
└── issues/
    └── <ID>-<slug>/             one self-contained sub-tree per bug
        ├── README.md            ID, State, Severity, Kind, Component, Reproducer, History
        ├── repro.sh             exit 0 = reproduces · 3 = changed · 2 = could not run
        └── evidence/            its own inputs and its own frozen observation
```

Issues do not share evidence or reproducers. Acting on one bug never requires reading another's
directory — which is what makes a tracker usable by the person receiving it rather than only by
the person who wrote it.

## Adding a vendor

Create `<vendor>/` with the four files above, add a row here, and own the work with a task-tree
leaf like any other change. State changes are edits to two files — the issue and the register —
in one commit.
