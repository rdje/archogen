# Outbound feedback — vendor issue trackers

Defects archogen has reported to the projects it depends on. Each vendor gets a directory that is
a small, self-contained **issue tracker**: a register of every bug with its state, and one
sub-tree per bug holding its issue, its inputs, its reproducer and its frozen observation.

The recipient needs nothing from archogen to reproduce anything, and responds by editing the
state and history in the issue — so a report stays live instead of becoming a stale email.

## Vendors

| Vendor | Tracker | Component | Bugs | Open | Blockers |
| --- | --- | --- | --- | --- | --- |
| bedrock | [`bedrock/`](bedrock/INDEX.md) | the portable spine's handoff census | 1 | 1 | 0 |
| LinkedSpec | [`linkedspec/`](linkedspec/INDEX.md) | Rust backend + shipped Lispish grammar | 8 | 1 | 0 |

**Open** counts the bugs not yet resolved from archogen's side: `open`, `acknowledged` or
`fixed-upstream`. **Blockers** counts the open ones of severity Blocker. Per-vendor detail, meaning
severity, reproducibility and totals by state, is in that vendor's `INDEX.md`. This table is a
pointer; the vendor registers are the source. Both move in the same commit, and
`FEEDBACK-REGISTER` (`scripts/check_feedback_register.sh`) refuses a commit where they disagree.

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
└── issues/
    └── <ID>-<slug>/             one self-contained sub-tree per bug
        ├── README.md            ID, State, Severity, Kind, Component, Reproducer, History
        ├── SETUP.md             the environment and pins THIS bug needs, from scratch
        ├── repro.sh             exit 0 = reproduces · 3 = changed · 2 = could not run
        └── evidence/            its own inputs and its own frozen observation
```

**Issues share nothing.** Each carries its own setup, inputs, reproducer and frozen observation,
and no file in an issue directory references a path outside it — so a single bug can be
extracted and handed over complete. Acting on one never requires reading another's directory,
which is what makes a tracker usable by the person receiving it rather than only by the person
who wrote it.

This is enforced rather than promised: [`scripts/check_feedback_self_contained.sh`](../../scripts/check_feedback_self_contained.sh)
runs in the doctrine gate and fails the commit if an issue directory is incomplete, escapes
itself, carries a machine-specific path, or is missing from its vendor's register.

## Adding a vendor

Create `<vendor>/` with the four files above, add a row here, and own the work with a task-tree
leaf like any other change. State changes are edits to two files — the issue and the register —
in one commit.
