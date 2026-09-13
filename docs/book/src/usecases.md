# The use cases

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

## Three kinds of success

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

## The case whose answer is allowed to change

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

## The sealed set

Separately, five evaluation cases are **sealed** and may not be opened until reuse is measured
at M6. Not because they are secret — this repository is public — but because the measurement is
worthless once the engine has been shaped, even unconsciously, by what those cases need.

The seal is mechanical: a check refuses any edit to a sealed case, any unlisted file in the
sealed directory, and any tracked file outside it that names a sealed case. Its honest limit is
stated too — it cannot prove nobody read them. See `docs/evaluation/README.md`.
