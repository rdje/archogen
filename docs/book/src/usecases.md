# The use cases

## The idea, in plain words

A fair exam is written before the course is taught; written afterwards, it tends to test what the course happens
to cover. archogen's exam is a set of example systems written down before the engine existed, each with the answer
the finished toolchain must give:

- a simple system, which must **build** and come with a report;
- a crowded one, where the honest answer may be **"cannot be sure"** rather than a reassuring "it fits";
- one that asks for something the hardware does not directly offer, where archogen must **bridge the gap or say
  what is missing**, and never quietly ask for less;
- one that needs something the profile excludes, which must stay **refused**.

A further set is **sealed**: nobody may open it until the end, when it measures how much of the engine carries over
to systems it was not shaped by.

> **In one minute, for engineers.** `docs/usecases/` defines `uc1`–`uc4`, all numbers synthetic. `uc1` must build:
> the smallest system that exercises every stage with more than one interfering task. `uc2` may answer
> `not-established` where the idealized analysis fits but the runtime-applicable one, charged with switch and ISR
> costs, does not conclude. `uc3` reports a missing engine capability until its adapter lands at M3, and the
> description is never weakened to pass. `uc4` must stay refused with `unsupported-profile` naming `general-ipc`.
> The evaluation set stays sealed until reuse is measured at M6, and a check enforces the seal.

## How it works

Four systems define what the toolchain must do, and what it must refuse. They exist before the
engine does, so the engine is built to them rather than described after the fact.

All numbers in them are **synthetic** — not measurements, not attributed to any published task
set, not claims about any board.

| Case | Exercises | Status |
| --- | --- | --- |
| `uc1-periodic-three` | the straight-through path: three periodic tasks, one timer, one output | supported |
| `uc2-high-interference` | interference and honest refusal: heavier ISR load, tight deadlines | supported; the timing property may return `not-established` |
| `uc3-alternative-timer` | indirect realization: absolute deadlines required, only a relative timer offered | refused before M3, supported from M3 |
| `uc4-bounded-queue` | profile refusal: inter-task queues the profile excludes | **must stay refused** |

Full text: `docs/usecases/`.

## The precise rules

### Three kinds of success

**`uc1` succeeds by building.** It is the smallest system that still exercises every stage —
elaboration, joint resolution with a real timer binding, lowering, emission, a build, an
analysis with more than one interfering task, and a report with separate statuses. A two-task
system would let a scheduler with a broken priority comparison pass.

**`uc2` may succeed by refusing to conclude.** It is built so the idealized analysis says
"fits" while the runtime-applicable one, charged with switch and ISR costs, may not. The
required answer is then `not-established` — not a quiet pass, and not a claimed failure. A
toolchain that reports the idealized answer for a real build has the same defect F29's controls
hunt for at the ledger level, seen from the user's side.

**`uc4` succeeds by refusing.** Its system uses inter-task queues, which the profile excludes,
so the required outcome is `unsupported-profile` naming `general-ipc` and the obligations
admitting it would add. Not a build with the queue silently replaced by a shared variable; not
a build that succeeds with the blocking term quietly missing from the report.

### The case whose answer is allowed to change

`uc3` requires an absolute-deadline service on a platform offering only a relative delay timer.
Nothing matches directly.

| Situation | Required answer |
| --- | --- |
| Before the adapter exists | a **missing engine capability** report naming what is absent |
| After the adapter exists | a built system, with the adapter's costs and obligations in the plan |
| If the adapter cannot meet the bound | `infeasible-configuration` or `not-established` naming the violated bound — never "impossible" |

What must **not** happen is the description changing. If the only way to build this system is
to weaken `absolute-deadline` into `relative-delay` in the eADL source, the engine has failed
and the boundary has been crossed.

This is the only case whose expected answer changes as a capability lands, and that is the
point: adding an engine capability and weakening a requirement until it passes both turn a red
fixture green. Only one of them is progress, and `uc3` is where the difference is visible.

### The sealed set

Separately, five evaluation cases are **sealed** and may not be opened until reuse is measured
at M6. Not because they are secret — this repository is public — but because the measurement is
worthless once the engine has been shaped, even unconsciously, by what those cases need.

The seal is mechanical, and it guards reading as well as writing. While sealed, the cases' text
is in no file of the working tree: only the commit that sealed them holds it, and the manifest
keeps each one's digest, so no search of the working tree can land in a case, whatever it looks
for, and no diff of a commit that holds one shows its text. A check refuses a set that is not the
one sealed, a digest that no longer matches its sealed text, a case back in the tree — at its
path, copied elsewhere, or a long line of it quoted — anything else in the sealed directory, and
any tracked file outside it that names a sealed case; at unsealing a restore writes each case
back and verifies it. Its honest limit is stated too: the text stays in the published history, so
a deliberate read still reaches it and the check cannot prove nobody read one — and by the
project's rule a case read early is recorded and counted apart. See
`docs/evaluation/README.md`.
