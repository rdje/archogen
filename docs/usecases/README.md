# The initial use cases

`ROADMAP.md` §12 M0 requires three use cases before the engine exists — a basic periodic task
set, a higher-interference task set, and a system requiring an alternative timer realization —
and requires each to state **whether it is initially supported or a rejection/extension
fixture**. A fourth case is added here as an explicit rejection fixture, because a profile
that has never refused anything has not been tested as a profile.

All numbers in these cases are **synthetic**. They are not measurements, not attributed to
any published task set, and not claims about any board. Their job is to exercise the
toolchain's decisions, not to model a real product.

| Case | Exercises | Status | First gate |
| --- | --- | --- | --- |
| [`uc1-periodic-three`](uc1-periodic-three.md) | the straight-through path: three periodic tasks, one timer, one output | supported | S0 sketch, M4 complete |
| [`uc2-high-interference`](uc2-high-interference.md) | interference and honest refusal: heavier ISR load, tight deadlines | supported, result may be `not-established` | M2 analysis, M4 execution |
| [`uc3-alternative-timer`](uc3-alternative-timer.md) | indirect realization: absolute deadlines required, only a relative timer offered | rejection fixture before M3; supported from M3 | M3 |
| [`uc4-bounded-queue`](uc4-bounded-queue.md) | profile refusal: inter-task queues the profile excludes | **rejection fixture** — must stay refused | M1 |

## What each status means

- **supported** — the toolchain is expected to admit it and produce a complete generated
  system. Failure to do so is a defect.
- **supported, result may be `not-established`** — the toolchain must admit the *system* and
  build it, while the *timing property* may legitimately come back unestablished. That is a
  pass, not a failure: §7.4 requires that "conservative analysis failure is `not-established`
  unless an exact test or validated counterexample establishes failure". Quietly reporting a
  deadline as met would be the defect.
- **rejection fixture** — the toolchain must refuse it, by name, with the reason. §12 M6 is
  explicit that successful rejection is part of the result, "without counting rejection alone
  as successful generation".
- **rejection fixture before M3; supported from M3** — a case whose status *changes* when a
  capability lands. Before `M3.2` there is no indirect-realization search, so the honest
  answer is a missing-engine-capability report; after it, the same input must build. Both
  behaviors are tested, and the transition is what proves the capability was added rather
  than the requirement weakened.

## Why a rejection fixture is not optional

The profile's whole value is the promise in §3.1: a request outside it "returns an
unsupported-profile diagnostic rather than silently reducing the requested guarantee". A
promise nothing exercises decays into a comment. `uc4` exists so that the refusal path is run
as often as the success path.

## The frozen evaluation set

Separately from these four, a set of previously unused evaluation cases is **sealed** at M0
and may not be consulted until M6 measures reuse (§12 M0, §16). Its contract — including the
mechanical check that nothing outside the sealed directory names one of its cases — is
[`docs/evaluation/README.md`](../evaluation/README.md).
