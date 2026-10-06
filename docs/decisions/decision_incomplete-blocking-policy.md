# An `incomplete` verdict does not block a CI build — and cannot be caused by the CI environment

- **Type:** `decision`
- **Date:** `2026-09-30`
- **Status:** `active`. An engineering ruling made under leaf `PROGRAM.10.3`, whose acceptance asks the
  repository for "a recorded, reasoned answer". It is reversible by one change to `scripts/ci_integration.sh`,
  and the director may overrule it.
- **Owner / source:** `ROADMAP.md` §14.3 — "A required tool skipped or unavailable is reported as such, not a
  passed check. Quarantine requires a named issue, owner, affected claim, and bounded scope."
- **External sources:** [the CI actions](../book/src/ledger.md#github-actions) · [QEMU](../book/src/ledger.md#qemu) · [mdBook](../book/src/ledger.md#mdbook) — version, scope and limits in the ledger

## The fact / decision

In CI the `integration` tier runs as `cargo xtask verify --tier integration --provisioned`, through
`scripts/ci_integration.sh`, and its exit code decides the job:

| Tier exit | Verdict | The job |
| --- | --- | --- |
| `0` | `passed` | passes |
| `1` | `failed` | **fails**, a tool the workflow did not install included |
| `20` | `incomplete` | **passes**, with a `::warning::` annotation per owned gap and the same list in the job summary |
| any other | a runner fault | fails with that code |

On a developer's machine nothing changes: `COMMIT.md` step 2 still asks for what an `incomplete` tier names to be
read before a push, and a `failed` tier still stops one.

## Why

- **Blocking on `20` would make CI red for as long as another leaf is open.** Today the one gap is the emulator
  step, quarantined under `M2.8`: the §3.2 agreement it checks has nothing to compare until `M2.8.2` and
  `M2.8.3` write the fixture and the platform description. No change to the commit under test can close that
  gap. A red build that no fix to the commit can turn green is the build people learn to ignore, and a real
  failure hidden inside a permanent red is not seen. `PROGRAM.10`'s own text named this risk before it was built.
  *Amended `2026-10-06` (`PROGRAM.59`):* that gap closed on `2026-09-30`, when `M2.8.3.4` built the agreement and
  lifted the quarantine, so the tier passes where its tools are installed. The reasoning stands for the next gap a
  leaf owns.
- **Passing silently would hide the gap, which §14.3 forbids.** So the job passes *loudly*: every run carries a
  warning annotation per gap and a summary section headed "incomplete — not a pass". The runner's own report
  names the issue, the owner, and the claim left unproven.
- ⭐ **`--provisioned` is what makes a non-blocking `20` safe.** Without it, a workflow that forgot to install
  `mdbook` would report the `book` step *unavailable*, the tier *incomplete*, and the job green with a warning:
  the book would never be built in CI, and nothing would say so above a warning. With it, a missing tool on an
  environment that claims to provide every tool is that claim failing, and the job fails. What may still leave
  CI `incomplete` is only a gap a task-tree leaf owns:
  - a **quarantine**, a row of `QUARANTINES` in `xtask/src/main.rs` with an issue, owner, claim and scope;
  - or a **step not built**, `Action::NotBuilt` with its owner leaf.

  Each gets into the tree by a reviewed code change, and each owner is checked to be a declared leaf.
- **A closed gap cannot keep its exemption.** The runner refuses a quarantined step that passes as a *stale
  quarantine*. So the day `M2.8` makes the emulator step pass, the tier fails until the row is deleted, and the
  green state it returns to is a real `passed`.

Measured `2026-09-30`, on this machine, before any workflow runs it:

- `bash scripts/ci_integration.sh` exits `0` and annotates
  `emulator 0.12s QUARANTINED — could not be run; leaf M2.8 owns the gap`.
- With `qemu-system-riscv64` hidden from `PATH`, the same tier exits `1` under `--provisioned`
  (`UNAVAILABLE on a provisioned environment`) and `20` without it.

## Considered, and not chosen

- **Block on `20`.** Rejected for the reason above. It would be right if the only gaps were ones a commit could
  close; `--provisioned` removes exactly those.
- **An allowlist of permitted gaps in the workflow.** It would declare a second time what `QUARANTINES` and
  `NotBuilt` already declare. Two lists of the same thing drift apart. That is the defect `PROGRAM.20` spent six
  instruments on.
- **An expiry date on each quarantine.** It is a stronger bound, but the date would be invented: the gap
  closes when its owner leaf lands, and that leaf's place on its tree's frontier is the real schedule. Revisit
  if a quarantine outlives its owner leaf's position on the frontier.

## How to apply

- **Adding a quarantine is using this policy.** It makes a gap non-blocking in CI. It goes in only with its four
  fields and an owner leaf that exists, and it is read as a policy decision in review, not as a code detail.
- **A new tier in CI inherits the same mapping** by calling it with `--provisioned`. The `hardware` and
  `assurance` tiers would be permanently `incomplete` today, because their steps are not built. That is
  acceptable under this policy, but the change to run them in CI is the moment to re-read it.
- Related: [[decision_push-cadence]], whose deadlock premise `PROGRAM.10.1` lifted, and
  [[a-verified-row-must-name-what-you-still-owe]].
